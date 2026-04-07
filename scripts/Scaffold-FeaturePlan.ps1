param(
    [Parameter(Mandatory = $true)]
    [string]$FeatureSlug,

    [string]$FeatureName,

    [string]$BranchName
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
    $content = $content.Replace('<YYYY-MM-DD>', $date)

    Set-Content -Path $DestinationPath -Value $content -NoNewline
    Write-Host ("Created {0}: {1}" -f $StatusLabel, $DestinationPath)
}

Apply-Template -TemplatePath $planTemplate -DestinationPath $planPath -StatusLabel 'plan'
Apply-Template -TemplatePath $trackerTemplate -DestinationPath $trackerPath -StatusLabel 'tracker'

Write-Host "Feature planning scaffold ready for '$FeatureSlug'."
Write-Host "Branch: $BranchName"
Write-Host 'Reminder: Use the feature-plan-docs skill before implementation and keep the tracker updated with the feature-tracker-update skill during implementation.'
