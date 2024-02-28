pub mod structs;

use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    match item_type {
        "pending" => Ok(ItemTypes::Pending(Pending::new(item_title))),
        "done" => Ok(ItemTypes::Done(Done::new(item_title))),
        _ => Err("Invalid item type")
    }
}