use logger::Logger;

use std::io::{self, Write};
use std::time::Instant;

use std::rc::Rc;
use std::cell::{RefCell};

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
