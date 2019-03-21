use crate::core::audio::{CommandSet};

pub struct CliProcessor;

impl CliProcessor {
    pub fn listen(command_sender: crossbeam_channel::Sender<CommandSet>) -> Result<(), std::io::Error>{;
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim() {
                "sine" => command_sender.send(CommandSet::from_input("sine".to_owned(), 0.0)).unwrap(),
                "square" => command_sender.send(CommandSet::from_input("square".to_owned(), 0.0)).unwrap(),
                "sawtooth" => command_sender.send(CommandSet::from_input("sawtooth".to_owned(), 0.0)).unwrap(),
                "triangle" => command_sender.send(CommandSet::from_input("triangle".to_owned(), 0.0)).unwrap(),
                number => match number.parse::<f32>() {
                    Ok(frequency) => {
                        command_sender.send(CommandSet::from_input("frequency".to_owned(), frequency)).unwrap();
                    }
                    Err(_) => eprintln!("{:?} was not a number", input.trim())
                }
            }
        }
    }
}