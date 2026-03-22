/// Ракета
pub struct Rocket {
    warhead: String,
    range: f64,
}

impl Rocket {
    pub fn new(warhead: &str, range: f64) -> Self {
        Self {
            warhead: warhead.into(),
            range,
        }
    }
}
