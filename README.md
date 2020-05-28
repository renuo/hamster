# Rust and Pixels

This repository contains the Rust scaffold for the Renuo learning day 2020.
The day will follow this timetable:

| Time  | Activity          |
| ----: | ----------------- |
|  8:00 | Introduction      |
| 11:40 | Showcase          |
| 13:00 | Follow the rabbit |
| 17:00 | Wrap-up           |

## Introduction

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

Technically good graphics were possible already, but extremely expensive. The Amiga on the other hand was affordable.
It used a Motorola CPU clocked at 7.09MHz and PAL frames at 15'625 lines per second. Since its video memory is shared
with the CPU, this means that you already use up a lot of your [CPU cycles for managing graphics memory](https://retrocomputing.stackexchange.com/a/2149). So the Amiga engineers used a neat trick to cheat these limits.
It's called [Hold-and-Modify](https://en.wikipedia.org/wiki/Hold-And-Modify).

Today we're going to dig up some history and reimplement Hold-and-Modify (OCS HAM6 mode) in Rust.

_Why Rust?_

Rust is a language suited for system-programming. It's very close to the limiting hardware but at the same
time provides very powerful abstractions and safety guarantuees which help you to not fall into traps of
alternative languages (like C++). Its ecosystem is growing at an enormous pace with the support of C++ heavy-weights
like Microsoft.

The aim of today is not to grasp all concepts of Rust but simply to have a glimpse. But please read at least
the [Learn X in Y minutes guide](https://learnxinyminutes.com/docs/rust/) and try to roughly understand [ownership and borrowing concepts](https://doc.rust-lang.org/1.30.0/book/first-edition/ownership.html).

## Morning Track – HAM6 in Rust

If you didn't already, please install [rustup](https://www.rust-lang.org/tools/install).
Then checkout this repo and run `cargo test`.

The task of this morning is to implement a HAM6 encoder, so that `cargo run` (running `src/bin/main.rs`)

1. reads an image from `data_in` into an `RgbImage<u8>`
1. encodes it as a `HamImage<Ham6Pixel>`
1. decodes it back to an `RgbImage<u8>`
1. writes it to `data_out` as a Bitmap or PNG.

While the scaffold does all this already, it now simply converts the 24bit RGB image into a HAM image by
always using the closest color of the 16 colors base palette. This means that the last two bits of
`Ham6Pixel` are always zero. So you currently end up with a 16 color image in the `data_out` folder
instead of the promised 4096 colors.

Enhance the current solution to make full use of the last two bits by using your own HAM encoding strategy!

Some reading resources:

* http://theamigamuseum.com/the-hardware/the-amigas-graphic-modes/
* https://en.wikipedia.org/wiki/Hold-And-Modify
* https://wiki.amigaos.net/wiki/Classic_Graphics_Primitives#Advanced_Topics

Before lunch we'll meet and present our resulting images.

## Afternoon Tracks – Find and Discover

The afternoon is like a longer find-and-discover block of the Renuo Learning Week. You can follow down
the rabbit holes of three core areas being Rust, Retro Computing or Computer Graphics as long
as you start in the context of this morning's exercise.

Please prepare a condensed lightning talk which maps out the path you took for the next company meeting.

### Rust

Learn more about Rust by improving this scaffold. For example you could

* extend the [`image` crate](https://docs.rs/image/0.23.4/image/) with a HAM image (e.g. using the crate's `Pixel` trait).
* extend the [`image` crate](https://docs.rs/image/0.23.4/image/) with the [PPM image format](http://netpbm.sourceforge.net/doc/ppm.html#plainppm)
* create a binary image format to persist our HAM image
* adjust the current solution so that it uses planar pixels (what the Amiga did) instead of packed pixels (what we have today)
* reasearch the IFF image format
* improve performance of the current code (e.g. with a [flamegraph](https://github.com/flamegraph-rs/flamegraph)
  or [criterion](https://docs.rs/criterion/0.3.2/criterion/) directly)
* process the image lines multithreaded

### Retro Computing

Learn more about long forgotten restrictions and how they creatively have been overcome. For example you could

* research about color clocks and find out why the HAM mode was such a neat trick to overcome
  speed issues:
  [Amiga CPU speed](https://retrocomputing.stackexchange.com/a/2149),
  [Amiga DMA](http://amigadev.elowar.com/read/ADCD_2.1/Hardware_Manual_guide/node012A.html)
* research about other [advanced graphics features](https://wiki.amigaos.net/wiki/Classic_Graphics_Primitives#Advanced_Topics)
  of the Amiga, like Halfbrite or some creative Copper hacks, like the [DigiView Dynamic mode](https://amigalove.com/viewtopic.php?f=7&t=620).
* research and try CRT simulations. We forgot how different games looked on CRTs (e.g. [this one](https://www.gamasutra.com/blogs/KylePittman/20150420/241442/CRT_Simulation_in_Super_Win_the_Game.php) or
  [this one](https://web.archive.org/web/20180927020443/http://www.piratehearts.com/blog/2014/03/28/crt-simulation/))

### Computer Graphics

Learn about digital color representation. For example you could

* research CRT and color broadcasting formats like NTSC or PAL.
  Starting points:
  [CRT monitor](https://en.wikipedia.org/wiki/Cathode-ray_tube),
  [Interlacing](https://en.wikipedia.org/wiki/Interlaced_video),
  [Multisync](https://en.wikipedia.org/wiki/Multisync_monitor)
* research the history and details of
  [color palettes](https://en.wikipedia.org/wiki/List_of_16-bit_computer_color_palettes),
  [color spaces](https://de.wikipedia.org/wiki/Farbraum) and perception optimizations like
  [chroma subsampling](https://github.com/leandromoreira/digital_video_introduction#chroma-subsampling).
  Also interesting are the [wide-gamut efforts for CSS](https://webkit.org/blog/6682/improving-color-on-the-web).
* research HAM Palette optimization and encoding. There are techniques like dithering and
  [many more](http://mrsebe.bplaced.net/blog/wordpress/?p=1339), but why not apply some ML for example?
* Research [color distances](https://en.wikipedia.org/wiki/Color_difference) in color spaces
