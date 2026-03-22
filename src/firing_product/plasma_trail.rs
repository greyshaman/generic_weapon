use crate::firing_product::FiringProduct;

/// Шелест ветра
#[derive(Default)]
pub struct PlasmaTrail {
    pub temperature: f64,
}

impl FiringProduct for PlasmaTrail {}
