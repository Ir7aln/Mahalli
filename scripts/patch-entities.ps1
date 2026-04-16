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
            id: sea_orm::ActiveValue::Set(ulid::Ulid::new().to_string()),
            ..Default::default()
        }
    }
}
"@

foreach ($file in $files) {
    $content = Get-Content -LiteralPath $file.FullName -Raw
    $marker = "impl ActiveModelBehavior for ActiveModel"
    $start = $content.IndexOf($marker)

    if ($start -lt 0) {
        continue
    }

    $updated = $content.Substring(0, $start) + $replacement
    Set-Content -LiteralPath $file.FullName -Value $updated -NoNewline
}
