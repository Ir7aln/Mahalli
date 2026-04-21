param(
    [string]$EntityDir
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
    $content = Get-Content -LiteralPath $file.FullName -Raw
    $hasCreatedAt = $content.Contains("pub created_at:")

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

    $replacement = if ($hasCreatedAt) {
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

    $updated = $content.Substring(0, $start) + $replacement
    Set-Content -LiteralPath $file.FullName -Value $updated -NoNewline
}
