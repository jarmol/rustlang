# Sunrise-Sunset Calculator with Rust Programming Language  

## Description

This is a new version on the other branch using variable time arguments (hours, minutes, seconds) of the local time which can be entered on the terminal. The time zone is here fixed to 2 hours. As a consequence, a local time below 02:00 cannot be used as conversion to UTC times fails because it cannot be negative value.  

This version contains the calculation of the noon time,
daylength, Sun rise and Sun set times and the refraction 
corrected Sun elevation for the given latitude and
longitude, on a given date and local time.

The program can be run in the shell window with the Rust 
line command cargo if Rust is installed.

## Example
```
Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/suncalcargs 02 00`
Program:    target/debug/suncalcargs
Read 2 arguments
Argument 1: 02 hr
Argument 2: 00 mn
Location: latitude 65.85 °, longitude 24.18 °, time zone 2 h
Time now: Sun, 30 Jun 2019 13:58:16 +0300
Calculation date and time is 2019-06-07 02:00:00.
JDN = 2458642
UTC time: 0h 0min
JD = 2458641.5000
Declination                    =    22.701 °
True solar time                =    97.978 min
Hour angle                     =  -155.505 °
Solar zenith                   =    89.503 °
Sun altitude                   =     0.497 °
Atmospheric refraction         =     0.417 °
Refraction corrected elevation =     0.914 °
Day length             = 22:10:02
Sunrise time           = 01:17:00 
Noon time              = 12:22:01
Sunset time            = 23:27:02
```

