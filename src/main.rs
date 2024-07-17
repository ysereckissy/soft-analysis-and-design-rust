pub mod guitar;
pub mod inventory;

use crate::guitar::Guitar;
use crate::inventory::Inventory;

fn main() {
    let mut inventory = Inventory::new();
    initialize_inventory(&mut inventory);

    let what_erin_like = Guitar::new(
        String::from(""),
        0.0,
        String::from("fender"),
        String::from("Stratocastor"),
        String::from("electric"),
        String::from("Alder"),
        String::from("Alder"),
    );

    match inventory.search(&what_erin_like) {
        Some(guitar) => {
            println!(
                "Erin, you might like this {} {} {} guitar: ",
                guitar.get_builder(),
                guitar.get_model(),
                guitar.get_type()
            );
            println!("{} back and sides, ", guitar.get_back_wood());
            println!("{} top.", guitar.get_top_wood());
            println!("You can have it for only ${}!", guitar.get_price());
        }
        _ => println!("Sorry, Erin, we have nothing for you."),
    }
}

fn initialize_inventory(inventory: &mut Inventory) {
    inventory.add_guitar(
        String::from("11277"),
        3999.95,
        String::from("Collings"),
        String::from("CJ"),
        String::from("acoustic"),
        String::from("Indian Rosewood"),
        String::from("Sitka"),
    );
    inventory.add_guitar(
        String::from("V95693"),
        1499.95,
        String::from("Fender"),
        String::from("Stratocastor"),
        String::from("electric"),
        String::from("Alder"),
        String::from("Alder"),
    );
    inventory.add_guitar(
        String::from("V9512"),
        1549.95,
        String::from("Fender"),
        String::from("Stratocastor"),
        String::from("electric"),
        String::from("Alder"),
        String::from("Alder"),
    );
    inventory.add_guitar(
        String::from("122784"),
        5495.95,
        String::from("Martin"),
        String::from("D-18"),
        String::from("acoustic"),
        String::from("Mahogany"),
        String::from("Adirondack"),
    );
    inventory.add_guitar(
        String::from("76531"),
        6295.95,
        String::from("Martin"),
        String::from("OM-28"),
        String::from("acoustic"),
        String::from("Brazilian Rosewood"),
        String::from("Adriondack"),
    );
    inventory.add_guitar(
        String::from("70108276"),
        2295.95,
        String::from("Gibson"),
        String::from("Les Paul"),
        String::from("electric"),
        String::from("Mahogany"),
        String::from("Maple"),
    );
    inventory.add_guitar(
        String::from("82765501"),
        1890.95,
        String::from("Gibson"),
        String::from("SG '61 Reissue"),
        String::from("electric"),
        String::from("Mahogany"),
        String::from("Mahogany"),
    );
    inventory.add_guitar(
        String::from("77023"),
        6275.95,
        String::from("Martin"),
        String::from("D-28"),
        String::from("acoustic"),
        String::from("Brazilian Rosewood"),
        String::from("Adirondack"),
    );
    inventory.add_guitar(
        String::from("1092"),
        12995.95,
        String::from("Olson"),
        String::from("SJ"),
        String::from("acoustic"),
        String::from("Indian Rosewood"),
        String::from("Cedar"),
    );
    inventory.add_guitar(
        String::from("566-62"),
        8999.95,
        String::from("Ryan"),
        String::from("Cathedral"),
        String::from("acoustic"),
        String::from("Cocobolo"),
        String::from("Cedar"),
    );
    inventory.add_guitar(
        String::from("6 29584"),
        2100.95,
        String::from("PRS"),
        String::from("Dave Navarro Signature"),
        String::from("electric"),
        String::from("Mahogany"),
        String::from("Maple"),
    );
}
