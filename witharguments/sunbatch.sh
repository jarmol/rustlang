
echo 1. Helsinki 2019-07-23 DLST UTC + 3 h
#    1. Helsinki 2019-07-23 DLST UTC + 3 h
cargo run 13 27 0 2019 7 23 60.16 24.96 3.0
echo =====================================
echo 2. Oulu 2019-07-23 DLST UTC + 3 h
#    2. Oulu 2019-07-23 DLST UTC + 3 h
cargo run 13 27 0 2019 7 23 65.02 25.5  3.0
echo =====================================
echo 3. Stockholm 2019-07-23 DLST UTC + 2 h
#    3. Stockholm 2019-07-23 DLST UTC + 2 h
cargo run 12 54 12 2019 7 23 59.33 18.07 2.0
echo =====================================
echo 4. Reykjavik 2019-08-01 UTC - 1 h
cargo run 03 33 23 2019 8  1 64.135 -21.895 -1.0
cargo run 12 33 56 2019 8  1 64.135 -21.895 -1.0
