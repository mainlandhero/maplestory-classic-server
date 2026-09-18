
//===========================================================
// FUN_142382330 @ 142382330   (772 bytes)
//===========================================================

bool FUN_142382330(longlong *param_1,longlong param_2,longlong param_3,undefined4 param_4,
                  undefined4 param_5)

{
  longlong *plVar1;
  undefined1 uVar2;
  longlong *plVar3;
  char cVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  longlong lVar8;
  longlong lVar9;
  undefined8 *puVar10;
  int iVar11;
  uint uVar12;
  undefined2 local_res8;
  int local_58 [2];
  int local_50 [2];
  undefined8 local_48;
  longlong local_40;
  undefined1 local_38 [8];
  undefined8 *local_30;
  
  iVar5 = (**(code **)(*param_1 + 0x50))();
  if ((((iVar5 == 0) || (param_2 == 0)) || (param_3 == 0)) || (DAT_143aa84a0 == 0)) {
    return false;
  }
  iVar5 = FUN_141787900(param_2);
  uVar6 = FUN_141787910(param_2);
  if (*(int *)((longlong)param_1 + 0x304) != iVar5) {
    if (iVar5 - 1U < 6) {
      lVar8 = param_1[0x5f];
      if (lVar8 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar8 = param_1[0x5f];
      }
      FUN_14170af70(lVar8,iVar5 - 1U,1);
    }
    FUN_142c0c420(DAT_143abfdf8,param_3,param_2,0);
    return false;
  }
  lVar8 = FUN_142cbe730(DAT_143aa84a0);
  lVar9 = FUN_1402e3cd0(lVar8,local_38,iVar5,uVar6);
  puVar10 = local_30;
  plVar3 = *(longlong **)(lVar9 + 8);
  if (local_30 != (undefined8 *)0x0) {
    if (0xffffe < local_30[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = puVar10 + 1;
    lVar9 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar9 == 1) && (local_30 != (undefined8 *)0x0)) {
      (**(code **)*local_30)(local_30,1);
    }
    local_30 = (undefined8 *)0x0;
  }
  if (plVar3 == (longlong *)0x0) {
    return false;
  }
  if (uVar6 < 0x80000000) {
    return false;
  }
  iVar7 = *(int *)((longlong)param_1 + 0x304);
  if (iVar7 == 1) {
    local_58[0] = *(int *)(lVar8 + 0x37);
    local_res8 = CONCAT11(*(byte *)(lVar8 + 0x36) ^ *(byte *)(lVar8 + 0x34),
                          *(byte *)(lVar8 + 0x35) ^ *(byte *)(lVar8 + 0x33));
    uVar12 = *(byte *)(lVar8 + 0x33) ^ 0xbaadf00d;
    uVar12 = (uVar12 >> 5 | uVar12 << 0x1b) + (uint)*(byte *)(lVar8 + 0x35) ^
             (uint)*(byte *)(lVar8 + 0x34);
    local_50[0] = (uVar12 >> 5 | uVar12 << 0x1b) + (uint)*(byte *)(lVar8 + 0x36);
    if (local_50[0] != local_58[0]) {
      local_48 = FUN_1418039d0(5);
      puVar10 = (undefined8 *)FUN_1401a0ed0(&local_40,&local_48,local_50,local_58);
      FUN_141804970(&DAT_143271f04,0x53,5,*puVar10);
      if (local_40 != 0) {
        FUN_14019f2c0(local_40 + -0x10);
      }
    }
    uVar2 = *(undefined1 *)(lVar8 + 0x1a6);
    cVar4 = FUN_1402df070(uVar2,0);
    if (((cVar4 == '\0') && (cVar4 = (**(code **)(*plVar3 + 0x2c0))(plVar3), cVar4 != '\0')) &&
       (cVar4 = FUN_1402df080((int)local_res8,-uVar6), cVar4 != '\0')) {
      cVar4 = (**(code **)(*plVar3 + 0x2d8))(plVar3,uVar2,0);
      if (cVar4 != '\0') {
        return false;
      }
      iVar7 = FUN_1402df060(-uVar6);
      iVar11 = -iVar7;
      if (((0x1f < iVar11 - 3000U) && (0x1f < iVar11 - 0xc1cU)) && (0x1f < iVar11 - 0xc80U)) {
        return false;
      }
      goto LAB_1423824b7;
    }
  }
  else if ((iVar7 != 6) && (iVar7 != 5)) {
    return false;
  }
  iVar7 = FUN_142386a30(param_1 + -1,param_4,param_5);
LAB_1423824b7:
  iVar5 = FUN_1417dd7e0(iVar5,uVar6,iVar7);
  if (iVar5 != 0) {
    FUN_1429edb20(PTR_u_DragEnd_143a47bd8);
    return iVar5 != 0;
  }
  return false;
}


