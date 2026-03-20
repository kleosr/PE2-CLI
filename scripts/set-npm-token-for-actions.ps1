# Creates or updates Actions secret NPM_TOKEN on github.com/kleosr/PE2-CLI
# Requires: GitHub CLI (winget install GitHub.cli) and once: gh auth login -h github.com -s repo

$ErrorActionPreference = 'Stop'
$repo = 'kleosr/PE2-CLI'

if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host 'Install GitHub CLI, then re-run this script:'
    Write-Host '  winget install GitHub.cli'
    exit 1
}

gh auth status 2>$null | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host 'Run: gh auth login -h github.com -s repo'
    exit 1
}

$npmToken = Read-Host 'Paste npm granular token (publish access to @kleosr/pe2-cli)'
if ([string]::IsNullOrWhiteSpace($npmToken)) {
    exit 1
}

$npmToken | gh secret set NPM_TOKEN --repo $repo
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Write-Host "Secret NPM_TOKEN set for $repo"
