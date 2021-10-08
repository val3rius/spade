use crate::traits::{Reader, Writer};
use clap::{App, Arg};
use comrak::{markdown_to_html, ComrakOptions};
use meta::Meta;
use std::collections::HashMap;
use std::path;
use tera::Tera;
mod error;
mod filesystem;
mod links;
mod meta;
mod repository;
mod traits;
#[macro_use]
extern crate lazy_static;

/// Content is any item of data that we want to move or process
/// from our source to our destination.
#[derive(Debug)]
pub enum Content {
    Article(Article),
    Asset(Asset),
}

#[derive(Clone, Debug)]
pub struct Article {
    id: String,
    permalink: String,
    src: String,
    meta: Option<Meta>,
    content: String,
}
#[derive(Debug)]
pub struct Asset {
    id: String,
    permalink: String,
    src: String,
}

///
/// The core logic of the program is fairly minimal.
/// We recursively read all files from our source folder.
/// We then iterate over them once to build a map and parse out internal references.
/// After that we iterate over the graph once more in order to process, render and write the new files.
///
fn main() -> Result<(), error::Error> {
    let matches = App::new("Spade")
        .version("0.1.0-alpha")
        .about("A tool for digital gardeners")
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
        .get_matches();

    // These settings are all required, so let's bail early if they for some reason
    // show up as None.
    let src_path = matches.value_of("source").expect("Invalid source value");
    let dst_path = matches
        .value_of("destination")
        .expect("Invalid destination value");
    let theme_path = matches.value_of("theme").expect("Invalid theme path");

    // Register our template files as any file ending in .html in our templates
    // directory.
    let mut renderer = Tera::new(&format!("{}/**/*.html", tmpl_path))?; //TODO validate that path exists

    // Turn off autoescape for the HTML renderr since we trust our own content.
    renderer.autoescape_on(vec![]);

    // Load contents from the file system
    let src = filesystem::Filesystem::new(path::PathBuf::from(src_path));
    let dst = filesystem::Filesystem::new(path::PathBuf::from(dst_path));
    let repo = repository::Repository::new(src.read_all()?);

    //
    // Traverse the contents again to write to file, now that
    // all internal links have been resolved.
    //
    repo.contents.iter().for_each(|(id, content)| {
        match content {
            //
            // For Markdown files, we pass the content to the templating engine,
            // update the extension to .html and then write to file.
            //
            Content::Article(article) => {
                //
                // Resolve internal wikilinks and image links.
                //
                let mut article = links::replace(&repo, article).unwrap();

                //
                // Split out and parse the metadata YAML.
                //
                let (meta, content) = meta::extract(article.content);
                article.meta = meta;
                article.content = content;

                //
                // Render HTML from Markdown
                //
                let mut comrak_opts = ComrakOptions::default();
                comrak_opts.extension.tasklist = true;
                comrak_opts.extension.table = true;
                article.content = markdown_to_html(&article.content, &comrak_opts);

                //
                // Inbound references are the "backlinks" from other articles that get displayed
                // at the bottom of each article (or wherever).
                //
                let inbound_references = repo.get_inbound_references(id).into_iter().fold(
                    HashMap::new(),
                    |mut m, id| {
                        if let Some(referencing_article) = repo.get_article(&id) {
                            m.insert(id, format!("/{}", referencing_article.permalink));
                        }
                        m
                    },
                );

                //
                // Set up rendering context.
                //
                let mut ctx = tera::Context::new();
                ctx.insert("meta", &article.meta);
                ctx.insert("content", &article.content);
                ctx.insert("inbound_references", &inbound_references);

                let rendered = renderer.render("default.html", &ctx).unwrap(); //TODO

                //
                // Set up a writer for our output file and write the rendered
                // content to it.
                //
                let mut w = dst.get_writer(&article.permalink);
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

    Ok(())
}
