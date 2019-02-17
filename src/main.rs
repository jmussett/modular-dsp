extern crate portaudio;
extern crate crossbeam_channel;

mod midi;
mod audio;
mod sinemodule;
mod portmidiprocessor;

use audio::{AudioModule};
use midi::{MidiProcessor, MidiEvent};
use sinemodule::{SineModule};
use portaudio::{PortAudio};
use portmidiprocessor::{PortMidiProcessor};
use crossbeam_channel::Sender;

const CHANNELS: usize = 2;
const SAMPLE_RATE: f32 = 44_100.0;
const FRAMES_PER_BUFFER: usize = 128;
const SAMPLES_PER_BUFFER: usize = FRAMES_PER_BUFFER * CHANNELS;

pub enum Command<'a> {
    SendMidiEvents(Vec<MidiEvent>),
    SendCommandInput(&'a str, f32)
}

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
        let midiprocessor = &mut PortMidiProcessor::new();

        midiprocessor.process_midi(|events| {
            match midi_sender.send(Command::SendMidiEvents(events)) {
                Ok(_) => {},
                Err(e) => eprintln!("Unable to send midi command: {:?}", e)
            }
        });
    });

    match process_inputs(command_sender) {
        Ok(_) => {},
        Err(e) => eprintln!("Input Error: {:?}", e)
    }
}

fn process_inputs(command_sender: Sender<Command>) -> Result<(), std::io::Error> {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim().parse::<f32>() {
            Ok(frequency) => {
                command_sender.send(Command::SendCommandInput("frequency", frequency)).unwrap();
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

    let audio_module = &mut Module::new(SAMPLE_RATE);

    loop {
        while let Ok(command) = command_receiver.try_recv() {
            match command {
                Command::SendMidiEvents(midi_events) 
                    => audio_module.process_midi_input(midi_events),
                Command::SendCommandInput(command, input)
                    => audio_module.process_command_input(command, input)
            }
        }

        match stream.read(FRAMES_PER_BUFFER as u32) {
            Err(portaudio::Error::InputOverflowed) => println!("Input underflowed"),
            Err(err) => println!("Read from stream failed - {:?}", err),
            Ok(input) => {
                assert_eq!(input.len(), SAMPLES_PER_BUFFER);
                audio_module.process_audio_input(input);
            }
        }

        match stream.write(FRAMES_PER_BUFFER as u32, |output| {
            assert_eq!(output.len(), SAMPLES_PER_BUFFER);
            audio_module.process_audio_output(output);
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