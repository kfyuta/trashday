use chrono::{DateTime, Datelike, Local, Weekday};
#[macro_use]
extern crate diesel;

use diesel::prelude::*;

// models.rsとschema.rsのインポート
pub mod schema;
pub mod models;
use dotenv::dotenv;
use std::env;
use crate::models::TrashDay;

fn main() {
    let current_time = Local::now();
    let ddd = NthWeekday::new(current_time);
    println!("{}", ddd.is_nth_weekday());

    use schema::trash_days::dsl::*;

    let connection = establish_connection();
    let resultss = trash_days
        .filter(nth.eq(ddd.nth))
        .load::<TrashDay>(&connection)
        .expect("");
    println!("Displaying {} posts", resultss.len());
    for post in resultss {
        println!("{}の日です", post.trash_type);
    }
}



struct NthWeekday {
  nth: i32,
  weekday: String,
}

impl NthWeekday {
  fn new(date: DateTime<Local>) -> NthWeekday {
    let weekday = match date.date().weekday() {
      Weekday::Mon => String::from("月"),
      Weekday::Tue => String::from("火"),
      Weekday::Wed => String::from("水"),
      Weekday::Thu => String::from("木"),
      Weekday::Fri => String::from("金"),
      Weekday::Sat => String::from("土"),
      Weekday::Sun => String::from("日")
    };
    let nth= calc_nth(date.day() as i32);
    NthWeekday {nth, weekday}
  }

  fn is_nth_weekday(&self) -> String {
    format!("今日は 第{}{}曜日です", self.nth, self.weekday)
  }
}

pub fn calc_nth(n: i32) -> i32 {
  if n % 7 == 0 {
    n / 7 
  } else {
    n / 7 + 1 
  }
}

// Sqliteコネクションを作る。
pub fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[test]
fn nth_weekday_new_test() {
  assert_eq!(1, calc_nth(1));
  assert_eq!(1, calc_nth(7));
  assert_eq!(2, calc_nth(8));
  assert_eq!(2, calc_nth(14));
  assert_eq!(3, calc_nth(15));
  assert_eq!(3, calc_nth(21));
  assert_eq!(4, calc_nth(22));
  assert_eq!(4, calc_nth(28));
  assert_eq!(5, calc_nth(29));
  assert_eq!(5, calc_nth(30));
  assert_eq!(5, calc_nth(31));
}