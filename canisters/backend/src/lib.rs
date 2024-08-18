mod image_gen;
mod random_value;
mod utils;
use std::cell::RefCell;

use candid::{CandidType, Decode, Encode, Nat};
use ic_cdk::{init, post_upgrade, query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use image_gen::gen_image;
use rand::rngs::StdRng;
use random_value::init_ic_rand;
use serde::Deserialize;
use utils::CaptchaRequirement;

// BoundedNat
#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BoundedNat(Nat);

impl Storable for BoundedNat {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 100,
        is_fixed_size: false,
    };
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CaptchaDetail {
    pub ans: String,
    pub image: String,
}

impl Storable for CaptchaDetail {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

type Memory = VirtualMemory<DefaultMemoryImpl>;

fn default_captcha_map() -> StableBTreeMap<BoundedNat, CaptchaDetail, Memory> {
    StableBTreeMap::init(MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(1))))
}

struct CaptchaState {
    count: Nat,
    captchas: StableBTreeMap<BoundedNat, CaptchaDetail, Memory>,
}

impl CaptchaState {
    fn get_id(&mut self) -> Nat {
        self.count += Nat::from(1u128);
        self.count.clone()
    }
}

thread_local! {
    pub static RNG: RefCell<Option<StdRng>> = const { RefCell::new(None) };
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static CAPTCHA_STATE: RefCell<CaptchaState> =  RefCell::new(CaptchaState{
        count: Nat::from(0u128),
        captchas: default_captcha_map(),
    });
}

pub fn read_rng<F, R>(f: F) -> R
where
    F: FnOnce(&StdRng) -> R,
{
    RNG.with_borrow(|rng| f(rng.as_ref().unwrap()))
}

pub fn mutate_rng<F, R>(mut f: F) -> R
where
    F: FnMut(&mut StdRng) -> R,
{
    RNG.with_borrow_mut(|rng| f(rng.as_mut().unwrap()))
}

fn read_captcha_state<F, R>(f: F) -> R
where
    F: FnOnce(&CaptchaState) -> R,
{
    CAPTCHA_STATE.with_borrow(|state| f(state))
}

fn mutate_captcha_state<F, R>(mut f: F) -> R
where
    F: FnMut(&mut CaptchaState) -> R,
{
    CAPTCHA_STATE.with_borrow_mut(|state| f(state))
}

#[init]
pub fn init() {
    init_ic_rand();
}

#[post_upgrade]
pub fn post_upgrade() {
    init_ic_rand();
}

#[update]
pub fn generate_captcha(arg: CaptchaRequirement) -> (Nat, String) {
    let text = utils::generate_text(arg);
    let image = gen_image(vec![image_gen::TextOverlay {
        text: &text,
        x: 50,
        y: 100,
        font_size: 60.0,
        color: [0, 0, 0], // black color
    }]);
    mutate_captcha_state(|state| {
        let id = BoundedNat(state.get_id());
        state.captchas.insert(
            id.clone(),
            CaptchaDetail {
                ans: text.clone(),
                image: image.clone(),
            },
        );
        (id.0, image.clone())
    })
}

#[derive(CandidType, Deserialize)]
pub struct SolveCaptchaArgs {
    pub id: Nat,
    pub ans: String,
}

#[query]
pub fn solve_captcha(arg: SolveCaptchaArgs) -> bool {
    read_captcha_state(|state| {
        let CaptchaDetail { ans, image: _ } = match state.captchas.get(&BoundedNat(arg.id)) {
            None => ic_cdk::trap("Invalid Captch Id"),
            Some(v) => v,
        };
        ans.to_ascii_lowercase() == arg.ans.to_ascii_lowercase()
    })
}

ic_cdk::export_candid!();
