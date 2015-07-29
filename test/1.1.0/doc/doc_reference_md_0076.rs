fn main() {
    fn sum(v: &[f64]) -> f64 { 0.0 }
    fn len(v: &[f64]) -> i32 { 0 }
    
    fn avg(v: &[f64]) -> f64 {
      let sum: f64 = sum(v);
      let sz: f64 = len(v) as f64;
      return sum / sz;
    }
}
