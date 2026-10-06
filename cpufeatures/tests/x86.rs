//! `x86`/`x86_64` tests

#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

cpufeatures::new!(cpuid, "aes", "sha");

#[test]
fn init() {
    let token: cpuid::InitToken = cpuid::init();
    assert_eq!(token.get(), cpuid::get());
}

#[test]
fn init_get() {
    let (token, val) = cpuid::init_get();
    assert_eq!(val, token.get());
}

#[test]
fn runtime_detection_is_disabled() {
    let detected = cpufeatures::__unless_target_features! {
        "avx512f" => panic!("runtime detection must not run")
    };
    assert_eq!(detected, cfg!(target_feature = "avx512f"));
}
