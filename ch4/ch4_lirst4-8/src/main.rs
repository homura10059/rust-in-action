#![allow(unused_variables)]

extern crate core;

type Message = String;
#[derive(Debug)]
struct Mailbox {
    message: Vec<Message>,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.message.pop()
    }
}

#[derive(Debug)]
struct GrandStation;

impl GrandStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.message.push(msg)
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {
    let base = GrandStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { message: vec![] },
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
