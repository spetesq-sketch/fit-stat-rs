use fit_stats::{Gender, Unit, User, UserData, UserError};

#[test]
fn create_user() {
    let user = User::new(Gender::Female, 15, 165.0, 45.0, None, None);
    assert!(user.is_ok())
}

#[test]
fn create_user_wrong_weight() {
    let user = User::new(Gender::Female, 15, 165.0, 0.0, None, None);
    assert!(user.is_err())
}

#[test]
fn create_user_wrong_height() {
    let user = User::new(Gender::Female, 15, 0.0, 50.0, None, None);
    assert!(user.is_err())
}

#[test]
fn change_user_data() {
    let mut user = User::new(Gender::Female, 15, 165.0, 50.0, None, None).unwrap();
    let new_data = UserData {
        age: 124,
        gender: Gender::Male,
        height: 155.0,
        unit: Unit::Metric,
    };
    let result = user.change_user_data(new_data);
    assert!(result.is_ok())
}

#[test]
fn change_user_data_wrong_height() {
    let mut user = User::new(Gender::Female, 15, 165.0, 50.0, None, None).unwrap();
    let new_data = UserData {
        age: 124,
        gender: Gender::Male,
        height: 0.0,
        unit: Unit::Metric,
    };
    let result = user.change_user_data(new_data);
    assert!(result.is_err())
}

#[test]
fn add_weight_ok() -> Result<(), UserError> {
    let mut user = User::new(Gender::Female, 15, 165.0, 50.0, None, None)?;
    user.add_weight(100.0, None)?;
    assert_eq!(user.get_last_weight(), 100.0);
    assert_eq!(user.weight_history(None).len(), 2);

    user.add_weight(101.0, None)?;
    user.add_weight(102.0, None)?;
    user.add_weight(103.0, None)?;

    assert_eq!(user.weight_history(Some(2)).len(), 2);
    assert_eq!(user.weight_history(Some(3)).len(), 3);
    Ok(())
}

#[test]
fn add_weight_wrong() {
    let mut user = User::new(Gender::Female, 15, 165.0, 50.0, None, None).unwrap();
    let result = user.add_weight(0.0, None);
    assert!(result.is_err())
}
