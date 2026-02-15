extern crate libc;

use nix::unistd::getpid;
use nix::unistd::getppid;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    unsafe {
        match libc::fork() {
            -1 => {
                eprintln!("fork failed!");
                exit(1);
            }
            0 => loop {
                println!(
                    "子プロセス: pid={}, 親プロセスのpid={}",
                    getpid(),
                    getppid()
                );
                sleep(Duration::from_secs(1));
            },
            pid => loop {
                println!("親プロセス: pid={}, 子プロセスのpid={}", getpid(), pid);
                sleep(Duration::from_secs(1));
            },
        }
    }
}
