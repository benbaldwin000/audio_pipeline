pub trait AudioGateway {
    fn init(&mut self) -> Result<(), String>;
	fn run(&mut self) -> Result<(), String>;
}

pub struct HttpAudioGateway {
	port: u32,
}

impl HttpAudioGateway {
	pub fn new(port: u32) -> Self {
		Self {
			port
		}
	}
}

impl AudioGateway for HttpAudioGateway {
    fn init(&mut self) -> Result<(), String> {
		Ok(())
    }

    fn run(&mut self) -> Result<(), String> {
        todo!()
    }
}