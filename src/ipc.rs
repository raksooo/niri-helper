use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::window_rules::WindowRule;

const SOCKET_PATH: &str = "/tmp/niri-helper.sock";

#[derive(Deserialize, Serialize)]
enum IpcCommand {
    WindowRule(WindowRule),
}

pub struct Ipc;

impl Ipc {
    pub fn listen(config: Arc<Mutex<Config>>) {
        let _ = fs::remove_file(SOCKET_PATH);

        thread::spawn(move || {
            let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind to socket");

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let config = config.clone();
                        thread::spawn(move || Self::handle_client(stream, config));
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        break;
                    }
                }
            }
        });
    }

    fn handle_client(stream: UnixStream, config: Arc<Mutex<Config>>) {
        let stream = BufReader::new(stream);
        for line in stream.lines() {
            if let Ok(line) = line {
                if let Ok(command) = serde_json::from_str(&line) {
                    let mut config = config.lock().expect("Failed to lock config mutex");
                    match command {
                        IpcCommand::WindowRule(rule) => {
                            config.add_window_rule(rule);
                        }
                    }
                } else {
                    println!("Failed to parse line: {}", line);
                }
            } else {
                println!("Failed to read line, breaking");
                break;
            }
        }
    }

    pub fn register_window_rule(rule: WindowRule) {
        let mut stream: UnixStream;

        let mut counter = 0;
        loop {
            if let Ok(socket_stream) = UnixStream::connect(SOCKET_PATH) {
                stream = socket_stream;
                break;
            } else {
                println!("Failed to connect to niri-helper daemon");
                if counter >= 10 {
                    panic!("Failed to connect to socket, is the daemon running?");
                }

                counter += 1;
                thread::sleep(Duration::from_millis(1000));
            }
        }

        let command = serde_json::to_string(&IpcCommand::WindowRule(rule))
            .expect("Failed to serialize command");
        stream
            .write_all(command.as_bytes())
            .expect("Failed to send command");
    }
}

impl Drop for Ipc {
    fn drop(&mut self) {
        let _ = fs::remove_file(SOCKET_PATH);
    }
}
