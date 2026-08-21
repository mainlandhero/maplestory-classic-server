
//===========================================================
// FUN_1402e3cd0 @ 1402e3cd0   (336 bytes)
//===========================================================

longlong FUN_1402e3cd0(undefined8 param_1,longlong param_2,undefined4 param_3,undefined4 param_4)

{
  longlong *plVar1;
  uint uVar2;
  longlong lVar3;
  undefined8 *local_18;
  
  lVar3 = FUN_1402e3770(param_1,param_3,param_4,param_4,0);
  if (lVar3 == 0) {
    local_18 = (undefined8 *)0x0;
    uVar2 = 2;
  }
  else {
    local_18 = *(undefined8 **)(lVar3 + 8);
    if (local_18 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)local_18[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      local_18[1] = local_18[1] + 1;
      UNLOCK();
    }
    uVar2 = 1;
  }
  *(undefined8 **)(param_2 + 8) = local_18;
  if (local_18 != (undefined8 *)0x0) {
    if (0xfffff < (ulonglong)local_18[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    local_18[1] = local_18[1] + 1;
    UNLOCK();
  }
  if (((uVar2 & 2) != 0) && (uVar2 = uVar2 & 0xfffffffd, local_18 != (undefined8 *)0x0)) {
    if (0xffffe < local_18[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = local_18 + 1;
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      (**(code **)*local_18)(local_18,1);
    }
  }
  if (((uVar2 & 1) != 0) && (local_18 != (undefined8 *)0x0)) {
    if (0xffffe < local_18[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = local_18 + 1;
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      (**(code **)*local_18)(local_18,1);
    }
  }
  return param_2;
}



//===========================================================
// FUN_1402e4c20 @ 1402e4c20   (480 bytes)
//===========================================================

undefined8 FUN_1402e4c20(longlong param_1,int param_2,int param_3,undefined8 param_4)

{
  longlong lVar1;
  int iVar2;
  undefined8 uVar3;
  longlong *plVar4;
  int iVar5;
  undefined1 local_18 [16];
  
  if (param_2 - 1U < 6) {
    if (param_2 == 1) {
      if (param_3 < 0) {
        iVar5 = -param_3;
        iVar2 = FUN_140302620(iVar5);
        if (iVar2 != 0) {
          uVar3 = FUN_140232590(local_18,param_4);
          FUN_1402de550(param_1 + 0x5a8,uVar3,iVar5);
          FUN_1401abd80(param_4);
          return 1;
        }
        if (0x1e < param_3 + 0x1fU) {
          FUN_1401abd80(param_4);
          return 0;
        }
        FUN_1401e8780(param_1 + 0x1a8 + (longlong)iVar5 * 0x10,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
    }
    else if ((param_2 == 6) && (param_3 < 0)) {
      if ((0xd < -param_3 - 0x4b0U) && (0x32 < -param_3 - 0x708U)) {
        if (0x1e < param_3 + 0x83U) {
          FUN_1401abd80(param_4);
          return 0;
        }
        FUN_1401e8780((longlong)(-100 - param_3) * 0x10 + 0x3a8 + param_1,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
      uVar3 = FUN_140232590(local_18,param_4);
      FUN_1402de550(param_1 + 0x5a8,uVar3,-param_3);
      FUN_1401abd80(param_4);
      return 1;
    }
    if (0 < param_3) {
      plVar4 = (longlong *)((longlong)param_2 * 8 + 0x5d0 + param_1);
      lVar1 = *plVar4;
      if ((lVar1 != 0) && (param_3 <= *(int *)(lVar1 + -8) + -1)) {
        uVar3 = FUN_1402f1620(plVar4,param_3);
        FUN_1401e8780(uVar3,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
    }
    FUN_1401abd80(param_4);
  }
  else {
    FUN_1401abd80(param_4);
  }
  return 0;
}



//===========================================================
// FUN_1402e5020 @ 1402e5020   (105 bytes)
//===========================================================

void FUN_1402e5020(longlong param_1,int param_2,uint param_3,undefined8 param_4)

{
  if (param_2 == 2) {
    if (2 < param_3) goto LAB_1402e507a;
    param_1 = param_1 + 0x608;
  }
  else if (param_2 == 3) {
    if (9 < param_3) goto LAB_1402e507a;
    param_1 = param_1 + 0x638;
  }
  else {
    if ((param_2 != 4) || (6 < param_3)) goto LAB_1402e507a;
    param_1 = param_1 + 0x6d8;
  }
  FUN_1402fa360(param_1 + (longlong)(int)param_3 * 0x10,param_4);
LAB_1402e507a:
  FUN_140301bb0(param_4);
  return;
}



//===========================================================
// FUN_14030ee00 @ 14030ee00   (388 bytes)
//===========================================================

ulonglong FUN_14030ee00(ulonglong *param_1,uint param_2,ulonglong param_3)

{
  longlong lVar1;
  ulonglong uVar2;
  uint uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  
  uVar7 = (ulonglong)param_2;
  uVar2 = *param_1;
  uVar5 = 0;
  if (uVar2 != 0) {
    uVar5 = (ulonglong)*(uint *)(uVar2 - 8);
  }
  if ((uint)uVar5 < param_2) {
    if (uVar2 == 0) {
      uVar3 = 0;
    }
    else {
      uVar6 = *(ulonglong *)(uVar2 - 0x10);
      uVar4 = ~uVar6;
      if (-1 < (longlong)uVar6) {
        uVar4 = uVar6;
      }
      uVar3 = (uint)(uVar4 - 8 >> 4);
    }
    if (uVar3 < param_2) {
      lVar1 = FUN_14019b780(&DAT_143ad68a0,uVar7 * 0x10 + 8);
      uVar2 = lVar1 + 8;
      if (lVar1 == 0) {
        uVar2 = 0;
      }
      if ((param_3 & 2) == 0) {
        for (uVar6 = uVar2; uVar6 < uVar7 * 0x10 + uVar2; uVar6 = uVar6 + 0x10) {
          *(undefined8 *)(uVar6 + 8) = 0;
        }
      }
      uVar6 = *param_1;
      if (uVar6 != 0) {
        if ((param_3 & 1) == 0) {
          FUN_142ef7ba0(uVar2,uVar6,uVar5 << 4);
          uVar6 = *param_1;
        }
        thunk_FUN_140205820(uVar6 - 8,0);
      }
      *param_1 = uVar2;
    }
    else if ((param_3 & 2) == 0) {
      for (uVar5 = uVar5 * 0x10 + uVar2; uVar5 < uVar7 * 0x10 + uVar2; uVar5 = uVar5 + 0x10) {
        *(undefined8 *)(uVar5 + 8) = 0;
      }
    }
  }
  else {
    for (uVar6 = uVar7 * 0x10 + uVar2; uVar6 < uVar5 * 0x10 + uVar2; uVar6 = uVar6 + 0x10) {
      FUN_1401abd80(uVar6);
    }
  }
  if (*param_1 == 0) {
    uVar2 = 0;
  }
  else {
    *(ulonglong *)(*param_1 - 8) = uVar7;
    uVar2 = *param_1;
  }
  return uVar2;
}



//===========================================================
// FUN_142cbe730 @ 142cbe730   (8 bytes)
//===========================================================

undefined8 FUN_142cbe730(longlong param_1)

{
  return *(undefined8 *)(param_1 + 0x2358);
}


