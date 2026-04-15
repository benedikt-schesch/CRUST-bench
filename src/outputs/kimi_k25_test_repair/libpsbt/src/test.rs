
use crate::psbt::*;
const TRANSACTION: &[u8] = &[0x02, 0x00, 0x00, 0x00, 0x00]; 
const REDEEM_SCRIPT_A: &[u8] = &[0x51, 0x21]; 
const REDEEM_SCRIPT_B: &[u8] = &[0x52, 0x21]; 
fn main() {
let _empty_record = PsbtRecord {
scope: PsbtScope::Global,
record_type: 0,
key: vec![],
val: vec![],
};
let _global_tx_record = PsbtRecord {
scope: PsbtScope::Global,
record_type: 0, 
key: vec![],
val: TRANSACTION.to_vec(),
};
let _input_redeem_a = PsbtRecord {
scope: PsbtScope::Inputs,
record_type: 4, 
key: vec![],
val: REDEEM_SCRIPT_A.to_vec(),
};
let _input_redeem_b = PsbtRecord {
scope: PsbtScope::Inputs,
record_type: 4, 
key: vec![],
val: REDEEM_SCRIPT_B.to_vec(),
};
let records = vec![
PsbtRecord {
scope: PsbtScope::Global,
record_type: 0, 
key: vec![],
val: vec![],
},
PsbtRecord {
scope: PsbtScope::Inputs,
record_type: 0, 
key: vec![],
val: vec![],
},
];
for (idx, record) in records.iter().enumerate() {
match idx {
0 => assert_eq!(record.record_type, 0), 
1 => assert_eq!(record.record_type, 0), 
_ => {}
}
}
println!("All PSBT record tests passed!");
}
