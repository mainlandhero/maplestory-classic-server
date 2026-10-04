
//===========================================================
// FUN_141aaae90 @ 141aaae90   (809 bytes)
//===========================================================

longlong FUN_141aaae90(longlong *param_1,longlong param_2,undefined4 param_3,int param_4,int param_5
                      ,undefined4 param_6,undefined4 param_7,longlong *param_8,int param_9)

{
  undefined8 *puVar1;
  longlong *plVar2;
  code *pcVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong *plVar8;
  longlong local_res8;
  longlong local_res10;
  undefined4 local_res18 [4];
  undefined4 uVar9;
  int local_60;
  int local_5c;
  longlong *local_58;
  longlong *local_50;
  longlong local_48;
  longlong *local_40;
  
  local_res10 = param_2;
  local_res18[0] = param_3;
  local_res8 = FUN_14019b780(&DAT_143ad68a0,0x11e8);
  lVar4 = 0;
  if (local_res8 != 0) {
    lVar4 = FUN_141aeed80(local_res8);
  }
  lVar6 = lVar4 + 0x18;
  if (lVar4 == 0) {
    lVar6 = 0;
  }
  if (lVar6 == 0) {
    *(undefined8 *)(param_2 + 8) = 0;
  }
  else {
    *(longlong *)(param_2 + 8) = lVar6 + -0x18;
    if (lVar6 + -0x18 != 0) {
      if (0xfffff < *(ulonglong *)(lVar6 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar6 + 8) = *(longlong *)(lVar6 + 8) + 1;
      UNLOCK();
      param_3 = local_res18[0];
    }
  }
  uVar9 = 1;
  plVar8 = *(longlong **)(param_2 + 8);
  if (plVar8 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
    plVar8 = *(longlong **)(param_2 + 8);
  }
  puVar1 = (undefined8 *)*param_1;
  (**(code **)(*plVar8 + 0x10))
            (plVar8,*puVar1,param_3,*(int *)((longlong)puVar1 + 0xc) + param_4,
             param_5 + *(int *)(puVar1 + 2),param_6,param_7,0,uVar9);
  lVar4 = *(longlong *)(param_2 + 8);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar4 = *(longlong *)(param_2 + 8);
  }
  local_res8 = 0;
  FUN_14019a260(&local_res8,param_8);
  local_50 = &local_res8;
  (**(code **)(*(longlong *)(lVar4 + 8) + 0x80))((longlong *)(lVar4 + 8),1);
  FUN_14019a260(lVar4 + 0x11a0,&local_res8);
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  if (param_9 != 0) {
    lVar4 = *(longlong *)(param_2 + 8);
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = *(longlong *)(param_2 + 8);
    }
    uVar7 = *(ulonglong *)(lVar4 + 0x11c4);
    FUN_14170f7d0(lVar4,&local_60);
    if (param_9 == 2) {
      uVar7 = (ulonglong)(uint)-local_60;
    }
    else if (param_9 == 6) {
      uVar7 = (ulonglong)(uint)-local_5c << 0x20;
    }
    else if (param_9 == 8) {
      uVar7 = CONCAT44(-local_5c,-local_60);
    }
    lVar4 = *(longlong *)(param_2 + 8);
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = *(longlong *)(param_2 + 8);
    }
    *(ulonglong *)(lVar4 + 0x11c4) = uVar7;
  }
  plVar8 = *(longlong **)(param_2 + 8);
  if (plVar8 != (longlong *)0x0) {
    if (plVar8 == (longlong *)0xffffffffffffffe8) {
      plVar8 = (longlong *)0x0;
    }
    local_40 = plVar8;
    if (plVar8 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar8[4]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar8[4] = plVar8[4] + 1;
      UNLOCK();
    }
    plVar8 = local_40;
    local_50 = &local_48;
    if (local_40 == (longlong *)0x0) {
      FUN_14126b0d0(&local_48);
    }
    else {
      plVar2 = *(longlong **)(*param_1 + 0x18);
      if (plVar2 != (longlong *)0x0) {
        pcVar3 = *(code **)(*local_40 + 0x78);
        local_58 = plVar2;
        (**(code **)(*plVar2 + 8))();
        (*pcVar3)(plVar8,&local_58);
      }
      uVar5 = FUN_141afab70(*param_1 + 0x28,0xffffffff);
      FUN_14126acd0(uVar5,&local_48);
      FUN_14126b0d0(&local_48);
    }
    lVar4 = FUN_141af8580(*param_1 + 0x1f8,local_res18);
    if ((*(longlong *)(lVar4 + 0x20) - 1U < 999) || (*(longlong *)(lVar4 + 0x20) == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (lVar4 + 0x18 == param_2) {
      FUN_142e52d50(0x45c,1);
    }
    lVar6 = *(longlong *)(param_2 + 8);
    if (lVar6 != 0) {
      if (0xfffff < *(ulonglong *)(lVar6 + 0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar6 + 0x20) = *(longlong *)(lVar6 + 0x20) + 1;
      UNLOCK();
    }
    FUN_14147d360(lVar4 + 0x18);
    *(undefined8 *)(lVar4 + 0x20) = *(undefined8 *)(param_2 + 8);
  }
  if (*param_8 != 0) {
    FUN_14019f2c0(*param_8 + -0x10);
  }
  return param_2;
}


