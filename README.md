# Mahalli

<div>
   <a href="https://github.com/AbdelilahOu/Mahalli/releases"><img src="https://img.shields.io/github/release/AbdelilahOu/Mahalli.svg" alt="Latest Release"></a>
</div>

Mahalli is a desktop application for inventory and invoicing, it help you create and manage clients, products, quotes, commands and invoices.

## Video Showcase

Check out a quick video demonstration of Mahalli's features:

[Watch the Showcase Video](assets/showcase.mp4)

<div>
    <video src="assets/showcase.mp4" controls></video>
</div>

## Built for Moroccan B2B Workflow

Mahalli is specifically designed to streamline the business-to-business (B2B) workflow prevalent in Morocco. The typical process involves:

1.  **Quote Generation**: Businesses often start by generating a detailed quote for their clients.
2.  **Order Creation**: Once the quote is approved, it transitions into an official order.
3.  **Invoice Generation**: Upon payment or delivery, an invoice is created to finalize the transaction.

Mahalli provides comprehensive tools to manage each step of this process efficiently.

## Getting Started

Before you begin using Mahalli, ensure you have Node.js and npm (Node Package Manager) installed on your system. You can download and install them from the official Node.js website (https://nodejs.org/en/download/package-manager/current).

### Prerequisites for Running Tauri Apps

Tauri is the framework used to build Mahalli. Here are the additional requirements for running Tauri applications:

check : [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/#:~:text=Tauri%20heavily%20depends%20on%20WebView2,and%20version%20for%20your%20system.)

### Running Mahalli

1. Clone the Mahalli repository from GitHub.
2. Open a terminal and navigate to the root directory of the Mahalli project.
3. Install the project dependencies by running:

```Bash
npm install
```

4. Start the development server to run Mahalli in development mode:

```Bash
npm run tauri dev
```

This will launch Mahalli in your default desktop browser.

5. Building Mahalli
   To create a standalone desktop application for distribution, use the following command:

```Bash
npm run tauri build
```

This will generate an executable file in the target directory. The specific location and file name will depend on your operating system.

### Understanding the Makefile Commands

The project also includes a Makefile that defines various commands for managing the Tauri application. Here's a breakdown of some relevant commands:

- migrationsup: Runs database migrations to update the schema (use with caution in production).
- migrationslast: Reverts the most recent database migration.
- migrationsdown: Drops all tables and data, essentially resetting the database (use with extreme caution).
- entity: Generates the sea_orm entities from your database models.
- dev: Starts the development server.
- build: Builds a standalone desktop application.
- check: Runs static code checks on the Rust code.
- lint: Lints the JavaScript code for potential errors and style issues.
- migration: Generates a new database migration file:

```bash
make migration name=migration_name
```

- Note: These Makefile commands are typically used during development and may not be directly relevant for casual users of Mahalli.

We recommend referring to the Tauri documentation (https://tauri.app/) for a more comprehensive understanding of these commands and Tauri development in general.
