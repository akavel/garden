# Clawtype

[Firmware (AGPL; GH)](https://github.com/akavel/clawtype).

[Structural Design](https://www.printables.com/model/1231156-clawtype).

## NiMH charging

### software/firmware based

Software-based charger logic written in Rust
should be portavke enough to easily upload
to any current or future generic microcontroller.

Long-term goal is:
- can charge from 5V supplied by USB
- can detect reverse polarity
- can fast-charge (though at first could try just trickle-charge)
- 1 AA cell charging is enough
- can detect LiPo (and either reject or also charge appropriately)
- main device can run off USB while the battery is charging,
- then transparently switch to NiMH power source when USB is disconnected (and reverse)

Promising-looking writeups and software:
- https://github.com/msillano/NiMH_charger_logger
- https://github.com/stawel/cheali-charger/blob/master/docs/nimh_nicd_charging.md
- Wikipedia has some actually useful looking info about NiMH charging:
  https://en.wikipedia.org/wiki/Nickel%E2%80%93metal_hydride_battery
- not strictly a charger but still an interesting writeup:
  https://github.com/MarkusWandel/battery-tester

Not about charging at all,
but an interesting writeup about NiMH batteries:
https://github.com/dwyl/home/issues/209


### based on specialized ICs

Issue here is those seem to regularly go out of market,
and already rare desins based on those on the web
thus become difficult to reuse.

Try "Maxim DS2712" (or "DS2710") 
([via](https://youtu.be/S9PUO_Uw158),
[via](https://hackaday.com/2024/12/02/the-automatic-battery-charger-you-never-knew-you-needed/)).

TME search for [1+ NiMH charging ICs](https://www.tme.eu/pl/en/katalog/battery-and-battery-cells-controllers_112884/?params=2613:1503863;550:1925651,1834448&productListOrderBy=1000014)

## fanless, USBC-powered MiniPC

- "Mele 4C - N series" e.g. "Quieter 4C N100"? ([via](https://redd.it/1jgd7rr)) "Quieter2Q"? ([via](https://redd.it/vlyz96))
  - some [problems with USBC-PD if running off USB-C powerbank](https://old.reddit.com/r/MiniPCs/comments/1esw8w3/mele_mini_quieter_4c/lpy806e/)
- "Topton"? ([via](https://redd.it/1jgd7rr))

## fun with low-power displays

- "Nokia 5110 LCD" 84x48 px, b&w, 3.3V
- "Sharp memory LCDs"
  ([specs overview](https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/6165/lcd202009e.pdf);
  [digikey search](https://www.digikey.pl/pl/products/filter/lcd-oled-grafikk/107?s=N4IgjCBcoLQdIDGUBmBDANgZwKYBoQB7KAbRABYAmAVhAF0BfBggNilAEsATKEOABggEADgBdeIAgEdRAT15h%2BkkHOE5eaLMiZA),
  filter by Sharp)
  - the first number in their name seems to indicate size (diagonal?) in inches - e.g.: "LS027..." -> 2.7"
  - the connection seems to be "SPI over (10-pin? 0.5mm?) FPC" (or "...FFC")
    where FPC seems to mean "[flexible printed circuit](https://en.wikipedia.org/wiki/Flexible_printed_circuit)"
    and FFC to mean "[flexible flat cable](https://en.wikipedia.org/wiki/Flexible_flat_cable)".
    The FPC/FFC seems to be standardized,
    with 0.5mm pitch connectors and breakout boards existing for various pin-widths and easily available.
  - LS010B7DH04
  - LS011B7DH03 - 1.08" ~32x14mm outline, 160x68px;
    seems used in the popular "nano!view" display module;
  - LS012B7DD01
  - LS012B7DD06 (rgb) / LS012B7DD06A (rgb) / LS012B7DH06A (?) - (round??)
  - LS013B7DD02 (rgb)
  - LS013B7DH05 - 1.26" ~30x25mm outline, 168x144px;
    ([datasheet](https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/2328/LS013B7DH05.pdf))
    seems 3.3V based
  - LS013B7DH03 - 1.28" ~31x27mm outline, 128x128px;
  - LS013B7DH06 (rgb) - 1.33" ~32x27mm outline ~24x24mm active, 128x128px;
    ([datasheet](https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/734/LS013B7DH06_Spec.pdf))
    needs 5V :(
  - LS014B7DD01 (rgb, round??) - 1.39" ~39x38mm outline, 280x280px;
  - LS018B7DH02 - 1.8" ~42x31mm outline, 303x230px;
  - LS021B7DD02 - (rgb) 2.13" 320x240px seems colored and not easily available :(
    didn't check datasheet for voltages
  - LS027B7DH01 / LS027B7DH01A - 2.7" ~63x43mm (outline) 400x240px;
    from [datasheet](https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/1272/LS027B7DH01_Rev_Jun_2010.pdf)
    it seems they need 5V
    (so [an extra boost converter from 3.3v](https://www.tindie.com/products/kuzyatech/sharp-memory-lcd-breakout-a2/)) :(
    [Reportedly](https://forum.digikey.com/t/ls027b7dh01-vs-ls027b7dh01a/5682),
    the "...A" suffix should indicate up to 2 bad pixels per display, whereas non-A should indicate zero
    ([or untested](https://forum.digikey.com/t/sharp-lq057q3dc03-tft-lcd-module-a-last-suffix/4820)?).
- WeAct Studio ePaper ([github](https://github.com/WeActStudio/WeActStudio.EpaperModule))
  - 1.54" ~50x33mm module outline, 27x27mm active area, 200x200px, seems 3.3V;
    ([datasheet](https://github.com/WeActStudio/WeActStudio.EpaperModule/blob/master/Doc/ZJY200200-0154DAAMFGN.pdf))
    I think it supports partial refresh? not sure yet what is the power consumption

## v2.1 discussions

[HN](https://news.ycombinator.com/item?id=43588420),
[lobste.rs](https://lobste.rs/s/o0xmgd/clawtype_custom_wearable_chorded),
[r/ErgoMechKeyboards](https://redd.it/1jrg5ul),
[r/cyberdeck](https://redd.it/1jwrnkv),
[r/Xreal](https://redd.it/1jrfupe),
[r/ErgoMobileComputers](https://redd.it/1jrg6ti),
[r/PeripheralDesign](https://redd.it/1jsx7p9),
[mastodon](https://merveilles.town/@akavel/114278656676862031)

## Chordite
- [John W. McKown's patent](https://patents.google.com/patent/US6429854)
- the [chordite.com page on WebArchive](https://web.archive.org/web/20220201061603/http://chordite.com/)

## alternatives

- characorder 2 (via: TODO)
  - [some review but of CC1, and at least some they claim to have since addressed](https://www.youtube.com/watch?v=IxCm86IbLok) ([via](https://old.reddit.com/r/typing/comments/1c0oi1s/how_fast_is_the_charachorder_compared_to/ll5h3bi/))
