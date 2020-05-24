# Rust and Pixels

This is the scaffold for the Renuo learning day 2020.

## Expensive colors

Let's go back to 1985 when colors did cost a lot of money: Commodore announced its Amiga 1000 system which was
capable of showing all 4096 colors (12bit) on the screen simultaneously. This really blew people's minds. They were 
used to monochrome screens (PAL was introduced just some years ago) or to not more than 16 colors. If your screen was
able to show colors, you could choose between quite some different computer systems:


![](https://upload.wikimedia.org/wikipedia/commons/5/5a/Screen_color_test_AppleII_HighRes.png)
![](https://upload.wikimedia.org/wikipedia/commons/e/ef/Screen_color_test_Commodore64_Multicolor.png)
![](https://upload.wikimedia.org/wikipedia/commons/7/7b/Screen_color_test_EGA_16colors.png)
![](https://upload.wikimedia.org/wikipedia/commons/b/be/Screen_color_test_Amiga_4096colors_HAM.png)

Apple II with 6 colors | Commodore C64 with 16 colors | IBM EGA 16 colors | Amiga HAM6 mode

The Apple II was quite slow by 1985, but you would get good laser printer support only on Macs. The C64 was quite
cheap and you got tons of software for it. The IBM PC got the fastest CPU clock speeds. The Amiga revolutionized
the market for a short time with fantastic graphics and sound support.

Technically good graphics were possible already, but extremely expensive. The Amiga on the otherhand was affordable.
It used a Motorola CPU clocked at 7.09MHz and PAL frames at 15'625 lines per second. Since video memory is shared
with the CPU, this means that you already use up a lot of your [CPU cycles for managing graphics memory](https://retrocomputing.stackexchange.com/a/2149). So the Amiga engineers used a neat trick to cheat these limits.
It's called [Hold-and-Modify](https://en.wikipedia.org/wiki/Hold-And-Modify).

Today we're going to look a bit deeper into this trick and reimplement Hold-and-Modify (OCS HAM6 mode) in Rust.

## Timetable

| Time  | Activity |
| ----: | -------- |
|  8:00 | Kickoff  |
| 11:40 | Merging  |
| 13:00 | Forking  |
| 17:00 | Wrap-up  |

## Installation

Have [rustup](https://rustup.rs/) ready, checkout this repo and run `cargo test` and `cargo run`.

## Afternoon Tracks

In the afternoon you're free to follow down these rabbit holes.

* Retro Computing
  Color clocks https://retrocomputing.stackexchange.com/a/2149 http://amigadev.elowar.com/read/ADCD_2.1/Hardware_Manual_guide/node012A.html 
  Amiga Graphics modes: Halfbrite, DigiView Dynamic (Copper hacks) http://theamigamuseum.com/the-hardware/the-amigas-graphic-modes/ https://wiki.amigaos.net/wiki/Classic_Graphics_Primitives#Advanced_Topics
  CRT monitor simulation on today's systems
* Rust
  Rust language and libraries:
  image library extension for the Ham6Image, binary image format for ham6, planar instead of packed pixels
  improve the code of the HAM solution regarding performance, Amiga accuracy or clean code.
* Computer Graphics
  CRT & NTSC/PAL https://en.wikipedia.org/wiki/Interlaced_video https://en.wikipedia.org/wiki/Cathode-ray_tube https://en.wikipedia.org/wiki/Multisync_monitor
  Color palettes (https://en.wikipedia.org/wiki/List_of_16-bit_computer_color_palettes) and color spaces (https://de.wikipedia.org/wiki/Farbraum)
  HAM Palette optimization and encoding (e.g. with ML) http://theamigamuseum.com/the-hardware/the-amigas-graphic-modes/
  Chroma Subsampling (https://github.com/leandromoreira/digital_video_introduction#chroma-subsampling)
  Pixel (definition, history, size, length, etc.)
