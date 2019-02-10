extern crate portaudio;

use portaudio::{error::Error, PortAudio};
use std::f64::consts::PI;

const CHANNELS: usize = 2;
const SAMPLE_RATE: f32 = 44_100.0;
const FRAMES_PER_BUFFER: usize = 128;
const SAMPLES_PER_BUFFER: usize = FRAMES_PER_BUFFER * CHANNELS;
const SINE_FREQUENCY: f32 = 1000.0;

trait AudioModule {
    fn new() -> Self;
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]);
}

pub struct SineModule {
    left_phase: usize,
    right_phase: usize,
    sine: Vec<f32>,
    lookup_size: usize
}

impl AudioModule for SineModule {
    fn new() -> SineModule {
        let lookup_size = (SAMPLE_RATE / SINE_FREQUENCY).round() as usize;
        let sine = &mut vec![0.0; lookup_size];

        for i in 0..lookup_size {
            sine[i] = (i as f64 / lookup_size as f64 * PI * 2.0).sin() as f32;
        }

        SineModule {
            sine: sine.to_vec(),
            lookup_size: lookup_size,
            left_phase: 0,
            right_phase: 0
        }
    }
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]) {
        for i in (0..input.len()).into_iter().step_by(2) {
            output[i]   = self.sine[self.left_phase];
            output[i+1] = self.sine[self.right_phase];
            self.left_phase += 1;
            if self.left_phase >= self.lookup_size { self.left_phase -= self.lookup_size; }
            self.right_phase += 1;
            if self.right_phase >= self.lookup_size { self.right_phase -= self.lookup_size; }
        }
    }
}

fn main() {
    match play_audio::<SineModule>() {
        Ok(_) => {},
        e => eprintln!("Example failed with the following: {:?}", e)
    }
}

fn play_audio<Module: AudioModule>() -> Result<(), Error> {
    let pa = PortAudio::new()?;

    log_host(&pa)?;
    log_devices(&pa)?;

    let settings = pa.default_duplex_stream_settings::<f32, f32>(
            CHANNELS as i32,
            CHANNELS as i32,
            SAMPLE_RATE as f64,
            FRAMES_PER_BUFFER as u32,
    )?;

    let mut stream = pa.open_blocking_stream(settings)?;
    stream.start()?;

    let mut input_buffer = [0.0f32; SAMPLES_PER_BUFFER];
    let audio_module = &mut Module::new();

    loop {
        match stream.read(FRAMES_PER_BUFFER as u32) {
            Err(Error::InputOverflowed) => println!("Input underflowed"),
            Err(err) => println!("Read from stream failed - {:?}", err),
            Ok(input) => {
                assert_eq!(input.len(), SAMPLES_PER_BUFFER);
                input_buffer.copy_from_slice(input);
            }
        }

        match stream.write(FRAMES_PER_BUFFER as u32, |output| {
            assert_eq!(output.len(), SAMPLES_PER_BUFFER);
            audio_module.process_stereo(&input_buffer[0..SAMPLES_PER_BUFFER], output);
        }) {
            Err(Error::OutputUnderflowed) => println!("Output underflowed"),
            Err(err) => println!("Write to stream failed - {:?}", err),
            _ => (),
        };
    }
}

fn log_host(pa: &PortAudio) -> Result<(), Error>  {
    let host_index = pa.default_host_api()?;
    let default_host = pa.host_apis().find(|host| host.0 == host_index)
        .expect("Default Host does not exist").1;

    println!("Default Audio Host API: {:#?}", default_host.name);

    Ok(())
}

fn log_devices(pa: &PortAudio) -> Result<(), Error> {
    let input_device_index = pa.default_input_device()?;
    let output_device_index = pa.default_output_device()?;

    let input_device = pa.devices()?.find(|device| device.as_ref().unwrap().0 == input_device_index)
        .expect("Default Input Device does not exist")?.1;

    let output_device = pa.devices()?.find(|device| device.as_ref().unwrap().0 == output_device_index)
        .expect("Default Input Device does not exist")?.1;

    println!("Default Input Device: {:#?}", input_device.name);
    println!("Default Output Device: {:#?}", output_device.name);

    Ok(())
}