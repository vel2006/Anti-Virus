use std::io;
use std::mem;
use std::ptr;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HANDLE, HMODULE, MAX_PATH, CloseHandle, GetLastError, WIN32_ERROR},
        NetworkManagement::NetManagement::{NetUserEnum, MAX_PREFERRED_LENGTH, FILTER_NORMAL_ACCOUNT, NERR_Success, USER_INFO_0, NetApiBufferFree, NetUserDel},
        System::{
            ProcessStatus::{EnumProcessModules, EnumProcesses, GetModuleBaseNameA, GetModuleFileNameExA},
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, TerminateProcess, PROCESS_TERMINATE},
        }
    }
};

// A way to hold settings during runtime
struct Settings
{
    // Detection automation based
    kill_process_on_detection: bool,
    kill_user_on_detection: bool,
    run: bool,
    // Allow / exemptions
    whitelist_users: Vec<String>,
    whitelist_programs: Vec<String>
}

pub struct AVEngine
{
    // Using the defined settings for usability
    settings: Settings,
}

// Creating the settings for this program
impl Settings
{
    // Creating a new Settings struct
    fn new(stop_process_on_detection:bool, remove_user_on_detection:bool, whitelist_programs: Option<Vec<String>>, whitelist_users: Option<Vec<String>>) -> Self
    {
        let mut allowed_programs: Vec<String> = Vec::new();
        let mut allowed_users: Vec<String> = Vec::new();
        if let Some(mut programs) = whitelist_programs
        {
            allowed_programs.append(&mut programs);
        } else {
            let mut programs: Vec<String> = filter_program_list(get_running_processes());
            allowed_programs.append(&mut programs);
        }
        if let Some(mut users) = whitelist_users
        {
            allowed_users.append(&mut users);
        } else {
            let mut users: Vec<String> = get_current_users();
            allowed_users.append(&mut users);
        }
        let settings: Settings = Settings {
            kill_process_on_detection: stop_process_on_detection,
            kill_user_on_detection: remove_user_on_detection,
            run: true,
            whitelist_users: allowed_users,
            whitelist_programs: allowed_programs
        };
        return settings;
    }
    // Changing settings functions
    fn enable_process_on_detection(&mut self)
    {
        self.kill_process_on_detection = true;
    }
    fn disable_process_on_detection(&mut self)
    {
        self.kill_process_on_detection = false;
    }
    fn enable_user_on_detection(&mut self)
    {
        self.kill_user_on_detection = true;
    }
    fn disable_user_on_detection(&mut self)
    {
        self.kill_user_on_detection = false;
    }
    fn enable_av(&mut self)
    {
        self.run = true;
    }
    fn disable_av(&mut self)
    {
        self.run = false;
    }
    fn change_whitelist_programs(&mut self, new_list: Vec<String>)
    {
        self.whitelist_programs.clear();
        self.whitelist_programs = new_list;
    }
    fn add_whitelist_program(&mut self, new_program: String)
    {
        self.whitelist_programs.push(new_program);
    }
    fn add_whitelist_programs(&mut self, mut new_programs: Vec<String>)
    {
        self.whitelist_programs.append(&mut new_programs);
    }
    fn remove_whitelist_program(&mut self, program_index: usize)
    {
        self.whitelist_programs.remove(program_index);
    }
    fn change_whitelist_users(&mut self, new_list: Vec<String>)
    {
        self.whitelist_users.clear();
        self.whitelist_users = new_list;
    }
    fn add_whitelist_user(&mut self, new_user: String)
    {
        self.whitelist_users.push(new_user);
    }
    fn add_whitelist_users(&mut self, mut new_users: Vec<String>)
    {
        self.whitelist_users.append(&mut new_users);
    }
    fn remove_whitelist_user(&mut self, user_index: usize)
    {
        self.whitelist_users.remove(user_index);
    }
    // Getting settings functions
    fn get_process_on_detection(&mut self) -> bool
    {
        return self.kill_process_on_detection.clone();
    }
    fn get_user_on_detection(&mut self) -> bool
    {
        return self.kill_user_on_detection.clone();
    }
    fn get_av_status(&mut self) -> bool
    {
        return self.run.clone();
    }
    fn get_whitelist_programs(&mut self) -> Vec<String>
    {
        return self.whitelist_programs.clone();
    }
    fn get_whitelist_users(&mut self) -> Vec<String>
    {
        return self.whitelist_users.clone();
    }
}

// An 'API' for handling most of the connection logic and using the functions in a safe way
impl AVEngine
{
    // Creating a new AVEngine impl
    pub fn new(stop_process_on_detection: bool, remove_user_on_detection: bool, whitelist_programs: Option<Vec<String>>, whitelist_users: Option<Vec<String>>) -> Self
    {
        return AVEngine {
            settings: Settings::new(stop_process_on_detection, remove_user_on_detection, whitelist_programs, whitelist_users)
        };
    }
    // Getting the currently running programs names
    pub fn take_process_snapshot() -> Vec<String>
    {
        return filter_program_list(get_running_processes());
    }
    // Handling processes based on settings and updating allow lists during runtime based on user input
    pub fn handle_processes(&mut self)
    {
        if self.settings.get_av_status()
        {
            let allowed_programs: Vec<String> = self.settings.get_whitelist_programs();
            let running_programs: Vec<(String, u32)> = get_running_processes();
            for program in running_programs
            {
                let (program_name, program_pid) = program;
                if !allowed_programs.contains(&program_name)
                {
                    println!("Detected unsaved program: {} - PID: {:?}", program_name, program_pid);
                    if self.settings.get_process_on_detection()
                    {
                        kill_pid(program_pid);
                    } else {
                        let mut input_value: String = String::new();
                        println!("Select one of the following. [allow, deny] (deny is default)");
                        io::stdin().read_line(&mut input_value).expect("Cannot read user input.");
                        let check_value: &str = &input_value.as_str();
                        match check_value
                        {
                            "allow\r\n" => {
                                self.settings.add_whitelist_program(program_name.clone());
                                println!("Added program: {} to allow list.", program_name);
                            },
                            _ => {
                                kill_pid(program_pid);
                            }
                        }
                    }
                }
            }
        }
    }
    // Handling users based on settings and updating allow lists during runtime based on user input
    pub fn handle_users(&mut self)
    {
        if self.settings.get_av_status()
        {
            let allowed_users: Vec<String> = self.settings.get_whitelist_users();
            for user in get_current_users().iter()
            {
                if !allowed_users.contains(user)
                {
                    println!("Detected unknown user: {}", user);
                    if self.settings.get_user_on_detection()
                    {
                        remove_user(user.clone());
                    } else {
                        let mut input_value: String = String::new();
                        println!("Select one of the following. [allow, deny] (deny is default)");
                        io::stdin().read_line(&mut input_value).expect("Cannot read user input.");
                        let check_value: &str = &input_value.as_str();
                        match check_value
                        {
                            "allow\r\n" => {
                                self.settings.add_whitelist_user(user.clone());
                            },
                            _ => {
                                remove_user(user.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    // Editing the AV settings
    pub fn disable_engine(&mut self)
    {
        self.settings.disable_av();
    }
    pub fn enable_engine(&mut self)
    {
        self.settings.enable_av();
    }
    // Getting current settings
    pub fn get_users(&mut self) -> Vec<String>
    {
        return self.settings.get_whitelist_users();
    }
    pub fn get_programs(&mut self) -> Vec<String>
    {
        return self.settings.get_whitelist_programs();
    }
    // Snapshot functions
    pub fn take_program_snapshot() -> Vec<(String, u32)>
    {
        return get_running_processes();
    }
    pub fn take_program_name_snapshot() -> Vec<String>
    {
        return filter_program_list(get_running_processes());
    }
    pub fn take_user_snapshot() -> Vec<String>
    {
        return get_current_users();
    }
}

// Getting all running PIDs
fn get_running_pids() -> Vec<u32>
{
    let mut running_processes: Vec<u32> = Vec::new();
    let mut processes: [u32; 1024] = [0; 1024];
    let mut process_length: u32 = 0;
    unsafe
    {
        // Getting process PIDs
        let result = EnumProcesses(processes.as_mut_ptr(), (mem::size_of::<u32>() * processes.len()) as u32, &mut process_length);
        if result.is_err()
        {
            running_processes.push(0);
            return running_processes;
        }
    }
    // Removing any zeroes in the array
    for process in processes.iter()
    {
        if *process != 0
        {
            running_processes.push(process.clone());
        }
    }
    return running_processes;
}

// Getting information about a PID
fn get_process_information(process_pid: u32) -> Option<(String, String)>
{
    let mut process_path: String = "".to_string();
    let mut process_name: String = "".to_string();
    unsafe
    {
        let result = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, process_pid);
        if result.is_err()
        {
            return None;
        }
        let process_handle: HANDLE = result.ok().unwrap();
        let mut module_handle: HMODULE = Default::default();
        let mut module_length: u32 = 0;
        let result = EnumProcessModules(process_handle, &mut module_handle, size_of::<HMODULE>() as u32, &mut module_length);
        if result.is_err()
        {
            return None;
        }
        // Getting file path
        let mut file_path_bytes: [u8; MAX_PATH as usize] = [42; 260];
        let path_length: u32 = GetModuleFileNameExA(Some(process_handle), Some(module_handle), &mut file_path_bytes);
        if path_length != 0
        {
            process_path = String::from_utf8_lossy(&file_path_bytes[..path_length as usize]).to_string();
        }
        // Getting file name
        let mut file_name_bytes: [u8; MAX_PATH as usize] = [42; 260];
        let name_length: u32 = GetModuleBaseNameA(process_handle, Some(module_handle), &mut file_name_bytes);
        if name_length != 0
        {
            process_name = String::from_utf8_lossy(&file_name_bytes[..name_length as usize]).to_string();
        }
        // Getting file signature (once I feel like it)
        _ = CloseHandle(process_handle);
    }
    return Some((process_path, process_name));
}

// Getting all running processes and PIDs
fn get_running_processes() -> Vec<(String, u32)>
{
    let mut processes: Vec<(String, u32)> = Vec::new();
    let pids: Vec<u32> = get_running_pids();
    for pid in pids.iter()
    {
        let program_data: Option<(String, String)> = get_process_information(*pid);
        match program_data
        {
            Some(program_information) => {
                let (_, program_name) = program_information;
                processes.push((program_name, *pid));
            },
            None => {
                ();
            }
        }
    }
    return processes;
}

// Filtering out the program names to remove any repition and only hold the program name and no PID
fn filter_program_list(programs: Vec<(String, u32)>) -> Vec<String>
{
    let mut output: Vec<String> = Vec::new();
    for program_information in programs.iter()
    {
        let (program_name, _) = program_information;
        if !output.contains(program_name)
        {
            output.push(program_name.clone());
        }
    }
    return output;
}

// Closing target process via pid
fn kill_pid(process_pid: u32) -> Option<WIN32_ERROR>
{
    unsafe
    {
        let result = OpenProcess(PROCESS_TERMINATE, false, process_pid);
        if result.is_err()
        {
            return Some(GetLastError());
        }
        let process_handle: HANDLE = result.unwrap();
        let result = TerminateProcess(process_handle, 0);
        if result.is_err()
        {
            return Some(GetLastError());
        }
    }
    return None;
}

// Getting current user accounts
fn get_current_users() -> Vec<String>
{
    let mut users: Vec<String> = Vec::new();
    let mut buffer: *mut u8 = ptr::null_mut();
    let mut account_count: u32 = 0;
    let mut total_count: u32 = 0;
    unsafe
    {
        let result: u32 = NetUserEnum(PCWSTR::null(), 0, FILTER_NORMAL_ACCOUNT, &mut buffer, MAX_PREFERRED_LENGTH, &mut account_count, &mut total_count, None);
        if result == NERR_Success
        {
            let entries: *mut USER_INFO_0 = buffer as *mut USER_INFO_0;
            for i in 0..account_count
            {
                let entry: USER_INFO_0 = *entries.add(i as usize);
                let username: String = entry.usri0_name.to_string().unwrap_or("Unknown Username".to_string());
                users.push(username.clone());
            }
            NetApiBufferFree(Some(buffer as *mut _));
        }
    }
    return users;
}

// Deleting a user account
fn remove_user(username: String) -> Option<WIN32_ERROR>
{
    unsafe
    {
        let formatted_username: Vec<u16> = username.encode_utf16().chain(Some(0)).collect();
        let result: u32 = NetUserDel(PCWSTR::null(), PCWSTR(formatted_username.as_ptr()));
        if result != NERR_Success
        {
            return Some(GetLastError());
        }
        return None;
    }
}