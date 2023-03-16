#[derive(Clone, Debug, PartialEq)]
pub struct Temp {
    pub value: f64,
    pub units: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fan {
    pub value: f64,
    pub units: String,
}
