use nix::sys::signal;
use nix::sys::signal::{SigHandler, Signal};

pub fn run() {
    unsafe { signal::signal(Signal::SIGINT, SigHandler::SigIgn) }.unwrap();
    loop {}
}
