TENANT_MIGRATION_DIR=src-tauri/crates/tenant-migration
SYSTEM_MIGRATION_DIR=src-tauri/crates/system-migration
TENANT_ENTITY_OUT=src-tauri/crates/tenant-entity/src
SYSTEM_ENTITY_OUT=src-tauri/crates/system-entity/src
TENANT_DB_URL?=sqlite://mahalli.sqlite?mode=rwc
SYSTEM_DB_URL?=sqlite://catalog.sqlite?mode=rwc

tenant-migrationsup:
	@set "DATABASE_URL=$(TENANT_DB_URL)" && cargo run --manifest-path $(TENANT_MIGRATION_DIR)/Cargo.toml -- up

tenant-migrationslast:
	@set "DATABASE_URL=$(TENANT_DB_URL)" && cargo run --manifest-path $(TENANT_MIGRATION_DIR)/Cargo.toml -- down

tenant-migrationsdown:
	@set "DATABASE_URL=$(TENANT_DB_URL)" && cargo run --manifest-path $(TENANT_MIGRATION_DIR)/Cargo.toml -- fresh

system-migrationsup:
	@set "DATABASE_URL=$(SYSTEM_DB_URL)" && cargo run --manifest-path $(SYSTEM_MIGRATION_DIR)/Cargo.toml -- up

system-migrationslast:
	@set "DATABASE_URL=$(SYSTEM_DB_URL)" && cargo run --manifest-path $(SYSTEM_MIGRATION_DIR)/Cargo.toml -- down

system-migrationsdown:
	@set "DATABASE_URL=$(SYSTEM_DB_URL)" && cargo run --manifest-path $(SYSTEM_MIGRATION_DIR)/Cargo.toml -- fresh

tenant-entity:
	@sea-orm-cli generate entity --lib -u $(TENANT_DB_URL) -o $(TENANT_ENTITY_OUT)
	@powershell -ExecutionPolicy Bypass -File scripts/patch-entities.ps1 -EntityDir "$(TENANT_ENTITY_OUT)"

system-entity:
	@sea-orm-cli generate entity --lib -u $(SYSTEM_DB_URL) -o $(SYSTEM_ENTITY_OUT)
	@powershell -ExecutionPolicy Bypass -File scripts/patch-entities.ps1 -EntityDir "$(SYSTEM_ENTITY_OUT)"

migrationsup: tenant-migrationsup
migrationslast: tenant-migrationslast
migrationsdown: tenant-migrationsdown
entity: tenant-entity

dev:
	@bun run tauri dev

build: 
	@bun run tauri build --debug

check:
	@cd src-tauri && cargo check

lint:
	@cd src-tauri && cargo fmt --all
	@bun run lint:fix 
	
update-v:
	@cd scripts && update-version.sh $(v)

migration: 
	@cargo run --manifest-path $(TENANT_MIGRATION_DIR)/Cargo.toml -- generate $(name)

system-migration:
	@cargo run --manifest-path $(SYSTEM_MIGRATION_DIR)/Cargo.toml -- generate $(name)
