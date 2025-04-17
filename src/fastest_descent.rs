use crate::function_info::{f, x1_grad, x2_grad};

pub fn find_min(x1: f64, x2: f64, eps: f64) -> [f64; 2] {
    let grad1 = x1_grad(x1, x2);
    let grad2 = x2_grad(x1, x2);
    let grad_abs = (grad1.powi(2) + grad2.powi(2)).sqrt();
    if grad_abs < eps {
        [x1, x2]
    } else {
        let [s1, s2] = [grad1 / grad_abs, grad2 / grad_abs];
        let mut h1 = -10f64;
        let mut h2 = 10f64;
        let mut h: f64 = (h2 + h1) / 2.;
        let half_div_eps = eps;
        while h2 - h1 > half_div_eps {
            h = (h2 + h1) / 2.;
            if f(x1 - (h - half_div_eps) * s1, x2 - (h - half_div_eps) * s2)
                < f(x1 - (h + half_div_eps) * s1, x2 - (h + half_div_eps) * s2)
            {
                h2 = h;
            } else {
                h1 = h;
            }
        }
        find_min(x1 - h * s1, x2 - h * s2, eps)
    }
}
