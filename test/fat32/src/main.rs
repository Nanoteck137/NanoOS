use std::io::prelude::*;
use std::fs::File;

use std::error::Error;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
struct PartitionEntry {
    status: u8,
    chs_address_first: [u8; 3],
    partition_type: u8,
    chs_address_last: [u8; 3],
    lba_address: u32,
    number_of_sectors: u32
}

impl Default for PartitionEntry {
    fn default() -> PartitionEntry {
        PartitionEntry {
            status: 0,
            chs_address_first: [0; 3],
            partition_type: 0,
            chs_address_last: [0; 3],
            lba_address: 0,
            number_of_sectors: 0
        }
    }
}

fn parse_partition_entry(bytes: &[u8]) -> Option<PartitionEntry> {
    // One entry is only 16 bytes to check if the slice has more then 16 bytes
    // so we can parse it
    if bytes.len() < 16 {
        return None;
    }

    // Parse the status byte
    let status = bytes[0];
    // Parse the first absolute CHS of in the partition
    let chs_address_first: [u8; 3] = bytes[1..4].try_into().ok()?; 
    // Parse the partition type
    let partition_type = bytes[4];
    // Parse the last absolute CHS of in the partition
    let chs_address_last: [u8; 3] = bytes[5..8].try_into().ok()?; 
    // Parse the LBA address for where this partition starts 
    let lba_address = u32::from_le_bytes(bytes[8..12].try_into().ok()?);
    // Parse the number of sectors this partition takes up
    let number_of_sectors = 
        u32::from_le_bytes(bytes[12..16].try_into().ok()?);

    // Create the partition entry structure
    return Some(PartitionEntry {
        status, chs_address_first, partition_type, 
        chs_address_last, lba_address, number_of_sectors
    });
}

fn parse_extended_bpb_32(bytes: &[u8]) -> Option<()> {
    if bytes.len() > 447 {
        return None;
    }

    let fat_size = 
        u32::from_le_bytes(bytes[0..4].try_into().ok()?);
    println!("Fat Size: {}", fat_size);

    let flags = 
        u16::from_le_bytes(bytes[4..6].try_into().ok()?);
    println!("Flags: {}", flags);

    let fs_version = 
        u16::from_le_bytes(bytes[6..8].try_into().ok()?);
    println!("FS Version: {}", fs_version);

    let root_cluster = 
        u32::from_le_bytes(bytes[8..12].try_into().ok()?);
    println!("Root Cluster: {}", root_cluster);

    let fs_info = 
        u16::from_le_bytes(bytes[12..14].try_into().ok()?);
    println!("FS Info: {}", fs_info);
    
    let backup_boot_sector =
        u16::from_le_bytes(bytes[14..16].try_into().ok()?);
    println!("Backup boot sector: {}", backup_boot_sector);

    let reserved: [u8; 12] = bytes[16..28].try_into().ok()?;

    let drive_number = bytes[28];
    println!("Drive Number: {:#x}", drive_number);

    let reserved2 = bytes[29];

    let boot_signature = bytes[30];
    println!("Boot Signature: {:#x}", boot_signature);

    let volume_id = 
        u32::from_le_bytes(bytes[31..35].try_into().ok()?);
    println!("Volume ID: {:#x}", volume_id);

    let volume_label: [u8; 11] = bytes[35..46].try_into().ok()?;
    println!("Volume Label: {}", std::str::from_utf8(&volume_label).unwrap());

    let fs_type: [u8; 8] = bytes[46..54].try_into().ok()?;
    println!("FS Type: {}", std::str::from_utf8(&fs_type).unwrap());

    Some(())
}

fn parse_bpb(bytes: &[u8]) -> Option<()> {
    if bytes.len() > 512 {
        return None;
    }

    let jmp: [u8; 3] = bytes[0..3].try_into().ok()?;
    println!("Jump: {:x?}", jmp);

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

    parse_extended_bpb_32(&bytes[36..448])?;

    Some(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the disk file image
    let mut file = File::open("fat.fs")?; 
    // Create a buffer for the bytes of the file
    let mut bytes = Vec::new();

    // Read the file to the buffer
    file.read_to_end(&mut bytes)?;

    // Initialize an array of 4 partition entries, used to fill out all 
    // the primary partition that exists on the disk image
    let mut parition_entries: [PartitionEntry; 4] = 
        [PartitionEntry::default(); 4];

    // The start offset of the partition entries in the boot sector
    let parition_entry_start = 0x01BE;
    // There is only a max of 4 primary partitions in a boot sector
    for index in 0..4 {
        // Calculate the offset for this specific entry
        let offset = parition_entry_start + index * 16; 
        // Get the raw bytes of the entry
        let entry: [u8; 16] = bytes[offset..offset + 16].try_into()?;
        // Parse the entry and get some infomation from the entry data
        let entry = parse_partition_entry(&entry);

        // Add the entry to the list
        parition_entries[index] = entry.unwrap();
    }
    println!("Entries: {:#?}", parition_entries);

    // Hard code where the fat partition is, this should be found in 
    // partition_entries array, but we only have one partition on this
    // disk image so for now we can select the first entry
    let fat_partition_entry = parition_entries[0];
    let partition_offset = fat_partition_entry.lba_address as usize * 512;
    parse_bpb(&bytes[partition_offset..partition_offset + 512]).unwrap();

    Ok(())
}
