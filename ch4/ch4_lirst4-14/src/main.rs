#![allow(unused_variables)]

extern crate core;

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct GrandStation;

impl GrandStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };
    let base = GrandStation {};
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
