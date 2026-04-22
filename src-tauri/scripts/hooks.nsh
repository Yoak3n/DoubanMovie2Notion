!macro NSIS_HOOK_POSTINSTALL
  Delete "$INSTDIR\db2n.com"
  ${If} ${FileExists} "$INSTDIR\db2n-cli.exe"
    Rename "$INSTDIR\db2n-cli.exe" "$INSTDIR\db2n.com"
  ${ElseIf} ${FileExists} "$INSTDIR\resources\db2n-cli.exe"
    CopyFiles /SILENT "$INSTDIR\resources\db2n-cli.exe" "$INSTDIR\db2n.com"
  ${EndIf}
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ExecWait '"$INSTDIR\db2n.com" --remove-path'
  Delete "$INSTDIR\db2n.com"
!macroend
