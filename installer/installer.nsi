; Monitor Splitter NSIS Installer Script
; Bundles the IddCx driver, Tauri app, and enables test signing

!include "MUI2.nsh"
!include "LogicLib.nsh"

; ─── General ─────────────────────────────────────────────────────────────────

Name "Monitor Splitter"
OutFile "MonitorSplitter-Setup.exe"
InstallDir "$PROGRAMFILES64\MonitorSplitter"
RequestExecutionLevel admin

; ─── UI ──────────────────────────────────────────────────────────────────────

!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "..\LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

; ─── Sections ────────────────────────────────────────────────────────────────

Section "Monitor Splitter App" SecApp
  SectionIn RO ; Required

  SetOutPath "$INSTDIR"
  File "app\monitor-splitter.exe"

  ; Create shortcuts
  CreateDirectory "$SMPROGRAMS\Monitor Splitter"
  CreateShortCut "$SMPROGRAMS\Monitor Splitter\Monitor Splitter.lnk" "$INSTDIR\monitor-splitter.exe"
  CreateShortCut "$DESKTOP\Monitor Splitter.lnk" "$INSTDIR\monitor-splitter.exe"

  ; Write uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  ; Registry for Add/Remove Programs
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\MonitorSplitter" \
    "DisplayName" "Monitor Splitter"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\MonitorSplitter" \
    "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\MonitorSplitter" \
    "Publisher" "Monitor Splitter Project"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\MonitorSplitter" \
    "DisplayVersion" "0.1.6"
SectionEnd

!ifndef SKIP_DRIVER
Section "Virtual Display Driver" SecDriver
  SectionIn RO ; Required

  SetOutPath "$INSTDIR\driver"
  File "driver\monitor_splitter_driver.sys"
  File "driver\MonitorSplitter.inf"
  File "driver\MonitorSplitter.cat"

  ; Install the driver via pnputil
  nsExec::ExecToLog 'pnputil /add-driver "$INSTDIR\driver\MonitorSplitter.inf" /install'
SectionEnd

Section "Enable Test Signing" SecTestSign
  ; Required for self-signed driver to load
  nsExec::ExecToLog 'bcdedit /set testsigning on'

  MessageBox MB_YESNO "Test signing has been enabled. A reboot is required for the driver to load. Reboot now?" IDNO +2
    Reboot
SectionEnd
!endif

; ─── Uninstaller ─────────────────────────────────────────────────────────────

Section "Uninstall"
  ; Remove driver if it was installed
  IfFileExists "$INSTDIR\driver\MonitorSplitter.inf" 0 +2
    nsExec::ExecToLog 'pnputil /delete-driver "$INSTDIR\driver\MonitorSplitter.inf" /uninstall'

  ; Remove files
  Delete "$INSTDIR\monitor-splitter.exe"
  Delete "$INSTDIR\driver\*.*"
  RMDir "$INSTDIR\driver"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"

  ; Remove shortcuts
  Delete "$SMPROGRAMS\Monitor Splitter\*.*"
  RMDir "$SMPROGRAMS\Monitor Splitter"
  Delete "$DESKTOP\Monitor Splitter.lnk"

  ; Remove registry
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\MonitorSplitter"
SectionEnd

; ─── Descriptions ────────────────────────────────────────────────────────────

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecApp} "The Monitor Splitter application with UI and hotkey support."
!ifndef SKIP_DRIVER
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDriver} "The IddCx virtual display driver that creates virtual monitors."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecTestSign} "Enable Windows test signing mode (required for the driver to load without an EV certificate)."
!endif
!insertmacro MUI_FUNCTION_DESCRIPTION_END





