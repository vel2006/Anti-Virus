// Importing crates
use colored::*;

// Importing external scripts
mod allow_list;
mod av_engine;
mod logging;
mod gui;

use std::ptr;
use std::env;
use std::mem;
use std::process;
use windows::{
    core::{PCSTR, Error},
    Win32::{
        System::Threading::{OpenProcessToken, GetCurrentProcess},
        Foundation::{GetLastError, HINSTANCE, WIN32_ERROR, HANDLE, CloseHandle},
        Security::{TOKEN_QUERY, TOKEN_ELEVATION, TokenElevation, GetTokenInformation},
        UI::{
            Shell::ShellExecuteA,
            WindowsAndMessaging::SW_SHOWNORMAL
        }
    }
};

// Headers for print statements during debugging
fn impt_head() -> String
{
    return "[#]".cyan().to_string() + " ";
}
fn info_head() -> String
{
    return "[i]".blue().to_string() + " ";
}
fn misc_head() -> String
{
    return "[*]".magenta().to_string() + " ";
}
fn pass_head() -> String
{
    return "[+]".green().to_string() + " ";
}
fn fail_head() -> String
{
    return "[-]".bright_red().to_string() + " ";
}
fn eror_head() -> String
{
    return "[!]".red().to_string() + " ";
}

fn become_admin() -> bool
{
    // Gaining Administratro access through UAC
    unsafe
    {
        let operation: Vec<u8> = "runas".as_bytes().to_vec();
        let current_directory = env::current_exe();
        let mut current_path: Vec<u8> = Vec::default();
        if let Ok(directory) = current_directory
        {
            current_path = directory.as_path().as_os_str().to_str().unwrap().as_bytes().to_vec();
            current_path.push(0);
        } else {
            println!("Failed to get path, aborting.");
            return false;
        }
        let instance_handle: HINSTANCE = ShellExecuteA(None, PCSTR(operation.as_ptr()), PCSTR(current_path.as_ptr()), PCSTR(ptr::null()), PCSTR(ptr::null()), SW_SHOWNORMAL);
        if (instance_handle.0 as isize) <= 32
        {
            let error: WIN32_ERROR = GetLastError();
            if error == WIN32_ERROR(203)
            {
                let second_attempt: HINSTANCE = ShellExecuteA(None, PCSTR(operation.as_ptr()), PCSTR(current_path.as_ptr()), PCSTR(ptr::null()), PCSTR(ptr::null()), SW_SHOWNORMAL);
                if (second_attempt.0 as isize) > 32
                {
                    return true;
                }
            }
        }
        return true;
    }
}

fn main()
{
    let mut is_admin: bool = false;
    unsafe
    {
        // Checking to see if process is running with Administrator
        let mut current_token: HANDLE = HANDLE::default();
        let open_token: Result<(), Error> = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut current_token);
        if open_token.is_err()
        {
            println!("Cannot query current token, escilating.");
            _ = become_admin();
        }
        let mut elivated_token: TOKEN_ELEVATION = TOKEN_ELEVATION::default();
        let mut token_size: u32 = mem::size_of::<TOKEN_ELEVATION>() as u32;
        let success: Result<(), Error> = GetTokenInformation(current_token, TokenElevation, Some(&mut elivated_token as *mut _ as *mut _), token_size, &mut token_size);
        if success.is_err()
        {
            println!("Failed to escilate token, is not Administrator.");
            _ = CloseHandle(current_token);
            _ = become_admin();
        } else {
            if success.is_ok() && open_token.is_ok()
            {
                is_admin = true;
            }
        }
    }
    if is_admin == true
    {
        _ = gui::start();
    }
    return ();
}