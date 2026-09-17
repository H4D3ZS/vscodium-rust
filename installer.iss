#define MyAppName "VSCodium Rust"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Hades Cyber Systems"
#define MyAppExeName "vscodium-rust.exe"
#define MyPanelExeName "vscodium-ip-panel.exe"

[Setup]
AppId={{D3F9B7A2-1C2E-4D88-9B7E-99C128D3F921}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputDir=.
OutputBaseFilename=VSCodium-Rust-Setup-v1.0.0-x64
SetupIconFile=src-native\app_icon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ChangesEnvironment=yes
DisableProgramGroupPage=auto

CloseApplications=yes
RestartApplications=no
ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"
Name: "addtopath"; Description: "Add VSCodium Rust to PATH"; GroupDescription: "Other tasks:"
Name: "openwithcontext"; Description: "Add 'Open with VSCodium Rust' to Windows Explorer context menu"; GroupDescription: "Other tasks:"

[Files]
Source: "release-bundle\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\{#MyAppExeName}"
Name: "{group}\VSCodium IP Panel"; Filename: "{app}\{#MyPanelExeName}"; IconFilename: "{app}\{#MyPanelExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon; IconFilename: "{app}\{#MyAppExeName}"

[Registry]
; Open with VSCodium Rust file context menu
Root: HKA; Subkey: "Software\Classes\*\shell\VSCodiumRust"; ValueType: string; ValueData: "Open with VSCodium Rust"; Tasks: openwithcontext; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\*\shell\VSCodiumRust"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#MyAppExeName}"""; Tasks: openwithcontext
Root: HKA; Subkey: "Software\Classes\*\shell\VSCodiumRust\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Tasks: openwithcontext

; Open with VSCodium Rust directory context menu
Root: HKA; Subkey: "Software\Classes\Directory\shell\VSCodiumRust"; ValueType: string; ValueData: "Open with VSCodium Rust"; Tasks: openwithcontext; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Directory\shell\VSCodiumRust"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#MyAppExeName}"""; Tasks: openwithcontext
Root: HKA; Subkey: "Software\Classes\Directory\shell\VSCodiumRust\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" ""%V"""; Tasks: openwithcontext

; Directory Background context menu
Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\VSCodiumRust"; ValueType: string; ValueData: "Open with VSCodium Rust"; Tasks: openwithcontext; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\VSCodiumRust"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#MyAppExeName}"""; Tasks: openwithcontext
Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\VSCodiumRust\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" ""%V"""; Tasks: openwithcontext

; PATH environment variable for current user
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Tasks: addtopath; Check: (not IsAdminInstallMode) and NeedsAddPath(ExpandConstant('{app}'))

; PATH environment variable for all users (system-wide)
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Tasks: addtopath; Check: IsAdminInstallMode and NeedsAddPath(ExpandConstant('{app}'))

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Code]
var
  ExistingActionPage: TInputOptionWizardPage;
  ExistingInstalled: Boolean;
  ExistingDir: string;
  ExistingVer: string;
  ExistingUninstaller: string;
  IsRepairMode: Boolean;

function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
  RootKey: Integer;
  SubKey: string;
begin
  if IsAdminInstallMode then
  begin
    RootKey := HKEY_LOCAL_MACHINE;
    SubKey := 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
  end
  else
  begin
    RootKey := HKEY_CURRENT_USER;
    SubKey := 'Environment';
  end;

  if not RegQueryStringValue(RootKey, SubKey, 'Path', OrigPath) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') = 0;
end;

function FindExistingInstall(var OutDir, OutVer, OutUninstaller: string): Boolean;
var
  SubKey: string;
  Found: Boolean;
  CandidateDir: string;
begin
  Found := False;
  OutDir := '';
  OutVer := '';
  OutUninstaller := '';
  SubKey := 'Software\Microsoft\Windows\CurrentVersion\Uninstall\' + '{#SetupSetting("AppId")}' + '_is1';

  // Check Current User registry
  if RegKeyExists(HKEY_CURRENT_USER, SubKey) then
  begin
    RegQueryStringValue(HKEY_CURRENT_USER, SubKey, 'InstallLocation', OutDir);
    if OutDir = '' then
      RegQueryStringValue(HKEY_CURRENT_USER, SubKey, 'Inno Setup: App Path', OutDir);
    RegQueryStringValue(HKEY_CURRENT_USER, SubKey, 'DisplayVersion', OutVer);
    RegQueryStringValue(HKEY_CURRENT_USER, SubKey, 'UninstallString', OutUninstaller);
    Found := True;
  end;

  // Check Local Machine registry if not found
  if (not Found) and RegKeyExists(HKEY_LOCAL_MACHINE, SubKey) then
  begin
    RegQueryStringValue(HKEY_LOCAL_MACHINE, SubKey, 'InstallLocation', OutDir);
    if OutDir = '' then
      RegQueryStringValue(HKEY_LOCAL_MACHINE, SubKey, 'Inno Setup: App Path', OutDir);
    RegQueryStringValue(HKEY_LOCAL_MACHINE, SubKey, 'DisplayVersion', OutVer);
    RegQueryStringValue(HKEY_LOCAL_MACHINE, SubKey, 'UninstallString', OutUninstaller);
    Found := True;
  end;

  if (OutDir = '') and (OutUninstaller <> '') then
  begin
    OutDir := ExtractFileDir(RemoveQuotes(OutUninstaller));
  end;

  if (OutDir <> '') and DirExists(OutDir) and FileExists(AddBackslash(OutDir) + '{#MyAppExeName}') then
  begin
    Result := True;
    exit;
  end;

  // Fallback: check standard per-user install directory
  CandidateDir := ExpandConstant('{localappdata}\Programs\{#MyAppName}');
  if DirExists(CandidateDir) and FileExists(AddBackslash(CandidateDir) + '{#MyAppExeName}') then
  begin
    OutDir := CandidateDir;
    OutVer := 'Detected';
    OutUninstaller := AddBackslash(CandidateDir) + 'unins000.exe';
    Result := True;
    exit;
  end;

  // Fallback: check standard Program Files directory
  CandidateDir := ExpandConstant('{commonpf}\{#MyAppName}');
  if DirExists(CandidateDir) and FileExists(AddBackslash(CandidateDir) + '{#MyAppExeName}') then
  begin
    OutDir := CandidateDir;
    OutVer := 'Detected';
    OutUninstaller := AddBackslash(CandidateDir) + 'unins000.exe';
    Result := True;
    exit;
  end;

  Result := False;
end;

procedure InitializeWizard();
var
  PromptMsg: string;
begin
  ExistingInstalled := FindExistingInstall(ExistingDir, ExistingVer, ExistingUninstaller);

  if ExistingInstalled then
  begin
    if ExistingVer = '' then
      ExistingVer := 'Detected';

    PromptMsg := 'An existing installation of {#MyAppName} was found on this computer:' + #13#10 +
                 '  • Installed Folder: ' + ExistingDir + #13#10 +
                 '  • Installed Version: ' + ExistingVer + #13#10 +
                 '  • Installer Version: ' + '{#MyAppVersion}' + #13#10#13#10 +
                 'Select what you want to do:';

    ExistingActionPage := CreateInputOptionPage(
      wpWelcome,
      'Existing Installation Detected',
      'Update, repair, or uninstall your current installation.',
      PromptMsg,
      True, False
    );

    ExistingActionPage.Add('Update / Replace (Recommended)' + #13#10 +
                           '   Upgrade existing installation in-place. All settings and workspaces are preserved.');
    ExistingActionPage.Add('Repair / Reinstall' + #13#10 +
                           '   Reinstall and verify all application files, shortcuts, and Explorer context menus.');
    ExistingActionPage.Add('Uninstall' + #13#10 +
                           '   Completely remove the existing installation from this computer.');

    ExistingActionPage.SelectedValueIndex := 0;

    WizardForm.DirEdit.Text := ExistingDir;
  end;
end;

function ShouldSkipPage(PageID: Integer): Boolean;
begin
  Result := False;

  if ExistingInstalled then
  begin
    // If Update (0) or Repair (1) selected, skip directory selection page
    if (PageID = wpSelectDir) and (ExistingActionPage.SelectedValueIndex <> 2) then
    begin
      Result := True;
    end;
    // Skip start menu folder page when updating or repairing
    if (PageID = wpSelectProgramGroup) and (ExistingActionPage.SelectedValueIndex <> 2) then
    begin
      Result := True;
    end;
  end;
end;

function NextButtonClick(CurPageID: Integer): Boolean;
var
  ResultCode: Integer;
  UninstCmd: string;
  ContinueInstall: Integer;
  i: Integer;
begin
  Result := True;

  if ExistingInstalled and (CurPageID = ExistingActionPage.ID) then
  begin
    case ExistingActionPage.SelectedValueIndex of
      0: // Update / Replace
      begin
        WizardForm.DirEdit.Text := ExistingDir;
        IsRepairMode := False;
      end;
      1: // Repair
      begin
        WizardForm.DirEdit.Text := ExistingDir;
        IsRepairMode := True;
        if WizardForm.TasksList <> nil then
        begin
          for i := 0 to WizardForm.TasksList.Items.Count - 1 do
            WizardForm.TasksList.Checked[i] := True;
        end;
      end;
      2: // Uninstall
      begin
        if MsgBox('Are you sure you want to completely uninstall {#MyAppName} (' + ExistingVer + ')?' + #13#10#13#10 +
                  'Folder: ' + ExistingDir, mbConfirmation, MB_YESNO) = IDYES then
        begin
          UninstCmd := RemoveQuotes(ExistingUninstaller);
          if FileExists(UninstCmd) then
          begin
            Exec(UninstCmd, '', '', SW_SHOW, ewWaitUntilTerminated, ResultCode);
          end
          else
          begin
            MsgBox('Uninstaller executable not found at:' + #13#10 + UninstCmd, mbError, MB_OK);
          end;

          ContinueInstall := MsgBox('Uninstallation finished.' + #13#10#13#10 +
                                    'Would you like to continue installing the new release ({#MyAppVersion}) now?',
                                    mbConfirmation, MB_YESNO);
          if ContinueInstall = IDYES then
          begin
            ExistingInstalled := False;
            WizardForm.DirEdit.Text := ExpandConstant('{autopf}\{#MyAppName}');
            Result := True;
          end
          else
          begin
            WizardForm.Close;
            Result := False;
          end;
        end
        else
        begin
          Result := False;
        end;
      end;
    end;
  end;
end;
