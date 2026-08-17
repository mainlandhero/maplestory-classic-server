
//===========================================================
// FUN_141b2d860 @ 141b2d860   (456 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b2d860(longlong param_1)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int iVar5;
  int *piVar6;
  int *piVar7;
  undefined1 auStack_4a8 [32];
  wchar_t *local_488 [2];
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  if (*(int *)(param_1 + 0xd4) == 0) {
    if ((-1 < *(int *)(param_1 + 0x128)) && (*(int *)(param_1 + 0x128) < *(int *)(param_1 + 0xe8)))
    {
      piVar6 = (int *)FUN_14108cc50();
      if (*piVar6 != 0) {
        FUN_140da2650(param_1 + 0x140);
        local_488[0] = (wchar_t *)0x0;
        piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x54);
        piVar7[1] = 0x21;
        *piVar7 = -1;
        local_488[0] = (wchar_t *)(piVar7 + 4);
        piVar7[2] = 0;
        *local_488[0] = L'\0';
        uVar1 = u_confirmDeleteCharacterPermanentl_1433fe010._8_8_;
        *(undefined8 *)local_488[0] = u_confirmDeleteCharacterPermanentl_1433fe010._0_8_;
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
        local_488[0][0x21] = L'\0';
        if (piVar7[1] + 1 < 0x22) {
          FUN_142e54290(0x9c);
        }
        piVar7[2] = 0x42;
        iVar5 = FUN_141b4a290(local_488,param_1 + 0x140);
        if (iVar5 == 0) {
          FUN_140da2650(param_1 + 0x140);
        }
        else {
          FUN_1406ed520(local_478,0x8c);
          FUN_1406ed9d0(local_478,*piVar6);
          FUN_1415d01c0(local_478);
          *(undefined4 *)(param_1 + 0xd4) = 1;
          FUN_1406ed610(local_478);
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_141b2da30 @ 141b2da30   (421 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b2da30(longlong param_1)

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
        piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x4a);
        piVar7[1] = 0x1c;
        *piVar7 = -1;
        local_478[0] = (wchar_t *)(piVar7 + 4);
        piVar7[2] = 0;
        *local_478[0] = L'\0';
        uVar1 = u_confirmCancelDeleteCharacter_1433fe748._8_8_;
        *(undefined8 *)local_478[0] = u_confirmCancelDeleteCharacter_1433fe748._0_8_;
        *(undefined8 *)(piVar7 + 6) = uVar1;
        uVar1 = u_confirmCancelDeleteCharacter_1433fe748._24_8_;
        *(undefined8 *)(piVar7 + 8) = u_confirmCancelDeleteCharacter_1433fe748._16_8_;
        *(undefined8 *)(piVar7 + 10) = uVar1;
        uVar4 = u_confirmCancelDeleteCharacter_1433fe748._44_4_;
        uVar3 = u_confirmCancelDeleteCharacter_1433fe748._40_4_;
        uVar2 = u_confirmCancelDeleteCharacter_1433fe748._36_4_;
        piVar7[0xc] = u_confirmCancelDeleteCharacter_1433fe748._32_4_;
        piVar7[0xd] = uVar2;
        piVar7[0xe] = uVar3;
        piVar7[0xf] = uVar4;
        *(undefined8 *)(piVar7 + 0x10) = u_confirmCancelDeleteCharacter_1433fe748._48_8_;
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar7[1] < 0x1c) {
          FUN_142e54290(0x90,piVar7[1],0x1c);
        }
        *piVar7 = 1;
        local_478[0][0x1c] = L'\0';
        if (piVar7[1] + 1 < 0x1d) {
          FUN_142e54290(0x9c,0x1c);
        }
        piVar7[2] = 0x38;
        iVar5 = FUN_141b4a290(local_478,param_1 + 0x140);
        if (iVar5 != 0) {
          FUN_1406ed520(local_468,0x8d);
          FUN_1406ed9d0(local_468,*piVar6);
          FUN_1415d01c0(local_468);
          *(undefined4 *)(param_1 + 0xd4) = 1;
          FUN_1406ed610(local_468);
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_141b3bfd0 @ 141b3bfd0   (162 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b3bfd0(longlong param_1)

{
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  if (((2 < *(int *)(param_1 + 0xd0)) && (*(int *)(param_1 + 0x238) == 0)) &&
     ((DAT_143ac18a0 == 0 || (*(int *)(DAT_143ac18a0 + 0x48) != 0)))) {
    FUN_142cf4350(DAT_143aa84a0);
    FUN_141b45280(param_1 + 0x100);
    FUN_1406ed520(local_468,0x82);
    FUN_1415d01c0(local_468);
    FUN_142aa27f0(1);
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_141b2b930 @ 141b2b930   (290 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b2b930(longlong param_1,undefined4 param_2)

{
  char cVar1;
  undefined1 uVar2;
  undefined8 uVar3;
  undefined1 auStack_4a8 [32];
  longlong local_488;
  longlong local_480;
  code *local_478;
  undefined8 local_470;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  if (*(int *)(param_1 + 0xd4) == 0) {
    cVar1 = FUN_141087de0();
    if (cVar1 != '\0') {
      uVar3 = FUN_141b44080();
      FUN_142bf3f70(uVar3);
      local_488 = FUN_141b44080();
      if (local_488 == 0) {
        local_470 = 0;
      }
      else {
        local_480 = local_488 + 8;
        local_478 = (code *)**(undefined8 **)(local_488 + 8);
        local_470 = (*local_478)(local_480,1);
      }
    }
    FUN_1406ed520(local_468,0x75);
    uVar2 = FUN_140d21150();
    FUN_1406ed840(local_468,uVar2);
    FUN_1406ed9d0(local_468,param_2);
    uVar3 = FUN_140caa510();
    uVar2 = FUN_142cf42c0(uVar3);
    FUN_1406ed840(local_468,uVar2);
    FUN_141b2a250(param_1,local_468);
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_141b2a660 @ 141b2a660   (1301 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined4 FUN_141b2a660(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  byte bVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined1 *puVar6;
  undefined1 auStack_d48 [32];
  undefined4 local_d28;
  int local_d18;
  undefined4 local_d14;
  uint local_d10;
  undefined4 local_d0c;
  undefined4 local_d08;
  undefined4 local_d04;
  undefined4 local_d00;
  undefined4 local_cfc;
  undefined1 local_cf8 [8];
  longlong *local_cf0;
  undefined8 local_ce8;
  undefined8 local_ce0;
  undefined8 local_cd8;
  undefined8 local_cd0;
  undefined1 local_cc8 [1104];
  undefined1 local_878 [1104];
  undefined1 local_428 [16];
  undefined1 local_418 [1024];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_d48;
  if (*(int *)(param_1 + 0xd4) == 0) {
    *(undefined4 *)(param_1 + 0xd4) = 1;
    FUN_141b45280(param_1 + 0x100);
    FUN_141b45190(param_1 + 0x178);
    local_d10 = 1;
    cVar1 = FUN_1415e4010();
    if (cVar1 != '\0') {
      uVar4 = FUN_140caa4d0();
      bVar2 = FUN_1415dafd0(uVar4);
      local_d10 = (uint)bVar2;
    }
    if (local_d10 == 0) {
      local_d18 = 0;
    }
    else {
      local_ce8 = FUN_140c2f770(param_3);
      local_ce0 = FUN_140c2f770(param_2);
      local_d28 = 0;
      local_d18 = FUN_141d60eb0(local_ce0,local_ce8,0xc9,0);
    }
    if (local_d18 == 0) {
      FUN_141d60fa0(local_418);
    }
    else {
      local_d14 = 9;
      if ((local_d18 == 20000) || (local_d18 == 0x4e35)) {
        local_d14 = 0xc;
      }
      else if ((local_d18 == 0x4e22) || (local_d18 == 0x4e3e)) {
        local_d14 = 8;
      }
      else if (local_d18 == 0x4e23) {
        local_d14 = 0x88;
      }
      else if (local_d18 == 0x4e26) {
        local_d14 = 0xe;
      }
      else if (local_d18 == 0x4e27) {
        local_d14 = 2;
      }
      else if (local_d18 == 0x4e3a) {
        local_d14 = 4;
      }
      else if (local_d18 == 0x4e39) {
        local_d14 = 0xf;
      }
      else if ((local_d18 == 0x4e3b) || (local_d18 == 0x4e3c)) {
        local_d14 = 0x87;
      }
      else if (local_d18 == 0x4e45) {
        local_d14 = 7;
      }
      else if (local_d18 == 0x4e6c) {
        local_d14 = 0x44;
      }
      *(undefined4 *)(param_1 + 0xd4) = 0;
      FUN_1406ed520(local_878,0xc0);
      uVar4 = FUN_1415e3d00();
      local_d08 = FUN_142c4a810(uVar4);
      FUN_141b41d00(local_878,&local_d08);
      FUN_1406ed9d0(local_878,local_d18);
      FUN_1415d01c0(local_878);
      cVar1 = FUN_141b2a280(param_1,local_d14,0);
      if (cVar1 == '\0') {
        local_d04 = 0;
        FUN_1406ed610(local_878);
        FUN_140199470(param_2);
        FUN_140199470(param_3);
        return local_d04;
      }
      FUN_1406ed610(local_878);
    }
    FUN_140d21fb0(param_2);
    FUN_140d21fc0(param_2);
    FUN_140d21fe0(param_3);
    FUN_1406ed520(local_cc8,0x74);
    FUN_1406ed840(local_cc8,(undefined1)local_d10);
    FUN_1406edc80(local_cc8,param_3);
    if (local_d10 == 0) {
      FUN_1406edc80(local_cc8,param_2);
    }
    else {
      FUN_140196ed0(local_cf8,local_418,0xffffffff);
      FUN_1406edc80(local_cc8,local_cf8);
      FUN_140199470(local_cf8);
    }
    puVar6 = local_428;
    for (lVar5 = 0x10; lVar5 != 0; lVar5 = lVar5 + -1) {
      *puVar6 = 0;
      puVar6 = puVar6 + 1;
    }
    (*DAT_143262960)(local_428);
    FUN_1406ede20(local_cc8,local_428,0x10);
    uVar3 = (*DAT_143262958)();
    FUN_1406ed9d0(local_cc8,uVar3);
    uVar4 = FUN_1415e3d00();
    local_d00 = FUN_142c4a810(uVar4);
    FUN_141b41d00(local_cc8,&local_d00);
    FUN_1406ed840(local_cc8,0);
    FUN_1406ed840(local_cc8,0);
    uVar4 = FUN_1415e3cc0();
    uVar3 = FUN_141600f70(uVar4);
    FUN_1406ed9d0(local_cc8,uVar3);
    FUN_1415d01c0(local_cc8);
    local_cd0 = FUN_140caa510();
    local_cd8 = FUN_140c2f770(param_2);
    FUN_142cb6000(local_cd0,local_cd8);
    cVar1 = FUN_141b44830();
    if (cVar1 != '\0') {
      lVar5 = FUN_141b44070();
      local_cf0 = (longlong *)(lVar5 + 8);
      (**(code **)(*local_cf0 + 0xa0))(local_cf0);
    }
    local_cfc = 1;
    FUN_1406ed610(local_cc8);
    FUN_140199470(param_2);
    FUN_140199470(param_3);
    local_d0c = local_cfc;
  }
  else {
    local_d0c = 0;
    FUN_140199470(param_2);
    FUN_140199470(param_3);
  }
  return local_d0c;
}


