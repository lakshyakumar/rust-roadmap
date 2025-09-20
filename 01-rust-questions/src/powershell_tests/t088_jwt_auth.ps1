$token = Invoke-RestMethod -Uri "http://localhost:8080/login" -Method GET

Write-Host "Received Token:" $token


# Add "Bearer <token>" header
$headers = @{ "Authorization" = "Bearer $token" }

$response = Invoke-RestMethod -Uri "http://localhost:8080/protected" -Headers $headers -Method GET

Write-Host "Protected Response:" $response


$response = Invoke-RestMethod -Uri "http://localhost:8080/protected" -Method GET -ErrorAction SilentlyContinue

Write-Host "Unauthorized Response:" $response

$headers = @{ "Authorization" = "Bearer invalid.token.here" }

$response = Invoke-RestMethod -Uri "http://localhost:8080/protected" -Headers $headers -Method GET -ErrorAction SilentlyContinue

Write-Host "Invalid Token Response:" $response