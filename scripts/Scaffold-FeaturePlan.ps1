param(
    [Parameter(Mandatory = $true)]
    [string]$FeatureSlug,

    [string]$FeatureName,

    [string]$BranchName,

    [ValidateSet('game', 'engine', 'editor', 'multi-area', '<Feature Area>')]
    [string]$FeatureArea = '<Feature Area>',

    [ValidateSet('game', 'engine', 'editor', '<Primary Area>')]
    [string]$PrimaryArea = '<Primary Area>'
)

$ErrorActionPreference = 'Stop'

if (-not $FeatureName) {
    $featureNameParts = ($FeatureSlug -split '[-_]') | Where-Object { $_ -ne '' } | ForEach-Object {
        if ($_.Length -gt 1) {
            $_.Substring(0,1).ToUpper() + $_.Substring(1)
        } else {
            $_.ToUpper()
        }
    }
    $FeatureName = $featureNameParts -join ' '
}

if (-not $BranchName) {
    $BranchName = "feature/$FeatureSlug"
}

if ($FeatureSlug -notmatch '^[a-z0-9]+(-[a-z0-9]+)*$') {
    throw "FeatureSlug must be lowercase kebab-case using only letters, numbers, and hyphens: $FeatureSlug"
}

if ($BranchName -notmatch '^(feature|hotfix)/[a-z0-9]+(-[a-z0-9]+)*$') {
    throw "BranchName must match feature/<work-being-done> or hotfix/<work-being-done>: $BranchName"
}

if ($FeatureArea -eq '<Feature Area>') {
    throw "FeatureArea is required. Use one of: game, engine, editor, multi-area. Example: scripts/scaffold-feature-plan.cmd $FeatureSlug `"$FeatureName`" $BranchName engine engine"
}

if ($PrimaryArea -eq '<Primary Area>') {
    throw "PrimaryArea is required. Use one of: game, engine, editor. Example: scripts/scaffold-feature-plan.cmd $FeatureSlug `"$FeatureName`" $BranchName $FeatureArea engine"
}

if (($FeatureArea -ne 'multi-area') -and ($PrimaryArea -ne $FeatureArea)) {
    throw "PrimaryArea must match FeatureArea for single-area features. FeatureArea: $FeatureArea; PrimaryArea: $PrimaryArea"
}

$root = Split-Path -Parent $PSScriptRoot
$templatesDir = Join-Path $root 'docs/plans/_templates'
$targetDir = Join-Path $root "docs/plans/$FeatureSlug"
$planTemplate = Join-Path $templatesDir 'plan.template.md'
$trackerTemplate = Join-Path $templatesDir 'tracker.template.md'
$planPath = Join-Path $targetDir 'plan.md'
$trackerPath = Join-Path $targetDir 'tracker.md'
$date = Get-Date -Format 'yyyy-MM-dd'

foreach ($required in @($planTemplate, $trackerTemplate)) {
    if (-not (Test-Path $required)) {
        throw "Required template not found: $required"
    }
}

New-Item -ItemType Directory -Path $targetDir -Force | Out-Null

function Apply-Template {
    param(
        [string]$TemplatePath,
        [string]$DestinationPath,
        [string]$StatusLabel
    )

    if (Test-Path $DestinationPath) {
        Write-Host ("{0} already exists: {1}" -f $StatusLabel, $DestinationPath)
        return
    }

    $content = Get-Content -Path $TemplatePath -Raw
    $content = $content.Replace('<Feature Name>', $FeatureName)
    $content = $content.Replace('<new-feature>', $FeatureSlug)
    $content = $content.Replace('feature/<work-being-done>', $BranchName)
    $content = $content.Replace('<Feature Area>', $FeatureArea)
    $content = $content.Replace('<Primary Area>', $PrimaryArea)
    $content = $content.Replace('<YYYY-MM-DD>', $date)

    Set-Content -Path $DestinationPath -Value $content -NoNewline
    Write-Host ("Created {0}: {1}" -f $StatusLabel, $DestinationPath)
}

Apply-Template -TemplatePath $planTemplate -DestinationPath $planPath -StatusLabel 'plan'
Apply-Template -TemplatePath $trackerTemplate -DestinationPath $trackerPath -StatusLabel 'tracker'

Write-Host "Feature planning scaffold ready for '$FeatureSlug'."
Write-Host "Branch: $BranchName"
Write-Host "Feature area: $FeatureArea"
Write-Host "Primary area: $PrimaryArea"
Write-Host 'Reminder: Use the feature-plan-docs skill before implementation and keep the tracker updated with the feature-tracker-update skill during implementation.'
