use crate::dicom_value::DicomValue;

pub fn process_dicom_value_without_type(dicom_value: Vec<u8>, 
                                        value_representation: &str) -> DicomValue
{
    let result = match value_representation
    {
        "AE" => DicomValue::ApplicationEntity{data: String::from_utf8(dicom_value).unwrap()},
        "AS" => DicomValue::AgeString{data: String::from_utf8(dicom_value).unwrap()},
        "AT" => DicomValue::AttributeTag{data: [0, 0, 0, 0]},
        "CS" => DicomValue::CodeString{data: String::from_utf8(dicom_value).unwrap()},
        "DA" => DicomValue::Date{data: [1,9,0,0,0,1,0,1]},
        "DS" => DicomValue::DecimalString{data: String::from_utf8(dicom_value).unwrap()},
        "DT" => DicomValue::DateTime{data: String::from_utf8(dicom_value).unwrap()},
        "FL" => DicomValue::FloatingPointSingle{data: 0.0},
        "FD" => DicomValue::FloatingPointDouble{data: 0.0},
        "IS" => DicomValue::IntegerString{data: String::from_utf8(dicom_value).unwrap()},
        "LO" => DicomValue::LongString{data: String::from_utf8(dicom_value).unwrap()},
        "LT" => DicomValue::LongText{data: String::from_utf8(dicom_value).unwrap()},
        "OB" => DicomValue::OtherByteString{data: dicom_value},
        "OD" => DicomValue::OtherDoubleString{data: dicom_value},
        "OF" => DicomValue::OtherFloatString{data: dicom_value},
        "OW" => DicomValue::OtherWordString{data: dicom_value},
        "PN" => DicomValue::PersonName{data: String::from_utf8(dicom_value).unwrap()},
        "SH" => DicomValue::ShortString{data: String::from_utf8(dicom_value).unwrap()},
        "SL" => DicomValue::SignedLong{data: 0},
        "SQ" => DicomValue::SequenceOfItems{data: vec!()},
        "SS" => DicomValue::SignedShort{data: 0},
        "ST" => DicomValue::ShortText{data: String::from_utf8(dicom_value).unwrap()},
        "TM" => DicomValue::Time{data: String::from_utf8(dicom_value).unwrap()},
        "UI" => DicomValue::UniqueIdentifier{data: String::from_utf8(dicom_value).unwrap()},
        "UL" => DicomValue::UnsignedLong{data: 0},
        "UN" => DicomValue::Unknown{data: vec!()},
        "US" => DicomValue::UnsignedShort{data: 0},
        "UT" => DicomValue::UnlimitedText{data: String::from_utf8(dicom_value).unwrap()},
        _ => DicomValue::NotOk,
    };

    result
}