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
        guitar::Builder::Fender,
        String::from("Stratocastor"),
        guitar::GuitarType::Electric,
        guitar::Wood::Alder,
        guitar::Wood::Alder,
    );

    let guitars = inventory.search(&what_erin_like);
    if guitars.len() > 0 {
        println!("Erin, you might like these guitars:");
        for guitar in guitars {
            println!(
                "We have a {} {} {} guitar:",
                guitar.get_builder(),
                guitar.get_model(),
                guitar.get_type()
            );
            println!("{} back side, ", guitar.get_back_wood());
            println!("{} top.", guitar.get_top_wood());
            println!("You can have it for only ${}!", guitar.get_price());
            println!("----");
        }
    } else {
        println!("Sorry, Erin, we have nothing for you.");
    }
}

fn initialize_inventory(inventory: &mut Inventory) {
    inventory.add_guitar(
        String::from("11277"),
        3999.95,
        guitar::Builder::Collings,
        String::from("CJ"),
        guitar::GuitarType::Acoustic,
        guitar::Wood::IndianRoseWood,
        guitar::Wood::Sitka,
    );
    inventory.add_guitar(
        String::from("V95693"),
        1499.95,
        guitar::Builder::Fender,
        String::from("Stratocastor"),
        guitar::GuitarType::Electric,
        guitar::Wood::Alder,
        guitar::Wood::Alder,
    );
    inventory.add_guitar(
        String::from("V9512"),
        1549.95,
        guitar::Builder::Fender,
        String::from("Stratocastor"),
        guitar::GuitarType::Electric,
        guitar::Wood::Alder,
        guitar::Wood::Alder,
    );
    inventory.add_guitar(
        String::from("122784"),
        5495.95,
        guitar::Builder::Martin,
        String::from("D-18"),
        guitar::GuitarType::Acoustic,
        guitar::Wood::Mahogany,
        guitar::Wood::Adirondack,
    );
    inventory.add_guitar(
        String::from("76531"),
        6295.95,
        guitar::Builder::Martin,
        String::from("OM-28"),
        guitar::GuitarType::Acoustic,
        guitar::Wood::BrazilianRoseWood,
        guitar::Wood::Adirondack,
    );
    inventory.add_guitar(
        String::from("70108276"),
        2295.95,
        guitar::Builder::Gibson,
        String::from("Les Paul"),
        guitar::GuitarType::Electric,
        guitar::Wood::Mahogany,
        guitar::Wood::Maple,
    );
    inventory.add_guitar(
        String::from("82765501"),
        1890.95,
        guitar::Builder::Gibson,
        String::from("SG '61 Reissue"),
        guitar::GuitarType::Electric,
        guitar::Wood::Mahogany,
        guitar::Wood::Mahogany,
    );
    inventory.add_guitar(
        String::from("77023"),
        6275.95,
        guitar::Builder::Martin,
        String::from("D-28"),
        guitar::GuitarType::Acoustic,
        guitar::Wood::BrazilianRoseWood,
        guitar::Wood::Adirondack,
    );
    inventory.add_guitar(
        String::from("1092"),
        12995.95,
        guitar::Builder::Olson,
        String::from("SJ"),
        guitar::GuitarType::Acoustic,
        guitar::Wood::IndianRoseWood,
        guitar::Wood::Cedar,
    );
    inventory.add_guitar(
        String::from("566-62"),
        8999.95,
        guitar::Builder::Ryan,
        String::from("Cathedral"),
        guitar::GuitarType::Acoustic,
        guitar::Wood::Cocobolo,
        guitar::Wood::Cedar,
    );
    inventory.add_guitar(
        String::from("6 29584"),
        2100.95,
        guitar::Builder::Prs,
        String::from("Dave Navarro Signature"),
        guitar::GuitarType::Electric,
        guitar::Wood::Mahogany,
        guitar::Wood::Maple,
    );
}
