// Importing the iced framework for the GUI (I am not making it from scratch.)
use iced::{
    Fill, Task, Theme, Length, Element, application, Subscription, time,
    widget::{text, column, row, container, button, Text, Column, Scrollable},
};
use std::time::Duration;

// Importing the AV's engine
use super::av_engine::*;

#[derive(Debug)]
struct GUIState
{
    status: bool,
    engine: AVEngine,
    running_programs: Vec<(String, u32)>,
    current_users: Vec<String>,
    blocked_programs: Vec<String>,
    target_suspect: String,
}

// Creating the default state for GUIState manually, so it holds values we want
impl Default for GUIState
{
    fn default() -> Self
    {
        return GUIState {
            engine: AVEngine::new(false, false, None, None, None, "C:\\Program Files\\Anti_Virus\\Logs".to_string(), 50),
            status: false,
            running_programs: Vec::default(),
            current_users: Vec::default(),
            blocked_programs: Vec::default(),
            target_suspect: String::default(),
        };
    }
}

#[derive(Debug, Clone)]
enum Message
{
    Tick,
    Deny,
    Allow,
    EnableAV,
    DisableAV,
}

fn view(state: &GUIState) -> Element<Message>
{
    // Formatting the lists of programs and users
    let running_programs: Vec<Element<Message>> = state.running_programs.iter().cloned().map(|program|{
        let (program_name, program_pid) = program;
        let program_information: String = format!("{} - PID: {:?}", program_name, program_pid);
        Text::new(program_information).size(18).into()
    }).collect();
    let current_users: Vec<Element<Message>> = state.current_users.iter().cloned().map(|user| {
        Text::new(user.clone()).size(18).into()
    }).collect();
    let blocked_programs: Vec<Element<Message>> = state.blocked_programs.iter().cloned().map(|program| {
        Text::new(program.clone()).size(18).into()
    }).collect();
    let running_program_column = Column::with_children(running_programs).spacing(5);
    let existing_users_column = Column::with_children(current_users).spacing(5);
    let currently_blocked_programs = Column::with_children(blocked_programs).spacing(5);
    let running_programs_view = Scrollable::new(running_program_column).height(Length::Fixed(400.0f32)).width(Length::Fixed(450.0f32));
    let existing_users_view = Scrollable::new(existing_users_column).height(Length::Fixed(180.0f32)).width(Length::Fixed(400.0f32));
    let blocked_programs_view = Scrollable::new(currently_blocked_programs).height(Length::Fixed(180.0f32)).width(Length::Fixed(400.0f32));
    // A variable to show the currently selected unknown program
    let target_suspect = Text::new(state.target_suspect.clone()).size(18);
    // Creating the layout (im treating it like it's a HTML file...)
    container(
        column![
            container(text(format!("Running status: {}", state.status)).size(24)).center_x(Fill),
            container(
                column![
                    text("Unknown program:").size(20),
                    row![
                        target_suspect,
                        button(text("Allow").size(18)).on_press(Message::Allow),
                        button(text("Block").size(18)).on_press(Message::Deny),
                    ],
                ]
            ).center_x(Length::Fill),
            container(
                row![
                    button(text("Enable AV").size(24)).on_press(Message::EnableAV),
                    button(text("Disable AV").size(24)).on_press(Message::DisableAV),
                ].spacing(15)
            ).center_x(Length::Fill),
            container(
                row![
                    column![
                        text("Running programs:").size(18),
                        running_programs_view,
                    ],
                    column![
                        text("Existing users:").size(18),
                        existing_users_view,
                        text("Banned / blacklisted programs:").size(18),
                        blocked_programs_view,
                    ]
                ],
            ).center_x(Length::Fill),
        ].spacing(50)
    ).padding(10).into()
}

// Adding functionality to the buttions
fn update(state: &mut GUIState, message: Message) -> Task<Message>
{
    match message
    {
        Message::EnableAV => {
            if !state.engine.status()
            {
                state.status = true;
                state.engine.enable_engine();
            }
            return Task::none();
        },
        Message::DisableAV => {
            state.status = false;
            state.engine.disable_engine();
            return Task::none();
        },
        Message::Allow => {
            state.engine.add_whitelist_program(state.target_suspect.clone());
            state.target_suspect = "".to_string();
            return Task::none();
        }
        Message::Deny => {
            state.engine.add_blacklist_program(state.target_suspect.clone());
            state.engine.kill_process(state.target_suspect.clone());
            state.target_suspect = "".to_string();
            return Task::none();
        }
        Message::Tick => {
            if state.engine.status()
            {
                state.engine.handle_processes();
            } else {
                let output: Option<Vec<String>> = state.engine.detect_programs();
                if let Some(mut processes) = output
                {
                    let process: String = processes.remove(0);
                    state.target_suspect = process;
                }
            }
            // Refreshing the display automatically
            state.running_programs = state.engine.take_program_snapshot();
            state.current_users = state.engine.get_users();
            state.blocked_programs = state.engine.get_blacklisted_programs();
            return Task::none();
        }
    }
}

fn subscription(state: &GUIState) -> Subscription<Message>
{
    return process_watcher();
}

fn process_watcher() -> Subscription<Message>
{
    return time::every(Duration::from_millis(1000)).map(|_| Message::Tick);
}

pub fn start() -> iced::Result
{
    application("Anti-Virus", update, view).theme(|_|Theme::TokyoNight).subscription(subscription).run()
}