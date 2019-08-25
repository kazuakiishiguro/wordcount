[![Build Status](https://travis-ci.com/kazuakiishiguro/wordcount.svg?token=EyEadznrtEZg89CPHxSF&branch=master)](https://travis-ci.com/kazuakiishiguro/wordcount)

# wordcount : rust
wordcount provides a simple count function for the appearance frequency of characters, words and lines.

See function documentation for details after following command :

``` bash
$ cargo doc --no-deps --open
```

## usage

here are the simple steps you can start using this tool.

```bash
$ git clone https://github.com/kazuakiishiguro/wordcount.git
$ cd wordcount
$ cargo build --release // OR simply run cargo run command

$ ./target/release/wordcount text.txt
{"aa": 1, "bb": 1, "cc": 1, "dd": 1}
```