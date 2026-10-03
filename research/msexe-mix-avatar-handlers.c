
//===========================================================
// FUN_14127e090 @ 14127e090   (1809 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x00014127e54b) */

void FUN_14127e090(longlong param_1,int param_2,uint param_3,undefined8 param_4)

{
  int *piVar1;
  char cVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int iVar5;
  int iVar6;
  longlong lVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  int *piVar10;
  undefined4 *puVar11;
  int *piVar12;
  uint uVar13;
  undefined4 *puVar14;
  ulonglong uVar15;
  uint uVar16;
  longlong *plVar17;
  longlong lVar18;
  longlong *plVar19;
  undefined1 auStack_5b8 [32];
  undefined4 *local_598;
  undefined4 local_590;
  undefined8 *local_588;
  undefined1 *local_580;
  undefined4 local_578;
  undefined4 local_570;
  undefined1 local_568;
  undefined1 local_567 [3];
  uint local_564;
  undefined4 local_560;
  int *local_558;
  int local_550;
  undefined8 local_548;
  uint local_540;
  undefined4 local_538;
  undefined1 local_534;
  undefined4 *local_530;
  undefined8 local_520;
  int *local_518;
  longlong local_510;
  longlong local_508 [2];
  longlong *local_4f8;
  longlong local_4f0;
  undefined1 *local_4e8;
  longlong *local_4d8;
  undefined1 local_4d0 [8];
  undefined8 local_4c8;
  undefined8 *local_4c0;
  int *local_4b8;
  undefined1 local_4a8 [1104];
  ulonglong local_58;
  
  local_58 = DAT_143a8b908 ^ (ulonglong)auStack_5b8;
  puVar14 = (undefined4 *)0x0;
  local_564 = 0;
  if (DAT_143aa84a0 == 0) {
    return;
  }
  local_550 = param_2;
  local_540 = param_3;
  lVar7 = FUN_142cbe730();
  if (lVar7 == 0) {
    return;
  }
  uVar3 = FUN_1406e8c20(param_4);
  FUN_1406e9050(param_4,&local_510);
  FUN_1406e9170(param_4,local_567,1);
  local_568 = local_567[0];
  uVar4 = FUN_1406e8c20(param_4);
  local_538 = 0;
  local_530 = (undefined4 *)0x0;
  local_534 = local_568;
  lVar7 = FUN_14019b780(&DAT_143ad68a0,0xc);
  puVar11 = (undefined4 *)(lVar7 + 8);
  if (lVar7 == 0) {
    puVar11 = puVar14;
  }
  if (local_530 != (undefined4 *)0x0) {
    thunk_FUN_140205820((longlong)local_530 + -8,0);
  }
  *(undefined8 *)(puVar11 + -2) = 0;
  *(longlong *)(puVar11 + -2) = *(longlong *)(puVar11 + -2) + 1;
  *puVar11 = uVar4;
  local_530 = puVar11;
  iVar5 = FUN_1401a8170(uVar3);
  uVar8 = 0;
  local_560 = 0;
  if (iVar5 == 0xe) {
    puVar14 = (undefined4 *)0x19;
    local_560 = 0x40;
  }
  else if (iVar5 == 0x18) {
    puVar14 = (undefined4 *)0x17;
    local_560 = 0x2a;
  }
  local_508[0] = FUN_14019b780(&DAT_143ad68a0,2000);
  if (local_508[0] != 0) {
    uVar8 = FUN_142a57d30(local_508[0],0,0,0);
  }
  lVar7 = FUN_141280700(param_1 + 0x10,uVar8);
  plVar19 = *(longlong **)(lVar7 + 8);
  local_4d8 = plVar19;
  if (plVar19 != (longlong *)0x0) {
    if (0xfffff < (ulonglong)plVar19[4]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    plVar19[4] = plVar19[4] + 1;
    UNLOCK();
  }
  plVar17 = local_4d8;
  if (plVar19 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  uVar16 = 0;
  local_548 = 0;
  FUN_14019a260(&local_548,&local_510);
  local_588 = (undefined8 *)((ulonglong)local_588 & 0xffffffffffffff00);
  local_598 = &local_538;
  local_590 = uVar3;
  FUN_142a8a3e0(plVar17,puVar14,param_3,&local_548);
  if (plVar17 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_142a5ee30(plVar17);
  if (plVar17 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  iVar6 = (**(code **)(*plVar17 + 0x130))(plVar17);
  uVar13 = (uint)(iVar6 == 1);
  uVar3 = FUN_142a8a7d0(plVar17);
  if ((local_530 == (undefined4 *)0x0) || (uVar16 = local_530[-2], uVar16 < 2)) {
    FUN_142e54290(0xbc,1,uVar16);
  }
  uVar4 = local_530[1];
  uVar3 = FUN_1401a8660(iVar5,uVar4,uVar3);
  cVar2 = FUN_1401a8500(iVar5,uVar4,uVar3);
  if (cVar2 == '\0') goto LAB_14127e675;
  if (iVar5 == 0xe) {
    puVar9 = (undefined8 *)FUN_1408a9e40(local_508,0x475);
    uVar16 = 1;
  }
  else {
    puVar9 = (undefined8 *)FUN_1408a9e40(&local_4f0,0x479);
    uVar16 = 2;
  }
  piVar12 = (int *)*puVar9;
  *puVar9 = 0;
  local_564 = uVar16;
  local_518 = piVar12;
  if (((uVar16 & 2) != 0) && (uVar16 = uVar16 & 0xfffffffd, local_564 = uVar16, local_4f0 != 0)) {
    FUN_14019f2c0(local_4f0 + -0x10);
  }
  if (((uVar16 & 1) != 0) && (local_508[0] != 0)) {
    FUN_14019f2c0(local_508[0] + -0x10);
  }
  local_4e8 = (undefined1 *)FUN_14019b780(&DAT_143ad68a0,2000);
  lVar7 = 0;
  if (local_4e8 != (undefined1 *)0x0) {
    lVar7 = FUN_142a57d30(local_4e8,0,0,0);
  }
  lVar18 = lVar7 + 0x18;
  if (lVar7 == 0) {
    lVar18 = 0;
  }
  if (lVar18 == 0) {
    local_4f8 = (longlong *)0x0;
  }
  else {
    local_4f8 = (longlong *)(lVar18 + -0x18);
    if (local_4f8 != (longlong *)0x0) {
      if (0xfffff < *(ulonglong *)(lVar18 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar18 + 8) = *(longlong *)(lVar18 + 8) + 1;
      UNLOCK();
      piVar12 = local_518;
    }
  }
  plVar19 = local_4f8;
  if (local_4f8 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  local_4e8 = local_4d0;
  local_4c8 = 0;
  local_4c0 = &local_520;
  local_520 = 0;
  local_558 = (int *)0x0;
  piVar1 = local_558;
  if ((piVar12 != (int *)0x0) && (piVar10 = piVar12 + -4, piVar10 != (int *)0x0)) {
    if (*piVar10 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar15 = 0xffffffffffffffff;
      do {
        uVar15 = uVar15 + 1;
      } while (*(char *)((longlong)piVar12 + uVar15) != '\0');
      iVar6 = (int)uVar15;
      iVar5 = 0;
      if (0 < iVar6) {
        iVar5 = iVar6;
      }
      piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
      piVar10[1] = iVar5;
      *piVar10 = -1;
      piVar1 = piVar10 + 4;
      piVar10[2] = 0;
      *(undefined1 *)piVar1 = 0;
      local_4b8 = piVar1;
      FUN_142ef7ba0(piVar1,piVar12,(longlong)iVar6);
      if (*piVar10 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar6 == -1) || (iVar6 <= piVar10[1])) {
        *piVar10 = 1;
        if (iVar6 != -1) goto LAB_14127e4dd;
        if (piVar1 == (int *)0x0) {
          uVar15 = 0;
        }
        else {
          uVar15 = 0xffffffffffffffff;
          do {
            uVar15 = uVar15 + 1;
          } while (*(char *)((longlong)piVar1 + uVar15) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar10[1],uVar15 & 0xffffffff);
        *piVar10 = 1;
LAB_14127e4dd:
        *(undefined1 *)((longlong)piVar1 + (longlong)iVar6) = 0;
      }
      iVar5 = (int)uVar15;
      if ((iVar5 < 0) || (piVar10[1] + 1 <= iVar5)) {
        FUN_142e54290(0x9c,uVar15 & 0xffffffff);
      }
      piVar10[2] = iVar5;
      param_3 = local_540;
      if (local_558 != (int *)0x0) {
        FUN_14019f2c0(local_558 + -4);
        param_3 = local_540;
      }
    }
    else {
      if (*piVar10 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar10 = *piVar10 + 1;
      UNLOCK();
      plVar19 = local_4f8;
      piVar1 = piVar12;
      if (local_558 != (int *)0x0) {
        FUN_14019f2c0(local_558 + -4);
        plVar19 = local_4f8;
      }
    }
  }
  local_558 = piVar1;
  uVar13 = 0;
  uVar16 = uVar13;
  if (local_550 == 4) {
    uVar16 = param_3;
  }
  local_570 = 0;
  local_578 = 0;
  local_580 = local_4d0;
  local_588 = &local_520;
  local_590 = 0;
  local_598 = (undefined4 *)((ulonglong)local_598 & 0xffffffff00000000);
  FUN_142a61900(plVar19,0,uVar16,&local_558);
  if (plVar19 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_142a62bf0(plVar19,0,0);
  if (plVar19 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_142a5ee30(plVar19);
  if (plVar19 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  (**(code **)(*plVar19 + 0x130))(plVar19);
  if (0xffffe < plVar19[4] - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar19 = plVar19 + 4;
  lVar7 = *plVar19;
  *plVar19 = *plVar19 + -1;
  UNLOCK();
  if (((int)lVar7 == 1) && (plVar19 = local_4f8 + 3, plVar19 != (longlong *)0x0)) {
    (**(code **)*plVar19)(plVar19,1);
  }
  plVar17 = local_4d8;
  if (local_518 != (int *)0x0) {
    FUN_14019f2c0(local_518 + -4);
    plVar17 = local_4d8;
  }
LAB_14127e675:
  FUN_1406ed520(local_4a8,0xf3);
  FUN_1406ed9d0(local_4a8,0);
  FUN_1406ed840(local_4a8,(undefined1)local_560);
  FUN_1406ed840(local_4a8,uVar13);
  if (uVar13 != 0) {
    uVar3 = FUN_142a8a7d0(plVar17);
    FUN_1406ede20(local_4a8,&local_568,1);
    FUN_1406ed840(local_4a8,0);
    uVar16 = 0;
    if (local_550 == 4) {
      uVar16 = param_3;
    }
    FUN_1406ed9d0(local_4a8,uVar16);
    FUN_1406ed9d0(local_4a8,uVar3);
  }
  FUN_1415d01c0(local_4a8);
  FUN_140da25d0(param_1 + 0x10);
  FUN_1406ed610(local_4a8);
  if (0xffffe < plVar17[4] - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar17 = plVar17 + 4;
  lVar7 = *plVar17;
  *plVar17 = *plVar17 + -1;
  UNLOCK();
  if (((int)lVar7 == 1) && (plVar19 = local_4d8 + 3, plVar19 != (longlong *)0x0)) {
    (**(code **)*plVar19)(plVar19,1);
  }
  if (local_530 != (undefined4 *)0x0) {
    thunk_FUN_140205820(local_530 + -2,0);
    local_530 = (undefined4 *)0x0;
  }
  if (local_510 != 0) {
    FUN_14019f2c0(local_510 + -0x10);
  }
  return;
}



//===========================================================
// FUN_14127e7b0 @ 14127e7b0   (1351 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14127e7b0(longlong param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4)

{
  longlong *plVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined4 *puVar7;
  longlong *plVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  undefined4 *puVar11;
  uint uVar12;
  int iVar13;
  ulonglong uVar14;
  undefined4 *puVar15;
  undefined1 auStack_558 [32];
  undefined8 local_538;
  int local_530;
  longlong *local_528;
  undefined1 *local_520;
  undefined4 local_518;
  undefined4 local_510;
  undefined1 local_508 [8];
  ulonglong local_500;
  undefined4 local_4f8;
  undefined1 local_4f4;
  undefined4 *local_4f0;
  undefined8 local_4e0;
  longlong local_4d8;
  longlong local_4d0;
  longlong *local_4c8;
  longlong *local_4b8;
  undefined1 local_4b0 [8];
  undefined8 local_4a8;
  undefined1 *local_4a0;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_558;
  local_500 = CONCAT44(local_500._4_4_,param_3);
  if (DAT_143aa84a0 == 0) {
    return;
  }
  lVar5 = FUN_142cbe730();
  if (lVar5 == 0) {
    return;
  }
  FUN_1406e9050(param_4,&local_4d8);
  iVar2 = FUN_1406e8c20(param_4);
  if (iVar2 == 0) {
    local_4d0 = FUN_14019b780(&DAT_143ad68a0,2000);
    uVar6 = 0;
    if (local_4d0 != 0) {
      uVar6 = FUN_142a57d30(local_4d0,0,0,0);
    }
    lVar5 = FUN_141280700(param_1 + 0x10,uVar6);
    plVar8 = *(longlong **)(lVar5 + 8);
    local_4b8 = plVar8;
    if (plVar8 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar8[4]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar8[4] = plVar8[4] + 1;
      UNLOCK();
    }
    plVar1 = local_4b8;
    if (plVar8 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_4a0 = local_4b0;
    local_4a8 = 0;
    local_4c8 = &local_4d0;
    local_4d0 = 0;
    local_500 = 0;
    FUN_14019a260(&local_500,&local_4d8);
    local_510 = 0;
    local_518 = 0;
    local_520 = local_4b0;
    local_528 = &local_4d0;
    local_530 = 0;
    local_538 = (undefined4 *)((ulonglong)local_538._4_4_ << 0x20);
    FUN_142a61900(plVar1,0,param_3,&local_500);
    if (plVar1 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a62bf0(plVar1,0,0);
    if (plVar1 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a5ee30(plVar1);
    if (plVar1 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar1 + 0x130))(plVar1);
    if (0xffffe < plVar1[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = plVar1 + 4;
    lVar5 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar5 == 1) && (plVar8 = local_4b8 + 3, plVar8 != (longlong *)0x0)) {
      (**(code **)*plVar8)(plVar8,1);
    }
    goto LAB_14127ec6e;
  }
  FUN_1406e9170(param_4,local_508,1);
  uVar3 = FUN_1406e8c20(param_4);
  uVar4 = FUN_1406e8c20(param_4);
  FUN_1406e8c20(param_4);
  FUN_1406e8c20(param_4);
  puVar7 = (undefined4 *)0x0;
  uVar12 = 0;
  local_4f8 = 0;
  local_4f0 = (undefined4 *)0x0;
  local_4f4 = local_508[0];
  lVar5 = FUN_14019b780(&DAT_143ad68a0,0xc);
  puVar11 = (undefined4 *)(lVar5 + 8);
  if (lVar5 == 0) {
    puVar11 = puVar7;
  }
  if (local_4f0 != (undefined4 *)0x0) {
    thunk_FUN_140205820((longlong)local_4f0 + -8,0);
  }
  *(undefined8 *)(puVar11 + -2) = 0;
  *(longlong *)(puVar11 + -2) = *(longlong *)(puVar11 + -2) + 1;
  *puVar11 = uVar3;
  iVar13 = 0;
  local_4f0 = puVar11;
  if (puVar11 == (undefined4 *)0x0) {
    uVar14 = 1;
LAB_14127ea82:
    if (puVar11 != (undefined4 *)0x0) {
      uVar9 = *(ulonglong *)(puVar11 + -4);
      uVar10 = ~uVar9;
      if (-1 < (longlong)uVar9) {
        uVar10 = uVar9;
      }
      iVar13 = (int)(uVar10 - 8 >> 2);
    }
    if (iVar13 != (int)uVar14) {
      puVar15 = puVar7;
      if (puVar11 != (undefined4 *)0x0) {
        puVar15 = (undefined4 *)(ulonglong)(uint)puVar11[-2];
      }
      lVar5 = FUN_14019b780(&DAT_143ad68a0,uVar14 * 4 + 8);
      puVar11 = (undefined4 *)(lVar5 + 8);
      if (lVar5 == 0) {
        puVar11 = puVar7;
      }
      if (local_4f0 != (undefined4 *)0x0) {
        FUN_142ef7ba0(puVar11,local_4f0,(longlong)puVar15 << 2);
        thunk_FUN_140205820(local_4f0 + -2,0);
      }
      *(undefined4 **)(puVar11 + -2) = puVar15;
      local_4f0 = puVar11;
    }
  }
  else {
    uVar12 = puVar11[-2];
    uVar14 = *(ulonglong *)(puVar11 + -4);
    uVar9 = ~uVar14;
    if (-1 < (longlong)uVar14) {
      uVar9 = uVar14;
    }
    if ((uint)(uVar9 - 8 >> 2) <= uVar12) {
      if (uVar12 == 0) {
        uVar14 = 1;
      }
      else {
        uVar14 = (ulonglong)(uVar12 * 2);
      }
      goto LAB_14127ea82;
    }
  }
  *(longlong *)(local_4f0 + -2) = *(longlong *)(local_4f0 + -2) + 1;
  local_4f0[(int)uVar12] = uVar4;
  local_4c8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,2000);
  if (local_4c8 != (longlong *)0x0) {
    puVar7 = (undefined4 *)FUN_142a57d30(local_4c8,0,0,0);
  }
  lVar5 = FUN_141280700(param_1 + 0x10,puVar7);
  plVar8 = *(longlong **)(lVar5 + 8);
  local_4b8 = plVar8;
  if (plVar8 != (longlong *)0x0) {
    if (0xfffff < (ulonglong)plVar8[4]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    plVar8[4] = plVar8[4] + 1;
    UNLOCK();
  }
  plVar1 = local_4b8;
  if (plVar8 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  local_4e0 = 0;
  FUN_14019a260(&local_4e0,&local_4d8);
  local_528 = (longlong *)((ulonglong)local_528 & 0xffffffffffffff00);
  local_538 = &local_4f8;
  local_530 = iVar2;
  FUN_142a8a3e0(plVar1,0x18,local_500 & 0xffffffff,&local_4e0);
  if (plVar1 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_142a5ee30(plVar1);
  if (plVar1 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  (**(code **)(*plVar1 + 0x130))(plVar1);
  if (0xffffe < plVar1[4] - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar1 = plVar1 + 4;
  lVar5 = *plVar1;
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if (((int)lVar5 == 1) && (plVar8 = local_4b8 + 3, plVar8 != (longlong *)0x0)) {
    (**(code **)*plVar8)(plVar8,1);
  }
  if (local_4f0 != (undefined4 *)0x0) {
    thunk_FUN_140205820(local_4f0 + -2,0);
  }
LAB_14127ec6e:
  FUN_1406ed520(local_498,0xf3);
  FUN_1406ed9d0(local_498,0);
  FUN_1406ed840(local_498,0x2b);
  FUN_1406ed840(local_498,1);
  FUN_1415d01c0(local_498);
  FUN_140da25d0(param_1 + 0x10);
  FUN_1406ed610(local_498);
  if (local_4d8 != 0) {
    FUN_14019f2c0(local_4d8 + -0x10);
  }
  return;
}



//===========================================================
// FUN_14127ed00 @ 14127ed00   (261 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14127ed00(undefined8 param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4)

{
  undefined4 uVar1;
  int iVar2;
  undefined1 auStack_4a8 [32];
  undefined4 *local_488;
  undefined1 local_478 [4];
  undefined4 local_474;
  undefined4 local_470 [2];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  uVar1 = FUN_1406e8c20(param_4);
  FUN_1406e9170(param_4,local_478,1);
  FUN_1406e9170(param_4,local_478,1);
  local_470[0] = FUN_1406e8c20(param_4);
  local_474 = 0;
  local_488 = &local_474;
  iVar2 = FUN_142dcbbb0(uVar1,local_478[0],param_3,local_470);
  FUN_1406ed520(local_468,0xf3);
  FUN_1406ed9d0(local_468,0);
  FUN_1406ed840(local_468,0x39);
  FUN_1406ed840(local_468,(iVar2 - 1U & 0xffffdfff) == 0);
  FUN_1415d01c0(local_468);
  FUN_1406ed610(local_468);
  return;
}



//===========================================================
// FUN_14127ee10 @ 14127ee10   (622 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14127ee10(longlong param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4)

{
  longlong *plVar1;
  undefined4 uVar2;
  int iVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong *plVar6;
  undefined1 auStack_518 [32];
  undefined4 *local_4f8;
  undefined4 local_4f0;
  undefined1 local_4e8;
  undefined1 local_4d8 [8];
  longlong local_4d0;
  longlong local_4c8;
  undefined4 local_4c0;
  undefined1 local_4bc;
  longlong local_4b8;
  longlong *local_4a0;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_518;
  if ((DAT_143aa84a0 != 0) && (lVar4 = FUN_142cbe730(), lVar4 != 0)) {
    uVar2 = FUN_1406e8c20(param_4);
    FUN_1406e9170(param_4,local_4d8,1);
    FUN_1406e9050(param_4,&local_4c8);
    uVar5 = 0;
    local_4c0 = 0;
    local_4b8 = 0;
    local_4bc = local_4d8[0];
    local_4d0 = FUN_14019b780(&DAT_143ad68a0,2000);
    if (local_4d0 != 0) {
      uVar5 = FUN_142a57d30(local_4d0,0,0,0);
    }
    lVar4 = FUN_141280700(param_1 + 0x10,uVar5);
    plVar6 = *(longlong **)(lVar4 + 8);
    local_4a0 = plVar6;
    if (plVar6 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar6[4]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar6[4] = plVar6[4] + 1;
      UNLOCK();
    }
    plVar1 = local_4a0;
    if (plVar6 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_4d0 = 0;
    FUN_14019a260(&local_4d0,&local_4c8);
    local_4e8 = 0;
    local_4f8 = &local_4c0;
    local_4f0 = uVar2;
    FUN_142a8a3e0(plVar1,0x1b,param_3,&local_4d0);
    if (plVar1 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a5ee30(plVar1);
    if (plVar1 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar3 = (**(code **)(*plVar1 + 0x130))(plVar1);
    FUN_1406ed520(local_498,0xf3);
    FUN_1406ed9d0(local_498,0);
    FUN_1406ed840(local_498,0x3b);
    FUN_1406ed840(local_498,iVar3 == 1);
    FUN_1415d01c0(local_498);
    FUN_140da25d0(param_1 + 0x10);
    FUN_1406ed610(local_498);
    if (0xffffe < plVar1[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = plVar1 + 4;
    lVar4 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar4 == 1) && (plVar6 = local_4a0 + 3, plVar6 != (longlong *)0x0)) {
      (**(code **)*plVar6)(plVar6,1);
    }
    if (local_4b8 != 0) {
      thunk_FUN_140205820(local_4b8 + -8,0);
      local_4b8 = 0;
    }
    if (local_4c8 != 0) {
      FUN_14019f2c0(local_4c8 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_14127f090 @ 14127f090   (1282 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14127f090(longlong param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4)

{
  longlong *plVar1;
  byte bVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  longlong lVar6;
  longlong *plVar7;
  undefined8 *puVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  uint uVar13;
  uint uVar14;
  undefined1 auStack_578 [32];
  undefined4 local_558;
  undefined4 local_550;
  longlong *local_548;
  undefined1 *local_540;
  undefined4 local_538;
  undefined4 local_530;
  undefined8 local_528;
  longlong local_520;
  undefined4 local_518;
  undefined1 local_514;
  ulonglong local_510;
  undefined8 local_508;
  undefined4 local_500;
  longlong local_4f8;
  longlong local_4f0 [2];
  longlong *local_4e0;
  undefined1 local_4d8 [8];
  undefined8 local_4d0;
  undefined1 local_4c8 [8];
  longlong local_4c0;
  undefined1 local_4b8 [8];
  undefined1 *local_4b0;
  longlong *local_4a8;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_578;
  local_520 = param_1;
  local_500 = param_3;
  uVar3 = FUN_1406e8c20(param_4);
  local_528 = CONCAT44(local_528._4_4_,uVar3);
  FUN_1406e9050(param_4,local_4f0);
  if (((DAT_143aa84a0 == 0) || (lVar6 = FUN_142cbe730(), lVar6 == 0)) || (DAT_143aa8518 == 0))
  goto LAB_14127f555;
  iVar5 = 2;
  uVar9 = 0;
  FUN_1427be040(DAT_143aa8518,local_4c8);
  if (((local_4c0 == 0) ||
      (plVar7 = (longlong *)FUN_140192f00(*(undefined8 *)(lVar6 + 0x360)), plVar7 == (longlong *)0x0
      )) || (iVar4 = FUN_14019a5d0(plVar7 + 4), 9999 < iVar4 - 0x195460U)) {
LAB_14127f4e5:
    FUN_1406ed520(local_498,0xf3);
    FUN_1406ed9d0(local_498,0);
    FUN_1406ed840(local_498,0xb);
    if (iVar5 == 1) {
      FUN_1406ed840(local_498,1);
    }
    else {
      uVar9 = 0;
    }
    FUN_1406ed840(local_498,uVar9 & 0xff);
    FUN_1415d01c0(local_498);
    FUN_140da25d0(param_1 + 0x10);
    FUN_1406ed610(local_498);
  }
  else {
    local_518 = 0;
    local_510 = 0;
    local_514 = 100;
    puVar8 = (undefined8 *)(**(code **)(*plVar7 + 0x80))(plVar7,local_4b8);
    local_508 = *puVar8;
    FUN_1406e9050(param_4,&local_4f8);
    bVar2 = FUN_1406e8ae0(param_4);
    uVar14 = (uint)bVar2;
    if (bVar2 != 0) {
      do {
        uVar3 = FUN_1406e8c20(param_4);
        if (local_510 == 0) {
          uVar12 = 1;
          uVar13 = 0;
LAB_14127f1ff:
          iVar5 = 0;
          if (local_510 != 0) {
            uVar10 = *(ulonglong *)(local_510 - 0x10);
            uVar11 = ~uVar10;
            if (-1 < (longlong)uVar10) {
              uVar11 = uVar10;
            }
            iVar5 = (int)(uVar11 - 8 >> 2);
          }
          if (iVar5 != (int)uVar12) {
            uVar10 = uVar9;
            if (local_510 != 0) {
              uVar10 = (ulonglong)*(uint *)(local_510 - 8);
            }
            lVar6 = FUN_14019b780(&DAT_143ad68a0,uVar12 * 4 + 8);
            uVar12 = lVar6 + 8;
            if (lVar6 == 0) {
              uVar12 = uVar9;
            }
            if (local_510 != 0) {
              FUN_142ef7ba0(uVar12,local_510,uVar10 << 2);
              thunk_FUN_140205820(local_510 - 8,0);
            }
            *(ulonglong *)(uVar12 - 8) = uVar10;
            local_510 = uVar12;
          }
        }
        else {
          uVar13 = *(uint *)(local_510 - 8);
          uVar12 = *(ulonglong *)(local_510 - 0x10);
          uVar10 = ~uVar12;
          if (-1 < (longlong)uVar12) {
            uVar10 = uVar12;
          }
          if ((uint)(uVar10 - 8 >> 2) <= uVar13) {
            if (uVar13 == 0) {
              uVar12 = 1;
            }
            else {
              uVar12 = (ulonglong)(uVar13 * 2);
            }
            goto LAB_14127f1ff;
          }
        }
        *(longlong *)(local_510 - 8) = *(longlong *)(local_510 - 8) + 1;
        *(undefined4 *)(local_510 + (longlong)(int)uVar13 * 4) = uVar3;
        uVar14 = uVar14 - 1;
      } while (0 < (int)uVar14);
      param_1 = local_520;
      uVar3 = (undefined4)local_528;
    }
    local_520 = FUN_14019b780(&DAT_143ad68a0,2000);
    if (local_520 != 0) {
      uVar9 = FUN_142a57d30(local_520,0,0,0);
    }
    lVar6 = FUN_141280700(param_1 + 0x10,uVar9);
    plVar7 = *(longlong **)(lVar6 + 8);
    local_4e0 = plVar7;
    if (plVar7 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar7[4]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar7[4] = plVar7[4] + 1;
      UNLOCK();
    }
    plVar1 = local_4e0;
    if (plVar7 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_4b0 = local_4d8;
    local_4d0 = 0;
    local_4a8 = &local_520;
    local_520 = 0;
    local_528 = 0;
    FUN_14019a260(&local_528,&local_4f8);
    local_530 = 0;
    local_538 = 0;
    local_540 = local_4d8;
    local_548 = &local_520;
    local_550 = 0;
    local_558 = 0;
    FUN_142a61900(plVar1,0x14,local_500,&local_528);
    if (plVar1 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a8a190(plVar1,&local_518,uVar3,0);
    if (plVar1 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a5ee30(plVar1);
    if (plVar1 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar5 = (**(code **)(*plVar1 + 0x130))(plVar1);
    if (iVar5 != 3) {
      uVar14 = FUN_142a8a7c0(plVar1);
      uVar9 = (ulonglong)uVar14;
      if (0xffffe < plVar1[4] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = plVar1 + 4;
      lVar6 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if (((int)lVar6 == 1) && (plVar7 = local_4e0 + 3, plVar7 != (longlong *)0x0)) {
        (**(code **)*plVar7)(plVar7,1);
      }
      if (local_4f8 != 0) {
        FUN_14019f2c0(local_4f8 + -0x10);
      }
      if (local_510 != 0) {
        thunk_FUN_140205820(local_510 - 8,0);
      }
      goto LAB_14127f4e5;
    }
    if (0xffffe < plVar1[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = plVar1 + 4;
    lVar6 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (plVar7 = local_4e0 + 3, plVar7 != (longlong *)0x0)) {
      (**(code **)*plVar7)(plVar7,1);
    }
    if (local_4f8 != 0) {
      FUN_14019f2c0(local_4f8 + -0x10);
    }
    if (local_510 != 0) {
      thunk_FUN_140205820(local_510 - 8,0);
    }
  }
  FUN_140ce88a0(local_4c8);
LAB_14127f555:
  if (local_4f0[0] != 0) {
    FUN_14019f2c0(local_4f0[0] + -0x10);
  }
  return;
}



//===========================================================
// FUN_14127f5a0 @ 14127f5a0   (1488 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14127f5a0(longlong param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4)

{
  undefined4 **ppuVar1;
  longlong *plVar2;
  uint uVar3;
  undefined4 *puVar4;
  undefined1 *puVar5;
  byte bVar6;
  undefined4 uVar7;
  int iVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  undefined8 uVar11;
  longlong lVar12;
  longlong *plVar13;
  undefined1 *puVar14;
  longlong *plVar15;
  ulonglong uVar16;
  undefined1 *puVar17;
  undefined4 extraout_XMM0_Da;
  undefined1 auStack_598 [32];
  undefined4 local_578;
  undefined4 local_570;
  longlong *local_568;
  undefined1 *local_560;
  undefined4 local_558;
  undefined4 local_550;
  undefined1 local_548;
  undefined1 local_547;
  undefined4 *local_540;
  undefined8 local_538;
  longlong *local_530;
  longlong *plStack_528;
  longlong *local_520;
  undefined4 local_518 [2];
  longlong local_510;
  undefined4 local_508;
  undefined1 local_504;
  undefined4 *local_500 [2];
  longlong local_4f0;
  undefined1 *local_4e8;
  undefined1 *puStack_4e0;
  longlong local_4d8;
  longlong *local_4c8;
  undefined1 local_4c0 [8];
  undefined8 local_4b8;
  undefined1 *local_4b0;
  longlong *local_4a8;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_598;
  uVar7 = FUN_1406e8c20(param_4);
  FUN_1406e9050(param_4,&local_4f0);
  local_530 = (longlong *)0x0;
  plStack_528 = (longlong *)0x0;
  local_520 = (longlong *)0x0;
  bVar6 = FUN_1406e8ae0(param_4);
  if (bVar6 != 0) {
    uVar16 = (ulonglong)bVar6;
    do {
      local_540 = (undefined4 *)0x0;
      FUN_1401a75b0(&local_548,param_4);
      if (plStack_528 == local_520) {
        FUN_1412802a0(&local_530,plStack_528,&local_548);
      }
      else {
        *(undefined1 *)plStack_528 = local_548;
        *(undefined1 *)((longlong)plStack_528 + 1) = local_547;
        ppuVar1 = (undefined4 **)(plStack_528 + 1);
        *ppuVar1 = (undefined4 *)0x0;
        if ((ppuVar1 != &local_540) && (local_540 != (undefined4 *)0x0)) {
          uVar3 = local_540[-2];
          if (uVar3 != 0) {
            puVar9 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar3 * 4 + 8);
            if (puVar9 != (undefined4 *)0x0) {
              puVar9 = puVar9 + 2;
            }
            *ppuVar1 = puVar9;
            *(ulonglong *)(puVar9 + -2) = (ulonglong)uVar3;
            puVar9 = *ppuVar1;
            for (puVar10 = local_540; puVar10 < local_540 + uVar3; puVar10 = puVar10 + 1) {
              *puVar9 = *puVar10;
              puVar9 = puVar9 + 1;
            }
          }
        }
        plStack_528 = plStack_528 + 2;
      }
      if (local_540 != (undefined4 *)0x0) {
        thunk_FUN_140205820(local_540 + -2,0);
        local_540 = (undefined4 *)0x0;
      }
      uVar16 = uVar16 - 1;
    } while (uVar16 != 0);
  }
  local_508 = 0;
  local_504 = 0;
  local_500[0] = (undefined4 *)0x0;
  local_4e8 = (undefined1 *)0x0;
  puStack_4e0 = (undefined1 *)0x0;
  local_4d8 = 0;
  iVar8 = 2;
  if ((longlong)plStack_528 - (longlong)local_530 >> 4 == 1) {
    local_504 = (undefined1)*local_530;
    ppuVar1 = (undefined4 **)(local_530 + 1);
    if ((local_500 != ppuVar1) && (*ppuVar1 != (undefined4 *)0x0)) {
      uVar3 = (*ppuVar1)[-2];
      if (uVar3 != 0) {
        local_500[0] = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar3 * 4 + 8);
        if (local_500[0] != (undefined4 *)0x0) {
          local_500[0] = local_500[0] + 2;
        }
        *(ulonglong *)(local_500[0] + -2) = (ulonglong)uVar3;
        puVar10 = *ppuVar1;
        puVar9 = puVar10 + uVar3;
        puVar4 = local_500[0];
        for (; puVar10 < puVar9; puVar10 = puVar10 + 1) {
          *puVar4 = *puVar10;
          puVar4 = puVar4 + 1;
        }
      }
    }
    uVar11 = 0;
    local_510 = FUN_14019b780(&DAT_143ad68a0,2000);
    if (local_510 != 0) {
      uVar11 = FUN_142a57d30(local_510,0,0,0);
    }
    lVar12 = FUN_141280700(param_1 + 0x10,uVar11);
    plVar13 = *(longlong **)(lVar12 + 8);
    local_4c8 = plVar13;
    if (plVar13 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar13[4]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar13[4] = plVar13[4] + 1;
      UNLOCK();
    }
    plVar15 = local_4c8;
    if (plVar13 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_4b0 = local_4c0;
    local_4b8 = 0;
    local_4a8 = &local_510;
    local_510 = 0;
    local_538 = 0;
    FUN_14019a260(&local_538,&local_4f0);
    local_550 = 0;
    local_558 = 0;
    local_560 = local_4c0;
    local_568 = &local_510;
    local_570 = 0;
    local_578 = 0;
    FUN_142a61900(plVar15,0x14,param_3,&local_538);
    if (plVar15 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a8a190(plVar15,&local_508,uVar7,0);
    if (plVar15 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a5ee30(plVar15);
    if (plVar15 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar8 = (**(code **)(*plVar15 + 0x130))(plVar15);
    if (iVar8 == 3) {
      uVar7 = extraout_XMM0_Da;
      if (0xffffe < plVar15[4] - 1U) {
        uVar7 = FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar15 = plVar15 + 4;
      lVar12 = *plVar15;
      *plVar15 = *plVar15 + -1;
      UNLOCK();
      puVar17 = local_4e8;
      if (((int)lVar12 == 1) && (plVar13 = local_4c8 + 3, plVar13 != (longlong *)0x0)) {
        uVar7 = (**(code **)*plVar13)(plVar13,1);
        puVar17 = local_4e8;
      }
      goto LAB_14127fa4b;
    }
    local_518[0] = FUN_142a8a7c0(plVar15);
    FUN_1401d01c0(&local_4e8,0,local_518);
    if (0xffffe < plVar15[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar15 = plVar15 + 4;
    lVar12 = *plVar15;
    *plVar15 = *plVar15 + -1;
    UNLOCK();
    if (((int)lVar12 == 1) && (plVar13 = local_4c8 + 3, plVar13 != (longlong *)0x0)) {
      (**(code **)*plVar13)(plVar13,1);
    }
  }
  FUN_1406ed520(local_498,0xf3);
  FUN_1406ed9d0(local_498,0);
  FUN_1406ed840(local_498,0x34);
  puVar17 = local_4e8;
  if (iVar8 == 1) {
    FUN_1406ed840(local_498,1);
    puVar5 = puStack_4e0;
    FUN_1406ed840(local_498,(longlong)puStack_4e0 - (longlong)puVar17 >> 2);
    for (puVar14 = puVar17; puVar14 != puVar5; puVar14 = puVar14 + 4) {
      FUN_1406ed840(local_498,*puVar14);
    }
  }
  else {
    FUN_1406ed840(local_498,0);
  }
  FUN_1415d01c0(local_498);
  FUN_140da25d0(param_1 + 0x10);
  uVar7 = FUN_1406ed610(local_498);
LAB_14127fa4b:
  if (puVar17 != (undefined1 *)0x0) {
    uVar16 = (local_4d8 - (longlong)puVar17 >> 2) * 4;
    puVar14 = puVar17;
    if (0xfff < uVar16) {
      puVar14 = *(undefined1 **)(puVar17 + -8);
      if ((undefined1 *)0x1f < puVar17 + (-8 - (longlong)puVar14)) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(uVar7,uVar16 + 0x27);
      }
    }
    thunk_FUN_140205820(puVar14);
  }
  if (local_500[0] != (undefined4 *)0x0) {
    thunk_FUN_140205820(local_500[0] + -2,0);
    local_500[0] = (undefined4 *)0x0;
  }
  plVar13 = plStack_528;
  if (local_530 != (longlong *)0x0) {
    if (local_530 != plStack_528) {
      plVar15 = local_530 + 1;
      do {
        if (*plVar15 != 0) {
          thunk_FUN_140205820(*plVar15 + -8,0);
          *plVar15 = 0;
        }
        plVar2 = plVar15 + 1;
        plVar15 = plVar15 + 2;
      } while (plVar2 != plVar13);
    }
    uVar16 = (longlong)local_520 - (longlong)local_530 & 0xfffffffffffffff0;
    if (0xfff < uVar16) {
      if (0x1f < (ulonglong)((longlong)local_530 + (-8 - local_530[-1]))) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(local_530[-1],uVar16 + 0x27);
      }
    }
    thunk_FUN_140205820();
    local_530 = (longlong *)0x0;
    plStack_528 = (longlong *)0x0;
    local_520 = (longlong *)0x0;
  }
  if (local_4f0 != 0) {
    FUN_14019f2c0(local_4f0 + -0x10);
  }
  return;
}



//===========================================================
// FUN_14127fb80 @ 14127fb80   (1814 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14127fb80(longlong param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4)

{
  longlong *plVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined1 *puVar4;
  byte bVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  longlong lVar11;
  longlong *plVar12;
  undefined8 *puVar13;
  undefined4 **ppuVar14;
  longlong lVar15;
  undefined1 *puVar16;
  longlong *plVar17;
  ulonglong uVar18;
  undefined1 *puVar19;
  undefined4 uVar20;
  undefined1 auStack_5a8 [32];
  undefined4 local_588;
  undefined4 local_580;
  longlong *local_578;
  undefined1 *local_570;
  undefined4 local_568;
  undefined4 local_560;
  undefined1 local_558;
  undefined1 local_557;
  undefined4 *local_550;
  longlong *local_548;
  longlong *plStack_540;
  longlong *local_538;
  undefined8 local_530;
  undefined4 local_528 [2];
  longlong local_520;
  undefined4 local_518;
  undefined1 local_514;
  undefined4 *local_510;
  undefined8 local_508;
  longlong local_500;
  undefined1 *local_4f8;
  undefined1 *puStack_4f0;
  longlong local_4e8;
  longlong *local_4d8;
  undefined1 local_4d0 [8];
  undefined8 local_4c8;
  undefined1 local_4c0 [8];
  longlong local_4b8;
  undefined1 local_4b0 [8];
  undefined1 *local_4a8;
  longlong *local_4a0;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_5a8;
  iVar6 = FUN_1406e8c20(param_4);
  FUN_1406e9050(param_4,&local_500);
  local_548 = (longlong *)0x0;
  plStack_540 = (longlong *)0x0;
  local_538 = (longlong *)0x0;
  bVar5 = FUN_1406e8ae0(param_4);
  if (bVar5 != 0) {
    uVar18 = (ulonglong)bVar5;
    do {
      local_550 = (undefined4 *)0x0;
      FUN_1401a75b0(&local_558,param_4);
      if (plStack_540 == local_538) {
        FUN_1412802a0(&local_548,plStack_540,&local_558);
      }
      else {
        *(undefined1 *)plStack_540 = local_558;
        *(undefined1 *)((longlong)plStack_540 + 1) = local_557;
        ppuVar14 = (undefined4 **)(plStack_540 + 1);
        *ppuVar14 = (undefined4 *)0x0;
        if ((ppuVar14 != &local_550) && (local_550 != (undefined4 *)0x0)) {
          uVar2 = local_550[-2];
          if (uVar2 != 0) {
            puVar9 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar2 * 4 + 8);
            if (puVar9 != (undefined4 *)0x0) {
              puVar9 = puVar9 + 2;
            }
            *ppuVar14 = puVar9;
            *(ulonglong *)(puVar9 + -2) = (ulonglong)uVar2;
            puVar9 = *ppuVar14;
            for (puVar10 = local_550; puVar10 < local_550 + uVar2; puVar10 = puVar10 + 1) {
              *puVar9 = *puVar10;
              puVar9 = puVar9 + 1;
            }
          }
        }
        plStack_540 = plStack_540 + 2;
      }
      if (local_550 != (undefined4 *)0x0) {
        thunk_FUN_140205820(local_550 + -2,0);
        local_550 = (undefined4 *)0x0;
      }
      uVar18 = uVar18 - 1;
    } while (uVar18 != 0);
  }
  if (((DAT_143aa84a0 == 0) || (lVar11 = FUN_142cbe730(), lVar11 == 0)) || (DAT_143aa8518 == 0))
  goto LAB_1412801cb;
  iVar8 = 2;
  local_4f8 = (undefined1 *)0x0;
  puStack_4f0 = (undefined1 *)0x0;
  local_4e8 = 0;
  FUN_1427be040(DAT_143aa8518,local_4c0);
  if (((local_4b8 == 0) ||
      (plVar12 = (longlong *)FUN_140192f00(*(undefined8 *)(lVar11 + 0x360)),
      plVar12 == (longlong *)0x0)) ||
     ((iVar7 = FUN_14019a5d0(plVar12 + 4), 9999 < iVar7 - 0x195460U ||
      ((longlong)plStack_540 - (longlong)local_548 >> 4 != 1)))) {
LAB_1412800df:
    FUN_1406ed520(local_498,0xf3);
    FUN_1406ed9d0(local_498,0);
    FUN_1406ed840(local_498,0x35);
    puVar19 = local_4f8;
    if (iVar8 == 1) {
      FUN_1406ed840(local_498,1);
      puVar4 = puStack_4f0;
      FUN_1406ed840(local_498,(longlong)puStack_4f0 - (longlong)puVar19 >> 2);
      for (puVar16 = puVar19; puVar16 != puVar4; puVar16 = puVar16 + 4) {
        FUN_1406ed840(local_498,*puVar16);
      }
    }
    else {
      FUN_1406ed840(local_498,0);
    }
    FUN_1415d01c0(local_498);
    FUN_140da25d0(param_1 + 0x10);
    FUN_1406ed610(local_498);
  }
  else {
    local_518 = 0;
    local_510 = (undefined4 *)0x0;
    local_514 = 100;
    puVar13 = (undefined8 *)(**(code **)(*plVar12 + 0x80))(plVar12,local_4b0);
    local_508 = *puVar13;
    ppuVar14 = (undefined4 **)(local_548 + 1);
    if (&local_510 != ppuVar14) {
      if (local_510 != (undefined4 *)0x0) {
        thunk_FUN_140205820(local_510 + -2,0);
        local_510 = (undefined4 *)0x0;
      }
      if (*ppuVar14 != (undefined4 *)0x0) {
        uVar2 = (*ppuVar14)[-2];
        if (uVar2 != 0) {
          local_510 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar2 * 4 + 8);
          if (local_510 != (undefined4 *)0x0) {
            local_510 = local_510 + 2;
          }
          *(ulonglong *)(local_510 + -2) = (ulonglong)uVar2;
          puVar10 = *ppuVar14;
          puVar9 = puVar10 + uVar2;
          puVar3 = local_510;
          for (; puVar10 < puVar9; puVar10 = puVar10 + 1) {
            *puVar3 = *puVar10;
            puVar3 = puVar3 + 1;
          }
        }
      }
    }
    lVar11 = 0;
    local_520 = FUN_14019b780(&DAT_143ad68a0,2000);
    if (local_520 != 0) {
      lVar11 = FUN_142a57d30(local_520,0,0,0);
    }
    if ((*(longlong *)(param_1 + 0x18) - 1U < 999) || (*(longlong *)(param_1 + 0x18) == -1)) {
      FUN_142e52ed0(0x447);
    }
    lVar15 = lVar11 + 0x18;
    if (lVar11 == 0) {
      lVar15 = 0;
    }
    if (lVar15 == 0) {
      lVar11 = 0;
    }
    else {
      lVar11 = lVar15 + -0x18;
      if (lVar11 != 0) {
        if (0xfffff < *(ulonglong *)(lVar15 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar15 + 8) = *(longlong *)(lVar15 + 8) + 1;
        UNLOCK();
      }
    }
    lVar15 = *(longlong *)(param_1 + 0x18);
    *(longlong *)(param_1 + 0x18) = lVar11;
    if (lVar15 != 0) {
      if (0xffffe < *(longlong *)(lVar15 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar12 = (longlong *)(lVar15 + 0x20);
      lVar11 = *plVar12;
      *plVar12 = *plVar12 + -1;
      UNLOCK();
      if (((int)lVar11 == 1) &&
         (puVar13 = (undefined8 *)(lVar15 + 0x18), puVar13 != (undefined8 *)0x0)) {
        (**(code **)*puVar13)(puVar13,1);
      }
    }
    plVar12 = *(longlong **)(param_1 + 0x18);
    local_4d8 = plVar12;
    if (plVar12 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar12[4]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar12[4] = plVar12[4] + 1;
      UNLOCK();
    }
    plVar17 = local_4d8;
    if (plVar12 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_4a8 = local_4d0;
    local_4c8 = 0;
    local_4a0 = &local_520;
    local_520 = 0;
    local_530 = 0;
    FUN_14019a260(&local_530,&local_500);
    local_560 = 0;
    local_568 = 0;
    local_570 = local_4d0;
    local_578 = &local_520;
    local_580 = 0;
    local_588 = 0;
    FUN_142a61900(plVar17,0x14,param_3,&local_530);
    if (plVar17 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a8a190(plVar17,&local_518,0,iVar6 != 0);
    if (plVar17 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_142a5ee30(plVar17);
    if (plVar17 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar8 = (**(code **)(*plVar17 + 0x130))(plVar17);
    if (iVar8 != 3) {
      local_528[0] = FUN_142a8a7c0(plVar17);
      FUN_1401d01c0(&local_4f8,0,local_528);
      if (0xffffe < plVar17[4] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar17 = plVar17 + 4;
      lVar11 = *plVar17;
      *plVar17 = *plVar17 + -1;
      UNLOCK();
      if (((int)lVar11 == 1) && (plVar12 = local_4d8 + 3, plVar12 != (longlong *)0x0)) {
        (**(code **)*plVar12)(plVar12,1);
      }
      if (local_510 != (undefined4 *)0x0) {
        thunk_FUN_140205820(local_510 + -2,0);
      }
      goto LAB_1412800df;
    }
    if (0xffffe < plVar17[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar17 = plVar17 + 4;
    lVar11 = *plVar17;
    *plVar17 = *plVar17 + -1;
    UNLOCK();
    if (((int)lVar11 == 1) && (plVar12 = local_4d8 + 3, plVar12 != (longlong *)0x0)) {
      (**(code **)*plVar12)(plVar12,1);
    }
    puVar19 = local_4f8;
    if (local_510 != (undefined4 *)0x0) {
      thunk_FUN_140205820(local_510 + -2,0);
      puVar19 = local_4f8;
    }
  }
  uVar20 = FUN_140ce88a0(local_4c0);
  if (puVar19 != (undefined1 *)0x0) {
    uVar18 = (local_4e8 - (longlong)puVar19 >> 2) * 4;
    puVar16 = puVar19;
    if (0xfff < uVar18) {
      puVar16 = *(undefined1 **)(puVar19 + -8);
      if ((undefined1 *)0x1f < puVar19 + (-8 - (longlong)puVar16)) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(uVar20,uVar18 + 0x27);
      }
    }
    thunk_FUN_140205820(puVar16);
  }
LAB_1412801cb:
  plVar12 = plStack_540;
  if (local_548 != (longlong *)0x0) {
    if (local_548 != plStack_540) {
      plVar17 = local_548 + 1;
      do {
        if (*plVar17 != 0) {
          thunk_FUN_140205820(*plVar17 + -8,0);
          *plVar17 = 0;
        }
        plVar1 = plVar17 + 1;
        plVar17 = plVar17 + 2;
      } while (plVar1 != plVar12);
    }
    uVar18 = (longlong)local_538 - (longlong)local_548 & 0xfffffffffffffff0;
    if (0xfff < uVar18) {
      if (0x1f < (ulonglong)((longlong)local_548 + (-8 - local_548[-1]))) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(local_548[-1],uVar18 + 0x27);
      }
    }
    thunk_FUN_140205820();
    local_548 = (longlong *)0x0;
    plStack_540 = (longlong *)0x0;
    local_538 = (longlong *)0x0;
  }
  if (local_500 != 0) {
    FUN_14019f2c0(local_500 + -0x10);
  }
  return;
}



//===========================================================
// FUN_1412818f0 @ 1412818f0   (856 bytes)
//===========================================================

void FUN_1412818f0(longlong param_1,undefined8 param_2,uint param_3)

{
  longlong *plVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined8 *puVar7;
  longlong lVar8;
  longlong lVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  int local_res20 [2];
  undefined4 local_58;
  undefined4 local_54;
  undefined8 local_50;
  undefined8 local_48;
  longlong local_40;
  longlong local_38;
  undefined8 *local_30;
  
  FUN_1406e9050(param_2,&local_38);
  uVar2 = FUN_1406e8c20(param_2);
  uVar3 = FUN_1406e8c20(param_2);
  uVar4 = FUN_1406e8c20(param_2);
  uVar5 = FUN_1406e8c20(param_2);
  uVar6 = FUN_1406e8c20(param_2);
  local_54 = FUN_1406e8c20(param_2);
  FUN_1406e9050(param_2,&local_40);
  lVar8 = DAT_143abfdf0;
  if (DAT_143abfdf0 != 0) {
    local_30 = &local_50;
    local_50 = 0;
    FUN_14019a260(&local_50,&local_40);
    local_48 = 0;
    FUN_14019a260(&local_48,&local_38);
    FUN_140e42c80(lVar8,&local_48,uVar2,uVar3,uVar4,uVar5,uVar6,local_54,&local_50);
    puVar11 = (undefined8 *)0x0;
    local_res20[0] = 0;
    local_58 = 0xffffffff;
    if (*(longlong *)(DAT_143aa8518 + 0x56c0) == 0) {
      local_30 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x250,0xffffffff);
      puVar7 = puVar11;
      if (local_30 != (undefined8 *)0x0) {
        puVar7 = (undefined8 *)FUN_1422c9150(local_30,local_res20,&local_58,param_3 & 1);
      }
      lVar8 = DAT_143aa8518;
      if ((*(longlong *)(DAT_143aa8518 + 0x56c0) - 1U < 999) ||
         (*(longlong *)(DAT_143aa8518 + 0x56c0) == -1)) {
        FUN_142e52ed0(0x447);
      }
      puVar10 = puVar7 + 3;
      if (puVar7 == (undefined8 *)0x0) {
        puVar10 = puVar11;
      }
      puVar7 = puVar11;
      if ((puVar10 != (undefined8 *)0x0) && (puVar7 = puVar10 + -3, puVar7 != (undefined8 *)0x0)) {
        if (0xfffff < (ulonglong)puVar10[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar10[1] = puVar10[1] + 1;
        UNLOCK();
      }
      lVar9 = *(longlong *)(lVar8 + 0x56c0);
      *(undefined8 **)(lVar8 + 0x56c0) = puVar7;
      if (lVar9 != 0) {
        if (0xffffe < *(longlong *)(lVar9 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = (longlong *)(lVar9 + 0x20);
        lVar8 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if (((int)lVar8 == 1) &&
           (puVar7 = (undefined8 *)(lVar9 + 0x18), puVar7 != (undefined8 *)0x0)) {
          (**(code **)*puVar7)(puVar7,1);
        }
      }
      lVar9 = DAT_143aa8518;
      lVar8 = *(longlong *)(DAT_143aa8518 + 0x56c0);
      if (lVar8 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar8 = *(longlong *)(lVar9 + 0x56c0);
      }
      FUN_1422c95d0(lVar8);
    }
    FUN_1422c9620(param_3 & 1,local_res20[0] == 0,local_58);
    lVar8 = DAT_143aa8518;
    if (*(longlong *)(DAT_143aa8518 + 0x56c0) != 0) {
      lVar9 = *(longlong *)(DAT_143aa8518 + 0x56c0);
      if (lVar9 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar9 = *(longlong *)(lVar8 + 0x56c0);
      }
      FUN_142bf3f70(lVar9);
      FUN_141282c20(DAT_143aa8518 + 0x56b8);
    }
    FUN_140e47060(DAT_143abfdf0);
    lVar8 = *(longlong *)(param_1 + 0x18);
    if (lVar8 != 0) {
      if (0xffffe < *(longlong *)(lVar8 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = (longlong *)(lVar8 + 0x20);
      lVar8 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar8 == 1) {
        puVar7 = (undefined8 *)(*(longlong *)(param_1 + 0x18) + 0x18);
        if (*(longlong *)(param_1 + 0x18) == 0) {
          puVar7 = puVar11;
        }
        if (puVar7 != (undefined8 *)0x0) {
          (**(code **)*puVar7)(puVar7,1);
        }
      }
      *(undefined8 *)(param_1 + 0x18) = 0;
    }
  }
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
  return;
}



//===========================================================
// FUN_142a58880 @ 142a58880   (1157 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142a58880(longlong param_1)

{
  undefined8 *puVar1;
  undefined2 *puVar2;
  undefined4 uVar3;
  code *pcVar4;
  char *pcVar5;
  undefined4 uVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong lVar10;
  undefined1 *puVar11;
  int iVar12;
  longlong *plVar13;
  undefined8 uStack_90;
  undefined1 auStack_88 [32];
  undefined8 local_68;
  int local_60 [4];
  undefined4 local_50;
  undefined8 local_48;
  longlong local_38;
  ulonglong local_30;
  
  local_30 = DAT_143a8b908 ^ (ulonglong)&local_38;
  if (*(int *)(param_1 + 0x2a0) - 1U < 2) {
    uStack_90 = 0x142a588c2;
    FUN_142a7bd10();
  }
  else {
    uStack_90 = 0x142a588c9;
    FUN_142a77590();
  }
  switch(*(undefined4 *)(param_1 + 0x2a8)) {
  case 0:
    uStack_90 = 0x142a588f4;
    FUN_142a65740(param_1);
    break;
  case 1:
    uStack_90 = 0x142a58901;
    FUN_142a66070(param_1);
    break;
  case 2:
    uStack_90 = 0x142a5890e;
    FUN_142a66860(param_1);
    break;
  case 3:
  case 4:
  case 5:
    if (*(int *)(param_1 + 0x2d0) == 0) {
      uStack_90 = 0x142a5892f;
      FUN_142a66ae0(param_1);
    }
    else {
      uStack_90 = 0x142a58925;
      FUN_142a678c0();
    }
    break;
  case 6:
  case 7:
    uStack_90 = 0x142a58963;
    FUN_142a6ba20(param_1);
    break;
  case 8:
    uStack_90 = 0x142a5899f;
    FUN_142a6c3c0(param_1);
    break;
  case 9:
    uStack_90 = 0x142a589bd;
    FUN_142a6d2c0(param_1);
    break;
  case 10:
    uStack_90 = 0x142a589c7;
    FUN_142a6d680(param_1);
    break;
  case 0xb:
    uStack_90 = 0x142a589a9;
    FUN_142a6b1a0(param_1);
    break;
  case 0xc:
    uStack_90 = 0x142a58949;
    FUN_142a681e0(param_1);
    break;
  case 0xd:
    uStack_90 = 0x142a589b3;
    FUN_142a6cc10(param_1);
    break;
  case 0xe:
    uStack_90 = 0x142a58956;
    FUN_142a68b50(param_1);
    break;
  case 0xf:
    uStack_90 = 0x142a589d1;
    FUN_142a6dc50(param_1);
    break;
  case 0x10:
    uStack_90 = 0x142a589f9;
    FUN_142a6c0e0(param_1);
    break;
  case 0x11:
    uStack_90 = 0x142a5893c;
    FUN_142a67400(param_1);
    break;
  case 0x12:
    uStack_90 = 0x142a58995;
    FUN_142a8d2d0(param_1);
    break;
  case 0x13:
    uStack_90 = 0x142a5898b;
    FUN_142a8c4e0(param_1);
    break;
  case 0x14:
    uStack_90 = 0x142a58972;
    FUN_142a8b7a0(param_1,0);
    break;
  case 0x15:
    uStack_90 = 0x142a58981;
    FUN_142a8b7a0(param_1,0x140000001);
    break;
  case 0x17:
  case 0x19:
  case 0x1c:
  case 0x1d:
    uStack_90 = 0x142a589e5;
    FUN_142a8d5e0(param_1);
    break;
  case 0x18:
  case 0x1a:
    uStack_90 = 0x142a589db;
    FUN_142a8e830(param_1);
    break;
  case 0x1b:
    uStack_90 = 0x142a589ef;
    FUN_142a8de60(param_1);
  }
  lVar10 = 0;
  if (*(int *)(param_1 + 0x2c0) != 0) {
    iVar12 = 0x12;
    if (1 < *(int *)(param_1 + 0x2a0) - 1U) {
      iVar12 = 8;
    }
    iVar12 = (*(int *)(param_1 + 0x2b4) - *(int *)(param_1 + 0x2b8)) / iVar12;
    uStack_90 = 0x142a58a51;
    local_38 = FUN_14019b780(&DAT_143ad68a0,0x108);
    lVar7 = lVar10;
    if (local_38 != 0) {
      uStack_90 = 0x142a58a62;
      lVar7 = FUN_1416ed1a0(local_38);
    }
    if ((*(longlong *)(param_1 + 0x5d8) - 1U < 999) || (*(longlong *)(param_1 + 0x5d8) == -1)) {
      uStack_90 = 0x142a58a8d;
      FUN_142e52ed0(0x447);
    }
    lVar9 = lVar7 + 0x18;
    if (lVar7 == 0) {
      lVar9 = lVar10;
    }
    if ((lVar9 != 0) && (lVar10 = lVar9 + -0x18, lVar10 != 0)) {
      if (0xfffff < *(ulonglong *)(lVar9 + 8)) {
        uStack_90 = 0x142a58aba;
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar9 + 8) = *(longlong *)(lVar9 + 8) + 1;
      UNLOCK();
    }
    lVar7 = *(longlong *)(param_1 + 0x5d8);
    *(longlong *)(param_1 + 0x5d8) = lVar10;
    if (lVar7 != 0) {
      if (0xffffe < *(longlong *)(lVar7 + 0x20) - 1U) {
        uStack_90 = 0x142a58af1;
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar13 = (longlong *)(lVar7 + 0x20);
      lVar10 = *plVar13;
      *plVar13 = *plVar13 + -1;
      UNLOCK();
      if (((int)lVar10 == 1) && (puVar1 = (undefined8 *)(lVar7 + 0x18), puVar1 != (undefined8 *)0x0)
         ) {
        uStack_90 = 0x142a58b18;
        (**(code **)*puVar1)(puVar1,1);
      }
    }
    if (*(int *)(param_1 + 0x2a0) - 1U < 2) {
      uStack_90 = 0x142a58b2f;
      FUN_142a7c1d0(param_1);
    }
    else {
      plVar13 = *(longlong **)(param_1 + 0x5d8);
      if (plVar13 == (longlong *)0x0) {
        uStack_90 = 0x142a58b49;
        FUN_142e52ed0(0x431,0);
        plVar13 = *(longlong **)(param_1 + 0x5d8);
      }
      pcVar4 = *(code **)(*plVar13 + 0x80);
      uVar6 = *(undefined4 *)(param_1 + 0x2b8);
      uVar3 = *(undefined4 *)(param_1 + 0x2b0);
      uStack_90 = 0x142a58b70;
      local_60[0] = FUN_142a6f9c0(param_1);
      local_60[0] = *(int *)(param_1 + 0x2ac) + 1 + local_60[0];
      local_48 = 0;
      local_68 = CONCAT44(local_68._4_4_,4);
      uStack_90 = 0x142a58bab;
      local_60[2] = uVar3;
      local_50 = uVar6;
      (*pcVar4)(plVar13,param_1,0x3e9,1);
    }
    lVar10 = *(longlong *)(param_1 + 0x5d8);
    if (lVar10 == 0) {
      uStack_90 = 0x142a58bc3;
      FUN_142e52ed0(0x431,0);
      lVar10 = *(longlong *)(param_1 + 0x5d8);
    }
    uStack_90 = 0x142a58bde;
    FUN_1416ee330(lVar10,(iVar12 < 1) + 1 + iVar12);
    lVar10 = *(longlong *)(param_1 + 0x5d8);
    if (lVar10 == 0) {
      uStack_90 = 0x142a58bf6;
      FUN_142e52ed0(0x431,0);
      lVar10 = *(longlong *)(param_1 + 0x5d8);
    }
    uStack_90 = 0x142a58c05;
    uVar6 = FUN_142a6f9c0(param_1);
    *(undefined4 *)(lVar10 + 0x78) = uVar6;
  }
  puVar11 = auStack_88;
  if (*(int *)(param_1 + 0x2d0) == 0) {
    if (*(int *)(param_1 + 0x2a0) - 1U < 2) {
      *(undefined4 *)(param_1 + 0x6f0) = 0x13ddf7;
      uStack_90 = 0x142a58c37;
      FUN_142a7abc0();
    }
    else {
      uStack_90 = 0x142a58c3e;
      FUN_142a72d20(param_1);
    }
    pcVar5 = *(char **)(param_1 + 0x6c0);
    puVar11 = auStack_88;
    if ((pcVar5 != (char *)0x0) && (puVar11 = auStack_88, *pcVar5 != '\0')) {
      local_60[0] = 0;
      local_68 = 0;
      uStack_90 = 0x142a58c73;
      iVar12 = (*DAT_1432627f8)(0xfde9,0,pcVar5,0xffffffff);
      lVar10 = *(longlong *)(param_1 + 0x6c0);
      uVar8 = (longlong)(iVar12 * 2) + 0xf;
      if (uVar8 <= (ulonglong)(longlong)(iVar12 * 2)) {
        uVar8 = 0xffffffffffffff0;
      }
      uStack_90 = 0x142a58c9b;
      lVar7 = -(uVar8 & 0xfffffffffffffff0);
      puVar11 = auStack_88 + lVar7;
      puVar2 = (undefined2 *)((longlong)&local_38 + lVar7);
      if (lVar10 == 0) {
        if (puVar2 != (undefined2 *)0x0) {
          *puVar2 = 0;
        }
      }
      else {
        *(undefined4 *)((longlong)local_60 + lVar7) = 0x100000;
        *(undefined2 **)((longlong)local_60 + lVar7 + -8) = puVar2;
        *(undefined8 *)(auStack_88 + lVar7 + -8) = 0x142a58cd2;
        (*DAT_1432627f8)(0xfde9,0,lVar10,0xffffffff);
      }
      *(undefined8 *)(auStack_88 + lVar7 + -8) = 0x142a58cdf;
      FUN_1429f6d20(puVar2,100);
    }
  }
  *(undefined8 *)(puVar11 + -8) = 0x142a58ceb;
  return;
}


