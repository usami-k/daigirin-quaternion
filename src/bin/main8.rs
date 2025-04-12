// 割り算

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
    let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let q2 = Quaternion::new(1.0, -1.0, 1.0, 1.0);
    let result = q1 * q2.try_inverse().unwrap();
    println!("q1 / q2 = {}", result.display());
}
