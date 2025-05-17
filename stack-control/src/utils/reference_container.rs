pub struct ReferencesContainer<T: ?Sized> {
  list: Vec<Box<T>>
}

impl<T: ?Sized> ReferencesContainer<T> {
  pub fn new() -> Self {
    ReferencesContainer { list: Vec::new() }
  }

  pub fn make_reference(&mut self, fabric: impl Fn() -> Box<T>) -> &T {
    self.list.push(fabric());
    self.list.last().unwrap()
  }
}