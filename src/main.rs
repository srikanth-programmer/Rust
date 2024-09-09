extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;

// use: access the file name 
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
// use : time taken to compress
use std::time::Instant;

fn main(){
    if args().len() != 3{
        eprintln!("Usage: `source` `target`");
        return ;
    }
    // getting the file name
    let input_file_name = args().nth(1).unwrap();
    // Here File::open expects AsRef<Path> trait so we convert String -> &str
    let mut input = BufReader::new(File::open(&input_file_name).unwrap());
    let output_file_name = args().nth(2).unwrap();
    let output = File::create(output_file_name).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input,&mut encoder).unwrap();

    let output = encoder.finish().unwrap();
    println!(
        "Source len : {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len:{:?}",output.metadata().unwrap().len());
    println!("Elapsed:{:?}",start.elapsed());
}

