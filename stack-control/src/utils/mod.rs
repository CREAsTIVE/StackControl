use std::sync::Arc;

pub mod reference_container;
pub mod execution;

pub enum MBox<'a, T: ?Sized> {
  Ref(&'a T),
  Val(Box<T>)
}

impl<'a, T> MBox<'a, T> {
  pub fn to_ref(&'a self) -> &'a T {
    match self {
      MBox::Ref(r) => r,
      MBox::Val(boxed) => boxed.as_ref()
    }
  }
}

#[derive(Clone)]
pub enum MArc<'a, T: ?Sized> {
  Ref(&'a T),
  Val(Arc<T>)
}

impl<'a, T: ?Sized> MArc<'a, T> {
  pub fn to_ref(&'a self) -> &'a T {
    match self {
      MArc::Ref(r) => r,
      MArc::Val(boxed) => boxed.as_ref()
    }
  }
}