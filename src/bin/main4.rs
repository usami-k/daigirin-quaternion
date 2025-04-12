// かけ算、交換法則は成り立たない

use nalgebra::geometry::Quaternion;

trait QuaternionDisplay {
    fn display(&self) -> String;
}

impl QuaternionDisplay for Quaternion<f64> {
    fn display(&self) -> String {
        format!(
            "{} + {} i + {} j + {} k",
            self.w, self.i, self.j, self.k
        )
    }
}

fn main() {
    let q1 = Quaternion::new(1.0, 2.0, 0.0, 0.0);
    let q2 = Quaternion::new(0.0, 0.0, 3.0, 4.0);
    let result1 = q1 * q2;
    println!("q1 * q2 = {}", result1.display());
    let result2 = q2 * q1;
    println!("q2 * q1 = {}", result2.display());
}
