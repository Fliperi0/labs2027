use std::f64::consts::PI;
use std::io::{self, Write};

const EPS: f64 = 1e-12;

/// Вещественные корни линейного уравнения b*x + c = 0
fn solve_linear(b: f64, c: f64) -> Vec<f64> {
    if b.abs() < EPS {
        return vec![]; // либо нет корней, либо бесконечно много
    }
    vec![-c / b]
}

/// Вещественные корни квадратного уравнения a*x^2 + b*x + c = 0
fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a.abs() < EPS {
        return solve_linear(b, c);
    }
    let d = b * b - 4.0 * a * c;
    if d.abs() < EPS {
        vec![-b / (2.0 * a)]
    } else if d > 0.0 {
        let s = d.sqrt();
        vec![(-b - s) / (2.0 * a), (-b + s) / (2.0 * a)]
    } else {
        vec![]
    }
}

/// Вещественные корни кубического уравнения a*x^3 + b*x^2 + c*x + d = 0
fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
    if a.abs() < EPS {
        return solve_quadratic(b, c, d);
    }

    // Приводим к виду x^3 + B x^2 + C x + D = 0
    let (bb, cc, dd) = (b / a, c / a, d / a);

    // Подстановка x = t - B/3 даёт t^3 + p t + q = 0
    let p = cc - bb * bb / 3.0;
    let q = 2.0 * bb.powi(3) / 27.0 - bb * cc / 3.0 + dd;
    let shift = bb / 3.0;

    // Дискриминант: D = (q/2)^2 + (p/3)^3
    let disc = (q / 2.0).powi(2) + (p / 3.0).powi(3);
    let scale = (q / 2.0).powi(2).abs().max((p / 3.0).powi(3).abs()).max(1.0);

    let mut roots: Vec<f64> = if disc.abs() < EPS * scale {
        // Кратные корни
        if p.abs() < EPS {
            vec![0.0] // тройной корень
        } else {
            vec![3.0 * q / p, -3.0 * q / (2.0 * p)]
        }
    } else if disc > 0.0 {
        // Один вещественный корень (формула Кардано)
        let sq = disc.sqrt();
        let u = (-q / 2.0 + sq).cbrt();
        let v = (-q / 2.0 - sq).cbrt();
        vec![u + v]
    } else {
        // Три различных вещественных корня (тригонометрическая формула)
        let m = 2.0 * (-p / 3.0).sqrt();
        let arg = (3.0 * q / (2.0 * p)) * (-3.0 / p).sqrt();
        let theta = arg.clamp(-1.0, 1.0).acos() / 3.0;
        (0..3)
            .map(|k| m * (theta - 2.0 * PI * k as f64 / 3.0).cos())
            .collect()
    };

    for r in roots.iter_mut() {
        *r -= shift;
    }
    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    roots.dedup_by(|x, y| (*x - *y).abs() < 1e-9);
    roots
}

fn read_f64(name: &str) -> f64 {
    loop {
        print!("{} = ", name);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match s.trim().replace(',', ".").parse::<f64>() {
            Ok(v) => return v,
            Err(_) => println!("Введите число."),
        }
    }
}

fn main() {
    println!("Решение уравнения a·x³ + b·x² + c·x + d = 0 (вещественные корни)");
    let a = read_f64("a");
    let b = read_f64("b");
    let c = read_f64("c");
    let d = read_f64("d");

    let roots = solve_cubic(a, b, c, d);
    if roots.is_empty() {
        println!("Вещественных корней нет (или решений бесконечно много).");
    } else {
        for (i, r) in roots.iter().enumerate() {
            println!("x{} = {:.10}", i + 1, r);
        }
    }
}
