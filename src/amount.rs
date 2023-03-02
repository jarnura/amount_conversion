use once_cell::sync::Lazy;
use std::{collections::HashMap, hash::Hash};

pub trait FromCurrency: Eq + Hash + Copy {
    fn currency(&self) -> &str;
}

#[derive(Debug, PartialEq)]
pub enum AmmountConversionError<T> {
    CurrencyNotFoundInDenominationMap(T),
    F64ToI32ConversionFailed,
}

static DENOMINATION: Lazy<HashMap<&str, i32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("INR", 100);
    map.insert("USD", 100);
    map
});

static MAX_F64_ALLOWED: f64 = {
    let small = i32::MAX;
    small as f64
};
static MIN_F64_ALLOWED: f64 = {
    let small = i32::MIN;
    small as f64
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct AmountInner<Amt, Cur: FromCurrency> {
    amount: Amt,
    currency: Cur,
}

pub type LowestDenomination = i32;
pub type HighestDenomination = f64;

impl<Cur: FromCurrency> AmountInner<LowestDenomination, Cur> {
    pub fn new(amount: i32, currency: &Cur) -> Self {
        Self {
            amount,
            currency: *currency,
        }
    }

    pub fn convert(
        self,
    ) -> Result<AmountInner<HighestDenomination, Cur>, AmmountConversionError<Cur>> {
        self.try_into()
    }
}

impl<Cur: FromCurrency> TryFrom<AmountInner<LowestDenomination, Cur>>
    for AmountInner<HighestDenomination, Cur>
{
    type Error = AmmountConversionError<Cur>;

    fn try_from(value: AmountInner<LowestDenomination, Cur>) -> Result<Self, Self::Error> {
        let factor = get_factor(&value)?;
        Ok(AmountInner::<HighestDenomination, Cur>::new(
            (value.amount as f64) / factor,
            &value.currency,
        ))
    }
}

fn f64_to_i32<T>(f: f64) -> Result<i32, AmmountConversionError<T>> {
    if f > MAX_F64_ALLOWED || f < MIN_F64_ALLOWED {
        return Err(AmmountConversionError::F64ToI32ConversionFailed);
    }
    Ok(f as i32)
}

fn get_factor<T, Cur: FromCurrency>(
    amount: &AmountInner<T, Cur>,
) -> Result<f64, AmmountConversionError<Cur>> {
    Ok(*DENOMINATION.get(amount.currency.currency()).ok_or(
        AmmountConversionError::CurrencyNotFoundInDenominationMap(amount.currency),
    )? as f64)
}

impl<Cur: FromCurrency> TryFrom<AmountInner<HighestDenomination, Cur>>
    for AmountInner<LowestDenomination, Cur>
{
    type Error = AmmountConversionError<Cur>;

    fn try_from(value: AmountInner<HighestDenomination, Cur>) -> Result<Self, Self::Error> {
        let factor = get_factor(&value)?;
        Ok(AmountInner::<LowestDenomination, Cur>::new(
            f64_to_i32(value.amount * factor)?,
            &value.currency,
        ))
    }
}

impl<Cur: FromCurrency> AmountInner<HighestDenomination, Cur> {
    pub fn new(amount: f64, currency: &Cur) -> Self {
        Self {
            amount,
            currency: *currency,
        }
    }

    pub fn convert(
        self,
    ) -> Result<AmountInner<LowestDenomination, Cur>, AmmountConversionError<Cur>> {
        self.try_into()
    }
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

    type Amount = AmountInner<LowestDenomination, Currency>;
    type AmountHD = AmountInner<HighestDenomination, Currency>;

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
        let high_denomination: AmountHD = amount.convert()?;
        let lower_denomination: Amount = high_denomination.convert()?;
        assert_eq!(amount, lower_denomination);

        let amount = Amount::new(1, &Currency::Inr);
        let high_denomination: AmountHD = amount.convert()?;
        let lower_denomination: Amount = high_denomination.convert()?;
        assert_eq!(amount, lower_denomination);
        Ok(())
    }

    #[test]
    fn i32_max_number() -> Result<(), AmmountConversionError<Currency>> {
        let amount = Amount::new(i32::MAX, &Currency::Inr);
        let high_denomination: AmountHD = amount.convert()?;
        let lower_denomination: Amount = high_denomination.convert()?;

        assert_eq!(amount, lower_denomination);
        Ok(())
    }

    #[test]
    fn i32_max_number_without_amount() {
        let amount_lhs = i32::MAX;
        let high_denomination_lhs = amount_lhs as f32 / 100.0_f32;
        let lower_denomination_lhs = (high_denomination_lhs * 100.0_f32) as i32;

        let amount_rhs = i32::MAX - 1;
        let high_denomination_rhs = amount_rhs as f32 / 100.0_f32;
        let lower_denomination_rhs = (high_denomination_rhs * 100.0_f32) as i32;

        assert_eq!(amount_lhs, lower_denomination_lhs);
        assert_eq!(lower_denomination_lhs, lower_denomination_rhs); // This is invalid but as_conversion fails here
        assert_ne!(amount_rhs, lower_denomination_rhs); // This is invalid but as_conversion fails here
    }

    #[test]
    fn i32_max_number_with_amount() -> Result<(), AmmountConversionError<Currency>> {
        let amount_lhs = Amount::new(i32::MAX, &Currency::Inr);
        let high_denomination_lhs: AmountHD = amount_lhs.convert()?;
        let lower_denomination_lhs: Amount = high_denomination_lhs.convert()?;

        let amount_rhs = Amount::new(i32::MAX - 1, &Currency::Inr);
        let high_denomination_rhs = amount_rhs.convert()?;
        let lower_denomination_rhs = high_denomination_rhs.convert()?;

        assert_eq!(amount_lhs, lower_denomination_lhs);
        assert_ne!(lower_denomination_lhs, lower_denomination_rhs);
        assert_eq!(amount_rhs, lower_denomination_rhs);
        Ok(())
    }

    #[test]
    fn f64_max_number() {
        let amount_lhs = AmountHD::new(f64::MAX, &Currency::Usd);
        let lower_denomination: Result<Amount, _> = amount_lhs.convert();
        assert_eq!(
            lower_denomination,
            Err(AmmountConversionError::F64ToI32ConversionFailed)
        );
    }

    #[test]
    fn f64_max_number_without_amount() {
        let amount_lhs = f64::MAX;
        let lower_denomination: i32 = (amount_lhs / 100.0_f64) as i32;
        let high_denomination = lower_denomination as f64 * 100.0_f64;
        assert_ne!(amount_lhs, high_denomination); // This is invalid but as_conversion fails here
    }

    #[test]
    fn deserialize() -> Result<(), serde_json::Error>{
        let amount_str = r#"{
            "amount": 1,
            "currency": "Inr",
            "id": 1
        }"#;
        serde_json::from_str::<Request>(amount_str)?;
        Ok(())
    }
}
