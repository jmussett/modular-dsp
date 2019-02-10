pub enum Command {
    SetParameter(usize, f32)
}

pub trait AudioModule {
    fn new(f32) -> Self;
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]);
    fn handle_command(&mut self, command: Command);
}