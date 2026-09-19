
//===========================================================
// FUN_14127dc60 @ 14127dc60   (1060 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14127dc60(longlong param_1,int param_2,undefined4 param_3,undefined8 param_4)

{
  longlong *plVar1;
  undefined4 uVar2;
  byte bVar3;
  undefined1 uVar4;
  undefined4 uVar5;
  int iVar6;
  longlong lVar7;
  undefined8 uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *plVar11;
  uint uVar12;
  ulonglong uVar13;
  uint uVar14;
  undefined1 auStack_568 [32];
  undefined4 local_548;
  undefined4 local_540;
  longlong *local_538;
  undefined1 *local_530;
  undefined4 local_528;
  undefined4 local_520;
  undefined1 local_518 [8];
  undefined8 local_510;
  undefined4 local_508;
  undefined1 local_504 [4];
  longlong local_500;
  undefined4 local_4f0;
  int local_4ec;
  longlong local_4e8;
  longlong local_4e0;
  undefined1 local_4d8 [8];
  undefined8 local_4d0;
  longlong *local_4c0;
  undefined1 *local_4b8;
  longlong *local_4b0;
  undefined1 local_4a8 [1104];
  ulonglong local_58;
  
  local_58 = DAT_143a8b908 ^ (ulonglong)auStack_568;
  local_4f0 = param_3;
  local_4ec = param_2;
  uVar5 = FUN_1406e8c20(param_4);
  local_510 = CONCAT44(local_510._4_4_,uVar5);
  FUN_1406e9050(param_4,&local_4e0);
  local_508 = 0;
  local_504[0] = 0;
  local_500 = 0;
  FUN_1406e9170(param_4,local_518,1);
  local_504[0] = local_518[0];
  bVar3 = FUN_1406e8ae0(param_4);
  uVar14 = (uint)bVar3;
  if (bVar3 != 0) {
    do {
      uVar5 = FUN_1406e8c20();
      if (local_500 == 0) {
        uVar12 = 0;
        uVar13 = 1;
LAB_14127dd4f:
        if (local_500 == 0) {
          iVar6 = 0;
        }
        else {
          uVar9 = *(ulonglong *)(local_500 + -0x10);
          uVar10 = ~uVar9;
          if (-1 < (longlong)uVar9) {
            uVar10 = uVar9;
          }
          iVar6 = (int)(uVar10 - 8 >> 2);
        }
        if (iVar6 != (int)uVar13) {
          if (local_500 == 0) {
            uVar9 = 0;
          }
          else {
            uVar9 = (ulonglong)*(uint *)(local_500 + -8);
          }
          lVar7 = FUN_14019b780(&DAT_143ad68a0,uVar13 * 4 + 8);
          if (lVar7 == 0) {
            lVar7 = 0;
          }
          else {
            lVar7 = lVar7 + 8;
          }
          if (local_500 != 0) {
            FUN_142ef7ba0(lVar7,local_500,uVar9 << 2);
            thunk_FUN_140205820(local_500 + -8,0);
          }
          *(ulonglong *)(lVar7 + -8) = uVar9;
          local_500 = lVar7;
        }
      }
      else {
        uVar12 = *(uint *)(local_500 + -8);
        uVar13 = *(ulonglong *)(local_500 + -0x10);
        uVar9 = ~uVar13;
        if (-1 < (longlong)uVar13) {
          uVar9 = uVar13;
        }
        if ((uint)(uVar9 - 8 >> 2) <= uVar12) {
          if (uVar12 == 0) {
            uVar13 = 1;
          }
          else {
            uVar13 = (ulonglong)(uVar12 * 2);
          }
          goto LAB_14127dd4f;
        }
      }
      *(longlong *)(local_500 + -8) = *(longlong *)(local_500 + -8) + 1;
      *(undefined4 *)(local_500 + (longlong)(int)uVar12 * 4) = uVar5;
      uVar14 = uVar14 - 1;
    } while (0 < (int)uVar14);
    uVar5 = (undefined4)local_510;
  }
  FUN_1406e8c20(param_4);
  local_4e8 = FUN_14019b780(&DAT_143ad68a0,2000);
  uVar8 = 0;
  if (local_4e8 != 0) {
    uVar8 = FUN_142a57d30(local_4e8,0,0,0);
  }
  lVar7 = FUN_141280700(param_1 + 0x10,uVar8);
  plVar11 = *(longlong **)(lVar7 + 8);
  local_4c0 = plVar11;
  if (plVar11 != (longlong *)0x0) {
    if (0xfffff < (ulonglong)plVar11[4]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    plVar11[4] = plVar11[4] + 1;
    UNLOCK();
  }
  plVar1 = local_4c0;
  if (plVar11 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  local_4b8 = local_4d8;
  local_4d0 = 0;
  local_4b0 = &local_4e8;
  local_4e8 = 0;
  local_510 = 0;
  FUN_14019a260(&local_510,&local_4e0);
  uVar2 = local_4f0;
  local_520 = 0;
  local_528 = 0;
  local_530 = local_4d8;
  local_538 = &local_4e8;
  local_540 = 0;
  local_548 = 0;
  FUN_142a61900(plVar1,0x14,local_4f0,&local_510);
  if (plVar1 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_142a8a190(plVar1,&local_508,uVar5,0);
  if (plVar1 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_142a5ee30(plVar1);
  if (plVar1 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  iVar6 = (**(code **)(*plVar1 + 0x130))(plVar1);
  if (iVar6 != 3) {
    FUN_1406ed520(local_4a8,0xf3);
    FUN_1406ed9d0(local_4a8,0);
    FUN_1406ed840(local_4a8,10);
    if (iVar6 == 1) {
      FUN_1406ed840(local_4a8,1);
      FUN_1406ede20(local_4a8,local_504,1);
      FUN_1406ed840(local_4a8,0);
      uVar5 = 0;
      if (local_4ec == 4) {
        uVar5 = uVar2;
      }
      FUN_1406ed9d0(local_4a8,uVar5);
      uVar4 = FUN_142a8a7c0(plVar1);
    }
    else {
      uVar4 = 0;
    }
    FUN_1406ed840(local_4a8,uVar4);
    FUN_1415d01c0(local_4a8);
    FUN_140da25d0(param_1 + 0x10);
    FUN_1406ed610(local_4a8);
  }
  if (0xffffe < plVar1[4] - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar1 = plVar1 + 4;
  lVar7 = *plVar1;
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if (((int)lVar7 == 1) && (plVar11 = local_4c0 + 3, plVar11 != (longlong *)0x0)) {
    (**(code **)*plVar11)(plVar11,1);
  }
  if (local_500 != 0) {
    thunk_FUN_140205820(local_500 + -8,0);
    local_500 = 0;
  }
  if (local_4e0 != 0) {
    FUN_14019f2c0(local_4e0 + -0x10);
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


