// 足し算と引き算

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
    let q1 = Quaternion::new(1.0, 2.0, 1.0, 4.0);
    let q2 = Quaternion::new(5.0, 3.0, 4.0, -2.0);
    let result1 = q1 + q2;
    println!("q1 + q2 = {}", result1.display());

    let q3 = Quaternion::new(5.0, 6.0, 2.0, -3.0);
    let q4 = Quaternion::new(3.0, 2.0, 3.0, 1.0);
    let result2 = q3 - q4;
    println!("q3 - q4 = {}", result2.display());
}
