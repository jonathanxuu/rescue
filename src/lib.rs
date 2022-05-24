#![no_std]
use crypto::{Digest, ElementHasher, hash::rescue::rp64_256::ElementDigest};
// use log::debug;
use sp_std::{ vec, vec::Vec};
// use serde::{Serialize, Deserialize};
#[macro_use]
extern crate alloc;
use alloc::{ string::String};
// RE-EXPORTS
// ================================================================================================

use math::fields::f64::BaseElement;
use math::StarkField;

extern crate console_error_panic_hook;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// extern crate web_sys;
use wasm_bindgen_test::*;


// RESCUE
// ================================================================================================

///  Executes the `rescue` fucntion and returns the rescue hash result.
/// 
/// * `inputs` specifies the rescue input, it should be a vec with 8 (u64) elements;
/// *  Return the hash result [u8;32]
#[wasm_bindgen]
pub fn rescue(values: String) -> Vec<u64>{
    let mut values_in_u64 = vec![];
    if values.len() != 0{ 
        let values_a: Vec<&str> = values.split(',').collect();
        values_in_u64 = values_a
        .iter()
        // .map(|public_a| public_a.parse::<u64>().unwrap())
        .map(|public_a| public_a.parse::<u64>().expect("parse fail"))
        .collect();
    };
    assert!(
        values_in_u64.len() == 8,
        "expected len of values_in_u64 to be exactly 8 but received {}",
        values_in_u64.len()
    );
    let elements = from_vec(values_in_u64);
    let result = crypto::hashers::Rp64_256::hash_elements(&elements);

    return as_u64(result).to_vec()
}

pub fn as_u64(origin:ElementDigest) -> [u64; 4] {
    let mut result = [0; 4];
    result[..1].copy_from_slice(&[origin.0[0].as_int()]);
    result[1..2].copy_from_slice(&[origin.0[1].as_int()]);
    result[2..3].copy_from_slice(&[origin.0[2].as_int()]);
    result[3..4].copy_from_slice(&[origin.0[3].as_int()]);
    result
}



/// HELPER
    pub fn from_vec(origin: Vec<u64>) -> [BaseElement;8]{
        let mut res :Vec<BaseElement> = Vec::new();
        for i in 0..8{
            res.push(math::fields::f64::BaseElement::from(origin[i as usize]))
        };
        let result = res.try_into().unwrap();
        return result;
    }

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// GLOBAL CONSTANTS
// ================================================================================================

pub const MAX_CONTEXT_DEPTH : usize = 16;
pub const MAX_LOOP_DEPTH    : usize = 8;


