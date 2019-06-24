# rustlang

## Rust programming language stuff

This project is under construction. 
This version contains the calculation of the noon time,
daylength, Sun rise and Sun set times for the given 
latitude and longitude, on a given date.

The program can be run in the shell window with the Rust 
line command cargo if Rust is installed.

## Example
```
$ cargo run
Program: target/debug/suncalc
Location: latitude 65.85 °, longitude 24.18 °, time zone 2 h
Time now: Mon, 24 Jun 2019 22:08:30 +0300
Calculation date and time is 2019-06-07 12:22:06.
JDN = 2458642
UTC time: 10h 22min
JD = 2458641.9319
Declination            = 22.742 °
True solar time        = 720.001 min
Hour angle             = 0.000 °
Solar zenith           = 43.108 °
Sun altitude           = 46.892 °
Day length             = 22:13:44
Sunrise time           = 01:15:13 
Noon time              = 12:22:05
Sunset time            = 23:28:58
$
```

