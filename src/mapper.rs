use crate::mapping::{Mapping, MsgType};

use std::{fs};
use serde::{Deserialize, Serialize};
use midly::MidiMessage;
use midly::num::u7;

#[derive (Serialize, Deserialize, Debug, Clone)]
pub struct Mapper {
    mappings : Vec<Mapping>
}

fn midly_2_param(to : &MsgType, a_8 : u8, b_8 : u8) -> MidiMessage
{
    let a = u7::from_int_lossy(a_8);
    let b = u7::from_int_lossy(b_8);
    match to {
        MsgType::NoteOn => MidiMessage::NoteOn { key: a, vel: b },
        MsgType::NoteOff => MidiMessage::NoteOn { key: a, vel: b },
        MsgType::Aftertouch => MidiMessage::Aftertouch { key: a, vel: b },
        MsgType::Control => MidiMessage::Controller { controller: a, value: b },
    }
}

impl Mapper {
    pub fn new_from_json_file(filename : &str) -> Result<Mapper, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(filename)?;
        let m : Mapper = serde_json::from_str(&data)?;
        Ok(m)
    }

    pub fn map_message(self : &Self, in_message : MidiMessage) -> MidiMessage {
        match in_message {
            MidiMessage::NoteOff { key, vel } => {
                let mappings = self.mappings.iter().find(|x| {
                    x.from == MsgType::NoteOff && x.start <= key && x.stop >= key
                });
                return match mappings {
                    Some(m) => {
                        midly_2_param(&m.to, key.as_int() + m.to_start - m.start, vel.as_int())
                    }
                    None => in_message
                }
            }
            MidiMessage::NoteOn { key, vel } => {
                let mappings = self.mappings.iter().find(|x| {
                    x.from == MsgType::NoteOn && x.start <= key && x.stop >= key
                });
                return match mappings {
                    Some(m) => {
                        midly_2_param(&m.to, key.as_int() + m.to_start - m.start, vel.as_int())
                    }
                    None => in_message
                }
            }
            MidiMessage::Aftertouch { key, vel } => {
                let mappings = self.mappings.iter().find(|x| {
                    x.from == MsgType::Aftertouch && x.start <= key && x.stop >= key
                });
                return match mappings {
                    Some(m) => {
                        midly_2_param(&m.to, key.as_int() + m.to_start - m.start, vel.as_int())
                    }
                    None => in_message
                }
            }
            MidiMessage::Controller { controller, value } => {
                let mappings = self.mappings.iter().find(|x| {
                    x.from == MsgType::Control && x.start <= controller && x.stop >= controller
                });
                return match mappings {
                    Some(m) => {
                        midly_2_param(&m.to, controller.as_int() + m.to_start - m.start, value.as_int())
                    }
                    None => in_message
                }
            }
            MidiMessage::ProgramChange { .. } => {}
            MidiMessage::ChannelAftertouch { .. } => {}
            MidiMessage::PitchBend { .. } => {}
        }


        return in_message;
    }
}