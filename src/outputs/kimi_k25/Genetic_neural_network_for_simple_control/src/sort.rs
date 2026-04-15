//! Sorting utilities

/// Quick sort implementation for slices
pub fn quickSort<T: PartialOrd>(arr: &mut [T]) {
arr.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
}

/// Sort by fitness values (indices)
pub fn sort_by_fitness<T: Clone>(population: &[T], fitness: &[f64]) -> Vec<T> {
let mut indices: Vec<usize> = (0..population.len()).collect();
indices.sort_by(|&a, &b| fitness[a].partial_cmp(&fitness[b]).unwrap_or(std::cmp::Ordering::Equal));

indices.into_iter()
.map(|i| population[i].clone())
.collect()
}
