use bytes::Bytes;
use sha2::{Sha256, Digest};
use serde_json::{from_str, Value};
use base16ct::lower::encode_string;
use reqwest::{
    header::{ACCEPT, USER_AGENT, HeaderMap},
    blocking::{Client, RequestBuilder, Response}
};

use std::{
    fs::{File, create_dir, exists},
    io::{Error, ErrorKind, Write, copy}
};

// Headers for print statements during debugging
const IMPT_HEAD: &str = "[#] ";
const INFO_HEAD: &str = "[i] ";
const MISC_HEAD: &str = "[*] ";
const PASS_HEAD: &str = "[+] ";
const FAIL_HEAD: &str = "[-] ";
const EROR_HEAD: &str = "[!] ";

fn HandleError(error: Error)
{
    if error.kind() == ErrorKind::PermissionDenied
    {
        println!("{}Please run this prorgam with Local Administrator access.", EROR_HEAD);
    } else if let Some(raw_error) = error.raw_os_error()
    {
        if raw_error == 32 as i32
        {
            println!("{}Please stop the Anti-Virus before running theis program.", EROR_HEAD);
        }
    } else {
        println!("{}Unknown error, Error: {:?}", EROR_HEAD, error);
        println!("{}Please consult Windows Error Types at the following links and leave an issue report at \"https://github.com/vel2006/Anti-Virus/\".", IMPT_HEAD);
        println!("https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-\nhttps://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-");
        println!("https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--1000-1299-\nhttps://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--1300-1699-");
        println!("https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--1700-3999-\nhttps://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--4000-5999-");
        println!("https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--6000-8199-\nhttps://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--8200-8999-");
        println!("https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--9000-11999-\nhttps://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--12000-15999-");
    }
}

fn MakeHTTPRequest(url: String, download_file: bool) -> (bool, Option<String>, Option<Value>)
{
    // Creating the HTTP client
    let http_client: Client = Client::new();
    let mut http_headers: HeaderMap = HeaderMap::new();
    http_headers.insert(USER_AGENT, "Anti-Virus-Installer-Updater".parse().unwrap());
    if download_file
    {
        http_headers.insert(ACCEPT, "application/octet-stream".parse().unwrap());
    } else {
        http_headers.insert(ACCEPT, "application/vnd.github+json".parse().unwrap());
    }
    let http_request: RequestBuilder = http_client.get(url).headers(http_headers);
    let http_response: Response;
    if let Ok(response) = http_request.send()
    {
        http_response = response;
    } else {
        println!("{}Failed to get response from GitHub.", FAIL_HEAD);
        return (false, None, None);
    }

    // Either downloading the file or parsing the output from the request
    if download_file
    {
        let mut output_file: File;
        match File::create("C:\\Program Files\\Anti_Virus\\AV.exe")
        {
            Ok(file_handle) => output_file = file_handle,
            Err(error) => {
                println!("{}Failed to create file handle for install path: \"C:\\Program Files\\Anti_Virus\\AV.exe\"!", FAIL_HEAD);
                HandleError(error);
                return (false, None, None);
            }
        }
        let file_bytes: Bytes;
        match http_response.bytes()
        {
            Ok(bytes) => file_bytes = bytes,
            Err(error) => {
                println!("{}Failed to get bytes from request!\nError: {:?}", FAIL_HEAD, error);
                return (false, None, None);
            }
        }
        if let Err(error) = output_file.write_all(&file_bytes)
        {
            println!("{}Failed to write bytes to \"C:\\Program Files\\Anti_Virus\\AV.exe\"!", FAIL_HEAD);
            HandleError(error);
            return (false, None, None);
        }
        return (true, Some("C:\\Program Files\\Anti_Virus\\AV.exe".to_string()), None);
    } else {
        match http_response.text()
        {
            Ok(site_response) => {
                match from_str(&site_response)
                {
                    Ok(json_code) => return (true, None, Some(json_code)),
                    Err(error) => println!("{}Failed to parse output of text as JSON, error: {:?}\nText: {:?}", FAIL_HEAD, error, site_response)
                }
                return (true, None, None);
            },
            Err(error) => {
                println!("{}Failed to parse text response from github!\nError: {:?}", FAIL_HEAD, error);
            }
        }
    }
    return (true, None, None);
}

fn CheckFileHash(installed_file_path: String, release_hash: String) -> bool
{
    let mut file_handle: File;
    let mut hasher: Sha256 = Sha256::new();
    match exists(installed_file_path)
    {
        Ok(result) => {
            if !result
            {
                println!("{}AV is not installed, cannot check hash!", FAIL_HEAD);
                return false;
            }
        },
        Err(error) => {
            HandleError(error);
            return false;
        }
    }
    match File::open("C:\\Program Files\\Anti_Virus\\AV.exe")
    {
        Ok(handle) => file_handle = handle,
        Err(error) => {
            println!("{}Failed to open file \"C:\\Program Files\\Anti_Virus\\AV.exe\"", FAIL_HEAD);
            HandleError(error);
            return false;
        }
    }
    if let Err(error) = copy(&mut file_handle, &mut hasher)
    {
        println!("{}Failed to read file \"C:\\Program Files\\Anti_Virus\\AV.exe\" or write to SHA256 buffer!", FAIL_HEAD);
        HandleError(error);
        return false;
    }
    let file_hash: String = encode_string(&hasher.finalize().to_vec());
    if file_hash == release_hash
    {
        return true;
    }
    return false;
}

fn main() 
{
    // Check for the needed directories
    println!("{}Checking for install directories...", INFO_HEAD);
    let mut created_dirs: bool = false;
    match exists("C:\\Program Files\\Anti_Virus\\")
    {
        Ok(result) => {
            if !result
            {
                println!("{}Directory for Anti-Virus does not exists, creating...", INFO_HEAD);
                if let Err(error) = create_dir("C:\\Program Files\\Anti_Virus\\")
                {
                    HandleError(error);
                    return ();
                }
                if let Err(error) = create_dir("C:\\Program Files\\Anti_Virus\\Logs\\")
                {
                    HandleError(error);
                    return ();
                }
                created_dirs = true;
            }
        },
        Err(error) => {
            HandleError(error);
            return ();
        }
    }
    if !created_dirs
    {
        println!("{}Checking for log directories...", INFO_HEAD);
        match exists("C:\\Program Files\\Anti_Virus\\Logs\\")
        {
            Ok(result) => {
                if !result
                {
                    println!("{}Directory for logs does not exist, creating...", INFO_HEAD);
                    if let Err(error) = create_dir("C:\\Program Files\\Anti_Virus\\Logs\\")
                    {
                        if error.kind() != ErrorKind::AlreadyExists
                        {
                            println!("{}Failed to create log directory for Anti-Virus!\nError: {:?}", EROR_HEAD, error);
                            println!("{}Possible solution is to run with Local Administrator access.", IMPT_HEAD);
                            return ();
                        }
                    }
                }
            },
            Err(error) => {
                HandleError(error);
                return ();
            }
        }
    }

    // Getting information about the latest release
    println!("{}Getting raw binary to install...", INFO_HEAD);
    let json_response: Value;
    let (_, _, json_response_option) = MakeHTTPRequest("https://api.github.com/repos/vel2006/Anti-Virus/releases/latest".to_string(), false);
    if json_response_option.is_none()
    {
        println!("{}Failed to get response from Github, check errors.", EROR_HEAD);
        return ();
    }
    json_response = json_response_option.unwrap();
    let mut release_path: String = json_response["assets"][0]["browser_download_url"].to_string();
    let mut release_hash: String = json_response["assets"][0]["digest"].to_string();
    if let Some(new_path) = release_path.strip_prefix("\"")
    {
        release_path = new_path.to_string();
    }
    if let Some(new_path) = release_path.strip_suffix("\"")
    {
        release_path = new_path.to_string();
    }
    if let Some(new_hash) = release_hash.strip_prefix("\"")
    {
        release_hash = new_hash.to_string();
    }
    if let Some(new_hash) = release_hash.strip_suffix("\"")
    {
        release_hash = new_hash.to_string();
    }
    if let Some(new_hash) = release_hash.strip_prefix("sha256:")
    {
        release_hash = new_hash.to_string();
    }
    println!("{}Found release path: {} | {}", MISC_HEAD, release_path, release_hash);

    // Checking the installed file to see if the hashes match
    if CheckFileHash("C:\\Program Files\\Anti_Virus\\AV.exe".to_string(), release_hash)
    {
        println!("{}Installed version is already up to date!", PASS_HEAD);
        return ();
    }
    println!("{}File is outdated or not installed, installing newest version...", INFO_HEAD);

    // Downloading the raw binary that will be for the newest version of the software
    println!("{}Downloading release...", INFO_HEAD);
    let (result, output_path, _) = MakeHTTPRequest(release_path, true);
    if result
    {
        let binary_path: String = output_path.unwrap();
        println!("{}Downloaded release to path: {}", IMPT_HEAD, binary_path);
    } else {
        println!("{}Failed to download release to path, check errors.", EROR_HEAD);
    }
    return ();
}
