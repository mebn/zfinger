# zfinger
A command line tool for KTH zfinger

## Installation
To install locally, run:
```
cargo install --path .
```

## How to use
```
Usage: zfinger [options] query
List all students at KTH and see a picture of some of them.
The query should not have any quotes.
Flags can be combined, e.g. zfinger -fch firstname lastname

Options:
    -f : Open the first result.
    -c : Close prompt, disable interactive loop.
    -h : Hide results and close prompt. Equivalent to -ch.
    -a : Include all students, even those with no year set.
```