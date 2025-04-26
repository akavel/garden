# "Medion Life E1800" teardown

## other brands

Based on a search of the "SB1024H" marking
found on the pulse sensor's connector tape,
and then a visual comparison of the results,
the following smartwatch brands & models might be the same device:

- "Medion Life E1800" (obviously - that's the one I tore down)
- "iGET FIT F4" ([mention](https://iget.eu/sites/data/nositelnosti/ce/ce_declaration_of_conformity_fit%20f4%20silver.pdf))
- "Everis Neptune E4002" ([mention](https://play.google.com/store/apps/details?id=com.oplayer.everisapp&hl=en))

## display

([Question on r/hardwarehacking](https://redd.it/1k8lzl9))

The back side seems to show the following markings:

    R096HQ1501A(L?)
    180913 A2 P(O?)

## pulse/heart rate sensor

The elastic tape seems to have the markings:

    AB220-SB1024H-HRS33CO FPC-V1.0-20171215

_Update:_ based on googling, the third element of the first part
is most probably rather HRS3300,
as this seems to be a [known](https://docs.rs/hrs3300) heart rate
[sensor](https://files.pine64.org/doc/datasheet/pinetime/HRS3300%20Heart%20Rate%20Sensor.pdf).
And "H-R-S" can be an acronym of a "Heart Rate Sensor".
Thus, the markings are probably rather:

    AB220-SB1024H-HRS3300 FPC-V1.0-20171215

Interestingly, googling for SB1024H led me to
[a document on iget.eu website](https://iget.eu/sites/data/nositelnosti/ce/ce_declaration_of_conformity_fit%20f4%20silver.pdf),
and further googling up an "iGET FIT F4"
seems to show images looking very similar
to how the "Medion Life E1800" looks.
