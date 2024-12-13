#[derive(Hash, Eq, PartialEq)]
pub enum Diameter {
    Half,
    ThreeQuarter,
    One,
    OneAndOneQuarter,
    OneAndHalf,
    Two,
    TwoAndHalf,
    Three,
    ThreeAndHalf,
    Four,
    Five,
    Six,
    Eight,
    Ten,
    Twelve,
}

impl Diameter {
    pub fn value(&self) -> f64 {
        match *self {
            Self::Half => 0.5,
            Self::ThreeQuarter => 0.75,
            Self::One => 1.0,
            Self::OneAndOneQuarter => 1.25,
            Self::OneAndHalf => 1.50,
            Self::Two => 2.0,
            Self::TwoAndHalf => 2.50,
            Self::Three => 3.0,
            Self::ThreeAndHalf => 3.50,
            Self::Four => 4.0,
            Self::Five => 5.0,
            Self::Six => 6.0,
            Self::Eight => 8.0,
            Self::Ten => 10.0,
            Self::Twelve => 12.0,
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Self::Half => "½\"",
            Self::ThreeQuarter => "¾\"",
            Self::One => "1\"",
            Self::OneAndOneQuarter => "1¼\"",
            Self::OneAndHalf => "1½\"",
            Self::Two => "2\"",
            Self::TwoAndHalf => "2½\"",
            Self::Three => "3\"",
            Self::ThreeAndHalf => "3½\"",
            Self::Four => "4\"",
            Self::Five => "5\"",
            Self::Six => "6\"",
            Self::Eight => "8\"",
            Self::Ten => "10\"",
            Self::Twelve => "12\"",
        }
        .into()
    }
}
