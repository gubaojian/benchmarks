#[derive(Deserialize)]
pub struct Coordinate {
  x: f64,
  y: f64,
  z: f64,
  #[allow(dead_code)]
  name: Skip,
  #[allow(dead_code)]
  opts: Skip,
} 
