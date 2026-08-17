
//===========================================================
// FUN_141b28950 @ 141b28950   (654 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_141b28950(longlong param_1,longlong *param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  int *piVar5;
  int iVar6;
  char *pcVar7;
  undefined1 auStack_4a8 [32];
  wchar_t *local_488;
  wchar_t *local_480;
  wchar_t **local_478;
  longlong *local_470;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  local_470 = param_2;
  if (*(int *)(param_1 + 0x238) != 0) {
    if (*param_2 == 0) {
      return 0;
    }
    pcVar7 = (char *)(*param_2 + -0x10);
    goto LAB_141b28bb1;
  }
  pcVar7 = (char *)*param_2;
  if ((pcVar7 == (char *)0x0) || (*pcVar7 == '\0')) {
    if (pcVar7 == (char *)0x0) {
      return 0;
    }
    pcVar7 = pcVar7 + -0x10;
    goto LAB_141b28bb1;
  }
  iVar6 = 0;
  piVar5 = (int *)(param_1 + 0x220);
  if (piVar5 == (int *)(param_1 + 0x230)) {
LAB_141b28ad9:
    local_478 = &local_480;
    local_480 = (wchar_t *)0x0;
    local_488 = (wchar_t *)0x0;
    piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,0x22);
    piVar5[1] = 8;
    *piVar5 = -1;
    local_488 = (wchar_t *)(piVar5 + 4);
    piVar5[2] = 0;
    *local_488 = L'\0';
    uVar3 = u_useAllAP_1433fde38._12_4_;
    uVar2 = u_useAllAP_1433fde38._8_4_;
    uVar1 = u_useAllAP_1433fde38._4_4_;
    *(undefined4 *)local_488 = u_useAllAP_1433fde38._0_4_;
    piVar5[5] = uVar1;
    piVar5[6] = uVar2;
    piVar5[7] = uVar3;
    if (*piVar5 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar5[1] < 8) {
      FUN_142e54290(0x90,piVar5[1],8);
    }
    *piVar5 = 1;
    local_488[8] = L'\0';
    if (piVar5[1] + 1 < 9) {
      FUN_142e54290(0x9c,8);
    }
    piVar5[2] = 0x10;
    FUN_141b4ac80(&local_488,&local_480,0);
  }
  else {
    do {
      iVar6 = iVar6 + *piVar5;
      piVar5 = piVar5 + 1;
    } while (piVar5 != (int *)(param_1 + 0x230));
    if (iVar6 != 0x19) goto LAB_141b28ad9;
    if (*(int *)(param_1 + 0xd0) != 5) {
      FUN_14019f2c0(pcVar7 + -0x10);
      return 0;
    }
    if (((*(int *)(param_1 + 0xd4) == 0) && (DAT_143aa84a0 != 0)) && (DAT_143ac2f58 != 0)) {
      iVar6 = FUN_1408bd880(DAT_143ac2f58,param_2,0);
      if (iVar6 != 0) {
        FUN_1406ed520(local_468,0x81);
        FUN_1406edc80(local_468,param_2);
        FUN_1415d01c0(local_468);
        *(undefined4 *)(param_1 + 0xd4) = 1;
        FUN_141b3fd40(param_1,1);
        FUN_1406ed610(local_468);
        if (*param_2 != 0) {
          FUN_14019f2c0(*param_2 + -0x10);
        }
        return 1;
      }
      local_478 = &local_488;
      local_488 = (wchar_t *)0x0;
      uVar4 = FUN_1403edf80(&local_480,L"cannotUseThisName",0xffffffff);
      FUN_141b4ac80(uVar4,&local_488,0);
    }
  }
  if (*param_2 == 0) {
    return 0;
  }
  pcVar7 = (char *)(*param_2 + -0x10);
LAB_141b28bb1:
  FUN_14019f2c0(pcVar7);
  return 0;
}



//===========================================================
// FUN_141b28750 @ 141b28750   (472 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b28750(longlong param_1)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int iVar5;
  int *piVar6;
  int *piVar7;
  undefined1 auStack_498 [32];
  wchar_t *local_478 [2];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  if ((*(int *)(param_1 + 0x238) == 0) && (*(int *)(param_1 + 0xd4) == 0)) {
    if ((-1 < *(int *)(param_1 + 0x128)) && (*(int *)(param_1 + 0x128) < *(int *)(param_1 + 0xe8)))
    {
      piVar6 = (int *)FUN_14108cc50();
      if (*piVar6 != 0) {
        local_478[0] = (wchar_t *)0x0;
        piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x54);
        piVar7[1] = 0x21;
        *piVar7 = -1;
        local_478[0] = (wchar_t *)(piVar7 + 4);
        piVar7[2] = 0;
        *local_478[0] = L'\0';
        uVar1 = u_confirmDeleteCharacterPermanentl_1433fe010._8_8_;
        *(undefined8 *)local_478[0] = u_confirmDeleteCharacterPermanentl_1433fe010._0_8_;
        *(undefined8 *)(piVar7 + 6) = uVar1;
        uVar1 = u_confirmDeleteCharacterPermanentl_1433fe010._24_8_;
        *(undefined8 *)(piVar7 + 8) = u_confirmDeleteCharacterPermanentl_1433fe010._16_8_;
        *(undefined8 *)(piVar7 + 10) = uVar1;
        uVar4 = u_confirmDeleteCharacterPermanentl_1433fe010._44_4_;
        uVar3 = u_confirmDeleteCharacterPermanentl_1433fe010._40_4_;
        uVar2 = u_confirmDeleteCharacterPermanentl_1433fe010._36_4_;
        piVar7[0xc] = u_confirmDeleteCharacterPermanentl_1433fe010._32_4_;
        piVar7[0xd] = uVar2;
        piVar7[0xe] = uVar3;
        piVar7[0xf] = uVar4;
        uVar4 = u_confirmDeleteCharacterPermanentl_1433fe010._60_4_;
        uVar3 = u_confirmDeleteCharacterPermanentl_1433fe010._56_4_;
        uVar2 = u_confirmDeleteCharacterPermanentl_1433fe010._52_4_;
        piVar7[0x10] = u_confirmDeleteCharacterPermanentl_1433fe010._48_4_;
        piVar7[0x11] = uVar2;
        piVar7[0x12] = uVar3;
        piVar7[0x13] = uVar4;
        *(wchar_t *)(piVar7 + 0x14) = u_confirmDeleteCharacterPermanentl_1433fe010[0x20];
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar7[1] < 0x21) {
          FUN_142e54290(0x90,piVar7[1],0x21);
        }
        *piVar7 = 1;
        local_478[0][0x21] = L'\0';
        if (piVar7[1] + 1 < 0x22) {
          FUN_142e54290(0x9c);
        }
        piVar7[2] = 0x42;
        iVar5 = FUN_141b4a290(local_478,param_1 + 0x140);
        if (iVar5 == 0) {
          FUN_140da2650(param_1 + 0x140);
        }
        else {
          FUN_1406ed520(local_468,0x8b);
          FUN_1406ed9d0(local_468,*piVar6);
          FUN_1415d01c0(local_468);
          *(undefined4 *)(param_1 + 0xd4) = 1;
          FUN_1406ed610(local_468);
          if (DAT_143aca790 != 0) {
            FUN_1411797a0();
          }
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_141b282d0 @ 141b282d0   (655 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141b282d0(longlong param_1)

{
  undefined8 uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  undefined1 auStack_4a8 [32];
  int *local_488;
  undefined8 local_480;
  undefined8 *local_478;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  if (*(int *)(param_1 + 0x238) == 0) {
    iVar3 = FUN_14108db80();
    iVar4 = (iVar3 - *(int *)(param_1 + 0xe4)) + -1;
    iVar3 = 0;
    if (0 < iVar4) {
      iVar3 = iVar4;
    }
    piVar5 = (int *)FUN_14108c8e0(iVar3);
    if (*piVar5 == 0) {
      if ((*(int *)(param_1 + 0xe0) != 0) && (*(char *)(param_1 + 0xdc) == '\x01')) {
        local_488 = (int *)0x0;
        piVar5 = (int *)FUN_14019b600(&DAT_143ad6a30,0x12);
        piVar5[1] = 1;
        *piVar5 = -1;
        local_488 = piVar5 + 4;
        piVar5[2] = 0;
        *(undefined1 *)local_488 = 0;
        *(undefined1 *)local_488 = DAT_143275e10;
        if (*piVar5 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar5[1] < 1) {
          FUN_142e54290(0x90,piVar5[1],1);
        }
        *piVar5 = 1;
        *(undefined1 *)((longlong)local_488 + 1) = 0;
        if (piVar5[1] + 1 < 2) {
          FUN_142e54290(0x9c,1);
        }
        piVar5[2] = 1;
        FUN_1406ed520(local_468,0xa8);
        FUN_1406edc80(local_468,&local_488);
        FUN_1415d01c0(local_468);
        FUN_140da2650(param_1 + 0x140);
        FUN_1406ed610(local_468);
        if (local_488 != (int *)0x0) {
          FUN_14019f2c0(local_488 + -4);
        }
      }
    }
    else {
      local_478 = &local_480;
      local_480 = 0;
      local_488 = (int *)0x0;
      piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,0x44);
      piVar5[1] = 0x19;
      *piVar5 = -1;
      local_488 = piVar5 + 4;
      piVar5[2] = 0;
      *(undefined2 *)local_488 = 0;
      uVar1 = _UNK_1433fde08;
      *(undefined8 *)local_488 = _DAT_1433fde00;
      *(undefined8 *)(piVar5 + 6) = uVar1;
      iVar2 = _UNK_1433fde1c;
      iVar4 = _UNK_1433fde18;
      iVar3 = _UNK_1433fde14;
      piVar5[8] = _DAT_1433fde10;
      piVar5[9] = iVar3;
      piVar5[10] = iVar4;
      piVar5[0xb] = iVar2;
      iVar2 = _UNK_1433fde2c;
      iVar4 = _UNK_1433fde28;
      iVar3 = _UNK_1433fde24;
      piVar5[0xc] = _DAT_1433fde20;
      piVar5[0xd] = iVar3;
      piVar5[0xe] = iVar4;
      piVar5[0xf] = iVar2;
      *(undefined2 *)(piVar5 + 0x10) = DAT_1433fde30;
      if (*piVar5 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar5[1] < 0x19) {
        FUN_142e54290(0x90,piVar5[1],0x19);
      }
      *piVar5 = 1;
      *(undefined2 *)((longlong)local_488 + 0x32) = 0;
      if (piVar5[1] + 1 < 0x1a) {
        FUN_142e54290(0x9c,0x19);
      }
      piVar5[2] = 0x32;
      FUN_141b4ac80(&local_488,&local_480,param_1 + 0x140);
    }
  }
  return;
}



//===========================================================
// FUN_141b28bf0 @ 141b28bf0   (130 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b28bf0(longlong param_1,undefined4 param_2)

{
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  if (*(int *)(param_1 + 0xd4) == 0) {
    FUN_1406ed520(local_468,0x7b);
    FUN_1415d01c0(local_468);
    *(undefined4 *)(param_1 + 0xb8) = param_2;
    *(undefined4 *)(param_1 + 0xd4) = 1;
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_141b2cb70 @ 141b2cb70   (459 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b2cb70(longlong param_1)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int iVar5;
  int *piVar6;
  int *piVar7;
  undefined1 auStack_498 [32];
  wchar_t *local_478 [2];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  if (*(int *)(param_1 + 0xd4) == 0) {
    if ((-1 < *(int *)(param_1 + 0x128)) && (*(int *)(param_1 + 0x128) < *(int *)(param_1 + 0xe8)))
    {
      piVar6 = (int *)FUN_14108cc50();
      if (*piVar6 != 0) {
        local_478[0] = (wchar_t *)0x0;
        piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x54);
        piVar7[1] = 0x21;
        *piVar7 = -1;
        local_478[0] = (wchar_t *)(piVar7 + 4);
        piVar7[2] = 0;
        *local_478[0] = L'\0';
        uVar1 = u_confirmDeleteCharacterPermanentl_1433fe010._8_8_;
        *(undefined8 *)local_478[0] = u_confirmDeleteCharacterPermanentl_1433fe010._0_8_;
        *(undefined8 *)(piVar7 + 6) = uVar1;
        uVar1 = u_confirmDeleteCharacterPermanentl_1433fe010._24_8_;
        *(undefined8 *)(piVar7 + 8) = u_confirmDeleteCharacterPermanentl_1433fe010._16_8_;
        *(undefined8 *)(piVar7 + 10) = uVar1;
        uVar4 = u_confirmDeleteCharacterPermanentl_1433fe010._44_4_;
        uVar3 = u_confirmDeleteCharacterPermanentl_1433fe010._40_4_;
        uVar2 = u_confirmDeleteCharacterPermanentl_1433fe010._36_4_;
        piVar7[0xc] = u_confirmDeleteCharacterPermanentl_1433fe010._32_4_;
        piVar7[0xd] = uVar2;
        piVar7[0xe] = uVar3;
        piVar7[0xf] = uVar4;
        uVar4 = u_confirmDeleteCharacterPermanentl_1433fe010._60_4_;
        uVar3 = u_confirmDeleteCharacterPermanentl_1433fe010._56_4_;
        uVar2 = u_confirmDeleteCharacterPermanentl_1433fe010._52_4_;
        piVar7[0x10] = u_confirmDeleteCharacterPermanentl_1433fe010._48_4_;
        piVar7[0x11] = uVar2;
        piVar7[0x12] = uVar3;
        piVar7[0x13] = uVar4;
        *(wchar_t *)(piVar7 + 0x14) = u_confirmDeleteCharacterPermanentl_1433fe010[0x20];
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar7[1] < 0x21) {
          FUN_142e54290(0x90,piVar7[1],0x21);
        }
        *piVar7 = 1;
        local_478[0][0x21] = L'\0';
        if (piVar7[1] + 1 < 0x22) {
          FUN_142e54290(0x9c);
        }
        piVar7[2] = 0x42;
        iVar5 = FUN_141b4a290(local_478,param_1 + 0x140);
        if (iVar5 == 0) {
          FUN_140da2650(param_1 + 0x140);
        }
        else {
          FUN_1406ed520(local_468,0x8b);
          FUN_1406ed9d0(local_468,*piVar6);
          FUN_1415d01c0(local_468);
          *(undefined4 *)(param_1 + 0xd4) = 1;
          FUN_1406ed610(local_468);
          if (DAT_143aca790 != 0) {
            FUN_1411797a0();
          }
        }
      }
    }
  }
  return;
}


