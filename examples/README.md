# Spade example usage

This will generate a site in the `generated` directory and serve it using nginx.

```
spade --source content --destination generated --theme theme

docker run --rm --name some-nginx -p 8080:80 -v $(pwd)/nginx.conf:/etc/nginx/nginx.conf -v $(pwd)/generated:/usr/share/nginx/html:ro nginx

```
