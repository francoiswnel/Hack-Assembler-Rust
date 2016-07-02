use std::io;
use std::io::prelude::*;
use std::env;
use std::fs::File;

fn create_file (file_name: &str) -> Result<(), io::Error> {
    let mut output_file = try!(File::create(file_name));
    try!(output_file.write_all(b"Test"));
    Ok(())
}

fn main() {
    let argv: Vec<_> = env::args().collect();

    if argv.len() < 2 || argv.len() > 3 || argv[1].to_string().len() < 5 {
        println!("Usage: {} <inputfilename.asm> <(optional) outputfilename.hack>", argv[0]);
        return;
    }
    
    let input_file_name = argv[1].to_string();
    let mut output_file_name = input_file_name[0..input_file_name.len() - 4].to_string() + ".hack";

    if argv.len() == 3 {
        output_file_name = argv[2].to_string();
    }

    create_file(&output_file_name);
}
