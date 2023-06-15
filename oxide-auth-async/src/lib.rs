#![no_std]
extern crate sgx_tstd as std;
pub mod code_grant;
pub mod endpoint;
pub mod primitives;


#[cfg(test)]
mod tests;
