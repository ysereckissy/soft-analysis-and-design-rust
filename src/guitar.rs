pub struct Guitar {
    serial_number: String,
    builder: String,
    model: String,
    guitar_type: String,
    back_wood: String,
    top_wood: String,
    price: f64,
}

impl Guitar {
    pub fn new(
        serial_number: String,
        price: f64,
        builder: String,
        model: String,
        guitar_type: String,
        back_wood: String,
        top_wood: String,
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

    pub fn get_builder(&self) -> String {
        self.builder.clone()
    }

    pub fn get_model(&self) -> String {
        self.model.clone()
    }

    pub fn get_type(&self) -> String {
        self.guitar_type.clone()
    }

    pub fn get_top_wood(&self) -> String {
        self.top_wood.clone()
    }

    pub fn get_back_wood(&self) -> String {
        self.back_wood.clone()
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }
}
