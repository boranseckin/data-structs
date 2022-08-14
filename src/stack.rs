#[derive(Default, Debug)]
pub struct Stack {
  list: Vec<i32>,
  size: usize,
}

impl Stack {
  pub fn new() -> Stack {
    Stack {
      list: vec![],
      size: 0,
    }
  }

  pub fn from(values: &Vec<i32>) -> Stack {
    Stack {
      list: values.clone(),
      size: values.len(),
    }
  }

  pub fn push(&mut self, value: i32) {
    self.list.push(value);
    self.size += 1;
  }

  pub fn pop(&mut self) -> Option<i32> {
    let result = self.list.pop();

    match result {
      Some(value) => {
        self.size -= 1;
        Some(value)
      }
      None => None
    }
  }

  pub fn size(&mut self) -> usize {
    self.size
  }

  pub fn peek(&mut self) -> Option<i32> {
    match self.size {
      0 => None,
      _ => self.list.last().copied(),
    }
  }
}

