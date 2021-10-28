use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum MsgType {
    NoteOn,
    NoteOff,
    Aftertouch,
    Control,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Mapping {
    pub from: MsgType,
    pub to: MsgType,
    pub start: u8,
    pub stop: u8,
    pub to_start: u8,
}
