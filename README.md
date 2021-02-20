# canadian-mortgage

> library to calculate Canadian mortgage payments

This library calculates payments on Canadian mortages.
Canadian mortages are compounded semi-annually but payments are typically made monthly.
This means that rates must be [converted][compounding-basis] to accurately compute mortgage payments. 

There are no prepayment options included in this calculator.

For a more flexible and complete mortgage calculator, visit the [mortgage calculator][canadian-mortgage-calculator] from the Financial Consumer Agency of Canada.

## Usage

```rust
use rust_decimal_macros::*;
fn main() {
    let mortgage = canadian_mortgage::CanadianMortgage::new(
        dec!(500000),
        dec!(4.59),
        25,
        canadian_mortgage::PaymentFrequency::Monthly,
    )
    .unwrap();
    println!("mortgage payment: {}", mortgage.payment().unwrap());
}
```

## TODO

- [ ] Replicate all the other behaviour from a [more complete calculator][canadian-mortgage-calculator]

## Things I learned or re-learned

- [x] [floating point and decimal representation][floating-point-guide]
- [x] Writing tests in Rust
- [x] [Mortgage math, specifically in the Canadian context][semi-annually]
- [x] Writing a Rust library, as opposed to a binary
- [ ] Web assembly widget



[semi-annually]: https://www.yorku.ca/amarshal/mortgage.htm
[mortgage-payments]: https://en.wikipedia.org/wiki/Equated_monthly_installment
[compounding-basis]: https://en.wikipedia.org/wiki/Compound_interest#Compounding_basis
[canadian-mortgage-calculator]: https://itools-ioutils.fcac-acfc.gc.ca/MC-CH/MCCalc-CHCalc-eng.aspx
[floating-point-guide]: https://floating-point-gui.de/
