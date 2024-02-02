slint::include_modules!();

const TAX: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divideIncome(move |string|{
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax:f64 = num * TAX;
        let owner: f64 = num * OWNERPER;
        let profit: f64 = num * PROFITPER;
        let operating:f64 = num * OPEXPER;
        let results = format!("Taxes: {:.2}\n Owner: {:.2}\n Profit: {:.2}\n Operating Expenses: {:.2}",tax,owner,profit,operating);
        ui.set_results(results.into());
    });
    ui.run()
}
