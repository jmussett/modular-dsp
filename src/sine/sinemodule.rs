use audio::{AudioModule, Command};
use std::f64::consts::PI;
use num_traits::FromPrimitive;

const LOOKUP_SIZE: usize = 1000;

#[derive(FromPrimitive)]
pub enum SineParameter {
    Frequency
}

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
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]) {
        if self.frequency == 0.0 {
            for i in 0..input.len()
            {
                output[i] = 0.0;
            }
            return
        }
        
        let lookup_size = self.lookup_table.len();

        let step = (LOOKUP_SIZE as f32 / (self.sample_rate / self.frequency)) as usize;

        for i in (0..input.len()).into_iter().step_by(2) {
            output[i]   = self.lookup_table[self.left_phase];
            output[i+1] = self.lookup_table[self.right_phase];
            self.left_phase += step;
            if self.left_phase >= lookup_size { self.left_phase -= lookup_size; }
            self.right_phase += step;
            if self.right_phase >= lookup_size { self.right_phase -= lookup_size }
        }
    }
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::SetParameter(id, frequency) => match SineParameter::from_usize(id).unwrap() {
                SineParameter::Frequency => {
                    self.frequency = frequency;
                }
            },
        }
    }
}