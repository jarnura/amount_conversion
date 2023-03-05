use crate::factor::{get_factor, FromCurrency};

/// This library supports number till i32::MAX
static MAX_F64_ALLOWED: f64 = {
    let small = i32::MAX;
    small as f64
};

/// This library supports number till i32::MIN
static MIN_F64_ALLOWED: f64 = {
    let small = i32::MIN;
    small as f64
};

/// `MoneyInner` is a generic struct which combines amount and currency bounded to a single struct.
///
/// `amount` field also generic so that it can hold i16,i32,f32,f64 etc.
///
/// `currency` field also generic, since the user of the library can create their own enums for currency.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MoneyInner<Amt, Cur: FromCurrency> {
    pub(crate) amount: Amt,
    pub(crate) currency: Cur,
}

/// A possible error value when converting a `MoneyInner<T>` from a `MoneyInner<U>`.
///
#[derive(Debug, PartialEq)]
pub enum MoneyConversionError<T> {
    /// `CurrencyNotFoundInSubunitMap` - When the custom currency not found in the subunit map.
    CurrencyNotFoundInSubunitMap(T),

    /// `F64ToI32ConversionFailed` - The max number this library can process is i32::MAX, when a f64 is
    ///                              large than that this error will arise.
    F64ToI32ConversionFailed,
}

pub type LowestSubunit = i32;
pub type HighestUnit = f64;

impl<Cur: FromCurrency> MoneyInner<LowestSubunit, Cur> {
    pub fn new(amount: i32, currency: &Cur) -> Self {
        Self {
            amount,
            currency: *currency,
        }
    }

    pub fn convert(self) -> Result<MoneyInner<HighestUnit, Cur>, MoneyConversionError<Cur>> {
        self.try_into()
    }
}

impl<Cur: FromCurrency> TryFrom<MoneyInner<LowestSubunit, Cur>> for MoneyInner<HighestUnit, Cur> {
    type Error = MoneyConversionError<Cur>;

    fn try_from(value: MoneyInner<LowestSubunit, Cur>) -> Result<Self, Self::Error> {
        let factor = get_factor(&value)?;
        Ok(MoneyInner::<HighestUnit, Cur>::new(
            (value.amount as f64) / factor,
            &value.currency,
        ))
    }
}

impl<Cur: FromCurrency> TryFrom<MoneyInner<HighestUnit, Cur>> for MoneyInner<LowestSubunit, Cur> {
    type Error = MoneyConversionError<Cur>;

    fn try_from(value: MoneyInner<HighestUnit, Cur>) -> Result<Self, Self::Error> {
        let factor = get_factor(&value)?;
        Ok(MoneyInner::<LowestSubunit, Cur>::new(
            f64_to_i32(value.amount * factor)?,
            &value.currency,
        ))
    }
}

impl<Cur: FromCurrency> MoneyInner<HighestUnit, Cur> {
    pub fn new(amount: f64, currency: &Cur) -> Self {
        Self {
            amount,
            currency: *currency,
        }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn convert(self) -> Result<MoneyInner<LowestSubunit, Cur>, MoneyConversionError<Cur>> {
        self.try_into()
    }
}

fn f64_to_i32<T>(f: f64) -> Result<i32, MoneyConversionError<T>> {
    if f > MAX_F64_ALLOWED || f < MIN_F64_ALLOWED {
        return Err(MoneyConversionError::F64ToI32ConversionFailed);
    }
    Ok(f as i32)
}

#[cfg(test)]
mod tests {

    use crate::factor::{self, Currency::*};

    use super::*;

    impl FromCurrency for Currency {
        fn currency(&self) -> factor::Currency {
            match self {
                Currency::Inr => INR,
                Currency::Usd => USD,
            }
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize)]
    enum Currency {
        Inr,
        Usd,
    }

    type Money = MoneyInner<LowestSubunit, Currency>;
    type MoneyHD = MoneyInner<HighestUnit, Currency>;

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct Request {
        #[serde(flatten)]
        amount: Money,
        id: i8,
    }

    #[test]
    fn unit_case() -> Result<(), MoneyConversionError<Currency>> {
        let amount = Money::new(1, &Currency::Usd);
        let highest_unit: MoneyHD = amount.convert()?;
        let lowest_unit: Money = highest_unit.convert()?;
        assert_eq!(amount, lowest_unit);

        let amount = Money::new(1, &Currency::Inr);
        let highest_unit: MoneyHD = amount.convert()?;
        let lowest_unit: Money = highest_unit.convert()?;
        assert_eq!(amount, lowest_unit);
        Ok(())
    }

    #[test]
    fn i32_max_number() -> Result<(), MoneyConversionError<Currency>> {
        let amount = Money::new(i32::MAX, &Currency::Inr);
        let highest_unit: MoneyHD = amount.convert()?;
        let lowest_unit: Money = highest_unit.convert()?;

        assert_eq!(amount, lowest_unit);
        Ok(())
    }

    #[test]
    fn i32_max_number_without_amount() {
        let amount_lhs = i32::MAX;
        let highest_unit_lhs = amount_lhs as f32 / 100.0_f32;
        let lowest_unit_lhs = (highest_unit_lhs * 100.0_f32) as i32;

        let amount_rhs = i32::MAX - 1;
        let highest_unit_rhs = amount_rhs as f32 / 100.0_f32;
        let lowest_unit_rhs = (highest_unit_rhs * 100.0_f32) as i32;

        assert_eq!(amount_lhs, lowest_unit_lhs);
        assert_eq!(lowest_unit_lhs, lowest_unit_rhs); // This is invalid but as_conversion fails here
        assert_ne!(amount_rhs, lowest_unit_rhs); // This is invalid but as_conversion fails here
    }

    #[test]
    fn i32_max_number_with_amount() -> Result<(), MoneyConversionError<Currency>> {
        let amount_lhs = Money::new(i32::MAX, &Currency::Inr);
        let highest_unit_lhs: MoneyHD = amount_lhs.convert()?;
        let lowest_unit_lhs: Money = highest_unit_lhs.convert()?;

        let amount_rhs = Money::new(i32::MAX - 1, &Currency::Inr);
        let highest_unit_rhs = amount_rhs.convert()?;
        let lowest_unit_rhs = highest_unit_rhs.convert()?;

        assert_eq!(amount_lhs, lowest_unit_lhs);
        assert_ne!(lowest_unit_lhs, lowest_unit_rhs);
        assert_eq!(amount_rhs, lowest_unit_rhs);
        Ok(())
    }

    #[test]
    fn f64_max_number() {
        let amount_lhs = MoneyHD::new(f64::MAX, &Currency::Usd);
        let lowest_unit: Result<Money, _> = amount_lhs.convert();
        assert_eq!(
            lowest_unit,
            Err(MoneyConversionError::F64ToI32ConversionFailed)
        );
    }

    #[test]
    fn f64_max_number_without_amount() {
        let amount_lhs = f64::MAX;
        let lowest_unit: i32 = (amount_lhs / 100.0_f64) as i32;
        let highest_unit = lowest_unit as f64 * 100.0_f64;
        assert_ne!(amount_lhs, highest_unit); // This is invalid but as_conversion fails here
    }

    #[test]
    fn deserialize() -> Result<(), serde_json::Error> {
        let amount_str = r#"{
            "amount": 1,
            "currency": "Inr",
            "id": 1
        }"#;
        serde_json::from_str::<Request>(amount_str)?;
        Ok(())
    }
}
