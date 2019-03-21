use crate::core::audio::{AudioData, AudioModule};
use std::f64::consts::PI;
use cgmath::prelude::*;
use cgmath::Rad;

const TABLE_SIZE: usize = 100000;

pub struct WaveTable;

impl WaveTable {
    pub fn create_wave_form<C: Fn(f64) -> f64>(calculate: C) -> Vec<f32>  {
        let wave_table = &mut vec![0.0; TABLE_SIZE];

        for i in 0..TABLE_SIZE {
            let x = i as f64 / TABLE_SIZE as f64;
            wave_table[i] = calculate(x) as f32;
        }

        wave_table.to_vec()
    }
    pub fn create_sine_wave(amplitude: f64, cycles: f64, phase: f64) -> Vec<f32> {
        WaveTable::create_wave_form(|x: f64| {
            // sine - A * sin(2PIcx + p)

            let angle = Rad(2.0 * PI * cycles * x + phase);
            amplitude * Rad::sin(angle)
        })
    }
    pub fn create_square_wave(amplitude: f64, cycles: f64, phase: f64) -> Vec<f32> {
        WaveTable::create_wave_form(|x: f64| {
            // square - A * sign(sin(2PIcx + p))

            let angle = Rad(2.0 * PI * cycles * x + phase);
            amplitude * Rad::sin(angle).signum()
        })
    }
    pub fn create_sawtooth_wave(amplitude: f64, cycles: f64, phase: f64) -> Vec<f32> {
        WaveTable::create_wave_form(|x: f64| {
            // sawtooth - -2A/PI * arctan(cot(PIcx + p/2))

            let angle = Rad(PI * cycles * x + phase/2.0);
            -2.0 * amplitude / PI * Rad::cot(angle).atan()
        })
    }
    pub fn create_triangle_wave(amplitude: f64, cycles: f64, phase: f64) -> Vec<f32> {
        WaveTable::create_wave_form(|x: f64| {
            // triangle - 2A/PI * arcsin(sin(2PIcx + p))

            let angle = Rad(2.0 * PI * cycles * x + phase);
            2.0 * amplitude / PI * Rad::sin(angle).asin()
        })
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
    fn process_audio(&mut self, data: AudioData) {
        for event in data.events.midi_events {
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

        for input_parameter in data.events.input_parameters {
            match input_parameter.0.as_ref() {
                "frequency" => self.frequency = *input_parameter.1,
                "square" => self.wave_table = WaveTable::create_square_wave(1.0, 1.0, 0.0),
                "sine" => self.wave_table = WaveTable::create_sine_wave(1.0, 1.0, 0.0),
                "sawtooth" => self.wave_table = WaveTable::create_sawtooth_wave(1.0, 1.0, 0.0),
                "triangle" => self.wave_table = WaveTable::create_triangle_wave(1.0, 1.0, 0.0),
                _ => println!("Command Not Supported: {:x?}", input_parameter.0)
            }
        }

        let lookup_size = self.wave_table.len();

        let step = (lookup_size as f32 / (self.sample_rate / self.frequency)) as usize;

        for i in (0..data.output.len()).into_iter().step_by(2) {
            data.output[i]   = self.wave_table[self.left_phase];
            data.output[i+1] = self.wave_table[self.right_phase];
            self.left_phase += step;
            if self.left_phase >= lookup_size { self.left_phase -= lookup_size; }
            self.right_phase += step;
            if self.right_phase >= lookup_size { self.right_phase -= lookup_size }
        }
    }
}