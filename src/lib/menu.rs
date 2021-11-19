// pub mod menu {
use druid::{Data, Lens};
use enum_iterator::IntoEnumIterator;

pub struct Menu<'m> {
  pub title: &'m str,
  pub items: Vec<Item>,
}
// impl<'m> Menu<'m> {
//   pub fn new(title: &'m str) -> Self {
//     Self {
//       title,
//       items: Item::as_vec(),
//     }
//   }
// }
#[derive(Clone, Data, Lens)]
pub struct State {
  pub selected_item: Option<Item>,
}
impl State {
  pub fn new() -> Self {
    Self {
      selected_item: Some(Item::New),
    }
  }
}
#[derive(Clone, Copy, Data, PartialEq, IntoEnumIterator, Debug)]
pub enum Item {
  New,
  Load,
  Save,
  Quit,
}
impl Item {
  pub fn as_vec() -> Vec<Self> {
    Self::into_enum_iter().collect()
  }
}
impl Iterator for Item {
  type Item = Item;
  fn next(&mut self) -> Option<Self::Item> {
    Some(Self::New)
  }
}
// impl PartialEq for Item {
//   fn eq(&self, other: &Self) -> bool {
//     self.0 == other.0
//   }
// }
// }
