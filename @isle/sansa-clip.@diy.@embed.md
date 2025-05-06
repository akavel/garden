# sansa clip+

During some spring cleaning,
I just found the Sansa Clip+
I bought long ago and have installed [Rockbox](https://www.rockbox.org) on.
I think it would be interesting to try and
replace the battery in it
(I'm quite pretty sure it's dead or nearly so -
though maybe I should try just in case...),
now that I have some basic soldering skills.

Helpful videos and posts I found:

- "[Sansa Clip + Full Dissasembly Teardown and Reassembly](https://youtu.be/beMcITVdZcE)".
  Important and useful notes:
  - use a guitar pick (or, from other sources, just a credit card should probably work?),
    _not_ a screwdriver;
  - start prying from corners; be very patient walking around;
    be super careful around microSD slot and avoid strength there,
    it's a narrow strip of plastic and super easy to break;
  - top & bottom part have two battery wires near the bottom of the device,
    so when doing the final opening do it from the top;
  - there's some double-sided tape inside.
 
  Also, notably, as a replacement for the broken clip mechanism,
  the owner used a velcro strap glud to the back of the case -
  this is a cool idea for me too, as the clip in mine is also broken.

- "[Sansa Clip+ Battery Replacement](https://web.archive.org/web/20190512092935/http://connor-brooks.com/sansa-battery.html)"
  ([via](https://hackaday.com/2018/11/21/battery-swap-keeps-sansa-clip-chugging/),
  [via](https://hackaday.com/tag/sansa-clip/)) -
  some notable quotes:
  
  _"The OEM battery measures 30mm x 36mm x 3mm (...)
  it is possible to find a battery slightly smaller that will fit. At 30mm x 30mm x 3mm, the 303030 is a good fit. (...)
  **These batteries only have 2 wires whilst the original has 3.**
  At first this seemed an issue, as the third wire is generally used for an internal temperature sensor,
  and a lot electronics refuse to work without one. (...)
  **Luckily, it seems the third wire isn't actually required on the Sansa.**
  After trimming the wires and soldering the new battery onto the board, the device was able to boot. (...)
  For anyone attempting this repair who encounters any issues charging,
  it may be related to the lack of a 3rd wire on the new battery's IC.
  If this is the case, it should be possible to
  [fix this](https://electronics.stackexchange.com/questions/152053/replace-3-wires-tablet-battery-with-2-wires-one/152058#152058)
  by soldering a 10kilo ohm resistor between the T pin and ground.
  Just remember to take care when playing with batteries."_

- "[If We Can Do It, So Can You - Sansa Clip+ Battery Replacement](https://youtu.be/ramuFlFGSr8)" -
  they found some 3-wire battery;
  according to some comments,
  the battery seems to be a "323036(P)" (with or without the "P").
  Also, notably, even if the battery is drained when installed,
  the device should boot up when plugged in to USB.
