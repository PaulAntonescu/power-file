param(
    [Parameter(Mandatory=$True, Position=0)]
    [String[]] 
    $PathToFiles,

    [Parameter(Mandatory=$True, Position=1)]
    [System.String]
    $Grep
)

foreach ($PathToFile in $PathToFiles) {
    Select-String -Path $PathToFile -Pattern $Grep
    Write-Host ""
}
$dump = Read-Host "Enter To Continue"