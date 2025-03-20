use chrono::{Utc, Local};

fn main() {
  let now_utc = Utc::now();
  let now_local = Local::now();

  let formatted_utc = now_utc.format("%Y-%m-%d %H:%M:%S").to_string();
  let formatted_local = now_local.format("%Y-%m-%d %H:%M:%S").to_string();

  println!("Formatted UTC time: {}", formatted_utc);
  println!("Formatted local time: {}", formatted_local);

  let custom_utc = now_utc.format("%A, %B %d, %Y %I:%M %p").to_string();
  let custom_local = now_local.format("%A, %B %d, %Y %I:%M %p").to_string();

  println!("Custom formatted UTC time: {}", custom_utc);
  println!("Custom formatted local time: {}", custom_local);
}
