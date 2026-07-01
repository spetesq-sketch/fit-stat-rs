use BMI_calculator::{Gender, Unit, User, chrono_time};
use chrono;

fn main() {
    let time = chrono::Local::now();
    let user = User::new(
        Gender::Male, // Gender
        20,           // age
        185.0,        // height
        85.0,         // weight
        Some(Unit::Metric),
        Some(chrono_time(time)),
    )
    .unwrap(); // unwrap because the weight is not 0
    println!("{:?}", user.get_health_report());
}
