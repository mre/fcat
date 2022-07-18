# 😼 fcat

![Github Actions](https://action-badges.now.sh/mre/fcat?action=test)

![fastcat logo](/fastcat.svg)

`fcat`, short for *fastcat*, is a `cat` implementation in Rust using Linux's `splice` syscall.  
With that little trick, it's **more than three times as fast as the system `cat`** in our benchmarks.  
Read the [announcement here](https://endler.dev/2018/fastcat).

:warning: **This project is currently broken on newer Linux versions (5.9+) because of some changes  
concerning the `splice` system call. (See [here](http://archive.lwn.net:8080/linux-kernel/202105071116.638258236E@keescook/t/) and [here](https://cdn.kernel.org/pub/linux/kernel/v5.x/ChangeLog-5.9).) This can't be fixed unless changes to the kernel get made.**

## Performance

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
  If you're looking for a more *beautiful* cat, check out [bat](https://github.com/sharkdp/bat).

## Known issues

If you run `fcat /dev/zero >> myfile`, it will fail with exit code `EINVAL` because, according to the [splice manpage](http://man7.org/linux/man-pages/man2/splice.2.html): "The target file is opened in append mode."

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
