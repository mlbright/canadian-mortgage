use rust_decimal::prelude::*;
use rust_decimal_macros::*;
pub enum PaymentFrequency {
    Monthly,
    SemiMonthly,
    BiWeekly,
    AcceleratedBiWeekly,
    Weekly,
    AcceleratedWeekly,
}

pub struct CanadianMortage {
    principal: Decimal,
    interest_rate: Decimal,
    amortization_period: u64,
    payment_frequency: PaymentFrequency,
}

impl CanadianMortage {
    // mortgage_amount is the principal.
    // interest_rate is the annual interest rate as a percentage: 6.5% means r = 0.065 per year (see mortgage_payment below).
    // amortization_period is the number of years over which you will repay this loan.
    // payment_frequency determines the number of payments, which is also the compounding interval frequency.
    pub fn new(
        mortgage_amount: Decimal,
        interest_rate: Decimal,
        amortization_period: u64,
        payment_frequency: PaymentFrequency,
    ) -> anyhow::Result<CanadianMortage> {
        if interest_rate < dec!(0.0) || interest_rate > dec!(100.0) {
            anyhow::anyhow!("interest rate is the annual interest rate be between 0% and 100%");
        }

        // Convert the interest rate percentage to a decimal fraction
        let interest_rate = interest_rate / dec!(100);

        // Convert from an annual rate compounded semi-annually to an rate compounded monthly.
        // This is the strangeness of Canadian mortgages.
        let interest_rate = convert_compounding_basis(interest_rate, 2, 12)?;

        Ok(CanadianMortage {
            principal: mortgage_amount,
            interest_rate: interest_rate,
            amortization_period: amortization_period,
            payment_frequency,
        })
    }

    pub fn payment(&self) -> anyhow::Result<Decimal> {
        let monthly_payment = mortgage_payment(
            self.principal,
            self.interest_rate / dec!(12),
            self.amortization_period * 12,
        )?;

        let payment = match self.payment_frequency {
            PaymentFrequency::Monthly => monthly_payment,
            PaymentFrequency::SemiMonthly => monthly_payment / dec!(2),
            PaymentFrequency::BiWeekly => monthly_payment * dec!(12) / dec!(26),
            PaymentFrequency::AcceleratedBiWeekly => monthly_payment / dec!(2),
            PaymentFrequency::Weekly => monthly_payment * dec!(12) / dec!(52),
            PaymentFrequency::AcceleratedWeekly => monthly_payment / dec!(4),
        };

        Ok(payment)
    }
}

// https://en.wikipedia.org/wiki/Mortgage_loan
// https://www.yorku.ca/amarshal/mortgage.htm
// a = p * r * (1 + r)**n / ((1+r)**n - 1)
// a is the periodic amortization payment
// p is the principal amount borrowed
// r is the rate of interest expressed as a fraction; for a monthly payment, take the annual rate divided by 12
// n is the number of payments; for monthly payments over 30 years, 12 months x 30 years = 360 payments.
fn mortgage_payment(p: Decimal, r: Decimal, n: u64) -> anyhow::Result<Decimal> {
    Ok(p * r * (dec!(1.0) + r).powi(n) / ((dec!(1.0) + r).powi(n) - dec!(1.0)))
}

// https://en.wikipedia.org/wiki/Compound_interest#Compounding_basis
// r2 = ((1 + r1/n1) ** (n1/n2) - 1) * n2
// where r1 is the interest rate with compounding frequency n1, and r2 is the interest rate with compounding frequency n2
fn convert_compounding_basis(
    rate: Decimal,
    compounding_frequency1: u64,
    compounding_frequency2: u64,
) -> anyhow::Result<Decimal> {
    let n1 = Decimal::from_u64(compounding_frequency1).ok_or_else(|| {
        anyhow::anyhow!(
            "could not convert u64 to Decimal: {}",
            compounding_frequency1
        )
    })?;
    let n2 = Decimal::from_u64(compounding_frequency2).ok_or_else(|| {
        anyhow::anyhow!(
            "could not convert u64 to Decimal: {}",
            compounding_frequency2
        )
    })?;

    Ok((fractional_exponent(dec!(1) + (rate / n1), n1 / n2)? - dec!(1)) * n2)
}

fn fractional_exponent(base: Decimal, exponent: Decimal) -> anyhow::Result<Decimal> {
    let base = base
        .to_f64()
        .ok_or_else(|| anyhow::anyhow!("could not convert Decimal to f64: {}", base))?;
    let exponent = exponent
        .to_f64()
        .ok_or_else(|| anyhow::anyhow!("could not convert Decimal to f64: {}", exponent))?;

    Ok(Decimal::from_f64(base.powf(exponent))
        .ok_or_else(|| anyhow::anyhow!("could not convert from f64 to Decimal"))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_compounding_basis() {
        assert_eq!(
            convert_compounding_basis(dec!(0.06), 2, 1).unwrap(),
            dec!(0.0609),
            "Rate for Canadian mortages"
        );
        assert_eq!(
            convert_compounding_basis(dec!(0.06), 2, 12).unwrap(),
            dec!(0.059263464374364),
            "equivalent rate compounded monthly"
        );
    }

    #[test]
    fn mortgage_is_valid() {
        assert_eq!(
            mortgage_payment(dec!(10000000.0), dec!(0.105) / dec!(12), 10 * 12).unwrap(),
            dec!(134934.99677554698793630975554),
            "big mortgage"
        );
        assert_eq!(
            mortgage_payment(dec!(200000), dec!(0.065) / dec!(12), 30 * 12).unwrap(),
            dec!(1264.136046985927464091663357),
            "big mortgage"
        );
    }

    #[test]
    fn canadian_mortgage_payments_are_valid() {
        assert_eq!(
            CanadianMortage::new(
                dec!(430000.0),
                dec!(4.59),
                25,
                PaymentFrequency::AcceleratedWeekly,
            )
            .unwrap()
            .payment()
            .unwrap(),
            dec!(600.37384132280845354662242562),
            "old Canadian mortgage, accelerated weekly payments"
        );

        assert_eq!(
            CanadianMortage::new(
                dec!(430000.0),
                dec!(4.59),
                25,
                PaymentFrequency::AcceleratedBiWeekly,
            )
            .unwrap()
            .payment()
            .unwrap(),
            dec!(1200.7476826456169070932448512),
            "old Canadian mortgage, accelerated weekly payments"
        );

        assert_eq!(
            CanadianMortage::new(dec!(430000.0), dec!(4.59), 25, PaymentFrequency::Monthly)
                .unwrap()
                .payment()
                .unwrap(),
            dec!(2401.4953652912338141864897025),
            "old Canadian mortgage"
        );

        assert_eq!(
            CanadianMortage::new(dec!(100000.0), dec!(6), 25, PaymentFrequency::Monthly)
                .unwrap()
                .payment()
                .unwrap(),
            dec!(639.80662367674280200695111231),
            "tiny Canadian mortgage"
        );

        assert_eq!(
            CanadianMortage::new(dec!(100000.0), dec!(5), 25, PaymentFrequency::Monthly)
                .unwrap()
                .payment()
                .unwrap(),
            dec!(581.60498503699913800017437566),
            "small Canadian mortgage"
        );
    }
}
