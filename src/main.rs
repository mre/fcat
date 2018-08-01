extern crate nix;

use std::env;
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;

use nix::fcntl::{splice, SpliceFFlags};
use nix::unistd::pipe;

const BUF_SIZE: usize = 16384;

#[inline]
fn cat<T: AsRawFd>(input: &T) {
    let (rd, wr) = pipe().unwrap();
    let stdout = io::stdout();
    let _handle = stdout.lock();

    loop {
        let res = splice(
            input.as_raw_fd(),
            None,
            wr,
            None,
            BUF_SIZE,
            SpliceFFlags::empty(),
        ).unwrap();

        if res == 0 {
            // We read 0 bytes from the input,
            // which means we're done copying.
            break;
        }

        let _res = splice(
            rd,
            None,
            stdout.as_raw_fd(),
            None,
            BUF_SIZE,
            SpliceFFlags::empty(),
        ).unwrap();
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        let stdin = io::stdin();
        let _handle = stdin.lock();
        cat(&stdin);
    } else {
        for path in env::args().skip(1) {
            if path == "-" {
                let stdin = io::stdin();
                let _handle = stdin.lock();
                cat(&stdin);
            } else {
                cat(&File::open(&path)
                    .expect(&format!("fcat: {}: No such file or directory", path)))
            };
        }
    }
}
