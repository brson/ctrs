fn main() {
    let s = "Löwe 老虎 Léopard";
    
    unsafe {
        assert_eq!(s.slice_unchecked(0, 21), "Löwe 老虎 Léopard");
    }
}
