#[derive(Clone, Debug, PartialEq)]
pub struct Temp {
    pub label: String,
    pub value: f64,
    pub units: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fan {
    pub label: String,
    pub value: f64,
    pub units: String,
}
