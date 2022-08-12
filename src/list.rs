type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  value: T,
  next: Link<T>,
}

pub struct List<T> {
  head: Link<T>,
  size: usize,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List {
      head: None,
      size: 0,
    }
  }

  pub fn push(&mut self, value: T) {
    let node = Box::new(Node {
      value,
      next: self.head.take(),
    });

    self.head = Some(node);
    self.size += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      self.head = node.next;
      self.size -= 1;
      node.value
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| {
      &node.value
    })
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| {
      &mut node.value
    })
  }

  pub fn size(&self) -> usize {
    self.size
  }
  
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  pub fn iter(&self) -> Iter<T> {
    Iter { next: self.head.as_deref() }
  }

  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut { next: self.head.as_deref_mut() }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut temp = self.head.take();
    while let Some(mut boxed_node) = temp {
      temp = boxed_node.next.take();
    }
  }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_deref();
      &node.value
    })
  }
}

pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      self.next = node.next.as_deref_mut();
      &mut node.value
    })
  }
}

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
    let mut list = List::new();

    assert_eq!(list.pop(), None);
    assert_eq!(list.size(), 0);

    list.push(1);
    list.push(2);
    list.push(3);
    
    assert_eq!(list.size(), 3);
    
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    assert_eq!(list.size(), 1);

    list.push(4);
    list.push(5);

    assert_eq!(list.size(), 3);

    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    assert_eq!(list.size(), 1);

    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);

    assert_eq!(list.size(), 0);
  }
  
  #[test]
  fn peek() {
    let mut list = List::new();

    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|value| {
      *value = 42
    });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
  }

  #[test]
  fn into_iter() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();

    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
  }

  #[test]
  fn iter_mut() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
  }
}

