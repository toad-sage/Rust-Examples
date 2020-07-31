pub trait Messenger {
    fn send(&mut self, msg: &str);
}

pub struct LimitTracker<'a, T>
where
    T: Messenger,
{
    messenger: &'a mut T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &mut T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percenatge_of_max = self.value as f64 / self.max as f64;

        if percenatge_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota");
        } else if percenatge_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used 90% of your quota");
        } else if percenatge_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&mut self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_message() {
        let mut mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);

        limit_tracker.set_value(80);

        // for i in &mock_messenger.sent_messages {
        //     println!("{}", i);
        // }

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
