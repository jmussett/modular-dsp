use midi::MidiEvent;
use audio::{AudioModule, InputBuffer, OutputBuffer};
use std::f64::consts::PI;

const LOOKUP_SIZE: usize = 10000;

pub struct SineModule {
    left_phase: usize,
    right_phase: usize,
    frequency: f32,
    lookup_table: Vec<f32>,
    sample_rate: f32
}

impl AudioModule for SineModule {
    fn new(sample_rate: f32) -> SineModule {
        let lookup_table = &mut vec![0.0; LOOKUP_SIZE];

        for i in 0..LOOKUP_SIZE {
            lookup_table[i] = (i as f64 / LOOKUP_SIZE as f64 * PI * 2.0).sin() as f32;
        }

        SineModule {
            lookup_table: lookup_table.to_vec(),
            frequency: 0.0,
            left_phase: 0,
            right_phase: 0,
            sample_rate: sample_rate
        }
    }
    fn process_audio_input(&mut self, _input: InputBuffer) {}
    fn process_audio_output(&mut self, output: OutputBuffer) {
        if self.frequency == 0.0 {
            for i in 0..output.len()
            {
                output[i] = 0.0;
            }
            return
        }
        
        let lookup_size = self.lookup_table.len();

        let step = (LOOKUP_SIZE as f32 / (self.sample_rate / self.frequency)) as usize;

        for i in (0..output.len()).into_iter().step_by(2) {
            output[i]   = self.lookup_table[self.left_phase];
            output[i+1] = self.lookup_table[self.right_phase];
            self.left_phase += step;
            if self.left_phase >= lookup_size { self.left_phase -= lookup_size; }
            self.right_phase += step;
            if self.right_phase >= lookup_size { self.right_phase -= lookup_size }
        }
    }
    fn process_midi_input(&mut self, input: Vec<MidiEvent>) {
        for event in input {
            match event.message.status {
                // Note Off
                0x80 => {
                    self.frequency = 0.0;
                },
                // Note On
                0x90 => {
                    let note = event.message.data as f32;
                    self.frequency = 27.5 * 2f32.powf((note - 21.0)/12.0);
                },
                _ => println!("Midi Status Not Supported: {:x?}", event.message.status)
            }
        }
    }
    fn process_command_input(&mut self, command: &str, input: f32) {
        match command {
            "frequency" => self.frequency = input,
            _ => println!("Command Not Supported: {:x?}", input)
        }
    }
}