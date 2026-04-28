# B2B Moroccan Workflow Audit

Date: 2026-04-28

Scope: Full-app product audit against the target workflow in `docs/future-features.md`.

## Target Workflow

- quote / devis
- customer order / bon de commande
- delivery note / bon de livraison
- invoice / facture
- payment tracking
- credit note / avoir for finalized invoice corrections

## Findings

### High

1. Partial delivery is not implemented (full-delivery only)
- Current delivery-note creation copies all order items and quantities with no partial quantity handling or residual tracking.
- Evidence: `src-tauri/crates/tenant-service/src/delivery_notes/service.rs` (`create_delivery_note_from_order`)

2. Delivery note and credit note PDF generation are missing
- PDF generator currently supports only `order | invoice | quote`.
- No dedicated delivery-note or credit-note document type/print flow.
- Evidence:
  - `app/composables/usePdfGenerator.ts` (`type DocType = "order" | "invoice" | "quote"`)
  - `app/pages/credit-notes/[id].vue` (no print/export action)

3. Compliance-grade invoice metadata is incomplete
- Invoice model lacks explicit due-date and fiscal/year sequencing fields required by the stated compliance direction.
- No clear audit-trail record model in this flow.
- Evidence: `src-tauri/crates/tenant-entity/src/invoices.rs`

### Medium

4. Deletion rules may weaken B2B traceability
- Orders and quotes are soft-deletable, but there are no explicit guardrails here for downstream-linked records in the chain.
- Evidence:
  - `src-tauri/crates/tenant-service/src/orders/service.rs` (`delete_order`)
  - `src-tauri/crates/tenant-service/src/quotes/service.rs` (`delete_quote`)

## What Is Already Aligned

- Core chain exists end-to-end in backend commands/services:
  - quote -> order
  - order -> delivery note
  - delivery note -> invoice
  - finalized invoice -> credit note
- Status transitions and finalized-invoice protections are present.
- Moroccan legal identity fields and seller profile basics are present (ICE, IF, RC, patente, seller profile defaults).

## Conclusion

The app has a strong B2B foundation but has not fully reached the intended Moroccan B2B end-state yet.

Primary remaining gaps:
- Partial delivery support
- Delivery-note and credit-note PDF flows
- Compliance-grade invoice metadata and audit trail

