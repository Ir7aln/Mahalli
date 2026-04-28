param(
    [string]$EntityDir,
    [string]$Scope = "tenant" # tenant or system
)

if (-not $EntityDir) {
    throw "EntityDir is required."
}

$entityPath = Resolve-Path $EntityDir
$files = Get-ChildItem -LiteralPath $entityPath -Filter *.rs -File |
    Where-Object { $_.Name -notin @("lib.rs", "mod.rs", "prelude.rs") }

$oldImport = "use sea_orm::entity::prelude::*;"
$patchedImport = "use sea_orm::{entity::prelude::*, Set};"

foreach ($file in $files) {
    $fileName = $file.Name
    $content = Get-Content -LiteralPath $file.FullName -Raw
    $hasCreatedAt = $content.Contains("pub created_at:")
    $hasUpdatedAt = $content.Contains("pub updated_at:")
    $hasStatus = $content.Contains("pub status:")

    if (-not $content.Contains("use chrono::Utc;")) {
        if ($hasCreatedAt -and $content.Contains($patchedImport)) {
            $content = $content.Replace($patchedImport, "use chrono::Utc;`nuse sea_orm::{entity::prelude::*, Set};")
        } elseif ($hasCreatedAt -and $content.Contains($oldImport)) {
            $content = $content.Replace($oldImport, "use chrono::Utc;`nuse sea_orm::{entity::prelude::*, Set};")
        }
    }

    $marker = "impl ActiveModelBehavior for ActiveModel"
    $start = $content.IndexOf($marker)

    if ($start -lt 0) {
        continue
    }

    # Determine replacement based on file and scope
    $replacement = switch ($fileName) {
        "orders.rs" {
@"
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            created_at: Set(Utc::now().naive_utc()),
            status: Set("PENDING".to_string()),
            ..ActiveModelTrait::default()
        }
    }
}
"@
        }
        "invoices.rs" {
@"
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            created_at: Set(Utc::now().naive_utc()),
            status: Set("DRAFT".to_string()),
            ..ActiveModelTrait::default()
        }
    }
}
"@
        }
        "quotes.rs", "delivery_notes.rs" {
@"
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            created_at: Set(Utc::now().naive_utc()),
            status: Set("PENDING".to_string()),
            ..ActiveModelTrait::default()
        }
    }
}
"@
        }
        "seller_profile.rs" {
@"
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            default_currency: Set("MAD".to_string()),
            default_payment_terms_days: Set(30),
            ..ActiveModelTrait::default()
        }
    }
}
"@
        }
        "inventory_transactions.rs" {
@"
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            created_at: Set(Utc::now().naive_utc()),
            source_type: Set("INITIAL".to_string()),
            is_void: Set(false),
            ..ActiveModelTrait::default()
        }
    }
}
"@
        }
        default {
            if ($hasCreatedAt) {
@"
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            created_at: Set(Utc::now().naive_utc()),
            ..ActiveModelTrait::default()
        }
    }
}
"@
            } else {
@"
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            ..ActiveModelTrait::default()
        }
    }
}
"@
            }
        }
    }

    $updated = $content.Substring(0, $start) + $replacement
    Set-Content -LiteralPath $file.FullName -Value $updated -NoNewline
    Write-Host "Patched: $fileName"
}

Write-Host "Entity patching complete for $Scope scope."
