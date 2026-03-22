use crate::firing_product::FiringProduct;

/// Шелест ветра
#[derive(Default)]
pub struct WindRustle {
    pub volume: f64,
}

impl FiringProduct for WindRustle {}
