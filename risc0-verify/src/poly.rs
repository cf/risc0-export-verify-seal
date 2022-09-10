use crate::fp4::Fp4;

pub fn poly_eval(coeffs: &[Fp4], x: Fp4) -> Fp4 {
    let mut mul = Fp4::one();
    let mut tot = Fp4::zero();
    for i in 0..coeffs.len() {
        tot += coeffs[i] * mul;
        mul *= x;
    }
    tot
}
