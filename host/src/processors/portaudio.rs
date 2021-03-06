use std::collections::{HashMap};
use crate::core::audio::{Events, AudioData, AudioModule, AudioProcessor, CommandSet, Command};

pub struct PortAudioProcessor<'a> {
    channels: usize,
    sample_rate: f32,
    frames_per_buffer: usize,
    samples_per_buffer: usize,
    audio_module: &'a mut AudioModule,
}

impl<'a> PortAudioProcessor<'a> {
    pub fn new(audio_module: &'a mut AudioModule, channels: usize, sample_rate: f32, frames_per_buffer: usize) -> Self {
        PortAudioProcessor {
            channels: channels,
            sample_rate: sample_rate,
            frames_per_buffer: frames_per_buffer,
            samples_per_buffer: frames_per_buffer * channels,
            audio_module: audio_module
        }
    }
}

impl<'a> AudioProcessor for PortAudioProcessor<'a> {
    fn process_audio<RC: Fn() -> Option<CommandSet>>(&mut self, receive_commands: RC) {
        let run = &mut || -> Result<(), portaudio::Error> {
            let pa = portaudio::PortAudio::new()?;

            log_host(&pa)?;
            log_devices(&pa)?;

            let settings = pa.default_duplex_stream_settings::<f32, f32>(
                self.channels as i32,
                self.channels as i32,
                self.sample_rate as f64,
                self.frames_per_buffer as u32,
            )?;

            let mut stream = pa.open_blocking_stream(settings)?;
            stream.start()?;

            let mut input = Vec::with_capacity(self.samples_per_buffer);
            for _ in 0..self.samples_per_buffer {
                input.push(0.0f32);
            }

            match audio_thread_priority::promote_current_thread_to_real_time(512, 44100) {
                Ok(_) => {}
                Err(()) => println!("Could not run the audio in real time")
            }

            loop {
                let mut midi_events = Vec::new();
                let mut input_parameters = HashMap::new();

                if let Some(command_set) = receive_commands() {
                    for command in command_set.commands {
                        match command {
                            Command::MidiEvent(event) => {
                                midi_events.push(event);
                            },
                            Command::InputParameter(key, value) => {
                                input_parameters.insert(key, value);
                            }
                        }
                    }
                }

                match stream.read(self.frames_per_buffer as u32) {
                    Err(portaudio::Error::InputOverflowed) => println!("Input underflowed"),
                    Err(err) => println!("Read from stream failed - {:?}", err),
                    Ok(input_buffer) => {
                        assert_eq!(input_buffer.len(), self.samples_per_buffer, "Input buffer has incorrect length");
                        input.copy_from_slice(input_buffer);
                    }
                }

                match stream.write(self.frames_per_buffer as u32, |output| {
                    assert_eq!(output.len(), self.samples_per_buffer, "Output buffer has incorrect length");

                    let data = AudioData {
                        input: &input,
                        output: output,
                        events: Events {
                            midi_events: &midi_events,
                            input_parameters: &input_parameters
                        }
                    };

                    self.audio_module.process_audio(data);
                }) {
                    Err(portaudio::Error::OutputUnderflowed) => println!("Output underflowed"),
                    Err(err) => println!("Write to stream failed - {:?}", err),
                    _ => (),
                };
            }
        };

        match run() {
            Ok(_) => {},
            Err(e) => eprintln!("PortAudio Error: {:?}", e)
        };
    }
}

fn log_host(pa: &portaudio::PortAudio) -> Result<(), portaudio::Error>  {
    let host_index = pa.default_host_api()?;
    let default_host = pa.host_apis().find(|host| host.0 == host_index)
        .expect("Default Host does not exist").1;

    println!("Default Audio Host API: {:#?}", default_host.name);

    Ok(())
}

fn log_devices(pa: &portaudio::PortAudio) -> Result<(), portaudio::Error> {
    let input_device_index = pa.default_input_device()?;
    let output_device_index = pa.default_output_device()?;

    let input_device = pa.devices()?.find(|device| device.as_ref().unwrap().0 == input_device_index)
        .expect("Default Input Device does not exist")?.1;

    let output_device = pa.devices()?.find(|device| device.as_ref().unwrap().0 == output_device_index)
        .expect("Default Input Device does not exist")?.1;

    println!("Default Audio Input Device: {:#?}", input_device.name);
    println!("Default Audio Output Device: {:#?}", output_device.name);

    Ok(())
}