extern crate rayon;
extern crate time;

mod dicom_value;
mod file_reader;
mod dicom_tag_map;
mod dicom_validator;
mod without_type_processor;

use std;
use std::path::Path;
use rayon::prelude::*;
use time::PreciseTime;

fn main()
{
    let start = PreciseTime::now();

    let file_entries = file_reader::visit_dirs(Path::new("C://Users//MichałKluska//Desktop//Dane//ambrozy//ambrozy//ct_okrojone//ct"));

    let dicom_files_infos : Vec<Vec<(&str, dicom_value::DicomValue)>> = 
        file_entries.par_iter()
        .filter_map(|file_entry| process_dicom_file(file_entry))
        .collect();

    println!("{}", dicom_files_infos.len());

    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));

    let dataS = "-82.5882\\-152.901\\1663";
    let a : Vec<&str> = dataS.split('\\').collect();
    let mut b : Vec<f32> = Vec::new();

    for elem in a.iter()
    {
        let a = elem.parse::<f32>();

        match a
        {
            Ok(a) => b.push(a),
            _ => (),
        }
    }

    println!("{:#?}", dicom_files_infos[0]);

    // for value in dicom_files_infos[0].iter()
    // {
    //     match &value.1
    //     {
    //         dicom_value::DicomValue::DecimalString{data} => println!("{},  {}, {:#?}", value.0, data, data.parse::<u64>()),
    //         _ => (),    
    //     }
    // }
}

fn process_dicom_file(file_entry: &std::fs::DirEntry) -> Option<Vec<(&str, dicom_value::DicomValue)>>
{
    let path_buf = file_entry.path();
    let path = path_buf.as_path();

    let mut file = file_reader::read_file(path).ok().unwrap();

    let _preamble = file_reader::read_preamble(&mut file).ok().unwrap();
    let header = file_reader::read_header(&mut file).ok().unwrap();
    
    if !dicom_validator::validate_dicom_header(header)
    {
        return None;
    }

    let mut dicom_values : Vec<(&str, dicom_value::DicomValue)> = Vec::new();

    loop
    {
        let dicom_tag = file_reader::read_dicom_tag(&mut file);

        if let Ok(dicom_tag) = dicom_tag 
        {
            if dicom_tag == (0x7FE0, 0x0010)
            {
                break;
            }

            let iterator = dicom_tag_map::DICOM_TAG_MAPS.iter().find(|probe| probe.0 == dicom_tag);
            let mut actual_dicom_value = file_reader::read_dicom_value(&mut file).ok().unwrap();

            if let Some(arr_iterator) = iterator 
            {
                if let dicom_value::DicomValue::WithoutType{data} = actual_dicom_value
                {
                    let value_representation : &str = arr_iterator.1;
                    actual_dicom_value = without_type_processor::process_dicom_value_without_type(data, &value_representation);
                } 

                dicom_values.push((arr_iterator.2, actual_dicom_value));
            };
        }
    }

    Some(dicom_values)
}