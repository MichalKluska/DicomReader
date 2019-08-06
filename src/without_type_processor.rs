use crate::dicom_value::DicomValue;
extern crate byteorder;

use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};

use std::convert::AsMut;

fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

fn remove_space_on_last_position(mut string: String) -> String
{
    if string.as_bytes()[string.len()-1 as usize] as char == ' ' || 
       string.as_bytes()[string.len()-1 as usize] as char == '\u{0}' || 
       string.as_bytes()[string.len()-1 as usize] == 0 
    {
        string = string[0..string.len()-1].to_string()
    }

    string
}

fn read_single_float(bytes: [u8; 4]) -> f32
{
    let mut rdr = Cursor::new(bytes);
    let value = rdr.read_f32::<LittleEndian>().unwrap();
    return value;
}

fn read_double_float(bytes: [u8; 8]) -> f64
{
    let mut rdr = Cursor::new(bytes);
    let value = rdr.read_f64::<LittleEndian>().unwrap();
    return value;
}

pub fn process_dicom_value_without_type(dicom_value: Vec<u8>, 
                                        value_representation: &str) -> DicomValue
{
    let result = match value_representation
    {
        "AE" => DicomValue::ApplicationEntity{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "AS" => DicomValue::AgeString{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "AT" => DicomValue::AttributeTag{data: (u16::from_ne_bytes(clone_into_array(&dicom_value[0..2])), u16::from_ne_bytes(clone_into_array(&dicom_value[2..4])))},
        "CS" => DicomValue::CodeString{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "DA" => DicomValue::Date{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "DS" => DicomValue::DecimalString{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "DT" => DicomValue::DateTime{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "FL" => DicomValue::FloatingPointSingle{data: read_single_float(clone_into_array(&dicom_value[0..4]))},
        "FD" => DicomValue::FloatingPointDouble{data: read_double_float(clone_into_array(&dicom_value[0..8]))},
        "IS" => DicomValue::IntegerString{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "LO" => DicomValue::LongString{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "LT" => DicomValue::LongText{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "OB" => DicomValue::OtherByteString{data: dicom_value},
        "OD" => DicomValue::OtherDoubleString{data: dicom_value},
        "OF" => DicomValue::OtherFloatString{data: dicom_value},
        "OW" => DicomValue::OtherWordString{data: dicom_value},
        "PN" => DicomValue::PersonName{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "SH" => DicomValue::ShortString{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "SL" => DicomValue::SignedLong{data: i32::from_ne_bytes(clone_into_array(&dicom_value[0..4]))},
        "SQ" => DicomValue::SequenceOfItems{data: vec!()},
        "SS" => DicomValue::SignedShort{data: i16::from_ne_bytes(clone_into_array(&dicom_value[0..2]))},
        "ST" => DicomValue::ShortText{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "TM" => DicomValue::Time{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "UI" => DicomValue::UniqueIdentifier{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        "UL" => DicomValue::UnsignedLong{data: u32::from_ne_bytes(clone_into_array(&dicom_value[0..4]))},
        "UN" => DicomValue::Unknown{data: vec!()},
        "US" => DicomValue::UnsignedShort{data: u16::from_ne_bytes(clone_into_array(&dicom_value[0..2]))},
        "UT" => DicomValue::UnlimitedText{data: remove_space_on_last_position(String::from_utf8(dicom_value).unwrap())},
        _ => DicomValue::NotOk,
    };

    result
}