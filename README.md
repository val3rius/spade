# Spade

A digital gardening tool

Spade is a static site generator with an emphasis on zettelkasten style writing. It's intended to primarily drive my own personal site, and is therefore until further notice tightly coupled to that specific use case (e.g. the "default theme" may contain traces of my site, configuration options are low priority, et cetera).

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
