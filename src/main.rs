mod to_do;

use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let to_do_item = to_do_factory("pending", "washing");

    match to_do_item.unwrap() {
        ItemTypes::Done(to_do_item) => println!(
            "it's a done item with the title: {}",
            to_do_item.super_struct.title
        ),
        ItemTypes::Pending(to_do_item) => println!(
            "it's a pending item with the title: {}",
            to_do_item.super_struct.title
        ),
    }
}
