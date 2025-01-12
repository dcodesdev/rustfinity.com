pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn light_action(light: &TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Caution",
        TrafficLight::Green => "Go",
    }
}
