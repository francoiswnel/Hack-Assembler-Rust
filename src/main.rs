use std::env;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;

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

    let mut output_file = match File::create(&output_file_name) {
        Err(why) => panic!("Failed to create output file: {}", why.description()),
        Ok(output_file) => output_file,
    };

    match output_file.write_all(input_file_name.as_bytes()) {
        Err(why) => panic!("Failed to write to {}: {}", &output_file_name, why.description()),
        Ok(_) => return,
    }
}
