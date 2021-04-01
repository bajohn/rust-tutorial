#[derive(Debug)]
enum Message {
    Create(String),
}

impl Message {
    fn send(&self) {
        println!("{:#?}", self)
    }
}

fn main() {
    let msg = Message::Create(String::from("Test"));
    msg.send();
}
