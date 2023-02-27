use once_cell::sync::Lazy;
use std::{collections::HashMap, hash::Hash};

pub trait FromCurrency: Eq + Hash + Copy {
    fn currency(&self) -> &str;
}

static DENOMINATION: Lazy<HashMap<&str, i32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("INR", 100);
    map.insert("USD", 100);
    map
});

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AmountInner<Amt, Cur: FromCurrency> {
    amount: Amt,
    currency: Cur,
}

pub type LowestDenomination = i64;
pub type HighestDenomination = f64;

impl<Cur: FromCurrency> AmountInner<LowestDenomination, Cur> {
    pub fn new(amount: i64, currency: &Cur) -> Self {
        Self {
            amount,
            currency: *currency,
        }
    }

    pub fn convert(self) -> AmountInner<HighestDenomination, Cur> {
        self.into()
    }
}

impl<Cur: FromCurrency> From<AmountInner<LowestDenomination, Cur>>
    for AmountInner<HighestDenomination, Cur>
{
    fn from(value: AmountInner<LowestDenomination, Cur>) -> Self {
        let factor = DENOMINATION.get(&value.currency.currency()).unwrap();
        AmountInner::<HighestDenomination, Cur>::new(
            (value.amount as f64) / (*factor as f64),
            &value.currency,
        )
    }
}

impl<Cur: FromCurrency> From<AmountInner<HighestDenomination, Cur>>
    for AmountInner<LowestDenomination, Cur>
{
    fn from(value: AmountInner<HighestDenomination, Cur>) -> Self {
        let factor = DENOMINATION.get(&value.currency.currency()).unwrap();
        AmountInner::<LowestDenomination, Cur>::new(
            (value.amount * (*factor as f64)) as i64,
            &value.currency,
        )
    }
}

impl<Cur: FromCurrency> AmountInner<HighestDenomination, Cur> {
    pub fn new(amount: f64, currency: &Cur) -> Self {
        Self {
            amount,
            currency: *currency,
        }
    }

    pub fn convert(self) -> AmountInner<LowestDenomination, Cur> {
        self.into()
    }
}

#[cfg(test)]
mod tests {

    use super::{AmountInner, FromCurrency, HighestDenomination, LowestDenomination};

    impl FromCurrency for Currency {
        fn currency(&self) -> &str {
            match self {
                Currency::INR => "INR",
                Currency::USD => "USD",
            }
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    enum Currency {
        INR,
        USD,
    }

    type Amount = AmountInner<LowestDenomination, Currency>;
    type AmountHD = AmountInner<HighestDenomination, Currency>;

    #[test]
    fn testing() {
        let amount = Amount::new(1, &Currency::USD);
        let high_denomination: AmountHD = amount.convert();
        let lower_denomination: Amount = high_denomination.convert();
        assert_eq!(lower_denomination, amount);

        let amount = Amount::new(1, &Currency::INR);
        let high_denomination: AmountHD = amount.convert();
        let lower_denomination: Amount = high_denomination.convert();
        assert_eq!(lower_denomination, amount);
    }
}
