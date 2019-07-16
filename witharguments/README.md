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
Program:    target/debug/suncalcargs
Read 6 arguments
Argument 1: 12 hr
Argument 2: 22 mn
Argument 3: 6 ss
Argument 4: 2019 year
Argument 5: 6 month
Argument 6: 7 day
Location: latitude 65.85 °, longitude 24.18 °, time zone 2 h
Local time now: Tue, 16 Jul 2019 22:46:51 +0300
Universal time now: 2019-07-16 19:46:51.876489 UTC
Calculation time is 2019-06-07 12:22:06
JDN = 2458642
UTC time: 10h 22min
JD = 2458641.9319
Declination                    =   22.742 °
True solar time                =   720.001 min
Hour angle                     = 0.000 °
Solar zenith                   =   43.108 °
Solar azimuth                  =   180.000 °
Sun altitude                   =    46.892 °
Atmospheric refraction         =    0.015 °
Refraction corrected elevation =    46.907 °
Day length             = 22:13:44
Sunrise time           = 01:15:13 
Noon time              = 12:22:05
Sunset time            = 23:28:58
$
```

