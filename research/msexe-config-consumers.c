
//===========================================================
// FUN_142e14bc0 @ 142e14bc0   (383 bytes)
//===========================================================

undefined8 FUN_142e14bc0(undefined8 param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  char cVar3;
  undefined4 uVar4;
  int iVar5;
  undefined8 uVar6;
  int *piVar7;
  undefined8 uVar8;
  int *local_res10;
  longlong local_res18;
  
  uVar8 = 0;
  uVar4 = FUN_142c95c90();
  FUN_142c95cd0(param_1,&local_res18);
  iVar5 = FUN_142c95d00(param_1);
  uVar6 = FUN_1404c6160();
  iVar5 = FUN_1404c6460(uVar6,DAT_143ade330,&local_res18,(longlong)iVar5,uVar4);
  if (iVar5 != 0) {
    uVar8 = 1;
    cVar3 = FUN_14057e4c0();
    if (cVar3 != '\0') {
      uVar6 = FUN_1404c6160();
      local_res10 = (int *)0x0;
      piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,0x26);
      piVar7[1] = 0x15;
      *piVar7 = -1;
      local_res10 = piVar7 + 4;
      piVar7[2] = 0;
      *(undefined1 *)local_res10 = 0;
      uVar2 = s_GC_InitGameLogManager_143498420._12_4_;
      uVar1 = s_GC_InitGameLogManager_143498420._8_4_;
      uVar4 = s_GC_InitGameLogManager_143498420._4_4_;
      *local_res10 = s_GC_InitGameLogManager_143498420._0_4_;
      piVar7[5] = uVar4;
      piVar7[6] = uVar1;
      piVar7[7] = uVar2;
      piVar7[8] = s_GC_InitGameLogManager_143498420._16_4_;
      *(char *)(piVar7 + 9) = s_GC_InitGameLogManager_143498420[0x14];
      if (*piVar7 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar7[1] < 0x15) {
        FUN_142e54290(0x90,piVar7[1],0x15);
      }
      *piVar7 = 1;
      *(undefined1 *)((longlong)local_res10 + 0x15) = 0;
      if (piVar7[1] + 1 < 0x16) {
        FUN_142e54290(0x9c,0x15);
      }
      piVar7[2] = 0x15;
      FUN_1404c7800(uVar6,1,&local_res10);
      if (local_res10 != (int *)0x0) {
        FUN_14019f2c0(local_res10 + -4);
      }
    }
  }
  if (local_res18 != 0) {
    FUN_14019f2c0(local_res18 + -0x10);
  }
  return uVar8;
}



//===========================================================
// FUN_142c43db0 @ 142c43db0   (1432 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 * FUN_142c43db0(undefined8 *param_1,longlong param_2)

{
  longlong *plVar1;
  longlong *plVar2;
  byte bVar3;
  undefined4 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  undefined4 *puVar7;
  undefined8 *puVar8;
  undefined8 uVar9;
  undefined1 auStack_118 [32];
  undefined8 local_f8;
  undefined8 *local_f0;
  undefined8 local_e8;
  undefined4 local_c8 [4];
  int local_b8;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_118;
  plVar1 = param_1 + 1;
  puVar8 = (undefined8 *)0x0;
  DAT_143ac1898 = param_1;
  if (plVar1 == (longlong *)0x0) {
    DAT_143ac1898 = puVar8;
  }
  *param_1 = &PTR_FUN_1434923c0;
  *plVar1 = param_2;
  param_1[2] = 0;
  *(undefined4 *)((longlong)param_1 + 0x1c) = 0;
  param_1[4] = 0;
  param_1[6] = 0;
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  *(undefined8 *)((longlong)param_1 + 100) = 1;
  *(undefined8 *)((longlong)param_1 + 0x6c) = 0;
  *(undefined4 *)((longlong)param_1 + 0x74) = 0;
  plVar2 = param_1 + 0xf;
  *plVar2 = 0;
  param_1[0x10] = 0;
  local_f8 = plVar2;
  local_f0 = param_1;
  lVar5 = FUN_14019b780(&DAT_143ad68a0,0x40);
  *(longlong *)lVar5 = lVar5;
  *(longlong *)(lVar5 + 8) = lVar5;
  *(longlong *)(lVar5 + 0x10) = lVar5;
  *(undefined2 *)(lVar5 + 0x18) = 0x101;
  *plVar2 = lVar5;
  *(undefined4 *)((longlong)param_1 + 0x94) = 0;
  *(undefined4 *)(param_1 + 0x16) = 0;
  *(undefined1 *)((longlong)param_1 + 0xb4) = 1;
  param_1[0x17] = 0;
  *(undefined1 *)(param_1 + 0x18) = 0;
  *(undefined4 *)((longlong)param_1 + 0xc4) = 0;
  *(undefined1 *)(param_1 + 0x19) = 0;
  local_f8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x60);
  puVar6 = puVar8;
  if (local_f8 != (longlong *)0x0) {
    puVar6 = (undefined8 *)FUN_140d21160(local_f8);
  }
  param_1[0x1a] = puVar6;
  local_f8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x38);
  puVar6 = puVar8;
  if (local_f8 != (undefined8 *)0x0) {
    *local_f8 = 0;
    local_f8[1] = 0;
    local_f8[2] = 0;
    local_f8[3] = 0;
    local_f8[4] = 0;
    local_f8[5] = 0;
    local_f8[6] = 0;
    puVar6 = local_f8;
  }
  param_1[0x1b] = puVar6;
  param_1[0x1c] = 0;
  *(undefined4 *)(param_1 + 0x1d) = 0;
  *(undefined2 *)((longlong)param_1 + 0xec) = 0;
  *(undefined1 *)((longlong)param_1 + 0xee) = 0;
  *(undefined4 *)((longlong)param_1 + 0xf4) = 0;
  param_1[0x1f] = 0;
  param_1[0x20] = 0;
  param_1[0x23] = 0;
  param_1[0x25] = 0;
  param_1[0x27] = 0;
  *(undefined4 *)(param_1 + 0x28) = 0;
  param_1[0x29] = 0;
  puVar7 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
  puVar7[1] = 0;
  *puVar7 = 0xffffffff;
  param_1[0x29] = puVar7 + 4;
  puVar7[2] = 0;
  *(undefined1 *)param_1[0x29] = 0;
  lVar5 = param_1[0x29];
  if (*(int *)(lVar5 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)(lVar5 + -0xc) < 0) {
    FUN_142e54290(0x90,*(int *)(lVar5 + -0xc),0);
  }
  *(undefined4 *)(lVar5 + -0x10) = 1;
  *(undefined1 *)param_1[0x29] = 0;
  if (*(int *)(lVar5 + -0xc) + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  *(undefined4 *)(lVar5 + -8) = 0;
  param_1[0x2b] = 0;
  param_1[0x2c] = 0;
  lVar5 = FUN_14019b780(&DAT_143ad68a0,0x20);
  *(longlong *)lVar5 = lVar5;
  *(longlong *)(lVar5 + 8) = lVar5;
  param_1[0x2b] = lVar5;
  param_1[0x2d] = 0;
  param_1[0x2e] = 0;
  lVar5 = FUN_14019b780(&DAT_143ad68a0,0x28);
  *(longlong *)lVar5 = lVar5;
  *(longlong *)(lVar5 + 8) = lVar5;
  param_1[0x2d] = lVar5;
  param_1[0x2f] = 0;
  param_1[0x30] = 0;
  lVar5 = FUN_14019b780(&DAT_143ad68a0,0x38);
  *(longlong *)lVar5 = lVar5;
  *(longlong *)(lVar5 + 8) = lVar5;
  param_1[0x2f] = lVar5;
  param_1[0x32] = 0;
  *(undefined1 *)(param_1 + 0x33) = 0;
  *(undefined4 *)((longlong)param_1 + 0x19c) = 0;
  *(undefined1 *)(param_1 + 0x34) = 0;
  param_1[0x35] = 0;
  local_f8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,8);
  if (local_f8 != (longlong *)0x0) {
    puVar8 = (undefined8 *)FUN_142e0eb40(local_f8);
  }
  param_1[0x36] = puVar8;
  param_1[0x37] = 0;
  param_1[0x38] = 0;
  uVar4 = FUN_142c95c90(*plVar1);
  *(undefined4 *)(param_1 + 0xd) = uVar4;
  bVar3 = FUN_142c95d10(*plVar1);
  *(uint *)(param_1 + 0x2a) = (uint)bVar3;
  lVar5 = DAT_143aa84a0;
  puVar8 = (undefined8 *)FUN_142c95ca0(*plVar1,&local_f8);
  FUN_142cb5f70(lVar5,*puVar8);
  if (local_f8 != (longlong *)0x0) {
    FUN_14019f2c0((longlong)local_f8 + -0x10);
  }
  lVar5 = DAT_143aa84a0;
  uVar4 = FUN_142c95c80(*plVar1);
  FUN_142cb8410(lVar5,uVar4);
  *(undefined4 *)(DAT_143aa84a0 + 0x2520) = *(undefined4 *)(*plVar1 + 8);
  *(undefined4 *)(DAT_143aa84a0 + 0x2524) = *(undefined4 *)(*plVar1 + 0xc);
  *(undefined4 *)(DAT_143aa84a0 + 0x2528) = *(undefined4 *)(*plVar1 + 0x10);
  lVar5 = DAT_143aa84a0;
  uVar9 = FUN_142c95fb0(*plVar1,&local_f8);
  FUN_142ce96b0(lVar5,uVar9);
  uVar4 = (*DAT_143262db0)();
  *(undefined4 *)(param_1 + 0x26) = uVar4;
  *(undefined4 *)((longlong)param_1 + 0x134) = uVar4;
  if (param_1[0x1c] != 0) {
    thunk_FUN_140205820(param_1[0x1c] + -8,0);
    param_1[0x1c] = 0;
  }
  puVar8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,&DAT_00001008);
  if (puVar8 == (undefined8 *)0x0) {
    param_1[0x1c] = 0;
  }
  else {
    param_1[0x1c] = puVar8 + 1;
    if (puVar8 + 1 != (undefined8 *)0x0) {
      *puVar8 = 0x1000;
    }
  }
  uVar4 = (*DAT_143ad5448)();
  *(undefined4 *)(param_1 + 3) = uVar4;
  local_c8[0] = 0x94;
  (*DAT_143ad56a8)(local_c8);
  *(uint *)(param_1 + 5) = (uint)(local_b8 == 1);
  local_f8 = (longlong *)0x8;
  (*DAT_143262b98)(0x3a,8,&local_f8);
  *(undefined4 *)(param_1 + 0x17) = local_f8._4_4_;
  local_f8 = (longlong *)((ulonglong)local_f8 & 0xfffffff9ffffffff);
  (*DAT_143262b98)(0x3b,8,&local_f8);
  local_e8 = 0x18;
  (*DAT_143262b98)(0x32,0x18,&local_e8);
  *(undefined4 *)((longlong)param_1 + 0xbc) = local_e8._4_4_;
  local_e8 = local_e8 & 0xfffffff9ffffffff;
  (*DAT_143262b98)(0x33,0x18,&local_e8);
  puVar8 = (undefined8 *)FUN_142c95f80(*plVar1,&local_f8);
  if (param_1[0x38] != 0) {
    FUN_14019f2c0(param_1[0x38] + -0x10);
    param_1[0x38] = 0;
  }
  param_1[0x38] = *puVar8;
  *puVar8 = 0;
  if (local_f8 != (longlong *)0x0) {
    FUN_14019f2c0((longlong)local_f8 - 0x10);
  }
  lVar5 = (*DAT_1432622f8)("winemac.drv");
  if (lVar5 != 0) {
    *(undefined1 *)(DAT_143aa84a0 + 0x4194) = 1;
  }
  return param_1;
}


