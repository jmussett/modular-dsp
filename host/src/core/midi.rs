use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MidiMessage {
    pub status: u8,
    pub data: u8
}

#[derive(Serialize, Deserialize)]
pub struct MidiEvent {
    pub message: MidiMessage
}

pub trait MidiProcessor {
    fn process_midi<C: Fn(Vec<MidiEvent>)>(&mut self, callback: C);
}