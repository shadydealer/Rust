use logger::Logger;
use std::io;
use std::time::Instant;

pub struct MultiLogger {
  loggers: Vec<Box<dyn Logger>>
}

impl MultiLogger {
    pub fn new() -> Self {
        Self {
        	loggers: Vec::new()
        }
    }

    pub fn log_to<L: Logger + 'static>(&mut self, logger: L) {
        self.loggers.push(Box::new(logger));
    }
}

impl Logger for MultiLogger {
	
	fn push(&mut self, time: Instant, text: &str) {
    for logger in &mut self.loggers {
    	logger.push(time, text);
    }
  }

	fn try_flush(&mut self) -> io::Result<()>{

		for logger in &mut self.loggers {
			logger.try_flush()?;
		}
		Ok(())
	}
}