// 3次元回転

use nalgebra::geometry::Quaternion;
use nalgebra::{Point3, UnitQuaternion, Vector3};

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
    // x軸のまわりの45度回転をあらわすクォータニオン
    let alpha = UnitQuaternion::from_axis_angle(
        &Vector3::x_axis(),
        45.0 * std::f64::consts::PI / 180.0,
    );
    println!("alpha = {}", alpha.display());
    // 3次元空間の点
    let beta = Point3::new(0.0, 1.0, 0.0);
    println!("beta = {}", beta);
    // 3次元空間の点を回転させる
    let result = alpha.transform_point(&beta);
    println!("result = {}", result);
}
