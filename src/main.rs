slint::include_modules!();

const TAX_PERCENTAGE: f64 = 0.30;
const EXPENSES_PERCENTAGE: f64 = 0.40;
const INVESTMENTS_PERCENTAGE: f64 = 0.10;
const SAVINGS_PERCENTAGE: f64 = 0.10;
const EMERGENCY_PERCENTAGE: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_calculate(move |string| {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();

        let tax: f64 = num * TAX_PERCENTAGE;
        let expenses: f64 = num * EXPENSES_PERCENTAGE;
        let investments: f64 = num * INVESTMENTS_PERCENTAGE;
        let savings: f64 = num * SAVINGS_PERCENTAGE;
        let emergency: f64 = num * EMERGENCY_PERCENTAGE;

        let result = format!("Taxes: {tax:.2}\nExpenses: {expenses:.2}\nInvestments: {investments:.2}\nSavings: {savings:.2}\nEmergency: {emergency:.2}");
        ui.set_results(result.into());
    });

    ui.run()
}
