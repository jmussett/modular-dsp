use midi::MidiEvent;
use audio::{AudioModule, InputBuffer, OutputBuffer};
use std::f64::consts::PI;

pub struct WaveTable;

impl WaveTable {
    pub fn create_sine_wave() -> Vec<f32> {
        let wave_table = &mut vec![0.0; 10000];

        for i in 0..10000 {
            wave_table[i] = (i as f64 / 10000 as f64 * PI * 2.0).sin() as f32;
        }

        println!("{:?}", wave_table[7500]);

        wave_table.to_vec()
    }
    pub fn create_square_wave() -> Vec<f32> {
        let wave_table = &mut vec![0.0; 10000];

        for i in 0..10000 {
            if i < 5000 {
                wave_table[i] = 1.0;
            } else {
                wave_table[i] = -1.0;
            }
        }

        wave_table.to_vec()
    }
}

pub struct WaveTableModule {
    left_phase: usize,
    right_phase: usize,
    frequency: f32,
    wave_table: Vec<f32>,
    sample_rate: f32
}

impl WaveTableModule {
    pub fn new(wave_table: Vec<f32>, sample_rate: f32) -> Self {
        WaveTableModule {
            wave_table: wave_table.to_vec(),
            frequency: 0.0,
            left_phase: 0,
            right_phase: 0,
            sample_rate: sample_rate
        }
    }
}

impl AudioModule for WaveTableModule {
    fn process_audio_input(&mut self, _input: InputBuffer) {}
    fn process_audio_output(&mut self, output: OutputBuffer) {
        let lookup_size = self.wave_table.len();

        let step = (lookup_size as f32 / (self.sample_rate / self.frequency)) as usize;

        for i in (0..output.len()).into_iter().step_by(2) {
            output[i]   = self.wave_table[self.left_phase];
            output[i+1] = self.wave_table[self.right_phase];
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
            "square" => self.wave_table = WaveTable::create_square_wave(),
            "sine" => self.wave_table = WaveTable::create_sine_wave(),
            _ => println!("Command Not Supported: {:x?}", input)
        }
    }
}