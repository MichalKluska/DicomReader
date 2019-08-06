mod binary_data_reader;

use crate::dicom_value::DicomValue;
use std::str;
use std::fs::File;
use std::io::Read;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn visit_dirs(dir: &Path) -> Vec<DirEntry>
{
    let mut files : Vec<DirEntry> = Vec::new();
    if dir.is_dir() 
    {
        let read_dir = fs::read_dir(dir);

        if let Ok(dir) = read_dir 
        {
            for entry in dir
            {
                let entry = entry;

                if let Ok(entry) = entry 
                {
                    let path = entry.path();
                    if path.is_dir() 
                    {
                        let mut read_files = visit_dirs(&path);
                        files.append(&mut read_files);
                    }
                    else 
                    {
                        files.push(entry);
                    }
                }
            };
        }
    }
    return files;
}

pub fn read_file(filename : &Path) -> std::io::Result<std::fs::File>
{
    let file = File::open(filename)?;
    return Ok(file);
}

pub fn read_header(file: &mut std::fs::File) -> std::io::Result<[u8;4]>
{
    const HEADER_LENGTH : usize = 4;
    let mut header = [0; HEADER_LENGTH];
    file.read(&mut header)?;

    Ok(header)
}

pub fn read_preamble(file: &mut std::fs::File) -> std::io::Result<[u8;128]>
{
    const PREAMBLE_LENGTH : usize = 128;
    let mut preamble = [0; PREAMBLE_LENGTH];
    file.read(&mut preamble)?;

    Ok(preamble)
}

pub fn read_dicom_tag(file: &mut std::fs::File) -> std::io::Result<(u16, u16)>
{
    let group_number   = binary_data_reader::read_uint16(file)?;
    let element_number = binary_data_reader::read_uint16(file)?;

    Ok((group_number, element_number))
}

pub fn read_dicom_value(file: &mut std::fs::File) -> std::io::Result<DicomValue>
{
    let mut value_representation : [u8; 2] = [0x00, 0x00];
    file.read(&mut value_representation)?;

    let result = match str::from_utf8(&value_representation).unwrap()
    {
        "AE" => Ok(DicomValue::ApplicationEntity{data: read_string_value(file)?}),
        "AS" => Ok(DicomValue::AgeString{data: read_string_value(file)?}),
        "AT" => Ok(DicomValue::AttributeTag{data: [0, 0, 0, 0]}),
        "CS" => Ok(DicomValue::CodeString{data: read_string_value(file)?}),
        "DA" => Ok(DicomValue::Date{data: [1,9,0,0,0,1,0,1]}),
        "DS" => Ok(DicomValue::DecimalString{data: read_string_value(file)?}),
        "DT" => Ok(DicomValue::DateTime{data: read_string_value(file)?}),
        "FL" => Ok(DicomValue::FloatingPointSingle{data: 0.0}),
        "FD" => Ok(DicomValue::FloatingPointDouble{data: 0.0}),
        "IS" => Ok(DicomValue::IntegerString{data: read_string_value(file)?}),
        "LO" => Ok(DicomValue::DecimalString{data: read_string_value(file)?}),
        "LS" => Ok(DicomValue::LongString{data: read_string_value(file)?}),
        "LT" => Ok(DicomValue::LongText{data: read_string_value(file)?}),
        "OB" => Ok(DicomValue::OtherByteString{data: read_other_byte_word(file)?}),
        "OD" => Ok(DicomValue::OtherDoubleString{data: read_other_byte_word(file)?}),
        "OF" => Ok(DicomValue::OtherFloatString{data: read_other_byte_word(file)?}),
        "OW" => Ok(DicomValue::OtherWordString{data: read_other_byte_word(file)?}),
        "PN" => Ok(DicomValue::PersonName{data: read_string_value(file)?}),
        "SH" => Ok(DicomValue::ShortString{data: read_string_value(file)?}),
        "SL" => Ok(DicomValue::SignedLong{data: 0}),
        "SQ" => Ok(DicomValue::SequenceOfItems{data: vec!()}),
        "SS" => Ok(DicomValue::SignedShort{data: 0}),
        "ST" => Ok(DicomValue::ShortText{data: read_string_value(file)?}),
        "TM" => Ok(DicomValue::Time{data: read_string_value(file)?}),
        "UI" => Ok(DicomValue::UniqueIdentifier{data: read_string_value(file)?}),
        "UL" => Ok(DicomValue::UnsignedLong{data: read_unsigned_long(file)?}),
        "UN" => Ok(DicomValue::Unknown{data: vec!()}),
        "US" => Ok(DicomValue::UnsignedShort{data: 0}),
        "UT" => Ok(DicomValue::UnlimitedText{data: read_string_value(file)?}),
        _    => Ok(DicomValue::WithoutType{ data: read_other_element(file, value_representation)?}),
    };

    result
}

fn read_unsigned_long(file: &mut std::fs::File) -> std::io::Result<u32>
{
    binary_data_reader::read_uint16(file)?;

    let value = binary_data_reader::read_uint32(file)?;

    Ok(value)
}

fn read_other_byte_word(file: &mut std::fs::File) -> std::io::Result<Vec<u8>>
{
    binary_data_reader::read_uint16(file)?;

    let size = binary_data_reader::read_uint32(file)?;
    let mut data: Vec<u8> = Vec::new();
    data.resize(size as usize, 0);
    file.read_exact(&mut data)?;

    Ok(data)
}

fn read_string_value(file: &mut std::fs::File) -> std::io::Result<String>
{
    let size = binary_data_reader::read_uint16(file)?;

    let mut data: Vec<u8> = Vec::new();
    data.resize(size as usize, 0);
    file.read_exact(&mut data)?;

    if data[data.len()-1 as usize] as char == ' ' || 
       data[data.len()-1 as usize] as char == '\u{0}' || 
       data[data.len()-1 as usize] == 0 
    {
        println!("przed {:#?}", data);
        data.resize(data.len()-1, 0);
        println!("po {:#?}", data);
    }

    Ok(String::from_utf8(data).unwrap())
}

fn read_other_element(file: &mut std::fs::File, value_representation : [u8; 2]) -> std::io::Result<Vec<u8>>
{
    let mut size_bytes : [u8; 2] = [0x0, 0x0];
    file.read(&mut size_bytes)?;

    let size: u32 = value_representation[0] as u32 + ((value_representation[1] as u32) << 8) + 
                    ((size_bytes[0] as u32 + ((size_bytes[1] as u32) << 8)) << 16);

    let mut data: Vec<u8> = Vec::new();
    data.resize(size as usize, 0);
    file.read_exact(&mut data)?;

    Ok(data)
}