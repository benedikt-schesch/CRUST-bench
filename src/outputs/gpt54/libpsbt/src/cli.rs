use crate::psbt::psbt_decode;
use crate::psbt::psbt_encode;
use crate::psbt::psbt_init;
use crate::psbt::psbt_read;
use crate::psbt::Psbt;
use crate::psbt::PsbtElem;
use crate::psbt::PsbtElemHandler;
use crate::psbt::PsbtEncoding;
use crate::psbt::PsbtResult;
use std::any::Any;
use std::env;

const PSBT_EXAMPLE: &str = "70736274ff0100a00200000002ab0949a08c5af7c49b8212f417e2f15ab3f5c33dcf153821a8139f877a5b7be40000000000feffffffab0949a08c5af7c49b8212f417e2f15ab3f5c33dcf153821a8139f877a5b7be40100000000feffffff02603bea0b000000001976a914768a40bbd740cbe81d988e71de2a4d5c71396b1d88ac8e240000000000001976a9146f4620b553fa095e721b9ee0efe9fa039cca459788ac00000000000100df0200000001268171371edff285e937adeea4b37b78000c0566cbb3ad64641713ca42171bf6000000006a473044022070b2245123e6bf474d60c5b50c043d4c691a5d2435f09a34a7662a9dc251790a022001329ca9dacf280bdf30740ec0390422422c81cb45839457aeb76fc12edd95b3012102657d118d3357b8e0f4c2cd46db7b39f6d9c38d9a70abcb9b2de5dc8dbfe4ce31feffffff02d3dff505000000001976a914d0c59903c5bac2868760e90fd521a4665aa7652088ac00e1f5050000000017a9143545e6e33b832c47050f24d3eeb93c9c03948bc787b32e13000001012000e1f5050000000017a9143545e6e33b832c47050f24d3eeb93c9c03948bc787010416001485d13537f2e265405a34dbafa9e3dda01fb8230800220202ead596687ca806043edc3de116cdf29d5e9257c196cd055cf698c8d02bf24e9910b4a6ba670000008000000080020000800022020394f62be9df19952c5587768aeb7698061ad2c4a25c894f47d8c162b4d7213d0510b4a6ba6700000080010000800200008000";

fn hex_print(data: &[u8]) {
for b in data {
print!("{:02x}", b);
}
}

fn print_rec(elem: &mut PsbtElem, _user_data: &mut dyn Any) {
match elem {
PsbtElem::Record { index, record } => {
let type_str = crate::psbt::psbt_type_tostr(record.record_type, record.scope.clone());
print!("{}\t{} ", type_str, *index);
if !record.key.is_empty() {
hex_print(&record.key);
print!(" ");
}
hex_print(&record.val);
println!();
}
PsbtElem::TxElem { .. } => {}
}
}

fn usage() -> i32 {
println!("usage: psbt <psbt>");
1
}

fn main() -> i32 {
let args: Vec<String> = env::args().collect();
let _ = PSBT_EXAMPLE;

if args.len() < 2 {
return usage();
}

let mut psbt = Psbt::new(4096);
let mut buffer = vec![0u8; 4096];
let mut psbt_len = 0usize;
let mut out_len = 0usize;

let _ = psbt_init(&mut psbt, &mut buffer, 4096);

let res = psbt_decode(&args[1], args[1].len(), &mut buffer, 4096, &mut psbt_len);
if res != PsbtResult::Ok {
println!("error: {}", res);
return 1;
}

let mut user_data = ();
let handler: Option<PsbtElemHandler> = Some(print_rec);
let res = psbt_read(&buffer[..psbt_len], psbt_len, &mut psbt, handler, &mut user_data);
if res != PsbtResult::Ok {
println!("error: {}", res);
return 1;
}

let res = psbt_encode(&psbt, PsbtEncoding::Base62, &mut buffer, 4096, &mut out_len);
if res != PsbtResult::Ok {
println!("error: {}", res);
return 1;
}

0
}
