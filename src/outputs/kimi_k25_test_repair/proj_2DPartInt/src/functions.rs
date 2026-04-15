use crate::data::{Particle, ParticleProperties, Vector, Contact};
pub fn compute_forces(
_dt: f64,
_size: usize,
_contacts_size: usize,
_particles: &[Particle],
_properties: &[ParticleProperties],
_contacts: &[Contact],
_velocities: &[Vector],
_normal_forces: &mut [Vector],
_tangent_forces: &mut [Vector],
_resultant_forces: &mut [Vector]
) {
}
pub fn compute_acceleration(
_idx: usize,
_properties: &[ParticleProperties],
_forces: &[Vector],
_accelerations: &mut [Vector]
) {
}
pub fn compute_velocity(
_dt: f64,
_idx: usize,
_accelerations: &[Vector],
_velocities: &mut [Vector]
) {
}
pub fn displace_particle(
_idx: usize,
_displacements: &[Vector],
_particles: &mut [Particle]
) {
}
