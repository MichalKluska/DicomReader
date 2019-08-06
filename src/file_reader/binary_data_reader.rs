extern crate byteorder;

use std::io::Read;
use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};

pub fn read_uint16(file : &mut std::fs::File) -> std::io::Result<u16>
{
    let mut bytes: [u8; 2] = [0x00, 0x00]; 
    file.read(&mut bytes)?;

    let value: u16 = ((bytes[1] as u16) << 8 ) | (bytes[0] as u16);

    Ok(value)
}

pub fn read_int16(file : &mut std::fs::File) -> std::io::Result<i16>
{
    let mut bytes: [u8; 2] = [0x00, 0x00]; 
    file.read(&mut bytes)?;

    let value: i16 = ((bytes[1] as i16) << 8 ) | (bytes[0] as i16);

    Ok(value)
}

pub fn read_uint32(file : &mut std::fs::File) -> std::io::Result<u32>
{
    let mut bytes: [u8; 4] = [0x00, 0x00, 0x00, 0x00]; 
    file.read(&mut bytes)?;

    let value: u32 = ((bytes[3] as u32) << 24)  |
                     ((bytes[2] as u32) << 16 ) |
                     ((bytes[1] as u32) << 8  ) |
                     ((bytes[0] as u32));
    
    Ok(value)
}

pub fn read_int32(file : &mut std::fs::File) -> std::io::Result<i32>
{
    let mut bytes: [u8; 4] = [0x00, 0x00, 0x00, 0x00]; 
    file.read(&mut bytes)?;

    let value: i32 = ((bytes[3] as i32) << 24)  |
                     ((bytes[2] as i32) << 16 ) |
                     ((bytes[1] as i32) << 8  ) |
                     ((bytes[0] as i32));
    
    Ok(value)
}

pub fn read_single_float(file : &mut std::fs::File) -> std::io::Result<f32>
{
    let mut bytes: [u8; 4] = [0x00, 0x00, 0x00, 0x00]; 
    file.read(&mut bytes)?;

    let mut rdr = Cursor::new(bytes);
    let value = rdr.read_f32::<LittleEndian>().unwrap();
    
    Ok(value)
}

pub fn read_double_float(file : &mut std::fs::File) -> std::io::Result<f64>
{
    let mut bytes: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; 
    file.read(&mut bytes)?;

    let mut rdr = Cursor::new(bytes);
    let value = rdr.read_f64::<LittleEndian>().unwrap();
    
    Ok(value)
}