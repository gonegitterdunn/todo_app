pub mod structs;

use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
  Done(Done),
  Pending(Pending),
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
  if item_type == "done" {
    let done_item = Done::new(item_title);
    Ok(ItemTypes::Done(done_item))
  } else if item_type == "pending" {
    let pending_item = Pending::new(item_title);
    Ok(ItemTypes::Pending(pending_item))
  } else {
    Err("item_type not accepted")
  }
}
