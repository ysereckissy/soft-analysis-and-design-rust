use super::Guitar;

pub struct Inventory {
    guitars: Vec<Guitar>,
}

impl Inventory {
    pub fn new() -> Inventory {
        let guitars: Vec<Guitar> = Vec::new();
        Inventory { guitars }
    }
    pub fn add_guitar(
        &mut self,
        serial_number: String,
        price: f64,
        builder: String,
        model: String,
        guitar_type: String,
        back_wood: String,
        top_wood: String,
    ) {
        self.guitars.push(Guitar::new(
            serial_number,
            price,
            builder,
            model,
            guitar_type,
            back_wood,
            top_wood,
        ));
    }

    pub fn get_guitar(&self, serial_number: String) -> Option<&Guitar> {
        for guitar in &self.guitars {
            if guitar.get_serial_number() == serial_number {
                return Some(guitar);
            }
        }
        return None;
    }

    pub fn search(&self, searched_guitar: &Guitar) -> Option<&Guitar> {
        for guitar in &self.guitars {
            if (guitar.get_builder() != String::from(""))
                && (guitar.get_builder() != searched_guitar.get_builder())
            {
                continue;
            }
            if (guitar.get_model() != String::from(""))
                && (guitar.get_model() != searched_guitar.get_model())
            {
                continue;
            }
            if (guitar.get_type() != String::from(""))
                && (guitar.get_type() != searched_guitar.get_type())
            {
                continue;
            }
            if (guitar.get_back_wood() != String::from(""))
                && (guitar.get_back_wood() != searched_guitar.get_back_wood())
            {
                continue;
            }
            if (guitar.get_top_wood() != String::from(""))
                && (guitar.get_top_wood() != searched_guitar.get_top_wood())
            {
                continue;
            }
            return Some(guitar);
        }
        return None;
    }
}
