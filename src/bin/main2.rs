// コンソール出力のカスタマイズ

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
    let quaternion = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    println!("{}", quaternion.display());
}
