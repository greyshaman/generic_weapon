/// Пуля
pub struct Bullet {
    caliber: f64,
}

impl Bullet {
    pub fn new(caliber: f64) -> Self {
        Self { caliber }
    }

    pub fn caliber(&self) -> f64 {
        self.caliber
    }
}
