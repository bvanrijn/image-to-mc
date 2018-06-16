use rgb;

#[allow(non_snake_case)]
pub fn color_difference(a: rgb::RGB8, b: rgb::RGB8) -> f64 {
    let _a = [a.r, a.g, a.b];
    let _b = [b.r, b.g, b.b];

    let R_1: i32 = (_a[0]).into();
    let G_1: i32 = (_a[1]).into();
    let B_1: i32 = (_a[2]).into();
    let R_2: i32 = (_b[0]).into();
    let G_2: i32 = (_b[1]).into();
    let B_2: i32 = (_b[2]).into();

    let D_R: i32 = (R_1 - R_2).into();
    let D_G: i32 = (G_1 - G_2).into();
    let D_B: i32 = (B_1 - B_2).into();

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
    fn test_color_difference() {
        let a = rgb::RGB { r: 0, g: 0, b: 0 };
        let b = rgb::RGB { r: 255, g: 255, b: 255 };

        assert_eq!(color_difference(a, b), 765.0);
    }
}
