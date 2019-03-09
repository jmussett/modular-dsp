use std::collections::{HashMap};
use serde::{Deserialize, Serialize};
use crate::core::midi::MidiEvent;

#[derive(Serialize, Deserialize)]
pub struct CommandSet {
    pub commands: Vec<Command>
}

impl CommandSet {
    pub fn from_midi(events: Vec<MidiEvent>) -> CommandSet {
        CommandSet {
            commands: events.into_iter().map(|e| Command::MidiEvent(e)).collect()
        }
    }
    pub fn from_input(key: String, value: f32) -> CommandSet {
        CommandSet {
            commands: vec!(Command::InputParameter(key, value))
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Command {
    InputParameter(String, f32),
    MidiEvent(MidiEvent)
}

pub type InputBuffer<'a> = &'a [f32];
pub type OutputBuffer<'a> = &'a mut [f32];

pub struct Events<'a> {
    pub input_parameters: &'a HashMap<String, f32>,
    pub midi_events: &'a Vec<MidiEvent>
}

pub struct AudioData<'a> {
    pub input: InputBuffer<'a>,
    pub output: OutputBuffer<'a>,
    pub events: Events<'a>
}

pub trait AudioModule {
    fn process_audio(&mut self, data: AudioData);
}

pub trait AudioProcessor {
    fn process_audio<RC: Fn() -> Option<CommandSet>>(&mut self, receive_commands: RC);
}