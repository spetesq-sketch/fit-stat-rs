use BMI_calculator::{HealthReport, Unit, User};

fn main() {
    let mut user = User::new(BMI_calculator::Gender::Male, 75, 175.0, 78.35, None, None).unwrap();
    user.add_weight(70.0, None);
    user.add_weight(99.0, None);
    user.add_weight(65.0, None);
    let r = user.get_health_report();
    println!("{:?}", r);
    println!("{:?}", user.weight_history(None));
}
