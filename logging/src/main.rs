use std::time::{Instant, Duration};
use std::io;
use std::fs::OpenOptions;

mod buffered_logger;
use buffered_logger::BufferedLogger;

mod multi_logger;
use multi_logger::MultiLogger;

mod scoped_logger;
use scoped_logger::ScopedLogger;

//because logger is a sibling module of buffered_logger
mod logger;
use logger::Logger;

fn main() {
  unimplemented!();
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