; Per-user install under %LOCALAPPDATA%\Programs\<ProductName> (Antigravity / VS Code style).
; PREINSTALL runs after the template's first SetOutPath — re-point before files copy.
!macro NSIS_HOOK_PREINSTALL
  StrCpy $INSTDIR "$LOCALAPPDATA\Programs\${PRODUCTNAME}"
  SetOutPath $INSTDIR
!macroend
