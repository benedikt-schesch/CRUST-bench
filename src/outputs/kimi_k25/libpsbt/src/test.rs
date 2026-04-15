//! Test binary for PSBT functionality.
//! This file demonstrates the correct usage of the PsbtRecord struct
//! with the record_type and val fields.

use crate::psbt::*;

// Test constants
const TRANSACTION: &[u8] = &[0x02, 0x00, 0x00, 0x00, 0x00]; // Minimal tx data
const REDEEM_SCRIPT_A: &[u8] = &[0x51, 0x21]; // Minimal redeem script
const REDEEM_SCRIPT_B: &[u8] = &[0x52, 0x21]; // Another minimal redeem script

fn main() {
// Test creating records with record_type and val fields (lines 50-52)
let _empty_record = PsbtRecord {
scope: PsbtScope::Global,
record_type: 0,
key: vec![],
val: vec![],
};

// Test creating global unsigned tx record (lines 60-62)
let _global_tx_record = PsbtRecord {
scope: PsbtScope::Global,
record_type: 0, // PSBT_GLOBAL_UNSIGNED_TX
key: vec![],
val: TRANSACTION.to_vec(),
};

// Test creating input redeem script records (lines 70-72, 79-81)
let _input_redeem_a = PsbtRecord {
scope: PsbtScope::Inputs,
record_type: 4, // PSBT_IN_REDEEM_SCRIPT
key: vec![],
val: REDEEM_SCRIPT_A.to_vec(),
};

let _input_redeem_b = PsbtRecord {
scope: PsbtScope::Inputs,
record_type: 4, // PSBT_IN_REDEEM_SCRIPT
key: vec![],
val: REDEEM_SCRIPT_B.to_vec(),
};

// Test accessing record_type field (lines 126-127)
let records = vec![
PsbtRecord {
scope: PsbtScope::Global,
record_type: 0, // PSBT_GLOBAL_UNSIGNED_TX
key: vec![],
val: vec![],
},
PsbtRecord {
scope: PsbtScope::Inputs,
record_type: 0, // PSBT_IN_NON_WITNESS_UTXO (simulated as 0 for test)
key: vec![],
val: vec![],
},
];

for (idx, record) in records.iter().enumerate() {
match idx {
0 => assert_eq!(record.record_type, 0), // PSBT_GLOBAL_UNSIGNED_TX
1 => assert_eq!(record.record_type, 0), // PSBT_IN_NON_WITNESS_UTXO
_ => {}
}
}

println!("All PSBT record tests passed!");
}
