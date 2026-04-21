# find_dups
Poking at finding cases where glyphs were copied between families

## Sample usage

```shell
# Assumes ../fonts contains a clone of https://github.com/google/fonts
$ cargo run -- --match-pct 60 ../fonts/ofl/moulpali/Moulpali-Regular.ttf ../fonts/ofl/share/Share-Regular.ttf ../fonts/ofl/bayon/Bayon-Regular.ttf ../fonts/ofl/koulen/Koulen-Regular.ttf ../fonts/ofl/angkor/Angkor-Regular.ttf ../fonts/ofl/moul/Moul-Regular.ttf ../fonts/ofl/lobster/Lobster-Regular.ttf ../fonts/ofl/galada/Galada-Regular.ttf 

...noise...

Showing groups where at least 56/92 glyphs match

Group, Score
{"../fonts/ofl/moulpali/Moulpali-Regular.ttf", "../fonts/ofl/share/Share-Regular.ttf"}, 90/92
{"../fonts/ofl/angkor/Angkor-Regular.ttf", "../fonts/ofl/moul/Moul-Regular.ttf"}, 90/92
{"../fonts/ofl/galada/Galada-Regular.ttf", "../fonts/ofl/lobster/Lobster-Regular.ttf"}, 58/92
{"../fonts/ofl/bayon/Bayon-Regular.ttf", "../fonts/ofl/koulen/Koulen-Regular.ttf"}, 90/92

$ cargo run -- --dump-glyphs --match-pct 50 '../fonts/ofl/notosanstamil/NotoSansTamil[wdth,wght].ttf' '../fonts/ofl/notoseriftamil/NotoSerifTamil[wdth,wght].ttf'     '../fonts/ofl/notosans/NotoSans[wdth,wght].ttf' '../fonts/ofl/notoserif/NotoSerif[wdth,wght].ttf'

...noise...

Group, Score
{"../fonts/ofl/notosans/NotoSans[wdth,wght].ttf", "../fonts/ofl/notosanstamil/NotoSansTamil[wdth,wght].ttf"}, 49/70
{"../fonts/ofl/notoserif/NotoSerif[wdth,wght].ttf", "../fonts/ofl/notoseriftamil/NotoSerifTamil[wdth,wght].ttf"}, 68/70

```

## Results

Update me as program improves :)

### 2/26/2024

```shell
$ time cargo run -- --google-fonts ../fonts/

Showing groups where at least 74/92 glyphs match

Group, Score
{"../fonts/ofl/fasthand/Fasthand-Regular.ttf", "../fonts/ofl/freehand/Freehand-Regular.ttf", "../fonts/ofl/seaweedscript/SeaweedScript-Regular.ttf", "../fonts/ofl/taprom/Taprom-Regular.ttf"}, 81/92
{"../fonts/ofl/cairo/Cairo[slnt,wght].ttf", "../fonts/ofl/cairoplay/CairoPlay[slnt,wght].ttf"}, 86/92
{"../fonts/ofl/bokor/Bokor-Regular.ttf", "../fonts/ofl/pirataone/PirataOne-Regular.ttf"}, 85/92
{"../fonts/ofl/mclaren/McLaren-Regular.ttf", "../fonts/ofl/preahvihear/Preahvihear-Regular.ttf"}, 89/92
{"../fonts/ofl/oflsortsmillgoudytt/OFLGoudyStMTT.ttf", "../fonts/ofl/sortsmillgoudy/SortsMillGoudy-Regular.ttf"}, 89/92
{"../fonts/ofl/hind/Hind-Regular.ttf", "../fonts/ofl/hindcolombo/HindColombo-Regular.ttf", "../fonts/ofl/hindguntur/HindGuntur-Regular.ttf", "../fonts/ofl/hindjalandhar/HindJalandhar-Regular.ttf", "../fonts/ofl/hindkochi/HindKochi-Regular.ttf", "../fonts/ofl/hindmadurai/HindMadurai-Regular.ttf", "../fonts/ofl/hindmysuru/HindMysuru-Regular.ttf", "../fonts/ofl/hindsiliguri/HindSiliguri-Regular.ttf", "../fonts/ofl/hindvadodara/HindVadodara-Regular.ttf"}, 86/92
{"../fonts/ofl/zenkakugothicantique/ZenKakuGothicAntique-Regular.ttf", "../fonts/ofl/zenkakugothicnew/ZenKakuGothicNew-Regular.ttf"}, 82/92
{"../fonts/ofl/bayon/Bayon-Regular.ttf", "../fonts/ofl/koulen/Koulen-Regular.ttf", "../fonts/ofl/staatliches/Staatliches-Regular.ttf"}, 89/92
{"../fonts/ofl/akayakanadaka/AkayaKanadaka-Regular.ttf", "../fonts/ofl/akayatelivigala/AkayaTelivigala-Regular.ttf"}, 88/92
{"../fonts/ofl/devonshire/Devonshire-Regular.ttf", "../fonts/ofl/mashanzheng/MaShanZheng-Regular.ttf", "../fonts/ofl/zhimangxing/ZhiMangXing-Regular.ttf"}, 85/92
{"../fonts/ofl/lexend/Lexend[wght].ttf", "../fonts/ofl/lexenddeca/LexendDeca[wght].ttf", "../fonts/ofl/readexpro/ReadexPro[HEXP,wght].ttf"}, 85/92
{"../fonts/ofl/mochiypopone/MochiyPopOne-Regular.ttf", "../fonts/ofl/mochiypoppone/MochiyPopPOne-Regular.ttf"}, 89/92
{"../fonts/ofl/rasa/Rasa[wght].ttf", "../fonts/ofl/yrsa/Yrsa[wght].ttf"}, 88/92
{"../fonts/ofl/anekbangla/AnekBangla[wdth,wght].ttf", "../fonts/ofl/anekdevanagari/AnekDevanagari[wdth,wght].ttf", "../fonts/ofl/anekgujarati/AnekGujarati[wdth,wght].ttf", "../fonts/ofl/anekgurmukhi/AnekGurmukhi[wdth,wght].ttf", "../fonts/ofl/anekkannada/AnekKannada[wdth,wght].ttf", "../fonts/ofl/aneklatin/AnekLatin[wdth,wght].ttf", "../fonts/ofl/anekmalayalam/AnekMalayalam[wdth,wght].ttf", "../fonts/ofl/anekodia/AnekOdia[wdth,wght].ttf", "../fonts/ofl/anektamil/AnekTamil[wdth,wght].ttf", "../fonts/ofl/anektelugu/AnekTelugu[wdth,wght].ttf"}, 87/92
{"../fonts/ofl/angkor/Angkor-Regular.ttf", "../fonts/ofl/moul/Moul-Regular.ttf"}, 90/92
{"../fonts/ofl/librebarcode128/LibreBarcode128-Regular.ttf", "../fonts/ofl/librebarcode128text/LibreBarcode128Text-Regular.ttf"}, 90/92
{"../fonts/apache/creepstercaps/CreepsterCaps-Regular.ttf", "../fonts/ofl/creepster/Creepster-Regular.ttf"}, 90/92
{"../fonts/ofl/bungee/Bungee-Regular.ttf", "../fonts/ofl/bungeecolor/BungeeColor-Regular.ttf", "../fonts/ofl/bungeespice/BungeeSpice-Regular.ttf"}, 90/92
{"../fonts/ofl/amiri/Amiri-Regular.ttf", "../fonts/ofl/amiriquran/AmiriQuran-Regular.ttf"}, 85/92
{"../fonts/ofl/kaiseidecol/KaiseiDecol-Regular.ttf", "../fonts/ofl/kaiseiharunoumi/KaiseiHarunoUmi-Regular.ttf", "../fonts/ofl/kaiseiopti/KaiseiOpti-Regular.ttf", "../fonts/ofl/kaiseitokumin/KaiseiTokumin-Regular.ttf"}, 78/92
{"../fonts/ofl/notosanshk/NotoSansHK[wght].ttf", "../fonts/ofl/notosansjp/NotoSansJP[wght].ttf", "../fonts/ofl/notosanskr/NotoSansKR[wght].ttf", "../fonts/ofl/notosanssc/NotoSansSC[wght].ttf", "../fonts/ofl/notosanstc/NotoSansTC[wght].ttf"}, 89/92
{"../fonts/ofl/reemkufi/ReemKufi[wght].ttf", "../fonts/ofl/reemkufifun/ReemKufiFun[wght].ttf", "../fonts/ofl/reemkufiink/ReemKufiInk-Regular.ttf"}, 88/92
{"../fonts/ofl/yujihentaiganaakari/YujiHentaiganaAkari-Regular.ttf", "../fonts/ofl/yujihentaiganaakebono/YujiHentaiganaAkebono-Regular.ttf"}, 89/92
{"../fonts/ofl/nosifer/Nosifer-Regular.ttf", "../fonts/ofl/nosifercaps/NosiferCaps-Regular.ttf"}, 90/92
{"../fonts/ofl/odormeanchey/OdorMeanChey-Regular.ttf", "../fonts/ofl/patuaone/PatuaOne-Regular.ttf"}, 80/92
{"../fonts/ofl/blaka/Blaka-Regular.ttf", "../fonts/ofl/blakaink/BlakaInk-Regular.ttf"}, 81/92
{"../fonts/ofl/elmessiri/ElMessiri[wght].ttf", "../fonts/ofl/philosopher/Philosopher-Regular.ttf", "../fonts/ofl/zcoolxiaowei/ZCOOLXiaoWei-Regular.ttf"}, 78/92
{"../fonts/ofl/castoro/Castoro-Regular.ttf", "../fonts/ofl/tirobangla/TiroBangla-Regular.ttf", "../fonts/ofl/tirodevanagarihindi/TiroDevanagariHindi-Regular.ttf", "../fonts/ofl/tirodevanagarimarathi/TiroDevanagariMarathi-Regular.ttf", "../fonts/ofl/tirodevanagarisanskrit/TiroDevanagariSanskrit-Regular.ttf", "../fonts/ofl/tirogurmukhi/TiroGurmukhi-Regular.ttf", "../fonts/ofl/tirokannada/TiroKannada-Regular.ttf", "../fonts/ofl/tirotamil/TiroTamil-Regular.ttf", "../fonts/ofl/tirotelugu/TiroTelugu-Regular.ttf"}, 83/92
{"../fonts/ofl/fascinate/Fascinate-Regular.ttf", "../fonts/ofl/fascinateinline/FascinateInline-Regular.ttf"}, 87/92
{"../fonts/ofl/mplus1code/MPLUS1Code[wght].ttf", "../fonts/ofl/mpluscodelatin/MPLUSCodeLatin[wdth,wght].ttf"}, 86/92
{"../fonts/apache/robotoslab/RobotoSlab[wght].ttf", "../fonts/ofl/battambang/Battambang-Regular.ttf", "../fonts/ofl/hanuman/Hanuman-Regular.ttf", "../fonts/ofl/suwannaphum/Suwannaphum-Regular.ttf"}, 85/92
{"../fonts/ofl/arefruqaa/ArefRuqaa-Regular.ttf", "../fonts/ofl/arefruqaaink/ArefRuqaaInk-Regular.ttf"}, 88/92
{"../fonts/ofl/anuphan/Anuphan[wght].ttf", "../fonts/ofl/ibmplexsans/IBMPlexSans-Regular.ttf", "../fonts/ofl/ibmplexsansarabic/IBMPlexSansArabic-Regular.ttf", "../fonts/ofl/ibmplexsansdevanagari/IBMPlexSansDevanagari-Regular.ttf", "../fonts/ofl/ibmplexsanshebrew/IBMPlexSansHebrew-Regular.ttf", "../fonts/ofl/ibmplexsanskr/IBMPlexSansKR-Regular.ttf", "../fonts/ofl/ibmplexsansthai/IBMPlexSansThai-Regular.ttf", "../fonts/ofl/ibmplexsansthailooped/IBMPlexSansThaiLooped-Regular.ttf"}, 85/92
{"../fonts/ofl/concertone/ConcertOne-Regular.ttf", "../fonts/ofl/dangrek/Dangrek-Regular.ttf"}, 90/92
{"../fonts/ofl/geostar/Geostar-Regular.ttf", "../fonts/ofl/geostarfill/GeostarFill-Regular.ttf"}, 89/92
{"../fonts/ofl/notoserifhk/NotoSerifHK[wght].ttf", "../fonts/ofl/notoserifjp/NotoSerifJP[wght].ttf", "../fonts/ofl/notoserifkr/NotoSerifKR[wght].ttf", "../fonts/ofl/notoserifsc/NotoSerifSC[wght].ttf", "../fonts/ofl/notoseriftc/NotoSerifTC[wght].ttf"}, 88/92
{"../fonts/apache/irishgrover/IrishGrover-Regular.ttf", "../fonts/ofl/lakkireddy/LakkiReddy-Regular.ttf"}, 90/92

real	57m23.476s
user	57m30.812s
sys	0m5.124s

# If I had remembered to build --release then run:
real	13m44.851s
user	13m43.066s
sys	0m1.734s
```

That was appallingly slow.

BUG: a few static families were omitted due to not having a -Regular. Now fixed.

### 4/24/2026

```
➜  find_dups git:(main) time cargo run --release -- --google-fonts ../fonts/

    Finished `release` profile [optimized] target(s) in 0.09s
     Running `target/release/find_dups --google-fonts ../fonts/`
Showing groups where at least 74/92 glyphs match

Group, Score
{"../fonts/apache/irishgrover/IrishGrover-Regular.ttf", "../fonts/ofl/lakkireddy/LakkiReddy-Regular.ttf"}, 90/92
{"../fonts/ofl/playpensans/PlaypenSans[wght].ttf", "../fonts/ofl/playpensansarabic/PlaypenSansArabic[wght].ttf", "../fonts/ofl/playpensansdeva/PlaypenSansDeva[wght].ttf", "../fonts/ofl/playpensanshebrew/PlaypenSansHebrew[wght].ttf", "../fonts/ofl/playpensansthai/PlaypenSansThai[wght].ttf"}, 85/92
{"../fonts/ofl/anton/Anton-Regular.ttf", "../fonts/ofl/antonsc/AntonSC-Regular.ttf"}, 86/92
{"../fonts/ofl/oflsortsmillgoudytt/OFLGoudyStMTT.ttf", "../fonts/ofl/sortsmillgoudy/SortsMillGoudy-Regular.ttf"}, 88/92
{"../fonts/ofl/sixtyfour/Sixtyfour[BLED,SCAN].ttf", "../fonts/ofl/sixtyfourconvergence/SixtyfourConvergence[BLED,SCAN,XELA,YELA].ttf"}, 90/92
{"../fonts/ofl/mplus1code/MPLUS1Code[wght].ttf", "../fonts/ofl/mpluscodelatin/MPLUSCodeLatin[wdth,wght].ttf"}, 86/92
{"../fonts/ofl/bpmfiansui/BpmfIansui-Regular.ttf", "../fonts/ofl/iansui/Iansui-Regular.ttf", "../fonts/ofl/lxgwwenkaitc/LXGWWenKaiTC-Regular.ttf"}, 81/92
{"../fonts/ofl/fragmentmono/FragmentMono-Regular.ttf", "../fonts/ofl/fragmentmonosc/FragmentMonoSC-Regular.ttf"}, 88/92
{"../fonts/ofl/yujihentaiganaakari/YujiHentaiganaAkari-Regular.ttf", "../fonts/ofl/yujihentaiganaakebono/YujiHentaiganaAkebono-Regular.ttf"}, 89/92
{"../fonts/ofl/devonshire/Devonshire-Regular.ttf", "../fonts/ofl/mashanzheng/MaShanZheng-Regular.ttf", "../fonts/ofl/zhimangxing/ZhiMangXing-Regular.ttf"}, 85/92
{"../fonts/ofl/andika/Andika-Regular.ttf", "../fonts/ofl/kanchenjunga/Kanchenjunga-Regular.ttf"}, 86/92
{"../fonts/ofl/librebarcode128/LibreBarcode128-Regular.ttf", "../fonts/ofl/librebarcode128text/LibreBarcode128Text-Regular.ttf"}, 88/92
{"../fonts/ofl/alumnisanscollegiateone/AlumniSansCollegiateOne-Regular.ttf", "../fonts/ofl/alumnisanscollegiateonesc/AlumniSansCollegiateOneSC-Regular.ttf"}, 87/92
{"../fonts/ofl/mochiypopone/MochiyPopOne-Regular.ttf", "../fonts/ofl/mochiypoppone/MochiyPopPOne-Regular.ttf"}, 89/92
{"../fonts/ofl/eduauvicwanthand/EduAUVICWANTHand[wght].ttf", "../fonts/ofl/eduauvicwantpre/EduAUVICWANTPre[wght].ttf"}, 87/92
{"../fonts/ofl/cairo/Cairo[slnt,wght].ttf", "../fonts/ofl/cairoplay/CairoPlay[slnt,wght].ttf"}, 83/92
{"../fonts/ofl/notosanshk/NotoSansHK[wght].ttf", "../fonts/ofl/notosansjp/NotoSansJP[wght].ttf", "../fonts/ofl/notosanskr/NotoSansKR[wght].ttf", "../fonts/ofl/notosanssc/NotoSansSC[wght].ttf", "../fonts/ofl/notosanstc/NotoSansTC[wght].ttf"}, 89/92
{"../fonts/ofl/concertone/ConcertOne-Regular.ttf", "../fonts/ofl/dangrek/Dangrek-Regular.ttf"}, 90/92
{"../fonts/ofl/cascadiacode/CascadiaCode[wght].ttf", "../fonts/ofl/cascadiamono/CascadiaMono[wght].ttf"}, 88/92
{"../fonts/ofl/reemkufi/ReemKufi[wght].ttf", "../fonts/ofl/reemkufifun/ReemKufiFun[wght].ttf", "../fonts/ofl/reemkufiink/ReemKufiInk-Regular.ttf"}, 87/92
{"../fonts/ofl/castoro/Castoro-Regular.ttf", "../fonts/ofl/tirobangla/TiroBangla-Regular.ttf", "../fonts/ofl/tirodevanagarihindi/TiroDevanagariHindi-Regular.ttf", "../fonts/ofl/tirodevanagarimarathi/TiroDevanagariMarathi-Regular.ttf", "../fonts/ofl/tirodevanagarisanskrit/TiroDevanagariSanskrit-Regular.ttf", "../fonts/ofl/tirogurmukhi/TiroGurmukhi-Regular.ttf", "../fonts/ofl/tirokannada/TiroKannada-Regular.ttf", "../fonts/ofl/tirotamil/TiroTamil-Regular.ttf", "../fonts/ofl/tirotelugu/TiroTelugu-Regular.ttf"}, 83/92
{"../fonts/ofl/anuphan/Anuphan[wght].ttf", "../fonts/ofl/ibmplexsans/IBMPlexSans[wdth,wght].ttf", "../fonts/ofl/ibmplexsansarabic/IBMPlexSansArabic-Regular.ttf", "../fonts/ofl/ibmplexsansdevanagari/IBMPlexSansDevanagari-Regular.ttf", "../fonts/ofl/ibmplexsanshebrew/IBMPlexSansHebrew-Regular.ttf", "../fonts/ofl/ibmplexsanskr/IBMPlexSansKR-Regular.ttf", "../fonts/ofl/ibmplexsansthai/IBMPlexSansThai-Regular.ttf", "../fonts/ofl/ibmplexsansthailooped/IBMPlexSansThaiLooped-Regular.ttf"}, 84/92
{"../fonts/ofl/nosifer/Nosifer-Regular.ttf", "../fonts/ofl/nosifercaps/NosiferCaps-Regular.ttf"}, 90/92
{"../fonts/ofl/anekbangla/AnekBangla[wdth,wght].ttf", "../fonts/ofl/anekdevanagari/AnekDevanagari[wdth,wght].ttf", "../fonts/ofl/anekgujarati/AnekGujarati[wdth,wght].ttf", "../fonts/ofl/anekgurmukhi/AnekGurmukhi[wdth,wght].ttf", "../fonts/ofl/anekkannada/AnekKannada[wdth,wght].ttf", "../fonts/ofl/aneklatin/AnekLatin[wdth,wght].ttf", "../fonts/ofl/anekmalayalam/AnekMalayalam[wdth,wght].ttf", "../fonts/ofl/anekodia/AnekOdia[wdth,wght].ttf", "../fonts/ofl/anektamil/AnekTamil[wdth,wght].ttf", "../fonts/ofl/anektelugu/AnekTelugu[wdth,wght].ttf"}, 87/92
{"../fonts/ofl/hind/Hind-Regular.ttf", "../fonts/ofl/hindcolombo/HindColombo-Regular.ttf", "../fonts/ofl/hindguntur/HindGuntur-Regular.ttf", "../fonts/ofl/hindjalandhar/HindJalandhar-Regular.ttf", "../fonts/ofl/hindkochi/HindKochi-Regular.ttf", "../fonts/ofl/hindmadurai/HindMadurai-Regular.ttf", "../fonts/ofl/hindmysuru/HindMysuru-Regular.ttf", "../fonts/ofl/hindsiliguri/HindSiliguri-Regular.ttf", "../fonts/ofl/hindvadodara/HindVadodara-Regular.ttf"}, 84/92
{"../fonts/ofl/angkor/Angkor-Regular.ttf", "../fonts/ofl/moul/Moul-Regular.ttf"}, 90/92
{"../fonts/ofl/geostar/Geostar-Regular.ttf", "../fonts/ofl/geostarfill/GeostarFill-Regular.ttf"}, 89/92
{"../fonts/ofl/sedan/Sedan-Regular.ttf", "../fonts/ofl/sedansc/SedanSC-Regular.ttf"}, 85/92
{"../fonts/ofl/bodonimoda/BodoniModa[opsz,wght].ttf", "../fonts/ofl/bodonimodasc/BodoniModaSC[opsz,wght].ttf"}, 87/92
{"../fonts/ofl/elmessiri/ElMessiri[wght].ttf", "../fonts/ofl/philosopher/Philosopher-Regular.ttf", "../fonts/ofl/zcoolxiaowei/ZCOOLXiaoWei-Regular.ttf"}, 77/92
{"../fonts/ofl/zenkakugothicantique/ZenKakuGothicAntique-Regular.ttf", "../fonts/ofl/zenkakugothicnew/ZenKakuGothicNew-Regular.ttf"}, 82/92
{"../fonts/ofl/notoserifhk/NotoSerifHK[wght].ttf", "../fonts/ofl/notoserifjp/NotoSerifJP[wght].ttf", "../fonts/ofl/notoserifkr/NotoSerifKR[wght].ttf", "../fonts/ofl/notoserifsc/NotoSerifSC[wght].ttf", "../fonts/ofl/notoseriftc/NotoSerifTC[wght].ttf"}, 87/92
{"../fonts/ofl/rasa/Rasa[wght].ttf", "../fonts/ofl/yrsa/Yrsa[wght].ttf"}, 87/92
{"../fonts/ofl/arsenal/Arsenal-Regular.ttf", "../fonts/ofl/arsenalsc/ArsenalSC-Regular.ttf"}, 86/92
{"../fonts/apache/robotoslab/RobotoSlab[wght].ttf", "../fonts/ofl/battambang/Battambang-Regular.ttf", "../fonts/ofl/hanuman/Hanuman[wght].ttf", "../fonts/ofl/suwannaphum/Suwannaphum-Regular.ttf"}, 85/92
{"../fonts/ofl/playwriteczguides/PlaywriteCZGuides-Regular.ttf", "../fonts/ofl/playwriteskguides/PlaywriteSKGuides-Regular.ttf"}, 87/92
{"../fonts/ofl/betaniapatmos/BetaniaPatmos-Regular.ttf", "../fonts/ofl/betaniapatmosin/BetaniaPatmosIn-Regular.ttf"}, 88/92
{"../fonts/ofl/odormeanchey/OdorMeanChey-Regular.ttf", "../fonts/ofl/patuaone/PatuaOne-Regular.ttf"}, 80/92
{"../fonts/ofl/bungee/Bungee-Regular.ttf", "../fonts/ofl/bungeecolor/BungeeColor-Regular.ttf", "../fonts/ofl/bungeeinline/BungeeInline-Regular.ttf", "../fonts/ofl/bungeespice/BungeeSpice-Regular.ttf", "../fonts/ofl/bungeetint/BungeeTint-Regular.ttf"}, 87/92
{"../fonts/ofl/bokor/Bokor-Regular.ttf", "../fonts/ofl/pirataone/PirataOne-Regular.ttf"}, 85/92
{"../fonts/ofl/jaini/Jaini-Regular.ttf", "../fonts/ofl/jainipurva/JainiPurva-Regular.ttf"}, 90/92
{"../fonts/ofl/akayakanadaka/AkayaKanadaka-Regular.ttf", "../fonts/ofl/akayatelivigala/AkayaTelivigala-Regular.ttf"}, 88/92
{"../fonts/ofl/betaniapatmosgdl/BetaniaPatmosGDL-Regular.ttf", "../fonts/ofl/betaniapatmosingdl/BetaniaPatmosInGDL-Regular.ttf"}, 84/92
{"../fonts/ofl/bigshouldersinlinetext/BigShouldersInlineText[wght].ttf", "../fonts/ofl/bigshouldersinlinetextsc/BigShouldersInlineTextSC[wght].ttf"}, 89/92
{"../fonts/ofl/bonanova/BonaNova-Regular.ttf", "../fonts/ofl/bonanovasc/BonaNovaSC-Regular.ttf"}, 87/92
{"../fonts/ofl/akatab/Akatab-Regular.ttf", "../fonts/ofl/tirra/Tirra-Regular.ttf"}, 86/92
{"../fonts/ofl/bpmfhuninn/BpmfHuninn-Regular.ttf", "../fonts/ofl/huninn/Huninn-Regular.ttf"}, 89/92
{"../fonts/apache/creepstercaps/CreepsterCaps-Regular.ttf", "../fonts/ofl/creepster/Creepster-Regular.ttf"}, 90/92
{"../fonts/ofl/amiri/Amiri-Regular.ttf", "../fonts/ofl/amiriquran/AmiriQuran-Regular.ttf"}, 85/92
{"../fonts/ofl/fasthand/Fasthand-Regular.ttf", "../fonts/ofl/freehand/Freehand-Regular.ttf", "../fonts/ofl/seaweedscript/SeaweedScript-Regular.ttf", "../fonts/ofl/taprom/Taprom-Regular.ttf"}, 81/92
{"../fonts/ofl/alumnisans/AlumniSans[wght].ttf", "../fonts/ofl/alumnisanssc/AlumniSansSC[wght].ttf"}, 84/92
{"../fonts/ofl/ibmplexmono/IBMPlexMono-Regular.ttf", "../fonts/ofl/lilex/Lilex[wght].ttf"}, 84/92
{"../fonts/ofl/iosevkacharon/IosevkaCharon-Regular.ttf", "../fonts/ofl/iosevkacharonmono/IosevkaCharonMono-Regular.ttf"}, 77/92
{"../fonts/ofl/kaiseidecol/KaiseiDecol-Regular.ttf", "../fonts/ofl/kaiseiharunoumi/KaiseiHarunoUmi-Regular.ttf", "../fonts/ofl/kaiseiopti/KaiseiOpti-Regular.ttf", "../fonts/ofl/kaiseitokumin/KaiseiTokumin-Regular.ttf"}, 79/92
{"../fonts/ofl/arefruqaa/ArefRuqaa-Regular.ttf", "../fonts/ofl/arefruqaaink/ArefRuqaaInk-Regular.ttf"}, 88/92
{"../fonts/ofl/baskervville/Baskervville[wght].ttf", "../fonts/ofl/baskervvillesc/BaskervvilleSC[wght].ttf"}, 88/92
{"../fonts/ofl/42dotsans/42dotSans[wght].ttf", "../fonts/ofl/astasans/AstaSans[wght].ttf"}, 86/92
{"../fonts/ofl/mclaren/McLaren-Regular.ttf", "../fonts/ofl/preahvihear/Preahvihear-Regular.ttf"}, 89/92
{"../fonts/ofl/afacad/Afacad[wght].ttf", "../fonts/ofl/afacadflux/AfacadFlux[slnt,wght].ttf"}, 83/92
{"../fonts/ofl/alyamama/Alyamama[wght].ttf", "../fonts/ofl/ancizarserif/AncizarSerif[wght].ttf"}, 86/92
{"../fonts/ofl/bayon/Bayon-Regular.ttf", "../fonts/ofl/koulen/Koulen-Regular.ttf", "../fonts/ofl/staatliches/Staatliches-Regular.ttf"}, 89/92
{"../fonts/ofl/lexend/Lexend[wght].ttf", "../fonts/ofl/lexenddeca/LexendDeca[wght].ttf", "../fonts/ofl/readexpro/ReadexPro[HEXP,wght].ttf"}, 85/92
{"../fonts/ofl/spectral/Spectral-Regular.ttf", "../fonts/ofl/spectralsc/SpectralSC-Regular.ttf"}, 87/92
{"../fonts/ofl/blaka/Blaka-Regular.ttf", "../fonts/ofl/blakahollow/BlakaHollow-Regular.ttf", "../fonts/ofl/blakaink/BlakaInk-Regular.ttf"}, 80/92
cargo run --release -- --google-fonts ../fonts/  745.62s user 11.72s system 98% cpu 12:51.34 total
```
