#[allow(non_snake_case)]
pub fn color_difference(a: &mut [i32], b: &mut [i32]) -> f64 {
    let (R_1, R_2, G_1, G_2, B_1, B_2);

    R_1 = a[0];
    G_1 = a[1];
    B_1 = a[2];
    R_2 = b[0];
    G_2 = b[1];
    B_2 = b[2];

    let D_R = R_1 - R_2;
    let D_G = G_1 - G_2;
    let D_B = B_1 - B_2;

    let D_R_S: f64 = (i32::pow(D_R, 2)).into();
    let D_G_S: f64 = (i32::pow(D_G, 2)).into();
    let D_B_S: f64 = (i32::pow(D_B, 2)).into();

    let r_: f64 = ((R_1 + R_2) / 2).into();

    let f = 2.0 * D_R_S + 4.0 * D_G_S + 3.0 * D_B_S + (r_ * (D_R_S - D_B_S)) / 256.0;

    return f64::sqrt(f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = [0, 0, 0];
        let mut b = [255, 255, 255];
        assert_eq!(color_difference(&mut a, &mut b), 765.0);
    }
}
