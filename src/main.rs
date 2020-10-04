use std::process::Command;
fn main() {
    // TODO(patrik): Build the bootloader
      // TODO(patrik): Create a PE Parser
      // TODO(patrik): Flatten the PE
    // TODO(patrik): Assemble start.asm

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
    println!("{:?}", output.stdout);
}
