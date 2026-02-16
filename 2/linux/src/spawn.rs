extern crate nix;

use nix::spawn::PosixSpawnAttr;
use nix::spawn::PosixSpawnFileActions;
use nix::spawn::posix_spawn;
use std::ffi::CString;

pub fn run() {
    let path = CString::new("/bin/echo").unwrap();
    let file_actions = PosixSpawnFileActions::init().unwrap();
    let attr = PosixSpawnAttr::init().unwrap();
    let args: Vec<CString> = ["echo", "echo", "posix_spawn()によって生成されました"]
        .into_iter()
        .map(|v| CString::new(v).unwrap())
        .collect();
    let envp: [CString; 0] = [];

    match posix_spawn(path.as_c_str(), &file_actions, &attr, &args, &envp) {
        Ok(pid) => {
            println!("子プロセスを生成しました (PID: {})", pid);
            println!("echoコマンドを生成しました")
        }
        Err(e) => eprintln!("spawn 失敗: {}", e),
    }
}
