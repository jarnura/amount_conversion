use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::amount;

static SUBUNIT: Lazy<HashMap<&str, i16>> = Lazy::new(|| {
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

static ZERO_DECIMAL_PAIR: Lazy<[(&str, i8); 16]> = Lazy::new(|| {
    [
        "BIF", "CLP", "DJF", "GNF", "JPY", "KMF", "KRW", "MGA", "PYG", "RWF", "UGX", "VND", "VUV",
        "XAF", "XOF", "XPF",
    ]
    .map(|currency| (currency, 1))
});

static TWO_DECIMAL_PAIR: Lazy<[(&str, i8); 98]> = Lazy::new(|| {
    [
        "AED", "ALL", "AMD", "ANG", "ARS", "AUD", "AWG", "AZN", "BBD", "BDT", "BMD", "BND", "BOB",
        "BRL", "BSD", "BWP", "BZD", "CAD", "CHF", "CNY", "COP", "CRC", "CUP", "CZK", "DKK", "DOP",
        "DZD", "EGP", "ETB", "EUR", "FJD", "GBP", "GHS", "GIP", "GMD", "GTQ", "GYD", "HKD", "HNL",
        "HRK", "HTG", "HUF", "IDR", "ILS", "INR", "JMD", "KES", "KGS", "KHR", "KYD", "KZT", "LAK",
        "LBP", "LKR", "LRD", "LSL", "MAD", "MDL", "MKD", "MMK", "MNT", "MOP", "MUR", "MVR", "MWK",
        "MXN", "MYR", "NAD", "NGN", "NIO", "NOK", "NPR", "NZD", "PEN", "PGK", "PHP", "PKR", "PLN",
        "QAR", "RUB", "SAR", "SCR", "SEK", "SGD", "SLL", "SOS", "SSP", "SVC", "SZL", "THB", "TTD",
        "TWD", "TZS", "USD", "UYU", "UZS", "YER", "ZAR",
    ]
    .map(|currency| (currency, 100))
});

static THREE_DECIMAL_PAIR: Lazy<[(&str, i16); 5]> =
    Lazy::new(|| ["BHD", "JOD", "KWD", "OMR", "TND"].map(|currency| (currency, 1000)));

pub(crate) fn get_factor<T, Cur: amount::FromCurrency>(
    amount: &amount::AmountInner<T, Cur>,
) -> Result<f64, amount::AmmountConversionError<Cur>> {
    Ok(*SUBUNIT
        .get(amount.currency.currency())
        .ok_or(amount::AmmountConversionError::CurrencyNotFoundInSubunitMap(amount.currency))?
        as f64)
}
