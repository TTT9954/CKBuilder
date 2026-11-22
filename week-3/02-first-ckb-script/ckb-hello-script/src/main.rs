
#![no_std]
#![no_main]

use ckb_std::default_alloc;
use ckb_std::entry;

default_alloc!();

entry!(program_entry);

fn program_entry() -> i8 {
    let script = ckb_std::high_level::load_script();

    match script {
        Ok(script) => {
            let args = script.args().raw_data();

            ckb_std::debug!("Script loaded successfully!");
            ckb_std::debug!("Args length: {}", args.len());
            ckb_std::debug!("Args data: {:02x?}", args.as_ref());

            if args.as_ref() == b"hello" {
                ckb_std::debug!("Validation passed!");
                0
            } else {
                ckb_std::debug!("Validation failed!");
                1
            }
        }
        Err(err) => {
            ckb_std::debug!("Failed to load script: {:?}", err);
            -1
        }
    }
}
