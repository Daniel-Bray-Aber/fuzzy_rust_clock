extern crate chrono;
use chrono::prelude::*;

fn main() {
    let dt = Local::now();
    let time = (dt.hour(), dt.minute());
    println!("{}", get_fuzz(time));
}

fn get_fuzz(time : (u32, u32)) -> String {
    let mut hr = time.0;
    if time.1 < 33 {
        if time.1 < 13 {
            if time.1 < 3 {
                return format!("{} o'clock", num_to_text(hr))
            } else if time.1 < 8 {
                return format!("five past {}", num_to_text(hr))
            } else {
                return format!("ten past {}", num_to_text(hr))
            }
        } else {
            if time.1 < 23 {
                if time.1 < 18 {
                    return format!("quater past {}", num_to_text(hr))
                } else {
                    return format!("twenty past {}", num_to_text(hr))
                }
            } else {
                if time.1 < 28 {
                    return format!("twenty-five past {}", num_to_text(hr))
                } else {
                    return format!("half past {}", num_to_text(hr))
                }
            }
        }
    } else {
        hr += 1;
        if time.1 < 48 {
            if time.1 < 38 {
                return format!("twenty-five to {}", num_to_text(hr))
            } else if time.1 < 43 {
                return format!("twenty to {}", num_to_text(hr))
            } else {
                return format!("quarter to {}", num_to_text(hr))
            }
        } else {
            if time.1 < 53 {
                return format!("ten to {}", num_to_text(hr))
            } else if time.1 < 58 {
                return format!("five to {}", num_to_text(hr))
            } else {
                return format!("{} o'clock", num_to_text(hr))
            }
        }
    }
}

fn num_to_text(number : u32) -> String {
    let n = number % 12;
    match n {
        0 => String::from("twelve"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        _ => String::from("Failure")
    }
}
