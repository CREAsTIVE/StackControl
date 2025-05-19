use std::sync::Arc;

pub mod reference_container;
pub mod execution;

pub enum Holder<'a, T: ?Sized> {
  Ref(&'a T),
  Val(Box<T>)
}

impl<'a, T> Holder<'a, T> {
  pub fn to_ref(&'a self) -> &'a T {
    match self {
      Holder::Ref(r) => r,
      Holder::Val(boxed) => boxed.as_ref()
    }
  }
}

pub enum ArcHolder<'a, T: ?Sized> {
  Ref(&'a T),
  Val(Arc<T>)
}

impl<'a, T: ?Sized> ArcHolder<'a, T> {
  pub fn to_ref(&'a self) -> &'a T {
    match self {
      ArcHolder::Ref(r) => r,
      ArcHolder::Val(boxed) => boxed.as_ref()
    }
  }
}