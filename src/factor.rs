use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::hash::Hash;

use crate::amount;

use self::Currency::*;

static SUBUNIT: Lazy<HashMap<Currency, i16>> = Lazy::new(|| {
    let mut map = HashMap::new();
    ZERO_DECIMAL_PAIR.into_iter().for_each(|f| {
        map.insert(f.0, f.1 as i16);
    });
    TWO_DECIMAL_PAIR.into_iter().for_each(|f| {
        map.insert(f.0, f.1 as i16);
    });
    THREE_DECIMAL_PAIR.into_iter().for_each(|f| {
        map.insert(f.0, f.1);
    });
    map
});

static ZERO_DECIMAL_PAIR: Lazy<[(Currency, i8); 16]> = Lazy::new(|| {
    [
        BIF, CLP, DJF, GNF, JPY, KMF, KRW, MGA, PYG, RWF, UGX, VND, VUV, XAF, XOF, XPF,
    ]
    .map(|currency| (currency, 1))
});

static TWO_DECIMAL_PAIR: Lazy<[(Currency, i8); 98]> = Lazy::new(|| {
    [
        AED, ALL, AMD, ANG, ARS, AUD, AWG, AZN, BBD, BDT, BMD, BND, BOB, BRL, BSD, BWP, BZD, CAD,
        CHF, CNY, COP, CRC, CUP, CZK, DKK, DOP, DZD, EGP, ETB, EUR, FJD, GBP, GHS, GIP, GMD, GTQ,
        GYD, HKD, HNL, HRK, HTG, HUF, IDR, ILS, INR, JMD, KES, KGS, KHR, KYD, KZT, LAK, LBP, LKR,
        LRD, LSL, MAD, MDL, MKD, MMK, MNT, MOP, MUR, MVR, MWK, MXN, MYR, NAD, NGN, NIO, NOK, NPR,
        NZD, PEN, PGK, PHP, PKR, PLN, QAR, RUB, SAR, SCR, SEK, SGD, SLL, SOS, SSP, SVC, SZL, THB,
        TTD, TWD, TZS, USD, UYU, UZS, YER, ZAR,
    ]
    .map(|currency| (currency, 100))
});

static THREE_DECIMAL_PAIR: Lazy<[(Currency, i16); 5]> =
    Lazy::new(|| [BHD, JOD, KWD, OMR, TND].map(|currency| (currency, 1000)));

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
    ///  use amount_conversion::factor::{Currency::{self,*}, FromCurrency};
    ///
    /// #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    /// enum UserCurrency {
    ///     Inr,
    ///     Usd,
    /// }
    ///
    /// impl FromCurrency for UserCurrency {
    /// fn currency(&self) -> Currency {
    ///    match self {
    ///        UserCurrency::Inr => INR,
    ///        UserCurrency::Usd => USD,
    ///    }
    ///  }
    /// }
    /// let custom_currency = UserCurrency::Inr;
    /// let currency = INR;
    ///
    /// assert_eq!(currency, custom_currency.currency());
    /// ```
    fn currency(&self) -> Currency;
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Currency {
    AED,
    ALL,
    AMD,
    ANG,
    ARS,
    AUD,
    AWG,
    AZN,
    BBD,
    BDT,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BRL,
    BSD,
    BWP,
    BZD,
    CAD,
    CHF,
    CLP,
    CNY,
    COP,
    CRC,
    CUP,
    CZK,
    DKK,
    DOP,
    DJF,
    DZD,
    EGP,
    ETB,
    EUR,
    FJD,
    GBP,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RUB,
    RWF,
    SAR,
    SCR,
    SEK,
    SGD,
    SLL,
    SOS,
    SSP,
    SVC,
    SZL,
    THB,
    TND,
    TTD,
    TWD,
    TZS,
    UGX,
    USD,
    UYU,
    UZS,
    VND,
    VUV,
    XAF,
    XOF,
    XPF,
    YER,
    ZAR,
}

pub(crate) fn get_factor<T, Cur: FromCurrency>(
    amount: &amount::MoneyInner<T, Cur>,
) -> Result<f64, amount::MoneyConversionError<Cur>> {
    Ok(*SUBUNIT
        .get(&amount.currency.currency())
        .ok_or(amount::MoneyConversionError::CurrencyNotFoundInSubunitMap(amount.currency))?
        as f64)
}
