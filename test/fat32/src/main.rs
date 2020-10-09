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

#[derive(Debug)]
struct ExtendedBPB32 {
    fat_size: u32,
    flags: u16,
    fs_version: u16,
    root_cluster: u32,
    fs_info: u16,
    backup_boot_sector: u16,
    drive_number: u8,
    boot_signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    fs_type: [u8; 8]
}

fn parse_extended_bpb_32(bytes: &[u8]) -> Option<ExtendedBPB32> {
    if bytes.len() > 447 {
        return None;
    }

    let fat_size = 
        u32::from_le_bytes(bytes[0..4].try_into().ok()?);

    let flags = 
        u16::from_le_bytes(bytes[4..6].try_into().ok()?);

    let fs_version = 
        u16::from_le_bytes(bytes[6..8].try_into().ok()?);

    let root_cluster = 
        u32::from_le_bytes(bytes[8..12].try_into().ok()?);

    let fs_info = 
        u16::from_le_bytes(bytes[12..14].try_into().ok()?);
    
    let backup_boot_sector =
        u16::from_le_bytes(bytes[14..16].try_into().ok()?);

    // let reserved: [u8; 12] = bytes[16..28].try_into().ok()?;

    let drive_number = bytes[28];

    // let reserved2 = bytes[29];

    let boot_signature = bytes[30];

    let volume_id = 
        u32::from_le_bytes(bytes[31..35].try_into().ok()?);

    let volume_label: [u8; 11] = bytes[35..46].try_into().ok()?;

    let fs_type: [u8; 8] = bytes[46..54].try_into().ok()?;

    Some(ExtendedBPB32 {
        fat_size,
        flags,
        fs_version,
        root_cluster,
        fs_info,
        backup_boot_sector,
        drive_number,
        boot_signature,
        volume_id,
        volume_label,
        fs_type
    })
}

#[derive(Debug)]
struct BPB {
    jmp: [u8; 3],
    oem_name: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sector_count: u16,
    num_fats: u8,
    root_entry_count: u16,
    total_sectors_16: u16,
    media: u8,
    fat_size_16: u16,
    sectors_per_track: u16,
    number_of_heads: u16,
    hidden_sectors: u32,
    total_sectors_32: u32,
}

fn parse_bpb(bytes: &[u8]) -> Option<(BPB, ExtendedBPB32)> {
    if bytes.len() > 512 {
        return None;
    }

    let jmp: [u8; 3] = bytes[0..3].try_into().ok()?;

    let oem_name: [u8; 8] = bytes[3..11].try_into().ok()?;

    let bytes_per_sector = 
        u16::from_le_bytes(bytes[11..13].try_into().ok()?);

    let sectors_per_cluster: u8 = bytes[13];

    let reserved_sector_count = 
        u16::from_le_bytes(bytes[14..16].try_into().ok()?);

    let num_fats = bytes[16];

    let root_entry_count = u16::from_le_bytes(bytes[17..19].try_into().ok()?);

    let total_sectors_16 = 
        u16::from_le_bytes(bytes[19..21].try_into().ok()?);

    let media = bytes[21];

    let fat_size_16 = 
        u16::from_le_bytes(bytes[22..24].try_into().ok()?);

    let sectors_per_track = 
        u16::from_le_bytes(bytes[24..26].try_into().ok()?);
    
    let number_of_heads = 
        u16::from_le_bytes(bytes[26..28].try_into().ok()?);

    let hidden_sectors = 
        u32::from_le_bytes(bytes[28..32].try_into().ok()?);

    let total_sectors_32 = 
        u32::from_le_bytes(bytes[32..36].try_into().ok()?);

    let extended_bpb_32 = parse_extended_bpb_32(&bytes[36..448])?;

    Some((BPB {
        jmp,
        oem_name,
        bytes_per_sector,
        sectors_per_cluster,
        reserved_sector_count,
        num_fats,
        root_entry_count,
        total_sectors_16,
        media,
        fat_size_16,
        sectors_per_track,
        number_of_heads,
        hidden_sectors,
        total_sectors_32,
    }, extended_bpb_32))
}

#[derive(Debug)]
struct DirectoryEntry {
    name: [u8; 8],
    ext: [u8; 3],
    attributes: u8,
    undelete: u8,
    creation_time: u16,
    creation_date: u16,
    last_accessed_date: u16,
    last_modification_time: u16,
    last_modification_date: u16,
    cluster: u32,
    file_size: u32, 
}

fn parse_directory_entry(bytes: &[u8]) -> Option<DirectoryEntry> {
    let name: [u8; 8] = bytes[0..8].try_into().ok()?;
    let ext: [u8; 3] = bytes[8..11].try_into().ok()?;
    let attributes = bytes[11];
    
    let reserved = bytes[12];

    let undelete = bytes[13];

    let creation_time = 
        u16::from_le_bytes(bytes[14..16].try_into().ok()?);

    let creation_date =
        u16::from_le_bytes(bytes[16..18].try_into().ok()?);

    let last_accessed_date =
        u16::from_le_bytes(bytes[18..20].try_into().ok()?);

    let cluster_high = 
        u16::from_le_bytes(bytes[20..22].try_into().ok()?);

    let last_modification_time =
        u16::from_le_bytes(bytes[22..24].try_into().ok()?);

    let last_modification_date =
        u16::from_le_bytes(bytes[24..26].try_into().ok()?);

    let cluster_low =
        u16::from_le_bytes(bytes[26..28].try_into().ok()?);

    let cluster = ((cluster_high as u32) << 16) | cluster_low as u32;

    let file_size = 
        u32::from_le_bytes(bytes[28..32].try_into().ok()?);

    Some(DirectoryEntry {
        name,
        ext,
        attributes,
        undelete,
        creation_time,
        creation_date,
        last_accessed_date,
        last_modification_time,
        last_modification_date,
        cluster,
        file_size
    })
}

#[derive(Debug)]
struct LFNEntry {
    entry_order: u8,
    name: [u16; 13],
    attribute: u8,
    long_entry_type: u8,
    checksum: u8,
}

fn parse_lfn_entry(bytes: &[u8]) -> Option<LFNEntry> {
    let entry_order = bytes[0];
    let name_first: [u8; 10] = bytes[1..11].try_into().ok()?;
    let attribute = bytes[11];
    let long_entry_type = bytes[12];
    let checksum = bytes[13];
    let name_middle: [u8; 12] = bytes[14..26].try_into().ok()?;
    let name_last: [u8; 4] = bytes[28..32].try_into().ok()?;

    let mut name_bytes: [u8; 26] = [0u8; 26];
    name_bytes[0..10].copy_from_slice(&name_first);
    name_bytes[10..22].copy_from_slice(&name_middle);
    name_bytes[22..26].copy_from_slice(&name_last);

    let mut name: [u16; 13] = [0u16; 13];
    for index in 0..13 {
        name[index] = 
            u16::from_le_bytes(name_bytes[index * 2..index * 2 + 2]
                               .try_into().ok()?);
    }

    Some(LFNEntry {
        entry_order,
        name,
        attribute,
        long_entry_type,
        checksum,
    })
}

fn parse_directory(bytes: &[u8]) -> Option<()> {
    let mut file_name: [u8; 255] = [0u8; 255];
    for index in 0..16 {
        let offset = index * 32;
        if bytes[offset] == 0 { break; }
        if bytes[offset] == 0xE5 { continue; }

        if bytes[offset + 11] == 0xf { 
            let lfn_entry = parse_lfn_entry(&bytes[offset..offset+32])?; 

            if (lfn_entry.entry_order & 0x40) != 0 {
                let order = lfn_entry.entry_order & 0xf; 
                let offset = order * 13;
                for index in 0..13 {
                    let character = lfn_entry.name[index] & 0xff;
                    if character != 0xff {
                        file_name[index + offset as usize] = 
                            character as u8;
                    }
                }
            } else {
                let order = lfn_entry.entry_order & 0xf; 
                let offset = order * 13;
                for index in 0..13 {
                    file_name[index + offset as usize] = 
                        (lfn_entry.name[index] & 0xff) as u8;
                }
            }
        } else {
            println!("File: {}", std::str::from_utf8(&file_name).unwrap());
            let directory_entry = 
                parse_directory_entry(&bytes[offset..offset+32])?;
            println!("Directory Entry: {:#?}", directory_entry);
            file_name = [0u8; 255];
            println!();
        }

    }

    Some(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the disk file image
    let mut file = File::open("disk_image.img")?; 
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
    let (bpb, extended_bpb_32) = 
        parse_bpb(&bytes[partition_offset..partition_offset + 512]).unwrap();
    println!("BPB: {:#?}", bpb);
    println!("Extended BPB: {:#?}", extended_bpb_32);

    let root_dir_sectors = 
        ((bpb.root_entry_count * 32) + (bpb.bytes_per_sector - 1)) / bpb.bytes_per_sector;
    println!("Root Dir Sectors: {}", root_dir_sectors);

    let root_dir_sectors = root_dir_sectors as u32;
    let reserved_sector_count = bpb.reserved_sector_count as u32;
    let num_fats = bpb.num_fats as u32;
    let fat_size = extended_bpb_32.fat_size;
    let first_data_sector = 
        reserved_sector_count + (num_fats * fat_size) + root_dir_sectors;
    println!("First data sector: {}", first_data_sector);

    let cluster_number = extended_bpb_32.root_cluster;
    let sectors_per_cluster = bpb.sectors_per_cluster as u32;
    let first_sector_of_cluster = 
        ((cluster_number - 2) * sectors_per_cluster) + first_data_sector;
    println!("First Sector of cluster 2: {}", first_sector_of_cluster);

    let offset: usize = 
        ((first_sector_of_cluster + fat_partition_entry.lba_address) * 512).try_into()?;
    parse_directory(&bytes[offset..offset+512]).unwrap();

    let first_fat_sector = bpb.reserved_sector_count as u32;
    let fat_offset = 6 * 4;
    let fat_sector = first_fat_sector + (fat_offset / 512);
    let fat_sector = fat_sector as usize;
    let entry_offset = fat_offset % 512;
    let entry_offset = entry_offset as usize;
    println!("Fat Sector: {}", fat_sector);
    println!("First Fat Sector: {:#x}", first_fat_sector);
    println!("Entry Offset: {:#x}", entry_offset);

    let fat_lba = (fat_sector + fat_partition_entry.lba_address as usize) * 512;
    println!("Fat LBA: {:#x}", fat_lba);
    let table: [u8; 512] = 
        bytes[fat_lba..fat_lba + 512]
            .try_into()?;
    println!("Table: {:?}", table);
    let table_entry = 
        u32::from_le_bytes(table[entry_offset..entry_offset+4].try_into()?);
    println!("Table Entry: {:#x}", table_entry);

    Ok(())
}
