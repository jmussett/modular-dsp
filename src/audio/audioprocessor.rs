use midi::MidiEvent;

pub enum Command<'a> {
    SendMidiEvents(Vec<MidiEvent>),
    SendCommandInput(&'a str, f32)
}

pub trait AudioProcessor<'a> {
    fn process_audio<RC: Fn() -> Option<Command<'a>>>(&mut self, receive_commands: RC);
}