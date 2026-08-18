
//===========================================================
// FUN_1402ee8d0 @ 1402ee8d0   (621 bytes)
//===========================================================

void FUN_1402ee8d0(longlong param_1,undefined8 param_2,longlong *param_3)

{
  undefined1 uVar1;
  byte bVar2;
  char cVar3;
  undefined4 uVar4;
  int iVar5;
  
  uVar1 = FUN_1406e8ae0(param_2);
  *(undefined1 *)(param_1 + 0x20) = uVar1;
  bVar2 = FUN_1406e8ae0(param_2);
  *(uint *)(param_1 + 0x21) = (uint)bVar2;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x25) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x29) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1bd) = uVar4;
  *(undefined8 *)(param_1 + 0x39) = 0;
  *(undefined8 *)(param_1 + 0x41) = 0;
  *(undefined8 *)(param_1 + 0x49) = 0;
  *(undefined8 *)(param_1 + 0x51) = 0;
  *(undefined8 *)(param_1 + 0x59) = 0;
  *(undefined8 *)(param_1 + 0x61) = 0;
  *(undefined8 *)(param_1 + 0x69) = 0;
  *(undefined8 *)(param_1 + 0x71) = 0;
  *(undefined8 *)(param_1 + 0x79) = 0;
  *(undefined8 *)(param_1 + 0x81) = 0;
  *(undefined8 *)(param_1 + 0x89) = 0;
  *(undefined8 *)(param_1 + 0x91) = 0;
  *(undefined8 *)(param_1 + 0x99) = 0;
  *(undefined8 *)(param_1 + 0xa1) = 0;
  *(undefined8 *)(param_1 + 0xa9) = 0;
  *(undefined8 *)(param_1 + 0xb1) = 0;
  *(undefined8 *)(param_1 + 0xb9) = 0;
  *(undefined8 *)(param_1 + 0xc1) = 0;
  *(undefined8 *)(param_1 + 0xc9) = 0;
  *(undefined8 *)(param_1 + 0xd1) = 0;
  *(undefined8 *)(param_1 + 0xd9) = 0;
  *(undefined8 *)(param_1 + 0xe1) = 0;
  *(undefined8 *)(param_1 + 0xe9) = 0;
  *(undefined8 *)(param_1 + 0xf1) = 0;
  *(undefined8 *)(param_1 + 0xf9) = 0;
  *(undefined8 *)(param_1 + 0x101) = 0;
  *(undefined8 *)(param_1 + 0x109) = 0;
  *(undefined8 *)(param_1 + 0x111) = 0;
  *(undefined8 *)(param_1 + 0x119) = 0;
  *(undefined8 *)(param_1 + 0x121) = 0;
  *(undefined8 *)(param_1 + 0x129) = 0;
  *(undefined8 *)(param_1 + 0x131) = 0;
  FUN_1406e8ae0(param_2);
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x39) = uVar4;
  bVar2 = FUN_1406e8ae0(param_2);
  while (bVar2 != 0xff) {
    uVar4 = FUN_1406e8c20(param_2);
    if (((byte)(bVar2 - 1) < 0x1f) && (iVar5 = FUN_140253980(uVar4,bVar2,2,1), iVar5 != 0)) {
      *(undefined4 *)(param_1 + 0x39 + (ulonglong)bVar2 * 4) = uVar4;
    }
    bVar2 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1406e8ae0(param_2);
  while (bVar2 != 0xff) {
    uVar4 = FUN_1406e8c20(param_2);
    if (((byte)(bVar2 - 1) < 0x1f) && (iVar5 = FUN_140253980(uVar4,bVar2,2,1), iVar5 != 0)) {
      *(undefined4 *)(param_1 + 0xb9 + (ulonglong)bVar2 * 4) = uVar4;
    }
    bVar2 = FUN_1406e8ae0(param_2);
  }
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x2d) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x31) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x35) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1c1) = uVar4;
  iVar5 = FUN_1406e8c20(param_2);
  if (iVar5 < 1) {
    iVar5 = 0;
  }
  else {
    iVar5 = iVar5 % 0x168;
  }
  *(int *)(param_1 + 0x1c5) = iVar5;
  cVar3 = FUN_1406e8ae0(param_2);
  *(bool *)(param_1 + 0x1c9) = cVar3 != '\0';
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1ca) = uVar4;
  FUN_1406e9170(param_2,param_1 + 0x1b9,4);
  FUN_1406e9170(param_2,param_1 + 0x139,0x80);
  uVar4 = thunk_FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1d2) = uVar4;
  FUN_1406e9170(param_2,param_1 + 0x1d6,0xd);
  if (*param_3 != 0) {
    FUN_14019f2c0(*param_3 + -0x10);
  }
  return;
}


