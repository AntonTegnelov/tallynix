use anyhow::{Context, Result};
use std::path::Path;

use crate::config;
use crate::model::Invoice;
use crate::renderer;
use crate::storage;

fn format_swedish_amount(amount: f64) -> String {
    let rounded = (amount * 100.0).round() / 100.0;
    let integer_part = rounded as i64;
    let decimal_part = ((rounded - integer_part as f64) * 100.0).round() as i64;

    let int_str = integer_part.to_string();
    let mut with_spaces = String::new();
    for (i, ch) in int_str.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            with_spaces.push(' ');
        }
        with_spaces.push(ch);
    }
    let int_formatted: String = with_spaces.chars().rev().collect();

    format!("{},{:02}", int_formatted, decimal_part)
}

pub fn generate(config_dir: &Path, project_root: &Path) -> Result<()> {
    let (company, payment, customer, invoice) = config::load_all(config_dir)?;
    let invoice = Invoice::from_configs(company, payment, customer, invoice);

    let record_path = storage::save_invoice(&project_root.join("invoices"), &invoice)?;
    println!("Invoice record saved: {}", record_path.display());

    let pdf_path = renderer::render(&invoice, project_root)?;

    // Rename to canonical filename
    let output_dir = project_root.join("output");
    let canonical_name = format!(
        "FAKTURA {} - {} - {} {}.pdf",
        invoice.invoice_number,
        invoice.customer_name,
        format_swedish_amount(invoice.summa_att_betala),
        invoice.currency,
    );
    let canonical_path = output_dir.join(&canonical_name);

    if pdf_path != canonical_path {
        std::fs::rename(&pdf_path, &canonical_path)
            .with_context(|| format!("Failed to rename PDF to {}", canonical_path.display()))?;
    }

    println!("Invoice PDF generated: {}", canonical_path.display());
    Ok(())
}
