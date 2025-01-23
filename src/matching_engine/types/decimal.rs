use bytes::BytesMut;
use postgres_types::{FromSql, IsNull, ToSql, Type};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Price(pub Decimal);

impl<'a> FromSql<'a> for Price {
    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = String::from_sql(ty, raw)?;
        let decimal = Decimal::from_str(&s)?;
        Ok(Price(decimal))
    }

    fn accepts(ty: &Type) -> bool {
        matches!(ty.name(), "numeric" | "money")
    }
}

impl ToSql for Price {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        matches!(ty.name(), "numeric" | "money")
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.to_sql(ty, out)
    }
}

// Common operations
impl From<Decimal> for Price {
    fn from(decimal: Decimal) -> Self {
        Price(decimal)
    }
}

impl From<Price> for Decimal {
    fn from(price: Price) -> Self {
        price.0
    }
}

// Add arithmetic operations if needed
impl std::ops::Add for Price {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Price(self.0 + other.0)
    }
}

impl std::ops::Sub for Price {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Price(self.0 - other.0)
    }
}

impl std::ops::Mul for Price {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Price(self.0 * other.0)
    }
}

impl std::ops::Div for Price {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Price(self.0 / other.0)
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
