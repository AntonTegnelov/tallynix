use chrono::NaiveDate;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::config::{CompanyConfig, CustomerConfig, InvoiceConfig, PaymentConfig};

#[derive(Debug, Serialize)]
pub struct Invoice {
    // Company
    pub company_name: String,
    pub company_address_line1: String,
    pub company_address_line2: Option<String>,
    pub company_postal_code: String,
    pub company_city: String,
    pub company_org_number: String,
    pub company_vat_number: String,
    pub company_f_skatt: bool,
    pub logo_path: String,
    // Payment
    pub bank: String,
    pub swift_bic: String,
    pub iban: String,
    pub payment_terms_days: u32,
    pub late_interest_rate_percent: f64,
    pub contact_name: String,
    pub contact_email: String,
    pub website: String,
    // Customer
    pub customer_name: String,
    pub customer_contact_name: String,
    pub customer_address_line1: String,
    pub customer_postal_code: String,
    pub customer_city: String,
    pub customer_org_number: String,
    pub customer_vat_number: String,
    // Invoice
    pub invoice_number: String,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub buyer_po_number: String,
    pub buyer_reference: String,
    pub currency: String,
    pub line_items: Vec<LineItem>,
    // Computed
    pub totalt_exkl_moms: f64,
    pub vat_groups: Vec<VatGroup>,
    pub summa_att_betala: f64,
}

#[derive(Debug, Serialize)]
pub struct LineItem {
    pub description: String,
    pub date: NaiveDate,
    pub quantity: f64,
    pub unit: String,
    pub unit_price: f64,
    pub vat_rate_percent: f64,
    pub belopp: f64,
    pub sub_items: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct VatGroup {
    pub rate: f64,
    pub amount: f64,
}

impl Invoice {
    pub fn from_configs(
        company: CompanyConfig,
        payment: PaymentConfig,
        customer: CustomerConfig,
        invoice: InvoiceConfig,
    ) -> Invoice {
        let line_items: Vec<LineItem> = invoice
            .line_items
            .into_iter()
            .map(|li| {
                let belopp = li.quantity * li.unit_price * (1.0 + li.vat_rate_percent / 100.0);
                LineItem {
                    description: li.description,
                    date: li.date,
                    quantity: li.quantity,
                    unit: li.unit,
                    unit_price: li.unit_price,
                    vat_rate_percent: li.vat_rate_percent,
                    belopp,
                    sub_items: li.sub_items.unwrap_or_default(),
                }
            })
            .collect();

        let totalt_exkl_moms: f64 = line_items
            .iter()
            .map(|li| li.quantity * li.unit_price)
            .sum();

        // Group VAT by rate using BTreeMap for deterministic ordering
        let mut vat_map: BTreeMap<String, f64> = BTreeMap::new();
        for li in &line_items {
            let key = format!("{:.2}", li.vat_rate_percent);
            *vat_map.entry(key).or_insert(0.0) +=
                li.quantity * li.unit_price * li.vat_rate_percent / 100.0;
        }
        let vat_groups: Vec<VatGroup> = vat_map
            .into_iter()
            .map(|(key, amount)| VatGroup {
                rate: key.parse().unwrap_or(0.0),
                amount,
            })
            .collect();

        let total_moms: f64 = vat_groups.iter().map(|vg| vg.amount).sum();
        let summa_att_betala = totalt_exkl_moms + total_moms;

        Invoice {
            company_name: company.name,
            company_address_line1: company.address_line1,
            company_address_line2: company.address_line2,
            company_postal_code: company.postal_code,
            company_city: company.city,
            company_org_number: company.org_number,
            company_vat_number: company.vat_number,
            company_f_skatt: company.f_skatt,
            logo_path: company.logo_path,
            bank: payment.bank,
            swift_bic: payment.swift_bic,
            iban: payment.iban,
            payment_terms_days: payment.default_payment_terms_days,
            late_interest_rate_percent: payment.late_interest_rate_percent,
            contact_name: payment.contact_name,
            contact_email: payment.contact_email,
            website: payment.website,
            customer_name: customer.name,
            customer_contact_name: customer.contact_name,
            customer_address_line1: customer.address_line1,
            customer_postal_code: customer.postal_code,
            customer_city: customer.city,
            customer_org_number: customer.org_number,
            customer_vat_number: customer.vat_number,
            invoice_number: invoice.invoice_number,
            invoice_date: invoice.invoice_date,
            due_date: invoice.due_date,
            buyer_po_number: invoice.buyer_po_number,
            buyer_reference: invoice.buyer_reference,
            currency: invoice.currency,
            line_items,
            totalt_exkl_moms,
            vat_groups,
            summa_att_betala,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::invoice::LineItemConfig;

    fn sample_configs() -> (CompanyConfig, PaymentConfig, CustomerConfig, InvoiceConfig) {
        let company = CompanyConfig {
            name: "Parabel Game Studio AB".into(),
            address_line1: "Vivelvägen 22".into(),
            address_line2: Some("LGH 1110".into()),
            postal_code: "756 60".into(),
            city: "Uppsala".into(),
            org_number: "559502-5072".into(),
            vat_number: "SE559502507201".into(),
            f_skatt: true,
            logo_path: "assets/parabel_logo_dark.png".into(),
        };
        let payment = PaymentConfig {
            bank: "Handelsbanken".into(),
            swift_bic: "HANDSESS".into(),
            iban: "SE27 6000 0000 0005 2805 2632".into(),
            default_payment_terms_days: 30,
            late_interest_rate_percent: 8.0,
            contact_name: "Anton Tegnelöv".into(),
            contact_email: "anton.tegnelov@parabelgamestudio.com".into(),
            website: "www.parabelgamestudio.com".into(),
        };
        let customer = CustomerConfig {
            name: "SLU Holding AB".into(),
            contact_name: "Henrik Landgren".into(),
            address_line1: "Ulls väg 29C".into(),
            postal_code: "756 51".into(),
            city: "Uppsala".into(),
            org_number: "556518-7423".into(),
            vat_number: "SE556518742301".into(),
        };
        let invoice = InvoiceConfig {
            invoice_number: "26-1".into(),
            invoice_date: NaiveDate::from_ymd_opt(2026, 1, 22).unwrap(),
            due_date: NaiveDate::from_ymd_opt(2026, 2, 21).unwrap(),
            buyer_po_number: "Proj.nr. 9550071".into(),
            buyer_reference: "Att. Henrik Landgren".into(),
            currency: "SEK".into(),
            line_items: vec![
                LineItemConfig {
                    description: "Modul 1: Grundplattform och arkitektur".into(),
                    date: NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
                    quantity: 1.0,
                    unit: "st".into(),
                    unit_price: 25000.0,
                    vat_rate_percent: 25.0,
                    sub_items: None,
                },
                LineItemConfig {
                    description: "Modul 2: Proof of Concept".into(),
                    date: NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
                    quantity: 1.0,
                    unit: "st".into(),
                    unit_price: 25000.0,
                    vat_rate_percent: 25.0,
                    sub_items: None,
                },
            ],
        };
        (company, payment, customer, invoice)
    }

    #[test]
    fn test_computed_fields() {
        let (company, payment, customer, invoice) = sample_configs();
        let inv = Invoice::from_configs(company, payment, customer, invoice);

        assert_eq!(inv.totalt_exkl_moms, 50_000.0);
        assert_eq!(inv.vat_groups.len(), 1);
        assert_eq!(inv.vat_groups[0].rate, 25.0);
        assert_eq!(inv.vat_groups[0].amount, 12_500.0);
        assert_eq!(inv.summa_att_betala, 62_500.0);
        assert_eq!(inv.line_items[0].belopp, 31_250.0);
        assert_eq!(inv.line_items[1].belopp, 31_250.0);
    }
}
