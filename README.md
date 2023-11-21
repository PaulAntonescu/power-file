# Power-File
Use SendTo to run PowerShell on Selected Files

# How to use
High-light Folder(s)/Files(s) you want to run the powershell command on.

![image](https://github.com/PaulAntonescu/power-file/assets/74125937/475bca74-1409-4142-996c-59bf78f1320e)

Right Click and Click power_file.exe

![image](https://github.com/PaulAntonescu/power-file/assets/74125937/03f9cdbd-2cb9-461b-91c7-5455a664c0cb)


## Interface
Enter index of powershell file you want to run.

![image](https://github.com/PaulAntonescu/power-file/assets/74125937/5fdd5a89-946d-4520-a6e6-4a7739510938)

It will open a powershell window and run the selected script

![image](https://github.com/PaulAntonescu/power-file/assets/74125937/daa32b27-a04a-48f9-b3ab-134260418c08)


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
