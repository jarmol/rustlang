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
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/suncalc`
Program: target/debug/suncalc
Location: latitude 65.85 °, longitude 24.18 °
Time now: Sat, 22 Jun 2019 09:33:48 +0300
Calculation date and time is 2019-06-07 12:22:06.
JDN = 2458642
UTC time: 10h 22min
JD = 2458641.9319
Declination            = 22.742 °
Day length             = 22:13:44
Sunrise time           = 01:15:13 
Noon time              = 12:22:05
Sunset time            = 23:28:58
$
```

