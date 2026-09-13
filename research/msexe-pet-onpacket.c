
//===========================================================
// FUN_142795b20 @ 142795b20   (307 bytes)
//===========================================================

void FUN_142795b20(longlong *param_1,int param_2,undefined8 param_3)

{
  undefined4 uVar1;
  longlong lVar2;
  
  if (param_2 != 0x277) {
    uVar1 = FUN_1406e8c20(param_3);
    lVar2 = FUN_1427703d0(param_1,uVar1);
    if (lVar2 != 0) {
      switch(param_2) {
      case 0x278:
        FUN_141ec3f20(lVar2,param_3);
        return;
      case 0x279:
        FUN_141ec3fa0(lVar2,param_3);
        return;
      case 0x27a:
        FUN_141ec4050(lVar2,param_3);
        return;
      case 0x27b:
        FUN_141ec4660(lVar2,param_3);
        return;
      case 0x27c:
        FUN_141ec5750(lVar2,param_3);
        return;
      case 0x27d:
        FUN_141ec5980(lVar2,param_3);
        break;
      case 0x27e:
        FUN_141ec4780(lVar2,param_3);
        return;
      }
    }
    return;
  }
                    /* WARNING: Could not recover jumptable at 0x000142795b54. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(*param_1 + 0x98))(param_1,param_3);
  return;
}



//===========================================================
// FUN_142798a40 @ 142798a40   (472 bytes)
//===========================================================

void FUN_142798a40(longlong *param_1,int param_2,undefined8 param_3)

{
  char cVar1;
  int iVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined1 local_18 [8];
  longlong local_10;
  
  if (param_2 == 0x27f) {
    FUN_14287f2b0(param_1 + 0x273);
    lVar3 = FUN_14019b780(&DAT_143ad68a0,0xac0);
    lVar6 = 0;
    lVar4 = lVar6;
    if (lVar3 != 0) {
      lVar4 = FUN_140da2a20(lVar3);
    }
    if ((param_1[0x274] - 1U < 999) || (param_1[0x274] == -1)) {
      FUN_142e52ed0(0x447);
    }
    lVar3 = lVar4 + 0x10;
    if (lVar4 == 0) {
      lVar3 = lVar6;
    }
    local_10 = lVar6;
    if ((lVar3 != 0) && (local_10 = lVar3 + -0x10, local_10 != 0)) {
      if (0xfffff < *(ulonglong *)(lVar3 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar3 + 8) = *(longlong *)(lVar3 + 8) + 1;
      UNLOCK();
    }
    lVar4 = param_1[0x274];
    param_1[0x274] = local_10;
    local_10 = lVar4;
    FUN_14287f2b0(local_18);
    lVar4 = param_1[0x274];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0x274];
    }
    FUN_140dac9b0(lVar4,param_1,param_3);
  }
  else if (param_2 == 0x283) {
    FUN_14287f2b0(param_1 + 0x273);
  }
  else if (param_1[0x274] != 0) {
    if (param_2 == 0x280) {
      uVar5 = FUN_14286e800(param_1 + 0x273);
      FUN_140daf4e0(uVar5,param_3);
    }
    else if (param_2 == 0x281) {
      uVar5 = FUN_14286e800(param_1 + 0x273);
      FUN_140daf530(uVar5,param_3);
    }
    else {
      if (param_2 == 0x282) {
        uVar5 = FUN_14286e800(param_1 + 0x273);
        FUN_140daf5a0(uVar5,param_3);
      }
      else if (param_2 != 0x282) {
        return;
      }
      iVar2 = (**(code **)(*param_1 + 0x50))(param_1);
      if ((iVar2 != 0) && (cVar1 = FUN_1406e8ae0(param_3), cVar1 != '\0')) {
        FUN_142cc4430(DAT_143aa84a0,0);
      }
    }
  }
  return;
}


