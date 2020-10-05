use std::process::Command;

use std::convert::TryInto;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

extern crate pe_parser;

fn flatten_pe_to_image<F: AsRef<Path>>(filename: F) -> Option<(u32, u32, Vec<u8>)> {
    let mut f = File::open(filename)
            .expect("Could not open bootloader pe");

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)
        .expect("Could not read the bootloader exe");

    let pe = pe_parser::PeParser::parse(&bytes).expect("Failed to parse pe");
    println!("{:?}", pe);

    let mut image_start = None;
    let mut image_end = None;

    pe.sections(|base, size, raw_data| {
        // Convert the size to a u64
        let size = size as u64;

        // Calculate the end
        let end = base.checked_add(size.checked_sub(1)?)?;

        // If we don't have set the start or the end then initialize 
        // those to the first section we encounter
        if image_start.is_none() {
            image_start = Some(base);
            image_end = Some(end);
        }

        // Find the lowest base address we encounter
        image_start = image_start.map(|x| core::cmp::min(x, base));
        // Find the highest end address we encounter
        image_end = image_end.map(|x| core::cmp::max(x, end));

        Some(())
    });

    
    let image_start = image_start?;
    let image_end = image_end?;
    
    let image_size: usize = 
        image_end.checked_sub(image_start)?.checked_add(1)?
        .try_into().ok()?;

    let mut flatten_image = vec![0u8; image_size];

    pe.sections(|base, size, raw_data| {
        let offset: usize = 
            (base.checked_sub(image_start)?).try_into().ok()?;
        let size: usize = size.try_into().ok()?;

        flatten_image[offset..offset.checked_add(size)?]
            .copy_from_slice(raw_data);

        Some(())
    })?;

    println!("Image Start: 0x{:x}", image_start);
    println!("Image End: 0x{:x}", image_end);
    println!("Image Size: {}", image_size);
    println!("Image: {:?}", flatten_image);

    Some((pe.entry_point.try_into().ok()?, 
        image_start.try_into().ok()?, 
        flatten_image))
}

fn main() {
    // TODO(patrik): Build the bootloader
    // TODO(patrik): Create a PE Parser
    // TODO(patrik): Flatten the PE

    let image = flatten_pe_to_image(
        "bootloader/target/i586-pc-windows-msvc/debug/bootloader.exe");
        println!("Image: {:x?}", image);
/*
    let mut f = File::open(
        "bootloader/target/i586-pc-windows-msvc/debug/bootloader.exe")
            .expect("Could not open bootloader pe");

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)
        .expect("Could not read the bootloader exe");

    let pe = pe_parser::PeParser::parse(&bytes).expect("Failed to parse pe");
    // println!("PE: {:?}", pe);

    let x = || -> Option<()> {
        
    };

    x().unwrap();
*/
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
