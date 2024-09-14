use niri_ipc::{Action, Event, Request, Response, Window};
use std::collections::VecDeque;
use std::env;
use std::io::{prelude::*, BufReader};
use std::os::unix::net::UnixStream;

const EVENT_STREAM_COMMAND: &[u8] = b"\"EventStream\"\n";
const SOCKET_ENV_VARIABLE: &str = "NIRI_SOCKET";

#[derive(Debug)]
enum Reply {
    Response(Response),
    Event(Event),
}

fn main() {
    let socket_path = env::var(SOCKET_ENV_VARIABLE).unwrap();
    let mut socket = UnixStream::connect(socket_path.clone()).unwrap();

    let mut known_window_ids: Vec<u64> = Vec::new();

    socket.write_all(EVENT_STREAM_COMMAND).unwrap();

    let mut reader = BufReader::new(&mut socket);
    let mut line = String::new();
    let mut command_queue = VecDeque::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => match parse_line(&line) {
                Reply::Response(Response::Windows(windows)) => {
                    update_known_window_ids(&mut known_window_ids, &windows)
                }
                Reply::Event(event) => {
                    handle_event(&event, &mut known_window_ids, &mut command_queue)
                }
                _ => (),
            },
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }

        while let Some(command) = command_queue.pop_front() {
            let mut socket = UnixStream::connect(socket_path.clone()).unwrap();
            let mut json = serde_json::to_vec(&command).unwrap();
            json.push(b'\n');
            socket.write_all(&json).unwrap();
        }
    }
}

fn update_known_window_ids(known_window_ids: &mut Vec<u64>, windows: &Vec<Window>) {
    known_window_ids.clear();
    for window in windows {
        known_window_ids.push(window.id);
    }
}

fn parse_line(line: &str) -> Reply {
    let response_result: Result<Result<Response, String>, serde_json::Error> =
        serde_json::from_str(&line);
    if let Ok(Ok(response)) = response_result {
        Reply::Response(response)
    } else {
        let event: Result<Event, serde_json::Error> = serde_json::from_str(&line);
        Reply::Event(event.unwrap())
    }
}

fn handle_event(
    event: &Event,
    known_window_ids: &mut Vec<u64>,
    command_queue: &mut VecDeque<Request>,
) {
    match event {
        Event::WindowsChanged { windows } => update_known_window_ids(known_window_ids, windows),
        Event::WindowOpenedOrChanged { window } => {
            handle_window_opened_or_changed(window, known_window_ids, command_queue);
        }
        Event::WindowClosed { id } => {
            known_window_ids.retain(|x| x != id);
        }
        _ => (),
    }
}

fn handle_window_opened_or_changed(
    window: &Window,
    known_window_ids: &mut Vec<u64>,
    command_queue: &mut VecDeque<Request>,
) {
    if !known_window_ids.contains(&window.id) {
        known_window_ids.push(window.id);

        handle_window_opened(window, command_queue);
    }
}

fn handle_window_opened(window: &Window, command_queue: &mut VecDeque<Request>) {
    match window {
        Window {
            app_id: Some(id), ..
        } if id == "alacritty-in-column" => {
            command_queue.push_back(Request::Action(Action::ConsumeOrExpelWindowLeft {}));
        }
        _ => (),
    }
}
