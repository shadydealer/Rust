use logger::Logger;

use std::io;
use std::time::Instant;

pub struct ScopedLogger<L: Logger> {
	tag: String,
	logger: L	
}

impl<L: Logger> ScopedLogger<L> {
	pub fn new(tag: &str, base_logger: L) -> Self {
		Self {
			tag: String::from(tag),
			logger: base_logger
		}
	}
}

impl<L: Logger> Logger for ScopedLogger<L> {
	fn push(&mut self, time: Instant, text: &str) {
		let mut tagging = String::from("[");
		tagging.push_str(&self.tag);
		tagging.push_str("] ");
		tagging.push_str(text);
		self.logger.push(time, &tagging);
	}

	fn try_flush(&mut self) -> io::Result<()>{
		self.logger.try_flush()?;
		Ok(())
	}
}