# Power-File
Use SendTo to run PowerShell on Selected Files

# How to use
High-light Folder(s)/Files(s) you want to run the powershell command on.

![image](https://github.com/PaulAntonescu/power-file/assets/74125937/a9e5f522-24e7-4583-8e3a-306c1f665a2f)

Right Click and Run power_file.exe

![image](https://github.com/PaulAntonescu/power-file/assets/74125937/2f7a9810-07c4-4a90-b3c4-5664af25aa97)

## Interface
Enter index of powershell file you want to run.

![image](https://github.com/PaulAntonescu/power-file/assets/74125937/36dc02ca-bf29-4e5a-ad0c-52a8ea1ab6f2)

It will open a powershell window and run the selected script

![image](https://github.com/PaulAntonescu/power-file/assets/74125937/3048d53c-e11b-4ce3-8a3c-f531330d750d)

# Set up
1. After building binary power_file.exe add to ```"C:\Users\%USERNAME%\AppData\Roaming\Microsoft\SendTo"```
2. Create a folder ```".executables"``` to ```"C:\Windows\PowerFile\.executables\"```
3. Add your PowerShell Scripts to ```".executables"```


# Adding PowerShell Scripts
1. powershell input interface prameter
```
param(
	[Parameter(Mandatory=$True, Position=0)]
	[String[]] 
	$PathToFiles
)
```
2. If you want powershell to not exit on completion get user input
```
$dump = Read-Host "Enter To Continue"
```
