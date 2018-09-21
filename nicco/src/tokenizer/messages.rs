use proc_macro2::{ Span };
use std::fmt;

#[derive(Clone, Debug)]
pub enum MessageContexts {
    CallSite (Span),
    Span (Span),
}

impl Eq for MessageContexts {}

/// * Using Eq from Span caused runtime panic in proc_marco2 so we fell back to string compare
impl PartialEq for MessageContexts {
    fn eq(&self, other: &MessageContexts) -> bool {
        match self {
            MessageContexts::CallSite (ref span) => {
                let l = format!("{:?}", span);
                match other {
                    MessageContexts::CallSite (ref other) => {
                        let r = format!("{:?}", other);
                        l == r
                    },
                    _ => false
                }
            }
            MessageContexts::Span (span) => {
                let l = format!("{:?}", span);
                match other {
                    MessageContexts::Span (other) => {
                        let r = format!("{:?}", other);
                        l == r
                    },
                    _ => false
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageTypes {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Message {
    context: MessageContexts,
    message_type: MessageTypes,
    message: String,
}

impl Message {
    pub fn call_site(message_type: MessageTypes, message: &str) -> Self {
        Message {
            context: MessageContexts::CallSite (Span::call_site()),
            message_type,
            message: message.to_owned(),
        }
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let message_type = match self.message_type {
            MessageTypes::Error => "Error:  ",
            MessageTypes::Info => "Info:   ",
            MessageTypes::Warning => "Warning:",
        };
        write!(f, "{:}: {:}", message_type, self.message);

        Ok (())
    }
}

pub struct Info {}

impl Info {
    pub fn call_site(message: &str) -> Message {
        Message::call_site(MessageTypes::Info, message)
    }
}

pub struct Warning {}

impl Warning {
    pub fn call_site(message: &str) -> Message {
        Message::call_site(MessageTypes::Warning, message)
    }
}

pub struct Error {}

impl Error {
    pub fn call_site(message: &str) -> Message {
        Message::call_site(MessageTypes::Error, message)
    }
}

pub trait IValidated {
    fn get_messages(&self) -> Vec<Message>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Messages (Option<Vec<Message>>);

impl Messages {
    pub fn empty() -> Self {
        Messages ( None )
    }

    pub fn emit(self) {
        let messages: Vec<Message> = self.into();
        for Message { context, message_type, message } in messages {
            let ctx = match context {
                MessageContexts::CallSite (ctx) => ctx,
                MessageContexts::Span (ctx) => ctx,
            };
            match message_type {
                MessageTypes::Info => println!("INFO:\t{:?}", message),
                MessageTypes::Error => ctx.unstable().error(message).emit(),
                MessageTypes::Warning => ctx.unstable().warning(message).emit(),
            }
        }
    }
}

impl From<Vec<Message>> for Messages {
    fn from(src: Vec<Message>) -> Messages {
        if src.len() == 0 {
            return Messages (None)
        }
        Messages (Some(src))
    }
}

impl Into<Vec<Message>> for Messages {
    fn into(self) -> Vec<Message> {
        if let Some (messages) = self.0 {
            return messages
        }
        vec![]
    }
}