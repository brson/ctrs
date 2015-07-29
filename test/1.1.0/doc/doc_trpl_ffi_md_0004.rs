#![feature(libc)]
extern crate libc;
use libc::{size_t, c_int};
unsafe fn snappy_uncompress(compressed: *const u8,
                            compressed_length: size_t,
                            uncompressed: *mut u8,
                            uncompressed_length: *mut size_t) -> c_int { 0 }
unsafe fn snappy_uncompressed_length(compressed: *const u8,
                                     compressed_length: size_t,
                                     result: *mut size_t) -> c_int { 0 }
fn main() {}
pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen: size_t = 0;
        snappy_uncompressed_length(psrc, srclen, &mut dstlen);

        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        if snappy_uncompress(psrc, srclen, pdst, &mut dstlen) == 0 {
            dst.set_len(dstlen as usize);
            Some(dst)
        } else {
            None // SNAPPY_INVALID_INPUT
        }
    }
}
