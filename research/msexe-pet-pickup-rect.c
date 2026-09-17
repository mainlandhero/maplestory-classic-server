
//===========================================================
// FUN_14038a5b0 @ 14038a5b0   (161 bytes)
//===========================================================

ulonglong FUN_14038a5b0(undefined8 param_1,longlong *param_2)

{
  short sVar1;
  int iVar2;
  undefined4 uVar3;
  longlong lVar4;
  ulonglong uVar5;
  
  if (param_2 == (longlong *)0x0) {
    return 0;
  }
  iVar2 = (**(code **)(*param_2 + 0x88))(param_2);
  if (((iVar2 == 3) && (lVar4 = FUN_140192f80(param_2), lVar4 != 0)) &&
     (sVar1 = FUN_1401ab420(lVar4 + 0xba,*(undefined4 *)(lVar4 + 0xbe)), 0 < sVar1)) {
    sVar1 = FUN_1401ab420(lVar4 + 0xba,*(undefined4 *)(lVar4 + 0xbe));
    return (ulonglong)(uint)(int)sVar1;
  }
  uVar3 = FUN_14019a5d0(param_2 + 4);
  uVar5 = FUN_14038a530(param_1,uVar3);
  return uVar5;
}



//===========================================================
// FUN_14038a530 @ 14038a530   (113 bytes)
//===========================================================

ulonglong FUN_14038a530(undefined8 param_1,undefined4 param_2)

{
  ulonglong uVar1;
  longlong *local_res18;
  longlong *local_res20;
  
  FUN_14039f600(param_1,&local_res20,param_2,0);
  local_res18 = local_res20;
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 8))(local_res20);
  }
  uVar1 = FUN_140910eb0(&local_res18,L"wonderGrade",0);
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))(local_res20);
    return uVar1 & 0xffffffff;
  }
  return uVar1;
}



//===========================================================
// FUN_141ebdad0 @ 141ebdad0   (135 bytes)
//===========================================================

undefined8 FUN_141ebdad0(longlong param_1)

{
  int iVar1;
  undefined4 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  
  if (*(longlong **)(param_1 + 0x120) != (longlong *)0x0) {
    iVar1 = (**(code **)(**(longlong **)(param_1 + 0x120) + 0x50))();
    if (iVar1 != 0) {
      lVar3 = FUN_142cbe730(DAT_143aa84a0);
      if (lVar3 != 0) {
        uVar4 = FUN_142cbe730(DAT_143aa84a0);
        uVar2 = FUN_140230cb0(uVar4,5,*(undefined8 *)(param_1 + 0x150));
        lVar3 = FUN_1402e3e30(uVar4,5,uVar2);
        if (lVar3 != 0) {
          uVar4 = FUN_140192f80(lVar3);
          return uVar4;
        }
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_14113d940 @ 14113d940   (261 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_14113d940(undefined8 *param_1,undefined4 param_2)

{
  longlong *plVar1;
  undefined8 uVar2;
  bool bVar3;
  longlong lVar4;
  longlong *plVar5;
  char cVar6;
  int iVar7;
  undefined8 *puVar8;
  longlong *plVar9;
  
  lVar4 = DAT_143aa84a0;
  if ((DAT_143aa84a0 == 0) || (cVar6 = FUN_140374c80(param_2), cVar6 == '\0')) {
    uVar2 = _UNK_14327c638;
    *param_1 = _DAT_14327c630;
    param_1[1] = uVar2;
    return param_1;
  }
  bVar3 = false;
  plVar9 = (longlong *)*DAT_143aca548;
  cVar6 = *(char *)((longlong)plVar9 + 0x19);
  do {
    if (cVar6 != '\0') {
LAB_14113d9fa:
      puVar8 = (undefined8 *)&DAT_143aca538;
      if (!bVar3) {
        puVar8 = (undefined8 *)&DAT_143aca528;
      }
      uVar2 = puVar8[1];
      *param_1 = *puVar8;
      param_1[1] = uVar2;
      return param_1;
    }
    iVar7 = FUN_140909e30(lVar4,*(undefined4 *)((longlong)plVar9 + 0x1c));
    if (0 < iVar7) {
      bVar3 = true;
      goto LAB_14113d9fa;
    }
    plVar1 = (longlong *)plVar9[2];
    if (*(char *)((longlong)plVar1 + 0x19) == '\0') {
      cVar6 = *(char *)(*plVar1 + 0x19);
      plVar9 = plVar1;
      plVar1 = (longlong *)*plVar1;
      while (cVar6 == '\0') {
        cVar6 = *(char *)(*plVar1 + 0x19);
        plVar9 = plVar1;
        plVar1 = (longlong *)*plVar1;
      }
    }
    else {
      cVar6 = *(char *)(plVar9[1] + 0x19);
      plVar5 = (longlong *)plVar9[1];
      plVar1 = plVar9;
      while ((plVar9 = plVar5, cVar6 == '\0' && (plVar1 == (longlong *)plVar9[2]))) {
        cVar6 = *(char *)(plVar9[1] + 0x19);
        plVar5 = (longlong *)plVar9[1];
        plVar1 = plVar9;
      }
    }
    cVar6 = *(char *)((longlong)plVar9 + 0x19);
  } while( true );
}



//===========================================================
// FUN_140909e30 @ 140909e30   (69 bytes)
//===========================================================

undefined8 FUN_140909e30(longlong param_1,int param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar2 = DAT_143aa84d0;
  if ((param_1 != 0) && (param_2 != 0)) {
    uVar1 = FUN_142cbe740();
    uVar2 = FUN_1407b3df0(uVar2,uVar1,param_2,0);
    return uVar2;
  }
  return 0;
}



//===========================================================
// FUN_143225f30 @ 143225f30   (99 bytes)
//===========================================================

void FUN_143225f30(void)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  
  cVar1 = *(char *)((longlong)*(longlong **)(DAT_143aca548 + 8) + 0x19);
  plVar3 = *(longlong **)(DAT_143aca548 + 8);
  while (cVar1 == '\0') {
    FUN_1401ba2c0(&DAT_143aca548,&DAT_143aca548,plVar3[2]);
    plVar2 = (longlong *)*plVar3;
    thunk_FUN_140205820(plVar3,0x20);
    plVar3 = plVar2;
    cVar1 = *(char *)((longlong)plVar2 + 0x19);
  }
  thunk_FUN_140205820(DAT_143aca548,0x20);
  return;
}



//===========================================================
// FUN_141ec1e70 @ 141ec1e70   (162 bytes)
//===========================================================

void FUN_141ec1e70(longlong param_1)

{
  int iVar1;
  longlong lVar2;
  longlong *plVar3;
  int *piVar4;
  int local_res8 [2];
  undefined1 local_res10 [8];
  
  (**(code **)(*(longlong *)(param_1 + 8) + 0x30))(param_1 + 8,local_res8);
  lVar2 = FUN_141ebdad0(param_1);
  if (lVar2 != 0) {
    plVar3 = (longlong *)FUN_141ebdad0(param_1);
    iVar1 = (**(code **)(*plVar3 + 0x378))(plVar3);
    if (iVar1 != 0) {
      FUN_14179e990(DAT_143ace240,param_1,local_res8,param_1 + 0x550);
    }
  }
  piVar4 = (int *)(**(code **)(*(longlong *)(param_1 + 8) + 0x30))(param_1 + 8,local_res10);
  if (local_res8[0] == *piVar4) {
    (**(code **)(*(longlong *)(param_1 + 8) + 0x30))(param_1 + 8,local_res10);
  }
  return;
}


