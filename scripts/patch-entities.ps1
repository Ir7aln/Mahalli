param(
    [string]$EntityDir
)

if (-not $EntityDir) {
    throw "EntityDir is required."
}

$entityPath = Resolve-Path $EntityDir
$files = Get-ChildItem -LiteralPath $entityPath -Filter *.rs -File |
    Where-Object { $_.Name -notin @("lib.rs", "mod.rs", "prelude.rs") }

$replacement = @"
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            ..ActiveModelTrait::default()
        }
    }
}
"@

$oldImport = "use sea_orm::entity::prelude::*;"
$newImport = "use sea_orm::{Set, entity::prelude::*};"

foreach ($file in $files) {
    $content = Get-Content -LiteralPath $file.FullName -Raw
    if ($content.Contains($oldImport)) {
        $content = $content.Replace($oldImport, $newImport)
    }

    $marker = "impl ActiveModelBehavior for ActiveModel"
    $start = $content.IndexOf($marker)

    if ($start -lt 0) {
        continue
    }

    $updated = $content.Substring(0, $start) + $replacement
    Set-Content -LiteralPath $file.FullName -Value $updated -NoNewline
}
