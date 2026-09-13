
//===========================================================
// FUN_14266f2d0 @ 14266f2d0   (13834 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014266f991) */
/* WARNING: Removing unreachable block (ram,0x000142671423) */

void FUN_14266f2d0(longlong param_1,int param_2,int param_3,longlong *param_4,int param_5,
                  int param_6,longlong param_7,int param_8,int param_9,longlong param_10,
                  int param_11,int param_12,longlong param_13,undefined8 param_14,char param_15)

{
  longlong *plVar1;
  longlong *plVar2;
  bool bVar3;
  undefined *puVar4;
  char cVar5;
  ushort uVar6;
  int iVar7;
  undefined4 uVar8;
  int iVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  longlong lVar12;
  undefined8 *puVar13;
  int *piVar14;
  longlong *plVar15;
  undefined4 *puVar16;
  longlong lVar17;
  longlong *plVar18;
  undefined1 *puVar19;
  undefined2 *puVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  longlong **pplVar23;
  longlong **pplVar24;
  int iVar25;
  uint uVar26;
  uint uVar27;
  uint uVar28;
  longlong **pplVar29;
  char *pcVar30;
  ulonglong uVar31;
  longlong *plVar32;
  longlong lVar33;
  char *pcVar34;
  int iVar35;
  char *pcVar36;
  int iVar37;
  char *pcVar38;
  undefined8 in_stack_fffffffffffffd38;
  char *pcVar39;
  ulonglong uVar40;
  ulonglong in_stack_fffffffffffffd68;
  ulonglong uVar41;
  longlong **local_278;
  longlong *local_270;
  longlong *local_268;
  int local_260;
  int local_25c;
  longlong *local_258;
  undefined4 local_250;
  undefined4 uStack_24c;
  uint local_248;
  longlong **local_240;
  longlong local_238;
  longlong **local_230;
  uint local_228;
  undefined4 uStack_224;
  char *local_220;
  longlong *local_218;
  int local_210;
  undefined4 uStack_20c;
  longlong *local_208;
  char *local_200;
  undefined8 local_1f8;
  longlong local_1f0;
  char *local_1e8;
  longlong *local_1e0;
  char *local_1d8;
  char *local_1d0;
  undefined1 local_1c8 [8];
  undefined8 *local_1c0;
  ulonglong local_1b8;
  int local_1b0;
  undefined4 uStack_1ac;
  longlong *local_1a8;
  undefined8 local_1a0;
  longlong *local_198;
  char *local_190;
  longlong local_188;
  undefined8 local_180;
  char *local_178;
  char *local_170;
  undefined1 *local_168;
  undefined8 local_160;
  undefined1 local_158 [8];
  char *local_150;
  longlong local_148;
  longlong local_140;
  undefined1 local_138 [8];
  char *local_130;
  char *local_128;
  undefined1 local_120 [8];
  undefined1 local_118 [8];
  longlong local_110;
  longlong local_108;
  undefined1 *local_100;
  longlong local_f8;
  undefined1 local_f0 [8];
  undefined1 local_e8 [8];
  undefined1 local_e0 [8];
  longlong local_d8;
  longlong *local_d0;
  undefined1 local_c8 [8];
  longlong local_c0;
  undefined1 local_b8 [8];
  longlong local_b0;
  undefined1 local_a8 [8];
  undefined1 local_a0 [8];
  longlong local_98;
  undefined1 local_90 [8];
  longlong local_88;
  longlong local_80;
  undefined1 local_78 [8];
  longlong *local_70;
  undefined1 local_68 [8];
  longlong *local_60;
  undefined1 local_58 [8];
  longlong local_50;
  
  lVar17 = DAT_143aa8328;
  uVar10 = (undefined4)((ulonglong)in_stack_fffffffffffffd38 >> 0x20);
  local_248 = 0;
  local_238 = DAT_143aa8328;
  if (DAT_143aa8328 == 0) {
    return;
  }
  if (DAT_143aa84a0 == 0) {
    return;
  }
  if (((*(longlong *)(param_1 + 0x48) != 0) && (*(int *)(param_1 + 0x80) == param_2)) &&
     (*(int *)(param_1 + 0x84) == param_3)) {
    return;
  }
  if (param_4 == (longlong *)0x0) {
    return;
  }
  local_190 = (char *)0x0;
  local_1d0 = (char *)0x0;
  local_1e8 = (char *)0x0;
  local_150 = (char *)0x0;
  FUN_1426b7dc0(param_1,param_4,param_5,&local_1d0);
  pcVar34 = local_1d0;
  if ((local_1d0 == (char *)0x0) || (iVar25 = 0x10, *local_1d0 == '\0')) {
    iVar25 = 0;
  }
  local_260 = iVar25;
  if (param_7 != 0) {
    lVar12 = -1;
    do {
      lVar12 = lVar12 + 1;
    } while (*(char *)(param_7 + lVar12) != '\0');
    if (lVar12 != 0) {
      puVar13 = (undefined8 *)FUN_1408a9e40(&local_200,0x3a2);
      FUN_14019ba10(&local_190,*puVar13,param_7);
      if (local_200 != (char *)0x0) {
        FUN_14019f2c0(local_200 + -0x10);
      }
      local_260 = iVar25 + 0x10;
    }
  }
  local_208 = (longlong *)0x0;
  pcVar30 = (char *)0x0;
  if (param_12 != 0) {
    FUN_142d47770(DAT_143aa84a0,local_1c8,param_12);
    if (local_1c0 != (undefined8 *)0x0) {
      FUN_140226d80(local_1c0,&local_208,0);
      iVar25 = 0;
      if (local_208 != (longlong *)0x0) {
        iVar25 = (int)local_208[-1];
      }
      iVar25 = local_260 + iVar25 * 0x10;
      local_260 = iVar25;
      if (local_1c0 != (undefined8 *)0x0) {
        iVar7 = FUN_140226640(local_1c0);
        if (iVar7 == 0) {
          if (local_1c0 == (undefined8 *)0x0) {
            FUN_142e52ed0(0x431,0);
          }
          iVar7 = FUN_140226650(local_1c0);
          if (iVar7 != 0) {
            if (local_1c0 == (undefined8 *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            iVar7 = FUN_1402ef9b0(*(undefined4 *)(local_1c0 + 0x1a));
            if (iVar7 == 0) {
              puVar13 = (undefined8 *)FUN_1408a9e40(&local_270,0xcc2);
              FUN_14019ba10(&local_1e8,*puVar13);
            }
            else {
              puVar13 = (undefined8 *)FUN_1408a9e40(&local_270,0xcc1);
              FUN_14019ba10(&local_1e8,*puVar13);
            }
            if (local_270 != (longlong *)0x0) {
              FUN_14019f2c0(local_270 + -2);
            }
            goto LAB_14266f58a;
          }
          if (local_1c0 == (undefined8 *)0x0) {
            FUN_142e52ed0(0x431,0);
          }
          iVar7 = FUN_1402266a0(local_1c0);
          if (iVar7 != 0) {
            puVar13 = (undefined8 *)FUN_1408a9e40(&local_270,0xcbf);
            FUN_14019ba10(&local_1e8,*puVar13);
            if (local_270 != (longlong *)0x0) {
              FUN_14019f2c0(local_270 + -2);
            }
            goto LAB_14266f58a;
          }
        }
        else {
          puVar13 = (undefined8 *)FUN_1408a9e40(&local_270,0xcc0);
          FUN_14019ba10(&local_1e8,*puVar13);
          if (local_270 != (longlong *)0x0) {
            FUN_14019f2c0(local_270 + -2);
          }
LAB_14266f58a:
          iVar25 = iVar25 + 0x10;
          local_260 = iVar25;
        }
        if (param_11 != 0) {
          if (local_1c0 == (undefined8 *)0x0) {
            FUN_142e52ed0(0x431,0);
          }
          iVar7 = FUN_1402266c0(local_1c0);
          if (iVar7 != 0) {
            puVar13 = (undefined8 *)FUN_1408a9e40(&local_268,0xcc3);
            FUN_14019ba10(&local_150,*puVar13);
            if (local_268 != (longlong *)0x0) {
              FUN_14019f2c0(local_268 + -2);
            }
            local_260 = iVar25 + 0x10;
          }
        }
      }
    }
    puVar13 = local_1c0;
    pcVar30 = local_150;
    if (local_1c0 != (undefined8 *)0x0) {
      if (0xffffe < local_1c0[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar18 = puVar13 + 1;
      lVar12 = *plVar18;
      *plVar18 = *plVar18 + -1;
      UNLOCK();
      pcVar30 = local_150;
      pcVar34 = local_1d0;
      if (((int)lVar12 == 1) && (local_1c0 != (undefined8 *)0x0)) {
        (**(code **)*local_1c0)(local_1c0,1);
        pcVar30 = local_150;
        pcVar34 = local_1d0;
      }
    }
  }
  pplVar23 = (longlong **)0x0;
  local_278 = (longlong **)0x0;
  local_1e0 = param_4 + 4;
  uVar8 = FUN_14019a5d0(local_1e0);
  FUN_140398ba0(lVar17,&local_220,uVar8);
  local_250 = CONCAT22(local_250._2_2_,(ushort)local_250);
  if ((local_220 == (char *)0x0) ||
     (local_250 = CONCAT22(local_250._2_2_,(ushort)local_250), *local_220 == '\0'))
  goto LAB_142672817;
  if ((param_13 == 0) || (iVar25 = (*DAT_1432625e0)(param_13), iVar25 < 1)) {
    if ((param_5 == 0) &&
       (iVar25 = FUN_1402cf680(param_4,0), puVar4 = PTR_s_descD_143a45780, iVar25 != 0)) {
      uVar8 = FUN_14019a5d0(param_4 + 4);
      puVar13 = (undefined8 *)FUN_1403996a0(lVar17,&local_210,uVar8,puVar4);
      local_278 = (longlong **)*puVar13;
      *puVar13 = 0;
      lVar12 = CONCAT44(uStack_20c,local_210);
    }
    else {
      uVar8 = FUN_14019a5d0(param_4 + 4);
      puVar13 = (undefined8 *)FUN_140399280(lVar17,&local_228,uVar8);
      local_278 = (longlong **)*puVar13;
      *puVar13 = 0;
      lVar12 = CONCAT44(uStack_224,local_228);
    }
    if (lVar12 != 0) {
      FUN_14019f2c0(lVar12 + -0x10);
    }
  }
  else {
    uVar31 = 0xffffffffffffffff;
    do {
      uVar31 = uVar31 + 1;
    } while (*(char *)(param_13 + uVar31) != '\0');
    iVar7 = (int)uVar31;
    iVar25 = 0;
    if (0 < iVar7) {
      iVar25 = iVar7;
    }
    piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar25 + 0x11));
    piVar14[1] = iVar25;
    *piVar14 = -1;
    pplVar23 = (longlong **)(piVar14 + 4);
    piVar14[2] = 0;
    *(char *)pplVar23 = '\0';
    local_230 = pplVar23;
    FUN_142ef7ba0(pplVar23,param_13,(longlong)iVar7);
    if (*piVar14 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar7 == -1) || (iVar7 <= piVar14[1])) {
      *piVar14 = 1;
      if (iVar7 != -1) goto LAB_14266f730;
      if (pplVar23 == (longlong **)0x0) {
        uVar31 = 0;
      }
      else {
        uVar31 = 0xffffffffffffffff;
        do {
          uVar31 = uVar31 + 1;
        } while (*(char *)((longlong)pplVar23 + uVar31) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar14[1],uVar31 & 0xffffffff);
      *piVar14 = 1;
LAB_14266f730:
      *(char *)((longlong)iVar7 + (longlong)pplVar23) = '\0';
    }
    iVar25 = (int)uVar31;
    if ((iVar25 < 0) || (piVar14[1] + 1 <= iVar25)) {
      FUN_142e54290(0x9c,uVar31 & 0xffffffff);
    }
    piVar14[2] = iVar25;
    local_278 = pplVar23;
  }
  plVar18 = local_1e0;
  uVar8 = FUN_14019a5d0(local_1e0);
  FUN_1426d2c60(uVar8,&local_278);
  uVar8 = FUN_14019a5d0(plVar18);
  local_1b8 = 0;
  plVar15 = (longlong *)FUN_14019ba10(&local_1b8,"( %d )",uVar8);
  lVar12 = *plVar15;
  if (lVar12 == 0) {
    uVar8 = 0;
  }
  else {
    uVar8 = *(undefined4 *)(lVar12 + -8);
  }
  FUN_1401abc80(&local_220,&local_1b0,lVar12,uVar8);
  local_248 = 0x10;
  if (local_1b8 != 0) {
    FUN_14019f2c0(local_1b8 - 0x10);
  }
  lVar12 = DAT_143aa84a0;
  local_240 = (longlong **)&local_1f8;
  local_1f8 = (longlong *)0x0;
  plVar2 = (longlong *)CONCAT44(uStack_1ac,local_1b0);
  plVar32 = plVar2;
  plVar15 = local_1f8;
  if ((plVar2 != (longlong *)0x0) && (plVar1 = plVar2 + -2, plVar1 != (longlong *)0x0)) {
    if ((int)*plVar1 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar31 = 0xffffffffffffffff;
      do {
        uVar31 = uVar31 + 1;
      } while (*(char *)((longlong)plVar2 + uVar31) != '\0');
      iVar7 = (int)uVar31;
      iVar25 = 0;
      if (0 < iVar7) {
        iVar25 = iVar7;
      }
      piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar25 + 0x11));
      piVar14[1] = iVar25;
      *piVar14 = -1;
      plVar15 = (longlong *)(piVar14 + 4);
      piVar14[2] = 0;
      *(char *)plVar15 = '\0';
      local_268 = plVar15;
      FUN_142ef7ba0(plVar15,plVar2,(longlong)iVar7);
      if (*piVar14 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar14[1])) {
        *piVar14 = 1;
        if (iVar7 != -1) goto LAB_14266f950;
        if (plVar15 == (longlong *)0x0) {
          uVar31 = 0;
        }
        else {
          uVar31 = 0xffffffffffffffff;
          do {
            uVar31 = uVar31 + 1;
          } while (*(char *)((longlong)plVar15 + uVar31) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar14[1],uVar31 & 0xffffffff);
        *piVar14 = 1;
LAB_14266f950:
        *(char *)((longlong)plVar15 + (longlong)iVar7) = '\0';
      }
      iVar25 = (int)uVar31;
      if ((iVar25 < 0) || (piVar14[1] + 1 <= iVar25)) {
        FUN_142e54290(0x9c,uVar31 & 0xffffffff);
      }
      piVar14[2] = iVar25;
      plVar18 = local_1e0;
      if (local_1f8 != (longlong *)0x0) {
        FUN_14019f2c0();
        plVar18 = local_1e0;
      }
    }
    else {
      if ((int)*plVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *(int *)plVar1 = (int)*plVar1 + 1;
      UNLOCK();
      if (local_1f8 != (longlong *)0x0) {
        FUN_14019f2c0(local_1f8 + -2);
      }
      plVar32 = (longlong *)CONCAT44(uStack_1ac,local_1b0);
      plVar15 = plVar2;
    }
  }
  local_1f8 = plVar15;
  local_1a0 = 0;
  FUN_14019a260(&local_1a0,&local_220);
  puVar13 = (undefined8 *)FUN_142cf47f0(lVar12,&local_250,&local_1a0,&local_1f8);
  if (local_220 != (char *)0x0) {
    FUN_14019f2c0(local_220 + -0x10);
  }
  local_220 = (char *)*puVar13;
  *puVar13 = 0;
  lVar12 = CONCAT44(uStack_24c,CONCAT22(local_250._2_2_,(ushort)local_250));
  if (lVar12 != 0) {
    FUN_14019f2c0(lVar12 + -0x10);
  }
  if (plVar32 != (longlong *)0x0) {
    FUN_14019f2c0(plVar32 + -2);
  }
  local_1b0 = 0x74;
  if (param_11 == 0) {
LAB_142670514:
    iVar25 = (**(code **)(*param_4 + 0x38))();
    pplVar23 = local_278;
    if (iVar25 != 0) {
      uVar8 = FUN_14019a5d0(plVar18);
      iVar25 = FUN_14038ac80(lVar17,uVar8);
      pplVar23 = local_278;
      if (iVar25 == 0) {
        if ((local_278 != (longlong **)0x0) && (*(char *)local_278 != '\0')) {
          lVar17 = -1;
          do {
            lVar12 = lVar17 + 1;
            pcVar34 = &DAT_1434b7dfd + lVar17;
            lVar17 = lVar12;
          } while (*pcVar34 != '\0');
          iVar25 = (int)lVar12;
          if (iVar25 != 0) {
            iVar7 = *(int *)(local_278 + -1);
            for (iVar9 = *(int *)((longlong)local_278 + -0xc); iVar9 < iVar7 + iVar25;
                iVar9 = iVar9 * 2) {
            }
            lVar17 = FUN_14019bd40(&local_278,iVar9,1);
            pplVar23 = local_278;
            if (local_278 == (longlong **)0x0) {
              iVar9 = 0;
            }
            else {
              iVar9 = *(int *)(local_278 + -1);
            }
            FUN_142ef7ba0(iVar9 + lVar17,&DAT_1434b7dfc,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar7 + iVar25);
          }
        }
        plVar18 = (longlong *)FUN_1408a9e40(&local_200,0xcb0);
        lVar17 = *plVar18;
        if (lVar17 != 0) {
          iVar25 = *(int *)(lVar17 + -8);
          if (iVar25 != 0) {
            if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
              uVar22 = FUN_14019bd40(&local_278,iVar25,0);
              FUN_142ef7ba0(uVar22,lVar17,(longlong)iVar25);
              FUN_14019c870(&local_278,iVar25);
              pplVar23 = local_278;
            }
            else {
              iVar7 = *(int *)(pplVar23 + -1);
              for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                  iVar9 = iVar9 * 2) {
              }
              lVar12 = FUN_14019bd40(&local_278,iVar9,1);
              pplVar23 = local_278;
              if (local_278 == (longlong **)0x0) {
                iVar9 = 0;
              }
              else {
                iVar9 = *(int *)(local_278 + -1);
              }
              FUN_142ef7ba0(iVar9 + lVar12,lVar17,(longlong)iVar25);
              FUN_14019c870(&local_278,iVar7 + iVar25);
            }
          }
        }
        if (local_200 != (char *)0x0) {
          FUN_14019f2c0(local_200 + -0x10);
        }
      }
    }
  }
  else {
    uVar8 = FUN_14019a5d0(plVar18);
    iVar25 = FUN_1404179d0(uVar8);
    if (((iVar25 == 0) || (iVar25 = FUN_140286f80(param_12), iVar25 == 0)) ||
       (iVar25 = FUN_1426c6df0(param_12), pplVar29 = local_278, iVar25 == 0)) goto LAB_142670514;
    pplVar23 = local_278;
    if ((local_278 != (longlong **)0x0) && (*(char *)local_278 != '\0')) {
      lVar12 = -1;
      do {
        lVar33 = lVar12 + 1;
        pcVar34 = &DAT_1434b7dfd + lVar12;
        lVar12 = lVar33;
      } while (*pcVar34 != '\0');
      iVar25 = (int)lVar33;
      if (iVar25 != 0) {
        iVar7 = *(int *)(local_278 + -1);
        for (iVar9 = *(int *)((longlong)local_278 + -0xc); iVar9 < iVar7 + iVar25; iVar9 = iVar9 * 2
            ) {
        }
        pplVar24 = local_278 + -2;
        if (pplVar24 == (longlong **)0x0) {
          iVar37 = 0;
LAB_14266fb40:
          if (iVar37 < iVar9) {
            iVar37 = iVar9;
          }
          puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar37 + 0x11));
          puVar16[1] = iVar37;
          *puVar16 = 0xffffffff;
          pplVar23 = (longlong **)(puVar16 + 4);
          local_278 = pplVar23;
          if (pplVar24 == (longlong **)0x0) {
            puVar16[2] = 0;
            *(char *)pplVar23 = '\0';
            lVar17 = local_238;
          }
          else {
            iVar9 = *(int *)(pplVar29 + -1) + 1;
            iVar35 = iVar37 + 1;
            if (iVar35 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar35);
              iVar9 = iVar35;
            }
            FUN_142ef7ba0(pplVar23,pplVar29,(longlong)iVar9);
            puVar16[2] = *(int *)(pplVar29 + -1);
            *(char *)((longlong)iVar37 + (longlong)pplVar23) = '\0';
            FUN_14019f2c0(pplVar24);
            lVar17 = local_238;
          }
        }
        else {
          if ((1 < *(int *)pplVar24) || (*(int *)((longlong)local_278 + -0xc) < iVar9)) {
            iVar37 = *(int *)(local_278 + -1);
            goto LAB_14266fb40;
          }
          if (*(int *)pplVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *(int *)pplVar24 = -1;
          pplVar23 = pplVar29;
        }
        if (pplVar23 == (longlong **)0x0) {
          iVar9 = 0;
        }
        else {
          iVar9 = *(int *)(pplVar23 + -1);
        }
        FUN_142ef7ba0((char *)((longlong)iVar9 + (longlong)pplVar23),&DAT_1434b7dfc,(longlong)iVar25
                     );
        FUN_14019c870(&local_278,iVar7 + iVar25);
        plVar18 = local_1e0;
      }
    }
    uVar8 = FUN_14019a5d0(plVar18);
    iVar25 = FUN_14038b870(lVar17,uVar8);
    if (iVar25 != 0) {
      puVar13 = (undefined8 *)FUN_1408a9e40(&local_200,0xcb9);
      plVar15 = (longlong *)*puVar13;
      local_268 = plVar15;
      if (plVar15 != (longlong *)0x0) {
        iVar25 = (int)plVar15[-1];
        uVar31 = (ulonglong)iVar25;
        if (iVar25 != 0) {
          if (pplVar23 == (longlong **)0x0) {
LAB_14266fd96:
            pplVar29 = (longlong **)0x0;
LAB_14266fd98:
            iVar7 = 0;
LAB_14266fd9a:
            if (iVar7 < iVar25) {
              iVar7 = iVar25;
            }
            puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
            puVar16[1] = iVar7;
            *puVar16 = 0xffffffff;
            pplVar23 = (longlong **)(puVar16 + 4);
            puVar16[2] = 0;
            *(char *)pplVar23 = '\0';
            local_278 = pplVar23;
            if (pplVar29 != (longlong **)0x0) {
              FUN_14019f2c0(pplVar29);
            }
          }
          else {
            if (*(char *)pplVar23 != '\0') {
              iVar7 = *(int *)(pplVar23 + -1);
              for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                  iVar9 = iVar9 * 2) {
              }
              pplVar29 = pplVar23 + -2;
              if (pplVar29 == (longlong **)0x0) {
                iVar37 = 0;
LAB_14266fca2:
                if (iVar37 < iVar9) {
                  iVar37 = iVar9;
                }
                puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar37 + 0x11));
                puVar16[1] = iVar37;
                *puVar16 = 0xffffffff;
                pplVar24 = (longlong **)(puVar16 + 4);
                local_278 = pplVar24;
                if (pplVar29 == (longlong **)0x0) {
                  puVar16[2] = 0;
                  *(char *)pplVar24 = '\0';
                  lVar17 = local_238;
                  plVar15 = local_268;
                }
                else {
                  iVar9 = *(int *)(pplVar23 + -1) + 1;
                  iVar35 = iVar37 + 1;
                  if (iVar35 < iVar9) {
                    FUN_142e54290(0x5c,iVar9,iVar35);
                    iVar9 = iVar35;
                  }
                  FUN_142ef7ba0(pplVar24,pplVar23,(longlong)iVar9);
                  puVar16[2] = *(int *)(pplVar23 + -1);
                  *(char *)((longlong)iVar37 + (longlong)pplVar24) = '\0';
                  FUN_14019f2c0(pplVar29);
                  lVar17 = local_238;
                  plVar15 = local_268;
                }
              }
              else {
                if ((1 < *(int *)pplVar29) || (*(int *)((longlong)pplVar23 + -0xc) < iVar9)) {
                  iVar37 = *(int *)(pplVar23 + -1);
                  goto LAB_14266fca2;
                }
                if (*(int *)pplVar29 != 1) {
                  FUN_142e52dd0(0x74);
                }
                *(int *)pplVar29 = -1;
                pplVar24 = pplVar23;
              }
              if (pplVar24 == (longlong **)0x0) {
                iVar9 = 0;
              }
              else {
                iVar9 = *(int *)(pplVar24 + -1);
              }
              FUN_142ef7ba0((char *)((longlong)iVar9 + (longlong)pplVar24),plVar15,uVar31);
              FUN_14019c870(&local_278,iVar7 + iVar25);
              pplVar23 = pplVar24;
              plVar18 = local_1e0;
              goto LAB_14266fe49;
            }
            if (pplVar23 == (longlong **)0x0) goto LAB_14266fd96;
            pplVar29 = pplVar23 + -2;
            if (pplVar29 == (longlong **)0x0) goto LAB_14266fd98;
            if ((1 < *(int *)pplVar29) || (*(int *)((longlong)pplVar23 + -0xc) < iVar25)) {
              iVar7 = *(int *)(pplVar23 + -1);
              goto LAB_14266fd9a;
            }
            if (*(int *)pplVar29 != 1) {
              FUN_142e52dd0(0x74);
            }
            *(int *)pplVar29 = -1;
          }
          FUN_142ef7ba0(pplVar23,plVar15,uVar31);
          if (*(int *)(pplVar23 + -2) != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar25 == -1) || (iVar25 <= *(int *)((longlong)pplVar23 + -0xc))) {
            *(int *)(pplVar23 + -2) = 1;
            if (iVar25 != -1) goto LAB_14266fe22;
            if (pplVar23 == (longlong **)0x0) {
              uVar31 = 0;
            }
            else {
              uVar31 = 0xffffffffffffffff;
              do {
                uVar31 = uVar31 + 1;
              } while (*(char *)((longlong)pplVar23 + uVar31) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,*(int *)((longlong)pplVar23 + -0xc),iVar25);
            *(int *)(pplVar23 + -2) = 1;
LAB_14266fe22:
            *(char *)(uVar31 + (longlong)pplVar23) = '\0';
          }
          iVar25 = (int)uVar31;
          if ((iVar25 < 0) || (*(int *)((longlong)pplVar23 + -0xc) + 1 <= iVar25)) {
            FUN_142e54290(0x9c,uVar31 & 0xffffffff);
          }
          *(int *)(pplVar23 + -1) = iVar25;
          plVar18 = local_1e0;
        }
      }
LAB_14266fe49:
      if (local_200 != (char *)0x0) {
        FUN_14019f2c0(local_200 + -0x10);
      }
    }
    cVar5 = FUN_140841850();
    if (cVar5 == '\0') {
      uVar8 = FUN_14019a5d0(plVar18);
      iVar25 = FUN_14038ac80(lVar17,uVar8);
      if ((iVar25 == 0) && ((param_15 == '\0' || (iVar25 = FUN_1426c6e90(param_12), iVar25 != 0))))
      {
        iVar25 = FUN_1426c6f30(param_12);
        if (iVar25 == 0) {
          plVar18 = (longlong *)FUN_1408a9e40(&local_270,0xcb4);
          lVar17 = *plVar18;
          if (lVar17 != 0) {
            iVar25 = *(int *)(lVar17 + -8);
            if (iVar25 != 0) {
              if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
                uVar22 = FUN_14019bd40(&local_278,iVar25,0);
                FUN_142ef7ba0(uVar22,lVar17,(longlong)iVar25);
                FUN_14019c870(&local_278,iVar25);
                pplVar23 = local_278;
              }
              else {
                iVar7 = *(int *)(pplVar23 + -1);
                for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                    iVar9 = iVar9 * 2) {
                }
                lVar12 = FUN_14019bd40(&local_278,iVar9,1);
                pplVar23 = local_278;
                if (local_278 == (longlong **)0x0) {
                  iVar9 = 0;
                }
                else {
                  iVar9 = *(int *)(local_278 + -1);
                }
                FUN_142ef7ba0(iVar9 + lVar12,lVar17,(longlong)iVar25);
                FUN_14019c870(&local_278,iVar7 + iVar25);
              }
            }
          }
        }
        else {
          plVar18 = (longlong *)FUN_1408a9e40(&local_270,0xcb5);
          lVar17 = *plVar18;
          if (lVar17 != 0) {
            iVar25 = *(int *)(lVar17 + -8);
            if (iVar25 != 0) {
              if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
                uVar22 = FUN_14019bd40(&local_278,iVar25,0);
                FUN_142ef7ba0(uVar22,lVar17,(longlong)iVar25);
                FUN_14019c870(&local_278,iVar25);
                pplVar23 = local_278;
              }
              else {
                iVar7 = *(int *)(pplVar23 + -1);
                for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                    iVar9 = iVar9 * 2) {
                }
                lVar12 = FUN_14019bd40(&local_278,iVar9,1);
                pplVar23 = local_278;
                if (local_278 == (longlong **)0x0) {
                  iVar9 = 0;
                }
                else {
                  iVar9 = *(int *)(local_278 + -1);
                }
                FUN_142ef7ba0(iVar9 + lVar12,lVar17,(longlong)iVar25);
                FUN_14019c870(&local_278,iVar7 + iVar25);
              }
            }
          }
        }
      }
      else {
        puVar13 = (undefined8 *)FUN_1408a9e40(&local_270,0xcb7);
        plVar18 = (longlong *)*puVar13;
        local_268 = plVar18;
        if (plVar18 != (longlong *)0x0) {
          iVar25 = (int)plVar18[-1];
          uVar31 = (ulonglong)iVar25;
          if (iVar25 != 0) {
            if (pplVar23 == (longlong **)0x0) {
LAB_14267041f:
              pplVar29 = (longlong **)0x0;
LAB_142670421:
              iVar7 = 0;
LAB_142670423:
              if (iVar7 < iVar25) {
                iVar7 = iVar25;
              }
              puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
              puVar16[1] = iVar7;
              *puVar16 = 0xffffffff;
              pplVar23 = (longlong **)(puVar16 + 4);
              puVar16[2] = 0;
              *(char *)pplVar23 = '\0';
              local_278 = pplVar23;
              if (pplVar29 != (longlong **)0x0) {
                FUN_14019f2c0(pplVar29);
              }
            }
            else {
              if (*(char *)pplVar23 != '\0') {
                iVar7 = *(int *)(pplVar23 + -1);
                for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                    iVar9 = iVar9 * 2) {
                }
                pplVar29 = pplVar23 + -2;
                if (pplVar29 == (longlong **)0x0) {
                  iVar37 = 0;
LAB_14267032f:
                  if (iVar37 < iVar9) {
                    iVar37 = iVar9;
                  }
                  puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar37 + 0x11));
                  puVar16[1] = iVar37;
                  *puVar16 = 0xffffffff;
                  pplVar24 = (longlong **)(puVar16 + 4);
                  local_278 = pplVar24;
                  if (pplVar29 == (longlong **)0x0) {
                    puVar16[2] = 0;
                    *(char *)pplVar24 = '\0';
                    plVar18 = local_268;
                  }
                  else {
                    iVar9 = *(int *)(pplVar23 + -1) + 1;
                    iVar35 = iVar37 + 1;
                    if (iVar35 < iVar9) {
                      FUN_142e54290(0x5c,iVar9,iVar35);
                      iVar9 = iVar35;
                    }
                    FUN_142ef7ba0(pplVar24,pplVar23,(longlong)iVar9);
                    puVar16[2] = *(int *)(pplVar23 + -1);
                    *(char *)((longlong)iVar37 + (longlong)pplVar24) = '\0';
                    FUN_14019f2c0(pplVar29);
                    plVar18 = local_268;
                  }
                }
                else {
                  if ((1 < *(int *)pplVar29) || (*(int *)((longlong)pplVar23 + -0xc) < iVar9)) {
                    iVar37 = *(int *)(pplVar23 + -1);
                    goto LAB_14267032f;
                  }
                  if (*(int *)pplVar29 != 1) {
                    FUN_142e52dd0(0x74);
                  }
                  *(int *)pplVar29 = -1;
                  pplVar24 = pplVar23;
                }
                if (pplVar24 == (longlong **)0x0) {
                  iVar9 = 0;
                }
                else {
                  iVar9 = *(int *)(pplVar24 + -1);
                }
                FUN_142ef7ba0((char *)((longlong)iVar9 + (longlong)pplVar24),plVar18,uVar31);
                FUN_14019c870(&local_278,iVar7 + iVar25);
                pplVar23 = pplVar24;
                goto LAB_1426704c6;
              }
              if (pplVar23 == (longlong **)0x0) goto LAB_14267041f;
              pplVar29 = pplVar23 + -2;
              if (pplVar29 == (longlong **)0x0) goto LAB_142670421;
              if ((1 < *(int *)pplVar29) || (*(int *)((longlong)pplVar23 + -0xc) < iVar25)) {
                iVar7 = *(int *)(pplVar23 + -1);
                goto LAB_142670423;
              }
              if (*(int *)pplVar29 != 1) {
                FUN_142e52dd0(0x74);
              }
              *(int *)pplVar29 = -1;
            }
            FUN_142ef7ba0(pplVar23,plVar18,uVar31);
            if (*(int *)(pplVar23 + -2) != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar25 == -1) || (iVar25 <= *(int *)((longlong)pplVar23 + -0xc))) {
              *(int *)(pplVar23 + -2) = 1;
              if (iVar25 != -1) goto LAB_1426704a3;
              if (pplVar23 == (longlong **)0x0) {
                uVar31 = 0;
              }
              else {
                uVar31 = 0xffffffffffffffff;
                do {
                  uVar31 = uVar31 + 1;
                } while (*(char *)((longlong)pplVar23 + uVar31) != '\0');
              }
            }
            else {
              FUN_142e54290(0x90,*(int *)((longlong)pplVar23 + -0xc),iVar25);
              *(int *)(pplVar23 + -2) = 1;
LAB_1426704a3:
              *(char *)(uVar31 + (longlong)pplVar23) = '\0';
            }
            iVar25 = (int)uVar31;
            if ((iVar25 < 0) || (*(int *)((longlong)pplVar23 + -0xc) + 1 <= iVar25)) {
              FUN_142e54290(0x9c,uVar31 & 0xffffffff);
            }
            *(int *)(pplVar23 + -1) = iVar25;
          }
        }
      }
    }
    else {
      puVar13 = (undefined8 *)FUN_1408a9e40(&local_270,0xcb8);
      plVar18 = (longlong *)*puVar13;
      local_268 = plVar18;
      if (plVar18 != (longlong *)0x0) {
        iVar25 = (int)plVar18[-1];
        uVar31 = (ulonglong)iVar25;
        if (iVar25 != 0) {
          if (pplVar23 == (longlong **)0x0) {
LAB_14267001c:
            pplVar29 = (longlong **)0x0;
LAB_14267001e:
            iVar7 = 0;
LAB_142670020:
            if (iVar7 < iVar25) {
              iVar7 = iVar25;
            }
            puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
            puVar16[1] = iVar7;
            *puVar16 = 0xffffffff;
            pplVar23 = (longlong **)(puVar16 + 4);
            puVar16[2] = 0;
            *(char *)pplVar23 = '\0';
            local_278 = pplVar23;
            if (pplVar29 != (longlong **)0x0) {
              FUN_14019f2c0(pplVar29);
            }
          }
          else {
            if (*(char *)pplVar23 != '\0') {
              iVar7 = *(int *)(pplVar23 + -1);
              for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                  iVar9 = iVar9 * 2) {
              }
              pplVar29 = pplVar23 + -2;
              if (pplVar29 == (longlong **)0x0) {
                iVar37 = 0;
LAB_14266ff2e:
                if (iVar37 < iVar9) {
                  iVar37 = iVar9;
                }
                puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar37 + 0x11));
                puVar16[1] = iVar37;
                *puVar16 = 0xffffffff;
                pplVar24 = (longlong **)(puVar16 + 4);
                local_278 = pplVar24;
                if (pplVar29 == (longlong **)0x0) {
                  puVar16[2] = 0;
                  *(char *)pplVar24 = '\0';
                  pplVar23 = pplVar24;
                  plVar18 = local_268;
                }
                else {
                  iVar9 = *(int *)(pplVar23 + -1) + 1;
                  iVar35 = iVar37 + 1;
                  if (iVar35 < iVar9) {
                    FUN_142e54290(0x5c,iVar9,iVar35);
                    iVar9 = iVar35;
                  }
                  FUN_142ef7ba0(pplVar24,pplVar23,(longlong)iVar9);
                  puVar16[2] = *(int *)(pplVar23 + -1);
                  *(char *)((longlong)iVar37 + (longlong)pplVar24) = '\0';
                  FUN_14019f2c0(pplVar29);
                  pplVar23 = pplVar24;
                  plVar18 = local_268;
                }
              }
              else {
                if ((1 < *(int *)pplVar29) || (*(int *)((longlong)pplVar23 + -0xc) < iVar9)) {
                  iVar37 = *(int *)(pplVar23 + -1);
                  goto LAB_14266ff2e;
                }
                if (*(int *)pplVar29 != 1) {
                  FUN_142e52dd0(0x74);
                }
                *(int *)pplVar29 = -1;
              }
              if (pplVar23 == (longlong **)0x0) {
                iVar9 = 0;
              }
              else {
                iVar9 = *(int *)(pplVar23 + -1);
              }
              FUN_142ef7ba0((char *)((longlong)iVar9 + (longlong)pplVar23),plVar18,uVar31);
              FUN_14019c870(&local_278,iVar7 + iVar25);
              goto LAB_1426704c6;
            }
            if (pplVar23 == (longlong **)0x0) goto LAB_14267001c;
            pplVar29 = pplVar23 + -2;
            if (pplVar29 == (longlong **)0x0) goto LAB_14267001e;
            if ((1 < *(int *)pplVar29) || (*(int *)((longlong)pplVar23 + -0xc) < iVar25)) {
              iVar7 = *(int *)(pplVar23 + -1);
              goto LAB_142670020;
            }
            if (*(int *)pplVar29 != 1) {
              FUN_142e52dd0(0x74);
            }
            *(int *)pplVar29 = -1;
          }
          FUN_142ef7ba0(pplVar23,plVar18,uVar31);
          if (*(int *)(pplVar23 + -2) != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar25 == -1) || (iVar25 <= *(int *)((longlong)pplVar23 + -0xc))) {
            *(int *)(pplVar23 + -2) = 1;
            if (iVar25 != -1) goto LAB_1426700a0;
            if (pplVar23 == (longlong **)0x0) {
              uVar31 = 0;
            }
            else {
              uVar31 = 0xffffffffffffffff;
              do {
                uVar31 = uVar31 + 1;
              } while (*(char *)((longlong)pplVar23 + uVar31) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,*(int *)((longlong)pplVar23 + -0xc),iVar25);
            *(int *)(pplVar23 + -2) = 1;
LAB_1426700a0:
            *(char *)(uVar31 + (longlong)pplVar23) = '\0';
          }
          iVar25 = (int)uVar31;
          if ((iVar25 < 0) || (*(int *)((longlong)pplVar23 + -0xc) + 1 <= iVar25)) {
            FUN_142e54290(0x9c,uVar31 & 0xffffffff);
          }
          *(int *)(pplVar23 + -1) = iVar25;
        }
      }
    }
LAB_1426704c6:
    if (local_270 != (longlong *)0x0) {
      FUN_14019f2c0(local_270 + -2);
    }
  }
  if (param_12 == 0) {
    iVar25 = (**(code **)(*param_4 + 0x48))(param_4);
    if (iVar25 == 0) {
      iVar25 = (**(code **)(*param_4 + 0x40))(param_4);
      if (iVar25 == 0) goto LAB_142670b38;
      if ((pplVar23 != (longlong **)0x0) && (*(char *)pplVar23 != '\0')) {
        lVar17 = -1;
        do {
          lVar12 = lVar17 + 1;
          pcVar34 = &DAT_1434b7dfd + lVar17;
          lVar17 = lVar12;
        } while (*pcVar34 != '\0');
        iVar25 = (int)lVar12;
        if (iVar25 != 0) {
          iVar7 = *(int *)(pplVar23 + -1);
          for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
              iVar9 = iVar9 * 2) {
          }
          lVar17 = FUN_14019bd40(&local_278,iVar9,1);
          pplVar23 = local_278;
          if (local_278 == (longlong **)0x0) {
            iVar9 = 0;
          }
          else {
            iVar9 = *(int *)(local_278 + -1);
          }
          FUN_142ef7ba0(iVar9 + lVar17,&DAT_1434b7dfc,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar7 + iVar25);
        }
      }
      plVar18 = (longlong *)FUN_1408a9e40(&local_1f0,0xca0);
      lVar17 = *plVar18;
      if (lVar17 != 0) {
        iVar25 = *(int *)(lVar17 + -8);
        if (iVar25 != 0) {
          if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
            uVar22 = FUN_14019bd40(&local_278,iVar25,0);
            FUN_142ef7ba0(uVar22,lVar17,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar25);
            pplVar23 = local_278;
          }
          else {
            iVar7 = *(int *)(pplVar23 + -1);
            for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                iVar9 = iVar9 * 2) {
            }
            lVar12 = FUN_14019bd40(&local_278,iVar9,1);
            pplVar23 = local_278;
            if (local_278 == (longlong **)0x0) {
              iVar9 = 0;
            }
            else {
              iVar9 = *(int *)(local_278 + -1);
            }
            FUN_142ef7ba0(iVar9 + lVar12,lVar17,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar7 + iVar25);
          }
        }
      }
    }
    else {
      if ((pplVar23 != (longlong **)0x0) && (*(char *)pplVar23 != '\0')) {
        lVar17 = -1;
        do {
          lVar12 = lVar17 + 1;
          pcVar34 = &DAT_1434b7dfd + lVar17;
          lVar17 = lVar12;
        } while (*pcVar34 != '\0');
        iVar25 = (int)lVar12;
        if (iVar25 != 0) {
          iVar7 = *(int *)(pplVar23 + -1);
          for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
              iVar9 = iVar9 * 2) {
          }
          lVar17 = FUN_14019bd40(&local_278,iVar9,1);
          pplVar23 = local_278;
          if (local_278 == (longlong **)0x0) {
            iVar9 = 0;
          }
          else {
            iVar9 = *(int *)(local_278 + -1);
          }
          FUN_142ef7ba0(iVar9 + lVar17,&DAT_1434b7dfc,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar7 + iVar25);
        }
      }
      plVar18 = (longlong *)FUN_1408a9e40(&local_1f0,0xca1);
      lVar17 = *plVar18;
      if (lVar17 != 0) {
        iVar25 = *(int *)(lVar17 + -8);
        if (iVar25 != 0) {
          if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
            uVar22 = FUN_14019bd40(&local_278,iVar25,0);
            FUN_142ef7ba0(uVar22,lVar17,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar25);
            pplVar23 = local_278;
          }
          else {
            iVar7 = *(int *)(pplVar23 + -1);
            for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                iVar9 = iVar9 * 2) {
            }
            lVar12 = FUN_14019bd40(&local_278,iVar9,1);
            pplVar23 = local_278;
            if (local_278 == (longlong **)0x0) {
              iVar9 = 0;
            }
            else {
              iVar9 = *(int *)(local_278 + -1);
            }
            FUN_142ef7ba0(iVar9 + lVar12,lVar17,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar7 + iVar25);
          }
        }
      }
    }
    if (local_1f0 != 0) {
      FUN_14019f2c0(local_1f0 + -0x10);
    }
  }
  else {
    FUN_142d47770(DAT_143aa84a0,local_58,param_12);
    if (local_50 != 0) {
      plVar18 = (longlong *)0x0;
      iVar25 = 0;
      local_268 = (longlong *)0x0;
      if (*(int *)(local_50 + 0x6c) == 0) {
        if (*(int *)(local_50 + 0x104) != 0) {
          puVar13 = (undefined8 *)FUN_1408a9e40(&local_1f0,0xca0);
          plVar18 = (longlong *)*puVar13;
          *puVar13 = 0;
          local_268 = plVar18;
          if (local_1f0 != 0) {
            FUN_14019f2c0(local_1f0 + -0x10);
          }
          goto LAB_142670741;
        }
      }
      else {
        puVar13 = (undefined8 *)FUN_1408a9e40(&local_1f0,0xca1);
        plVar18 = (longlong *)*puVar13;
        *puVar13 = 0;
        local_268 = plVar18;
        if (local_1f0 != 0) {
          FUN_14019f2c0(local_1f0 + -0x10);
        }
LAB_142670741:
        if ((plVar18 != (longlong *)0x0) && ((char)*plVar18 != '\0')) {
          if ((pplVar23 != (longlong **)0x0) && (*(char *)pplVar23 != '\0')) {
            lVar17 = -1;
            do {
              lVar12 = lVar17 + 1;
              pcVar34 = &DAT_1434b7dfd + lVar17;
              lVar17 = lVar12;
            } while (*pcVar34 != '\0');
            iVar7 = (int)lVar12;
            if (iVar7 != 0) {
              iVar9 = *(int *)(pplVar23 + -1);
              for (iVar37 = *(int *)((longlong)pplVar23 + -0xc); iVar37 < iVar9 + iVar7;
                  iVar37 = iVar37 * 2) {
              }
              lVar17 = FUN_14019bd40(&local_278,iVar37,1);
              pplVar23 = local_278;
              iVar37 = iVar25;
              if (local_278 != (longlong **)0x0) {
                iVar37 = *(int *)(local_278 + -1);
              }
              FUN_142ef7ba0(iVar37 + lVar17,&DAT_1434b7dfc,(longlong)iVar7);
              FUN_14019c870(&local_278,iVar9 + iVar7);
            }
          }
          iVar7 = (int)plVar18[-1];
          if (iVar7 != 0) {
            if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
              uVar22 = FUN_14019bd40(&local_278,iVar7,0);
              FUN_142ef7ba0(uVar22,plVar18,(longlong)iVar7);
              FUN_14019c870(&local_278,iVar7);
              pplVar23 = local_278;
            }
            else {
              iVar9 = *(int *)(pplVar23 + -1);
              for (iVar37 = *(int *)((longlong)pplVar23 + -0xc); iVar37 < iVar9 + iVar7;
                  iVar37 = iVar37 * 2) {
              }
              lVar17 = FUN_14019bd40(&local_278,iVar37,1);
              pplVar23 = local_278;
              if (local_278 != (longlong **)0x0) {
                iVar25 = *(int *)(local_278 + -1);
              }
              FUN_142ef7ba0(iVar25 + lVar17,plVar18,(longlong)iVar7);
              FUN_14019c870(&local_278,iVar9 + iVar7);
            }
          }
        }
      }
      if (plVar18 != (longlong *)0x0) {
        FUN_14019f2c0(plVar18 + -2);
      }
    }
    FUN_1401d1a30(local_58);
  }
LAB_142670b38:
  lVar17 = DAT_143aa8328;
  uVar8 = FUN_14019a5d0(param_4 + 4);
  iVar25 = FUN_14038a040(lVar17,uVar8);
  if (iVar25 == 0) {
    if ((pplVar23 != (longlong **)0x0) && (*(char *)pplVar23 != '\0')) {
      lVar17 = -1;
      do {
        lVar12 = lVar17 + 1;
        pcVar34 = &DAT_1434b7dfd + lVar17;
        lVar17 = lVar12;
      } while (*pcVar34 != '\0');
      iVar25 = (int)lVar12;
      if (iVar25 != 0) {
        iVar7 = *(int *)(pplVar23 + -1);
        for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25; iVar9 = iVar9 * 2)
        {
        }
        lVar17 = FUN_14019bd40(&local_278,iVar9,1);
        pplVar23 = local_278;
        if (local_278 == (longlong **)0x0) {
          iVar9 = 0;
        }
        else {
          iVar9 = *(int *)(local_278 + -1);
        }
        FUN_142ef7ba0(iVar9 + lVar17,&DAT_1434b7dfc,(longlong)iVar25);
        FUN_14019c870(&local_278,iVar7 + iVar25);
      }
    }
    plVar18 = (longlong *)FUN_1408a9e40(&local_188,0xc98);
    lVar17 = *plVar18;
    if (lVar17 != 0) {
      iVar25 = *(int *)(lVar17 + -8);
      if (iVar25 != 0) {
        if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
          uVar22 = FUN_14019bd40(&local_278,iVar25,0);
          FUN_142ef7ba0(uVar22,lVar17,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar25);
          pplVar23 = local_278;
        }
        else {
          iVar7 = *(int *)(pplVar23 + -1);
          for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
              iVar9 = iVar9 * 2) {
          }
          lVar12 = FUN_14019bd40(&local_278,iVar9,1);
          pplVar23 = local_278;
          if (local_278 == (longlong **)0x0) {
            iVar9 = 0;
          }
          else {
            iVar9 = *(int *)(local_278 + -1);
          }
          FUN_142ef7ba0(iVar9 + lVar12,lVar17,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar7 + iVar25);
        }
      }
    }
    if (local_188 != 0) {
      FUN_14019f2c0(local_188 + -0x10);
    }
  }
  uVar8 = FUN_14019a5d0(param_4 + 4);
  lVar17 = FUN_141ed3540(uVar8);
  local_188 = lVar17;
  if ((lVar17 != 0) && (*(int *)(lVar17 + 0x44) != 0)) {
    if ((pplVar23 != (longlong **)0x0) && (*(char *)pplVar23 != '\0')) {
      lVar12 = -1;
      do {
        lVar33 = lVar12 + 1;
        pcVar34 = &DAT_1434b7dfd + lVar12;
        lVar12 = lVar33;
      } while (*pcVar34 != '\0');
      iVar25 = (int)lVar33;
      if (iVar25 != 0) {
        iVar7 = *(int *)(pplVar23 + -1);
        for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25; iVar9 = iVar9 * 2)
        {
        }
        lVar12 = FUN_14019bd40(&local_278,iVar9,1);
        pplVar23 = local_278;
        if (local_278 == (longlong **)0x0) {
          iVar9 = 0;
        }
        else {
          iVar9 = *(int *)(local_278 + -1);
        }
        FUN_142ef7ba0(iVar9 + lVar12,&DAT_1434b7dfc,(longlong)iVar25);
        FUN_14019c870(&local_278,iVar7 + iVar25);
      }
    }
    plVar18 = (longlong *)FUN_1408a9e40(&local_d8,0x3b2);
    lVar12 = *plVar18;
    if (lVar12 != 0) {
      iVar25 = *(int *)(lVar12 + -8);
      if (iVar25 != 0) {
        if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
          uVar22 = FUN_14019bd40(&local_278,iVar25,0);
          FUN_142ef7ba0(uVar22,lVar12,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar25);
          pplVar23 = local_278;
        }
        else {
          iVar7 = *(int *)(pplVar23 + -1);
          for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
              iVar9 = iVar9 * 2) {
          }
          lVar33 = FUN_14019bd40(&local_278,iVar9,1);
          pplVar23 = local_278;
          if (local_278 == (longlong **)0x0) {
            iVar9 = 0;
          }
          else {
            iVar9 = *(int *)(local_278 + -1);
          }
          FUN_142ef7ba0(iVar9 + lVar33,lVar12,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar7 + iVar25);
        }
      }
    }
    if (local_d8 != 0) {
      FUN_14019f2c0(local_d8 + -0x10);
    }
  }
  lVar12 = DAT_143aa8328;
  uVar8 = FUN_14019a5d0(param_4 + 4);
  iVar25 = FUN_14038a300(lVar12,uVar8);
  if (iVar25 != 0) {
    if ((pplVar23 != (longlong **)0x0) && (*(char *)pplVar23 != '\0')) {
      lVar12 = -1;
      do {
        lVar33 = lVar12 + 1;
        pcVar34 = &DAT_1434b7dfd + lVar12;
        lVar12 = lVar33;
      } while (*pcVar34 != '\0');
      iVar25 = (int)lVar33;
      if (iVar25 != 0) {
        iVar7 = *(int *)(pplVar23 + -1);
        for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25; iVar9 = iVar9 * 2)
        {
        }
        lVar12 = FUN_14019bd40(&local_278,iVar9,1);
        pplVar23 = local_278;
        if (local_278 == (longlong **)0x0) {
          iVar9 = 0;
        }
        else {
          iVar9 = *(int *)(local_278 + -1);
        }
        FUN_142ef7ba0(iVar9 + lVar12,&DAT_1434b7dfc,(longlong)iVar25);
        FUN_14019c870(&local_278,iVar7 + iVar25);
      }
    }
    plVar18 = (longlong *)FUN_1408a9e40(&local_130,0x3b3);
    lVar12 = *plVar18;
    if (lVar12 != 0) {
      iVar25 = *(int *)(lVar12 + -8);
      if (iVar25 != 0) {
        if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
          uVar22 = FUN_14019bd40(&local_278,iVar25,0);
          FUN_142ef7ba0(uVar22,lVar12,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar25);
          pplVar23 = local_278;
        }
        else {
          iVar7 = *(int *)(pplVar23 + -1);
          for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
              iVar9 = iVar9 * 2) {
          }
          lVar33 = FUN_14019bd40(&local_278,iVar9,1);
          pplVar23 = local_278;
          if (local_278 == (longlong **)0x0) {
            iVar9 = 0;
          }
          else {
            iVar9 = *(int *)(local_278 + -1);
          }
          FUN_142ef7ba0(iVar9 + lVar33,lVar12,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar7 + iVar25);
        }
      }
    }
    if (local_130 != (char *)0x0) {
      FUN_14019f2c0(local_130 + -0x10);
    }
  }
  lVar12 = DAT_143aa8328;
  if (param_11 == 0) {
    iVar25 = (**(code **)(*param_4 + 0x38))(param_4);
    if (iVar25 != 0) goto LAB_142671282;
    plVar18 = (longlong *)FUN_1408a9e40(&local_c0,0x3bb);
    uVar26 = 0x11;
  }
  else {
    uVar8 = FUN_14019a5d0(param_4 + 4);
    iVar25 = FUN_14038a180(lVar12,uVar8);
    if (iVar25 != 0) {
      if ((pplVar23 != (longlong **)0x0) && (*(char *)pplVar23 != '\0')) {
        lVar12 = -1;
        do {
          lVar33 = lVar12 + 1;
          pcVar34 = &DAT_1434b7dfd + lVar12;
          lVar12 = lVar33;
        } while (*pcVar34 != '\0');
        iVar25 = (int)lVar33;
        if (iVar25 != 0) {
          iVar7 = *(int *)(pplVar23 + -1);
          for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
              iVar9 = iVar9 * 2) {
          }
          lVar12 = FUN_14019bd40(&local_278,iVar9,1);
          pplVar23 = local_278;
          if (local_278 == (longlong **)0x0) {
            iVar9 = 0;
          }
          else {
            iVar9 = *(int *)(local_278 + -1);
          }
          FUN_142ef7ba0(iVar9 + lVar12,&DAT_1434b7dfc,(longlong)iVar25);
          FUN_14019c870(&local_278,iVar7 + iVar25);
        }
      }
      plVar18 = (longlong *)FUN_1408a9e40(&local_238,0xc99);
      lVar12 = *plVar18;
      if (lVar12 != 0) {
        iVar25 = *(int *)(lVar12 + -8);
        if (iVar25 != 0) {
          if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
            uVar22 = FUN_14019bd40(&local_278,iVar25,0);
            FUN_142ef7ba0(uVar22,lVar12,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar25);
            pplVar23 = local_278;
          }
          else {
            iVar7 = *(int *)(pplVar23 + -1);
            for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                iVar9 = iVar9 * 2) {
            }
            lVar33 = FUN_14019bd40(&local_278,iVar9,1);
            pplVar23 = local_278;
            if (local_278 == (longlong **)0x0) {
              iVar9 = 0;
            }
            else {
              iVar9 = *(int *)(local_278 + -1);
            }
            FUN_142ef7ba0(iVar9 + lVar33,lVar12,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar7 + iVar25);
          }
        }
      }
      if (local_238 != 0) {
        FUN_14019f2c0(local_238 + -0x10);
      }
    }
    lVar12 = DAT_143aa8328;
    uVar8 = FUN_14019a5d0(param_4 + 4);
    iVar25 = FUN_14038a290(lVar12,uVar8);
    if (iVar25 != 0) {
      if ((pplVar23 != (longlong **)0x0) && (*(char *)pplVar23 != '\0')) {
        lVar12 = -1;
        do {
          lVar33 = lVar12 + 1;
          pcVar34 = &DAT_1434b7dfd + lVar12;
          lVar12 = lVar33;
        } while (*pcVar34 != '\0');
        iVar7 = (int)lVar33;
        if (iVar7 != 0) {
          iVar9 = *(int *)(pplVar23 + -1);
          for (iVar37 = *(int *)((longlong)pplVar23 + -0xc); iVar37 < iVar9 + iVar7;
              iVar37 = iVar37 * 2) {
          }
          lVar12 = FUN_14019bd40(&local_278,iVar37,1);
          pplVar23 = local_278;
          if (local_278 == (longlong **)0x0) {
            iVar37 = 0;
          }
          else {
            iVar37 = *(int *)(local_278 + -1);
          }
          FUN_142ef7ba0(iVar37 + lVar12,&DAT_1434b7dfc,(longlong)iVar7);
          FUN_14019c870(&local_278,iVar9 + iVar7);
        }
      }
      local_268 = (longlong *)0x0;
      puVar13 = (undefined8 *)FUN_1408a9e40(&local_170,0xc9a);
      plVar18 = (longlong *)FUN_14019ba10(&local_268,*puVar13,iVar25);
      lVar12 = *plVar18;
      if (lVar12 != 0) {
        iVar25 = *(int *)(lVar12 + -8);
        if (iVar25 != 0) {
          if ((pplVar23 == (longlong **)0x0) || (*(char *)pplVar23 == '\0')) {
            uVar22 = FUN_14019bd40(&local_278,iVar25,0);
            FUN_142ef7ba0(uVar22,lVar12,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar25);
            pplVar23 = local_278;
          }
          else {
            iVar7 = *(int *)(pplVar23 + -1);
            for (iVar9 = *(int *)((longlong)pplVar23 + -0xc); iVar9 < iVar7 + iVar25;
                iVar9 = iVar9 * 2) {
            }
            lVar33 = FUN_14019bd40(&local_278,iVar9,1);
            pplVar23 = local_278;
            if (local_278 == (longlong **)0x0) {
              iVar9 = 0;
            }
            else {
              iVar9 = *(int *)(local_278 + -1);
            }
            FUN_142ef7ba0(iVar9 + lVar33,lVar12,(longlong)iVar25);
            FUN_14019c870(&local_278,iVar7 + iVar25);
          }
        }
      }
      if (local_170 != (char *)0x0) {
        FUN_14019f2c0(local_170 + -0x10);
      }
      if (local_268 != (longlong *)0x0) {
        FUN_14019f2c0(local_268 + -2);
      }
    }
LAB_142671282:
    local_238 = 0;
    FUN_1401d66b0(&local_238,&DAT_1434b2af1,0xffffffff);
    plVar18 = &local_238;
    uVar26 = 0x12;
  }
  pcVar34 = (char *)0x0;
  pcVar36 = (char *)*plVar18;
  local_170 = pcVar36;
  local_130 = pcVar36;
  *plVar18 = 0;
  if (((uVar26 & 2) != 0) && (uVar26 = uVar26 & 0xfffffffd, local_248 = uVar26, local_238 != 0)) {
    FUN_14019f2c0(local_238 + -0x10);
  }
  if (((uVar26 & 1) != 0) && (uVar26 = uVar26 & 0xfffffffe, local_c0 != 0)) {
    FUN_14019f2c0(local_c0 + -0x10);
  }
  if ((pcVar36 != (char *)0x0) && (*pcVar36 != '\0')) {
    local_260 = local_260 + 0x10;
  }
  iVar25 = local_260;
  local_238 = 0;
  FUN_1401d66b0(&local_238,(longlong)param_4 + 0x4d,0xffffffff);
  lVar12 = local_238;
  lVar33 = DAT_143aa8360;
  if (*(longlong *)(lVar17 + 8) != 0) {
    lVar33 = *(longlong *)(lVar17 + 8);
  }
  lVar17 = DAT_143aa8360;
  if (local_238 != 0) {
    lVar17 = local_238;
  }
  iVar7 = FUN_142f100b0(lVar17,lVar33);
  if (iVar7 != 0) {
    lVar17 = -1;
    do {
      pcVar30 = &DAT_1434796cd + lVar17;
      lVar17 = lVar17 + 1;
    } while (*pcVar30 != '\0');
    FUN_1401abc80(&local_238,&local_218);
    pcVar30 = pcVar34;
    if (local_220 != (char *)0x0) {
      pcVar30 = (char *)(ulonglong)*(uint *)(local_220 + -8);
    }
    FUN_1401abc80(&local_218,&local_258,local_220,pcVar30);
    lVar17 = -1;
    do {
      pcVar30 = &DAT_1432841a9 + lVar17;
      lVar17 = lVar17 + 1;
    } while (*pcVar30 != '\0');
    FUN_1401abc80(&local_258,&local_128);
    uVar26 = uVar26 | 0x340;
    if (local_220 != (char *)0x0) {
      FUN_14019f2c0(local_220 + -0x10);
    }
    local_220 = local_128;
    if (local_258 != (longlong *)0x0) {
      FUN_14019f2c0(local_258 + -2);
    }
    if (local_218 != (longlong *)0x0) {
      FUN_14019f2c0(local_218 + -2);
    }
  }
  local_200 = (char *)0x0;
  iVar7 = (**(code **)(*param_4 + 0x88))(param_4);
  if ((iVar7 == 3) && (DAT_143aa8328 != 0)) {
    iVar7 = FUN_14038a5b0(DAT_143aa8328,param_4);
    puVar13 = (undefined8 *)FUN_1403e5020(&local_b0,iVar7);
    pcVar34 = (char *)*puVar13;
    *puVar13 = 0;
    local_200 = pcVar34;
    if (local_b0 != 0) {
      FUN_14019f2c0(local_b0 + -0x10);
    }
    if (iVar7 == 1) {
      uVar22 = 0xffffcc00;
    }
    else if (iVar7 == 4) {
      uVar22 = 0xffff66cc;
    }
    else if (iVar7 == 5) {
      uVar22 = 0xff00ccff;
    }
    else {
      if (iVar7 != 6) goto LAB_142671513;
      uVar22 = 0xffbb77ff;
    }
    FUN_1429fa100(&local_1a8,uVar22,0xb,0,CONCAT44(uVar10,2));
  }
  else {
LAB_142671513:
    local_1a8 = (longlong *)0x0;
  }
  uVar26 = uVar26 | 0x400;
  if (local_1a8 != (longlong *)0x0) {
    local_260 = iVar25 + 0xf;
  }
  local_240 = &local_268;
  plVar18 = (longlong *)0x0;
  local_268 = (longlong *)0x0;
  local_230 = &local_258;
  local_258 = (longlong *)0x0;
  local_248 = uVar26;
  uVar22 = FUN_142699220(param_1,local_e0,10);
  in_stack_fffffffffffffd68 = in_stack_fffffffffffffd68 & 0xffffffffffffff00;
  iVar25 = FUN_1426a6b20(param_1,0x5a,0x10e,0,pplVar23,uVar22,0,0,1,&local_258,
                         in_stack_fffffffffffffd68,0,0,&local_268);
  local_1a0 = CONCAT44(local_1a0._4_4_,iVar25);
  if (0x44 < iVar25) {
    local_1b0 = iVar25 + 0x30;
  }
  local_1b8 = local_1b8 & 0xffffffff00000000;
  iVar25 = 0;
  if ((param_8 != 0) && (iVar25 = 0, param_8 != param_9)) {
    iVar25 = FUN_1426c6fe0(param_12);
    local_1b8 = CONCAT44(local_1b8._4_4_,iVar25);
  }
  local_1f8 = (longlong *)(CONCAT44(local_1f8._4_4_,-(uint)(iVar25 != 0)) & 0xffffffff00000023);
  local_210 = 0;
  local_258 = (longlong *)0x0;
  if (param_6 != 0) {
    uVar6 = FUN_1401b00e0((longlong)param_4 + 0x7a,*(undefined4 *)((longlong)param_4 + 0x7e));
    lVar17 = local_188;
    local_228 = 0;
    plVar15 = plVar18;
    do {
      local_250._0_2_ = FUN_1426dc0d0(plVar15);
      local_250._0_2_ = (ushort)local_250 & uVar6;
      if (((ushort)local_250 == 0) || (iVar25 = FUN_1426dc320(lVar17,plVar15), iVar25 != 0)) {
        bVar3 = false;
        if (((ushort)local_250 != 0) || (iVar25 = FUN_1426dc320(lVar17,local_228), iVar25 == 0))
        goto LAB_142671662;
        local_250._0_2_ = 1;
LAB_142671670:
        local_250._2_2_ = 0;
        local_240 = &local_268;
        local_268 = (longlong *)0x0;
        FUN_1401d66b0(&local_268,&DAT_1434b2af1,0xffffffff);
        uVar22 = FUN_141ed1ad0(local_b8,local_228);
        local_270 = (longlong *)FUN_1408e56b0(&local_98,uVar22,&local_268);
        local_270 = (longlong *)*local_270;
        if (local_270 != (longlong *)0x0) {
          iVar25 = (int)local_270[-1];
          if (iVar25 != 0) {
            if ((plVar18 == (longlong *)0x0) || ((char)*plVar18 == '\0')) {
              uVar22 = FUN_14019bd40(&local_258,iVar25,0);
              FUN_142ef7ba0(uVar22,local_270,(longlong)iVar25);
              FUN_14019c870(&local_258,iVar25);
              plVar18 = local_258;
            }
            else {
              iVar9 = (int)plVar18[-1] + iVar25;
              local_218 = (longlong *)CONCAT44(local_218._4_4_,iVar9);
              for (iVar7 = *(int *)((longlong)plVar18 + -0xc); iVar7 < iVar9; iVar7 = iVar7 * 2) {
              }
              lVar12 = FUN_14019bd40(&local_258,iVar7,1);
              plVar18 = local_258;
              if (local_258 == (longlong *)0x0) {
                iVar7 = 0;
              }
              else {
                iVar7 = (int)local_258[-1];
              }
              FUN_142ef7ba0(iVar7 + lVar12,local_270,(longlong)iVar25);
              FUN_14019c870(&local_258,(ulonglong)local_218 & 0xffffffff);
            }
          }
        }
        if (local_98 != 0) {
          FUN_14019f2c0(local_98 + -0x10);
        }
        if ((plVar18 == (longlong *)0x0) || ((char)*plVar18 == '\0')) {
          puVar19 = (undefined1 *)FUN_14019bd40(&local_258,1);
          *puVar19 = 0x20;
          iVar25 = 1;
        }
        else {
          iVar25 = (int)plVar18[-1] + 1;
          for (iVar7 = *(int *)((longlong)plVar18 + -0xc); iVar7 < iVar25; iVar7 = iVar7 * 2) {
          }
          puVar19 = (undefined1 *)FUN_14019bd40(&local_258,iVar7,1);
          if (local_258 == (longlong *)0x0) {
            *puVar19 = 0x20;
          }
          else {
            puVar19[(int)local_258[-1]] = 0x20;
          }
        }
        plVar18 = local_258;
        FUN_14019c870(&local_258,iVar25);
        if (CONCAT22(local_250._2_2_,(ushort)local_250) == 0) {
          local_270 = (longlong *)FUN_1408a9e40(&local_80,0x9e5);
          uVar26 = uVar26 | 8;
        }
        else {
          local_270 = (longlong *)FUN_1408a9e40(&local_110,0x9e6);
          uVar26 = uVar26 | 4;
        }
        local_270 = (longlong *)*local_270;
        local_248 = uVar26;
        if (local_270 != (longlong *)0x0) {
          iVar25 = (int)local_270[-1];
          if (iVar25 != 0) {
            if ((plVar18 == (longlong *)0x0) || ((char)*plVar18 == '\0')) {
              uVar22 = FUN_14019bd40(&local_258,iVar25,0);
              FUN_142ef7ba0(uVar22,local_270,(longlong)iVar25);
              FUN_14019c870(&local_258,iVar25);
              plVar18 = local_258;
            }
            else {
              iVar9 = (int)plVar18[-1] + iVar25;
              local_218 = (longlong *)CONCAT44(local_218._4_4_,iVar9);
              for (iVar7 = *(int *)((longlong)plVar18 + -0xc); iVar7 < iVar9; iVar7 = iVar7 * 2) {
              }
              lVar12 = FUN_14019bd40(&local_258,iVar7,1);
              plVar18 = local_258;
              if (local_258 == (longlong *)0x0) {
                iVar7 = 0;
              }
              else {
                iVar7 = (int)local_258[-1];
              }
              FUN_142ef7ba0(iVar7 + lVar12,local_270,(longlong)iVar25);
              FUN_14019c870(&local_258,(ulonglong)local_218 & 0xffffffff);
            }
          }
        }
        if (((uVar26 & 8) != 0) && (uVar26 = uVar26 & 0xfffffff7, local_248 = uVar26, local_80 != 0)
           ) {
          FUN_14019f2c0(local_80 + -0x10);
        }
        if (((uVar26 & 4) != 0) &&
           (uVar26 = uVar26 & 0xfffffffb, local_248 = uVar26, local_110 != 0)) {
          FUN_14019f2c0(local_110 + -0x10);
        }
        if ((plVar18 == (longlong *)0x0) || ((char)*plVar18 == '\0')) {
          puVar20 = (undefined2 *)FUN_14019bd40(&local_258,2);
          *puVar20 = 0x6e5c;
          FUN_14019c870(&local_258,2);
          plVar18 = local_258;
        }
        else {
          iVar7 = (int)plVar18[-1] + 2;
          for (iVar25 = *(int *)((longlong)plVar18 + -0xc); iVar25 < iVar7; iVar25 = iVar25 * 2) {
          }
          lVar12 = FUN_14019bd40(&local_258,iVar25,1);
          plVar18 = local_258;
          if (local_258 == (longlong *)0x0) {
            iVar25 = 0;
          }
          else {
            iVar25 = (int)local_258[-1];
          }
          *(undefined2 *)(iVar25 + lVar12) = 0x6e5c;
          FUN_14019c870(&local_258,iVar7);
        }
      }
      else {
        bVar3 = true;
LAB_142671662:
        local_250._0_2_ = 0;
        local_250._2_2_ = 0;
        if (bVar3) goto LAB_142671670;
      }
      pcVar36 = local_170;
      pcVar34 = local_200;
      lVar12 = local_238;
      local_228 = local_228 + 1;
      plVar15 = (longlong *)(ulonglong)local_228;
    } while ((int)local_228 < 0xb);
    if ((plVar18 != (longlong *)0x0) && ((char)*plVar18 != '\0')) {
      local_240 = &local_270;
      local_270 = (longlong *)0x0;
      local_230 = &local_268;
      local_268 = (longlong *)0x0;
      uVar22 = FUN_142699220(param_1,&local_178,0xe);
      in_stack_fffffffffffffd68 = in_stack_fffffffffffffd68 & 0xffffffffffffff00;
      local_210 = FUN_1426a6b20(param_1,0x5a,0x10e,0,plVar18,uVar22,0,0,1,&local_268,
                                in_stack_fffffffffffffd68,0,0,&local_270);
      if ((int)local_1a0 < 0x44) {
        local_210 = local_210 + -0x44 + (int)local_1a0;
      }
    }
  }
  pcVar38 = (char *)0x0;
  uVar10 = (**(code **)(*param_4 + 0x180))(param_4);
  lVar17 = FUN_141ed3540(uVar10);
  local_178 = (char *)0x0;
  if (lVar17 != 0) {
    puVar13 = (undefined8 *)FUN_141ed3710(lVar17,&local_108);
    pcVar38 = (char *)*puVar13;
    *puVar13 = 0;
    local_178 = pcVar38;
    if (local_108 != 0) {
      FUN_14019f2c0(local_108 + -0x10);
    }
  }
  FUN_1408a9e40(&local_148,0x3b1);
  local_228 = 0;
  if ((pcVar38 != (char *)0x0) && (*pcVar38 != '\0')) {
    local_240 = &local_270;
    local_270 = (longlong *)0x0;
    local_230 = &local_268;
    local_268 = (longlong *)0x0;
    uVar22 = FUN_142699220(param_1,local_118,10);
    in_stack_fffffffffffffd68 = in_stack_fffffffffffffd68 & 0xffffffffffffff00;
    local_228 = FUN_1426a6b20(param_1,0x5a,0x10e,0,pcVar38,uVar22,0,0,1,&local_268,
                              in_stack_fffffffffffffd68,0,0,&local_270);
  }
  local_240 = &local_270;
  local_270 = (longlong *)0x0;
  local_230 = &local_268;
  local_268 = (longlong *)0x0;
  uVar31 = FUN_142699220(param_1,&local_100,0xe);
  in_stack_fffffffffffffd68 = in_stack_fffffffffffffd68 & 0xffffffffffffff00;
  pplVar29 = &local_268;
  uVar8 = 0;
  lVar17 = local_148;
  iVar25 = FUN_1426a6b20(param_1,0x5a,0x10e,0,local_148,uVar31,0,0,1,pplVar29,
                         in_stack_fffffffffffffd68,0,0,&local_270);
  uVar10 = (undefined4)((ulonglong)lVar17 >> 0x20);
  local_228 = local_228 + iVar25;
  local_250._0_2_ = 0;
  local_250._2_2_ = 0;
  iVar25 = FUN_1401ba9d0((longlong)param_4 + 0xa6,*(undefined4 *)((longlong)param_4 + 0xae));
  if (-1 < iVar25) {
    local_240 = &local_270;
    local_270 = (longlong *)0x0;
    local_230 = &local_268;
    local_268 = (longlong *)0x0;
    local_100 = local_90;
    uVar31 = FUN_142699220(param_1,local_90,0x11);
    puVar13 = (undefined8 *)FUN_1408a9e40(&local_88);
    uVar22 = *puVar13;
    in_stack_fffffffffffffd68 = in_stack_fffffffffffffd68 & 0xffffffffffffff00;
    pplVar29 = &local_268;
    uVar8 = 0;
    uVar11 = FUN_1426a6b20(param_1,0x5a,0x10e,0,uVar22,uVar31,0,0,1,pplVar29,
                           in_stack_fffffffffffffd68,0,0,&local_270);
    uVar10 = (undefined4)((ulonglong)uVar22 >> 0x20);
    local_250._0_2_ = (ushort)uVar11;
    local_250._2_2_ = (undefined2)((uint)uVar11 >> 0x10);
    if (local_88 != 0) {
      FUN_14019f2c0(local_88 + -0x10);
    }
  }
  lVar17 = DAT_143aa8328;
  local_248 = 0;
  uVar11 = (**(code **)(*param_4 + 0x180))();
  FUN_140398ba0(lVar17,&local_1d8,uVar11);
  iVar25 = (**(code **)(*param_4 + 0x1b8))();
  pcVar30 = local_1d8;
  uVar26 = local_248;
  if (((iVar25 != 0) && (local_1d8 != (char *)0x0)) && (*local_1d8 != '\0')) {
    local_270 = (longlong *)0x0;
    puVar13 = (undefined8 *)FUN_1408a9e40(&local_f8,0xfef);
    uVar22 = FUN_14019ba10(&local_270,*puVar13,pcVar30);
    FUN_14019a260(&local_1d8,uVar22);
    if (local_f8 != 0) {
      FUN_14019f2c0(local_f8 + -0x10);
    }
    if (local_270 != (longlong *)0x0) {
      FUN_14019f2c0();
    }
    local_240 = &local_270;
    local_270 = (longlong *)0x0;
    local_230 = &local_268;
    local_268 = (longlong *)0x0;
    uVar31 = FUN_142699220(param_1,local_78,0x2c);
    pplVar29 = &local_268;
    uVar8 = 0;
    pcVar30 = local_1d8;
    uVar26 = FUN_1426a6b20(param_1,0x5a,0x10e,0,local_1d8,uVar31,0,0,1,pplVar29,
                           in_stack_fffffffffffffd68 & 0xffffffffffffff00,0,0,&local_270);
    uVar10 = (undefined4)((ulonglong)pcVar30 >> 0x20);
  }
  uVar22 = CONCAT44(uVar10,0xffffffff);
  FUN_1426453f0(param_1,6,0x122,
                local_1b0 + uVar26 + CONCAT22(local_250._2_2_,(ushort)local_250) + local_228 +
                local_210 + (int)local_1f8 + local_260,uVar22);
  uVar10 = (undefined4)((ulonglong)uVar22 >> 0x20);
  local_210 = *(int *)(param_1 + 0x3c);
  local_218 = (longlong *)0x0;
  FUN_142688320(param_1,&local_218,param_10);
  iVar25 = FUN_1426bea90(param_1,0,param_4,0);
  *(int *)(param_1 + 0x3c) = *(int *)(param_1 + 0x3c) + iVar25;
  uVar41 = 0;
  uVar26 = 0;
  uVar40 = CONCAT44(uVar8,0xcc0e395a);
  FUN_1426974b0(param_1,&local_70,param_2,param_3,CONCAT44(uVar10,1),uVar31 & 0xffffffff00000000,0,
                uVar40,0,(ulonglong)pplVar29 & 0xffffffff00000000,0,1);
  if (local_70 != (longlong *)0x0) {
    (**(code **)(*local_70 + 0x10))();
  }
  uVar22 = FUN_142699220(param_1,local_f0,1);
  FUN_1426a5a40(param_1,10,local_220,uVar22);
  uVar22 = 0x1f;
  local_250._0_2_ = 0x1f;
  local_250._2_2_ = 0;
  if (((pcVar34 != (char *)0x0) && (*pcVar34 != '\0')) && (local_1a8 != (longlong *)0x0)) {
    local_270 = local_1a8;
    (**(code **)(*local_1a8 + 8))();
    FUN_1426a5a40(param_1,0x1f,pcVar34,&local_270);
    uVar22 = 0x2f;
    local_250._0_2_ = 0x2f;
  }
  local_250._2_2_ = 0;
  uVar28 = (uint)(ushort)local_250;
  if ((pcVar36 != (char *)0x0) && (uVar28 = (uint)(ushort)local_250, *pcVar36 != '\0')) {
    uVar21 = FUN_142699220(param_1,local_e8,0xe);
    FUN_1426a5a40(param_1,uVar22,pcVar36,uVar21);
    uVar28 = CONCAT22(local_250._2_2_,(ushort)local_250) + 0x10;
  }
  pcVar30 = local_1d0;
  if ((local_1d0 != (char *)0x0) && (*local_1d0 != '\0')) {
    local_250 = uVar28;
    uVar22 = FUN_142699220(param_1,local_a0,10);
    FUN_1426a5a40(param_1,local_250,pcVar30,uVar22);
    uVar28 = local_250 + 0x10;
  }
  pcVar30 = local_190;
  if ((local_190 != (char *)0x0) && (*local_190 != '\0')) {
    local_250 = uVar28;
    uVar22 = FUN_142699220(param_1,&local_180,10);
    FUN_1426a5a40(param_1,local_250,pcVar30,uVar22);
    uVar28 = local_250 + 0x10;
  }
  if ((local_208 != (longlong *)0x0) && ((int)local_208[-1] != 0)) {
    local_268 = (longlong *)0x0;
    for (uVar27 = 0;
        (lVar12 = local_238, local_208 != (longlong *)0x0 && (uVar27 < *(uint *)(local_208 + -1)));
        uVar27 = uVar27 + 1) {
      local_240 = &local_d0;
      local_250 = uVar28;
      local_180 = FUN_142699220(param_1,&local_d0,0xe);
      if (local_208 == (longlong *)0x0) {
        uVar28 = 0;
      }
      else {
        uVar28 = *(uint *)(local_208 + -1);
      }
      if (((int)uVar27 < 0) || (uVar28 <= uVar27)) {
        if (local_208 == (longlong *)0x0) {
          uVar10 = 0;
        }
        else {
          uVar10 = (undefined4)local_208[-1];
        }
        FUN_142e54290(0xbc,uVar27,uVar10);
      }
      FUN_1426a5a40(param_1,local_250,*(undefined8 *)((longlong)local_268 + (longlong)local_208),
                    local_180);
      uVar28 = local_250 + 0x10;
      local_268 = local_268 + 1;
    }
  }
  pcVar30 = local_1e8;
  if ((local_1e8 != (char *)0x0) && (*local_1e8 != '\0')) {
    local_250 = uVar28;
    uVar22 = FUN_142699220(param_1,local_c8,0xe);
    uVar28 = local_250;
    FUN_1426a5a40(param_1,local_250,pcVar30,uVar22);
    uVar28 = uVar28 + 0x10;
  }
  pcVar30 = local_150;
  local_250 = uVar28;
  if ((local_150 != (char *)0x0) && (*local_150 != '\0')) {
    uVar22 = FUN_142699220(param_1,local_138,0x12);
    FUN_1426a5a40(param_1,local_250,pcVar30,uVar22);
  }
  local_25c = CONCAT13(local_25c._3_1_,1);
  local_240 = (longlong **)local_68;
  local_60 = param_4;
  FUN_14030e710(local_68);
  uVar10 = FUN_14019a5d0(local_1e0);
  FUN_1426a4bd0(param_1,3,local_260 + 0x20,uVar10,local_68,1,0xffffffff,uVar40 & 0xffffffffffffff00,
                uVar26 & 0xffffff00,&local_25c);
  local_240 = &local_270;
  local_270 = (longlong *)0x0;
  local_230 = &local_268;
  local_268 = (longlong *)0x0;
  uVar22 = FUN_142699220(param_1,&local_168,10);
  uVar41 = uVar41 & 0xffffffffffffff00;
  local_25c = FUN_1426a6b20(param_1,0x5a,0x10e,local_260 + 0x23,pplVar23,uVar22,1,0,1,&local_268,
                            uVar41,0,0,&local_270);
  if ((plVar18 != (longlong *)0x0) && ((char)*plVar18 != '\0')) {
    local_240 = &local_270;
    local_270 = (longlong *)0x0;
    local_230 = &local_268;
    local_268 = (longlong *)0x0;
    uVar22 = FUN_142699220(param_1,&local_160,0xe);
    uVar41 = uVar41 & 0xffffffffffffff00;
    iVar25 = FUN_1426a6b20(param_1,0x5a,0x10e,local_260 + local_25c + 0x23,plVar18,uVar22,1,0,1,
                           &local_268,uVar41,0,0,&local_270);
    local_25c = local_25c + iVar25;
  }
  iVar25 = FUN_1401ba9d0((longlong)param_4 + 0xa6,*(undefined4 *)((longlong)param_4 + 0xae));
  if (-1 < iVar25) {
    local_240 = &local_270;
    local_270 = (longlong *)0x0;
    local_230 = &local_268;
    local_268 = (longlong *)0x0;
    local_168 = local_a8;
    local_160 = FUN_142699220(param_1,local_a8,0x11);
    puVar13 = (undefined8 *)FUN_1408a9e40(&local_140,0x102a);
    uVar41 = uVar41 & 0xffffffffffffff00;
    iVar25 = FUN_1426a6b20(param_1,0x5a,0x10e,local_260 + local_25c + 0x23,*puVar13,local_160,1,0,1,
                           &local_268,uVar41,0,0,&local_270);
    local_25c = local_25c + iVar25;
    if (local_140 != 0) {
      FUN_14019f2c0();
    }
  }
  if ((pcVar38 != (char *)0x0) && (*pcVar38 != '\0')) {
    local_240 = &local_270;
    local_270 = (longlong *)0x0;
    local_230 = &local_268;
    local_268 = (longlong *)0x0;
    uVar22 = FUN_142699220(param_1,local_120,10);
    uVar41 = uVar41 & 0xffffffffffffff00;
    iVar25 = FUN_1426a6b20(param_1,0x5a,0x10e,local_260 + local_25c + 0x23,pcVar38,uVar22,1,0,1,
                           &local_268,uVar41,0,0,&local_270);
    local_25c = local_25c + iVar25;
  }
  local_240 = &local_270;
  local_270 = (longlong *)0x0;
  local_230 = &local_268;
  local_268 = (longlong *)0x0;
  uVar22 = FUN_142699220(param_1,&local_198,0xe);
  uVar41 = uVar41 & 0xffffffffffffff00;
  lVar17 = local_148;
  iVar25 = FUN_1426a6b20(param_1,0x5a,0x10e,local_260 + local_25c + 0x23,local_148,uVar22,1,0,1,
                         &local_268,uVar41,0,0,&local_270);
  uVar10 = (undefined4)((ulonglong)lVar17 >> 0x20);
  local_25c = local_25c + iVar25;
  iVar7 = (**(code **)(*param_4 + 0x1b8))();
  iVar25 = local_25c;
  if (((iVar7 != 0) && (local_1d8 != (char *)0x0)) && (*local_1d8 != '\0')) {
    local_240 = &local_270;
    local_270 = (longlong *)0x0;
    local_230 = &local_268;
    local_268 = (longlong *)0x0;
    uVar22 = FUN_142699220(param_1,local_158,0x2c);
    pcVar39 = local_1d8;
    iVar25 = FUN_1426a6b20(param_1,0x5a,0x10e,local_260 + local_25c + 0x23,local_1d8,uVar22,1,0,1,
                           &local_268,uVar41 & 0xffffffffffffff00,0,0,&local_270);
    uVar10 = (undefined4)((ulonglong)pcVar39 >> 0x20);
    iVar25 = local_25c + iVar25;
  }
  if ((int)local_1b8 != 0) {
    if (iVar25 < 0x44) {
      iVar25 = 0x44;
    }
    FUN_1426aacb0(param_1,local_260 + 0x2b + iVar25,param_8,param_9,CONCAT44(uVar10,param_12));
  }
  iVar25 = local_210;
  if (param_10 != 0) {
    iVar25 = FUN_142689ce0(param_1,local_210,&local_218);
    iVar25 = local_210 + iVar25;
  }
  FUN_1426bea90(param_1,iVar25,param_4,1);
  local_270 = local_218;
  if (local_218 != (longlong *)0x0) {
    local_198 = local_218 + -1;
    plVar15 = local_218 + *local_198;
    for (; local_270 < plVar15; local_270 = local_270 + 1) {
      if (*local_270 != 0) {
        FUN_14019f2c0(*local_270 + -0x10);
      }
      lVar12 = local_238;
    }
    thunk_FUN_140205820(local_198,0);
  }
  if (local_1d8 != (char *)0x0) {
    FUN_14019f2c0(local_1d8 + -0x10);
  }
  if (local_148 != 0) {
    FUN_14019f2c0(local_148 + -0x10);
  }
  if (pcVar38 != (char *)0x0) {
    FUN_14019f2c0(pcVar38 + -0x10);
  }
  if (plVar18 != (longlong *)0x0) {
    FUN_14019f2c0(plVar18 + -2);
  }
  if (local_1a8 != (longlong *)0x0) {
    (**(code **)(*local_1a8 + 0x10))();
  }
  if (pcVar34 != (char *)0x0) {
    FUN_14019f2c0(pcVar34 + -0x10);
  }
  if (lVar12 != 0) {
    FUN_14019f2c0(lVar12 + -0x10);
  }
  pcVar34 = local_1d0;
  if (pcVar36 != (char *)0x0) {
    FUN_14019f2c0(pcVar36 + -0x10);
    pcVar34 = local_1d0;
  }
LAB_142672817:
  if (local_220 != (char *)0x0) {
    FUN_14019f2c0(local_220 + -0x10);
  }
  if (pplVar23 != (longlong **)0x0) {
    FUN_14019f2c0(pplVar23 + -2);
  }
  if (local_208 != (longlong *)0x0) {
    plVar18 = local_208 + local_208[-1];
    for (plVar15 = local_208; plVar15 < plVar18; plVar15 = plVar15 + 1) {
      if (*plVar15 != 0) {
        FUN_14019f2c0(*plVar15 + -0x10);
      }
    }
    thunk_FUN_140205820(local_208 + -1,0);
    local_208 = (longlong *)0x0;
  }
  if (pcVar30 != (char *)0x0) {
    FUN_14019f2c0(pcVar30 + -0x10);
  }
  if (local_1e8 != (char *)0x0) {
    FUN_14019f2c0(local_1e8 + -0x10);
  }
  if (pcVar34 != (char *)0x0) {
    FUN_14019f2c0(pcVar34 + -0x10);
  }
  if (local_190 != (char *)0x0) {
    FUN_14019f2c0(local_190 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141ec3fa0 @ 141ec3fa0   (155 bytes)
//===========================================================

void FUN_141ec3fa0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  char cVar2;
  char cVar3;
  int iVar4;
  longlong local_res8;
  undefined1 local_res18 [16];
  
  if ((*(longlong *)(param_1 + 0x120) != 0) &&
     (iVar4 = FUN_140f8abc0(*(longlong *)(param_1 + 0x120) + 0x100), iVar4 != 0)) {
    return;
  }
  cVar2 = FUN_1406e8ae0(param_2);
  cVar3 = FUN_1406e8ae0(param_2);
  FUN_1406e9050(param_2,&local_res8);
  uVar1 = FUN_1401a5780(local_res18,local_res8);
  FUN_141ec6680(param_1,(int)cVar2,(int)cVar3,uVar1,0);
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141ec3f20 @ 141ec3f20   (117 bytes)
//===========================================================

void FUN_141ec3f20(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  
  uVar1 = 0;
  lVar3 = *(longlong *)(param_1 + 0x118) + -0x20;
  if (*(longlong *)(param_1 + 0x118) == 0) {
    lVar3 = 0;
  }
  uVar2 = FUN_14099fe20(lVar3);
  lVar3 = FUN_141892840();
  if (lVar3 != 0) {
    uVar4 = FUN_141892840();
    uVar1 = FUN_141829fd0(uVar4);
  }
  FUN_141d598b0(uVar2,param_2,0,uVar1,1,0);
  return;
}


