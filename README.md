# Mahalli

<div>
   <a href="https://github.com/AbdelilahOu/Mahalli/releases"><img src="https://img.shields.io/github/release/AbdelilahOu/Mahalli.svg" alt="Latest Release"></a>
</div>

Mahalli is a desktop application for inventory and invoicing. It helps you manage clients, products, quotes, orders, and invoices — across multiple workspaces.

## Video Showcase

Check out a quick video demonstration of Mahalli's features:

[Watch the Showcase Video](assets/showcase.mp4)

<div>
    <video src="assets/showcase.mp4" controls></video>
</div>

## Built for Moroccan B2B Workflow

Mahalli is specifically designed around the B2B document chain used by Moroccan businesses. The full flow is:

1. **Quote / Devis** — The seller generates a quote and sends it to the client for approval.
2. **Customer Order / Bon de commande** — Once the quote is approved, it converts into a formal purchase order.
3. **Delivery Note / Bon de livraison** — When goods are dispatched, a delivery note is issued against the order to confirm what was shipped and when.
4. **Invoice / Facture** — The final billing document, generated after delivery. Invoices are immutable once finalized.
5. **Payment Tracking** — Partial and full payments are recorded against invoices. Outstanding balances are computed per client.
6. **Credit Note / Avoir** — When a finalized invoice needs a correction (returned goods, pricing errors), a credit note is issued against it rather than modifying or deleting the original.

Each document type carries the Moroccan legal identity fields required on printed paperwork: **ICE**, **IF** (identifiant fiscal), **RC** (registre de commerce), and **Patente / TP**, for both clients and the seller's own profile.

Mahalli provides comprehensive tools to manage each step of this process efficiently.

## Architecture

Mahalli uses a two-layer SQLite database architecture:

### System Database (`catalog.sqlite`)

A permanent catalog database that runs at all times. It tracks all registered tenant databases — their names, slugs, file paths, and which one is currently active.

### Tenant Databases

Each tenant database is an independent SQLite file that holds all business data (clients, products, orders, quotes, invoices, inventory) for a given workspace. The active tenant connection is hot-swappable at runtime — users can create new workspaces, clone existing ones, and switch between them without restarting the app.

### Crate Structure

```
src-tauri/
├── src/                        # Main Tauri app
│   ├── commands/               # Tauri commands exposed to the frontend
│   │   └── databases.rs        # create, switch, list databases
│   ├── db/                     # DB manager, path resolution, system setup
│   └── jobs/                   # Background jobs (image optimizer)
└── crates/
    ├── system-entity/          # SeaORM entities for the system/catalog DB
    ├── system-migration/       # Migrations for the system DB
    ├── system-service/         # Queries/mutations for the system DB
    ├── tenant-entity/          # SeaORM entities for tenant databases
    ├── tenant-migration/       # Migrations for tenant databases
    └── tenant-service/         # Queries/mutations for tenant databases
```

## Getting Started

Before you begin, ensure you have [Bun](https://bun.sh) and the [Tauri prerequisites](https://tauri.app/start/prerequisites/) installed.
For SeaORM entity and migration generation, install the matching v2 CLI:

```bash
cargo install sea-orm-cli@^2.0.0-rc.34
```

### Running in Development

1. Clone the repository and navigate to the project root.
2. Install frontend dependencies:

```bash
bun install
```

3. Start the development server:

```bash
bun run tauri dev
```

### Building for Production

```bash
bun run tauri build
```

## Makefile Commands

### Development

| Command      | Description                             |
| ------------ | --------------------------------------- |
| `make dev`   | Start the development server            |
| `make build` | Build a debug desktop executable        |
| `make check` | Run `cargo check` on the Rust code      |
| `make lint`  | Format Rust code and lint frontend code |

### Tenant Database (business data)

| Command                      | Description                                   |
| ---------------------------- | --------------------------------------------- |
| `make tenant-migrationsup`   | Run all pending tenant migrations             |
| `make tenant-migrationslast` | Revert the last tenant migration              |
| `make tenant-migrationsdown` | Drop and reset the tenant database            |
| `make tenant-entity`         | Regenerate SeaORM entities from the tenant DB |
| `make migration name=<name>` | Generate a new tenant migration file          |

Shorthands `make migrationsup`, `make migrationslast`, `make migrationsdown`, and `make entity` all target the tenant database.

### System Database (catalog)

| Command                             | Description                                   |
| ----------------------------------- | --------------------------------------------- |
| `make system-migrationsup`          | Run all pending system migrations             |
| `make system-migrationslast`        | Revert the last system migration              |
| `make system-migrationsdown`        | Drop and reset the system database            |
| `make system-entity`                | Regenerate SeaORM entities from the system DB |
| `make system-migration name=<name>` | Generate a new system migration file          |

### Version

```bash
make update-v v=1.2.3
```
