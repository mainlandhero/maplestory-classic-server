
//===========================================================
// FUN_1426df300 @ 1426df300   (4849 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1426df300(longlong *param_1,undefined8 param_2,undefined8 param_3,longlong *param_4,
                  longlong *param_5,undefined4 *param_6,int param_7,char param_8)

{
  char *pcVar1;
  char *pcVar2;
  bool bVar3;
  char cVar4;
  char cVar5;
  int iVar6;
  undefined4 uVar7;
  int iVar8;
  longlong *plVar9;
  int *piVar10;
  longlong *plVar11;
  longlong lVar12;
  longlong lVar13;
  undefined4 *puVar14;
  longlong lVar15;
  undefined8 uVar16;
  int iVar17;
  int *piVar18;
  int *piVar19;
  int *piVar20;
  int iVar21;
  int *piVar22;
  uint uVar23;
  uint uVar24;
  int *local_58;
  longlong local_50;
  int *local_48 [2];
  
  puVar14 = param_6;
  plVar11 = param_5;
  piVar20 = (int *)0x0;
  uVar23 = 0;
  iVar8 = 0;
  if (*param_5 == 0) {
    return;
  }
  iVar6 = FUN_14038a960(*param_5,*param_6,param_3,param_4,0);
  if (iVar6 != 0) {
    plVar9 = (longlong *)FUN_1408a9e40(&local_58,0x3b8);
    if (*param_1 != 0) {
      FUN_14019f2c0(*param_1 + -0x10);
      *param_1 = 0;
    }
    *param_1 = *plVar9;
    *plVar9 = 0;
    if (local_58 != (int *)0x0) {
      FUN_14019f2c0(local_58 + -4);
    }
  }
  iVar6 = FUN_14038a860(*plVar11,*puVar14);
  if (iVar6 != 0) {
    if (((char *)*param_1 == (char *)0x0) || (*(char *)*param_1 == '\0')) {
      local_58 = (int *)0x0;
      uVar24 = 1;
      piVar18 = (int *)0x0;
    }
    else {
      piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x13);
      piVar10[1] = 2;
      *piVar10 = -1;
      piVar18 = piVar10 + 4;
      piVar10[2] = 0;
      *(undefined1 *)piVar18 = 0;
      *(undefined2 *)piVar18 = _DAT_143275ba0;
      local_58 = piVar18;
      if (*piVar10 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar10[1] < 2) {
        FUN_142e54290(0x90,piVar10[1],2);
      }
      *piVar10 = 1;
      *(undefined1 *)((longlong)piVar10 + 0x12) = 0;
      if (piVar10[1] + 1 < 3) {
        FUN_142e54290(0x9c,2);
      }
      piVar10[2] = 2;
      uVar24 = 2;
    }
    piVar10 = local_58;
    uVar23 = uVar24;
    plVar11 = (longlong *)FUN_1408a9e40(local_48,0x3b9);
    lVar13 = *plVar11;
    piVar19 = piVar20;
    if (lVar13 != 0) {
      piVar19 = (int *)(ulonglong)*(uint *)(lVar13 + -8);
    }
    FUN_1401abc80(&local_58,&local_50,lVar13,piVar19,uVar23);
    lVar13 = local_50;
    uVar23 = uVar24 | 0x200;
    if ((local_50 != 0) && (iVar6 = *(int *)(local_50 + -8), iVar6 != 0)) {
      pcVar2 = (char *)*param_1;
      if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
        piVar19 = (int *)(pcVar2 + -0x10);
        if (pcVar2 == (char *)0x0) {
          piVar19 = piVar20;
        }
        iVar21 = iVar8;
        if (piVar19 == (int *)0x0) {
LAB_1426df56a:
          if (iVar21 < iVar6) {
            iVar21 = iVar6;
          }
          puVar14 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar21 + 0x11));
          puVar14[1] = iVar21;
          *puVar14 = 0xffffffff;
          *param_1 = (longlong)(puVar14 + 4);
          puVar14[2] = 0;
          *(undefined1 *)*param_1 = 0;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar19[1] < iVar6)) {
            iVar21 = piVar19[2];
            goto LAB_1426df56a;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        lVar12 = *param_1;
        iVar21 = iVar6;
      }
      else {
        iVar21 = *(int *)(pcVar2 + -8) + iVar6;
        for (iVar17 = *(int *)(pcVar2 + -0xc); iVar17 < iVar21; iVar17 = iVar17 * 2) {
        }
        lVar12 = FUN_14019bd40(param_1,iVar17,1);
        if (*param_1 != 0) {
          lVar12 = *(int *)(*param_1 + -8) + lVar12;
        }
      }
      FUN_142ef7ba0(lVar12,lVar13,(longlong)iVar6);
      FUN_14019c870(param_1,iVar21);
      puVar14 = param_6;
    }
    if (lVar13 != 0) {
      FUN_14019f2c0(lVar13 + -0x10);
    }
    if (local_48[0] != (int *)0x0) {
      FUN_14019f2c0(local_48[0] + -4);
    }
    if (((uVar24 & 2) != 0) && (uVar23 = uVar24 & 0xfffffffd | 0x200, piVar18 != (int *)0x0)) {
      FUN_14019f2c0(piVar18 + -4);
    }
    if (((uVar23 & 1) != 0) && (uVar23 = uVar23 & 0xfffffffe, piVar10 != (int *)0x0)) {
      FUN_14019f2c0(piVar10 + -4);
    }
  }
  iVar6 = FUN_14038a8e0(*param_5,*puVar14);
  if (iVar6 != 0) {
    if (((char *)*param_1 == (char *)0x0) || (*(char *)*param_1 == '\0')) {
      local_58 = (int *)0x0;
      uVar24 = uVar23 | 4;
      piVar18 = (int *)0x0;
    }
    else {
      piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x13);
      piVar10[1] = 2;
      *piVar10 = -1;
      piVar18 = piVar10 + 4;
      piVar10[2] = 0;
      *(undefined1 *)piVar18 = 0;
      *(undefined2 *)piVar18 = _DAT_143275ba0;
      local_58 = piVar18;
      if (*piVar10 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar10[1] < 2) {
        FUN_142e54290(0x90,piVar10[1],2);
      }
      *piVar10 = 1;
      *(undefined1 *)((longlong)piVar10 + 0x12) = 0;
      if (piVar10[1] + 1 < 3) {
        FUN_142e54290(0x9c,2);
      }
      piVar10[2] = 2;
      uVar24 = uVar23 | 8;
    }
    piVar10 = local_58;
    uVar23 = uVar24;
    plVar11 = (longlong *)FUN_1408a9e40(&local_50,0x3ba);
    lVar13 = *plVar11;
    piVar19 = piVar20;
    if (lVar13 != 0) {
      piVar19 = (int *)(ulonglong)*(uint *)(lVar13 + -8);
    }
    FUN_1401abc80(&local_58,local_48,lVar13,piVar19,uVar23);
    piVar19 = local_48[0];
    uVar23 = uVar24 | 0x400;
    if ((local_48[0] != (int *)0x0) && (iVar6 = local_48[0][-2], iVar6 != 0)) {
      pcVar2 = (char *)*param_1;
      if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
        piVar22 = (int *)(pcVar2 + -0x10);
        if (pcVar2 == (char *)0x0) {
          piVar22 = piVar20;
        }
        iVar21 = iVar8;
        if (piVar22 == (int *)0x0) {
LAB_1426df802:
          if (iVar21 < iVar6) {
            iVar21 = iVar6;
          }
          puVar14 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar21 + 0x11));
          puVar14[1] = iVar21;
          *puVar14 = 0xffffffff;
          *param_1 = (longlong)(puVar14 + 4);
          puVar14[2] = 0;
          *(undefined1 *)*param_1 = 0;
          if (piVar22 != (int *)0x0) {
            FUN_14019f2c0(piVar22);
          }
        }
        else {
          if ((1 < *piVar22) || (piVar22[1] < iVar6)) {
            iVar21 = piVar22[2];
            goto LAB_1426df802;
          }
          if (*piVar22 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar22 = -1;
        }
        lVar13 = *param_1;
        iVar21 = iVar6;
      }
      else {
        iVar21 = *(int *)(pcVar2 + -8) + iVar6;
        for (iVar17 = *(int *)(pcVar2 + -0xc); iVar17 < iVar21; iVar17 = iVar17 * 2) {
        }
        lVar13 = FUN_14019bd40(param_1,iVar17,1);
        if (*param_1 != 0) {
          lVar13 = *(int *)(*param_1 + -8) + lVar13;
        }
      }
      FUN_142ef7ba0(lVar13,piVar19,(longlong)iVar6);
      FUN_14019c870(param_1,iVar21);
      puVar14 = param_6;
    }
    if (piVar19 != (int *)0x0) {
      FUN_14019f2c0(piVar19 + -4);
    }
    if (local_50 != 0) {
      FUN_14019f2c0(local_50 + -0x10);
    }
    if (((uVar24 & 8) != 0) && (uVar23 = uVar24 & 0xfffffff7 | 0x400, piVar18 != (int *)0x0)) {
      FUN_14019f2c0(piVar18 + -4);
    }
    if (((uVar23 & 4) != 0) && (uVar23 = uVar23 & 0xfffffffb, piVar10 != (int *)0x0)) {
      FUN_14019f2c0(piVar10 + -4);
    }
  }
  cVar4 = param_8;
  uVar16 = DAT_143aa8328;
  if (*param_4 == 0) {
LAB_1426dfb9b:
    cVar5 = FUN_140841850();
    if ((cVar5 == '\0') || (iVar6 = FUN_140389c10(DAT_143aa8328,*puVar14), iVar6 == 0)) {
      iVar6 = FUN_14038aae0(*param_5,*puVar14);
      if (((iVar6 == 0) || (cVar4 != '\0')) &&
         ((param_7 == 0 && (iVar6 = FUN_1404179d0(*puVar14), plVar11 = param_5, iVar6 != 0)))) {
        iVar6 = FUN_14038d050(*param_5,*puVar14);
        if (iVar6 != 0) {
          uVar16 = FUN_1403e8e00(&param_5,*puVar14,0);
          FUN_1407781d0(param_3,uVar16);
          if (param_5 != (longlong *)0x0) {
            FUN_14019f2c0(param_5 + -2);
          }
        }
      }
      else {
        if (((longlong *)*param_4 != (longlong *)0x0) &&
           ((iVar6 = (**(code **)(*(longlong *)*param_4 + 0x38))(), iVar6 != 0 &&
            (iVar6 = (**(code **)(*(longlong *)*param_4 + 0x68))(), uVar16 = DAT_143aa8328,
            iVar6 == 0)))) {
          uVar7 = FUN_14019a5d0(*param_4 + 0x20);
          iVar6 = FUN_14038ac80(uVar16,uVar7);
          if (iVar6 == 0) {
            if (((char *)*param_1 == (char *)0x0) || (*(char *)*param_1 == '\0')) {
              plVar11 = (longlong *)FUN_1408a9e40(local_48,0x3bc);
              lVar13 = *plVar11;
              if (lVar13 != 0) {
                iVar6 = *(int *)(lVar13 + -8);
                lVar12 = (longlong)iVar6;
                if (iVar6 != 0) {
                  pcVar2 = (char *)*param_1;
                  if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
                    piVar18 = (int *)(pcVar2 + -0x10);
                    if (pcVar2 == (char *)0x0) {
                      piVar18 = piVar20;
                    }
                    iVar21 = iVar8;
                    if (piVar18 == (int *)0x0) {
LAB_1426dfda7:
                      if (iVar21 < iVar6) {
                        iVar21 = iVar6;
                      }
                      puVar14 = (undefined4 *)
                                FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar21 + 0x11));
                      puVar14[1] = iVar21;
                      *puVar14 = 0xffffffff;
                      *param_1 = (longlong)(puVar14 + 4);
                      puVar14[2] = 0;
                      *(undefined1 *)*param_1 = 0;
                      if (piVar18 != (int *)0x0) {
                        FUN_14019f2c0(piVar18);
                      }
                    }
                    else {
                      if ((1 < *piVar18) || (piVar18[1] < iVar6)) {
                        iVar21 = piVar18[2];
                        goto LAB_1426dfda7;
                      }
                      if (*piVar18 != 1) {
                        FUN_142e52dd0(0x74);
                      }
                      *piVar18 = -1;
                    }
                    FUN_142ef7ba0(*param_1,lVar13,lVar12);
                  }
                  else {
                    iVar6 = *(int *)(pcVar2 + -8) + iVar6;
                    for (iVar21 = *(int *)(pcVar2 + -0xc); iVar21 < iVar6; iVar21 = iVar21 * 2) {
                    }
                    lVar15 = FUN_14019bd40(param_1,iVar21,1);
                    iVar21 = iVar8;
                    if (*param_1 != 0) {
                      iVar21 = *(int *)(*param_1 + -8);
                    }
                    FUN_142ef7ba0(iVar21 + lVar15,lVar13,lVar12);
                  }
                  FUN_14019c870(param_1,iVar6);
                }
              }
            }
            else {
              uVar16 = FUN_1408a9e40(local_48,0x3bc);
              FUN_140319ad0(param_2,uVar16);
            }
            plVar11 = param_5;
            if (local_48[0] != (int *)0x0) {
              FUN_14019f2c0(local_48[0] + -4);
              plVar11 = param_5;
            }
            goto LAB_1426e013b;
          }
        }
        if (((char *)*param_1 == (char *)0x0) || (*(char *)*param_1 == '\0')) {
          local_48[0] = (int *)0x0;
          uVar23 = uVar23 | 0x40;
          piVar18 = (int *)0x0;
        }
        else {
          piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x13);
          piVar10[1] = 2;
          *piVar10 = -1;
          piVar18 = piVar10 + 4;
          piVar10[2] = 0;
          *(undefined1 *)piVar18 = 0;
          *(undefined2 *)piVar18 = _DAT_143275ba0;
          local_48[0] = piVar18;
          if (*piVar10 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar10[1] < 2) {
            FUN_142e54290(0x90,piVar10[1],2);
          }
          *piVar10 = 1;
          *(undefined1 *)((longlong)piVar10 + 0x12) = 0;
          if (piVar10[1] + 1 < 3) {
            FUN_142e54290(0x9c,2);
          }
          piVar10[2] = 2;
          uVar23 = uVar23 | 0x80;
          iVar6 = piVar10[2];
          if (iVar6 != 0) {
            pcVar2 = (char *)*param_1;
            if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
              piVar10 = (int *)(pcVar2 + -0x10);
              if (pcVar2 == (char *)0x0) {
                piVar10 = piVar20;
              }
              iVar21 = iVar8;
              if (piVar10 == (int *)0x0) {
LAB_1426dff68:
                if (iVar21 < iVar6) {
                  iVar21 = iVar6;
                }
                puVar14 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar21 + 0x11));
                puVar14[1] = iVar21;
                *puVar14 = 0xffffffff;
                *param_1 = (longlong)(puVar14 + 4);
                puVar14[2] = 0;
                *(undefined1 *)*param_1 = 0;
                if (piVar10 != (int *)0x0) {
                  FUN_14019f2c0(piVar10);
                }
              }
              else {
                if ((1 < *piVar10) || (piVar10[1] < iVar6)) {
                  iVar21 = piVar10[2];
                  goto LAB_1426dff68;
                }
                if (*piVar10 != 1) {
                  FUN_142e52dd0(0x74);
                }
                *piVar10 = -1;
              }
              FUN_142ef7ba0(*param_1,piVar18,(longlong)iVar6);
              FUN_14019c870(param_1,iVar6);
            }
            else {
              iVar21 = *(int *)(pcVar2 + -8);
              for (iVar17 = *(int *)(pcVar2 + -0xc); iVar17 < iVar21 + iVar6; iVar17 = iVar17 * 2) {
              }
              lVar13 = FUN_14019bd40(param_1,iVar17,1);
              iVar17 = iVar8;
              if (*param_1 != 0) {
                iVar17 = *(int *)(*param_1 + -8);
              }
              FUN_142ef7ba0(iVar17 + lVar13,piVar18,(longlong)iVar6);
              FUN_14019c870(param_1,iVar21 + iVar6);
            }
          }
        }
        if (((char)uVar23 < '\0') && (uVar23 = uVar23 & 0xffffff7f, piVar18 != (int *)0x0)) {
          FUN_14019f2c0(piVar18 + -4);
        }
        if (((uVar23 & 0x40) != 0) && (local_48[0] != (int *)0x0)) {
          FUN_14019f2c0(local_48[0] + -4);
        }
        plVar11 = (longlong *)FUN_1408a9e40(local_48,0x3bb);
        lVar13 = *plVar11;
        if (lVar13 != 0) {
          iVar6 = *(int *)(lVar13 + -8);
          lVar12 = (longlong)iVar6;
          if (iVar6 != 0) {
            pcVar2 = (char *)*param_1;
            if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
              piVar18 = (int *)(pcVar2 + -0x10);
              if (pcVar2 == (char *)0x0) {
                piVar18 = piVar20;
              }
              iVar21 = iVar8;
              if (piVar18 == (int *)0x0) {
LAB_1426e00c9:
                if (iVar21 < iVar6) {
                  iVar21 = iVar6;
                }
                puVar14 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar21 + 0x11));
                puVar14[1] = iVar21;
                *puVar14 = 0xffffffff;
                *param_1 = (longlong)(puVar14 + 4);
                puVar14[2] = 0;
                *(undefined1 *)*param_1 = 0;
                if (piVar18 != (int *)0x0) {
                  FUN_14019f2c0(piVar18);
                }
              }
              else {
                if ((1 < *piVar18) || (piVar18[1] < iVar6)) {
                  iVar21 = piVar18[2];
                  goto LAB_1426e00c9;
                }
                if (*piVar18 != 1) {
                  FUN_142e52dd0(0x74);
                }
                *piVar18 = -1;
              }
              FUN_142ef7ba0(*param_1,lVar13,lVar12);
            }
            else {
              iVar6 = *(int *)(pcVar2 + -8) + iVar6;
              for (iVar21 = *(int *)(pcVar2 + -0xc); iVar21 < iVar6; iVar21 = iVar21 * 2) {
              }
              lVar15 = FUN_14019bd40(param_1,iVar21,1);
              iVar21 = iVar8;
              if (*param_1 != 0) {
                iVar21 = *(int *)(*param_1 + -8);
              }
              FUN_142ef7ba0(iVar21 + lVar15,lVar13,lVar12);
            }
            FUN_14019c870(param_1,iVar6);
          }
        }
        plVar11 = param_5;
        if (local_48[0] != (int *)0x0) {
          FUN_14019f2c0(local_48[0] + -4);
          plVar11 = param_5;
        }
      }
    }
    else {
      uVar16 = FUN_1408a9e40(local_48,0x3bb);
      FUN_1407781d0(param_3,uVar16);
      plVar11 = param_5;
      if (local_48[0] != (int *)0x0) {
        FUN_14019f2c0(local_48[0] + -4);
        plVar11 = param_5;
      }
    }
  }
  else {
    uVar7 = FUN_14019a5d0(*param_4 + 0x20);
    iVar6 = FUN_140389c10(uVar16,uVar7);
    if (((iVar6 == 0) || (cVar4 != '\0')) ||
       (iVar6 = (**(code **)(*(longlong *)*param_4 + 0x38))(), iVar6 != 0)) goto LAB_1426dfb9b;
    if (((char *)*param_1 == (char *)0x0) || (*(char *)*param_1 == '\0')) {
      local_58 = (int *)0x0;
      uVar23 = uVar23 | 0x10;
      piVar18 = (int *)0x0;
    }
    else {
      piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x13);
      piVar10[1] = 2;
      *piVar10 = -1;
      piVar18 = piVar10 + 4;
      piVar10[2] = 0;
      *(undefined1 *)piVar18 = 0;
      *(undefined2 *)piVar18 = _DAT_143275ba0;
      local_58 = piVar18;
      if (*piVar10 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar10[1] < 2) {
        FUN_142e54290(0x90,piVar10[1],2);
      }
      *piVar10 = 1;
      *(undefined1 *)((longlong)piVar10 + 0x12) = 0;
      if (piVar10[1] + 1 < 3) {
        FUN_142e54290(0x9c,2);
      }
      piVar10[2] = 2;
      uVar23 = uVar23 | 0x20;
    }
    piVar10 = local_58;
    uVar24 = uVar23;
    plVar11 = (longlong *)FUN_1408a9e40(&local_50,0x3bb);
    lVar13 = *plVar11;
    piVar19 = piVar20;
    if (lVar13 != 0) {
      piVar19 = (int *)(ulonglong)*(uint *)(lVar13 + -8);
    }
    FUN_1401abc80(&local_58,local_48,lVar13,piVar19,uVar24);
    piVar19 = local_48[0];
    if ((local_48[0] != (int *)0x0) && (iVar6 = local_48[0][-2], iVar6 != 0)) {
      pcVar2 = (char *)*param_1;
      if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
        piVar22 = (int *)(pcVar2 + -0x10);
        if (pcVar2 == (char *)0x0) {
          piVar22 = piVar20;
        }
        iVar21 = iVar8;
        if (piVar22 == (int *)0x0) {
LAB_1426dfad4:
          if (iVar21 < iVar6) {
            iVar21 = iVar6;
          }
          puVar14 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar21 + 0x11));
          puVar14[1] = iVar21;
          *puVar14 = 0xffffffff;
          *param_1 = (longlong)(puVar14 + 4);
          puVar14[2] = 0;
          *(undefined1 *)*param_1 = 0;
          if (piVar22 != (int *)0x0) {
            FUN_14019f2c0(piVar22);
          }
        }
        else {
          iVar21 = *piVar22;
          if ((1 < iVar21) || (piVar22[1] < iVar6)) {
            iVar21 = piVar22[2];
            goto LAB_1426dfad4;
          }
          if (iVar21 != 1) {
            FUN_142e52dd0(0x74,iVar21);
          }
          *piVar22 = -1;
        }
        lVar13 = *param_1;
        iVar21 = iVar6;
      }
      else {
        iVar21 = *(int *)(pcVar2 + -8) + iVar6;
        for (iVar17 = *(int *)(pcVar2 + -0xc); iVar17 < iVar21; iVar17 = iVar17 * 2) {
        }
        lVar13 = FUN_14019bd40(param_1,iVar17,1);
        if (*param_1 != 0) {
          lVar13 = *(int *)(*param_1 + -8) + lVar13;
        }
      }
      FUN_142ef7ba0(lVar13,piVar19,(longlong)iVar6);
      FUN_14019c870(param_1,iVar21);
    }
    if (piVar19 != (int *)0x0) {
      FUN_14019f2c0(piVar19 + -4);
    }
    if (local_50 != 0) {
      FUN_14019f2c0(local_50 + -0x10);
    }
    if (((uVar23 & 0x20) != 0) && (uVar23 = uVar23 & 0xffffffdf, piVar18 != (int *)0x0)) {
      FUN_14019f2c0(piVar18 + -4);
    }
    plVar11 = param_5;
    if (((uVar23 & 0x10) != 0) && (piVar10 != (int *)0x0)) {
      FUN_14019f2c0(piVar10 + -4);
      plVar11 = param_5;
    }
  }
LAB_1426e013b:
  bVar3 = false;
  puVar14 = param_6;
  if ((((*param_4 == 0) || (iVar6 = FUN_14038c1f0(*plVar11), puVar14 = param_6, iVar6 == 0)) ||
      (iVar6 = FUN_14038aae0(*plVar11,*param_6), iVar6 != 0)) || (param_7 != 0)) {
    iVar8 = FUN_14038c0f0(DAT_143aa8328,*puVar14);
    if (iVar8 == 0) {
      iVar8 = FUN_14038c170(DAT_143aa8328,*puVar14);
      if (iVar8 == 0) {
        return;
      }
      uVar16 = FUN_1408a9e40(&param_5,0xcca);
      FUN_1407781d0(param_3,uVar16);
      if (param_5 != (longlong *)0x0) {
        FUN_14019f2c0(param_5 + -2);
      }
      uVar16 = FUN_1408a9e40(&param_5,0xccb);
      FUN_1407781d0(param_3,uVar16);
    }
    else {
      plVar11 = (longlong *)FUN_1408a9e40(&param_5,0xcc9);
      if (*param_1 != 0) {
        FUN_14019f2c0(*param_1 + -0x10);
        *param_1 = 0;
      }
      *param_1 = *plVar11;
      *plVar11 = 0;
    }
    if (param_5 == (longlong *)0x0) {
      return;
    }
    FUN_14019f2c0(param_5 + -2);
    return;
  }
  iVar6 = (**(code **)(*(longlong *)*param_4 + 0x68))();
  if (iVar6 != 0) {
    FUN_1426dd220(param_3);
    return;
  }
  iVar6 = FUN_14038c3c0(*plVar11,*param_4);
  if (iVar6 != 0) {
    iVar6 = (**(code **)(*(longlong *)*param_4 + 0x290))();
    if (iVar6 == 0) {
      FUN_1426dd220(param_3);
      goto LAB_1426e035c;
    }
    plVar11 = (longlong *)FUN_1408a9e40(&param_5,0x3bb);
    bVar3 = true;
    pcVar2 = (char *)*param_1;
    if ((pcVar2 != (char *)0x0) && (*pcVar2 != '\0')) {
      lVar13 = -1;
      do {
        lVar12 = lVar13 + 1;
        pcVar1 = &DAT_143275ba1 + lVar13;
        lVar13 = lVar12;
      } while (*pcVar1 != '\0');
      iVar6 = (int)lVar12;
      if (iVar6 != 0) {
        iVar21 = *(int *)(pcVar2 + -8);
        for (iVar17 = *(int *)(pcVar2 + -0xc); iVar17 < iVar21 + iVar6; iVar17 = iVar17 * 2) {
        }
        lVar13 = FUN_14019bd40(param_1,iVar17,1);
        iVar17 = iVar8;
        if (*param_1 != 0) {
          iVar17 = *(int *)(*param_1 + -8);
        }
        FUN_142ef7ba0(iVar17 + lVar13,&DAT_143275ba0,(longlong)iVar6);
        FUN_14019c870(param_1,iVar21 + iVar6);
      }
    }
    lVar13 = *plVar11;
    if (lVar13 == 0) goto LAB_1426e035c;
    iVar6 = *(int *)(lVar13 + -8);
    if (iVar6 == 0) goto LAB_1426e035c;
    pcVar2 = (char *)*param_1;
    if ((pcVar2 != (char *)0x0) && (*pcVar2 != '\0')) {
      iVar21 = *(int *)(pcVar2 + -8);
      for (iVar17 = *(int *)(pcVar2 + -0xc); iVar17 < iVar21 + iVar6; iVar17 = iVar17 * 2) {
      }
      lVar12 = FUN_14019bd40(param_1,iVar17,1);
      if (*param_1 != 0) {
        iVar8 = *(int *)(*param_1 + -8);
      }
      FUN_142ef7ba0(iVar8 + lVar12,lVar13,(longlong)iVar6);
      FUN_14019c870(param_1,iVar21 + iVar6);
      goto LAB_1426e035c;
    }
    piVar18 = (int *)(pcVar2 + -0x10);
    if (pcVar2 == (char *)0x0) {
      piVar18 = piVar20;
    }
    if (piVar18 == (int *)0x0) {
LAB_1426e02fc:
      if (iVar8 < iVar6) {
        iVar8 = iVar6;
      }
      puVar14 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
      puVar14[1] = iVar8;
      *puVar14 = 0xffffffff;
      *param_1 = (longlong)(puVar14 + 4);
      puVar14[2] = 0;
      *(undefined1 *)*param_1 = 0;
      if (piVar18 != (int *)0x0) {
        FUN_14019f2c0(piVar18);
      }
    }
    else {
      if ((1 < *piVar18) || (piVar18[1] < iVar6)) {
        iVar8 = piVar18[2];
        goto LAB_1426e02fc;
      }
      if (*piVar18 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar18 = -1;
    }
    FUN_142ef7ba0(*param_1,lVar13,(longlong)iVar6);
    FUN_14019c870(param_1,iVar6);
LAB_1426e035c:
    if (!bVar3) {
      return;
    }
    if (param_5 == (longlong *)0x0) {
      return;
    }
    FUN_14019f2c0(param_5 + -2);
    return;
  }
  plVar11 = (longlong *)FUN_1408a9e40(&param_5,0xcc9);
  pcVar2 = (char *)*param_1;
  if ((pcVar2 != (char *)0x0) && (*pcVar2 != '\0')) {
    lVar13 = -1;
    do {
      lVar12 = lVar13 + 1;
      pcVar1 = &DAT_143275ba1 + lVar13;
      lVar13 = lVar12;
    } while (*pcVar1 != '\0');
    iVar6 = (int)lVar12;
    if (iVar6 != 0) {
      iVar21 = *(int *)(pcVar2 + -8);
      for (iVar17 = *(int *)(pcVar2 + -0xc); iVar17 < iVar21 + iVar6; iVar17 = iVar17 * 2) {
      }
      lVar13 = FUN_14019bd40(param_1,iVar17,1);
      iVar17 = iVar8;
      if (*param_1 != 0) {
        iVar17 = *(int *)(*param_1 + -8);
      }
      FUN_142ef7ba0(iVar17 + lVar13,&DAT_143275ba0,(longlong)iVar6);
      FUN_14019c870(param_1,iVar21 + iVar6);
    }
  }
  lVar13 = *plVar11;
  if (lVar13 == 0) goto LAB_1426e050a;
  iVar6 = *(int *)(lVar13 + -8);
  lVar12 = (longlong)iVar6;
  if (iVar6 == 0) goto LAB_1426e050a;
  pcVar2 = (char *)*param_1;
  if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
    piVar18 = (int *)(pcVar2 + -0x10);
    if (pcVar2 == (char *)0x0) {
      piVar18 = piVar20;
    }
    if (piVar18 == (int *)0x0) {
LAB_1426e04b3:
      if (iVar8 < iVar6) {
        iVar8 = iVar6;
      }
      puVar14 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
      puVar14[1] = iVar8;
      *puVar14 = 0xffffffff;
      *param_1 = (longlong)(puVar14 + 4);
      puVar14[2] = 0;
      *(undefined1 *)*param_1 = 0;
      if (piVar18 != (int *)0x0) {
        FUN_14019f2c0(piVar18);
      }
    }
    else {
      if ((1 < *piVar18) || (piVar18[1] < iVar6)) {
        iVar8 = piVar18[2];
        goto LAB_1426e04b3;
      }
      if (*piVar18 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar18 = -1;
    }
    FUN_142ef7ba0(*param_1,lVar13,lVar12);
  }
  else {
    iVar6 = *(int *)(pcVar2 + -8) + iVar6;
    for (iVar21 = *(int *)(pcVar2 + -0xc); iVar21 < iVar6; iVar21 = iVar21 * 2) {
    }
    lVar15 = FUN_14019bd40(param_1,iVar21,1);
    if (*param_1 != 0) {
      iVar8 = *(int *)(*param_1 + -8);
    }
    FUN_142ef7ba0(iVar8 + lVar15,lVar13,lVar12);
  }
  FUN_14019c870(param_1,iVar6);
LAB_1426e050a:
  if (param_5 != (longlong *)0x0) {
    FUN_14019f2c0(param_5 + -2);
  }
  return;
}



//===========================================================
// FUN_1426e10e0 @ 1426e10e0   (1626 bytes)
//===========================================================

void FUN_1426e10e0(undefined8 param_1,longlong *param_2,longlong param_3,char param_4)

{
  char cVar1;
  undefined4 uVar2;
  int iVar3;
  undefined8 uVar4;
  longlong lVar5;
  bool bVar6;
  longlong local_38;
  longlong local_30;
  
  uVar4 = DAT_143aa8328;
  if (param_2 == (longlong *)0x0) {
    return;
  }
  uVar2 = FUN_1401b0340(param_2 + 4);
  iVar3 = FUN_14038a960(uVar4,uVar2);
  if (iVar3 != 0) {
    uVar4 = FUN_1408a9e40(&local_38,0x3b8);
    FUN_1407781d0(param_1,uVar4);
    if (local_38 != 0) {
      FUN_14019f2c0(local_38 + -0x10);
    }
  }
  uVar4 = DAT_143aa8328;
  uVar2 = FUN_1401b0340(param_2 + 4);
  iVar3 = FUN_14038a860(uVar4,uVar2);
  if (iVar3 != 0) {
    uVar4 = FUN_1408a9e40(&local_38,0x3b9);
    FUN_1407781d0(param_1,uVar4);
    if (local_38 != 0) {
      FUN_14019f2c0(local_38 + -0x10);
    }
  }
  uVar4 = DAT_143aa8328;
  uVar2 = FUN_1401b0340(param_2 + 4);
  iVar3 = FUN_14038a8e0(uVar4,uVar2);
  if (iVar3 != 0) {
    uVar4 = FUN_1408a9e40(&local_38,0x3ba);
    FUN_1407781d0(param_1,uVar4);
    if (local_38 != 0) {
      FUN_14019f2c0(local_38 + -0x10);
    }
  }
  uVar4 = DAT_143aa8328;
  uVar2 = FUN_1401b0340(param_2 + 4);
  iVar3 = FUN_140389c10(uVar4,uVar2);
  if (((iVar3 == 0) || (param_4 != '\0')) ||
     (iVar3 = (**(code **)(*param_2 + 0x38))(param_2), iVar3 != 0)) {
    cVar1 = FUN_140841850();
    uVar4 = DAT_143aa8328;
    if (cVar1 != '\0') {
      uVar2 = FUN_1401b0340(param_2 + 4);
      iVar3 = FUN_140389c10(uVar4,uVar2);
      if (iVar3 != 0) {
        uVar4 = FUN_1408a9e40(&local_38,0x3bb);
        FUN_1407781d0(param_1,uVar4);
        goto LAB_1426e137a;
      }
    }
    iVar3 = FUN_14038ab40(DAT_143aa8328,param_2);
    if (iVar3 == 0) {
      uVar2 = FUN_1401b0340(param_2 + 4);
      iVar3 = FUN_1404179d0(uVar2);
      if (iVar3 == 0) goto LAB_1426e12c0;
    }
    else {
LAB_1426e12c0:
      iVar3 = (**(code **)(*param_2 + 0x68))(param_2);
      if (iVar3 == 0) {
        iVar3 = (**(code **)(*param_2 + 0x38))(param_2);
        uVar4 = DAT_143aa8328;
        if (iVar3 == 0) {
LAB_1426e1315:
          uVar4 = FUN_1408a9e40(&local_38,0x3bb);
          FUN_1407781d0(param_1,uVar4);
        }
        else {
          uVar2 = FUN_1401b0340(param_2 + 4);
          iVar3 = FUN_14038ac80(uVar4,uVar2);
          if (iVar3 != 0) goto LAB_1426e1315;
          uVar4 = FUN_1408a9e40(&local_38,0x3bc);
          FUN_1407781d0(param_1,uVar4);
        }
        if (local_38 != 0) {
          FUN_14019f2c0(local_38 + -0x10);
        }
      }
    }
    cVar1 = FUN_14038cf10(DAT_143aa8328,param_2);
    if (cVar1 == '\0') goto LAB_1426e138d;
    uVar2 = FUN_1401b0340(param_2 + 4);
    uVar4 = FUN_1403e8e00(&local_38,uVar2,0);
    FUN_1407781d0(param_1,uVar4);
  }
  else {
    uVar4 = FUN_1408a9e40(&local_38,0x3bb);
    FUN_1407781d0(param_1,uVar4);
  }
LAB_1426e137a:
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
LAB_1426e138d:
  uVar4 = DAT_143aa8328;
  uVar2 = FUN_1401b0340(param_2 + 4);
  iVar3 = FUN_14038a9c0(uVar4,uVar2);
  if (iVar3 != 0) {
    uVar4 = FUN_1408a9e40(&local_38,0x3c0);
    FUN_1407781d0(param_1,uVar4);
    if (local_38 != 0) {
      FUN_14019f2c0(local_38 + -0x10);
    }
  }
  iVar3 = FUN_14038c1f0(DAT_143aa8328,param_2);
  if (iVar3 != 0) {
    iVar3 = (**(code **)(*param_2 + 0x68))(param_2);
    if (iVar3 == 0) {
      iVar3 = FUN_14038c3c0(DAT_143aa8328,param_2);
      if (iVar3 == 0) {
        iVar3 = (**(code **)(*param_2 + 0x28))(param_2);
        if (iVar3 == 0) {
          uVar4 = FUN_1408a9e40(&local_38,0xcc9);
          FUN_1407781d0(param_1,uVar4);
          if (local_38 != 0) {
            FUN_14019f2c0(local_38 + -0x10);
          }
        }
      }
      else {
        iVar3 = (**(code **)(*param_2 + 0x290))();
        if (iVar3 == 0) {
          FUN_1426dd290(param_1);
        }
        else {
          uVar4 = FUN_1408a9e40(&local_38,0x3bb);
          FUN_1407781d0(param_1,uVar4);
          if (local_38 != 0) {
            FUN_14019f2c0(local_38 + -0x10);
          }
        }
      }
    }
    else {
      FUN_1426dd290(param_1);
    }
  }
  uVar4 = DAT_143aa8328;
  uVar2 = FUN_1401b0340(param_2 + 4);
  iVar3 = FUN_14038bef0(uVar4,uVar2);
  if (iVar3 != 0) {
    uVar4 = FUN_1408a9e40(&local_38,0x3c4);
    FUN_1407781d0(param_1,uVar4);
    if (local_38 != 0) {
      FUN_14019f2c0(local_38 + -0x10);
    }
  }
  uVar4 = DAT_143aa8328;
  uVar2 = FUN_1401b0340(param_2 + 4);
  lVar5 = FUN_140388c60(uVar4,uVar2);
  cVar1 = FUN_1408419d0();
  if (((cVar1 == '\0') && (lVar5 != 0)) && (*(char *)(lVar5 + 0x1a4) != '\0')) {
    uVar4 = FUN_1408a9e40(&local_38,0x3c5);
    FUN_1407781d0(param_1,uVar4);
    if (local_38 != 0) {
      FUN_14019f2c0(local_38 + -0x10);
    }
  }
  iVar3 = FUN_1401b0340(param_2 + 4);
  uVar4 = DAT_143aa8328;
  if (iVar3 - 0x116520U < 10000) {
    uVar2 = FUN_1401b0340(param_2 + 4);
    iVar3 = FUN_14038d260(uVar4,uVar2);
    if (iVar3 == 0) {
      uVar4 = FUN_1408a9e40(&local_38,0x3d0);
      FUN_1407781d0(param_1,uVar4);
      if (local_38 != 0) {
        FUN_14019f2c0(local_38 + -0x10);
      }
    }
  }
  uVar4 = DAT_143aa8328;
  uVar2 = FUN_1401b0340(param_2 + 4);
  iVar3 = FUN_1403a75d0(uVar4,uVar2);
  if (iVar3 != 0) {
    uVar4 = FUN_1408a9e40(&local_38,0x3a1);
    FUN_1407781d0(param_1,uVar4);
    if (local_38 != 0) {
      FUN_14019f2c0(local_38 + -0x10);
    }
  }
  iVar3 = FUN_1403c3c00(DAT_143aa8328,param_2);
  if (((iVar3 != 0) && (iVar3 = (**(code **)(*param_2 + 0x10))(param_2), iVar3 == 0)) &&
     ((param_3 == 0 || (*(int *)(param_3 + 0x40) != 0)))) {
    uVar4 = FUN_1408a9e40(&local_38,0x37a);
    FUN_1407781d0(param_1,uVar4);
    if (local_38 != 0) {
      FUN_14019f2c0(local_38 + -0x10);
    }
  }
  uVar2 = FUN_1401b0340(param_2 + 4);
  FUN_140253d30(uVar2,2);
  uVar4 = DAT_143aa8328;
  uVar2 = FUN_1401b0340(param_2 + 4);
  cVar1 = FUN_1403e14e0(uVar4,uVar2);
  if (cVar1 == '\0') {
    iVar3 = FUN_1401b0340(param_2 + 4);
    bVar6 = 9999 < iVar3 - 0x116520U;
    if (bVar6) {
      uVar4 = FUN_1408a9e40(&local_38,0x3d1);
    }
    else {
      uVar4 = FUN_1408a9e40(&local_30,0x3d2);
    }
    FUN_1407781d0(param_1,uVar4);
    if ((bVar6) && (local_38 != 0)) {
      FUN_14019f2c0(local_38 + -0x10);
    }
    if ((!bVar6) && (local_30 != 0)) {
      FUN_14019f2c0(local_30 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1426aed50 @ 1426aed50   (2340 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001426aef40) */
/* WARNING: Removing unreachable block (ram,0x0001426af574) */

void FUN_1426aed50(undefined8 param_1,longlong param_2,longlong param_3)

{
  longlong lVar1;
  char *pcVar2;
  char cVar3;
  undefined4 uVar4;
  int iVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  int *piVar8;
  undefined4 *puVar9;
  int iVar10;
  int *piVar11;
  int iVar12;
  ulonglong uVar13;
  int *piVar14;
  ulonglong uVar15;
  int *piVar16;
  int *piVar17;
  ulonglong uVar18;
  int iVar19;
  int *local_res10;
  int *local_res20;
  int *local_68;
  int *local_60;
  undefined1 local_58 [8];
  longlong local_50;
  longlong local_48;
  int *local_40;
  
  uVar6 = DAT_143aa8328;
  if (param_2 == 0) {
    return;
  }
  if (param_3 == 0) {
    return;
  }
  lVar1 = param_2 + 0x20;
  uVar4 = FUN_1401b0340(lVar1);
  iVar5 = FUN_14038aa60(uVar6,uVar4);
  piVar11 = (int *)0x0;
  if (iVar5 != 0) {
    uVar6 = FUN_1408a9e40(local_58,0x3e0);
    FUN_14269a1d0(param_1,0x2c,uVar6,1000,0,0);
  }
  FUN_1426adc20(param_1,param_2,param_3);
  FUN_1426adf00(param_1,param_2);
  local_res10 = (int *)0x0;
  cVar3 = *(char *)(param_3 + 0x173);
  uVar18 = 0xffffffffffffffff;
  if (cVar3 != '\0') {
    puVar7 = (undefined8 *)FUN_1408a9e40(&local_50,0x37e);
    FUN_14019ba10(&local_res10,*puVar7,cVar3);
    if (local_50 != 0) {
      FUN_14019f2c0(local_50 + -0x10);
    }
    piVar11 = local_res10;
    local_res20 = (int *)0x0;
    piVar17 = local_res20;
    if ((local_res10 != (int *)0x0) && (piVar8 = local_res10 + -4, piVar8 != (int *)0x0)) {
      if (*piVar8 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar13 = 0xffffffffffffffff;
        do {
          uVar13 = uVar13 + 1;
        } while (*(char *)((longlong)piVar11 + uVar13) != '\0');
        iVar12 = (int)uVar13;
        iVar5 = 0;
        if (0 < iVar12) {
          iVar5 = iVar12;
        }
        piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
        piVar8[1] = iVar5;
        *piVar8 = -1;
        piVar17 = piVar8 + 4;
        piVar8[2] = 0;
        *(char *)piVar17 = '\0';
        local_40 = piVar17;
        FUN_142ef7ba0(piVar17,piVar11,(longlong)iVar12);
        if (*piVar8 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar12 == -1) || (iVar12 <= piVar8[1])) {
          *piVar8 = 1;
          if (iVar12 != -1) goto LAB_1426aeefb;
          if (piVar17 == (int *)0x0) {
            uVar13 = 0;
          }
          else {
            uVar13 = 0xffffffffffffffff;
            do {
              uVar13 = uVar13 + 1;
            } while (*(char *)((longlong)piVar17 + uVar13) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar8[1],uVar13 & 0xffffffff);
          *piVar8 = 1;
LAB_1426aeefb:
          *(char *)((longlong)iVar12 + (longlong)piVar17) = '\0';
        }
        iVar5 = (int)uVar13;
        if ((iVar5 < 0) || (piVar8[1] + 1 <= iVar5)) {
          FUN_142e54290(0x9c,uVar13 & 0xffffffff);
        }
        piVar8[2] = iVar5;
        if (local_res20 != (int *)0x0) {
          FUN_14019f2c0(local_res20 + -4);
        }
      }
      else {
        if (*piVar8 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar8 = *piVar8 + 1;
        UNLOCK();
        if (local_res20 != (int *)0x0) {
          FUN_14019f2c0(local_res20 + -4);
        }
        local_res20 = piVar11;
        piVar11 = local_res10;
        piVar17 = local_res20;
      }
    }
    local_res20 = piVar17;
    FUN_14269a1d0(param_1,0x28,&local_res20,0x3e9,0,0);
  }
  piVar17 = (int *)0x0;
  uVar4 = FUN_1401b0340(lVar1);
  FUN_1403e32b0(&local_60,uVar4);
  if ((local_60 == (int *)0x0) || (iVar5 = FUN_1401b0340(lVar1), iVar5 == 0x147a75))
  goto LAB_1426af63e;
  puVar7 = (undefined8 *)FUN_1408a9e40(&local_48,0x37c);
  FUN_14019ba10(&local_res10,*puVar7);
  if (local_48 != 0) {
    FUN_14019f2c0(local_48 + -0x10);
  }
  piVar11 = local_res10;
  piVar8 = (int *)0xffffffffffffffff;
  do {
    piVar16 = (int *)((longlong)piVar8 + 1);
    pcVar2 = (char *)((longlong)piVar8 + 0x143275b99);
    piVar8 = piVar16;
  } while (*pcVar2 != '\0');
  iVar5 = (int)piVar16;
  piVar8 = local_res10;
  if (iVar5 != 0) {
    iVar12 = 0;
    piVar14 = piVar17;
    if (local_res10 == (int *)0x0) goto LAB_1426af192;
    piVar8 = piVar11;
    if ((char)*local_res10 != '\0') {
      iVar12 = local_res10[-2];
      for (iVar10 = local_res10[-3]; iVar10 < iVar12 + iVar5; iVar10 = iVar10 * 2) {
      }
      piVar16 = local_res10 + -4;
      if (piVar16 == (int *)0x0) {
LAB_1426af09e:
        iVar19 = (int)piVar17;
        if ((int)piVar17 < iVar10) {
          iVar19 = iVar10;
        }
        puVar9 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar19 + 0x11));
        puVar9[1] = iVar19;
        *puVar9 = 0xffffffff;
        piVar8 = puVar9 + 4;
        local_res10 = piVar8;
        if (piVar16 == (int *)0x0) {
          puVar9[2] = 0;
          *(char *)piVar8 = '\0';
        }
        else {
          iVar10 = piVar11[-2] + 1;
          if (iVar19 + 1 < iVar10) {
            FUN_142e54290(0x5c,iVar10,iVar19 + 1);
            iVar10 = iVar19 + 1;
          }
          FUN_142ef7ba0(piVar8,piVar11,(longlong)iVar10);
          puVar9[2] = piVar11[-2];
          *(char *)((longlong)iVar19 + (longlong)piVar8) = '\0';
          FUN_14019f2c0(piVar16);
        }
      }
      else {
        if ((1 < *piVar16) || (local_res10[-3] < iVar10)) {
          piVar17 = (int *)(ulonglong)(uint)local_res10[-2];
          goto LAB_1426af09e;
        }
        if (*piVar16 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar16 = -1;
      }
      if (piVar8 == (int *)0x0) {
        iVar10 = 0;
      }
      else {
        iVar10 = piVar8[-2];
      }
      FUN_142ef7ba0((char *)((longlong)iVar10 + (longlong)piVar8),&DAT_143275b98,(longlong)iVar5);
      FUN_14019c870(&local_res10,iVar12 + iVar5);
      goto LAB_1426af239;
    }
    if ((local_res10 == (int *)0x0) || (piVar14 = local_res10 + -4, piVar14 == (int *)0x0)) {
LAB_1426af192:
      if (iVar12 < iVar5) {
        iVar12 = iVar5;
      }
      puVar9 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar12 + 0x11));
      puVar9[1] = iVar12;
      *puVar9 = 0xffffffff;
      piVar8 = puVar9 + 4;
      puVar9[2] = 0;
      *(char *)piVar8 = '\0';
      local_res10 = piVar8;
      if (piVar14 != (int *)0x0) {
        FUN_14019f2c0(piVar14);
      }
    }
    else {
      if ((1 < *piVar14) || (local_res10[-3] < iVar5)) {
        iVar12 = local_res10[-2];
        goto LAB_1426af192;
      }
      if (*piVar14 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar14 = -1;
    }
    FUN_142ef7ba0(piVar8,&DAT_143275b98,(longlong)iVar5);
    if (piVar8[-4] != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar5 == -1) || (iVar5 <= piVar8[-3])) {
      piVar8[-4] = 1;
      if (iVar5 != -1) goto LAB_1426af213;
      if (piVar8 != (int *)0x0) {
        piVar17 = (int *)0xffffffffffffffff;
        do {
          piVar17 = (int *)((longlong)piVar17 + 1);
        } while (*(char *)((longlong)piVar8 + (longlong)piVar17) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar8[-3],(ulonglong)piVar16 & 0xffffffff);
      piVar8[-4] = 1;
LAB_1426af213:
      *(char *)((longlong)iVar5 + (longlong)piVar8) = '\0';
      piVar17 = piVar16;
    }
    iVar5 = (int)piVar17;
    if ((iVar5 < 0) || (piVar8[-3] + 1 <= iVar5)) {
      FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
    }
    piVar8[-2] = iVar5;
  }
LAB_1426af239:
  piVar11 = local_60;
  local_68 = local_60;
  piVar17 = piVar8;
  if (local_60 != (int *)0x0) {
    iVar5 = local_60[-2];
    uVar13 = (ulonglong)iVar5;
    if (iVar5 != 0) {
      if (piVar8 == (int *)0x0) {
LAB_1426af3c6:
        piVar16 = (int *)0x0;
LAB_1426af3c8:
        iVar12 = 0;
LAB_1426af3ca:
        if (iVar12 < iVar5) {
          iVar12 = iVar5;
        }
        puVar9 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar12 + 0x11));
        puVar9[1] = iVar12;
        *puVar9 = 0xffffffff;
        piVar17 = puVar9 + 4;
        puVar9[2] = 0;
        *(char *)piVar17 = '\0';
        local_res10 = piVar17;
        if (piVar16 != (int *)0x0) {
          FUN_14019f2c0(piVar16);
        }
      }
      else {
        if ((char)*piVar8 != '\0') {
          iVar12 = piVar8[-2];
          for (iVar10 = piVar8[-3]; iVar10 < iVar12 + iVar5; iVar10 = iVar10 * 2) {
          }
          piVar16 = piVar8 + -4;
          if (piVar16 == (int *)0x0) {
            iVar19 = 0;
LAB_1426af2e1:
            if (iVar19 < iVar10) {
              iVar19 = iVar10;
            }
            puVar9 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar19 + 0x11));
            puVar9[1] = iVar19;
            *puVar9 = 0xffffffff;
            piVar17 = puVar9 + 4;
            local_res10 = piVar17;
            if (piVar16 == (int *)0x0) {
              puVar9[2] = 0;
              *(char *)piVar17 = '\0';
              piVar11 = local_68;
            }
            else {
              iVar10 = piVar8[-2] + 1;
              if (iVar19 + 1 < iVar10) {
                FUN_142e54290(0x5c,iVar10,iVar19 + 1);
                iVar10 = iVar19 + 1;
              }
              FUN_142ef7ba0(piVar17,piVar8,(longlong)iVar10);
              puVar9[2] = piVar8[-2];
              *(char *)((longlong)iVar19 + (longlong)piVar17) = '\0';
              FUN_14019f2c0(piVar16);
              piVar11 = local_68;
            }
          }
          else {
            if ((1 < *piVar16) || (piVar8[-3] < iVar10)) {
              iVar19 = piVar8[-2];
              goto LAB_1426af2e1;
            }
            if (*piVar16 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar16 = -1;
          }
          if (piVar17 == (int *)0x0) {
            iVar10 = 0;
          }
          else {
            iVar10 = piVar17[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar10 + (longlong)piVar17),piVar11,uVar13);
          FUN_14019c870(&local_res10,iVar12 + iVar5);
          goto LAB_1426af471;
        }
        if (piVar8 == (int *)0x0) goto LAB_1426af3c6;
        piVar16 = piVar8 + -4;
        if (piVar16 == (int *)0x0) goto LAB_1426af3c8;
        if ((1 < *piVar16) || (piVar8[-3] < iVar5)) {
          iVar12 = piVar8[-2];
          goto LAB_1426af3ca;
        }
        if (*piVar16 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar16 = -1;
      }
      FUN_142ef7ba0(piVar17,piVar11,uVar13);
      if (piVar17[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar5 == -1) || (iVar5 <= piVar17[-3])) {
        piVar17[-4] = 1;
        if (iVar5 != -1) goto LAB_1426af44e;
        if (piVar17 == (int *)0x0) {
          uVar13 = 0;
        }
        else {
          uVar13 = 0xffffffffffffffff;
          do {
            uVar13 = uVar13 + 1;
          } while (*(char *)((longlong)piVar17 + uVar13) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar17[-3],iVar5);
        piVar17[-4] = 1;
LAB_1426af44e:
        *(char *)(uVar13 + (longlong)piVar17) = '\0';
      }
      iVar5 = (int)uVar13;
      if ((iVar5 < 0) || (piVar17[-3] + 1 <= iVar5)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar17[-2] = iVar5;
    }
  }
LAB_1426af471:
  uVar13 = 0;
  local_68 = (int *)0x0;
  piVar11 = piVar17;
  piVar8 = local_68;
  if ((piVar17 != (int *)0x0) && (piVar16 = piVar17 + -4, piVar16 != (int *)0x0)) {
    if (*piVar16 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar15 = 0xffffffffffffffff;
      do {
        uVar15 = uVar15 + 1;
      } while (*(char *)((longlong)piVar17 + uVar15) != '\0');
      iVar12 = (int)uVar15;
      iVar5 = 0;
      if (0 < iVar12) {
        iVar5 = iVar12;
      }
      piVar16 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
      piVar16[1] = iVar5;
      *piVar16 = -1;
      piVar8 = piVar16 + 4;
      piVar16[2] = 0;
      *(char *)piVar8 = '\0';
      local_40 = piVar8;
      FUN_142ef7ba0(piVar8,piVar17,(longlong)iVar12);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar12 == -1) || (iVar12 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar12 != -1) goto LAB_1426af533;
        if (piVar8 != (int *)0x0) {
          do {
            uVar18 = uVar18 + 1;
          } while (*(char *)((longlong)piVar8 + uVar18) != '\0');
          uVar13 = uVar18 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar15 & 0xffffffff);
        *piVar16 = 1;
LAB_1426af533:
        *(char *)((longlong)iVar12 + (longlong)piVar8) = '\0';
        uVar13 = uVar15;
      }
      iVar5 = (int)uVar13;
      if ((iVar5 < 0) || (piVar16[1] + 1 <= iVar5)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar16[2] = iVar5;
      if (local_68 != (int *)0x0) {
        FUN_14019f2c0(local_68 + -4);
      }
    }
    else {
      if (*piVar16 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar16 = *piVar16 + 1;
      UNLOCK();
      piVar11 = local_res10;
      piVar8 = piVar17;
      if (local_68 != (int *)0x0) {
        FUN_14019f2c0(local_68 + -4);
        piVar11 = local_res10;
      }
    }
  }
  local_68 = piVar8;
  FUN_14269a1d0(param_1,0x28,&local_68,0x3e9,0,0);
LAB_1426af63e:
  if (local_60 != (int *)0x0) {
    FUN_14019f2c0(local_60 + -4);
  }
  if (piVar11 != (int *)0x0) {
    FUN_14019f2c0(piVar11 + -4);
  }
  return;
}



//===========================================================
// FUN_1426af680 @ 1426af680   (566 bytes)
//===========================================================

void FUN_1426af680(undefined8 param_1,longlong param_2,longlong param_3,undefined4 *param_4)

{
  undefined8 uVar1;
  short sVar2;
  short sVar3;
  longlong local_res10;
  longlong local_28 [2];
  
  if ((param_2 != 0) && (param_3 != 0)) {
    local_res10 = 0;
    FUN_1401b0340(param_2 + 0x20);
    uVar1 = FUN_1408a9e40(local_28,0x663);
    sVar2 = FUN_1401ab420(param_2 + 0x313,*(undefined4 *)(param_2 + 0x317));
    sVar3 = FUN_1401ab420(param_2 + 0x62,*(undefined4 *)(param_2 + 0x66));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xba),*param_4,(int)sVar3,
                  (int)sVar2,uVar1);
    if (local_28[0] != 0) {
      FUN_14019f2c0(local_28[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_28,0x664);
    sVar2 = FUN_1401ab420(param_2 + 0x31b,*(undefined4 *)(param_2 + 799));
    sVar3 = FUN_1401ab420(param_2 + 0x6a,*(undefined4 *)(param_2 + 0x6e));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xbc),param_4[1],(int)sVar3,
                  (int)sVar2,uVar1);
    if (local_28[0] != 0) {
      FUN_14019f2c0(local_28[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_28,0x665);
    sVar2 = FUN_1401ab420(param_2 + 0x323,*(undefined4 *)(param_2 + 0x327));
    sVar3 = FUN_1401ab420(param_2 + 0x72,*(undefined4 *)(param_2 + 0x76));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xbe),param_4[2],(int)sVar3,
                  (int)sVar2,uVar1);
    if (local_28[0] != 0) {
      FUN_14019f2c0(local_28[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_28,0x666);
    sVar2 = FUN_1401ab420(param_2 + 0x32b,*(undefined4 *)(param_2 + 0x32f));
    sVar3 = FUN_1401ab420(param_2 + 0x7a,*(undefined4 *)(param_2 + 0x7e));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xc0),param_4[3],(int)sVar3,
                  (int)sVar2,uVar1);
    if (local_28[0] != 0) {
      FUN_14019f2c0(local_28[0] + -0x10);
    }
    if (local_res10 != 0) {
      FUN_14019f2c0(local_res10 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1426af8c0 @ 1426af8c0   (1269 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001426afb33) */
/* WARNING: Removing unreachable block (ram,0x0001426afcfd) */

void FUN_1426af8c0(undefined8 param_1,longlong param_2,longlong param_3,longlong param_4)

{
  int *piVar1;
  short sVar2;
  short sVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  int *piVar6;
  int *piVar7;
  int iVar8;
  int iVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  int *local_res10;
  int *local_68;
  int *local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  int *local_38;
  
  if (param_2 == 0) {
    return;
  }
  if (param_3 == 0) {
    return;
  }
  local_res10 = (int *)0x0;
  uVar4 = FUN_1408a9e40(&local_58,0x668);
  sVar2 = FUN_1401ab420(param_2 + 0x333,*(undefined4 *)(param_2 + 0x337));
  sVar3 = FUN_1401ab420(param_2 + 0x82,*(undefined4 *)(param_2 + 0x86));
  FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xc2),*(undefined4 *)(param_4 + 0x14)
                ,(int)sVar3,(int)sVar2,uVar4);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  uVar4 = FUN_1408a9e40(&local_50,0x669);
  sVar2 = FUN_1401ab420(param_2 + 0x33b,*(undefined4 *)(param_2 + 0x33f));
  sVar3 = FUN_1401ab420(param_2 + 0x8a,*(undefined4 *)(param_2 + 0x8e));
  FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xc4),*(undefined4 *)(param_4 + 0x18)
                ,(int)sVar3,(int)sVar2,uVar4);
  if (local_50 != 0) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  sVar2 = *(short *)(param_3 + 0xc6);
  uVar12 = 0xffffffffffffffff;
  piVar7 = local_res10;
  if (0 < sVar2) {
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_48,0x66e);
    FUN_14019ba10(&local_res10,*puVar5,(int)sVar2);
    if (local_48 != 0) {
      FUN_14019f2c0(local_48 + -0x10);
    }
    piVar7 = local_res10;
    local_68 = (int *)0x0;
    piVar1 = local_68;
    if ((local_res10 != (int *)0x0) && (piVar6 = local_res10 + -4, piVar6 != (int *)0x0)) {
      if (*piVar6 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar10 = 0xffffffffffffffff;
        do {
          uVar10 = uVar10 + 1;
        } while (*(char *)((longlong)piVar7 + uVar10) != '\0');
        iVar8 = (int)uVar10;
        iVar9 = 0;
        if (0 < iVar8) {
          iVar9 = iVar8;
        }
        piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
        piVar6[1] = iVar9;
        *piVar6 = -1;
        piVar1 = piVar6 + 4;
        piVar6[2] = 0;
        *(undefined1 *)piVar1 = 0;
        local_38 = piVar1;
        FUN_142ef7ba0(piVar1,piVar7,(longlong)iVar8);
        if (*piVar6 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar8 == -1) || (iVar8 <= piVar6[1])) {
          *piVar6 = 1;
          if (iVar8 != -1) goto LAB_1426afaee;
          if (piVar1 != (int *)0x0) {
            uVar10 = 0xffffffffffffffff;
            do {
              uVar10 = uVar10 + 1;
            } while (*(char *)((longlong)piVar1 + uVar10) != '\0');
            goto LAB_1426afaf3;
          }
          iVar9 = 0;
        }
        else {
          FUN_142e54290(0x90,piVar6[1],uVar10 & 0xffffffff);
          *piVar6 = 1;
LAB_1426afaee:
          *(undefined1 *)((longlong)piVar1 + (longlong)iVar8) = 0;
LAB_1426afaf3:
          iVar9 = (int)uVar10;
        }
        if ((iVar9 < 0) || (piVar6[1] + 1 <= iVar9)) {
          FUN_142e54290(0x9c,iVar9);
        }
        piVar6[2] = iVar9;
        if (local_68 != (int *)0x0) {
          FUN_14019f2c0(local_68 + -4);
        }
      }
      else {
        if (*piVar6 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar6 = *piVar6 + 1;
        UNLOCK();
        if (local_68 != (int *)0x0) {
          FUN_14019f2c0(local_68 + -4);
        }
        local_68 = piVar7;
        piVar7 = local_res10;
        piVar1 = local_68;
      }
    }
    local_68 = piVar1;
    FUN_14269a1d0(param_1,0x28,&local_68,0x3e9,0,0);
  }
  uVar10 = 0;
  sVar2 = *(short *)(param_3 + 200);
  if (sVar2 < 1) goto LAB_1426afd85;
  puVar5 = (undefined8 *)FUN_1408a9e40(&local_40,0x66f);
  FUN_14019ba10(&local_res10,*puVar5,(int)sVar2);
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  piVar7 = local_res10;
  local_60 = (int *)0x0;
  piVar1 = local_60;
  if ((local_res10 != (int *)0x0) && (piVar6 = local_res10 + -4, piVar6 != (int *)0x0)) {
    if (*piVar6 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar11 = 0xffffffffffffffff;
      do {
        uVar11 = uVar11 + 1;
      } while (*(char *)((longlong)piVar7 + uVar11) != '\0');
      iVar8 = (int)uVar11;
      iVar9 = 0;
      if (0 < iVar8) {
        iVar9 = iVar8;
      }
      piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
      piVar6[1] = iVar9;
      *piVar6 = -1;
      piVar1 = piVar6 + 4;
      piVar6[2] = 0;
      *(undefined1 *)piVar1 = 0;
      local_38 = piVar1;
      FUN_142ef7ba0(piVar1,piVar7,(longlong)iVar8);
      if (*piVar6 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar8 == -1) || (iVar8 <= piVar6[1])) {
        *piVar6 = 1;
        if (iVar8 != -1) goto LAB_1426afcbb;
        if (piVar1 != (int *)0x0) {
          do {
            uVar12 = uVar12 + 1;
          } while (*(char *)((longlong)piVar1 + uVar12) != '\0');
          uVar10 = uVar12 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar6[1],uVar11 & 0xffffffff);
        *piVar6 = 1;
LAB_1426afcbb:
        *(undefined1 *)((longlong)piVar1 + (longlong)iVar8) = 0;
        uVar10 = uVar11;
      }
      iVar9 = (int)uVar10;
      if ((iVar9 < 0) || (piVar6[1] + 1 <= iVar9)) {
        FUN_142e54290(0x9c,uVar10 & 0xffffffff);
      }
      piVar6[2] = iVar9;
      if (local_60 != (int *)0x0) {
        FUN_14019f2c0(local_60 + -4);
      }
    }
    else {
      if (*piVar6 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar6 = *piVar6 + 1;
      UNLOCK();
      if (local_60 != (int *)0x0) {
        FUN_14019f2c0(local_60 + -4);
      }
      local_60 = piVar7;
      piVar7 = local_res10;
      piVar1 = local_60;
    }
  }
  local_60 = piVar1;
  FUN_14269a1d0(param_1,0x28,&local_60,0x3e9,0,0);
LAB_1426afd85:
  if (piVar7 != (int *)0x0) {
    FUN_14019f2c0(piVar7 + -4);
  }
  return;
}



//===========================================================
// FUN_1426afdc0 @ 1426afdc0   (1161 bytes)
//===========================================================

void FUN_1426afdc0(undefined8 param_1,longlong param_2,longlong param_3,longlong param_4)

{
  undefined8 uVar1;
  short sVar2;
  short sVar3;
  longlong local_res10;
  longlong local_38 [2];
  
  if ((param_2 != 0) && (param_3 != 0)) {
    local_res10 = 0;
    uVar1 = FUN_1408a9e40(local_38,0x37f);
    sVar2 = FUN_1401ab420(param_2 + 0x393,*(undefined4 *)(param_2 + 0x397));
    sVar3 = FUN_1401ab420(param_2 + 0xe2,*(undefined4 *)(param_2 + 0xe6));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xca),0,(int)sVar3,(int)sVar2,uVar1
                 );
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_38,0x380);
    sVar2 = FUN_1401ab420(param_2 + 0x353,*(undefined4 *)(param_2 + 0x357));
    sVar3 = FUN_1401ab420(param_2 + 0xa2,*(undefined4 *)(param_2 + 0xa6));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xcc),
                  *(undefined4 *)(param_4 + 0x34),(int)sVar3,(int)sVar2,uVar1);
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_38,0x381);
    sVar2 = FUN_1401ab420(param_2 + 0x35b,*(undefined4 *)(param_2 + 0x35f));
    sVar3 = FUN_1401ab420(param_2 + 0xaa,*(undefined4 *)(param_2 + 0xae));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xce),
                  *(undefined4 *)(param_4 + 0x38),(int)sVar3,(int)sVar2,uVar1);
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_38,899);
    sVar2 = FUN_1401ab420(param_2 + 0x363,*(undefined4 *)(param_2 + 0x367));
    sVar3 = FUN_1401ab420(param_2 + 0xb2,*(undefined4 *)(param_2 + 0xb6));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xd0),
                  *(undefined4 *)(param_4 + 0x1c),(int)sVar3,(int)sVar2,uVar1);
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_38,900);
    sVar2 = FUN_1401ab420(param_2 + 0x36b,*(undefined4 *)(param_2 + 0x36f));
    sVar3 = FUN_1401ab420(param_2 + 0xba,*(undefined4 *)(param_2 + 0xbe));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xd2),
                  *(undefined4 *)(param_4 + 0x20),(int)sVar3,(int)sVar2,uVar1);
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_38,0x385);
    sVar2 = FUN_1401ab420(param_2 + 0x373,*(undefined4 *)(param_2 + 0x377));
    sVar3 = FUN_1401ab420(param_2 + 0xc2,*(undefined4 *)(param_2 + 0xc6));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xd4),
                  *(undefined4 *)(param_4 + 0x24),(int)sVar3,(int)sVar2,uVar1);
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_38,0x386);
    sVar2 = FUN_1401ab420(param_2 + 0x37b,*(undefined4 *)(param_2 + 0x37f));
    sVar3 = FUN_1401ab420(param_2 + 0xca,*(undefined4 *)(param_2 + 0xce));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xd6),
                  *(undefined4 *)(param_4 + 0x28),(int)sVar3,(int)sVar2,uVar1);
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_38,0x387);
    sVar2 = FUN_1401ab420(param_2 + 0x373,*(undefined4 *)(param_2 + 0x377));
    sVar3 = FUN_1401ab420(param_2 + 0xd2,*(undefined4 *)(param_2 + 0xd6));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xd8),
                  *(undefined4 *)(param_4 + 0x2c),(int)sVar3,(int)sVar2,uVar1);
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    uVar1 = FUN_1408a9e40(local_38,0x388);
    sVar2 = FUN_1401ab420(param_2 + 0x37b,*(undefined4 *)(param_2 + 0x37f));
    sVar3 = FUN_1401ab420(param_2 + 0xda,*(undefined4 *)(param_2 + 0xde));
    FUN_142699710(param_1,&local_res10,(int)*(short *)(param_3 + 0xda),
                  *(undefined4 *)(param_4 + 0x30),(int)sVar3,(int)sVar2,uVar1);
    if (local_38[0] != 0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
    if (local_res10 != 0) {
      FUN_14019f2c0(local_res10 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1426b0250 @ 1426b0250   (584 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001426b03f1) */

void FUN_1426b0250(undefined8 param_1,longlong param_2)

{
  int *piVar1;
  longlong lVar2;
  undefined8 uVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  int *piVar8;
  ulonglong uVar9;
  int *piVar10;
  int *local_res10;
  int *local_res18;
  int *local_res20;
  
  if (param_2 == 0) {
    return;
  }
  if (*(int *)(param_2 + 0x26c) < 1) {
    return;
  }
  lVar2 = FUN_140378ec0(DAT_143aa83c0);
  if (lVar2 == 0) {
    return;
  }
  piVar8 = (int *)0x0;
  local_res18 = (int *)0x0;
  FUN_14019a260(&local_res18,lVar2 + 0x18);
  uVar3 = FUN_1426d3490(lVar2 + 0x20,*(int *)(param_2 + 0x270) + -1);
  FUN_1403e5fd0(&local_res18,uVar3);
  piVar1 = local_res18;
  local_res10 = (int *)0x0;
  piVar10 = local_res10;
  if ((local_res18 == (int *)0x0) || (piVar7 = local_res18 + -4, piVar7 == (int *)0x0))
  goto LAB_1426b044f;
  if (*piVar7 != -1) {
    if (*piVar7 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar7 = *piVar7 + 1;
    UNLOCK();
    if (local_res10 != (int *)0x0) {
      FUN_14019f2c0(local_res10 + -4);
    }
    local_res10 = piVar1;
    piVar10 = local_res10;
    goto LAB_1426b044f;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  piVar1 = local_res18;
  local_res20 = (int *)0x0;
  piVar10 = piVar8;
  if (local_res18 != (int *)0x0) {
    uVar9 = 0xffffffffffffffff;
    piVar7 = (int *)0xffffffffffffffff;
    do {
      piVar7 = (int *)((longlong)piVar7 + 1);
    } while (*(char *)((longlong)piVar7 + (longlong)local_res18) != '\0');
    iVar5 = (int)piVar7;
    iVar6 = 0;
    if (0 < iVar5) {
      iVar6 = iVar5;
    }
    piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
    piVar4[1] = iVar6;
    *piVar4 = -1;
    piVar10 = piVar4 + 4;
    piVar4[2] = 0;
    *(undefined1 *)piVar10 = 0;
    local_res20 = piVar10;
    FUN_142ef7ba0(piVar10,piVar1,(longlong)iVar5);
    if (*piVar4 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar5 == -1) || (iVar5 <= piVar4[1])) {
      *piVar4 = 1;
      if (iVar5 != -1) goto LAB_1426b03ad;
      if (piVar10 != (int *)0x0) {
        do {
          uVar9 = uVar9 + 1;
        } while (*(char *)((longlong)piVar10 + uVar9) != '\0');
        piVar8 = (int *)(uVar9 & 0xffffffff);
      }
    }
    else {
      FUN_142e54290(0x90,piVar4[1],(ulonglong)piVar7 & 0xffffffff);
      *piVar4 = 1;
LAB_1426b03ad:
      *(undefined1 *)((longlong)piVar10 + (longlong)iVar5) = 0;
      piVar8 = piVar7;
    }
    iVar6 = (int)piVar8;
    if ((iVar6 < 0) || (piVar4[1] + 1 <= iVar6)) {
      FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
    }
    piVar4[2] = iVar6;
  }
  if (local_res10 != (int *)0x0) {
    FUN_14019f2c0(local_res10 + -4);
  }
LAB_1426b044f:
  local_res10 = piVar10;
  FUN_14269a1d0(param_1,0x28,&local_res10,0x3e9,0,0);
  if (local_res18 != (int *)0x0) {
    FUN_14019f2c0(local_res18 + -4);
  }
  return;
}



//===========================================================
// FUN_1426b04a0 @ 1426b04a0   (3721 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001426b101e) */
/* WARNING: Removing unreachable block (ram,0x0001426b0beb) */
/* WARNING: Removing unreachable block (ram,0x0001426b085b) */
/* WARNING: Removing unreachable block (ram,0x0001426b069e) */
/* WARNING: Removing unreachable block (ram,0x0001426b0a2a) */
/* WARNING: Removing unreachable block (ram,0x0001426b0dd9) */
/* WARNING: Removing unreachable block (ram,0x0001426b1259) */

void FUN_1426b04a0(undefined8 param_1,longlong param_2)

{
  int *piVar1;
  int iVar2;
  undefined4 uVar3;
  int iVar4;
  longlong lVar5;
  undefined8 *puVar6;
  int *piVar7;
  int *piVar8;
  undefined8 uVar9;
  int *piVar10;
  ulonglong uVar11;
  int **ppiVar12;
  int iVar13;
  int iVar14;
  ulonglong uVar15;
  int *local_res10;
  int *local_res18;
  int *local_res20;
  int *local_f8;
  int *local_f0;
  int *local_e8;
  int *local_e0;
  int *local_d8;
  int *local_d0;
  longlong local_c8;
  int *local_c0;
  int *local_b8;
  longlong local_b0;
  int *local_a8;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  int *local_68;
  int *local_60;
  int *local_58;
  
  if (param_2 == 0) {
    return;
  }
  param_2 = param_2 + 0x20;
  local_c8 = param_2;
  iVar2 = FUN_1401b0340(param_2);
  uVar9 = DAT_143aa8328;
  if ((9999 < iVar2 - 1500000U) && (9999 < iVar2 - 0x170a70U)) {
    return;
  }
  uVar3 = FUN_1401b0340(param_2);
  lVar5 = FUN_1403a66f0(uVar9,uVar3);
  if (lVar5 == 0) {
    return;
  }
  piVar10 = (int *)0x0;
  local_res10 = (int *)0x0;
  iVar2 = FUN_1401b0340(param_2);
  uVar15 = 0xffffffffffffffff;
  if (iVar2 - 1500000U < 10000) {
    iVar2 = *(int *)(lVar5 + 0x14);
    if (0 < iVar2) {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_b0,0x395);
      FUN_14019ba10(&local_res10,*puVar6,iVar2);
      if (local_b0 != 0) {
        FUN_14019f2c0(local_b0 + -0x10);
      }
      piVar10 = local_res10;
      local_res18 = (int *)0x0;
      piVar1 = local_res18;
      if ((local_res10 != (int *)0x0) && (piVar7 = local_res10 + -4, piVar7 != (int *)0x0)) {
        if (*piVar7 == -1) {
          FUN_142e52d50(0xcb,0xffffff01);
          uVar11 = uVar15;
          do {
            uVar11 = uVar11 + 1;
          } while (*(char *)((longlong)piVar10 + uVar11) != '\0');
          iVar4 = (int)uVar11;
          iVar2 = 0;
          if (0 < iVar4) {
            iVar2 = iVar4;
          }
          piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
          piVar7[1] = iVar2;
          *piVar7 = -1;
          piVar1 = piVar7 + 4;
          piVar7[2] = 0;
          *(undefined1 *)piVar1 = 0;
          local_a8 = piVar1;
          FUN_142ef7ba0(piVar1,piVar10,(longlong)iVar4);
          if (*piVar7 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar4 == -1) || (iVar4 <= piVar7[1])) {
            *piVar7 = 1;
            if (iVar4 != -1) goto LAB_1426b0634;
            uVar11 = uVar15;
            if (piVar1 == (int *)0x0) {
              uVar11 = 0;
            }
            else {
              do {
                uVar11 = uVar11 + 1;
              } while (*(char *)((longlong)piVar1 + uVar11) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar7[1],uVar11 & 0xffffffff);
            *piVar7 = 1;
LAB_1426b0634:
            *(undefined1 *)((longlong)iVar4 + (longlong)piVar1) = 0;
          }
          iVar2 = (int)uVar11;
          if ((iVar2 < 0) || (piVar7[1] + 1 <= iVar2)) {
            FUN_142e54290(0x9c,uVar11 & 0xffffffff);
          }
          piVar7[2] = iVar2;
          if (local_res18 != (int *)0x0) {
            FUN_14019f2c0(local_res18 + -4);
          }
        }
        else {
          if (*piVar7 < 1) {
            FUN_142e52dd0(0xd2);
          }
          LOCK();
          *piVar7 = *piVar7 + 1;
          UNLOCK();
          if (local_res18 != (int *)0x0) {
            FUN_14019f2c0(local_res18 + -4);
          }
          local_res18 = piVar10;
          piVar10 = local_res10;
          piVar1 = local_res18;
        }
      }
      local_res18 = piVar1;
      FUN_14269a1d0(param_1,0x28,&local_res18,0x3e9,0,0);
    }
    iVar2 = *(int *)(lVar5 + 0x10);
    if (iVar2 < 0x65) goto LAB_1426b0c4d;
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_98,0x397);
    FUN_14019ba10(&local_res10,*puVar6,iVar2);
    if (local_98 != 0) {
      FUN_14019f2c0(local_98 + -0x10);
    }
    piVar10 = local_res10;
    local_res20 = (int *)0x0;
    if ((local_res10 != (int *)0x0) && (piVar1 = local_res10 + -4, piVar1 != (int *)0x0)) {
      if (*piVar1 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar11 = uVar15;
        do {
          uVar11 = uVar11 + 1;
        } while (*(char *)((longlong)piVar10 + uVar11) != '\0');
        iVar4 = (int)uVar11;
        iVar2 = 0;
        if (0 < iVar4) {
          iVar2 = iVar4;
        }
        piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
        piVar7[1] = iVar2;
        *piVar7 = -1;
        piVar1 = piVar7 + 4;
        piVar7[2] = 0;
        *(undefined1 *)piVar1 = 0;
        local_68 = piVar1;
        FUN_142ef7ba0(piVar1,piVar10,(longlong)iVar4);
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar4 == -1) || (iVar4 <= piVar7[1])) {
          *piVar7 = 1;
          if (iVar4 != -1) goto LAB_1426b07f1;
          uVar11 = uVar15;
          if (piVar1 == (int *)0x0) {
            uVar11 = 0;
          }
          else {
            do {
              uVar11 = uVar11 + 1;
            } while (*(char *)((longlong)piVar1 + uVar11) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar7[1],uVar11 & 0xffffffff);
          *piVar7 = 1;
LAB_1426b07f1:
          *(undefined1 *)((longlong)iVar4 + (longlong)piVar1) = 0;
        }
        iVar2 = (int)uVar11;
        if ((iVar2 < 0) || (piVar7[1] + 1 <= iVar2)) {
          FUN_142e54290(0x9c,uVar11 & 0xffffffff);
        }
        piVar7[2] = iVar2;
        if (local_res20 != (int *)0x0) {
          FUN_14019f2c0(local_res20 + -4);
        }
        ppiVar12 = &local_res20;
        local_res20 = piVar1;
        goto LAB_1426b0c33;
      }
      if (*piVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      if (local_res20 != (int *)0x0) {
        FUN_14019f2c0(local_res20 + -4);
      }
      local_res20 = piVar10;
    }
    ppiVar12 = &local_res20;
    piVar10 = local_res10;
LAB_1426b0c33:
    FUN_14269a1d0(param_1,0x28,ppiVar12,0x3e9,0,0);
  }
  else {
    iVar2 = FUN_1401b0340(param_2);
    if (iVar2 - 0x170a70U < 10000) {
      iVar2 = *(int *)(lVar5 + 0x14);
      if (0 < iVar2) {
        puVar6 = (undefined8 *)FUN_1408a9e40(&local_b0,0x396);
        FUN_14019ba10(&local_res10,*puVar6,iVar2);
        if (local_b0 != 0) {
          FUN_14019f2c0(local_b0 + -0x10);
        }
        piVar10 = local_res10;
        local_d8 = (int *)0x0;
        piVar1 = local_d8;
        if ((local_res10 != (int *)0x0) && (piVar7 = local_res10 + -4, piVar7 != (int *)0x0)) {
          if (*piVar7 == -1) {
            FUN_142e52d50(0xcb,0xffffff01);
            uVar11 = uVar15;
            do {
              uVar11 = uVar11 + 1;
            } while (*(char *)((longlong)piVar10 + uVar11) != '\0');
            iVar4 = (int)uVar11;
            iVar2 = 0;
            if (0 < iVar4) {
              iVar2 = iVar4;
            }
            piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
            piVar7[1] = iVar2;
            *piVar7 = -1;
            piVar1 = piVar7 + 4;
            piVar7[2] = 0;
            *(undefined1 *)piVar1 = 0;
            local_a8 = piVar1;
            FUN_142ef7ba0(piVar1,piVar10,(longlong)iVar4);
            if (*piVar7 != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar4 == -1) || (iVar4 <= piVar7[1])) {
              *piVar7 = 1;
              if (iVar4 != -1) goto LAB_1426b09bd;
              uVar11 = uVar15;
              if (piVar1 == (int *)0x0) {
                uVar11 = 0;
              }
              else {
                do {
                  uVar11 = uVar11 + 1;
                } while (*(char *)((longlong)piVar1 + uVar11) != '\0');
              }
            }
            else {
              FUN_142e54290(0x90,piVar7[1],uVar11 & 0xffffffff);
              *piVar7 = 1;
LAB_1426b09bd:
              *(undefined1 *)((longlong)iVar4 + (longlong)piVar1) = 0;
            }
            iVar2 = (int)uVar11;
            if ((iVar2 < 0) || (piVar7[1] + 1 <= iVar2)) {
              FUN_142e54290(0x9c,uVar11 & 0xffffffff);
            }
            piVar7[2] = iVar2;
            if (local_d8 != (int *)0x0) {
              FUN_14019f2c0(local_d8 + -4);
            }
          }
          else {
            if (*piVar7 < 1) {
              FUN_142e52dd0(0xd2);
            }
            LOCK();
            *piVar7 = *piVar7 + 1;
            UNLOCK();
            if (local_d8 != (int *)0x0) {
              FUN_14019f2c0(local_d8 + -4);
            }
            local_d8 = piVar10;
            piVar10 = local_res10;
            piVar1 = local_d8;
          }
        }
        local_d8 = piVar1;
        FUN_14269a1d0(param_1,0x28,&local_d8,0x3e9,0,0);
      }
      iVar2 = *(int *)(lVar5 + 0x10);
      if (100 < iVar2) {
        puVar6 = (undefined8 *)FUN_1408a9e40(&local_90,0x398);
        FUN_14019ba10(&local_res10,*puVar6,iVar2);
        if (local_90 != 0) {
          FUN_14019f2c0(local_90 + -0x10);
        }
        piVar10 = local_res10;
        local_f8 = (int *)0x0;
        piVar1 = local_f8;
        if ((local_res10 != (int *)0x0) && (piVar7 = local_res10 + -4, piVar7 != (int *)0x0)) {
          if (*piVar7 == -1) {
            FUN_142e52d50(0xcb,0xffffff01);
            uVar11 = uVar15;
            do {
              uVar11 = uVar11 + 1;
            } while (*(char *)((longlong)piVar10 + uVar11) != '\0');
            iVar4 = (int)uVar11;
            iVar2 = 0;
            if (0 < iVar4) {
              iVar2 = iVar4;
            }
            piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
            piVar7[1] = iVar2;
            *piVar7 = -1;
            piVar1 = piVar7 + 4;
            piVar7[2] = 0;
            *(undefined1 *)piVar1 = 0;
            local_60 = piVar1;
            FUN_142ef7ba0(piVar1,piVar10,(longlong)iVar4);
            if (*piVar7 != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar4 == -1) || (iVar4 <= piVar7[1])) {
              *piVar7 = 1;
              if (iVar4 != -1) goto LAB_1426b0b7e;
              uVar11 = uVar15;
              if (piVar1 == (int *)0x0) {
                uVar11 = 0;
              }
              else {
                do {
                  uVar11 = uVar11 + 1;
                } while (*(char *)((longlong)piVar1 + uVar11) != '\0');
              }
            }
            else {
              FUN_142e54290(0x90,piVar7[1],uVar11 & 0xffffffff);
              *piVar7 = 1;
LAB_1426b0b7e:
              *(undefined1 *)((longlong)iVar4 + (longlong)piVar1) = 0;
            }
            iVar2 = (int)uVar11;
            if ((iVar2 < 0) || (piVar7[1] + 1 <= iVar2)) {
              FUN_142e54290(0x9c,uVar11 & 0xffffffff);
            }
            piVar7[2] = iVar2;
            if (local_f8 != (int *)0x0) {
              FUN_14019f2c0(local_f8 + -4);
            }
          }
          else {
            if (*piVar7 < 1) {
              FUN_142e52dd0(0xd2);
            }
            LOCK();
            *piVar7 = *piVar7 + 1;
            UNLOCK();
            if (local_f8 != (int *)0x0) {
              FUN_14019f2c0(local_f8 + -4);
            }
            local_f8 = piVar10;
            piVar10 = local_res10;
            piVar1 = local_f8;
          }
        }
        local_f8 = piVar1;
        ppiVar12 = &local_f8;
        goto LAB_1426b0c33;
      }
    }
  }
LAB_1426b0c4d:
  iVar2 = *(int *)(lVar5 + 0x20);
  if ((0 < iVar2) && (iVar4 = *(int *)(lVar5 + 0x1c), 0 < iVar4)) {
    local_d0 = (int *)0x0;
    if (iVar4 < 100) {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x399);
      FUN_14019ba10(&local_d0,*puVar6,iVar4,iVar2);
    }
    else {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_80,0x39a);
      FUN_14019ba10(&local_d0,*puVar6,iVar2);
      local_88 = local_80;
    }
    if (local_88 != 0) {
      FUN_14019f2c0(local_88 + -0x10);
    }
    piVar7 = local_d0;
    local_f0 = (int *)0x0;
    piVar1 = local_f0;
    if ((local_d0 != (int *)0x0) && (piVar8 = local_d0 + -4, piVar8 != (int *)0x0)) {
      if (*piVar8 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar11 = uVar15;
        do {
          uVar11 = uVar11 + 1;
        } while (*(char *)((longlong)piVar7 + uVar11) != '\0');
        iVar4 = (int)uVar11;
        iVar2 = 0;
        if (0 < iVar4) {
          iVar2 = iVar4;
        }
        piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
        piVar8[1] = iVar2;
        *piVar8 = -1;
        piVar1 = piVar8 + 4;
        piVar8[2] = 0;
        *(undefined1 *)piVar1 = 0;
        local_58 = piVar1;
        FUN_142ef7ba0(piVar1,piVar7,(longlong)iVar4);
        if (*piVar8 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
          *piVar8 = 1;
          if (iVar4 != -1) goto LAB_1426b0d92;
          uVar11 = uVar15;
          if (piVar1 == (int *)0x0) {
            uVar11 = 0;
          }
          else {
            do {
              uVar11 = uVar11 + 1;
            } while (*(char *)((longlong)piVar1 + uVar11) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar8[1],uVar11 & 0xffffffff);
          *piVar8 = 1;
LAB_1426b0d92:
          *(undefined1 *)((longlong)iVar4 + (longlong)piVar1) = 0;
        }
        iVar2 = (int)uVar11;
        if ((iVar2 < 0) || (piVar8[1] + 1 <= iVar2)) {
          FUN_142e54290(0x9c,uVar11 & 0xffffffff);
        }
        piVar8[2] = iVar2;
        param_2 = local_c8;
        if (local_f0 != (int *)0x0) {
          FUN_14019f2c0();
          param_2 = local_c8;
        }
      }
      else {
        if (*piVar8 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar8 = *piVar8 + 1;
        UNLOCK();
        if (local_f0 != (int *)0x0) {
          FUN_14019f2c0(local_f0 + -4);
        }
        local_f0 = piVar7;
        piVar10 = local_res10;
        piVar7 = local_d0;
        piVar1 = local_f0;
      }
    }
    local_f0 = piVar1;
    FUN_14269a1d0(param_1,0x28,&local_f0,0x3e9,0,0);
    if (piVar7 != (int *)0x0) {
      FUN_14019f2c0(piVar7 + -4);
    }
  }
  if (*(int *)(lVar5 + 0x18) < 1) goto LAB_1426b1306;
  iVar2 = FUN_1401b0340(param_2);
  if (iVar2 - 1500000U < 10000) {
    iVar4 = FUN_1428de230(DAT_143aa8518,92000000,0);
    iVar2 = *(int *)(lVar5 + 0x18);
    local_a0 = 0;
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_78,0x39b);
    uVar9 = FUN_14019ba10(&local_a0,*puVar6,iVar2);
    local_c0 = (int *)0x0;
    FUN_14019a260(&local_c0,uVar9);
    if (local_78 != 0) {
      FUN_14019f2c0(local_78 + -0x10);
    }
    if (local_a0 != 0) {
      FUN_14019f2c0(local_a0 + -0x10);
    }
    piVar7 = local_c0;
    local_e8 = (int *)0x0;
    piVar1 = local_e8;
    if ((local_c0 != (int *)0x0) && (piVar8 = local_c0 + -4, piVar8 != (int *)0x0)) {
      if (*piVar8 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar11 = uVar15;
        do {
          uVar11 = uVar11 + 1;
        } while (*(char *)((longlong)piVar7 + uVar11) != '\0');
        iVar13 = (int)uVar11;
        iVar14 = 0;
        if (0 < iVar13) {
          iVar14 = iVar13;
        }
        piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        piVar8[1] = iVar14;
        *piVar8 = -1;
        piVar1 = piVar8 + 4;
        piVar8[2] = 0;
        *(undefined1 *)piVar1 = 0;
        local_a8 = piVar1;
        FUN_142ef7ba0(piVar1,piVar7,(longlong)iVar13);
        if (*piVar8 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar13 == -1) || (iVar13 <= piVar8[1])) {
          *piVar8 = 1;
          if (iVar13 != -1) goto LAB_1426b0fd7;
          if (piVar1 == (int *)0x0) {
            uVar11 = 0;
          }
          else {
            do {
              uVar15 = uVar15 + 1;
            } while (*(char *)((longlong)piVar1 + uVar15) != '\0');
            uVar11 = uVar15 & 0xffffffff;
          }
        }
        else {
          FUN_142e54290(0x90,piVar8[1],uVar11 & 0xffffffff);
          *piVar8 = 1;
LAB_1426b0fd7:
          *(undefined1 *)((longlong)piVar1 + (longlong)iVar13) = 0;
        }
        iVar14 = (int)uVar11;
        if ((iVar14 < 0) || (piVar8[1] + 1 <= iVar14)) {
          FUN_142e54290(0x9c,uVar11 & 0xffffffff);
        }
        piVar8[2] = iVar14;
        if (local_e8 != (int *)0x0) {
          FUN_14019f2c0();
        }
      }
      else {
        if (*piVar8 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar8 = *piVar8 + 1;
        UNLOCK();
        if (local_e8 != (int *)0x0) {
          FUN_14019f2c0(local_e8 + -4);
        }
        local_e8 = piVar7;
        piVar10 = local_res10;
        piVar7 = local_c0;
        piVar1 = local_e8;
      }
    }
    local_e8 = piVar1;
    uVar3 = 0x28;
    if (iVar4 < iVar2) {
      uVar3 = 0x2b;
    }
    FUN_14269a1d0(param_1,uVar3,&local_e8,0x3e9,0,0);
  }
  else {
    iVar2 = FUN_1401b0340(param_2);
    if (9999 < iVar2 - 0x170a70U) goto LAB_1426b1306;
    iVar4 = FUN_1428de230(DAT_143aa8518,0x57bf610,0);
    iVar2 = *(int *)(lVar5 + 0x18);
    local_c8 = 0;
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_70,0x39c);
    uVar9 = FUN_14019ba10(&local_c8,*puVar6,iVar2);
    local_b8 = (int *)0x0;
    FUN_14019a260(&local_b8,uVar9);
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    if (local_c8 != 0) {
      FUN_14019f2c0(local_c8 + -0x10);
    }
    piVar7 = local_b8;
    local_e0 = (int *)0x0;
    piVar1 = local_e0;
    if ((local_b8 != (int *)0x0) && (piVar8 = local_b8 + -4, piVar8 != (int *)0x0)) {
      if (*piVar8 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar11 = uVar15;
        do {
          uVar11 = uVar11 + 1;
        } while (*(char *)((longlong)piVar7 + uVar11) != '\0');
        iVar13 = (int)uVar11;
        iVar14 = 0;
        if (0 < iVar13) {
          iVar14 = iVar13;
        }
        piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        piVar8[1] = iVar14;
        *piVar8 = -1;
        piVar1 = piVar8 + 4;
        piVar8[2] = 0;
        *(undefined1 *)piVar1 = 0;
        local_a8 = piVar1;
        FUN_142ef7ba0(piVar1,piVar7,(longlong)iVar13);
        if (*piVar8 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar13 == -1) || (iVar13 <= piVar8[1])) {
          *piVar8 = 1;
          if (iVar13 != -1) goto LAB_1426b1212;
          if (piVar1 == (int *)0x0) {
            uVar11 = 0;
          }
          else {
            do {
              uVar15 = uVar15 + 1;
            } while (*(char *)((longlong)piVar1 + uVar15) != '\0');
            uVar11 = uVar15 & 0xffffffff;
          }
        }
        else {
          FUN_142e54290(0x90,piVar8[1],uVar11 & 0xffffffff);
          *piVar8 = 1;
LAB_1426b1212:
          *(undefined1 *)((longlong)iVar13 + (longlong)piVar1) = 0;
        }
        iVar14 = (int)uVar11;
        if ((iVar14 < 0) || (piVar8[1] + 1 <= iVar14)) {
          FUN_142e54290(0x9c,uVar11 & 0xffffffff);
        }
        piVar8[2] = iVar14;
        if (local_e0 != (int *)0x0) {
          FUN_14019f2c0();
        }
      }
      else {
        if (*piVar8 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar8 = *piVar8 + 1;
        UNLOCK();
        if (local_e0 != (int *)0x0) {
          FUN_14019f2c0(local_e0 + -4);
        }
        local_e0 = piVar7;
        piVar10 = local_res10;
        piVar7 = local_b8;
        piVar1 = local_e0;
      }
    }
    local_e0 = piVar1;
    uVar3 = 0x28;
    if (iVar4 < iVar2) {
      uVar3 = 0x2b;
    }
    FUN_14269a1d0(param_1,uVar3,&local_e0,0x3e9,0,0);
  }
  if (piVar7 != (int *)0x0) {
    FUN_14019f2c0(piVar7 + -4);
  }
LAB_1426b1306:
  if (piVar10 != (int *)0x0) {
    FUN_14019f2c0(piVar10 + -4);
  }
  return;
}



//===========================================================
// FUN_1401ab420 @ 1401ab420   (179 bytes)
//===========================================================

undefined2 FUN_1401ab420(byte *param_1,int param_2)

{
  undefined8 *puVar1;
  uint uVar2;
  undefined2 local_res8;
  int local_res10 [2];
  int local_res18 [2];
  undefined8 local_res20;
  longlong local_18 [3];
  
  local_res8 = CONCAT11(param_1[3] ^ param_1[1],param_1[2] ^ *param_1);
  uVar2 = *param_1 ^ 0xbaadf00d;
  uVar2 = (uVar2 >> 5 | uVar2 << 0x1b) + (uint)param_1[2] ^ (uint)param_1[1];
  local_res18[0] = (uVar2 >> 5 | uVar2 << 0x1b) + (uint)param_1[3];
  if (local_res18[0] != param_2) {
    local_res10[0] = param_2;
    local_res20 = FUN_1418039d0(5);
    puVar1 = (undefined8 *)FUN_1401a0ed0(local_18,&local_res20,local_res18,local_res10);
    FUN_141804970(&DAT_143271f04,0x53,5,*puVar1);
    if (local_18[0] != 0) {
      FUN_14019f2c0(local_18[0] + -0x10);
    }
  }
  return local_res8;
}



//===========================================================
// FUN_1401b0050 @ 1401b0050   (142 bytes)
//===========================================================

byte FUN_1401b0050(byte *param_1,int param_2)

{
  byte bVar1;
  byte bVar2;
  undefined8 *puVar3;
  uint uVar4;
  int local_res8 [2];
  int local_res10 [2];
  undefined8 local_res18;
  longlong local_res20;
  
  bVar1 = *param_1;
  bVar2 = param_1[1];
  uVar4 = bVar1 ^ 0xbaadf00d;
  local_res8[0] = (uVar4 >> 5 | uVar4 << 0x1b) + (uint)bVar2;
  if (local_res8[0] != param_2) {
    local_res10[0] = param_2;
    local_res18 = FUN_1418039d0(5);
    puVar3 = (undefined8 *)FUN_1401a0ed0(&local_res20,&local_res18,local_res8,local_res10);
    FUN_141804970(&DAT_143271f04,0x53,5,*puVar3);
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return bVar2 ^ bVar1;
}



//===========================================================
// FUN_1401b0340 @ 1401b0340   (1057 bytes)
//===========================================================

ulonglong FUN_1401b0340(int *param_1)

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



//===========================================================
// FUN_14038abd0 @ 14038abd0   (168 bytes)
//===========================================================

undefined4 FUN_14038abd0(undefined8 param_1,int param_2)

{
  longlong lVar1;
  
  if (999999 < param_2 - 5000000U) {
    if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
      lVar1 = FUN_140388c60();
    }
    else {
      lVar1 = FUN_14039b100();
    }
    if ((lVar1 == 0) || (*(int *)(lVar1 + 0x18) == 0)) {
      if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
        lVar1 = FUN_140388c60(param_1,param_2);
      }
      else {
        lVar1 = FUN_14039b100(param_1,param_2);
      }
      if (lVar1 != 0) {
        return *(undefined4 *)(lVar1 + 0x20);
      }
    }
  }
  return 0;
}


