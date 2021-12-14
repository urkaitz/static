// Copyright (c) 2021, Tecnalia RI, Derio, Bizkaia.
// Inaki Seco Aguirre <inaki.seco@tecnalia.com>

#![no_std]
#![no_main]
#![feature(const_evaluatable_checked)]

use scale_std::ieee::ClearIEEE;
use scale_std::ieee::SecretIEEE;
use scale_std::local_functions::JSON_set_key;

#[scale::main(KAPPA = 40)]
#[inline(always)]
fn main() {
    println!("\n# Mean ALGORITHM");

    const SECRET_DIV: bool = true;

    const PARTY_LEN: i64 = 3;
    const DATA_LEN: i64 = 4;

    let mut count: i64 = 0;
    let mut sum_secret = SecretModp::from(0);

    JSON_set_key("salary");

    for i in 1..PARTY_LEN {
        for _j in 0..DATA_LEN {
            let input = SecretModp::private_input(i, 10);
            sum_secret = sum_secret + input;
            count = count + 1;
        }
    }

    if count == 0 {
        let result_secret = SecretModp::from(0);
        result_secret.private_output(0, 10);
    } else {
        if !SECRET_DIV {
            // PUBLIC VERSION
            let sum_clear_i64 = i64::from(sum_secret.reveal());
            let sum_clear_ieee = ClearIEEE::from(sum_clear_i64);

            let divisor_clear_i64 = i64::from(count);
            let divisor_clear_ieee = ClearIEEE::from(divisor_clear_i64);

            let result_clear_ieee = sum_clear_ieee / divisor_clear_ieee;

            let result_clear_i64 = i64::from(result_clear_ieee);
            let result_clear = ClearModp::from(result_clear_i64);

            let result_secret = SecretModp::from(result_clear);
            result_secret.private_output(0, 10);
        } else {
            // SECRET VERSION
            let sum_secret_i64 = SecretI64::from(sum_secret);
            let sum_secret_ieee = SecretIEEE::from(sum_secret_i64);

            let divisor_secret_i64 = SecretI64::from(count);
            let divisor_secret_ieee = SecretIEEE::from(divisor_secret_i64);

            let result_secret_ieee = sum_secret_ieee / divisor_secret_ieee;

            let result_secret_i64 = SecretI64::from(result_secret_ieee);
            let result_secret = SecretModp::from(result_secret_i64);
            result_secret.private_output(0, 10);
        }
    }
}
