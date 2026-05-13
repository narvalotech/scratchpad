# Install Dell Optimizer first
# https://www.dell.com/support/home/en-ca/drivers/driversdetails?driverid=c28mr

# --- Logging ---
$LogFile = "$env:TEMP\ThermalControl.log"
if (Test-Path $LogFile) { Remove-Item $LogFile -Force }
Start-Transcript -Path $LogFile -Append

$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
Write-Host "Running as admin: $isAdmin"

# --- Configuration ---
$Threshold = 25              # CPU % Threshold (Aggressive)
$TriggerIntervals = 1        # Number of checks to trigger Ultra
$CheckInterval = 5           # Seconds between checks
$CooldownSeconds = 120       # 2 minutes of inactivity before Quiet
$CCTKPath = "C:\Program Files\Dell\DellOptimizer\do-cli.exe"

# Variables to track state
$HighLoadCount = 0
$InactivityDuration = 0
$CurrentMode = "Quiet"

Write-Host "Monitoring Core Ultra 7 165H - Aggressive Mode Active" -ForegroundColor Cyan
Write-Host "Threshold: $Threshold% | Cooldown: $CooldownSeconds s" -ForegroundColor Gray

& $CCTKPath /configure -name=SystemPowerConfiguration.ThermalMode -value=Quiet
$CurrentMode = "Quiet"

while ($true) {
    # Get average CPU utilization over 1 second
    $CPUPerf = Get-Counter '\Processor(_Total)\% Processor Time' -SampleInterval 1
    $CPULoad = [Math]::Round($CPUPerf.CounterSamples.CookedValue)

    if ($CPULoad -ge $Threshold) {
        # LOAD IS HIGH
        $HighLoadCount++
        $InactivityDuration = 0 # Reset cooldown timer because we are busy
        Write-Host "Load: $CPULoad% (High - Check $HighLoadCount/$TriggerIntervals)" -ForegroundColor Yellow
    } else {
        # LOAD IS LOW
        $HighLoadCount = 0 # Reset trigger count
        $InactivityDuration += $CheckInterval
        Write-Host "Load: $CPULoad% (Low - Inactivity: $InactivityDuration/$CooldownSeconds s)" -ForegroundColor DarkGray
    }

    # TRIGGER: Switch to Ultra Performance
    if ($HighLoadCount -ge $TriggerIntervals -and $CurrentMode -ne "UltraPerformance") {
        Write-Host ">>> TRIGGER: Aggressive Load detected. Switching to ULTRA." -ForegroundColor Red
        & $CCTKPath /configure -name=SystemPowerConfiguration.ThermalMode -value=Ultra
        $CurrentMode = "UltraPerformance"
    }

    # TRIGGER: Switch back to Quiet (Only after 2 solid minutes of low load)
    if ($InactivityDuration -ge $CooldownSeconds -and $CurrentMode -ne "Quiet") {
        Write-Host "<<< TRIGGER: 2min Inactivity reached. Switching to QUIET." -ForegroundColor Green
        & $CCTKPath /configure -name=SystemPowerConfiguration.ThermalMode -value=Quiet
        $CurrentMode = "Quiet"
    }

    Start-Sleep -Seconds $CheckInterval
}
