$response = Invoke-RestMethod -Uri "http://localhost:3000/health" -Method Get
if ($response.status -eq "OK") {
    Write-Host "GET API Test passed: Health endpoint returned 'ok'"
} else {
    Write-Host "Test failed: Unexpected response '$response'"
    exit 1
}


$item = @{ id = 1; name = "TestItem" }
$response = Invoke-RestMethod -Uri "http://localhost:3000/items" -Method Post -Body ($item | ConvertTo-Json) -ContentType "application/json"

if ($response.id -eq $item.id -and $response.name -eq $item.name) {
    Write-Host "POST API Test passed: Item created successfully"
} else {
    Write-Host "Test failed: Unexpected response '$($response | ConvertTo-Json)'"
    exit 1
}

exit 0;