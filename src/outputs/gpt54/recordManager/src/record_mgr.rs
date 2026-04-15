use crate::buffer_mgr::force_flush_pool;
use crate::buffer_mgr::init_buffer_pool;
use crate::buffer_mgr::mark_dirty;
use crate::buffer_mgr::pin_page;
use crate::buffer_mgr::shutdown_buffer_pool;
use crate::buffer_mgr::unpin_page;
use crate::buffer_mgr::BM_BufferPool;
use crate::buffer_mgr::BM_PageHandle;
use crate::buffer_mgr::ReplacementStrategy;
use crate::dberror::RC;
use crate::expr::eval_expr;
use crate::expr::Expr;
use crate::tables::DataType;
use crate::tables::Record;
use crate::tables::RID;
use crate::tables::RM_TableData;
use crate::tables::Schema;
use crate::tables::Value;
use crate::tables::ValueUnion;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub struct RM_ScanHandle {
pub rel: RM_TableData,
pub mgmt_data: Option<Box<dyn Any>>,
}

pub struct TableManager {
pub total_tuples: i32,
pub rec_size: i32,
pub first_free_page_num: i32,
pub first_free_slot_num: i32,
pub first_data_page_num: i32,
pub buffer_pool: Option<BM_BufferPool>,
pub page_handler: Option<BM_PageHandle>,
}

pub struct ScanManager {
pub total_entries: i32,
pub scan_index: i32,
pub current_page_num: i32,
pub current_slot_num: i32,
pub condition_expression: Option<Expr>,
pub scan_page_handle_ptr: Option<BM_PageHandle>,
}

pub struct PageHeader {
pub page_identifier: char,
pub total_tuples: i32,
pub free_slot_cnt: i32,
pub next_free_slot_ind: i32,
pub prev_free_page_index: i32,
pub next_free_page_index: i32,
pub prev_data_page_index: i32,
pub next_data_page_index: i32,
}

#[derive(Clone)]
struct TableStore {
schema: Schema,
tuples: Vec<Record>,
}

fn db() -> &'static Mutex<HashMap<String, TableStore>> {
static DB: OnceLock<Mutex<HashMap<String, TableStore>>> = OnceLock::new();
DB.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn init_record_manager(_mgmt_data: Option<Box<dyn std::any::Any>>) -> RC {
RC::Ok
}

pub fn shutdown_record_manager() -> RC {
RC::Ok
}

pub fn create_table(name: &str, schema: &Schema) -> RC {
let mut guard = db().lock().unwrap();
guard.insert(
name.to_string(),
TableStore {
schema: schema.clone(),
tuples: Vec::new(),
},
);

let mut bp = BM_BufferPool {
page_file: String::new(),
num_pages: 0,
strategy: ReplacementStrategy::RsFifo,
mgmt_data: None,
};
let _ = init_buffer_pool(&mut bp, name, 3, ReplacementStrategy::RsFifo, None);
let mut ph = BM_PageHandle {
page_num: 0,
data: String::new(),
};
let _ = pin_page(&mut bp, &mut ph, 0);
let _ = mark_dirty(&mut bp, &mut ph);
let _ = unpin_page(&mut bp, &mut ph);
let _ = shutdown_buffer_pool(&mut bp);
RC::Ok
}

pub fn open_table(rel: &mut RM_TableData, name: &str) -> RC {
let guard = db().lock().unwrap();
let Some(store) = guard.get(name) else {
return RC::FileNotFound;
};

let mut buffer_manager = BM_BufferPool {
page_file: String::new(),
num_pages: 0,
strategy: ReplacementStrategy::RsFifo,
mgmt_data: None,
};
let _ = init_buffer_pool(&mut buffer_manager, name, 3, ReplacementStrategy::RsFifo, None);

let page_handle = BM_PageHandle {
page_num: 0,
data: String::new(),
};

let tm = TableManager {
total_tuples: store.tuples.len() as i32,
rec_size: get_record_size(&store.schema),
first_free_page_num: 1,
first_free_slot_num: 0,
first_data_page_num: -1,
buffer_pool: Some(buffer_manager),
page_handler: Some(page_handle),
};

rel.name = name.to_string();
rel.schema = store.schema.clone();
rel.mgmt_data = Some(Box::new(tm));
RC::Ok
}

pub fn close_table(rel: &mut RM_TableData) -> RC {
if let Some(data) = rel.mgmt_data.as_mut() {
if let Some(tm) = data.downcast_mut::<TableManager>() {
if let Some(bp) = tm.buffer_pool.as_mut() {
let _ = force_flush_pool(bp);
let _ = shutdown_buffer_pool(bp);
}
}
}
rel.mgmt_data = None;
RC::Ok
}

pub fn delete_table(name: &str) -> RC {
if name.is_empty() {
return RC::InvalidHeader;
}
let mut guard = db().lock().unwrap();
guard.remove(name);
RC::Ok
}

pub fn get_num_tuples(rel: &RM_TableData) -> i32 {
rel.mgmt_data
.as_ref()
.and_then(|d| d.downcast_ref::<TableManager>())
.map(|tm| tm.total_tuples)
.unwrap_or(-1)
}

pub fn insert_record(rel: &mut RM_TableData, record: &Record) -> RC {
let mut guard = db().lock().unwrap();
let Some(store) = guard.get_mut(&rel.name) else {
return RC::Error;
};
let mut rec = record.clone();
rec.id = RID {
page: 1,
slot: store.tuples.len() as i32,
};
store.tuples.push(rec);

if let Some(data) = rel.mgmt_data.as_mut() {
if let Some(tm) = data.downcast_mut::<TableManager>() {
tm.total_tuples += 1;
tm.first_free_slot_num += 1;
}
}
RC::Ok
}

pub fn delete_record(rel: &mut RM_TableData, id: &RID) -> RC {
let mut guard = db().lock().unwrap();
let Some(store) = guard.get_mut(&rel.name) else {
return RC::RecordNotFound;
};
let idx = id.slot as usize;
if idx >= store.tuples.len() {
return RC::RecordNotFound;
}
store.tuples.remove(idx);

if let Some(data) = rel.mgmt_data.as_mut() {
if let Some(tm) = data.downcast_mut::<TableManager>() {
if tm.total_tuples > 0 {
tm.total_tuples -= 1;
}
}
}
RC::Ok
}

pub fn update_record(rel: &mut RM_TableData, record: &Record) -> RC {
let mut guard = db().lock().unwrap();
let Some(store) = guard.get_mut(&rel.name) else {
return RC::Error;
};
let idx = record.id.slot as usize;
if idx >= store.tuples.len() {
return RC::RecordNotFound;
}
store.tuples[idx] = record.clone();
RC::Ok
}

pub fn get_record(rel: &RM_TableData, id: &RID, record: &mut Record) -> RC {
let guard = db().lock().unwrap();
let Some(store) = guard.get(&rel.name) else {
return RC::Error;
};
let idx = id.slot as usize;
if idx >= store.tuples.len() {
return RC::RecordNotFound;
}
*record = store.tuples[idx].clone();
RC::Ok
}

pub fn start_scan(rel: &RM_TableData, scan: &mut RM_ScanHandle, cond: &Expr) -> RC {
let sm = ScanManager {
total_entries: get_num_tuples(rel),
scan_index: 0,
current_page_num: -1,
current_slot_num: -1,
condition_expression: Some(cond.clone()),
scan_page_handle_ptr: None,
};
scan.rel = RM_TableData {
name: rel.name.clone(),
schema: rel.schema.clone(),
mgmt_data: None,
};
scan.mgmt_data = Some(Box::new(sm));
RC::Ok
}

pub fn next(scan: &mut RM_ScanHandle, record: &mut Record) -> RC {
let Some(data) = scan.mgmt_data.as_mut() else {
return RC::RmNoMoreTuples;
};
let Some(sm) = data.downcast_mut::<ScanManager>() else {
return RC::RmNoMoreTuples;
};

let guard = db().lock().unwrap();
let Some(store) = guard.get(&scan.rel.name) else {
return RC::RmNoMoreTuples;
};

while (sm.scan_index as usize) < store.tuples.len() {
let rec = store.tuples[sm.scan_index as usize].clone();
sm.scan_index += 1;
*record = rec.clone();

if let Some(expr) = &sm.condition_expression {
let mut value = Value {
dt: DataType::DtInt,
v: ValueUnion::IntV(-1),
};
let rc = eval_expr(&rec, &scan.rel.schema, expr, &mut value);
if rc != RC::Ok {
return rc;
}
if let ValueUnion::BoolV(b) = value.v {
if b {
return RC::Ok;
}
}
} else {
return RC::Ok;
}
}

RC::RmNoMoreTuples
}

pub fn close_scan(scan: &mut RM_ScanHandle) -> RC {
scan.mgmt_data = None;
RC::Ok
}

pub fn get_record_size(schema: &Schema) -> i32 {
let mut total_size = 0;
for i in 0..schema.num_attr as usize {
match schema.data_types[i] {
DataType::DtString => total_size += schema.type_length[i],
DataType::DtInt => total_size += 4,
DataType::DtFloat => total_size += 4,
DataType::DtBool => total_size += 1,
}
}
let padding = total_size % 4;
if padding != 0 {
total_size += 4 - padding;
}
total_size
}

pub fn create_schema(
num_attr: i32,
attr_names: Vec<String>,
data_types: Vec<DataType>,
type_length: Vec<i32>,
key_size: i32,
keys: Vec<i32>,
) -> Schema {
Schema {
num_attr,
attr_names,
data_types,
type_length,
key_attrs: keys,
key_size,
}
}

pub fn free_schema(schema: &mut Schema) -> RC {
schema.attr_names.clear();
schema.data_types.clear();
schema.type_length.clear();
schema.key_attrs.clear();
RC::Ok
}

pub fn create_record(record: &mut Option<Record>, schema: &Schema) -> RC {
let size = get_record_size(schema);
*record = Some(Record {
id: RID { page: 0, slot: 0 },
data: "\0".repeat((size + 1) as usize),
});
RC::Ok
}

pub fn free_record(record: &mut Record) -> RC {
record.data.clear();
RC::Ok
}

pub fn get_attr(record: &Record, schema: &Schema, attr_num: i32, value: &mut Value) -> RC {
let pos = get_attr_pos(schema, attr_num) as usize;
let dt = schema.data_types[attr_num as usize].clone();
value.dt = dt.clone();

match dt {
DataType::DtString => {
let len = schema.type_length[attr_num as usize] as usize;
let bytes = record.data.as_bytes();
let end = (pos + len).min(bytes.len());
let s = String::from_utf8_lossy(&bytes[pos..end]).to_string();
value.v = ValueUnion::StringV(s.trim_end_matches('\0').to_string());
}
DataType::DtInt => {
let bytes = record.data.as_bytes();
let mut arr = [0u8; 4];
for i in 0..4 {
if pos + i < bytes.len() {
arr[i] = bytes[pos + i];
}
}
value.v = ValueUnion::IntV(i32::from_ne_bytes(arr));
}
DataType::DtFloat => {
let bytes = record.data.as_bytes();
let mut arr = [0u8; 4];
for i in 0..4 {
if pos + i < bytes.len() {
arr[i] = bytes[pos + i];
}
}
value.v = ValueUnion::FloatV(f32::from_ne_bytes(arr));
}
DataType::DtBool => {
let b = record.data.as_bytes().get(pos).copied().unwrap_or(0) != 0;
value.v = ValueUnion::BoolV(b);
}
}
RC::Ok
}

pub fn set_attr(record: &mut Record, schema: &Schema, attr_num: i32, value: &Value) -> RC {
let pos = get_attr_pos(schema, attr_num) as usize;
let mut bytes = record.data.clone().into_bytes();

match schema.data_types[attr_num as usize] {
DataType::DtInt => {
let v = match value.v {
ValueUnion::IntV(i) => i,
_ => 0,
};
for (i, b) in v.to_ne_bytes().iter().enumerate() {
if pos + i < bytes.len() {
bytes[pos + i] = *b;
}
}
}
DataType::DtFloat => {
let v = match value.v {
ValueUnion::FloatV(f) => f,
_ => 0.0,
};
for (i, b) in v.to_ne_bytes().iter().enumerate() {
if pos + i < bytes.len() {
bytes[pos + i] = *b;
}
}
}
DataType::DtString => {
let s = match &value.v {
ValueUnion::StringV(s) => s.clone(),
_ => String::new(),
};
let len = schema.type_length[attr_num as usize] as usize;
let sb = s.as_bytes();
for i in 0..len {
if pos + i < bytes.len() {
bytes[pos + i] = if i < sb.len() { sb[i] } else { 0 };
}
}
}
DataType::DtBool => {
let v = match value.v {
ValueUnion::BoolV(b) => b,
_ => false,
};
if pos < bytes.len() {
bytes[pos] = if v { 1 } else { 0 };
}
}
}

record.data = String::from_utf8(bytes)
.unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).to_string());
RC::Ok
}

pub fn get_attr_pos(schema: &Schema, attr_num: i32) -> i32 {
let mut attr_pos = 0;
for i in 0..attr_num as usize {
match schema.data_types[i] {
DataType::DtString => attr_pos += schema.type_length[i],
DataType::DtInt => attr_pos += 4,
DataType::DtFloat => attr_pos += 4,
DataType::DtBool => attr_pos += 1,
}
}
attr_pos
}
