use queue::{VerdictHandler,PacketHandler,Verdict,QueueHandle,CopyMode};
use message::Message;
use handle::{Handle, ProtocolFamily};
use error::*;

#[derive(Debug)]
struct Callback(u32);
#[derive(Debug)]
struct Decider(u32);

impl PacketHandler for Callback {
    fn handle(&mut self, hq: QueueHandle, message: Result<&Message, &Error>) -> i32 {
        match self.0 {
            42 => -1,
            _ => panic!()
        }
    }
}

impl VerdictHandler for Decider {
    fn decide(&mut self, message: &Message) -> Verdict {
        match self.0 {
            42 => panic!(),
            _  => Verdict::Accept
        }
    }
}

#[test]
fn bind() {
    let decider = Decider(42);
    let mut handle = Handle::new().ok().unwrap();

    let mut queue = handle.queue(0, decider).ok().unwrap();
    queue.set_mode(CopyMode::None).ok().unwrap();

    handle.bind(ProtocolFamily::INET).ok().unwrap();
}

#[test]
#[should_panic]
fn decider() {
    println!("decider1");
    let decider = Decider(42);
    println!("{:?}",decider);
    let mut handle = Handle::new().ok().unwrap();
    println!("decider2");

    let mut queue = handle.queue(0, decider).ok().unwrap();
    println!("decider3");
    queue.set_mode(CopyMode::None).ok().unwrap();
    println!("decider4");

    handle.bind(ProtocolFamily::INET).ok().unwrap();
    println!("decider5");
    handle.start(4096);
    println!("decider6");
}

#[test]
fn callback() {
    println!("callback1");
    let callback = Callback(42);
    println!("{:?}",callback);
    let mut handle = Handle::new().ok().unwrap();

    println!("callback2");
    let mut queue = handle.queue(0, callback).ok().unwrap();
    println!("callback3");
    queue.set_mode(CopyMode::None).ok().unwrap();

    println!("callback4");
    handle.bind(ProtocolFamily::INET).ok().unwrap();
    println!("callback5");
    handle.start(4096);
}
