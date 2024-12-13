/// Formulas based on NFPA 13 28.2.2 for the design of sprinkler systems [SI Units]

/// 28.2.2.1.1-2 Pipe Friction Losses [Hazen-Williams formula]
pub fn frictional_resistance(
    flow: f64,
    friction_loss_coefficient: f64,
    inside_diameter: f64,
) -> f64 {
    4.52 * flow.powf(1.85) / (friction_loss_coefficient.powf(1.85) * inside_diameter.powf(4.87))
}

/// 28.2.2.1.3 Friction Loss [Darcy-Weisbach formula]
pub fn friction_loss(
    friction_loss_factor: f64,
    pipe_length: f64,
    fluid_density: f64,
    flow: f64,
    inside_diameter: f64,
) -> f64 {
    2.252 * friction_loss_factor * pipe_length * fluid_density * flow.powi(2)
        / inside_diameter.powi(5)
}

/// 28.2.2.2 Velocity Pressure Formula
pub fn velocity_pressure(flow: f64, inside_diameter: f64) -> f64 {
    0.001123 * flow.powi(2) / inside_diameter.powi(4)
}

/// 28.2.2.3 Normal Pressure
pub fn normal_pressure(total_pressure: f64, velocity_pressure: f64) -> f64 {
    total_pressure - velocity_pressure
}

/// 28.2.2.5 K-Factor
pub fn K_factor(flow: f64, pressure: f64) {}
