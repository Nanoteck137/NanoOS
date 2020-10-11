use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO(patrik): Should we disable interupts?
    println!("---------- BOOTLOADER PANIC ----------");
    
    if let Some(msg) = info.message() {
        println!("Message: {}", msg);
    }

    if let Some(loc) = info.location() {
        println!("Location: {}:{}:{}", 
                 loc.file(), loc.line(), loc.column());
    }

    println!("--------------------------------------");

    // TODO(patrik): Replace with a halt instruction
    loop {}
}
