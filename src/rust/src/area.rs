use crate::h3::H3;
use extendr_api::prelude::*;

#[extendr]
fn area_km2_(x: List) -> Doubles {
    x.into_iter()
        .map(|(_, x)| Rfloat::from(<&H3>::try_from(&x).unwrap().index.area_km2()))
        .collect::<Doubles>()
}

#[extendr]
fn area_m2_(x: List) -> Doubles {
    x.into_iter()
        .map(|(_, x)| Rfloat::from(<&H3>::try_from(&x).unwrap().index.area_m2()))
        .collect::<Doubles>()
}

#[extendr]
fn area_rads2_(x: List) -> Doubles {
    x.into_iter()
        .map(|(_, x)| Rfloat::from(<&H3>::try_from(&x).unwrap().index.area_rads2()))
        .collect::<Doubles>()
}

extendr_module! {
    mod area;
    fn area_km2_;
    fn area_m2_;
    fn area_rads2_;
}
