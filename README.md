# Amount_conversion
Amount conversion from lower subunit to higher unit and vice-versa

<p></p>

```rust

use amount_conversion::amount::FromCurrency;
   
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
enum Currency {
    Inr,
    Usd,
}

impl FromCurrency for Currency {
    fn currency(&self) -> &str {
        match self {
            Currency::Inr => "INR",
            Currency::Usd => "USD",
        }
    }
}

type Amount = AmountInner<LowestSubunit, Currency>;
type AmountH = AmountInner<HighestUnit, Currency>;

#[derive(serde::Deserialize)]
struct Request {
    #[serde(flatten)]
    amount: Amount,
    id: i8,
}

let amount_str = r#"{
    "amount": 1,
    "currency": "Inr",
    "id": 1
}"#;

let request = serde_json::from_str::<Request>(amount_str)?;

let highest_unit: AmountH = request.amount.convert()?;
let lowest_unit: Amount = highest_unit.convert()?;
assert_eq!(request.amount, lowest_unit);
```

