#![feature(asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() {
let result: i32;
unsafe {
   asm!("mov eax, 2" : "={eax}"(result) : : : "intel")
}
println!("eax is currently {}", result);
}
