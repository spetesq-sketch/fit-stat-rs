use BMI_calculator::{HealthReport, User};

fn main() {
    let user = User::new(BMI_calculator::Gender::Male, 16, 175.0, 78.35, None);
    if let Ok(rep) = user.get_health_report() {
        println!("{:?}", rep);
    }
}
