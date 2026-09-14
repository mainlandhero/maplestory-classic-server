
//===========================================================
// FUN_141eb9760 @ 141eb9760   (10998 bytes)
//===========================================================

/* WARNING: Heritage AFTER dead removal. Example location: s0x00000018 : 0x000141ebc11f */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

undefined8 FUN_141eb9760(longlong *param_1,longlong param_2,undefined4 param_3,undefined8 param_4)

{
  longlong *plVar1;
  ushort uVar2;
  longlong *plVar3;
  code *pcVar4;
  longlong *plVar5;
  IUnknown *pIVar6;
  undefined4 *puVar7;
  undefined1 uVar8;
  byte bVar9;
  char cVar10;
  short sVar11;
  undefined2 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined8 *puVar17;
  longlong *plVar18;
  longlong lVar19;
  longlong lVar20;
  undefined8 uVar21;
  undefined4 *puVar22;
  byte bVar23;
  int iVar24;
  byte *pbVar25;
  longlong *plVar26;
  uint uVar27;
  byte bVar28;
  uint uVar29;
  longlong *local_res8;
  undefined8 local_res10;
  undefined4 local_res18;
  undefined8 local_res20;
  undefined8 in_stack_fffffffffffffe28;
  undefined4 uVar31;
  uint *puVar30;
  undefined8 *puVar32;
  undefined8 *puVar33;
  longlong **pplVar34;
  longlong local_198;
  short local_190;
  undefined6 uStack_18e;
  longlong lStack_188;
  undefined8 local_180;
  short local_178;
  undefined6 uStack_176;
  longlong lStack_170;
  undefined8 local_168;
  short local_160;
  undefined6 uStack_15e;
  longlong lStack_158;
  undefined8 local_150;
  longlong *local_148;
  longlong *local_140;
  longlong *local_138;
  uint local_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined8 local_120;
  uint local_118;
  undefined4 uStack_114;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined1 local_f8 [16];
  undefined8 local_e8;
  longlong lStack_e0;
  undefined8 local_d8;
  undefined8 local_c8;
  longlong lStack_c0;
  undefined8 local_b8;
  undefined8 local_a8;
  longlong lStack_a0;
  undefined8 local_98;
  uint local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_78;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  
  uVar31 = (undefined4)((ulonglong)in_stack_fffffffffffffe28 >> 0x20);
  iVar24 = (int)param_1[0x27] + 1;
  *(int *)(param_1 + 0x27) = iVar24;
  local_res8 = param_1;
  local_res10 = param_2;
  local_res18 = param_3;
  local_res20 = param_4;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)param_1[0x28];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0x28] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(param_1[0x28] + 4) = uVar8;
  lVar20 = param_1[0x28];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar27 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  uVar29 = uVar27;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(param_1[0x28] + 8);
    *(ushort *)(param_1[0x28] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(param_1[0x28] + 8);
    *(ushort *)(param_1[0x28] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar28 ^ bVar9;
    bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0x28] + 8);
    *(ushort *)(param_1[0x28] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0x28] + 8);
    *(ushort *)(param_1[0x28] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  param_1[0x24] = local_res10;
  uVar13 = FUN_14276df20(local_res10);
  uVar21 = local_res20;
  *(undefined4 *)(param_1 + 0x25) = uVar13;
  if (((longlong *)param_1[0x24] != (longlong *)0x0) &&
     (iVar24 = (**(code **)(*(longlong *)param_1[0x24] + 0x58))(), iVar24 != 0)) {
    return 0;
  }
  uVar13 = FUN_1406e8c20(uVar21);
  local_res10 = CONCAT44(local_res10._4_4_,uVar13);
  plVar18 = (longlong *)FUN_1406e9050(uVar21,&local_148);
  plVar5 = param_1 + 0x26;
  if (*plVar5 != 0) {
    FUN_14019f2c0(*plVar5 + -0x10);
    *plVar5 = 0;
  }
  *plVar5 = *plVar18;
  *plVar18 = 0;
  if (local_148 != (longlong *)0x0) {
    FUN_14019f2c0(local_148 + -2);
  }
  plVar18 = (longlong *)FUN_14040dab0(plVar5,&local_198,&DAT_143295a74,&DAT_143275b98);
  if (*plVar5 != 0) {
    FUN_14019f2c0(*plVar5 + -0x10);
    *plVar5 = 0;
  }
  *plVar5 = *plVar18;
  *plVar18 = 0;
  if (local_198 != 0) {
    FUN_14019f2c0(local_198 + -0x10);
  }
  FUN_1406e9170(uVar21,param_1 + 0x2a,8);
  local_198 = FUN_141ebdad0(param_1);
  if (local_198 != 0) {
    sVar11 = FUN_1401ab420(local_198 + 0x62,*(undefined4 *)(local_198 + 0x66));
    local_res18 = (uint)sVar11;
    iVar24 = (int)param_1[0x2b] + 1;
    *(int *)(param_1 + 0x2b) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x2c];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x2c] = (longlong)puVar17;
      *puVar17 = *puVar33;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
      thunk_FUN_140205820(puVar33,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x2c] + 4) = uVar8;
    lVar20 = param_1[0x2c];
    bVar28 = *(byte *)(lVar20 + 4);
    *(undefined2 *)(lVar20 + 8) = 0x9a65;
    pbVar25 = (byte *)(lVar20 + 2);
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
      pbVar25[-2] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2c] + 8);
      *(ushort *)(param_1[0x2c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
      pbVar25[-1] = bVar9 ^ bVar28;
      bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2c] + 8);
      *(ushort *)(param_1[0x2c] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
      *pbVar25 = bVar28 ^ bVar9;
      bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2c] + 8);
      *(ushort *)(param_1[0x2c] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
      pbVar25[1] = bVar28 ^ bVar9;
      bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2c] + 8);
      *(ushort *)(param_1[0x2c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 4;
      pbVar25 = pbVar25 + 4;
    } while (uVar29 < 4);
    bVar28 = FUN_1401b0050(local_198 + 0x6a,*(undefined4 *)(local_198 + 0x6e));
    local_res18 = (uint)bVar28;
    iVar24 = (int)param_1[0x2e] + 1;
    *(int *)(param_1 + 0x2e) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x2f];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x2f] = (longlong)puVar17;
      *puVar17 = *puVar33;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
      thunk_FUN_140205820(puVar33,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x2f] + 4) = uVar8;
    lVar20 = param_1[0x2f];
    bVar28 = *(byte *)(lVar20 + 4);
    *(undefined2 *)(lVar20 + 8) = 0x9a65;
    pbVar25 = (byte *)(lVar20 + 2);
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
      pbVar25[-2] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2f] + 8);
      *(ushort *)(param_1[0x2f] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
      pbVar25[-1] = bVar9 ^ bVar28;
      bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2f] + 8);
      *(ushort *)(param_1[0x2f] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
      *pbVar25 = bVar28 ^ bVar9;
      bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2f] + 8);
      *(ushort *)(param_1[0x2f] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
      pbVar25[1] = bVar28 ^ bVar9;
      bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2f] + 8);
      *(ushort *)(param_1[0x2f] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 4;
      pbVar25 = pbVar25 + 4;
    } while (uVar29 < 4);
    sVar11 = FUN_1401ab420(local_198 + 0x72,*(undefined4 *)(local_198 + 0x76));
    local_res18 = (uint)sVar11;
    iVar24 = (int)param_1[0x31] + 1;
    *(int *)(param_1 + 0x31) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x32];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x32] = (longlong)puVar17;
      *puVar17 = *puVar33;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
      thunk_FUN_140205820(puVar33,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x32] + 4) = uVar8;
    lVar20 = param_1[0x32];
    bVar28 = *(byte *)(lVar20 + 4);
    *(undefined2 *)(lVar20 + 8) = 0x9a65;
    pbVar25 = (byte *)(lVar20 + 2);
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
      pbVar25[-2] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x32] + 8);
      *(ushort *)(param_1[0x32] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
      pbVar25[-1] = bVar9 ^ bVar28;
      bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x32] + 8);
      *(ushort *)(param_1[0x32] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
      *pbVar25 = bVar28 ^ bVar9;
      bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x32] + 8);
      *(ushort *)(param_1[0x32] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
      pbVar25[1] = bVar28 ^ bVar9;
      bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x32] + 8);
      *(ushort *)(param_1[0x32] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 4;
      pbVar25 = pbVar25 + 4;
    } while (uVar29 < 4);
    local_res18 = FUN_1401ba9d0(local_198 + 0xa6,*(undefined4 *)(local_198 + 0xae));
    iVar24 = (int)param_1[0x43] + 1;
    *(int *)(param_1 + 0x43) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x44];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x44] = (longlong)puVar17;
      *puVar17 = *puVar33;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
      thunk_FUN_140205820(puVar33,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x44] + 4) = uVar8;
    lVar20 = param_1[0x44];
    bVar28 = *(byte *)(lVar20 + 4);
    *(undefined2 *)(lVar20 + 8) = 0x9a65;
    pbVar25 = (byte *)(lVar20 + 2);
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
      pbVar25[-2] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x44] + 8);
      *(ushort *)(param_1[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
      pbVar25[-1] = bVar9 ^ bVar28;
      bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x44] + 8);
      *(ushort *)(param_1[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
      *pbVar25 = bVar28 ^ bVar9;
      bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x44] + 8);
      *(ushort *)(param_1[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
      pbVar25[1] = bVar28 ^ bVar9;
      bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x44] + 8);
      *(ushort *)(param_1[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 4;
      pbVar25 = pbVar25 + 4;
    } while (uVar29 < 4);
    uVar12 = FUN_1401ab420(local_198 + 0xb2,*(undefined4 *)(local_198 + 0xb6));
    uVar21 = local_res20;
    local_res18 = CONCAT22(local_res18._2_2_,uVar12);
    iVar24 = (int)param_1[0x46] + 1;
    *(int *)(param_1 + 0x46) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x47];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,8);
      param_1[0x47] = (longlong)puVar17;
      *puVar17 = *puVar33;
      thunk_FUN_140205820(puVar33,8);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x47] + 2) = uVar8;
    lVar20 = param_1[0x47];
    bVar28 = *(byte *)(lVar20 + 2);
    *(undefined2 *)(lVar20 + 6) = 0x9a65;
    pbVar25 = (byte *)&local_res18;
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = *pbVar25;
      pbVar25[lVar20 - (longlong)&local_res18] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x47] + 6);
      *(ushort *)(param_1[0x47] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[1];
      pbVar25[(lVar20 - (longlong)&local_res18) + 1] = bVar9 ^ bVar28;
      bVar28 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x47] + 6);
      *(ushort *)(param_1[0x47] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 2;
      pbVar25 = pbVar25 + 2;
    } while (uVar29 < 2);
  }
  FUN_141ec20f0(param_1);
  sVar11 = FUN_1406e8b80(uVar21);
  local_res18 = (uint)sVar11;
  iVar24 = (int)param_1[0xb3] + 1;
  *(int *)(param_1 + 0xb3) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)param_1[0xb4];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0xb4] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(param_1[0xb4] + 4) = uVar8;
  lVar20 = param_1[0xb4];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  pbVar25 = (byte *)(lVar20 + 2);
  uVar29 = uVar27;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb4] + 8);
    *(ushort *)(param_1[0xb4] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb4] + 8);
    *(ushort *)(param_1[0xb4] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar28 ^ bVar9;
    bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb4] + 8);
    *(ushort *)(param_1[0xb4] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb4] + 8);
    *(ushort *)(param_1[0xb4] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  sVar11 = FUN_1406e8b80(local_res20);
  local_res18 = (uint)sVar11;
  plVar5 = param_1 + 0xb0;
  iVar24 = (int)*plVar5 + 1;
  *(int *)plVar5 = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)param_1[0xb1];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0xb1] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  plVar18 = local_res8;
  *(undefined1 *)(param_1[0xb1] + 4) = uVar8;
  lVar20 = param_1[0xb1];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  pbVar25 = (byte *)(lVar20 + 2);
  uVar29 = uVar27;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb1] + 8);
    *(ushort *)(param_1[0xb1] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb1] + 8);
    *(ushort *)(param_1[0xb1] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar28 ^ bVar9;
    bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb1] + 8);
    *(ushort *)(param_1[0xb1] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb1] + 8);
    *(ushort *)(param_1[0xb1] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  FUN_1409d3c60(local_res8 + 0xaa,plVar5);
  bVar28 = FUN_1406e8ae0(local_res20);
  local_res18 = (uint)bVar28;
  iVar24 = (int)plVar18[0x5b] + 1;
  *(int *)(plVar18 + 0x5b) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar18[0x5c];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar18[0x5c] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar18[0x5c] + 4) = uVar8;
  lVar20 = plVar18[0x5c];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0x5c] + 8);
    *(ushort *)(plVar18[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar28 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(plVar18[0x5c] + 8);
    *(ushort *)(plVar18[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0x5c] + 8);
    *(ushort *)(plVar18[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0x5c] + 8);
    *(ushort *)(plVar18[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar21 = DAT_143ac18d8;
    uVar27 = uVar27 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar27 < 4);
  sVar11 = FUN_1406e8b80(local_res20);
  local_100 = FUN_142df6c50(uVar21,(int)sVar11);
  local_res18 = FUN_1406e8c20(local_res20);
  plVar5 = local_res8;
  iVar24 = (int)local_res8[0x43] + 1;
  *(int *)(local_res8 + 0x43) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)local_res8[0x44];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x44] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  uVar21 = local_res20;
  *(undefined1 *)(plVar5[0x44] + 4) = uVar8;
  lVar20 = plVar5[0x44];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x44] + 8);
    *(ushort *)(plVar5[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x44] + 8);
    *(ushort *)(plVar5[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x44] + 8);
    *(ushort *)(plVar5[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x44] + 8);
    *(ushort *)(plVar5[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  uVar13 = FUN_1406e8c20(local_res20);
  local_198 = CONCAT44(local_198._4_4_,uVar13);
  uVar12 = FUN_1406e8b80(uVar21);
  local_res18 = CONCAT22(local_res18._2_2_,uVar12);
  iVar24 = (int)plVar5[0x49] + 1;
  *(int *)(plVar5 + 0x49) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x4a];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,8);
    plVar5[0x4a] = (longlong)puVar17;
    *puVar17 = *puVar33;
    thunk_FUN_140205820(puVar33,8);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x4a] + 2) = uVar8;
  lVar20 = plVar5[0x4a];
  bVar28 = *(byte *)(lVar20 + 2);
  *(undefined2 *)(lVar20 + 6) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)&local_res18;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = *pbVar25;
    pbVar25[lVar20 - (longlong)&local_res18] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4a] + 6);
    *(ushort *)(plVar5[0x4a] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[1];
    pbVar25[(lVar20 - (longlong)&local_res18) + 1] = bVar9 ^ bVar28;
    bVar28 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4a] + 6);
    *(ushort *)(plVar5[0x4a] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 2;
    pbVar25 = pbVar25 + 2;
  } while (uVar29 < 2);
  uVar12 = FUN_1406e8b80(uVar21);
  local_res18 = CONCAT22(local_res18._2_2_,uVar12);
  iVar24 = (int)plVar5[0x46] + 1;
  *(int *)(plVar5 + 0x46) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x47];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,8);
    plVar5[0x47] = (longlong)puVar17;
    *puVar17 = *puVar33;
    thunk_FUN_140205820(puVar33,8);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x47] + 2) = uVar8;
  lVar20 = plVar5[0x47];
  bVar28 = *(byte *)(lVar20 + 2);
  *(undefined2 *)(lVar20 + 6) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)&local_res18;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = *pbVar25;
    pbVar25[lVar20 - (longlong)&local_res18] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x47] + 6);
    *(ushort *)(plVar5[0x47] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[1];
    pbVar25[(lVar20 - (longlong)&local_res18) + 1] = bVar9 ^ bVar28;
    bVar28 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x47] + 6);
    *(ushort *)(plVar5[0x47] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 2;
    pbVar25 = pbVar25 + 2;
  } while (uVar29 < 2);
  bVar28 = FUN_1406e8ae0(uVar21);
  local_res18 = (uint)bVar28;
  iVar24 = (int)plVar5[0x4f] + 1;
  *(int *)(plVar5 + 0x4f) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x50];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x50] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x50] + 4) = uVar8;
  lVar20 = plVar5[0x50];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x50] + 8);
    *(ushort *)(plVar5[0x50] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x50] + 8);
    *(ushort *)(plVar5[0x50] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x50] + 8);
    *(ushort *)(plVar5[0x50] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x50] + 8);
    *(ushort *)(plVar5[0x50] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  bVar28 = FUN_1406e8ae0(local_res20);
  local_res18 = (uint)bVar28;
  iVar24 = (int)plVar5[0x52] + 1;
  *(int *)(plVar5 + 0x52) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x53];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x53] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x53] + 4) = uVar8;
  lVar20 = plVar5[0x53];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x53] + 8);
    *(ushort *)(plVar5[0x53] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x53] + 8);
    *(ushort *)(plVar5[0x53] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x53] + 8);
    *(ushort *)(plVar5[0x53] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x53] + 8);
    *(ushort *)(plVar5[0x53] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  local_res18 = 0;
  iVar24 = (int)plVar5[0x4c] + 1;
  *(int *)(plVar5 + 0x4c) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x4d];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x4d] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x4d] + 4) = uVar8;
  lVar20 = plVar5[0x4d];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4d] + 8);
    *(ushort *)(plVar5[0x4d] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4d] + 8);
    *(ushort *)(plVar5[0x4d] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4d] + 8);
    *(ushort *)(plVar5[0x4d] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4d] + 8);
    *(ushort *)(plVar5[0x4d] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  local_res18 = 0;
  iVar24 = (int)plVar5[0x74] + 1;
  *(int *)(plVar5 + 0x74) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x75];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x75] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x75] + 4) = uVar8;
  lVar20 = plVar5[0x75];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x75] + 8);
    *(ushort *)(plVar5[0x75] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x75] + 8);
    *(ushort *)(plVar5[0x75] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x75] + 8);
    *(ushort *)(plVar5[0x75] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x75] + 8);
    *(ushort *)(plVar5[0x75] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  plVar18 = (longlong *)FUN_142b5a560();
  local_148 = plVar18;
  if (plVar18 == (longlong *)0x0) {
    iVar24 = -0x7fffbffe;
    plVar26 = (longlong *)0x0;
  }
  else {
    local_140 = (longlong *)0x0;
    iVar24 = (**(code **)plVar18[4])(plVar18 + 4,&DAT_143273488,&local_140);
    plVar26 = (longlong *)0x0;
    if (-1 < iVar24) {
      plVar26 = local_140;
    }
  }
  plVar1 = plVar5 + 0x5b;
  if (((iVar24 + 0x80000000U & 0x80000000) == 0) && (iVar24 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0();
  }
  plVar3 = (longlong *)plVar5[0x23];
  if (plVar3 != plVar26) {
    plVar5[0x23] = (longlong)plVar26;
    plVar26 = (longlong *)0x0;
    if (plVar3 != (longlong *)0x0) {
      (**(code **)(*plVar3 + 0x10))();
    }
  }
  if (plVar26 != (longlong *)0x0) {
    (**(code **)(*plVar26 + 0x10))(plVar26);
  }
  lVar19 = FUN_141ed3540(local_res10 & 0xffffffff);
  plVar5[0x21] = lVar19;
  lVar20 = lVar19;
  if ((int)local_198 != 0) {
    lVar19 = FUN_141ed3540();
    lVar20 = plVar5[0x21];
  }
  plVar5[0x22] = lVar19;
  if (lVar20 == 0) {
    return 0;
  }
  if (lVar19 == 0) {
    return 0;
  }
  FUN_142b599f0(plVar18,plVar5 + 1,*(undefined4 *)(lVar20 + 0x10));
  pcVar4 = *(code **)(*plVar18 + 0x118);
  uVar13 = FUN_14019a5d0(plVar1);
  uVar14 = FUN_14019a5d0(plVar5 + 0xaa);
  uVar15 = FUN_14019a5d0(plVar5 + 0xad);
  uVar16 = (**(code **)(*(longlong *)plVar5[0x24] + 0x50))();
  (*pcVar4)(local_148,uVar16,uVar15,uVar14,0,0,CONCAT44(uVar31,uVar13),local_100);
  local_res18 = FUN_1409c6d00(local_148);
  iVar24 = (int)*plVar1 + 1;
  *(int *)plVar1 = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x5c];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x5c] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  plVar18 = local_res8;
  *(undefined1 *)(plVar5[0x5c] + 4) = uVar8;
  lVar20 = plVar5[0x5c];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x5c] + 8);
    *(ushort *)(plVar5[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x5c] + 8);
    *(ushort *)(plVar5[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x5c] + 8);
    *(ushort *)(plVar5[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x5c] + 8);
    *(ushort *)(plVar5[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    pIVar6 = DAT_143add050;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  if (DAT_143add050 == (IUnknown *)0x0) goto LAB_141ebc21e;
  local_res8 = (longlong *)0x0;
  iVar24 = (**(code **)(*(longlong *)DAT_143add050 + 0x1d8))(DAT_143add050,0,&local_res8);
  if (iVar24 < 0) {
    _com_issue_errorex(iVar24,pIVar6,(_GUID *)&DAT_14327fcd0);
  }
  plVar5 = (longlong *)plVar18[0x77];
  plVar26 = local_res8;
  if (plVar5 != local_res8) {
    plVar18[0x77] = (longlong)local_res8;
    plVar26 = (longlong *)0x0;
    if (plVar5 != (longlong *)0x0) {
      (**(code **)(*plVar5 + 0x10))();
    }
  }
  if (plVar26 != (longlong *)0x0) {
    (**(code **)(*plVar26 + 0x10))(plVar26);
  }
  pIVar6 = DAT_143add050;
  if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_160);
  if (DAT_143a8b8d8 == 8) {
    if (local_160 == 8) {
      local_160 = 0;
      if (lStack_158 != 0) {
        (*DAT_143ad5990)(lStack_158 + -4);
      }
    }
    else {
      iVar24 = (*DAT_143262a18)(&local_160);
      if (iVar24 < 0) goto LAB_141ebc23f;
    }
    local_160 = 8;
    if (DAT_143a8b8e0 == 0) {
      lStack_158 = FUN_1401a5fa0(0,0);
    }
    else {
      lStack_158 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
  }
  else {
    if ((local_160 == 8) && (local_160 = 0, lStack_158 != 0)) {
      (*DAT_143ad5990)(lStack_158 + -4);
    }
    iVar24 = (*DAT_143262a28)(&local_160,&DAT_143a8b8d8);
    if (iVar24 < 0) {
LAB_141ebc23f:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar24);
    }
  }
  (*DAT_143262a20)(&local_178);
  if (DAT_143a8b8d8 == 8) {
    if (local_178 == 8) {
      local_178 = 0;
      if (lStack_170 != 0) {
        (*DAT_143ad5990)(lStack_170 + -4);
      }
    }
    else {
      iVar24 = (*DAT_143262a18)(&local_178);
      if (iVar24 < 0) goto LAB_141ebc229;
    }
    local_178 = 8;
    if (DAT_143a8b8e0 == 0) {
      uVar29 = 0;
    }
    else {
      uVar29 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_170 = FUN_1401a5fa0(DAT_143a8b8e0,uVar29);
  }
  else {
    if ((local_178 == 8) && (local_178 = 0, lStack_170 != 0)) {
      (*DAT_143ad5990)(lStack_170 + -4);
    }
    iVar24 = (*DAT_143262a28)(&local_178,&DAT_143a8b8d8);
    if (iVar24 < 0) {
LAB_141ebc229:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar24);
    }
  }
  (*DAT_143262a20)(&local_190);
  if (DAT_143a8b8d8 == 8) {
    if (local_190 == 8) {
      local_190 = 0;
      if (lStack_188 != 0) {
        (*DAT_143ad5990)(lStack_188 + -4);
      }
    }
    else {
      iVar24 = (*DAT_143262a18)(&local_190);
      if (iVar24 < 0) goto LAB_141ebc231;
    }
    local_190 = 8;
    if (DAT_143a8b8e0 == 0) {
      uVar29 = 0;
    }
    else {
      uVar29 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_188 = FUN_1401a5fa0(DAT_143a8b8e0,uVar29);
  }
  else {
    if ((local_190 == 8) && (local_190 = 0, lStack_188 != 0)) {
      (*DAT_143ad5990)(lStack_188 + -4);
    }
    iVar24 = (*DAT_143262a28)(&local_190,&DAT_143a8b8d8);
    if (iVar24 < 0) {
LAB_141ebc231:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar24);
    }
  }
  local_130 = CONCAT22(local_130._2_2_,3);
  uStack_128 = 0;
  local_res8 = (longlong *)0x0;
  local_e8 = CONCAT62(uStack_15e,local_160);
  lStack_e0 = lStack_158;
  local_d8 = local_150;
  local_c8 = CONCAT62(uStack_176,local_178);
  lStack_c0 = lStack_170;
  local_b8 = local_168;
  local_a8 = CONCAT62(uStack_18e,local_190);
  lStack_a0 = lStack_188;
  local_98 = local_180;
  local_88 = local_130;
  uStack_84 = uStack_12c;
  uStack_80 = 0;
  uStack_7c = uStack_124;
  local_78 = local_120;
  pplVar34 = &local_res8;
  puVar33 = &local_e8;
  puVar17 = &local_c8;
  puVar32 = &local_a8;
  puVar30 = &local_88;
  iVar24 = (**(code **)(*(longlong *)pIVar6 + 0x168))
                     (pIVar6,0,0,0,0,0,puVar30,puVar32,puVar17,puVar33,pplVar34);
  if (iVar24 < 0) {
    _com_issue_errorex(iVar24,pIVar6,(_GUID *)&DAT_14327fcd0);
  }
  plVar5 = (longlong *)plVar18[0x79];
  plVar26 = local_res8;
  if (plVar5 != local_res8) {
    plVar18[0x79] = (longlong)local_res8;
    plVar26 = (longlong *)0x0;
    if (plVar5 != (longlong *)0x0) {
      (**(code **)(*plVar5 + 0x10))();
    }
  }
  if (plVar26 != (longlong *)0x0) {
    (**(code **)(*plVar26 + 0x10))(plVar26);
  }
  if ((short)local_130 == 8) {
    local_130 = local_130 & 0xffff0000;
    if (CONCAT44(uStack_124,uStack_128) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_124,uStack_128) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_130);
  }
  if (local_190 == 8) {
    local_190 = 0;
    if (lStack_188 != 0) {
      (*DAT_143ad5990)(lStack_188 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_190);
  }
  if (local_178 == 8) {
    local_178 = 0;
    if (lStack_170 != 0) {
      (*DAT_143ad5990)(lStack_170 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_178);
  }
  if (local_160 == 8) {
    local_160 = 0;
    if (lStack_158 != 0) {
      (*DAT_143ad5990)(lStack_158 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_160);
  }
  pIVar6 = (IUnknown *)plVar18[0x79];
  if (pIVar6 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  uStack_110 = (longlong *)plVar18[0x77];
  local_118 = CONCAT22(local_118._2_2_,0xd);
  if (uStack_110 != (longlong *)0x0) {
    (**(code **)(*uStack_110 + 8))();
  }
  local_68 = local_118;
  uStack_64 = uStack_114;
  uStack_60 = (undefined4)uStack_110;
  uStack_5c = uStack_110._4_4_;
  local_58 = local_108;
  iVar24 = (**(code **)(*(longlong *)pIVar6 + 0x238))(pIVar6,&local_68);
  if (iVar24 < 0) {
    _com_issue_errorex(iVar24,pIVar6,(_GUID *)&DAT_14327fcb0);
  }
  if ((short)local_118 == 8) {
    local_118 = local_118 & 0xffff0000;
    if (uStack_110 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)uStack_110 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_118);
  }
  local_res8 = (longlong *)0x0;
  FUN_141ec2a10(plVar18,&local_res8);
  iVar24 = (**(code **)(*(longlong *)plVar18[0x24] + 0x50))();
  if ((iVar24 == 0) && (lVar20 = FUN_141892840(), lVar20 != 0)) {
    uVar21 = FUN_141892840();
    cVar10 = FUN_14183a640(uVar21);
    if (cVar10 == '\0') goto LAB_141ebbded;
  }
  else {
LAB_141ebbded:
    uVar31 = *(undefined4 *)(plVar18[0x22] + 0x18);
    uVar21 = FUN_141ece680(local_f8,local_148);
    local_138 = (longlong *)plVar18[0x79];
    if (local_138 != (longlong *)0x0) {
      (**(code **)(*local_138 + 8))();
    }
    FUN_141b054f0(plVar18,plVar18[0x26],&local_138,uVar21,0x3eb,uVar31,
                  (ulonglong)puVar30 & 0xffffffffffff0000,(ulonglong)puVar32 & 0xffffffffffffff00,
                  (ulonglong)puVar17 & 0xffffffffffff0000,(ulonglong)puVar33 & 0xffffffffffffff00,
                  (ulonglong)pplVar34 & 0xffffffff00000000,0,0xffffffff,0);
  }
  uVar8 = FUN_141ec7e90(plVar18);
  FUN_141ec7880(plVar18,uVar8,1);
  FUN_141ec87b0(plVar18);
  local_res18 = 0;
  iVar24 = (int)plVar18[0xb6] + 1;
  *(int *)(plVar18 + 0xb6) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar18[0xb7];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar18[0xb7] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar18[0xb7] + 4) = uVar8;
  lVar20 = plVar18[0xb7];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0xb7] + 8);
    *(ushort *)(plVar18[0xb7] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0xb7] + 8);
    *(ushort *)(plVar18[0xb7] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0xb7] + 8);
    *(ushort *)(plVar18[0xb7] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0xb7] + 8);
    *(ushort *)(plVar18[0xb7] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  local_res18 = local_res18 & 0xffffff00;
  iVar24 = (int)plVar18[0xb9] + 1;
  *(int *)(plVar18 + 0xb9) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar7 = (undefined4 *)plVar18[0xba];
    puVar22 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,6);
    plVar18[0xba] = (longlong)puVar22;
    *puVar22 = *puVar7;
    *(undefined2 *)(puVar22 + 1) = *(undefined2 *)(puVar7 + 1);
    thunk_FUN_140205820(puVar7,6);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar18[0xba] + 1) = uVar8;
  pbVar25 = (byte *)plVar18[0xba];
  pbVar25[4] = 0x65;
  pbVar25[5] = 0x9a;
  bVar28 = pbVar25[1];
  if (pbVar25[1] == 0) {
    bVar28 = 0x2a;
  }
  *pbVar25 = bVar28 ^ (byte)local_res18;
  uVar2 = *(ushort *)(plVar18[0xba] + 4);
  *(ushort *)(plVar18[0xba] + 4) =
       (uVar2 >> 0xd) + (ushort)(byte)(bVar28 + (bVar28 ^ (byte)local_res18) + 0x2a) | uVar2 << 3;
  FUN_141ecaf00(plVar18);
  lVar20 = DAT_143ac87a0;
  if (DAT_143ac87a0 != 0) {
    if (((longlong *)plVar18[0x24] == (longlong *)0x0) ||
       (iVar24 = (**(code **)(*(longlong *)plVar18[0x24] + 0x50))(), iVar24 == 0)) {
      iVar24 = *(int *)(lVar20 + 0x5c);
      lVar20 = 1;
    }
    else {
      iVar24 = *(int *)(lVar20 + 0x58);
      lVar20 = 0;
    }
    *(int *)(plVar18 + 0x78) = (iVar24 * 0xff) / 100;
    pIVar6 = (IUnknown *)plVar18[0x77];
    if (pIVar6 == (IUnknown *)0x0) {
LAB_141ebc21e:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar24 = (**(code **)(*(longlong *)pIVar6 + 0x68))
                       (pIVar6,*(undefined8 *)(DAT_143aa84a0 + 0x178 + lVar20 * 8));
    if (iVar24 < 0) {
      _com_issue_errorex(iVar24,pIVar6,(_GUID *)&DAT_14336fcd0);
    }
  }
  FUN_141ecf340(plVar18,plVar18 + 0xaa);
  return 1;
}



//===========================================================
// FUN_141ed3540 @ 141ed3540   (457 bytes)
//===========================================================

longlong FUN_141ed3540(int param_1)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined1 local_18 [8];
  longlong local_10;
  
  local_10 = 0;
  if (DAT_143a88bf8 != 0) {
    for (lVar3 = *(longlong *)
                  (DAT_143a88bf8 + ((ulonglong)(longlong)param_1 % (ulonglong)DAT_143a88c00) * 8);
        lVar3 != 0; lVar3 = *(longlong *)(lVar3 + 8)) {
      if (*(int *)(lVar3 + 0x10) == param_1) {
        if (local_18 == (undefined1 *)(lVar3 + 0x18)) {
          FUN_142e52d50(0x45c);
        }
        lVar2 = *(longlong *)(lVar3 + 0x20);
        if (lVar2 != 0) {
          if (0xfffff < *(ulonglong *)(lVar2 + -0x20)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar2 + -0x20) = *(longlong *)(lVar2 + -0x20) + 1;
          UNLOCK();
        }
        lVar3 = *(longlong *)(lVar3 + 0x20);
        local_10 = lVar3;
        if (lVar3 != 0) goto LAB_141ed3678;
        break;
      }
    }
  }
  lVar3 = local_10;
  if ((DAT_143ad3164 != '\0') && (FUN_141ed8f70(param_1), DAT_143a88bf8 != 0)) {
    for (lVar2 = *(longlong *)
                  (DAT_143a88bf8 + ((ulonglong)(longlong)param_1 % (ulonglong)DAT_143a88c00) * 8);
        lVar2 != 0; lVar2 = *(longlong *)(lVar2 + 8)) {
      if (*(int *)(lVar2 + 0x10) == param_1) {
        if (local_18 == (undefined1 *)(lVar2 + 0x18)) {
          FUN_142e52d50(0x45c,CONCAT71((int7)((ulonglong)local_18 >> 8),1));
        }
        lVar3 = *(longlong *)(lVar2 + 0x20);
        if (lVar3 != 0) {
          if (0xfffff < *(ulonglong *)(lVar3 + -0x20)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar3 + -0x20) = *(longlong *)(lVar3 + -0x20) + 1;
          UNLOCK();
        }
        lVar3 = *(longlong *)(lVar2 + 0x20);
        local_10 = lVar3;
        break;
      }
    }
  }
LAB_141ed3678:
  if (lVar3 == 0) {
    lVar2 = 0;
  }
  else {
    puVar4 = (undefined8 *)(lVar3 + -0x28);
    if (0xffffe < *(longlong *)(lVar3 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    lVar2 = local_10;
    LOCK();
    plVar1 = (longlong *)(lVar3 + -0x20);
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      if (*(longlong *)(local_10 + -0x10) != 0) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_10 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_10 + -0x10) + 4) != 0);
      }
      if (puVar4 != (undefined8 *)0x0) {
        (**(code **)*puVar4)(puVar4,1);
      }
    }
  }
  return lVar2;
}



//===========================================================
// FUN_141ebfa30 @ 141ebfa30   (9264 bytes)
//===========================================================

void FUN_141ebfa30(ulonglong param_1)

{
  ushort uVar1;
  code *pcVar2;
  bool bVar3;
  char cVar4;
  undefined1 uVar5;
  byte bVar6;
  undefined4 uVar7;
  int iVar8;
  int iVar9;
  long lVar10;
  uint uVar11;
  undefined4 uVar12;
  longlong lVar13;
  ulonglong uVar14;
  int *piVar15;
  undefined8 uVar16;
  longlong *plVar17;
  undefined1 *puVar18;
  undefined8 *puVar19;
  undefined8 *puVar20;
  byte *pbVar21;
  uint uVar22;
  longlong *plVar24;
  IUnknown *pIVar25;
  longlong lVar26;
  longlong lVar27;
  byte bVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  undefined8 local_res8;
  undefined4 local_res10;
  uint local_res18;
  undefined4 local_res20 [2];
  ulonglong uVar31;
  undefined8 uVar32;
  uint *puVar33;
  undefined4 uVar34;
  byte abStack_22a [2];
  undefined4 local_228;
  undefined8 local_220;
  undefined4 local_218;
  byte abStack_212 [2];
  undefined4 local_210;
  byte abStack_20a [8];
  byte abStack_202 [8];
  byte abStack_1fa [8];
  byte abStack_1f2 [8];
  byte abStack_1ea [8];
  byte abStack_1e2 [2];
  undefined4 local_1e0;
  int local_1d8;
  int local_1d4;
  undefined8 *local_1d0;
  int local_1c8;
  int local_1c0;
  int local_1bc;
  undefined8 *local_1b8;
  int local_1b0;
  int local_1a8;
  int local_1a4;
  undefined8 *local_1a0;
  int local_198;
  longlong *local_190;
  undefined4 local_188;
  undefined4 uStack_184;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 local_170;
  byte local_168 [8];
  longlong **local_160;
  int local_158 [2];
  longlong local_150;
  undefined1 *local_148;
  undefined4 local_140;
  undefined4 local_13c;
  longlong *local_138;
  undefined1 local_130 [4];
  undefined1 local_12c [4];
  uint local_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined8 local_118;
  undefined1 *local_108;
  undefined1 local_100 [8];
  undefined1 local_f8 [8];
  longlong local_f0;
  longlong local_e8;
  undefined8 *local_e0;
  ulonglong local_d8;
  longlong local_d0;
  ulonglong local_c8;
  undefined8 *local_c0;
  undefined1 local_b8 [8];
  undefined1 local_b0 [8];
  undefined1 local_a8 [8];
  undefined1 local_a0 [8];
  undefined1 local_98 [8];
  undefined1 local_90 [8];
  uint local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_78;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  ulonglong uVar23;
  
  uVar29 = 0;
  local_res18 = 0;
  if (*(longlong *)(param_1 + 0x120) == 0) {
    return;
  }
  local_res8 = param_1;
  uVar7 = FUN_1429e3ef0();
  local_190 = (longlong *)CONCAT44(local_190._4_4_,uVar7);
  if ((*(longlong **)(param_1 + 0x120) == (longlong *)0x0) ||
     (iVar8 = (**(code **)(**(longlong **)(param_1 + 0x120) + 0x58))(), iVar8 == 0)) {
    local_res10 = local_res10 & 0xffffff00;
    FUN_14159a920(*(undefined8 *)(param_1 + 0x40),0);
  }
  else {
    local_res10 = CONCAT31(local_res10._1_3_,1);
  }
  uVar30 = *(longlong *)(param_1 + 0x118) - 0x20;
  if (*(longlong *)(param_1 + 0x118) == 0) {
    uVar30 = uVar29;
  }
  local_220 = uVar30;
  iVar8 = FUN_14276e860(*(undefined8 *)(param_1 + 0x120));
  if ((*(longlong *)(param_1 + 0x650) != 0) && (cVar4 = FUN_142104aa0(), cVar4 != '\0')) {
    if (*(longlong *)(param_1 + 0x650) == 0) {
      return;
    }
    FUN_142104e20(*(longlong *)(param_1 + 0x650),uVar7);
    return;
  }
  iVar9 = FUN_1409c5080(uVar30);
  if ((((iVar9 != 0) && (iVar9 = FUN_1409bd0c0(uVar30), iVar9 != 0)) &&
      (lVar13 = thunk_FUN_1409dc380(uVar30), lVar13 != 0)) &&
     (cVar4 = FUN_142df6260(lVar13), cVar4 != '\0')) {
    FUN_14099ddf0(uVar30);
  }
  iVar9 = FUN_1409c5080(uVar30);
  if (iVar9 == 0) {
    uVar14 = FUN_1409c0cc0(uVar30);
    if (uVar14 != 0xffffffffffffffff) {
      if ((uVar14 >> 0x13 & 1) != 0) {
        FUN_141ebf340(param_1,1);
      }
      if ((uVar14 >> 0xb & 1) != 0) {
        FUN_141ebf340(param_1,2);
      }
    }
  }
  else {
    iVar9 = FUN_1401b0340(param_1 + 0x308);
    if ((iVar9 < 0) || (iVar8 != 0)) {
      FUN_1409c0b90(uVar30);
    }
  }
  uVar7 = FUN_1409c6d00(uVar30);
  FUN_141ebdc70(param_1,uVar7,0);
  local_res20[0] = FUN_141ebe350(param_1,local_100);
  FUN_141ec3380(param_1,local_res20);
  uVar14 = uVar29;
  if (*(longlong *)(param_1 + 0x120) != 0) {
    iVar8 = FUN_14276e860();
    if (iVar8 == 0) {
      iVar8 = FUN_141ebe350(param_1,0);
      uVar14 = 0;
      if (iVar8 != 8) goto LAB_141ebfc0d;
    }
    uVar14 = 1;
  }
LAB_141ebfc0d:
  FUN_141ec7880(param_1,uVar14,0);
  iVar8 = FUN_140fbf570(param_1 + 0x330,0x1e);
  if (iVar8 < 1) {
    iVar8 = FUN_1409d4840(param_1 + 0x278);
    lVar13 = param_1 + 0x368;
    if (iVar8 == 0) {
      lVar13 = param_1 + 0x360;
    }
    lVar13 = FUN_141ece7c0(lVar13,local_res20[0]);
    if (*(int *)(lVar13 + 4) != 0) {
      FUN_140cf16d0(param_1 + 0x328);
      iVar8 = FUN_1401b0340(param_1 + 0x308);
      lVar26 = *(longlong *)(param_1 + 0x328);
      if (iVar8 < 0) {
        if (lVar26 == 0) {
          *(undefined4 *)(param_1 + 800) = 0;
          iVar8 = FUN_1409c5080(uVar30);
          if (iVar8 != 0) {
            piVar15 = (int *)(param_1 + 0x2f0);
            iVar8 = FUN_14019a5d0(piVar15);
            if ((0 < iVar8) && (iVar8 = FUN_14019a5d0(piVar15), iVar8 < 3)) {
              uVar22 = FUN_1407386b0(&DAT_143ac1ab0);
              local_228 = (uVar22 & 1) + 1;
              iVar8 = *piVar15 + 1;
              *piVar15 = iVar8;
              if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
                puVar20 = *(undefined8 **)(param_1 + 0x2f8);
                puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                *(undefined8 **)(param_1 + 0x2f8) = puVar19;
                *puVar19 = *puVar20;
                *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
                thunk_FUN_140205820(puVar20,0xc);
              }
              uVar5 = FUN_142f04924();
              uVar30 = local_220;
              *(undefined1 *)(*(longlong *)(param_1 + 0x2f8) + 4) = uVar5;
              lVar13 = *(longlong *)(param_1 + 0x2f8);
              bVar28 = *(byte *)(lVar13 + 4);
              *(undefined2 *)(lVar13 + 8) = 0x9a65;
              pbVar21 = (byte *)(lVar13 + 2);
              do {
                if (bVar28 == 0) {
                  bVar28 = 0x2a;
                }
                bVar6 = pbVar21[(longlong)(abStack_22a + -lVar13)];
                pbVar21[-2] = bVar28 ^ bVar6;
                bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
                uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8);
                *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8) =
                     (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                bVar6 = 0x2a;
                if (bVar28 != 0) {
                  bVar6 = bVar28;
                }
                bVar28 = pbVar21[(longlong)(abStack_22a + -lVar13 + 1)];
                pbVar21[-1] = bVar6 ^ bVar28;
                bVar6 = bVar6 + (bVar6 ^ bVar28) + 0x2a;
                uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8);
                *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8) =
                     (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                bVar28 = 0x2a;
                if (bVar6 != 0) {
                  bVar28 = bVar6;
                }
                bVar6 = pbVar21[(longlong)&local_228 - lVar13];
                *pbVar21 = bVar28 ^ bVar6;
                bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
                uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8);
                *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8) =
                     (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                bVar6 = 0x2a;
                if (bVar28 != 0) {
                  bVar6 = bVar28;
                }
                bVar28 = pbVar21[(longlong)&local_228 + -lVar13 + 1];
                pbVar21[1] = bVar6 ^ bVar28;
                bVar28 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
                uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8);
                *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8) =
                     (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                uVar22 = (int)uVar29 + 4;
                uVar29 = (ulonglong)uVar22;
                pbVar21 = pbVar21 + 4;
              } while (uVar22 < 4);
              FUN_1409c6d20(local_220);
              FUN_141ebdf10(param_1);
              uVar7 = FUN_1409c6d00(uVar30);
              FUN_141ebdc70(param_1,uVar7,1);
              goto LAB_141ec080f;
            }
          }
          lVar26 = *(longlong *)(lVar13 + 8);
          *(longlong *)(param_1 + 0x328) = lVar26;
        }
      }
      else if (lVar26 == 0) {
        FUN_141ebdf10(param_1);
        FUN_141ec87b0(param_1);
        goto LAB_141ec080f;
      }
      lVar13 = *(longlong *)(lVar26 + 8);
      pIVar25 = *(IUnknown **)(param_1 + 0x3c8);
      if (pIVar25 != (IUnknown *)0x0) {
        local_128 = CONCAT22(local_128._2_2_,3);
        uStack_120 = 1;
        local_88 = local_128;
        uStack_84 = uStack_124;
        uStack_80 = 1;
        uStack_7c = uStack_11c;
        local_78 = local_118;
        lVar10 = (**(code **)(*(longlong *)pIVar25 + 0x278))(pIVar25,&local_88);
        if (lVar10 < 0) {
          _com_issue_errorex(lVar10,pIVar25,(_GUID *)&DAT_14327fcb0);
        }
        if ((short)local_128 == 8) {
          local_128 = local_128 & 0xffff0000;
          if (CONCAT44(uStack_11c,uStack_120) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_11c,uStack_120) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_128);
        }
      }
      iVar8 = FUN_14019a5d0(param_1 + 0x348);
      if ((iVar8 < 1) || (iVar8 = FUN_1401b0340(param_1 + 0x308), iVar8 < 0)) {
        iVar8 = *(int *)(lVar13 + 0x58);
        uVar22 = 0;
        local_1c0 = 0;
        local_1b8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
        iVar9 = (int)&local_1c0 + -0x3ff8;
        local_1bc = FUN_142f04924();
        local_1bc = local_1bc + iVar9;
        local_1b0 = FUN_142f04924();
        puVar20 = local_1b8;
        local_1b0 = local_1b0 + iVar9;
        *(undefined1 *)((longlong)local_1b8 + 5) = (undefined1)local_1bc;
        *(undefined1 *)((longlong)local_1b8 + 6) = (undefined1)local_1b0;
        local_1c0 = local_1c0 + 1;
        local_228 = iVar8;
        if (local_1c0 == (local_1c0 / 0x6f) * 0x6f) {
          local_1b8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
          *local_1b8 = *puVar20;
          *(undefined4 *)(local_1b8 + 1) = *(undefined4 *)(puVar20 + 1);
          thunk_FUN_140205820(puVar20,0xc);
        }
        bVar28 = FUN_142f04924();
        *(byte *)((longlong)local_1b8 + 4) = bVar28;
        *(undefined2 *)(local_1b8 + 1) = 0x9a65;
        pbVar21 = (byte *)((longlong)local_1b8 + 2);
        do {
          if (bVar28 == 0) {
            bVar28 = 0x2a;
          }
          bVar6 = pbVar21[(longlong)(abStack_22a + -(longlong)local_1b8)];
          pbVar21[-2] = bVar28 ^ bVar6;
          bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
          *(ushort *)(local_1b8 + 1) =
               (*(ushort *)(local_1b8 + 1) >> 0xd) + (ushort)bVar28 |
               *(ushort *)(local_1b8 + 1) << 3;
          bVar6 = 0x2a;
          if (bVar28 != 0) {
            bVar6 = bVar28;
          }
          bVar28 = pbVar21[(longlong)(abStack_22a + -(longlong)local_1b8 + 1)];
          pbVar21[-1] = bVar6 ^ bVar28;
          bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
          *(ushort *)(local_1b8 + 1) =
               (*(ushort *)(local_1b8 + 1) >> 0xd) + (ushort)bVar6 | *(ushort *)(local_1b8 + 1) << 3
          ;
          bVar28 = 0x2a;
          if (bVar6 != 0) {
            bVar28 = bVar6;
          }
          bVar6 = pbVar21[(longlong)&local_228 - (longlong)local_1b8];
          *pbVar21 = bVar28 ^ bVar6;
          bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
          *(ushort *)(local_1b8 + 1) =
               (*(ushort *)(local_1b8 + 1) >> 0xd) + (ushort)bVar6 | *(ushort *)(local_1b8 + 1) << 3
          ;
          bVar28 = 0x2a;
          if (bVar6 != 0) {
            bVar28 = bVar6;
          }
          bVar6 = pbVar21[(longlong)&local_228 + -(longlong)local_1b8 + 1];
          pbVar21[1] = bVar28 ^ bVar6;
          bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
          *(ushort *)(local_1b8 + 1) =
               (*(ushort *)(local_1b8 + 1) >> 0xd) + (ushort)bVar28 |
               *(ushort *)(local_1b8 + 1) << 3;
          uVar22 = uVar22 + 4;
          pbVar21 = pbVar21 + 4;
        } while (uVar22 < 4);
        local_res18 = 2;
        uVar7 = FUN_14019a5d0(&local_1c0);
        uVar22 = 0;
        local_1d8 = 0;
        local_1d0 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
        iVar8 = (int)&local_1d8 + -0x3ff8;
        local_1d4 = FUN_142f04924();
        local_1d4 = local_1d4 + iVar8;
        local_1c8 = FUN_142f04924();
        puVar20 = local_1d0;
        local_1c8 = local_1c8 + iVar8;
        *(undefined1 *)((longlong)local_1d0 + 5) = (undefined1)local_1d4;
        *(undefined1 *)((longlong)local_1d0 + 6) = (undefined1)local_1c8;
        local_1d8 = local_1d8 + 1;
        local_218 = uVar7;
        if (local_1d8 == (local_1d8 / 0x6f) * 0x6f) {
          local_1d0 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
          *local_1d0 = *puVar20;
          *(undefined4 *)(local_1d0 + 1) = *(undefined4 *)(puVar20 + 1);
          thunk_FUN_140205820(puVar20,0xc);
        }
        bVar28 = FUN_142f04924();
        *(byte *)((longlong)local_1d0 + 4) = bVar28;
        *(undefined2 *)(local_1d0 + 1) = 0x9a65;
        pbVar21 = (byte *)((longlong)local_1d0 + 2);
        do {
          if (bVar28 == 0) {
            bVar28 = 0x2a;
          }
          bVar6 = pbVar21[(longlong)&local_220 + (6 - (longlong)local_1d0)];
          pbVar21[-2] = bVar28 ^ bVar6;
          bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
          *(ushort *)(local_1d0 + 1) =
               (*(ushort *)(local_1d0 + 1) >> 0xd) + (ushort)bVar28 |
               *(ushort *)(local_1d0 + 1) << 3;
          bVar6 = 0x2a;
          if (bVar28 != 0) {
            bVar6 = bVar28;
          }
          bVar28 = pbVar21[(longlong)&local_220 + -(longlong)local_1d0 + 7];
          pbVar21[-1] = bVar6 ^ bVar28;
          bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
          *(ushort *)(local_1d0 + 1) =
               (*(ushort *)(local_1d0 + 1) >> 0xd) + (ushort)bVar6 | *(ushort *)(local_1d0 + 1) << 3
          ;
          bVar28 = 0x2a;
          if (bVar6 != 0) {
            bVar28 = bVar6;
          }
          bVar6 = pbVar21[(longlong)&local_218 - (longlong)local_1d0];
          *pbVar21 = bVar28 ^ bVar6;
          bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
          *(ushort *)(local_1d0 + 1) =
               (*(ushort *)(local_1d0 + 1) >> 0xd) + (ushort)bVar6 | *(ushort *)(local_1d0 + 1) << 3
          ;
          bVar28 = 0x2a;
          if (bVar6 != 0) {
            bVar28 = bVar6;
          }
          bVar6 = pbVar21[(longlong)&local_218 + -(longlong)local_1d0 + 1];
          pbVar21[1] = bVar28 ^ bVar6;
          bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
          *(ushort *)(local_1d0 + 1) =
               (*(ushort *)(local_1d0 + 1) >> 0xd) + (ushort)bVar28 |
               *(ushort *)(local_1d0 + 1) << 3;
          uVar22 = uVar22 + 4;
          pbVar21 = pbVar21 + 4;
        } while (uVar22 < 4);
        piVar15 = &local_1d8;
        uVar22 = 6;
      }
      else {
        iVar8 = FUN_14019a5d0(param_1 + 0x348);
        uVar22 = 0;
        local_1a8 = 0;
        local_1a0 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
        iVar9 = (int)&local_1a8 + -0x3ff8;
        local_1a4 = FUN_142f04924();
        local_1a4 = local_1a4 + iVar9;
        local_198 = FUN_142f04924();
        puVar20 = local_1a0;
        local_198 = local_198 + iVar9;
        *(undefined1 *)((longlong)local_1a0 + 5) = (undefined1)local_1a4;
        *(undefined1 *)((longlong)local_1a0 + 6) = (undefined1)local_198;
        local_1a8 = local_1a8 + 1;
        local_228 = iVar8;
        if (local_1a8 == (local_1a8 / 0x6f) * 0x6f) {
          local_1a0 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
          *local_1a0 = *puVar20;
          *(undefined4 *)(local_1a0 + 1) = *(undefined4 *)(puVar20 + 1);
          thunk_FUN_140205820(puVar20,0xc);
        }
        bVar28 = FUN_142f04924();
        *(byte *)((longlong)local_1a0 + 4) = bVar28;
        *(undefined2 *)(local_1a0 + 1) = 0x9a65;
        pbVar21 = (byte *)((longlong)local_1a0 + 2);
        do {
          if (bVar28 == 0) {
            bVar28 = 0x2a;
          }
          bVar6 = pbVar21[(longlong)(abStack_22a + -(longlong)local_1a0)];
          pbVar21[-2] = bVar28 ^ bVar6;
          bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
          *(ushort *)(local_1a0 + 1) =
               (*(ushort *)(local_1a0 + 1) >> 0xd) + (ushort)bVar28 |
               *(ushort *)(local_1a0 + 1) << 3;
          bVar6 = 0x2a;
          if (bVar28 != 0) {
            bVar6 = bVar28;
          }
          bVar28 = pbVar21[(longlong)(abStack_22a + -(longlong)local_1a0 + 1)];
          pbVar21[-1] = bVar6 ^ bVar28;
          bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
          *(ushort *)(local_1a0 + 1) =
               (*(ushort *)(local_1a0 + 1) >> 0xd) + (ushort)bVar6 | *(ushort *)(local_1a0 + 1) << 3
          ;
          bVar28 = 0x2a;
          if (bVar6 != 0) {
            bVar28 = bVar6;
          }
          bVar6 = pbVar21[(longlong)&local_228 - (longlong)local_1a0];
          *pbVar21 = bVar28 ^ bVar6;
          bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
          *(ushort *)(local_1a0 + 1) =
               (*(ushort *)(local_1a0 + 1) >> 0xd) + (ushort)bVar6 | *(ushort *)(local_1a0 + 1) << 3
          ;
          bVar28 = 0x2a;
          if (bVar6 != 0) {
            bVar28 = bVar6;
          }
          bVar6 = pbVar21[(longlong)&local_228 + -(longlong)local_1a0 + 1];
          pbVar21[1] = bVar28 ^ bVar6;
          bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
          *(ushort *)(local_1a0 + 1) =
               (*(ushort *)(local_1a0 + 1) >> 0xd) + (ushort)bVar28 |
               *(ushort *)(local_1a0 + 1) << 3;
          uVar22 = uVar22 + 4;
          pbVar21 = pbVar21 + 4;
        } while (uVar22 < 4);
        piVar15 = &local_1a8;
        uVar22 = 1;
      }
      local_res18 = uVar22;
      local_210 = FUN_14019a5d0(piVar15);
      iVar8 = *(int *)(param_1 + 0x330) + 1;
      *(int *)(param_1 + 0x330) = iVar8;
      if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
        puVar20 = *(undefined8 **)(param_1 + 0x338);
        puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
        *(undefined8 **)(param_1 + 0x338) = puVar19;
        *puVar19 = *puVar20;
        *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
        thunk_FUN_140205820(puVar20,0xc);
      }
      uVar5 = FUN_142f04924();
      uVar30 = local_220;
      uVar29 = local_res8;
      *(undefined1 *)(*(longlong *)(param_1 + 0x338) + 4) = uVar5;
      lVar13 = *(longlong *)(param_1 + 0x338);
      bVar28 = *(byte *)(lVar13 + 4);
      *(undefined2 *)(lVar13 + 8) = 0x9a65;
      uVar11 = 0;
      pbVar21 = (byte *)(lVar13 + 2);
      do {
        if (bVar28 == 0) {
          bVar28 = 0x2a;
        }
        bVar6 = pbVar21[(longlong)(abStack_212 + -lVar13)];
        pbVar21[-2] = bVar28 ^ bVar6;
        bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
        uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x338) + 8);
        *(ushort *)(*(longlong *)(param_1 + 0x338) + 8) =
             (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
        bVar6 = 0x2a;
        if (bVar28 != 0) {
          bVar6 = bVar28;
        }
        bVar28 = pbVar21[(longlong)(abStack_212 + -lVar13 + 1)];
        pbVar21[-1] = bVar6 ^ bVar28;
        bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
        uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x338) + 8);
        *(ushort *)(*(longlong *)(param_1 + 0x338) + 8) =
             (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
        bVar28 = 0x2a;
        if (bVar6 != 0) {
          bVar28 = bVar6;
        }
        bVar6 = pbVar21[(longlong)&local_210 - lVar13];
        *pbVar21 = bVar28 ^ bVar6;
        bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
        uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x338) + 8);
        *(ushort *)(*(longlong *)(param_1 + 0x338) + 8) =
             (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
        bVar28 = 0x2a;
        if (bVar6 != 0) {
          bVar28 = bVar6;
        }
        bVar6 = pbVar21[(longlong)&local_210 + -lVar13 + 1];
        pbVar21[1] = bVar28 ^ bVar6;
        bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
        uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x338) + 8);
        *(ushort *)(*(longlong *)(param_1 + 0x338) + 8) =
             (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
        uVar11 = uVar11 + 4;
        pbVar21 = pbVar21 + 4;
      } while (uVar11 < 4);
      if (((uVar22 & 4) != 0) &&
         (uVar22 = uVar22 & 0xfffffffb, local_res18 = uVar22, local_1d0 != (undefined8 *)0x0)) {
        thunk_FUN_140205820(local_1d0,0xc);
      }
      if (((uVar22 & 2) != 0) &&
         (uVar22 = uVar22 & 0xfffffffd, local_res18 = uVar22, local_1b8 != (undefined8 *)0x0)) {
        thunk_FUN_140205820(local_1b8,0xc);
      }
      param_1 = uVar29;
      if (((uVar22 & 1) != 0) && (local_1a0 != (undefined8 *)0x0)) {
        thunk_FUN_140205820(local_1a0,0xc);
      }
    }
  }
LAB_141ec080f:
  uVar29 = 0;
  lVar13 = param_1 + 0x550;
  FUN_1409d3c60(param_1 + 0x580,lVar13);
  pIVar25 = *(IUnknown **)(param_1 + 0x118);
  if (pIVar25 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_188);
  lVar26 = 8;
  if (DAT_143a8b8d8 == 8) {
    if ((short)local_188 == 8) {
      local_188 = (uint)local_188._2_2_ << 0x10;
      if (uStack_180 != 0) {
        (*DAT_143ad5990)(uStack_180 + -4);
      }
    }
    else {
      iVar8 = (*DAT_143262a18)(&local_188);
      if (iVar8 < 0) goto LAB_141ec1e50;
    }
    local_188 = CONCAT22(local_188._2_2_,8);
    uVar14 = uVar29;
    if (DAT_143a8b8e0 != 0) {
      uVar14 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    uStack_180 = FUN_1401a5fa0(DAT_143a8b8e0,uVar14);
  }
  else {
    if (((short)local_188 == 8) && (local_188 = (uint)local_188._2_2_ << 0x10, uStack_180 != 0)) {
      (*DAT_143ad5990)(uStack_180 + -4);
    }
    iVar8 = (*DAT_143262a28)(&local_188,&DAT_143a8b8d8);
    if (iVar8 < 0) {
LAB_141ec1e50:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar8);
    }
  }
  local_68 = local_188;
  uStack_64 = uStack_184;
  uStack_60 = (undefined4)uStack_180;
  uStack_5c = uStack_180._4_4_;
  local_58 = local_178;
  puVar33 = &local_68;
  uVar32 = 0;
  uVar31 = 0;
  uVar14 = 0;
  iVar8 = (**(code **)(*(longlong *)pIVar25 + 0x138))
                    (pIVar25,local_130,local_12c,0,0,0,0,0,0,puVar33);
  if (iVar8 < 0) {
    _com_issue_errorex(iVar8,pIVar25,(_GUID *)&DAT_143273488);
  }
  if ((short)local_188 == 8) {
    local_188 = local_188 & 0xffff0000;
    if (uStack_180 != 0) {
      (*DAT_143ad5990)(uStack_180 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_188);
  }
  FUN_1409d4040(lVar13,local_130);
  local_168[0] = 0x70;
  local_168[1] = 0;
  local_168[2] = 0;
  local_168[3] = 0;
  pbVar21 = local_168;
  lVar27 = 4;
  uVar23 = uVar29;
  do {
    uVar22 = (int)uVar23 << 8 ^
             *(uint *)(&DAT_143a41330 + (uVar23 >> 0x18 ^ (ulonglong)*pbVar21) * 4);
    uVar23 = (ulonglong)uVar22;
    pbVar21 = pbVar21 + 1;
    lVar27 = lVar27 + -1;
  } while (lVar27 != 0);
  uVar11 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0x5e0) = uVar11;
  uVar22 = uVar11 ^ uVar22;
  uVar22 = uVar22 >> 5 | uVar22 << 0x1b;
  *(uint *)(param_1 + 0x5e4) = uVar22;
  *(uint *)(param_1 + 0x5e8) = ((uVar11 ^ 0xbaadf00d) >> 5 | (uVar11 ^ 0xbaadf00d) << 0x1b) + uVar22
  ;
  uVar22 = FUN_141ed0f60(param_1);
  local_140 = FUN_1401b0340(param_1 + 0x568);
  local_13c = FUN_1401b0340(lVar13);
  pbVar21 = (byte *)&local_140;
  do {
    uVar22 = uVar22 << 8 ^
             *(uint *)(&DAT_143a41330 + ((ulonglong)(uVar22 >> 0x18) ^ (ulonglong)*pbVar21) * 4);
    pbVar21 = pbVar21 + 1;
    lVar26 = lVar26 + -1;
  } while (lVar26 != 0);
  uVar11 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0x5e0) = uVar11;
  uVar22 = (uVar11 ^ uVar22) >> 5 | (uVar11 ^ uVar22) << 0x1b;
  *(uint *)(param_1 + 0x5e4) = uVar22;
  *(uint *)(param_1 + 0x5e8) = ((uVar11 ^ 0xbaadf00d) >> 5 | (uVar11 ^ 0xbaadf00d) << 0x1b) + uVar22
  ;
  plVar24 = (longlong *)(param_1 + 8);
  pcVar2 = *(code **)(*plVar24 + 0x30);
  piVar15 = (int *)(**(code **)(*plVar24 + 0x38))(plVar24,local_90);
  iVar8 = *piVar15;
  piVar15 = (int *)(*pcVar2)(plVar24,local_b8);
  if (*piVar15 == iVar8) {
    pcVar2 = *(code **)(*plVar24 + 0x30);
    lVar13 = (**(code **)(*plVar24 + 0x38))(plVar24,local_b0);
    iVar8 = *(int *)(lVar13 + 4);
    lVar13 = (*pcVar2)(plVar24,local_a8);
    if (*(int *)(lVar13 + 4) != iVar8) goto LAB_141ec0ad7;
    FUN_14100cf40(param_1 + 0x2c0,0x1e);
  }
  else {
LAB_141ec0ad7:
    abStack_20a[2] = 1;
    abStack_20a[3] = 0;
    abStack_20a[4] = 0;
    abStack_20a[5] = 0;
    iVar8 = *(int *)(param_1 + 0x2f0) + 1;
    *(int *)(param_1 + 0x2f0) = iVar8;
    if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
      puVar20 = *(undefined8 **)(param_1 + 0x2f8);
      puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 **)(param_1 + 0x2f8) = puVar19;
      *puVar19 = *puVar20;
      *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
      thunk_FUN_140205820(puVar20,0xc);
    }
    uVar5 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(param_1 + 0x2f8) + 4) = uVar5;
    lVar13 = *(longlong *)(param_1 + 0x2f8);
    bVar28 = *(byte *)(lVar13 + 4);
    *(undefined2 *)(lVar13 + 8) = 0x9a65;
    pbVar21 = (byte *)(lVar13 + 2);
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar6 = pbVar21[(longlong)(abStack_20a + -lVar13)];
      pbVar21[-2] = bVar28 ^ bVar6;
      bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3
      ;
      bVar6 = 0x2a;
      if (bVar28 != 0) {
        bVar6 = bVar28;
      }
      bVar28 = pbVar21[(longlong)(abStack_20a + -lVar13 + 1)];
      pbVar21[-1] = bVar6 ^ bVar28;
      bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
      bVar28 = 0x2a;
      if (bVar6 != 0) {
        bVar28 = bVar6;
      }
      bVar6 = pbVar21[(longlong)(abStack_20a + (2 - lVar13))];
      *pbVar21 = bVar28 ^ bVar6;
      bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
      bVar28 = 0x2a;
      if (bVar6 != 0) {
        bVar28 = bVar6;
      }
      bVar6 = pbVar21[(longlong)(abStack_20a + -lVar13 + 3)];
      pbVar21[1] = bVar28 ^ bVar6;
      bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2f8) + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3
      ;
      uVar22 = (int)uVar29 + 4;
      uVar29 = (ulonglong)uVar22;
      pbVar21 = pbVar21 + 4;
    } while (uVar22 < 4);
    abStack_202[2] = 0;
    abStack_202[3] = 0;
    abStack_202[4] = 0;
    abStack_202[5] = 0;
    iVar8 = *(int *)(param_1 + 0x2c0) + 1;
    *(int *)(param_1 + 0x2c0) = iVar8;
    if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
      puVar20 = *(undefined8 **)(param_1 + 0x2c8);
      puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 **)(param_1 + 0x2c8) = puVar19;
      *puVar19 = *puVar20;
      *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
      thunk_FUN_140205820(puVar20,0xc);
    }
    uVar5 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(param_1 + 0x2c8) + 4) = uVar5;
    lVar13 = *(longlong *)(param_1 + 0x2c8);
    bVar28 = *(byte *)(lVar13 + 4);
    *(undefined2 *)(lVar13 + 8) = 0x9a65;
    uVar22 = 0;
    pbVar21 = (byte *)(lVar13 + 2);
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar6 = pbVar21[(longlong)(abStack_202 + -lVar13)];
      pbVar21[-2] = bVar28 ^ bVar6;
      bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2c8) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2c8) + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3
      ;
      bVar6 = 0x2a;
      if (bVar28 != 0) {
        bVar6 = bVar28;
      }
      bVar28 = pbVar21[(longlong)(abStack_202 + -lVar13 + 1)];
      pbVar21[-1] = bVar6 ^ bVar28;
      bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2c8) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2c8) + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
      bVar28 = 0x2a;
      if (bVar6 != 0) {
        bVar28 = bVar6;
      }
      bVar6 = pbVar21[(longlong)(abStack_202 + (2 - lVar13))];
      *pbVar21 = bVar28 ^ bVar6;
      bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2c8) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2c8) + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
      bVar28 = 0x2a;
      if (bVar6 != 0) {
        bVar28 = bVar6;
      }
      bVar6 = pbVar21[(longlong)(abStack_202 + -lVar13 + 3)];
      pbVar21[1] = bVar28 ^ bVar6;
      bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2c8) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2c8) + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3
      ;
      uVar22 = uVar22 + 4;
      pbVar21 = pbVar21 + 4;
      uVar30 = local_220;
    } while (uVar22 < 4);
  }
  plVar24 = (longlong *)(param_1 + 8);
  iVar8 = FUN_1409c5080(uVar30);
  if ((((iVar8 != 0) && ((char)local_res10 == '\0')) &&
      (iVar8 = (**(code **)(**(longlong **)(param_1 + 0x120) + 0x68))(), iVar8 == 0)) &&
     (iVar8 = FUN_140f810b0(*(longlong *)(param_1 + 0x120) + 0x100), iVar8 == 0)) {
    uVar16 = (**(code **)(*(longlong *)(*(longlong *)(param_1 + 0x120) + 8) + 0x50))();
    iVar8 = FUN_14094fe70(uVar16);
    if ((iVar8 == 0) && (iVar8 = FUN_14279c310(*(undefined8 *)(param_1 + 0x120)), iVar8 == 0)) {
      lVar13 = FUN_141892840();
      if (lVar13 != 0) {
        uVar16 = FUN_141892840();
        iVar8 = FUN_14182e770(uVar16);
        if (iVar8 != 0) goto LAB_141ec0fa3;
      }
      (**(code **)(*plVar24 + 0x30))(plVar24,local_158);
      lVar13 = FUN_141ebdad0(param_1);
      if (lVar13 != 0) {
        plVar17 = (longlong *)FUN_141ebdad0(param_1);
        iVar8 = (**(code **)(*plVar17 + 0x378))(plVar17);
        if (iVar8 != 0) {
          FUN_14179e990(DAT_143ace240,param_1,local_158,param_1 + 0x550);
        }
      }
      piVar15 = (int *)(**(code **)(*plVar24 + 0x30))(plVar24,local_a0);
      if (local_158[0] == *piVar15) {
        (**(code **)(*plVar24 + 0x30))(plVar24,local_98);
      }
    }
  }
LAB_141ec0fa3:
  bVar3 = false;
  puVar18 = (undefined1 *)FUN_141892840();
  if ((puVar18 == (undefined1 *)0x0) || (puVar18 == (undefined1 *)0xffffffffffffffe0)) {
    local_108 = (undefined1 *)0x0;
  }
  else {
    local_108 = puVar18;
    if (0xfffff < *(ulonglong *)(puVar18 + 0x28)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(puVar18 + 0x28) = *(longlong *)(puVar18 + 0x28) + 1;
    UNLOCK();
  }
  puVar18 = local_108;
  uVar7 = (undefined4)((ulonglong)uVar32 >> 0x20);
  uVar34 = (undefined4)((ulonglong)puVar33 >> 0x20);
  local_148 = local_108;
  if (local_108 == (undefined1 *)0x0) {
LAB_141ec1032:
    uVar7 = (undefined4)((ulonglong)uVar32 >> 0x20);
    uVar34 = (undefined4)((ulonglong)puVar33 >> 0x20);
    if (DAT_143aa84a0 != 0) {
      iVar8 = (**(code **)(**(longlong **)(param_1 + 0x120) + 0x50))();
      uVar7 = (undefined4)((ulonglong)uVar32 >> 0x20);
      uVar34 = (undefined4)((ulonglong)puVar33 >> 0x20);
      if (iVar8 != 0) {
        iVar8 = FUN_140f810b0(*(longlong *)(param_1 + 0x120) + 0x100);
        uVar7 = (undefined4)((ulonglong)uVar32 >> 0x20);
        uVar34 = (undefined4)((ulonglong)puVar33 >> 0x20);
        if (iVar8 == 0) {
          lVar13 = FUN_141ebdad0(param_1);
          uVar7 = (undefined4)((ulonglong)uVar32 >> 0x20);
          uVar34 = (undefined4)((ulonglong)puVar33 >> 0x20);
          if (lVar13 != 0) {
            uVar16 = FUN_141ebdad0(param_1);
            cVar4 = FUN_140d85ab0(uVar16);
            uVar7 = (undefined4)((ulonglong)uVar32 >> 0x20);
            uVar34 = (undefined4)((ulonglong)puVar33 >> 0x20);
            if (cVar4 < '\0') {
              piVar15 = (int *)(param_1 + 0x1a0);
              iVar8 = FUN_14019a5d0(piVar15);
              uVar7 = (undefined4)((ulonglong)uVar32 >> 0x20);
              uVar34 = (undefined4)((ulonglong)puVar33 >> 0x20);
              if ((iVar8 + 1000 < (int)local_190) && (!bVar3)) {
                local_160 = (longlong **)FUN_142cbe730(DAT_143aa84a0);
                iVar8 = 0;
                local_220 = local_220 & 0xffffffff00000000;
                plVar24 = (longlong *)(param_1 + 0x1c0);
                lVar13 = param_1 + 0x1e8;
                do {
                  lVar26 = DAT_143aa84a0;
                  local_150 = lVar13;
                  uVar7 = FUN_1401b0340(param_1 + 0x138);
                  uVar22 = FUN_142d175c0(lVar26,uVar7,iVar8);
                  if (uVar22 != 0) {
                    uVar11 = FUN_1407ea820(uVar22,local_160,0);
                    if (0 < (int)uVar11) {
                      uVar22 = uVar11;
                    }
                    iVar9 = FUN_1407e6cb0(uVar22);
                    if (iVar9 != 0) {
                      uVar11 = FUN_14019a5d0(lVar13);
                      if (uVar11 != uVar22) {
                        iVar8 = (int)plVar24[5] + 1;
                        *(int *)(plVar24 + 5) = iVar8;
                        local_res10 = uVar22;
                        if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
                          puVar20 = (undefined8 *)plVar24[6];
                          puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                          plVar24[6] = (longlong)puVar19;
                          *puVar19 = *puVar20;
                          *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
                          thunk_FUN_140205820(puVar20,0xc);
                        }
                        uVar5 = FUN_142f04924();
                        *(undefined1 *)(plVar24[6] + 4) = uVar5;
                        lVar13 = plVar24[6];
                        bVar28 = *(byte *)(lVar13 + 4);
                        *(undefined2 *)(lVar13 + 8) = 0x9a65;
                        uVar11 = 0;
                        pbVar21 = (byte *)(lVar13 + 2);
                        lVar26 = (longlong)&local_res10 + (1 - lVar13);
                        do {
                          if (bVar28 == 0) {
                            bVar28 = 0x2a;
                          }
                          bVar6 = pbVar21[(longlong)&local_res8 + (6 - lVar13)];
                          pbVar21[-2] = bVar28 ^ bVar6;
                          bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
                          uVar1 = *(ushort *)(plVar24[6] + 8);
                          *(ushort *)(plVar24[6] + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3
                          ;
                          bVar6 = 0x2a;
                          if (bVar28 != 0) {
                            bVar6 = bVar28;
                          }
                          bVar28 = pbVar21[lVar26 + -2];
                          pbVar21[-1] = bVar6 ^ bVar28;
                          bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
                          uVar1 = *(ushort *)(plVar24[6] + 8);
                          *(ushort *)(plVar24[6] + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                          bVar28 = 0x2a;
                          if (bVar6 != 0) {
                            bVar28 = bVar6;
                          }
                          bVar6 = pbVar21[(longlong)&local_res10 - lVar13];
                          *pbVar21 = bVar28 ^ bVar6;
                          bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                          uVar1 = *(ushort *)(plVar24[6] + 8);
                          *(ushort *)(plVar24[6] + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                          bVar28 = 0x2a;
                          if (bVar6 != 0) {
                            bVar28 = bVar6;
                          }
                          bVar6 = pbVar21[lVar26];
                          pbVar21[1] = bVar28 ^ bVar6;
                          bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                          uVar1 = *(ushort *)(plVar24[6] + 8);
                          *(ushort *)(plVar24[6] + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3
                          ;
                          uVar11 = uVar11 + 4;
                          pbVar21 = pbVar21 + 4;
                        } while (uVar11 < 4);
                        abStack_1fa[2] = 1;
                        abStack_1fa[3] = 0;
                        abStack_1fa[4] = 0;
                        abStack_1fa[5] = 0;
                        iVar8 = (int)plVar24[-1] + 1;
                        *(int *)(plVar24 + -1) = iVar8;
                        if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
                          puVar20 = (undefined8 *)*plVar24;
                          puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                          *plVar24 = (longlong)puVar19;
                          *puVar19 = *puVar20;
                          *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
                          thunk_FUN_140205820(puVar20,0xc);
                        }
                        uVar5 = FUN_142f04924();
                        *(undefined1 *)(*plVar24 + 4) = uVar5;
                        lVar13 = *plVar24;
                        bVar28 = *(byte *)(lVar13 + 4);
                        *(undefined2 *)(lVar13 + 8) = 0x9a65;
                        uVar11 = 0;
                        pbVar21 = (byte *)(lVar13 + 2);
                        do {
                          if (bVar28 == 0) {
                            bVar28 = 0x2a;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1fa + -lVar13)];
                          pbVar21[-2] = bVar28 ^ bVar6;
                          bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                          bVar6 = 0x2a;
                          if (bVar28 != 0) {
                            bVar6 = bVar28;
                          }
                          bVar28 = pbVar21[(longlong)(abStack_1fa + -lVar13 + 1)];
                          pbVar21[-1] = bVar6 ^ bVar28;
                          bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                          bVar28 = 0x2a;
                          if (bVar6 != 0) {
                            bVar28 = bVar6;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1fa + (2 - lVar13))];
                          *pbVar21 = bVar28 ^ bVar6;
                          bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                          bVar28 = 0x2a;
                          if (bVar6 != 0) {
                            bVar28 = bVar6;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1fa + -lVar13 + 3)];
                          pbVar21[1] = bVar28 ^ bVar6;
                          bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                          uVar11 = uVar11 + 4;
                          pbVar21 = pbVar21 + 4;
                        } while (uVar11 < 4);
                        piVar15 = (int *)(local_res8 + 0x1a0);
                        lVar13 = local_150;
                        param_1 = local_res8;
                      }
                      uVar16 = *(undefined8 *)(param_1 + 0x120);
                      uVar7 = FUN_14019a5d0(lVar13 + -0x30);
                      uVar11 = 0;
                      local_170 = 0;
                      FUN_14019a260(&local_170,param_1 + 0x130);
                      iVar8 = FUN_14296fd50(uVar16,uVar22,&local_170,uVar7);
                      if (iVar8 == 1) {
                        abStack_1f2[2] = 0;
                        abStack_1f2[3] = 0;
                        abStack_1f2[4] = 0;
                        abStack_1f2[5] = 0;
                        iVar8 = (int)plVar24[-1] + 1;
                        *(int *)(plVar24 + -1) = iVar8;
                        if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
                          puVar20 = (undefined8 *)*plVar24;
                          puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                          *plVar24 = (longlong)puVar19;
                          *puVar19 = *puVar20;
                          *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
                          thunk_FUN_140205820(puVar20,0xc);
                        }
                        uVar5 = FUN_142f04924();
                        *(undefined1 *)(*plVar24 + 4) = uVar5;
                        lVar13 = *plVar24;
                        bVar28 = *(byte *)(lVar13 + 4);
                        *(undefined2 *)(lVar13 + 8) = 0x9a65;
                        pbVar21 = (byte *)(lVar13 + 2);
                        do {
                          if (bVar28 == 0) {
                            bVar28 = 0x2a;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1f2 + -lVar13)];
                          pbVar21[-2] = bVar28 ^ bVar6;
                          bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                          bVar6 = 0x2a;
                          if (bVar28 != 0) {
                            bVar6 = bVar28;
                          }
                          bVar28 = pbVar21[(longlong)(abStack_1f2 + -lVar13 + 1)];
                          pbVar21[-1] = bVar6 ^ bVar28;
                          bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                          bVar28 = 0x2a;
                          if (bVar6 != 0) {
                            bVar28 = bVar6;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1f2 + (2 - lVar13))];
                          *pbVar21 = bVar28 ^ bVar6;
                          bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                          bVar28 = 0x2a;
                          if (bVar6 != 0) {
                            bVar28 = bVar6;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1f2 + -lVar13 + 3)];
                          pbVar21[1] = bVar28 ^ bVar6;
                          bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                          uVar11 = uVar11 + 4;
                          pbVar21 = pbVar21 + 4;
                        } while (uVar11 < 4);
                        piVar15 = (int *)(param_1 + 0x1a0);
                      }
                      else if (iVar8 == 2) {
                        abStack_1ea[2] = 1;
                        abStack_1ea[3] = 0;
                        abStack_1ea[4] = 0;
                        abStack_1ea[5] = 0;
                        iVar8 = (int)plVar24[-1] + 1;
                        *(int *)(plVar24 + -1) = iVar8;
                        if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
                          puVar20 = (undefined8 *)*plVar24;
                          puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                          *plVar24 = (longlong)puVar19;
                          *puVar19 = *puVar20;
                          *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
                          thunk_FUN_140205820(puVar20,0xc);
                        }
                        uVar5 = FUN_142f04924();
                        *(undefined1 *)(*plVar24 + 4) = uVar5;
                        lVar13 = *plVar24;
                        bVar28 = *(byte *)(lVar13 + 4);
                        *(undefined2 *)(lVar13 + 8) = 0x9a65;
                        pbVar21 = (byte *)(lVar13 + 2);
                        do {
                          if (bVar28 == 0) {
                            bVar28 = 0x2a;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1ea + -lVar13)];
                          pbVar21[-2] = bVar28 ^ bVar6;
                          bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                          bVar6 = 0x2a;
                          if (bVar28 != 0) {
                            bVar6 = bVar28;
                          }
                          bVar28 = pbVar21[(longlong)(abStack_1ea + -lVar13 + 1)];
                          pbVar21[-1] = bVar6 ^ bVar28;
                          bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                          bVar28 = 0x2a;
                          if (bVar6 != 0) {
                            bVar28 = bVar6;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1ea + (2 - lVar13))];
                          *pbVar21 = bVar28 ^ bVar6;
                          bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                          bVar28 = 0x2a;
                          if (bVar6 != 0) {
                            bVar28 = bVar6;
                          }
                          bVar6 = pbVar21[(longlong)(abStack_1ea + -lVar13 + 3)];
                          pbVar21[1] = bVar28 ^ bVar6;
                          bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                          uVar1 = *(ushort *)(*plVar24 + 8);
                          *(ushort *)(*plVar24 + 8) = (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                          uVar11 = uVar11 + 4;
                          pbVar21 = pbVar21 + 4;
                        } while (uVar11 < 4);
                      }
                      uVar22 = 0;
                      local_1e0 = (int)local_190;
                      iVar8 = *piVar15 + 1;
                      *piVar15 = iVar8;
                      if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
                        puVar20 = *(undefined8 **)(piVar15 + 2);
                        puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                        *(undefined8 **)(piVar15 + 2) = puVar19;
                        *puVar19 = *puVar20;
                        *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar20 + 1);
                        thunk_FUN_140205820(puVar20,0xc);
                      }
                      uVar5 = FUN_142f04924();
                      *(undefined1 *)(*(longlong *)(piVar15 + 2) + 4) = uVar5;
                      lVar13 = *(longlong *)(piVar15 + 2);
                      bVar28 = *(byte *)(lVar13 + 4);
                      *(undefined2 *)(lVar13 + 8) = 0x9a65;
                      pbVar21 = (byte *)(lVar13 + 2);
                      do {
                        if (bVar28 == 0) {
                          bVar28 = 0x2a;
                        }
                        bVar6 = pbVar21[(longlong)(abStack_1e2 + -lVar13)];
                        pbVar21[-2] = bVar28 ^ bVar6;
                        bVar28 = bVar28 + (bVar28 ^ bVar6) + 0x2a;
                        uVar1 = *(ushort *)(*(longlong *)(piVar15 + 2) + 8);
                        *(ushort *)(*(longlong *)(piVar15 + 2) + 8) =
                             (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                        bVar6 = 0x2a;
                        if (bVar28 != 0) {
                          bVar6 = bVar28;
                        }
                        bVar28 = pbVar21[(longlong)(abStack_1e2 + -lVar13 + 1)];
                        pbVar21[-1] = bVar6 ^ bVar28;
                        bVar6 = (bVar6 ^ bVar28) + bVar6 + 0x2a;
                        uVar1 = *(ushort *)(*(longlong *)(piVar15 + 2) + 8);
                        *(ushort *)(*(longlong *)(piVar15 + 2) + 8) =
                             (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                        bVar28 = 0x2a;
                        if (bVar6 != 0) {
                          bVar28 = bVar6;
                        }
                        bVar6 = pbVar21[(longlong)&local_1e0 - lVar13];
                        *pbVar21 = bVar28 ^ bVar6;
                        bVar6 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                        uVar1 = *(ushort *)(*(longlong *)(piVar15 + 2) + 8);
                        *(ushort *)(*(longlong *)(piVar15 + 2) + 8) =
                             (uVar1 >> 0xd) + (ushort)bVar6 | uVar1 << 3;
                        bVar28 = 0x2a;
                        if (bVar6 != 0) {
                          bVar28 = bVar6;
                        }
                        bVar6 = pbVar21[(longlong)&local_1e0 + -lVar13 + 1];
                        pbVar21[1] = bVar28 ^ bVar6;
                        bVar28 = (bVar28 ^ bVar6) + bVar28 + 0x2a;
                        uVar1 = *(ushort *)(*(longlong *)(piVar15 + 2) + 8);
                        *(ushort *)(*(longlong *)(piVar15 + 2) + 8) =
                             (uVar1 >> 0xd) + (ushort)bVar28 | uVar1 << 3;
                        uVar22 = uVar22 + 4;
                        pbVar21 = pbVar21 + 4;
                      } while (uVar22 < 4);
                      lVar13 = local_150;
                      param_1 = local_res8;
                      iVar8 = (int)local_220;
                    }
                  }
                  uVar7 = (undefined4)((ulonglong)uVar32 >> 0x20);
                  uVar34 = (undefined4)((ulonglong)puVar33 >> 0x20);
                  iVar8 = iVar8 + 1;
                  local_220 = CONCAT44(local_220._4_4_,iVar8);
                  lVar13 = lVar13 + 0x18;
                  plVar24 = plVar24 + 3;
                  puVar18 = local_148;
                  local_150 = lVar13;
                } while (iVar8 < 2);
              }
            }
          }
        }
      }
    }
  }
  else if (DAT_143aa84a0 != 0) {
    iVar8 = FUN_14182e410(local_108);
    if ((iVar8 == 0) || (iVar8 = FUN_142cf33e0(DAT_143aa84a0), iVar8 == 0)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    goto LAB_141ec1032;
  }
  if (*(longlong *)(param_1 + 0x3c8) != 0) {
    iVar8 = FUN_14019a5d0(param_1 + 0x218);
    pIVar25 = *(IUnknown **)(param_1 + 0x3c8);
    if (pIVar25 == (IUnknown *)0x0) {
LAB_141ec1e45:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    if (iVar8 < 0) {
      iVar8 = (**(code **)(*(longlong *)pIVar25 + 0x318))(pIVar25,0x40);
    }
    else {
      iVar8 = (**(code **)(*(longlong *)pIVar25 + 0x310))();
      if (iVar8 < 0) {
        _com_issue_errorex(iVar8,pIVar25,(_GUID *)&DAT_14327fcb0);
      }
      pIVar25 = *(IUnknown **)(param_1 + 0x3c8);
      if (pIVar25 == (IUnknown *)0x0) goto LAB_141ec1e45;
      uVar12 = FUN_14019a5d0(param_1 + 0x218);
      iVar8 = (**(code **)(*(longlong *)pIVar25 + 0x330))(pIVar25,3,uVar12);
    }
    if (iVar8 < 0) {
      _com_issue_errorex(iVar8,pIVar25,(_GUID *)&DAT_14327fcb0);
    }
  }
  piVar15 = *(int **)(param_1 + 0x110);
  if ((((piVar15 != (int *)0x0) || (piVar15 = *(int **)(param_1 + 0x108), piVar15 != (int *)0x0)) &&
      (*piVar15 - 5000000U < 10000)) && (*piVar15 - 0x4c4f28U < 1000)) {
    FUN_141ec7ee0(param_1);
    iVar8 = FUN_141ece0a0(param_1);
    if (iVar8 != 0) {
      iVar8 = (**(code **)(**(longlong **)(param_1 + 0x120) + 0x50))();
      if (iVar8 == 0) {
        iVar8 = FUN_1409d4410(param_1 + 0x290);
        if (iVar8 != 0) {
LAB_141ec1bed:
          uVar32 = DAT_143abfdf0;
          if (*(longlong *)(param_1 + 0x100) == 0) {
            local_160 = &local_190;
            local_190 = *(longlong **)(param_1 + 0x3c8);
            if (local_190 != (longlong *)0x0) {
              (**(code **)(*local_190 + 8))();
            }
            local_148 = local_f8;
            uVar16 = FUN_141ec2c20(param_1,local_f8);
            puVar20 = (undefined8 *)FUN_1408a9d20(&local_f0,0x609);
            uVar32 = FUN_140e16f90(uVar32,*puVar20,0,0,uVar16,uVar14 & 0xffffffff00000000,
                                   uVar31 & 0xffffffff00000000,&local_190,CONCAT44(uVar7,1),
                                   CONCAT44(uVar34,0xff),1,0,0);
            *(undefined8 *)(param_1 + 0x100) = uVar32;
            if (local_f0 != 0) {
              FUN_1401bebb0(local_f0 + -0x10);
            }
          }
          goto LAB_141ec1cba;
        }
      }
      else {
        lVar13 = FUN_141ebdad0(param_1);
        if (lVar13 != 0) {
          lVar13 = FUN_141ebdad0(param_1);
          bVar28 = FUN_1401b0050(lVar13 + 0x6a,*(undefined4 *)(lVar13 + 0x6e));
          if (100 < bVar28) goto LAB_141ec1bed;
        }
      }
    }
    if (*(longlong *)(param_1 + 0x100) != 0) {
      FUN_140dd7fc0(DAT_143abfdf0,*(longlong *)(param_1 + 0x100),0);
      *(undefined8 *)(param_1 + 0x100) = 0;
    }
  }
LAB_141ec1cba:
  cVar4 = FUN_141ecce00(param_1);
  if (cVar4 == '\0') {
    if (*(longlong **)(param_1 + 0x3d0) != (longlong *)0x0) {
      (**(code **)(**(longlong **)(param_1 + 0x3d0) + 0x10))();
    }
    *(undefined8 *)(param_1 + 0x3d0) = 0;
  }
  FUN_141ecde00(param_1);
  lVar13 = FUN_141892840();
  if (lVar13 == 0) goto LAB_141ec1de5;
  lVar26 = FUN_141892840();
  local_res8 = local_res8 & 0xffffffffffffff00;
  local_e0 = &local_res8;
  local_e8 = lVar26;
  local_d8 = param_1;
  FUN_141ed0760(&local_e8);
  lVar13 = 1;
  if ((*(longlong **)(param_1 + 0x120) != (longlong *)0x0) &&
     (iVar8 = (**(code **)(**(longlong **)(param_1 + 0x120) + 0x50))(), iVar8 != 0)) {
    lVar13 = 0;
  }
  lVar27 = *(longlong *)(lVar26 + 0xa8);
  cVar4 = *(char *)(lVar27 + 0x1d30 + lVar13 * 0x18);
  if (cVar4 == '\0') {
    if (*(char *)(param_1 + 0x660) != '\0') {
      *(undefined1 *)(param_1 + 0x660) = 0;
      cVar4 = *(char *)(lVar27 + 0x1d30 + lVar13 * 0x18);
      goto LAB_141ec1d79;
    }
  }
  else {
LAB_141ec1d79:
    *(char *)(param_1 + 0x660) = cVar4;
    uVar7 = *(undefined4 *)(lVar27 + 0x1d34 + lVar13 * 0x18);
    uVar5 = *(undefined1 *)(lVar27 + 0x1d30 + lVar13 * 0x18);
    local_138 = *(longlong **)(param_1 + 0x3c8);
    if (local_138 != (longlong *)0x0) {
      (**(code **)(*local_138 + 8))();
    }
    FUN_141ed0450(&local_res8,&local_138,uVar5,uVar7);
  }
  local_c0 = &local_res8;
  local_d0 = lVar26;
  local_c8 = param_1;
  FUN_141ed04c0(&local_d0);
LAB_141ec1de5:
  FUN_141ecedb0(param_1,param_1 + 0x550);
  if (puVar18 != (undefined1 *)0x0) {
    if (0xffffe < *(longlong *)(puVar18 + 0x28) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar24 = (longlong *)(puVar18 + 0x28);
    lVar13 = *plVar24;
    *plVar24 = *plVar24 + -1;
    UNLOCK();
    if (((int)lVar13 == 1) &&
       (puVar20 = (undefined8 *)(local_108 + 0x20), puVar20 != (undefined8 *)0x0)) {
      (**(code **)*puVar20)(puVar20,1);
    }
  }
  return;
}



//===========================================================
// FUN_14159b0a0 @ 14159b0a0   (254 bytes)
//===========================================================

void FUN_14159b0a0(longlong param_1,undefined4 param_2)

{
  IUnknown *pIVar1;
  int iVar2;
  
  pIVar1 = *(IUnknown **)(param_1 + 0x18);
  if (pIVar1 != (IUnknown *)0x0) {
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
  }
  pIVar1 = *(IUnknown **)(param_1 + 0x20);
  if (pIVar1 != (IUnknown *)0x0) {
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1,param_2);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
  }
  pIVar1 = *(IUnknown **)(param_1 + 0x30);
  if (pIVar1 != (IUnknown *)0x0) {
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1,param_2);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
  }
  pIVar1 = *(IUnknown **)(param_1 + 0x38);
  if (pIVar1 != (IUnknown *)0x0) {
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1,param_2);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
  }
  pIVar1 = *(IUnknown **)(param_1 + 0x40);
  if (pIVar1 != (IUnknown *)0x0) {
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1,param_2);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
  }
  return;
}



//===========================================================
// FUN_141ecde00 @ 141ecde00   (483 bytes)
//===========================================================

void FUN_141ecde00(longlong *param_1)

{
  IUnknown *pIVar1;
  bool bVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  longlong lVar6;
  undefined8 uVar7;
  int iVar8;
  int local_res8 [2];
  
  iVar5 = 0;
  cVar3 = FUN_1409d6150(param_1 + 0xc6);
  iVar8 = iVar5;
  if ((cVar3 == '\0') && (param_1[0x24] != 0)) {
    lVar6 = param_1[0x23] + -0x20;
    if (param_1[0x23] == 0) {
      lVar6 = 0;
    }
    if (lVar6 != 0) {
      cVar3 = FUN_142826340();
      if (cVar3 == '\0') {
        cVar3 = FUN_140f80830(param_1[0x24] + 0x100);
        if (cVar3 == '\0') {
          cVar3 = FUN_140f80860(param_1[0x24] + 0x100);
          if (cVar3 == '\0') {
            iVar4 = (**(code **)(*(longlong *)param_1[0x24] + 0x50))();
            if (iVar4 == 0) {
              cVar3 = FUN_142d0f360(DAT_143aa84a0);
              if (cVar3 != '\0') goto LAB_141ecdf15;
            }
            cVar3 = FUN_1409bd2f0(lVar6);
            if (cVar3 == '\0') {
              lVar6 = FUN_141892840();
              if (lVar6 == 0) {
LAB_141ecdeeb:
                bVar2 = false;
              }
              else {
                uVar7 = FUN_141892840();
                cVar3 = FUN_14183a640(uVar7);
                if (cVar3 == '\0') goto LAB_141ecdeeb;
                bVar2 = true;
              }
              iVar4 = (**(code **)(*(longlong *)param_1[0x24] + 0x50))();
              if ((iVar4 != 0) || (!bVar2)) {
                iVar4 = FUN_142cc1e40(DAT_143aa84a0);
                if (iVar4 == 0) {
                  iVar8 = 1;
                }
              }
            }
          }
        }
      }
    }
  }
LAB_141ecdf15:
  pIVar1 = (IUnknown *)param_1[0x79];
  if (pIVar1 != (IUnknown *)0x0) {
    local_res8[0] = 0;
    iVar4 = (**(code **)(*(longlong *)pIVar1 + 0x2b0))(pIVar1,local_res8);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    iVar4 = 1;
    if (local_res8[0] != 0) goto LAB_141ecdf56;
  }
  iVar4 = iVar5;
LAB_141ecdf56:
  if (iVar4 != iVar8) {
    pIVar1 = (IUnknown *)param_1[0x79];
    if (pIVar1 != (IUnknown *)0x0) {
      iVar5 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1,iVar8);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
    }
    FUN_14159b0a0(param_1[8],iVar8);
    (**(code **)(*param_1 + 0x18))(param_1,iVar8,0);
    FUN_141ecaf00(param_1);
    if (iVar8 == 0) {
      if (param_1[0x20] != 0) {
        FUN_140dd7fc0(DAT_143abfdf0,param_1[0x20],0);
      }
      param_1[0x20] = 0;
    }
  }
  return;
}



//===========================================================
// FUN_141ecdff0 @ 141ecdff0   (156 bytes)
//===========================================================

void FUN_141ecdff0(longlong *param_1,int param_2)

{
  IUnknown *pIVar1;
  int iVar2;
  
  pIVar1 = (IUnknown *)param_1[0x79];
  if (pIVar1 != (IUnknown *)0x0) {
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
  }
  FUN_14159b0a0(param_1[8],param_2);
  (**(code **)(*param_1 + 0x18))(param_1,param_2,0);
  FUN_141ecaf00(param_1);
  if (param_2 == 0) {
    if (param_1[0x20] != 0) {
      FUN_140dd7fc0(DAT_143abfdf0,param_1[0x20],0);
    }
    param_1[0x20] = 0;
  }
  return;
}


