use crate::core::audio::{CommandSet, AudioProcessor};
use crate::core::midi::{MidiProcessor};
use crate::modules::wavetable::{WaveTable, WaveTableModule};
use crate::processors::portaudio::{PortAudioProcessor};
use crate::processors::portmidi::{PortMidiProcessor};
use crate::transport::{ConnectionFactory};

pub struct AudioThread;
pub struct TransportThread;
pub struct MidiThread;

impl AudioThread {
    pub fn init(channel_receiver: crossbeam_channel::Receiver<CommandSet>) -> AudioThread {
        std::thread::spawn(move || {
            let sine_wave = WaveTable::create_sine_wave();
            let wavetable_module = &mut WaveTableModule::new(sine_wave, 44_100.0);
            let audio_processor = &mut PortAudioProcessor::new(wavetable_module, 2, 44_100.0, 128);
            
            audio_processor.process_audio(
                || match channel_receiver.try_recv() {
                    Ok(command) => Some(command),
                    Err(err) => match err {
                        crossbeam_channel::TryRecvError::Empty => None,
                        crossbeam_channel::TryRecvError::Disconnected => {
                            println!("Communication Channel to audio stream has been disconnected");
                            None
                        }
                    }
                }
            );
        });

        AudioThread
    }
}

impl TransportThread {
    pub fn init(channel_sender: crossbeam_channel::Sender<CommandSet>) -> TransportThread {
        std::thread::spawn(move || {

            println!("WebSocket host listening on port 3012");

            let factory = ConnectionFactory::new(channel_sender);
            match ws::WebSocket::new(factory)
                .expect("Failed to build WebSocket")
                .listen("127.0.0.1:3012") {
                    Ok(_) => {},
                    Err(err) => print!("WebSocker Error: {:?}", err)
                };
        });

        TransportThread
    }
}

impl MidiThread {
    pub fn init(channel_sender: crossbeam_channel::Sender<CommandSet>) -> TransportThread {
        std::thread::spawn(move || {
            let midiprocessor = &mut PortMidiProcessor::new();

            midiprocessor.process_midi(|events| {
                match channel_sender.send(CommandSet::from_midi(events)) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Unable to send midi command: {:?}", e)
                }
            });
        });

        TransportThread
    }
}