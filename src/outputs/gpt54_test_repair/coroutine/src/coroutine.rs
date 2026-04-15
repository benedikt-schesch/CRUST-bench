use std::any::Any;

pub const COROUTINE_DEAD: i32 = 0;
pub const COROUTINE_READY: i32 = 1;
pub const COROUTINE_RUNNING: i32 = 2;
pub const COROUTINE_SUSPEND: i32 = 3;
pub const STACK_SIZE: usize = 1024 * 1024;
pub const DEFAULT_COROUTINE: usize = 16;

pub type CoroutineFunc = fn(schedule: &mut Schedule, data: &mut dyn Any);

pub struct Coroutine {
pub func: CoroutineFunc,
pub data: Box<dyn Any>,
pub cap: isize,
pub size: isize,
pub status: i32,
pub stack: Option<Box<[u8]>>,
}

pub struct Schedule {
pub stack: Box<[u8]>,
pub nco: usize,
pub cap: usize,
pub running: i32,
pub co: Vec<Option<Box<Coroutine>>>,
}

fn co_new(func: CoroutineFunc, data: Box<dyn Any>) -> Box<Coroutine> {
Box::new(Coroutine {
func,
data,
cap: 0,
size: 0,
status: COROUTINE_READY,
stack: None,
})
}

pub fn coroutine_open() -> Box<Schedule> {
let mut co = Vec::with_capacity(DEFAULT_COROUTINE);
co.resize_with(DEFAULT_COROUTINE, || None);
Box::new(Schedule {
stack: vec![0u8; STACK_SIZE].into_boxed_slice(),
nco: 0,
cap: DEFAULT_COROUTINE,
running: -1,
co,
})
}

pub fn coroutine_close(_schedule: Box<Schedule>) {}

pub fn coroutine_new(schedule: &mut Schedule, func: CoroutineFunc, data: Box<dyn Any>) -> i32 {
let co = co_new(func, data);

if schedule.nco >= schedule.cap {
let id = schedule.cap;
schedule.co.resize_with(schedule.cap * 2, || None);
schedule.co[id] = Some(co);
schedule.cap *= 2;
schedule.nco += 1;
id as i32
} else {
for i in 0..schedule.cap {
let id = (i + schedule.nco) % schedule.cap;
if schedule.co[id].is_none() {
schedule.co[id] = Some(co);
schedule.nco += 1;
return id as i32;
}
}
panic!("assertion failed");
}
}

pub fn coroutine_resume(schedule: &mut Schedule, id: i32) {
assert!(schedule.running == -1);
assert!(id >= 0 && (id as usize) < schedule.cap);

let idx = id as usize;
if schedule.co[idx].is_none() {
return;
}

let status = schedule.co[idx]
.as_ref()
.map(|c| c.status)
.unwrap_or(COROUTINE_DEAD);

match status {
COROUTINE_READY | COROUTINE_SUSPEND => {
schedule.running = id;

let mut co = schedule.co[idx].take().expect("coroutine should exist");
co.status = COROUTINE_RUNNING;

let func = co.func;
let data = co.data.as_mut();

func(schedule, data);

let suspended = schedule.running == -1;
schedule.running = -1;

if suspended {
co.status = COROUTINE_SUSPEND;
schedule.co[idx] = Some(co);
} else {
schedule.nco = schedule.nco.saturating_sub(1);
schedule.co[idx] = None;
}
}
_ => {
panic!("assertion failed");
}
}
}

pub fn coroutine_yield(schedule: &mut Schedule) {
let id = schedule.running;
assert!(id >= 0);

schedule.running = -1;
}

pub fn coroutine_status(schedule: &Schedule, id: i32) -> i32 {
assert!(id >= 0 && (id as usize) < schedule.cap);
match &schedule.co[id as usize] {
None => COROUTINE_DEAD,
Some(co) => co.status,
}
}

pub fn coroutine_running(schedule: &Schedule) -> i32 {
schedule.running
}
