
//===========================================================
// FUN_14170f740 @ 14170f740   (132 bytes)
//===========================================================

ulonglong FUN_14170f740(longlong param_1,undefined4 param_2,undefined4 param_3)

{
  uint uVar1;
  ulonglong uVar2;
  int iVar3;
  
  uVar1 = FUN_141710510();
  if (uVar1 == 0) {
    FUN_142645170(param_1 + 0x78);
    *(undefined4 *)(param_1 + 0x11b0) = 0;
    uVar2 = FUN_14170ff00(param_1 + 8,param_2,param_3);
  }
  else {
    iVar3 = *(int *)(param_1 + 0x11cc);
    if (iVar3 == 0) {
      (**(code **)(*(longlong *)(param_1 + 8) + 0x20))(param_1 + 8,param_2,param_3);
      iVar3 = *(int *)(param_1 + 0x11cc);
    }
    uVar2 = 0;
    if (iVar3 != 0) {
      uVar2 = (ulonglong)uVar1;
    }
  }
  return uVar2;
}



//===========================================================
// FUN_141aeed80 @ 141aeed80   (278 bytes)
//===========================================================

undefined8 * FUN_141aeed80(undefined8 *param_1)

{
  longlong lVar1;
  undefined4 *puVar2;
  
  FUN_14170fd10();
  *param_1 = &PTR_FUN_1433fa300;
  param_1[1] = &PTR_LAB_1433fa380;
  param_1[3] = &PTR_FUN_1433fa458;
  FUN_142644810(param_1 + 0xf);
  param_1[0x234] = 0;
  puVar2 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
  puVar2[1] = 0;
  *puVar2 = 0xffffffff;
  param_1[0x234] = puVar2 + 4;
  puVar2[2] = 0;
  *(undefined1 *)param_1[0x234] = 0;
  lVar1 = param_1[0x234];
  if (*(int *)(lVar1 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)(lVar1 + -0xc) < 0) {
    FUN_142e54290(0x90,*(int *)(lVar1 + -0xc),0);
  }
  *(undefined4 *)(lVar1 + -0x10) = 1;
  *(undefined1 *)param_1[0x234] = 0;
  if (*(int *)(lVar1 + -0xc) + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  *(undefined4 *)(lVar1 + -8) = 0;
  param_1[0x235] = 0;
  *(undefined4 *)(param_1 + 0x236) = 0;
  *(undefined4 *)(param_1 + 0x238) = 0;
  *(undefined4 *)((longlong)param_1 + 0x11cc) = 1;
  param_1[0x23b] = 0;
  *(undefined4 *)(param_1 + 0x23c) = 0;
  return param_1;
}



//===========================================================
// FUN_1416da1a0 @ 1416da1a0   (636 bytes)
//===========================================================

ulonglong FUN_1416da1a0(longlong *param_1,int param_2,int param_3)

{
  longlong *plVar1;
  undefined8 *puVar2;
  int iVar3;
  int iVar4;
  longlong lVar5;
  int *piVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  int *local_res8;
  undefined1 local_58 [8];
  undefined8 local_50;
  undefined8 *local_40;
  
  lVar5 = 0x90;
  if (param_1[0x12] == 0) {
    lVar5 = 0x80;
  }
  puVar2 = *(undefined8 **)(lVar5 + (longlong)param_1);
  local_40 = puVar2;
  if (puVar2 != (undefined8 *)0x0) {
    if (0xfffff < (ulonglong)puVar2[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    puVar2[1] = puVar2[1] + 1;
    UNLOCK();
  }
  puVar2 = local_40;
  if (((((int)param_1[0x238] == 0) || (param_2 < *(int *)((longlong)param_1 + 0x11c4))) ||
      (*(int *)((longlong)param_1 + 0x11cc) <= param_2)) ||
     (((param_3 < (int)param_1[0x239] || ((int)param_1[0x23a] <= param_3)) ||
      (local_40 == (undefined8 *)0x0)))) {
    FUN_142645170(param_1 + 0x13);
  }
  else {
    iVar3 = (**(code **)(*param_1 + 0x90))(param_1);
    iVar4 = (**(code **)(*param_1 + 0x98))(param_1);
    if ((int)param_1[0x1a] == 0) {
      local_50 = 0;
      local_res8 = (int *)0x0;
      piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
      piVar6[1] = 0;
      *piVar6 = -1;
      local_res8 = piVar6 + 4;
      piVar6[2] = 0;
      *(undefined1 *)local_res8 = 0;
      if (*piVar6 != -1) {
        FUN_142e52dd0();
      }
      if (piVar6[1] < 0) {
        FUN_142e54290(0x90,piVar6[1],0);
      }
      *piVar6 = 1;
      *(undefined1 *)local_res8 = 0;
      if (piVar6[1] + 1 < 1) {
        FUN_142e54290(0x9c,0);
      }
      piVar6[2] = 0;
      FUN_142694130(param_1 + 0x13,iVar3 + param_2 + 0x14,iVar4 + param_3 + 0x14,puVar2,0,0,0,0,0,0,
                    1,&local_res8,local_58,0);
    }
    else {
      FUN_142664b50(param_1 + 0x13,iVar3 + param_2 + 0x14,iVar4 + param_3 + 0x14,0);
    }
  }
  uVar7 = FUN_14170ff00(param_1,param_2,param_3);
  uVar8 = uVar7;
  if (puVar2 != (undefined8 *)0x0) {
    if (0xffffe < puVar2[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = puVar2 + 1;
    lVar5 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    uVar8 = uVar7 & 0xffffffff;
    if ((int)lVar5 == 1) {
      (**(code **)*local_40)(local_40,1);
    }
  }
  return uVar8;
}



//===========================================================
// FUN_1416da0f0 @ 1416da0f0   (162 bytes)
//===========================================================

void FUN_1416da0f0(longlong param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 in_stack_00000030;
  undefined4 in_stack_00000038;
  undefined4 *in_stack_00000040;
  
  *(undefined8 *)(param_1 + 0x11cc) = 0;
  *(undefined4 *)(param_1 + 0x11d4) = in_stack_00000030;
  *(undefined4 *)(param_1 + 0x11d8) = in_stack_00000038;
  *(undefined4 *)(param_1 + 0x11f4) = 0;
  if (in_stack_00000040 != (undefined4 *)0x0) {
    *(undefined4 *)(param_1 + 0x11c8) = *in_stack_00000040;
    if (0 < (int)(in_stack_00000040[3] - in_stack_00000040[1])) {
      uVar1 = in_stack_00000040[2];
      uVar2 = in_stack_00000040[3];
      uVar3 = in_stack_00000040[4];
      *(undefined4 *)(param_1 + 0x11cc) = in_stack_00000040[1];
      *(undefined4 *)(param_1 + 0x11d0) = uVar1;
      *(undefined4 *)(param_1 + 0x11d4) = uVar2;
      *(undefined4 *)(param_1 + 0x11d8) = uVar3;
    }
    *(undefined4 *)(param_1 + 0x11dc) = in_stack_00000040[5];
    *(undefined4 *)(param_1 + 0x11e0) = in_stack_00000040[6];
    *(undefined4 *)(param_1 + 0x11e4) = in_stack_00000040[7];
    *(undefined4 *)(param_1 + 0x11e8) = in_stack_00000040[8];
    *(undefined4 *)(param_1 + 0x11f0) = in_stack_00000040[10];
    *(undefined4 *)(param_1 + 0x11ec) = in_stack_00000040[9];
  }
  FUN_141710020();
  return;
}


