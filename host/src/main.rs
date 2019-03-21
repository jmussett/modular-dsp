extern crate portaudio;
extern crate portmidi;
extern crate crossbeam_channel;
extern crate ws;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate cgmath;

pub mod core;
pub mod modules;
pub mod processors;
pub mod threads;
pub mod transport;
pub mod input;

use crate::threads::{AudioThread, TransportThread, MidiThread};
use crate::input::{CliProcessor};

fn main() {
    let (channel_sender, channel_receiver) = crossbeam_channel::bounded(1024);

    MidiThread::init(channel_sender.clone());
    TransportThread::init(channel_sender.clone());
    AudioThread::init(channel_receiver);

    match CliProcessor::listen(channel_sender) {
        Ok(_) => {}
        Err(e) => eprintln!("Input Error: {:?}", e)
    }
}