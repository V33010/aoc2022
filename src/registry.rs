use crate::days::*;

pub fn get(day: u32) -> Option<fn(&str)> {
    match day {
        3 => Some(day03::run),
        2 => Some(day02::run),
        4 => Some(day04::run),
        1 => Some(day01::run),
        _ => None,
    }
}
