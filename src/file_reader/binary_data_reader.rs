use std::io::Read;

pub fn read_uint16(file : &mut std::fs::File) -> std::io::Result<u16>
{
    let mut bytes: [u8; 2] = [0x00, 0x00]; 
    file.read(&mut bytes)?;

    let value: u16 = ((bytes[1] as u16) << 8 ) | (bytes[0] as u16);

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