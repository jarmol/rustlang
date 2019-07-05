# Sunrise-Sunset Calculator with Rust Programming Language  

## Description

 This is a new version on the other branch using variable time arguments (hours, minutes, seconds)
 of the local time which can be entered on the terminal. The time zone is here fixed to 2 hours.
 As a consequence, a local time below 02:00 cannot be used as conversion to UTC times fails
 because it cannot be negative value.  

This version contains the calculation of the noon time,
daylength, Sun rise and Sun set times, the refraction 
corrected Sun elevation and the solar azimuth angle for the given latitude and
longitude, on a given date and local time.

The program can be run in the shell window with the Rust 
line command cargo if Rust is installed.

I'll add later the remaining variables: calculation day, month and year
 and the location arguments, latitude and longitude
which are still fixed values in the present version.

## Example
```
$ cargo run 23 20 0
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/suncalcargs 23 20 0`
Program:    target/debug/suncalcargs
Read 3 arguments
Argument 1: 23 hr
Argument 2: 20 mn
Location: latitude 65.85 °, longitude 24.18 °, time zone 2 h
Time now: Fri,  5 Jul 2019 09:44:18 +0300
Calculation date and time is 2019-06-07 23:20:00.
JDN = 2458642
UTC time: 21h 20min
JD = 2458642.3889
Declination                    =   22.785 °
True solar time                =   1377.816 min
Hour angle                     = 164.454 °
Solar zenith                   =   90.574 °
Solar azimuth                  =   345.693 °
Sun altitude                   =    -0.574 °
Atmospheric refraction         =    0.575 °
Refraction corrected elevation =    0.001 °
Day length             = 22:17:49
Sunrise time           = 01:13:16 
Noon time              = 12:22:11
Sunset time            = 23:31:05
$
```

