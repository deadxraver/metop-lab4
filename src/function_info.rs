pub fn f(x1: f64, x2: f64) -> f64 {
    x1.powi(2) + 3. * x2.powi(2) + 3. * x1 * x2 - x1 - 2. * x2 - 1.
}

pub fn x1_from_zero_der(x2: f64) -> f64 {
    (1. - 3. * x2) / 2.
}

pub fn x2_from_zero_der(x1: f64) -> f64 {
    (2. - 3. * x1) / 6.
}

pub fn x1_grad(x1: f64, x2: f64) -> f64 {
    2. * x1 + 3. * x2 - 1.
}

pub fn x2_grad(x1: f64, x2: f64) -> f64 {
    6. * x2 + 3. * x1 - 2.
}