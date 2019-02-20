extern crate portaudio;

use portaudio::{PortAudio};
use modulardsp::audio::{AudioModule, AudioProcessor, Command};

pub struct PortAudioProcessor<'a> {
    channels: usize,
    sample_rate: f32,
    frames_per_buffer: usize,
    audio_module: &'a mut AudioModule,
}

impl<'a> PortAudioProcessor<'a> {
    pub fn new(audio_module: &'a mut AudioModule, channels: usize, sample_rate: f32, frames_per_buffer: usize) -> Self {
        PortAudioProcessor {
            channels: channels,
            sample_rate: sample_rate,
            frames_per_buffer: frames_per_buffer,
            audio_module: audio_module
        }
    }
}

impl<'a> AudioProcessor<'a> for PortAudioProcessor<'a> {
    fn process_audio<RC: Fn() -> Option<Command<'a>>>(&mut self, receive_command: RC) {
        let run = &mut || -> Result<(), portaudio::Error> {
            let pa = PortAudio::new()?;

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

            loop {
                while let Some(command) = receive_command() {
                    match command {
                        Command::SendMidiEvents(midi_events) 
                            => self.audio_module.process_midi_input(midi_events),
                        Command::SendCommandInput(command, input)
                            => self.audio_module.process_command_input(command, input)
                    }
                }

                match stream.read(self.frames_per_buffer as u32) {
                    Err(portaudio::Error::InputOverflowed) => println!("Input underflowed"),
                    Err(err) => println!("Read from stream failed - {:?}", err),
                    Ok(input) => {
                        self.audio_module.process_audio_input(input);
                    }
                }

                match stream.write(self.frames_per_buffer as u32, |output| {
                    self.audio_module.process_audio_output(output)
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

fn log_host(pa: &PortAudio) -> Result<(), portaudio::Error>  {
    let host_index = pa.default_host_api()?;
    let default_host = pa.host_apis().find(|host| host.0 == host_index)
        .expect("Default Host does not exist").1;

    println!("Default Audio Host API: {:#?}", default_host.name);

    Ok(())
}

fn log_devices(pa: &PortAudio) -> Result<(), portaudio::Error> {
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