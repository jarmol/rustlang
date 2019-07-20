# Sunrise-Sunset Calculator with Rust Programming Language  

## Description

 This is a new version using variable arguments of the local time (hours, minutes, seconds) and of the date (year, month, day)
 which can be entered after the line command. The time zone is here fixed to 2 hours.
 The location variables (latitude, longitude, timezone) are still fixed values. 
 They are arguments to be added in a later version. 
 
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
jarmo$ cargo run 6 22 03 2019 6 7
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/suncalcargs 6 22 03 2019 6 7`
Program:    target/debug/suncalcargs
Read 6 arguments
Arguments: hours, min, sec
Argument 1: 6 hours
Argument 2: 22 min
Argument 3: 03 sec

Arguments: year, month, day
Argument 4: 2019 year
Argument 5: 6 month
Argument 6: 7 day
Location: latitude 65.85 °, longitude 24.18 °, time zone 2 h
Local time now: Sat, 20 Jul 2019 19:25:25 +0300
Universal time now: 2019-07-20 16:25:25.715904 UTC
Calculation local time 2019-06-07 06:22:03
Calculation   UTC time 2019-06-07 04:22:03
JDN = 2458642
UTC time: 4h 22min
JD = 2458641.6819
Declination                    =    22.717 °
True solar time                =   359.997 min
Hour angle                     =   -90.001 °
Solar zenith                   =    69.367 °
Solar azimuth                  =    80.280 °
Sun altitude                   =    20.633 °
Atmospheric refraction         =     0.043 °
Refraction corrected elevation =    20.675 °
Day length                     =    22:11:31
Sunrise time                   =    01:16:17
Noon time                      =    12:22:03
Sunset time                    =    23:27:48
bash-3.2$ exit
```
## Case the first day of month near to midnight
The local time and UTC may have different date near to midnight.
On the first day of each month especially, also the months are different for both in the midnight.
That makes the conversion of the local time and date to UTC as required for the Julian date number more
complicated. This case is solved now in this version.
```
jarmo$ cargo run 0 12 0 2019 12 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/suncalcargs 0 12 0 2019 12 1`
Program:    target/debug/suncalcargs
Read 6 arguments
Arguments: hours, min, sec
Argument 1: 0 hours
Argument 2: 12 min
Argument 3: 0 sec

Arguments: year, month, day
Argument 4: 2019 year
Argument 5: 12 month
Argument 6: 1 day
Location: latitude 65.85 °, longitude 24.18 °, time zone 2 h
Local time now: Sat, 20 Jul 2019 19:50:24 +0300
Universal time now: 2019-07-20 16:50:24.231673 UTC
Calculation local time 2019-12-01 00:12:00
Calculation   UTC time 2019-11-30 22:12:00
JDN = 2458818
UTC time: 22h 12min
JD = 2458818.4250
Declination                    =   -21.704 °
True solar time                =     0.002 min
Hour angle                     =   -180.000 °
Solar zenith                   =   135.854 °
Solar azimuth                  =     0.001 °
Sun altitude                   =   -45.854 °
Atmospheric refraction         =     0.006 °
Refraction corrected elevation =   -45.848 °
Day length                     =    04:14:38
Sunrise time                   =    10:04:40
Noon time                      =    12:11:59
Sunset time                    =    14:19:19
jarmo$
```
jarmo$ 
