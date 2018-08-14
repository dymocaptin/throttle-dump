extern crate serial;

use std::env;

fn main() {
	use std::path::Path;
    
    for arg in env::args_os().skip(1) {
        println!("opening port: {:?}", arg);
        let mut port = serial::unix::TTYPort::open(Path::new(&arg)).unwrap();
    }
}