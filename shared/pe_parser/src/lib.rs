use core::convert::TryInto;

const IMAGE_FILE_MACHINE_I386: u16 = 0x14c;
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664; 

// Represents a supported machine of the PE file that we support as a parser
#[derive(Debug)]
enum PeMachine {
    Unknown,
    AMD64,
    I386
}

// Infomation about the PE file we parsed and infomation we need to 
// parse out the sections
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
        let bytes = bytes.as_ref();

        // Check the DOS Header signature, and the signature 
        // should be 0x5A4D or 'MZ'
        if bytes.get(0..2) != Some(b"MZ") { return None; }

        // Parse the 'offset' to the pe header, and the offset 
        // should be at the file offset 0x3C
        let pe_offset: usize = 
            u32::from_le_bytes(bytes.get(0x3c..0x40)?
                .try_into().ok()?).try_into().ok()?;

        // Check the PE signature, and it should be 0x50450000 or 'PE\0\0'
        if bytes.get(pe_offset..pe_offset.checked_add(4)?) != Some(b"PE\0\0") {
            return None;
        }

        // Check so the PE header fits in the file, otherwise just return none
        if pe_offset.checked_add(24)? > bytes.len() {
            return None;
        }

        // From here on out the range pe_offset to pe_offset + 24 
        // should be safe to parse out, but past the pe_offset + 24 that 
        // might not be safe because the didn't get that data, how knows

        // Parse out the machine identifier
        let machine = 
            u16::from_le_bytes(bytes[pe_offset + 4..pe_offset + 6]
                .try_into().ok()?);

        // Get the machine identifier from the parsed identifier
        let machine = match machine {
            IMAGE_FILE_MACHINE_I386 => PeMachine::I386,
            IMAGE_FILE_MACHINE_AMD64 => PeMachine::AMD64,

            _ => PeMachine::Unknown
        };

        // Parse the number of sections
        let num_sections: usize = 
            u16::from_le_bytes(bytes[pe_offset + 6..pe_offset + 8]
                .try_into().ok()?)
                .try_into().ok()?;

        // Parse the size of the optional header
        let optional_header_size: usize = 
            u16::from_le_bytes(bytes[pe_offset + 20..pe_offset + 22]
                .try_into().ok()?)
                .try_into().ok()?;

        // Parse the image base, the image base have diffrent offsets and 
        // sizes based on the magic number or the arch
        // TODO(patrik): use the magic number in side the optional header 
        // and see if the pe is a PE32 or PE32+ because that could tell us 
        // which offset we should use for the image base
        let image_base = match machine {
            PeMachine::I386 => { 
                u32::from_le_bytes(bytes.get(pe_offset + 52..pe_offset + 56)?
                    .try_into().ok()?) as u64 
            }

            PeMachine::AMD64 => { 
                u64::from_le_bytes(bytes.get(pe_offset + 48..pe_offset + 56)?
                    .try_into().ok()?) 
            }

            _ => { panic!("Unknown machine"); }
        };

        // Parse the entry point offset
        let entry_point = 
            u32::from_le_bytes(bytes.get(pe_offset + 0x28..pe_offset+0x2c)?
                .try_into().ok()?) as u64;
        // Calculate the real entry point based on the image base 
        // and the entry point offset
        let entry_point = image_base.checked_add(entry_point)?;

        // Calculate the header size
        let header_size = pe_offset.checked_add(0x18)?
            .checked_add(optional_header_size)?
            .checked_add(num_sections.checked_mul(0x28)?)?;

        // Check the header size so it fits in the file bytes
        if header_size > bytes.len() {
            return None;
        }

        // Calculate the offset were the sections start
        let section_offset = pe_offset + 0x18 + optional_header_size;

        Some(PeParser { bytes, machine, image_base, entry_point, 
                        num_sections, section_offset })
    }

    pub fn sections<F>(&self, mut func: F) -> Option<()>
        where F: FnMut(u64, u32, &[u8]) -> Option<()> {
        
        let bytes = self.bytes;

        // Go through all the sections and parse out some infomation 
        // then call the closure passed in with the parsed info
        for section in 0..self.num_sections {
            // Calculate the offset for this specific section 
            let offset = self.section_offset + section * 0x28;

            // let name = bytes.get(offset + 0..offset + 8)?;

            // Parse the virtual size of the sections
            let virtual_size = 
                u32::from_le_bytes(bytes.get(offset + 8..offset + 12)?
                    .try_into().ok()?);

            // Parse the virtual address, this is a actually an offset and 
            // should be added to the image base to get the real address
            let virtual_address = 
                u32::from_le_bytes(bytes.get(offset + 12..offset + 16)?
                    .try_into().ok()?);

            // Get the size of the raw data for the section
            let raw_size = 
                u32::from_le_bytes(bytes.get(offset + 16..offset + 20)?
                    .try_into().ok()?);

            // Get the offset of the raw data, this is a offset from the 
            // file and not the image base
            let raw_offset: usize = 
                u32::from_le_bytes(bytes.get(offset + 20..offset + 24)?
                    .try_into().ok()?)
                    .try_into().ok()?;

            // TODO(patrik): Get the 'Characteristics' field from 
            // the section header and get the permissions from that
            // let characteristics = 
            //     u32::from_le_bytes(bytes.get(offset + 36..offset + 40)?
            //         .try_into().ok()?);

            // Get the minimal size required for the raw data
            let raw_size: usize = 
                core::cmp::min(raw_size, virtual_size)
                    .try_into().ok()?;

            // Call the closure with the 
            // virtual_address, size of the data, and the bytes of the raw data
            func(
                self.image_base.checked_add(virtual_address as u64)?,
                virtual_size,
                bytes.get(raw_offset..raw_offset.checked_add(raw_size)?)?
            )?;
        }

        Some(())
    }
}