mod audiomodule;
pub use self::audiomodule::{AudioModule, InputBuffer, OutputBuffer};

mod audioprocessor;
pub use self::audioprocessor::{AudioProcessor, Command};