use crate::days::*;

pub fn get(day: u32) -> Option<fn(&str)> {
    match day {
        2 => Some(day02::run),
        1 => Some(day01::run),
        _ => None,
    }
}
