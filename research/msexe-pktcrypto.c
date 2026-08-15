
//===========================================================
// FUN_1406e9910 @ 1406e9910   (188 bytes)
//===========================================================

undefined4 FUN_1406e9910(longlong param_1,undefined4 param_2,int param_3)

{
  char *pcVar1;
  longlong lVar2;
  undefined4 local_res10 [6];
  
  if (*(uint *)(param_1 + 0x20) < 2) {
    return 0;
  }
  lVar2 = *(longlong *)(param_1 + 0x10);
  local_res10[0] = param_2;
  if (lVar2 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar2 = *(longlong *)(param_1 + 0x10);
    if (lVar2 != 0) goto LAB_1406e9955;
  }
  else {
LAB_1406e9955:
    if (*(int *)(lVar2 + -8) != 0) goto LAB_1406e9973;
  }
  FUN_142e54290(0xbc,0,0);
  lVar2 = *(longlong *)(param_1 + 0x10);
LAB_1406e9973:
  pcVar1 = (char *)(lVar2 + 4);
  if (param_3 != 1) {
    if (param_3 == 2) {
      lVar2 = 2;
      do {
        *pcVar1 = *pcVar1 - (char)local_res10[0];
        pcVar1 = pcVar1 + 1;
        lVar2 = lVar2 + -1;
      } while (lVar2 != 0);
    }
    return 1;
  }
  FUN_140c75880(pcVar1,pcVar1,2,local_res10,1);
  return 1;
}



//===========================================================
// FUN_1406e99e0 @ 1406e99e0   (313 bytes)
//===========================================================

undefined8 FUN_1406e99e0(longlong param_1,undefined4 param_2,int param_3)

{
  uint uVar1;
  uint uVar2;
  longlong lVar3;
  uint uVar4;
  ulonglong uVar5;
  undefined4 local_res10 [2];
  
  uVar4 = *(uint *)(param_1 + 0x20);
  uVar2 = 0x5b0;
  if (uVar4 < 0x5b0) {
    uVar2 = uVar4;
  }
  uVar1 = uVar2 - 4;
  if (uVar4 < 0xff01) {
    uVar1 = uVar2;
  }
  uVar5 = (ulonglong)uVar1;
  if (uVar4 == 0) {
    return 0;
  }
  lVar3 = *(longlong *)(param_1 + 0x10);
  local_res10[0] = param_2;
  if (lVar3 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar3 = *(longlong *)(param_1 + 0x10);
    if (lVar3 != 0) goto LAB_1406e9a4c;
  }
  else {
LAB_1406e9a4c:
    if (*(int *)(lVar3 + -8) != 0) goto LAB_1406e9a65;
  }
  FUN_142e54290(0xbc,0,0);
  lVar3 = *(longlong *)(param_1 + 0x10);
LAB_1406e9a65:
  lVar3 = lVar3 + 4;
  if (param_3 == 1) {
    FUN_140c75880(lVar3,lVar3,uVar5,local_res10,1);
  }
  else if (param_3 == 2) {
    FUN_1406ef9f0(lVar3,lVar3,uVar5,local_res10,1);
  }
  lVar3 = lVar3 + uVar5;
  for (uVar4 = uVar4 - uVar1; uVar4 != 0; uVar4 = uVar4 - uVar2) {
    uVar2 = 0x5b4;
    if (uVar4 < 0x5b4) {
      uVar2 = uVar4;
    }
    if (param_3 == 1) {
      FUN_140c75880(lVar3,lVar3,uVar2,local_res10,1);
    }
    else if (param_3 == 2) {
      FUN_1406ef9f0(lVar3,lVar3,uVar2,local_res10,1);
    }
    lVar3 = lVar3 + (ulonglong)uVar2;
  }
  return 1;
}



//===========================================================
// FUN_1406e9510 @ 1406e9510   (12 bytes)
//===========================================================

void FUN_1406e9510(longlong param_1)

{
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0x18) = 0;
  *(undefined4 *)(param_1 + 0x24) = 0;
  return;
}


