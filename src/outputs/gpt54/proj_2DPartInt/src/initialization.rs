// Generated Rust Code
use crate::config;
use crate::data;

pub fn build_particle_properties(cfg: &config::Config, count: usize) -> Vec<data::ParticleProperties> {
let mass = cfg.compute_mass();
let mut properties = Vec::with_capacity(count);
for _ in 0..count {
properties.push(data::ParticleProperties {
kn: cfg.kn,
ks: cfg.ks,
mass,
});
}
properties
}
