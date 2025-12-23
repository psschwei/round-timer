use iced::time;
use iced::Subscription;
use std::time::Duration;

use crate::Message;

pub fn timer_subscription() -> Subscription<Message> {
    time::every(Duration::from_secs(1)).map(|_| Message::Tick)
}
