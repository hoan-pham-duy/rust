#![allow(unused)]
pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
    const num_car:f64 = 221.0;
    match(speed){
        0 => 0.0,
        1..=4 => speed as f64 * num_car,
        5..=8 => speed as f64 * num_car*0.9,
        9..=10 => speed as f64 * num_car*0.77,
        _ => panic!("Un catch production_rate_per_hour")
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    (production_rate_per_hour(speed)/60.0) as u32
}
