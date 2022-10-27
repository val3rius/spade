use crate::content::{Article, ArticleContent, Asset, Content};
use crate::error::Error;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{fs, path};

#[derive(Clone)]
pub struct Filesystem {
    path: path::PathBuf,
}

impl Filesystem {
    pub fn new(path: path::PathBuf) -> Self {
        Filesystem { path }
    }
}

impl crate::traits::Reader for Filesystem {
    fn read_all(&self) -> Result<HashMap<String, Content>, crate::error::Error> {
        let mut hm = HashMap::new();
        recursive_read(path::PathBuf::default(), &self.path)?
            .into_iter()
            .for_each(|(path, mut file)| {
                if Filetype::from(&path) == Filetype::Markdown {
                    let mut buf: Vec<u8> = vec![];
                    file.read_to_end(&mut buf).unwrap(); //TODO
                    let id = id_from_path(&path);
                    hm.insert(
                        id.clone(),
                        Content::Article(Article {
                            id,
                            permalink: permalink_from_path(&path),
                            src: path.to_str().unwrap().to_string(),
                            meta: None,
                            raw: String::from_utf8_lossy(&buf).into(),
                            content: None,
                        }),
                    );
                } else {
                    let id = id_from_path(&path);
                    hm.insert(
                        id.clone(),
                        Content::Asset(Asset {
                            id,
                            permalink: permalink_from_path(&path),
                            src: path.into_os_string().into_string().unwrap(),
                        }),
                    );
                }
            });
        Ok(hm)
    }
    fn get_reader(&self, src: &str) -> Box<dyn Read> {
        let file = std::fs::File::open(format!("{}/{}", self.path.to_str().unwrap(), src)).unwrap();
        let bf = std::io::BufReader::new(file);
        Box::new(bf)
    }
}

impl crate::traits::Writer for Filesystem {
    fn get_writer(&self, permalink: &str) -> Box<dyn Write> {
        let path =
            std::path::PathBuf::from(format!("{}/{}", self.path.to_str().unwrap(), &(*permalink)));
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        Box::new(std::io::BufWriter::new(
            std::fs::File::create(path).unwrap(),
        ))
    }
}

fn recursive_read(
    path_prefix: path::PathBuf,
    path: &path::Path,
) -> Result<Vec<(path::PathBuf, std::fs::File)>, Error> {
    let mut content = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let metadata = fs::metadata(&entry_path)?;

        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or("");

        // Ignore hidden files
        if file_name.starts_with('.') {
            continue;
        }
        let mut new_path = path_prefix.clone();
        new_path.push(file_name);

        if metadata.is_dir() {
            if let Ok(mut inner_content) = recursive_read(new_path.clone(), &entry_path.clone()) {
                content.append(&mut inner_content)
            }
        }

        if metadata.is_file() {
            let file = fs::File::open(entry_path)?;
            content.push((new_path, file));
        }
    }

    Ok(content)
}

#[derive(Debug, PartialEq)]
enum Filetype {
    Markdown,
    Other,
}

impl std::convert::From<&std::path::PathBuf> for Filetype {
    fn from(path: &std::path::PathBuf) -> Self {
        match path.extension() {
            None => Filetype::Other,
            Some(ext) => match ext.to_str() {
                Some("md") => Filetype::Markdown,
                _ => Filetype::Other,
            },
        }
    }
}

pub fn id_from_path(path: &std::path::Path) -> String {
    let mut path = path.to_path_buf();
    if let Filetype::Markdown = Filetype::from(&path) {
        path.set_extension("");
    }
    path.to_str().unwrap_or("").to_string()
}

pub fn permalink_from_path(path: &std::path::Path) -> String {
    let mut path = path.to_path_buf();
    if let Filetype::Markdown = Filetype::from(&path) {
        path.set_extension("");
    }
    let slugified = slugify_path(&path).to_str().unwrap_or("").to_string();
    format!("/{}", slugified)
}

// slugify_path takes a PathBuf and makes it URL friendly
// by running all fragments (except file extension) through `slug::slugify`.
fn slugify_path(path: &path::Path) -> path::PathBuf {
    let mut new_path = path.to_owned();
    let ext = path.extension();
    new_path.set_extension("");
    new_path = new_path
        .iter()
        .map(|fragment| {
            return slug::slugify(fragment.to_str().unwrap_or(""));
        })
        .collect::<path::PathBuf>();
    if let Some(extension) = ext {
        new_path.set_extension(extension);
    }
    new_path
}
