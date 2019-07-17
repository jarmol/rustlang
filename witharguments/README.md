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
## Case the first day of month near to midnight
The local time and UTC may have different date near to midnight.
On the first day of each month especially, also the months are different for both in the midnight.
That makes the conversion of the local time and date to UTC as required for the Julian date number more
complicated. This case is handled now in this version.
```
$
suncalcargs 0 12 0 2019 12 1`
Program:    target/debug/suncalcargs
Read 6 arguments
Argument 1: 0 hr
Argument 2: 12 mn
Argument 3: 0 ss
Argument 4: 2019 year
Argument 5: 12 month
Argument 6: 1 day
Location: latitude 65.85 °, longitude 24.18 °, time zone 2 h
Testi UTC 30.11. time 22:12
Local time now: Wed, 17 Jul 2019 20:01:51 +0300
Universal time now: 2019-07-17 17:01:51.010426 UTC
Calculation local time 2019-12-01 00:12:00
Calculation   UTC time 2019-11-30 22:12:00
JDN = 2458818
UTC time: 22h 12min
JD = 2458818.4250
Declination                    =   -21.704 °
True solar time                =   0.002 min
Hour angle                     = -180.000 °
Solar zenith                   =   135.854 °
Solar azimuth                  =   0.001 °
Sun altitude                   =    -45.854 °
Atmospheric refraction         =    0.006 °
Refraction corrected elevation =    -45.848 °
Day length             = 04:14:38
Sunrise time           = 10:04:40 
Noon time              = 12:11:59
Sunset time            = 14:19:19
```
$
  
