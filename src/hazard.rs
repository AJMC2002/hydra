pub enum Hazard {
    Light,
    OrdinaryGroup1,
    OrdinaryGroup2,
    ExtraGroup1,
    ExtraGroup2,
}

impl Hazard {
    /// Based on Tables 10.2.4.2.1 (a), (b), (c), (d)
    pub fn maximum_protection_area(&self) -> u64 {
        match *self {
            Hazard::Light => 225,
            Hazard::OrdinaryGroup1 => 130,
            Hazard::OrdinaryGroup2 => 130,
            Hazard::ExtraGroup1 => 100,
            Hazard::ExtraGroup2 => 100,
        }
    }

    /// Based on Tables 10.2.4.2.1 (a), (b), (c), (d)
    pub fn maximum_spacing(&self) -> f64 {
        match *self {
            Hazard::Light => 15.0,
            Hazard::OrdinaryGroup1 => 15.0,
            Hazard::OrdinaryGroup2 => 15.0,
            Hazard::ExtraGroup1 => 12.0,
            Hazard::ExtraGroup2 => 12.0,
        }
    }

    /// Based on Table 19.2.3.1.1 Density/Area
    pub fn density_area(&self) -> [(f64, u64); 2] {
        match *self {
            Hazard::Light => [(0.1, 1500), (0.07, 3000)],
            Hazard::OrdinaryGroup1 => [(0.15, 1500), (0.12, 3000)],
            Hazard::OrdinaryGroup2 => [(0.2, 1500), (0.17, 3000)],
            Hazard::ExtraGroup1 => [(0.3, 2500), (0.28, 3000)],
            Hazard::ExtraGroup2 => [(0.4, 2500), (0.38, 3000)],
        }
    }

    /// Based on Table 19.2.3.1.2
    pub fn duration(&self) -> Vec<u64> {
        match *self {
            Hazard::Light => vec![30],
            Hazard::OrdinaryGroup1 => vec![60, 90],
            Hazard::OrdinaryGroup2 => vec![60, 90],
            Hazard::ExtraGroup1 => vec![90, 120],
            Hazard::ExtraGroup2 => vec![90, 120],
        }
    }
}
