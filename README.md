# 😼 fcat

![fastcat logo](/fastcat.svg)

`fcat`, short for *fastcat*, is a `cat` implementation in Rust using Linux's `splice` syscall.  
With that little trick, it's **more than three times as fast as the system `cat`** in my tests.  
Read the [announcement here](https://matthias-endler.de/2018/fastcat).

```
cat myfile | pv -r > /dev/null
[1.90GiB/s]
```

```
fcat myfile | pv -r > /dev/null
[5.90GiB/s]
```


## Installation

Note: Only works on Linux.  
(But you can send me a pull request for other operating systems.)

```
cargo install fcat
```

## Usage

```
fcat file1 file2 file3
```

## Project goals

* Be the fastest cat in town.
* Be a drop-in replacement for (POSIX) cat.

## Non-goals

* Provide any additional functionality other than what `cat` provides.  
  If you're looking for a more beautiful cat, check out [bat](https://github.com/sharkdp/bat).

## Trivia

* You probably won't ever need this, but it's a fun little experiment.  
  Still, I wonder why this is not part of e.g. GNU cat...
* What I like the most about the project is the logo.

## License

fcat is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
