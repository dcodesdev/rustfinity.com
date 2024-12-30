use enums::*;

#[test]
fn test_light_action_red() {
    let red = TrafficLight::Red;
    assert_eq!(light_action(&red), "Stop");
}

#[test]
fn test_light_action_yellow() {
    let yellow = TrafficLight::Yellow;
    assert_eq!(light_action(&yellow), "Caution");
}

#[test]
fn test_light_action_green() {
    let green = TrafficLight::Green;
    assert_eq!(light_action(&green), "Go");
}
