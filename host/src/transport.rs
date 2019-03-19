use crate::core::audio::{CommandSet};

pub struct Connection {
    channel_sender: crossbeam_channel::Sender<CommandSet>,
    id: uuid::Uuid
}

impl Connection {
    fn new(channel_sender: crossbeam_channel::Sender<CommandSet>, id: uuid::Uuid) -> Connection {
        Connection {
            channel_sender: channel_sender,
            id: id
        }
    }
}

impl ws::Handler for Connection {
    fn on_message(&mut self, msg: ws::Message) -> Result<(), ws::Error> {
        if msg.is_text() {
            let text = msg.into_text().unwrap();
            println!("New Message {:?}, Id: {:?}", text, self.id);
            
            match serde_json::from_str(&text) {
                Ok(a) => self.channel_sender.send(a).unwrap(),
                Err(err) => eprintln!("Unable to deserialize message: {:?}", err)
            }
        } else {
           eprintln!("Message in incorrect format: {:?}", msg);
        }

        Ok(())
    }

    fn on_open(&mut self, _: ws::Handshake) -> Result<(), ws::Error> {
        println!("WebSocket connection oppened. Id: {:?}", self.id);
        Ok(())
    }
    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        println!("WebSocket connection closed. Id: {:?}", self.id);
    }
}

pub struct ConnectionFactory {
    channel_sender: crossbeam_channel::Sender<CommandSet>
}

impl ConnectionFactory {
    pub fn new(channel_sender: crossbeam_channel::Sender<CommandSet>) -> ConnectionFactory {
        ConnectionFactory {
            channel_sender: channel_sender
        }
    }
}

impl ws::Factory for ConnectionFactory {
    type Handler = Connection;
    fn connection_made(&mut self, _: ws::Sender) -> Connection {
        Connection::new(self.channel_sender.clone(), uuid::Uuid::new_v4())
    }
}
