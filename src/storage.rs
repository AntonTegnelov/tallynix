use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::model::Invoice;

pub fn save_invoice(invoices_dir: &Path, invoice: &Invoice) -> Result<PathBuf> {
    std::fs::create_dir_all(invoices_dir)
        .with_context(|| format!("Failed to create directory {}", invoices_dir.display()))?;

    let filename = format!("{}.json", invoice.invoice_number);
    let path = invoices_dir.join(filename);

    let json = serde_json::to_string_pretty(invoice).context("Failed to serialize invoice")?;
    std::fs::write(&path, json).with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(path)
}
