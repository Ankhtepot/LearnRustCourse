#![allow(unused)]

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message_1 = ChatMessage { content: "content", time: "12:00".to_string() };
    let message_2 = ChatMessage { content: String::from("Sample Audio"), time: "12:00".to_string() };
    let message_3 = ChatMessage { content: DigitalContent::AudioFile, time: "12:00".to_string() };

    message_3.consume_entertainment();
    println!("{}", message_1.retrieve_time());
    println!("{}", message_2.retrieve_time());
    println!("{}", message_3.retrieve_time());
}