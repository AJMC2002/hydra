mod accessory;
mod consts;
mod diameter;
mod formulas;
mod hazard;

use accessory::*;
use consts::*;
use formulas::*;

slint::include_modules!();

#[allow(dead_code)]
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.run()
}
