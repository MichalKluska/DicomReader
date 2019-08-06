pub fn validate_dicom_header(header: [u8; 4]) -> bool
{
    return header == ['D' as u8, 'I' as u8, 'C' as u8, 'M' as u8]
}