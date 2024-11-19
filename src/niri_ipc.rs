use niri_ipc::{socket::Socket, Event, Request, Response};

pub fn get_event_reader() -> impl FnMut() -> Event {
    let socket = get_socket();
    let (_, mut event_reader) = socket
        .send(Request::EventStream)
        .expect("Failed to start event stream. Are you running niri 1.9+?");

    move || event_reader().expect("Received error in event stream")
}

pub fn send_command(request: Request) -> Response {
    get_socket()
        .send(request)
        .expect("Failed to send command")
        .0
        .expect("Command failed")
}

fn get_socket() -> Socket {
    Socket::connect().expect("Failed to connect to socket, is niri running?")
}
