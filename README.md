# find_dups
Poking at finding cases where glyphs were copied between families

```shell
# Assumes ../fonts contains a clone of https://github.com/google/fonts
$ cargo run -- ../fonts/ofl/moulpali/Moulpali-Regular.ttf ../fonts/ofl/share/Share-Regular.ttf ../fonts/ofl/bayon/Bayon-Regular.ttf ../fonts/ofl/koulen/Koulen-Regular.ttf ../fonts/ofl/angkor/Angkor-Regular.ttf ../fonts/ofl/moul/Moul-Regular.ttf ../fonts/ofl/lobster/Lobster-Regular.ttf ../fonts/ofl/galada/Galada-Regular.ttf 

...noise...

Showing groups where at least 56/70 glyphs match

Group, Score
{"../fonts/ofl/angkor/Angkor-Regular.ttf", "../fonts/ofl/moul/Moul-Regular.ttf"}, 61/70
{"../fonts/ofl/bayon/Bayon-Regular.ttf", "../fonts/ofl/koulen/Koulen-Regular.ttf"}, 61/70
{"../fonts/ofl/moulpali/Moulpali-Regular.ttf", "../fonts/ofl/share/Share-Regular.ttf"}, 61/70

$ cargo run -- --dump-glyphs --match-pct 50 '../fonts/ofl/notosanstamil/NotoSansTamil[wdth,wght].ttf' '../fonts/ofl/notoseriftamil/NotoSerifTamil[wdth,wght].ttf'     '../fonts/ofl/notosans/NotoSans[wdth,wght].ttf' '../fonts/ofl/notoserif/NotoSerif[wdth,wght].ttf'

...noise...

Group, Score
{"../fonts/ofl/notosans/NotoSans[wdth,wght].ttf", "../fonts/ofl/notosanstamil/NotoSansTamil[wdth,wght].ttf"}, 49/70
{"../fonts/ofl/notoserif/NotoSerif[wdth,wght].ttf", "../fonts/ofl/notoseriftamil/NotoSerifTamil[wdth,wght].ttf"}, 68/70

```