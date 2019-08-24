extern crate nix;

#[cfg(test)]
extern crate assert_cmd;
#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(test)]
extern crate tempfile;

use std::env;
use std::fs::File;
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

use nix::fcntl::{splice, SpliceFFlags};
use nix::unistd::pipe;
use std::thread;

const BUF_SIZE: usize = 16384;

fn splice_all(fd_in: RawFd, fd_out: RawFd) {
    loop {
        let res = splice(fd_in, None, fd_out, None, BUF_SIZE, SpliceFFlags::empty()).unwrap();

        if res == 0 {
            // We read 0 bytes from the input,
            // which means we're done copying.
            break;
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let (pipe_rd, pipe_wr) = pipe().unwrap();
    // We make `File` from the pipe to make sure they are closed, even in case of a panic.
    // We could also compile with panic=abort instead.
    let pipe_rd = unsafe { File::from_raw_fd(pipe_rd) };
    let pipe_wr = unsafe { File::from_raw_fd(pipe_wr) };

    let output_thread = thread::spawn(move || {
        let stdout = io::stdout();
        let _handle = stdout.lock();

        splice_all(pipe_rd.as_raw_fd(), stdout.as_raw_fd());
    });

    if args.is_empty() {
        let stdin = io::stdin();
        let _handle = stdin.lock();
        splice_all(stdin.as_raw_fd(), pipe_wr.as_raw_fd());
    } else {
        for path in args.into_iter() {
            if path == "-" {
                let stdin = io::stdin();
                let _handle = stdin.lock();
                splice_all(stdin.as_raw_fd(), pipe_wr.as_raw_fd());
            } else {
                splice_all(
                    File::open(&path)
                        .expect(&format!("fcat: {}: No such file or directory", path))
                        .as_raw_fd(),
                    pipe_wr.as_raw_fd(),
                )
            };
        }
    }

    // We explicitly drop `pipe_wr` so that its pipe end is closed. Otherwise, the splice call
    // in `output_thread` would wait forever and we would have a deadlock.
    drop(pipe_wr);

    let _ = output_thread.join();
}

#[cfg(test)]
mod integration {
    use assert_cmd::prelude::*;
    use std::{io::Write, process::Command};
    use tempfile::NamedTempFile;

    fn write_content_to_tempfile(content: &[u8]) -> String {
        let mut file = NamedTempFile::new().expect("Cannot create temporary file");
        file.write_all(content)
            .expect("Cannot write to temporary file");
        file.path()
            .to_str()
            .expect("Cannot get path from temporary file")
            .to_owned()
    }

    fn concat_vecs<T>(vec0: Vec<T>, vec1: Vec<T>) -> Vec<T> {
        let mut out = Vec::with_capacity(vec0.len() + vec1.len());
        out.extend(vec0.into_iter());
        out.extend(vec1.into_iter());
        out
    }

    proptest! {
        #[test]
        fn cat_single_file(content: Vec<u8>) {
            let path = write_content_to_tempfile(&content);
            let cmd = Command::main_binary().unwrap().assert().set_cmd(path);
            let out = cmd.get_output();
            out.stdout == content
        }

        #[test]
        fn cat_multiple_files(content0: Vec<u8>, content1: Vec<u8>) {
            let path0 = write_content_to_tempfile(&content0);
            let path1 = write_content_to_tempfile(&content1);

            let cmd = Command::main_binary()
                .unwrap()
                .assert()
                .set_cmd(format!("{} {}", path0, path1));
            let out = cmd.get_output();

            let expected = concat_vecs(content0, content1);
            out.stdout == expected
        }

        #[test]
        fn cat_stdin(content: Vec<u8>) {
            let cmd = Command::main_binary()
                .unwrap()
                .assert()
                .set_stdin(content.clone());
            let out = cmd.get_output();

            out.stdout == content
        }

        #[test]
        fn cat_stdin_dash(content: Vec<u8>) {
            let cmd = Command::main_binary()
                .unwrap()
                .assert()
                .set_stdin(content.clone())
                .set_cmd("-".to_owned());
            let out = cmd.get_output();

            out.stdout == content
        }

        #[test]
        fn cat_stdin_file(content0: Vec<u8>, content1: Vec<u8>) {
            let path = write_content_to_tempfile(&content1);

            let cmd = Command::main_binary()
                .unwrap()
                .assert()
                .set_stdin(content0.clone())
                .set_cmd(format!("- {}", path));
            let out = cmd.get_output();

            let expected = concat_vecs(content0, content1);
            out.stdout == expected
        }

        #[test]
        fn cat_file_stdin(content0: Vec<u8>, content1: Vec<u8>) {
            let path = write_content_to_tempfile(&content0);

            let cmd = Command::main_binary()
                .unwrap()
                .assert()
                .set_stdin(content0.clone())
                .set_cmd(format!("{} -", path));
            let out = cmd.get_output();

            let expected = concat_vecs(content0, content1);
            out.stdout == expected
        }
    }
}
