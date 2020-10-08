use std::io::prelude::*;
use std::fs::File;

use std::error::Error;
use std::convert::TryInto;

fn parse_bpb(bytes: &[u8]) -> Option<()> {
    if bytes.len() < 36 {
        return None;
    }

    let jmp: [u8; 3] = bytes[0..3].try_into().ok()?;
    println!("Jump: {:?}", jmp);

    let oem_name: [u8; 8] = bytes[3..11].try_into().ok()?;
    println!("OEM Name: {:?}", std::str::from_utf8(&oem_name).ok()?);

    let bytes_per_sector = 
        u16::from_le_bytes(bytes[11..13].try_into().ok()?);
    println!("Bytes per sector: {}", bytes_per_sector);

    let sectors_per_cluster: u8 = bytes[13];
    println!("Sectors per cluster: {}", sectors_per_cluster);

    let reserved_sector_count = 
        u16::from_le_bytes(bytes[14..16].try_into().ok()?);
    println!("Reserved sector count: {}", reserved_sector_count);

    let num_fats = bytes[16];
    println!("Num fats: {}", num_fats);

    let root_entry_count = u16::from_le_bytes(bytes[17..19].try_into().ok()?);
    println!("Root entry count: {}", root_entry_count);

    let total_sectors_16 = 
        u16::from_le_bytes(bytes[19..21].try_into().ok()?);
    println!("Total sectors 16: {}", total_sectors_16);

    let media = bytes[21];
    println!("Media: {}", media);

    let fat_size_16 = 
        u16::from_le_bytes(bytes[22..24].try_into().ok()?);
    println!("Fat size 16: {}", fat_size_16);

    let sectors_per_track = 
        u16::from_le_bytes(bytes[24..26].try_into().ok()?);
    println!("Sectors per track: {}", sectors_per_track);
    
    let number_of_heads = 
        u16::from_le_bytes(bytes[26..28].try_into().ok()?);
    println!("Number of heads: {}", number_of_heads);

    let hidden_sectors = 
        u32::from_le_bytes(bytes[28..32].try_into().ok()?);
    println!("Number of hidden sectors: {}", hidden_sectors);

    let total_sectors_32 = 
        u32::from_le_bytes(bytes[32..36].try_into().ok()?);
    println!("Total sectors 32: {}", total_sectors_32);

    Some(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("fat.fs")?; 
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;
    parse_bpb(&bytes).unwrap();
    let mut offset = 446;
    let entry: [u8; 16] = bytes[offset..offset + 16].try_into()?;
    println!("Entry: {:#x?}", entry);
    offset += 16; 

    let entry: [u8; 16] = bytes[offset..offset + 16].try_into()?;
    println!("Entry: {:#x?}", entry);
    offset += 16; 

    let entry: [u8; 16] = bytes[offset..offset + 16].try_into()?;
    println!("Entry: {:#x?}", entry);
    offset += 16; 

    let entry: [u8; 16] = bytes[offset..offset + 16].try_into()?;
    println!("Entry: {:#x?}", entry);
    offset += 16; 

    Ok(())
}
