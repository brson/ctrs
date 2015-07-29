#![feature(asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() { unsafe {
asm!("xor %eax, %eax" ::: "{eax}");
} }
