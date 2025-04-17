use crate::function_info::f;

mod coordinate_descent;
mod function_info;
mod gradient_descent;

fn main() {
    let eps: f64 = 0.0001;
    println!("Метод покоординатного спуска:");
    let m_coord = coordinate_descent::find_min(3., 3., eps);
    println!(
        "[x1, x2] = {:?}; f(x1, x2) = {:.3}",
        m_coord,
        f(*m_coord.get(0).expect(""), *m_coord.get(1).expect(""))
    );
    println!("Метод градиентного спуска:");
    let m_grad = gradient_descent::find_min(1., 0., eps, 0.25);
    println!(
        "[x1, x2] = {:?}; f(x1, x2) = {:.3}",
        m_grad,
        f(*m_grad.get(0).expect(""), *m_grad.get(1).expect(""))
    );
}
