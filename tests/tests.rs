use chrono::{TimeZone, Utc};
use cron_parser::{parse, parse_field};
use std::collections::BTreeSet;

macro_rules! parse_field_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, min, max, expected) = $value;
                let mut expect = BTreeSet::<u32>::new();
                for i in expected {
                    expect.insert(i);
                }
                assert_eq!(parse_field(input, min, max).unwrap(), expect);
            }
        )*
    }
}

parse_field_tests! {
    parse_any:("*", 0, 0, vec![0]),
    parse_minutes_0: ("0", 0, 59, vec![0]),
    parse_minutes_1: ("1", 0, 59, vec![1]),
    parse_hours: ("23", 0, 23, vec![23]),
    parse_days: ("31", 0, 31, vec![31]),
    parse_day_week: ("6", 0, 6,  vec![6]),
    parse_every_30: ("*/30", 0, 59, vec![0,30]),
    parse_every_5_minutes: ("*/5", 0, 59, vec![0,5,10,15,20,25,30,35,40,45,50,55]),
    parse_every_minute: ("*", 0, 59, vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59]),
    parse_every_minute_step: ("*/1", 0, 59, vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59]),
    parse_every_hour: ("*", 0, 23, vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23]),
    parse_every_hour_step: ("*/1", 0, 23, vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23]),
    parse_every_day: ("*", 1, 31, vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31]),
    parse_every_day_step: ("*/1", 1, 31, vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31]),
    parse_every_month: ("*", 1, 12, vec![1,2,3,4,5,6,7,8,9,10,11,12]),
    parse_every_month_step: ("*/1", 1, 12, vec![1,2,3,4,5,6,7,8,9,10,11,12]),
    parse_every_dweek: ("*", 0, 6, vec![0,1,2,3,4,5,6]),
    parse_every_dweek_step: ("*/1", 0, 6, vec![0,1,2,3,4,5,6]),
    parse_range_5_10_minutes: ("5-10", 0, 59, vec![5,6,7,8,9,10]),
    parse_range_5_10_hours: ("5-10", 0, 12, vec![5,6,7,8,9,10]),
    parse_list_minutes: ("15,30,45,0", 0, 59, vec![0,15,30,45]),
    parse_1024: ("1024", 0, 1024, vec![1024]),
    parse_repeat_values:("1,1,1,1,2", 0, 59, vec![1,2]),
    parse_range_and_list1: ("1-8,11", 0, 23, vec![1,2,3,4,5,6,7,8,11]),
    parse_range_and_list2: ("1-8,11,9,4,5", 0, 23, vec![1,2,3,4,5,6,7,8,9,11]),
    parse_range_and_list3: ("*,1-8,11,9,4,5", 0, 23, vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23]),
    parse_range_and_list4: ("2-3,*/15", 0, 23, vec![0,2,3,15]),
    parse_range_and_list5: ("2-3,9,*/15,1-8,11,9,4,5", 0, 23, vec![0,1,2,3,4,5,6,7,8,9,11,15]),
    parse_range_list_step: ("*/30,40-45,57", 0, 59, vec![0,30,40,41,42,43,44,45,57]),
    parse_range_list_step_repeated_values: ("*/30,40-45,57,30,44,41-45", 0, 59, vec![0,30,40,41,42,43,44,45,57]),
}

macro_rules! parse_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, ts, expected) = $value;
                let dt = Utc.timestamp(ts, 0);
                assert_eq!(parse(input, dt).unwrap().timestamp(), expected);
            }
        )*
    }
}

/// https://play.rust-lang.org/
/// ```ignore
/// extern crate chrono; // 0.4.9
/// use chrono::{TimeZone, Utc};
/// fn main() {
///     // 2019-11-05 15:56:35 UTC 1572969395
///     let dt = Utc.timestamp(1572969395, 0);
///     println!("{} {}", dt, dt.timestamp());
///     let next = Utc.ymd(2019, 11, 5).and_hms(16, 5, 0);
///     println!("{} {}", next, next.timestamp());
/// }
/// ```
parse_tests! {
    any_minute: ("* * * * *", 1572969395, 1572969420),
    any_minute2: ("*/5,* * * * *", 1572969395, 1572969420),
    every_5_mintues: ("*/5 * * * *", 1572969395, 1572969600),
    on_minute_5: ("5 * * * *", 1572969395, 1572969900),
//    every_minute_every_2nd_hour: ("* */2 * * *", 1572969395, 1572969600),
    every_minute_in_october: ("* * * 10 *", 1572969395, 1601510400),
    every_minute_on_day_4_in_november: ("* * 4 11 *", 1572969395, 1604448000),
}

#[test]
fn parse_field_double_field() {
    assert!(parse_field("**", 0, 0).is_err());
}

#[test]
fn parse_field_bad_range() {
    assert!(parse_field("1-2-3", 0, 0).is_err(),);
    assert!(parse_field("8-5", 0, 0).is_err(),);
}

#[test]
fn bad_minute_input() {
    assert!(parse_field("60", 0, 59).is_err(),);
}

#[test]
fn bad_minute_input_range() {
    assert!(parse_field("5-60", 0, 59).is_err());
}

#[test]
fn bad_minute_input_list() {
    assert!(parse_field("40,50,60", 0, 59).is_err());
}

#[test]
fn bad_hour_input_step() {
    assert!(parse_field("*/30", 0, 23).is_err());
}

#[test]
fn test_parse() {
    let next = parse("*/5 * * * *", Utc::now()).unwrap();
}