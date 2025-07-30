// Importing crates
use ctrlc;
use colored::*;
use std::time::Duration;
use std::thread::{spawn, JoinHandle, sleep};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};

// Importing external scripts
mod logging;
use logging::*;
mod allow_list;
use allow_list::*;
mod process_handling;
use process_handling::*;

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

fn main()
{
    // Basic variables
    let programs_path: &str = "programs.json";
    let users_path: &str = "users.json";
    let allowed_programs: Option<Vec<String>> = LoadData(programs_path);
    let allowed_users: Option<Vec<String>> = LoadData(users_path);
    // AV enging
    let engine = Arc::new(Mutex::new(AVEngine::new(false, true, allowed_programs, allowed_users)));
    // Variables for handling threads
    let shutdown = Arc::new(AtomicBool::new(false));
    let process_handler = Arc::clone(&engine);
    let process_shutdown = Arc::clone(&shutdown);
    let user_handler = Arc::clone(&engine);
    let user_shutdown = Arc::clone(&shutdown);
    let process_thread: JoinHandle<()> = spawn(move || {
        while !process_shutdown.load(Ordering::Relaxed)
        {
            let mut engine = process_handler.lock().unwrap();
            engine.handle_processes();
            sleep(Duration::from_millis(100));
        }
    });
    let user_thread: JoinHandle<()> = spawn(move || {
        while !user_shutdown.load(Ordering::Relaxed)
        {
            let mut engine = user_handler.lock().unwrap();
            engine.handle_users();
            sleep(Duration::from_millis(100));
        }
    });
    // Capturing ^C to end the script if pressed
    println!("{}Press ^C to end script.", info_head());
    ctrlc::set_handler(move || {
        shutdown.store(true, Ordering::Relaxed);
    }).expect("Error setting Ctrl-C handler");
    _ = user_thread.join();
    _ = process_thread.join();
    println!("{}Both threads joined!", pass_head());
    let users: Vec<String> = engine.lock().unwrap().get_users();
    let programs: Vec<String> = engine.lock().unwrap().get_programs();
    println!("{}Saving users...", misc_head());
    WriteData(users_path, users);
    println!("{}Saving programs...", misc_head());
    WriteData(programs_path, programs);
    return ();
}