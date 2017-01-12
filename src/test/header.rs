use queue::{VerdictHandler,PacketHandler,Verdict,QueueHandle};
use message::{Message, IPHeader};
use handle::{Handle, ProtocolFamily};
use error::*;

struct Void;
struct Callback;
struct Decider;

impl PacketHandler for Callback {
    fn handle(&mut self, _: QueueHandle, message: Result<&Message, &Error>) -> i32 {
        unsafe { message.ok().unwrap().ip_header().ok().unwrap(); }
        -1
    }
}

impl VerdictHandler for Decider {
    fn decide(&mut self, message: &Message) -> Verdict {
        unsafe { message.ip_header().ok().unwrap(); }
        panic!();
        Verdict::Accept
    }
}

#[test]
fn bind() {
    let decider = Decider;
    let mut handle = Handle::new().ok().unwrap();

    let mut queue = handle.queue(0,decider).ok().unwrap();
    queue.set_mode_sized::<IPHeader>().ok().unwrap();

    let _ = handle.bind(ProtocolFamily::INET).ok().unwrap();
}

#[test]
#[should_panic]
fn decide() {
    let decider = Decider;
    let mut handle = Handle::new().ok().unwrap();

    if let Ok(mut queue) = handle.queue(0,decider){
        if let Ok(_) = queue.set_mode_sized::<IPHeader>(){
            let _ = handle.bind(ProtocolFamily::INET).ok().unwrap();
            handle.start_sized::<IPHeader>().unwrap();
        }
        println!("something broke inside...");
    }
    else {
        println!("something broke outside...")
    }
}

#[test]
fn callback() {
    let callback = Callback;
    let mut handle = Handle::new().ok().unwrap();

    if let Ok(mut queue) = handle.queue(0,callback){
        if let Ok(_) = queue.set_mode_sized::<IPHeader>(){
            let _ = handle.bind(ProtocolFamily::INET6).ok().unwrap();
            handle.start_sized::<IPHeader>().unwrap();
        }
        println!("somebroke broke inside...");
    }
    else {
        println!("something broke outside...")
    }
}
