# Notes about bootloader on _Arduino Nano 33 BLE Rev2_

From Adafruit:

_"(...) the Nordic SoftDevice (...) must be loaded in flash at a fixed location: 0x0000 to 0x26000._
_But the Arduino bootloader is also at 0x0000, and does not include the SoftDevice._
_It also assumes that programs start at 0x2000, the usual offset. (I think: still studying this.)_
_The Mbed code compiled by the Arduino IDE does not use the SoftDevice for BLE: it uses an Mbed library."_
([via](https://forums.adafruit.com/viewtopic.php?f=60&t=158279))

From TockOS:

_"The bossa bootloader expects that all application code (i.e. not the bootloader) starts at address 0x10000._
_That is, when the bootloader finishes it starts executing at address 0x10000._
([via](https://github.com/tock/tock/tree/e1a744a4bb01f3f865616d9d5c31e1db9001bba9/boards/nano33ble#getting-started))



## JTAG/... Debug connection

For emergency programming or bootloader re-flashing,
a "normal" JTAG/SWD debug interface is available as well with some effort.
The bottom of the Nano33BLE board has the following 5 debug pads at the "radio" ("ublox") end:

    |         |
    |  o o o  |
    |  o   o  |
     ---------

The official docs give them numbers as below:

    5 3 1
    6   2

Two pads among them
([via](https://support.arduino.cc/hc/en-us/articles/8991429732124-Burn-the-bootloader-on-Arduino-Nano-33-IoT),
[and](https://forums.adafruit.com/viewtopic.php?f=60&t=158279))
are the most important ones for debugging:

- 2 - SWDIO
- 3 - SWCLK

Other than that, 3.3V and GND also need to be connected,
but that (as well as !RST if needed) is documented as being accessible
through appropriate "main" headers of the Arduino.
(Though, with more difficulty, also doable via the debug pads as:
pad 1 = 3.3V, pad 5 = GND, pad 6 = !RST.)
