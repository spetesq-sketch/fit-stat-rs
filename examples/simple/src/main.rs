use chrono::Utc;
use fit_stats_weight::{Gender, Unit, User};

fn main() {
    let time = Utc::now();
    let user = User::new(
        Gender::Male, // Gender
        20,           // age
        185.0,        // height
        85.0,         // weight
        Some(Unit::Imperial),
        Some(fit_stats::chrono_time(time)),
    )
    .unwrap(); // unwrap because the weight is not 0
    println!("{:#?}", user.get_health_report());
}
