// Tallynix Invoice Template
// All variables are injected via preamble by the renderer

#let fmt-currency(n) = {
  let rounded = calc.round(n * 100) / 100
  let integer = int(calc.floor(rounded))
  let decimal = int(calc.round((rounded - integer) * 100))

  let int-str = str(integer)
  let digits = int-str.clusters()
  let len = digits.len()
  let parts = ()
  let i = 0
  while i < len {
    let first-group = calc.rem(len, 3)
    if first-group == 0 { first-group = 3 }
    let group-size = if i == 0 { first-group } else { 3 }
    let group = digits.slice(i, i + group-size)
    parts.push(group.join())
    i += group-size
  }

  let decimal-str = if decimal < 10 { "0" + str(decimal) } else { str(decimal) }
  parts.join("\u{00A0}") + "," + decimal-str + " kr"
}

#let fmt-percent(n) = {
  let rounded = calc.round(n * 100) / 100
  let integer = int(calc.floor(rounded))
  let decimal = int(calc.round((rounded - integer) * 100))
  let decimal-str = if decimal < 10 { "0" + str(decimal) } else { str(decimal) }
  str(integer) + "," + decimal-str + "\u{00A0}%"
}

#let fmt-quantity(n) = {
  let rounded = calc.round(n * 100) / 100
  let integer = int(calc.floor(rounded))
  let decimal = int(calc.round((rounded - integer) * 100))
  let decimal-str = if decimal < 10 { "0" + str(decimal) } else { str(decimal) }
  str(integer) + "," + decimal-str
}

#let accent-color = rgb("#8B1A1A")
#let header-bg = luma(60)


#set page(
  paper: "a4",
  margin: (top: 2cm, bottom: 5.5cm, left: 2cm, right: 2cm),
  numbering: none,
  footer: {
    line(length: 100%, stroke: 0.5pt + luma(180))
    v(8pt)
    set text(size: 7.5pt)
    grid(
      columns: (1fr, 1fr, 1fr),
      column-gutter: 16pt,
      {
        text(weight: "bold", company_name)
        linebreak()
        company_address_line1
        linebreak()
        if company_address_line2 != none {
          company_address_line2
          linebreak()
        }
        company_postal_code + "\u{00A0}" + company_city
        linebreak()
        "Organisationsnummer: " + company_org_number
        linebreak()
        "Momsregistreringsnummer:" + linebreak() + company_vat_number
        linebreak()
        if company_f_skatt {
          "Godkänd för F-skatt"
        }
      },
      {
        text(weight: "bold", "Kontaktuppgifter")
        linebreak()
        contact_name
        linebreak()
        contact_email
        linebreak()
        website
      },
      {
        text(weight: "bold", "Betalningsuppgifter")
        linebreak()
        "Bank: " + bank
        linebreak()
        "SWIFT/BIC: " + swift_bic
        linebreak()
        "IBAN: " + iban
      },
    )
  }
)

#set text(font: "Libertinus Sans", size: 9.5pt)

// Header
#grid(
  columns: (auto, 1fr),
  column-gutter: 12pt,
  align(left + horizon, image(logo_path, width: 120pt)),
  align(right + bottom, text(size: 22pt, weight: "bold", "FAKTURA")),
)

#v(16pt)

// Info block: metadata (left) and customer (right)
#{
  let info-row(label, value) = {
    grid(
      columns: (55%, 1fr),
      column-gutter: 8pt,
      text(weight: "bold", size: 8.5pt, label),
      text(size: 8.5pt, value),
    )
    v(1.5pt)
  }

  grid(
    columns: (1fr, 1fr),
    column-gutter: 20pt,
    {
      info-row("Fakturanummer:", invoice_number)
      info-row("Fakturadatum:", invoice_date)
      info-row("Betalningsvillkor:", str(payment_terms_days) + " dagar")
      info-row("Förfallodag:", due_date)
      info-row("Dröjsmålsränta:", fmt-percent(late_interest_rate_percent))
      info-row("Köparens inköpsordernummer:", buyer_po_number)
      info-row("Köparens referens:", buyer_reference)
    },
    {
      set text(size: 8.5pt)
      text(weight: "bold", customer_name)
      linebreak()
      customer_contact_name
      linebreak()
      customer_address_line1
      linebreak()
      customer_postal_code + "\u{00A0}" + customer_city
      v(8pt)
      "Organisationsnummer " + customer_org_number
      linebreak()
      "Momsregistreringsnummer " + customer_vat_number
    },
  )
}

#v(16pt)

// Line items table
#{
  set text(size: 8.5pt)

  let col-widths = (3fr, 62pt, 28pt, 28pt, 72pt, 46pt, 80pt)

  table(
    columns: col-widths,
    column-gutter: 0pt,
    inset: (x: 4pt, y: 5pt),
    stroke: (x: none, y: 0.5pt + luma(200)),
    align: (left, left, right, left, right, right, right),
    fill: (_, row) => if row == 0 { header-bg } else { none },

    // Header
    text(weight: "bold", fill: white, "Beskrivning"),
    text(weight: "bold", fill: white, "Datum"),
    text(weight: "bold", fill: white, "Antal"),
    text(weight: "bold", fill: white, "Enhet"),
    text(weight: "bold", fill: white, "À pris"),
    text(weight: "bold", fill: white, "Moms %"),
    text(weight: "bold", fill: white, "Belopp"),

    // Line items
    ..for item in line_items {
      (
        item.description,
        item.date,
        fmt-quantity(item.quantity),
        item.unit,
        fmt-currency(item.unit_price),
        fmt-percent(item.vat_rate_percent),
        fmt-currency(item.belopp),
      ) + item.sub_items.map(sub =>
        table.cell(colspan: 7, inset: (x: 4pt, y: 2pt), stroke: none,
          text(fill: luma(120), style: "italic", size: 7pt, h(12pt) + sub)
        )
      )
    },
  )
}

#v(12pt)

// Totals
#{
  set text(size: 8.5pt)
  align(right, {
    grid(
      columns: (auto, 90pt),
      column-gutter: 16pt,
      row-gutter: 4pt,
      align(right, "Totalt exkl moms"),
      align(right, fmt-currency(totalt_exkl_moms)),
      ..for vg in vat_groups {
        (
          align(right, "Moms " + fmt-percent(vg.rate)),
          align(right, fmt-currency(vg.amount)),
        )
      },
    )
    v(8pt)
    line(length: 200pt, stroke: 0.5pt + luma(180))
    v(4pt)
    grid(
      columns: (auto, 90pt),
      column-gutter: 16pt,
      align(right, text(weight: "bold", "Summa att betala")),
      align(right, text(weight: "bold", fmt-currency(summa_att_betala))),
    )
  })
}
