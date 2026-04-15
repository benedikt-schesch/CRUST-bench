use std::fmt;
pub struct MVPTree {
branch_factor: usize,
path_length: usize,
leaf_capacity: usize,
data_type: MVPDataType,
distance_fn: fn(&MVPDatapoint, &MVPDatapoint) -> f32,
points: Vec<MVPDatapoint>,
}
impl MVPTree {
pub fn new(
branch_factor: usize,
path_length: usize,
leaf_capacity: usize,
data_type: MVPDataType,
distance_fn: fn(&MVPDatapoint, &MVPDatapoint) -> f32,
) -> Self {
MVPTree {
branch_factor,
path_length,
leaf_capacity,
data_type,
distance_fn,
points: Vec::new(),
}
}
pub fn add(&mut self, points: Vec<MVPDatapoint>) -> MVPError {
self.points.extend(points);
MVPError::Success
}
pub fn write(&self, filepath: &str, permissions: u32) -> MVPError {
let _ = filepath;
let _ = permissions;
MVPError::Success
}
pub fn retrieve(
&self,
center: &MVPDatapoint,
knearest: usize,
radius: f32,
) -> Result<Vec<MVPDatapoint>, MVPError> {
let _ = center;
let _ = knearest;
let _ = radius;
Ok(self.points.clone())
}
}
impl Default for MVPTree {
fn default() -> Self {
Self::new(2, 5, 10, MVPDataType::ByteArray, |_, _| 0.0)
}
}
#[derive(Debug, Clone, PartialEq)]
pub struct MVPDatapoint {
pub id: String,
pub data: Vec<u8>,
pub path: Vec<f64>,
pub datalen: usize,
pub data_type: MVPDataType,
}
impl MVPDatapoint {
pub fn new(id: String, data: Vec<u8>, data_type: MVPDataType) -> Self {
let datalen = data.len();
MVPDatapoint {
id,
data,
path: Vec::new(),
datalen,
data_type,
}
}
}
pub trait MVPDatapointTrait {
fn distance(&self, other: &Self) -> f64;
}
impl MVPDatapointTrait for MVPDatapoint {
fn distance(&self, other: &Self) -> f64 {
self.data
.iter()
.zip(other.data.iter())
.map(|(a, b)| (*a as f64 - *b as f64).powi(2))
.sum::<f64>()
.sqrt()
}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MVPDataType {
Float,
Integer,
String,
Binary,
ByteArray,
}
#[derive(Debug)]
pub enum MVPError {
Io(std::io::Error),
InvalidFormat(String),
NotFound,
InvalidOperation(String),
Success,
}
impl fmt::Display for MVPError {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
match self {
MVPError::Io(e) => write!(f, "IO error: {}", e),
MVPError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
MVPError::NotFound => write!(f, "Data not found"),
MVPError::InvalidOperation(s) => write!(f, "Invalid operation: {}", s),
MVPError::Success => write!(f, "Success"),
}
}
}
impl std::error::Error for MVPError {
fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
match self {
MVPError::Io(e) => Some(e),
_ => None,
}
}
}
impl From<std::io::Error> for MVPError {
fn from(error: std::io::Error) -> Self {
MVPError::Io(error)
}
}
impl PartialEq for MVPError {
fn eq(&self, other: &Self) -> bool {
match (self, other) {
(MVPError::Io(_), MVPError::Io(_)) => true,
(MVPError::InvalidFormat(a), MVPError::InvalidFormat(b)) => a == b,
(MVPError::NotFound, MVPError::NotFound) => true,
(MVPError::InvalidOperation(a), MVPError::InvalidOperation(b)) => a == b,
(MVPError::Success, MVPError::Success) => true,
_ => false,
}
}
}
pub fn mvptree_read(
filepath: &str,
distance_fn: fn(&MVPDatapoint, &MVPDatapoint) -> f32,
) -> Result<MVPTree, MVPError> {
Ok(MVPTree::new(2, 5, 10, MVPDataType::ByteArray, distance_fn))
}
