///
/// Hack Assembler in Rust
/// Created by Francois W. Nel on 2 Jul 2016.
///
/// Description:
///  The assembler converts Hack assembly instructions
///  into machine code for the Hack computer architecture.
///  Assembly instructions should be provided in a .asm source file,
///  and the machine code will be stored in a .hack file
///  with either the same filename, or using the provided
///  filename and extension.
///
/// Usage:
///   $ rhasm <inputfilename.asm> <(optional) outputfilename.hack>
///
/// Note:
///  This is a reimplementation of my Hack Assembler
///  originally implemented in C++. This is my first experience
///  with Rust, so the code is likely not meeting standards.
///

use std::env;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;

fn main() {
    let arguments: Vec<_> = env::args().collect();

    // Get the input and output file names, and provide usage instructions
    //  if too few or too many arguments are provided.
    if arguments.len() < 2 || arguments.len() > 3 || arguments[1].to_string().len() < 5 {
        println!("Usage: {} <inputfilename.asm> <(optional) outputfilename.hack>", arguments[0]);
        return;
    }

    let input_file_name = arguments[1].to_string();
    let mut output_file_name = input_file_name[0..input_file_name.len() - 4].to_string() + ".hack";

    if arguments.len() == 3 {
        output_file_name = arguments[2].to_string();
    }

    let mut output_file = match File::create(&output_file_name) {
        Err(why) => panic!("Failed to create output file: {}", why.description()),
        Ok(output_file) => output_file,
    };

    // TODO: Implement parser

    // TODO: Implement code converter

    // TODO: Output the assembled buffer.
    match output_file.write_all(input_file_name.as_bytes()) {
        Err(why) => panic!("Failed to write to {}: {}", &output_file_name, why.description()),
        Ok(_) => return,
    }
}
