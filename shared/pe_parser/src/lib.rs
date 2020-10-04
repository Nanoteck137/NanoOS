use core::convert::TryInto;

const IMAGE_FILE_MACHINE_I386: u16 = 0x14c;
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664; 

#[derive(Debug)]
enum PeMachine {
    Unknown,
    AMD64,
    I386
}

#[derive(Debug)]
pub struct PeParser<'a> {
    bytes: &'a [u8],

    machine: PeMachine,

    image_base: u64,
    entry_point: u64,

    num_sections: usize,
    section_offset: usize,
}

impl<'a> PeParser<'a> {
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let bytes: &[u8] = bytes.as_ref();

        if bytes.get(0..2) != Some(b"MZ") { return None; }

        let pe_offset: usize = 
            u32::from_le_bytes(bytes.get(0x3c..0x40)?
                .try_into().ok()?).try_into().ok()?;

        println!("PE Offset: {}", pe_offset);

        if bytes.get(pe_offset..pe_offset.checked_add(4)?) != Some(b"PE\0\0") {
            return None;
        }

        if pe_offset.checked_add(24)? > bytes.len() {
            return None;
        }

        let machine = 
            u16::from_le_bytes(bytes[pe_offset + 4..pe_offset + 6]
                .try_into().ok()?);

        let machine = match machine {
            IMAGE_FILE_MACHINE_I386 => PeMachine::I386,
            IMAGE_FILE_MACHINE_AMD64 => PeMachine::AMD64,

            _ => PeMachine::Unknown
        };

        println!("Machine: {:?}", machine);

        let num_sections: usize = 
            u16::from_le_bytes(bytes[pe_offset + 6..pe_offset + 8]
                .try_into().ok()?)
                .try_into().ok()?;
        println!("Number of sections: {}", num_sections);

        let optional_header_size: usize = 
            u16::from_le_bytes(bytes[pe_offset + 20..pe_offset + 22]
                .try_into().ok()?)
                .try_into().ok()?;
        println!("Optional header size: {}", optional_header_size);

        // TODO(patrik): use the magic number in side the optional header 
        // and see if the pe is a PE32 or PE32+ because that could tell us 
        // which offset we should use for the image base
        let image_base = match machine {
            PeMachine::I386 => { u32::from_le_bytes(bytes.get(pe_offset + 52..pe_offset + 56)?.try_into().ok()?) as u64 }
            PeMachine::AMD64 => { u64::from_le_bytes(bytes.get(pe_offset + 48..pe_offset + 56)?.try_into().ok()?) }

            _ => { panic!("Unknown machine"); }
        };

        let entry_point = 
        u32::from_le_bytes(bytes.get(pe_offset + 0x28..pe_offset+0x2c)?
            .try_into().ok()?) as u64;
        let entry_point = image_base.checked_add(entry_point)?;

        println!("Image Base: 0x{:x}", image_base);
        println!("Entry Point: 0x{:x}", entry_point);

        let header_size = pe_offset.checked_add(0x18)?
            .checked_add(optional_header_size)?
            .checked_add(num_sections.checked_mul(0x28)?)?;
        println!("Header Size: {}", header_size);

        if header_size > bytes.len() {
            return None;
        }

        let section_offset = pe_offset + 0x18 + optional_header_size;

        Some(PeParser { bytes, machine, image_base, entry_point, 
                        num_sections, section_offset })
    }
}