use crate::function_info::{x1_from_zero_der, x2_from_zero_der};

pub fn find_min(x1: f64, x2: f64, eps: f64) -> [f64; 2] {
    let x11 = x1_from_zero_der(x2);
    let x21 = x2_from_zero_der(x1);
    let d = ((x1 - x11).powi(2) - (x2 - x2).powi(2)).sqrt();
    // let f = f(x1, x2);
    // println!("x_1 = {x1:.3}; x_2 = {x2:.3}; f({x1:.3},{x2:.3}) = {f:.3}; d = {d:.3}");
    if d <= eps {
        [x11, x21]
    } else {
        find_min(x11, x21, eps)
    }
}
