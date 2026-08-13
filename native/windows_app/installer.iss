#define MyAppName "COSMOS HEARTLIGHT"
#define MyAppVersion "0.3.0"
#define MyAppPublisher "NavisWORLD"
#define MyAppExeName "COSMOS-HEARTLIGHT.exe"
#define PublishDir "..\..\publish\windows"

[Setup]
AppId={{C1B4B4B8-5B87-47A7-9B7B-6D4C30C5F633}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={localappdata}\Programs\COSMOS HEARTLIGHT
DisableProgramGroupPage=yes
OutputDir=..\..\release
OutputBaseFilename=COSMOS-HEARTLIGHT-Setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayIcon={app}\{#MyAppExeName}

[Files]
Source: "{#PublishDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\COSMOS HEARTLIGHT"; Filename: "{app}\{#MyAppExeName}"
Name: "{userdesktop}\COSMOS HEARTLIGHT"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional shortcuts:"

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch COSMOS HEARTLIGHT"; Flags: nowait postinstall skipifsilent
