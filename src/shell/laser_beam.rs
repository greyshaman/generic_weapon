/// Лазерный луч
pub struct LaserBeam {
    power: f64,
    wave_length: f64,
}

impl LaserBeam {
    pub fn new(power: f64, wave_length: f64) -> Self {
        Self { power, wave_length }
    }

    pub fn power(&self) -> f64 {
        self.power
    }
}
