use std::collections::HashMap;
use std::process::Command;
use std::process::Output;
use regex::Regex;
use std::str;
use std::vec::IntoIter;
use std::iter::FromIterator;

use crate::superxtractor;
use crate::superxtractor::SuperXtractor;

#[derive(Debug)]
pub struct HwInfo {
    model: String,
    hw_uuid: String,
    chip: String
}

impl HwInfo {
    pub fn new(from:Vec<superxtractor::Match>) -> Option<HwInfo> {
        let mut out = HwInfo {
            model: String::from(""),
            hw_uuid: String::from(""),
            chip: String::from(""),
        };

        for m in from.iter() {
            match m.field.as_str() {
                "model"=>{ out.model = m.text.clone() },
                "hw_uuid"=>{ out.hw_uuid = m.text.clone() },
                "chip" => { out.chip = m.text.clone() }, 
                &_ => ()
            }
        }
        return Some(out)
    }
}

fn sysprofiler(section: &str) -> Output {
    return Command::new("/usr/sbin/system_profiler")
        .arg(section)
        .output()
        .expect("Could not run system_profiler");
}

fn lines_from_output(inp: Vec<u8>) -> Vec<String> {
    let string_data = String::from_utf8(inp).expect("system_profiler returned non-unicode data :(");

    return str::split(&string_data, "\n").map(|s| s.to_string()).collect();
    
    // for part in str::split(&string_data, "\n") {
    //     out.push(part.to_string());
    // }
    // return out.to_owned();
}

pub fn get_ip_addresses() -> Vec<String> {
    let address_xtractor:regex::Regex = Regex::new(r"(?m)IPv4 Addresses:\s*(.*)$").unwrap();

    let response = sysprofiler("SPNetworkDataType").stdout;
    let string_data = String::from_utf8(response).expect("system_profiler returned non-unicode data :(");
    let mut results = vec![];
    for (_, [path]) in address_xtractor
        .captures_iter(&string_data)
        .map(|el| el.extract()) {
            results.push(path.to_owned());
        }

    return results.clone();
}

pub fn get_hw_info() -> Option<HwInfo> {
    let response = sysprofiler("SPHardwareDataType");

    let the_knowledge = vec![
        ("model", Regex::new(r"^\s+Model Name: (.*)\s*$").unwrap()),
        ("hw_uuid", Regex::new(r"^\s+Hardware UUID: (.*)\s*$").unwrap()),
        ("chip", Regex::new(r"^\s+Chip: (.*)\s*$").unwrap()),
    ].into_iter();
    
    let xt:SuperXtractor = SuperXtractor::new(the_knowledge);
    match String::from_utf8(response.stdout) {
        Err(err)=>{
            println!("ERROR Could not decode system_profiler response as UTF-8: {}", err);
            return None
        }
        Ok(output)=>{
            let result = xt.execute_by_line(output);
            return HwInfo::new(result);
        }
    }
}