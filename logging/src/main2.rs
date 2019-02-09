use std::time::{Instant, Duration};
use std::io::{self, Write};
use std::rc::Rc;
use std::cell::{RefCell};

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

pub struct BufferedLogger<W: Write> {
  out: Rc<RefCell<W>>,
  buffer_size: usize,
  data: Rc<RefCell<Vec<(Instant, String)>>>
}

impl<W: Write> BufferedLogger<W> {

  pub fn new(out: W, buffer_size: usize) -> Self {

    Self{
      out: Rc::new(RefCell::new(out)),
      buffer_size: buffer_size,
      data: Rc::new(RefCell::new(Vec::with_capacity(buffer_size)))
    }

  }

  pub fn buffered_entries(&self) -> Vec<String> {
    self.data.borrow().iter().map(|e| (e.1).clone()).collect()
  }
}

impl<W: Write> Clone for BufferedLogger<W> {

  fn clone(&self) -> Self {

    Self {
      out: self.out.clone(),
      buffer_size: self.buffer_size,
      data: self.data.clone()
    }

  }
}

impl<W: Write> Logger for BufferedLogger<W> {

  fn push(&mut self, time: Instant, text: &str) {

    let mut index: usize = 0;

    for entry in self.data.borrow().iter() {
      if entry.0 >= time { break; }
      index+=1;
    }

    self.data.borrow_mut().insert(index, (time, String::from(text)));

    if self.data.borrow().len() == self.buffer_size { self.flush() };

  }

  fn try_flush(&mut self) -> io::Result<()>{

    let mut data = self.data.borrow_mut();

    // so we pop them in the correct order
    // since they're sorted in ascending order rn
    data.reverse();

    let mut out = self.out.borrow_mut();

    while !data.is_empty() {

      let mut entry = (data.last().unwrap().1).clone();
      entry.push('\n');

      out.write(entry.as_bytes())?;

      data.pop();
    }

    Ok(())
  }
}

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

#[test]
fn test_buffered_logger_keeps_entries_sorted() {
  let mut logger = BufferedLogger::new(Vec::new(), 100);
  let now = Instant::now();

  logger.push(now + Duration::from_millis(2), "Test2");
  logger.push(now + Duration::from_millis(1), "Test1");

  assert_eq!(logger.buffered_entries(), vec!["Test1", "Test2"]);
}

#[test]
fn test_buffered_logger_clone_has_same_buffered_entries() {
  let mut logger = BufferedLogger::new(Vec::new(), 100);
  let mut cloned_logger = logger.clone();

  logger.log("Test");
  assert_eq!(logger.buffered_entries(), cloned_logger.buffered_entries());

  cloned_logger.log("I'm going mad");
  assert_eq!(logger.buffered_entries(), cloned_logger.buffered_entries());

  logger.flush();
  assert_eq!(logger.buffered_entries().len(), 0);
  assert_eq!(cloned_logger.buffered_entries().len(), 0);
}

#[test]
fn test_buffered_logger_flush_removes_entries() {
  let mut logger = BufferedLogger::new(Vec::new(), 100);
  logger.log("test");

  logger.flush();
  assert_eq!(logger.buffered_entries().len(), 0);
}

#[test]
fn test_buffered_logger_auto_flushes_on_entry_overload() {

  let mut logger = BufferedLogger::new(io::stdout(), 3);
  logger.log("Test");
  logger.log("Test");
  logger.log("Test");
  assert_eq!(logger.buffered_entries().len(), 0);

}

#[test]
fn test_buffered_logger_try_flush_returns_ok_on_successful_write() {

  let mut logger = BufferedLogger::new(io::stdout(), 1);
  logger.log("Test");
  
  let result = logger.try_flush();
  assert!(result.is_ok());
}

use std::fs::OpenOptions;

#[test]
fn test_buffered_logger_try_flush_returns_err_on_failed_write() {
  let _creating = OpenOptions::new().write(true).create(true).open("test.txt");
  let file = OpenOptions::new().read(true).open("test.txt").unwrap();

  let mut logger = BufferedLogger::new(file, 3);

  logger.log("Test1");

  let result = logger.try_flush();
  assert!(result.is_err());
}

#[test]
fn test_multi_logger() {

    let logger1 = BufferedLogger::new(Vec::new(), 100);
    let logger2 = BufferedLogger::new(io::stdout(), 100);
    let now = Instant::now();

    let mut logger = MultiLogger::new();
    logger.log_to(logger1.clone());
    logger.push(now + Duration::from_millis(1), "Test1");

    logger.log_to(ScopedLogger::new("Second", logger2.clone()));
    logger.push(now + Duration::from_millis(2), "Test2");
    logger.push(now + Duration::from_millis(3), "Test3");

    assert_eq!(logger1.buffered_entries(), vec!["Test1", "Test2", "Test3"]);

    assert_eq!(logger2.buffered_entries(), vec!["[Second] Test2", "[Second] Test3"]);

    logger.try_flush().unwrap();
    logger.flush();
}

#[test]
fn test_scoped_logger() {

    let base = BufferedLogger::new(Vec::new(), 100);
    let mut logger = ScopedLogger::new("Rust", ScopedLogger::new("FMI", base.clone()));
    logger.log("Test");

    assert_eq!(base.buffered_entries(), vec!["[FMI] [Rust] Test"]);
}