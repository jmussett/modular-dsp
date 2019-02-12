#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate portaudio;
extern crate portmidi;
extern crate crossbeam_channel;

mod sine;
mod audio;

use audio::{AudioModule, Command};
use sine::{SineModule, SineParameter};
use portaudio::{PortAudio};
use portmidi::{PortMidi};
use crossbeam_channel::Sender;

const CHANNELS: usize = 2;
const SAMPLE_RATE: f32 = 44_100.0;
const FRAMES_PER_BUFFER: usize = 128;
const SAMPLES_PER_BUFFER: usize = FRAMES_PER_BUFFER * CHANNELS;

fn main() {
    let (command_sender, command_receiver) = crossbeam_channel::bounded(1024);
    
    std::thread::spawn(move || {
        match play_audio::<SineModule>(command_receiver) {
            Ok(_) => {},
            Err(e) => eprintln!("Audio Error: {:?}", e)
        }
    });

    let midi_sender = command_sender.clone();

    std::thread::spawn(move || {
        match process_midi(midi_sender) {
            Ok(_) => {},
            Err(e) => eprintln!("Midi Error: {:?}", e)
        }
    });

    match process_inputs(command_sender) {
        Ok(_) => {},
        Err(e) => eprintln!("Input Error: {:?}", e)
    }
}

fn process_midi(command_sender: Sender<Command>) -> Result<(), portmidi::Error> {
    let pm = PortMidi::new()?;

    let default_device_id = pm.default_input_device_id()?;
    let device = pm.device(default_device_id)?;

    println!("Default Midi Input Device: {:#?}", device.name());

    let midi_port = pm.input_port(device, 1024)?;

    while let Ok(_) = midi_port.poll() {
        if let Ok(Some(events)) = midi_port.read_n(1024) {
           for event in events {
                match event.message.status {
                    // Note Off
                    0x80 => {
                        command_sender.send(Command::SetParameter(SineParameter::Frequency as usize, 0.0)).unwrap();
                    },
                    // Note On
                    0x90 => {
                        let note = event.message.data1 as f32;
                        let frequency = 27.5 * 2f32.powf((note - 21.0)/12.0);

                        println!("Note: {:#?}, Frequency: {:#?}", note, frequency);

                        command_sender.send(Command::SetParameter(SineParameter::Frequency as usize, frequency)).unwrap();
                    },
                    _ => println!("Midi Status Not Supported: {:x?}", event.message.status)
                }
            }
        }
    }

    println!("Midi Input Device Disconnected");

    Ok(())
}

fn process_inputs(command_sender: Sender<Command>) -> Result<(), std::io::Error> {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim().parse::<f32>() {
            Ok(frequency) => {
                command_sender.send(Command::SetParameter(SineParameter::Frequency as usize, frequency)).unwrap();
            }
            Err(_) => eprintln!("{:?} was not a number", input.trim())
        }
    }
}

fn play_audio<Module: AudioModule>(
    command_receiver: crossbeam_channel::Receiver<Command>
) -> Result<(), portaudio::Error> {
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

    loop {
        while let Ok(command) = command_receiver.try_recv() {
            audio_module.handle_command(command);
        }

        match stream.read(FRAMES_PER_BUFFER as u32) {
            Err(portaudio::Error::InputOverflowed) => println!("Input underflowed"),
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
            Err(portaudio::Error::OutputUnderflowed) => println!("Output underflowed"),
            Err(err) => println!("Write to stream failed - {:?}", err),
            _ => (),
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