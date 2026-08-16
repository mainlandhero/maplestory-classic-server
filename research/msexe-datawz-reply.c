
//===========================================================
// FUN_1406efcc0 @ 1406efcc0   (147 bytes)
//===========================================================

int FUN_1406efcc0(undefined8 param_1)

{
  byte bVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  
  uVar3 = 0;
  iVar4 = 5;
  uVar2 = uVar3;
  while( true ) {
    bVar1 = FUN_1406e8ae0(param_1);
    uVar2 = uVar2 | (bVar1 & 0x7f) << ((byte)uVar3 & 0x1f);
    if (-1 < (char)bVar1) break;
    iVar4 = iVar4 + -1;
    if (iVar4 < 1) {
      FUN_140919f30(&DAT_143271f04,0x398,0);
    }
    uVar3 = uVar3 + 7;
  }
  return ((uVar2 & 1) * -2 + 1) * (((int)uVar2 >> 1) + (uVar2 & 1));
}



//===========================================================
// FUN_1406e8c20 @ 1406e8c20   (145 bytes)
//===========================================================

undefined4 FUN_1406e8c20(longlong param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined1 local_30 [40];
  
  iVar1 = *(int *)(param_1 + 0x18);
  iVar2 = *(int *)(param_1 + 0x24);
  lVar4 = *(longlong *)(param_1 + 0x10);
  if (lVar4 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar4 = *(longlong *)(param_1 + 0x10);
    if (lVar4 != 0) goto LAB_1406e8c5b;
  }
  else {
LAB_1406e8c5b:
    if (*(int *)(lVar4 + -8) != 0) goto LAB_1406e8c70;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e8c70:
  if ((uint)(iVar1 - iVar2) < 4) {
    FUN_1401bb8b0(local_30,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_30,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar3 = *(undefined4 *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) + 4;
  return uVar3;
}



//===========================================================
// FUN_1406e9170 @ 1406e9170   (209 bytes)
//===========================================================

void FUN_1406e9170(longlong param_1,undefined1 *param_2,uint param_3)

{
  undefined1 uVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  longlong lVar5;
  undefined1 *puVar6;
  ulonglong uVar7;
  undefined1 local_30 [40];
  
  uVar7 = (ulonglong)param_3;
  iVar2 = *(int *)(param_1 + 0x18);
  iVar3 = *(int *)(param_1 + 0x24);
  lVar5 = *(longlong *)(param_1 + 0x10);
  if (lVar5 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar5 = *(longlong *)(param_1 + 0x10);
    if (lVar5 != 0) goto LAB_1406e91be;
  }
  else {
LAB_1406e91be:
    if (*(int *)(lVar5 + -8) != 0) goto LAB_1406e91d3;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e91d3:
  uVar4 = *(uint *)(param_1 + 0x24);
  puVar6 = (undefined1 *)((ulonglong)uVar4 + *(longlong *)(param_1 + 0x10));
  if ((uint)(iVar2 - iVar3) < param_3) {
    FUN_1401bb8b0(local_30,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_30,(ThrowInfo *)&DAT_143a3b118);
  }
  if (0 < (int)param_3) {
    do {
      uVar1 = *puVar6;
      puVar6 = puVar6 + 1;
      *param_2 = uVar1;
      param_2 = param_2 + 1;
      uVar7 = uVar7 - 1;
    } while (uVar7 != 0);
    uVar4 = *(uint *)(param_1 + 0x24);
  }
  *(uint *)(param_1 + 0x24) = uVar4 + param_3;
  return;
}



//===========================================================
// FUN_1408cc4f0 @ 1408cc4f0   (110 bytes)
//===========================================================

undefined4 FUN_1408cc4f0(longlong *param_1)

{
  longlong lVar1;
  int iVar2;
  longlong lVar3;
  undefined4 local_res8 [2];
  
  lVar1 = param_1[1];
  iVar2 = *(int *)((longlong)param_1 + 0xc);
  lVar3 = *param_1;
  if (lVar3 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar3 = *param_1;
    if (lVar3 != 0) goto LAB_1408cc524;
  }
  else {
LAB_1408cc524:
    if (*(int *)(lVar3 + -8) != 0) goto LAB_1408cc539;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1408cc539:
  iVar2 = FUN_1406e8380(local_res8,(ulonglong)*(uint *)((longlong)param_1 + 0xc) + *param_1,
                        (int)lVar1 - iVar2);
  *(int *)((longlong)param_1 + 0xc) = *(int *)((longlong)param_1 + 0xc) + iVar2;
  return local_res8[0];
}



//===========================================================
// FUN_1408cc3f0 @ 1408cc3f0   (111 bytes)
//===========================================================

undefined1 FUN_1408cc3f0(longlong *param_1)

{
  longlong lVar1;
  int iVar2;
  longlong lVar3;
  undefined1 local_res8 [8];
  
  lVar1 = param_1[1];
  iVar2 = *(int *)((longlong)param_1 + 0xc);
  lVar3 = *param_1;
  if (lVar3 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar3 = *param_1;
    if (lVar3 != 0) goto LAB_1408cc424;
  }
  else {
LAB_1408cc424:
    if (*(int *)(lVar3 + -8) != 0) goto LAB_1408cc439;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1408cc439:
  iVar2 = FUN_1406e82f0(local_res8,(ulonglong)*(uint *)((longlong)param_1 + 0xc) + *param_1,
                        (int)lVar1 - iVar2);
  *(int *)((longlong)param_1 + 0xc) = *(int *)((longlong)param_1 + 0xc) + iVar2;
  return local_res8[0];
}


