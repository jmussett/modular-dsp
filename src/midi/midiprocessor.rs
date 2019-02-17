pub struct MidiMessage {
    pub status: u8,
    pub data: u8
}

pub struct MidiEvent {
    pub message: MidiMessage
}

pub trait MidiProcessor {
    fn new() -> Self;
    fn process_midi<C: Fn(Vec<MidiEvent>)>(&mut self, callback: C);
}