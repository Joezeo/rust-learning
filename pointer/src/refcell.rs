#![allow(dead_code)]

pub trait Message {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Message> {
    message: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: 'a + Message> LimitTracker<'a, T> {
    pub fn new(message: &'a T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            message,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;

        if percentage > 1.0 {
            self.message.send("Error: You are over your quota!");
        } else if percentage >= 0.9 {
            self.message.send("Urgent warning: You've used up over 90% percentage of your quota!");
        } else if percentage >= 0.75 {
            self.message.send("Warning: You've used up over 75% percentage");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessage {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessage {
        fn new() -> MockMessage {
            MockMessage {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Message for MockMessage {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn test_sends_over_75_percent_warning_message() {
        let mock_message = MockMessage::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_message.sent_messages.borrow().len(), 1);
    }
}
