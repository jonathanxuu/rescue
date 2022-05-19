#![no_std]
// use log::debug;
use sp_std::{ops::Range, vec, vec::Vec};
// use serde::{Serialize, Deserialize};
pub mod utils;
#[macro_use]
extern crate alloc;
use alloc::{borrow::ToOwned, string::String};

// RE-EXPORTS
// ================================================================================================

pub mod math;

extern crate console_error_panic_hook;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// extern crate web_sys;
use wasm_bindgen_test::*;
// use codec::{Decode, Encode};
use crate::alloc::string::ToString;


// RESCUE
// ================================================================================================

///  Executes the `rescue` fucntion and returns the rescue hash result.
/// 
/// * `inputs` specifies the rescue input;
/// *  Return the hash result
#[wasm_bindgen]
pub fn rescue(values: String) -> Vec<u8>{
    let mut values_in_u128 = vec![];
    if values.len() != 0{ 
        let values_a: Vec<&str> = values.split(',').collect();
        values_in_u128= values_a
        .iter()
        .map(|public_a| public_a.parse::<u128>().unwrap())
        .collect();
    };
    debug_assert!(
        values_in_u128.len() == 4,
        "expected values_in_u128 to be exactly 4 bytes but received {}",
        values_in_u128.len()
    );
    let result = math::rescue::rescue(utils::as_bytes(&values_in_u128));
    debug_assert!(
        result.len() == 32,
        "expected result to be exactly 32 bytes but received {}",
        result.len()
    );
    return result.to_vec()
}




// GLOBAL CONSTANTS
// ================================================================================================

pub const MAX_CONTEXT_DEPTH : usize = 16;
pub const MAX_LOOP_DEPTH    : usize = 8;
const MIN_TRACE_LENGTH      : usize = 16;
const MAX_REGISTER_COUNT    : usize = 128;
const MIN_EXTENSION_FACTOR  : usize = 16;
const BASE_CYCLE_LENGTH     : usize = 16;

const MIN_STACK_DEPTH       : usize = 8;
const MIN_CONTEXT_DEPTH     : usize = 1;
const MIN_LOOP_DEPTH        : usize = 1;

// PUSH OPERATION
// ------------------------------------------------------------------------------------------------
const PUSH_OP_ALIGNMENT     : usize = 8;

// HASH OPERATION
// ------------------------------------------------------------------------------------------------
const HASH_STATE_RATE       : usize = 4;
const HASH_STATE_CAPACITY   : usize = 2;
const HASH_STATE_WIDTH      : usize = HASH_STATE_RATE + HASH_STATE_CAPACITY;
const HASH_NUM_ROUNDS       : usize = 10;
const HASH_DIGEST_SIZE      : usize = 2;

// OPERATION SPONGE
// ------------------------------------------------------------------------------------------------
const SPONGE_WIDTH          : usize = 4;
const PROGRAM_DIGEST_SIZE   : usize = 2;
const HACC_NUM_ROUNDS       : usize = 14;

// DECODER LAYOUT
// ------------------------------------------------------------------------------------------------
//
//  ctr ╒═════ sponge ══════╕╒═══ cf_ops ══╕╒═══════ ld_ops ═══════╕╒═ hd_ops ╕╒═ ctx ══╕╒═ loop ═╕
//   0    1    2    3    4    5    6    7    8    9    10   11   12   13   14   15   ..   ..   ..
// ├────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┤

const NUM_CF_OP_BITS        : usize = 3;
const NUM_LD_OP_BITS        : usize = 5;
const NUM_HD_OP_BITS        : usize = 2;

const NUM_CF_OPS            : usize = 8;
const NUM_LD_OPS            : usize = 32;
const NUM_HD_OPS            : usize = 4;

const OP_COUNTER_IDX        : usize = 0;
const SPONGE_RANGE          : Range<usize> = Range { start:  1, end:  5 };
const CF_OP_BITS_RANGE      : Range<usize> = Range { start:  5, end:  8 };
const LD_OP_BITS_RANGE      : Range<usize> = Range { start:  8, end: 13 };
const HD_OP_BITS_RANGE      : Range<usize> = Range { start: 13, end: 15 };

// STACK LAYOUT
// ------------------------------------------------------------------------------------------------
//
// ╒═══════════════════ user registers ════════════════════════╕
//    0      1    2    .................................    31
// ├─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┤

pub const MAX_PUBLIC_INPUTS : usize = 8;
pub const MAX_OUTPUTS       : usize = MAX_PUBLIC_INPUTS;
pub const MAX_STACK_DEPTH   : usize = 32;