// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Ensure that kani::any and kani::any_raw can be used with integers.

macro_rules! test {
    ( $type: ty ) => {{
        let v1 = kani::any::<$type>();
        let v2 = kani::any::<$type>();
        kani::expect_fail(v1 == v2, "This may not be true");
        kani::expect_fail(v1 != v2, "This may also not be true");
        kani::expect_fail(!v1.is_nan(), "NaN should be valid float");
        kani::expect_fail(!v1.is_subnormal(), "Subnormal should be valid float");
        kani::expect_fail(!v1.is_normal(), "Normal should be valid float");
        kani::expect_fail(v1.is_finite(), "Non-finite numbers are valid float");
    }};
}

#[kani::proof]
fn check_f32() {
    test!(f32);
}

#[kani::proof]
fn check_f64() {
    test!(f64);
}
