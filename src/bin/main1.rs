// Quaternion型

use nalgebra::geometry::Quaternion;

fn main() {
    let quaternion = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    println!("{}", quaternion);
}
