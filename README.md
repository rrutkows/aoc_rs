https://adventofcode.com/

Assumming `YYYY` is the year and `DD` is the day number,
either paste your input into `yearYYYY/src/dDD.txt`
or paste the value of AOC cookie `session` into `session` file in the project's root folder.

If the file `dDD.txt` does not exist, the app will try to download and save it
following the [guidelines].

Run:
```
cargo run --release -p yearYYYY --bin dDD
```

Benchmarks are available:
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

[guidelines]: https://www.reddit.com/r/adventofcode/wiki/faqs/automation/