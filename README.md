# find_dups
Poking at finding cases where glyphs were copied between families

```shell
# Assumes ../fonts contains a clone of https://github.com/google/fonts
$ cargo run -- ../fonts/ofl/moulpali/Moulpali-Regular.ttf ../fonts/ofl/share/Share-Regular.ttf
$ cargo run -- ../fonts/ofl/bayon/Bayon-Regular.ttf ../fonts/ofl/koulen/Koulen-Regular.ttf
$ cargo run -- ../fonts/ofl/angkor/Angkor-Regular.ttf ../fonts/ofl/moul/Moul-Regular.ttf

# all three should report all test glyphs match

```