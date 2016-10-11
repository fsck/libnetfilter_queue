use message::Message;
use queue::{CopyMode,VerdictHandler,PacketHandler,Verdict,QueueHandle};
use handle::{Handle, ProtocolFamily};
use error::*;

struct Void;
struct Callback;
struct Decider;

impl PacketHandler for Callback {
    fn handle(&mut self, hq: QueueHandle, message: Result<&Message, &Error>) -> i32 { -1 }
}

impl VerdictHandler for Decider {
    fn decide(&mut self, message: &Message) -> Verdict { panic!(); Verdict::Accept }
}

#[test]
fn bind() {
    let decider = Decider;
    let mut handle = Handle::new().ok().unwrap();

    let mut queue = handle.queue(0,decider).ok().unwrap();
    queue.set_mode(CopyMode::None).ok().unwrap();

    handle.bind(ProtocolFamily::INET).ok().unwrap();
}

#[test]
#[should_panic]
fn decider() {
    let decider = Decider;
    let mut handle = Handle::new().ok().unwrap();

    let mut queue = handle.queue(0,decider).ok().unwrap();
    queue.set_mode(CopyMode::None).ok().unwrap();

    handle.bind(ProtocolFamily::INET).ok().unwrap();
    handle.start(4096);
}

#[test]
fn callback() {
    let callback = Callback;
    let mut handle = Handle::new().ok().unwrap();

    let mut queue = handle.queue(0,callback).ok().unwrap();
    queue.set_mode(CopyMode::None).ok().unwrap();

    handle.bind(ProtocolFamily::INET).ok().unwrap();
    handle.start(4096);
}
