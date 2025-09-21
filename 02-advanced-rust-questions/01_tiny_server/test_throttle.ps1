# test_throttle.ps1
# Purpose: Test per-IP throttling on tiny_server (token bucket)
# Author: Generated for tiny_server example
# Usage: powershell -ExecutionPolicy Bypass -File .\test_throttle.ps1

# Stop execution on errors
$ErrorActionPreference = "Stop"

# Function to send a GET request to the server and handle throttling response
function Test-Request($i) {
    try {
        # Send GET request to server
        $response = Invoke-WebRequest -Uri "http://localhost:7878/" -Method Get -ErrorAction Stop

        if ($response.StatusCode -eq 200) {
            Write-Host "✅ Request $i PASSED (200 OK)"
        } else {
            Write-Host "❌ Request $i FAILED. Unexpected status: $($response.StatusCode)"
        }
    }
    catch {
        # Handle throttled requests (429)
        $statusCode = $_.Exception.Response.StatusCode.value__
        if ($statusCode -eq 429) {
            Write-Host "🚦 Request $i THROTTLED (429 Too Many Requests)"
        } else {
            Write-Host "❌ Request $i FAILED. Error: $($_.Exception.Message)"
        }
    }
}

Write-Host "Starting throttling test (burst = 5, refill = 1/sec)..."
Write-Host "------------------------------------------------------"

# Step 1: Burst test — 10 quick requests
for ($i = 1; $i -le 10; $i++) {
    Test-Request $i
}

# Step 2: Wait to allow token bucket to refill
Write-Host "`nNow waiting 6 seconds to allow bucket refill..."
Start-Sleep -Seconds 6

# Step 3: Send 5 more requests after refill
for ($i = 11; $i -le 15; $i++) {
    Test-Request $i
}

Write-Host "`n🎉 Throttling test complete!"