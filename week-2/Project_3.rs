fn main() {
   let p:f64 = 210_000.0;
   let r:f64 = 5.0;
   let n:f64 = 3.0;

   //compound intrest for depreciation
   let a = p * (1.0 - (r/100.0)) * n;
   println!("Amount is {}", a);

   //depreciated value
   let ci = a - p;
   println!("compound intrest is {}", ci);

   }