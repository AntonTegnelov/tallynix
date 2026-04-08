use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::model::Invoice;

pub fn render(invoice: &Invoice, project_root: &Path) -> Result<PathBuf> {
    let tmp_dir = project_root.join("tmp");
    std::fs::create_dir_all(&tmp_dir)
        .with_context(|| format!("Failed to create {}", tmp_dir.display()))?;

    let output_dir = project_root.join("output");
    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create {}", output_dir.display()))?;

    let typ_path = tmp_dir.join(format!("{}.typ", invoice.invoice_number));
    let pdf_path = output_dir.join(format!("{}.pdf", invoice.invoice_number));

    let preamble = build_preamble(invoice);
    let template = std::fs::read_to_string(project_root.join("templates/invoice.typ"))
        .context("Failed to read templates/invoice.typ")?;
    let source = format!("{}\n{}", preamble, template);

    std::fs::write(&typ_path, &source)
        .with_context(|| format!("Failed to write {}", typ_path.display()))?;

    let output = Command::new("typst")
        .args([
            "compile",
            &typ_path.to_string_lossy(),
            &pdf_path.to_string_lossy(),
            "--root",
            &project_root.to_string_lossy(),
        ])
        .output()
        .context("Failed to run typst. Is typst installed and on PATH?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "typst compile failed (exit {}):\n{}\nTypst source preserved at: {}",
            output.status.code().unwrap_or(-1),
            stderr,
            typ_path.display()
        );
    }

    // Clean up temp file on success
    let _ = std::fs::remove_file(&typ_path);

    Ok(pdf_path)
}

fn escape_typst_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn build_preamble(invoice: &Invoice) -> String {
    let mut p = String::new();

    // Company
    p.push_str(&format!(
        "#let company_name = \"{}\"\n",
        escape_typst_string(&invoice.company_name)
    ));
    p.push_str(&format!(
        "#let company_address_line1 = \"{}\"\n",
        escape_typst_string(&invoice.company_address_line1)
    ));
    match &invoice.company_address_line2 {
        Some(line2) => p.push_str(&format!(
            "#let company_address_line2 = \"{}\"\n",
            escape_typst_string(line2)
        )),
        None => p.push_str("#let company_address_line2 = none\n"),
    }
    p.push_str(&format!(
        "#let company_postal_code = \"{}\"\n",
        escape_typst_string(&invoice.company_postal_code)
    ));
    p.push_str(&format!(
        "#let company_city = \"{}\"\n",
        escape_typst_string(&invoice.company_city)
    ));
    p.push_str(&format!(
        "#let company_org_number = \"{}\"\n",
        escape_typst_string(&invoice.company_org_number)
    ));
    p.push_str(&format!(
        "#let company_vat_number = \"{}\"\n",
        escape_typst_string(&invoice.company_vat_number)
    ));
    p.push_str(&format!(
        "#let company_f_skatt = {}\n",
        invoice.company_f_skatt
    ));
    p.push_str(&format!(
        "#let logo_path = \"/{}\"\n",
        escape_typst_string(&invoice.logo_path)
    ));

    // Payment
    p.push_str(&format!(
        "#let bank = \"{}\"\n",
        escape_typst_string(&invoice.bank)
    ));
    p.push_str(&format!(
        "#let swift_bic = \"{}\"\n",
        escape_typst_string(&invoice.swift_bic)
    ));
    p.push_str(&format!(
        "#let iban = \"{}\"\n",
        escape_typst_string(&invoice.iban)
    ));
    p.push_str(&format!(
        "#let payment_terms_days = {}\n",
        invoice.payment_terms_days
    ));
    p.push_str(&format!(
        "#let late_interest_rate_percent = {}\n",
        invoice.late_interest_rate_percent
    ));
    p.push_str(&format!(
        "#let contact_name = \"{}\"\n",
        escape_typst_string(&invoice.contact_name)
    ));
    p.push_str(&format!(
        "#let contact_email = \"{}\"\n",
        escape_typst_string(&invoice.contact_email)
    ));
    p.push_str(&format!(
        "#let website = \"{}\"\n",
        escape_typst_string(&invoice.website)
    ));

    // Customer
    p.push_str(&format!(
        "#let customer_name = \"{}\"\n",
        escape_typst_string(&invoice.customer_name)
    ));
    p.push_str(&format!(
        "#let customer_contact_name = \"{}\"\n",
        escape_typst_string(&invoice.customer_contact_name)
    ));
    p.push_str(&format!(
        "#let customer_address_line1 = \"{}\"\n",
        escape_typst_string(&invoice.customer_address_line1)
    ));
    p.push_str(&format!(
        "#let customer_postal_code = \"{}\"\n",
        escape_typst_string(&invoice.customer_postal_code)
    ));
    p.push_str(&format!(
        "#let customer_city = \"{}\"\n",
        escape_typst_string(&invoice.customer_city)
    ));
    p.push_str(&format!(
        "#let customer_org_number = \"{}\"\n",
        escape_typst_string(&invoice.customer_org_number)
    ));
    p.push_str(&format!(
        "#let customer_vat_number = \"{}\"\n",
        escape_typst_string(&invoice.customer_vat_number)
    ));

    // Invoice
    p.push_str(&format!(
        "#let invoice_number = \"{}\"\n",
        escape_typst_string(&invoice.invoice_number)
    ));
    p.push_str(&format!(
        "#let invoice_date = \"{}\"\n",
        invoice.invoice_date.format("%Y-%m-%d")
    ));
    p.push_str(&format!(
        "#let due_date = \"{}\"\n",
        invoice.due_date.format("%Y-%m-%d")
    ));
    p.push_str(&format!(
        "#let buyer_po_number = \"{}\"\n",
        escape_typst_string(&invoice.buyer_po_number)
    ));
    p.push_str(&format!(
        "#let buyer_reference = \"{}\"\n",
        escape_typst_string(&invoice.buyer_reference)
    ));
    p.push_str(&format!(
        "#let currency = \"{}\"\n",
        escape_typst_string(&invoice.currency)
    ));

    // Line items array
    p.push_str("#let line_items = (\n");
    for li in &invoice.line_items {
        let sub_items_typst = if li.sub_items.is_empty() {
            "()".to_string()
        } else {
            let items: Vec<String> = li
                .sub_items
                .iter()
                .map(|s| format!("\"{}\"", escape_typst_string(s)))
                .collect();
            format!("({},)", items.join(", "))
        };
        p.push_str(&format!(
            "  (description: \"{}\", date: \"{}\", quantity: {}, unit: \"{}\", unit_price: {}, vat_rate_percent: {}, belopp: {}, sub_items: {}),\n",
            escape_typst_string(&li.description),
            li.date.format("%Y-%m-%d"),
            li.quantity,
            escape_typst_string(&li.unit),
            li.unit_price,
            li.vat_rate_percent,
            li.belopp,
            sub_items_typst,
        ));
    }
    p.push_str(")\n");

    // VAT groups
    p.push_str("#let vat_groups = (\n");
    for vg in &invoice.vat_groups {
        p.push_str(&format!("  (rate: {}, amount: {}),\n", vg.rate, vg.amount,));
    }
    p.push_str(")\n");

    // Totals
    p.push_str(&format!(
        "#let totalt_exkl_moms = {}\n",
        invoice.totalt_exkl_moms
    ));
    p.push_str(&format!(
        "#let summa_att_betala = {}\n",
        invoice.summa_att_betala
    ));

    p
}
