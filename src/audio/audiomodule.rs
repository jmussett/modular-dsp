use midi::{MidiEvent};

pub type InputBuffer<'a> = &'a [f32];
pub type OutputBuffer<'a> = &'a mut [f32];

pub trait AudioModule {
    fn process_audio_input(&mut self, input: InputBuffer);
    fn process_audio_output(&mut self, output: OutputBuffer);
    fn process_midi_input(&mut self, midi_event: Vec<MidiEvent>);
    fn process_command_input(&mut self, command: &str, input: f32);
}