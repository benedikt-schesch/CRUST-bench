pub struct Signal {
data: Vec<(u64, String)>,
}

impl Signal {
pub fn get_value_at_timestamp(&self, timestamp: u64) -> Option<String> {
// Binary search or linear search for the value at timestamp
for (ts, val) in &self.data {
if *ts == timestamp {
return Some(val.clone());
}
}
None
}
}

pub fn get_test_signal() -> Signal {
Signal {
data: vec![
(0, "0".to_string()),
(100, "1".to_string()),
(200, "0".to_string()),
],
}
}
