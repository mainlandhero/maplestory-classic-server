
//===========================================================
// FUN_141b307b0 @ 141b307b0   (6191 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Removing unreachable block (ram,0x000141b30e37) */
/* WARNING: Removing unreachable block (ram,0x000141b315ba) */

void FUN_141b307b0(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  code *pcVar2;
  IUnknown *pIVar3;
  longlong lVar4;
  byte bVar5;
  char cVar6;
  undefined1 uVar7;
  int iVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  int iVar11;
  int iVar12;
  uint uVar13;
  int *piVar14;
  int **ppiVar15;
  longlong lVar16;
  undefined8 *puVar17;
  undefined8 uVar18;
  int *piVar19;
  undefined4 *puVar20;
  longlong *plVar21;
  undefined8 uVar22;
  char *pcVar23;
  uint uVar24;
  ulonglong uVar25;
  undefined2 *puVar26;
  undefined1 *puVar27;
  int *piVar28;
  int *piVar29;
  int *piVar30;
  int *piVar31;
  undefined8 uStack_640;
  undefined1 auStack_638 [32];
  longlong local_618;
  undefined4 local_610 [6];
  uint local_5f8 [2];
  uint local_5f0;
  int *local_5e8;
  int *local_5e0;
  int *local_5d8;
  longlong *local_5d0;
  longlong local_5c8;
  int *local_5c0;
  ulonglong local_5b8;
  int **local_5b0;
  int *local_5a8;
  int *local_5a0;
  longlong local_598;
  undefined4 local_590;
  undefined4 uStack_58c;
  undefined8 uStack_588;
  undefined8 local_580;
  short local_578;
  undefined6 uStack_576;
  longlong lStack_570;
  undefined8 local_568;
  int *local_560;
  int *local_558;
  uint local_550;
  undefined4 uStack_54c;
  undefined4 uStack_548;
  undefined4 uStack_544;
  undefined8 local_540;
  undefined4 local_538;
  undefined4 local_534;
  undefined4 local_530;
  undefined4 local_52c;
  int *local_528;
  int **local_520;
  longlong local_518;
  IUnknown *local_510;
  undefined8 local_508;
  undefined8 uStack_500;
  undefined8 local_4f8;
  uint local_4e8;
  undefined4 uStack_4e4;
  undefined4 uStack_4e0;
  undefined4 uStack_4dc;
  undefined8 local_4d8;
  undefined8 local_4d0;
  undefined1 local_4c8 [8];
  longlong *local_4c0;
  uint local_4b8;
  undefined4 uStack_4b4;
  undefined4 uStack_4b0;
  undefined4 uStack_4ac;
  undefined8 local_4a8;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)local_5f8;
  uStack_640 = 0x141b307f2;
  iVar8 = FUN_142c4a810(DAT_143ac1898);
  if (iVar8 == 5) {
    uStack_640 = 0x141b30802;
    FUN_141b32860(param_1,param_2);
    puVar27 = auStack_638;
    goto LAB_141b31f94;
  }
  piVar30 = (int *)0x0;
  *(undefined4 *)(param_1 + 0xd4) = 0;
  local_5e8 = DAT_143aa84a0;
  uStack_640 = 0x141b30824;
  bVar5 = FUN_1406e8ae0(param_2);
  uVar13 = (uint)bVar5;
  local_5f8[0] = (uint)bVar5;
  uStack_640 = 0x141b30839;
  FUN_1406e9050(param_2,&local_518);
  piVar31 = (int *)0xffffffffffffffff;
  if (local_518 == 0) {
    iVar8 = 2;
  }
  else {
    local_610[0] = 0;
    local_618 = 0;
    uStack_640 = 0x141b30868;
    iVar8 = (*DAT_1432627f8)(0xfde9,0,local_518,0xffffffff);
    iVar8 = iVar8 * 2;
  }
  lVar16 = local_518;
  uVar25 = (longlong)iVar8 + 0xf;
  if (uVar25 <= (ulonglong)(longlong)iVar8) {
    uVar25 = 0xffffffffffffff0;
  }
  uStack_640 = 0x141b30892;
  lVar4 = -(uVar25 & 0xfffffffffffffff0);
  puVar26 = (undefined2 *)((longlong)local_5f8 + lVar4);
  piVar28 = piVar31;
  if (local_518 == 0) {
    if (puVar26 == (undefined2 *)0x0) goto LAB_141b308c8;
    *puVar26 = 0;
LAB_141b308e0:
    do {
      piVar28 = (int *)((longlong)piVar28 + 1);
    } while (puVar26[(longlong)piVar28] != 0);
    iVar11 = (int)piVar28;
    iVar8 = 0;
    if (0 < iVar11) {
      iVar8 = iVar11;
    }
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30909;
    piVar14 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar8 * 2 + 0x12));
    piVar14[1] = iVar8;
    *piVar14 = -1;
    piVar30 = piVar14 + 4;
    piVar14[2] = 0;
    *(undefined2 *)piVar30 = 0;
    local_560 = piVar30;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30938;
    FUN_142ef7ba0(piVar30,puVar26,(longlong)iVar11 * 2);
    if (*piVar14 != -1) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3094b;
      FUN_142e52dd0();
    }
    if ((iVar11 == -1) || (iVar8 = piVar14[1], iVar11 <= iVar8)) {
      *piVar14 = 1;
      if (iVar11 != -1) goto LAB_141b30975;
      piVar28 = piVar31;
      if (piVar30 == (int *)0x0) {
        piVar28 = (int *)0x0;
      }
      else {
        do {
          piVar28 = (int *)((longlong)piVar28 + 1);
        } while (*(short *)((longlong)piVar30 + (longlong)piVar28 * 2) != 0);
      }
    }
    else {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3096d;
      FUN_142e54290(0x90,iVar8,(ulonglong)piVar28 & 0xffffffff);
      *piVar14 = 1;
LAB_141b30975:
      *(undefined2 *)((longlong)iVar11 * 2 + (longlong)piVar30) = 0;
    }
    iVar8 = (int)piVar28;
    if ((iVar8 < 0) || (piVar14[1] + 1 <= iVar8)) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30998;
      FUN_142e54290(0x9c,(ulonglong)piVar28 & 0xffffffff);
    }
    piVar14[2] = iVar8 * 2;
    uVar13 = local_5f8[0];
  }
  else {
    *(undefined4 *)((longlong)local_610 + lVar4) = 0x100000;
    *(undefined2 **)((longlong)local_610 + lVar4 + -8) = puVar26;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b308c8;
    (*DAT_1432627f8)(0xfde9,0,lVar16,0xffffffff);
LAB_141b308c8:
    local_560 = (int *)0x0;
    if (puVar26 != (undefined2 *)0x0) goto LAB_141b308e0;
  }
  if (uVar13 == 0x83) {
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b309b8;
    local_5f0 = FUN_1406e8c20(param_2);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b309c3;
    uVar9 = FUN_1406e8c20(param_2);
    local_5b8 = CONCAT44(local_5b8._4_4_,uVar9);
    lVar16 = DAT_143ac8208;
    if (DAT_143ac8208 == 0) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b309e1;
      ppiVar15 = (int **)FUN_14019b780(&DAT_143ad68a0,0x58);
      lVar16 = 0;
      if (ppiVar15 != (int **)0x0) {
        local_5b0 = ppiVar15;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b309f2;
        lVar16 = FUN_141d5c6a0(ppiVar15);
      }
    }
    local_5b0 = &local_5d8;
    local_5d8 = (int *)0x0;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30a49;
    piVar28 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar28[1] = 0;
    *piVar28 = -1;
    local_5d8 = piVar28 + 4;
    piVar28[2] = 0;
    *(undefined1 *)local_5d8 = 0;
    if (*piVar28 != -1) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30a77;
      FUN_142e52dd0(0x8b);
    }
    iVar8 = piVar28[1];
    if (iVar8 < 0) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30a8b;
      FUN_142e54290(0x90,iVar8,0);
    }
    *piVar28 = 1;
    *(undefined1 *)local_5d8 = 0;
    if (piVar28[1] + 1 < 1) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ab1;
      FUN_142e54290(0x9c,0);
    }
    piVar28[2] = 0;
    local_520 = &local_5e0;
    local_5e0 = (int *)0x0;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ad4;
    piVar28 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar28[1] = 0;
    *piVar28 = -1;
    local_5e0 = piVar28 + 4;
    piVar28[2] = 0;
    *(char *)local_5e0 = '\0';
    if (*piVar28 != -1) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30b02;
      FUN_142e52dd0(0x8b);
    }
    iVar8 = piVar28[1];
    if (iVar8 < 0) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30b16;
      FUN_142e54290(0x90,iVar8,0);
    }
    *piVar28 = 1;
    *(char *)local_5e0 = '\0';
    if (piVar28[1] + 1 < 1) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30b3c;
      FUN_142e54290(0x9c,0);
    }
    piVar28[2] = 0;
    local_5a0 = (int *)0x0;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30b52;
    uVar9 = FUN_14090d160(0x3c8,1);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30b63;
    uVar10 = FUN_14090d160(0x3c8,1);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30b73;
    puVar17 = (undefined8 *)FUN_1408a9e40(&local_5c0,0x75);
    *(undefined4 *)((longlong)local_610 + lVar4 + 0x10) = uVar9;
    *(undefined4 *)((longlong)local_610 + lVar4 + 8) = 1;
    *(undefined4 *)((longlong)local_610 + lVar4) = 0x3d1c3;
    *(undefined4 *)((longlong)local_610 + lVar4 + -8) = uVar10;
    uVar13 = local_5f0;
    uVar25 = local_5b8 & 0xffffffff;
    uVar18 = *puVar17;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ba0;
    uVar18 = FUN_14019ba10(&local_5a0,uVar18,uVar13,uVar25);
    local_5c8 = 0;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30bb2;
    FUN_14019a260(&local_5c8,uVar18);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30bc7;
    FUN_141d5c830(lVar16,&local_5c8,&local_5e0,&local_5d8);
    if (local_5c0 != (int *)0x0) {
      piVar28 = local_5c0 + -4;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30bda;
      FUN_14019f2c0(piVar28);
    }
    if (local_5a0 != (int *)0x0) {
      piVar28 = local_5a0 + -4;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30bed;
      FUN_14019f2c0(piVar28);
    }
    local_5b0 = (int **)0x0;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30c06;
    FUN_140919f30(&DAT_143271f04,0x992,0);
    uVar13 = local_5f8[0];
  }
  else {
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30c0e;
    cVar6 = FUN_140859ca0();
    if (cVar6 != '\0') {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30c1b;
      puVar17 = (undefined8 *)FUN_142c50940(&local_5c0);
      uVar18 = *puVar17;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30c2b;
      uVar22 = FUN_142c49f00(DAT_143ac1898);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30c37;
      (*DAT_143262b78)(uVar22,uVar18);
      uVar13 = local_5f8[0];
      if (local_5c0 != (int *)0x0) {
        piVar28 = local_5c0 + -4;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30c4a;
        FUN_14019f2c0(piVar28);
        uVar13 = local_5f8[0];
      }
    }
  }
  piVar28 = local_5e8;
  if (DAT_143ad2230 == 0) {
    if ((uVar13 - 0x65 & 0xfffffffd) == 0) {
LAB_141b30ce8:
      piVar28 = local_5e8;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30cf1;
      FUN_142cf4350(piVar28);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30cfb;
      FUN_14209c060(param_1,0);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30d0c;
      FUN_1406ed520(local_498,0x80);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30d19;
      FUN_1415d01c0(local_498);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30d26;
      FUN_1406ed610(local_498);
    }
    else if ((uVar13 == 0x22) && (local_5e8 != (int *)0x0)) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ce4;
      cVar6 = FUN_142cf42c0(piVar28);
      if (cVar6 != '\0') goto LAB_141b30ce8;
    }
  }
  else {
    if (DAT_143aa8520 != 0) {
      pcVar2 = *(code **)(*(longlong *)(DAT_143aa8520 + 8) + 0xd0);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30c7a;
      iVar8 = (*pcVar2)((longlong *)(DAT_143aa8520 + 8),&PTR_PTR_143a886f8);
      if ((iVar8 != 0) && (DAT_143aa8520 != 0)) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30c8f;
        FUN_141b6e530();
      }
    }
    if (DAT_143ad2230 != 0) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ca0;
      FUN_141b65d60();
    }
    if (DAT_143aca790 != 0) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30cb1;
      FUN_141179940();
      if (DAT_143aca790 != 0) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30cc2;
        FUN_141179940();
      }
    }
  }
  local_5a8 = (int *)0x0;
  piVar14 = piVar30;
  piVar28 = local_5a8;
  if ((piVar30 != (int *)0x0) && (piVar29 = piVar30 + -4, piVar29 != (int *)0x0)) {
    if (*piVar29 == -1) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30d57;
      FUN_142e52d50(0xcb,0xffffff01);
      piVar29 = piVar31;
      do {
        piVar29 = (int *)((longlong)piVar29 + 1);
      } while (*(short *)((longlong)piVar30 + (longlong)piVar29 * 2) != 0);
      iVar11 = (int)piVar29;
      iVar8 = 0;
      if (0 < iVar11) {
        iVar8 = iVar11;
      }
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30d8a;
      piVar19 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar8 * 2 + 0x12));
      piVar19[1] = iVar8;
      *piVar19 = -1;
      piVar28 = piVar19 + 4;
      piVar19[2] = 0;
      *(undefined2 *)piVar28 = 0;
      local_558 = piVar28;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30db6;
      FUN_142ef7ba0(piVar28,piVar30,(longlong)iVar11 * 2);
      if (*piVar19 != -1) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30dc8;
        FUN_142e52dd0();
      }
      if ((iVar11 == -1) || (iVar8 = piVar19[1], iVar11 <= iVar8)) {
        *piVar19 = 1;
        if (iVar11 != -1) goto LAB_141b30de8;
        piVar29 = piVar31;
        if (piVar28 == (int *)0x0) {
          piVar29 = (int *)0x0;
        }
        else {
          do {
            piVar29 = (int *)((longlong)piVar29 + 1);
          } while (*(short *)((longlong)piVar28 + (longlong)piVar29 * 2) != 0);
        }
      }
      else {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30de1;
        FUN_142e54290(0x90,iVar8,(ulonglong)piVar29 & 0xffffffff);
        *piVar19 = 1;
LAB_141b30de8:
        *(undefined2 *)((longlong)iVar11 * 2 + (longlong)piVar28) = 0;
      }
      iVar8 = (int)piVar29;
      if ((iVar8 < 0) || (piVar19[1] + 1 <= iVar8)) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30e0c;
        FUN_142e54290(0x9c,(ulonglong)piVar29 & 0xffffffff);
      }
      piVar19[2] = iVar8 * 2;
      if (local_5a8 != (int *)0x0) {
        piVar30 = local_5a8 + -4;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30e24;
        FUN_1401bebb0(piVar30);
      }
      local_558 = (int *)0x0;
    }
    else {
      if (*piVar29 < 1) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30e83;
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar29 = *piVar29 + 1;
      UNLOCK();
      piVar14 = local_560;
      piVar28 = piVar30;
      if (local_5a8 != (int *)0x0) {
        piVar30 = local_5a8 + -4;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30e98;
        FUN_1401bebb0(piVar30);
        piVar14 = local_560;
      }
    }
  }
  local_5a8 = piVar28;
  uVar13 = local_5f8[0];
  *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ebb;
  cVar6 = FUN_141b267c0(param_1,uVar13,0,&local_5a8);
  if ((cVar6 != '\0') && (uVar13 == 0)) {
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ed3;
    thunk_FUN_1406e8ae0(param_2);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30eea;
    FUN_1406e9170(param_2,&local_4d0,8);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ef6;
    FUN_1408f67d0(local_4d0);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30efe;
    iVar8 = FUN_1406e8c20(param_2);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f08;
    iVar11 = FUN_1406e8c20(param_2);
    piVar30 = local_5e8;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f13;
    iVar12 = FUN_142cb9230(piVar30);
    piVar30 = local_5e8;
    if (iVar8 == iVar12) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f20;
      iVar12 = FUN_142cb9260(piVar30);
      if (iVar11 != iVar12) goto LAB_141b30f24;
    }
    else {
LAB_141b30f24:
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f34;
      FUN_141b2c7c0(param_1,iVar8,iVar11,0);
    }
    piVar30 = local_5e8;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f40;
    uVar9 = FUN_142cb9230(piVar30);
    *(undefined4 *)(param_1 + 0x184) = uVar9;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f5d;
    FUN_1406e9170(param_2,&local_534,4);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f72;
    FUN_1406e9170(param_2,&local_538,4);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f83;
    FUN_1408414d0(local_534,local_538);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30f98;
    FUN_1406e9170(param_2,&local_530,4);
    local_52c = local_530;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30fb0;
    FUN_140842250(&local_52c);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30fb8;
    FUN_142cac0f0(piVar30);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30fc0;
    uVar9 = FUN_1406e8c20(param_2);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30fca;
    FUN_142cb8620(piVar30,uVar9);
    uVar18 = DAT_143ac87a0;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30fd9;
    uVar9 = FUN_142cb8460(piVar30);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30fe3;
    FUN_1415f5bd0(uVar18,uVar9);
    uVar18 = DAT_143ac87a0;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ff2;
    uVar9 = FUN_142cb9230(piVar30);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b30ffc;
    uVar9 = FUN_1415f6370(uVar18,uVar9);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31006;
    bVar5 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0x170) = (uint)bVar5;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31019;
    FUN_14108d290(param_2);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31021;
    FUN_14108bdf0(param_2);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31028;
    iVar8 = FUN_14108d9b0(uVar9);
    if (iVar8 < 0) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31033;
      iVar11 = FUN_14108cd40();
      if (0 < iVar11) {
        iVar8 = 0;
      }
    }
    *(int *)(param_1 + 0x128) = iVar8;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3104a;
    uVar9 = FUN_14108cd40();
    *(undefined4 *)(param_1 + 0xe8) = uVar9;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3105a;
    uVar7 = FUN_1406e8ae0(param_2);
    *(undefined1 *)(param_1 + 0xdc) = uVar7;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3106a;
    bVar5 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0xe0) = (uint)bVar5;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3107d;
    uVar9 = FUN_1406e8c20(param_2);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31084;
    FUN_14108db90(uVar9);
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3108c;
    uVar7 = thunk_FUN_1406e8ae0(param_2);
    *(undefined1 *)(piVar30 + 0xd38) = uVar7;
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3109a;
    cVar6 = FUN_1406e8ae0(param_2);
    DAT_143ad2102 = cVar6 != '\0';
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b310ab;
    cVar6 = FUN_1406e8ae0(param_2);
    DAT_143ad2103 = cVar6 != '\0';
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b310bc;
    iVar8 = FUN_1406e8c20(param_2);
    piVar30[0xd92] = iVar8;
    if (DAT_143aa84a0 != (int *)0x0) {
      iVar8 = piVar30[0xd6d];
      iVar11 = 0;
      local_5d0 = (longlong *)0x0;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b310e9;
      FUN_14019a260(&local_5d0,piVar30 + 0xd6e);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b310f2;
      FUN_142dd5210(piVar30);
      if (iVar8 != 0) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b310ff;
        cVar6 = FUN_14108d4b0(iVar8);
        if (cVar6 == '\0') {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3110c;
          iVar8 = FUN_14108d9b0(iVar8);
          plVar21 = local_5d0;
          if (-1 < iVar8) {
            *(int *)(param_1 + 0x128) = iVar8;
            ppiVar15 = (int **)(param_1 + 400);
            piVar30 = piVar31;
            if (local_5d0 == (longlong *)0x0) {
              piVar30 = *ppiVar15;
              if (piVar30 != (int *)0x0) {
                *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b311bd;
                FUN_14019f2c0(piVar30 + -4);
                *ppiVar15 = (int *)0x0;
              }
            }
            else {
              do {
                piVar30 = (int *)((longlong)piVar30 + 1);
              } while (*(char *)((longlong)local_5d0 + (longlong)piVar30) != '\0');
              piVar28 = *ppiVar15 + -4;
              if (*ppiVar15 == (int *)0x0) {
                piVar28 = (int *)0x0;
              }
              iVar8 = (int)piVar30;
              if (piVar28 == (int *)0x0) {
LAB_141b31212:
                if (iVar11 < iVar8) {
                  iVar11 = iVar8;
                }
                *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3122c;
                puVar20 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
                puVar20[1] = iVar11;
                *puVar20 = 0xffffffff;
                *ppiVar15 = puVar20 + 4;
                puVar20[2] = 0;
                *(undefined1 *)*ppiVar15 = 0;
                if (piVar28 != (int *)0x0) {
                  *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31254;
                  FUN_14019f2c0(piVar28);
                }
              }
              else {
                if ((1 < *piVar28) || (piVar28[1] < iVar8)) {
                  iVar11 = piVar28[2];
                  goto LAB_141b31212;
                }
                if (*piVar28 != 1) {
                  *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31209;
                  FUN_142e52dd0(0x74);
                }
                *piVar28 = -1;
              }
              piVar14 = (int *)0x0;
              piVar28 = *ppiVar15;
              *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31265;
              FUN_142ef7ba0(piVar28,plVar21,(longlong)iVar8);
              piVar28 = *ppiVar15;
              if (piVar28[-4] != -1) {
                *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3127a;
                FUN_142e52dd0(0x8b);
              }
              if ((iVar8 == -1) || (iVar11 = piVar28[-3], iVar8 <= iVar11)) {
                piVar28[-4] = 1;
                if (iVar8 != -1) goto LAB_141b3129a;
                piVar30 = piVar31;
                if (piVar28 != (int *)0x0) {
                  do {
                    piVar14 = (int *)((longlong)piVar30 + 1);
                    piVar30 = piVar14;
                  } while (*(char *)((longlong)piVar28 + (longlong)piVar14) != '\0');
                }
              }
              else {
                *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31293;
                FUN_142e54290(0x90,iVar11,(ulonglong)piVar30 & 0xffffffff);
                piVar28[-4] = 1;
LAB_141b3129a:
                *(undefined1 *)((longlong)iVar8 + (longlong)*ppiVar15) = 0;
                piVar14 = piVar30;
              }
              iVar8 = (int)piVar14;
              if ((iVar8 < 0) || (piVar28[-3] + 1 <= iVar8)) {
                *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b312be;
                FUN_142e54290(0x9c,(ulonglong)piVar14 & 0xffffffff);
              }
              piVar28[-2] = iVar8;
            }
            piVar30 = (int *)0x0;
            if (*ppiVar15 != (int *)0x0) {
              *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b312de;
              FUN_14019bd40(ppiVar15,0,1);
              piVar28 = piVar30;
              if (*ppiVar15 != (int *)0x0) {
                piVar28 = (int *)(ulonglong)(uint)(*ppiVar15)[-2];
              }
              *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3131b;
              FUN_14019c870(ppiVar15,piVar28);
              iVar8 = (*ppiVar15)[-2];
              piVar28 = piVar30;
              if (0 < (longlong)iVar8) {
                do {
                  bVar5 = *(byte *)((longlong)*ppiVar15 + (longlong)piVar28);
                  *(byte *)((longlong)*ppiVar15 + (longlong)piVar28) = bVar5 >> 4 | bVar5 << 4;
                  piVar28 = (int *)((longlong)piVar28 + 1);
                } while ((longlong)piVar28 < (longlong)iVar8);
              }
            }
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3135d;
            uVar13 = FUN_1407386b0(&DAT_143ac1ab0);
            uVar24 = *(int *)(param_1 + 0x19c) * 0x343fd + 0x269ec3;
            *(uint *)(param_1 + 0x19c) = uVar24;
            uVar24 = uVar24 >> 0x10 & 0x7fff;
            bVar5 = (char)uVar24 + (char)(uVar24 / 0xff) + 1;
            *(byte *)(param_1 + 0x1a0) = bVar5;
            local_5f8[0] = (uint)bVar5 * 0x1010101 ^ uVar13 & 7;
            *(uint *)(param_1 + 0x198) = local_5f8[0];
            local_5f0 = local_5f8[0];
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b313de;
            FUN_140c79130(0x43,param_1 + 0x302a12f72c0);
            local_5f0 = (uint)*(byte *)(param_1 + 0x1a0) * 0x1010101 ^ local_5f0;
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31413;
            FUN_140c78f50(0x47,param_1 + 0x21a3f060fc2f47);
            if (*ppiVar15 != (int *)0x0) {
              *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3142b;
              FUN_14019bd40(ppiVar15,0,1);
              piVar28 = piVar30;
              if (*ppiVar15 != (int *)0x0) {
                piVar28 = (int *)(ulonglong)(uint)(*ppiVar15)[-2];
              }
              *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31443;
              FUN_14019c870(ppiVar15,piVar28);
              bVar5 = (char)local_5f0 + (char)((int)local_5f0 / 7) * -7 + 1;
              iVar8 = (*ppiVar15)[-2];
              if (0 < (longlong)iVar8) {
                piVar28 = piVar30;
                do {
                  bVar1 = *(byte *)((longlong)*ppiVar15 + (longlong)piVar28);
                  *(byte *)((longlong)*ppiVar15 + (longlong)piVar28) =
                       bVar1 >> (8 - bVar5 & 0x1f) | bVar1 << (bVar5 & 0x1f);
                  piVar28 = (int *)((longlong)piVar28 + 1);
                } while ((longlong)piVar28 < (longlong)iVar8);
              }
            }
            local_5e8 = (int *)0x0;
            piVar28 = local_5e8;
            if (((&local_5e8 != ppiVar15) && (piVar14 = *ppiVar15, piVar14 != (int *)0x0)) &&
               (piVar29 = piVar14 + -4, piVar29 != (int *)0x0)) {
              if (*piVar29 == -1) {
                *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b314e5;
                FUN_142e52d50(0xcb,0xffffff01);
                piVar14 = *ppiVar15;
                local_5c0 = (int *)0x0;
                piVar28 = piVar30;
                piVar29 = piVar31;
                if (piVar14 != (int *)0x0) {
                  do {
                    piVar29 = (int *)((longlong)piVar29 + 1);
                  } while (*(char *)((longlong)piVar14 + (longlong)piVar29) != '\0');
                  iVar8 = (int)piVar29;
                  if (0 < iVar8) {
                    piVar28 = (int *)((ulonglong)piVar29 & 0xffffffff);
                  }
                  *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31520;
                  piVar19 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)((int)piVar28 + 0x11));
                  piVar19[1] = (int)piVar28;
                  *piVar19 = -1;
                  piVar28 = piVar19 + 4;
                  piVar19[2] = 0;
                  *(undefined1 *)piVar28 = 0;
                  local_5c0 = piVar28;
                  *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31546;
                  FUN_142ef7ba0(piVar28,piVar14,(longlong)iVar8);
                  if (*piVar19 != -1) {
                    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31558;
                    FUN_142e52dd0(0x8b);
                  }
                  if ((iVar8 == -1) || (iVar11 = piVar19[1], iVar8 <= iVar11)) {
                    *piVar19 = 1;
                    if (iVar8 != -1) goto LAB_141b31578;
                    if (piVar28 != (int *)0x0) {
                      do {
                        piVar31 = (int *)((longlong)piVar31 + 1);
                      } while (*(char *)((longlong)piVar28 + (longlong)piVar31) != '\0');
                      piVar30 = (int *)((ulonglong)piVar31 & 0xffffffff);
                    }
                  }
                  else {
                    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31571;
                    FUN_142e54290(0x90,iVar11,(ulonglong)piVar29 & 0xffffffff);
                    *piVar19 = 1;
LAB_141b31578:
                    *(undefined1 *)((longlong)piVar28 + (longlong)iVar8) = 0;
                    piVar30 = piVar29;
                  }
                  iVar8 = (int)piVar30;
                  if ((iVar8 < 0) || (piVar19[1] + 1 <= iVar8)) {
                    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31599;
                    FUN_142e54290(0x9c,(ulonglong)piVar30 & 0xffffffff);
                  }
                  piVar19[2] = iVar8;
                }
                if (local_5e8 != (int *)0x0) {
                  piVar31 = local_5e8 + -4;
                  *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b315ae;
                  FUN_14019f2c0(piVar31);
                }
              }
              else {
                if (*piVar29 < 1) {
                  *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31602;
                  FUN_142e52dd0(0xd2);
                }
                LOCK();
                *piVar29 = *piVar29 + 1;
                UNLOCK();
                piVar28 = piVar14;
                if (local_5e8 != (int *)0x0) {
                  piVar31 = local_5e8 + -4;
                  *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31617;
                  FUN_14019f2c0(piVar31);
                }
              }
            }
            local_5e8 = piVar28;
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3162a;
            FUN_141b28570(param_1,&local_5e8,0);
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3163d;
            FUN_141b3f050(param_1,3,800);
            piVar14 = local_560;
            if (local_5d0 != (longlong *)0x0) {
              plVar21 = local_5d0 + -2;
              *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31650;
              FUN_14019f2c0(plVar21);
              piVar14 = local_560;
            }
            goto LAB_141b31f6f;
          }
        }
      }
      if (local_5d0 != (longlong *)0x0) {
        plVar21 = local_5d0 + -2;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31122;
        FUN_14019f2c0(plVar21);
      }
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3112a;
      FUN_140d22050(0);
    }
    pIVar3 = *(IUnknown **)(param_1 + 200);
    if (pIVar3 != (IUnknown *)0x0) {
      pcVar2 = *(code **)(*(longlong *)pIVar3 + 0x2b8);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31145;
      iVar8 = (*pcVar2)(pIVar3,0);
      if (iVar8 < 0) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3115a;
        _com_issue_errorex(iVar8,pIVar3,(_GUID *)&DAT_14327fcb0);
      }
    }
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3116d;
    FUN_141b3f050(param_1,3,800);
    if ((*(char *)(param_1 + 0xdc) == '\x01') || ((byte)(*(char *)(param_1 + 0xdc) - 4U) < 2)) {
      uVar18 = 1;
    }
    else {
      uVar18 = 0;
    }
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3166d;
    FUN_142cb83e0(DAT_143aa84a0,uVar18);
    pIVar3 = DAT_143add058;
    piVar31 = (int *)0x0;
    if ((char)piVar30[0xd38] == -1) {
      local_5e0 = (int *)0x0;
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_638 + lVar4 + -8) = &UNK_141b31fda;
        FUN_142ef3ac0(0x80004003);
      }
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b316a1;
      (*DAT_143262a20)(&local_578);
      if (DAT_143a8b8d8 == 8) {
        if (local_578 == 8) {
          local_578 = 0;
          if (lStack_570 != 0) {
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b316d3;
            (*DAT_143ad5990)(lStack_570 + -4);
          }
        }
        else {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b316e2;
          iVar8 = (*DAT_143262a18)(&local_578);
          if (iVar8 < 0) goto LAB_141b31fdb;
        }
        local_578 = 8;
        if (DAT_143a8b8e0 == 0) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3170a;
          lStack_570 = FUN_1401a5fa0(0,0);
        }
        else {
          uVar13 = *(uint *)(DAT_143a8b8e0 + -4);
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3171d;
          lStack_570 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13 >> 1);
        }
      }
      else {
        if ((local_578 == 8) && (local_578 = 0, lStack_570 != 0)) {
          lVar16 = lStack_570 + -4;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3174e;
          (*DAT_143ad5990)(lVar16);
        }
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31762;
        iVar8 = (*DAT_143262a28)(&local_578,&DAT_143a8b8d8);
        if (iVar8 < 0) {
LAB_141b31fdb:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_638 + lVar4 + -8) = &UNK_141b31fe2;
          FUN_142ef3ac0(iVar8);
        }
      }
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31779;
      (*DAT_143262a20)(&local_590);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_590 == 8) {
          local_590 = (uint)local_590._2_2_ << 0x10;
          if (uStack_588 != 0) {
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b317a6;
            (*DAT_143ad5990)(uStack_588 + -4);
          }
        }
        else {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b317b2;
          iVar8 = (*DAT_143262a18)(&local_590);
          if (iVar8 < 0) goto LAB_141b31fc8;
        }
        local_590 = CONCAT22(local_590._2_2_,8);
        piVar30 = piVar31;
        if (DAT_143a8b8e0 != 0) {
          piVar30 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b317d9;
        uStack_588 = FUN_1401a5fa0(DAT_143a8b8e0,piVar30);
      }
      else {
        if (((short)local_590 == 8) && (local_590 = (uint)local_590._2_2_ << 0x10, uStack_588 != 0))
        {
          lVar16 = uStack_588 + -4;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31837;
          (*DAT_143ad5990)(lVar16);
        }
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31848;
        iVar8 = (*DAT_143262a28)(&local_590,&DAT_143a8b8d8);
        if (iVar8 < 0) {
LAB_141b31fc8:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_638 + lVar4 + -8) = &UNK_141b31fcf;
          FUN_142ef3ac0(iVar8);
        }
      }
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b317f0;
      plVar21 = (longlong *)FUN_1401a5890(local_4c8,L"String/Consent.img/Consent");
      local_5d0 = plVar21;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31804;
      (*DAT_143262a20)(&local_550);
      pcVar2 = *(code **)(*(longlong *)pIVar3 + 0x48);
      piVar30 = piVar31;
      if ((undefined8 *)*plVar21 != (undefined8 *)0x0) {
        piVar30 = *(int **)*plVar21;
      }
      local_508 = CONCAT62(uStack_576,local_578);
      uStack_500 = lStack_570;
      local_4f8 = local_568;
      local_4b8 = local_590;
      uStack_4b4 = uStack_58c;
      uStack_4b0 = (undefined4)uStack_588;
      uStack_4ac = uStack_588._4_4_;
      local_4a8 = local_580;
      *(uint **)((longlong)local_610 + lVar4 + -8) = &local_550;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b318aa;
      iVar8 = (*pcVar2)(pIVar3,piVar30,&local_4b8,&local_508);
      if (iVar8 < 0) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b318bf;
        _com_issue_errorex(iVar8,pIVar3,(_GUID *)&DAT_1432743e8);
      }
      local_4e8 = local_550;
      uStack_4e4 = uStack_54c;
      uStack_4e0 = uStack_548;
      uStack_4dc = uStack_544;
      local_4d8 = local_540;
      local_550 = local_550 & 0xffff0000;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b318ed;
      thunk_FUN_1401be120(plVar21);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31901;
      uVar18 = FUN_1409339d0(&local_4c0,&local_4e8);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31911;
      FUN_1401a5040(&local_510,uVar18);
      if (local_4c0 != (longlong *)0x0) {
        pcVar2 = *(code **)(*local_4c0 + 0x10);
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31924;
        (*pcVar2)();
      }
      if ((short)local_4e8 == 8) {
        local_4e8 = local_4e8 & 0xffff0000;
        lVar16 = CONCAT44(uStack_4dc,uStack_4e0);
        if (lVar16 != 0) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3194d;
          (*DAT_143ad5990)(lVar16 + -4);
        }
      }
      else {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3195c;
        (*DAT_143262a18)(&local_4e8);
      }
      if ((short)local_590 == 8) {
        local_590 = local_590 & 0xffff0000;
        if (uStack_588 != 0) {
          lVar16 = uStack_588 + -4;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b3197c;
          (*DAT_143ad5990)(lVar16);
        }
      }
      else {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31988;
        (*DAT_143262a18)(&local_590);
      }
      if (local_578 == 8) {
        local_578 = 0;
        if (lStack_570 != 0) {
          lVar16 = lStack_570 + -4;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b319b1;
          (*DAT_143ad5990)(lVar16);
        }
      }
      else {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b319c0;
        (*DAT_143262a18)(&local_578);
      }
      local_5f0 = 0;
      piVar30 = piVar31;
      if (local_510 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_638 + lVar4 + -8) = &UNK_141b31fc7;
        FUN_142ef3ac0(0x80004003);
      }
      while( true ) {
        uVar18 = 0;
        uVar13 = (uint)piVar30;
        local_5b8 = local_5b8 & 0xffffffff00000000;
        pcVar2 = *(code **)(*(longlong *)local_510 + 0x40);
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b319f1;
        iVar8 = (*pcVar2)(local_510,&local_5b8);
        if (iVar8 < 0) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31a06;
          _com_issue_errorex(iVar8,local_510,(_GUID *)&DAT_143272478);
        }
        if ((uint)local_5b8 <= uVar13) break;
        local_5b8 = 0;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31a27;
        puVar17 = (undefined8 *)FUN_14019ba10(&local_5b8,PTR_s_Text_02d_143a45778,piVar30);
        uVar22 = *puVar17;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31a36;
        plVar21 = (longlong *)FUN_1401a5780(&local_520,uVar22);
        local_5d0 = plVar21;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31a4a;
        (*DAT_143262a20)(&local_508);
        pcVar2 = *(code **)(*(longlong *)local_510 + 0x28);
        if ((undefined8 *)*plVar21 != (undefined8 *)0x0) {
          uVar18 = *(undefined8 *)*plVar21;
        }
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31a6d;
        iVar8 = (*pcVar2)(local_510,uVar18,&local_508);
        if (iVar8 < 0) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31a82;
          _com_issue_errorex(iVar8,local_510,(_GUID *)&DAT_143272478);
        }
        local_550 = (uint)local_508;
        uStack_54c = local_508._4_4_;
        uStack_548 = (undefined4)uStack_500;
        uStack_544 = uStack_500._4_4_;
        local_540 = local_4f8;
        local_508 = local_508 & 0xffffffffffff0000;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31ab0;
        thunk_FUN_1401be120(plVar21);
        puVar26 = &DAT_143278568;
        if ((short)local_550 == 8) {
          puVar26 = (undefined2 *)CONCAT44(uStack_544,uStack_548);
        }
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31ad4;
        FUN_14022d860(&local_5b0,puVar26);
        ppiVar15 = local_5b0;
        if ((local_5b0 != (int **)0x0) && (iVar8 = *(int *)(local_5b0 + -1), iVar8 != 0)) {
          if ((piVar31 == (int *)0x0) || ((char)*piVar31 == '\0')) {
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31b60;
            uVar18 = FUN_14019bd40(&local_5e0,iVar8,0);
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31b6e;
            FUN_142ef7ba0(uVar18,ppiVar15);
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31b7a;
            FUN_14019c870(&local_5e0,iVar8);
            piVar31 = local_5e0;
          }
          else {
            iVar11 = piVar31[-2];
            for (iVar12 = piVar31[-3]; iVar12 < iVar11 + iVar8; iVar12 = iVar12 * 2) {
            }
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31b1e;
            lVar16 = FUN_14019bd40(&local_5e0,iVar12,1);
            piVar31 = local_5e0;
            if (local_5e0 == (int *)0x0) {
              iVar12 = 0;
            }
            else {
              iVar12 = local_5e0[-2];
            }
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31b3f;
            FUN_142ef7ba0(iVar12 + lVar16,ppiVar15);
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31b4b;
            FUN_14019c870(&local_5e0,iVar11 + iVar8);
            uVar13 = local_5f0;
          }
        }
        if (ppiVar15 != (int **)0x0) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31b8f;
          FUN_14019f2c0(ppiVar15 + -2);
        }
        if ((short)local_550 == 8) {
          local_550 = local_550 & 0xffff0000;
          lVar16 = CONCAT44(uStack_544,uStack_548);
          if (lVar16 != 0) {
            *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31bb8;
            (*DAT_143ad5990)(lVar16 + -4);
          }
        }
        else {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31bc7;
          (*DAT_143262a18)(&local_550);
        }
        if (local_5b8 != 0) {
          lVar16 = local_5b8 - 0x10;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31bda;
          FUN_14019f2c0(lVar16);
        }
        local_5f0 = uVar13 + 1;
        piVar30 = (int *)(ulonglong)local_5f0;
      }
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31bf8;
      FUN_1406ed520(local_498,0xa0);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c07;
      FUN_1406ed840(local_498,1);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c13;
      FUN_1415d01c0(local_498);
      *(undefined1 *)(DAT_143aa84a0 + 0xd38) = 1;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c2d;
      FUN_1406ed610(local_498);
      pcVar2 = *(code **)(*(longlong *)local_510 + 0x10);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c37;
      (*pcVar2)(local_510);
      piVar30 = local_5e8;
      if (piVar31 != (int *)0x0) {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c46;
        FUN_14019f2c0(piVar31 + -4);
        piVar30 = local_5e8;
      }
    }
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c57;
    iVar8 = FUN_142c4f6d0(DAT_143ac1898);
    if (iVar8 != 0) {
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c67;
      uVar9 = FUN_142cb8460(piVar30);
      piVar31 = local_5e8;
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c72;
      uVar18 = FUN_142cb8480(piVar31);
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c7a;
      cVar6 = FUN_14057e4c0();
      if (cVar6 != '\0') {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31c87;
        uVar22 = FUN_1404c6160();
        local_5b0 = &local_528;
        local_558 = (int *)0x0;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31cb2;
        uVar18 = FUN_14019ba10(&local_558,&DAT_143274450,uVar18);
        local_528 = (int *)0x0;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31cc8;
        FUN_14019a260(&local_528,uVar18);
        local_5d8 = (int *)0x0;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31cdd;
        pcVar23 = (char *)FUN_14019bd40(&local_5d8,7);
        piVar31 = local_5d8;
        *(undefined4 *)pcVar23 = s_nexonsn_14328e4a8._0_4_;
        *(undefined2 *)(pcVar23 + 4) = s_nexonsn_14328e4a8._4_2_;
        pcVar23[6] = s_nexonsn_14328e4a8[6];
        if (local_5d8[-4] != -1) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31d10;
          FUN_142e52dd0(0x8b);
        }
        iVar8 = piVar31[-3];
        if (iVar8 < 7) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31d28;
          FUN_142e54290(0x90,iVar8,7);
        }
        piVar31[-4] = 1;
        *(undefined1 *)((longlong)local_5d8 + 7) = 0;
        if (piVar31[-3] + 1 < 8) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31d53;
          FUN_142e54290(0x9c,7);
        }
        piVar31[-2] = 7;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31d6f;
        FUN_1404c7490(uVar22,0,&local_5d8,&local_528);
        if (local_5d8 != (int *)0x0) {
          piVar31 = local_5d8 + -4;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31d82;
          FUN_14019f2c0(piVar31);
        }
        if (local_558 != (int *)0x0) {
          piVar31 = local_558 + -4;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31d98;
          FUN_14019f2c0(piVar31);
        }
      }
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31d9e;
      cVar6 = FUN_14057e4c0();
      if (cVar6 != '\0') {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31dab;
        uVar18 = FUN_1404c6160();
        local_5b0 = &local_5a0;
        local_5c0 = (int *)0x0;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31dcd;
        uVar22 = FUN_14019ba10(&local_5c0,&DAT_143274450,uVar9);
        local_5a0 = (int *)0x0;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31ddd;
        FUN_14019a260(&local_5a0,uVar22);
        local_598 = 0;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31df2;
        pcVar23 = (char *)FUN_14019bd40(&local_598,9);
        lVar16 = local_598;
        *(undefined8 *)pcVar23 = s_accountno_14328e448._0_8_;
        pcVar23[8] = s_accountno_14328e448[8];
        if (*(int *)(local_598 + -0x10) != -1) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31e1e;
          FUN_142e52dd0(0x8b);
        }
        iVar8 = *(int *)(lVar16 + -0xc);
        if (iVar8 < 9) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31e36;
          FUN_142e54290(0x90,iVar8,9);
        }
        *(undefined4 *)(lVar16 + -0x10) = 1;
        *(undefined1 *)(local_598 + 9) = 0;
        if (*(int *)(lVar16 + -0xc) + 1 < 10) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31e61;
          FUN_142e54290(0x9c,9);
        }
        *(undefined4 *)(lVar16 + -8) = 9;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31e7a;
        FUN_1404c7490(uVar18,0,&local_598,&local_5a0);
        if (local_598 != 0) {
          lVar16 = local_598 + -0x10;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31e8d;
          FUN_14019f2c0(lVar16);
        }
        if (local_5c0 != (int *)0x0) {
          piVar31 = local_5c0 + -4;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31ea0;
          FUN_14019f2c0(piVar31);
        }
      }
      *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31ea6;
      cVar6 = FUN_14057e4c0();
      if (cVar6 != '\0') {
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31eb3;
        uVar18 = FUN_1404c6160();
        local_5c8 = 0;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31eca;
        pcVar23 = (char *)FUN_14019bd40(&local_5c8,0xe);
        lVar16 = local_5c8;
        *(undefined8 *)pcVar23 = s_GC_SelectWorld_1433fe310._0_8_;
        *(undefined4 *)(pcVar23 + 8) = s_GC_SelectWorld_1433fe310._8_4_;
        *(undefined2 *)(pcVar23 + 0xc) = s_GC_SelectWorld_1433fe310._12_2_;
        if (*(int *)(local_5c8 + -0x10) != -1) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31f00;
          FUN_142e52dd0(0x8b);
        }
        iVar8 = *(int *)(lVar16 + -0xc);
        if (iVar8 < 0xe) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31f18;
          FUN_142e54290(0x90,iVar8,0xe);
        }
        *(undefined4 *)(lVar16 + -0x10) = 1;
        *(undefined1 *)(local_5c8 + 0xe) = 0;
        if (*(int *)(lVar16 + -0xc) + 1 < 0xf) {
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31f43;
          FUN_142e54290(0x9c,0xe);
        }
        *(undefined4 *)(lVar16 + -8) = 0xe;
        *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31f5b;
        FUN_1404c7800(uVar18,0xd,&local_5c8);
        if (local_5c8 != 0) {
          lVar16 = local_5c8 + -0x10;
          *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31f6e;
          FUN_14019f2c0(lVar16);
        }
      }
    }
  }
LAB_141b31f6f:
  if (piVar14 != (int *)0x0) {
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31f7d;
    FUN_1401bebb0(piVar14 + -4);
  }
  puVar27 = auStack_638 + lVar4;
  if (local_518 != 0) {
    *(undefined8 *)(auStack_638 + lVar4 + -8) = 0x141b31f93;
    FUN_14019f2c0(local_518 + -0x10);
    puVar27 = auStack_638 + lVar4;
  }
LAB_141b31f94:
  *(undefined8 *)(puVar27 + -8) = 0x141b31fa3;
  return;
}



//===========================================================
// FUN_1415daff0 @ 1415daff0   (99 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1415daff0(undefined8 param_1)

{
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  FUN_1406ed520(local_468,0xbf);
  FUN_1415d3990(param_1,local_468);
  FUN_1406ed610(local_468);
  return;
}


