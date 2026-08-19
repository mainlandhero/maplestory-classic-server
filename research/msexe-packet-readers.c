
//===========================================================
// FUN_1406e8ae0 @ 1406e8ae0   (145 bytes)
//===========================================================

undefined1 FUN_1406e8ae0(longlong param_1)

{
  undefined1 uVar1;
  int iVar2;
  int iVar3;
  longlong lVar4;
  undefined1 local_30 [40];
  
  iVar2 = *(int *)(param_1 + 0x18);
  iVar3 = *(int *)(param_1 + 0x24);
  lVar4 = *(longlong *)(param_1 + 0x10);
  if (lVar4 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar4 = *(longlong *)(param_1 + 0x10);
    if (lVar4 != 0) goto LAB_1406e8b1b;
  }
  else {
LAB_1406e8b1b:
    if (*(int *)(lVar4 + -8) != 0) goto LAB_1406e8b30;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e8b30:
  if (iVar2 == iVar3) {
    FUN_1401bb8b0(local_30,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_30,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar1 = *(undefined1 *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) + 1;
  return uVar1;
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
// FUN_1406e9050 @ 1406e9050   (242 bytes)
//===========================================================

undefined8 *
FUN_1406e9050(longlong param_1,undefined8 *param_2,undefined8 param_3,undefined8 param_4)

{
  ushort uVar1;
  longlong lVar2;
  ushort *puVar3;
  uint uVar4;
  undefined1 local_48 [16];
  undefined1 local_38 [32];
  
  *param_2 = 0;
  uVar4 = *(int *)(param_1 + 0x18) - *(int *)(param_1 + 0x24);
  lVar2 = *(longlong *)(param_1 + 0x10);
  if (lVar2 == 0) {
    FUN_142e52d50(0xd0,1,param_3,param_4,1);
    lVar2 = *(longlong *)(param_1 + 0x10);
    if (lVar2 != 0) goto LAB_1406e90a4;
  }
  else {
LAB_1406e90a4:
    if (*(int *)(lVar2 + -8) != 0) goto LAB_1406e90bc;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e90bc:
  puVar3 = (ushort *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  if (uVar4 < 2) {
    FUN_1401bb8b0(local_48,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_48,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar1 = *puVar3;
  if (uVar4 < uVar1 + 2) {
    FUN_1401bb8b0(local_38,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_38,(ThrowInfo *)&DAT_143a3b118);
  }
  FUN_1401d66b0(param_2,puVar3 + 1);
  *(int *)(param_1 + 0x24) = *(int *)(param_1 + 0x24) + uVar1 + 2;
  return param_2;
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
// FUN_1406e8ae0 @ 1406e8ae0   (145 bytes)
//===========================================================

undefined1 FUN_1406e8ae0(longlong param_1)

{
  undefined1 uVar1;
  int iVar2;
  int iVar3;
  longlong lVar4;
  undefined1 local_30 [40];
  
  iVar2 = *(int *)(param_1 + 0x18);
  iVar3 = *(int *)(param_1 + 0x24);
  lVar4 = *(longlong *)(param_1 + 0x10);
  if (lVar4 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar4 = *(longlong *)(param_1 + 0x10);
    if (lVar4 != 0) goto LAB_1406e8b1b;
  }
  else {
LAB_1406e8b1b:
    if (*(int *)(lVar4 + -8) != 0) goto LAB_1406e8b30;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e8b30:
  if (iVar2 == iVar3) {
    FUN_1401bb8b0(local_30,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_30,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar1 = *(undefined1 *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) + 1;
  return uVar1;
}



//===========================================================
// FUN_1406e8b80 @ 1406e8b80   (146 bytes)
//===========================================================

undefined2 FUN_1406e8b80(longlong param_1)

{
  undefined2 uVar1;
  int iVar2;
  int iVar3;
  longlong lVar4;
  undefined1 local_30 [40];
  
  iVar2 = *(int *)(param_1 + 0x18);
  iVar3 = *(int *)(param_1 + 0x24);
  lVar4 = *(longlong *)(param_1 + 0x10);
  if (lVar4 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar4 = *(longlong *)(param_1 + 0x10);
    if (lVar4 != 0) goto LAB_1406e8bbb;
  }
  else {
LAB_1406e8bbb:
    if (*(int *)(lVar4 + -8) != 0) goto LAB_1406e8bd0;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e8bd0:
  if ((uint)(iVar2 - iVar3) < 2) {
    FUN_1401bb8b0(local_30,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_30,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar1 = *(undefined2 *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) + 2;
  return uVar1;
}


