#![feature(asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() { unsafe {
// Put the value 0x200 in eax
asm!("mov $$0x200, %eax" : /* no outputs */ : /* no inputs */ : "{eax}");
} }
