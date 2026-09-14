
//===========================================================
// FUN_1409d6150 @ 1409d6150   (698 bytes)
//===========================================================

/* WARNING: Heritage AFTER dead removal. Example location: s0x00000020 : 0x0001409d639f */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

ulonglong FUN_1409d6150(int *param_1)

{
  ushort uVar1;
  byte *pbVar2;
  longlong lVar3;
  undefined4 *puVar4;
  undefined1 uVar5;
  byte bVar6;
  undefined8 *puVar7;
  undefined4 *puVar8;
  int iVar9;
  int iVar10;
  ulonglong uVar11;
  byte local_res8 [8];
  undefined1 local_res10 [8];
  byte local_res18 [8];
  byte local_res20;
  ushort local_68 [2];
  undefined4 local_64;
  ulonglong local_60;
  undefined8 local_58;
  longlong local_50 [2];
  
  pbVar2 = *(byte **)(param_1 + 2);
  local_res8[0] = *pbVar2;
  local_res18[0] = pbVar2[1];
  if (pbVar2[1] == 0) {
    local_res18[0] = 0x2a;
  }
  local_res8[0] = *pbVar2 ^ local_res18[0];
  local_res18[0] = *pbVar2 + 0x2a + local_res18[0];
  local_68[0] = local_res18[0] + 4 | 0xd328;
  FUN_140c78f50(0x56,(longlong)param_1 + 0x21a3f060fc2daf);
  bVar6 = local_res8[0];
  lVar3 = *(longlong *)(param_1 + 2);
  uVar11 = (ulonglong)local_res8[0];
  if (((local_68[0] != *(ushort *)(lVar3 + 4)) || ((char)param_1[1] != *(char *)(lVar3 + 2))) ||
     ((char)param_1[4] != *(char *)(lVar3 + 3))) {
    local_res8[0] = *(byte *)(param_1 + 4);
    local_res10[0] = (undefined1)param_1[1];
    local_64 = 1;
    local_60 = uVar11;
    local_58 = FUN_1418039d0(5);
    puVar7 = (undefined8 *)
             FUN_140197ac0(local_50,&local_58,&local_64,&local_60,local_res18,local_68,
                           (ushort *)(lVar3 + 4),local_res10,lVar3 + 2,local_res8,lVar3 + 3);
    FUN_141804970(&DAT_143271f04,0x61,5,*puVar7);
    if (local_50[0] != 0) {
      FUN_14019f2c0(local_50[0] + -0x10);
    }
  }
  iVar9 = *param_1;
  iVar10 = iVar9 + 1;
  *param_1 = iVar10;
  if (iVar10 == (iVar10 / 0x37) * 0x37) {
    local_res20 = bVar6;
    iVar9 = iVar9 + 2;
    *param_1 = iVar9;
    if (iVar9 == (iVar9 / 0x6f) * 0x6f) {
      puVar4 = *(undefined4 **)(param_1 + 2);
      puVar8 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,6);
      *(undefined4 **)(param_1 + 2) = puVar8;
      *puVar8 = *puVar4;
      *(undefined2 *)(puVar8 + 1) = *(undefined2 *)(puVar4 + 1);
      thunk_FUN_140205820(puVar4,6);
    }
    uVar5 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(param_1 + 2) + 1) = uVar5;
    pbVar2 = *(byte **)(param_1 + 2);
    bVar6 = pbVar2[1];
    pbVar2[4] = 0x65;
    pbVar2[5] = 0x9a;
    if (bVar6 == 0) {
      bVar6 = 0x2a;
    }
    *pbVar2 = bVar6 ^ local_res20;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 2) + 4);
    *(ushort *)(*(longlong *)(param_1 + 2) + 4) =
         (uVar1 >> 0xd) + (ushort)(byte)(bVar6 + (bVar6 ^ local_res20) + 0x2a) | uVar1 << 3;
  }
  FUN_140c79130(0x67,param_1 + 0xc0a84bdc4a);
  return uVar11;
}



//===========================================================
// FUN_14182ffd0 @ 14182ffd0   (15 bytes)
//===========================================================

undefined1 FUN_14182ffd0(longlong param_1)

{
  return *(undefined1 *)(*(longlong *)(param_1 + 0xa8) + 0x2d9);
}



//===========================================================
// FUN_1409bd2f0 @ 1409bd2f0   (3792 bytes)
//===========================================================

ulonglong FUN_1409bd2f0(longlong param_1)

{
  longlong *plVar1;
  double *pdVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  double dVar11;
  double dVar12;
  undefined1 uVar13;
  byte bVar14;
  ushort uVar15;
  ushort uVar16;
  ulonglong uVar17;
  undefined8 *puVar18;
  undefined4 *puVar19;
  byte bVar20;
  byte bVar21;
  int iVar22;
  longlong lVar23;
  byte bVar24;
  longlong lVar25;
  byte *pbVar26;
  uint uVar27;
  ushort uVar28;
  ushort uVar29;
  ushort uVar30;
  int iVar31;
  ushort uVar32;
  ushort uVar33;
  ushort uVar34;
  byte *pbVar35;
  undefined1 local_res8 [8];
  undefined1 local_res10 [8];
  byte local_res18 [8];
  uint local_res20 [2];
  undefined1 local_118;
  byte local_117 [3];
  ushort local_114 [2];
  ushort local_110 [4];
  undefined8 local_108;
  undefined8 local_100;
  uint local_f8;
  undefined8 local_f0;
  int local_e8;
  int local_e4;
  undefined4 local_e0;
  undefined4 local_dc;
  undefined8 local_d8;
  undefined1 local_d0 [8];
  undefined8 *local_c8;
  longlong local_c0;
  undefined8 local_b8;
  longlong local_b0;
  longlong local_a8;
  undefined8 local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  
  uVar17 = (**(code **)**(undefined8 **)(param_1 + 0x100))();
  if (((((((int)uVar17 == 0) ||
         (uVar17 = (**(code **)**(undefined8 **)(param_1 + 0x100))(), (int)uVar17 == 4)) ||
        (uVar17 = (**(code **)**(undefined8 **)(param_1 + 0x100))(), (int)uVar17 == 5)) ||
       ((uVar17 = (**(code **)**(undefined8 **)(param_1 + 0x100))(), (int)uVar17 == 8 ||
        (uVar17 = (**(code **)**(undefined8 **)(param_1 + 0x100))(), (int)uVar17 == 9)))) ||
      (uVar17 = (**(code **)**(undefined8 **)(param_1 + 0x100))(), (int)uVar17 == 0xb)) &&
     (DAT_143ac18d8 != 0)) {
    FUN_142df6c00(DAT_143ac18d8,local_d0);
    if ((local_c8 == (undefined8 *)0x0) || (local_c8[0x22] == 0)) {
      uVar17 = 0;
    }
    else {
      pdVar2 = *(double **)(param_1 + 0x178);
      local_d8 = *pdVar2;
      local_f8 = 0;
      uVar27 = 0;
      local_res18[0] = *(byte *)(pdVar2 + 1);
      local_114[0] = 0x9a65;
      pbVar35 = (byte *)&local_d8;
      pbVar26 = (byte *)((longlong)pdVar2 + 2);
      do {
        if (local_res18[0] == 0) {
          local_res18[0] = 0x2a;
        }
        *pbVar35 = local_res18[0] ^ pbVar35[(longlong)pdVar2 - (longlong)&local_d8];
        bVar24 = local_res18[0] + 0x2a + pbVar35[(longlong)pdVar2 - (longlong)&local_d8];
        uVar28 = (local_114[0] >> 0xd) + (ushort)bVar24;
        uVar32 = local_114[0] << 3;
        if (bVar24 == 0) {
          bVar24 = 0x2a;
        }
        pbVar35[1] = bVar24 ^ pbVar26[-1];
        bVar24 = bVar24 + 0x2a + pbVar26[-1];
        uVar33 = ((local_114[0] & 0x1fff) >> 10) + (ushort)bVar24;
        uVar29 = (uVar28 | uVar32) << 3;
        if (bVar24 == 0) {
          bVar24 = 0x2a;
        }
        pbVar35[2] = bVar24 ^ *pbVar26;
        bVar24 = bVar24 + 0x2a + *pbVar26;
        uVar30 = ((uVar32 & 0x1fff) >> 10) + (ushort)bVar24;
        uVar34 = (uVar33 | uVar29) << 3;
        if (bVar24 == 0) {
          bVar24 = 0x2a;
        }
        pbVar35[3] = bVar24 ^ pbVar26[1];
        bVar24 = bVar24 + 0x2a + pbVar26[1];
        uVar32 = ((uVar28 | uVar32 & 0x3ff) >> 7) + (ushort)bVar24;
        uVar28 = (uVar30 | uVar34) << 3;
        if (bVar24 == 0) {
          bVar24 = 0x2a;
        }
        pbVar35[4] = bVar24 ^ pbVar26[2];
        bVar24 = bVar24 + 0x2a + pbVar26[2];
        uVar29 = ((uVar33 | uVar29 & 0x3ff) >> 7) + (ushort)bVar24;
        uVar33 = (uVar32 | uVar28) << 3;
        if (bVar24 == 0) {
          bVar24 = 0x2a;
        }
        pbVar35[5] = bVar24 ^ pbVar26[3];
        bVar24 = bVar24 + 0x2a + pbVar26[3];
        uVar15 = (ushort)bVar24;
        if (bVar24 == 0) {
          bVar24 = 0x2a;
        }
        pbVar35[6] = bVar24 ^ pbVar26[4];
        bVar24 = bVar24 + 0x2a + pbVar26[4];
        uVar16 = (ushort)bVar24;
        if (bVar24 == 0) {
          bVar24 = 0x2a;
        }
        pbVar35[7] = bVar24 ^ pbVar26[5];
        local_res18[0] = bVar24 + 0x2a + pbVar26[5];
        local_114[0] = ((uVar29 | uVar33 & 0x3ff) >> 7) + (ushort)local_res18[0] |
                       (((uVar32 | uVar28 & 0x3ff) >> 7) + uVar16 |
                       (((uVar30 | uVar34 & 0x3ff) >> 7) + uVar15 | (uVar29 | uVar33) << 3) << 3) <<
                       3;
        uVar27 = uVar27 + 8;
        pbVar35 = pbVar35 + 8;
        pbVar26 = pbVar26 + 8;
      } while (uVar27 < 8);
      FUN_140c78f50(0x171,param_1 + 0x21a3f060fc2f1f);
      dVar12 = local_d8;
      lVar23 = *(longlong *)(param_1 + 0x178);
      if (((local_114[0] != *(ushort *)(lVar23 + 0xc)) ||
          (*(char *)(param_1 + 0x174) != *(char *)(lVar23 + 9))) ||
         (*(char *)(param_1 + 0x180) != *(char *)(lVar23 + 10))) {
        local_res8[0] = *(undefined1 *)(param_1 + 0x180);
        local_res10[0] = *(undefined1 *)(param_1 + 0x174);
        local_c0 = (longlong)local_d8;
        local_e0 = 2;
        local_b8 = FUN_1418039d0(5);
        puVar18 = (undefined8 *)
                  FUN_140197ac0(&local_b0,&local_b8,&local_e0,&local_c0,local_res18,local_114,
                                (ushort *)(lVar23 + 0xc),local_res10,lVar23 + 9,local_res8,
                                lVar23 + 10);
        FUN_141804970(&DAT_143271f04,0x17c,5,*puVar18);
        if (local_b0 != 0) {
          FUN_14019f2c0(local_b0 + -0x10);
        }
      }
      iVar22 = *(int *)(param_1 + 0x170);
      iVar31 = iVar22 + 1;
      *(int *)(param_1 + 0x170) = iVar31;
      if (iVar31 == (iVar31 / 0x37) * 0x37) {
        local_100 = dVar12;
        iVar22 = iVar22 + 2;
        *(int *)(param_1 + 0x170) = iVar22;
        if (iVar22 == (iVar22 / 0x6f) * 0x6f) {
          puVar3 = *(undefined4 **)(param_1 + 0x178);
          puVar19 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,0x10);
          *(undefined4 **)(param_1 + 0x178) = puVar19;
          uVar4 = puVar3[1];
          uVar5 = puVar3[2];
          uVar6 = puVar3[3];
          *puVar19 = *puVar3;
          puVar19[1] = uVar4;
          puVar19[2] = uVar5;
          puVar19[3] = uVar6;
          thunk_FUN_140205820(puVar3,0x10);
        }
        uVar13 = FUN_142f04924();
        *(undefined1 *)(*(longlong *)(param_1 + 0x178) + 8) = uVar13;
        pbVar35 = *(byte **)(param_1 + 0x178);
        bVar24 = pbVar35[8];
        pbVar35[0xc] = 0x65;
        pbVar35[0xd] = 0x9a;
        local_res20[0] = 0;
        lVar25 = (longlong)&local_100 - (longlong)pbVar35;
        lVar23 = 1 - (longlong)pbVar35;
        lVar7 = 2 - (longlong)pbVar35;
        lVar8 = 3 - (longlong)pbVar35;
        lVar9 = 4 - (longlong)pbVar35;
        lVar10 = 5 - (longlong)pbVar35;
        local_f0 = (double)((longlong)&local_100 + (6 - (longlong)pbVar35));
        local_108 = (double)((longlong)&local_100 + (7 - (longlong)pbVar35));
        do {
          if (bVar24 == 0) {
            bVar24 = 0x2a;
          }
          bVar14 = pbVar35[lVar25];
          *pbVar35 = bVar24 ^ bVar14;
          bVar24 = bVar24 + (bVar24 ^ bVar14) + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar24 | uVar28 << 3;
          bVar14 = 0x2a;
          if (bVar24 != 0) {
            bVar14 = bVar24;
          }
          bVar24 = pbVar35[(longlong)&local_100 + lVar23];
          pbVar35[1] = bVar14 ^ bVar24;
          bVar14 = (bVar14 ^ bVar24) + bVar14 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)&local_100 + lVar7];
          pbVar35[2] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)&local_100 + lVar8];
          pbVar35[3] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)&local_100 + lVar9];
          pbVar35[4] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)&local_100 + lVar10];
          pbVar35[5] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)local_f0];
          pbVar35[6] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)local_108];
          pbVar35[7] = bVar24 ^ bVar14;
          bVar24 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x178) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar24 | uVar28 << 3;
          local_res20[0] = local_res20[0] + 8;
          pbVar35 = pbVar35 + 8;
        } while (local_res20[0] < 8);
      }
      bVar24 = 0x2a;
      uVar27 = 0;
      FUN_140c79130(0x182,param_1 + 0x302a12f7298);
      dVar11 = DAT_143304308;
      if (dVar12 < 0.0) {
        local_e8 = -(int)(DAT_143304308 - dVar12);
      }
      else {
        local_e8 = (int)(dVar12 + DAT_1434b95d8);
      }
      pdVar2 = *(double **)(param_1 + 0x130);
      local_f0 = *pdVar2;
      local_117[0] = *(byte *)(pdVar2 + 1);
      local_110[0] = 0x9a65;
      pbVar35 = (byte *)&local_f0;
      pbVar26 = (byte *)((longlong)pdVar2 + 2);
      do {
        if (local_117[0] == 0) {
          local_117[0] = 0x2a;
        }
        bVar14 = pbVar35[(longlong)pdVar2 - (longlong)&local_f0];
        *pbVar35 = local_117[0] ^ bVar14;
        local_117[0] = local_117[0] + bVar14 + 0x2a;
        uVar28 = (local_110[0] >> 0xd) + (ushort)local_117[0];
        uVar32 = local_110[0] << 3;
        if (local_117[0] == 0) {
          local_117[0] = bVar24;
        }
        bVar14 = pbVar26[-1];
        pbVar35[1] = local_117[0] ^ bVar14;
        local_117[0] = local_117[0] + bVar14 + 0x2a;
        uVar33 = ((local_110[0] & 0x1fff) >> 10) + (ushort)local_117[0];
        uVar29 = (uVar28 | uVar32) << 3;
        if (local_117[0] == 0) {
          local_117[0] = bVar24;
        }
        bVar14 = *pbVar26;
        pbVar35[2] = bVar14 ^ local_117[0];
        bVar14 = bVar14 + local_117[0] + 0x2a;
        uVar30 = ((uVar32 & 0x1fff) >> 10) + (ushort)bVar14;
        uVar34 = (uVar33 | uVar29) << 3;
        if (bVar14 == 0) {
          bVar14 = bVar24;
        }
        bVar20 = pbVar26[1];
        pbVar35[3] = bVar20 ^ bVar14;
        bVar20 = bVar20 + bVar14 + 0x2a;
        uVar32 = ((uVar28 | uVar32 & 0x3ff) >> 7) + (ushort)bVar20;
        uVar28 = (uVar30 | uVar34) << 3;
        if (bVar20 == 0) {
          bVar20 = bVar24;
        }
        bVar14 = pbVar26[2];
        pbVar35[4] = bVar14 ^ bVar20;
        bVar14 = bVar14 + bVar20 + 0x2a;
        uVar29 = ((uVar33 | uVar29 & 0x3ff) >> 7) + (ushort)bVar14;
        uVar33 = (uVar32 | uVar28) << 3;
        if (bVar14 == 0) {
          bVar14 = bVar24;
        }
        bVar20 = pbVar26[3];
        pbVar35[5] = bVar20 ^ bVar14;
        bVar20 = bVar20 + bVar14 + 0x2a;
        bVar14 = bVar20;
        if (bVar20 == 0) {
          bVar14 = bVar24;
        }
        bVar21 = pbVar26[4];
        pbVar35[6] = bVar21 ^ bVar14;
        bVar21 = bVar21 + bVar14 + 0x2a;
        local_117[0] = bVar21;
        if (bVar21 == 0) {
          local_117[0] = bVar24;
        }
        bVar14 = pbVar26[5];
        pbVar35[7] = bVar14 ^ local_117[0];
        local_117[0] = bVar14 + 0x2a + local_117[0];
        local_110[0] = ((uVar29 | uVar33 & 0x3ff) >> 7) + (ushort)local_117[0] |
                       (((uVar32 | uVar28 & 0x3ff) >> 7) + (ushort)bVar21 |
                       (((uVar30 | uVar34 & 0x3ff) >> 7) + (ushort)bVar20 | (uVar29 | uVar33) << 3)
                       << 3) << 3;
        uVar27 = uVar27 + 8;
        pbVar35 = pbVar35 + 8;
        pbVar26 = pbVar26 + 8;
      } while (uVar27 < 8);
      FUN_140c78f50(0x171,param_1 + 0x21a3f060fc2ed7);
      dVar12 = local_f0;
      lVar23 = *(longlong *)(param_1 + 0x130);
      if (((local_110[0] != *(ushort *)(lVar23 + 0xc)) ||
          (*(char *)(param_1 + 300) != *(char *)(lVar23 + 9))) ||
         (*(char *)(param_1 + 0x138) != *(char *)(lVar23 + 10))) {
        local_res20[0] = CONCAT31(local_res20[0]._1_3_,*(undefined1 *)(param_1 + 0x138));
        local_118 = *(undefined1 *)(param_1 + 300);
        local_a8 = (longlong)local_f0;
        local_dc = 2;
        local_a0 = FUN_1418039d0(5);
        puVar18 = (undefined8 *)
                  FUN_140197ac0(&local_98,&local_a0,&local_dc,&local_a8,local_117,local_110,
                                (ushort *)(lVar23 + 0xc),&local_118,lVar23 + 9,local_res20,
                                lVar23 + 10);
        FUN_141804970(&DAT_143271f04,0x17c,5,*puVar18);
        if (local_98 != 0) {
          FUN_14019f2c0(local_98 + -0x10);
        }
      }
      iVar22 = *(int *)(param_1 + 0x128);
      iVar31 = iVar22 + 1;
      *(int *)(param_1 + 0x128) = iVar31;
      if (iVar31 == (iVar31 / 0x37) * 0x37) {
        local_108 = dVar12;
        iVar22 = iVar22 + 2;
        *(int *)(param_1 + 0x128) = iVar22;
        if (iVar22 == (iVar22 / 0x6f) * 0x6f) {
          puVar3 = *(undefined4 **)(param_1 + 0x130);
          puVar19 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,0x10);
          *(undefined4 **)(param_1 + 0x130) = puVar19;
          uVar4 = puVar3[1];
          uVar5 = puVar3[2];
          uVar6 = puVar3[3];
          *puVar19 = *puVar3;
          puVar19[1] = uVar4;
          puVar19[2] = uVar5;
          puVar19[3] = uVar6;
          thunk_FUN_140205820(puVar3,0x10);
        }
        uVar13 = FUN_142f04924();
        *(undefined1 *)(*(longlong *)(param_1 + 0x130) + 8) = uVar13;
        pbVar35 = *(byte **)(param_1 + 0x130);
        bVar24 = pbVar35[8];
        pbVar35[0xc] = 0x65;
        pbVar35[0xd] = 0x9a;
        lVar25 = (longlong)&local_108 - (longlong)pbVar35;
        lVar23 = 1 - (longlong)pbVar35;
        lVar7 = 2 - (longlong)pbVar35;
        lVar8 = 3 - (longlong)pbVar35;
        lVar9 = 4 - (longlong)pbVar35;
        lVar10 = 5 - (longlong)pbVar35;
        local_90 = (longlong)&local_108 + (6 - (longlong)pbVar35);
        local_88 = (longlong)&local_108 + (7 - (longlong)pbVar35);
        do {
          if (bVar24 == 0) {
            bVar24 = 0x2a;
          }
          bVar14 = pbVar35[lVar25];
          *pbVar35 = bVar24 ^ bVar14;
          bVar24 = bVar24 + (bVar24 ^ bVar14) + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar24 | uVar28 << 3;
          bVar14 = 0x2a;
          if (bVar24 != 0) {
            bVar14 = bVar24;
          }
          bVar24 = pbVar35[(longlong)&local_108 + lVar23];
          pbVar35[1] = bVar14 ^ bVar24;
          bVar14 = (bVar14 ^ bVar24) + bVar14 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)&local_108 + lVar7];
          pbVar35[2] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)&local_108 + lVar8];
          pbVar35[3] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)&local_108 + lVar9];
          pbVar35[4] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[(longlong)&local_108 + lVar10];
          pbVar35[5] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[local_90];
          pbVar35[6] = bVar24 ^ bVar14;
          bVar14 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar14 | uVar28 << 3;
          bVar24 = 0x2a;
          if (bVar14 != 0) {
            bVar24 = bVar14;
          }
          bVar14 = pbVar35[local_88];
          pbVar35[7] = bVar24 ^ bVar14;
          bVar24 = (bVar24 ^ bVar14) + bVar24 + 0x2a;
          uVar28 = *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc);
          *(ushort *)(*(longlong *)(param_1 + 0x130) + 0xc) =
               (uVar28 >> 0xd) + (ushort)bVar24 | uVar28 << 3;
          local_f8 = local_f8 + 8;
          pbVar35 = pbVar35 + 8;
        } while (local_f8 < 8);
      }
      FUN_140c79130(0x182,param_1 + 0x302a12f7250);
      if (dVar12 < 0.0) {
        local_e4 = -(int)(dVar11 - dVar12);
      }
      else {
        local_e4 = (int)(dVar12 + DAT_1434b95d8);
      }
      if (local_c8 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      puVar18 = local_c8;
      lVar23 = local_c8[0x22];
      if (lVar23 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar23 = puVar18[0x22];
      }
      uVar17 = FUN_140364ca0(lVar23,&local_e8);
      puVar18 = local_c8;
      if ((char)uVar17 == '\0') {
        if (local_c8 != (undefined8 *)0x0) {
          if (0xffffe < local_c8[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar1 = puVar18 + 1;
          lVar23 = *plVar1;
          *plVar1 = *plVar1 + -1;
          UNLOCK();
          uVar17 = lVar23 - 1;
          if (((int)uVar17 == 0) && (local_c8 != (undefined8 *)0x0)) {
            uVar17 = (**(code **)*local_c8)(local_c8,1);
          }
        }
        goto LAB_1409be145;
      }
      uVar17 = 1;
    }
    puVar18 = local_c8;
    if (local_c8 != (undefined8 *)0x0) {
      if (0xffffe < local_c8[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = puVar18 + 1;
      lVar23 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if (((int)lVar23 == 1) && (local_c8 != (undefined8 *)0x0)) {
        (**(code **)*local_c8)(local_c8,1);
      }
    }
  }
  else {
LAB_1409be145:
    uVar17 = uVar17 & 0xffffffffffffff00;
  }
  return uVar17;
}



//===========================================================
// FUN_141715f80 @ 141715f80   (64 bytes)
//===========================================================

ulonglong FUN_141715f80(longlong param_1)

{
  int iVar1;
  uint uVar2;
  ulonglong in_RAX;
  
  if ((param_1 != 0) && (*(longlong **)(param_1 + 0x3c18) != (longlong *)0x0)) {
    iVar1 = (**(code **)(**(longlong **)(param_1 + 0x3c18) + 0xa0))();
    in_RAX = (ulonglong)(iVar1 - 0x1aU);
    if ((iVar1 - 0x1aU != 0) && (in_RAX = (ulonglong)(iVar1 - 0x1bU), iVar1 - 0x1bU != 0)) {
      uVar2 = iVar1 - 0x1c;
      in_RAX = (ulonglong)uVar2;
      if ((uVar2 != 0) && (uVar2 != 4)) goto LAB_141715fb9;
    }
    return CONCAT71((int7)(in_RAX >> 8),1);
  }
LAB_141715fb9:
  return in_RAX & 0xffffffffffffff00;
}


