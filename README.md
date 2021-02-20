# Canadian mortgage library

> library to calculate Canadian mortage payments

This library calculates payments on Canadian mortages.
Canadian mortages are compounded semi-annually but payments are typically made monthly.
This means that rates must be [converted][compounding-basis] to accurately compute mortgage payments.

There are no prepayment options included in this calculator.

For a more flexible and complete mortgage calculator, visit the [mortgage calculator][canadian-mortgage-calculator] from the Financial Consumer Agency of Canada.

## Things I learned

Items I explored or learned something about:

- [x] [floating point and decimal representation][floating-point-guide]
- [x] Writing tests in Rust
- [x] Mortgage math, specifically in the Canadian context
- [x] Writing a Rust library, as opposed to a binary
- [ ] Web assembly widget



[semi-annually]: https://www.yorku.ca/amarshal/mortgage.htm
[mortgage-payments]: https://en.wikipedia.org/wiki/Equated_monthly_installment
[compounding-basis]: https://en.wikipedia.org/wiki/Compound_interest#Compounding_basis
[canadian-mortgage-calculator]: https://itools-ioutils.fcac-acfc.gc.ca/MC-CH/MCCalc-CHCalc-eng.aspx
[floating-point-guide]: https://floating-point-gui.de/
