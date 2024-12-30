use vehicle_status::*;

#[test]
fn test_is_operational() {
    let parked = VehicleStatus::Parked;
    assert!(parked.is_operational());

    let driving = VehicleStatus::Driving { speed: 80 };
    assert!(driving.is_operational());

    let broken_down = VehicleStatus::BrokenDown("Flat tire".to_string());
    assert!(!broken_down.is_operational());
}

#[test]
fn test_description_parked() {
    let parked = VehicleStatus::Parked;
    assert_eq!(parked.description(), "The vehicle is parked.");
}

#[test]
fn test_description_driving() {
    let driving = VehicleStatus::Driving { speed: 120 };
    assert_eq!(driving.description(), "The vehicle is driving at 120 km/h.");
}

#[test]
fn test_description_broken_down() {
    let broken_down = VehicleStatus::BrokenDown("Engine failure".to_string());
    assert_eq!(
        broken_down.description(),
        "The vehicle is broken down: Engine failure."
    );
}

#[test]
fn test_mixed_scenarios() {
    let parked = VehicleStatus::Parked;
    assert!(parked.is_operational());
    assert_eq!(parked.description(), "The vehicle is parked.");

    let driving = VehicleStatus::Driving { speed: 50 };
    assert!(driving.is_operational());
    assert_eq!(driving.description(), "The vehicle is driving at 50 km/h.");

    let broken_down = VehicleStatus::BrokenDown("No fuel".to_string());
    assert!(!broken_down.is_operational());
    assert_eq!(
        broken_down.description(),
        "The vehicle is broken down: No fuel."
    );
}
