use crate::midi::MidiEvent;

pub struct InputParameter {
    pub key: &'static str,
    pub value: f32
}

pub enum Command {
    SendMidiEvents(Vec<MidiEvent>),
    SendInputParameter(&'static str, f32)
}

pub type InputBuffer<'a> = &'a [f32];
pub type OutputBuffer<'a> = &'a mut [f32];

pub struct Events<'a> {
    pub input_parameters: &'a Vec<InputParameter>,
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
    fn process_audio<RC: Fn() -> Option<Command>>(&mut self, receive_commands: RC);
}