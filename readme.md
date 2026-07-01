# BMI Calculator (no_std Rust library)

A lightweight `no_std` Rust library for calculating BMI and generating basic health reports.

It supports both **Metric** and **Imperial** units and provides simple health classification based on BMI and age adjustment.

---

## Features

- BMI calculation (Oxford-style formula)
- Support for Metric and Imperial units
- User weight history tracking
- Basic health status classification
- Age-adjusted BMI interpretation
- `no_std` compatible (uses `alloc`)
- Optional `serde` / `bincode` / `chrono` support via features

---

## Health classification

The library categorizes BMI into:

Severe weight deficit
Underweight
Normal weight
Overweight
Obesity

Classification is slightly adjusted based on age.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fit_stats = { version = "*", features = ["serde", "bincode", "chrono"] }
```

## Usage

You can check examples/ but there are few now

### Warning

If you use the imperial system, use inches for height

```rust

use chrono;
use fit_stats::{Gender, Unit, User};

fn main() {
    let time = chrono::Utc::now();
    let user = User::new(
        Gender::Male, // Gender
        20,           // age
        185.0,        // height
        85.0,         // weight
        Some(Unit::Metric),
        Some(fit_stats::chrono_time(time)),
    )
    .unwrap(); // unwrap because the weight is not 0
    println!("{:#?}", user.get_health_report());
}
```

### Result

```
HealthReport {
    bmi: 23.737387,
    target_weight: 76.25,
    age_offset: 19.0,
    actual_bmi: 23.737387,
    extra_weight: 8.75,
    status: NormalWeight,
}
```

---

## Future

I also want to add tracking of fat content, approximate calorie consumption, and so on, but that's for the future.
