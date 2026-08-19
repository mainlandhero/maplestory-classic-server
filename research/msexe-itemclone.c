
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



//===========================================================
// FUN_1402d4100 @ 1402d4100   (1865 bytes)
//===========================================================

void FUN_1402d4100(undefined4 *param_1,undefined4 *param_2,longlong param_3)

{
  longlong *plVar1;
  ushort uVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  undefined8 uVar23;
  undefined8 uVar24;
  undefined8 uVar25;
  undefined8 uVar26;
  longlong lVar27;
  undefined1 uVar28;
  byte bVar29;
  undefined4 uVar30;
  undefined8 *puVar31;
  undefined8 *puVar32;
  undefined8 *puVar33;
  longlong lVar34;
  longlong lVar35;
  int iVar36;
  longlong lVar37;
  byte bVar38;
  byte *pbVar39;
  uint uVar40;
  undefined8 *puVar41;
  undefined8 local_res10;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  
  *param_1 = *param_2;
  *(undefined8 *)(param_1 + 1) = *(undefined8 *)(param_2 + 1);
  FUN_1408ad0e0(param_1 + 3,0x29,param_2 + 3);
  FUN_1408ad0e0((longlong)param_1 + 0x35,0xc9,(longlong)param_2 + 0x35);
  *(undefined4 *)((longlong)param_1 + 0xfe) = *(undefined4 *)((longlong)param_2 + 0xfe);
  *(undefined4 *)((longlong)param_1 + 0x102) = *(undefined4 *)((longlong)param_2 + 0x102);
  *(undefined4 *)((longlong)param_1 + 0x106) = *(undefined4 *)((longlong)param_2 + 0x106);
  *(undefined4 *)((longlong)param_1 + 0x10a) = *(undefined4 *)((longlong)param_2 + 0x10a);
  *(undefined8 *)((longlong)param_1 + 0x10e) = *(undefined8 *)((longlong)param_2 + 0x10e);
  local_res10 = FUN_14019b780(&DAT_143ad68a0,0x467);
  puVar41 = (undefined8 *)0x0;
  puVar31 = puVar41;
  if (local_res10 != 0) {
    puVar31 = (undefined8 *)FUN_1402f7da0(local_res10);
  }
  if (puVar31 != (undefined8 *)0x0) {
    if (0xfffff < (ulonglong)puVar31[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    puVar31[1] = puVar31[1] + 1;
    UNLOCK();
  }
  uVar30 = FUN_1401b0340(param_3 + 0x20);
  if (puVar31 == (undefined8 *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  local_res10 = CONCAT44(local_res10._4_4_,uVar30);
  iVar36 = *(int *)(puVar31 + 4) + 1;
  *(int *)(puVar31 + 4) = iVar36;
  if (iVar36 == (iVar36 / 0x6f) * 0x6f) {
    puVar3 = (undefined8 *)puVar31[5];
    puVar32 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    puVar31[5] = puVar32;
    *puVar32 = *puVar3;
    *(undefined4 *)(puVar32 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar28 = FUN_142f04924();
  *(undefined1 *)(puVar31[5] + 4) = uVar28;
  pbVar39 = (byte *)puVar31[5];
  bVar38 = pbVar39[4];
  pbVar39[8] = 0x65;
  pbVar39[9] = 0x9a;
  lVar35 = (longlong)&local_res10 - (longlong)pbVar39;
  lVar34 = 1 - (longlong)pbVar39;
  lVar37 = 2 - (longlong)pbVar39;
  lVar27 = 3 - (longlong)pbVar39;
  do {
    if (bVar38 == 0) {
      bVar38 = 0x2a;
    }
    bVar29 = pbVar39[lVar35];
    *pbVar39 = bVar38 ^ bVar29;
    bVar38 = bVar38 + (bVar38 ^ bVar29) + 0x2a;
    uVar2 = *(ushort *)(puVar31[5] + 8);
    *(ushort *)(puVar31[5] + 8) = (uVar2 >> 0xd) + (ushort)bVar38 | uVar2 << 3;
    bVar29 = 0x2a;
    if (bVar38 != 0) {
      bVar29 = bVar38;
    }
    bVar38 = pbVar39[(longlong)&local_res10 + lVar34];
    pbVar39[1] = bVar29 ^ bVar38;
    bVar29 = (bVar29 ^ bVar38) + bVar29 + 0x2a;
    uVar2 = *(ushort *)(puVar31[5] + 8);
    *(ushort *)(puVar31[5] + 8) = (uVar2 >> 0xd) + (ushort)bVar29 | uVar2 << 3;
    bVar38 = 0x2a;
    if (bVar29 != 0) {
      bVar38 = bVar29;
    }
    bVar29 = pbVar39[(longlong)&local_res10 + lVar37];
    pbVar39[2] = bVar38 ^ bVar29;
    bVar29 = (bVar38 ^ bVar29) + bVar38 + 0x2a;
    uVar2 = *(ushort *)(puVar31[5] + 8);
    *(ushort *)(puVar31[5] + 8) = (uVar2 >> 0xd) + (ushort)bVar29 | uVar2 << 3;
    bVar38 = 0x2a;
    if (bVar29 != 0) {
      bVar38 = bVar29;
    }
    bVar29 = pbVar39[(longlong)&local_res10 + lVar27];
    pbVar39[3] = bVar38 ^ bVar29;
    bVar38 = (bVar38 ^ bVar29) + bVar38 + 0x2a;
    uVar2 = *(ushort *)(puVar31[5] + 8);
    *(ushort *)(puVar31[5] + 8) = (uVar2 >> 0xd) + (ushort)bVar38 | uVar2 << 3;
    uVar40 = (int)puVar41 + 4;
    puVar41 = (undefined8 *)(ulonglong)uVar40;
    pbVar39 = pbVar39 + 4;
  } while (uVar40 < 4);
  uVar4 = *(undefined8 *)(param_3 + 0x38);
  if (puVar31 == (undefined8 *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  puVar31[7] = uVar4;
  puVar31[8] = *(undefined8 *)(param_3 + 0x40);
  *(undefined4 *)(puVar31 + 9) = *(undefined4 *)(param_3 + 0x48);
  *(undefined1 *)((longlong)puVar31 + 0x4c) = *(undefined1 *)(param_3 + 0x4c);
  *(undefined8 *)((longlong)puVar31 + 0x4d) = *(undefined8 *)(param_3 + 0x4d);
  lVar34 = 2;
  lVar37 = 2;
  puVar41 = &local_1a8;
  puVar3 = (undefined8 *)(param_3 + 0x62);
  do {
    puVar33 = puVar3;
    puVar32 = puVar41;
    uVar4 = puVar33[1];
    *puVar32 = *puVar33;
    puVar32[1] = uVar4;
    uVar4 = puVar33[3];
    puVar32[2] = puVar33[2];
    puVar32[3] = uVar4;
    uVar4 = puVar33[5];
    puVar32[4] = puVar33[4];
    puVar32[5] = uVar4;
    uVar4 = puVar33[7];
    puVar32[6] = puVar33[6];
    puVar32[7] = uVar4;
    uVar4 = puVar33[9];
    puVar32[8] = puVar33[8];
    puVar32[9] = uVar4;
    uVar4 = puVar33[0xb];
    puVar32[10] = puVar33[10];
    puVar32[0xb] = uVar4;
    uVar4 = puVar33[0xd];
    puVar32[0xc] = puVar33[0xc];
    puVar32[0xd] = uVar4;
    uVar4 = puVar33[0xf];
    puVar32[0xe] = puVar33[0xe];
    puVar32[0xf] = uVar4;
    lVar37 = lVar37 + -1;
    puVar41 = puVar32 + 0x10;
    puVar3 = puVar33 + 0x10;
  } while (lVar37 != 0);
  uVar4 = puVar33[0x11];
  puVar32[0x10] = puVar33[0x10];
  puVar32[0x11] = uVar4;
  uVar4 = puVar33[0x13];
  puVar32[0x12] = puVar33[0x12];
  puVar32[0x13] = uVar4;
  uVar4 = puVar33[0x15];
  puVar32[0x14] = puVar33[0x14];
  puVar32[0x15] = uVar4;
  uVar4 = puVar33[0x17];
  puVar32[0x16] = puVar33[0x16];
  puVar32[0x17] = uVar4;
  uVar4 = puVar33[0x19];
  puVar32[0x18] = puVar33[0x18];
  puVar32[0x19] = uVar4;
  uVar4 = puVar33[0x1b];
  puVar32[0x1a] = puVar33[0x1a];
  puVar32[0x1b] = uVar4;
  uVar4 = puVar33[0x1d];
  puVar32[0x1c] = puVar33[0x1c];
  puVar32[0x1d] = uVar4;
  puVar41 = (undefined8 *)((longlong)puVar31 + 0x62);
  puVar3 = &local_1a8;
  do {
    puVar33 = puVar3;
    puVar32 = puVar41;
    uVar4 = puVar33[1];
    *puVar32 = *puVar33;
    puVar32[1] = uVar4;
    uVar4 = puVar33[3];
    puVar32[2] = puVar33[2];
    puVar32[3] = uVar4;
    uVar4 = puVar33[5];
    puVar32[4] = puVar33[4];
    puVar32[5] = uVar4;
    uVar4 = puVar33[7];
    puVar32[6] = puVar33[6];
    puVar32[7] = uVar4;
    uVar4 = puVar33[9];
    puVar32[8] = puVar33[8];
    puVar32[9] = uVar4;
    uVar4 = puVar33[0xb];
    puVar32[10] = puVar33[10];
    puVar32[0xb] = uVar4;
    uVar4 = puVar33[0xd];
    puVar32[0xc] = puVar33[0xc];
    puVar32[0xd] = uVar4;
    uVar4 = puVar33[0xf];
    puVar32[0xe] = puVar33[0xe];
    puVar32[0xf] = uVar4;
    lVar34 = lVar34 + -1;
    puVar41 = puVar32 + 0x10;
    puVar3 = puVar33 + 0x10;
  } while (lVar34 != 0);
  uVar4 = puVar33[0x11];
  puVar32[0x10] = puVar33[0x10];
  puVar32[0x11] = uVar4;
  uVar4 = puVar33[0x13];
  puVar32[0x12] = puVar33[0x12];
  puVar32[0x13] = uVar4;
  uVar4 = puVar33[0x15];
  puVar32[0x14] = puVar33[0x14];
  puVar32[0x15] = uVar4;
  uVar4 = puVar33[0x17];
  puVar32[0x16] = puVar33[0x16];
  puVar32[0x17] = uVar4;
  uVar4 = puVar33[0x19];
  puVar32[0x18] = puVar33[0x18];
  puVar32[0x19] = uVar4;
  uVar4 = puVar33[0x1b];
  puVar32[0x1a] = puVar33[0x1a];
  puVar32[0x1b] = uVar4;
  uVar4 = puVar33[0x1d];
  puVar32[0x1c] = puVar33[0x1c];
  puVar32[0x1d] = uVar4;
  *(undefined8 *)((longlong)puVar31 + 0x1f2) = *(undefined8 *)(param_3 + 0x1f2);
  *(undefined8 *)((longlong)puVar31 + 0x1fa) = *(undefined8 *)(param_3 + 0x1fa);
  *(undefined4 *)((longlong)puVar31 + 0x202) = *(undefined4 *)(param_3 + 0x202);
  *(undefined4 *)((longlong)puVar31 + 0x206) = *(undefined4 *)(param_3 + 0x206);
  *(undefined4 *)((longlong)puVar31 + 0x20a) = *(undefined4 *)(param_3 + 0x20a);
  *(undefined4 *)((longlong)puVar31 + 0x20e) = *(undefined4 *)(param_3 + 0x20e);
  *(undefined8 *)((longlong)puVar31 + 0x232) = *(undefined8 *)(param_3 + 0x232);
  *(undefined4 *)((longlong)puVar31 + 0x23a) = *(undefined4 *)(param_3 + 0x23a);
  *(undefined4 *)((longlong)puVar31 + 0x23e) = *(undefined4 *)(param_3 + 0x23e);
  uVar8 = *(undefined8 *)(param_3 + 0x24a);
  uVar30 = *(undefined4 *)(param_3 + 0x252);
  uVar5 = *(undefined4 *)(param_3 + 0x256);
  uVar6 = *(undefined4 *)(param_3 + 0x25a);
  uVar7 = *(undefined4 *)(param_3 + 0x25e);
  uVar4 = *(undefined8 *)(param_3 + 0x262);
  uVar28 = *(undefined1 *)(param_3 + 0x26a);
  *(undefined8 *)((longlong)puVar31 + 0x242) = *(undefined8 *)(param_3 + 0x242);
  *(undefined8 *)((longlong)puVar31 + 0x24a) = uVar8;
  *(undefined4 *)((longlong)puVar31 + 0x252) = uVar30;
  *(undefined4 *)((longlong)puVar31 + 0x256) = uVar5;
  *(undefined4 *)((longlong)puVar31 + 0x25a) = uVar6;
  *(undefined4 *)((longlong)puVar31 + 0x25e) = uVar7;
  *(undefined8 *)((longlong)puVar31 + 0x262) = uVar4;
  *(undefined1 *)((longlong)puVar31 + 0x26a) = uVar28;
  uVar4 = *(undefined8 *)(param_3 + 0x3b7);
  uVar8 = *(undefined8 *)(param_3 + 0x3bf);
  uVar9 = *(undefined8 *)(param_3 + 0x3c7);
  uVar10 = *(undefined8 *)(param_3 + 0x3cf);
  uVar11 = *(undefined8 *)(param_3 + 0x3d7);
  uVar12 = *(undefined8 *)(param_3 + 0x3df);
  uVar13 = *(undefined8 *)(param_3 + 999);
  uVar14 = *(undefined8 *)(param_3 + 0x3ef);
  uVar15 = *(undefined8 *)(param_3 + 0x3f7);
  uVar16 = *(undefined8 *)(param_3 + 0x3ff);
  uVar17 = *(undefined8 *)(param_3 + 0x407);
  uVar18 = *(undefined8 *)(param_3 + 0x40f);
  uVar19 = *(undefined8 *)(param_3 + 0x417);
  uVar20 = *(undefined8 *)(param_3 + 0x41f);
  uVar21 = *(undefined8 *)(param_3 + 0x427);
  uVar22 = *(undefined8 *)(param_3 + 0x42f);
  uVar23 = *(undefined8 *)(param_3 + 0x437);
  uVar24 = *(undefined8 *)(param_3 + 0x43f);
  uVar25 = *(undefined8 *)(param_3 + 0x447);
  uVar26 = *(undefined8 *)(param_3 + 0x44f);
  uStack_100 = *(undefined8 *)(param_3 + 0x457);
  local_f8 = *(undefined8 *)(param_3 + 0x45f);
  *(undefined8 *)((longlong)puVar31 + 0x3af) = *(undefined8 *)(param_3 + 0x3af);
  *(undefined8 *)((longlong)puVar31 + 0x3b7) = uVar4;
  *(undefined8 *)((longlong)puVar31 + 0x3bf) = uVar8;
  *(undefined8 *)((longlong)puVar31 + 0x3c7) = uVar9;
  *(undefined8 *)((longlong)puVar31 + 0x3cf) = uVar10;
  *(undefined8 *)((longlong)puVar31 + 0x3d7) = uVar11;
  *(undefined8 *)((longlong)puVar31 + 0x3df) = uVar12;
  *(undefined8 *)((longlong)puVar31 + 999) = uVar13;
  *(undefined8 *)((longlong)puVar31 + 0x3ef) = uVar14;
  *(undefined8 *)((longlong)puVar31 + 0x3f7) = uVar15;
  *(undefined8 *)((longlong)puVar31 + 0x3ff) = uVar16;
  *(undefined8 *)((longlong)puVar31 + 0x407) = uVar17;
  *(undefined8 *)((longlong)puVar31 + 0x40f) = uVar18;
  *(undefined8 *)((longlong)puVar31 + 0x417) = uVar19;
  *(undefined8 *)((longlong)puVar31 + 0x41f) = uVar20;
  *(undefined8 *)((longlong)puVar31 + 0x427) = uVar21;
  *(undefined8 *)((longlong)puVar31 + 0x42f) = uVar22;
  *(undefined8 *)((longlong)puVar31 + 0x437) = uVar23;
  *(undefined8 *)((longlong)puVar31 + 0x43f) = uVar24;
  *(undefined8 *)((longlong)puVar31 + 0x447) = uVar25;
  *(undefined8 *)((longlong)puVar31 + 0x44f) = uVar26;
  *(undefined8 *)((longlong)puVar31 + 0x457) = uStack_100;
  *(undefined8 *)((longlong)puVar31 + 0x45f) = local_f8;
  local_1a8 = *(undefined8 *)(param_3 + 0x26b);
  uStack_1a0 = *(undefined8 *)(param_3 + 0x273);
  local_198 = *(undefined8 *)(param_3 + 0x27b);
  uStack_190 = *(undefined8 *)(param_3 + 0x283);
  local_188 = *(undefined8 *)(param_3 + 0x28b);
  uStack_180 = *(undefined8 *)(param_3 + 0x293);
  local_178 = *(undefined8 *)(param_3 + 0x29b);
  uStack_170 = *(undefined8 *)(param_3 + 0x2a3);
  local_168 = *(undefined8 *)(param_3 + 0x2ab);
  uStack_160 = *(undefined8 *)(param_3 + 0x2b3);
  local_158 = *(undefined8 *)(param_3 + 699);
  uStack_150 = *(undefined8 *)(param_3 + 0x2c3);
  local_148 = *(undefined8 *)(param_3 + 0x2cb);
  uStack_140 = *(undefined8 *)(param_3 + 0x2d3);
  local_138 = *(undefined8 *)(param_3 + 0x2db);
  uStack_130 = *(undefined8 *)(param_3 + 0x2e3);
  uVar4 = *(undefined8 *)(param_3 + 0x2eb);
  uVar8 = *(undefined8 *)(param_3 + 0x2f3);
  uVar9 = *(undefined8 *)(param_3 + 0x2fb);
  uVar10 = *(undefined8 *)(param_3 + 0x303);
  local_108 = *(undefined8 *)(param_3 + 0x30b);
  *(undefined8 *)((longlong)puVar31 + 0x26b) = local_1a8;
  *(undefined8 *)((longlong)puVar31 + 0x273) = uStack_1a0;
  *(undefined8 *)((longlong)puVar31 + 0x27b) = local_198;
  *(undefined8 *)((longlong)puVar31 + 0x283) = uStack_190;
  *(undefined8 *)((longlong)puVar31 + 0x28b) = local_188;
  *(undefined8 *)((longlong)puVar31 + 0x293) = uStack_180;
  *(undefined8 *)((longlong)puVar31 + 0x29b) = local_178;
  *(undefined8 *)((longlong)puVar31 + 0x2a3) = uStack_170;
  *(undefined8 *)((longlong)puVar31 + 0x2ab) = local_168;
  *(undefined8 *)((longlong)puVar31 + 0x2b3) = uStack_160;
  *(undefined8 *)((longlong)puVar31 + 699) = local_158;
  *(undefined8 *)((longlong)puVar31 + 0x2c3) = uStack_150;
  *(undefined8 *)((longlong)puVar31 + 0x2cb) = local_148;
  *(undefined8 *)((longlong)puVar31 + 0x2d3) = uStack_140;
  *(undefined8 *)((longlong)puVar31 + 0x2db) = local_138;
  *(undefined8 *)((longlong)puVar31 + 0x2e3) = uStack_130;
  local_128._0_4_ = (undefined4)uVar4;
  local_128._4_4_ = (undefined4)((ulonglong)uVar4 >> 0x20);
  uStack_120._0_4_ = (undefined4)uVar8;
  uStack_120._4_4_ = (undefined4)((ulonglong)uVar8 >> 0x20);
  *(undefined4 *)((longlong)puVar31 + 0x2eb) = (undefined4)local_128;
  *(undefined4 *)((longlong)puVar31 + 0x2ef) = local_128._4_4_;
  *(undefined4 *)((longlong)puVar31 + 0x2f3) = (undefined4)uStack_120;
  *(undefined4 *)((longlong)puVar31 + 0x2f7) = uStack_120._4_4_;
  local_118._0_4_ = (undefined4)uVar9;
  local_118._4_4_ = (undefined4)((ulonglong)uVar9 >> 0x20);
  uStack_110._0_4_ = (undefined4)uVar10;
  uStack_110._4_4_ = (undefined4)((ulonglong)uVar10 >> 0x20);
  *(undefined4 *)((longlong)puVar31 + 0x2fb) = (undefined4)local_118;
  *(undefined4 *)((longlong)puVar31 + 0x2ff) = local_118._4_4_;
  *(undefined4 *)((longlong)puVar31 + 0x303) = (undefined4)uStack_110;
  *(undefined4 *)((longlong)puVar31 + 0x307) = uStack_110._4_4_;
  *(undefined8 *)((longlong)puVar31 + 0x30b) = local_108;
  local_128 = uVar4;
  uStack_120 = uVar8;
  local_118 = uVar9;
  uStack_110 = uVar10;
  FUN_1408ad0e0((longlong)puVar31 + 0x55,0xd,param_3 + 0x55);
  FUN_1402fa3f0((longlong)param_1 + 0x11a,puVar31);
  if (0xffffe < puVar31[1] - 1) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar1 = puVar31 + 1;
  lVar34 = *plVar1;
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if ((int)lVar34 == 1) {
    (**(code **)*puVar31)(puVar31,1);
  }
  return;
}


