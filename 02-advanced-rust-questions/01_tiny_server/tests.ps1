# test_router.ps1
# Script to test router-based tiny_server API
# Usage: powershell -ExecutionPolicy Bypass -File .\test_router.ps1

$ErrorActionPreference = "Stop"

function Test-Endpoint($description, $uri, $method, $body = $null, $headers = $null, $expectedContains = $null) {
    try {
        if ($null -ne $headers) {
            $response = Invoke-RestMethod -Uri $uri -Method $method -Body $body -Headers $headers -ContentType "application/json"
        } elseif ($null -ne $body) {
            $response = Invoke-RestMethod -Uri $uri -Method $method -Body $body -ContentType "application/json"
        } else {
            $response = Invoke-RestMethod -Uri $uri -Method $method
        }

        $text = $response | Out-String
        if ($expectedContains -and ($text -notmatch $expectedContains)) {
            Write-Host "❌ $description FAILED. Response: $text"
            exit 1
        }
        Write-Host "✅ $description PASSED"
    }
    catch {
        Write-Host "❌ $description FAILED. Error: $($_.Exception.Message)"
        exit 1
    }
}

# --- TESTS BEGIN ---
Write-Host "Running router tests..."

# 1. GET /
Test-Endpoint "GET / root" "http://localhost:7878/" "GET" $null $null "Welcome"

# 2. GET /time
Test-Endpoint "GET /time" "http://localhost:7878/time" "GET" $null $null "time"

# 3. POST /echo
$echoBody = "HelloRouter"
$response = Invoke-RestMethod -Uri "http://localhost:7878/echo" -Method Post -Body $echoBody -ContentType "text/plain"
if ($response -eq $echoBody) {
    Write-Host "✅ POST /echo PASSED"
} else {
    Write-Host "❌ POST /echo FAILED. Response: $response"
    exit 1
}

# 4. POST /json
$jsonBody = @{ message = "Hello JSON" } | ConvertTo-Json
$headers = @{ "Content-Type" = "application/json" }
$response = Invoke-RestMethod -Uri "http://localhost:7878/json" -Method Post -Body $jsonBody -Headers $headers -ContentType "application/json"
if ($response.message -eq "Hello JSON") {
    Write-Host "✅ POST /json PASSED"
} else {
    Write-Host "❌ POST /json FAILED. Response: $($response | ConvertTo-Json)"
    exit 1
}

# 5. GET /users/:id
$response = Invoke-RestMethod -Uri "http://localhost:7878/users/42" -Method Get -ErrorAction Stop
if ($response -match "User ID requested: 42") {
    Write-Host "✅ GET /users/:id PASSED"
} else {
    Write-Host "❌ GET /users/:id FAILED. Response: $response"
    exit 1
}

# 6. Invalid route
try {
    Invoke-RestMethod -Uri "http://localhost:7878/doesnotexist" -Method Get -ErrorAction Stop
    Write-Host "❌ GET /doesnotexist FAILED. Expected 404 but got success."
    exit 1
}
catch {
    if ($_.Exception.Response.StatusCode.value__ -eq 404) {
        Write-Host "✅ GET /doesnotexist PASSED (404 returned)"
    } else {
        Write-Host "❌ GET /doesnotexist FAILED. Error: $($_.Exception.Message)"
        exit 1
    }
}

Write-Host "`n🎉 All router endpoint tests passed!"
exit 0