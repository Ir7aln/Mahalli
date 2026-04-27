# Future Features

## Product Direction

Mahalli should prioritize a complete **B2B workflow for Moroccan businesses**.

The product should focus on company-to-company document flow, stock traceability,
payment follow-up, and invoice compliance rather than retail POS/caisse features.

## Target Workflow

The core B2B flow should be:

- quote / devis
- customer order / bon de commande
- delivery note / bon de livraison
- invoice / facture
- payment tracking
- credit note / avoir when a finalized invoice needs correction

## Missing Features

### 1. Legal identity fields

Clients and the seller profile need Moroccan business identifiers.

Needed:

- ICE
- IF
- RC
- patente / TP
- legal name
- address
- city
- tax settings

### 2. Seller legal profile

Printed documents need a single source of truth for the issuing business.

Needed:

- legal business name
- commercial name
- address
- phone
- email
- ICE
- IF
- RC
- patente / TP
- logo
- default payment terms
- default invoice footer

### 3. Delivery note / bon de livraison

This is the main workflow gap.

Needed:

- create delivery note from order
- partial delivery support
- delivery note PDF
- delivery status on orders
- link delivery notes to invoices

### 4. Credit note / avoir

B2B invoice corrections should not rely on deleting finalized invoices.

Needed:

- full or partial credit note
- returned quantities
- corrected totals
- credit note PDF
- link credit notes to original invoices

### 5. Invoice compliance

Invoices should be stable, traceable, and ready for stricter requirements.

Needed:

- fiscal year numbering
- immutable finalized invoices
- due date
- payment terms
- VAT summary lines
- legal footer
- audit trail for document changes

### 6. E-invoicing readiness

The app should prepare for Moroccan DGI electronic invoicing without requiring
full integration immediately.

Needed:

- structured invoice export
- immutable invoice metadata
- DGI-ready identifiers
- future signed submission payload support
- validation-ready tax totals

## Recommended Build Order

1. Legal fields for clients and seller profile
2. Delivery note / bon de livraison
3. Quote -> order -> delivery note -> invoice conversion chain
4. Credit note / avoir
5. Compliance-grade invoice numbering and audit trail
6. E-invoicing readiness

## Out Of Scope For Now

- POS / caisse mode
- ticket de caisse
- walk-in customer checkout
- cash register sessions
- B2C receipt flow
