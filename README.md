# fastcat

![fastcat logo](/fastcat.svg)

This is a `cat` implementation in Rust using Linux' `splice` syscall.  
With that little trick, it's **twice as fast as the system `cat`** in my tests.  
Read the [announcement here](https://matthias-endler.de/2018/fastcat).

## Installation

Note: Only works on Linux.  
(But you can send me a pull request for other Operating Systems.)

```
cargo install fcat
```

## Usage

```
fcat file1 file2 file3
```


## Trivia

* You probably won't ever need that, but it's a fun little experiment.  
  Still, I wonder why this is not part of e.g. GNU cat...
* What I like the most about the project is the logo.

## License

fcat is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.