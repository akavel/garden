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

From one guy's sample blinky project I found on github for Nano-33-BLE,
I copy-pasted the following `memory.x` file,
which seemed to work for me
with the builtin bossa bootloader:
```
MEMORY
{
  FLASH (rx) : ORIGIN = 0x10000, LENGTH = 0xf0000
  RAM_NVIC (rwx) : ORIGIN = 0x20000000, LENGTH = 0x100
  RAM_CRASH_DATA (rwx) : ORIGIN = (0x20000000 + 0x100), LENGTH = 0x100
  RAM (rwx) : ORIGIN = ((0x20000000 + 0x100) + 0x100), LENGTH = (0x40000 - (0x100 + 0x100))
}
OUTPUT_FORMAT ("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
```
([via](https://github.com/NorbertSzydlik/rust-arduino-nano-33-ble/blob/50c97b32cc5e115ca8ef50ab08eba4f05170cad7/memory.x))


## SoftDevice

According to an example in the embassy project,
the SoftDevice S140 (v7.3.0)
expects the user application to start at 0x27000
and RAM at 0x2002_0000:
```
  /* These values correspond to the NRF52840 with Softdevices S140 7.3.0 */
  /*
     FLASH : ORIGIN = 0x00027000, LENGTH = 868K
     RAM : ORIGIN = 0x20020000, LENGTH = 128K
  */
```
([via](https://github.com/embassy-rs/embassy/blob/ca5ebe859a40af38a889553334afbcc22cf1aba7/examples/nrf52840/memory.x#L7-L11))



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
