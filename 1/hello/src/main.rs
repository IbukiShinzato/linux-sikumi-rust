extern crate system_pause;

use system_pause::pause;

fn main() {
    println!("Pausing...");
    pause!(); 
    println!("Continuing...");
    
}
