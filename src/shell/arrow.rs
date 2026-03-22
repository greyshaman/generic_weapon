/// Стрела
#[derive(Clone)]
pub struct Arrow {
    length: f64,
    tip_type: String,
}

impl Arrow {
    pub fn new(length: f64, tip_type: &str) -> Self {
        Self {
            length,
            tip_type: tip_type.into(),
        }
    }
}
