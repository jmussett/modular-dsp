use crate::core::midi::{MidiProcessor, MidiEvent, MidiMessage};

pub struct PortMidiProcessor;

impl PortMidiProcessor {
    pub fn new() -> Self {
        PortMidiProcessor
    }
}

impl MidiProcessor for PortMidiProcessor {
    fn process_midi<C: Fn(Vec<MidiEvent>)>(&mut self, callback: C) {
        let run = || -> Result<(), portmidi::Error> {
            let pm = portmidi::PortMidi::new()?;

            let default_device_id = pm.default_input_device_id()?;
            let device = pm.device(default_device_id)?;

            println!("Default Midi Input Device: {:#?}", device.name());

            let midi_port = pm.input_port(device, 1024)?;

            while let Ok(_) = midi_port.poll() {
                if let Ok(Some(events)) = midi_port.read_n(1024) {
                    callback(events.into_iter().map(|e| MidiEvent {
                        message: MidiMessage {
                            status: e.message.status,
                            data: e.message.data1
                        }
                    }).collect());
                }
            }

            println!("Midi Input Device Disconnected");

            Ok(())
        };

        match run() {
            Ok(_) => {},
            Err(e) => eprintln!("PortMidi Error: {:?}", e)
        }
    }
}