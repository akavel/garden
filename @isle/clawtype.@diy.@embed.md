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
h Wikipedia has some actually useful looking info about NiMH charging:
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
