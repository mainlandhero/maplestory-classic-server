
//===========================================================
// FUN_141c4ff80 @ 141c4ff80   (16424 bytes)
//===========================================================

void FUN_141c4ff80(longlong *param_1,int param_2,undefined8 param_3)

{
  ushort uVar1;
  uint uVar2;
  code *pcVar3;
  IUnknown *pIVar4;
  undefined *puVar5;
  undefined1 uVar6;
  byte bVar7;
  char cVar8;
  undefined2 uVar9;
  short sVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  int iVar13;
  int iVar14;
  undefined4 uVar15;
  undefined8 *puVar16;
  longlong lVar17;
  longlong lVar18;
  longlong *plVar19;
  int *piVar20;
  undefined8 *puVar21;
  longlong *plVar22;
  undefined8 uVar23;
  undefined8 uVar24;
  undefined4 *puVar25;
  IUnknown *pIVar26;
  longlong lVar27;
  int iVar28;
  uint uVar29;
  ulonglong uVar30;
  IUnknown *pIVar31;
  uint uVar32;
  IUnknown *pIVar33;
  ulonglong uVar34;
  byte bVar35;
  byte *pbVar36;
  ulonglong uVar37;
  uint uVar38;
  undefined2 uStackX_8;
  short sStackX_a;
  undefined4 uStackX_c;
  int aiStackX_10 [2];
  undefined8 uStackX_18;
  IUnknown *pIStackX_20;
  undefined8 **in_stack_fffffffffffffd48;
  undefined8 in_stack_fffffffffffffd50;
  IUnknown **ppIVar39;
  IUnknown **ppIVar40;
  undefined8 in_stack_fffffffffffffd58;
  short sStack_278;
  undefined2 uStack_276;
  undefined4 uStack_274;
  IUnknown *pIStack_270;
  longlong *plStack_268;
  IUnknown *pIStack_260;
  IUnknown *pIStack_258;
  longlong *plStack_250;
  short sStack_248;
  undefined2 uStack_246;
  undefined4 uStack_244;
  undefined8 uStack_240;
  longlong *plStack_238;
  IUnknown *pIStack_230;
  IUnknown *pIStack_228;
  longlong *plStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  longlong *plStack_208;
  undefined8 uStack_1f8;
  IUnknown *pIStack_1f0;
  longlong *plStack_1e8;
  longlong *plStack_1d8;
  int iStack_1d0;
  int iStack_1cc;
  longlong *aplStack_1c8 [2];
  undefined8 uStack_1b8;
  IUnknown *pIStack_1b0;
  longlong *plStack_1a8;
  longlong *plStack_198;
  undefined8 *puStack_190;
  IUnknown *pIStack_188;
  undefined8 uStack_180;
  longlong *plStack_178;
  longlong *aplStack_168 [2];
  IUnknown *pIStack_158;
  IUnknown *pIStack_150;
  longlong *plStack_148;
  undefined8 uStack_138;
  IUnknown *pIStack_130;
  longlong *plStack_128;
  undefined8 uStack_118;
  IUnknown *pIStack_110;
  longlong *plStack_108;
  longlong **pplStack_f8;
  IUnknown **ppIStack_f0;
  undefined8 *puStack_e0;
  undefined1 auStack_d8 [152];
  
  aiStackX_10[0] = param_2;
  uStackX_18 = param_3;
  iStack_1d0 = FUN_1429e3ef0();
  *(int *)(param_1 + 0x74) = param_2;
  FUN_141c5ae70(param_1);
  uStackX_8 = thunk_FUN_1406e8b80(param_3);
  sStackX_a = uStackX_8 >> 0xf;
  iVar28 = (int)param_1[0x10c] + 1;
  *(int *)(param_1 + 0x10c) = iVar28;
  if (iVar28 == (iVar28 / 0x6f) * 0x6f) {
    puVar21 = (undefined8 *)param_1[0x10d];
    puVar16 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0x10d] = (longlong)puVar16;
    *puVar16 = *puVar21;
    *(undefined4 *)(puVar16 + 1) = *(undefined4 *)(puVar21 + 1);
    thunk_FUN_140205820(puVar21,0xc);
  }
  uVar6 = FUN_142f04924();
  *(undefined1 *)(param_1[0x10d] + 4) = uVar6;
  lVar17 = param_1[0x10d];
  bVar35 = *(byte *)(lVar17 + 4);
  *(undefined2 *)(lVar17 + 8) = 0x9a65;
  uVar38 = 0;
  pbVar36 = (byte *)(lVar17 + 2);
  do {
    if (bVar35 == 0) {
      bVar35 = 0x2a;
    }
    bVar7 = pbVar36[(longlong)(&stack0x00000006 + -lVar17)];
    pbVar36[-2] = bVar35 ^ bVar7;
    bVar35 = bVar35 + (bVar35 ^ bVar7) + 0x2a;
    uVar1 = *(ushort *)(param_1[0x10d] + 8);
    *(ushort *)(param_1[0x10d] + 8) = (uVar1 >> 0xd) + (ushort)bVar35 | uVar1 << 3;
    bVar7 = 0x2a;
    if (bVar35 != 0) {
      bVar7 = bVar35;
    }
    bVar35 = pbVar36[(longlong)(&stack0x00000007 + -lVar17)];
    pbVar36[-1] = bVar7 ^ bVar35;
    bVar7 = (bVar7 ^ bVar35) + bVar7 + 0x2a;
    uVar1 = *(ushort *)(param_1[0x10d] + 8);
    *(ushort *)(param_1[0x10d] + 8) = (uVar1 >> 0xd) + (ushort)bVar7 | uVar1 << 3;
    bVar35 = 0x2a;
    if (bVar7 != 0) {
      bVar35 = bVar7;
    }
    bVar7 = pbVar36[(longlong)&uStackX_8 - lVar17];
    *pbVar36 = bVar35 ^ bVar7;
    bVar7 = (bVar35 ^ bVar7) + bVar35 + 0x2a;
    uVar1 = *(ushort *)(param_1[0x10d] + 8);
    *(ushort *)(param_1[0x10d] + 8) = (uVar1 >> 0xd) + (ushort)bVar7 | uVar1 << 3;
    bVar35 = 0x2a;
    if (bVar7 != 0) {
      bVar35 = bVar7;
    }
    bVar7 = pbVar36[(longlong)&uStackX_8 + -lVar17 + 1];
    pbVar36[1] = bVar35 ^ bVar7;
    bVar35 = (bVar35 ^ bVar7) + bVar35 + 0x2a;
    uVar1 = *(ushort *)(param_1[0x10d] + 8);
    *(ushort *)(param_1[0x10d] + 8) = (uVar1 >> 0xd) + (ushort)bVar35 | uVar1 << 3;
    uVar38 = uVar38 + 4;
    pbVar36 = pbVar36 + 4;
  } while (uVar38 < 4);
  uStackX_8 = thunk_FUN_1406e8b80(param_3);
  plVar22 = param_1 + 0x109;
  sStackX_a = uStackX_8 >> 0xf;
  iVar28 = (int)*plVar22 + 1;
  *(int *)plVar22 = iVar28;
  if (iVar28 == (iVar28 / 0x6f) * 0x6f) {
    puVar21 = (undefined8 *)param_1[0x10a];
    puVar16 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0x10a] = (longlong)puVar16;
    *puVar16 = *puVar21;
    *(undefined4 *)(puVar16 + 1) = *(undefined4 *)(puVar21 + 1);
    thunk_FUN_140205820(puVar21,0xc);
  }
  uVar6 = FUN_142f04924();
  *(undefined1 *)(param_1[0x10a] + 4) = uVar6;
  lVar17 = param_1[0x10a];
  bVar35 = *(byte *)(lVar17 + 4);
  *(undefined2 *)(lVar17 + 8) = 0x9a65;
  uVar38 = 0;
  pbVar36 = (byte *)(lVar17 + 2);
  do {
    if (bVar35 == 0) {
      bVar35 = 0x2a;
    }
    bVar7 = pbVar36[(longlong)(&stack0x00000006 + -lVar17)];
    pbVar36[-2] = bVar35 ^ bVar7;
    bVar35 = bVar35 + (bVar35 ^ bVar7) + 0x2a;
    uVar1 = *(ushort *)(param_1[0x10a] + 8);
    *(ushort *)(param_1[0x10a] + 8) = (uVar1 >> 0xd) + (ushort)bVar35 | uVar1 << 3;
    bVar7 = 0x2a;
    if (bVar35 != 0) {
      bVar7 = bVar35;
    }
    bVar35 = pbVar36[(longlong)(&stack0x00000007 + -lVar17)];
    pbVar36[-1] = bVar7 ^ bVar35;
    bVar7 = (bVar7 ^ bVar35) + bVar7 + 0x2a;
    uVar1 = *(ushort *)(param_1[0x10a] + 8);
    *(ushort *)(param_1[0x10a] + 8) = (uVar1 >> 0xd) + (ushort)bVar7 | uVar1 << 3;
    bVar35 = 0x2a;
    if (bVar7 != 0) {
      bVar35 = bVar7;
    }
    bVar7 = pbVar36[(longlong)&uStackX_8 - lVar17];
    *pbVar36 = bVar35 ^ bVar7;
    bVar7 = (bVar35 ^ bVar7) + bVar35 + 0x2a;
    uVar1 = *(ushort *)(param_1[0x10a] + 8);
    *(ushort *)(param_1[0x10a] + 8) = (uVar1 >> 0xd) + (ushort)bVar7 | uVar1 << 3;
    bVar35 = 0x2a;
    if (bVar7 != 0) {
      bVar35 = bVar7;
    }
    bVar7 = pbVar36[(longlong)&uStackX_8 + -lVar17 + 1];
    pbVar36[1] = bVar35 ^ bVar7;
    bVar35 = (bVar35 ^ bVar7) + bVar35 + 0x2a;
    uVar1 = *(ushort *)(param_1[0x10a] + 8);
    *(ushort *)(param_1[0x10a] + 8) = (uVar1 >> 0xd) + (ushort)bVar35 | uVar1 << 3;
    uVar38 = uVar38 + 4;
    pbVar36 = pbVar36 + 4;
  } while (uVar38 < 4);
  FUN_1409d3c60(param_1 + 0x103,plVar22);
  FUN_1409d3c60(param_1 + 0x10f,param_1 + 0x103);
  FUN_1409d3c60(param_1 + 0x18b,param_1 + 0x103);
  uVar11 = FUN_14019a5d0(param_1 + 0x106);
  uStackX_8 = (short)uVar11;
  sStackX_a = (short)((uint)uVar11 >> 0x10);
  uStackX_c = FUN_14019a5d0(param_1 + 0x103);
  uVar24 = uStackX_18;
  param_1[0x1a6] = CONCAT44(uStackX_c,CONCAT22(sStackX_a,uStackX_8));
  bVar35 = FUN_1406e8ae0(uStackX_18);
  uVar38 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x3dc) = uVar38;
  uVar32 = (bVar35 ^ uVar38) >> 5 | (bVar35 ^ uVar38) << 0x1b;
  *(uint *)(param_1 + 0x7c) = uVar32;
  *(uint *)((longlong)param_1 + 0x3e4) =
       ((uVar38 ^ 0xbaadf00d) >> 5 | (uVar38 ^ 0xbaadf00d) << 0x1b) + uVar32;
  iVar28 = FUN_14045b1a0(*(undefined4 *)(param_1[0x75] + 0x60));
  if ((iVar28 != 0) && (cVar8 = FUN_1406e8ae0(uVar24), cVar8 != '\0')) {
    FUN_141c56870(param_1,0x33);
  }
  uVar9 = FUN_1406e8b80(uVar24);
  pIStackX_20 = (IUnknown *)CONCAT62(pIStackX_20._2_6_,uVar9);
  uStackX_8 = FUN_1406e8b80(uVar24);
  bVar35 = FUN_1406e8ae0(uVar24);
  *(uint *)((longlong)param_1 + 0x116c) = (uint)bVar35;
  cVar8 = FUN_1406e8ae0(uVar24);
  iVar28 = (int)cVar8;
  aplStack_168[0] = (longlong *)CONCAT44(aplStack_168[0]._4_4_,iVar28);
  *(int *)(param_1 + 0x22d) = iVar28;
  sVar10 = FUN_1406e8b80(uVar24);
  *(int *)(param_1 + 0x245) = (int)sVar10;
  sVar10 = FUN_1406e8b80(uVar24);
  *(int *)((longlong)param_1 + 0x122c) = (int)sVar10;
  iStack_1cc = 0;
  if (((iVar28 == -3) || (iVar28 == -6)) || (-1 < cVar8)) {
    iStack_1cc = FUN_1406e8c20(uVar24);
  }
  uVar11 = FUN_1406e8c20(uVar24);
  *(undefined4 *)(param_1 + 0x118) = uVar11;
  lVar17 = FUN_1406e8f10(uVar24);
  lVar18 = FUN_141c8a730(param_1);
  *(int *)(param_1 + 0x16c) = (int)((lVar17 * 100) / lVar18);
  uVar11 = FUN_1406e8c20(uVar24,(lVar17 * 100) % lVar18);
  pplStack_f8 = (longlong **)CONCAT44(pplStack_f8._4_4_,uVar11);
  if (*(char *)(param_1[0x75] + 0x104) != '\0') {
    uVar11 = FUN_1406e8c20(uVar24);
    *(undefined4 *)((longlong)param_1 + 0xae4) = uVar11;
    uVar11 = FUN_1406e8c20(uVar24);
    *(undefined4 *)(param_1 + 0x15d) = uVar11;
    uVar11 = FUN_1406e8c20(uVar24);
    uVar12 = FUN_1406e8c20(uVar24);
    FUN_141eb4b50(param_1 + 0x15e,param_1,uVar11,uVar12);
  }
  uVar11 = FUN_1406e8c20(uVar24);
  cVar8 = FUN_140479e60(param_1[0x75]);
  if (cVar8 == '\0') {
    lVar17 = param_1[0x16d];
    *(undefined4 *)(param_1 + 0x16d) = uVar11;
    (**(code **)(*param_1 + 0x110))(param_1,(int)lVar17,uVar11);
  }
  uVar11 = FUN_1406e8c20(uVar24);
  if ((param_1[0x75] != 0) && (cVar8 = FUN_14047a660(), cVar8 != '\0')) {
    *(undefined4 *)((longlong)param_1 + 0xb6c) = uVar11;
    plVar19 = (longlong *)FUN_141cd9e30(param_1,&plStack_198,0);
    plVar22 = (longlong *)param_1[0x16f];
    if (plVar22 != (longlong *)*plVar19) {
      param_1[0x16f] = *plVar19;
      *plVar19 = 0;
      if (plVar22 != (longlong *)0x0) {
        (**(code **)(*plVar22 + 0x10))();
      }
    }
    if (plStack_198 != (longlong *)0x0) {
      (**(code **)(*plStack_198 + 0x10))();
    }
    uVar11 = FUN_1429e3ef0();
    FUN_141cda400(param_1,uVar11);
  }
  uVar11 = FUN_1406e8c20(uVar24);
  *(undefined4 *)((longlong)param_1 + 0xb54) = uVar11;
  if (param_1[0x16b] != 0) {
    FUN_14019f2c0(param_1[0x16b] + -0x10);
    param_1[0x16b] = 0;
  }
  if ((*(int *)((longlong)param_1 + 0xb54) != 0) && (lVar17 = FUN_140495990(), lVar17 != 0)) {
    FUN_14019a260(param_1 + 0x16b,lVar17 + 0x68);
  }
  iVar28 = FUN_1406e8c20(uVar24);
  ppIStack_f0 = (IUnknown **)CONCAT44(ppIStack_f0._4_4_,iVar28);
  iVar13 = FUN_1406e8c20(uVar24);
  uVar11 = FUN_1406e8c20(uVar24);
  uVar6 = FUN_1406e8ae0(uVar24);
  if ((-1 < iVar28) && (0 < iVar13)) {
    in_stack_fffffffffffffd48 =
         (undefined8 **)CONCAT44((int)((ulonglong)in_stack_fffffffffffffd48 >> 0x20),iVar13);
    FUN_141cdf1e0(param_1,iVar28 + -0xd,uVar11,uVar6,in_stack_fffffffffffffd48);
  }
  uVar38 = FUN_1406e8c20(uVar24);
  if (0 < (int)uVar38) {
    plVar22 = param_1 + 0x187;
    uVar37 = (ulonglong)uVar38;
    do {
      iVar28 = FUN_1406e8c20(uVar24);
      iVar13 = FUN_1406e8c20(uVar24);
      iVar13 = iStack_1d0 + iVar13;
      if (iVar13 == 0) {
        iVar13 = 1;
      }
      iVar14 = 0;
      lVar17 = *plVar22;
      if (lVar17 == 0) {
        iVar14 = (int)param_1[0x188];
      }
      else if (*(uint *)((longlong)param_1 + 0xc4c) < *(uint *)((longlong)param_1 + 0xc44)) {
        iVar14 = (int)param_1[0x188] * 2;
      }
      if (iVar14 != 0) {
        FUN_1402ff650(plVar22,iVar14,0);
        lVar17 = *plVar22;
      }
      uVar30 = (ulonglong)(longlong)iVar28 % (ulonglong)*(uint *)(param_1 + 0x188);
      for (lVar18 = *(longlong *)(lVar17 + uVar30 * 8); lVar18 != 0;
          lVar18 = *(longlong *)(lVar18 + 8)) {
        if (*(int *)(lVar18 + 0x10) == iVar28) {
          *(int *)(lVar18 + 0x14) = iVar13;
          goto LAB_141c50790;
        }
      }
      *(int *)((longlong)param_1 + 0xc44) = *(int *)((longlong)param_1 + 0xc44) + 1;
      pIStack_230 = (IUnknown *)FUN_14036f510(0x18);
      if (pIStack_230 != (IUnknown *)0x0) {
        lVar18 = *(longlong *)(lVar17 + uVar30 * 8);
        *(undefined ***)pIStack_230 = &PTR_FUN_14327ecd8;
        *(longlong *)(pIStack_230 + 8) = lVar18;
        *(longlong *)(pIStack_230 + 0x10) = 0;
        *(int *)(pIStack_230 + 0x10) = iVar28;
      }
      *(int *)(pIStack_230 + 0x14) = iVar13;
      *(IUnknown **)(lVar17 + uVar30 * 8) = pIStack_230;
LAB_141c50790:
      FUN_142130f20((int)param_1[0x74],plVar22);
      uVar37 = uVar37 - 1;
      uVar24 = uStackX_18;
    } while (uVar37 != 0);
  }
  iVar28 = 0;
  uVar11 = FUN_1406e8c20(uVar24);
  *(undefined4 *)((longlong)param_1 + 0xd64) = uVar11;
  cVar8 = FUN_1406e8ae0(uVar24);
  if (cVar8 != '\0') {
    uVar11 = FUN_1406e8c20(uVar24);
    aplStack_1c8[0] = (longlong *)CONCAT44(aplStack_1c8[0]._4_4_,uVar11);
    iVar13 = FUN_1406e8c20(uVar24);
    if (0 < iVar13) {
      iVar14 = FUN_1429e3ef0();
      piVar20 = (int *)FUN_14028c520(param_1 + 0x1bf,aplStack_1c8);
      *piVar20 = iVar13 + iVar14;
    }
  }
  iVar13 = FUN_1406e8c20(uVar24);
  uVar23 = uStackX_18;
  if (0 < iVar13) {
    do {
      plVar22 = (longlong *)0x0;
      uVar11 = FUN_1406e8c20(uVar23);
      uVar12 = FUN_1406e8c20(uVar23);
      iVar14 = FUN_14047a330(param_1[0x75]);
      if (iVar14 == 1) {
        lVar17 = FUN_141d2efc0(DAT_143abfe00,uVar12);
        plStack_198 = (longlong *)0x0;
        if (lVar17 != 0) {
          puVar21 = (undefined8 *)FUN_1409397b0(lVar17 + 8,&plStack_1d8);
          plVar19 = (longlong *)*puVar21;
          if (plVar19 != (longlong *)0x0) {
            *puVar21 = 0;
            plVar22 = plVar19;
            plStack_198 = plVar19;
          }
          if (plStack_1d8 != (longlong *)0x0) {
            (**(code **)(*plStack_1d8 + 0x10))();
          }
        }
        pIStack_270 = (IUnknown *)0x0;
        plStack_268 = (longlong *)0x0;
        puStack_190 = (undefined8 *)0x0;
        aplStack_1c8[0] = plVar22;
        if (plVar22 != (longlong *)0x0) {
          (**(code **)(*plVar22 + 8))(plVar22);
        }
        in_stack_fffffffffffffd48 = &puStack_190;
        FUN_1408d4ab0(&sStack_278,uVar11,uVar12,aplStack_1c8,in_stack_fffffffffffffd48);
        lVar17 = param_1[0x174];
        if (param_1[0x175] == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
          FUN_142ed3068("list too long");
        }
        pIStack_258 = (IUnknown *)0x0;
        pIStack_260 = (IUnknown *)(param_1 + 0x174);
        plVar19 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
        *(uint *)(plVar19 + 2) = CONCAT22(uStack_276,sStack_278);
        *(undefined4 *)((longlong)plVar19 + 0x14) = uStack_274;
        plVar19[3] = (longlong)pIStack_270;
        pIStack_258 = (IUnknown *)plVar19;
        if (pIStack_270 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)pIStack_270 + 8))();
        }
        plVar19[4] = (longlong)plStack_268;
        if (plStack_268 != (longlong *)0x0) {
          (**(code **)(*plStack_268 + 8))();
        }
        param_1[0x175] = param_1[0x175] + 1;
        puVar21 = *(undefined8 **)(lVar17 + 8);
        *plVar19 = lVar17;
        plVar19[1] = (longlong)puVar21;
        pIStack_258 = (IUnknown *)0x0;
        *(longlong **)(lVar17 + 8) = plVar19;
        *puVar21 = plVar19;
        if (plStack_268 != (longlong *)0x0) {
          (**(code **)(*plStack_268 + 0x10))();
        }
        if (pIStack_270 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)pIStack_270 + 0x10))();
        }
        uVar23 = uStackX_18;
        if (plVar22 != (longlong *)0x0) {
          (**(code **)(*plVar22 + 0x10))(plVar22);
          uVar23 = uStackX_18;
        }
      }
      iVar28 = iVar28 + 1;
      uVar24 = uStackX_18;
    } while (iVar28 < iVar13);
  }
  pIVar31 = (IUnknown *)0x0;
  puStack_190 = (undefined8 *)0x0;
  puStack_e0 = (undefined8 *)0x0;
  cVar8 = FUN_1406e8ae0(uVar24);
  if (cVar8 != '\0') {
    pIStack_230 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x1e3);
    if (pIStack_230 == (IUnknown *)0x0) {
      puStack_190 = (undefined8 *)0x0;
    }
    else {
      puStack_190 = (undefined8 *)FUN_14042c280(pIStack_230);
    }
    puVar21 = puStack_190;
    if (puStack_190[1] != 0) {
      FUN_142e541f0(0x2fe);
    }
    puVar21[1] = 1;
    puStack_e0 = puVar21;
    FUN_1406e9170(uVar24,auStack_d8,0x78);
    FUN_140250a50(auStack_d8,puVar21);
  }
  plVar22 = (longlong *)FUN_1406e9050(uVar24,&plStack_198);
  if (param_1[0x1ef] != 0) {
    FUN_14019f2c0(param_1[0x1ef] + -0x10);
    param_1[0x1ef] = 0;
  }
  param_1[0x1ef] = *plVar22;
  *plVar22 = 0;
  if (plStack_198 != (longlong *)0x0) {
    FUN_14019f2c0(plStack_198 + -2);
  }
  cVar8 = FUN_140479e70(param_1[0x75]);
  if (cVar8 != '\0') {
    uVar11 = FUN_1406e8c20(uVar24);
    *(undefined4 *)(param_1 + 0x23) = uVar11;
  }
  FUN_1402bf500(param_1 + 0x19f);
  uVar38 = FUN_1406e8c20(uVar24);
  uVar32 = (uint)((ulonglong)in_stack_fffffffffffffd50 >> 0x20);
  uVar11 = (undefined4)((ulonglong)in_stack_fffffffffffffd58 >> 0x20);
  if (0 < (int)uVar38) {
    uVar37 = (ulonglong)uVar38;
    do {
      uVar11 = FUN_1406e8c20(uVar24);
      lVar17 = param_1[0x1a1];
      pIStack_230 = (IUnknown *)FUN_1403f6b70(0x30);
      pIVar33 = pIVar31;
      if (pIStack_230 != (IUnknown *)0x0) {
        *(longlong *)(pIStack_230 + 0x18) = 0;
        *(longlong *)(pIStack_230 + 8) = 0;
        *(longlong *)(pIStack_230 + 0x10) = 0;
        *(undefined ***)pIStack_230 = &PTR_FUN_14327ce40;
        *(undefined ***)(pIStack_230 + 0x20) = &PTR_LAB_14327ce48;
        *(undefined4 *)(pIStack_230 + 0x28) = uVar11;
        pIVar33 = pIStack_230;
      }
      pIVar26 = pIVar31;
      if (lVar17 != 0) {
        pIVar26 = (IUnknown *)(lVar17 + -0x28);
      }
      if (*(longlong *)(pIVar33 + 0x10) - 1U < 0x10000) {
        FUN_142e52ed0(0x330);
      }
      *(IUnknown **)(pIVar33 + 0x10) = pIVar26;
      if (*(longlong *)(pIVar33 + 8) - 1U < 0x10000) {
        FUN_142e52ed0(0x33e);
      }
      *(longlong *)(pIVar33 + 8) = 0;
      *(int *)((longlong)param_1 + 0xcfc) = *(int *)((longlong)param_1 + 0xcfc) + 1;
      pIVar26 = pIVar33 + 0x28;
      lVar17 = param_1[0x1a1];
      if (lVar17 == 0) {
        param_1[0x1a0] = (longlong)pIVar26;
      }
      else {
        if (pIVar26 == (IUnknown *)0x0) {
          pIVar33 = pIVar31;
        }
        if (*(longlong *)(lVar17 + -0x20) - 1U < 0x10000) {
          FUN_142e52ed0(0x33e);
        }
        *(IUnknown **)(lVar17 + -0x20) = pIVar33;
        pIVar26 = pIVar31;
        if (pIVar33 != (IUnknown *)0x0) {
          pIVar26 = pIVar33 + 0x28;
        }
      }
      uVar32 = (uint)((ulonglong)in_stack_fffffffffffffd50 >> 0x20);
      uVar11 = (undefined4)((ulonglong)in_stack_fffffffffffffd58 >> 0x20);
      param_1[0x1a1] = (longlong)pIVar26;
      uVar37 = uVar37 - 1;
    } while (uVar37 != 0);
  }
  uVar23 = FUN_142df6c50(DAT_143ac18d8,(int)(short)pIStackX_20);
  lVar17 = FUN_142df6c50(DAT_143ac18d8,(int)uStackX_8);
  if (lVar17 != 0) {
    uVar38 = FUN_142df56c0(lVar17);
    pIVar31 = (IUnknown *)(ulonglong)uVar38;
  }
  plVar22 = (longlong *)FUN_142af7be0();
  plStack_198 = plVar22;
  if (plVar22 == (longlong *)0x0) {
    iVar28 = -0x7fffbffe;
    pIVar33 = (IUnknown *)0x0;
  }
  else {
    pIStackX_20 = (IUnknown *)0x0;
    iVar28 = (**(code **)plVar22[4])(plVar22 + 4,&DAT_143273488,&pIStackX_20);
    pIVar33 = (IUnknown *)0x0;
    if (-1 < iVar28) {
      pIVar33 = pIStackX_20;
    }
  }
  if (((iVar28 + 0x80000000U & 0x80000000) == 0) && (iVar28 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0();
  }
  pIVar26 = (IUnknown *)param_1[0x57];
  if ((pIVar26 != pIVar33) &&
     (param_1[0x57] = (longlong)pIVar33, pIVar33 = (IUnknown *)0x0, pIVar26 != (IUnknown *)0x0)) {
    (**(code **)(*(longlong *)pIVar26 + 0x10))();
  }
  if (pIVar33 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)pIVar33 + 0x10))(pIVar33);
  }
  FUN_142ac0310(plVar22,param_1 + 1,*(undefined4 *)(param_1[0x75] + 0x74),pIVar31);
  pcVar3 = *(code **)(*plVar22 + 0x118);
  pIStackX_20 = (IUnknown *)CONCAT44(pIStackX_20._4_4_,*(int *)((longlong)param_1 + 0x3e4));
  uVar38 = *(uint *)((longlong)param_1 + 0x3dc);
  uVar2 = *(uint *)(param_1 + 0x7c);
  uVar29 = uVar38 ^ 0xbaadf00d;
  iVar28 = uVar2 + (uVar29 >> 5 | uVar29 << 0x1b);
  aplStack_1c8[0] = (longlong *)CONCAT44(aplStack_1c8[0]._4_4_,iVar28);
  if (iVar28 != *(int *)((longlong)param_1 + 0x3e4)) {
    plStack_1d8 = (longlong *)FUN_1418039d0(5);
    puVar21 = (undefined8 *)FUN_1401a0ed0(&pIStack_230,&plStack_1d8,aplStack_1c8,&pIStackX_20);
    FUN_141804970(&DAT_143271f04,0x53,5,*puVar21);
    if (pIStack_230 != (IUnknown *)0x0) {
      FUN_14019f2c0(pIStack_230 + -0x10);
    }
  }
  uVar12 = FUN_14019a5d0(param_1 + 0x103);
  uVar15 = FUN_14019a5d0();
  plVar22 = plStack_198;
  uVar34 = CONCAT44(uVar11,(uVar2 << 5 | uVar2 >> 0x1b) ^ uVar38);
  uVar30 = (ulonglong)uVar32 << 0x20;
  uVar37 = (ulonglong)in_stack_fffffffffffffd48 & 0xffffffff00000000;
  (*pcVar3)(plStack_198,0,uVar15,uVar12,uVar37,uVar30,uVar34,uVar23);
  uVar38 = FUN_1409c6d00(plVar22);
  pcVar3 = *(code **)(*param_1 + 0xd0);
  switch((int)uVar38 >> 1) {
  case 1:
    cVar8 = FUN_141d15b20(param_1 + 0x17b);
    bVar35 = -(cVar8 != '\0') & 0x4e;
    break;
  case 2:
    bVar35 = 1;
    break;
  case 3:
    bVar35 = 2;
    break;
  default:
    bVar35 = 1;
    if (*(int *)(param_1[0x75] + 0x74) == 3) {
      bVar35 = 3;
    }
    break;
  case 6:
    bVar35 = 3;
    break;
  case 8:
    bVar35 = 4;
    break;
  case 0xd:
    bVar35 = 0x43;
    break;
  case 0x10:
    bVar35 = 0x2f;
  }
  uVar11 = (*pcVar3)(param_1,bVar35);
  iVar28 = FUN_141cb4520(param_1,uVar11);
  uVar32 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x3dc) = uVar32;
  uVar38 = uVar32 ^ (iVar28 * 2 | uVar38 & 1);
  uVar38 = uVar38 >> 5 | uVar38 << 0x1b;
  *(uint *)(param_1 + 0x7c) = uVar38;
  *(uint *)((longlong)param_1 + 0x3e4) =
       ((uVar32 ^ 0xbaadf00d) >> 5 | (uVar32 ^ 0xbaadf00d) << 0x1b) + uVar38;
  plVar19 = (longlong *)FUN_142af7be0();
  plStack_1d8 = plVar19;
  if (plVar19 == (longlong *)0x0) {
    iVar28 = -0x7fffbffe;
    pIVar33 = (IUnknown *)0x0;
  }
  else {
    pIStackX_20 = (IUnknown *)0x0;
    iVar28 = (**(code **)plVar19[4])(plVar19 + 4,&DAT_143273488,&pIStackX_20);
    pIVar33 = (IUnknown *)0x0;
    if (-1 < iVar28) {
      pIVar33 = pIStackX_20;
    }
  }
  pIVar26 = (IUnknown *)0x0;
  if (((iVar28 + 0x80000000U & 0x80000000) == 0) && (iVar28 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0();
  }
  pIVar4 = (IUnknown *)param_1[0x58];
  if ((pIVar4 != pIVar33) &&
     (param_1[0x58] = (longlong)pIVar33, pIVar33 = pIVar26, pIVar4 != (IUnknown *)0x0)) {
    (**(code **)(*(longlong *)pIVar4 + 0x10))();
  }
  if (pIVar33 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)pIVar33 + 0x10))(pIVar33);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0x59,0);
  cVar8 = FUN_140479e60(param_1[0x75]);
  if (cVar8 != '\0') {
    pIVar33 = (IUnknown *)param_1[0x59];
    if (pIVar33 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    pIStack_270 = (IUnknown *)param_1[0x57];
    sStack_278 = 0xd;
    if (pIStack_270 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIStack_270 + 8))();
    }
    uStack_218 = (IUnknown *)CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
    uStack_210 = pIStack_270;
    plStack_208 = plStack_268;
    iVar28 = (**(code **)(*(longlong *)pIVar33 + 200))(pIVar33,&uStack_218);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_143273488);
    }
    if (sStack_278 == 8) {
      sStack_278 = 0;
      if (pIStack_270 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_270 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_278);
    }
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0x5a,0);
  cVar8 = FUN_140479e60(param_1[0x75]);
  if (cVar8 != '\0') {
    pIVar33 = (IUnknown *)param_1[0x5a];
    if (pIVar33 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    pIStack_270 = (IUnknown *)param_1[0x57];
    sStack_278 = 0xd;
    if (pIStack_270 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIStack_270 + 8))();
    }
    uStack_218 = (IUnknown *)CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
    uStack_210 = pIStack_270;
    plStack_208 = plStack_268;
    iVar28 = (**(code **)(*(longlong *)pIVar33 + 200))(pIVar33,&uStack_218);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_143273488);
    }
    if (sStack_278 == 8) {
      sStack_278 = 0;
      if (pIStack_270 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_270 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_278);
    }
  }
  FUN_142ac0310(plVar19,param_1 + 1,*(undefined4 *)(param_1[0x75] + 0x74),pIVar31);
  uVar30 = uVar30 & 0xffffffff00000000;
  uVar37 = uVar37 & 0xffffffff00000000;
  (**(code **)(*plVar19 + 0x118))(plVar19,0,0,0,uVar37,uVar30,uVar34 & 0xffffffff00000000,0);
  iVar28 = FUN_14049f3d0(param_1[0x75]);
  if (iVar28 != 0) {
    pIVar31 = (IUnknown *)(param_1[0x57] + -0x20);
    if (param_1[0x57] == 0) {
      pIVar31 = pIVar26;
    }
    pIVar33 = (IUnknown *)(param_1[0x58] + -0x20);
    if (param_1[0x58] == 0) {
      pIVar33 = pIVar26;
    }
    if ((pIVar31 != (IUnknown *)0x0) && (pIVar33 != (IUnknown *)0x0)) {
      pIVar31[0x824] = (IUnknown)0x1;
      pIVar33[0x824] = (IUnknown)0x1;
    }
  }
  lVar17 = param_1[0x79];
  if (lVar17 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar17 = param_1[0x79];
  }
  if (0 < *(int *)(lVar17 + 0x404)) {
    *(undefined1 *)(plVar22 + 0x209) = 1;
    *(undefined1 *)(plVar19 + 0x209) = 1;
  }
  pIVar31 = DAT_143add050;
  if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&sStack_278);
  if (DAT_143a8b8d8 == 8) {
    if (sStack_278 == 8) {
      sStack_278 = 0;
      if (pIStack_270 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_270 + -4);
      }
    }
    else {
      iVar28 = (*DAT_143262a18)(&sStack_278);
      if (iVar28 < 0) goto LAB_141c53dd6;
    }
    lVar17 = DAT_143a8b8e0;
    sStack_278 = 8;
    if (DAT_143a8b8e0 == 0) {
      uVar34 = 0;
    }
    else {
      uVar34 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    piVar20 = (int *)(*DAT_143ad5980)((ulonglong)((int)uVar34 + 1) * 2 + 4);
    if (piVar20 == (int *)0x0) {
      pIStack_270 = (IUnknown *)0x0;
    }
    else {
      *piVar20 = (int)uVar34 * 2;
      pIVar33 = (IUnknown *)(piVar20 + 1);
      if (lVar17 != 0) {
        FUN_142ef7ba0(pIVar33,lVar17,uVar34 * 2);
      }
      *(undefined2 *)(pIVar33 + uVar34 * 2) = 0;
      pIStack_270 = pIVar33;
    }
  }
  else {
    if ((sStack_278 == 8) && (sStack_278 = 0, pIStack_270 != (IUnknown *)0x0)) {
      (*DAT_143ad5990)(pIStack_270 + -4);
    }
    iVar28 = (*DAT_143262a28)(&sStack_278,&DAT_143a8b8d8);
    if (iVar28 < 0) {
LAB_141c53dd6:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
  }
  (*DAT_143262a20)(&pIStack_230);
  if (DAT_143a8b8d8 == 8) {
    if ((short)pIStack_230 == 8) {
      pIStack_230 = (IUnknown *)((ulonglong)pIStack_230._2_6_ << 0x10);
      if (pIStack_228 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_228 + -4);
      }
    }
    else {
      iVar28 = (*DAT_143262a18)(&pIStack_230);
      if (iVar28 < 0) goto LAB_141c53dde;
    }
    lVar17 = DAT_143a8b8e0;
    pIStack_230 = (IUnknown *)CONCAT62(pIStack_230._2_6_,8);
    if (DAT_143a8b8e0 == 0) {
      uVar34 = 0;
    }
    else {
      uVar34 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    pIStack_228 = (IUnknown *)(*DAT_143ad5980)();
    if (pIStack_228 != (IUnknown *)0x0) {
      *(int *)pIStack_228 = (int)uVar34 * 2;
      pIVar33 = pIStack_228 + 4;
      if (lVar17 != 0) {
        FUN_142ef7ba0(pIVar33,lVar17,uVar34 * 2);
      }
      *(undefined2 *)(pIVar33 + uVar34 * 2) = 0;
      pIStack_228 = pIVar33;
    }
  }
  else {
    if (((short)pIStack_230 == 8) &&
       (pIStack_230 = (IUnknown *)((ulonglong)pIStack_230._2_6_ << 0x10),
       pIStack_228 != (IUnknown *)0x0)) {
      (*DAT_143ad5990)(pIStack_228 + -4);
    }
    iVar28 = (*DAT_143262a28)(&pIStack_230,&DAT_143a8b8d8);
    if (iVar28 < 0) {
LAB_141c53dde:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
  }
  (*DAT_143262a20)(&pIStack_260);
  if (DAT_143a8b8d8 == 8) {
    if ((short)pIStack_260 == 8) {
      pIStack_260 = (IUnknown *)((ulonglong)pIStack_260._2_6_ << 0x10);
      if (pIStack_258 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_258 + -4);
      }
    }
    else {
      iVar28 = (*DAT_143262a18)(&pIStack_260);
      if (iVar28 < 0) goto LAB_141c53de6;
    }
    lVar17 = DAT_143a8b8e0;
    pIStack_260 = (IUnknown *)CONCAT62(pIStack_260._2_6_,8);
    if (DAT_143a8b8e0 == 0) {
      uVar34 = 0;
    }
    else {
      uVar34 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    pIVar33 = (IUnknown *)(*DAT_143ad5980)();
    if (pIVar33 != (IUnknown *)0x0) {
      *(int *)pIVar33 = (int)uVar34 * 2;
      pIVar33 = pIVar33 + 4;
      if (lVar17 != 0) {
        FUN_142ef7ba0(pIVar33,lVar17,uVar34 * 2);
      }
      *(undefined2 *)(pIVar33 + uVar34 * 2) = 0;
    }
  }
  else {
    if (((short)pIStack_260 == 8) &&
       (pIStack_260 = (IUnknown *)((ulonglong)pIStack_260._2_6_ << 0x10),
       pIStack_258 != (IUnknown *)0x0)) {
      (*DAT_143ad5990)(pIStack_258 + -4);
    }
    iVar28 = (*DAT_143262a28)(&pIStack_260,&DAT_143a8b8d8);
    pIVar33 = pIStack_258;
    if (iVar28 < 0) {
LAB_141c53de6:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
  }
  pIStack_258 = pIVar33;
  uVar38 = uStack_240._4_4_;
  sStack_248 = 3;
  uVar11 = 0;
  uStack_240 = (IUnknown *)((ulonglong)uStack_240._4_4_ << 0x20);
  pIStackX_20 = (IUnknown *)0x0;
  uStack_218 = (IUnknown *)CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
  uStack_210 = pIStack_270;
  plStack_208 = plStack_268;
  uStack_1f8 = pIStack_230;
  pIStack_1f0 = pIStack_228;
  plStack_1e8 = plStack_220;
  pIStack_188 = pIStack_260;
  uStack_180 = pIStack_258;
  plStack_178 = plStack_250;
  uStack_1b8 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,3));
  pIStack_1b0 = (IUnknown *)((ulonglong)uVar38 << 0x20);
  plStack_1a8 = plStack_238;
  ppIVar39 = &pIStack_188;
  uVar30 = uVar30 & 0xffffffff00000000;
  uVar37 = uVar37 & 0xffffffff00000000;
  iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x168))
                     (pIVar31,0,0,0,uVar37,uVar30,&uStack_1b8,ppIVar39,&uStack_1f8,&uStack_218,
                      &pIStackX_20);
  if (iVar28 < 0) {
    _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcd0);
  }
  pIVar31 = (IUnknown *)param_1[0xc2];
  pIVar33 = pIStackX_20;
  if (pIVar31 != pIStackX_20) {
    param_1[0xc2] = (longlong)pIStackX_20;
    pIVar33 = (IUnknown *)0x0;
    if (pIVar31 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar31 + 0x10))();
      pIVar33 = (IUnknown *)0x0;
    }
  }
  if (pIVar33 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)pIVar33 + 0x10))(pIVar33);
  }
  if (sStack_248 == 8) {
    sStack_248 = 0;
    if (uStack_240 != (IUnknown *)0x0) {
      (*DAT_143ad5990)((longlong)uStack_240 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&sStack_248);
  }
  if ((short)pIStack_260 == 8) {
    pIStack_260 = (IUnknown *)((ulonglong)pIStack_260 & 0xffffffffffff0000);
    if (pIStack_258 != (IUnknown *)0x0) {
      (*DAT_143ad5990)(pIStack_258 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&pIStack_260);
  }
  if ((short)pIStack_230 == 8) {
    pIStack_230 = (IUnknown *)((ulonglong)pIStack_230 & 0xffffffffffff0000);
    if (pIStack_228 != (IUnknown *)0x0) {
      (*DAT_143ad5990)(pIStack_228 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&pIStack_230);
  }
  if (sStack_278 == 8) {
    sStack_278 = 0;
    if (pIStack_270 != (IUnknown *)0x0) {
      (*DAT_143ad5990)(pIStack_270 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&sStack_278);
  }
  pIVar31 = (IUnknown *)param_1[0xc2];
  if (pIVar31 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  uStack_240 = (IUnknown *)param_1[0x57];
  sStack_248 = 0xd;
  if (uStack_240 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)uStack_240 + 8))();
  }
  uStack_1b8 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,sStack_248));
  pIStack_1b0 = uStack_240;
  plStack_1a8 = plStack_238;
  iVar28 = (**(code **)(*(longlong *)pIVar31 + 200))(pIVar31,&uStack_1b8);
  if (iVar28 < 0) {
    _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_143273488);
  }
  if (sStack_248 == 8) {
    sStack_248 = 0;
    if (uStack_240 != (IUnknown *)0x0) {
      (*DAT_143ad5990)(uStack_240 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&sStack_248);
  }
  pIVar31 = (IUnknown *)param_1[0xc2];
  if (pIVar31 == (IUnknown *)0x0) goto LAB_141c53fd8;
  iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x200))(pIVar31,0xffffff);
  if (iVar28 < 0) {
    _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
  }
  cVar8 = FUN_140479e60(param_1[0x75]);
  pIVar31 = DAT_143add050;
  if (cVar8 == '\0') {
    if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&pIStack_230);
    iVar28 = FUN_14023c4c0(&pIStack_230,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&pIStack_260);
    iVar28 = FUN_14023c4c0(&pIStack_260,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&sStack_278);
    iVar28 = FUN_14023c4c0(&sStack_278,&DAT_143a8b8d8);
    pIVar33 = uStack_240;
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    sStack_248 = 3;
    uStack_240 = (IUnknown *)((ulonglong)uStack_240 & 0xffffffff00000000);
    pIStackX_20 = (IUnknown *)0x0;
    uStack_1b8 = pIStack_230;
    pIStack_1b0 = pIStack_228;
    plStack_1a8 = plStack_220;
    pIStack_188 = pIStack_260;
    uStack_180 = pIStack_258;
    plStack_178 = plStack_250;
    uStack_218 = (IUnknown *)CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
    uStack_210 = pIStack_270;
    plStack_208 = plStack_268;
    uStack_1f8 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,3));
    pIStack_1f0 = (IUnknown *)((ulonglong)pIVar33 & 0xffffffff00000000);
    plStack_1e8 = plStack_238;
    ppIVar39 = (IUnknown **)&uStack_218;
    uVar30 = uVar30 & 0xffffffff00000000;
    uVar37 = uVar37 & 0xffffffff00000000;
    iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x168))
                       (pIVar31,0,0,0,uVar37,uVar30,&uStack_1f8,ppIVar39,&pIStack_188,&uStack_1b8,
                        &pIStackX_20);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcd0);
    }
    pIVar31 = (IUnknown *)param_1[0xc3];
    pIVar33 = pIStackX_20;
    if (pIVar31 != pIStackX_20) {
      param_1[0xc3] = (longlong)pIStackX_20;
      pIVar33 = (IUnknown *)0x0;
      if (pIVar31 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar31 + 0x10))();
        pIVar33 = (IUnknown *)0x0;
      }
    }
    if (pIVar33 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar33 + 0x10))(pIVar33);
    }
    if (sStack_248 == 8) {
      sStack_248 = 0;
      if (uStack_240 != (IUnknown *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_240 - 4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_248);
    }
    if (sStack_278 == 8) {
      sStack_278 = 0;
      if (pIStack_270 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_270 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_278);
    }
    if ((short)pIStack_260 == 8) {
      pIStack_260 = (IUnknown *)((ulonglong)pIStack_260 & 0xffffffffffff0000);
      if (pIStack_258 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_258 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&pIStack_260);
    }
    if ((short)pIStack_230 == 8) {
      pIStack_230 = (IUnknown *)((ulonglong)pIStack_230 & 0xffffffffffff0000);
      if (pIStack_228 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_228 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&pIStack_230);
    }
    pIVar31 = (IUnknown *)param_1[0xc3];
    if (pIVar31 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    uStack_240 = (IUnknown *)param_1[0x57];
    sStack_248 = 0xd;
    if (uStack_240 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)uStack_240 + 8))();
    }
    uStack_1b8 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,sStack_248));
    pIStack_1b0 = uStack_240;
    plStack_1a8 = plStack_238;
    iVar28 = (**(code **)(*(longlong *)pIVar31 + 200))(pIVar31,&uStack_1b8);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_143273488);
    }
    if (sStack_248 == 8) {
      sStack_248 = 0;
      if (uStack_240 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(uStack_240 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_248);
    }
    pIVar31 = (IUnknown *)param_1[0xc3];
    if (pIVar31 == (IUnknown *)0x0) {
LAB_141c53fd8:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x200))(pIVar31,0xffffffff);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
    }
    pIVar31 = (IUnknown *)param_1[0xc3];
    if (pIVar31 == (IUnknown *)0x0) goto LAB_141c53fd8;
    iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x338))(pIVar31,1,8);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
    }
    pIVar31 = (IUnknown *)param_1[0xc3];
    if (pIVar31 == (IUnknown *)0x0) goto LAB_141c53fd8;
    iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x198))(pIVar31,1000);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
    }
    pIVar31 = (IUnknown *)param_1[0xc3];
    if (pIVar31 == (IUnknown *)0x0) goto LAB_141c53fd8;
    iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x2b8))(pIVar31,0);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
    }
  }
  iVar28 = iStack_1cc;
  uVar12 = aplStack_168[0]._0_4_;
  switch(aplStack_168[0]._0_4_) {
  case 0xfffffffa:
    FUN_141c56870(param_1,iStack_1cc);
    pIVar31 = (IUnknown *)param_1[0xc2];
    if (pIVar31 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    pIStackX_20 = (IUnknown *)0x0;
    iVar13 = (**(code **)(*(longlong *)pIVar31 + 0x208))(pIVar31,&pIStackX_20);
    if (iVar13 < 0) {
      _com_issue_errorex(iVar13,pIVar31,(_GUID *)&DAT_14327fcb0);
    }
    pIVar31 = pIStackX_20;
    pIStack_230 = pIStackX_20;
    if (pIStackX_20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&pIStack_260);
    iVar13 = FUN_14023c4c0(&pIStack_260,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar13);
    }
    (*DAT_143262a20)(&sStack_278);
    iVar13 = FUN_14023c4c0(&sStack_278,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar13);
    }
    (*DAT_143262a20)(&sStack_248);
    iVar13 = FUN_14023c4c0(&sStack_248,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar13);
    }
    (*DAT_143262a20)(&uStack_1f8);
    iVar13 = FUN_14023c4c0(&uStack_1f8,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar13);
    }
    (*DAT_143262a20)(&uStack_218);
    iVar13 = FUN_14023c4c0(&uStack_218,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar13);
    }
    uStack_118 = pIStack_260;
    pIStack_110 = pIStack_258;
    plStack_108 = plStack_250;
    uStack_138 = CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
    pIStack_130 = pIStack_270;
    plStack_128 = plStack_268;
    pIStack_158 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,sStack_248));
    pIStack_150 = uStack_240;
    plStack_148 = plStack_238;
    uStack_1b8 = uStack_1f8;
    pIStack_1b0 = pIStack_1f0;
    plStack_1a8 = plStack_1e8;
    pIStack_188 = uStack_218;
    uStack_180 = uStack_210;
    plStack_178 = plStack_208;
    puVar16 = &uStack_138;
    ppIVar39 = &pIStack_158;
    puVar21 = &uStack_1b8;
    iVar13 = (**(code **)(*(longlong *)pIVar31 + 0x140))
                       (pIVar31,0xff,0,&pIStack_188,puVar21,ppIVar39,puVar16,&uStack_118);
    if (iVar13 < 0) {
      _com_issue_errorex(iVar13,pIVar31,(_GUID *)&DAT_143273488);
    }
    if ((short)uStack_218 == 8) {
      uStack_218 = (IUnknown *)((ulonglong)uStack_218 & 0xffffffffffff0000);
      if (uStack_210 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(uStack_210 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&uStack_218);
    }
    if ((short)uStack_1f8 == 8) {
      uStack_1f8 = (IUnknown *)((ulonglong)uStack_1f8 & 0xffffffffffff0000);
      if (pIStack_1f0 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_1f0 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&uStack_1f8);
    }
    if (sStack_248 == 8) {
      sStack_248 = 0;
      if (uStack_240 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(uStack_240 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_248);
    }
    if (sStack_278 == 8) {
      sStack_278 = 0;
      if (pIStack_270 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_270 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_278);
    }
    if ((short)pIStack_260 == 8) {
      pIStack_260 = (IUnknown *)((ulonglong)pIStack_260 & 0xffffffffffff0000);
      if (pIStack_258 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_258 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&pIStack_260);
    }
    (**(code **)(*(longlong *)pIVar31 + 0x10))(pIVar31);
    if (iVar28 - 0x33U < 0x10) {
      uVar11 = FUN_14019a5d0(param_1 + 0x103);
      uVar12 = FUN_14019a5d0(param_1 + 0x106);
      uVar11 = FUN_1429ed960(uVar12,uVar11);
      FUN_1429f28f0(*(undefined4 *)(param_1[0x75] + 0x60),iVar28 + 3,uVar11,0,
                    (ulonglong)puVar21 & 0xffffffff00000000,(ulonglong)ppIVar39 & 0xffffffff00000000
                    ,(ulonglong)puVar16 & 0xffffffff00000000);
    }
    break;
  case 0xfffffffb:
    iVar28 = iStack_1d0 + 0x32a;
    if (iVar28 == 0) {
      iVar28 = 1;
    }
    *(int *)(param_1 + 0x9c) = iVar28;
    *(undefined4 *)((longlong)param_1 + 0x504) = 1;
    break;
  case 0xfffffffc:
    lVar17 = param_1[0x75];
    if (*(int *)(lVar17 + 0x2c8) != 0) {
      FUN_141c56870(param_1,5);
      pIVar31 = (IUnknown *)param_1[0xc2];
      if (pIVar31 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      pIStackX_20 = (IUnknown *)0x0;
      iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x208))(pIVar31,&pIStackX_20);
      if (iVar28 < 0) {
        _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
      }
      pIVar31 = pIStackX_20;
      pIStack_230 = pIStackX_20;
      if (pIStackX_20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&pIStack_260);
      iVar28 = FUN_14023c4c0(&pIStack_260,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&sStack_278);
      iVar28 = FUN_14023c4c0(&sStack_278,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&sStack_248);
      iVar28 = FUN_14023c4c0(&sStack_248,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&uStack_1f8);
      iVar28 = FUN_14023c4c0(&uStack_1f8,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&uStack_218);
      iVar28 = FUN_14023c4c0(&uStack_218,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      uStack_118 = pIStack_260;
      pIStack_110 = pIStack_258;
      plStack_108 = plStack_250;
      uStack_138 = CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
      pIStack_130 = pIStack_270;
      plStack_128 = plStack_268;
      pIStack_158 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,sStack_248));
      pIStack_150 = uStack_240;
      plStack_148 = plStack_238;
      uStack_1b8 = uStack_1f8;
      pIStack_1b0 = pIStack_1f0;
      plStack_1a8 = plStack_1e8;
      pIStack_188 = uStack_218;
      uStack_180 = uStack_210;
      plStack_178 = plStack_208;
      ppIVar39 = (IUnknown **)&uStack_118;
      puVar16 = &uStack_138;
      ppIVar40 = &pIStack_158;
      puVar21 = &uStack_1b8;
      iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x140))
                         (pIVar31,0xff,0,&pIStack_188,puVar21,ppIVar40,puVar16,ppIVar39);
      if (iVar28 < 0) {
        _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_143273488);
      }
      if ((short)uStack_218 == 8) {
        uStack_218 = (IUnknown *)((ulonglong)uStack_218 & 0xffffffffffff0000);
        if (uStack_210 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(uStack_210 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&uStack_218);
      }
      if ((short)uStack_1f8 == 8) {
        uStack_1f8 = (IUnknown *)((ulonglong)uStack_1f8 & 0xffffffffffff0000);
        if (pIStack_1f0 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_1f0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&uStack_1f8);
      }
      if (sStack_248 == 8) {
        sStack_248 = 0;
        if (uStack_240 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(uStack_240 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&sStack_248);
      }
      if (sStack_278 == 8) {
        sStack_278 = 0;
        if (pIStack_270 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_270 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&sStack_278);
      }
      if ((short)pIStack_260 == 8) {
        pIStack_260 = (IUnknown *)((ulonglong)pIStack_260 & 0xffffffffffff0000);
        if (pIStack_258 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_258 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&pIStack_260);
      }
      (**(code **)(*(longlong *)pIVar31 + 0x10))(pIVar31);
      lVar17 = (**(code **)(param_1[1] + 0x30))(param_1 + 1,&pIStackX_20);
      uVar11 = *(undefined4 *)(lVar17 + 4);
      puVar25 = (undefined4 *)(**(code **)(param_1[1] + 0x30))(param_1 + 1,&pIStack_230);
      uVar11 = FUN_1429ed960(*puVar25,uVar11);
      uVar30 = (ulonglong)ppIVar40 & 0xffffffff00000000;
      uVar37 = (ulonglong)puVar21 & 0xffffffff00000000;
      FUN_1429f28f0(*(undefined4 *)(param_1[0x75] + 0x60),0x48,uVar11,0,uVar37,uVar30,
                    (ulonglong)puVar16 & 0xffffffff00000000);
      lVar17 = param_1[0x75];
    }
    *(undefined4 *)(param_1 + 0x9c) = 0;
    *(undefined4 *)((longlong)param_1 + 0x504) = 1;
    if (*(int *)(lVar17 + 0x60) == 0x8dea53) {
      FUN_140e1ddf0(DAT_143abfdf0,DAT_14327aac0,0,0,uVar37 & 0xffffffff00000000,
                    uVar30 & 0xffffffff00000000,0,(ulonglong)ppIVar39 & 0xffffffff00000000);
    }
    break;
  case 0xfffffffd:
    lVar17 = FUN_141d2efc0(DAT_143abfe00,iStack_1cc);
    if (lVar17 != 0) {
      pIStackX_20 = (IUnknown *)CONCAT44(pIStackX_20._4_4_,(int)param_1[0x74]);
      FUN_14025cbd0(lVar17 + 0x548,&pIStackX_20);
      *(undefined4 *)((longlong)param_1 + 0x504) = 1;
      *(int *)(param_1 + 0x22c) = iVar28;
    }
    break;
  case 0xfffffffe:
    if (*(int *)(param_1[0x75] + 0x2c8) == 0) {
      iVar28 = iStack_1d0 + 800;
      if (iVar28 == 0) {
        iVar28 = 1;
      }
      *(int *)(param_1 + 0x9c) = iVar28;
      pIVar31 = (IUnknown *)param_1[0xc2];
      if (pIVar31 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      pIStackX_20 = (IUnknown *)0x0;
      iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x208))(pIVar31,&pIStackX_20);
      if (iVar28 < 0) {
        _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
      }
      pIVar31 = pIStackX_20;
      pIStack_230 = pIStackX_20;
      if (pIStackX_20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&pIStack_260);
      iVar28 = FUN_14023c4c0(&pIStack_260,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&sStack_278);
      iVar28 = FUN_14023c4c0(&sStack_278,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&sStack_248);
      iVar28 = FUN_14023c4c0(&sStack_248,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&uStack_1f8);
      iVar28 = FUN_14023c4c0(&uStack_1f8,&DAT_143a8b8d8);
      pIVar33 = uStack_218;
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      uStack_218 = (IUnknown *)CONCAT62(uStack_218._2_6_,3);
      pIVar26 = uStack_218;
      uStack_210 = (IUnknown *)CONCAT44(uStack_210._4_4_,(int)param_1[0x9c]);
      uStack_118 = pIStack_260;
      pIStack_110 = pIStack_258;
      plStack_108 = plStack_250;
      uStack_138 = CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
      pIStack_130 = pIStack_270;
      plStack_128 = plStack_268;
      pIStack_158 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,sStack_248));
      pIStack_150 = uStack_240;
      plStack_148 = plStack_238;
      uStack_1b8 = uStack_1f8;
      pIStack_1b0 = pIStack_1f0;
      plStack_1a8 = plStack_1e8;
      uStack_218._4_4_ = SUB84(pIVar33,4);
      pIStack_188 = (IUnknown *)CONCAT44(uStack_218._4_4_,(int)pIVar26);
      uStack_180 = (IUnknown *)CONCAT44(uStack_210._4_4_,(int)param_1[0x9c]);
      plStack_178 = plStack_208;
      uStack_218 = pIVar26;
      iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x140))
                         (pIVar31,0xff,0xff,&pIStack_188,&uStack_1b8,&pIStack_158,&uStack_138,
                          &uStack_118);
      if (iVar28 < 0) {
        _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_143273488);
      }
      if ((short)uStack_218 == 8) {
        uStack_218 = (IUnknown *)((ulonglong)uStack_218 & 0xffffffffffff0000);
        if (uStack_210 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(uStack_210 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&uStack_218);
      }
      if ((short)uStack_1f8 == 8) {
        uStack_1f8 = (IUnknown *)((ulonglong)uStack_1f8 & 0xffffffffffff0000);
        if (pIStack_1f0 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_1f0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&uStack_1f8);
      }
      if (sStack_248 == 8) {
        sStack_248 = 0;
        if (uStack_240 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(uStack_240 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&sStack_248);
      }
      if (sStack_278 == 8) {
        sStack_278 = 0;
        if (pIStack_270 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_270 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&sStack_278);
      }
      if ((short)pIStack_260 == 8) {
        pIStack_260 = (IUnknown *)((ulonglong)pIStack_260 & 0xffffffffffff0000);
        if (pIStack_258 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_258 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&pIStack_260);
      }
      (**(code **)(*(longlong *)pIVar31 + 0x10))(pIVar31);
    }
    else {
      FUN_141c56870(param_1,5);
      pIVar31 = (IUnknown *)param_1[0xc2];
      if (pIVar31 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      pIStackX_20 = (IUnknown *)0x0;
      iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x208))(pIVar31,&pIStackX_20);
      if (iVar28 < 0) {
        _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
      }
      pIVar31 = pIStackX_20;
      pIStack_230 = pIStackX_20;
      if (pIStackX_20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&pIStack_260);
      iVar28 = FUN_14023c4c0(&pIStack_260,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&sStack_278);
      iVar28 = FUN_14023c4c0(&sStack_278,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&sStack_248);
      iVar28 = FUN_14023c4c0(&sStack_248,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&uStack_1f8);
      iVar28 = FUN_14023c4c0(&uStack_1f8,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      (*DAT_143262a20)(&uStack_218);
      iVar28 = FUN_14023c4c0(&uStack_218,&DAT_143a8b8d8);
      if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar28);
      }
      uStack_118 = pIStack_260;
      pIStack_110 = pIStack_258;
      plStack_108 = plStack_250;
      uStack_138 = CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
      pIStack_130 = pIStack_270;
      plStack_128 = plStack_268;
      pIStack_158 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,sStack_248));
      pIStack_150 = uStack_240;
      plStack_148 = plStack_238;
      uStack_1b8 = uStack_1f8;
      pIStack_1b0 = pIStack_1f0;
      plStack_1a8 = plStack_1e8;
      pIStack_188 = uStack_218;
      uStack_180 = uStack_210;
      plStack_178 = plStack_208;
      puVar16 = &uStack_138;
      ppIVar39 = &pIStack_158;
      puVar21 = &uStack_1b8;
      iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x140))
                         (pIVar31,0xff,0,&pIStack_188,puVar21,ppIVar39,puVar16,&uStack_118);
      if (iVar28 < 0) {
        _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_143273488);
      }
      if ((short)uStack_218 == 8) {
        uStack_218 = (IUnknown *)((ulonglong)uStack_218 & 0xffffffffffff0000);
        if (uStack_210 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(uStack_210 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&uStack_218);
      }
      if ((short)uStack_1f8 == 8) {
        uStack_1f8 = (IUnknown *)((ulonglong)uStack_1f8 & 0xffffffffffff0000);
        if (pIStack_1f0 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_1f0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&uStack_1f8);
      }
      if (sStack_248 == 8) {
        sStack_248 = 0;
        if (uStack_240 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(uStack_240 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&sStack_248);
      }
      if (sStack_278 == 8) {
        sStack_278 = 0;
        if (pIStack_270 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_270 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&sStack_278);
      }
      if ((short)pIStack_260 == 8) {
        pIStack_260 = (IUnknown *)((ulonglong)pIStack_260 & 0xffffffffffff0000);
        if (pIStack_258 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIStack_258 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&pIStack_260);
      }
      (**(code **)(*(longlong *)pIVar31 + 0x10))(pIVar31);
      lVar17 = (**(code **)(param_1[1] + 0x30))(param_1 + 1,&pIStackX_20);
      uVar11 = *(undefined4 *)(lVar17 + 4);
      puVar25 = (undefined4 *)(**(code **)(param_1[1] + 0x30))(param_1 + 1,&pIStack_230);
      uVar11 = FUN_1429ed960(*puVar25,uVar11);
      FUN_1429f28f0(*(undefined4 *)(param_1[0x75] + 0x60),0x48,uVar11,0,
                    (ulonglong)puVar21 & 0xffffffff00000000,(ulonglong)ppIVar39 & 0xffffffff00000000
                    ,(ulonglong)puVar16 & 0xffffffff00000000);
      FUN_141cf70a0(param_1,param_1[0x75] + 0x1f60);
    }
    break;
  case 0xffffffff:
    pIVar31 = (IUnknown *)param_1[0xc2];
    if (pIVar31 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    pIStackX_20 = (IUnknown *)0x0;
    iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x208))(pIVar31,&pIStackX_20);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_14327fcb0);
    }
    pIVar31 = pIStackX_20;
    pIStack_230 = pIStackX_20;
    if (pIStackX_20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&uStack_218);
    iVar28 = FUN_14023c4c0(&uStack_218,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&uStack_1f8);
    iVar28 = FUN_14023c4c0(&uStack_1f8,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&pIStack_260);
    iVar28 = FUN_14023c4c0(&pIStack_260,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&sStack_278);
    iVar28 = FUN_14023c4c0(&sStack_278,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&sStack_248);
    iVar28 = FUN_14023c4c0(&sStack_248,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    uStack_1b8 = uStack_218;
    pIStack_1b0 = uStack_210;
    plStack_1a8 = plStack_208;
    pIStack_188 = uStack_1f8;
    uStack_180 = pIStack_1f0;
    plStack_178 = plStack_1e8;
    pIStack_158 = pIStack_260;
    pIStack_150 = pIStack_258;
    plStack_148 = plStack_250;
    uStack_138 = CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
    pIStack_130 = pIStack_270;
    plStack_128 = plStack_268;
    uStack_118 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,sStack_248));
    pIStack_110 = uStack_240;
    plStack_108 = plStack_238;
    iVar28 = (**(code **)(*(longlong *)pIVar31 + 0x140))
                       (pIVar31,0xff,0,&uStack_118,&uStack_138,&pIStack_158,&pIStack_188,&uStack_1b8
                       );
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar31,(_GUID *)&DAT_143273488);
    }
    if (sStack_248 == 8) {
      sStack_248 = 0;
      if (uStack_240 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(uStack_240 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_248);
    }
    if (sStack_278 == 8) {
      sStack_278 = 0;
      if (pIStack_270 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_270 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_278);
    }
    if ((short)pIStack_260 == 8) {
      pIStack_260 = (IUnknown *)((ulonglong)pIStack_260 & 0xffffffffffff0000);
      if (pIStack_258 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_258 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&pIStack_260);
    }
    if ((short)uStack_1f8 == 8) {
      uStack_1f8 = (IUnknown *)((ulonglong)uStack_1f8 & 0xffffffffffff0000);
      if (pIStack_1f0 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_1f0 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&uStack_1f8);
    }
    if ((short)uStack_218 == 8) {
      uStack_218 = (IUnknown *)((ulonglong)uStack_218 & 0xffffffffffff0000);
      if (uStack_210 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(uStack_210 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&uStack_218);
    }
    (**(code **)(*(longlong *)pIVar31 + 0x10))(pIVar31);
    break;
  default:
    pIStackX_20 = (IUnknown *)0x0;
    FUN_1401c21c0(&pIStackX_20,PTR_u_Effect_Summon_img__d_143a48f68,
                  (ulonglong)aplStack_168[0] & 0xffffffff);
    pIVar31 = pIStackX_20;
    FUN_14090ead0(aplStack_1c8,pIStackX_20);
    puVar5 = PTR_u_delay_143a45cd8;
    switch(uVar12) {
    case 0x54:
      uVar11 = 0x960;
      break;
    case 0x55:
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
      uVar11 = 0x4b0;
    }
    aplStack_168[0] = aplStack_1c8[0];
    if (aplStack_1c8[0] != (longlong *)0x0) {
      (**(code **)(*aplStack_1c8[0] + 8))();
    }
    iVar28 = FUN_140910eb0(aplStack_168,puVar5,uVar11);
    *(int *)((longlong)param_1 + 0x304) = iStack_1cc + iStack_1d0;
    if (iStack_1cc + iStack_1d0 == 0) {
      *(undefined4 *)((longlong)param_1 + 0x304) = 1;
    }
    iVar28 = iVar28 + iStack_1cc + iStack_1d0;
    if (iVar28 == 0) {
      iVar28 = 1;
    }
    *(int *)(param_1 + 0x9c) = iVar28;
    *(undefined4 *)(param_1 + 0x60) = uVar12;
    *(undefined4 *)((longlong)param_1 + 0x504) = 1;
    if (aplStack_1c8[0] != (longlong *)0x0) {
      (**(code **)(*aplStack_1c8[0] + 0x10))();
    }
    if (pIVar31 != (IUnknown *)0x0) {
      FUN_1401bebb0(pIVar31 + -0x10);
    }
  }
  iVar28 = 0;
  FUN_141cb9fd0(param_1);
  (**(code **)(*param_1 + 0xc0))(param_1);
  FUN_141c9c210(param_1,0);
  uVar38 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x2e4) = uVar38;
  uVar32 = ~uVar38 >> 5 | ~uVar38 << 0x1b;
  *(uint *)(param_1 + 0x5d) = uVar32;
  *(uint *)((longlong)param_1 + 0x2ec) =
       ((uVar38 ^ 0xbaadf00d) >> 5 | (uVar38 ^ 0xbaadf00d) << 0x1b) + uVar32;
  FUN_141d23780(param_1,0);
  *(undefined4 *)(param_1 + 0x3f) = 0;
  ppIVar39 = (IUnknown **)(param_1 + 0x41);
  if (ppIVar39 != &pIStackX_20) {
    FUN_1401be120(ppIVar39);
    *ppIVar39 = (IUnknown *)0x0;
  }
  uVar11 = FUN_141d238c0(param_1,0);
  FUN_141d23820(param_1,uVar11);
  *(undefined4 *)(param_1 + 0x1d0) = 0;
  param_1[0x43] = 0;
  ppIVar39 = (IUnknown **)(param_1 + 0x42);
  if (ppIVar39 != &pIStackX_20) {
    FUN_1401be120(ppIVar39);
    *ppIVar39 = (IUnknown *)0x0;
  }
  if (0 < *(int *)(param_1[0x75] + 0x130)) {
    FUN_141caaee0(param_1,1,1,1);
  }
  if (DAT_143aa8518 != (longlong *)0x0) {
    uVar23 = (**(code **)(*DAT_143aa8518 + 0x48))();
    iVar13 = aiStackX_10[0];
    plVar22 = param_1 + 0x55;
    do {
      plVar19 = (longlong *)FUN_140878880(uVar23,iVar28);
      cVar8 = (**(code **)(*plVar19 + 8))(plVar19);
      if (((cVar8 != '\0') && (lVar17 = FUN_140878880(uVar23,iVar28), lVar17 != 0)) &&
         (iVar14 = FUN_1408f5740(lVar17), iVar14 == iVar13)) {
        uVar24 = FUN_140878880(uVar23,iVar28);
        puVar25 = (undefined4 *)FUN_1408f51a0(uVar24);
        uVar11 = *puVar25;
        iVar14 = FUN_1429e3ef0();
        puVar21 = (undefined8 *)FUN_141d1ab50(param_1 + 0x4b);
        *(undefined4 *)(puVar21 + 3) = uVar11;
        *(int *)(puVar21 + 4) = iVar14;
        if (iVar14 == 0) {
          *(undefined4 *)(puVar21 + 4) = 1;
        }
        *(undefined4 *)((longlong)puVar21 + 0x2c) = 1;
        *puVar21 = 0;
        puVar21[1] = 0;
        *(undefined4 *)((longlong)puVar21 + 0x34) = 0xff;
        *(undefined4 *)(puVar21 + 10) = 0;
        *plVar22 = param_1[0x4d];
        FUN_142861760(3,param_1,*(undefined4 *)(puVar21 + 3));
      }
      iVar28 = iVar28 + 1;
      plVar22 = plVar22 + 1;
      uVar24 = uStackX_18;
    } while (iVar28 < 2);
  }
  pIVar31 = (IUnknown *)0x0;
  FUN_141ce0d50(param_1);
  if ((int)pplStack_f8 != 0) {
    plVar19 = (longlong *)FUN_141c97ff0(param_1,&pIStackX_20,(ulonglong)pplStack_f8 & 0xffffffff);
    plVar22 = (longlong *)param_1[199];
    if (plVar22 != (longlong *)*plVar19) {
      param_1[199] = *plVar19;
      *plVar19 = 0;
      if (plVar22 != (longlong *)0x0) {
        (**(code **)(*plVar22 + 0x10))();
      }
    }
    if (pIStackX_20 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIStackX_20 + 0x10))();
    }
  }
  lVar17 = param_1[0x75];
  if (*(char *)(lVar17 + 0xdb) != '\0') {
    pIVar33 = (IUnknown *)param_1[0xc2];
    if (pIVar33 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    pIStackX_20 = (IUnknown *)0x0;
    iVar28 = (**(code **)(*(longlong *)pIVar33 + 0x208))(pIVar33,&pIStackX_20);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_14327fcb0);
    }
    pIVar33 = pIStackX_20;
    pIStack_230 = pIStackX_20;
    if (pIStackX_20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&pIStack_260);
    iVar28 = FUN_14023c4c0(&pIStack_260,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&sStack_278);
    iVar28 = FUN_14023c4c0(&sStack_278,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&sStack_248);
    iVar28 = FUN_14023c4c0(&sStack_248,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&uStack_1f8);
    iVar28 = FUN_14023c4c0(&uStack_1f8,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    (*DAT_143262a20)(&uStack_218);
    iVar28 = FUN_14023c4c0(&uStack_218,&DAT_143a8b8d8);
    if (iVar28 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar28);
    }
    uStack_118 = pIStack_260;
    pIStack_110 = pIStack_258;
    plStack_108 = plStack_250;
    uStack_138 = CONCAT44(uStack_274,CONCAT22(uStack_276,sStack_278));
    pIStack_130 = pIStack_270;
    plStack_128 = plStack_268;
    pIStack_158 = (IUnknown *)CONCAT44(uStack_244,CONCAT22(uStack_246,sStack_248));
    pIStack_150 = uStack_240;
    plStack_148 = plStack_238;
    uStack_1b8 = uStack_1f8;
    pIStack_1b0 = pIStack_1f0;
    plStack_1a8 = plStack_1e8;
    pIStack_188 = uStack_218;
    uStack_180 = uStack_210;
    plStack_178 = plStack_208;
    iVar28 = (**(code **)(*(longlong *)pIVar33 + 0x140))
                       (pIVar33,0,0,&pIStack_188,&uStack_1b8,&pIStack_158,&uStack_138,&uStack_118);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_143273488);
    }
    if ((short)uStack_218 == 8) {
      uStack_218 = (IUnknown *)((ulonglong)uStack_218 & 0xffffffffffff0000);
      if (uStack_210 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(uStack_210 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&uStack_218);
    }
    if ((short)uStack_1f8 == 8) {
      uStack_1f8 = (IUnknown *)((ulonglong)uStack_1f8 & 0xffffffffffff0000);
      if (pIStack_1f0 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_1f0 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&uStack_1f8);
    }
    if (sStack_248 == 8) {
      sStack_248 = 0;
      if (uStack_240 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(uStack_240 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_248);
    }
    if (sStack_278 == 8) {
      sStack_278 = 0;
      if (pIStack_270 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_270 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&sStack_278);
    }
    if ((short)pIStack_260 == 8) {
      pIStack_260 = (IUnknown *)((ulonglong)pIStack_260 & 0xffffffffffff0000);
      if (pIStack_258 != (IUnknown *)0x0) {
        (*DAT_143ad5990)(pIStack_258 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&pIStack_260);
    }
    (**(code **)(*(longlong *)pIVar33 + 0x10))(pIVar33);
    lVar17 = param_1[0x75];
  }
  plVar22 = plStack_1d8;
  if (*(int *)(lVar17 + 0x178) - 1U < 2) {
    *(undefined4 *)(plStack_1d8 + 0xf5) = 1;
    lVar17 = param_1[0x75];
  }
  if (*(char *)(lVar17 + 0x17e) != '\0') {
    FUN_141cc94d0(param_1);
  }
  if (-1 < (int)ppIStack_f0) {
    *(undefined4 *)(param_1 + 0x19a) = 1;
  }
  pIVar33 = (IUnknown *)param_1[0xc2];
  if (((pIVar33 == (IUnknown *)0x0) || (*(int *)((longlong)param_1 + 0xd64) < 1)) ||
     (*(int *)((longlong)param_1 + 0xd64) == 100)) {
    aiStackX_10[0] = DAT_143286d20;
    pIVar33 = (IUnknown *)param_1[0xc3];
    if (pIVar33 == (IUnknown *)0x0) goto LAB_141c5322b;
  }
  else {
    iVar28 = (**(code **)(*(longlong *)pIVar33 + 0x300))(pIVar33,2);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_14327fcb0);
    }
    pIVar33 = (IUnknown *)param_1[0xc2];
    if (pIVar33 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar28 = (**(code **)(*(longlong *)pIVar33 + 800))
                       (pIVar33,8,*(undefined4 *)((longlong)param_1 + 0xd64));
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_14327fcb0);
    }
    pIVar33 = (IUnknown *)param_1[0xc3];
    if (pIVar33 == (IUnknown *)0x0) goto LAB_141c5322b;
    iVar28 = (**(code **)(*(longlong *)pIVar33 + 0x300))(pIVar33,2);
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_14327fcb0);
    }
    pIVar33 = (IUnknown *)param_1[0xc3];
    if (pIVar33 == (IUnknown *)0x0) {
LAB_141c53dee:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar28 = (**(code **)(*(longlong *)pIVar33 + 800))
                       (pIVar33,8,*(undefined4 *)((longlong)param_1 + 0xd64));
    if (iVar28 < 0) {
      _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_14327fcb0);
    }
    aiStackX_10[0] = DAT_1434b9590;
    pIVar33 = (IUnknown *)param_1[0xc3];
    if (pIVar33 == (IUnknown *)0x0) goto LAB_141c53dee;
  }
  iVar28 = (**(code **)(*(longlong *)pIVar33 + 0x338))(pIVar33,1,0x40,aiStackX_10);
  if (iVar28 < 0) {
    _com_issue_errorex(iVar28,pIVar33,(_GUID *)&DAT_14327fcb0);
  }
LAB_141c5322b:
  if (*(int *)(param_1[0x75] + 0x35c) != 0) {
    ppIStack_f0 = &pIStackX_20;
    pIStackX_20 = (IUnknown *)param_1[0x57];
    if (pIStackX_20 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIStackX_20 + 8))();
    }
    plStack_1d8 = (longlong *)param_1[0xc2];
    pplStack_f8 = &plStack_1d8;
    if (plStack_1d8 != (longlong *)0x0) {
      (**(code **)(*plStack_1d8 + 8))();
    }
    uVar23 = FUN_141ce16c0(param_1,&pIStack_230);
    FUN_141c77860(param_1,uVar23,&plStack_1d8,&pIStackX_20);
  }
  cVar8 = FUN_1406e8ae0(uVar24);
  if (cVar8 != '\0') {
    pIVar26 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0xb0);
    pIVar33 = pIVar31;
    pIStackX_20 = pIVar26;
    if (pIVar26 != (IUnknown *)0x0) {
      *(longlong *)(pIVar26 + 0x18) = 0;
      *(longlong *)(pIVar26 + 8) = 0;
      *(longlong *)(pIVar26 + 0x10) = 0;
      *(undefined ***)pIVar26 = &PTR_FUN_1432792a8;
      *(undefined8 *)(pIVar26 + 0x34) = 0;
      *(longlong *)(pIVar26 + 0x40) = 0;
      pIVar33 = pIVar26 + 0x50;
      *(longlong *)pIVar33 = 0;
      *(longlong *)(pIVar26 + 0x58) = 0;
      pIStack_230 = pIVar33;
      lVar17 = FUN_14019b780(&DAT_143ad68a0,0x48);
      *(longlong *)lVar17 = lVar17;
      *(longlong *)(lVar17 + 8) = lVar17;
      *(longlong *)(lVar17 + 0x10) = lVar17;
      *(undefined2 *)(lVar17 + 0x18) = 0x101;
      *(longlong *)pIVar33 = lVar17;
      *(longlong *)(pIVar26 + 0x60) = 0;
      *(longlong *)(pIVar26 + 0x80) = 0;
      *(longlong *)(pIVar26 + 0x88) = 0;
      *(longlong *)(pIVar26 + 0x90) = 0;
      pIVar33 = pIVar26;
    }
    if ((param_1[0x1d2] - 1U < 999) || (param_1[0x1d2] == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (pIVar33 != (IUnknown *)0x0) {
      FUN_14040eba0(pIVar33);
    }
    pIStack_258 = (IUnknown *)param_1[0x1d2];
    param_1[0x1d2] = (longlong)pIVar33;
    FUN_141d232a0(&pIStack_260);
    lVar17 = param_1[0x1d2];
    if (lVar17 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar17 = param_1[0x1d2];
    }
    FUN_14023dca0(lVar17,uVar24);
    lVar17 = param_1[0x79];
    if (lVar17 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar17 = param_1[0x79];
    }
    lVar18 = param_1[0x144];
    lVar27 = FUN_141892840();
    if (lVar27 == 0) {
      uVar11 = 100;
    }
    else {
      uVar23 = FUN_141892840();
      uVar11 = FUN_141866780(uVar23);
    }
    FUN_14046ad90(lVar17,param_1[0x75],uVar11,lVar18);
    pIStackX_20 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x98);
    pIVar33 = pIVar31;
    if (pIStackX_20 != (IUnknown *)0x0) {
      *(longlong *)pIStackX_20 = 0;
      *(longlong *)(pIStackX_20 + 8) = 0;
      *(undefined4 *)(pIStackX_20 + 8) = 1;
      *(undefined4 *)(pIStackX_20 + 0xc) = 1;
      *(undefined ***)pIStackX_20 = &PTR_LAB_143408918;
      *(longlong *)(pIStackX_20 + 0x10) = 0;
      *(longlong *)(pIStackX_20 + 0x18) = 0;
      *(longlong *)(pIStackX_20 + 0x20) = 0;
      *(longlong *)(pIStackX_20 + 0x28) = 0;
      *(longlong *)(pIStackX_20 + 0x30) = 0;
      *(longlong *)(pIStackX_20 + 0x38) = 0;
      *(longlong *)(pIStackX_20 + 0x40) = 0;
      *(longlong *)(pIStackX_20 + 0x48) = 0;
      *(longlong *)(pIStackX_20 + 0x50) = 0;
      *(longlong *)(pIStackX_20 + 0x58) = 0;
      *(longlong *)(pIStackX_20 + 0x60) = 0;
      *(longlong *)(pIStackX_20 + 0x68) = 0;
      *(longlong *)(pIStackX_20 + 0x70) = 0;
      *(longlong *)(pIStackX_20 + 0x78) = 0;
      *(longlong *)(pIStackX_20 + 0x80) = 0;
      *(longlong *)(pIStackX_20 + 0x88) = 0;
      *(longlong *)(pIStackX_20 + 0x90) = 0;
      *(longlong *)(pIStackX_20 + 0x58) = 0;
      *(longlong *)(pIStackX_20 + 0x60) = 0;
      *(longlong *)(pIStackX_20 + 0x68) = 0;
      pIStackX_20[0x70] = (IUnknown)0x0;
      *(longlong *)(pIStackX_20 + 0x78) = 0;
      pIStackX_20[0x80] = (IUnknown)0x0;
      *(longlong *)(pIStackX_20 + 0x88) = 0;
      *(undefined4 *)(pIStackX_20 + 0x90) = 0;
      pIVar33 = pIStackX_20;
    }
    param_1[0x1d3] = (longlong)(pIVar33 + 0x10);
    lVar17 = param_1[0x1d4];
    param_1[0x1d4] = (longlong)pIVar33;
    if (lVar17 != 0) {
      FUN_1402abcb0(lVar17);
    }
  }
  bVar35 = FUN_1406e8ae0(uVar24);
  *(uint *)((longlong)param_1 + 0xfdc) = (uint)bVar35;
  uVar11 = FUN_1406e8c20(uVar24);
  *(undefined4 *)(param_1 + 0x1fc) = uVar11;
  iVar28 = FUN_1406e8c20(uVar24);
  *(int *)((longlong)param_1 + 0xfe4) = iVar28;
  if (iVar28 != 0) {
    uVar11 = FUN_1406e8c20(uVar24);
    *(undefined4 *)(param_1 + 0x1fd) = uVar11;
  }
  iVar28 = FUN_1406e8c20(uVar24);
  if (iVar28 != 0) {
    pIStackX_20 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x20);
    pIVar33 = pIVar31;
    if (pIStackX_20 != (IUnknown *)0x0) {
      *(longlong *)pIStackX_20 = 0;
      *(longlong *)(pIStackX_20 + 8) = 0;
      *(undefined4 *)(pIStackX_20 + 8) = 1;
      *(undefined4 *)(pIStackX_20 + 0xc) = 1;
      *(undefined ***)pIStackX_20 = &PTR_LAB_143408938;
      *(longlong *)(pIStackX_20 + 0x10) = 0;
      *(undefined8 *)(pIStackX_20 + 0x14) = 0;
      *(undefined4 *)(pIStackX_20 + 0x10) = 0;
      *(undefined4 *)(pIStackX_20 + 0x18) = 0;
      pIVar33 = pIStackX_20;
    }
    pIVar26 = pIVar33 + 0x10;
    param_1[0x221] = (longlong)pIVar26;
    lVar17 = param_1[0x222];
    param_1[0x222] = (longlong)pIVar33;
    if (lVar17 != 0) {
      FUN_1402abcb0(lVar17);
      pIVar26 = (IUnknown *)param_1[0x221];
    }
    FUN_14028d820(pIVar26,uVar24);
  }
  iVar28 = FUN_1406e8c20(uVar24);
  iVar13 = FUN_1406e8c20(uVar24);
  *(int *)(param_1 + 0x223) = iVar13;
  if ((0 < iVar28) && (iVar13 != 0)) {
    pIVar33 = (IUnknown *)param_1[0x57];
    pIStack_230 = pIVar33;
    if (pIVar33 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar33 + 8))(pIVar33);
    }
    plStack_1d8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x50);
    pIVar26 = pIVar31;
    if (plStack_1d8 != (longlong *)0x0) {
      pIStack_230 = (IUnknown *)0x0;
      pIStackX_20 = pIVar33;
      pIVar26 = (IUnknown *)FUN_141a8efe0(plStack_1d8,iVar28,&pIStackX_20);
      pIVar33 = pIVar31;
    }
    ppIVar39 = (IUnknown **)(param_1 + 0x20e);
    if (ppIVar39 == &pIStackX_20) {
      if (pIVar26 != (IUnknown *)0x0) {
        FUN_141a8f070(pIVar26);
        thunk_FUN_140205820(pIVar26,0x50);
      }
    }
    else {
      pIVar31 = *ppIVar39;
      *ppIVar39 = pIVar26;
      if (pIVar31 != (IUnknown *)0x0) {
        FUN_141a8f070(pIVar31);
        thunk_FUN_140205820(pIVar31,0x50);
      }
    }
    if (pIVar33 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar33 + 0x10))(pIVar33);
    }
    FUN_141a90360(*ppIVar39,(int)param_1[0x223]);
    uVar24 = uStackX_18;
  }
  uVar38 = FUN_1406e8c20(uVar24);
  plVar19 = DAT_143aa8518;
  if (0 < (int)uVar38) {
    uVar37 = (ulonglong)uVar38;
    do {
      iVar28 = FUN_1406e8c20(uVar24);
      if ((plVar19 != (longlong *)0x0) && (iVar13 = FUN_14276df20(plVar19), iVar13 == iVar28)) {
        *(undefined4 *)((longlong)param_1 + 0x1104) = 1;
      }
      uVar37 = uVar37 - 1;
    } while (uVar37 != 0);
  }
  pIVar31 = (IUnknown *)0x0;
  iVar28 = FUN_1406e8c20(uVar24);
  if ((((param_1[0x75] != 0) && (*(char *)(param_1[0x75] + 0x2aa) != '\0')) &&
      (iVar13 = (**(code **)(*param_1 + 0x48))(param_1), iVar13 == 0)) &&
     ((plVar19 != (longlong *)0x0 && (iVar13 = FUN_14276df20(plVar19), iVar13 == iVar28)))) {
    (**(code **)(*param_1 + 0x40))(param_1,1);
    uVar38 = FUN_1407386b0(&DAT_143ac1ab0);
    *(uint *)((longlong)param_1 + 0x2e4) = uVar38;
    uVar32 = uVar38 >> 5 | (uVar38 ^ 4) << 0x1b;
    *(uint *)(param_1 + 0x5d) = uVar32;
    *(uint *)((longlong)param_1 + 0x2ec) =
         ((uVar38 ^ 0xbaadf00d) >> 5 | (uVar38 ^ 0xbaadf00d) << 0x1b) + uVar32;
  }
  if ((int)param_1[0x1fc] < 1) {
    *(undefined4 *)((longlong)param_1 + 0xfdc) = 0;
  }
  else if (*(int *)((longlong)param_1 + 0xfdc) != 0) {
    pIVar33 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x60);
    pIStackX_20 = pIVar33;
    if (pIVar33 == (IUnknown *)0x0) {
      pIStack_228 = (IUnknown *)0x0;
      pIVar33 = pIVar31;
    }
    else {
      *(longlong *)(pIVar33 + 0x18) = 0;
      *(longlong *)(pIVar33 + 8) = 0;
      *(longlong *)(pIVar33 + 0x10) = 0;
      *(undefined ***)pIVar33 = &PTR_FUN_143407a20;
      *(longlong *)(pIVar33 + 0x20) = 0;
      *(longlong *)(pIVar33 + 0x28) = 0;
      *(longlong *)(pIVar33 + 0x38) = 0;
      *(undefined4 *)(pIVar33 + 0x40) = 0;
      pIVar33[0x44] = (IUnknown)0x0;
      *(longlong *)(pIVar33 + 0x48) = 0;
      *(longlong *)(pIVar33 + 0x50) = 0;
      *(undefined4 *)(pIVar33 + 0x58) = 0;
      pIStack_228 = pIVar33;
      if (pIVar33 != (IUnknown *)0x0) {
        FUN_14040eba0(pIVar33);
      }
    }
    if ((param_1[0x1ff] - 1U < 999) || (param_1[0x1ff] == -1)) {
      FUN_142e52ed0(0x447);
    }
    if ((IUnknown **)(param_1 + 0x1fe) == &pIStack_230) {
      FUN_142e52d50(0x45c,1);
    }
    FUN_141d202b0(&pIStack_230);
    FUN_141d23230(param_1 + 0x1fe);
    param_1[0x1ff] = (longlong)pIVar33;
    FUN_141d23230(&pIStack_230);
    lVar17 = param_1[0x1ff];
    if (lVar17 != 0) {
      pIStackX_20 = (IUnknown *)param_1[0x57];
      if (pIStackX_20 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIStackX_20 + 8))();
      }
      FUN_141d08380(lVar17,(int)param_1[0x1fc],&pIStackX_20);
    }
  }
  pIStackX_20 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x28);
  pIVar33 = pIVar31;
  if (pIStackX_20 != (IUnknown *)0x0) {
    *(longlong *)pIStackX_20 = 0;
    *(longlong *)(pIStackX_20 + 8) = 0;
    *(undefined4 *)(pIStackX_20 + 8) = 1;
    *(undefined4 *)(pIStackX_20 + 0xc) = 1;
    *(undefined ***)pIStackX_20 = &PTR_LAB_143408958;
    *(longlong *)(pIStackX_20 + 0x10) = 0;
    *(longlong *)(pIStackX_20 + 0x18) = 0;
    *(longlong *)(pIStackX_20 + 0x10) = 0;
    *(longlong *)(pIStackX_20 + 0x18) = 0;
    *(longlong *)(pIStackX_20 + 0x20) = 0;
    pIVar33 = pIStackX_20;
  }
  param_1[0x1d5] = (longlong)(pIVar33 + 0x10);
  lVar17 = param_1[0x1d6];
  param_1[0x1d6] = (longlong)pIVar33;
  if (lVar17 != 0) {
    FUN_1402abcb0(lVar17);
  }
  pIStackX_20 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x20);
  pIVar33 = pIVar31;
  if (pIStackX_20 != (IUnknown *)0x0) {
    *(longlong *)pIStackX_20 = 0;
    *(longlong *)(pIStackX_20 + 8) = 0;
    *(undefined4 *)(pIStackX_20 + 8) = 1;
    *(undefined4 *)(pIStackX_20 + 0xc) = 1;
    *(undefined ***)pIStackX_20 = &PTR_LAB_143408978;
    *(longlong *)(pIStackX_20 + 0x10) = 0;
    *(longlong *)(pIStackX_20 + 0x18) = 0;
    pIVar33 = pIStackX_20;
  }
  param_1[0x1d7] = (longlong)(pIVar33 + 0x10);
  lVar17 = param_1[0x1d8];
  param_1[0x1d8] = (longlong)pIVar33;
  if (lVar17 != 0) {
    FUN_1402abcb0(lVar17);
  }
  pIStackX_20 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x30);
  pIVar33 = pIVar31;
  if (pIStackX_20 != (IUnknown *)0x0) {
    *(longlong *)pIStackX_20 = 0;
    *(longlong *)(pIStackX_20 + 8) = 0;
    *(undefined4 *)(pIStackX_20 + 8) = 1;
    *(undefined4 *)(pIStackX_20 + 0xc) = 1;
    *(undefined ***)pIStackX_20 = &PTR_LAB_143408998;
    *(longlong *)(pIStackX_20 + 0x10) = 0;
    *(longlong *)(pIStackX_20 + 0x18) = 0;
    *(longlong *)(pIStackX_20 + 0x20) = 0;
    *(longlong *)(pIStackX_20 + 0x28) = 0;
    *(undefined4 *)(pIStackX_20 + 0x10) = 0;
    *(longlong *)(pIStackX_20 + 0x18) = 0;
    *(longlong *)(pIStackX_20 + 0x20) = 0;
    *(longlong *)(pIStackX_20 + 0x28) = 0;
    pIVar33 = pIStackX_20;
  }
  param_1[0x172] = (longlong)(pIVar33 + 0x10);
  lVar17 = param_1[0x173];
  param_1[0x173] = (longlong)pIVar33;
  if (lVar17 != 0) {
    FUN_1402abcb0(lVar17);
  }
  puVar21 = puStack_190;
  pIStack_258 = (IUnknown *)puStack_190;
  if (puStack_190 != (undefined8 *)0x0) {
    FUN_14040eba0(puStack_190);
  }
  FUN_141cd1620(param_1,&pIStack_260);
  (**(code **)(*param_1 + 0x108))(param_1,uVar24);
  if ((param_1[0x75] != 0) && (*(longlong *)(param_1[0x75] + 0x268) != 0)) {
    FUN_141d14db0(param_1 + 0x210);
  }
  pIVar33 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x48);
  pIStackX_20 = pIVar33;
  if (pIVar33 != (IUnknown *)0x0) {
    *(longlong *)(pIVar33 + 0x18) = 0;
    *(longlong *)(pIVar33 + 8) = 0;
    *(longlong *)(pIVar33 + 0x10) = 0;
    *(undefined ***)pIVar33 = &PTR_FUN_143407a28;
    *(longlong *)(pIVar33 + 0x20) = 0;
    uVar11 = FUN_1429e3ef0();
    *(undefined4 *)(pIVar33 + 0x28) = uVar11;
    *(undefined8 *)(pIVar33 + 0x2c) = 0;
    *(longlong *)(pIVar33 + 0x40) = 0;
    pIVar31 = pIVar33;
  }
  pIStack_258 = pIVar31;
  if (pIVar31 != (IUnknown *)0x0) {
    FUN_14040eba0(pIVar31);
  }
  if ((param_1[0x213] - 1U < 999) || (param_1[0x213] == -1)) {
    FUN_142e52ed0(0x447);
  }
  if ((IUnknown **)(param_1 + 0x212) == &pIStack_260) {
    FUN_142e52d50(0x45c,1);
  }
  FUN_141d20270(&pIStack_260);
  FUN_141d22f80(param_1 + 0x212);
  param_1[0x213] = (longlong)pIVar31;
  FUN_141d22f80(&pIStack_260);
  lVar17 = param_1[0x213];
  if (lVar17 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar17 = param_1[0x213];
  }
  if (param_1 == (longlong *)0xfffffffffffffff0) {
    pIStack_228 = (IUnknown *)0x0;
  }
  else {
    pIStack_228 = (IUnknown *)param_1;
    FUN_14040eba0(param_1 + 2);
  }
  pIStackX_20 = (IUnknown *)&pIStack_230;
  if (pIStack_228 == (IUnknown *)0x0) {
    FUN_140f08f00(&pIStack_230);
  }
  else {
    iVar28 = FUN_14045b440(*(undefined4 *)(*(longlong *)((longlong)pIStack_228 + 0x3a8) + 0x60));
    *(int *)(lVar17 + 0x2c) = iVar28;
    if (iVar28 == 0) {
      *(undefined4 *)(lVar17 + 0x20) = 0;
      FUN_140f08f00(&pIStack_230);
    }
    else {
      plVar19 = (longlong *)((longlong)pIStack_228 + 0x30);
      if (pIStack_228 == (IUnknown *)0x0) {
        plVar19 = (longlong *)&DAT_00000020;
      }
      if ((*(longlong *)(lVar17 + 0x40) - 1U < 999) || (*(longlong *)(lVar17 + 0x40) == -1)) {
        FUN_142e52ed0(0x447);
      }
      if ((longlong *)(lVar17 + 0x38) == plVar19) {
        FUN_142e52d50(0x45c,1);
      }
      FUN_141d202f0(plVar19);
      FUN_140f09180((longlong *)(lVar17 + 0x38));
      *(longlong *)(lVar17 + 0x40) = plVar19[1];
      if (pIStack_228 == (IUnknown *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      uVar11 = FUN_141c56cc0(pIStack_228,0);
      *(undefined4 *)(lVar17 + 0x24) = uVar11;
      uVar11 = FUN_1429e3ef0();
      *(undefined4 *)(lVar17 + 0x28) = uVar11;
      *(undefined4 *)(lVar17 + 0x20) = 1;
      FUN_140f08f00(&pIStack_230);
      puVar21 = puStack_190;
    }
  }
  if ((param_1[0x75] != 0) && (cVar8 = FUN_1409175a0(param_1[0x75] + 0x2008), cVar8 == '\0')) {
    FUN_142b4bb10(plVar22,param_1[0x75] + 0x2008,0);
  }
  lVar17 = param_1[0x79];
  if (lVar17 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar17 = param_1[0x79];
  }
  if (0 < *(int *)(lVar17 + 0x578)) {
    FUN_141c89b30(param_1);
  }
  *(undefined4 *)(param_1 + 0x178) = 0;
  FUN_141cbaaf0(param_1);
  if ((*(char *)(param_1[0x75] + 0x81) == '\0') &&
     ((((int)param_1[0x245] != 0 || (*(int *)((longlong)param_1 + 0x122c) != 0)) &&
      (iVar28 = (int)uStackX_8, uStackX_8 != 0)))) {
    *(undefined1 *)(plVar22 + 0x215) = 1;
    *(longlong *)((longlong)plVar22 + 0x10ac) = param_1[0x245];
    *(int *)((longlong)plVar22 + 0x10b4) = iVar28;
    *(undefined1 *)(plStack_198 + 0x215) = 1;
    uVar11 = (undefined4)param_1[0x245];
    uStackX_8 = (short)uVar11;
    sStackX_a = (short)((uint)uVar11 >> 0x10);
    uStackX_c = *(undefined4 *)((longlong)param_1 + 0x122c);
    *(longlong *)((longlong)plStack_198 + 0x10ac) = param_1[0x245];
    *(int *)((longlong)plStack_198 + 0x10b4) = iVar28;
  }
  FUN_141d0d660(param_1,param_1 + 0x103);
  if ((puVar21 != (undefined8 *)0x0) && (iVar28 = FUN_14022eb80(puVar21), iVar28 == 0)) {
    (**(code **)*puVar21)(puVar21,1);
  }
  return;
}


