pub enum TrafficLight {
    Red(u8),
    Yellow(u8),
    Green(u8),
}

pub fn light_duration(light: TrafficLight) -> u8 {
    match light {
        TrafficLight::Red(duration) => duration,
        TrafficLight::Yellow(duration) => duration,
        TrafficLight::Green(duration) => duration,
    }
}
