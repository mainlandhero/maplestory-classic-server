
//===========================================================
// FUN_1403035a0 @ 1403035a0   (598 bytes)
//===========================================================

void FUN_1403035a0(longlong param_1,undefined8 param_2)

{
  byte *pbVar1;
  ushort uVar2;
  undefined8 *puVar3;
  byte *pbVar4;
  undefined1 uVar5;
  byte bVar6;
  char cVar7;
  undefined4 uVar8;
  undefined8 *puVar9;
  int iVar10;
  byte bVar11;
  byte *pbVar12;
  uint uVar13;
  undefined4 local_res8;
  
  local_res8 = FUN_1406e8c20(param_2);
  iVar10 = *(int *)(param_1 + 0x20) + 1;
  *(int *)(param_1 + 0x20) = iVar10;
  if (iVar10 == (iVar10 / 0x6f) * 0x6f) {
    puVar3 = *(undefined8 **)(param_1 + 0x28);
    puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 0x28) = puVar9;
    *puVar9 = *puVar3;
    *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar5 = FUN_142f04924();
  uVar13 = 0;
  *(undefined1 *)(*(longlong *)(param_1 + 0x28) + 4) = uVar5;
  pbVar4 = *(byte **)(param_1 + 0x28);
  bVar11 = pbVar4[4];
  pbVar4[8] = 0x65;
  pbVar4[9] = 0x9a;
  pbVar12 = pbVar4;
  do {
    pbVar1 = pbVar12 + 4;
    if (bVar11 == 0) {
      bVar11 = 0x2a;
    }
    bVar6 = pbVar1[(longlong)(&stack0x00000004 + -(longlong)pbVar4)];
    *pbVar12 = bVar11 ^ bVar6;
    bVar11 = bVar11 + (bVar11 ^ bVar6) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x28) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x28) + 8) = (uVar2 >> 0xd) + (ushort)bVar11 | uVar2 << 3;
    bVar6 = 0x2a;
    if (bVar11 != 0) {
      bVar6 = bVar11;
    }
    bVar11 = pbVar12[(longlong)&local_res8 + (1 - (longlong)pbVar4)];
    pbVar12[1] = bVar6 ^ bVar11;
    bVar6 = (bVar6 ^ bVar11) + bVar6 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x28) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x28) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar11 = 0x2a;
    if (bVar6 != 0) {
      bVar11 = bVar6;
    }
    bVar6 = pbVar1[(longlong)(&stack0x00000006 + -(longlong)pbVar4)];
    pbVar12[2] = bVar11 ^ bVar6;
    bVar6 = (bVar11 ^ bVar6) + bVar11 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x28) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x28) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar11 = 0x2a;
    if (bVar6 != 0) {
      bVar11 = bVar6;
    }
    uVar13 = uVar13 + 4;
    bVar6 = pbVar1[(longlong)(&stack0x00000007 + -(longlong)pbVar4)];
    pbVar12[3] = bVar11 ^ bVar6;
    bVar11 = (bVar11 ^ bVar6) + bVar11 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x28) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x28) + 8) = (uVar2 >> 0xd) + (ushort)bVar11 | uVar2 << 3;
    pbVar12 = pbVar1;
  } while (uVar13 < 4);
  cVar7 = FUN_1406e8ae0(param_2);
  if (cVar7 == '\0') {
    *(undefined8 *)(param_1 + 0x38) = 0;
  }
  else {
    FUN_1406e9170(param_2,param_1 + 0x38,8);
  }
  FUN_1406e9170(param_2,param_1 + 0x40,8);
  uVar8 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x48) = uVar8;
  cVar7 = FUN_1406e8ae0(param_2);
  *(bool *)(param_1 + 0x4c) = cVar7 != '\0';
  return;
}



//===========================================================
// FUN_140303b40 @ 140303b40   (1453 bytes)
//===========================================================

void FUN_140303b40(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  byte bVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  uint uVar5;
  undefined4 uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  
  FUN_140303800();
  uVar5 = FUN_1406e8c20(param_2);
  if ((uVar5 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x98) = bVar2;
  *(byte *)(param_1 + 0x99) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x9c) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 & 2) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0xa0) = bVar2;
  uVar12 = 0;
  uVar10 = 0;
  *(byte *)(param_1 + 0xa1) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0xa4) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  uVar4 = 0;
  uVar3 = uVar4;
  if ((uVar5 & 4) != 0) {
    uVar3 = FUN_1406e8b80(param_2);
  }
  uVar6 = FUN_1402f7010(uVar3,param_1 + 0xa8);
  *(undefined4 *)(param_1 + 0xac) = uVar6;
  if ((uVar5 & 8) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0xb0) = bVar2;
  *(byte *)(param_1 + 0xb1) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0xb4) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 & 0x10) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0xb8) = bVar2;
  *(byte *)(param_1 + 0xb9) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0xbc) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  uVar11 = uVar12;
  if ((uVar5 & 0x20) != 0) {
    uVar11 = FUN_1406e8f10(param_2);
  }
  uVar6 = FUN_1402f7170(uVar11,param_1 + 0xc0);
  *(undefined4 *)(param_1 + 0xd0) = uVar6;
  uVar9 = 0xffffffff;
  if ((uVar5 & 0x40) == 0) {
    uVar7 = 0xffffffff;
  }
  else {
    uVar7 = FUN_1406e8c20(param_2);
  }
  uVar8 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0xd8) = uVar8;
  uVar7 = (uVar8 ^ uVar7) >> 5 | (uVar8 ^ uVar7) << 0x1b;
  *(uint *)(param_1 + 0xdc) = uVar7;
  *(uint *)(param_1 + 0xe0) = ((uVar8 ^ 0xbaadf00d) >> 5 | (uVar8 ^ 0xbaadf00d) << 0x1b) + uVar7;
  if ((char)uVar5 < '\0') {
    uVar7 = FUN_1406e8c20(param_2);
  }
  else {
    uVar7 = 0;
  }
  uVar8 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0xe4) = uVar8;
  uVar7 = (uVar8 ^ uVar7) >> 5 | (uVar8 ^ uVar7) << 0x1b;
  *(uint *)(param_1 + 0xe8) = uVar7;
  *(uint *)(param_1 + 0xec) = ((uVar8 ^ 0xbaadf00d) >> 5 | (uVar8 ^ 0xbaadf00d) << 0x1b) + uVar7;
  if ((uVar5 >> 8 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0xf0) = bVar2;
  *(byte *)(param_1 + 0xf1) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0xf4) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 9 & 1) != 0) {
    uVar4 = FUN_1406e8b80(param_2);
  }
  uVar6 = FUN_1402f7010(uVar4,param_1 + 0xf8);
  *(undefined4 *)(param_1 + 0xfc) = uVar6;
  if ((uVar5 >> 10 & 1) != 0) {
    uVar9 = FUN_1406e8c20(param_2);
  }
  uVar7 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0x100) = uVar7;
  uVar9 = (uVar7 ^ uVar9) >> 5 | (uVar7 ^ uVar9) << 0x1b;
  *(uint *)(param_1 + 0x104) = uVar9;
  *(uint *)(param_1 + 0x108) = ((uVar7 ^ 0xbaadf00d) >> 5 | (uVar7 ^ 0xbaadf00d) << 0x1b) + uVar9;
  if ((uVar5 >> 0xb & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x10c) = bVar2;
  *(byte *)(param_1 + 0x10d) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x110) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0xc & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x114) = bVar2;
  *(byte *)(param_1 + 0x115) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x118) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0xd & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x11c) = bVar2;
  *(byte *)(param_1 + 0x11d) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x120) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0xe & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x124) = bVar2;
  *(byte *)(param_1 + 0x125) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x128) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0xf & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 300) = bVar2;
  *(byte *)(param_1 + 0x12d) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x130) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0x10 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x134) = bVar2;
  *(byte *)(param_1 + 0x135) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x138) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0x11 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x13c) = bVar2;
  *(byte *)(param_1 + 0x13d) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x140) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0x12 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x144) = bVar2;
  *(byte *)(param_1 + 0x145) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x148) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  if ((uVar5 >> 0x13 & 1) != 0) {
    uVar12 = FUN_1406e8f10(param_2);
  }
  uVar6 = FUN_1402f7170(uVar12,param_1 + 0x14c);
  *(undefined4 *)(param_1 + 0x15c) = uVar6;
  if ((uVar5 >> 0x14 & 1) != 0) {
    uVar10 = FUN_1406e8c20(param_2);
  }
  uVar5 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0x164) = uVar5;
  uVar10 = (uVar5 ^ uVar10) >> 5 | (uVar5 ^ uVar10) << 0x1b;
  *(uint *)(param_1 + 0x168) = uVar10;
  *(uint *)(param_1 + 0x16c) = ((uVar5 ^ 0xbaadf00d) >> 5 | (uVar5 ^ 0xbaadf00d) << 0x1b) + uVar10;
  return;
}



//===========================================================
// FUN_1402cce00 @ 1402cce00   (132 bytes)
//===========================================================

void FUN_1402cce00(undefined8 param_1,longlong param_2)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  longlong lVar3;
  undefined8 local_res10;
  undefined8 local_res18;
  
  FUN_1406e9170(param_1,&local_res10,8);
  *(undefined8 *)(param_2 + 0x20) = local_res10;
  FUN_1406e9170(param_1,&local_res18,8);
  *(undefined8 *)(param_2 + 0x28) = local_res18;
  uVar1 = FUN_1406e8c20(param_1);
  *(undefined4 *)(param_2 + 0x30) = uVar1;
  lVar3 = 3;
  puVar2 = (undefined4 *)(param_2 + 0x34);
  do {
    uVar1 = FUN_1406e8c20(param_1);
    *puVar2 = uVar1;
    puVar2 = puVar2 + 1;
    lVar3 = lVar3 + -1;
  } while (lVar3 != 0);
  return;
}



//===========================================================
// FUN_1402cd090 @ 1402cd090   (63 bytes)
//===========================================================

void FUN_1402cd090(undefined8 param_1,longlong param_2)

{
  undefined4 uVar1;
  undefined8 local_res10 [3];
  
  FUN_1406e9170(param_1,local_res10,8);
  *(undefined8 *)(param_2 + 0x20) = local_res10[0];
  uVar1 = FUN_1406e8c20(param_1);
  *(undefined4 *)(param_2 + 0x28) = uVar1;
  return;
}



//===========================================================
// FUN_140303800 @ 140303800   (621 bytes)
//===========================================================

void FUN_140303800(longlong param_1,undefined8 param_2)

{
  undefined2 uVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  uint uVar4;
  undefined4 uVar5;
  
  uVar4 = FUN_1406e8c20(param_2);
  uVar3 = 0;
  uVar2 = 0;
  uVar1 = uVar2;
  if ((uVar4 & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1);
  *(undefined4 *)(param_1 + 4) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 2) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 8);
  *(undefined4 *)(param_1 + 0xc) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 4) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x10);
  *(undefined4 *)(param_1 + 0x14) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 8) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x18);
  *(undefined4 *)(param_1 + 0x1c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 0x10) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x20);
  *(undefined4 *)(param_1 + 0x24) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 0x20) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x28);
  *(undefined4 *)(param_1 + 0x2c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 0x40) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x30);
  *(undefined4 *)(param_1 + 0x34) = uVar5;
  uVar1 = uVar2;
  if ((char)uVar4 < '\0') {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x38);
  *(undefined4 *)(param_1 + 0x3c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 8 & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x40);
  *(undefined4 *)(param_1 + 0x44) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 9 & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x48);
  *(undefined4 *)(param_1 + 0x4c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 10 & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x50);
  *(undefined4 *)(param_1 + 0x54) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 0xb & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x58);
  *(undefined4 *)(param_1 + 0x5c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 0xc & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x60);
  *(undefined4 *)(param_1 + 100) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 0xd & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x68);
  *(undefined4 *)(param_1 + 0x6c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 0xe & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x70);
  *(undefined4 *)(param_1 + 0x74) = uVar5;
  if ((uVar4 >> 0xf & 1) != 0) {
    uVar2 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar2,param_1 + 0x78);
  *(undefined4 *)(param_1 + 0x7c) = uVar5;
  if ((uVar4 >> 0x10 & 1) != 0) {
    uVar3 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar3,param_1 + 0x80);
  *(undefined4 *)(param_1 + 0x84) = uVar5;
  return;
}



//===========================================================
// FUN_1402cb4f0 @ 1402cb4f0   (166 bytes)
//===========================================================

void FUN_1402cb4f0(uint *param_1,undefined8 param_2)

{
  ushort uVar1;
  uint uVar2;
  undefined4 uVar3;
  undefined8 *puVar4;
  undefined1 *puVar5;
  longlong local_res8;
  
  uVar1 = FUN_1406e8b80(param_2);
  *param_1 = (uint)uVar1;
  uVar2 = FUN_1406e8c20(param_2);
  param_1[2] = uVar2;
  uVar2 = FUN_1406e8c20(param_2);
  param_1[3] = uVar2;
  puVar4 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
  puVar5 = (undefined1 *)*puVar4;
  if (param_1 + 4 != (uint *)0x0) {
    if (puVar5 == (undefined1 *)0x0) {
      puVar5 = &DAT_1434b2af1;
    }
    (*DAT_143262858)(param_1 + 4,puVar5);
  }
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)((longlong)param_1 + 0x1d) = uVar3;
  FUN_1406e9170(param_2,(longlong)param_1 + 0x21,8);
  uVar2 = FUN_1406e8c20(param_2);
  param_1[1] = uVar2;
  return;
}



//===========================================================
// FUN_1402f70a0 @ 1402f70a0   (131 bytes)
//===========================================================

uint FUN_1402f70a0(undefined2 param_1,longlong param_2)

{
  byte bVar1;
  byte bVar2;
  longlong lVar3;
  byte *pbVar4;
  uint uVar5;
  undefined2 local_res8 [4];
  
  uVar5 = 0xbaadf00d;
  pbVar4 = (byte *)local_res8;
  lVar3 = 2;
  local_res8[0] = param_1;
  do {
    bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
    bVar1 = *pbVar4;
    pbVar4[param_2 - (longlong)local_res8] = bVar2;
    pbVar4[(param_2 + 2) - (longlong)local_res8] = bVar2 ^ bVar1;
    pbVar4 = pbVar4 + 1;
    uVar5 = ((uVar5 ^ bVar2) >> 5 | (uVar5 ^ bVar2) << 0x1b) + (uint)(bVar2 ^ bVar1);
    lVar3 = lVar3 + -1;
  } while (lVar3 != 0);
  return uVar5;
}



//===========================================================
// FUN_1402cc180 @ 1402cc180   (229 bytes)
//===========================================================

longlong FUN_1402cc180(longlong param_1,int param_2)

{
  longlong lVar1;
  
  if (param_2 == 1) {
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0x467);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f7da0(lVar1);
    }
  }
  else if (param_2 == 2) {
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0x7e);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f7cd0(lVar1);
    }
  }
  else {
    if (param_2 != 3) {
      *(undefined8 *)(param_1 + 8) = 0;
      return param_1;
    }
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0xce);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f8b40(lVar1);
    }
  }
  *(longlong *)(param_1 + 8) = lVar1;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 8) = *(longlong *)(lVar1 + 8) + 1;
    UNLOCK();
  }
  return param_1;
}


