#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
_marker: std::marker::PhantomData<(K, V)>,
}
impl<K, V> HashMap<K, V> {
pub fn new() -> Self {
HashMap {
_marker: std::marker::PhantomData,
}
}
pub fn insert(&mut self, _key: K, _value: V) {}
pub fn get(&self, _key: &K) -> Option<&V> {
None
}
}
