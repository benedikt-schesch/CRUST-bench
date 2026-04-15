pub const PAGE_SIZE: i32 = 4096;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RC {
Ok = 0,
FileNotFound = 1,
FileHandleNotInit = 2,
WriteFailed = 3,
ReadNonExistingPage = 4,
RmCompareValueOfDifferentDatatype = 200,
RmExprResultIsNotBoolean = 201,
RmBooleanExprArgIsNotBoolean = 202,
RmNoMoreTuples = 203,
RmNoPrintForDatatype = 204,
RmUnknownDatatype = 205,
ImKeyNotFound = 300,
ImKeyAlreadyExists = 301,
ImNToLarge = 302,
ImNoMoreEntries = 303,
MemoryAllocationFail = 401,
BufferpoolInUse = 402,
CloseFailed = 403,
Error = 404,
BufferpoolFull = 405,
ReadFailed = 406,
InvalidHeader = 407,
SeekFailed = 408,
DestroyFailed = 409,
RecordNotFound = 410,
GeneralError = 411,
ShutdownWithoutInit = 420,
LoggingSetupFailure = 430,
}

pub fn throw(rc: RC, _message: &str) -> i32 {
rc as i32
}

pub fn check(code: i32) {
if code != RC::Ok as i32 {
panic!("Operation returned error code: {}", code);
}
}

pub fn print_error(error: RC) {
println!("EC ({})", error as i32);
}

pub fn error_message(error: RC) -> String {
format!("EC ({})\n", error as i32)
}
