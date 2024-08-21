use std::fmt;

#[derive(PartialEq, Eq)]
pub enum GuitarType {
    Acoustic,
    Electric,
}

impl fmt::Display for GuitarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuitarType::Acoustic => write!(f, "{}", "Acoustic"),
            GuitarType::Electric => write!(f, "{}", "Electric"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Builder {
    Fender,
    Martin,
    Gibson,
    Collings,
    Olson,
    Ryan,
    Prs,
}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Builder::Fender => write!(f, "{}", "Fender"),
            Builder::Martin => write!(f, "{}", "Martin"),
            Builder::Gibson => write!(f, "{}", "Gibson"),
            Builder::Collings => write!(f, "{}", "Collings"),
            Builder::Olson => write!(f, "{}", "Olson"),
            Builder::Ryan => write!(f, "{}", "Ryan"),
            Builder::Prs => write!(f, "{}", "Prs"),
        }
    }
}
#[derive(PartialEq, Eq)]
pub enum Wood {
    IndianRoseWood,
    BrazilianRoseWood,
    Mahogany,
    Maple,
    Cocobolo,
    Cedar,
    Adirondack,
    Alder,
    Sitka,
}

impl fmt::Display for Wood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wood::IndianRoseWood => write!(f, "{}", "Indian Rose Wood"),
            Wood::BrazilianRoseWood => write!(f, "{}", "Indian Rose Wood"),
            Wood::Mahogany => write!(f, "{}", "Indian Rose Wood"),
            Wood::Maple => write!(f, "{}", "Indian Rose Wood"),
            Wood::Cocobolo => write!(f, "{}", "Indian Rose Wood"),
            Wood::Cedar => write!(f, "{}", "Indian Rose Wood"),
            Wood::Adirondack => write!(f, "{}", "Indian Rose Wood"),
            Wood::Alder => write!(f, "{}", "Indian Rose Wood"),
            Wood::Sitka => write!(f, "{}", "Indian Rose Wood"),
        }
    }
}

pub struct Guitar {
    serial_number: String,
    builder: Builder,
    model: String,
    guitar_type: GuitarType,
    back_wood: Wood,
    top_wood: Wood,
    price: f64,
}

impl Guitar {
    pub fn new(
        serial_number: String,
        price: f64,
        builder: Builder,
        model: String,
        guitar_type: GuitarType,
        back_wood: Wood,
        top_wood: Wood,
    ) -> Guitar {
        Guitar {
            serial_number,
            builder,
            model,
            guitar_type,
            back_wood,
            top_wood,
            price,
        }
    }

    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }

    pub fn get_builder(&self) -> &Builder {
        &self.builder
    }

    pub fn get_model(&self) -> String {
        self.model.clone()
    }

    pub fn get_type(&self) -> &GuitarType {
        &self.guitar_type
    }

    pub fn get_top_wood(&self) -> &Wood {
        &self.top_wood
    }

    pub fn get_back_wood(&self) -> &Wood {
        &self.back_wood
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }
}
