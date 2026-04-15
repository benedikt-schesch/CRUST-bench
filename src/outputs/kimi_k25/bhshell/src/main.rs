mod bhshell;
mod dynamicarr;
mod input;
mod xalloc;

use crate::bhshell::bhshell_loop;

/// Entry point of the application.
/// In C, this was 'int main(void)'.
pub fn main() {
bhshell_loop();
}
