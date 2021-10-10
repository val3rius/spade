# Spade

A digital gardening tool

Spade is a static site generator with an emphasis on zettelkasten style writing.

## Features (so far)

- Internal [[links]] are automatically resolved (see examples/content)
- A graph structure of all internal links are produced to navigate the site/garden

## Usage (--help)

```
Spade 0.1.0-alpha
A tool for digital gardeners

USAGE:
    spade [FLAGS] --destination <destination> --source <source> --theme <theme>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --watch      Re-generate the site whenever the source or theme directories change

OPTIONS:
    -d, --destination <destination>    Sets the destination folder path
    -s, --source <source>              Sets the source folder path
    -t, --theme <theme>                Sets the theme folder path
```
