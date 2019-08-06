extern crate arrayvec;

use arrayvec::ArrayString;

#[derive(Debug)]
pub enum DicomValue
{
    ApplicationEntity { data: String}, 
    AgeString {data: String},
    AttributeTag {data: (u16, u16)},
    CodeString {data: String}, 
    Date {data: String},
    DecimalString {data: String},
    DateTime {data: String},
    FloatingPointSingle {data: f32},
    FloatingPointDouble {data: f64},
    IntegerString {data: String},
    LongString {data: String},
    LongText {data: String},
    OtherByteString {data: Vec<u8>},         //Do doczytania
    OtherDoubleString {data: Vec<u8>},       //Do doczytania
    OtherFloatString {data: Vec<u8>},        //Do doczytania
    OtherWordString {data: Vec<u8>},         //Do doczytania
    PersonName {data: String},
    ShortString {data: String},
    SignedLong {data: i32},
    SequenceOfItems {data: Vec<u8>},
    SignedShort {data: i16},
    ShortText {data: String},
    Time {data: String},
    UniqueIdentifier {data: String},
    UnsignedLong {data: u32},
    Unknown {data: Vec<u8>},
    UnsignedShort {data: u16},
    UnlimitedText {data: String},
    WithoutType{data: Vec<u8>},
    NotOk
}
