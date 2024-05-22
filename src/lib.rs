#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, URef};

#[no_mangle]
pub extern "C" fn store_record() {
    let patient_id: String = runtime::get_named_arg("patient_id");
    let diagnosis: String = runtime::get_named_arg("diagnosis");
    let treatment: String = runtime::get_named_arg("treatment");

    let record = vec![
        (String::from("diagnosis"), diagnosis),
        (String::from("treatment"), treatment),
    ];

    let key = alloc::format!("patient_{}", patient_id);
    let uref: URef = storage::new_uref(record);
    runtime::put_key(&key, uref.into());
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let store_record = EntryPoint::new(
            "store_record",
            vec![
                Parameter::new("patient_id", CLType::String),
                Parameter::new("diagnosis", CLType::String),
                Parameter::new("treatment", CLType::String),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(store_record);
        entry_points
    };

    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("healthcare_contract", contract_hash.into());
}
