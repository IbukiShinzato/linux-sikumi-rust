extern crate libc;

use nix::unistd::execve;
use nix::unistd::getpid;
use nix::unistd::getppid;
use std::ffi::CString;
use std::process::exit;

pub fn run() {
    unsafe {
        match libc::fork() {
            -1 => {
                eprintln!("fork failed!");
                exit(1);
            }
            // 子プロセス
            0 => {
                println!(
                    "子プロセス: pid={}, 親プロセスのpid={}",
                    getpid(),
                    getppid()
                );
                let path = CString::new("/bin/echo").unwrap();
                let args = [
                    CString::new("echo").unwrap(),
                    CString::new(format!("pid={} からこんにちは", getpid())).unwrap(),
                ];
                let env: [CString; 0] = [];

                let _ = execve(&path, &args, &env);
                exit(0)
            }
            // 親プロセス
            pid => loop {
                println!("親プロセス: pid={}, 子プロセスのpid={}", getpid(), pid);
                exit(0);
            },
        }
    }
}
