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

Mahalli is specifically designed to streamline the business-to-business (B2B) workflow prevalent in Morocco. The typical process involves:

1. **Quote Generation**: Businesses often start by generating a detailed quote for their clients.
2. **Order Creation**: Once the quote is approved, it transitions into an official order.
3. **Invoice Generation**: Upon payment or delivery, an invoice is created to finalize the transaction.

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
