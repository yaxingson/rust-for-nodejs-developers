fn sum(nums:&[i32]) -> i32 {
  nums.iter().sum()
}

fn main() {
  let total = sum(&[1, 2, 3, 4, 5]);   
  println!("{}", total);
}

