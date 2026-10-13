use num_bigint::BigInt;
use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Eq)]
pub struct Decimal {
    mantissa: BigInt,
    exponent: i32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Self> {
        let (sign, digits) = match input.strip_prefix('-') {
            Some(rest) => (-1, rest),
            None => (1, input.strip_prefix('+').unwrap_or(input)),
        };

        let parts: Vec<&str> = digits.split('.').collect();
        if parts.len() > 2 {
            return None; // Invalid input with multiple decimal points
        }

        let int_part = parts[0];
        let frac_part = parts.get(1).copied().unwrap_or("");

        // Combined string of digits
        let combined = format!("{int_part}{frac_part}");
        let mantissa_raw = combined.parse::<BigInt>().ok()? * sign;
        let exponent = -(frac_part.len() as i32);

        Some(Decimal {
            mantissa: mantissa_raw,
            exponent,
        }
        .normalize())
    }

    /// Trims trailing zeros from mantissa while adjusting exponent
    fn normalize(mut self) -> Self {
        if self.mantissa == BigInt::from(0) {
            return Decimal {
                mantissa: BigInt::from(0),
                exponent: 0,
            };
        }

        let ten = BigInt::from(10);
        while &self.mantissa % &ten == BigInt::from(0) {
            self.mantissa /= &ten;
            self.exponent += 1;
        }
        self
    }

    /// Aligns two decimals to share the same exponent
    fn align_exponents(a: &Decimal, b: &Decimal) -> (BigInt, BigInt, i32) {
        let target_exp = a.exponent.min(b.exponent);

        let scale_a = a.exponent - target_exp;
        let scale_b = b.exponent - target_exp;

        let ten = BigInt::from(10);
        let mantissa_a = &a.mantissa * ten.pow(scale_a as u32);
        let mantissa_b = &b.mantissa * ten.pow(scale_b as u32);

        (mantissa_a, mantissa_b, target_exp)
    }
}

// --- Addition ---
impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (m_a, m_b, exp) = Decimal::align_exponents(&self, &rhs);
        Decimal {
            mantissa: m_a + m_b,
            exponent: exp,
        }
        .normalize()
    }
}

// --- Negation & Subtraction ---
impl Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Decimal {
            mantissa: -self.mantissa,
            exponent: self.exponent,
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

// --- Multiplication ---
impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Decimal {
            mantissa: self.mantissa * rhs.mantissa,
            exponent: self.exponent + rhs.exponent,
        }
        .normalize()
    }
}

// --- Equality & Comparisons ---
impl PartialEq for Decimal {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Decimal {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let (m_a, m_b, _) = Decimal::align_exponents(self, rhs);
        m_a.cmp(&m_b)
    }
}