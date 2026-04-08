# Tallynix

A CLI tool for generating Swedish invoices as PDF files.

## Prerequisites

- [Rust toolchain](https://rustup.rs/)
- [Typst CLI](https://github.com/typst/typst) installed and on PATH

## Setup

Build the project:

```sh
cargo build --release
```

## Configuration

All input is provided via JSON files in a `config/` directory (four files):

### `config/company.json`

Your company details. Typically set once.

```json
{
  "name": "Företag AB",
  "address_line1": "Gatuvägen 1",
  "address_line2": "LGH 1001",
  "postal_code": "123 45",
  "city": "Stockholm",
  "org_number": "559999-1234",
  "vat_number": "SE559999123401",
  "f_skatt": true,
  "logo_path": "assets/logo.png"
}

```

`address_line2` is optional (set to `null` or omit it).

### `config/payment.json`

Bank and contact details. Typically set once.

```json
{
  "bank": "Handelsbanken",
  "swift_bic": "HANDSESS",
  "iban": "SE27 6000 0000 0005 1234 5678",
  "default_payment_terms_days": 30,
  "late_interest_rate_percent": 8.0,
  "contact_name": "Namn Namnsson",
  "contact_email": "namn@foretag.se",
  "website": "www.foretag.se"
}
```

### `config/customer.json`

The invoice recipient. Update per customer.

```json
{
  "name": "Kund AB",
  "contact_name": "Anna Andersson",
  "address_line1": "Kundvägen 5",
  "postal_code": "543 21",
  "city": "Uppsala",
  "org_number": "556789-0123",
  "vat_number": "SE556789012301"
}
```

### `config/invoice.json`

The invoice-specific data. Update for each new invoice.

```json
{
  "invoice_number": "26-1",
  "invoice_date": "2026-01-22",
  "due_date": "2026-02-21",
  "buyer_po_number": "Proj.nr. 9550071",
  "buyer_reference": "Att. Anna Andersson",
  "currency": "SEK",
  "line_items": [
    {
      "description": "Konsulttjänst",
      "date": "2026-01-15",
      "quantity": 1.0,
      "unit": "st",
      "unit_price": 25000.0,
      "vat_rate_percent": 25.0
    }
  ]
}
```

#### Sub-items

Line items can include an optional `sub_items` array for display-only detail rows (e.g. re-invoiced costs):

```json
{
  "description": "Driftskostnader februari (vidarefakturerade utan påslag)",
  "date": "2026-02-28",
  "quantity": 1.0,
  "unit": "st",
  "unit_price": 233.89,
  "vat_rate_percent": 25.0,
  "sub_items": [
    "Relational Database Service",
    "Elastic Compute Cloud",
    "Virtual Private Cloud"
  ]
}
```

Sub-items appear as small italic text below the parent row in the PDF. They do not affect totals.

## Generating an Invoice

```sh
tallynix generate
```

Or with a custom config directory:

```sh
tallynix generate --config-dir path/to/config
```

This produces:

- **PDF** in `output/` (e.g. `FAKTURA 26-1 - Kund AB - 62 500,00 SEK.pdf`)
- **JSON record** in `invoices/` (e.g. `26-1.json`) with all computed fields

## License

MIT
