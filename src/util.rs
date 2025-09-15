use rand::Rng;

pub fn crecimiento_gompertz(a: f64, b: f64, k: f64) -> impl Fn(u32) -> f64 {
    move |edad: u32| {
        let t = edad as f64;
        a * (-b * (-k * t).exp()).exp()
    }
}

pub fn probabilidad(p: f64, rng: &mut impl Rng) -> bool {
    rng.gen_bool(p)
}
