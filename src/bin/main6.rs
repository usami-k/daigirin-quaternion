// ノルム

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
    let q1 = Quaternion::new(1.0, 1.0, 2.0, 3.0);
    println!("q1 = {}", q1.display());
    let result1 = q1.norm();
    println!("|q1| = {}", result1);
    let result2 = q1.norm_squared();
    println!("|q1|^2 = {}", result2);
}
