
//===========================================================
// FUN_141b28570 @ 141b28570   (290 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b28570(longlong param_1,undefined8 param_2)

{
  char cVar1;
  undefined1 auStack_4a8 [32];
  int *local_488;
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  cVar1 = FUN_141b3faf0(param_1);
  if (cVar1 == '\0') {
    if (*(int *)(param_1 + 0xd4) == 0) {
      cVar1 = FUN_1408fcfc0(0,*(int *)(param_1 + 0xe8) + -1,*(undefined4 *)(param_1 + 0x128));
      if (cVar1 == '\0') {
        FUN_140199470(param_2);
      }
      else {
        local_488 = (int *)FUN_141b3b0f0(param_1);
        if (*local_488 == 0) {
          FUN_140199470(param_2);
        }
        else {
          cVar1 = FUN_14108d4b0(*local_488);
          if (cVar1 == '\0') {
                    /* WARNING: Bad instruction - Truncating control flow here */
            halt_baddata();
          }
          FUN_140199470(param_2);
        }
      }
    }
    else {
      FUN_140199470(param_2);
    }
  }
  else {
    FUN_140199470(param_2);
  }
  return;
}



//===========================================================
// FUN_141b38d90 @ 141b38d90   (1083 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b38d90(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  char cVar5;
  undefined1 uVar6;
  int iVar7;
  int *piVar8;
  int *piVar9;
  undefined1 auStack_4b8 [32];
  wchar_t *local_498 [2];
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_4b8;
  *(undefined4 *)(param_1 + 0xd4) = 0;
  cVar5 = FUN_1406e8ae0(param_2);
  uVar6 = FUN_1406e8ae0(param_2);
  FUN_142cb7e30(DAT_143aa84a0,uVar6);
  FUN_142cb83e0(DAT_143aa84a0,cVar5 == '\x01');
  *(char *)(param_1 + 0xdc) = cVar5;
  iVar7 = *(int *)(param_1 + 0xb8);
  if (iVar7 == 1) {
    local_498[0] = (wchar_t *)0x0;
    piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar8[1] = 0;
    *piVar8 = -1;
    local_498[0] = (wchar_t *)(piVar8 + 4);
    piVar8[2] = 0;
    *(undefined1 *)local_498[0] = 0;
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar8[1] < 0) {
      FUN_142e54290(0x90,piVar8[1],0);
    }
    *piVar8 = 1;
    *(undefined1 *)local_498[0] = 0;
    if (piVar8[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar8[2] = 0;
    FUN_141b28570(param_1,local_498,0);
  }
  else if (iVar7 == 3) {
    if (*(int *)(param_1 + 0x170) == 0) {
      FUN_141b282d0(param_1);
    }
  }
  else if (iVar7 == 4) {
    if ((((*(int *)(param_1 + 0xd4) == 0) && (-1 < *(int *)(param_1 + 0x128))) &&
        (*(int *)(param_1 + 0x128) < *(int *)(param_1 + 0xe8))) &&
       (piVar8 = (int *)FUN_14108cc50(), *piVar8 != 0)) {
      FUN_140da2650(param_1 + 0x140);
      local_498[0] = (wchar_t *)0x0;
      piVar9 = (int *)FUN_1401bc720(&DAT_143ad6980,0x54);
      piVar9[1] = 0x21;
      *piVar9 = -1;
      local_498[0] = (wchar_t *)(piVar9 + 4);
      piVar9[2] = 0;
      *local_498[0] = L'\0';
      uVar1 = u_confirmDeleteCharacterPermanentl_1433fe010._8_8_;
      *(undefined8 *)local_498[0] = u_confirmDeleteCharacterPermanentl_1433fe010._0_8_;
      *(undefined8 *)(piVar9 + 6) = uVar1;
      uVar1 = u_confirmDeleteCharacterPermanentl_1433fe010._24_8_;
      *(undefined8 *)(piVar9 + 8) = u_confirmDeleteCharacterPermanentl_1433fe010._16_8_;
      *(undefined8 *)(piVar9 + 10) = uVar1;
      uVar4 = u_confirmDeleteCharacterPermanentl_1433fe010._44_4_;
      uVar3 = u_confirmDeleteCharacterPermanentl_1433fe010._40_4_;
      uVar2 = u_confirmDeleteCharacterPermanentl_1433fe010._36_4_;
      piVar9[0xc] = u_confirmDeleteCharacterPermanentl_1433fe010._32_4_;
      piVar9[0xd] = uVar2;
      piVar9[0xe] = uVar3;
      piVar9[0xf] = uVar4;
      uVar4 = u_confirmDeleteCharacterPermanentl_1433fe010._60_4_;
      uVar3 = u_confirmDeleteCharacterPermanentl_1433fe010._56_4_;
      uVar2 = u_confirmDeleteCharacterPermanentl_1433fe010._52_4_;
      piVar9[0x10] = u_confirmDeleteCharacterPermanentl_1433fe010._48_4_;
      piVar9[0x11] = uVar2;
      piVar9[0x12] = uVar3;
      piVar9[0x13] = uVar4;
      *(wchar_t *)(piVar9 + 0x14) = u_confirmDeleteCharacterPermanentl_1433fe010[0x20];
      if (*piVar9 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar9[1] < 0x21) {
        FUN_142e54290(0x90,piVar9[1],0x21);
      }
      *piVar9 = 1;
      local_498[0][0x21] = L'\0';
      if (piVar9[1] + 1 < 0x22) {
        FUN_142e54290(0x9c);
      }
      piVar9[2] = 0x42;
      iVar7 = FUN_141b4a290(local_498,param_1 + 0x140);
      if (iVar7 == 0) {
        FUN_140da2650(param_1 + 0x140);
      }
      else {
        FUN_1406ed520(local_488,0x8c);
        FUN_1406ed9d0(local_488,*piVar8);
        FUN_1415d01c0(local_488);
        *(undefined4 *)(param_1 + 0xd4) = 1;
        FUN_1406ed610(local_488);
      }
    }
  }
  else if (iVar7 == 6) {
    iVar7 = FUN_142cb85a0(DAT_143aa84a0);
    if (iVar7 != 0) {
      local_498[0] = (wchar_t *)0x0;
      piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x44);
      piVar8[1] = 0x19;
      *piVar8 = -1;
      local_498[0] = (wchar_t *)(piVar8 + 4);
      piVar8[2] = 0;
      *local_498[0] = L'\0';
      uVar1 = u_identifyVerficationFailed_1433fe870._8_8_;
      *(undefined8 *)local_498[0] = u_identifyVerficationFailed_1433fe870._0_8_;
      *(undefined8 *)(piVar8 + 6) = uVar1;
      uVar4 = u_identifyVerficationFailed_1433fe870._28_4_;
      uVar3 = u_identifyVerficationFailed_1433fe870._24_4_;
      uVar2 = u_identifyVerficationFailed_1433fe870._20_4_;
      piVar8[8] = u_identifyVerficationFailed_1433fe870._16_4_;
      piVar8[9] = uVar2;
      piVar8[10] = uVar3;
      piVar8[0xb] = uVar4;
      uVar4 = u_identifyVerficationFailed_1433fe870._44_4_;
      uVar3 = u_identifyVerficationFailed_1433fe870._40_4_;
      uVar2 = u_identifyVerficationFailed_1433fe870._36_4_;
      piVar8[0xc] = u_identifyVerficationFailed_1433fe870._32_4_;
      piVar8[0xd] = uVar2;
      piVar8[0xe] = uVar3;
      piVar8[0xf] = uVar4;
      *(wchar_t *)(piVar8 + 0x10) = u_identifyVerficationFailed_1433fe870[0x18];
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar8[1] < 0x19) {
        FUN_142e54290(0x90,piVar8[1],0x19);
      }
      *piVar8 = 1;
      local_498[0][0x19] = L'\0';
      if (piVar8[1] + 1 < 0x1a) {
        FUN_142e54290(0x9c,0x19);
      }
      piVar8[2] = 0x32;
      FUN_141b4a840(local_498,param_1 + 0x140);
    }
  }
  else if ((iVar7 == 9) && (iVar7 = FUN_142cb85a0(DAT_143aa84a0), iVar7 != 0)) {
    FUN_1406ed520(local_488,0x7c);
    FUN_1415d01c0(local_488);
    FUN_1406ed610(local_488);
  }
  *(undefined4 *)(param_1 + 0xb8) = 0;
  return;
}


