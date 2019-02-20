extern crate crossbeam_channel;
extern crate modulardsp;

mod portaudio;
mod portmidi;

use modulardsp::audio::{Command, AudioProcessor};
use modulardsp::midi::{MidiProcessor};
use modulardsp::modules::wavetable::{WaveTable, WaveTableModule};
use crossbeam_channel::Sender;
use crate::portaudio::{PortAudioProcessor};
use crate::portmidi::{PortMidiProcessor};

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