extern crate nix;

use nix::unistd::pause;

pub fn run() {
    pause();
}
