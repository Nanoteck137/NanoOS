use std::process::Command;

use std::io::prelude::*;
use std::fs::File;

extern crate pe_parser;

fn main() {
    // TODO(patrik): Build the bootloader
    // TODO(patrik): Create a PE Parser
    // TODO(patrik): Flatten the PE

    let mut f =
        File::open("bootloader/target/i586-pc-windows-msvc/debug/bootloader.exe")
            .expect("Could not open bootloader pe");

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)
        .expect("Could not read the bootloader exe");

    let pe = pe_parser::PeParser::parse(&bytes).expect("Failed to parse pe");
    println!("PE: {:?}", pe);

    pe.sections(|base, size, raw_data| {
        println!("Section");
        println!("Base: 0x{:x}", base);
        println!("Size: {}", size);
        println!("raw_data: {:?}", raw_data);

        Some(())
    });

    std::fs::create_dir_all("build")
        .expect("Failed to create the build directory");

    println!("Assembling 'start.asm'");
    let output = Command::new("nasm")
                    .args(&[
                        "-f", "bin",
                        "bootloader/src/start.asm",
                        "-o", "build/start.bin"])
                    .output()
                    .expect("Failed to assemble 'start.asm'");
                    
    println!("{:?}", output.stderr);
}
