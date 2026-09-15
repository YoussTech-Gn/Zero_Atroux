#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time_in_sec(&self) -> u8{
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 60,
            // what the difference between Self and TrafficLight in this context?
            // Self::Red => 60,
            // Self::Yellow => 5,
            // Self::Green => 60,
        }
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("Time for {:?} light: {} seconds", light, light.time_in_sec());
}