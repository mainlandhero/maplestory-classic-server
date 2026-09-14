
//===========================================================
// FUN_1416ed1a0 @ 1416ed1a0   (126 bytes)
//===========================================================

undefined8 * FUN_1416ed1a0(undefined8 *param_1)

{
  FUN_14170fd10();
  *(undefined2 *)((longlong)param_1 + 0x7c) = 0;
  *param_1 = &PTR_FUN_1433ccdc0;
  param_1[1] = &PTR_LAB_1433cce60;
  param_1[3] = &PTR_FUN_1433ccf38;
  param_1[0x1c] = 0;
  *(undefined4 *)(param_1 + 0xb) = 0;
  *(undefined4 *)(param_1 + 0x1a) = 0;
  *(undefined2 *)((longlong)param_1 + 0xd4) = 0;
  param_1[0x15] = 0;
  param_1[0x20] = 0;
  *(undefined4 *)(param_1 + 0xf) = 0xffffffff;
  *(undefined4 *)(param_1 + 0x1b) = 0xffffffff;
  return param_1;
}



//===========================================================
// FUN_1416ee330 @ 1416ee330   (65 bytes)
//===========================================================

void FUN_1416ee330(longlong param_1,int param_2)

{
  *(int *)(param_1 + 0x84) = param_2;
  if (param_2 < 2) {
    *(undefined8 *)(param_1 + 0x80) = 0;
    FUN_141710f60();
    return;
  }
  if (*(int *)(param_1 + 0x80) < 0) {
    *(undefined4 *)(param_1 + 0x80) = 0;
    FUN_141710f60();
    return;
  }
  if (param_2 + -1 < *(int *)(param_1 + 0x80)) {
    *(int *)(param_1 + 0x80) = param_2 + -1;
  }
  FUN_141710f60();
  return;
}



//===========================================================
// FUN_14170fd10 @ 14170fd10   (194 bytes)
//===========================================================

undefined8 * FUN_14170fd10(undefined8 *param_1)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined1 local_28 [32];
  
  *param_1 = &PTR_LAB_143296d40;
  uVar2 = FUN_141d5d8c0(local_28,&DAT_143271f04,&DAT_143271f04,0x1c,0);
  FUN_141d5f4b0(param_1 + 1,uVar2);
  param_1[6] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  *param_1 = &PTR_FUN_1433ceb40;
  param_1[1] = &PTR_LAB_1433cebc0;
  param_1[3] = &PTR_FUN_1433cec98;
  *(undefined4 *)(param_1 + 7) = 0xffffffff;
  uVar1 = FUN_1429e3ef0();
  *(undefined4 *)((longlong)param_1 + 0x3c) = uVar1;
  param_1[8] = 0;
  param_1[10] = 0;
  *(undefined4 *)(param_1 + 0xb) = 1;
  *(undefined4 *)((longlong)param_1 + 0x5c) = 1;
  *(undefined4 *)(param_1 + 0xc) = 1;
  param_1[0xd] = 0;
  *(undefined4 *)(param_1 + 0xe) = 0;
  return param_1;
}



//===========================================================
// FUN_141710f60 @ 141710f60   (77 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141710f60(longlong *param_1)

{
  longlong *plVar1;
  undefined1 auStack_48 [32];
  undefined1 local_28 [16];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_48;
  (**(code **)(*param_1 + 0x38))(param_1,local_28);
  plVar1 = (longlong *)param_1[10];
  if (plVar1 != (longlong *)0x0) {
    (**(code **)(*plVar1 + 0x90))(plVar1,local_28);
  }
  return;
}



//===========================================================
// FUN_1429e3ef0 @ 1429e3ef0   (12 bytes)
//===========================================================

void FUN_1429e3ef0(void)

{
  FUN_142c4a030(DAT_143ac1898);
  return;
}



//===========================================================
// FUN_141d5d8c0 @ 141d5d8c0   (22 bytes)
//===========================================================

undefined8 *
FUN_141d5d8c0(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined4 param_4,
             undefined4 param_5)

{
  *(undefined4 *)((longlong)param_1 + 0x14) = param_5;
  *param_1 = param_2;
  param_1[1] = param_3;
  *(undefined4 *)(param_1 + 2) = param_4;
  return param_1;
}



//===========================================================
// FUN_141d5f4b0 @ 141d5f4b0   (82 bytes)
//===========================================================

undefined8 * FUN_141d5f4b0(undefined8 *param_1)

{
  int iVar1;
  longlong lVar2;
  
  *(undefined4 *)(param_1 + 1) = 0;
  *param_1 = &PTR_FUN_143409768;
  do {
    do {
      iVar1 = FUN_140738830();
    } while (iVar1 == 0);
    lVar2 = FUN_141d5f870(iVar1);
  } while (lVar2 != 0);
  FUN_141d5f6e0(iVar1,param_1);
  *(int *)(param_1 + 1) = iVar1;
  return param_1;
}



//===========================================================
// __security_check_cookie @ 142ef44b0   (30 bytes)
//===========================================================

/* WARNING: This is an inlined function */

void __cdecl __security_check_cookie(uintptr_t _StackCookie)

{
  if ((_StackCookie == DAT_143a8b908) && ((short)(_StackCookie >> 0x30) == 0)) {
    return;
  }
  FUN_142ef3e44(_StackCookie);
  return;
}



//===========================================================
// FUN_142c4a030 @ 142c4a030   (4 bytes)
//===========================================================

undefined4 FUN_142c4a030(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x60);
}



//===========================================================
// FUN_141d5f6e0 @ 141d5f6e0   (340 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141d5f6e0(int param_1,undefined8 param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  undefined1 auStack_88 [32];
  undefined8 local_68 [2];
  undefined8 *local_58;
  undefined8 uStack_50;
  undefined8 *local_48;
  uint uStack_40;
  undefined4 uStack_3c;
  undefined8 local_30;
  undefined8 uStack_28;
  undefined8 local_20;
  undefined8 uStack_18;
  ulonglong local_10;
  
  local_10 = DAT_143a8b908 ^ (ulonglong)auStack_88;
  local_30 = 0;
  uStack_28 = 0;
  local_20 = 0;
  uStack_18 = 0;
  local_68[0] = param_2;
  FUN_14075a550(local_68,&local_30);
  puVar3 = DAT_143ad2cc0;
  puVar4 = (undefined8 *)DAT_143ad2cc0[1];
  uStack_40 = 0;
  cVar1 = *(char *)((longlong)puVar4 + 0x19);
  local_48 = puVar4;
  puVar5 = DAT_143ad2cc0;
  while (puVar2 = puVar4, cVar1 == '\0') {
    if (param_1 <= *(int *)(puVar2 + 4)) {
      puVar4 = (undefined8 *)*puVar2;
      puVar5 = puVar2;
    }
    else {
      puVar4 = (undefined8 *)puVar2[2];
    }
    uStack_40 = (uint)(param_1 <= *(int *)(puVar2 + 4));
    cVar1 = *(char *)((longlong)puVar4 + 0x19);
    local_48 = puVar2;
  }
  if ((*(char *)((longlong)puVar5 + 0x19) != '\0') || (param_1 < *(int *)(puVar5 + 4))) {
    if (DAT_143ad2cc8 == 0x38e38e38e38e38e) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    local_58 = &DAT_143ad2cc0;
    uStack_50 = 0;
    puVar4 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x48);
    *(int *)(puVar4 + 4) = param_1;
    puVar4[5] = 0;
    puVar4[6] = 0;
    puVar4[7] = 0;
    puVar4[8] = 0;
    *puVar4 = puVar3;
    puVar4[1] = puVar3;
    puVar4[2] = puVar3;
    *(undefined2 *)(puVar4 + 3) = 0;
    local_58 = local_48;
    uStack_50 = CONCAT44(uStack_3c,uStack_40);
    puVar5 = (undefined8 *)FUN_141d600c0(&DAT_143ad2cc0,&local_58,puVar4);
  }
  puVar5[5] = local_30;
  puVar5[6] = uStack_28;
  puVar5[7] = local_20;
  puVar5[8] = uStack_18;
  return;
}



//===========================================================
// FUN_141d5f870 @ 141d5f870   (181 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_141d5f870(int param_1)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined1 auStack_58 [32];
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  ulonglong local_10;
  
  local_10 = DAT_143a8b908 ^ (ulonglong)auStack_58;
  cVar1 = *(char *)((longlong)DAT_143ad2cc0[1] + 0x19);
  puVar3 = (undefined8 *)DAT_143ad2cc0[1];
  puVar2 = DAT_143ad2cc0;
  while (puVar4 = puVar3, cVar1 == '\0') {
    if (*(int *)(puVar4 + 4) < param_1) {
      puVar3 = (undefined8 *)puVar4[2];
      puVar4 = puVar2;
    }
    else {
      puVar3 = (undefined8 *)*puVar4;
    }
    cVar1 = *(char *)((longlong)puVar3 + 0x19);
    puVar2 = puVar4;
  }
  if (((*(char *)((longlong)puVar2 + 0x19) == '\0') && (*(int *)(puVar2 + 4) <= param_1)) &&
     (puVar2 != DAT_143ad2cc0)) {
    local_30 = puVar2[5];
    local_28 = puVar2[6];
    local_20 = puVar2[7];
    local_18 = puVar2[8];
    local_38 = 0;
    FUN_14075a6d0(&local_38,&local_30);
    return local_38;
  }
  return 0;
}



//===========================================================
// FUN_140738830 @ 140738830   (12 bytes)
//===========================================================

void FUN_140738830(void)

{
  FUN_1407386b0(&DAT_143ac1ab0);
  return;
}



//===========================================================
// FUN_142ef3e44 @ 142ef3e44   (210 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142ef3e44(void)

{
  code *pcVar1;
  int iVar2;
  undefined1 *puVar3;
  undefined1 auStack_38 [8];
  undefined1 auStack_30 [48];
  
  puVar3 = auStack_38;
  iVar2 = (*DAT_143262740)(0x17);
  if (iVar2 != 0) {
    pcVar1 = (code *)swi(0x29);
    (*pcVar1)(2);
    puVar3 = auStack_30;
  }
  *(undefined8 *)(puVar3 + -8) = 0x142ef3e6f;
  capture_previous_context(&DAT_143ae26f0);
  _DAT_143ae2660 = *(undefined8 *)(puVar3 + 0x38);
  _DAT_143ae2788 = puVar3 + 0x40;
  _DAT_143ae2770 = *(undefined8 *)(puVar3 + 0x40);
  _DAT_143ae2650 = 0xc0000409;
  _DAT_143ae2654 = 1;
  _DAT_143ae2668 = 1;
  DAT_143ae2670 = 2;
  *(undefined8 *)(puVar3 + 0x20) = DAT_143a8b908;
  *(undefined8 *)(puVar3 + 0x28) = DAT_143a8b900;
  *(undefined8 *)(puVar3 + -8) = 0x142ef3f11;
  DAT_143ae27e8 = _DAT_143ae2660;
  __raise_securityfailure(&PTR_DAT_1434a1978);
  return;
}



//===========================================================
// FUN_14019f9d0 @ 14019f9d0   (16 bytes)
//===========================================================

void FUN_14019f9d0(void)

{
                    /* WARNING: Subroutine does not return */
  FUN_142ed3068("map/set too long");
}



//===========================================================
// FUN_14019b780 @ 14019b780   (374 bytes)
//===========================================================

void FUN_14019b780(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  int iVar6;
  undefined8 uVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar7 = 0x80;
  if (param_2 < 0x21) {
    uVar10 = (uint)(0x10 < param_2);
LAB_14019b7de:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x10;
      goto LAB_14019b825;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x20;
      goto LAB_14019b825;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b825;
    }
  }
  else {
    if (0x40 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x81) {
        uVar10 = 3;
      }
      goto LAB_14019b7de;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x40;
LAB_14019b825:
  lVar11 = (longlong)(int)uVar10;
  lVar8 = lVar11 * 0x10 + param_1;
  plVar1 = (longlong *)(lVar8 + 0x28);
  LOCK();
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar4 == 0) {
LAB_14019b889:
    *(undefined4 *)(lVar8 + 0x30) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar4 = *plVar1;
      if (lVar4 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar4 == 0) goto LAB_14019b889;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar7,iVar6);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar6;
    puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  }
  piVar2 = (int *)(param_1 + 0x14 + lVar11 * 4);
  *piVar2 = *piVar2 + 1;
  *(undefined8 *)(param_1 + 0x68 + lVar11 * 8) = *puVar5;
  *piVar9 = *piVar9 + -1;
  if (*piVar9 == 0) {
    *plVar1 = 0;
  }
  return;
}



//===========================================================
// FUN_141d600c0 @ 141d600c0   (637 bytes)
//===========================================================

longlong * FUN_141d600c0(longlong *param_1,longlong *param_2,longlong *param_3)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong *plVar10;
  
  param_1[1] = param_1[1] + 1;
  plVar2 = (longlong *)*param_1;
  plVar8 = (longlong *)*param_2;
  param_3[1] = (longlong)plVar8;
  if (plVar8 == plVar2) {
    *plVar2 = (longlong)param_3;
    plVar2[1] = (longlong)param_3;
    plVar2[2] = (longlong)param_3;
    *(undefined1 *)(param_3 + 3) = 1;
    return param_3;
  }
  if ((int)param_2[1] == 0) {
    plVar8[2] = (longlong)param_3;
    if (plVar8 == (longlong *)plVar2[2]) {
      plVar2[2] = (longlong)param_3;
    }
  }
  else {
    *plVar8 = (longlong)param_3;
    if (plVar8 == (longlong *)*plVar2) {
      *plVar2 = (longlong)param_3;
    }
  }
  cVar1 = *(char *)(param_3[1] + 0x18);
  plVar8 = param_3;
  do {
    if (cVar1 != '\0') {
      *(undefined1 *)(plVar2[1] + 0x18) = 1;
      return param_3;
    }
    plVar9 = (longlong *)plVar8[1];
    plVar7 = plVar8 + 1;
    plVar10 = plVar9 + 1;
    lVar5 = *(longlong *)plVar9[1];
    if (plVar9 == (longlong *)lVar5) {
      lVar5 = ((longlong *)plVar9[1])[2];
      if (*(char *)(lVar5 + 0x18) != '\0') {
        plVar3 = (longlong *)plVar9[2];
        if (plVar8 == plVar3) {
          plVar9[2] = *plVar3;
          if (*(char *)(*plVar3 + 0x19) == '\0') {
            *(longlong **)(*plVar3 + 8) = plVar9;
          }
          plVar3[1] = *plVar10;
          if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
            *(longlong **)(*param_1 + 8) = plVar3;
            *plVar3 = (longlong)plVar9;
            *plVar10 = (longlong)plVar3;
            plVar8 = plVar9;
            plVar9 = plVar3;
            plVar7 = plVar10;
          }
          else {
            plVar8 = (longlong *)*plVar10;
            if (plVar9 == (longlong *)*plVar8) {
              *plVar8 = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
            else {
              plVar8[2] = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
          }
        }
        *(undefined1 *)(plVar9 + 3) = 1;
        *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
        plVar7 = *(longlong **)(*plVar7 + 8);
        plVar10 = (longlong *)*plVar7;
        *plVar7 = plVar10[2];
        if (*(char *)(plVar10[2] + 0x19) == '\0') {
          *(longlong **)(plVar10[2] + 8) = plVar7;
        }
        plVar10[1] = plVar7[1];
        if (plVar7 == *(longlong **)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar10;
          plVar10[2] = (longlong)plVar7;
        }
        else {
          plVar9 = (longlong *)plVar7[1];
          if (plVar7 == (longlong *)plVar9[2]) {
            plVar9[2] = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
          else {
            *plVar9 = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
        }
        goto LAB_141d60315;
      }
LAB_141d60247:
      *(undefined1 *)(plVar9 + 3) = 1;
      *(undefined1 *)(lVar5 + 0x18) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar8 = *(longlong **)(*plVar7 + 8);
    }
    else {
      if (*(char *)(lVar5 + 0x18) == '\0') goto LAB_141d60247;
      plVar3 = (longlong *)*plVar9;
      plVar6 = plVar9;
      if (plVar8 == plVar3) {
        *plVar9 = plVar3[2];
        if (*(char *)(plVar3[2] + 0x19) == '\0') {
          *(longlong **)(plVar3[2] + 8) = plVar9;
        }
        plVar3[1] = *plVar10;
        if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar3;
        }
        else {
          puVar4 = (undefined8 *)*plVar10;
          if (plVar9 == (longlong *)puVar4[2]) {
            puVar4[2] = plVar3;
          }
          else {
            *puVar4 = plVar3;
          }
        }
        plVar3[2] = (longlong)plVar9;
        *plVar10 = (longlong)plVar3;
        plVar6 = plVar3;
        plVar8 = plVar9;
        plVar7 = plVar10;
      }
      *(undefined1 *)(plVar6 + 3) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar7 = *(longlong **)(*plVar7 + 8);
      plVar10 = (longlong *)plVar7[2];
      plVar7[2] = *plVar10;
      if (*(char *)(*plVar10 + 0x19) == '\0') {
        *(longlong **)(*plVar10 + 8) = plVar7;
      }
      plVar10[1] = plVar7[1];
      if (plVar7 == *(longlong **)(*param_1 + 8)) {
        *(longlong **)(*param_1 + 8) = plVar10;
      }
      else {
        puVar4 = (undefined8 *)plVar7[1];
        if (plVar7 == (longlong *)*puVar4) {
          *puVar4 = plVar10;
        }
        else {
          puVar4[2] = plVar10;
        }
      }
      *plVar10 = (longlong)plVar7;
LAB_141d60315:
      plVar7[1] = (longlong)plVar10;
    }
    cVar1 = *(char *)(plVar8[1] + 0x18);
  } while( true );
}



//===========================================================
// FUN_14075a550 @ 14075a550   (368 bytes)
//===========================================================

void FUN_14075a550(ulonglong *param_1,ulonglong *param_2)

{
  ulonglong uVar1;
  ulonglong uVar2;
  uint uVar3;
  
  *param_2 = *param_1;
  uVar3 = 0;
  *param_1 = 0;
  uVar1 = *param_2;
  param_2[1] = uVar1 & 0xffffffff;
  *param_2 = uVar1 >> 0x20;
  uVar2 = uVar1 >> 0x20 ^ *(ulonglong *)(&DAT_14329c470 + (uVar1 & 0xff) * 8);
  *param_2 = uVar2;
  param_2[1] = uVar1 & 0xffffffff ^ *(ulonglong *)(&DAT_14329c470 + (uVar2 & 0xff) * 8);
  FUN_142e5c620(0xe0,0x100203415f6278);
  uVar1 = param_2[1];
  do {
    uVar1 = uVar1 >> 1 | (ulonglong)((uVar1 & 1) != 0) << 0x3f;
    param_2[1] = uVar1;
    uVar2 = (ulonglong)(int)uVar3;
    uVar3 = uVar3 + 0x1b;
    uVar1 = uVar1 ^ *(ulonglong *)(&DAT_14329c470 + (uVar2 & 0xff) * 8);
    param_2[1] = uVar1;
    uVar1 = uVar1 + *(longlong *)(&DAT_14329c470 + (uVar2 + 1 & 0xff) * 8);
    param_2[1] = uVar1;
  } while (uVar3 < 0x40);
  uVar3 = 5;
  do {
    uVar1 = uVar1 >> 1 | (ulonglong)((uVar1 & 1) != 0) << 0x3f;
    param_2[1] = uVar1;
    uVar2 = (ulonglong)(int)uVar3;
    uVar3 = uVar3 + 0x13;
    uVar1 = uVar1 ^ *(ulonglong *)(&DAT_14329c470 + (uVar2 & 0xff) * 8);
    param_2[1] = uVar1;
    uVar1 = uVar1 + *(longlong *)(&DAT_14329c470 + (uVar2 + 1 & 0xff) * 8);
    param_2[1] = uVar1;
  } while (uVar3 < 0x40);
  param_2[2] = uVar1 & 0xffffffff;
  param_2[1] = uVar1 >> 0x20;
  uVar2 = uVar1 >> 0x20 ^ *(ulonglong *)(&DAT_14329c470 + (uVar1 & 0xff) * 8);
  param_2[1] = uVar2;
  param_2[2] = uVar1 & 0xffffffff ^ *(ulonglong *)(&DAT_14329c470 + (uVar2 & 0xff) * 8);
  FUN_142e5c620(0xe0,0x100203415f6278);
  uVar1 = param_2[2];
  param_2[3] = uVar1 & 0xffffffff;
  param_2[2] = uVar1 >> 0x20;
  uVar2 = uVar1 >> 0x20 ^ *(ulonglong *)(&DAT_14329c470 + (uVar1 & 0xff) * 8);
  param_2[2] = uVar2;
  param_2[3] = uVar1 & 0xffffffff ^ *(ulonglong *)(&DAT_14329c470 + (uVar2 & 0xff) * 8);
  FUN_142e5c620(0xe0,0x100203415f6278);
  FUN_142e5c620(0xf9,0x100203415f6278);
  return;
}



//===========================================================
// FUN_14075a6d0 @ 14075a6d0   (516 bytes)
//===========================================================

void FUN_14075a6d0(ulonglong *param_1,ulonglong *param_2)

{
  ulonglong uVar1;
  ulonglong uVar2;
  
  uVar2 = *(ulonglong *)(&DAT_14329c470 + (ulonglong)(byte)param_2[2] * 8) ^ param_2[3];
  param_2[3] = uVar2;
  uVar1 = *(ulonglong *)(&DAT_14329c470 + (uVar2 & 0xff) * 8);
  param_2[3] = 0;
  param_2[2] = (uVar1 ^ param_2[2]) << 0x20 | uVar2;
  FUN_142e5c620(0xe8,0x100203415f6278);
  uVar2 = *(ulonglong *)(&DAT_14329c470 + (ulonglong)(byte)param_2[1] * 8) ^ param_2[2];
  param_2[2] = uVar2;
  uVar1 = *(ulonglong *)(&DAT_14329c470 + (uVar2 & 0xff) * 8);
  param_2[2] = 0;
  param_2[1] = (uVar1 ^ param_2[1]) << 0x20 | uVar2;
  FUN_142e5c620(0xe8,0x100203415f6278);
  uVar1 = param_2[1] + 0x738352d8ba47cabc ^ 0xfcfce480c3ce0713;
  uVar1 = (uVar1 << 1 | (ulonglong)((longlong)uVar1 < 0)) + 0x125fd6fae2b6e86b ^ 0xf426cd2d196feda2;
  uVar1 = (uVar1 << 1 | (ulonglong)((longlong)uVar1 < 0)) + 0x6d9b2969e4bd8a0f ^ 0xde994b55942da111;
  uVar1 = (uVar1 << 1 | (ulonglong)((longlong)uVar1 < 0)) + 0x2ec1d5d54860840e ^ 0x2221dcf97c3e0543;
  param_2[1] = uVar1 << 1 | (ulonglong)((longlong)uVar1 < 0);
  FUN_140c79130(0xd3,0x302a12f7128);
  uVar1 = param_2[1] + 0x74537b5863e98325 ^ 0xe1536879f75dba35;
  uVar1 = (uVar1 << 1 | (ulonglong)((longlong)uVar1 < 0)) + 0x86bf952ff7bf3e7d ^ 0x573e198fe4deb090;
  uVar1 = (uVar1 << 1 | (ulonglong)((longlong)uVar1 < 0)) + 0xd87c90ed3265452c ^ 0xf26f7f97c40e9160;
  param_2[1] = uVar1 << 1 | (ulonglong)((longlong)uVar1 < 0);
  FUN_140c79130(0xbd,0x302a12f7128);
  uVar2 = *(ulonglong *)(&DAT_14329c470 + (ulonglong)(byte)*param_2 * 8) ^ param_2[1];
  param_2[1] = uVar2;
  uVar1 = *(ulonglong *)(&DAT_14329c470 + (uVar2 & 0xff) * 8);
  param_2[1] = 0;
  *param_2 = (uVar1 ^ *param_2) << 0x20 | uVar2;
  FUN_142e5c620(0xe8,0x100203415f6278);
  FUN_142e5c620(0x104,0x100203415f6278);
  *param_1 = *param_2;
  *param_2 = 0;
  return;
}


