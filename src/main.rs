#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate portaudio;
extern crate crossbeam_channel;
extern crate audio_thread_priority;

mod sine;
mod audio;

use audio::{AudioModule, Command};
use sine::{SineModule, SineParameter};
use portaudio::{error::Error, PortAudio};

const CHANNELS: usize = 2;
const SAMPLE_RATE: f32 = 44_100.0;
const FRAMES_PER_BUFFER: usize = 128;
const SAMPLES_PER_BUFFER: usize = FRAMES_PER_BUFFER * CHANNELS;

fn main() {
    let (command_sender, command_receiver) = crossbeam_channel::bounded(1024);

    std::thread::spawn(move || {
        match play_audio::<SineModule>(command_receiver) {
            Ok(_) => {},
            e => eprintln!("Error: {:?}", e)
        }
    });

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let frequency = input.trim().parse::<f32>().unwrap();
                command_sender.send(Command::SetParameter(SineParameter::Frequency as usize, frequency)).unwrap()
            },
            Err(e) => eprintln!("Error: {:?}", e)
        }
    }
}

fn play_audio<Module: AudioModule>(
    command_receiver: crossbeam_channel::Receiver<Command>
) -> Result<(), Error> {
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
    let audio_module = &mut Module::new(SAMPLE_RATE);

    audio_thread_priority::promote_current_thread_to_real_time(
        SAMPLES_PER_BUFFER as u32,
        SAMPLE_RATE as u32,
    ).unwrap();

    loop {
        while let Ok(command) = command_receiver.try_recv() {
            audio_module.handle_command(command);
        }

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