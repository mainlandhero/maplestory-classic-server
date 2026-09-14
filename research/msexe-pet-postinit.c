
//===========================================================
// FUN_141ec87b0 @ 141ec87b0   (8007 bytes)
//===========================================================

void FUN_141ec87b0(longlong param_1)

{
  ushort uVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  IUnknown *pIVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined1 uVar8;
  byte bVar9;
  char cVar10;
  short sVar11;
  int iVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  int iVar16;
  undefined8 *puVar17;
  byte bVar18;
  longlong lVar19;
  ulonglong uVar20;
  undefined4 *puVar21;
  uint uVar22;
  uint uVar24;
  byte bVar25;
  byte *pbVar26;
  longlong lVar27;
  undefined8 uVar28;
  undefined8 local_res8;
  undefined8 local_res10;
  undefined8 local_res18;
  uint local_res20 [2];
  undefined8 in_stack_fffffffffffffe20;
  undefined4 local_1c8;
  int iStack_1c4;
  byte *pbStack_1c0;
  int local_1b8;
  undefined4 uStack_1b4;
  undefined4 local_1b0;
  int iStack_1ac;
  byte *pbStack_1a8;
  int local_1a0;
  undefined4 uStack_19c;
  undefined4 local_198;
  int iStack_194;
  byte *pbStack_190;
  int local_188;
  undefined4 uStack_184;
  short local_180;
  undefined2 uStack_17e;
  undefined4 uStack_17c;
  undefined8 uStack_178;
  undefined8 local_170;
  int local_168;
  int local_164;
  longlong local_160;
  uint local_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 local_148;
  uint local_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 local_128;
  longlong local_118;
  ulonglong local_110;
  undefined8 local_108;
  byte *pbStack_100;
  undefined8 local_f8;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  uint local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 local_b8;
  undefined8 local_a8;
  byte *pbStack_a0;
  undefined8 local_98;
  undefined8 local_88;
  byte *pbStack_80;
  undefined8 local_78;
  undefined8 local_68;
  byte *pbStack_60;
  undefined8 local_58;
  ulonglong uVar23;
  
  local_res8 = param_1;
  while( true ) {
    while( true ) {
      uVar24 = 0;
      local_res10 = local_res10 & 0xffffffff00000000;
      if ((*(longlong *)(param_1 + 0x120) != 0) &&
         (iVar12 = FUN_140f8abc0(*(longlong *)(param_1 + 0x120) + 0x100), iVar12 != 0)) {
        return;
      }
      local_res20[0] = FUN_141ebe350(param_1,&local_164);
      FUN_141ec3380(param_1,local_res20);
      sVar11 = FUN_141ed0120(param_1 + 0x230);
      local_168 = (int)sVar11;
      iVar12 = FUN_1409d4840(param_1 + 0x278);
      uVar22 = local_res20[0];
      uVar15 = (undefined4)((ulonglong)in_stack_fffffffffffffe20 >> 0x20);
      lVar19 = 0x360;
      if (iVar12 != 0) {
        lVar19 = 0x368;
      }
      if (((-1 < (int)local_res20[0]) && (lVar2 = *(longlong *)(lVar19 + param_1), lVar2 != 0)) &&
         (local_res20[0] < *(uint *)(lVar2 + -8))) break;
      iVar12 = FUN_1401b0340(param_1 + 0x308);
      if (-1 < iVar12) {
        FUN_141ebdf10(param_1);
      }
      FUN_141ec86c0(param_1);
      local_res10 = CONCAT44(local_res10._4_4_,2);
      iVar12 = *(int *)(param_1 + 0x2d8) + 1;
      *(int *)(param_1 + 0x2d8) = iVar12;
      if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
        puVar4 = *(undefined8 **)(param_1 + 0x2e0);
        puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
        *(undefined8 **)(param_1 + 0x2e0) = puVar17;
        *puVar17 = *puVar4;
        *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar4 + 1);
        thunk_FUN_140205820(puVar4,0xc);
      }
      uVar8 = FUN_142f04924();
      *(undefined1 *)(*(longlong *)(param_1 + 0x2e0) + 4) = uVar8;
      pbVar26 = *(byte **)(param_1 + 0x2e0);
      bVar25 = pbVar26[4];
      pbVar26[8] = 0x65;
      pbVar26[9] = 0x9a;
      lVar19 = -(longlong)pbVar26;
      do {
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        bVar9 = pbVar26[(longlong)&local_res10 + lVar19];
        *pbVar26 = bVar25 ^ bVar9;
        bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
        uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8);
        *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8) =
             (uVar1 >> 0xd) + (ushort)bVar25 | uVar1 << 3;
        bVar9 = 0x2a;
        if (bVar25 != 0) {
          bVar9 = bVar25;
        }
        bVar25 = pbVar26[(longlong)&local_res10 + lVar19 + 1];
        pbVar26[1] = bVar9 ^ bVar25;
        bVar9 = (bVar9 ^ bVar25) + bVar9 + 0x2a;
        uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8);
        *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8) =
             (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
        bVar25 = 0x2a;
        if (bVar9 != 0) {
          bVar25 = bVar9;
        }
        bVar9 = pbVar26[(longlong)&local_res10 + lVar19 + 2];
        pbVar26[2] = bVar25 ^ bVar9;
        bVar9 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
        uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8);
        *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8) =
             (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
        bVar25 = 0x2a;
        if (bVar9 != 0) {
          bVar25 = bVar9;
        }
        bVar9 = pbVar26[(longlong)&local_res10 + lVar19 + 3];
        pbVar26[3] = bVar25 ^ bVar9;
        bVar25 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
        uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8);
        *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8) =
             (uVar1 >> 0xd) + (ushort)bVar25 | uVar1 << 3;
        uVar24 = uVar24 + 4;
        pbVar26 = pbVar26 + 4;
      } while (uVar24 < 4);
    }
    lVar19 = FUN_141ece7c0((longlong *)(lVar19 + param_1),local_res20[0]);
    uVar28 = DAT_143ac0188;
    local_160 = lVar19;
    if (*(int *)(lVar19 + 4) != 0) break;
    uVar13 = FUN_1409d4840(param_1 + 0x278);
    uVar3 = *(undefined8 *)(param_1 + 0x120);
    uVar14 = FUN_14019a5d0(param_1 + 0x138);
    uVar14 = FUN_142770a00(uVar3,uVar14);
    in_stack_fffffffffffffe20 = CONCAT44(uVar15,uVar13);
    FUN_140cd8da0(uVar28,*(undefined8 *)(param_1 + 0x110),uVar22,uVar14,lVar19,
                  in_stack_fffffffffffffe20);
    if (*(int *)(lVar19 + 4) != 0) break;
    iVar12 = FUN_1401b0340(param_1 + 0x308);
    if (-1 < iVar12) {
      FUN_141ebdf10(param_1);
    }
    FUN_141ec86c0(param_1);
    local_res10 = CONCAT44(local_res10._4_4_,2);
    iVar12 = *(int *)(param_1 + 0x2d8) + 1;
    *(int *)(param_1 + 0x2d8) = iVar12;
    if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
      puVar4 = *(undefined8 **)(param_1 + 0x2e0);
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 **)(param_1 + 0x2e0) = puVar17;
      *puVar17 = *puVar4;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar4 + 1);
      thunk_FUN_140205820(puVar4,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(param_1 + 0x2e0) + 4) = uVar8;
    pbVar26 = *(byte **)(param_1 + 0x2e0);
    bVar25 = pbVar26[4];
    pbVar26[8] = 0x65;
    pbVar26[9] = 0x9a;
    lVar19 = -(longlong)pbVar26;
    do {
      if (bVar25 == 0) {
        bVar25 = 0x2a;
      }
      bVar9 = pbVar26[(longlong)&local_res10 + lVar19];
      *pbVar26 = bVar25 ^ bVar9;
      bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8) = (uVar1 >> 0xd) + (ushort)bVar25 | uVar1 << 3
      ;
      bVar9 = 0x2a;
      if (bVar25 != 0) {
        bVar9 = bVar25;
      }
      bVar25 = pbVar26[(longlong)&local_res10 + lVar19 + 1];
      pbVar26[1] = bVar9 ^ bVar25;
      bVar9 = (bVar9 ^ bVar25) + bVar9 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8) = (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
      bVar25 = 0x2a;
      if (bVar9 != 0) {
        bVar25 = bVar9;
      }
      bVar9 = pbVar26[(longlong)&local_res10 + lVar19 + 2];
      pbVar26[2] = bVar25 ^ bVar9;
      bVar9 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8) = (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
      bVar25 = 0x2a;
      if (bVar9 != 0) {
        bVar25 = bVar9;
      }
      bVar9 = pbVar26[(longlong)&local_res10 + lVar19 + 3];
      pbVar26[3] = bVar25 ^ bVar9;
      bVar25 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
      uVar1 = *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8);
      *(ushort *)(*(longlong *)(param_1 + 0x2e0) + 8) = (uVar1 >> 0xd) + (ushort)bVar25 | uVar1 << 3
      ;
      uVar24 = uVar24 + 4;
      pbVar26 = pbVar26 + 4;
    } while (uVar24 < 4);
  }
  pIVar5 = *(IUnknown **)(param_1 + 0x3c8);
  if (pIVar5 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  local_180 = 3;
  uStack_178 = (byte *)CONCAT44(uStack_178._4_4_,0xfffffffe);
  local_res18 = (longlong *)0x0;
  local_138 = CONCAT22(uStack_17e,3);
  uStack_134 = uStack_17c;
  uStack_130 = 0xfffffffe;
  uStack_12c = uStack_178._4_4_;
  local_128 = local_170;
  iVar12 = (**(code **)(*(longlong *)pIVar5 + 0x268))(pIVar5,&local_138,&local_res18);
  if (iVar12 < 0) {
    _com_issue_errorex(iVar12,pIVar5,(_GUID *)&DAT_14327fcb0);
  }
  uVar24 = 0x40;
  local_res10 = CONCAT44(local_res10._4_4_,0x40);
  if (local_res18 != (longlong *)0x0) {
    (**(code **)(*local_res18 + 0x10))();
  }
  if (local_180 == 8) {
    local_180 = 0;
    if (uStack_178 != (byte *)0x0) {
      (*DAT_143ad5990)(uStack_178 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_180);
  }
  uVar7 = *(ulonglong *)(lVar19 + 8);
  iVar12 = local_168;
  local_1b0 = CONCAT22(local_1b0._2_2_,(short)local_1b0);
  while (local_168 = iVar12, uVar7 != 0) {
    uVar23 = 0;
    uVar20 = *(ulonglong *)(uVar7 - 0x20);
    if ((uVar20 != 0) && (uVar20 < 0x10001)) {
      FUN_142e52ed0(0x33e);
      uVar20 = *(ulonglong *)(uVar7 - 0x20);
    }
    local_110 = uVar23;
    if (uVar20 != 0) {
      local_110 = uVar20 + 0x28;
    }
    lVar19 = *(longlong *)(uVar7 + 8);
    local_118 = lVar19;
    iVar12 = FUN_14019a5d0(param_1 + 0x348);
    if (iVar12 < 1) {
LAB_141ec9077:
      uVar15 = *(undefined4 *)(lVar19 + 0x58);
      local_198._0_2_ = 0;
      local_198._2_2_ = 0;
      pbStack_190 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      iVar12 = (int)&local_198 + -0x3ff8;
      iStack_194 = FUN_142f04924();
      iStack_194 = iStack_194 + iVar12;
      local_188 = FUN_142f04924();
      pbVar26 = pbStack_190;
      local_188 = local_188 + iVar12;
      pbStack_190[5] = (byte)iStack_194;
      pbStack_190[6] = (byte)local_188;
      local_198 = CONCAT22(local_198._2_2_,(short)local_198) + 1;
      local_res18._0_4_ = uVar15;
      if (local_198 == (local_198 / 0x6f) * 0x6f) {
        pbStack_190 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
        *(undefined8 *)pbStack_190 = *(undefined8 *)pbVar26;
        *(undefined4 *)(pbStack_190 + 8) = *(undefined4 *)(pbVar26 + 8);
        thunk_FUN_140205820(pbVar26,0xc);
      }
      bVar25 = FUN_142f04924();
      pbStack_190[4] = bVar25;
      pbStack_190[8] = 0x65;
      pbStack_190[9] = 0x9a;
      pbVar26 = pbStack_190;
      do {
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        bVar9 = pbVar26[(longlong)local_res20 + (-8 - (longlong)pbStack_190)];
        *pbVar26 = bVar25 ^ bVar9;
        bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
        *(ushort *)(pbStack_190 + 8) =
             (*(ushort *)(pbStack_190 + 8) >> 0xd) + (ushort)bVar25 |
             *(ushort *)(pbStack_190 + 8) << 3;
        bVar9 = 0x2a;
        if (bVar25 != 0) {
          bVar9 = bVar25;
        }
        bVar25 = pbVar26[(longlong)local_res20 + (-7 - (longlong)pbStack_190)];
        pbVar26[1] = bVar9 ^ bVar25;
        bVar9 = (bVar9 ^ bVar25) + bVar9 + 0x2a;
        *(ushort *)(pbStack_190 + 8) =
             (*(ushort *)(pbStack_190 + 8) >> 0xd) + (ushort)bVar9 |
             *(ushort *)(pbStack_190 + 8) << 3;
        bVar25 = 0x2a;
        if (bVar9 != 0) {
          bVar25 = bVar9;
        }
        bVar9 = pbVar26[(longlong)local_res20 + (-6 - (longlong)pbStack_190)];
        pbVar26[2] = bVar25 ^ bVar9;
        bVar9 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
        *(ushort *)(pbStack_190 + 8) =
             (*(ushort *)(pbStack_190 + 8) >> 0xd) + (ushort)bVar9 |
             *(ushort *)(pbStack_190 + 8) << 3;
        bVar25 = 0x2a;
        if (bVar9 != 0) {
          bVar25 = bVar9;
        }
        bVar9 = pbVar26[(longlong)local_res20 + (-5 - (longlong)pbStack_190)];
        pbVar26[3] = bVar25 ^ bVar9;
        bVar25 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
        *(ushort *)(pbStack_190 + 8) =
             (*(ushort *)(pbStack_190 + 8) >> 0xd) + (ushort)bVar25 |
             *(ushort *)(pbStack_190 + 8) << 3;
        uVar22 = (int)uVar23 + 4;
        uVar23 = (ulonglong)uVar22;
        pbVar26 = pbVar26 + 4;
      } while (uVar22 < 4);
      local_res10 = CONCAT44(local_res10._4_4_,uVar24) | 2;
      uVar15 = FUN_14019a5d0(&local_198);
      local_1b0._0_2_ = 0;
      local_1b0._2_2_ = 0;
      pbStack_1a8 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      iVar12 = (int)&local_1b0 + -0x3ff8;
      iStack_1ac = FUN_142f04924();
      iStack_1ac = iStack_1ac + iVar12;
      local_1a0 = FUN_142f04924();
      pbVar26 = pbStack_1a8;
      local_1a0 = local_1a0 + iVar12;
      pbStack_1a8[5] = (byte)iStack_1ac;
      pbStack_1a8[6] = (byte)local_1a0;
      local_res18 = (longlong *)CONCAT44(local_res18._4_4_,uVar15);
      local_1b0 = CONCAT22(local_1b0._2_2_,(short)local_1b0) + 1;
      if (local_1b0 == (local_1b0 / 0x6f) * 0x6f) {
        pbStack_1a8 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
        *(undefined8 *)pbStack_1a8 = *(undefined8 *)pbVar26;
        *(undefined4 *)(pbStack_1a8 + 8) = *(undefined4 *)(pbVar26 + 8);
        thunk_FUN_140205820(pbVar26,0xc);
      }
      bVar25 = FUN_142f04924();
      pbStack_1a8[4] = bVar25;
      pbStack_1a8[8] = 0x65;
      pbStack_1a8[9] = 0x9a;
      uVar22 = 0;
      pbVar26 = pbStack_1a8;
      do {
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        bVar9 = pbVar26[(longlong)local_res20 + (-8 - (longlong)pbStack_1a8)];
        *pbVar26 = bVar25 ^ bVar9;
        bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
        *(ushort *)(pbStack_1a8 + 8) =
             (*(ushort *)(pbStack_1a8 + 8) >> 0xd) + (ushort)bVar25 |
             *(ushort *)(pbStack_1a8 + 8) << 3;
        bVar9 = 0x2a;
        if (bVar25 != 0) {
          bVar9 = bVar25;
        }
        bVar25 = pbVar26[(longlong)local_res20 + (-7 - (longlong)pbStack_1a8)];
        pbVar26[1] = bVar9 ^ bVar25;
        bVar9 = (bVar9 ^ bVar25) + bVar9 + 0x2a;
        *(ushort *)(pbStack_1a8 + 8) =
             (*(ushort *)(pbStack_1a8 + 8) >> 0xd) + (ushort)bVar9 |
             *(ushort *)(pbStack_1a8 + 8) << 3;
        bVar25 = 0x2a;
        if (bVar9 != 0) {
          bVar25 = bVar9;
        }
        bVar9 = pbVar26[(longlong)local_res20 + (-6 - (longlong)pbStack_1a8)];
        pbVar26[2] = bVar25 ^ bVar9;
        bVar9 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
        *(ushort *)(pbStack_1a8 + 8) =
             (*(ushort *)(pbStack_1a8 + 8) >> 0xd) + (ushort)bVar9 |
             *(ushort *)(pbStack_1a8 + 8) << 3;
        bVar25 = 0x2a;
        if (bVar9 != 0) {
          bVar25 = bVar9;
        }
        bVar9 = pbVar26[(longlong)local_res20 + (-5 - (longlong)pbStack_1a8)];
        pbVar26[3] = bVar25 ^ bVar9;
        bVar25 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
        *(ushort *)(pbStack_1a8 + 8) =
             (*(ushort *)(pbStack_1a8 + 8) >> 0xd) + (ushort)bVar25 |
             *(ushort *)(pbStack_1a8 + 8) << 3;
        uVar22 = uVar22 + 4;
        pbVar26 = pbVar26 + 4;
      } while (uVar22 < 4);
      puVar21 = &local_1b0;
      uVar22 = uVar24 | 6;
    }
    else {
      iVar12 = FUN_1401b0340(param_1 + 0x308);
      if (iVar12 < 0) goto LAB_141ec9077;
      uVar15 = FUN_14019a5d0(param_1 + 0x348);
      local_1c8._0_2_ = 0;
      local_1c8._2_2_ = 0;
      pbStack_1c0 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      iVar12 = (int)&local_1c8 + -0x3ff8;
      iStack_1c4 = FUN_142f04924();
      iStack_1c4 = iStack_1c4 + iVar12;
      local_1b8 = FUN_142f04924();
      pbVar26 = pbStack_1c0;
      local_1b8 = local_1b8 + iVar12;
      pbStack_1c0[5] = (byte)iStack_1c4;
      pbStack_1c0[6] = (byte)local_1b8;
      local_res10 = CONCAT44(local_res10._4_4_,uVar15);
      local_1c8 = CONCAT22(local_1c8._2_2_,(short)local_1c8) + 1;
      if (local_1c8 == (local_1c8 / 0x6f) * 0x6f) {
        pbStack_1c0 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
        *(undefined8 *)pbStack_1c0 = *(undefined8 *)pbVar26;
        *(undefined4 *)(pbStack_1c0 + 8) = *(undefined4 *)(pbVar26 + 8);
        thunk_FUN_140205820(pbVar26,0xc);
      }
      bVar25 = FUN_142f04924();
      pbStack_1c0[4] = bVar25;
      pbStack_1c0[8] = 0x65;
      pbStack_1c0[9] = 0x9a;
      pbVar26 = pbStack_1c0;
      do {
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        bVar9 = pbVar26[(longlong)&local_res10 - (longlong)pbStack_1c0];
        *pbVar26 = bVar25 ^ bVar9;
        bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
        *(ushort *)(pbStack_1c0 + 8) =
             (*(ushort *)(pbStack_1c0 + 8) >> 0xd) + (ushort)bVar25 |
             *(ushort *)(pbStack_1c0 + 8) << 3;
        bVar9 = 0x2a;
        if (bVar25 != 0) {
          bVar9 = bVar25;
        }
        bVar25 = pbVar26[(longlong)&local_res10 + (1 - (longlong)pbStack_1c0)];
        pbVar26[1] = bVar9 ^ bVar25;
        bVar9 = (bVar9 ^ bVar25) + bVar9 + 0x2a;
        *(ushort *)(pbStack_1c0 + 8) =
             (*(ushort *)(pbStack_1c0 + 8) >> 0xd) + (ushort)bVar9 |
             *(ushort *)(pbStack_1c0 + 8) << 3;
        bVar25 = 0x2a;
        if (bVar9 != 0) {
          bVar25 = bVar9;
        }
        bVar9 = pbVar26[(longlong)&local_res10 + (2 - (longlong)pbStack_1c0)];
        pbVar26[2] = bVar25 ^ bVar9;
        bVar9 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
        *(ushort *)(pbStack_1c0 + 8) =
             (*(ushort *)(pbStack_1c0 + 8) >> 0xd) + (ushort)bVar9 |
             *(ushort *)(pbStack_1c0 + 8) << 3;
        bVar25 = 0x2a;
        if (bVar9 != 0) {
          bVar25 = bVar9;
        }
        bVar9 = pbVar26[(longlong)&local_res10 + (3 - (longlong)pbStack_1c0)];
        pbVar26[3] = bVar25 ^ bVar9;
        bVar25 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
        *(ushort *)(pbStack_1c0 + 8) =
             (*(ushort *)(pbStack_1c0 + 8) >> 0xd) + (ushort)bVar25 |
             *(ushort *)(pbStack_1c0 + 8) << 3;
        uVar22 = (int)uVar23 + 4;
        uVar23 = (ulonglong)uVar22;
        pbVar26 = pbVar26 + 4;
      } while (uVar22 < 4);
      puVar21 = &local_1c8;
      uVar22 = uVar24 | 1;
    }
    param_1 = local_res8;
    local_res10 = CONCAT44(local_res10._4_4_,uVar22);
    uVar15 = FUN_14019a5d0(puVar21);
    if (((uVar22 & 4) != 0) && (uVar22 = uVar22 & 0xfffffffb, pbStack_1a8 != (byte *)0x0)) {
      thunk_FUN_140205820(pbStack_1a8,0xc);
    }
    if (((uVar22 & 2) != 0) && (uVar22 = uVar22 & 0xfffffffd, pbStack_190 != (byte *)0x0)) {
      thunk_FUN_140205820(pbStack_190,0xc);
    }
    if (((uVar22 & 1) != 0) && (uVar22 = uVar22 & 0xfffffffe, pbStack_1c0 != (byte *)0x0)) {
      thunk_FUN_140205820(pbStack_1c0,0xc);
    }
    pIVar5 = *(IUnknown **)(param_1 + 0x3c8);
    if (pIVar5 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_180);
    if (DAT_143a8b8d8 == 8) {
      if (local_180 == 8) {
        local_180 = 0;
        if (uStack_178 != (byte *)0x0) {
          (*DAT_143ad5990)(uStack_178 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_180);
        if (iVar12 < 0) goto LAB_141eca6e3;
      }
      local_180 = 8;
      if (DAT_143a8b8e0 == 0) {
        uStack_178 = (byte *)FUN_1401a5fa0(0,0);
      }
      else {
        uStack_178 = (byte *)FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
    }
    else {
      if ((local_180 == 8) && (local_180 = 0, uStack_178 != (byte *)0x0)) {
        (*DAT_143ad5990)(uStack_178 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_180,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_141eca6e3:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_1b0);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_1b0 == 8) {
        local_1b0._0_2_ = 0;
        if (pbStack_1a8 != (byte *)0x0) {
          (*DAT_143ad5990)(pbStack_1a8 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_1b0);
        if (iVar12 < 0) goto LAB_141eca6db;
      }
      local_1b0._0_2_ = 8;
      if (DAT_143a8b8e0 == 0) {
        pbStack_1a8 = (byte *)FUN_1401a5fa0(0,0);
      }
      else {
        pbStack_1a8 = (byte *)FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
    }
    else {
      iVar12 = local_1b0;
      if ((short)local_1b0 == 8) {
        local_1b0._0_2_ = 0;
        iVar12 = (uint)local_1b0._2_2_ << 0x10;
        if (pbStack_1a8 != (byte *)0x0) {
          (*DAT_143ad5990)(pbStack_1a8 + -4);
          iVar12 = CONCAT22(local_1b0._2_2_,(short)local_1b0);
        }
      }
      local_1b0 = iVar12;
      iVar12 = (*DAT_143262a28)(&local_1b0,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_141eca6db:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_198);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_198 == 8) {
        local_198._0_2_ = 0;
        if (pbStack_190 != (byte *)0x0) {
          (*DAT_143ad5990)(pbStack_190 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_198);
        if (iVar12 < 0) goto LAB_141eca6d3;
      }
      local_198._0_2_ = 8;
      if (DAT_143a8b8e0 == 0) {
        pbStack_190 = (byte *)FUN_1401a5fa0(0,0);
      }
      else {
        pbStack_190 = (byte *)FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
    }
    else {
      iVar12 = local_198;
      if ((short)local_198 == 8) {
        local_198._0_2_ = 0;
        iVar12 = (uint)local_198._2_2_ << 0x10;
        if (pbStack_190 != (byte *)0x0) {
          (*DAT_143ad5990)(pbStack_190 + -4);
          iVar12 = CONCAT22(local_198._2_2_,(short)local_198);
        }
      }
      local_198 = iVar12;
      iVar12 = (*DAT_143262a28)(&local_198,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_141eca6d3:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_1c8);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_1c8 == 8) {
        local_1c8._0_2_ = 0;
        if (pbStack_1c0 != (byte *)0x0) {
          (*DAT_143ad5990)(pbStack_1c0 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_1c8);
        if (iVar12 < 0) goto LAB_141eca6cb;
      }
      local_1c8._0_2_ = 8;
      if (DAT_143a8b8e0 == 0) {
        pbStack_1c0 = (byte *)FUN_1401a5fa0(0,0);
      }
      else {
        pbStack_1c0 = (byte *)FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
    }
    else {
      iVar12 = local_1c8;
      if ((short)local_1c8 == 8) {
        local_1c8._0_2_ = 0;
        iVar12 = (uint)local_1c8._2_2_ << 0x10;
        if (pbStack_1c0 != (byte *)0x0) {
          (*DAT_143ad5990)(pbStack_1c0 + -4);
          iVar12 = CONCAT22(local_1c8._2_2_,(short)local_1c8);
        }
      }
      local_1c8 = iVar12;
      iVar12 = (*DAT_143262a28)(&local_1c8,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_141eca6cb:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    local_158 = CONCAT22(local_158._2_2_,3);
    uVar28 = *(undefined8 *)(local_118 + 0x20);
    uStack_150 = uVar15;
    (*DAT_143262a20)(&local_138);
    local_a8 = CONCAT44(uStack_17c,CONCAT22(uStack_17e,local_180));
    pbStack_a0 = uStack_178;
    local_98 = local_170;
    local_88 = CONCAT44(iStack_1ac,local_1b0);
    pbStack_80 = pbStack_1a8;
    local_78 = CONCAT44(uStack_19c,local_1a0);
    local_68 = CONCAT44(iStack_194,local_198);
    pbStack_60 = pbStack_190;
    local_58 = CONCAT44(uStack_184,local_188);
    local_108 = CONCAT44(iStack_1c4,local_1c8);
    pbStack_100 = pbStack_1c0;
    local_f8 = CONCAT44(uStack_1b4,local_1b8);
    local_e8 = CONCAT44(uStack_154,local_158);
    uStack_e0 = (byte *)CONCAT44(uStack_14c,uStack_150);
    local_d8 = local_148;
    iVar12 = (**(code **)(*(longlong *)pIVar5 + 600))
                       (pIVar5,uVar28,&local_e8,&local_108,&local_68,&local_88,&local_a8,&local_138)
    ;
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar5,(_GUID *)&DAT_14327fcb0);
    }
    local_c8 = local_138;
    uStack_c4 = uStack_134;
    uStack_c0 = uStack_130;
    uStack_bc = uStack_12c;
    local_b8 = local_128;
    uVar24 = uVar22 | 0x80;
    local_res10 = CONCAT44(local_res10._4_4_,uVar22) | 0x80;
    if ((short)local_138 == 8) {
      local_c8 = local_138 & 0xffff0000;
      if (CONCAT44(uStack_12c,uStack_130) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_12c,uStack_130) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_c8);
    }
    if ((short)local_158 == 8) {
      local_158 = local_158 & 0xffff0000;
      iVar12 = local_1c8;
      if (CONCAT44(uStack_14c,uStack_150) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
        iVar12 = local_1c8;
      }
    }
    else {
      (*DAT_143262a18)(&local_158);
      iVar12 = local_1c8;
    }
    local_1c8._2_2_ = (ushort)((uint)iVar12 >> 0x10);
    local_1c8._0_2_ = (short)iVar12;
    if ((short)local_1c8 == 8) {
      local_1c8._0_2_ = 0;
      iVar12 = (uint)local_1c8._2_2_ << 0x10;
      iVar16 = local_198;
      if (pbStack_1c0 != (byte *)0x0) {
        (*DAT_143ad5990)(pbStack_1c0 + -4);
        iVar12 = CONCAT22(local_1c8._2_2_,(short)local_1c8);
        iVar16 = local_198;
      }
    }
    else {
      local_1c8 = iVar12;
      (*DAT_143262a18)(&local_1c8);
      iVar12 = local_1c8;
      iVar16 = local_198;
    }
    local_198._2_2_ = (ushort)((uint)iVar16 >> 0x10);
    local_198._0_2_ = (short)iVar16;
    local_1c8 = iVar12;
    if ((short)local_198 == 8) {
      local_198._0_2_ = 0;
      iVar12 = local_1b0;
      iVar16 = (uint)local_198._2_2_ << 0x10;
      if (pbStack_190 != (byte *)0x0) {
        (*DAT_143ad5990)(pbStack_190 + -4);
        iVar12 = local_1b0;
        iVar16 = CONCAT22(local_198._2_2_,(short)local_198);
      }
    }
    else {
      local_198 = iVar16;
      (*DAT_143262a18)(&local_198);
      iVar12 = local_1b0;
      iVar16 = local_198;
    }
    local_1b0._2_2_ = (ushort)((uint)iVar12 >> 0x10);
    local_1b0._0_2_ = (short)iVar12;
    local_198 = iVar16;
    if ((short)local_1b0 == 8) {
      local_1b0._0_2_ = 0;
      iVar12 = (uint)local_1b0._2_2_ << 0x10;
      if (pbStack_1a8 != (byte *)0x0) {
        (*DAT_143ad5990)(pbStack_1a8 + -4);
        iVar12 = CONCAT22(local_1b0._2_2_,(short)local_1b0);
      }
    }
    else {
      local_1b0 = iVar12;
      (*DAT_143262a18)(&local_1b0);
      iVar12 = local_1b0;
    }
    local_1b0 = iVar12;
    if (local_180 == 8) {
      local_180 = 0;
      uVar7 = local_110;
      lVar19 = local_160;
      iVar12 = local_168;
      if (uStack_178 != (byte *)0x0) {
        (*DAT_143ad5990)(uStack_178 + -4);
        uVar7 = local_110;
        lVar19 = local_160;
        iVar12 = local_168;
      }
    }
    else {
      (*DAT_143262a18)(&local_180);
      uVar7 = local_110;
      lVar19 = local_160;
      iVar12 = local_168;
    }
  }
  if (iVar12 != 100) {
    pIVar5 = *(IUnknown **)(param_1 + 0x3c8);
    if (pIVar5 == (IUnknown *)0x0) goto LAB_141eca6a5;
    iVar16 = (**(code **)(*(longlong *)pIVar5 + 0x300))(pIVar5,2);
    if (iVar16 < 0) {
      _com_issue_errorex(iVar16,pIVar5,(_GUID *)&DAT_14327fcb0);
    }
    pIVar5 = *(IUnknown **)(param_1 + 0x3c8);
    if (pIVar5 == (IUnknown *)0x0) goto LAB_141eca6a5;
    iVar12 = (**(code **)(*(longlong *)pIVar5 + 800))(pIVar5,8,iVar12);
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar5,(_GUID *)&DAT_14327fcb0);
    }
  }
  iVar12 = local_1c8;
  if (*(longlong *)(param_1 + 0x650) != 0) {
    cVar10 = FUN_142104aa0();
    iVar12 = local_1c8;
    if (cVar10 != '\0') {
      pIVar5 = *(IUnknown **)(param_1 + 0x3c8);
      if (pIVar5 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_1c8);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_1c8 == 8) {
          local_1c8._0_2_ = 0;
          if (pbStack_1c0 != (byte *)0x0) {
            (*DAT_143ad5990)(pbStack_1c0 + -4);
          }
        }
        else {
          iVar12 = (*DAT_143262a18)(&local_1c8);
          if (iVar12 < 0) goto LAB_141eca6b0;
        }
        local_1c8._0_2_ = 8;
        if (DAT_143a8b8e0 == 0) {
          uVar22 = 0;
        }
        else {
          uVar22 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        pbStack_1c0 = (byte *)FUN_1401a5fa0(DAT_143a8b8e0,uVar22);
      }
      else {
        iVar12 = local_1c8;
        if ((short)local_1c8 == 8) {
          local_1c8._0_2_ = 0;
          iVar12 = (uint)local_1c8._2_2_ << 0x10;
          if (pbStack_1c0 != (byte *)0x0) {
            (*DAT_143ad5990)(pbStack_1c0 + -4);
            iVar12 = CONCAT22(local_1c8._2_2_,(short)local_1c8);
          }
        }
        local_1c8 = iVar12;
        iVar12 = (*DAT_143262a28)(&local_1c8,&DAT_143a8b8d8);
        if (iVar12 < 0) {
LAB_141eca6b0:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar12);
        }
      }
      (*DAT_143262a20)(&local_180);
      if (DAT_143a8b8d8 == 8) {
        if (local_180 == 8) {
          local_180 = 0;
          if (uStack_178 != (byte *)0x0) {
            (*DAT_143ad5990)(uStack_178 + -4);
          }
        }
        else {
          iVar12 = (*DAT_143262a18)(&local_180);
          if (iVar12 < 0) goto LAB_141eca6b8;
        }
        local_180 = 8;
        if (DAT_143a8b8e0 == 0) {
          uVar22 = 0;
        }
        else {
          uVar22 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        uStack_178 = (byte *)FUN_1401a5fa0(DAT_143a8b8e0,uVar22);
      }
      else {
        if ((local_180 == 8) && (local_180 = 0, uStack_178 != (byte *)0x0)) {
          (*DAT_143ad5990)(uStack_178 + -4);
        }
        iVar12 = (*DAT_143262a28)(&local_180,&DAT_143a8b8d8);
        if (iVar12 < 0) {
LAB_141eca6b8:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar12);
        }
      }
      local_e8 = CONCAT44(iStack_1c4,local_1c8);
      uStack_e0 = pbStack_1c0;
      local_d8 = CONCAT44(uStack_1b4,local_1b8);
      local_108 = CONCAT44(uStack_17c,CONCAT22(uStack_17e,local_180));
      pbStack_100 = uStack_178;
      local_f8 = local_170;
      iVar12 = (**(code **)(*(longlong *)pIVar5 + 0x280))(pIVar5,0x20,&local_108,&local_e8);
      if (iVar12 < 0) {
        _com_issue_errorex(iVar12,pIVar5,(_GUID *)&DAT_14327fcb0);
      }
      if (local_180 == 8) {
        local_180 = 0;
        iVar12 = local_1c8;
        if (uStack_178 != (byte *)0x0) {
          (*DAT_143ad5990)(uStack_178 + -4);
          iVar12 = local_1c8;
        }
      }
      else {
        (*DAT_143262a18)(&local_180);
        iVar12 = local_1c8;
      }
      local_1c8._2_2_ = (ushort)((uint)iVar12 >> 0x10);
      local_1c8._0_2_ = (short)iVar12;
      if ((short)local_1c8 == 8) {
        local_1c8._0_2_ = 0;
        iVar12 = (uint)local_1c8._2_2_ << 0x10;
        if (pbStack_1c0 != (byte *)0x0) {
          (*DAT_143ad5990)(pbStack_1c0 + -4);
          iVar12 = CONCAT22(local_1c8._2_2_,(short)local_1c8);
        }
      }
      else {
        local_1c8 = iVar12;
        (*DAT_143262a18)(&local_1c8);
        iVar12 = local_1c8;
      }
    }
  }
  *(undefined8 *)(param_1 + 0x328) = *(undefined8 *)(lVar19 + 8);
  lVar19 = *(longlong *)(*(longlong *)(lVar19 + 8) + 8);
  local_1c8 = iVar12;
  iVar12 = FUN_14019a5d0(param_1 + 0x348);
  if (iVar12 < 1) {
LAB_141ec9f7d:
    uVar15 = *(undefined4 *)(lVar19 + 0x58);
    local_198._0_2_ = 0;
    local_198._2_2_ = 0;
    pbStack_190 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
    iVar12 = (int)&local_198 + -0x3ff8;
    iStack_194 = FUN_142f04924();
    iStack_194 = iStack_194 + iVar12;
    local_188 = FUN_142f04924();
    pbVar26 = pbStack_190;
    local_188 = local_188 + iVar12;
    pbStack_190[5] = (byte)iStack_194;
    pbStack_190[6] = (byte)local_188;
    local_198 = CONCAT22(local_198._2_2_,(short)local_198) + 1;
    local_res18._0_4_ = uVar15;
    if (local_198 == (local_198 / 0x6f) * 0x6f) {
      pbStack_190 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 *)pbStack_190 = *(undefined8 *)pbVar26;
      *(undefined4 *)(pbStack_190 + 8) = *(undefined4 *)(pbVar26 + 8);
      thunk_FUN_140205820(pbVar26,0xc);
    }
    bVar25 = FUN_142f04924();
    pbStack_190[4] = bVar25;
    pbStack_190[8] = 0x65;
    pbStack_190[9] = 0x9a;
    uVar22 = 0;
    pbVar26 = pbStack_190;
    do {
      if (bVar25 == 0) {
        bVar25 = 0x2a;
      }
      bVar9 = pbVar26[(longlong)local_res20 + (-8 - (longlong)pbStack_190)];
      *pbVar26 = bVar25 ^ bVar9;
      bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
      *(ushort *)(pbStack_190 + 8) =
           (*(ushort *)(pbStack_190 + 8) >> 0xd) + (ushort)bVar25 |
           *(ushort *)(pbStack_190 + 8) << 3;
      bVar9 = 0x2a;
      if (bVar25 != 0) {
        bVar9 = bVar25;
      }
      bVar25 = pbVar26[(longlong)local_res20 + (-7 - (longlong)pbStack_190)];
      pbVar26[1] = bVar9 ^ bVar25;
      bVar9 = (bVar9 ^ bVar25) + bVar9 + 0x2a;
      *(ushort *)(pbStack_190 + 8) =
           (*(ushort *)(pbStack_190 + 8) >> 0xd) + (ushort)bVar9 | *(ushort *)(pbStack_190 + 8) << 3
      ;
      bVar25 = 0x2a;
      if (bVar9 != 0) {
        bVar25 = bVar9;
      }
      bVar9 = pbVar26[(longlong)local_res20 + (-6 - (longlong)pbStack_190)];
      pbVar26[2] = bVar25 ^ bVar9;
      bVar9 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
      *(ushort *)(pbStack_190 + 8) =
           (*(ushort *)(pbStack_190 + 8) >> 0xd) + (ushort)bVar9 | *(ushort *)(pbStack_190 + 8) << 3
      ;
      bVar25 = 0x2a;
      if (bVar9 != 0) {
        bVar25 = bVar9;
      }
      bVar9 = pbVar26[(longlong)local_res20 + (-5 - (longlong)pbStack_190)];
      pbVar26[3] = bVar25 ^ bVar9;
      bVar25 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
      *(ushort *)(pbStack_190 + 8) =
           (*(ushort *)(pbStack_190 + 8) >> 0xd) + (ushort)bVar25 |
           *(ushort *)(pbStack_190 + 8) << 3;
      uVar22 = uVar22 + 4;
      pbVar26 = pbVar26 + 4;
    } while (uVar22 < 4);
    local_res10 = CONCAT44(local_res10._4_4_,uVar24) | 0x10;
    uVar15 = FUN_14019a5d0(&local_198);
    local_1c8._0_2_ = 0;
    local_1c8._2_2_ = 0;
    pbStack_1c0 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
    iVar12 = (int)&local_1c8 + -0x3ff8;
    iStack_1c4 = FUN_142f04924();
    iStack_1c4 = iStack_1c4 + iVar12;
    local_1b8 = FUN_142f04924();
    pbVar26 = pbStack_1c0;
    local_1b8 = local_1b8 + iVar12;
    pbStack_1c0[5] = (byte)iStack_1c4;
    pbStack_1c0[6] = (byte)local_1b8;
    local_res18 = (longlong *)CONCAT44(local_res18._4_4_,uVar15);
    local_1c8 = CONCAT22(local_1c8._2_2_,(short)local_1c8) + 1;
    if (local_1c8 == (local_1c8 / 0x6f) * 0x6f) {
      pbStack_1c0 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 *)pbStack_1c0 = *(undefined8 *)pbVar26;
      *(undefined4 *)(pbStack_1c0 + 8) = *(undefined4 *)(pbVar26 + 8);
      thunk_FUN_140205820(pbVar26,0xc);
    }
    bVar25 = FUN_142f04924();
    pbStack_1c0[4] = bVar25;
    pbStack_1c0[8] = 0x65;
    pbStack_1c0[9] = 0x9a;
    uVar22 = 0;
    pbVar26 = pbStack_1c0;
    do {
      if (bVar25 == 0) {
        bVar25 = 0x2a;
      }
      bVar9 = pbVar26[(longlong)local_res20 + (-8 - (longlong)pbStack_1c0)];
      *pbVar26 = bVar25 ^ bVar9;
      bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
      *(ushort *)(pbStack_1c0 + 8) =
           (*(ushort *)(pbStack_1c0 + 8) >> 0xd) + (ushort)bVar25 |
           *(ushort *)(pbStack_1c0 + 8) << 3;
      bVar9 = 0x2a;
      if (bVar25 != 0) {
        bVar9 = bVar25;
      }
      bVar25 = pbVar26[(longlong)local_res20 + (-7 - (longlong)pbStack_1c0)];
      pbVar26[1] = bVar9 ^ bVar25;
      bVar9 = bVar9 + (bVar9 ^ bVar25) + 0x2a;
      *(ushort *)(pbStack_1c0 + 8) =
           (*(ushort *)(pbStack_1c0 + 8) >> 0xd) + (ushort)bVar9 | *(ushort *)(pbStack_1c0 + 8) << 3
      ;
      bVar25 = 0x2a;
      if (bVar9 != 0) {
        bVar25 = bVar9;
      }
      bVar9 = pbVar26[(longlong)local_res20 + (-6 - (longlong)pbStack_1c0)];
      pbVar26[2] = bVar25 ^ bVar9;
      bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
      *(ushort *)(pbStack_1c0 + 8) =
           (*(ushort *)(pbStack_1c0 + 8) >> 0xd) + (ushort)bVar25 |
           *(ushort *)(pbStack_1c0 + 8) << 3;
      bVar9 = 0x2a;
      if (bVar25 != 0) {
        bVar9 = bVar25;
      }
      bVar25 = pbVar26[(longlong)local_res20 + (-5 - (longlong)pbStack_1c0)];
      pbVar26[3] = bVar9 ^ bVar25;
      bVar25 = bVar9 + 0x2a + (bVar9 ^ bVar25);
      *(ushort *)(pbStack_1c0 + 8) =
           (*(ushort *)(pbStack_1c0 + 8) >> 0xd) + (ushort)bVar25 |
           *(ushort *)(pbStack_1c0 + 8) << 3;
      uVar22 = uVar22 + 4;
      pbVar26 = pbVar26 + 4;
    } while (uVar22 < 4);
    puVar21 = &local_1c8;
    uVar24 = uVar24 | 0x30;
  }
  else {
    iVar12 = FUN_1401b0340(param_1 + 0x308);
    if (iVar12 < 0) goto LAB_141ec9f7d;
    uVar15 = FUN_14019a5d0(param_1 + 0x348);
    local_1b0._0_2_ = 0;
    local_1b0._2_2_ = 0;
    pbStack_1a8 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
    iVar12 = (int)&local_1b0 + -0x3ff8;
    iStack_1ac = FUN_142f04924();
    iStack_1ac = iStack_1ac + iVar12;
    local_1a0 = FUN_142f04924();
    pbVar26 = pbStack_1a8;
    local_1a0 = local_1a0 + iVar12;
    pbStack_1a8[5] = (byte)iStack_1ac;
    pbStack_1a8[6] = (byte)local_1a0;
    local_res10 = CONCAT44(local_res10._4_4_,uVar15);
    local_1b0 = CONCAT22(local_1b0._2_2_,(short)local_1b0) + 1;
    if (local_1b0 == (local_1b0 / 0x6f) * 0x6f) {
      pbStack_1a8 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 *)pbStack_1a8 = *(undefined8 *)pbVar26;
      *(undefined4 *)(pbStack_1a8 + 8) = *(undefined4 *)(pbVar26 + 8);
      thunk_FUN_140205820(pbVar26,0xc);
    }
    bVar25 = FUN_142f04924();
    pbStack_1a8[4] = bVar25;
    pbStack_1a8[8] = 0x65;
    pbStack_1a8[9] = 0x9a;
    uVar22 = 0;
    pbVar26 = pbStack_1a8;
    do {
      if (bVar25 == 0) {
        bVar25 = 0x2a;
      }
      bVar9 = pbVar26[(longlong)&local_res10 - (longlong)pbStack_1a8];
      *pbVar26 = bVar25 ^ bVar9;
      bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
      *(ushort *)(pbStack_1a8 + 8) =
           (*(ushort *)(pbStack_1a8 + 8) >> 0xd) + (ushort)bVar25 |
           *(ushort *)(pbStack_1a8 + 8) << 3;
      bVar9 = 0x2a;
      if (bVar25 != 0) {
        bVar9 = bVar25;
      }
      bVar25 = pbVar26[(longlong)&local_res10 + (1 - (longlong)pbStack_1a8)];
      pbVar26[1] = bVar9 ^ bVar25;
      bVar9 = (bVar9 ^ bVar25) + bVar9 + 0x2a;
      *(ushort *)(pbStack_1a8 + 8) =
           (*(ushort *)(pbStack_1a8 + 8) >> 0xd) + (ushort)bVar9 | *(ushort *)(pbStack_1a8 + 8) << 3
      ;
      bVar25 = 0x2a;
      if (bVar9 != 0) {
        bVar25 = bVar9;
      }
      bVar9 = pbVar26[(longlong)&local_res10 + (2 - (longlong)pbStack_1a8)];
      pbVar26[2] = bVar25 ^ bVar9;
      bVar9 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
      *(ushort *)(pbStack_1a8 + 8) =
           (*(ushort *)(pbStack_1a8 + 8) >> 0xd) + (ushort)bVar9 | *(ushort *)(pbStack_1a8 + 8) << 3
      ;
      bVar25 = 0x2a;
      if (bVar9 != 0) {
        bVar25 = bVar9;
      }
      bVar9 = pbVar26[(longlong)&local_res10 + (3 - (longlong)pbStack_1a8)];
      pbVar26[3] = bVar25 ^ bVar9;
      bVar25 = (bVar25 ^ bVar9) + bVar25 + 0x2a;
      *(ushort *)(pbStack_1a8 + 8) =
           (*(ushort *)(pbStack_1a8 + 8) >> 0xd) + (ushort)bVar25 |
           *(ushort *)(pbStack_1a8 + 8) << 3;
      uVar22 = uVar22 + 4;
      pbVar26 = pbVar26 + 4;
    } while (uVar22 < 4);
    puVar21 = &local_1b0;
    uVar24 = uVar24 | 8;
  }
  lVar19 = local_res8;
  local_res10 = CONCAT44(local_res10._4_4_,uVar24);
  local_res18 = (longlong *)CONCAT44(local_res18._4_4_,uVar24);
  uVar15 = FUN_14019a5d0(puVar21);
  local_res8 = CONCAT44(local_res8._4_4_,uVar15);
  iVar12 = *(int *)(lVar19 + 0x330) + 1;
  *(int *)(lVar19 + 0x330) = iVar12;
  if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
    puVar4 = *(undefined8 **)(lVar19 + 0x338);
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(lVar19 + 0x338) = puVar17;
    *puVar17 = *puVar4;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar4 + 1);
    thunk_FUN_140205820(puVar4,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(*(longlong *)(lVar19 + 0x338) + 4) = uVar8;
  pbVar26 = *(byte **)(lVar19 + 0x338);
  bVar25 = pbVar26[4];
  pbVar26[8] = 0x65;
  pbVar26[9] = 0x9a;
  uVar24 = 0;
  lVar27 = (longlong)&local_res8 - (longlong)pbVar26;
  lVar2 = 1 - (longlong)pbVar26;
  local_res10 = (longlong)&local_res8 + (2 - (longlong)pbVar26);
  lVar6 = 3 - (longlong)pbVar26;
  do {
    if (bVar25 == 0) {
      bVar25 = 0x2a;
    }
    bVar9 = pbVar26[lVar27];
    *pbVar26 = bVar25 ^ bVar9;
    bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(lVar19 + 0x338) + 8);
    *(ushort *)(*(longlong *)(lVar19 + 0x338) + 8) = (uVar1 >> 0xd) + (ushort)bVar25 | uVar1 << 3;
    bVar9 = 0x2a;
    if (bVar25 != 0) {
      bVar9 = bVar25;
    }
    bVar25 = pbVar26[(longlong)&local_res8 + lVar2];
    pbVar26[1] = bVar9 ^ bVar25;
    bVar9 = bVar9 + (bVar9 ^ bVar25) + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(lVar19 + 0x338) + 8);
    *(ushort *)(*(longlong *)(lVar19 + 0x338) + 8) = (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
    bVar18 = 0x2a;
    if (bVar9 != 0) {
      bVar18 = bVar9;
    }
    bVar25 = pbVar26[local_res10];
    pbVar26[2] = bVar18 ^ bVar25;
    bVar18 = bVar18 + (bVar18 ^ bVar25) + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(lVar19 + 0x338) + 8);
    *(ushort *)(*(longlong *)(lVar19 + 0x338) + 8) = (uVar1 >> 0xd) + (ushort)bVar18 | uVar1 << 3;
    bVar25 = 0x2a;
    if (bVar18 != 0) {
      bVar25 = bVar18;
    }
    bVar9 = pbVar26[(longlong)&local_res8 + lVar6];
    pbVar26[3] = bVar25 ^ bVar9;
    bVar25 = bVar25 + (bVar25 ^ bVar9) + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(lVar19 + 0x338) + 8);
    *(ushort *)(*(longlong *)(lVar19 + 0x338) + 8) = (uVar1 >> 0xd) + (ushort)bVar25 | uVar1 << 3;
    uVar24 = uVar24 + 4;
    pbVar26 = pbVar26 + 4;
  } while (uVar24 < 4);
  uVar28 = 0;
  uVar24 = (uint)local_res18;
  if ((((ulonglong)local_res18 & 0x20) != 0) &&
     (uVar24 = (uint)local_res18 & 0xffffffdf, pbStack_1c0 != (byte *)0x0)) {
    thunk_FUN_140205820(pbStack_1c0,0xc);
  }
  if (((uVar24 & 0x10) != 0) && (uVar24 = uVar24 & 0xffffffef, pbStack_190 != (byte *)0x0)) {
    thunk_FUN_140205820(pbStack_190,0xc);
  }
  if (((uVar24 & 8) != 0) && (pbStack_1a8 != (byte *)0x0)) {
    thunk_FUN_140205820(pbStack_1a8,0xc);
  }
  pIVar5 = *(IUnknown **)(lVar19 + 0x3c8);
  if (pIVar5 != (IUnknown *)0x0) {
    if ((local_164 == 0) && (local_res20[0] != 8)) {
      uVar28 = 1;
    }
    iVar12 = (**(code **)(*(longlong *)pIVar5 + 0x1e0))(pIVar5,uVar28);
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar5,(_GUID *)&DAT_14327fcb0);
    }
    FUN_141ece820(lVar19,lVar19 + 0x550);
    return;
  }
LAB_141eca6a5:
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}


