#!/bin/sh

cargo -q run run \
  --code cHViIGZuIGhlbGxvX3dvcmxkKCkgewogICAgcHJpbnRsbiEoImhlbGxvIHdvcmxkIikKfQo= \
  --challenge printing-hello-world \
  -n 10
