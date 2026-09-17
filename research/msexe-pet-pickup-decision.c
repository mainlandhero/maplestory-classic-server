
//===========================================================
// FUN_14179e990 @ 14179e990   (12951 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14179e990(longlong param_1,undefined8 param_2,int *param_3,longlong param_4)

{
  undefined4 *puVar1;
  longlong lVar2;
  uint *puVar3;
  longlong lVar4;
  undefined8 uVar5;
  char cVar6;
  undefined1 uVar7;
  byte bVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined8 uVar11;
  int *piVar12;
  longlong lVar13;
  undefined8 *puVar14;
  undefined8 *puVar15;
  longlong lVar16;
  byte bVar17;
  int iVar18;
  ushort uVar19;
  ushort uVar20;
  uint uVar21;
  longlong lVar22;
  byte *pbVar23;
  longlong lVar24;
  byte bVar25;
  ushort uVar26;
  ushort uVar27;
  ushort uVar28;
  ushort uVar29;
  int iVar30;
  byte *pbVar31;
  ulonglong uVar32;
  undefined1 auStack_268 [32];
  byte *local_248;
  ushort *local_240;
  ushort *local_238;
  undefined1 *local_230;
  longlong local_228;
  undefined1 *local_220;
  longlong local_218;
  undefined4 local_208;
  byte local_200;
  byte local_1ff;
  byte local_1fe;
  byte local_1fd;
  byte local_1fc;
  byte local_1fb;
  byte local_1fa;
  byte local_1f9;
  byte local_1f8;
  byte local_1f7 [7];
  undefined4 local_1f0;
  ushort local_1e8 [2];
  ushort local_1e4 [2];
  ushort local_1e0 [2];
  ushort local_1dc [2];
  ushort local_1d8 [2];
  ushort local_1d4 [2];
  ushort local_1d0 [2];
  ushort local_1cc [2];
  ushort local_1c8 [2];
  ushort local_1c4 [2];
  undefined8 local_1c0;
  undefined1 local_1b8;
  undefined1 local_1b7;
  undefined1 local_1b6;
  undefined1 local_1b5;
  undefined1 local_1b4;
  undefined1 local_1b3;
  undefined1 local_1b2;
  undefined1 local_1b1;
  undefined1 local_1b0;
  undefined1 local_1af;
  undefined1 local_1ae;
  undefined1 local_1ad;
  undefined1 local_1ac;
  undefined1 local_1ab;
  undefined1 local_1aa;
  undefined1 local_1a9;
  undefined1 local_1a8;
  undefined1 local_1a7;
  undefined1 local_1a6;
  undefined1 local_1a5;
  undefined1 local_1a4 [4];
  longlong local_1a0;
  undefined1 local_198 [4];
  int local_194;
  undefined4 local_190;
  undefined4 local_18c;
  undefined4 local_188;
  undefined4 local_184;
  undefined4 local_180;
  undefined4 local_17c;
  undefined4 local_178;
  undefined4 local_174;
  undefined4 local_170;
  undefined4 local_16c;
  longlong local_168;
  undefined8 local_160;
  int *local_158;
  longlong local_150;
  undefined8 local_148;
  longlong local_140;
  longlong local_138;
  undefined8 local_130;
  longlong local_128;
  longlong local_120;
  undefined8 local_118;
  longlong local_110;
  longlong local_108;
  undefined8 local_100;
  longlong local_f8;
  ulonglong local_f0;
  undefined8 local_e8;
  longlong local_e0;
  ulonglong local_d8;
  undefined8 local_d0;
  longlong local_c8;
  longlong local_c0;
  undefined8 local_b8;
  longlong local_b0;
  ulonglong local_a8;
  undefined8 local_a0;
  longlong local_98;
  longlong local_90;
  undefined8 local_88;
  longlong local_80;
  ulonglong local_78;
  undefined8 local_70;
  longlong local_68;
  longlong local_60 [2];
  int local_50;
  int iStack_4c;
  int iStack_48;
  int iStack_44;
  ulonglong local_40;
  
  uVar5 = DAT_143aa8328;
  local_40 = DAT_143a8b908 ^ (ulonglong)auStack_268;
  local_160 = param_2;
  local_158 = param_3;
  local_68 = param_4;
  uVar11 = FUN_141ebdad0(param_2);
  uVar9 = FUN_14038a5b0(uVar5,uVar11);
  cVar6 = FUN_140374c80(uVar9);
  local_50 = _DAT_14327c630;
  iStack_4c = _UNK_14327c634;
  iStack_48 = _UNK_14327c638;
  iStack_44 = _UNK_14327c63c;
  if (cVar6 != '\0') {
    piVar12 = (int *)FUN_14113d940(local_60,uVar9);
    local_50 = *piVar12;
    iStack_4c = piVar12[1];
    iStack_48 = piVar12[2];
    iStack_44 = piVar12[3];
  }
  local_50 = local_50 + *param_3;
  iStack_48 = iStack_48 + *param_3;
  iStack_4c = iStack_4c + param_3[1];
  iStack_44 = iStack_44 + param_3[1];
  local_194 = FUN_1429e3ef0();
  FUN_142d205e0(&local_1b8,0,0xed);
  lVar16 = *(longlong *)(param_1 + 0x48);
  do {
    local_1a0 = lVar16;
    if (lVar16 == 0) {
LAB_1417a1b59:
      lVar16 = local_68;
      iVar18 = FUN_1401b0340(local_68 + 0x18);
      if ((*param_3 == iVar18) && (iVar18 = FUN_1401b0340(lVar16), param_3[1] == iVar18)) {
        FUN_142d205e0(&local_1b7,0,0x140);
      }
      return;
    }
    if ((((*(longlong *)(lVar16 + 8) != 0) && (*(int *)(*(longlong *)(lVar16 + 8) + 0x160) != 0)) &&
        (local_168 = DAT_143aa8518, DAT_143aa8518 != 0)) &&
       (lVar13 = FUN_142770440(DAT_143aa8518), lVar13 != 0)) {
      lVar13 = *(longlong *)(lVar16 + 8);
      if (lVar13 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar13 = *(longlong *)(lVar16 + 8);
      }
      puVar1 = *(undefined4 **)(lVar13 + 0x80);
      local_1c0 = CONCAT44(local_1c0._4_4_,*puVar1);
      local_200 = *(byte *)(puVar1 + 1);
      uVar20 = 0x9a65;
      local_1e0[0] = 0x9a65;
      uVar21 = 0;
      pbVar31 = (byte *)&local_1c0;
      pbVar23 = (byte *)((longlong)puVar1 + 2);
      do {
        if (local_200 == 0) {
          local_200 = 0x2a;
          local_200 = 0x2a;
        }
        *pbVar31 = local_200 ^ pbVar31[(longlong)puVar1 - (longlong)&local_1c0];
        bVar25 = local_200 + 0x2a + pbVar31[(longlong)puVar1 - (longlong)&local_1c0];
        uVar26 = (uVar20 >> 0xd) + (ushort)bVar25;
        uVar19 = uVar20 << 3;
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        pbVar31[1] = bVar25 ^ pbVar23[-1];
        bVar25 = bVar25 + 0x2a + pbVar23[-1];
        uVar28 = (ushort)bVar25;
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        pbVar31[2] = bVar25 ^ *pbVar23;
        bVar25 = bVar25 + 0x2a + *pbVar23;
        uVar27 = (ushort)bVar25;
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        pbVar31[3] = bVar25 ^ pbVar23[1];
        local_200 = bVar25 + 0x2a + pbVar23[1];
        uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)local_200 |
                 (((uVar19 & 0x1fff) >> 10) + uVar27 |
                 (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3) << 3;
        local_1e0[0] = uVar20;
        uVar21 = uVar21 + 4;
        pbVar31 = pbVar31 + 4;
        pbVar23 = pbVar23 + 4;
      } while (uVar21 < 4);
      FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e27);
      lVar2 = *(longlong *)(lVar13 + 0x80);
      uVar21 = (uint)local_1c0;
      if (((local_1e0[0] != *(ushort *)(lVar2 + 8)) ||
          (*(char *)(lVar13 + 0x7c) != *(char *)(lVar2 + 5))) ||
         (*(char *)(lVar13 + 0x88) != *(char *)(lVar2 + 6))) {
        local_1b6 = *(undefined1 *)(lVar13 + 0x88);
        local_1b5 = *(undefined1 *)(lVar13 + 0x7c);
        local_16c = 2;
        local_150 = (longlong)(int)(uint)local_1c0;
        local_148 = FUN_1418039d0(5);
        local_220 = &local_1b6;
        local_230 = &local_1b5;
        local_240 = local_1e0;
        local_248 = &local_200;
        local_238 = (ushort *)(lVar2 + 8);
        local_228 = lVar2 + 5;
        local_218 = lVar2 + 6;
        puVar14 = (undefined8 *)FUN_140197ac0(&local_140,&local_148,&local_16c,&local_150);
        FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
        if (local_140 != 0) {
          FUN_14019f2c0(local_140 + -0x10);
        }
      }
      iVar18 = *(int *)(lVar13 + 0x78);
      iVar30 = iVar18 + 1;
      *(int *)(lVar13 + 0x78) = iVar30;
      if (iVar30 == (iVar30 / 0x37) * 0x37) {
        local_208 = uVar21;
        iVar18 = iVar18 + 2;
        *(int *)(lVar13 + 0x78) = iVar18;
        if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
          puVar14 = *(undefined8 **)(lVar13 + 0x80);
          puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
          *(undefined8 **)(lVar13 + 0x80) = puVar15;
          *puVar15 = *puVar14;
          *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
          thunk_FUN_140205820(puVar14,0xc);
        }
        uVar7 = FUN_142f04924();
        *(undefined1 *)(*(longlong *)(lVar13 + 0x80) + 4) = uVar7;
        pbVar31 = *(byte **)(lVar13 + 0x80);
        bVar25 = pbVar31[4];
        pbVar31[8] = 0x65;
        pbVar31[9] = 0x9a;
        uVar21 = 0;
        lVar24 = (longlong)&local_208 - (longlong)pbVar31;
        lVar16 = 1 - (longlong)pbVar31;
        lVar2 = 2 - (longlong)pbVar31;
        lVar4 = 3 - (longlong)pbVar31;
        do {
          if (bVar25 == 0) {
            bVar25 = 0x2a;
          }
          bVar8 = pbVar31[lVar24];
          *pbVar31 = bVar25 ^ bVar8;
          bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
          uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
          *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
               (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
          bVar8 = 0x2a;
          if (bVar25 != 0) {
            bVar8 = bVar25;
          }
          bVar25 = pbVar31[(longlong)&local_208 + lVar16];
          pbVar31[1] = bVar8 ^ bVar25;
          bVar8 = (bVar8 ^ bVar25) + bVar8 + 0x2a;
          uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
          *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
               (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
          bVar25 = 0x2a;
          if (bVar8 != 0) {
            bVar25 = bVar8;
          }
          bVar8 = pbVar31[(longlong)&local_208 + lVar2];
          pbVar31[2] = bVar25 ^ bVar8;
          bVar8 = (bVar25 ^ bVar8) + bVar25 + 0x2a;
          uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
          *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
               (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
          bVar25 = 0x2a;
          if (bVar8 != 0) {
            bVar25 = bVar8;
          }
          bVar8 = pbVar31[(longlong)&local_208 + lVar4];
          pbVar31[3] = bVar25 ^ bVar8;
          bVar25 = (bVar25 ^ bVar8) + bVar25 + 0x2a;
          uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
          *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
               (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
          uVar21 = uVar21 + 4;
          pbVar31 = pbVar31 + 4;
        } while (uVar21 < 4);
        lVar16 = local_1a0;
        uVar21 = (uint)local_1c0;
      }
      FUN_140c79130(0x182,lVar13 + 0x302a12f71a0);
      if (uVar21 != 0) {
        lVar13 = *(longlong *)(lVar16 + 8);
        if (lVar13 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar13 = *(longlong *)(lVar16 + 8);
        }
        puVar1 = *(undefined4 **)(lVar13 + 0x98);
        local_1c0 = CONCAT44(local_1c0._4_4_,*puVar1);
        local_1ff = *(byte *)(puVar1 + 1);
        uVar20 = 0x9a65;
        local_1dc[0] = 0x9a65;
        uVar21 = 0;
        pbVar31 = (byte *)&local_1c0;
        pbVar23 = (byte *)((longlong)puVar1 + 2);
        do {
          if (local_1ff == 0) {
            local_1ff = 0x2a;
            local_1ff = 0x2a;
          }
          *pbVar31 = local_1ff ^ pbVar31[(longlong)puVar1 - (longlong)&local_1c0];
          bVar25 = local_1ff + 0x2a + pbVar31[(longlong)puVar1 - (longlong)&local_1c0];
          uVar26 = (uVar20 >> 0xd) + (ushort)bVar25;
          uVar19 = uVar20 << 3;
          if (bVar25 == 0) {
            bVar25 = 0x2a;
          }
          pbVar31[1] = bVar25 ^ pbVar23[-1];
          bVar25 = bVar25 + 0x2a + pbVar23[-1];
          uVar28 = (ushort)bVar25;
          if (bVar25 == 0) {
            bVar25 = 0x2a;
          }
          pbVar31[2] = bVar25 ^ *pbVar23;
          bVar25 = bVar25 + 0x2a + *pbVar23;
          uVar27 = (ushort)bVar25;
          if (bVar25 == 0) {
            bVar25 = 0x2a;
          }
          pbVar31[3] = bVar25 ^ pbVar23[1];
          local_1ff = bVar25 + 0x2a + pbVar23[1];
          uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)local_1ff |
                   (((uVar19 & 0x1fff) >> 10) + uVar27 |
                   (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3) << 3;
          local_1dc[0] = uVar20;
          uVar21 = uVar21 + 4;
          pbVar31 = pbVar31 + 4;
          pbVar23 = pbVar23 + 4;
        } while (uVar21 < 4);
        FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e3f);
        lVar2 = *(longlong *)(lVar13 + 0x98);
        uVar21 = (uint)local_1c0;
        if (((local_1dc[0] != *(ushort *)(lVar2 + 8)) ||
            (*(char *)(lVar13 + 0x94) != *(char *)(lVar2 + 5))) ||
           (*(char *)(lVar13 + 0xa0) != *(char *)(lVar2 + 6))) {
          local_1b4 = *(undefined1 *)(lVar13 + 0xa0);
          local_1b3 = *(undefined1 *)(lVar13 + 0x94);
          local_138 = (longlong)(int)(uint)local_1c0;
          local_190 = 2;
          local_130 = FUN_1418039d0(5);
          local_220 = &local_1b4;
          local_230 = &local_1b3;
          local_240 = local_1dc;
          local_248 = &local_1ff;
          local_238 = (ushort *)(lVar2 + 8);
          local_228 = lVar2 + 5;
          local_218 = lVar2 + 6;
          puVar14 = (undefined8 *)FUN_140197ac0(&local_128,&local_130,&local_190,&local_138);
          FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
          if (local_128 != 0) {
            FUN_14019f2c0(local_128 + -0x10);
          }
        }
        iVar18 = *(int *)(lVar13 + 0x90);
        iVar30 = iVar18 + 1;
        *(int *)(lVar13 + 0x90) = iVar30;
        if (iVar30 == (iVar30 / 0x37) * 0x37) {
          local_208 = uVar21;
          iVar18 = iVar18 + 2;
          *(int *)(lVar13 + 0x90) = iVar18;
          if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
            puVar14 = *(undefined8 **)(lVar13 + 0x98);
            puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
            *(undefined8 **)(lVar13 + 0x98) = puVar15;
            *puVar15 = *puVar14;
            *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
            thunk_FUN_140205820(puVar14,0xc);
          }
          uVar7 = FUN_142f04924();
          *(undefined1 *)(*(longlong *)(lVar13 + 0x98) + 4) = uVar7;
          pbVar31 = *(byte **)(lVar13 + 0x98);
          bVar25 = pbVar31[4];
          pbVar31[8] = 0x65;
          pbVar31[9] = 0x9a;
          uVar21 = 0;
          lVar22 = (longlong)&local_208 - (longlong)pbVar31;
          lVar2 = 1 - (longlong)pbVar31;
          lVar4 = 2 - (longlong)pbVar31;
          lVar24 = 3 - (longlong)pbVar31;
          do {
            if (bVar25 == 0) {
              bVar25 = 0x2a;
            }
            bVar8 = pbVar31[lVar22];
            *pbVar31 = bVar25 ^ bVar8;
            bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
            uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
            *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                 (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
            bVar8 = 0x2a;
            if (bVar25 != 0) {
              bVar8 = bVar25;
            }
            bVar25 = pbVar31[(longlong)&local_208 + lVar2];
            pbVar31[1] = bVar8 ^ bVar25;
            bVar8 = (bVar8 ^ bVar25) + bVar8 + 0x2a;
            uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
            *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                 (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
            bVar25 = 0x2a;
            if (bVar8 != 0) {
              bVar25 = bVar8;
            }
            bVar8 = pbVar31[(longlong)&local_208 + lVar4];
            pbVar31[2] = bVar25 ^ bVar8;
            bVar8 = (bVar25 ^ bVar8) + bVar25 + 0x2a;
            uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
            *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                 (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
            bVar25 = 0x2a;
            if (bVar8 != 0) {
              bVar25 = bVar8;
            }
            bVar8 = pbVar31[(longlong)&local_208 + lVar24];
            pbVar31[3] = bVar25 ^ bVar8;
            bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
            uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
            *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                 (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
            uVar21 = uVar21 + 4;
            pbVar31 = pbVar31 + 4;
            lVar16 = local_1a0;
          } while (uVar21 < 4);
        }
        FUN_140c79130(0x182,lVar13 + 0x302a12f71b8);
        if ((uint)local_1c0 == 0) goto LAB_1417a1b18;
      }
      lVar13 = *(longlong *)(lVar16 + 8);
      if (lVar13 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar13 = *(longlong *)(lVar16 + 8);
      }
      puVar1 = *(undefined4 **)(lVar13 + 0x80);
      local_1c0 = CONCAT44(local_1c0._4_4_,*puVar1);
      local_1fe = *(byte *)(puVar1 + 1);
      uVar20 = 0x9a65;
      local_1d8[0] = 0x9a65;
      uVar21 = 0;
      pbVar31 = (byte *)&local_1c0;
      pbVar23 = (byte *)((longlong)puVar1 + 2);
      do {
        if (local_1fe == 0) {
          local_1fe = 0x2a;
          local_1fe = 0x2a;
        }
        bVar25 = pbVar31[(longlong)puVar1 - (longlong)&local_1c0];
        *pbVar31 = bVar25 ^ local_1fe;
        bVar25 = bVar25 + local_1fe + 0x2a;
        uVar26 = (uVar20 >> 0xd) + (ushort)bVar25;
        uVar19 = uVar20 << 3;
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        bVar8 = pbVar23[-1];
        pbVar31[1] = bVar8 ^ bVar25;
        bVar8 = bVar8 + bVar25 + 0x2a;
        uVar28 = (ushort)bVar8;
        if (bVar8 == 0) {
          bVar8 = 0x2a;
        }
        bVar25 = *pbVar23;
        pbVar31[2] = bVar25 ^ bVar8;
        bVar25 = bVar25 + bVar8 + 0x2a;
        uVar27 = (ushort)bVar25;
        if (bVar25 == 0) {
          bVar25 = 0x2a;
        }
        local_1fe = pbVar23[1];
        pbVar31[3] = local_1fe ^ bVar25;
        local_1fe = local_1fe + bVar25 + 0x2a;
        uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)local_1fe |
                 (((uVar19 & 0x1fff) >> 10) + uVar27 |
                 (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3) << 3;
        local_1d8[0] = uVar20;
        uVar21 = uVar21 + 4;
        pbVar31 = pbVar31 + 4;
        pbVar23 = pbVar23 + 4;
      } while (uVar21 < 4);
      FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e27);
      lVar2 = *(longlong *)(lVar13 + 0x80);
      uVar21 = (uint)local_1c0;
      if (((local_1d8[0] != *(ushort *)(lVar2 + 8)) ||
          (*(char *)(lVar13 + 0x7c) != *(char *)(lVar2 + 5))) ||
         (*(char *)(lVar13 + 0x88) != *(char *)(lVar2 + 6))) {
        local_1b2 = *(undefined1 *)(lVar13 + 0x88);
        local_1b1 = *(undefined1 *)(lVar13 + 0x7c);
        local_120 = (longlong)(int)(uint)local_1c0;
        local_18c = 2;
        local_118 = FUN_1418039d0(5);
        local_220 = &local_1b2;
        local_230 = &local_1b1;
        local_240 = local_1d8;
        local_248 = &local_1fe;
        local_238 = (ushort *)(lVar2 + 8);
        local_228 = lVar2 + 5;
        local_218 = lVar2 + 6;
        puVar14 = (undefined8 *)FUN_140197ac0(&local_110,&local_118,&local_18c,&local_120);
        FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
        if (local_110 != 0) {
          FUN_14019f2c0(local_110 + -0x10);
        }
      }
      iVar18 = *(int *)(lVar13 + 0x78);
      iVar30 = iVar18 + 1;
      *(int *)(lVar13 + 0x78) = iVar30;
      if (iVar30 == (iVar30 / 0x37) * 0x37) {
        local_208 = uVar21;
        iVar18 = iVar18 + 2;
        *(int *)(lVar13 + 0x78) = iVar18;
        if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
          puVar14 = *(undefined8 **)(lVar13 + 0x80);
          puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
          *(undefined8 **)(lVar13 + 0x80) = puVar15;
          *puVar15 = *puVar14;
          *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
          thunk_FUN_140205820(puVar14,0xc);
        }
        uVar7 = FUN_142f04924();
        *(undefined1 *)(*(longlong *)(lVar13 + 0x80) + 4) = uVar7;
        pbVar31 = *(byte **)(lVar13 + 0x80);
        bVar25 = pbVar31[4];
        pbVar31[8] = 0x65;
        pbVar31[9] = 0x9a;
        uVar21 = 0;
        lVar22 = (longlong)&local_208 - (longlong)pbVar31;
        lVar2 = 1 - (longlong)pbVar31;
        lVar4 = 2 - (longlong)pbVar31;
        lVar24 = 3 - (longlong)pbVar31;
        do {
          if (bVar25 == 0) {
            bVar25 = 0x2a;
          }
          bVar8 = pbVar31[lVar22];
          *pbVar31 = bVar25 ^ bVar8;
          bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
          uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
          *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
               (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
          bVar8 = 0x2a;
          if (bVar25 != 0) {
            bVar8 = bVar25;
          }
          bVar25 = pbVar31[(longlong)&local_208 + lVar2];
          pbVar31[1] = bVar8 ^ bVar25;
          bVar8 = bVar8 + (bVar8 ^ bVar25) + 0x2a;
          uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
          *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
               (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
          bVar17 = 0x2a;
          if (bVar8 != 0) {
            bVar17 = bVar8;
          }
          bVar25 = pbVar31[(longlong)&local_208 + lVar4];
          pbVar31[2] = bVar17 ^ bVar25;
          bVar17 = bVar17 + (bVar17 ^ bVar25) + 0x2a;
          uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
          *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
               (uVar20 >> 0xd) + (ushort)bVar17 | uVar20 << 3;
          bVar25 = 0x2a;
          if (bVar17 != 0) {
            bVar25 = bVar17;
          }
          bVar8 = pbVar31[(longlong)&local_208 + lVar24];
          pbVar31[3] = bVar25 ^ bVar8;
          bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
          uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
          *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
               (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
          uVar21 = uVar21 + 4;
          pbVar31 = pbVar31 + 4;
          lVar16 = local_1a0;
        } while (uVar21 < 4);
      }
      FUN_140c79130(0x182,lVar13 + 0x302a12f71a0);
      if ((uint)local_1c0 != 0) {
        lVar13 = *(longlong *)(lVar16 + 8);
        if (lVar13 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar13 = *(longlong *)(lVar16 + 8);
        }
        if (*(char *)(lVar13 + 0x16d) != '\0') goto LAB_1417a1b18;
      }
      lVar13 = *(longlong *)(lVar16 + 8);
      if (lVar13 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar13 = *(longlong *)(lVar16 + 8);
      }
      if ((*(longlong *)(lVar13 + 0x1f8) < 1) && (*(int *)(lVar13 + 0x1e0) < 1)) {
        uVar20 = FUN_141ec2170(local_160);
        local_1c0 = CONCAT62(local_1c0._2_6_,uVar20);
        lVar13 = *(longlong *)(lVar16 + 8);
        if (lVar13 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar13 = *(longlong *)(lVar16 + 8);
        }
        puVar3 = *(uint **)(lVar13 + 0x80);
        local_208 = *puVar3;
        bVar25 = (byte)puVar3[1];
        local_1fd = bVar25;
        uVar19 = 0x9a65;
        local_1d4[0] = 0x9a65;
        uVar21 = 0;
        pbVar23 = (byte *)&local_208;
        pbVar31 = (byte *)((longlong)puVar3 + 2);
        do {
          if (bVar25 == 0) {
            local_1fd = 0x2a;
            bVar25 = 0x2a;
          }
          *pbVar23 = bVar25 ^ pbVar23[(longlong)puVar3 - (longlong)&local_208];
          bVar25 = bVar25 + 0x2a + pbVar23[(longlong)puVar3 - (longlong)&local_208];
          uVar28 = (uVar19 >> 0xd) + (ushort)bVar25;
          uVar26 = uVar19 << 3;
          if (bVar25 == 0) {
            bVar25 = 0x2a;
          }
          pbVar23[1] = bVar25 ^ pbVar31[-1];
          bVar25 = bVar25 + 0x2a + pbVar31[-1];
          uVar27 = (ushort)bVar25;
          if (bVar25 == 0) {
            bVar25 = 0x2a;
          }
          pbVar23[2] = bVar25 ^ *pbVar31;
          bVar25 = bVar25 + 0x2a + *pbVar31;
          uVar29 = (ushort)bVar25;
          if (bVar25 == 0) {
            bVar25 = 0x2a;
          }
          pbVar23[3] = bVar25 ^ pbVar31[1];
          bVar25 = bVar25 + 0x2a + pbVar31[1];
          local_1fd = bVar25;
          uVar19 = ((uVar28 | uVar26 & 0x3ff) >> 7) + (ushort)bVar25 |
                   (((uVar26 & 0x1fff) >> 10) + uVar29 |
                   (((uVar19 & 0x1fff) >> 10) + uVar27 | (uVar28 | uVar26) << 3) << 3) << 3;
          local_1d4[0] = uVar19;
          uVar21 = uVar21 + 4;
          pbVar23 = pbVar23 + 4;
          pbVar31 = pbVar31 + 4;
        } while (uVar21 < 4);
        FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e27);
        uVar21 = local_208;
        lVar2 = *(longlong *)(lVar13 + 0x80);
        if (((local_1d4[0] != *(ushort *)(lVar2 + 8)) ||
            (*(char *)(lVar13 + 0x7c) != *(char *)(lVar2 + 5))) ||
           (*(char *)(lVar13 + 0x88) != *(char *)(lVar2 + 6))) {
          local_1b0 = *(undefined1 *)(lVar13 + 0x88);
          local_1af = *(undefined1 *)(lVar13 + 0x7c);
          local_108 = (longlong)(int)local_208;
          local_188 = 2;
          local_100 = FUN_1418039d0(5);
          local_220 = &local_1b0;
          local_230 = &local_1af;
          local_240 = local_1d4;
          local_248 = &local_1fd;
          local_238 = (ushort *)(lVar2 + 8);
          local_228 = lVar2 + 5;
          local_218 = lVar2 + 6;
          puVar14 = (undefined8 *)FUN_140197ac0(&local_f8,&local_100,&local_188,&local_108);
          FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
          if (local_f8 != 0) {
            FUN_14019f2c0(local_f8 + -0x10);
          }
        }
        iVar18 = *(int *)(lVar13 + 0x78);
        iVar30 = iVar18 + 1;
        *(int *)(lVar13 + 0x78) = iVar30;
        if (iVar30 == (iVar30 / 0x37) * 0x37) {
          local_1f0 = uVar21;
          iVar18 = iVar18 + 2;
          *(int *)(lVar13 + 0x78) = iVar18;
          if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
            puVar14 = *(undefined8 **)(lVar13 + 0x80);
            puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
            *(undefined8 **)(lVar13 + 0x80) = puVar15;
            *puVar15 = *puVar14;
            *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
            thunk_FUN_140205820(puVar14,0xc);
          }
          uVar7 = FUN_142f04924();
          *(undefined1 *)(*(longlong *)(lVar13 + 0x80) + 4) = uVar7;
          pbVar31 = *(byte **)(lVar13 + 0x80);
          bVar25 = pbVar31[4];
          pbVar31[8] = 0x65;
          pbVar31[9] = 0x9a;
          uVar21 = 0;
          lVar24 = (longlong)&local_1f0 - (longlong)pbVar31;
          lVar16 = 1 - (longlong)pbVar31;
          lVar2 = 2 - (longlong)pbVar31;
          lVar4 = 3 - (longlong)pbVar31;
          do {
            if (bVar25 == 0) {
              bVar25 = 0x2a;
            }
            bVar8 = pbVar31[lVar24];
            *pbVar31 = bVar25 ^ bVar8;
            bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
            uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
            *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                 (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
            bVar8 = 0x2a;
            if (bVar25 != 0) {
              bVar8 = bVar25;
            }
            bVar25 = pbVar31[(longlong)&local_1f0 + lVar16];
            pbVar31[1] = bVar8 ^ bVar25;
            bVar8 = bVar8 + (bVar8 ^ bVar25) + 0x2a;
            uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
            *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                 (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
            bVar17 = 0x2a;
            if (bVar8 != 0) {
              bVar17 = bVar8;
            }
            bVar25 = pbVar31[(longlong)&local_1f0 + lVar2];
            pbVar31[2] = bVar17 ^ bVar25;
            bVar17 = bVar17 + (bVar17 ^ bVar25) + 0x2a;
            uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
            *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                 (uVar20 >> 0xd) + (ushort)bVar17 | uVar20 << 3;
            bVar25 = 0x2a;
            if (bVar17 != 0) {
              bVar25 = bVar17;
            }
            bVar8 = pbVar31[(longlong)&local_1f0 + lVar4];
            pbVar31[3] = bVar25 ^ bVar8;
            bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
            uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
            *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                 (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
            uVar21 = uVar21 + 4;
            pbVar31 = pbVar31 + 4;
          } while (uVar21 < 4);
          lVar16 = local_1a0;
          uVar20 = (ushort)local_1c0;
        }
        FUN_140c79130(0x182,lVar13 + 0x302a12f71a0);
        if (local_208 == 0) {
          if ((uVar20 & 1) != 0) {
            local_1c0 = DAT_143aa8328;
            lVar13 = *(longlong *)(lVar16 + 8);
            if (lVar13 == 0) {
              FUN_142e52ed0(0x431,0);
              lVar13 = *(longlong *)(lVar16 + 8);
            }
            puVar3 = *(uint **)(lVar13 + 0x98);
            local_208 = *puVar3;
            bVar25 = (byte)puVar3[1];
            local_1fc = bVar25;
            uVar20 = 0x9a65;
            local_1d0[0] = 0x9a65;
            uVar21 = 0;
            pbVar23 = (byte *)&local_208;
            pbVar31 = (byte *)((longlong)puVar3 + 2);
            do {
              if (bVar25 == 0) {
                local_1fc = 0x2a;
                bVar25 = 0x2a;
              }
              *pbVar23 = bVar25 ^ pbVar23[(longlong)puVar3 - (longlong)&local_208];
              bVar25 = bVar25 + 0x2a + pbVar23[(longlong)puVar3 - (longlong)&local_208];
              uVar26 = (uVar20 >> 0xd) + (ushort)bVar25;
              uVar19 = uVar20 << 3;
              if (bVar25 == 0) {
                bVar25 = 0x2a;
              }
              pbVar23[1] = bVar25 ^ pbVar31[-1];
              bVar25 = bVar25 + 0x2a + pbVar31[-1];
              uVar28 = (ushort)bVar25;
              if (bVar25 == 0) {
                bVar25 = 0x2a;
              }
              pbVar23[2] = bVar25 ^ *pbVar31;
              bVar25 = bVar25 + 0x2a + *pbVar31;
              uVar27 = (ushort)bVar25;
              if (bVar25 == 0) {
                bVar25 = 0x2a;
              }
              pbVar23[3] = bVar25 ^ pbVar31[1];
              bVar25 = bVar25 + 0x2a + pbVar31[1];
              local_1fc = bVar25;
              uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)bVar25 |
                       (((uVar19 & 0x1fff) >> 10) + uVar27 |
                       (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3) << 3;
              local_1d0[0] = uVar20;
              uVar21 = uVar21 + 4;
              pbVar23 = pbVar23 + 4;
              pbVar31 = pbVar31 + 4;
            } while (uVar21 < 4);
            FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e3f);
            uVar21 = local_208;
            lVar2 = *(longlong *)(lVar13 + 0x98);
            uVar32 = (ulonglong)(int)local_208;
            if (((local_1d0[0] != *(ushort *)(lVar2 + 8)) ||
                (*(char *)(lVar13 + 0x94) != *(char *)(lVar2 + 5))) ||
               (*(char *)(lVar13 + 0xa0) != *(char *)(lVar2 + 6))) {
              local_1ae = *(undefined1 *)(lVar13 + 0xa0);
              local_1ad = *(undefined1 *)(lVar13 + 0x94);
              local_184 = 2;
              local_f0 = uVar32;
              local_e8 = FUN_1418039d0(5);
              local_220 = &local_1ae;
              local_230 = &local_1ad;
              local_240 = local_1d0;
              local_248 = &local_1fc;
              local_238 = (ushort *)(lVar2 + 8);
              local_228 = lVar2 + 5;
              local_218 = lVar2 + 6;
              puVar14 = (undefined8 *)FUN_140197ac0(&local_e0,&local_e8,&local_184,&local_f0);
              FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
              if (local_e0 != 0) {
                FUN_14019f2c0(local_e0 + -0x10);
              }
            }
            iVar18 = *(int *)(lVar13 + 0x90);
            iVar30 = iVar18 + 1;
            *(int *)(lVar13 + 0x90) = iVar30;
            if (iVar30 == (iVar30 / 0x37) * 0x37) {
              local_1f0 = uVar21;
              iVar18 = iVar18 + 2;
              *(int *)(lVar13 + 0x90) = iVar18;
              if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
                puVar14 = *(undefined8 **)(lVar13 + 0x98);
                puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                *(undefined8 **)(lVar13 + 0x98) = puVar15;
                *puVar15 = *puVar14;
                *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
                thunk_FUN_140205820(puVar14,0xc);
              }
              uVar7 = FUN_142f04924();
              *(undefined1 *)(*(longlong *)(lVar13 + 0x98) + 4) = uVar7;
              pbVar31 = *(byte **)(lVar13 + 0x98);
              bVar25 = pbVar31[4];
              pbVar31[8] = 0x65;
              pbVar31[9] = 0x9a;
              uVar21 = 0;
              lVar24 = (longlong)&local_1f0 - (longlong)pbVar31;
              lVar16 = 1 - (longlong)pbVar31;
              lVar2 = 2 - (longlong)pbVar31;
              lVar4 = 3 - (longlong)pbVar31;
              do {
                if (bVar25 == 0) {
                  bVar25 = 0x2a;
                }
                bVar8 = pbVar31[lVar24];
                *pbVar31 = bVar25 ^ bVar8;
                bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                     (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                bVar8 = 0x2a;
                if (bVar25 != 0) {
                  bVar8 = bVar25;
                }
                bVar25 = pbVar31[(longlong)&local_1f0 + lVar16];
                pbVar31[1] = bVar8 ^ bVar25;
                bVar8 = bVar8 + (bVar8 ^ bVar25) + 0x2a;
                uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                     (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
                bVar17 = 0x2a;
                if (bVar8 != 0) {
                  bVar17 = bVar8;
                }
                bVar25 = pbVar31[(longlong)&local_1f0 + lVar2];
                pbVar31[2] = bVar17 ^ bVar25;
                bVar17 = bVar17 + (bVar17 ^ bVar25) + 0x2a;
                uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                     (uVar20 >> 0xd) + (ushort)bVar17 | uVar20 << 3;
                bVar25 = 0x2a;
                if (bVar17 != 0) {
                  bVar25 = bVar17;
                }
                bVar8 = pbVar31[(longlong)&local_1f0 + lVar4];
                pbVar31[3] = bVar25 ^ bVar8;
                bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                     (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                uVar21 = uVar21 + 4;
                pbVar31 = pbVar31 + 4;
              } while (uVar21 < 4);
              uVar32 = (ulonglong)local_208;
              lVar16 = local_1a0;
            }
            FUN_140c79130(0x182,lVar13 + 0x302a12f71b8);
            iVar18 = FUN_14038c030(local_1c0,uVar32 & 0xffffffff);
            if (iVar18 == 0) {
              local_1c0 = DAT_143aa8328;
              lVar13 = *(longlong *)(lVar16 + 8);
              if (lVar13 == 0) {
                FUN_142e52ed0(0x431,0);
                lVar13 = *(longlong *)(lVar16 + 8);
              }
              puVar3 = *(uint **)(lVar13 + 0x98);
              local_208 = *puVar3;
              bVar25 = (byte)puVar3[1];
              local_1fb = bVar25;
              uVar20 = 0x9a65;
              local_1cc[0] = 0x9a65;
              uVar21 = 0;
              pbVar23 = (byte *)&local_208;
              pbVar31 = (byte *)((longlong)puVar3 + 2);
              do {
                if (bVar25 == 0) {
                  local_1fb = 0x2a;
                  bVar25 = 0x2a;
                }
                *pbVar23 = bVar25 ^ pbVar23[(longlong)puVar3 - (longlong)&local_208];
                bVar25 = bVar25 + 0x2a + pbVar23[(longlong)puVar3 - (longlong)&local_208];
                uVar26 = (uVar20 >> 0xd) + (ushort)bVar25;
                uVar19 = uVar20 << 3;
                if (bVar25 == 0) {
                  bVar25 = 0x2a;
                }
                pbVar23[1] = bVar25 ^ pbVar31[-1];
                bVar25 = bVar25 + 0x2a + pbVar31[-1];
                uVar28 = (ushort)bVar25;
                if (bVar25 == 0) {
                  bVar25 = 0x2a;
                }
                pbVar23[2] = bVar25 ^ *pbVar31;
                bVar25 = bVar25 + 0x2a + *pbVar31;
                uVar27 = (ushort)bVar25;
                if (bVar25 == 0) {
                  bVar25 = 0x2a;
                }
                pbVar23[3] = bVar25 ^ pbVar31[1];
                bVar25 = bVar25 + 0x2a + pbVar31[1];
                local_1fb = bVar25;
                uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)bVar25 |
                         (((uVar19 & 0x1fff) >> 10) + uVar27 |
                         (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3) << 3;
                local_1cc[0] = uVar20;
                uVar21 = uVar21 + 4;
                pbVar23 = pbVar23 + 4;
                pbVar31 = pbVar31 + 4;
              } while (uVar21 < 4);
              FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e3f);
              uVar21 = local_208;
              lVar2 = *(longlong *)(lVar13 + 0x98);
              uVar32 = (ulonglong)(int)local_208;
              if (((local_1cc[0] != *(ushort *)(lVar2 + 8)) ||
                  (*(char *)(lVar13 + 0x94) != *(char *)(lVar2 + 5))) ||
                 (*(char *)(lVar13 + 0xa0) != *(char *)(lVar2 + 6))) {
                local_1ac = *(undefined1 *)(lVar13 + 0xa0);
                local_1a4[0] = *(undefined1 *)(lVar13 + 0x94);
                local_180 = 2;
                local_d8 = uVar32;
                local_d0 = FUN_1418039d0(5);
                local_220 = &local_1ac;
                local_230 = local_1a4;
                local_240 = local_1cc;
                local_248 = &local_1fb;
                local_238 = (ushort *)(lVar2 + 8);
                local_228 = lVar2 + 5;
                local_218 = lVar2 + 6;
                puVar14 = (undefined8 *)FUN_140197ac0(&local_c8,&local_d0,&local_180,&local_d8);
                FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
                if (local_c8 != 0) {
                  FUN_14019f2c0(local_c8 + -0x10);
                }
              }
              iVar18 = *(int *)(lVar13 + 0x90);
              iVar30 = iVar18 + 1;
              *(int *)(lVar13 + 0x90) = iVar30;
              if (iVar30 == (iVar30 / 0x37) * 0x37) {
                local_1f0 = uVar21;
                iVar18 = iVar18 + 2;
                *(int *)(lVar13 + 0x90) = iVar18;
                if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
                  puVar14 = *(undefined8 **)(lVar13 + 0x98);
                  puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                  *(undefined8 **)(lVar13 + 0x98) = puVar15;
                  *puVar15 = *puVar14;
                  *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
                  thunk_FUN_140205820(puVar14,0xc);
                }
                uVar7 = FUN_142f04924();
                *(undefined1 *)(*(longlong *)(lVar13 + 0x98) + 4) = uVar7;
                pbVar31 = *(byte **)(lVar13 + 0x98);
                bVar25 = pbVar31[4];
                pbVar31[8] = 0x65;
                pbVar31[9] = 0x9a;
                uVar21 = 0;
                lVar24 = (longlong)&local_1f0 - (longlong)pbVar31;
                lVar16 = 1 - (longlong)pbVar31;
                lVar2 = 2 - (longlong)pbVar31;
                lVar4 = 3 - (longlong)pbVar31;
                do {
                  if (bVar25 == 0) {
                    bVar25 = 0x2a;
                  }
                  bVar8 = pbVar31[lVar24];
                  *pbVar31 = bVar25 ^ bVar8;
                  bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                  uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                  *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                       (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                  bVar8 = 0x2a;
                  if (bVar25 != 0) {
                    bVar8 = bVar25;
                  }
                  bVar25 = pbVar31[(longlong)&local_1f0 + lVar16];
                  pbVar31[1] = bVar8 ^ bVar25;
                  bVar8 = bVar8 + (bVar8 ^ bVar25) + 0x2a;
                  uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                  *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                       (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
                  bVar17 = 0x2a;
                  if (bVar8 != 0) {
                    bVar17 = bVar8;
                  }
                  bVar25 = pbVar31[(longlong)&local_1f0 + lVar2];
                  pbVar31[2] = bVar17 ^ bVar25;
                  bVar17 = bVar17 + (bVar17 ^ bVar25) + 0x2a;
                  uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                  *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                       (uVar20 >> 0xd) + (ushort)bVar17 | uVar20 << 3;
                  bVar25 = 0x2a;
                  if (bVar17 != 0) {
                    bVar25 = bVar17;
                  }
                  bVar8 = pbVar31[(longlong)&local_1f0 + lVar4];
                  pbVar31[3] = bVar25 ^ bVar8;
                  bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                  uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                  *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                       (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                  uVar21 = uVar21 + 4;
                  pbVar31 = pbVar31 + 4;
                } while (uVar21 < 4);
                uVar32 = (ulonglong)local_208;
                lVar16 = local_1a0;
              }
              FUN_140c79130(0x182,lVar13 + 0x302a12f71b8);
              iVar18 = FUN_14038c0b0(local_1c0,uVar32 & 0xffffffff);
              if (iVar18 == 0) goto LAB_1417a0699;
            }
          }
        }
        else {
LAB_1417a0699:
          cVar6 = FUN_1417d0180(lVar16);
          if (cVar6 != '\0') {
            FUN_142d205e0(local_198,0,0x10a);
            lVar13 = *(longlong *)(lVar16 + 8);
            if (lVar13 == 0) {
              FUN_142e52ed0(0x431,0);
              lVar13 = *(longlong *)(lVar16 + 8);
            }
            if (((*(char *)(lVar13 + 0x61) != '\0') && (*(int *)(lVar13 + 0xd8) == 3)) &&
               (2999 < local_194 - *(int *)(lVar13 + 0xe8))) {
              iVar18 = FUN_14019a5d0(lVar13 + 0x140);
              iVar30 = FUN_14019a5d0(lVar13 + 0x128);
              lVar13 = local_168;
              if (((((local_50 <= iVar18) && (iVar18 < iStack_48)) && (iStack_4c <= iVar30)) &&
                  ((iVar30 < iStack_44 && (iVar18 = FUN_14279c310(local_168), iVar18 == 0)))) &&
                 (iVar18 = FUN_142904a40(lVar13), iVar18 == 0)) {
                lVar13 = *(longlong *)(lVar16 + 8);
                if (lVar13 == 0) {
                  FUN_142e52ed0(0x431,0);
                  lVar13 = *(longlong *)(lVar16 + 8);
                }
                puVar3 = *(uint **)(lVar13 + 0x80);
                local_208 = *puVar3;
                bVar25 = (byte)puVar3[1];
                local_1fa = bVar25;
                uVar20 = 0x9a65;
                local_1c8[0] = 0x9a65;
                uVar21 = 0;
                pbVar23 = (byte *)&local_208;
                pbVar31 = (byte *)((longlong)puVar3 + 2);
                do {
                  if (bVar25 == 0) {
                    local_1fa = 0x2a;
                    bVar25 = 0x2a;
                  }
                  *pbVar23 = bVar25 ^ pbVar23[(longlong)puVar3 - (longlong)&local_208];
                  bVar25 = bVar25 + 0x2a + pbVar23[(longlong)puVar3 - (longlong)&local_208];
                  uVar26 = (uVar20 >> 0xd) + (ushort)bVar25;
                  uVar19 = uVar20 << 3;
                  if (bVar25 == 0) {
                    bVar25 = 0x2a;
                  }
                  pbVar23[1] = bVar25 ^ pbVar31[-1];
                  bVar25 = bVar25 + 0x2a + pbVar31[-1];
                  uVar28 = (ushort)bVar25;
                  if (bVar25 == 0) {
                    bVar25 = 0x2a;
                  }
                  pbVar23[2] = bVar25 ^ *pbVar31;
                  bVar25 = bVar25 + 0x2a + *pbVar31;
                  uVar27 = (ushort)bVar25;
                  if (bVar25 == 0) {
                    bVar25 = 0x2a;
                  }
                  pbVar23[3] = bVar25 ^ pbVar31[1];
                  bVar25 = bVar25 + 0x2a + pbVar31[1];
                  local_1fa = bVar25;
                  uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)bVar25 |
                           (((uVar19 & 0x1fff) >> 10) + uVar27 |
                           (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3) << 3;
                  local_1c8[0] = uVar20;
                  uVar21 = uVar21 + 4;
                  pbVar23 = pbVar23 + 4;
                  pbVar31 = pbVar31 + 4;
                } while (uVar21 < 4);
                FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e27);
                uVar21 = local_208;
                lVar2 = *(longlong *)(lVar13 + 0x80);
                if (((local_1c8[0] != *(ushort *)(lVar2 + 8)) ||
                    (*(char *)(lVar13 + 0x7c) != *(char *)(lVar2 + 5))) ||
                   (*(char *)(lVar13 + 0x88) != *(char *)(lVar2 + 6))) {
                  local_1a5 = *(undefined1 *)(lVar13 + 0x88);
                  local_1a6 = *(undefined1 *)(lVar13 + 0x7c);
                  local_c0 = (longlong)(int)local_208;
                  local_17c = 2;
                  local_b8 = FUN_1418039d0(5);
                  local_220 = &local_1a5;
                  local_230 = &local_1a6;
                  local_240 = local_1c8;
                  local_248 = &local_1fa;
                  local_238 = (ushort *)(lVar2 + 8);
                  local_228 = lVar2 + 5;
                  local_218 = lVar2 + 6;
                  puVar14 = (undefined8 *)FUN_140197ac0(&local_b0,&local_b8,&local_17c,&local_c0);
                  FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
                  if (local_b0 != 0) {
                    FUN_14019f2c0(local_b0 + -0x10);
                  }
                }
                iVar18 = *(int *)(lVar13 + 0x78);
                iVar30 = iVar18 + 1;
                *(int *)(lVar13 + 0x78) = iVar30;
                if (iVar30 == (iVar30 / 0x37) * 0x37) {
                  local_1f0 = uVar21;
                  iVar18 = iVar18 + 2;
                  *(int *)(lVar13 + 0x78) = iVar18;
                  if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
                    puVar14 = *(undefined8 **)(lVar13 + 0x80);
                    puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                    *(undefined8 **)(lVar13 + 0x80) = puVar15;
                    *puVar15 = *puVar14;
                    *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
                    thunk_FUN_140205820(puVar14,0xc);
                  }
                  uVar7 = FUN_142f04924();
                  *(undefined1 *)(*(longlong *)(lVar13 + 0x80) + 4) = uVar7;
                  pbVar31 = *(byte **)(lVar13 + 0x80);
                  bVar25 = pbVar31[4];
                  pbVar31[8] = 0x65;
                  pbVar31[9] = 0x9a;
                  uVar21 = 0;
                  lVar22 = (longlong)&local_1f0 - (longlong)pbVar31;
                  lVar2 = 1 - (longlong)pbVar31;
                  lVar4 = 2 - (longlong)pbVar31;
                  lVar24 = 3 - (longlong)pbVar31;
                  do {
                    if (bVar25 == 0) {
                      bVar25 = 0x2a;
                    }
                    bVar8 = pbVar31[lVar22];
                    *pbVar31 = bVar25 ^ bVar8;
                    bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                    uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
                    *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                         (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                    bVar8 = 0x2a;
                    if (bVar25 != 0) {
                      bVar8 = bVar25;
                    }
                    bVar25 = pbVar31[(longlong)&local_1f0 + lVar2];
                    pbVar31[1] = bVar8 ^ bVar25;
                    bVar8 = bVar8 + (bVar8 ^ bVar25) + 0x2a;
                    uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
                    *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                         (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
                    bVar17 = 0x2a;
                    if (bVar8 != 0) {
                      bVar17 = bVar8;
                    }
                    bVar25 = pbVar31[(longlong)&local_1f0 + lVar4];
                    pbVar31[2] = bVar17 ^ bVar25;
                    bVar17 = bVar17 + (bVar17 ^ bVar25) + 0x2a;
                    uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
                    *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                         (uVar20 >> 0xd) + (ushort)bVar17 | uVar20 << 3;
                    bVar25 = 0x2a;
                    if (bVar17 != 0) {
                      bVar25 = bVar17;
                    }
                    bVar8 = pbVar31[(longlong)&local_1f0 + lVar24];
                    pbVar31[3] = bVar25 ^ bVar8;
                    bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                    uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
                    *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                         (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                    uVar21 = uVar21 + 4;
                    pbVar31 = pbVar31 + 4;
                    lVar16 = local_1a0;
                  } while (uVar21 < 4);
                }
                FUN_140c79130(0x182,lVar13 + 0x302a12f71a0);
                if (local_208 == 0) {
                  lVar13 = *(longlong *)(lVar16 + 8);
                  if (lVar13 == 0) {
                    FUN_142e52ed0(0x431,0);
                    lVar13 = *(longlong *)(lVar16 + 8);
                  }
                  puVar3 = *(uint **)(lVar13 + 0x98);
                  local_1f0 = *puVar3;
                  local_1f9 = (byte)puVar3[1];
                  uVar20 = 0x9a65;
                  local_1c4[0] = 0x9a65;
                  uVar21 = 0;
                  pbVar23 = (byte *)&local_1f0;
                  pbVar31 = (byte *)((longlong)puVar3 + 2);
                  do {
                    if (local_1f9 == 0) {
                      local_1f9 = 0x2a;
                      local_1f9 = 0x2a;
                    }
                    *pbVar23 = local_1f9 ^ pbVar23[(longlong)puVar3 - (longlong)&local_1f0];
                    bVar25 = local_1f9 + 0x2a + pbVar23[(longlong)puVar3 - (longlong)&local_1f0];
                    uVar26 = (uVar20 >> 0xd) + (ushort)bVar25;
                    uVar19 = uVar20 << 3;
                    if (bVar25 == 0) {
                      bVar25 = 0x2a;
                    }
                    pbVar23[1] = bVar25 ^ pbVar31[-1];
                    bVar25 = bVar25 + 0x2a + pbVar31[-1];
                    uVar28 = (ushort)bVar25;
                    if (bVar25 == 0) {
                      bVar25 = 0x2a;
                    }
                    pbVar23[2] = bVar25 ^ *pbVar31;
                    bVar25 = bVar25 + 0x2a + *pbVar31;
                    uVar27 = (ushort)bVar25;
                    if (bVar25 == 0) {
                      bVar25 = 0x2a;
                    }
                    pbVar23[3] = bVar25 ^ pbVar31[1];
                    local_1f9 = bVar25 + 0x2a + pbVar31[1];
                    uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)local_1f9 |
                             (((uVar19 & 0x1fff) >> 10) + uVar27 |
                             (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3) <<
                             3;
                    local_1c4[0] = uVar20;
                    uVar21 = uVar21 + 4;
                    pbVar23 = pbVar23 + 4;
                    pbVar31 = pbVar31 + 4;
                  } while (uVar21 < 4);
                  FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e3f);
                  uVar21 = local_1f0;
                  lVar2 = *(longlong *)(lVar13 + 0x98);
                  uVar32 = (ulonglong)(int)local_1f0;
                  local_208 = local_1f0;
                  if (((local_1c4[0] != *(ushort *)(lVar2 + 8)) ||
                      (*(char *)(lVar13 + 0x94) != *(char *)(lVar2 + 5))) ||
                     (*(char *)(lVar13 + 0xa0) != *(char *)(lVar2 + 6))) {
                    local_1a7 = *(undefined1 *)(lVar13 + 0xa0);
                    local_1ab = *(undefined1 *)(lVar13 + 0x94);
                    local_178 = 2;
                    local_a8 = uVar32;
                    local_a0 = FUN_1418039d0(5);
                    local_220 = &local_1a7;
                    local_230 = &local_1ab;
                    local_240 = local_1c4;
                    local_248 = &local_1f9;
                    local_238 = (ushort *)(lVar2 + 8);
                    local_228 = lVar2 + 5;
                    local_218 = lVar2 + 6;
                    puVar14 = (undefined8 *)FUN_140197ac0(&local_98,&local_a0,&local_178,&local_a8);
                    FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
                    if (local_98 != 0) {
                      FUN_14019f2c0(local_98 + -0x10);
                    }
                  }
                  iVar18 = *(int *)(lVar13 + 0x90);
                  iVar30 = iVar18 + 1;
                  *(int *)(lVar13 + 0x90) = iVar30;
                  if (iVar30 == (iVar30 / 0x37) * 0x37) {
                    iVar18 = iVar18 + 2;
                    *(int *)(lVar13 + 0x90) = iVar18;
                    local_1f0 = uVar21;
                    if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
                      puVar14 = *(undefined8 **)(lVar13 + 0x98);
                      puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                      *(undefined8 **)(lVar13 + 0x98) = puVar15;
                      *puVar15 = *puVar14;
                      *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
                      thunk_FUN_140205820(puVar14,0xc);
                    }
                    uVar7 = FUN_142f04924();
                    *(undefined1 *)(*(longlong *)(lVar13 + 0x98) + 4) = uVar7;
                    pbVar31 = *(byte **)(lVar13 + 0x98);
                    bVar25 = pbVar31[4];
                    pbVar31[8] = 0x65;
                    pbVar31[9] = 0x9a;
                    uVar21 = 0;
                    lVar24 = (longlong)&local_1f0 - (longlong)pbVar31;
                    lVar16 = 1 - (longlong)pbVar31;
                    lVar2 = 2 - (longlong)pbVar31;
                    lVar4 = 3 - (longlong)pbVar31;
                    do {
                      if (bVar25 == 0) {
                        bVar25 = 0x2a;
                      }
                      bVar8 = pbVar31[lVar24];
                      *pbVar31 = bVar25 ^ bVar8;
                      bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                      uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                      *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                           (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                      bVar8 = 0x2a;
                      if (bVar25 != 0) {
                        bVar8 = bVar25;
                      }
                      bVar25 = pbVar31[(longlong)&local_1f0 + lVar16];
                      pbVar31[1] = bVar8 ^ bVar25;
                      bVar8 = bVar8 + (bVar8 ^ bVar25) + 0x2a;
                      uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                      *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                           (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
                      bVar17 = 0x2a;
                      if (bVar8 != 0) {
                        bVar17 = bVar8;
                      }
                      bVar25 = pbVar31[(longlong)&local_1f0 + lVar2];
                      pbVar31[2] = bVar17 ^ bVar25;
                      bVar17 = bVar17 + (bVar17 ^ bVar25) + 0x2a;
                      uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                      *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                           (uVar20 >> 0xd) + (ushort)bVar17 | uVar20 << 3;
                      bVar25 = 0x2a;
                      if (bVar17 != 0) {
                        bVar25 = bVar17;
                      }
                      bVar8 = pbVar31[(longlong)&local_1f0 + lVar4];
                      pbVar31[3] = bVar25 ^ bVar8;
                      bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                      uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                      *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                           (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                      uVar21 = uVar21 + 4;
                      pbVar31 = pbVar31 + 4;
                    } while (uVar21 < 4);
                    uVar32 = (ulonglong)local_208;
                    lVar16 = local_1a0;
                  }
                  FUN_140c79130(0x182,lVar13 + 0x302a12f71b8);
                }
                else {
                  uVar32 = 0x7fffffff;
                }
                iVar18 = FUN_1428d78a0(local_168,uVar32 & 0xffffffff);
                if (iVar18 == 0) {
                  FUN_142d205e0(&local_1b7,0,0x11c);
                  lVar13 = *(longlong *)(lVar16 + 8);
                  if (lVar13 == 0) {
                    FUN_142e52ed0(0x431,0);
                    lVar13 = *(longlong *)(lVar16 + 8);
                  }
                  puVar3 = *(uint **)(lVar13 + 0x80);
                  local_208 = *puVar3;
                  bVar25 = (byte)puVar3[1];
                  local_1f8 = bVar25;
                  uVar20 = 0x9a65;
                  local_1e8[0] = 0x9a65;
                  uVar21 = 0;
                  pbVar23 = (byte *)&local_208;
                  pbVar31 = (byte *)((longlong)puVar3 + 2);
                  do {
                    if (bVar25 == 0) {
                      local_1f8 = 0x2a;
                      bVar25 = 0x2a;
                    }
                    bVar8 = pbVar23[(longlong)puVar3 - (longlong)&local_208];
                    *pbVar23 = bVar8 ^ bVar25;
                    bVar8 = bVar8 + bVar25 + 0x2a;
                    uVar26 = (uVar20 >> 0xd) + (ushort)bVar8;
                    uVar19 = uVar20 << 3;
                    if (bVar8 == 0) {
                      bVar8 = 0x2a;
                    }
                    bVar17 = pbVar31[-1];
                    pbVar23[1] = bVar17 ^ bVar8;
                    bVar17 = bVar17 + bVar8 + 0x2a;
                    uVar28 = (ushort)bVar17;
                    if (bVar17 == 0) {
                      bVar17 = 0x2a;
                    }
                    bVar25 = *pbVar31;
                    pbVar23[2] = bVar25 ^ bVar17;
                    bVar25 = bVar25 + bVar17 + 0x2a;
                    uVar27 = (ushort)bVar25;
                    if (bVar25 == 0) {
                      bVar25 = 0x2a;
                    }
                    bVar8 = pbVar31[1];
                    pbVar23[3] = bVar8 ^ bVar25;
                    bVar25 = bVar8 + 0x2a + bVar25;
                    local_1f8 = bVar25;
                    uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)bVar25 |
                             (((uVar19 & 0x1fff) >> 10) + uVar27 |
                             (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3) <<
                             3;
                    local_1e8[0] = uVar20;
                    uVar21 = uVar21 + 4;
                    pbVar23 = pbVar23 + 4;
                    pbVar31 = pbVar31 + 4;
                  } while (uVar21 < 4);
                  FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e27);
                  uVar21 = local_208;
                  lVar2 = *(longlong *)(lVar13 + 0x80);
                  if (((local_1e8[0] != *(ushort *)(lVar2 + 8)) ||
                      (*(char *)(lVar13 + 0x7c) != *(char *)(lVar2 + 5))) ||
                     (*(char *)(lVar13 + 0x88) != *(char *)(lVar2 + 6))) {
                    local_1aa = *(undefined1 *)(lVar13 + 0x88);
                    local_1a9 = *(undefined1 *)(lVar13 + 0x7c);
                    local_90 = (longlong)(int)local_208;
                    local_174 = 2;
                    local_88 = FUN_1418039d0(5);
                    local_220 = &local_1aa;
                    local_230 = &local_1a9;
                    local_240 = local_1e8;
                    local_248 = &local_1f8;
                    local_238 = (ushort *)(lVar2 + 8);
                    local_228 = lVar2 + 5;
                    local_218 = lVar2 + 6;
                    puVar14 = (undefined8 *)FUN_140197ac0(&local_80,&local_88,&local_174,&local_90);
                    FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
                    if (local_80 != 0) {
                      FUN_14019f2c0(local_80 + -0x10);
                    }
                  }
                  iVar18 = *(int *)(lVar13 + 0x78);
                  iVar30 = iVar18 + 1;
                  *(int *)(lVar13 + 0x78) = iVar30;
                  if (iVar30 == (iVar30 / 0x37) * 0x37) {
                    local_1f0 = uVar21;
                    iVar18 = iVar18 + 2;
                    *(int *)(lVar13 + 0x78) = iVar18;
                    if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
                      puVar14 = *(undefined8 **)(lVar13 + 0x80);
                      puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                      *(undefined8 **)(lVar13 + 0x80) = puVar15;
                      *puVar15 = *puVar14;
                      *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
                      thunk_FUN_140205820(puVar14,0xc);
                    }
                    uVar7 = FUN_142f04924();
                    *(undefined1 *)(*(longlong *)(lVar13 + 0x80) + 4) = uVar7;
                    pbVar31 = *(byte **)(lVar13 + 0x80);
                    bVar25 = pbVar31[4];
                    pbVar31[8] = 0x65;
                    pbVar31[9] = 0x9a;
                    uVar21 = 0;
                    lVar22 = (longlong)&local_1f0 - (longlong)pbVar31;
                    lVar2 = 1 - (longlong)pbVar31;
                    lVar4 = 2 - (longlong)pbVar31;
                    lVar24 = 3 - (longlong)pbVar31;
                    do {
                      if (bVar25 == 0) {
                        bVar25 = 0x2a;
                      }
                      bVar8 = pbVar31[lVar22];
                      *pbVar31 = bVar25 ^ bVar8;
                      bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                      uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
                      *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                           (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                      bVar8 = 0x2a;
                      if (bVar25 != 0) {
                        bVar8 = bVar25;
                      }
                      bVar25 = pbVar31[(longlong)&local_1f0 + lVar2];
                      pbVar31[1] = bVar8 ^ bVar25;
                      bVar8 = bVar8 + (bVar8 ^ bVar25) + 0x2a;
                      uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
                      *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                           (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
                      bVar17 = 0x2a;
                      if (bVar8 != 0) {
                        bVar17 = bVar8;
                      }
                      bVar25 = pbVar31[(longlong)&local_1f0 + lVar4];
                      pbVar31[2] = bVar17 ^ bVar25;
                      bVar17 = bVar17 + (bVar17 ^ bVar25) + 0x2a;
                      uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
                      *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                           (uVar20 >> 0xd) + (ushort)bVar17 | uVar20 << 3;
                      bVar25 = 0x2a;
                      if (bVar17 != 0) {
                        bVar25 = bVar17;
                      }
                      bVar8 = pbVar31[(longlong)&local_1f0 + lVar24];
                      pbVar31[3] = bVar25 ^ bVar8;
                      bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                      uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8);
                      *(ushort *)(*(longlong *)(lVar13 + 0x80) + 8) =
                           (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                      uVar21 = uVar21 + 4;
                      pbVar31 = pbVar31 + 4;
                      lVar16 = local_1a0;
                    } while (uVar21 < 4);
                  }
                  FUN_140c79130(0x182,lVar13 + 0x302a12f71a0);
                  if (local_208 == 0) {
                    local_1c0 = DAT_143aa8328;
                    lVar13 = *(longlong *)(lVar16 + 8);
                    if (lVar13 == 0) {
                      FUN_142e52ed0(0x431,0);
                      lVar13 = *(longlong *)(lVar16 + 8);
                    }
                    puVar3 = *(uint **)(lVar13 + 0x98);
                    local_208 = *puVar3;
                    bVar25 = (byte)puVar3[1];
                    local_1f7[0] = bVar25;
                    uVar20 = 0x9a65;
                    local_1e4[0] = 0x9a65;
                    uVar21 = 0;
                    pbVar23 = (byte *)&local_208;
                    pbVar31 = (byte *)((longlong)puVar3 + 2);
                    do {
                      if (bVar25 == 0) {
                        local_1f7[0] = 0x2a;
                        bVar25 = 0x2a;
                      }
                      bVar8 = pbVar23[(longlong)puVar3 - (longlong)&local_208];
                      *pbVar23 = bVar8 ^ bVar25;
                      bVar8 = bVar8 + bVar25 + 0x2a;
                      uVar26 = (uVar20 >> 0xd) + (ushort)bVar8;
                      uVar19 = uVar20 << 3;
                      if (bVar8 == 0) {
                        bVar8 = 0x2a;
                      }
                      bVar25 = pbVar31[-1];
                      pbVar23[1] = bVar25 ^ bVar8;
                      bVar25 = bVar25 + bVar8 + 0x2a;
                      uVar28 = (ushort)bVar25;
                      if (bVar25 == 0) {
                        bVar25 = 0x2a;
                      }
                      bVar8 = *pbVar31;
                      pbVar23[2] = bVar8 ^ bVar25;
                      bVar8 = bVar8 + bVar25 + 0x2a;
                      uVar27 = (ushort)bVar8;
                      if (bVar8 == 0) {
                        bVar8 = 0x2a;
                      }
                      bVar25 = pbVar31[1];
                      pbVar23[3] = bVar25 ^ bVar8;
                      bVar25 = bVar25 + bVar8 + 0x2a;
                      local_1f7[0] = bVar25;
                      uVar20 = ((uVar26 | uVar19 & 0x3ff) >> 7) + (ushort)bVar25 |
                               (((uVar19 & 0x1fff) >> 10) + uVar27 |
                               (((uVar20 & 0x1fff) >> 10) + uVar28 | (uVar26 | uVar19) << 3) << 3)
                               << 3;
                      local_1e4[0] = uVar20;
                      uVar21 = uVar21 + 4;
                      pbVar23 = pbVar23 + 4;
                      pbVar31 = pbVar31 + 4;
                    } while (uVar21 < 4);
                    FUN_140c78f50(0x171,lVar13 + 0x21a3f060fc2e3f);
                    uVar21 = local_208;
                    lVar2 = *(longlong *)(lVar13 + 0x98);
                    uVar32 = (ulonglong)(int)local_208;
                    if (((local_1e4[0] != *(ushort *)(lVar2 + 8)) ||
                        (*(char *)(lVar13 + 0x94) != *(char *)(lVar2 + 5))) ||
                       (*(char *)(lVar13 + 0xa0) != *(char *)(lVar2 + 6))) {
                      local_1a8 = *(undefined1 *)(lVar13 + 0xa0);
                      local_1b8 = *(undefined1 *)(lVar13 + 0x94);
                      local_170 = 2;
                      local_78 = uVar32;
                      local_70 = FUN_1418039d0(5);
                      local_220 = &local_1a8;
                      local_230 = &local_1b8;
                      local_240 = local_1e4;
                      local_248 = local_1f7;
                      local_238 = (ushort *)(lVar2 + 8);
                      local_228 = lVar2 + 5;
                      local_218 = lVar2 + 6;
                      puVar14 = (undefined8 *)FUN_140197ac0(local_60,&local_70,&local_170,&local_78)
                      ;
                      FUN_141804970(&DAT_143271f04,0x17c,5,*puVar14);
                      if (local_60[0] != 0) {
                        FUN_14019f2c0(local_60[0] + -0x10);
                      }
                    }
                    iVar18 = *(int *)(lVar13 + 0x90);
                    iVar30 = iVar18 + 1;
                    *(int *)(lVar13 + 0x90) = iVar30;
                    if (iVar30 == (iVar30 / 0x37) * 0x37) {
                      local_1f0 = uVar21;
                      iVar18 = iVar18 + 2;
                      *(int *)(lVar13 + 0x90) = iVar18;
                      if (iVar18 == (iVar18 / 0x6f) * 0x6f) {
                        puVar14 = *(undefined8 **)(lVar13 + 0x98);
                        puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
                        *(undefined8 **)(lVar13 + 0x98) = puVar15;
                        *puVar15 = *puVar14;
                        *(undefined4 *)(puVar15 + 1) = *(undefined4 *)(puVar14 + 1);
                        thunk_FUN_140205820(puVar14,0xc);
                      }
                      uVar7 = FUN_142f04924();
                      *(undefined1 *)(*(longlong *)(lVar13 + 0x98) + 4) = uVar7;
                      pbVar31 = *(byte **)(lVar13 + 0x98);
                      bVar25 = pbVar31[4];
                      pbVar31[8] = 0x65;
                      pbVar31[9] = 0x9a;
                      uVar21 = 0;
                      lVar24 = (longlong)&local_1f0 - (longlong)pbVar31;
                      lVar16 = 1 - (longlong)pbVar31;
                      lVar2 = 2 - (longlong)pbVar31;
                      lVar4 = 3 - (longlong)pbVar31;
                      do {
                        if (bVar25 == 0) {
                          bVar25 = 0x2a;
                        }
                        bVar8 = pbVar31[lVar24];
                        *pbVar31 = bVar25 ^ bVar8;
                        bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                        uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                        *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                             (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                        bVar8 = 0x2a;
                        if (bVar25 != 0) {
                          bVar8 = bVar25;
                        }
                        bVar25 = pbVar31[(longlong)&local_1f0 + lVar16];
                        pbVar31[1] = bVar8 ^ bVar25;
                        bVar8 = bVar8 + (bVar8 ^ bVar25) + 0x2a;
                        uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                        *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                             (uVar20 >> 0xd) + (ushort)bVar8 | uVar20 << 3;
                        bVar17 = 0x2a;
                        if (bVar8 != 0) {
                          bVar17 = bVar8;
                        }
                        bVar25 = pbVar31[(longlong)&local_1f0 + lVar2];
                        pbVar31[2] = bVar17 ^ bVar25;
                        bVar17 = bVar17 + (bVar17 ^ bVar25) + 0x2a;
                        uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                        *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                             (uVar20 >> 0xd) + (ushort)bVar17 | uVar20 << 3;
                        bVar25 = 0x2a;
                        if (bVar17 != 0) {
                          bVar25 = bVar17;
                        }
                        bVar8 = pbVar31[(longlong)&local_1f0 + lVar4];
                        pbVar31[3] = bVar25 ^ bVar8;
                        bVar25 = bVar25 + (bVar25 ^ bVar8) + 0x2a;
                        uVar20 = *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8);
                        *(ushort *)(*(longlong *)(lVar13 + 0x98) + 8) =
                             (uVar20 >> 0xd) + (ushort)bVar25 | uVar20 << 3;
                        uVar21 = uVar21 + 4;
                        pbVar31 = pbVar31 + 4;
                      } while (uVar21 < 4);
                      uVar32 = (ulonglong)local_208;
                      lVar16 = local_1a0;
                    }
                    FUN_140c79130(0x182,lVar13 + 0x302a12f71b8);
                    uVar9 = FUN_1403a1f90(local_1c0,uVar32 & 0xffffffff);
                  }
                  else {
                    uVar9 = 0;
                  }
                  lVar13 = *(longlong *)(lVar16 + 8);
                  if (lVar13 == 0) {
                    FUN_142e52ed0(0x431,0);
                    lVar13 = *(longlong *)(lVar16 + 8);
                  }
                  iVar18 = FUN_14019a5d0(lVar13 + 0x140);
                  iVar30 = FUN_14019a5d0(lVar13 + 0x128);
                  if ((((local_50 <= iVar18) && (iVar18 < iStack_48)) && (iStack_4c <= iVar30)) &&
                     (iVar30 < iStack_44)) {
                    uVar10 = FUN_140831940(&local_50,0);
                    lVar13 = *(longlong *)(lVar16 + 8);
                    if (lVar13 == 0) {
                      FUN_142e52ed0(0x431,0);
                      lVar13 = *(longlong *)(lVar16 + 8);
                    }
                    iVar18 = FUN_14019a5d0(lVar13 + 0x140);
                    iVar30 = FUN_14019a5d0(lVar13 + 0x128);
                    if (((local_50 <= iVar18) && (iVar18 < iStack_48)) &&
                       ((iStack_4c <= iVar30 && (iVar30 < iStack_44)))) {
                      FUN_14290fe40(local_168,&local_50,0x5000ffff);
                      lVar13 = FUN_1417d2850(lVar16);
                      param_3 = local_158;
                      local_248 = (byte *)CONCAT44(local_248._4_4_,uVar10);
                      FUN_141ec1f20(local_160,local_158,*(undefined4 *)(lVar13 + 100),uVar9);
                      lVar16 = FUN_1417d2850(lVar16);
                      *(int *)(lVar16 + 0xe8) = local_194;
                      FUN_142d205e0(&local_1b7,0,0x133);
                      goto LAB_1417a1b59;
                    }
                  }
                }
                else if (iVar18 == 2) {
                  *(undefined4 *)(DAT_143aa8518 + 0x5628) = 1;
                }
              }
            }
          }
        }
      }
    }
LAB_1417a1b18:
    uVar32 = *(ulonglong *)(lVar16 + -0x20);
    if ((uVar32 != 0) && (uVar32 < 0x10001)) {
      FUN_142e52ed0(0x33e);
      uVar32 = *(ulonglong *)(lVar16 + -0x20);
    }
    lVar16 = 0;
    param_3 = local_158;
    if (uVar32 != 0) {
      lVar16 = uVar32 + 0x28;
    }
  } while( true );
}



//===========================================================
// FUN_1403d2200 @ 1403d2200   (4914 bytes)
//===========================================================

longlong FUN_1403d2200(longlong param_1,longlong param_2,int param_3,int param_4,int param_5)

{
  undefined8 *puVar1;
  longlong lVar2;
  undefined *puVar3;
  longlong *plVar4;
  undefined1 uVar5;
  byte bVar6;
  char cVar7;
  undefined2 uVar8;
  short sVar9;
  ushort uVar10;
  int iVar11;
  undefined4 uVar12;
  int iVar13;
  uint uVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  int *piVar17;
  longlong lVar18;
  longlong *plVar19;
  longlong *plVar20;
  undefined8 uVar21;
  byte *pbVar22;
  int *piVar23;
  longlong lVar24;
  ulonglong uVar25;
  uint uVar26;
  byte bVar27;
  longlong lVar28;
  undefined1 local_78 [2];
  byte abStack_76 [6];
  int *local_70;
  longlong *local_68;
  longlong *local_60;
  longlong *local_58;
  longlong *local_50 [2];
  
  iVar11 = FUN_1401b1040(param_3);
  _local_78 = param_3;
  if (iVar11 != 1) {
    if (iVar11 == 2) {
      if (((param_3 - 2000000U < 1000000) || (param_3 - 3000000U < 1000000)) ||
         (param_3 - 4000000U < 1000000)) {
        FUN_14039e630(param_1,local_50,param_3);
        if (local_50[0] == (longlong *)0x0) {
          *(undefined8 *)(param_2 + 8) = 0;
          return param_2;
        }
        (**(code **)(*local_50[0] + 0x10))();
      }
      else {
        if (999999 < param_3 - 5000000U) goto LAB_1403d224f;
        FUN_14039f600(param_1,local_50,param_3,0);
        if (local_50[0] == (longlong *)0x0) {
          *(undefined8 *)(param_2 + 8) = 0;
          return param_2;
        }
        (**(code **)(*local_50[0] + 0x10))();
      }
      plVar19 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x7e);
      local_58 = plVar19;
      if (plVar19 == (longlong *)0x0) {
        plVar19 = (longlong *)0x0;
      }
      else {
        FUN_1402f7aa0(plVar19);
        *plVar19 = (longlong)&PTR_FUN_14327e588;
        uVar12 = FUN_1402f70a0(0,(longlong)plVar19 + 0x4d);
        *(undefined4 *)((longlong)plVar19 + 0x51) = uVar12;
        uVar12 = FUN_1402f7010(0,(longlong)plVar19 + 0x55);
        *(undefined4 *)((longlong)plVar19 + 0x59) = uVar12;
        bVar27 = FUN_1407386b0(&DAT_143ac1ab0);
        *(byte *)((longlong)plVar19 + 0x5d) = bVar27;
        *(byte *)((longlong)plVar19 + 0x5e) = bVar27;
        *(uint *)((longlong)plVar19 + 0x61) =
             ((bVar27 ^ 0xbaadf00d) >> 5 | (bVar27 ^ 0xbaadf00d) << 0x1b) + (uint)bVar27;
        uVar12 = FUN_1402f70a0(0,(longlong)plVar19 + 0x4d);
        *(undefined4 *)((longlong)plVar19 + 0x51) = uVar12;
        uVar12 = FUN_1402f7010(0,(longlong)plVar19 + 0x55);
        *(undefined4 *)((longlong)plVar19 + 0x59) = uVar12;
        bVar27 = FUN_1407386b0(&DAT_143ac1ab0);
        *(byte *)((longlong)plVar19 + 0x5d) = bVar27;
        *(byte *)((longlong)plVar19 + 0x5e) = bVar27;
        *(uint *)((longlong)plVar19 + 0x61) =
             ((bVar27 ^ 0xbaadf00d) >> 5 | (bVar27 ^ 0xbaadf00d) << 0x1b) + (uint)bVar27;
        *(undefined8 *)((longlong)plVar19 + 0x65) = 0;
        *(undefined1 *)((longlong)plVar19 + 0x6d) = 0;
        *(undefined4 *)((longlong)plVar19 + 0x7a) = 0;
      }
      iVar11 = (int)plVar19[4] + 1;
      *(int *)(plVar19 + 4) = iVar11;
      if (iVar11 == (iVar11 / 0x6f) * 0x6f) {
        puVar1 = (undefined8 *)plVar19[5];
        puVar16 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
        plVar19[5] = (longlong)puVar16;
        *puVar16 = *puVar1;
        *(undefined4 *)(puVar16 + 1) = *(undefined4 *)(puVar1 + 1);
        thunk_FUN_140205820(puVar1,0xc);
      }
      uVar5 = FUN_142f04924();
      *(undefined1 *)(plVar19[5] + 4) = uVar5;
      pbVar22 = (byte *)plVar19[5];
      bVar27 = pbVar22[4];
      pbVar22[8] = 0x65;
      pbVar22[9] = 0x9a;
      uVar26 = 0;
      lVar28 = (longlong)local_78 - (longlong)pbVar22;
      lVar18 = 1 - (longlong)pbVar22;
      lVar24 = 2 - (longlong)pbVar22;
      local_58 = (longlong *)(local_78 + lVar24);
      lVar2 = 3 - (longlong)pbVar22;
      do {
        if (bVar27 == 0) {
          bVar27 = 0x2a;
        }
        bVar6 = pbVar22[lVar28];
        *pbVar22 = bVar27 ^ bVar6;
        bVar27 = bVar27 + (bVar27 ^ bVar6) + 0x2a;
        uVar10 = *(ushort *)(plVar19[5] + 8);
        *(ushort *)(plVar19[5] + 8) = (uVar10 >> 0xd) + (ushort)bVar27 | uVar10 << 3;
        bVar6 = 0x2a;
        if (bVar27 != 0) {
          bVar6 = bVar27;
        }
        bVar27 = pbVar22[(longlong)(local_78 + lVar18)];
        pbVar22[1] = bVar6 ^ bVar27;
        bVar6 = (bVar6 ^ bVar27) + bVar6 + 0x2a;
        uVar10 = *(ushort *)(plVar19[5] + 8);
        *(ushort *)(plVar19[5] + 8) = (uVar10 >> 0xd) + (ushort)bVar6 | uVar10 << 3;
        bVar27 = 0x2a;
        if (bVar6 != 0) {
          bVar27 = bVar6;
        }
        bVar6 = pbVar22[(longlong)(local_78 + lVar24)];
        pbVar22[2] = bVar27 ^ bVar6;
        bVar6 = (bVar27 ^ bVar6) + bVar27 + 0x2a;
        uVar10 = *(ushort *)(plVar19[5] + 8);
        *(ushort *)(plVar19[5] + 8) = (uVar10 >> 0xd) + (ushort)bVar6 | uVar10 << 3;
        bVar27 = 0x2a;
        if (bVar6 != 0) {
          bVar27 = bVar6;
        }
        bVar6 = pbVar22[(longlong)(local_78 + lVar2)];
        pbVar22[3] = bVar27 ^ bVar6;
        bVar27 = (bVar27 ^ bVar6) + bVar27 + 0x2a;
        uVar10 = *(ushort *)(plVar19[5] + 8);
        *(ushort *)(plVar19[5] + 8) = (uVar10 >> 0xd) + (ushort)bVar27 | uVar10 << 3;
        uVar26 = uVar26 + 4;
        pbVar22 = pbVar22 + 4;
      } while (uVar26 < 4);
      uVar12 = FUN_1402f70a0(1,(longlong)plVar19 + 0x4d);
      *(undefined4 *)((longlong)plVar19 + 0x51) = uVar12;
      *(undefined1 *)((longlong)plVar19 + 0x6d) = 0;
      uVar12 = FUN_1402f7010(0,(longlong)plVar19 + 0x55);
      *(undefined4 *)((longlong)plVar19 + 0x59) = uVar12;
      *(undefined8 *)((longlong)plVar19 + 0x65) = 0;
      *(longlong **)(param_2 + 8) = plVar19;
      if (0xfffff < (ulonglong)plVar19[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar19[1] = plVar19[1] + 1;
      UNLOCK();
      return param_2;
    }
    if (iVar11 == 3) {
      FUN_14039f600(param_1,&local_68,param_3,0);
      if (local_68 == (longlong *)0x0) {
        *(undefined8 *)(param_2 + 8) = 0;
        return param_2;
      }
      local_50[0] = (longlong *)FUN_14019b780(&DAT_143ad68a0,0xce);
      uVar25 = 0;
      uVar15 = uVar25;
      if (local_50[0] != (longlong *)0x0) {
        uVar15 = FUN_1402f8b40(local_50[0]);
      }
      iVar11 = *(int *)(uVar15 + 0x20) + 1;
      *(int *)(uVar15 + 0x20) = iVar11;
      if (iVar11 == (iVar11 / 0x6f) * 0x6f) {
        puVar1 = *(undefined8 **)(uVar15 + 0x28);
        puVar16 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
        *(undefined8 **)(uVar15 + 0x28) = puVar16;
        *puVar16 = *puVar1;
        *(undefined4 *)(puVar16 + 1) = *(undefined4 *)(puVar1 + 1);
        thunk_FUN_140205820(puVar1,0xc);
      }
      uVar5 = FUN_142f04924();
      *(undefined1 *)(*(longlong *)(uVar15 + 0x28) + 4) = uVar5;
      pbVar22 = *(byte **)(uVar15 + 0x28);
      bVar27 = pbVar22[4];
      pbVar22[8] = 0x65;
      pbVar22[9] = 0x9a;
      lVar28 = (longlong)local_78 - (longlong)pbVar22;
      lVar18 = 1 - (longlong)pbVar22;
      local_58 = (longlong *)(local_78 + lVar18);
      lVar24 = 2 - (longlong)pbVar22;
      local_60 = (longlong *)(local_78 + lVar24);
      lVar2 = 3 - (longlong)pbVar22;
      do {
        if (bVar27 == 0) {
          bVar27 = 0x2a;
        }
        bVar6 = pbVar22[lVar28];
        *pbVar22 = bVar27 ^ bVar6;
        bVar27 = bVar27 + (bVar27 ^ bVar6) + 0x2a;
        uVar10 = *(ushort *)(*(longlong *)(uVar15 + 0x28) + 8);
        *(ushort *)(*(longlong *)(uVar15 + 0x28) + 8) =
             (uVar10 >> 0xd) + (ushort)bVar27 | uVar10 << 3;
        bVar6 = 0x2a;
        if (bVar27 != 0) {
          bVar6 = bVar27;
        }
        bVar27 = pbVar22[(longlong)(local_78 + lVar18)];
        pbVar22[1] = bVar6 ^ bVar27;
        bVar6 = (bVar6 ^ bVar27) + bVar6 + 0x2a;
        uVar10 = *(ushort *)(*(longlong *)(uVar15 + 0x28) + 8);
        *(ushort *)(*(longlong *)(uVar15 + 0x28) + 8) =
             (uVar10 >> 0xd) + (ushort)bVar6 | uVar10 << 3;
        bVar27 = 0x2a;
        if (bVar6 != 0) {
          bVar27 = bVar6;
        }
        bVar6 = pbVar22[(longlong)(local_78 + lVar24)];
        pbVar22[2] = bVar27 ^ bVar6;
        bVar6 = (bVar27 ^ bVar6) + bVar27 + 0x2a;
        uVar10 = *(ushort *)(*(longlong *)(uVar15 + 0x28) + 8);
        *(ushort *)(*(longlong *)(uVar15 + 0x28) + 8) =
             (uVar10 >> 0xd) + (ushort)bVar6 | uVar10 << 3;
        bVar27 = 0x2a;
        if (bVar6 != 0) {
          bVar27 = bVar6;
        }
        bVar6 = pbVar22[(longlong)(local_78 + lVar2)];
        pbVar22[3] = bVar27 ^ bVar6;
        bVar27 = (bVar27 ^ bVar6) + bVar27 + 0x2a;
        uVar10 = *(ushort *)(*(longlong *)(uVar15 + 0x28) + 8);
        *(ushort *)(*(longlong *)(uVar15 + 0x28) + 8) =
             (uVar10 >> 0xd) + (ushort)bVar27 | uVar10 << 3;
        uVar26 = (int)uVar25 + 4;
        uVar25 = (ulonglong)uVar26;
        pbVar22 = pbVar22 + 4;
      } while (uVar26 < 4);
      local_50[0] = (longlong *)PTR_DAT_143a49018;
      if (param_1 == 0) {
        piVar17 = (int *)FUN_14019b600(&DAT_143ad6a30,0x14);
        piVar17[1] = 3;
        *piVar17 = -1;
        piVar23 = piVar17 + 4;
        piVar17[2] = 0;
        *(char *)piVar23 = '\0';
        *(undefined2 *)piVar23 = DAT_1432841a0;
        *(undefined1 *)((longlong)piVar17 + 0x12) = DAT_1432841a2;
        local_70 = piVar23;
        if (*piVar17 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar17[1] < 3) {
          FUN_142e54290(0x90,piVar17[1],3);
        }
        *piVar17 = 1;
        *(undefined1 *)((longlong)piVar17 + 0x13) = 0;
        if (piVar17[1] + 1 < 4) {
          FUN_142e54290(0x9c,3);
        }
        piVar17[2] = 3;
      }
      else {
        if (*(longlong *)(param_1 + 0xb8) != 0) {
          for (lVar18 = *(longlong *)
                         (*(longlong *)(param_1 + 0xb8) +
                         ((ulonglong)(longlong)param_3 % (ulonglong)*(uint *)(param_1 + 0xc0)) * 8);
              lVar18 != 0; lVar18 = *(longlong *)(lVar18 + 8)) {
            if (*(int *)(lVar18 + 0x10) == param_3) {
              if ((lVar18 + 0x18 != 0) &&
                 (lVar18 = FUN_14019bc60(lVar18 + 0x18,local_50), lVar18 != 0)) {
                local_70 = (int *)0x0;
                FUN_14019a260(&local_70,lVar18);
                piVar23 = local_70;
                goto LAB_1403d257a;
              }
              break;
            }
          }
        }
        local_70 = (int *)0x0;
        piVar23 = (int *)0x0;
      }
LAB_1403d257a:
      plVar19 = local_68;
      if ((piVar23 == (int *)0x0) || ((char)*piVar23 == '\0')) {
        piVar23 = (int *)&DAT_14328486c;
      }
      (*DAT_143ad5660)(uVar15 + 0x4d,piVar23);
      FUN_1404157e0(uVar15,1);
      uVar12 = FUN_1402f7010(0,uVar15 + 0x62);
      *(undefined4 *)(uVar15 + 0x66) = uVar12;
      FUN_1404158f0(uVar15,100);
      *(undefined8 *)(uVar15 + 0x82) = 0;
      uVar12 = FUN_1402f7010(0,uVar15 + 0x72);
      *(undefined4 *)(uVar15 + 0x76) = uVar12;
      local_60 = plVar19;
      (**(code **)(*plVar19 + 8))(plVar19);
      uVar8 = FUN_1403e54e0(&local_60);
      uVar12 = FUN_1402f70a0(uVar8,uVar15 + 0x7a);
      *(undefined4 *)(uVar15 + 0x7e) = uVar12;
      puVar3 = PTR_u_limitedLife_143a46180;
      local_58 = plVar19;
      (**(code **)(*plVar19 + 8))(plVar19);
      uVar12 = FUN_140910eb0(&local_58,puVar3,0);
      FUN_1404158a0(uVar15,uVar12);
      uVar12 = FUN_1402f7010(0,uVar15 + 0x96);
      *(undefined4 *)(uVar15 + 0x9a) = uVar12;
      FUN_140415460(uVar15,0);
      uVar12 = FUN_1402f7010(100,uVar15 + 0xb2);
      *(undefined4 *)(uVar15 + 0xb6) = uVar12;
      uVar12 = FUN_1402f7010(0,uVar15 + 0xba);
      *(undefined4 *)(uVar15 + 0xbe) = uVar12;
      *(ulonglong *)(param_2 + 8) = uVar15;
      if (0xfffff < *(ulonglong *)(uVar15 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(uVar15 + 8) = *(longlong *)(uVar15 + 8) + 1;
      UNLOCK();
      if (local_70 != (int *)0x0) {
        FUN_14019f2c0(local_70 + -4);
      }
      (**(code **)(*local_68 + 0x10))();
      return param_2;
    }
LAB_1403d224f:
    *(undefined8 *)(param_2 + 8) = 0;
    return param_2;
  }
  local_68 = (longlong *)FUN_140388c60(param_1,param_3);
  if (local_68 == (longlong *)0x0) goto LAB_1403d224f;
  local_50[0] = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x467);
  plVar20 = (longlong *)0x0;
  plVar19 = plVar20;
  if (local_50[0] != (longlong *)0x0) {
    plVar19 = (longlong *)FUN_1402f7da0(local_50[0]);
  }
  iVar11 = (int)plVar19[4] + 1;
  *(int *)(plVar19 + 4) = iVar11;
  if (iVar11 == (iVar11 / 0x6f) * 0x6f) {
    puVar1 = (undefined8 *)plVar19[5];
    puVar16 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar19[5] = (longlong)puVar16;
    *puVar16 = *puVar1;
    *(undefined4 *)(puVar16 + 1) = *(undefined4 *)(puVar1 + 1);
    thunk_FUN_140205820(puVar1,0xc);
  }
  uVar5 = FUN_142f04924();
  plVar4 = local_68;
  *(undefined1 *)(plVar19[5] + 4) = uVar5;
  pbVar22 = (byte *)plVar19[5];
  bVar27 = pbVar22[4];
  pbVar22[8] = 0x65;
  pbVar22[9] = 0x9a;
  lVar28 = (longlong)local_78 - (longlong)pbVar22;
  lVar18 = 1 - (longlong)pbVar22;
  local_50[0] = (longlong *)(local_78 + lVar18);
  lVar24 = 2 - (longlong)pbVar22;
  local_58 = (longlong *)(local_78 + lVar24);
  lVar2 = 3 - (longlong)pbVar22;
  do {
    if (bVar27 == 0) {
      bVar27 = 0x2a;
    }
    bVar6 = pbVar22[lVar28];
    *pbVar22 = bVar27 ^ bVar6;
    bVar27 = bVar27 + (bVar27 ^ bVar6) + 0x2a;
    uVar10 = *(ushort *)(plVar19[5] + 8);
    *(ushort *)(plVar19[5] + 8) = (uVar10 >> 0xd) + (ushort)bVar27 | uVar10 << 3;
    bVar6 = 0x2a;
    if (bVar27 != 0) {
      bVar6 = bVar27;
    }
    bVar27 = pbVar22[(longlong)(local_78 + lVar18)];
    pbVar22[1] = bVar6 ^ bVar27;
    bVar6 = (bVar6 ^ bVar27) + bVar6 + 0x2a;
    uVar10 = *(ushort *)(plVar19[5] + 8);
    *(ushort *)(plVar19[5] + 8) = (uVar10 >> 0xd) + (ushort)bVar6 | uVar10 << 3;
    bVar27 = 0x2a;
    if (bVar6 != 0) {
      bVar27 = bVar6;
    }
    bVar6 = pbVar22[(longlong)(local_78 + lVar24)];
    pbVar22[2] = bVar27 ^ bVar6;
    bVar6 = (bVar27 ^ bVar6) + bVar27 + 0x2a;
    uVar10 = *(ushort *)(plVar19[5] + 8);
    *(ushort *)(plVar19[5] + 8) = (uVar10 >> 0xd) + (ushort)bVar6 | uVar10 << 3;
    bVar27 = 0x2a;
    if (bVar6 != 0) {
      bVar27 = bVar6;
    }
    bVar6 = pbVar22[(longlong)(local_78 + lVar2)];
    pbVar22[3] = bVar27 ^ bVar6;
    bVar27 = (bVar27 ^ bVar6) + bVar27 + 0x2a;
    uVar10 = *(ushort *)(plVar19[5] + 8);
    *(ushort *)(plVar19[5] + 8) = (uVar10 >> 0xd) + (ushort)bVar27 | uVar10 << 3;
    uVar26 = (int)plVar20 + 4;
    plVar20 = (longlong *)(ulonglong)uVar26;
    pbVar22 = pbVar22 + 4;
  } while (uVar26 < 4);
  FUN_140415840((longlong)plVar19 + 0x62,(char)local_68[0x17]);
  FUN_140415580((longlong)plVar19 + 0x62,0);
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xba),(longlong)plVar19 + 0x62);
  *(undefined4 *)((longlong)plVar19 + 0x66) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xbc),(longlong)plVar19 + 0x6a);
  *(undefined4 *)((longlong)plVar19 + 0x6e) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xbe),(longlong)plVar19 + 0x72);
  *(undefined4 *)((longlong)plVar19 + 0x76) = uVar12;
  uVar12 = FUN_1402f7010((short)plVar4[0x18],(longlong)plVar19 + 0x7a);
  *(undefined4 *)((longlong)plVar19 + 0x7e) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xc2),(longlong)plVar19 + 0x82);
  *(undefined4 *)((longlong)plVar19 + 0x86) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xc4),(longlong)plVar19 + 0x8a);
  *(undefined4 *)((longlong)plVar19 + 0x8e) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xdc),(longlong)plVar19 + 0x92);
  *(undefined4 *)((longlong)plVar19 + 0x96) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xde),(longlong)plVar19 + 0x9a);
  *(undefined4 *)((longlong)plVar19 + 0x9e) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xca),(longlong)plVar19 + 0xe2);
  *(undefined4 *)((longlong)plVar19 + 0xe6) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xcc),(longlong)plVar19 + 0xa2);
  *(undefined4 *)((longlong)plVar19 + 0xa6) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xce),(longlong)plVar19 + 0xaa);
  *(undefined4 *)((longlong)plVar19 + 0xae) = uVar12;
  uVar12 = FUN_1402f7010((short)plVar4[0x1a],(longlong)plVar19 + 0xb2);
  *(undefined4 *)((longlong)plVar19 + 0xb6) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xd2),(longlong)plVar19 + 0xba);
  *(undefined4 *)((longlong)plVar19 + 0xbe) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xd4),(longlong)plVar19 + 0xc2);
  *(undefined4 *)((longlong)plVar19 + 0xc6) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xd6),(longlong)plVar19 + 0xca);
  *(undefined4 *)((longlong)plVar19 + 0xce) = uVar12;
  uVar12 = FUN_1402f7010((short)plVar4[0x1b],(longlong)plVar19 + 0xd2);
  *(undefined4 *)((longlong)plVar19 + 0xd6) = uVar12;
  uVar12 = FUN_1402f7010(*(undefined2 *)((longlong)plVar4 + 0xda),(longlong)plVar19 + 0xda);
  *(undefined4 *)((longlong)plVar19 + 0xde) = uVar12;
  *(undefined1 *)((longlong)plVar19 + 0x55) = 0;
  uVar12 = FUN_1402f7010(0);
  *(undefined4 *)((longlong)plVar19 + 0x10e) = uVar12;
  iVar11 = 0;
  if (plVar4[0x41] != 0) {
    FUN_1403fc480(plVar4 + 0x40);
  }
  FUN_1403020d0((longlong)plVar19 + 0x62);
  if (plVar4[0x41] != 0) {
    plVar20 = (longlong *)FUN_1403fc480(plVar4 + 0x40);
    if ((*plVar20 != 0) && (uVar26 = *(uint *)(*plVar20 + -8), uVar26 != 0)) {
      uVar15 = FUN_1407386b0(&DAT_143ac1ab0);
      uVar15 = (uVar15 & 0xffffffff) % (ulonglong)uVar26;
      lVar18 = *plVar20;
      if (lVar18 == 0) {
        uVar26 = 0;
LAB_1403d2ee8:
        FUN_142e54290(0xc6,uVar15,uVar26);
        lVar18 = *plVar20;
      }
      else {
        uVar26 = *(uint *)(lVar18 + -8);
        if (uVar26 <= (uint)uVar15) goto LAB_1403d2ee8;
      }
      uVar12 = *(undefined4 *)(lVar18 + uVar15 * 4);
      goto LAB_1403d2f09;
    }
  }
  uVar12 = 0;
LAB_1403d2f09:
  FUN_140302130((longlong)plVar19 + 0x62,uVar12);
  uVar12 = FUN_1402f7170(0,(longlong)plVar19 + 0x122);
  *(undefined4 *)((longlong)plVar19 + 0x132) = uVar12;
  FUN_140415640((longlong)plVar19 + 0x62,(int)plVar4[0x72]);
  *(undefined8 *)((longlong)plVar19 + 0x4d) = 0;
  FUN_140415790((longlong)plVar19 + 0x62,0);
  FUN_140415a10((longlong)plVar19 + 0x62,*(undefined1 *)((longlong)plVar4 + 0x117));
  FUN_1404159b0((longlong)plVar19 + 0x62,(char)plVar4[0x23]);
  FUN_1404154c0((longlong)plVar19 + 0x62,*(undefined1 *)((longlong)plVar4 + 0x119));
  FUN_140415730((longlong)plVar19 + 0x62,*(undefined1 *)((longlong)plVar4 + 0x11a));
  FUN_1404155e0((longlong)plVar19 + 0x62,*(undefined1 *)((longlong)plVar4 + 0x11b));
  FUN_140415950((longlong)plVar19 + 0x62,*(undefined1 *)((longlong)plVar4 + 0x11c));
  FUN_140301fc0((longlong)plVar19 + 0x62,*(undefined1 *)((longlong)plVar4 + 0x11d));
  if (param_4 == 0) {
    if ((int)plVar4[8] != 0) {
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0x62,*(undefined4 *)((longlong)plVar19 + 0x66));
      uVar12 = FUN_1402f7010(sVar9 + (short)plVar4[0x1d],(longlong)plVar19 + 0x62);
      *(undefined4 *)((longlong)plVar19 + 0x66) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0x6a,*(undefined4 *)((longlong)plVar19 + 0x6e));
      uVar12 = FUN_1402f7010(sVar9 + *(short *)((longlong)plVar4 + 0xea),(longlong)plVar19 + 0x6a);
      *(undefined4 *)((longlong)plVar19 + 0x6e) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0x72,*(undefined4 *)((longlong)plVar19 + 0x76));
      uVar12 = FUN_1402f7010(sVar9 + *(short *)((longlong)plVar4 + 0xec),(longlong)plVar19 + 0x72);
      *(undefined4 *)((longlong)plVar19 + 0x76) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0x7a,*(undefined4 *)((longlong)plVar19 + 0x7e));
      uVar12 = FUN_1402f7010(sVar9 + *(short *)((longlong)plVar4 + 0xee),(longlong)plVar19 + 0x7a);
      *(undefined4 *)((longlong)plVar19 + 0x7e) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0x82,*(undefined4 *)((longlong)plVar19 + 0x86));
      uVar12 = FUN_1402f7010(sVar9 + (short)plVar4[0x1e],(longlong)plVar19 + 0x82);
      *(undefined4 *)((longlong)plVar19 + 0x86) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0x8a,*(undefined4 *)((longlong)plVar19 + 0x8e));
      uVar12 = FUN_1402f7010(sVar9 + *(short *)((longlong)plVar4 + 0xf2),(longlong)plVar19 + 0x8a);
      *(undefined4 *)((longlong)plVar19 + 0x8e) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0xa2,*(undefined4 *)((longlong)plVar19 + 0xa6));
      uVar12 = FUN_1402f7010(sVar9 + *(short *)((longlong)plVar4 + 0xf4),(longlong)plVar19 + 0xa2);
      *(undefined4 *)((longlong)plVar19 + 0xa6) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0xaa,*(undefined4 *)((longlong)plVar19 + 0xae));
      uVar12 = FUN_1402f7010(sVar9 + *(short *)((longlong)plVar4 + 0xf6),(longlong)plVar19 + 0xaa);
      *(undefined4 *)((longlong)plVar19 + 0xae) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0xb2,*(undefined4 *)((longlong)plVar19 + 0xb6));
      uVar12 = FUN_1402f7010(sVar9 + (short)plVar4[0x1f],(longlong)plVar19 + 0xb2);
      *(undefined4 *)((longlong)plVar19 + 0xb6) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0x92,*(undefined4 *)((longlong)plVar19 + 0x96));
      uVar12 = FUN_1402f7010(sVar9 + *(short *)((longlong)plVar4 + 0x104),(longlong)plVar19 + 0x92);
      *(undefined4 *)((longlong)plVar19 + 0x96) = uVar12;
      sVar9 = FUN_1401ab420((longlong)plVar19 + 0x9a,*(undefined4 *)((longlong)plVar19 + 0x9e));
      uVar12 = FUN_1402f7010(sVar9 + *(short *)((longlong)plVar4 + 0x106),(longlong)plVar19 + 0x9a);
      *(undefined4 *)((longlong)plVar19 + 0x9e) = uVar12;
      cVar7 = FUN_1401b0050((longlong)plVar19 + 0x186,*(undefined4 *)((longlong)plVar19 + 0x18a));
      FUN_1404154c0((longlong)plVar19 + 0x62,cVar7 + (char)plVar4[0x21]);
      cVar7 = FUN_1401b0050((longlong)plVar19 + 0x18e,*(undefined4 *)((longlong)plVar19 + 0x192));
      FUN_140415730((longlong)plVar19 + 0x62,cVar7 + *(char *)((longlong)plVar4 + 0x109));
      cVar7 = FUN_1401b0050((longlong)plVar19 + 0x196,*(undefined4 *)((longlong)plVar19 + 0x19a));
      FUN_1404155e0((longlong)plVar19 + 0x62,cVar7 + *(char *)((longlong)plVar4 + 0x10a));
      cVar7 = FUN_1401b0050((longlong)plVar19 + 0x19e,*(undefined4 *)((longlong)plVar19 + 0x1a2));
      FUN_140415950((longlong)plVar19 + 0x62,cVar7 + *(char *)((longlong)plVar4 + 0x10b));
    }
  }
  else if ((int)plVar4[8] != 0) {
    plVar19[8] = DAT_143282b88;
  }
  lVar18 = (longlong)plVar19 + 0x15a;
  if ((int)plVar4[0x4b] != 0) {
    uVar10 = FUN_1401ab420(lVar18,*(undefined4 *)((longlong)plVar19 + 0x15e));
    uVar12 = FUN_1402f7010(uVar10 | 1,lVar18);
    *(undefined4 *)((longlong)plVar19 + 0x15e) = uVar12;
  }
  if ((int)plVar4[0x4c] != 0) {
    uVar10 = FUN_1401ab420(lVar18,*(undefined4 *)((longlong)plVar19 + 0x15e));
    uVar12 = FUN_1402f7010(uVar10 | 4,lVar18);
    *(undefined4 *)((longlong)plVar19 + 0x15e) = uVar12;
  }
  if (*(int *)((longlong)plVar4 + 0x264) != 0) {
    uVar10 = FUN_1401ab420(lVar18,*(undefined4 *)((longlong)plVar19 + 0x15e));
    uVar12 = FUN_1402f7010(uVar10 | 0x10,lVar18);
    *(undefined4 *)((longlong)plVar19 + 0x15e) = uVar12;
  }
  iVar13 = FUN_1401ba9d0((longlong)plVar19 + 0x162,*(undefined4 *)((longlong)plVar19 + 0x16a));
  if (iVar13 == 0) {
    uVar26 = 0xffffffff;
  }
  else {
    uVar26 = FUN_1401ba9d0((longlong)plVar19 + 0x162,*(undefined4 *)((longlong)plVar19 + 0x16a));
  }
  uVar14 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)plVar19 + 0x162) = uVar14;
  uVar26 = (uVar14 ^ uVar26) >> 5 | (uVar14 ^ uVar26) << 0x1b;
  *(uint *)((longlong)plVar19 + 0x166) = uVar26;
  *(uint *)((longlong)plVar19 + 0x16a) =
       ((uVar14 ^ 0xbaadf00d) >> 5 | (uVar14 ^ 0xbaadf00d) << 0x1b) + uVar26;
  FUN_140415520((longlong)plVar19 + 0x3af,(char)plVar4[0x2e]);
  FUN_140402c20((longlong)plVar19 + 0x26b);
  FUN_1402ce8a0(plVar19);
  FUN_1402ce940(plVar19);
  if (((int)plVar4[0x43] < 1) || (plVar4[0x44] == plVar4[0x45])) {
    if (((0 < param_5) && ((char)plVar4[0x17] != '\0')) &&
       ((((int)plVar4[0x43] < 1 || (plVar4[0x44] == plVar4[0x45])) && ((char)plVar4[0x49] == '\0')))
       ) {
      uVar12 = FUN_140253130((int)plVar4[7]);
      FUN_140253d30((int)plVar4[7],uVar12);
      switch(param_5) {
      case 0:
        uVar21 = 0;
        break;
      default:
        uVar21 = 1;
        break;
      case 2:
      case 4:
      case 6:
        uVar21 = 5;
        break;
      case 3:
        uVar21 = 2;
        break;
      case 5:
        uVar21 = 3;
        break;
      case 7:
        uVar21 = 4;
      }
      FUN_140415690((longlong)plVar19 + 0x3af,uVar21);
    }
  }
  else if ((char)plVar4[0x49] == '\0') {
    uVar12 = FUN_140253130((int)plVar4[7]);
    FUN_140253d30((int)plVar4[7],uVar12);
    switch((int)plVar4[0x43]) {
    case 0:
      uVar21 = 0;
      break;
    default:
      uVar21 = 1;
      break;
    case 2:
    case 4:
    case 6:
      uVar21 = 5;
      break;
    case 3:
      uVar21 = 2;
      break;
    case 5:
      uVar21 = 3;
      break;
    case 7:
      uVar21 = 4;
    }
    FUN_140415690((longlong)plVar19 + 0x3af,uVar21);
    cVar7 = (**(code **)(*plVar19 + 400))(plVar19);
    if ((cVar7 != '\0') && (bVar27 = (**(code **)(*plVar19 + 400))(plVar19), bVar27 < 5)) {
      lVar18 = plVar4[0x44];
      if (plVar4[0x45] - lVar18 >> 3 != 0) {
        lVar24 = 0;
        do {
          if ((0 < *(int *)(lVar18 + 4 + lVar24)) && (0 < (int)*(uint *)(lVar18 + lVar24))) {
            FUN_1402ceb50(plVar19,iVar11,*(uint *)(lVar18 + lVar24) & 0xffff);
          }
          iVar11 = iVar11 + 1;
          lVar24 = lVar24 + 8;
          lVar18 = plVar4[0x44];
        } while ((ulonglong)(longlong)iVar11 < (ulonglong)(plVar4[0x45] - lVar18 >> 3));
      }
      (**(code **)(*plVar19 + 0x390))(plVar19,1);
    }
  }
  *(longlong **)(param_2 + 8) = plVar19;
  if (0xfffff < (ulonglong)plVar19[1]) {
    FUN_142e541f0(0x30f);
  }
  LOCK();
  plVar19[1] = plVar19[1] + 1;
  UNLOCK();
  return param_2;
}



//===========================================================
// FUN_1426d2c60 @ 1426d2c60   (1816 bytes)
//===========================================================

void FUN_1426d2c60(undefined4 param_1,undefined8 param_2)

{
  char *pcVar1;
  longlong lVar2;
  undefined2 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong *plVar6;
  longlong lVar7;
  undefined4 *puVar8;
  int iVar9;
  int iVar10;
  char *pcVar11;
  int iVar12;
  int *piVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  int iVar16;
  longlong lVar17;
  undefined4 uVar18;
  longlong *local_res18;
  char *local_res20;
  longlong *local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong *local_58;
  longlong *plStack_50;
  longlong local_48;
  
  FUN_14039f600(DAT_143aa8328,&local_88,param_1,0);
  if (local_88 == (longlong *)0x0) goto LAB_1426d334f;
  local_res18 = local_88;
  (**(code **)(*local_88 + 8))();
  uVar3 = FUN_1403e54e0(&local_res18);
  lVar4 = FUN_141ed3540(param_1);
  if (lVar4 == 0) goto LAB_1426d334f;
  FUN_141ed1c20(&local_58,uVar3);
  uVar5 = FUN_1408a9e40(&local_60,0x9dd);
  uVar5 = FUN_1401aa220(&local_68,&DAT_14329e0d0,uVar5);
  plVar6 = (longlong *)FUN_1408a9e40(&local_70,0x9dc);
  lVar4 = *plVar6;
  if (lVar4 == 0) {
    uVar18 = 0;
  }
  else {
    uVar18 = *(undefined4 *)(lVar4 + -8);
  }
  FUN_1401abc80(uVar5,&local_res20,lVar4,uVar18);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  pcVar11 = local_res20;
  if ((longlong)plStack_50 - (longlong)local_58 >> 3 != 0) {
    uVar14 = 0xffffffffffffffff;
    do {
      uVar15 = uVar14 + 1;
      pcVar1 = &DAT_143275ba1 + uVar14;
      uVar14 = uVar15;
    } while (*pcVar1 != '\0');
    iVar16 = (int)uVar15;
    if (iVar16 != 0) {
      if (local_res20 == (char *)0x0) {
LAB_1426d2e39:
        piVar13 = (int *)0x0;
LAB_1426d2e3b:
        iVar12 = 0;
LAB_1426d2e3d:
        if (iVar12 < iVar16) {
          iVar12 = iVar16;
        }
        puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar12 + 0x11));
        puVar8[1] = iVar12;
        *puVar8 = 0xffffffff;
        pcVar11 = (char *)(puVar8 + 4);
        puVar8[2] = 0;
        *pcVar11 = '\0';
        local_res20 = pcVar11;
        if (piVar13 != (int *)0x0) {
          FUN_14019f2c0(piVar13);
        }
      }
      else {
        if (*local_res20 != '\0') {
          iVar12 = *(int *)(local_res20 + -8);
          for (iVar9 = *(int *)(local_res20 + -0xc); iVar9 < iVar12 + iVar16; iVar9 = iVar9 * 2) {
          }
          lVar4 = FUN_14019bd40(&local_res20,iVar9,1);
          pcVar11 = local_res20;
          if (local_res20 == (char *)0x0) {
            iVar9 = 0;
          }
          else {
            iVar9 = *(int *)(local_res20 + -8);
          }
          FUN_142ef7ba0(iVar9 + lVar4,&DAT_143275ba0,(longlong)iVar16);
          FUN_14019c870(&local_res20,iVar12 + iVar16);
          goto LAB_1426d2f08;
        }
        if (local_res20 == (char *)0x0) goto LAB_1426d2e39;
        piVar13 = (int *)(local_res20 + -0x10);
        if (piVar13 == (int *)0x0) goto LAB_1426d2e3b;
        if ((1 < *piVar13) || (*(int *)(local_res20 + -0xc) < iVar16)) {
          iVar12 = *(int *)(local_res20 + -8);
          goto LAB_1426d2e3d;
        }
        if (*piVar13 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar13 = -1;
      }
      FUN_142ef7ba0(pcVar11,&DAT_143275ba0,(longlong)iVar16);
      if (*(int *)(pcVar11 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= *(int *)(pcVar11 + -0xc))) {
        pcVar11[-0x10] = '\x01';
        pcVar11[-0xf] = '\0';
        pcVar11[-0xe] = '\0';
        pcVar11[-0xd] = '\0';
        if (iVar16 != -1) goto LAB_1426d2eb9;
        if (pcVar11 == (char *)0x0) {
          uVar15 = 0;
        }
        else {
          uVar15 = 0xffffffffffffffff;
          do {
            uVar15 = uVar15 + 1;
          } while (pcVar11[uVar15] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar11 + -0xc),uVar15 & 0xffffffff);
        pcVar11[-0x10] = '\x01';
        pcVar11[-0xf] = '\0';
        pcVar11[-0xe] = '\0';
        pcVar11[-0xd] = '\0';
LAB_1426d2eb9:
        pcVar11[iVar16] = '\0';
      }
      iVar16 = (int)uVar15;
      if ((iVar16 < 0) || (*(int *)(pcVar11 + -0xc) + 1 <= iVar16)) {
        FUN_142e54290(0x9c,uVar15 & 0xffffffff);
      }
      *(int *)(pcVar11 + -8) = iVar16;
    }
  }
LAB_1426d2f08:
  iVar16 = 0;
  lVar4 = (longlong)plStack_50 - (longlong)local_58 >> 3;
  local_78 = lVar4;
  if (0 < (int)lVar4) {
    local_res18 = (longlong *)CONCAT44(local_res18._4_4_,(int)lVar4 + -1);
    lVar17 = 0;
    do {
      local_80 = 0;
      FUN_14019a260(&local_80,(longlong)local_58 + lVar17);
      lVar2 = local_80;
      if (local_80 != 0) {
        iVar12 = *(int *)(local_80 + -8);
        uVar14 = (ulonglong)iVar12;
        if (iVar12 != 0) {
          if (pcVar11 == (char *)0x0) {
LAB_1426d2ffe:
            piVar13 = (int *)0x0;
LAB_1426d3001:
            iVar9 = 0;
LAB_1426d3003:
            if (iVar9 < iVar12) {
              iVar9 = iVar12;
            }
            puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
            puVar8[1] = iVar9;
            *puVar8 = 0xffffffff;
            pcVar11 = (char *)(puVar8 + 4);
            puVar8[2] = 0;
            *pcVar11 = '\0';
            local_res20 = pcVar11;
            if (piVar13 != (int *)0x0) {
              FUN_14019f2c0(piVar13);
            }
          }
          else {
            if (*pcVar11 != '\0') {
              iVar9 = *(int *)(pcVar11 + -8);
              for (iVar10 = *(int *)(pcVar11 + -0xc); iVar10 < iVar9 + iVar12; iVar10 = iVar10 * 2)
              {
              }
              lVar4 = FUN_14019bd40(&local_res20,iVar10,1);
              pcVar11 = local_res20;
              if (local_res20 == (char *)0x0) {
                iVar10 = 0;
              }
              else {
                iVar10 = *(int *)(local_res20 + -8);
              }
              FUN_142ef7ba0(iVar10 + lVar4,lVar2,uVar14);
              FUN_14019c870(&local_res20,iVar9 + iVar12);
              lVar4 = local_78;
              goto LAB_1426d30ae;
            }
            if (pcVar11 == (char *)0x0) goto LAB_1426d2ffe;
            piVar13 = (int *)(pcVar11 + -0x10);
            if (piVar13 == (int *)0x0) goto LAB_1426d3001;
            if ((1 < *piVar13) || (*(int *)(pcVar11 + -0xc) < iVar12)) {
              iVar9 = *(int *)(pcVar11 + -8);
              goto LAB_1426d3003;
            }
            if (*piVar13 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar13 = -1;
          }
          FUN_142ef7ba0(pcVar11,lVar2,uVar14);
          if (*(int *)(pcVar11 + -0x10) != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar12 == -1) || (iVar12 <= *(int *)(pcVar11 + -0xc))) {
            pcVar11[-0x10] = '\x01';
            pcVar11[-0xf] = '\0';
            pcVar11[-0xe] = '\0';
            pcVar11[-0xd] = '\0';
            if (iVar12 != -1) goto LAB_1426d3087;
            if (pcVar11 == (char *)0x0) {
              uVar14 = 0;
            }
            else {
              uVar14 = 0xffffffffffffffff;
              do {
                uVar14 = uVar14 + 1;
              } while (pcVar11[uVar14] != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,*(int *)(pcVar11 + -0xc),iVar12);
            pcVar11[-0x10] = '\x01';
            pcVar11[-0xf] = '\0';
            pcVar11[-0xe] = '\0';
            pcVar11[-0xd] = '\0';
LAB_1426d3087:
            pcVar11[uVar14] = '\0';
          }
          iVar12 = (int)uVar14;
          if ((iVar12 < 0) || (*(int *)(pcVar11 + -0xc) + 1 <= iVar12)) {
            FUN_142e54290(0x9c,uVar14 & 0xffffffff);
          }
          *(int *)(pcVar11 + -8) = iVar12;
          lVar4 = local_78;
        }
      }
LAB_1426d30ae:
      if (iVar16 < (int)local_res18) {
        if (pcVar11 == (char *)0x0) {
LAB_1426d317c:
          piVar13 = (int *)0x0;
LAB_1426d317e:
          iVar12 = 0;
LAB_1426d3180:
          if (iVar12 < 2) {
            iVar12 = 2;
          }
          puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar12 + 0x11));
          puVar8[1] = iVar12;
          *puVar8 = 0xffffffff;
          pcVar11 = (char *)(puVar8 + 4);
          puVar8[2] = 0;
          *pcVar11 = '\0';
          local_res20 = pcVar11;
          if (piVar13 != (int *)0x0) {
            FUN_14019f2c0(piVar13);
          }
        }
        else {
          if (*pcVar11 != '\0') {
            iVar12 = *(int *)(pcVar11 + -8);
            for (iVar9 = *(int *)(pcVar11 + -0xc); iVar9 < iVar12 + 2; iVar9 = iVar9 * 2) {
            }
            lVar7 = FUN_14019bd40(&local_res20,iVar9,1);
            pcVar11 = local_res20;
            if (local_res20 == (char *)0x0) {
              iVar9 = 0;
            }
            else {
              iVar9 = *(int *)(local_res20 + -8);
            }
            *(undefined2 *)(iVar9 + lVar7) = 0x202c;
            FUN_14019c870(&local_res20,iVar12 + 2);
            goto LAB_1426d321c;
          }
          if (pcVar11 == (char *)0x0) goto LAB_1426d317c;
          piVar13 = (int *)(pcVar11 + -0x10);
          if (piVar13 == (int *)0x0) goto LAB_1426d317e;
          if ((1 < *piVar13) || (*(int *)(pcVar11 + -0xc) < 2)) {
            iVar12 = *(int *)(pcVar11 + -8);
            goto LAB_1426d3180;
          }
          if (*piVar13 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar13 = -1;
        }
        pcVar11[0] = ',';
        pcVar11[1] = ' ';
        if (*(int *)(pcVar11 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (*(int *)(pcVar11 + -0xc) < 2) {
          FUN_142e54290(0x90,*(int *)(pcVar11 + -0xc),2);
        }
        pcVar11[-0x10] = '\x01';
        pcVar11[-0xf] = '\0';
        pcVar11[-0xe] = '\0';
        pcVar11[-0xd] = '\0';
        pcVar11[2] = '\0';
        if (*(int *)(pcVar11 + -0xc) + 1 < 3) {
          FUN_142e54290(0x9c,2);
        }
        pcVar11[-8] = '\x02';
        pcVar11[-7] = '\0';
        pcVar11[-6] = '\0';
        pcVar11[-5] = '\0';
      }
LAB_1426d321c:
      if (lVar2 != 0) {
        FUN_14019f2c0(lVar2 + -0x10);
      }
      iVar16 = iVar16 + 1;
      lVar17 = lVar17 + 8;
    } while (iVar16 < (int)lVar4);
  }
  lVar4 = -1;
  do {
    pcVar1 = &DAT_143295a71 + lVar4;
    lVar4 = lVar4 + 1;
  } while (*pcVar1 != '\0');
  FUN_14019d8c0(param_2);
  lVar4 = -1;
  do {
    lVar17 = lVar4 + 1;
    pcVar1 = &DAT_14329e0d1 + lVar4;
    lVar4 = lVar17;
  } while (*pcVar1 != '\0');
  FUN_1401abc80(&local_res20,&local_res18,&DAT_14329e0d0,lVar17);
  if (local_res18 == (longlong *)0x0) {
    uVar18 = 0;
    plVar6 = (longlong *)0x0;
  }
  else {
    uVar18 = (undefined4)local_res18[-1];
    plVar6 = local_res18;
  }
  uVar18 = FUN_14019d8c0(param_2,plVar6,uVar18);
  if (plVar6 != (longlong *)0x0) {
    uVar18 = FUN_14019f2c0(plVar6 + -2);
  }
  if (pcVar11 != (char *)0x0) {
    uVar18 = FUN_14019f2c0(pcVar11 + -0x10);
  }
  plVar6 = local_58;
  if (local_58 != (longlong *)0x0) {
    for (; plVar6 != plStack_50; plVar6 = plVar6 + 1) {
      if (*plVar6 != 0) {
        uVar18 = FUN_14019f2c0(*plVar6 + -0x10);
      }
    }
    uVar14 = (local_48 - (longlong)local_58 >> 3) * 8;
    plVar6 = local_58;
    if (0xfff < uVar14) {
      plVar6 = (longlong *)local_58[-1];
      if (0x1f < (ulonglong)((longlong)local_58 + (-8 - (longlong)plVar6))) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(uVar18,uVar14 + 0x27);
      }
    }
    thunk_FUN_140205820(plVar6);
    local_58 = (longlong *)0x0;
    plStack_50 = (longlong *)0x0;
    local_48 = 0;
  }
LAB_1426d334f:
  if (local_88 != (longlong *)0x0) {
    (**(code **)(*local_88 + 0x10))();
  }
  return;
}


