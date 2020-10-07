use std::process::Command;

use std::convert::TryInto;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

extern crate pe_parser;
extern crate dunce;

fn flatten_pe_to_image<P: AsRef<Path>>(filename: P) 
        -> Option<(u32, u32, Vec<u8>)> {
    // Open the pe file
    let mut f = File::open(filename)
        .expect("Could not open bootloader pe");

    // Initialize a vec to store the file bytes
    let mut bytes = Vec::new();
    // Read the file and put it in the vec
    f.read_to_end(&mut bytes)
        .expect("Could not read the bootloader exe");

    // Parse the pe file
    let pe = pe_parser::PeParser::parse(&bytes)
        .expect("Failed to parse pe");

    // Image start and end used to calculate the image size and calculate 
    // the offset in the new image
    let mut image_start = None;
    let mut image_end = None;

    // Search through the sections and calculate the bounds of the image 
    // i.e the image_start and image_end
    pe.sections(|base, size, _| {
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

    // Remove the option part    
    let image_start = image_start?;
    let image_end = image_end?;
    
    // Calculate the image size
    let image_size: usize = 
        image_end.checked_sub(image_start)?.checked_add(1)?
        .try_into().ok()?;

    // Initialize a vec to hold the final flatten image
    let mut flatten_image = vec![0u8; image_size];

    // Go through the sections again to put them in the storage
    pe.sections(|base, size, raw_data| {
        // Calculate the offset in the flatten_image vec
        let offset: usize = 
            (base.checked_sub(image_start)?).try_into().ok()?;
        // Get the size
        let size: usize = size.try_into().ok()?;

        // Put the sections raw data in the flatten_image vec
        flatten_image[offset..offset.checked_add(size)?]
            .copy_from_slice(raw_data);

        Some(())
    })?;

    // Return the entry point, the image start and the final flatten image
    Some((pe.entry_point.try_into().ok()?, 
        image_start.try_into().ok()?, 
        flatten_image))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create the build directories we need
    std::fs::create_dir_all("build")?;
    std::fs::create_dir_all("build/bootloader")?;
    std::fs::create_dir_all("build/kernel")?;

    // Construct the path to the bootloader directory where the 
    // bootloader project is located
    let bootloader_path = Path::new("bootloader").canonicalize()?;

    // On Windows the path is a UNC and we need a normal path,
    // so we use 'dunce' to convert the
    // UNC path to the normal path
    let bootloader_path = dunce::canonicalize(bootloader_path)?;
    println!("Bootloader Path: {:?}", bootloader_path);

    // Construct the path to the bootloader build directory
    let bootloader_target_path =                                         
        Path::new("build").join("bootloader").canonicalize()?;
    // Convert the path
    let bootloader_target_path = dunce::canonicalize(bootloader_target_path)?;
    
    // Build the bootloader rust project
    println!("Building the bootloader");
    Command::new("cargo")
        .current_dir(bootloader_path)
        .args(&[
              "build", 
              "--target-dir", 
              bootloader_target_path.to_str().unwrap()])
        .status()?.success();

    // Construct the path to the bootloader executable
    let bootloader_exe = 
        Path::new(&bootloader_target_path)
        .join("i586-pc-windows-msvc")
        .join("debug")
        .join("bootloader.exe")
        .canonicalize()?;
    // Convert the path if on we are on Windows
    let bootloader_exe = dunce::canonicalize(bootloader_exe)?;

    // Flatten the bootloader pe
    let (entry_point, _start, bytes) = 
        flatten_pe_to_image(bootloader_exe).unwrap();

    // Write out the bootloader code to a file so the stage0 can include it
    std::fs::write("build/bootloader_code.bin", &bytes)?;

    // Construct the path to the stage0 assembly file
    let bootloader_stage0_asm = 
        Path::new("bootloader")
            .join("src")
            .join("arch")
            .join("x86_64")
            .join("stage0.asm")
            .canonicalize()?;
    // Convert the path if on we are on Windows
    let bootloader_stage0_asm = 
        dunce::canonicalize(bootloader_stage0_asm)?;

    // Run nasm and assemble the start.asm assembly file
    println!("Assembling 'start.asm'");
    Command::new("nasm")
        .args(&[
            "-f", "bin",
            &format!("-Dentry_point={:#x}", entry_point),
            bootloader_stage0_asm.to_str().unwrap(),
            "-o", "build/start.bin"])
        .status()?.success();
                    
    Ok(())
}
