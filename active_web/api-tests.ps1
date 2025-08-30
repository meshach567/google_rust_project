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
    ContentType = 'application/json'
}
Invoke-RestMethod @Params

$Params = @{
    Uri = 'http://localhost:6570/vehicle?manufacturer=Tesla&model=Model Y&year=2024'
    Method = 'Post'
}
Invoke-RestMethod @Params

# Uses query string parameters instead of body payload

$Params = @{
    Uri = 'http://localhost:6570/vehicle?manufacturer=Tesla&model=Model Y&year=2024'
    Method = 'Post'
}
Invoke-RestMethod @Params