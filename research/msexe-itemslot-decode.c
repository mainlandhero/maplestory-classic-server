
//===========================================================
// FUN_140304100 @ 140304100   (828 bytes)
//===========================================================

void FUN_140304100(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  byte bVar2;
  char cVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  int iVar6;
  
  FUN_1403035a0();
  FUN_140303b40(param_1 + 0x62,param_2);
  FUN_1406e9170(param_2,param_1 + 0x55,0xd);
  *(undefined1 *)(param_1 + 0x61) = 0;
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x3af) = bVar2;
  *(byte *)(param_1 + 0x3b0) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x3b3) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x3b7) = bVar2;
  *(byte *)(param_1 + 0x3b8) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x3bb) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3bf);
  *(undefined4 *)(param_1 + 0x3c3) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3c7);
  *(undefined4 *)(param_1 + 0x3cb) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3cf);
  *(undefined4 *)(param_1 + 0x3d3) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3d7);
  *(undefined4 *)(param_1 + 0x3db) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 999);
  *(undefined4 *)(param_1 + 0x3eb) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3ef);
  *(undefined4 *)(param_1 + 0x3f3) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3df);
  *(undefined4 *)(param_1 + 0x3e3) = uVar5;
  if (*(longlong *)(param_1 + 0x38) == 0) {
    FUN_1406e9170(param_2,param_1 + 0x4d,8);
  }
  else {
    *(undefined8 *)(param_1 + 0x4d) = 0;
  }
  FUN_1402cce00(param_2,param_1 + 0x1d2);
  FUN_1402cd090(param_2,param_1 + 0x212);
  uVar5 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x23e) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f7010(uVar4,param_1 + 0x3f7);
  *(undefined4 *)(param_1 + 0x3fb) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f7010(uVar4,param_1 + 0x3ff);
  *(undefined4 *)(param_1 + 0x403) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x407);
  *(undefined4 *)(param_1 + 0x40b) = uVar5;
  iVar6 = FUN_14019a5d0(param_1 + 0x20);
  if (iVar6 / 10000 == 0xa6) {
    FUN_1402cb4f0(param_1 + 0x242,param_2);
  }
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x303) = bVar2;
  *(byte *)(param_1 + 0x304) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x307) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x30b) = bVar2;
  *(byte *)(param_1 + 0x30c) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x30f) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  FUN_140303800(param_1 + 0x26b,param_2);
  cVar3 = FUN_1406e8ae0(param_2);
  if (cVar3 == '\0') {
    *(undefined4 *)(param_1 + 0x3ab) = 0;
    FUN_1402fb7e0(param_1 + 0x313);
  }
  else {
    FUN_140303800(param_1 + 0x313,param_2);
  }
  return;
}



//===========================================================
// FUN_140304450 @ 140304450   (238 bytes)
//===========================================================

void FUN_140304450(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  byte bVar2;
  undefined2 uVar3;
  undefined4 uVar4;
  int iVar5;
  
  FUN_1403035a0();
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f70a0(uVar3,param_1 + 0x4d);
  *(undefined4 *)(param_1 + 0x51) = uVar4;
  FUN_1406e9170(param_2,param_1 + 0x6d,0xd);
  *(undefined1 *)(param_1 + 0x79) = 0;
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f7010(uVar3,param_1 + 0x55);
  *(undefined4 *)(param_1 + 0x59) = uVar4;
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x5d) = bVar2;
  *(byte *)(param_1 + 0x5e) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x61) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  iVar5 = FUN_14019a5d0(param_1 + 0x20);
  if ((iVar5 - 0x1f95f0U < 10000) || (iVar5 - 0x238d90U < 10000)) {
    FUN_1406e9170(param_2,param_1 + 0x65,8);
  }
  else {
    *(undefined8 *)(param_1 + 0x65) = 0;
  }
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x7a) = uVar4;
  return;
}



//===========================================================
// FUN_140303530 @ 140303530   (101 bytes)
//===========================================================

longlong FUN_140303530(longlong param_1,undefined8 param_2)

{
  undefined1 uVar1;
  undefined1 local_18 [8];
  longlong *local_10;
  
  uVar1 = FUN_1406e8ae0(param_2);
  FUN_1402cc180(local_18,uVar1);
  if (local_10 == (longlong *)0x0) {
    *(undefined8 *)(param_1 + 8) = 0;
    return param_1;
  }
  (**(code **)(*local_10 + 0x358))(local_10,param_2);
  *(longlong **)(param_1 + 8) = local_10;
  return param_1;
}



//===========================================================
// FUN_1402fbb30 @ 1402fbb30   (8 bytes)
//===========================================================

longlong FUN_1402fbb30(longlong param_1)

{
  return param_1 + 0x242;
}


