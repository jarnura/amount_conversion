# Money_conversion
Money conversion from lower subunit to higher unit and vice-versa

<p></p>

```rust

use amount_conversion::factor::{Currency::{self,*}, FromCurrency};
   
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
enum UserCurrency {
    Inr,
    Usd,
}

impl FromCurrency for UserCurrency {
fn currency(&self) -> Currency {
   match self {
       UserCurrency::Inr => INR,
       UserCurrency::Usd => USD,
   }
}   

type Money = MoneyInner<LowestSubunit, Currency>;
type MoneyH = MoneyInner<HighestUnit, Currency>;

#[derive(serde::Deserialize)]
struct Request {
    #[serde(flatten)]
    amount: Money,
    id: i8,
}

let amount_str = r#"{
    "amount": 1,
    "currency": "Inr",
    "id": 1
}"#;

let request = serde_json::from_str::<Request>(amount_str)?;

let highest_unit: MoneyH = request.amount.convert()?;
let lowest_unit: Money = highest_unit.convert()?;
assert_eq!(request.amount, lowest_unit);
```

