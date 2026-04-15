use twoDPartInt::data::{Particle, ParticleProperties, Vector, Contact};
use twoDPartInt::functions::compute_forces;

fn main() {
let dt = 0.01;
let size = 10;
let contacts_size = 5;

let particles = vec![Particle {
x_coordinate: 0.0,
y_coordinate: 0.0,
radius: 1.0,
next: None,
idx: 0
}; size];

let properties = vec![ParticleProperties {
mass: 1.0,
kn: 100.0,
ks: 50.0
}; size];

let contacts = vec![Contact {
p1_idx: 0,
p2_idx: 1,
overlap: 0.1
}; contacts_size];

let velocities = vec![Vector {
x_component: 0.0,
y_component: 0.0
}; size];

// Initialize force vectors as Vector types, not f64
let mut normal_forces = vec![Vector {
x_component: 0.0,
y_component: 0.0
}; size];

let mut tangent_forces = vec![Vector {
x_component: 0.0,
y_component: 0.0
}; size];

let mut resultant_forces = vec![Vector {
x_component: 0.0,
y_component: 0.0
}; size];

// First call at line 33
compute_forces(
dt,
size,
contacts_size,
&particles,
&properties,
&contacts,
&velocities,
&mut normal_forces,
&mut tangent_forces,
&mut resultant_forces
);

// Second call at line 102 (simulating another computation step)
compute_forces(
dt,
size,
contacts_size,
&particles,
&properties,
&contacts,
&velocities,
&mut normal_forces,
&mut tangent_forces,
&mut resultant_forces
);
}
