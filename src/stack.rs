#[derive(Default, Debug)]
pub struct Stack<T> {
  list: Vec<T>,
  size: usize,
}

impl<T: std::clone::Clone> Stack<T> {
  pub fn new() -> Self {
    Stack {
      list: Vec::new(),
      size: 0,
    }
  }

  pub fn from(values: &mut [T]) -> Stack<T> {
    Stack {
      list: values.to_vec(),
      size: values.len(),
    }
  }

  pub fn push(&mut self, value: T) {
    self.list.push(value);
    self.size += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    let result = self.list.pop();

    match result {
      Some(value) => {
        self.size -= 1;
        Some(value)
      }
      None => None
    }
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn peek(&self) -> Option<&T> {
    match self.size {
      0 => None,
      _ => self.list.last(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::Stack;

  #[test]
  fn basics_new() {
    let mut stack: Stack<i32> = Stack::new();

    assert_eq!(stack.size(), 0);
    assert_eq!(stack.peek(), None);
    assert_eq!(stack.pop(), None);

    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&3));
    assert_eq!(stack.pop(), Some(3));

    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&2));
    assert_eq!(stack.pop(), Some(2));

    stack.push(4);
    stack.push(5);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&5));
    assert_eq!(stack.pop(), Some(5));
  }

  #[test]
  fn basics_from() {
    let mut stack: Stack<i32> = Stack::from(&mut [1, 2, 3]);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&3));
    assert_eq!(stack.pop(), Some(3));

    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&2));
    assert_eq!(stack.pop(), Some(2));

    stack.push(4);
    stack.push(5);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&5));
    assert_eq!(stack.pop(), Some(5));
  }
}

