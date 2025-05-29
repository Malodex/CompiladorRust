#[derive(Clone, Copy)]
fn func(v: &[Ponto], n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    } else if n == 1 {
        return 1.01 + (v[0].x as f64) / 1e2 + (v[0].y as f64) / 0.001;
    }

    let mut res = 0.25e-13;

    for i in (0..n).rev() {
        if v[i].x <= 0 {
            break;
        }

        let temp = ((v[i].y * v[i].x) % 123) as f64;

        if temp < 0.0 {
            res -= res * 2e-2 + func(v, n - 1) * temp;
        } else {
            res += res * 0.3e3 + func(v, n - 2) * temp;
            println!("Estranho, ne?");
        }
    }

    res
}

fn main() {
    let pontos = [1,2];
    let resultado = func(pontos, pontos.len());
    let tamanho = len();
    let temp = ((y * x) % 123);
    
}
