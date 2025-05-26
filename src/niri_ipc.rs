use niri_ipc::{socket::Socket, Event, Request, Response};

pub fn get_event_reader() -> impl FnMut() -> Event {
    let mut socket = get_socket();
    let _ = socket
        .send(Request::EventStream)
        .expect("Failed to send Event steam request");
    let mut read_event = socket.read_events();
    move || read_event().expect("Received error in event stream")
}

pub fn send_command(request: Request) -> Response {
    get_socket()
        .send(request)
        .expect("Failed to send command")
        .expect("Failed to send command")
}

fn get_socket() -> Socket {
    Socket::connect().expect("Failed to connect to socket, is niri running?")
}
