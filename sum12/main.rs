// Copyright (c) 2021, Tecnalia RI, Derio, Bizkaia.
// Inaki Seco Aguirre <inaki.seco@tecnalia.com>

#![no_std]
#![no_main]
#![feature(const_evaluatable_checked)]

use scale_std::local_functions::JSON_set_key;

#[scale::main(KAPPA = 40)]
#[inline(always)]

fn main() {
    println!("\n# Sum ALGORITHM");

    const PARTY_LEN:i64 = 3;
    const DATA_LEN:i64 = 12;

    let mut sum = SecretModp::from(0);

    JSON_set_key("salary");
    
    for i in 1..PARTY_LEN { 
        for _j in 0..DATA_LEN {
            let input = SecretModp::private_input(i, 10);
            sum = sum + input;
        }
    }

    sum.private_output(0, 10);
}
