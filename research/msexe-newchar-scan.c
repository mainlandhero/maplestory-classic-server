
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



//===========================================================
// FUN_141b391e0 @ 141b391e0   (629 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b391e0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int *piVar5;
  undefined1 auStack_4a8 [32];
  wchar_t *local_488;
  longlong local_480;
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  local_488 = (wchar_t *)0x0;
  piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,0x44);
  piVar5[1] = 0x19;
  *piVar5 = -1;
  local_488 = (wchar_t *)(piVar5 + 4);
  piVar5[2] = 0;
  *local_488 = L'\0';
  uVar1 = u_goToNexonAuthPageToVerify_1433fe810._8_8_;
  *(undefined8 *)local_488 = u_goToNexonAuthPageToVerify_1433fe810._0_8_;
  *(undefined8 *)(piVar5 + 6) = uVar1;
  uVar4 = u_goToNexonAuthPageToVerify_1433fe810._28_4_;
  uVar3 = u_goToNexonAuthPageToVerify_1433fe810._24_4_;
  uVar2 = u_goToNexonAuthPageToVerify_1433fe810._20_4_;
  piVar5[8] = u_goToNexonAuthPageToVerify_1433fe810._16_4_;
  piVar5[9] = uVar2;
  piVar5[10] = uVar3;
  piVar5[0xb] = uVar4;
  uVar4 = u_goToNexonAuthPageToVerify_1433fe810._44_4_;
  uVar3 = u_goToNexonAuthPageToVerify_1433fe810._40_4_;
  uVar2 = u_goToNexonAuthPageToVerify_1433fe810._36_4_;
  piVar5[0xc] = u_goToNexonAuthPageToVerify_1433fe810._32_4_;
  piVar5[0xd] = uVar2;
  piVar5[0xe] = uVar3;
  piVar5[0xf] = uVar4;
  *(wchar_t *)(piVar5 + 0x10) = u_goToNexonAuthPageToVerify_1433fe810[0x18];
  if (*piVar5 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar5[1] < 0x19) {
    FUN_142e54290(0x90,piVar5[1],0x19);
  }
  *piVar5 = 1;
  local_488[0x19] = L'\0';
  if (piVar5[1] + 1 < 0x1a) {
    FUN_142e54290(0x9c,0x19);
  }
  piVar5[2] = 0x32;
  FUN_141b4a840(&local_488,param_1 + 0x140);
  FUN_1406e9050(param_2,&local_480);
  FUN_1429e4fa0(local_480,0,0);
  local_488 = (wchar_t *)0x0;
  piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,0x34);
  piVar5[1] = 0x11;
  *piVar5 = -1;
  local_488 = (wchar_t *)(piVar5 + 4);
  piVar5[2] = 0;
  *local_488 = L'\0';
  uVar4 = u_clickOKIfVerified_1433fe848._12_4_;
  uVar3 = u_clickOKIfVerified_1433fe848._8_4_;
  uVar2 = u_clickOKIfVerified_1433fe848._4_4_;
  *(undefined4 *)local_488 = u_clickOKIfVerified_1433fe848._0_4_;
  piVar5[5] = uVar2;
  piVar5[6] = uVar3;
  piVar5[7] = uVar4;
  uVar4 = u_clickOKIfVerified_1433fe848._28_4_;
  uVar3 = u_clickOKIfVerified_1433fe848._24_4_;
  uVar2 = u_clickOKIfVerified_1433fe848._20_4_;
  piVar5[8] = u_clickOKIfVerified_1433fe848._16_4_;
  piVar5[9] = uVar2;
  piVar5[10] = uVar3;
  piVar5[0xb] = uVar4;
  *(wchar_t *)(piVar5 + 0xc) = u_clickOKIfVerified_1433fe848[0x10];
  if (*piVar5 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar5[1] < 0x11) {
    FUN_142e54290(0x90,piVar5[1],0x11);
  }
  *piVar5 = 1;
  local_488[0x11] = L'\0';
  if (piVar5[1] + 1 < 0x12) {
    FUN_142e54290(0x9c,0x11);
  }
  piVar5[2] = 0x22;
  FUN_141b4a840(&local_488,param_1 + 0x140);
  if (*(int *)(param_1 + 0xd4) == 0) {
    FUN_1406ed520(local_478,0x7b);
    FUN_1415d01c0(local_478);
    *(undefined4 *)(param_1 + 0xb8) = 6;
    *(undefined4 *)(param_1 + 0xd4) = 1;
    FUN_1406ed610(local_478);
  }
  if (local_480 != 0) {
    FUN_14019f2c0(local_480 + -0x10);
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
// FUN_141b32860 @ 141b32860   (5817 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141b32e09) */
/* WARNING: Removing unreachable block (ram,0x000141b3355a) */

void FUN_141b32860(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  longlong *plVar2;
  code *pcVar3;
  IUnknown *pIVar4;
  longlong lVar5;
  byte bVar6;
  char cVar7;
  undefined1 uVar8;
  int iVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  int iVar13;
  int iVar14;
  uint uVar15;
  int *piVar16;
  int **ppiVar17;
  longlong lVar18;
  undefined8 *puVar19;
  undefined8 uVar20;
  int *piVar21;
  undefined4 *puVar22;
  short *psVar23;
  undefined8 uVar24;
  char *pcVar25;
  uint uVar26;
  ulonglong uVar27;
  undefined2 *puVar28;
  int *piVar29;
  int *piVar30;
  int *piVar31;
  int *piVar32;
  undefined8 auStack_640 [5];
  longlong local_618;
  undefined4 local_610 [6];
  uint local_5f8 [2];
  int *local_5f0;
  int *local_5e8;
  longlong local_5e0;
  int *local_5d8;
  undefined8 *local_5d0;
  longlong local_5c8;
  int *local_5c0;
  uint local_5b8;
  ulonglong local_5b0;
  int *local_5a8;
  int *local_5a0;
  int **local_598;
  undefined4 local_590;
  undefined4 uStack_58c;
  undefined8 uStack_588;
  undefined8 local_580;
  short local_578;
  undefined6 uStack_576;
  longlong lStack_570;
  undefined8 local_568;
  int *local_560;
  undefined4 local_558;
  undefined4 local_554;
  undefined4 local_550;
  undefined4 local_54c;
  int *local_548;
  int *local_540;
  int **local_538;
  longlong local_530;
  uint local_528;
  undefined4 uStack_524;
  undefined4 uStack_520;
  undefined4 uStack_51c;
  undefined8 local_518;
  IUnknown *local_510;
  uint local_508;
  undefined4 uStack_504;
  undefined4 uStack_500;
  undefined4 uStack_4fc;
  undefined8 local_4f8;
  undefined8 local_4f0;
  undefined1 local_4e8 [8];
  longlong *local_4e0;
  undefined8 local_4d8;
  longlong lStack_4d0;
  undefined8 local_4c8;
  uint local_4b8;
  undefined4 uStack_4b4;
  undefined4 uStack_4b0;
  undefined4 uStack_4ac;
  undefined8 local_4a8;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)local_5f8;
  piVar31 = (int *)0x0;
  *(undefined4 *)(param_1 + 0xd4) = 0;
  local_5f0 = DAT_143aa84a0;
  auStack_640[0] = 0x141b328b2;
  local_5e0 = param_1;
  bVar6 = FUN_1406e8ae0(param_2);
  uVar15 = (uint)bVar6;
  auStack_640[0] = 0x141b328c9;
  local_5f8[0] = (uint)bVar6;
  FUN_1406e9050(param_2,&local_530);
  piVar32 = (int *)0xffffffffffffffff;
  if (local_530 == 0) {
    iVar9 = 2;
  }
  else {
    local_610[0] = 0;
    local_618 = 0;
    auStack_640[0] = 0x141b328f8;
    iVar9 = (*DAT_1432627f8)(0xfde9,0,local_530,0xffffffff);
    iVar9 = iVar9 * 2;
  }
  lVar18 = local_530;
  uVar27 = (longlong)iVar9 + 0xf;
  if (uVar27 <= (ulonglong)(longlong)iVar9) {
    uVar27 = 0xffffffffffffff0;
  }
  auStack_640[0] = 0x141b32922;
  lVar5 = -(uVar27 & 0xfffffffffffffff0);
  puVar28 = (undefined2 *)((longlong)local_5f8 + lVar5);
  piVar29 = piVar32;
  if (local_530 == 0) {
    if (puVar28 == (undefined2 *)0x0) goto LAB_141b32958;
    *puVar28 = 0;
LAB_141b32970:
    do {
      piVar29 = (int *)((longlong)piVar29 + 1);
    } while (puVar28[(longlong)piVar29] != 0);
    iVar13 = (int)piVar29;
    iVar9 = 0;
    if (0 < iVar13) {
      iVar9 = iVar13;
    }
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32999;
    piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
    piVar16[1] = iVar9;
    *piVar16 = -1;
    piVar31 = piVar16 + 4;
    piVar16[2] = 0;
    *(undefined2 *)piVar31 = 0;
    local_548 = piVar31;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b329c8;
    FUN_142ef7ba0(piVar31,puVar28,(longlong)iVar13 * 2);
    if (*piVar16 != -1) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b329db;
      FUN_142e52dd0();
    }
    if ((iVar13 == -1) || (iVar9 = piVar16[1], iVar13 <= iVar9)) {
      *piVar16 = 1;
      if (iVar13 != -1) goto LAB_141b32a05;
      piVar29 = piVar32;
      if (piVar31 == (int *)0x0) {
        piVar29 = (int *)0x0;
      }
      else {
        do {
          piVar29 = (int *)((longlong)piVar29 + 1);
        } while (*(short *)((longlong)piVar31 + (longlong)piVar29 * 2) != 0);
      }
    }
    else {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b329fd;
      FUN_142e54290(0x90,iVar9,(ulonglong)piVar29 & 0xffffffff);
      *piVar16 = 1;
LAB_141b32a05:
      *(undefined2 *)((longlong)iVar13 * 2 + (longlong)piVar31) = 0;
    }
    iVar9 = (int)piVar29;
    if ((iVar9 < 0) || (piVar16[1] + 1 <= iVar9)) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32a28;
      FUN_142e54290(0x9c,(ulonglong)piVar29 & 0xffffffff);
    }
    piVar16[2] = iVar9 * 2;
  }
  else {
    *(undefined4 *)((longlong)local_610 + lVar5) = 0x100000;
    *(undefined2 **)((longlong)local_610 + lVar5 + -8) = puVar28;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32958;
    (*DAT_1432627f8)(0xfde9,0,lVar18,0xffffffff);
LAB_141b32958:
    local_548 = (int *)0x0;
    if (puVar28 != (undefined2 *)0x0) goto LAB_141b32970;
  }
  if (bVar6 == 0x83) {
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32a46;
    uVar10 = FUN_1406e8c20(param_2);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32a51;
    uVar11 = FUN_1406e8c20(param_2);
    local_5b0 = CONCAT44(local_5b0._4_4_,uVar11);
    lVar18 = DAT_143ac8208;
    if (DAT_143ac8208 == 0) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32a6f;
      ppiVar17 = (int **)FUN_14019b780(&DAT_143ad68a0,0x58);
      lVar18 = 0;
      if (ppiVar17 != (int **)0x0) {
        local_598 = ppiVar17;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32a80;
        lVar18 = FUN_141d5c6a0(ppiVar17);
      }
    }
    local_598 = &local_5d8;
    local_5d8 = (int *)0x0;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ad9;
    piVar29 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar29[1] = 0;
    *piVar29 = -1;
    local_5d8 = piVar29 + 4;
    piVar29[2] = 0;
    *(undefined1 *)local_5d8 = 0;
    if (*piVar29 != -1) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32b07;
      FUN_142e52dd0(0x8b);
    }
    iVar9 = piVar29[1];
    if (iVar9 < 0) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32b1b;
      FUN_142e54290(0x90,iVar9,0);
    }
    *piVar29 = 1;
    *(undefined1 *)local_5d8 = 0;
    if (piVar29[1] + 1 < 1) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32b41;
      FUN_142e54290(0x9c,0);
    }
    piVar29[2] = 0;
    local_538 = &local_5e8;
    local_5e8 = (int *)0x0;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32b64;
    piVar29 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar29[1] = 0;
    *piVar29 = -1;
    local_5e8 = piVar29 + 4;
    piVar29[2] = 0;
    *(char *)local_5e8 = '\0';
    if (*piVar29 != -1) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32b92;
      FUN_142e52dd0(0x8b);
    }
    iVar9 = piVar29[1];
    if (iVar9 < 0) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ba6;
      FUN_142e54290(0x90,iVar9,0);
    }
    *piVar29 = 1;
    *(char *)local_5e8 = '\0';
    if (piVar29[1] + 1 < 1) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32bcc;
      FUN_142e54290(0x9c,0);
    }
    piVar29[2] = 0;
    local_5a8 = (int *)0x0;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32be2;
    uVar11 = FUN_14090d160(0x3c8,1);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32bf3;
    uVar12 = FUN_14090d160(0x3c8,1);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32c03;
    puVar19 = (undefined8 *)FUN_1408a9e40(&local_5c0,0x75);
    *(undefined4 *)((longlong)local_610 + lVar5 + 0x10) = uVar11;
    *(undefined4 *)((longlong)local_610 + lVar5 + 8) = 1;
    *(undefined4 *)((longlong)local_610 + lVar5) = 0x3d1c3;
    *(undefined4 *)((longlong)local_610 + lVar5 + -8) = uVar12;
    uVar27 = local_5b0 & 0xffffffff;
    uVar20 = *puVar19;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32c2f;
    uVar20 = FUN_14019ba10(&local_5a8,uVar20,uVar10,uVar27);
    local_5c8 = 0;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32c41;
    FUN_14019a260(&local_5c8,uVar20);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32c56;
    FUN_141d5c830(lVar18,&local_5c8,&local_5e8,&local_5d8);
    if (local_5c0 != (int *)0x0) {
      piVar29 = local_5c0 + -4;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32c69;
      FUN_14019f2c0(piVar29);
    }
    if (local_5a8 != (int *)0x0) {
      piVar29 = local_5a8 + -4;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32c7c;
      FUN_14019f2c0(piVar29);
    }
    local_598 = (int **)0x0;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32c97;
    FUN_140919f30(&DAT_143271f04,0xaa9,0);
    uVar15 = local_5f8[0];
  }
  else {
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ca3;
    cVar7 = FUN_140859ca0();
    if (cVar7 != '\0') {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32cb0;
      puVar19 = (undefined8 *)FUN_142c50940(&local_5c0);
      uVar20 = *puVar19;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32cc0;
      uVar24 = FUN_142c49f00(DAT_143ac1898);
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ccc;
      (*DAT_143262b78)(uVar24,uVar20);
      if (local_5c0 != (int *)0x0) {
        piVar29 = local_5c0 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32cdf;
        FUN_14019f2c0(piVar29);
      }
    }
  }
  lVar18 = local_5e0;
  plVar2 = *(longlong **)(local_5e0 + 0x270);
  if (plVar2 != (longlong *)0x0) {
    pcVar3 = *(code **)(*plVar2 + 0x138);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32d00;
    (*pcVar3)(plVar2,3);
  }
  local_5a0 = (int *)0x0;
  piVar16 = piVar31;
  piVar29 = local_5a0;
  if ((piVar31 != (int *)0x0) && (piVar30 = piVar31 + -4, piVar30 != (int *)0x0)) {
    if (*piVar30 == -1) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32d31;
      FUN_142e52d50(0xcb,0xffffff01);
      piVar30 = piVar32;
      do {
        piVar30 = (int *)((longlong)piVar30 + 1);
      } while (*(short *)((longlong)piVar31 + (longlong)piVar30 * 2) != 0);
      iVar13 = (int)piVar30;
      iVar9 = 0;
      if (0 < iVar13) {
        iVar9 = iVar13;
      }
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32d5c;
      piVar21 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
      piVar21[1] = iVar9;
      *piVar21 = -1;
      piVar29 = piVar21 + 4;
      piVar21[2] = 0;
      *(undefined2 *)piVar29 = 0;
      local_560 = piVar29;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32d88;
      FUN_142ef7ba0(piVar29,piVar31,(longlong)iVar13 * 2);
      if (*piVar21 != -1) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32d9a;
        FUN_142e52dd0();
      }
      if ((iVar13 == -1) || (iVar9 = piVar21[1], iVar13 <= iVar9)) {
        *piVar21 = 1;
        if (iVar13 != -1) goto LAB_141b32dba;
        piVar30 = piVar32;
        if (piVar29 == (int *)0x0) {
          piVar30 = (int *)0x0;
        }
        else {
          do {
            piVar30 = (int *)((longlong)piVar30 + 1);
          } while (*(short *)((longlong)piVar29 + (longlong)piVar30 * 2) != 0);
        }
      }
      else {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32db3;
        FUN_142e54290(0x90,iVar9,(ulonglong)piVar30 & 0xffffffff);
        *piVar21 = 1;
LAB_141b32dba:
        *(undefined2 *)((longlong)iVar13 * 2 + (longlong)piVar29) = 0;
      }
      iVar9 = (int)piVar30;
      if ((iVar9 < 0) || (piVar21[1] + 1 <= iVar9)) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32dde;
        FUN_142e54290(0x9c,(ulonglong)piVar30 & 0xffffffff);
      }
      piVar21[2] = iVar9 * 2;
      if (local_5a0 != (int *)0x0) {
        piVar31 = local_5a0 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32df6;
        FUN_1401bebb0(piVar31);
      }
      local_560 = (int *)0x0;
      lVar18 = local_5e0;
    }
    else {
      if (*piVar30 < 1) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32e50;
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar30 = *piVar30 + 1;
      UNLOCK();
      piVar16 = local_548;
      piVar29 = piVar31;
      if (local_5a0 != (int *)0x0) {
        piVar31 = local_5a0 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32e65;
        FUN_1401bebb0(piVar31);
        piVar16 = local_548;
      }
    }
  }
  local_5a0 = piVar29;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32e86;
  cVar7 = FUN_141b267c0(lVar18,uVar15,0,&local_5a0);
  if ((cVar7 == '\0') || (uVar15 != 0)) goto LAB_141b33ea9;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32e9f;
  thunk_FUN_1406e8ae0(param_2);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32eb7;
  FUN_1406e9170(param_2,&local_4f0,8);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ec3;
  FUN_1408f67d0(local_4f0);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ecb;
  iVar9 = FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ed5;
  iVar13 = FUN_1406e8c20(param_2);
  piVar31 = local_5f0;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ee0;
  iVar14 = FUN_142cb9230(piVar31);
  piVar31 = local_5f0;
  if (iVar9 == iVar14) {
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32eed;
    iVar14 = FUN_142cb9260(piVar31);
    if (iVar13 != iVar14) goto LAB_141b32ef1;
  }
  else {
LAB_141b32ef1:
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f01;
    FUN_141b2c7c0(lVar18,iVar9,iVar13,0);
  }
  piVar31 = local_5f0;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f0d;
  uVar10 = FUN_142cb9230(piVar31);
  *(undefined4 *)(lVar18 + 0x184) = uVar10;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f28;
  FUN_1406e9170(param_2,&local_554,4);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f3d;
  FUN_1406e9170(param_2,&local_558,4);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f4e;
  FUN_1408414d0(local_554,local_558);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f63;
  FUN_1406e9170(param_2,&local_550,4);
  local_54c = local_550;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f7b;
  FUN_140842250(&local_54c);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f83;
  FUN_142cac0f0(piVar31);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f8b;
  uVar10 = FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32f95;
  FUN_142cb8620(piVar31,uVar10);
  uVar20 = DAT_143ac87a0;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32fa4;
  uVar10 = FUN_142cb8460(piVar31);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32fae;
  FUN_1415f5bd0(uVar20,uVar10);
  uVar20 = DAT_143ac87a0;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32fbd;
  uVar10 = FUN_142cb9230(piVar31);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32fc7;
  uVar10 = FUN_1415f6370(uVar20,uVar10);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32fd1;
  bVar6 = FUN_1406e8ae0(param_2);
  *(uint *)(lVar18 + 0x170) = (uint)bVar6;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32fe2;
  FUN_14108d290(param_2);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32fea;
  FUN_14108bdf0(param_2);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ff1;
  iVar9 = FUN_14108d9b0(uVar10);
  if (iVar9 < 0) {
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b32ffc;
    iVar13 = FUN_14108cd40();
    if (0 < iVar13) {
      iVar9 = 0;
    }
  }
  *(int *)(lVar18 + 0x128) = iVar9;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33011;
  uVar10 = FUN_14108cd40();
  *(undefined4 *)(lVar18 + 0xe8) = uVar10;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3301f;
  uVar8 = FUN_1406e8ae0(param_2);
  *(undefined1 *)(lVar18 + 0xdc) = uVar8;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3302d;
  bVar6 = FUN_1406e8ae0(param_2);
  *(uint *)(lVar18 + 0xe0) = (uint)bVar6;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3303e;
  uVar10 = FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33045;
  FUN_14108db90(uVar10);
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3304d;
  uVar8 = thunk_FUN_1406e8ae0(param_2);
  *(undefined1 *)(piVar31 + 0xd38) = uVar8;
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3305b;
  cVar7 = FUN_1406e8ae0(param_2);
  DAT_143ad2102 = cVar7 != '\0';
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3306c;
  cVar7 = FUN_1406e8ae0(param_2);
  DAT_143ad2103 = cVar7 != '\0';
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3307d;
  iVar9 = FUN_1406e8c20(param_2);
  piVar31[0xd92] = iVar9;
  piVar29 = (int *)0x0;
  iVar9 = 0;
  if (DAT_143aa84a0 != (int *)0x0) {
    iVar13 = piVar31[0xd6d];
    local_5d0 = (undefined8 *)0x0;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b330a9;
    FUN_14019a260(&local_5d0,piVar31 + 0xd6e);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b330b2;
    FUN_142dd5210(piVar31);
    if (iVar13 != 0) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b330bf;
      cVar7 = FUN_14108d4b0(iVar13);
      if (cVar7 == '\0') {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b330cc;
        iVar13 = FUN_14108d9b0(iVar13);
        puVar19 = local_5d0;
        if (-1 < iVar13) {
          *(int *)(lVar18 + 0x128) = iVar13;
          ppiVar17 = (int **)(lVar18 + 400);
          piVar31 = piVar32;
          if (local_5d0 == (undefined8 *)0x0) {
            piVar31 = *ppiVar17;
            if (piVar31 != (int *)0x0) {
              *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33177;
              FUN_14019f2c0(piVar31 + -4);
              *ppiVar17 = (int *)0x0;
            }
          }
          else {
            do {
              piVar31 = (int *)((longlong)piVar31 + 1);
            } while (*(char *)((longlong)local_5d0 + (longlong)piVar31) != '\0');
            piVar16 = *ppiVar17 + -4;
            if (*ppiVar17 == (int *)0x0) {
              piVar16 = piVar29;
            }
            iVar13 = (int)piVar31;
            if (piVar16 == (int *)0x0) {
LAB_141b331c4:
              if (iVar9 < iVar13) {
                iVar9 = iVar13;
              }
              *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b331de;
              puVar22 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
              puVar22[1] = iVar9;
              *puVar22 = 0xffffffff;
              *ppiVar17 = puVar22 + 4;
              puVar22[2] = 0;
              *(undefined1 *)*ppiVar17 = 0;
              if (piVar16 != (int *)0x0) {
                *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33206;
                FUN_14019f2c0(piVar16);
              }
            }
            else {
              if ((1 < *piVar16) || (piVar16[1] < iVar13)) {
                iVar9 = piVar16[2];
                goto LAB_141b331c4;
              }
              if (*piVar16 != 1) {
                *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b331bb;
                FUN_142e52dd0(0x74);
              }
              *piVar16 = -1;
            }
            piVar16 = (int *)0x0;
            piVar29 = *ppiVar17;
            *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33217;
            FUN_142ef7ba0(piVar29,puVar19,(longlong)iVar13);
            piVar29 = *ppiVar17;
            if (piVar29[-4] != -1) {
              *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3322c;
              FUN_142e52dd0(0x8b);
            }
            if ((iVar13 == -1) || (iVar9 = piVar29[-3], iVar13 <= iVar9)) {
              piVar29[-4] = 1;
              if (iVar13 != -1) goto LAB_141b3324c;
              piVar31 = piVar32;
              if (piVar29 != (int *)0x0) {
                do {
                  piVar16 = (int *)((longlong)piVar31 + 1);
                  piVar31 = piVar16;
                } while (*(char *)((longlong)piVar29 + (longlong)piVar16) != '\0');
              }
            }
            else {
              *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33245;
              FUN_142e54290(0x90,iVar9,(ulonglong)piVar31 & 0xffffffff);
              piVar29[-4] = 1;
LAB_141b3324c:
              *(undefined1 *)((longlong)iVar13 + (longlong)*ppiVar17) = 0;
              piVar16 = piVar31;
            }
            iVar9 = (int)piVar16;
            if ((iVar9 < 0) || (piVar29[-3] + 1 <= iVar9)) {
              *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33270;
              FUN_142e54290(0x9c,(ulonglong)piVar16 & 0xffffffff);
            }
            piVar29[-2] = iVar9;
            lVar18 = local_5e0;
          }
          piVar31 = (int *)0x0;
          if (*ppiVar17 != (int *)0x0) {
            *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3328f;
            FUN_14019bd40(ppiVar17,0,1);
            piVar29 = piVar31;
            if (*ppiVar17 != (int *)0x0) {
              piVar29 = (int *)(ulonglong)(uint)(*ppiVar17)[-2];
            }
            *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b332cb;
            FUN_14019c870(ppiVar17,piVar29);
            iVar9 = (*ppiVar17)[-2];
            piVar29 = piVar31;
            if (0 < (longlong)iVar9) {
              do {
                bVar6 = *(byte *)((longlong)*ppiVar17 + (longlong)piVar29);
                *(byte *)((longlong)*ppiVar17 + (longlong)piVar29) = bVar6 >> 4 | bVar6 << 4;
                piVar29 = (int *)((longlong)piVar29 + 1);
              } while ((longlong)piVar29 < (longlong)iVar9);
            }
          }
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3330d;
          uVar15 = FUN_1407386b0(&DAT_143ac1ab0);
          uVar26 = *(int *)(lVar18 + 0x19c) * 0x343fd + 0x269ec3;
          *(uint *)(lVar18 + 0x19c) = uVar26;
          uVar26 = uVar26 >> 0x10 & 0x7fff;
          bVar6 = (char)uVar26 + (char)(uVar26 / 0xff) + 1;
          *(byte *)(lVar18 + 0x1a0) = bVar6;
          local_5f8[0] = (uint)bVar6 * 0x1010101 ^ uVar15 & 7;
          *(uint *)(lVar18 + 0x198) = local_5f8[0];
          local_5b8 = local_5f8[0];
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33386;
          FUN_140c79130(0x43,lVar18 + 0x302a12f72c0);
          local_5f8[0] = (uint)*(byte *)(lVar18 + 0x1a0) * 0x1010101 ^ local_5f8[0];
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b333b9;
          FUN_140c78f50(0x47,lVar18 + 0x21a3f060fc2f47);
          if (*ppiVar17 != (int *)0x0) {
            *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b333d1;
            FUN_14019bd40(ppiVar17,0,1);
            piVar29 = piVar31;
            if (*ppiVar17 != (int *)0x0) {
              piVar29 = (int *)(ulonglong)(uint)(*ppiVar17)[-2];
            }
            *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b333e9;
            FUN_14019c870(ppiVar17,piVar29);
            bVar6 = (char)local_5f8[0] + (char)((int)local_5f8[0] / 7) * -7 + 1;
            iVar9 = (*ppiVar17)[-2];
            if (0 < (longlong)iVar9) {
              piVar29 = piVar31;
              do {
                bVar1 = *(byte *)((longlong)*ppiVar17 + (longlong)piVar29);
                *(byte *)((longlong)*ppiVar17 + (longlong)piVar29) =
                     bVar1 << (bVar6 & 0x1f) | bVar1 >> (8 - bVar6 & 0x1f);
                piVar29 = (int *)((longlong)piVar29 + 1);
              } while ((longlong)piVar29 < (longlong)iVar9);
            }
          }
          local_5f0 = (int *)0x0;
          piVar29 = local_5f0;
          if (((&local_5f0 != ppiVar17) &&
              (piVar16 = *ppiVar17, lVar18 = local_5e0, piVar16 != (int *)0x0)) &&
             (piVar30 = piVar16 + -4, piVar30 != (int *)0x0)) {
            if (*piVar30 == -1) {
              *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33486;
              FUN_142e52d50(0xcb,0xffffff01);
              piVar16 = *ppiVar17;
              local_5c0 = (int *)0x0;
              piVar29 = piVar31;
              piVar30 = piVar32;
              if (piVar16 != (int *)0x0) {
                do {
                  piVar30 = (int *)((longlong)piVar30 + 1);
                } while (*(char *)((longlong)piVar16 + (longlong)piVar30) != '\0');
                iVar9 = (int)piVar30;
                if (0 < iVar9) {
                  piVar29 = (int *)((ulonglong)piVar30 & 0xffffffff);
                }
                *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b334c0;
                piVar21 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)((int)piVar29 + 0x11));
                piVar21[1] = (int)piVar29;
                *piVar21 = -1;
                piVar29 = piVar21 + 4;
                piVar21[2] = 0;
                *(undefined1 *)piVar29 = 0;
                local_5c0 = piVar29;
                *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b334e6;
                FUN_142ef7ba0(piVar29,piVar16,(longlong)iVar9);
                if (*piVar21 != -1) {
                  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b334f8;
                  FUN_142e52dd0(0x8b);
                }
                if ((iVar9 == -1) || (iVar13 = piVar21[1], iVar9 <= iVar13)) {
                  *piVar21 = 1;
                  if (iVar9 != -1) goto LAB_141b33518;
                  if (piVar29 != (int *)0x0) {
                    do {
                      piVar32 = (int *)((longlong)piVar32 + 1);
                    } while (*(char *)((longlong)piVar29 + (longlong)piVar32) != '\0');
                    piVar31 = (int *)((ulonglong)piVar32 & 0xffffffff);
                  }
                }
                else {
                  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33511;
                  FUN_142e54290(0x90,iVar13,(ulonglong)piVar30 & 0xffffffff);
                  *piVar21 = 1;
LAB_141b33518:
                  *(undefined1 *)((longlong)iVar9 + (longlong)piVar29) = 0;
                  piVar31 = piVar30;
                }
                iVar9 = (int)piVar31;
                if ((iVar9 < 0) || (piVar21[1] + 1 <= iVar9)) {
                  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33539;
                  FUN_142e54290(0x9c,(ulonglong)piVar31 & 0xffffffff);
                }
                piVar21[2] = iVar9;
              }
              lVar18 = local_5e0;
              if (local_5f0 != (int *)0x0) {
                piVar32 = local_5f0 + -4;
                *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3354e;
                FUN_14019f2c0(piVar32);
                lVar18 = local_5e0;
              }
            }
            else {
              if (*piVar30 < 1) {
                *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b335a2;
                FUN_142e52dd0(0xd2);
              }
              LOCK();
              *piVar30 = *piVar30 + 1;
              UNLOCK();
              lVar18 = local_5e0;
              piVar29 = piVar16;
              if (local_5f0 != (int *)0x0) {
                piVar32 = local_5f0 + -4;
                *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b335b7;
                FUN_14019f2c0(piVar32);
                lVar18 = local_5e0;
              }
            }
          }
          local_5f0 = piVar29;
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b335ce;
          FUN_141b28570(lVar18,&local_5f0,0);
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b335e1;
          FUN_141b3f050(lVar18,4,600);
          piVar16 = local_548;
          if (local_5d0 != (undefined8 *)0x0) {
            puVar19 = local_5d0 + -2;
            *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b335f4;
            FUN_14019f2c0(puVar19);
            piVar16 = local_548;
          }
          goto LAB_141b33ea9;
        }
      }
    }
    if (local_5d0 != (undefined8 *)0x0) {
      puVar19 = local_5d0 + -2;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b330e2;
      FUN_14019f2c0(puVar19);
    }
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b330ea;
    FUN_140d22050(0);
  }
  pIVar4 = *(IUnknown **)(lVar18 + 200);
  if (pIVar4 != (IUnknown *)0x0) {
    pcVar3 = *(code **)(*(longlong *)pIVar4 + 0x2b8);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33104;
    iVar9 = (*pcVar3)(pIVar4,0);
    if (iVar9 < 0) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33119;
      _com_issue_errorex(iVar9,pIVar4,(_GUID *)&DAT_14327fcb0);
    }
  }
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3312c;
  FUN_141b3f050(lVar18,4,600);
  if ((*(char *)(lVar18 + 0xdc) == '\x01') || ((byte)(*(char *)(lVar18 + 0xdc) - 4U) < 2)) {
    uVar20 = 1;
  }
  else {
    uVar20 = 0;
  }
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33612;
  FUN_142cb83e0(DAT_143aa84a0,uVar20);
  pIVar4 = DAT_143add058;
  if ((char)piVar31[0xd38] == -1) {
    local_5e8 = (int *)0x0;
    if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      *(undefined **)((longlong)auStack_640 + lVar5) = &UNK_141b33f01;
      FUN_142ef3ac0(0x80004003);
    }
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33643;
    (*DAT_143262a20)(&local_578);
    if (DAT_143a8b8d8 == 8) {
      if (local_578 == 8) {
        local_578 = 0;
        if (lStack_570 != 0) {
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33679;
          (*DAT_143ad5990)(lStack_570 + -4);
        }
      }
      else {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33688;
        iVar9 = (*DAT_143262a18)(&local_578);
        if (iVar9 < 0) goto LAB_141b33f02;
      }
      local_578 = 8;
      piVar32 = piVar29;
      if (DAT_143a8b8e0 != 0) {
        piVar32 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b336b3;
      lStack_570 = FUN_1401a5fa0(DAT_143a8b8e0,piVar32);
    }
    else {
      if ((local_578 == 8) && (local_578 = 0, lStack_570 != 0)) {
        lVar18 = lStack_570 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3371b;
        (*DAT_143ad5990)(lVar18);
      }
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3372f;
      iVar9 = (*DAT_143262a28)(&local_578,&DAT_143a8b8d8);
      if (iVar9 < 0) {
LAB_141b33f02:
                    /* WARNING: Subroutine does not return */
        *(undefined **)((longlong)auStack_640 + lVar5) = &UNK_141b33f09;
        FUN_142ef3ac0(iVar9);
      }
    }
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b336c4;
    (*DAT_143262a20)(&local_590);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_590 == 8) {
        local_590 = (uint)local_590._2_2_ << 0x10;
        if (uStack_588 != 0) {
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b336f1;
          (*DAT_143ad5990)(uStack_588 + -4);
        }
      }
      else {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33743;
        iVar9 = (*DAT_143262a18)(&local_590);
        if (iVar9 < 0) goto LAB_141b33f0a;
      }
      local_590 = CONCAT22(local_590._2_2_,8);
      piVar32 = piVar29;
      if (DAT_143a8b8e0 != 0) {
        piVar32 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3376b;
      uStack_588 = FUN_1401a5fa0(DAT_143a8b8e0,piVar32);
    }
    else {
      if (((short)local_590 == 8) && (local_590 = (uint)local_590._2_2_ << 0x10, uStack_588 != 0)) {
        lVar18 = uStack_588 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b337c9;
        (*DAT_143ad5990)(lVar18);
      }
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b337da;
      iVar9 = (*DAT_143262a28)(&local_590,&DAT_143a8b8d8);
      if (iVar9 < 0) {
LAB_141b33f0a:
                    /* WARNING: Subroutine does not return */
        *(undefined **)((longlong)auStack_640 + lVar5) = &UNK_141b33f11;
        FUN_142ef3ac0(iVar9);
      }
    }
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33782;
    puVar19 = (undefined8 *)FUN_1401a5890(local_4e8,L"String/Consent.img/Consent");
    local_5d0 = puVar19;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33796;
    (*DAT_143262a20)(&local_528);
    pcVar3 = *(code **)(*(longlong *)pIVar4 + 0x48);
    piVar32 = piVar29;
    if ((undefined8 *)*puVar19 != (undefined8 *)0x0) {
      piVar32 = *(int **)*puVar19;
    }
    local_4d8 = CONCAT62(uStack_576,local_578);
    lStack_4d0 = lStack_570;
    local_4c8 = local_568;
    local_4b8 = local_590;
    uStack_4b4 = uStack_58c;
    uStack_4b0 = (undefined4)uStack_588;
    uStack_4ac = uStack_588._4_4_;
    local_4a8 = local_580;
    *(uint **)((longlong)local_610 + lVar5 + -8) = &local_528;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3383c;
    iVar9 = (*pcVar3)(pIVar4,piVar32,&local_4b8,&local_4d8);
    if (iVar9 < 0) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33851;
      _com_issue_errorex(iVar9,pIVar4,(_GUID *)&DAT_1432743e8);
    }
    local_508 = local_528;
    uStack_504 = uStack_524;
    uStack_500 = uStack_520;
    uStack_4fc = uStack_51c;
    local_4f8 = local_518;
    local_528 = local_528 & 0xffff0000;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3387f;
    thunk_FUN_1401be120(puVar19);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33893;
    uVar20 = FUN_1409339d0(&local_4e0,&local_508);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b338a3;
    FUN_1401a5040(&local_510,uVar20);
    if (local_4e0 != (longlong *)0x0) {
      pcVar3 = *(code **)(*local_4e0 + 0x10);
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b338b6;
      (*pcVar3)();
    }
    if ((short)local_508 == 8) {
      local_508 = local_508 & 0xffff0000;
      lVar18 = CONCAT44(uStack_4fc,uStack_500);
      if (lVar18 != 0) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b338df;
        (*DAT_143ad5990)(lVar18 + -4);
      }
    }
    else {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b338ee;
      (*DAT_143262a18)(&local_508);
    }
    if ((short)local_590 == 8) {
      local_590 = local_590 & 0xffff0000;
      if (uStack_588 != 0) {
        lVar18 = uStack_588 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3390e;
        (*DAT_143ad5990)(lVar18);
      }
    }
    else {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b3391a;
      (*DAT_143262a18)(&local_590);
    }
    if (local_578 == 8) {
      local_578 = 0;
      if (lStack_570 != 0) {
        lVar18 = lStack_570 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33943;
        (*DAT_143ad5990)(lVar18);
      }
    }
    else {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33952;
      (*DAT_143262a18)(&local_578);
    }
    local_5f8[0] = 0;
    piVar32 = piVar29;
    if (local_510 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      *(undefined **)((longlong)auStack_640 + lVar5) = &UNK_141b33f1c;
      FUN_142ef3ac0(0x80004003);
    }
    while( true ) {
      uVar15 = (uint)piVar32;
      local_5b0 = local_5b0 & 0xffffffff00000000;
      pcVar3 = *(code **)(*(longlong *)local_510 + 0x40);
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33981;
      iVar9 = (*pcVar3)(local_510,&local_5b0);
      if (iVar9 < 0) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33996;
        _com_issue_errorex(iVar9,local_510,(_GUID *)&DAT_143272478);
      }
      if ((uint)local_5b0 <= uVar15) break;
      local_5b0 = 0;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b339b7;
      puVar19 = (undefined8 *)FUN_14019ba10(&local_5b0,PTR_s_Text_02d_143a45778,piVar32);
      uVar20 = *puVar19;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b339c6;
      uVar20 = FUN_1401a5780(&local_538,uVar20);
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b339d8;
      psVar23 = (short *)FUN_1401e4330(local_510,&local_528,uVar20);
      if (*psVar23 == 8) {
        puVar28 = *(undefined2 **)(psVar23 + 4);
      }
      else {
        puVar28 = &DAT_143278568;
      }
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b339f8;
      FUN_14022d860(&local_598,puVar28);
      ppiVar17 = local_598;
      if ((local_598 != (int **)0x0) && (iVar9 = *(int *)(local_598 + -1), iVar9 != 0)) {
        if ((piVar29 == (int *)0x0) || ((char)*piVar29 == '\0')) {
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33a88;
          uVar20 = FUN_14019bd40(&local_5e8,iVar9,0);
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33a96;
          FUN_142ef7ba0(uVar20,ppiVar17);
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33aa2;
          FUN_14019c870(&local_5e8,iVar9);
          piVar29 = local_5e8;
        }
        else {
          iVar13 = piVar29[-2];
          for (iVar14 = piVar29[-3]; iVar14 < iVar13 + iVar9; iVar14 = iVar14 * 2) {
          }
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33a46;
          lVar18 = FUN_14019bd40(&local_5e8,iVar14,1);
          piVar29 = local_5e8;
          if (local_5e8 == (int *)0x0) {
            iVar14 = 0;
          }
          else {
            iVar14 = local_5e8[-2];
          }
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33a67;
          FUN_142ef7ba0(iVar14 + lVar18,ppiVar17);
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33a73;
          FUN_14019c870(&local_5e8,iVar13 + iVar9);
          uVar15 = local_5f8[0];
        }
      }
      if (ppiVar17 != (int **)0x0) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33ab7;
        FUN_14019f2c0(ppiVar17 + -2);
      }
      if ((short)local_528 == 8) {
        local_528 = local_528 & 0xffff0000;
        lVar18 = CONCAT44(uStack_51c,uStack_520);
        if (lVar18 != 0) {
          *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33ae0;
          (*DAT_143ad5990)(lVar18 + -4);
        }
      }
      else {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33aef;
        (*DAT_143262a18)(&local_528);
      }
      if (local_5b0 != 0) {
        lVar18 = local_5b0 - 0x10;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b02;
        FUN_14019f2c0(lVar18);
      }
      local_5f8[0] = uVar15 + 1;
      piVar32 = (int *)(ulonglong)local_5f8[0];
    }
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b20;
    FUN_1406ed520(local_498,0xa0);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b2f;
    FUN_1406ed840(local_498,1);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b3b;
    FUN_1415d01c0(local_498);
    *(undefined1 *)(DAT_143aa84a0 + 0xd38) = 1;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b55;
    FUN_1406ed610(local_498);
    pcVar3 = *(code **)(*(longlong *)local_510 + 0x10);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b5f;
    (*pcVar3)(local_510);
    piVar31 = local_5f0;
    if (piVar29 != (int *)0x0) {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b6e;
      FUN_14019f2c0(piVar29 + -4);
      piVar31 = local_5f0;
    }
  }
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b84;
  iVar9 = FUN_142c4f6d0(DAT_143ac1898);
  if (iVar9 != 0) {
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b94;
    uVar10 = FUN_142cb8460(piVar31);
    piVar32 = local_5f0;
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33b9f;
    uVar20 = FUN_142cb8480(piVar32);
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33ba7;
    cVar7 = FUN_14057e4c0();
    if (cVar7 != '\0') {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33bb4;
      uVar24 = FUN_1404c6160();
      local_598 = &local_540;
      local_560 = (int *)0x0;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33bdf;
      uVar20 = FUN_14019ba10(&local_560,&DAT_143274450,uVar20);
      local_540 = (int *)0x0;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33bf5;
      FUN_14019a260(&local_540,uVar20);
      local_5d8 = (int *)0x0;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33c0a;
      pcVar25 = (char *)FUN_14019bd40(&local_5d8,7);
      piVar32 = local_5d8;
      *(undefined4 *)pcVar25 = s_nexonsn_14328e4a8._0_4_;
      *(undefined2 *)(pcVar25 + 4) = s_nexonsn_14328e4a8._4_2_;
      pcVar25[6] = s_nexonsn_14328e4a8[6];
      if (local_5d8[-4] != -1) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33c3d;
        FUN_142e52dd0(0x8b);
      }
      iVar9 = piVar32[-3];
      if (iVar9 < 7) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33c55;
        FUN_142e54290(0x90,iVar9,7);
      }
      piVar32[-4] = 1;
      *(undefined1 *)((longlong)local_5d8 + 7) = 0;
      if (piVar32[-3] + 1 < 8) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33c80;
        FUN_142e54290(0x9c,7);
      }
      piVar32[-2] = 7;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33c9c;
      FUN_1404c7490(uVar24,0,&local_5d8,&local_540);
      if (local_5d8 != (int *)0x0) {
        piVar32 = local_5d8 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33caf;
        FUN_14019f2c0(piVar32);
      }
      if (local_560 != (int *)0x0) {
        piVar32 = local_560 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33cc5;
        FUN_14019f2c0(piVar32);
      }
    }
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33ccb;
    cVar7 = FUN_14057e4c0();
    if (cVar7 != '\0') {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33cd8;
      uVar20 = FUN_1404c6160();
      local_598 = &local_5a8;
      local_5c0 = (int *)0x0;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33cfa;
      uVar24 = FUN_14019ba10(&local_5c0,&DAT_143274450,uVar10);
      local_5a8 = (int *)0x0;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33d0a;
      FUN_14019a260(&local_5a8,uVar24);
      local_5e0 = 0;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33d1f;
      pcVar25 = (char *)FUN_14019bd40(&local_5e0,9);
      lVar18 = local_5e0;
      *(undefined8 *)pcVar25 = s_accountno_14328e448._0_8_;
      pcVar25[8] = s_accountno_14328e448[8];
      if (*(int *)(local_5e0 + -0x10) != -1) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33d4b;
        FUN_142e52dd0(0x8b);
      }
      iVar9 = *(int *)(lVar18 + -0xc);
      if (iVar9 < 9) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33d63;
        FUN_142e54290(0x90,iVar9,9);
      }
      *(undefined4 *)(lVar18 + -0x10) = 1;
      *(undefined1 *)(local_5e0 + 9) = 0;
      if (*(int *)(lVar18 + -0xc) + 1 < 10) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33d8e;
        FUN_142e54290(0x9c,9);
      }
      *(undefined4 *)(lVar18 + -8) = 9;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33da7;
      FUN_1404c7490(uVar20,0,&local_5e0,&local_5a8);
      if (local_5e0 != 0) {
        lVar18 = local_5e0 + -0x10;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33dba;
        FUN_14019f2c0(lVar18);
      }
      if (local_5c0 != (int *)0x0) {
        piVar32 = local_5c0 + -4;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33dcd;
        FUN_14019f2c0(piVar32);
      }
    }
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33dd3;
    cVar7 = FUN_14057e4c0();
    if (cVar7 != '\0') {
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33de0;
      uVar20 = FUN_1404c6160();
      local_5c8 = 0;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33df7;
      pcVar25 = (char *)FUN_14019bd40(&local_5c8,0xe);
      lVar18 = local_5c8;
      *(undefined8 *)pcVar25 = s_GC_SelectWorld_1433fe310._0_8_;
      *(undefined4 *)(pcVar25 + 8) = s_GC_SelectWorld_1433fe310._8_4_;
      *(undefined2 *)(pcVar25 + 0xc) = s_GC_SelectWorld_1433fe310._12_2_;
      if (*(int *)(local_5c8 + -0x10) != -1) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33e2d;
        FUN_142e52dd0(0x8b);
      }
      iVar9 = *(int *)(lVar18 + -0xc);
      if (iVar9 < 0xe) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33e45;
        FUN_142e54290(0x90,iVar9,0xe);
      }
      *(undefined4 *)(lVar18 + -0x10) = 1;
      *(undefined1 *)(local_5c8 + 0xe) = 0;
      if (*(int *)(lVar18 + -0xc) + 1 < 0xf) {
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33e70;
        FUN_142e54290(0x9c,0xe);
      }
      *(undefined4 *)(lVar18 + -8) = 0xe;
      *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33e88;
      FUN_1404c7800(uVar20,0xd,&local_5c8);
      if (local_5c8 != 0) {
        lVar18 = local_5c8 + -0x10;
        *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33e9b;
        FUN_14019f2c0(lVar18);
      }
    }
  }
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33ea8;
  FUN_141177e40(DAT_143aca790);
LAB_141b33ea9:
  if (piVar16 != (int *)0x0) {
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33eb7;
    FUN_1401bebb0(piVar16 + -4);
  }
  if (local_530 != 0) {
    *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33ecd;
    FUN_14019f2c0(local_530 + -0x10);
  }
  *(undefined8 *)((longlong)auStack_640 + lVar5) = 0x141b33edd;
  return;
}



//===========================================================
// FUN_141b3a2f0 @ 141b3a2f0   (528 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b3a2f0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  char cVar5;
  int iVar6;
  int *piVar7;
  longlong *plVar8;
  undefined1 auStack_498 [32];
  wchar_t *local_478;
  longlong local_470;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  cVar5 = FUN_1406e8ae0(param_2);
  *(undefined4 *)(param_1 + 0xd4) = 0;
  if (cVar5 == '\0') {
    *(undefined4 *)(param_1 + 0xec) = 0xbb9;
    FUN_141b3a560(param_1);
  }
  else {
    FUN_1406e9050(param_2,&local_470);
    FUN_1429e5410(local_470,0,0);
    local_478 = (wchar_t *)0x0;
    piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x46);
    piVar7[1] = 0x1a;
    *piVar7 = -1;
    local_478 = (wchar_t *)(piVar7 + 4);
    piVar7[2] = 0;
    *local_478 = L'\0';
    uVar1 = u_confirmPlayerAuthCompleted_1433feb10._8_8_;
    *(undefined8 *)local_478 = u_confirmPlayerAuthCompleted_1433feb10._0_8_;
    *(undefined8 *)(piVar7 + 6) = uVar1;
    uVar4 = u_confirmPlayerAuthCompleted_1433feb10._28_4_;
    uVar3 = u_confirmPlayerAuthCompleted_1433feb10._24_4_;
    uVar2 = u_confirmPlayerAuthCompleted_1433feb10._20_4_;
    piVar7[8] = u_confirmPlayerAuthCompleted_1433feb10._16_4_;
    piVar7[9] = uVar2;
    piVar7[10] = uVar3;
    piVar7[0xb] = uVar4;
    uVar4 = u_confirmPlayerAuthCompleted_1433feb10._44_4_;
    uVar3 = u_confirmPlayerAuthCompleted_1433feb10._40_4_;
    uVar2 = u_confirmPlayerAuthCompleted_1433feb10._36_4_;
    piVar7[0xc] = u_confirmPlayerAuthCompleted_1433feb10._32_4_;
    piVar7[0xd] = uVar2;
    piVar7[0xe] = uVar3;
    piVar7[0xf] = uVar4;
    piVar7[0x10] = u_confirmPlayerAuthCompleted_1433feb10._48_4_;
    if (*piVar7 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar7[1] < 0x1a) {
      FUN_142e54290(0x90,piVar7[1],0x1a);
    }
    *piVar7 = 1;
    local_478[0x1a] = L'\0';
    if (piVar7[1] + 1 < 0x1b) {
      FUN_142e54290(0x9c,0x1a);
    }
    piVar7[2] = 0x34;
    iVar6 = FUN_141b4c230(&local_478,0);
    plVar8 = *(longlong **)(param_1 + 0x148);
    if (plVar8 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar8 = *(longlong **)(param_1 + 0x148);
    }
    (**(code **)(*plVar8 + 0x138))(plVar8,3000);
    FUN_1406ed520(local_468,0xb3);
    if (iVar6 == 1) {
      FUN_1406ed840(local_468,1);
      FUN_1415d01c0(local_468);
      *(undefined4 *)(param_1 + 0xd4) = 1;
    }
    else {
      FUN_1406ed840(local_468,0);
      FUN_1415d01c0(local_468);
      *(undefined4 *)(param_1 + 0xd4) = 1;
    }
    FUN_1406ed610(local_468);
    if (local_470 != 0) {
      FUN_14019f2c0(local_470 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_141b3c090 @ 141b3c090   (122 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b3c090(longlong param_1)

{
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  if (*(int *)(param_1 + 0xd4) == 0) {
    FUN_1406ed520(local_468,0xb6);
    FUN_1406ed940(local_468,0);
    FUN_1415d01c0(local_468);
    *(undefined4 *)(param_1 + 0xd4) = 1;
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_141b3c110 @ 141b3c110   (158 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b3c110(longlong param_1)

{
  undefined4 uVar1;
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  uVar1 = FUN_1420a3630(&DAT_143ad53e0);
  *(undefined4 *)(param_1 + 0x1d8) = uVar1;
  *(undefined4 *)(param_1 + 0x1dc) = 0;
  FUN_141b45370(param_1 + 0x1d0);
  FUN_1406ed520(local_468,0xbb);
  FUN_1406ed9d0(local_468,uVar1);
  FUN_1415d01c0(local_468);
  FUN_1406ed610(local_468);
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
// FUN_141b0ecc0 @ 141b0ecc0   (303 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b0ecc0(undefined4 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  FUN_1406ed520(local_468,0x79);
  FUN_1406ed9d0(local_468,param_1);
  FUN_1406edc80(local_468,param_2);
  uVar1 = FUN_141b0ec20(0);
  FUN_1406ed9d0(local_468,uVar1);
  uVar1 = FUN_141b0ec20(1);
  FUN_1406ed9d0(local_468,uVar1);
  uVar1 = FUN_141b0ec20(2);
  FUN_1406ed9d0(local_468,uVar1);
  uVar1 = FUN_141b0ec20(3);
  FUN_1406ed9d0(local_468,uVar1);
  FUN_1406ed9d0(local_468,DAT_143ad1ef4 - DAT_143ad1ef0);
  FUN_1406ede20(local_468,&DAT_143ad1ed0,0x10);
  FUN_1406ede20(local_468,&DAT_143ad1ee0,0x10);
  FUN_1415d01c0(local_468);
  FUN_1406ed610(local_468);
  FUN_140199470(param_2);
  return;
}



//===========================================================
// FUN_141b2dc40 @ 141b2dc40   (158 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b2dc40(undefined8 param_1,undefined4 param_2,longlong *param_3)

{
  int iVar1;
  undefined4 uVar2;
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  iVar1 = FUN_140859e30();
  if (iVar1 != 0) {
    if (*param_3 == 0) {
      uVar2 = 0;
    }
    else {
      uVar2 = FUN_142f11a94();
    }
    FUN_1406ed520(local_468,4000);
    FUN_1406ed9d0(local_468,param_2);
    FUN_1406ed9d0(local_468,uVar2);
    FUN_1415d01c0(local_468);
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_1402cb0d0 @ 1402cb0d0   (280 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1402cb0d0(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  uint uVar2;
  int iVar3;
  ulonglong uVar4;
  undefined1 auStack_68 [32];
  byte local_48;
  byte local_47;
  int local_44;
  uint local_40;
  uint local_3c;
  int local_38;
  ulonglong local_30;
  
  local_30 = DAT_143a8b908 ^ (ulonglong)auStack_68;
  FUN_1402fe400();
  iVar3 = 0;
  bVar1 = FUN_1406e8ae0(param_2);
  if (bVar1 != 0) {
    uVar4 = (ulonglong)bVar1;
    do {
      bVar1 = FUN_1406e8ae0(param_2);
      uVar2 = FUN_1406e8c20(param_2);
      FUN_1407386b0(&DAT_143ac1ab0);
      FUN_1407386b0(&DAT_143ac1ab0);
      local_48 = FUN_1407386b0(&DAT_143ac1ab0);
      local_47 = local_48 ^ bVar1;
      local_44 = ((local_48 ^ 0xbaadf00d) >> 5 | (local_48 ^ 0xbaadf00d) << 0x1b) + (uint)local_47;
      local_40 = FUN_1407386b0(&DAT_143ac1ab0);
      local_3c = (local_40 ^ uVar2) >> 5 | (local_40 ^ uVar2) << 0x1b;
      local_38 = ((local_40 ^ 0xbaadf00d) >> 5 | (local_40 ^ 0xbaadf00d) << 0x1b) + local_3c;
      FUN_140300b60(param_1,&local_48);
      iVar3 = iVar3 + uVar2;
      uVar4 = uVar4 - 1;
    } while (uVar4 != 0);
  }
  *(int *)(param_1 + 0x18) = iVar3;
  return;
}



//===========================================================
// FUN_1406e8b80 @ 1406e8b80   (146 bytes)
//===========================================================

undefined2 FUN_1406e8b80(longlong param_1)

{
  undefined2 uVar1;
  int iVar2;
  int iVar3;
  longlong lVar4;
  undefined1 local_30 [40];
  
  iVar2 = *(int *)(param_1 + 0x18);
  iVar3 = *(int *)(param_1 + 0x24);
  lVar4 = *(longlong *)(param_1 + 0x10);
  if (lVar4 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar4 = *(longlong *)(param_1 + 0x10);
    if (lVar4 != 0) goto LAB_1406e8bbb;
  }
  else {
LAB_1406e8bbb:
    if (*(int *)(lVar4 + -8) != 0) goto LAB_1406e8bd0;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e8bd0:
  if ((uint)(iVar2 - iVar3) < 2) {
    FUN_1401bb8b0(local_30,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_30,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar1 = *(undefined2 *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) + 2;
  return uVar1;
}



//===========================================================
// FUN_1402f7010 @ 1402f7010   (131 bytes)
//===========================================================

uint FUN_1402f7010(undefined2 param_1,longlong param_2)

{
  byte bVar1;
  byte bVar2;
  longlong lVar3;
  byte *pbVar4;
  uint uVar5;
  undefined2 local_res8 [4];
  
  uVar5 = 0xbaadf00d;
  pbVar4 = (byte *)local_res8;
  lVar3 = 2;
  local_res8[0] = param_1;
  do {
    bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
    bVar1 = *pbVar4;
    pbVar4[param_2 - (longlong)local_res8] = bVar2;
    pbVar4[(param_2 + 2) - (longlong)local_res8] = bVar2 ^ bVar1;
    pbVar4 = pbVar4 + 1;
    uVar5 = ((uVar5 ^ bVar2) >> 5 | (uVar5 ^ bVar2) << 0x1b) + (uint)(bVar2 ^ bVar1);
    lVar3 = lVar3 + -1;
  } while (lVar3 != 0);
  return uVar5;
}



//===========================================================
// FUN_1402f7170 @ 1402f7170   (130 bytes)
//===========================================================

uint FUN_1402f7170(undefined8 param_1,longlong param_2)

{
  uint uVar1;
  uint uVar2;
  longlong lVar3;
  uint *puVar4;
  uint uVar5;
  undefined8 local_res8;
  
  uVar5 = 0xbaadf00d;
  lVar3 = 2;
  puVar4 = (uint *)&local_res8;
  local_res8 = param_1;
  do {
    uVar1 = FUN_1407386b0(&DAT_143ac1ab0);
    *(uint *)((param_2 - (longlong)&local_res8) + (longlong)puVar4) = uVar1;
    uVar2 = (uVar1 ^ *puVar4) >> 5 | (uVar1 ^ *puVar4) << 0x1b;
    uVar5 = ((uVar5 ^ uVar1) >> 5 | (uVar5 ^ uVar1) << 0x1b) + uVar2;
    *(uint *)((param_2 - (longlong)&local_res8) + 4 + (longlong)(puVar4 + 1)) = uVar2;
    lVar3 = lVar3 + -1;
    puVar4 = puVar4 + 1;
  } while (lVar3 != 0);
  return uVar5;
}



//===========================================================
// FUN_1401ab420 @ 1401ab420   (179 bytes)
//===========================================================

undefined2 FUN_1401ab420(byte *param_1,int param_2)

{
  undefined8 *puVar1;
  uint uVar2;
  undefined2 local_res8;
  int local_res10 [2];
  int local_res18 [2];
  undefined8 local_res20;
  longlong local_18 [3];
  
  local_res8 = CONCAT11(param_1[3] ^ param_1[1],param_1[2] ^ *param_1);
  uVar2 = *param_1 ^ 0xbaadf00d;
  uVar2 = (uVar2 >> 5 | uVar2 << 0x1b) + (uint)param_1[2] ^ (uint)param_1[1];
  local_res18[0] = (uVar2 >> 5 | uVar2 << 0x1b) + (uint)param_1[3];
  if (local_res18[0] != param_2) {
    local_res10[0] = param_2;
    local_res20 = FUN_1418039d0(5);
    puVar1 = (undefined8 *)FUN_1401a0ed0(local_18,&local_res20,local_res18,local_res10);
    FUN_141804970(&DAT_143271f04,0x53,5,*puVar1);
    if (local_18[0] != 0) {
      FUN_14019f2c0(local_18[0] + -0x10);
    }
  }
  return local_res8;
}



//===========================================================
// FUN_14108d290 @ 14108d290   (443 bytes)
//===========================================================

void FUN_14108d290(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  int iVar6;
  uint uVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  int iVar10;
  bool bVar11;
  undefined8 local_res18;
  undefined8 local_res20;
  undefined8 *puVar12;
  undefined8 *local_58;
  uint uStack_50;
  
  puVar9 = DAT_143ac9868;
  cVar1 = *(char *)((longlong)DAT_143ac9868[1] + 0x19);
  plVar3 = (longlong *)DAT_143ac9868[1];
  while (cVar1 == '\0') {
    FUN_14108e6e0(&DAT_143ac9868,&DAT_143ac9868,plVar3[2]);
    plVar2 = (longlong *)*plVar3;
    thunk_FUN_140205820(plVar3,0x28);
    plVar3 = plVar2;
    cVar1 = *(char *)((longlong)plVar2 + 0x19);
  }
  puVar9[1] = puVar9;
  *puVar9 = puVar9;
  puVar9[2] = puVar9;
  iVar10 = 0;
  DAT_143ac9870 = 0;
  iVar6 = FUN_1406e8c20(param_1);
  if (0 < iVar6) {
    do {
      uVar7 = FUN_1406e8c20(param_1);
      local_res18 = DAT_143379de0;
      FUN_1406e9170(param_1,&local_res18,8);
      local_res20 = local_res18;
      uVar8 = FUN_1408f63b0(&local_res20,1);
      puVar5 = DAT_143ac9868;
      puVar9 = (undefined8 *)DAT_143ac9868[1];
      uStack_50 = 0;
      cVar1 = *(char *)((longlong)puVar9 + 0x19);
      local_58 = puVar9;
      puVar12 = DAT_143ac9868;
      while (puVar4 = puVar9, cVar1 == '\0') {
        bVar11 = uVar7 <= *(uint *)((longlong)puVar4 + 0x1c);
        if (bVar11) {
          puVar9 = (undefined8 *)*puVar4;
          puVar12 = puVar4;
        }
        else {
          puVar9 = (undefined8 *)puVar4[2];
        }
        uStack_50 = (uint)bVar11;
        cVar1 = *(char *)((longlong)puVar9 + 0x19);
        local_58 = puVar4;
      }
      if ((*(char *)((longlong)puVar12 + 0x19) != '\0') ||
         (uVar7 < *(uint *)((longlong)puVar12 + 0x1c))) {
        if (DAT_143ac9870 == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
          FUN_14019f9d0();
        }
        puVar12 = &DAT_143ac9868;
        puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x28);
        *(uint *)((longlong)puVar9 + 0x1c) = uVar7;
        puVar9[4] = 0;
        *puVar9 = puVar5;
        puVar9[1] = puVar5;
        puVar9[2] = puVar5;
        *(undefined2 *)(puVar9 + 3) = 0;
        puVar12 = (undefined8 *)FUN_141090030(&DAT_143ac9868,&local_58,puVar9,param_4,puVar12,0);
      }
      puVar12[4] = uVar8;
      iVar10 = iVar10 + 1;
    } while (iVar10 < iVar6);
  }
  return;
}


