use std::time::Duration;

use candid::Principal;
use getrandom::register_custom_getrandom;
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

use crate::mutate_rng;

async fn set_rand() {
    let (seed,) = ic_cdk::call(Principal::management_canister(), "raw_rand", ())
        .await
        .unwrap();
    mutate_rng(|rng| {
        *rng = StdRng::from_seed(seed);
    });
}

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    mutate_rng(|rng| {
        rng.fill_bytes(buf);
        Ok(())
    })
}

pub fn init_ic_rand() {
    ic_cdk_timers::set_timer(Duration::from_secs(0), || ic_cdk::spawn(set_rand()));
    register_custom_getrandom!(custom_getrandom);
}

pub fn gen_bool() -> bool {
    mutate_rng(|rng| {
        let rand_value: f64 = rng.gen_range(0f64..100f64);
        rng.gen_bool(rand_value)
    })
}
