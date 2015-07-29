#![feature(libc)]
extern crate libc;
use libc::{size_t, c_int};
unsafe fn snappy_compress(a: *const u8, b: size_t, c: *mut u8,
                          d: *mut size_t) -> c_int { 0 }
unsafe fn snappy_max_compressed_length(a: size_t) -> size_t { a }
fn main() {}
pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen = snappy_max_compressed_length(srclen);
        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        snappy_compress(psrc, srclen, pdst, &mut dstlen);
        dst.set_len(dstlen as usize);
        dst
    }
}
