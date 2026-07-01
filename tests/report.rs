use fit_stats_weight::{Gender, Unit, User};

#[test]
fn health_report_is_created() {
    let user = User::new(Gender::Male, 25, 180.0, 80.0, Some(Unit::Metric), None).unwrap();

    let report = user.get_health_report();

    assert!(report.bmi > 20.0);
    assert!(report.bmi < 30.0);
}
