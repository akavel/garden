# NiMH batteries charging - notes

*WORK IN PROGRESS NOTES*

(see also: [clawtype](clawtype))

## software/firmware based {#software-firmware-based}

### @bigclivedotcom

I asked [a question on Mastodon](https://merveilles.town/@akavel/117314143744202236).
Right after posting it, and also in the only till now useful answer,
I discovered some videos on bigclivedotcom's youtube channel.
The most important introductory one is:
- [Charging NiMH cells - smart vs dumb chargers](https://www.youtube.com/watch?v=5yFwhWeqyq4)

Another super useful one from the same channel:
- [Simple NiMH battery charger (with PCB files)](https://www.youtube.com/watch?v=wVnAH17f4jg)

The guy has a website at https://bigclive.com,
and it looks like there's also a subreddit r/BigCliveDotCom.

### other notes

Software-based charger logic written in Rust
should be portable enough to easily upload
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


## based on specialized ICs {#based-on-specialized-ics}

Issue here is those seem to regularly go out of market,
and already rare desins based on those on the web
thus become difficult to reuse.

Try "Maxim DS2712" (or "DS2710") 
([via](https://youtu.be/S9PUO_Uw158),
[via](https://hackaday.com/2024/12/02/the-automatic-battery-charger-you-never-knew-you-needed/)).

TME search for [1+ NiMH charging ICs](https://www.tme.eu/pl/en/katalog/battery-and-battery-cells-controllers_112884/?params=2613:1503863;550:1925651,1834448&productListOrderBy=1000014)
