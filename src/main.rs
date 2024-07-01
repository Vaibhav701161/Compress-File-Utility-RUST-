extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        println!("usage: `source` `target`");
        return;
    }
    let source = args().nth(1).unwrap();
    let target = args().nth(2).unwrap();

    let mut  input = BufReader::new(File::open(&source).unwrap());
    let output = File::create(&target).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!("source len: {:?}", File::open(&source).unwrap().metadata().unwrap().len());
    println!("target len: {:?}", output.metadata().unwrap().len());
    println!("compress time: {:?}", start.elapsed());
}
