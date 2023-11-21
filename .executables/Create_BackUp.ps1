param(
    [Parameter(Mandatory=$True, Position=0)]
    [String[]] 
    $PathToFiles
)

foreach ($PathToFile in $PathToFiles) {

    $File = $PathToFile.split("\")[-1] 
    Write-Output $File

    $Dest = "C:\Temp\$File$(Get-Date -format "_yyyyMMdd-hhmmss")"

    Write-Output $Dest

    Copy-Item -Path $PathToFile -Destination $Dest -Recurse
}