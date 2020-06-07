use kes_mmm_sumed25519::sumed25519 as kes;
use std::slice::{from_raw_parts, from_raw_parts_mut};

const DEPTH: kes::Depth = kes::Depth(KES_MMM_SUMED25519_TOTAL_UPDATE_LOG as usize);

pub const KES_MMM_SUMED25519_TOTAL_UPDATE_LOG: u32 = 12;
pub const KES_MMM_SUMED25519_TOTAL_UPDATE: usize = 4096;

pub const KES_MMM_SUMED25519_SECRET_KEY_SIZE: usize = 1220;
pub const KES_MMM_SUMED25519_PUBLIC_KEY_SIZE: usize = 32;
pub const KES_MMM_SUMED25519_SIGNATURE_SIZE: usize = 484;

#[no_mangle]
pub static KES_MMM_SUMED25519_VERSION: &str = kes_mmm_sumed25519::version::PKG;

#[no_mangle]
pub static KES_MMM_SUMED25519_GIT_VERSION: &str = kes_mmm_sumed25519::version::SOURCE;

macro_rules! assert_static_eq {
    ($sz1:expr, $sz2:expr) => {
        const _: fn() = || {
            let _ = core::mem::transmute::<[u8; $sz1], [u8; $sz2]>;
        };
    };
}

assert_static_eq!(
    KES_MMM_SUMED25519_SECRET_KEY_SIZE,
    kes::maximum_secretkey_size(DEPTH)
);
assert_static_eq!(KES_MMM_SUMED25519_PUBLIC_KEY_SIZE, kes::PUBLIC_KEY_SIZE);
assert_static_eq!(
    KES_MMM_SUMED25519_SIGNATURE_SIZE,
    kes::signature_size(DEPTH)
);

#[no_mangle]
pub extern "C" fn kes_mmm_sumed25519_secretkey_generate(
    seed: *const u8,
    secret_ptr: *mut u8,
    public_ptr: *mut u8,
) {
    let in_seed = unsafe { from_raw_parts(seed, kes::Seed::SIZE) };
    let out_sec = unsafe { from_raw_parts_mut(secret_ptr, kes::maximum_secretkey_size(DEPTH)) };
    let out_pub = unsafe { from_raw_parts_mut(public_ptr, kes::PUBLIC_KEY_SIZE) };

    let mut seed_bytes = [0u8; kes::Seed::SIZE];
    seed_bytes.copy_from_slice(in_seed);

    let mut seed = kes::Seed::from_bytes(seed_bytes);
    let (secretkey, publickey) = kes::keygen(DEPTH, &seed);
    seed.set_zero();

    out_sec.copy_from_slice(secretkey.as_ref());
    out_pub.copy_from_slice(publickey.as_ref());
}

#[no_mangle]
pub extern "C" fn kes_mmm_sumed25519_secretkey_sign(
    secret_ptr: *const u8,
    message_ptr: *const u8,
    message_size: usize,
    signature_ptr: *mut u8,
) {
    let in_sec = unsafe { from_raw_parts(secret_ptr, kes::maximum_secretkey_size(DEPTH)) };
    let in_msg = unsafe { from_raw_parts(message_ptr, message_size) };
    let out_sig = unsafe { from_raw_parts_mut(signature_ptr, kes::signature_size(DEPTH)) };

    let sk = kes::SecretKey::from_bytes(DEPTH, in_sec).unwrap();
    let signature = kes::sign(&sk, in_msg);

    out_sig.copy_from_slice(signature.as_ref());
}

#[no_mangle]
pub extern "C" fn kes_mmm_sumed25519_secretkey_t(secret_ptr: *const u8) -> u32 {
    let in_sec = unsafe { from_raw_parts(secret_ptr, kes::maximum_secretkey_size(DEPTH)) };
    let sk = kes::SecretKey::from_bytes(DEPTH, in_sec).unwrap();
    let t = sk.t() as u32;
    t
}

#[no_mangle]
pub extern "C" fn kes_mmm_sumed25519_secretkey_compute_public(
    secret_ptr: *const u8,
    public_ptr: *mut u8,
) {
    let in_sec = unsafe { from_raw_parts(secret_ptr, kes::maximum_secretkey_size(DEPTH)) };
    let out_pub = unsafe { from_raw_parts_mut(public_ptr, kes::PUBLIC_KEY_SIZE) };
    let sk = kes::SecretKey::from_bytes(DEPTH, in_sec).unwrap();
    let publickey = sk.compute_public();
    out_pub.copy_from_slice(publickey.as_ref());
}

#[no_mangle]
pub extern "C" fn kes_mmm_sumed25519_secretkey_update(secret_ptr: *mut u8) {
    let io_sec = unsafe { from_raw_parts_mut(secret_ptr, kes::maximum_secretkey_size(DEPTH)) };
    let mut sk = kes::SecretKey::from_bytes(DEPTH, io_sec).unwrap();
    kes::update(&mut sk).unwrap();
    io_sec.copy_from_slice(sk.as_ref());
}

#[no_mangle]
pub extern "C" fn kes_mmm_sumed25519_publickey_verify(
    public_ptr: *const u8,
    message_ptr: *const u8,
    message_size: usize,
    signature_ptr: *const u8,
) -> bool {
    let in_msg = unsafe { from_raw_parts(message_ptr, message_size) };
    let in_pub = unsafe { from_raw_parts(public_ptr, kes::PUBLIC_KEY_SIZE) };
    let in_sig = unsafe { from_raw_parts(signature_ptr, kes::signature_size(DEPTH)) };

    let pk = kes::PublicKey::from_bytes(in_pub).unwrap();
    let sig = kes::Signature::from_bytes(DEPTH, in_sig).unwrap();

    kes::verify(&pk, in_msg, &sig)
}
