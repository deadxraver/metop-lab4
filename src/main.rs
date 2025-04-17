use crate::function_info::f;

mod coordinate_descent;
mod function_info;

fn main() {
    let eps: f64 = 0.0001;
    println!("Метод покоординатного спуска:");
    let m = coordinate_descent::find_min(3., 3., eps);
    println!(
        "[x1, x2] = {:?}; f(x1, x2) = {:.3}",
        m,
        f(*m.get(0).expect(""), *m.get(1).expect(""))
    );
}
