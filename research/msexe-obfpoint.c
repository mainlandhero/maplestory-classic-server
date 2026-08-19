
//===========================================================
// FUN_1409d3c60 @ 1409d3c60   (980 bytes)
//===========================================================

int * FUN_1409d3c60(int *param_1,longlong param_2)

{
  byte *pbVar1;
  ushort uVar2;
  undefined8 *puVar3;
  byte *pbVar4;
  undefined1 uVar5;
  byte bVar6;
  undefined8 *puVar7;
  int iVar8;
  byte bVar9;
  byte *pbVar10;
  uint uVar11;
  undefined4 local_res8;
  
  local_res8 = FUN_1401b0340(param_2 + 0x18);
  iVar8 = param_1[6] + 1;
  param_1[6] = iVar8;
  if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
    puVar3 = *(undefined8 **)(param_1 + 8);
    puVar7 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 8) = puVar7;
    *puVar7 = *puVar3;
    *(undefined4 *)(puVar7 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar5 = FUN_142f04924();
  uVar11 = 0;
  *(undefined1 *)(*(longlong *)(param_1 + 8) + 4) = uVar5;
  pbVar4 = *(byte **)(param_1 + 8);
  bVar9 = pbVar4[4];
  pbVar4[8] = 0x65;
  pbVar4[9] = 0x9a;
  pbVar10 = pbVar4;
  do {
    pbVar1 = pbVar10 + 4;
    if (bVar9 == 0) {
      bVar9 = 0x2a;
    }
    bVar6 = pbVar1[(longlong)(&stack0x00000004 + -(longlong)pbVar4)];
    *pbVar10 = bVar9 ^ bVar6;
    bVar9 = bVar9 + (bVar9 ^ bVar6) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 8) + 8);
    *(ushort *)(*(longlong *)(param_1 + 8) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar6 = 0x2a;
    if (bVar9 != 0) {
      bVar6 = bVar9;
    }
    bVar9 = pbVar1[(longlong)(&stack0x00000005 + -(longlong)pbVar4)];
    pbVar10[1] = bVar6 ^ bVar9;
    bVar6 = (bVar6 ^ bVar9) + bVar6 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 8) + 8);
    *(ushort *)(*(longlong *)(param_1 + 8) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar6 != 0) {
      bVar9 = bVar6;
    }
    bVar6 = pbVar10[(longlong)&local_res8 + (2 - (longlong)pbVar4)];
    pbVar10[2] = bVar9 ^ bVar6;
    bVar6 = (bVar9 ^ bVar6) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 8) + 8);
    *(ushort *)(*(longlong *)(param_1 + 8) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar6 != 0) {
      bVar9 = bVar6;
    }
    uVar11 = uVar11 + 4;
    bVar6 = pbVar1[(longlong)(&stack0x00000007 + -(longlong)pbVar4)];
    pbVar10[3] = bVar9 ^ bVar6;
    bVar9 = (bVar9 ^ bVar6) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 8) + 8);
    *(ushort *)(*(longlong *)(param_1 + 8) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    pbVar10 = pbVar1;
  } while (uVar11 < 4);
  local_res8 = FUN_1401b0340(param_2);
  uVar11 = 0;
  iVar8 = *param_1 + 1;
  *param_1 = iVar8;
  if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
    puVar3 = *(undefined8 **)(param_1 + 2);
    puVar7 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 2) = puVar7;
    *puVar7 = *puVar3;
    *(undefined4 *)(puVar7 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar5 = FUN_142f04924();
  *(undefined1 *)(*(longlong *)(param_1 + 2) + 4) = uVar5;
  pbVar4 = *(byte **)(param_1 + 2);
  bVar9 = pbVar4[4];
  pbVar4[8] = 0x65;
  pbVar4[9] = 0x9a;
  pbVar10 = pbVar4;
  do {
    pbVar1 = pbVar10 + 4;
    if (bVar9 == 0) {
      bVar9 = 0x2a;
    }
    bVar6 = pbVar1[(longlong)(&stack0x00000004 + -(longlong)pbVar4)];
    *pbVar10 = bVar9 ^ bVar6;
    bVar9 = bVar9 + (bVar9 ^ bVar6) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar6 = 0x2a;
    if (bVar9 != 0) {
      bVar6 = bVar9;
    }
    bVar9 = pbVar1[(longlong)(&stack0x00000005 + -(longlong)pbVar4)];
    pbVar10[1] = bVar6 ^ bVar9;
    bVar6 = (bVar6 ^ bVar9) + bVar6 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar6 != 0) {
      bVar9 = bVar6;
    }
    bVar6 = pbVar10[(longlong)&local_res8 + (2 - (longlong)pbVar4)];
    pbVar10[2] = bVar9 ^ bVar6;
    bVar6 = (bVar9 ^ bVar6) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar6 != 0) {
      bVar9 = bVar6;
    }
    uVar11 = uVar11 + 4;
    bVar6 = pbVar1[(longlong)(&stack0x00000007 + -(longlong)pbVar4)];
    pbVar10[3] = bVar9 ^ bVar6;
    bVar9 = (bVar9 ^ bVar6) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    pbVar10 = pbVar1;
  } while (uVar11 < 4);
  return param_1;
}



//===========================================================
// FUN_14019a5d0 @ 14019a5d0   (1057 bytes)
//===========================================================

ulonglong FUN_14019a5d0(int *param_1)

{
  uint *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  byte bVar10;
  byte bVar11;
  int iVar12;
  uint uVar13;
  byte *pbVar14;
  longlong lVar15;
  ushort uVar16;
  int iVar17;
  ushort uVar18;
  byte *pbVar19;
  uint uVar20;
  ulonglong uVar21;
  undefined1 local_res8 [8];
  undefined1 local_res10 [8];
  byte local_res18 [8];
  ushort local_res20 [4];
  undefined4 local_78;
  undefined4 local_70;
  undefined4 local_6c;
  ulonglong local_68;
  undefined8 local_60;
  longlong local_58 [3];
  
  puVar1 = *(uint **)(param_1 + 2);
  local_70 = *puVar1;
  uVar20 = 0;
  uVar13 = 0;
  local_res18[0] = (byte)puVar1[1];
  local_res20[0] = 0x9a65;
  pbVar19 = (byte *)&local_70;
  pbVar14 = (byte *)((longlong)puVar1 + 2);
  do {
    if (local_res18[0] == 0) {
      local_res18[0] = 0x2a;
    }
    bVar10 = pbVar19[(longlong)puVar1 - (longlong)&local_70];
    *pbVar19 = bVar10 ^ local_res18[0];
    bVar10 = bVar10 + local_res18[0] + 0x2a;
    uVar16 = (local_res20[0] >> 0xd) + (ushort)bVar10;
    uVar18 = local_res20[0] << 3;
    if (bVar10 == 0) {
      bVar10 = 0x2a;
    }
    bVar11 = pbVar14[-1];
    pbVar19[1] = bVar11 ^ bVar10;
    bVar11 = bVar11 + bVar10 + 0x2a;
    uVar6 = (ushort)bVar11;
    if (bVar11 == 0) {
      bVar11 = 0x2a;
    }
    local_res18[0] = *pbVar14;
    pbVar19[2] = local_res18[0] ^ bVar11;
    local_res18[0] = local_res18[0] + bVar11 + 0x2a;
    uVar7 = (ushort)local_res18[0];
    if (local_res18[0] == 0) {
      local_res18[0] = 0x2a;
    }
    bVar10 = pbVar14[1];
    pbVar19[3] = bVar10 ^ local_res18[0];
    local_res18[0] = bVar10 + 0x2a + local_res18[0];
    local_res20[0] =
         ((uVar16 | uVar18 & 0x3ff) >> 7) + (ushort)local_res18[0] |
         (((uVar18 & 0x1fff) >> 10) + uVar7 |
         (((local_res20[0] & 0x1fff) >> 10) + uVar6 | (uVar16 | uVar18) << 3) << 3) << 3;
    uVar13 = uVar13 + 4;
    pbVar19 = pbVar19 + 4;
    pbVar14 = pbVar14 + 4;
  } while (uVar13 < 4);
  FUN_140c78f50(0x56,(longlong)param_1 + 0x21a3f060fc2daf);
  uVar13 = local_70;
  lVar2 = *(longlong *)(param_1 + 2);
  uVar21 = (ulonglong)(int)local_70;
  if (((local_res20[0] != *(ushort *)(lVar2 + 8)) || ((char)param_1[1] != *(char *)(lVar2 + 5))) ||
     ((char)param_1[4] != *(char *)(lVar2 + 6))) {
    local_res8[0] = (undefined1)param_1[4];
    local_res10[0] = (undefined1)param_1[1];
    local_6c = 1;
    local_68 = uVar21;
    local_60 = FUN_1418039d0(5);
    puVar8 = (undefined8 *)
             FUN_140197ac0(local_58,&local_60,&local_6c,&local_68,local_res18,local_res20,
                           (ushort *)(lVar2 + 8),local_res10,lVar2 + 5,local_res8,lVar2 + 6);
    FUN_141804970(&DAT_143271f04,0x61,5,*puVar8);
    if (local_58[0] != 0) {
      FUN_14019f2c0(local_58[0] + -0x10);
    }
  }
  iVar12 = *param_1;
  iVar17 = iVar12 + 1;
  *param_1 = iVar17;
  if (iVar17 == (iVar17 / 0x37) * 0x37) {
    local_78 = uVar13;
    iVar12 = iVar12 + 2;
    *param_1 = iVar12;
    if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
      puVar8 = *(undefined8 **)(param_1 + 2);
      puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 **)(param_1 + 2) = puVar9;
      *puVar9 = *puVar8;
      *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar8 + 1);
      thunk_FUN_140205820(puVar8,0xc);
    }
    uVar5 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(param_1 + 2) + 4) = uVar5;
    pbVar19 = *(byte **)(param_1 + 2);
    bVar10 = pbVar19[4];
    pbVar19[8] = 0x65;
    pbVar19[9] = 0x9a;
    lVar15 = (longlong)&local_78 - (longlong)pbVar19;
    lVar2 = 1 - (longlong)pbVar19;
    lVar3 = 2 - (longlong)pbVar19;
    lVar4 = 3 - (longlong)pbVar19;
    do {
      if (bVar10 == 0) {
        bVar10 = 0x2a;
      }
      bVar11 = pbVar19[lVar15];
      *pbVar19 = bVar10 ^ bVar11;
      bVar10 = bVar10 + (bVar10 ^ bVar11) + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar10 | uVar16 << 3;
      bVar11 = 0x2a;
      if (bVar10 != 0) {
        bVar11 = bVar10;
      }
      bVar10 = pbVar19[(longlong)&local_78 + lVar2];
      pbVar19[1] = bVar11 ^ bVar10;
      bVar11 = (bVar11 ^ bVar10) + bVar11 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar11 | uVar16 << 3;
      bVar10 = 0x2a;
      if (bVar11 != 0) {
        bVar10 = bVar11;
      }
      bVar11 = pbVar19[(longlong)&local_78 + lVar3];
      pbVar19[2] = bVar10 ^ bVar11;
      bVar11 = (bVar10 ^ bVar11) + bVar10 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar11 | uVar16 << 3;
      bVar10 = 0x2a;
      if (bVar11 != 0) {
        bVar10 = bVar11;
      }
      bVar11 = pbVar19[(longlong)&local_78 + lVar4];
      pbVar19[3] = bVar10 ^ bVar11;
      bVar10 = (bVar10 ^ bVar11) + bVar10 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar10 | uVar16 << 3;
      uVar20 = uVar20 + 4;
      pbVar19 = pbVar19 + 4;
    } while (uVar20 < 4);
    uVar21 = (ulonglong)local_70;
  }
  FUN_140c79130(0x67,param_1 + 0xc0a84bdc4a);
  return uVar21 & 0xffffffff;
}


