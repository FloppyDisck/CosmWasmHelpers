use cosmwasm_std::{Uint128, Uint256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
// Allows to easily calculate fees
pub struct FeeSplit {
    // The resulting amount after getting the fee
    pub result: Uint128,
    // Fee received
    pub fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub enum FeePercent {
    #[default]
    Disabled,
    /// Simple numbers: 90 = 90%
    Simple(u8),
    /// Decimal granularity: 9050 = 90.50%
    Decimal(u16),
}

impl FeePercent {
    // Check for disabled fees
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Simple(value) => *value == 0,
            Self::Decimal(value) => *value == 0,
            Self::Disabled => true
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Simple(value) => *value == 100,
            Self::Decimal(value) => *value == 10000
        }
    }
}

impl Into<Uint256> for FeePercent {
    fn into(self) -> Uint256 {
        match self {
            FeePercent::Disabled => Uint256::zero(),
            FeePercent::Simple(value) => Uint256::from(value as u16 * 100),
            FeePercent::Decimal(value) => Uint256::from(value)
        }
    }
}

impl From<u8> for FeePercent {
    fn from(value: u8) -> Self {
        Self::Simple(value)
    }
}

impl From<Option<u8>> for FeePercent {
    fn from(value: Option<u8>) -> Self {
        match value {
            None => Self::Disabled,
            Some(value) => value.into()
        }
    }
}

impl From<u16> for FeePercent {
    fn from(value: u16) -> Self {
        Self::Decimal(value)
    }
}

impl From<Option<u16>> for FeePercent {
    fn from(value: Option<u16>) -> Self {
        match value {
            None => Self::Disabled,
            Some(value) => value.into()
        }
    }
}

impl FeeSplit {
    pub fn new(amount: impl Into<Uint128>, fee: impl Into<FeePercent>) -> Self {
        let amount = amount.into();
        let fee = fee.into();

        // Return seller amount since fee is zero
        if fee.is_zero() {
            return Self::only_seller(amount);
        }

        // Return full fee since fee is full
        if fee.is_full() {
            return Self::only_fee(amount);
        }

        // Allocate extra space
        let decimal_amount = Uint256::from_uint128(amount) * Uint256::from(10000u32);

        // Get fee and "floor" and we can safely unwrap because we've tested against max
        let calculated_fee: Uint256 = fee.into();
        let fee: Uint128 = (decimal_amount * calculated_fee).checked_div(Uint256::from(100000000u64)).unwrap_or(Uint256::zero()).try_into().unwrap();

        Self {
            result: amount - fee,
            fee,
        }
    }

    pub fn only_seller(amount: impl Into<Uint128>) -> Self {
        Self {
            result: amount.into(),
            fee: Uint128::zero(),
        }
    }

    pub fn only_fee(amount: impl Into<Uint128>) -> Self {
        Self {
            result: Uint128::zero(),
            fee: amount.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::Uint128;
    use crate::fees::FeePercent::Simple;
    use crate::fees::FeeSplit;

    #[test]
    fn fee_percentage_overflow() {
        // Test small number
        let amount = Uint128::new(123456);
        let expected = Uint128::new(43209);
        assert_eq!(FeeSplit::new(amount, Simple(35)).fee, expected);

        // Test largest number
        let amount = Uint128::new(100000000000000000000000000000000000000);
        let expected = Uint128::new(50000000000000000000000000000000000000);
        assert_eq!(FeeSplit::new(amount, Simple(50)).fee, expected);

        // Testing for overflow
        FeeSplit::new(Uint128::MAX, Simple(1));
        FeeSplit::new(Uint128::MAX, Simple(0));
        FeeSplit::new(Uint128::MAX, Simple(100));

        // Testing for underflow
        assert_eq!(FeeSplit::new(Uint128::one(), Simple(0)).fee, Uint128::zero());
        assert_eq!(FeeSplit::new(Uint128::one(), Simple(1)).fee, Uint128::zero());
        assert_eq!(FeeSplit::new(Uint128::one(), Simple(10)).fee, Uint128::zero());
        assert_eq!(FeeSplit::new(Uint128::one(), Simple(100)).fee, Uint128::one());

        assert_eq!(FeeSplit::new(Uint128::zero(), Simple(0)).fee, Uint128::zero());
        assert_eq!(FeeSplit::new(Uint128::zero(), Simple(1)).fee, Uint128::zero());
        assert_eq!(FeeSplit::new(Uint128::zero(), Simple(10)).fee, Uint128::zero());
        assert_eq!(FeeSplit::new(Uint128::zero(), Simple(100)).fee, Uint128::zero());
    }
}