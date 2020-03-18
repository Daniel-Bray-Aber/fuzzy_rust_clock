extern crate chrono;
use chrono::prelude::*;

fn main() {
    let dt = Local::now();
    let time = (dt.hour(), dt.minute(), dt.second());
    println!("{}", get_fuzz(time));


}


fn get_fuzz(time : (u32, u32, u32)) -> String {

    let m : f32 = time.1 as f32 + (time.2 as f32 /60.0);
    if m < 2.5 {
        return format!("{} o'clock", num_to_text(time.0))
    } else if m  < 7.5 {
        return format!("five past {}", num_to_text(time.0))
    } else if m < 12.5 {
        return format!("ten past {}", num_to_text(time.0))
    } else if m  < 17.5 {
        return format!("quater past {}", num_to_text(time.0))
    } else if m < 22.5 {
        return format!("twenty past {}", num_to_text(time.0))
    } else if m < 27.5 {
        return format!("twenty-five past {}", num_to_text(time.0))
    } else if m < 32.5 {
        return format!("half past {}", num_to_text(time.0))
    } else if m < 37.5 {
        return format!("twenty-five to {}", num_to_text(time.0 + 1))
    } else if m < 42.5 {
        return format!("twenty to {}", num_to_text(time.0 + 1))
    } else if m < 47.5 {
        return format!("quarter to {}", num_to_text(time.0 + 1))
    } else if m < 52.5 {
        return format!("ten to {}", num_to_text(time.0 + 1))
    } else if m < 57.5 {
        return format!("five to {}", num_to_text(time.0 + 1))
    } else if m < 60.0 {
        return format!("{} o'clock", num_to_text(time.0 + 1))
    }
    String::from("failure")
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
