# find_dups
Poking at finding cases where glyphs were copied between families

```shell
# Assumes ../fonts contains a clone of https://github.com/google/fonts
# Galada has Latin copied from Lobster
$ cargo run -- ../fonts/ofl/galada/Galada-Regular.ttf \
    ../fonts/ofl/lobster/Lobster-Regular.ttf
```