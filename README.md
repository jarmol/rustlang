# Sunrise-Sunset Calculator with Rust Programming Language  

## Description

This version calculates:
   - the noon time,
   - the daylength, 
   - the Sun rise and Sun set times 
   - the maximum Sun elevation at the noon time as corrected with the refraction 
     for the given latitude, longitude and time zone  on a given date and local time.
     DLST option is not used here 

If Rust is installed, the program can be run in the shell window with the line command: cargo run

## Example
```
$ cargo run
Program: target/debug/suncalc
Location: latitude 65.85 °, longitude 24.18 °, time zone 2 h
Time now: Mon, 15 Jul 2019 18:59:25 +0300
UTC time now is 2019-07-15 15:59:25.827113 UTC
Calculation date and time is 2019-06-07 12:22:06.
JDN = 2458642
UTC time: 10:22
JD = 2458641.9319
Declination                    =  22.742 °
True solar time                = 720.001 min
Hour angle                     =  0.000 °
Solar zenith                   = 43.108 °
Sun altitude                   = 46.892 °
Atmospheric refraction         =  0.015 °
Refraction corrected elevation = 46.907 °
Day length             = 22:13:44
Sunrise time           = 01:15:13 
Noon time              = 12:22:05
Sunset time            = 23:28:58
$
```

