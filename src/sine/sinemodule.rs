use audio::{AudioModule, Command};
use std::f64::consts::PI;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum SineParameter {
    Frequency
}

pub struct SineModule {
    left_phase: usize,
    right_phase: usize,
    sine: Vec<f32>,
    sample_rate: f32
}

impl SineModule {
    fn calculate_sine(frequency: f32, sample_rate: f32) -> Vec<f32> {
        let lookup_size = (sample_rate / frequency).round() as usize;
        let sine = &mut vec![0.0; lookup_size];

        for i in 0..lookup_size {
            sine[i] = (i as f64 / lookup_size as f64 * PI * 2.0).sin() as f32;
        }

        sine.to_vec()
    }
}

impl AudioModule for SineModule {
    fn new(sample_rate: f32) -> SineModule {
        SineModule {
            sine: SineModule::calculate_sine(500.0, sample_rate),
            left_phase: 0,
            right_phase: 0,
            sample_rate: sample_rate
        }
    }
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]) {
        let lookup_size = self.sine.len();
        for i in (0..input.len()).into_iter().step_by(2) {
            output[i]   = self.sine[self.left_phase];
            output[i+1] = self.sine[self.right_phase];
            self.left_phase += 1;
            if self.left_phase >= lookup_size { self.left_phase -= lookup_size; }
            self.right_phase += 1;
            if self.right_phase >= lookup_size { self.right_phase -= lookup_size }
        }
    }
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::SetParameter(id, value) => match SineParameter::from_usize(id).unwrap() {
                SineParameter::Frequency => {
                    self.sine = SineModule::calculate_sine(value, self.sample_rate);
                    self.left_phase = 0;
                    self.right_phase = 0;
                }
            },
        }
    }
}