extern crate crossbeam_channel;

mod midi;
mod audio;
mod wavetablemodule;
mod portaudioprocessor;
mod portmidiprocessor;

use audio::{Command, AudioProcessor};
use midi::{MidiProcessor};
use wavetablemodule::{WaveTable, WaveTableModule};
use portaudioprocessor::{PortAudioProcessor};
use portmidiprocessor::{PortMidiProcessor};
use crossbeam_channel::Sender;

fn main() {
    let (command_sender, command_receiver) = crossbeam_channel::bounded(1024);
    
    std::thread::spawn(move || {
        let sine_wave = WaveTable::create_sine_wave();
        let wavetable_module = &mut WaveTableModule::new(sine_wave, 44_100.0);
        let audio_processor = &mut PortAudioProcessor::new(wavetable_module, 2, 44_100.0, 128);
        
        audio_processor.process_audio(
            || match command_receiver.try_recv() {
                Ok(command) => Some(command),
                Err(err) => match err {
                    crossbeam_channel::TryRecvError::Empty => None,
                    crossbeam_channel::TryRecvError::Disconnected => {
                        println!("Communication Channel to audio stream has been disconnected");
                        None
                    }
                }
            },
        );
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
        if input.trim() == "sine" {
            command_sender.send(Command::SendCommandInput("sine", 0.0)).unwrap()
        } else if input.trim() == "square" {
            command_sender.send(Command::SendCommandInput("square", 0.0)).unwrap()
        } else {
            match input.trim().parse::<f32>() {
                Ok(frequency) => {
                    command_sender.send(Command::SendCommandInput("frequency", frequency)).unwrap();
                }
                Err(_) => eprintln!("{:?} was not a number", input.trim())
            }
        }
    }
}