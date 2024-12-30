use enums::*;

#[test]
fn test_light_duration_red() {
    let red = TrafficLight::Red(30);
    assert_eq!(light_duration(red), 30);
}

#[test]
fn test_light_duration_yellow() {
    let yellow = TrafficLight::Yellow(10);
    assert_eq!(light_duration(yellow), 10);
}

#[test]
fn test_light_duration_green() {
    let green = TrafficLight::Green(60);
    assert_eq!(light_duration(green), 60);
}
