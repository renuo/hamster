# Rust and Pixels

This is the scaffold for the Renuo learning day 2020.

Let's go back to 1985 when colors did cost a lot of money. Commodore announced its Amiga 1000 system which was
capable of showing 4096 colors on the screen simultaneously. This really blew the minds of people who were 
used to monochrome screens (PAL was just introduced some years ago) or to not more than 16 colors.

Here you see the Commodore C64 with 16 colors, IBM EGA 16 colors and the Amiga HAM6 mode in order:

![](https://upload.wikimedia.org/wikipedia/commons/9/96/Commodore64_palette_sample_image.png)
![](https://upload.wikimedia.org/wikipedia/commons/7/7b/Screen_color_test_EGA_16colors.png)
![](https://upload.wikimedia.org/wikipedia/commons/b/be/Screen_color_test_Amiga_4096colors_HAM.png)

Because the Amiga had a CPU speed of about 7MHz (IBM often had some more), they needed to do a trick which is called
[Hold-and-Modify](https://en.wikipedia.org/wiki/Hold-And-Modify).

Today we're going to implement HAM6 in Rust.

## Timetable

| Time  | Activity |
| ----: | -------- |
|  8:00 | Kickoff  |
| 11:40 | Merging  |
| 13:00 | Forking  |
| 17:00 | Wrap-up  |
