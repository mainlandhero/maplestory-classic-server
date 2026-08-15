
//===========================================================
// FUN_1406ed520 @ 1406ed520   (106 bytes)
//===========================================================

undefined4 * FUN_1406ed520(undefined4 *param_1,int param_2)

{
  *(undefined8 *)(param_1 + 0x102) = 0;
  *(undefined8 *)(param_1 + 0x104) = 0;
  *(undefined8 *)(param_1 + 0x106) = 0;
  *(undefined8 *)(param_1 + 0x110) = 0;
  param_1[0x108] = 0x100;
  param_1[0x10d] = param_2;
  *param_1 = 0;
  param_1[0x10a] = 0;
  if (param_2 != 0x7fffffff) {
    FUN_1406ed940();
  }
  *(undefined8 *)(param_1 + 0x10b) = 0;
  return param_1;
}



//===========================================================
// FUN_1406ed840 @ 1406ed840   (152 bytes)
//===========================================================

void FUN_1406ed840(longlong param_1,undefined1 param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x428);
  if (*(uint *)(param_1 + 0x420) < iVar1 + 1U) {
    FUN_1406ec380(param_1 + 8);
    iVar1 = *(int *)(param_1 + 0x428);
  }
  if (iVar1 < 0x400) {
    *(undefined1 *)((longlong)iVar1 + 8 + param_1) = param_2;
    *(int *)(param_1 + 0x428) = *(int *)(param_1 + 0x428) + 1;
    return;
  }
  *(undefined1 *)
   ((longlong)((iVar1 + -0x400) % 0xff8) +
   *(longlong *)(*(longlong *)(param_1 + 0x408) + (longlong)((iVar1 + -0x400) / 0xff8) * 8)) =
       param_2;
  *(int *)(param_1 + 0x428) = *(int *)(param_1 + 0x428) + 1;
  return;
}



//===========================================================
// FUN_1406ed9d0 @ 1406ed9d0   (89 bytes)
//===========================================================

void FUN_1406ed9d0(longlong param_1,undefined4 param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x428);
  if (*(uint *)(param_1 + 0x420) < iVar1 + 4U) {
    FUN_1406ec380(param_1 + 8,iVar1 + 4U);
    iVar1 = *(int *)(param_1 + 0x428);
  }
  FUN_1406ec880(param_1 + 8,iVar1,param_2);
  *(int *)(param_1 + 0x428) = *(int *)(param_1 + 0x428) + 4;
  return;
}


### no function at 1406ed650

//===========================================================
// FUN_1415dc6f0 @ 1415dc6f0   (51 bytes)
//===========================================================

void FUN_1415dc6f0(undefined8 param_1)

{
  undefined1 local_28 [40];
  
  FUN_1415dd590(local_28);
  FUN_1415dea70(param_1,local_28);
  FUN_1406f12e0(local_28);
  return;
}



//===========================================================
// FUN_1406ed660 @ 1406ed660   (10 bytes)
//===========================================================

int FUN_1406ed660(longlong param_1)

{
  return *(int *)(param_1 + 0x428) + 4;
}



//===========================================================
// FUN_1406ed670 @ 1406ed670   (351 bytes)
//===========================================================

uint FUN_1406ed670(longlong param_1)

{
  uint *puVar1;
  byte bVar2;
  uint *puVar3;
  uint uVar4;
  undefined4 local_res8;
  
  uVar4 = 0;
  local_res8 = 0;
  if (*(int *)(param_1 + 0x420) < 3) {
    return 0;
  }
  puVar1 = (uint *)(param_1 + 8);
  if (*(int *)(param_1 + 0x430) != 0) {
    return *puVar1;
  }
  if (*(int *)(param_1 + 0x42c) != 0) {
    return (uint)*(byte *)puVar1;
  }
  puVar3 = puVar1;
  do {
    if ((longlong)puVar3 - (longlong)puVar1 < 0x400) {
      bVar2 = (byte)*puVar3;
    }
    else {
      bVar2 = *(byte *)((longlong)((int)(uVar4 - 0x400) % 0xff8) +
                       *(longlong *)
                        (*(longlong *)(param_1 + 0x408) +
                        (longlong)((int)(uVar4 - 0x400) / 0xff8) * 8));
    }
    *(byte *)((longlong)puVar3 + ((longlong)&local_res8 - (longlong)puVar1)) = bVar2;
    if ((longlong)((longlong)puVar3 + (1 - (longlong)puVar1)) < 0x400) {
      bVar2 = *(byte *)((longlong)puVar3 + 1);
    }
    else {
      bVar2 = *(byte *)((longlong)((int)(uVar4 - 0x3ff) % 0xff8) +
                       *(longlong *)
                        (*(longlong *)(param_1 + 0x408) +
                        (longlong)((int)(uVar4 - 0x3ff) / 0xff8) * 8));
    }
    *(byte *)((longlong)puVar3 + (longlong)&local_res8 + (1 - (longlong)puVar1)) = bVar2;
    uVar4 = uVar4 + 2;
    puVar3 = (uint *)((longlong)puVar3 + 2);
  } while (uVar4 < 2);
  return local_res8 & 0xffff;
}


