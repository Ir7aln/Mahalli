# Future Features

## Product Direction

Mahalli should now prioritize **B2C for the Moroccan market**.

The current app already fits a simple B2B flow:

- quote
- order
- invoice
- inventory
- partial payments

But the next product phase should focus on **retail / counter sales / fast checkout** rather than deeper B2B-only workflow.

## B2C Priority

The main target should be:

- small shops
- local commerce
- quick over-the-counter sales
- simple ticket / receipt flow
- inventory linked to direct sales

The app should feel closer to a lightweight **caisse / POS + stock + customer tracking** tool for Morocco.

## Missing Features

### 1. POS / Caisse mode

This is the biggest gap for B2C.

Needed:

- fast sale screen
- add products quickly
- barcode-friendly flow
- cart / basket flow
- direct checkout without quote or order first
- receipt / ticket printing
- reopen or reprint receipt

### 2. Payment methods

Needed:

- cash
- card
- bank transfer
- mixed payment
- partial payment at checkout
- payment status per sale

### 3. Receipt / ticket flow

For B2C, invoice should not be the only output.

Needed:

- ticket de caisse
- convert ticket to invoice when customer asks
- customer-facing receipt details
- clean print layout for retail

### 4. Customer identity split

Current client model is too B2B-shaped for future compliance and too generic for mixed use.

Needed:

- simple walk-in customer sale
- optional named customer
- optional pro customer
- legal fields only when needed for pro invoice

Suggested pro fields:

- ICE
- IF
- RC
- patente

### 5. Seller legal profile

The app also needs a real business profile for printed documents.

Needed:

- shop / business legal name
- address
- phone
- ICE
- IF
- RC
- patente
- logo

### 6. Credit note / avoir

Even with B2C focus, this is still important.

Needed:

- refund flow
- return flow
- partial cancellation
- avoir / credit note document

### 7. Supplier purchasing flow

This is not the main B2C focus, but still useful for shops.

Needed later:

- purchase order
- goods receipt
- supplier invoice
- stock entry from supplier flow

### 8. Cash register operations

Needed for real B2C use:

- opening cash
- closing cash
- cash in / cash out
- daily summary
- cashier shift summary

### 9. Multi-store / multi-register support

Useful later if the product grows.

Needed later:

- multiple points of sale
- multiple registers
- stock by location
- transfers between locations

### 10. E-invoicing readiness

This is not the first B2C build priority, but it should remain on the roadmap.

Needed later:

- structured invoice export
- immutable document trail
- DGI-ready invoice metadata
- future electronic invoicing integration

## Recommended Build Order

1. POS / caisse mode
2. payment methods
3. receipt / ticket printing
4. seller legal profile
5. walk-in customer + optional pro customer fields
6. refund / avoir flow
7. supplier purchasing flow
8. cash opening / closing
9. multi-store support
10. e-invoicing readiness

## Notes

- Keep the current quote -> order -> invoice flow for businesses that need it.
- Do not force B2C sales through quote creation first.
- The B2C flow should be faster, simpler, and optimized for daily sales volume.
