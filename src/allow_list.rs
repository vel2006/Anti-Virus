use std::io::Write;
use std::fs::{self, File};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProgramEntry
{
    name: String,
    signature: Option<String>
}

pub type ProgramAllowList = Vec<ProgramEntry>;

#[derive(Serialize, Deserialize)]
pub struct UserEntry
{
    name: String
}

// Writing JSON data to the path
pub fn WriteData(list_path: &str, programs: Vec<String>) -> bool
{
    let file = File::create(list_path);
    if file.is_err()
    {
        return false;
    }
    let mut file_handle = file.unwrap();
    _ = file_handle.write_all("[\n".as_bytes());
    for program in programs[..programs.len() - 1].iter()
    {
        let line = format!("\t{{\n\t\t\"name\": \"{}\"\n\t}},\n", program);
        _ = file_handle.write_all(line.as_bytes());
    }
    let line = format!("\t{{\n\t\t\"name\": \"{}\"\n\t}}\n", programs.last().unwrap());
    _ = file_handle.write_all(line.as_bytes());
    _ = file_handle.write_all("]".as_bytes());
    return true;
}

// Loading JSON data from the path
pub fn LoadData(list_path: &str) -> Option<Vec<String>>
{
    let content = fs::read_to_string(list_path);
    if content.is_err()
    {
        println!("File doesnt exist.");
        return None;
    }
    let raw_data: String = content.unwrap();
    let json_content = serde_json::from_str(&raw_data);
    if json_content.is_err()
    {
        println!("Not valid JSON data:\n{:?}", raw_data);
        return None;
    }
    let allow_list: ProgramAllowList = json_content.unwrap();
    let mut programs: Vec<String> = Vec::new();
    for entry in allow_list.iter()
    {
        let program_name: String = entry.name.clone();
        programs.push(program_name);
    }
    return Some(programs);
}