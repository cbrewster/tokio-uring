use tokio_uring::fs::File;

use std::io::Write;
use std::{env, io};

fn main() {
    // The file to `cat` is passed as a CLI argument
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        panic!("no path specified");
    }

    let path = &args[1];

    tokio_uring::start(async {
        // Open the file without blocking
        let file = File::open(path).await.unwrap();
        dbg!(file.metadata().await.unwrap());

        // Include a new line
        println!("");
    });
}
