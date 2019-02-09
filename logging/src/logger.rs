use std::time::Instant;
use std::io;

pub trait Logger {

	fn push(&mut self, time: Instant, text: &str);

	fn log(&mut self, text: &str){
    self.push(Instant::now(), text);
  }

	fn try_flush(&mut self) -> io::Result<()>;

	fn flush(&mut self){
		match self.try_flush() {
			Err(error) => eprintln!("{}", error),
			Ok(_) => ()
		}
	}
}