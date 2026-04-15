// Generated Rust Code
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
pub x_component: f64,
pub y_component: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Particle {
pub idx: i32,
pub x_coordinate: f64,
pub y_coordinate: f64,
pub radius: f64,
pub next: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Contact {
pub p1_idx: usize,
pub p2_idx: usize,
pub overlap: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ParticleProperties {
pub kn: f64,
pub ks: f64,
pub mass: f64,
}
