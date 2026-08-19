
//===========================================================
// FUN_1406e9b20 @ 1406e9b20   (4 bytes)
//===========================================================

undefined4 FUN_1406e9b20(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x24);
}



//===========================================================
// FUN_1406e8460 @ 1406e8460   (92 bytes)
//===========================================================

uint FUN_1406e8460(undefined1 *param_1,uint param_2,undefined1 *param_3,uint param_4)

{
  undefined1 uVar1;
  ulonglong uVar2;
  undefined1 local_18 [24];
  
  if (param_2 <= param_4) {
    if (0 < (int)param_2) {
      uVar2 = (ulonglong)param_2;
      do {
        uVar1 = *param_3;
        param_3 = param_3 + 1;
        *param_1 = uVar1;
        param_1 = param_1 + 1;
        uVar2 = uVar2 - 1;
      } while (uVar2 != 0);
    }
    return param_2;
  }
  FUN_1401bb8b0(local_18,0x26);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_18,(ThrowInfo *)&DAT_143a3b118);
}



//===========================================================
// FUN_140738db0 @ 140738db0   (81 bytes)
//===========================================================

int FUN_140738db0(int param_1,int param_2)

{
  ulonglong uVar1;
  
  if (param_1 == param_2) {
    return param_1;
  }
  if (param_2 - param_1 != 0) {
    uVar1 = FUN_1407386b0(&DAT_143ac1ab0);
    return (int)((uVar1 & 0xffffffff) % (ulonglong)(uint)(param_2 - param_1)) + param_1;
  }
  return param_1;
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


