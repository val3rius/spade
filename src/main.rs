use crate::traits::{Reader, Writer};
use clap::{App, Arg};
use comrak::{markdown_to_html, ComrakOptions};
use content::Content;
use filesystem::Filesystem;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use tera::Tera;
mod content;
mod error;
mod filesystem;
mod links;
mod meta;
mod traits;
#[macro_use]
extern crate lazy_static;
///
/// The core logic of the program is fairly minimal.
/// We recursively read all files from our source folder.
/// We then iterate over them once to build a map and parse out internal references.
/// After that we iterate over the graph once more in order to process, render and write the new files.
///
fn main() -> Result<(), error::Error> {
    let matches = App::new("Spade")
        .version("0.1.0-alpha")
        .about("digital gardening tool")
        .arg(
            Arg::with_name("source")
                .long("source")
                .short("s")
                .takes_value(true)
                .required(true)
                .help("Sets the source folder path"),
        )
        .arg(
            Arg::with_name("destination")
                .long("destination")
                .short("d")
                .takes_value(true)
                .required(true)
                .help("Sets the destination folder path"),
        )
        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .takes_value(true)
                .required(true)
                .help("Sets the theme folder path"),
        )
        .arg(
            Arg::with_name("watch")
                .short("w")
                .long("watch")
                .help("Re-generate the site whenever the source or theme directories change"),
        )
        .get_matches();

    // These settings are all required, so let's bail early if they for some reason
    // show up as None.
    let src_path = matches.value_of("source").expect("Invalid source value");
    let dst_path = matches
        .value_of("destination")
        .expect("Invalid destination value");
    let theme_path = matches.value_of("theme").expect("Invalid theme path");

    // Ok 3, 2, 1, let's jam...!
    generate_site(src_path, dst_path, theme_path)?;

    // If the `watch` flag is set, we set up a notifier and loop indefinitely
    // to re-generate the site whenever there are file changes in our
    // source or theme directories.
    if matches.is_present("watch") {
        println!("Watching for changes...");
        let (tx, rx) = channel();

        let mut src_watcher: RecommendedWatcher = Watcher::new(tx.clone(), Duration::from_secs(1))?;
        src_watcher.watch(src_path, RecursiveMode::Recursive)?;

        let mut theme_watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
        theme_watcher.watch(theme_path, RecursiveMode::Recursive)?;

        loop {
            if let Ok(notify::DebouncedEvent::Write(_)) = rx.recv() {
                generate_site(src_path, dst_path, theme_path)?;
            }
        }
    }

    Ok(())
}

fn generate_site(src_path: &str, dst_path: &str, theme_path: &str) -> Result<(), error::Error> {
    println!("Generating site...");
    // Start timer
    let now = Instant::now();
    // Register our template files as any file ending in .html in our templates
    // directory.
    let mut renderer = Tera::new(&format!("{}/templates/**/*.html", theme_path))?; //TODO validate that path exists

    // Turn off autoescape for the HTML renderr since we trust our own content.
    renderer.autoescape_on(vec![]);

    // Set up our filesystem handlers for our source and destination directories.
    let src = Filesystem::new(path::PathBuf::from(src_path));
    let dst = Filesystem::new(path::PathBuf::from(dst_path));

    let contents = src.read_all()?;
    let references = content::get_references(&contents);
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();
    let graph = content::json_graph(&contents, &references);

    //
    // Traverse the contents again to write to file, now that
    // all internal links have been resolved.
    //
    contents.iter().for_each(|(_id, content)| {
        match content {
            //
            // For Markdown files, we pass the content to the templating engine,
            // update the extension to .html and then write to file.
            //
            Content::Article(article) => {
                //
                // Resolve internal wikilinks and image links.
                //
                let mut article = links::replace(&contents, article).unwrap();

                //
                // Split out and parse the metadata YAML.
                //
                let (meta, content) = meta::extract(article.content);
                article.meta = meta;
                article.content = content;

                let mut article_tags = vec![];

                // Update the tag lookup map
                if let Some(m) = article.meta.clone() {
                    if let Some(t) = m.tags {
                        t.into_iter().for_each(|tag| {
                            match tags.entry(tag.clone()) {
                                Entry::Vacant(e) => {
                                    e.insert(vec![article.id.clone()]);
                                }
                                Entry::Occupied(mut e) => {
                                    e.get_mut().push(article.id.clone());
                                }
                            };
                            article_tags.push(tag);
                        });
                    }
                }

                //
                // Render HTML from Markdown
                //
                let mut comrak_opts = ComrakOptions::default();
                comrak_opts.extension.tasklist = true;
                comrak_opts.extension.table = true;
                comrak_opts.render.unsafe_ = true;
                article.content = markdown_to_html(&article.content, &comrak_opts);

                //
                // Set up rendering context.
                //
                let mut ctx = tera::Context::new();
                ctx.insert("id", &article.id);
                ctx.insert("meta", &article.meta);
                ctx.insert("content", &article.content);
                ctx.insert("tags", &article_tags);

                let rendered = renderer.render("default.html", &ctx).unwrap(); //TODO

                //
                // Set up a writer for our output file and write the rendered
                // content to it.
                //
                let mut w = dst.get_writer(&format!("{}.html", &article.permalink));
                w.write_all(rendered.as_bytes())
                    .expect("Unable to write to destination");
            }
            //
            // For all other files, we just copy them over.
            //
            Content::Asset(asset) => {
                std::io::copy(
                    &mut src.get_reader(&asset.src),
                    &mut dst.get_writer(&asset.permalink),
                )
                .unwrap();
            }
        }
    });

    // Move theme assets
    // TODO ignore if the directory doesnt exist
    let theme_assets = Filesystem::new(path::PathBuf::from(format!("{}/assets", theme_path))); //TODO validate path
    let asset_files = theme_assets.read_all()?;
    asset_files
        .into_iter()
        .filter_map(|(_k, c)| match c {
            Content::Asset(a) => Some(a),
            _ => None,
        })
        .for_each(|asset| {
            std::io::copy(
                &mut theme_assets.get_reader(&asset.src),
                &mut dst.get_writer(&format!("/assets/{}", &asset.permalink)),
            )
            .unwrap();
        });

    // Write graph.json
    let mut w = dst.get_writer("/assets/graph.json");
    w.write_all(graph.as_bytes())
        .expect("Unable to write graph.json to destination");

    // Render and write tags pages
    tags.iter().for_each(|(tag, article_ids)| {
        let mut ctx = tera::Context::new();
        let link_map: HashMap<String, String> =
            article_ids.iter().fold(HashMap::new(), |mut m, id| {
                if let Some(article) = content::get_article(&contents, id) {
                    m.insert((&article.id).to_string(), (&article.permalink).to_string());
                }
                m
            });

        ctx.insert("links", &link_map);
        let rendered = renderer.render("tag.html", &ctx).unwrap(); //TODO
        let mut w = dst.get_writer(&format!("/tags/{}.html", tag));
        w.write_all(rendered.as_bytes())
            .expect("Unable to write graph.json to destination");
    });

    println!(
        "Site generated in {} milliseconds",
        now.elapsed().as_millis()
    );
    Ok(())
}
