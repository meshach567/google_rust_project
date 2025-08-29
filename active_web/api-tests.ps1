# Get vehicle basic example
Invoke-RestMethod -Uri http://localhost:6570/vehicle -Method Get

# post new vehicle
Invoke-RestMethod -Uri http://localhost:6570/vehicle -Method Post

$Params = @{
    Uri = 'https://localhost:6570/vehicle'
    Method = 'Post'
    Body = @{
        manufacturer = 'Tesla'
        model = 'Model Y'
        year = 2024
    } | ConvertTo-Json
}
Invoke-RestMethod @Params