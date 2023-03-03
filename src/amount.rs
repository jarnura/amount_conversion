use std::hash::Hash;

use crate::factor::get_factor;

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

/// A trait for converting a custom currency type to a `&str`.
///
/// This trait has currency function which generates a `&str`,
/// that slice is the key in `Subunit's` hashmap.
/// The `&str` value always in `Uppercase`.
/// The `Subunit's` hashmap contains factor for currency's subunit.
pub trait FromCurrency: Eq + Hash + Copy {
    /// Converts the custom type to a `&str`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    ///  use amount_conversion::amount::FromCurrency;
    ///
    /// #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    /// enum Currency {
    ///     Inr,
    ///     Usd,
    /// }
    ///
    /// impl FromCurrency for Currency {
    /// fn currency(&self) -> &str {
    ///    match self {
    ///        Currency::Inr => "INR",
    ///        Currency::Usd => "USD",
    ///    }
    ///  }
    /// }
    /// let custom_currency = Currency::Inr;
    /// let currency = "INR";
    ///
    /// assert_eq!(currency, custom_currency.currency());
    /// ```
    fn currency(&self) -> &str;
}

/// `AmountInner` is a generic struct which combines amount and currency bounded to a single struct.
///
/// `amount` field also generic so that it can hold i16,i32,f32,f64 etc.
///
/// `currency` field also generic, since the user of the library can create their own enums for currency.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AmountInner<Amt, Cur: FromCurrency> {
    pub(crate) amount: Amt,
    pub(crate) currency: Cur,
}

/// A possible error value when converting a `AmountInner<T>` from a `AmountInner<U>`.
///
#[derive(Debug, PartialEq)]
pub enum AmmountConversionError<T> {
    /// `CurrencyNotFoundInSubunitMap` - When the custom currency not found in the subunit map.
    CurrencyNotFoundInSubunitMap(T),

    /// `F64ToI32ConversionFailed` - The max number this library can process is i32::MAX, when a f64 is
    ///                              large than that this error will arise.
    F64ToI32ConversionFailed,
}

pub type LowestSubunit = i32;
pub type HighestUnit = f64;

impl<Cur: FromCurrency> AmountInner<LowestSubunit, Cur> {
    pub fn new(amount: i32, currency: &Cur) -> Self {
        Self {
            amount,
            currency: *currency,
        }
    }

    pub fn convert(self) -> Result<AmountInner<HighestUnit, Cur>, AmmountConversionError<Cur>> {
        self.try_into()
    }
}

impl<Cur: FromCurrency> TryFrom<AmountInner<LowestSubunit, Cur>> for AmountInner<HighestUnit, Cur> {
    type Error = AmmountConversionError<Cur>;

    fn try_from(value: AmountInner<LowestSubunit, Cur>) -> Result<Self, Self::Error> {
        let factor = get_factor(&value)?;
        Ok(AmountInner::<HighestUnit, Cur>::new(
            (value.amount as f64) / factor,
            &value.currency,
        ))
    }
}

impl<Cur: FromCurrency> TryFrom<AmountInner<HighestUnit, Cur>> for AmountInner<LowestSubunit, Cur> {
    type Error = AmmountConversionError<Cur>;

    fn try_from(value: AmountInner<HighestUnit, Cur>) -> Result<Self, Self::Error> {
        let factor = get_factor(&value)?;
        Ok(AmountInner::<LowestSubunit, Cur>::new(
            f64_to_i32(value.amount * factor)?,
            &value.currency,
        ))
    }
}

impl<Cur: FromCurrency> AmountInner<HighestUnit, Cur> {
    pub fn new(amount: f64, currency: &Cur) -> Self {
        Self {
            amount,
            currency: *currency,
        }
    }

    pub fn convert(self) -> Result<AmountInner<LowestSubunit, Cur>, AmmountConversionError<Cur>> {
        self.try_into()
    }
}

fn f64_to_i32<T>(f: f64) -> Result<i32, AmmountConversionError<T>> {
    if f > MAX_F64_ALLOWED || f < MIN_F64_ALLOWED {
        return Err(AmmountConversionError::F64ToI32ConversionFailed);
    }
    Ok(f as i32)
}

#[cfg(test)]
mod tests {

    use super::*;

    impl FromCurrency for Currency {
        fn currency(&self) -> &str {
            match self {
                Currency::Inr => "INR",
                Currency::Usd => "USD",
            }
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize)]
    enum Currency {
        Inr,
        Usd,
    }

    type Amount = AmountInner<LowestSubunit, Currency>;
    type AmountHD = AmountInner<HighestUnit, Currency>;

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct Request {
        #[serde(flatten)]
        amount: Amount,
        id: i8,
    }

    #[test]
    fn unit_case() -> Result<(), AmmountConversionError<Currency>> {
        let amount = Amount::new(1, &Currency::Usd);
        let highest_unit: AmountHD = amount.convert()?;
        let lowest_unit: Amount = highest_unit.convert()?;
        assert_eq!(amount, lowest_unit);

        let amount = Amount::new(1, &Currency::Inr);
        let highest_unit: AmountHD = amount.convert()?;
        let lowest_unit: Amount = highest_unit.convert()?;
        assert_eq!(amount, lowest_unit);
        Ok(())
    }

    #[test]
    fn i32_max_number() -> Result<(), AmmountConversionError<Currency>> {
        let amount = Amount::new(i32::MAX, &Currency::Inr);
        let highest_unit: AmountHD = amount.convert()?;
        let lowest_unit: Amount = highest_unit.convert()?;

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
    fn i32_max_number_with_amount() -> Result<(), AmmountConversionError<Currency>> {
        let amount_lhs = Amount::new(i32::MAX, &Currency::Inr);
        let highest_unit_lhs: AmountHD = amount_lhs.convert()?;
        let lowest_unit_lhs: Amount = highest_unit_lhs.convert()?;

        let amount_rhs = Amount::new(i32::MAX - 1, &Currency::Inr);
        let highest_unit_rhs = amount_rhs.convert()?;
        let lowest_unit_rhs = highest_unit_rhs.convert()?;

        assert_eq!(amount_lhs, lowest_unit_lhs);
        assert_ne!(lowest_unit_lhs, lowest_unit_rhs);
        assert_eq!(amount_rhs, lowest_unit_rhs);
        Ok(())
    }

    #[test]
    fn f64_max_number() {
        let amount_lhs = AmountHD::new(f64::MAX, &Currency::Usd);
        let lowest_unit: Result<Amount, _> = amount_lhs.convert();
        assert_eq!(
            lowest_unit,
            Err(AmmountConversionError::F64ToI32ConversionFailed)
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
