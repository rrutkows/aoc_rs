https://adventofcode.com/

Assumming `DD` is the day number, paste your input into `yearYYYY/src/dDD.txt` and run:
```
cargo run --release -p yearYYYY --bin dDD
```

Starting from d14 benchmarks are available:
```
cargo bench -p yearYYYY -- dDD
```

I have no generic solution for Y2022 D22. Your cube must have the following layout
```
 ##
 #
##
#
```
where `#` is a 50x50 fragment
