use crate::function_info::{f, x1_grad, x2_grad};

pub fn find_min(x1: f64, x2: f64, eps: f64, h: f64) -> [f64; 2] {
    let x11: f64 = x1 - h * x1_grad(x1, x2);
    let x21: f64 = x2 - h * x2_grad(x1, x2);
    let f1 = f(x11, x21);
    let f = f(x1, x2);
    if f1 > f {
        find_min(x1, x2, eps, h / 2.)
    } else if f - f1 <= eps {
        [x11, x21]
    } else {
        find_min(x11, x21, eps, h)
    }
}