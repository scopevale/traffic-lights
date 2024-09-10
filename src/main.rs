use std::process::exit;

#[derive(Clone, Debug, PartialEq)]
enum TrafficLight {
    Green,
    Yellow,
    Red,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self::Green
    }

    pub fn next(self) -> Self {
      match self {
        Self::Green => Self::Yellow,
        Self::Yellow => Self::Red,
        Self::Red => Self::Green
      }
    }  

    //pub fn fmt(self) -> String {
    //    match self {
    //        Self::Green => "Green".to_string(),
    //        Self::Yellow => "Yellow".to_string(),
    //        Self::Red => "Red".to_string()
    //    }
    //}
}

fn main() {
  let light = TrafficLight::new(); // TrafficLight<Green>
  println!("Traffic Light is currently {:?}",light);
  let light = light.next();                // TrafficLight<Yellow>
  println!("Traffic Light is currently {:?}",light);
  let light = light.next();                // TrafficLight<Red>
  println!("Traffic Light is currently {:?}",light);
  let light = light.next();                // TrafficLight<Green>
  println!("Traffic Light is currently {:?}",light);
  
  let light_a = light.clone().next();
  println!("Traffic Light is still {:?}",light);
  println!("Next Traffic Light is {:?}", light_a);

  if light == TrafficLight::Green {
      println!("Traffic Light is currently {:?}", TrafficLight::Green);
      exit(0)
  }
} 

