use std::collections::HashMap;

use crate::diameter::Diameter;

pub trait EquivalentLength {
    fn equivalent_length(&self, diameter: &Diameter) -> Option<f64>;
}

pub enum Fitting {
    Elbow45,
    Elbow90Standard,
    Elbow90Long,
    Tee,
    Cross,
}

impl EquivalentLength for Fitting {
    fn equivalent_length(&self, diameter: &Diameter) -> Option<f64> {
        let mut equivalent_lengths: HashMap<Diameter, f64> = HashMap::new();
        match self {
            Self::Elbow45 => {
                equivalent_lengths.insert(Diameter::ThreeQuarter, 1.0);
                equivalent_lengths.insert(Diameter::One, 1.0);
                equivalent_lengths.insert(Diameter::OneAndOneQuarter, 1.0);
                equivalent_lengths.insert(Diameter::OneAndHalf, 2.0);
                equivalent_lengths.insert(Diameter::Two, 2.0);
                equivalent_lengths.insert(Diameter::TwoAndHalf, 3.0);
                equivalent_lengths.insert(Diameter::Three, 3.0);
                equivalent_lengths.insert(Diameter::ThreeAndHalf, 3.0);
                equivalent_lengths.insert(Diameter::Four, 4.0);
                equivalent_lengths.insert(Diameter::Five, 5.0);
                equivalent_lengths.insert(Diameter::Six, 7.0);
                equivalent_lengths.insert(Diameter::Eight, 9.0);
                equivalent_lengths.insert(Diameter::Ten, 11.0);
                equivalent_lengths.insert(Diameter::Twelve, 13.0);
            }
            Self::Elbow90Standard => {
                equivalent_lengths.insert(Diameter::Half, 1.0);
                equivalent_lengths.insert(Diameter::ThreeQuarter, 2.0);
                equivalent_lengths.insert(Diameter::One, 2.0);
                equivalent_lengths.insert(Diameter::OneAndOneQuarter, 3.0);
                equivalent_lengths.insert(Diameter::OneAndHalf, 4.0);
                equivalent_lengths.insert(Diameter::Two, 5.0);
                equivalent_lengths.insert(Diameter::TwoAndHalf, 6.0);
                equivalent_lengths.insert(Diameter::Three, 7.0);
                equivalent_lengths.insert(Diameter::ThreeAndHalf, 8.0);
                equivalent_lengths.insert(Diameter::Four, 10.0);
                equivalent_lengths.insert(Diameter::Five, 12.0);
                equivalent_lengths.insert(Diameter::Six, 14.0);
                equivalent_lengths.insert(Diameter::Eight, 18.0);
                equivalent_lengths.insert(Diameter::Ten, 22.0);
                equivalent_lengths.insert(Diameter::Twelve, 27.0);
            }
            Self::Elbow90Long => {
                equivalent_lengths.insert(Diameter::Half, 0.5);
                equivalent_lengths.insert(Diameter::ThreeQuarter, 1.0);
                equivalent_lengths.insert(Diameter::One, 2.0);
                equivalent_lengths.insert(Diameter::OneAndOneQuarter, 2.0);
                equivalent_lengths.insert(Diameter::OneAndHalf, 2.0);
                equivalent_lengths.insert(Diameter::Two, 3.0);
                equivalent_lengths.insert(Diameter::TwoAndHalf, 4.0);
                equivalent_lengths.insert(Diameter::Three, 5.0);
                equivalent_lengths.insert(Diameter::ThreeAndHalf, 5.0);
                equivalent_lengths.insert(Diameter::Four, 6.0);
                equivalent_lengths.insert(Diameter::Five, 8.0);
                equivalent_lengths.insert(Diameter::Six, 9.0);
                equivalent_lengths.insert(Diameter::Eight, 13.0);
                equivalent_lengths.insert(Diameter::Ten, 16.0);
                equivalent_lengths.insert(Diameter::Twelve, 18.0);
            }
            Self::Tee | Self::Cross => {
                equivalent_lengths.insert(Diameter::Half, 3.0);
                equivalent_lengths.insert(Diameter::ThreeQuarter, 4.0);
                equivalent_lengths.insert(Diameter::One, 5.0);
                equivalent_lengths.insert(Diameter::OneAndOneQuarter, 6.0);
                equivalent_lengths.insert(Diameter::OneAndHalf, 8.0);
                equivalent_lengths.insert(Diameter::Two, 10.0);
                equivalent_lengths.insert(Diameter::TwoAndHalf, 12.0);
                equivalent_lengths.insert(Diameter::Three, 15.0);
                equivalent_lengths.insert(Diameter::ThreeAndHalf, 17.0);
                equivalent_lengths.insert(Diameter::Four, 20.0);
                equivalent_lengths.insert(Diameter::Five, 25.0);
                equivalent_lengths.insert(Diameter::Six, 30.0);
                equivalent_lengths.insert(Diameter::Eight, 35.0);
                equivalent_lengths.insert(Diameter::Ten, 50.0);
                equivalent_lengths.insert(Diameter::Twelve, 60.0);
            }
        };
        equivalent_lengths.get(diameter).cloned()
    }
}

pub enum Valve {
    Butterfly,
    Gate,
}

impl EquivalentLength for Valve {
    fn equivalent_length(&self, diameter: &Diameter) -> Option<f64> {
        let mut equivalent_lengths: HashMap<Diameter, f64> = HashMap::new();
        match self {
            Self::Butterfly => {
                equivalent_lengths.insert(Diameter::Two, 6.0);
                equivalent_lengths.insert(Diameter::TwoAndHalf, 7.0);
                equivalent_lengths.insert(Diameter::Three, 10.0);
                equivalent_lengths.insert(Diameter::Four, 12.0);
                equivalent_lengths.insert(Diameter::Five, 9.0);
                equivalent_lengths.insert(Diameter::Six, 10.0);
                equivalent_lengths.insert(Diameter::Eight, 12.0);
                equivalent_lengths.insert(Diameter::Ten, 19.0);
                equivalent_lengths.insert(Diameter::Twelve, 21.0);
            }
            Self::Gate => {
                equivalent_lengths.insert(Diameter::Two, 1.0);
                equivalent_lengths.insert(Diameter::TwoAndHalf, 1.0);
                equivalent_lengths.insert(Diameter::Three, 1.0);
                equivalent_lengths.insert(Diameter::ThreeAndHalf, 1.0);
                equivalent_lengths.insert(Diameter::Four, 2.0);
                equivalent_lengths.insert(Diameter::Five, 2.0);
                equivalent_lengths.insert(Diameter::Six, 3.0);
                equivalent_lengths.insert(Diameter::Eight, 4.0);
                equivalent_lengths.insert(Diameter::Ten, 5.0);
                equivalent_lengths.insert(Diameter::Twelve, 6.0);
            }
        };
        equivalent_lengths.get(diameter).cloned()
    }
}

pub enum Accessory {
    Fitting(Fitting),
    Valve(Valve),
}
