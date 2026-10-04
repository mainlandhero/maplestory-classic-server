// Character Info's CITIZENSHIP records (2026-10-04): the remote fill FUN_141196d70, the window
// layout FUN_141195060 (showCitizenship gate), the self fill FUN_141197a40, the panel refresh
// FUN_1411a00d0, and the three quest-510000 getters st/gr/ct. research/character-info-2026-09-18.md row 17a.

//===========================================================
// FUN_141196d70 @ 141196d70   (3269 bytes)
//===========================================================

void FUN_141196d70(longlong *param_1,int *param_2)

{
  IUnknown *pIVar1;
  int iVar2;
  int iVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  int *piVar10;
  longlong *plVar11;
  int iVar12;
  longlong *plVar13;
  longlong *plVar14;
  ulonglong uVar15;
  uint uVar16;
  longlong *plVar17;
  longlong **pplVar18;
  longlong lVar19;
  longlong *local_res18;
  longlong *local_res20;
  undefined4 local_e8;
  undefined4 uStack_e4;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined4 local_d0;
  undefined4 uStack_cc;
  undefined8 uStack_c8;
  undefined8 local_c0;
  longlong *local_b8;
  longlong *local_b0;
  undefined4 local_a8;
  undefined4 local_a4;
  longlong local_a0;
  longlong local_98;
  longlong *local_90;
  longlong *local_88;
  uint local_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  if (DAT_143aa84a0 == 0) {
    return;
  }
  iVar3 = *param_2;
  iVar2 = FUN_142cb9550();
  if (iVar2 == iVar3) {
    FUN_141197a40(param_1);
    return;
  }
  if (param_1[100] != 0) {
    FUN_140d2d420(param_1 + 99);
    iVar3 = *param_2;
  }
  lVar4 = FUN_1429b5cf0(DAT_143ac1b90,iVar3);
  if (lVar4 != 0) {
    local_res18 = (longlong *)0x0;
    FUN_141adbce0(param_1 + 0x5c,&local_a8,L"avatar",&local_res18);
    FUN_140d2d420(param_1 + 99);
    lVar5 = FUN_140d2d280(0);
    param_1[100] = lVar5;
    if (lVar5 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar5 = param_1[100];
    }
    local_res20 = &local_a0;
    puVar6 = (undefined8 *)FUN_142bf6010(param_1,&local_b0);
    uVar7 = FUN_140eca8f0(&local_a0,*puVar6);
    local_b8 = &local_98;
    puVar6 = (undefined8 *)FUN_142bf6010(param_1,&local_90);
    uVar8 = FUN_140eca980(&local_98,*puVar6);
    uVar9 = FUN_140f80130(lVar4 + 0x100);
    FUN_140f7f030(lVar5,uVar9,5,uVar8,uVar7,0,local_a8,local_a4,100,0,0,0,0,0);
    if (local_90 != (longlong *)0x0) {
      (**(code **)(*local_90 + 0x10))();
    }
    if (local_b0 != (longlong *)0x0) {
      (**(code **)(*local_b0 + 0x10))();
    }
  }
  plVar11 = (longlong *)0x0;
  *(int *)(param_1 + 0x65) = *param_2;
  FUN_14019a260(param_1 + 0x66,param_2 + 2);
  *(int *)(param_1 + 0x67) = param_2[4];
  *(int *)((longlong)param_1 + 0x33c) = param_2[5];
  *(int *)(param_1 + 0x68) = param_2[6];
  plVar14 = (longlong *)(param_2 + 8);
  local_b8 = plVar14;
  FUN_14019a260(param_1 + 0x69,plVar14);
  *(int *)(param_1 + 0x6a) = param_2[10];
  FUN_14019a260(param_1 + 0x6b,param_2 + 0xc);
  *(int *)(param_1 + 0x6c) = param_2[0xe];
  *(int *)((longlong)param_1 + 0x364) = param_2[0xf];
  *(int *)(param_1 + 0x6d) = param_2[0x10];
  *(int *)((longlong)param_1 + 0x36c) = param_2[0x11];
  *(int *)(param_1 + 0x6e) = param_2[0x12];
  FUN_1401e8780(param_1 + 0x6f,param_2 + 0x14);
  if (param_1 + 0x71 != (longlong *)(param_2 + 0x18)) {
    FUN_1411a0170(param_1 + 0x71,*(longlong *)(param_2 + 0x18),*(undefined8 *)(param_2 + 0x1a),
                  (ulonglong)local_res18 & 0xff);
  }
  plVar13 = param_1 + 0x74;
  if (plVar13 != (longlong *)(param_2 + 0x1e)) {
    lVar4 = *(longlong *)(param_2 + 0x1e);
    lVar19 = *(longlong *)(param_2 + 0x20) - lVar4;
    lVar5 = lVar19 / 6 + (lVar19 >> 0x3f);
    uVar15 = (lVar5 >> 1) - (lVar5 >> 0x3f);
    lVar5 = *plVar13;
    if ((ulonglong)((param_1[0x76] - lVar5) / 0xc) < uVar15) {
      FUN_1411a2af0(plVar13,uVar15);
      lVar5 = *plVar13;
    }
    FUN_142ef7ba0(lVar5,lVar4,lVar19);
    param_1[0x75] = lVar5 + lVar19;
  }
  iVar3 = 0;
  lVar4 = param_1[0x74];
  lVar5 = param_1[0x75] - lVar4 >> 0x3f;
  plVar13 = plVar11;
  plVar17 = plVar11;
  if ((param_1[0x75] - lVar4) / 0xc + lVar5 != lVar5) {
    do {
      iVar2 = *(int *)(lVar4 + (longlong)plVar13);
      if (iVar2 == 1) {
        *(undefined4 *)(param_1 + 0x79) = *(undefined4 *)(lVar4 + 4 + (longlong)plVar13);
        iVar2 = *(int *)(lVar4 + (longlong)plVar13);
      }
      if (iVar2 != 0) {
        *(undefined1 *)((longlong)param_1 + 0x3c4) = 0;
      }
      uVar16 = (int)plVar17 + 1;
      plVar13 = (longlong *)((longlong)plVar13 + 0xc);
      plVar17 = (longlong *)(ulonglong)uVar16;
    } while ((ulonglong)(longlong)(int)uVar16 < (ulonglong)((param_1[0x75] - lVar4) / 0xc));
  }
  if (param_1[0x61] != 0) {
    lVar4 = FUN_1402b0250(*(undefined4 *)((longlong)param_1 + 0x33c),0);
    local_res20 = (longlong *)0x0;
    plVar13 = plVar11;
    if (lVar4 != 0) {
      plVar14 = (longlong *)0xffffffffffffffff;
      do {
        plVar14 = (longlong *)((longlong)plVar14 + 1);
      } while (*(char *)(lVar4 + (longlong)plVar14) != '\0');
      iVar2 = (int)plVar14;
      if (0 < iVar2) {
        plVar13 = (longlong *)((ulonglong)plVar14 & 0xffffffff);
      }
      piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)((int)plVar13 + 0x11));
      piVar10[1] = (int)plVar13;
      *piVar10 = -1;
      plVar13 = (longlong *)(piVar10 + 4);
      piVar10[2] = 0;
      *(undefined1 *)plVar13 = 0;
      local_res20 = plVar13;
      FUN_142ef7ba0(plVar13,lVar4,(longlong)iVar2);
      if (*piVar10 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar2 == -1) || (iVar2 <= piVar10[1])) {
        *piVar10 = 1;
        if (iVar2 != -1) goto LAB_1411971a4;
        plVar14 = plVar11;
        if (plVar13 != (longlong *)0x0) {
          plVar14 = (longlong *)0xffffffffffffffff;
          do {
            plVar14 = (longlong *)((longlong)plVar14 + 1);
          } while (*(char *)((longlong)plVar13 + (longlong)plVar14) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar10[1],(ulonglong)plVar14 & 0xffffffff);
        *piVar10 = 1;
LAB_1411971a4:
        *(undefined1 *)((longlong)iVar2 + (longlong)plVar13) = 0;
      }
      iVar2 = (int)plVar14;
      if ((iVar2 < 0) || (piVar10[1] + 1 <= iVar2)) {
        FUN_142e54290(0x9c,(ulonglong)plVar14 & 0xffffffff);
      }
      piVar10[2] = iVar2;
    }
    lVar4 = param_1[0x61];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0x61];
    }
    pplVar18 = (longlong **)(lVar4 + 0x11b0);
    if (pplVar18 == &local_res20) goto LAB_1411973e4;
    plVar14 = *pplVar18;
    iVar2 = iVar3;
    if (plVar14 != (longlong *)0x0) {
      iVar2 = (int)plVar14[-1];
    }
    iVar12 = iVar3;
    if (plVar13 != (longlong *)0x0) {
      plVar11 = plVar13;
      iVar12 = (int)plVar13[-1];
    }
    if (((iVar2 == iVar12) && (iVar2 != 0)) && (plVar14 != (longlong *)0x0)) {
      if (plVar11 != (longlong *)0x0) {
        iVar2 = memcmp(plVar14,plVar11,(longlong)iVar2);
        if (iVar2 == 0) goto LAB_1411973e4;
        goto LAB_141197279;
      }
LAB_1411973d7:
      FUN_14019f2c0(plVar14 + -2);
      *pplVar18 = (longlong *)0x0;
    }
    else {
LAB_141197279:
      if ((plVar11 == (longlong *)0x0) || (plVar17 = plVar11 + -2, plVar17 == (longlong *)0x0)) {
        if (plVar14 != (longlong *)0x0) goto LAB_1411973d7;
      }
      else if ((int)*plVar17 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar15 = 0xffffffffffffffff;
        do {
          uVar15 = uVar15 + 1;
        } while (*(char *)((longlong)plVar11 + uVar15) != '\0');
        iVar2 = (int)uVar15;
        if (0 < iVar2) {
          iVar3 = iVar2;
        }
        piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
        piVar10[1] = iVar3;
        *piVar10 = -1;
        plVar14 = (longlong *)(piVar10 + 4);
        piVar10[2] = 0;
        *(undefined1 *)plVar14 = 0;
        local_88 = plVar14;
        FUN_142ef7ba0(plVar14,plVar11,(longlong)iVar2);
        if (*piVar10 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar2 == -1) || (iVar2 <= piVar10[1])) {
          *piVar10 = 1;
          if (iVar2 != -1) goto LAB_141197333;
          if (plVar14 != (longlong *)0x0) {
            uVar15 = 0xffffffffffffffff;
            do {
              uVar15 = uVar15 + 1;
            } while (*(char *)((longlong)plVar14 + uVar15) != '\0');
            goto LAB_141197338;
          }
          iVar3 = 0;
        }
        else {
          FUN_142e54290(0x90,piVar10[1],uVar15 & 0xffffffff);
          *piVar10 = 1;
LAB_141197333:
          *(undefined1 *)((longlong)plVar14 + (longlong)iVar2) = 0;
LAB_141197338:
          iVar3 = (int)uVar15;
        }
        if ((iVar3 < 0) || (piVar10[1] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,iVar3);
        }
        piVar10[2] = iVar3;
        if (*pplVar18 != (longlong *)0x0) {
          FUN_14019f2c0(*pplVar18 + -2);
        }
        *pplVar18 = plVar14;
      }
      else {
        if ((int)*plVar17 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *(int *)plVar17 = (int)*plVar17 + 1;
        UNLOCK();
        if (*pplVar18 != (longlong *)0x0) {
          FUN_14019f2c0(*pplVar18 + -2);
        }
        *pplVar18 = plVar11;
        plVar13 = local_res20;
      }
    }
LAB_1411973e4:
    plVar14 = (longlong *)0x0;
    lVar4 = param_1[0x61];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0x61];
    }
    pplVar18 = (longlong **)(lVar4 + 0x11c0);
    if (pplVar18 == &local_res20) goto LAB_1411975d1;
    plVar11 = *pplVar18;
    iVar2 = 0;
    iVar3 = iVar2;
    if (plVar11 != (longlong *)0x0) {
      iVar3 = (int)plVar11[-1];
    }
    iVar12 = iVar2;
    if (plVar13 != (longlong *)0x0) {
      plVar14 = plVar13;
      iVar12 = (int)plVar13[-1];
    }
    if (((iVar3 == iVar12) && (iVar3 != 0)) && (plVar11 != (longlong *)0x0)) {
      if (plVar14 != (longlong *)0x0) {
        iVar3 = memcmp(plVar11,plVar14,(longlong)iVar3);
        if (iVar3 == 0) goto LAB_1411975d1;
        goto LAB_141197467;
      }
LAB_1411975c4:
      FUN_14019f2c0(plVar11 + -2);
      *pplVar18 = (longlong *)0x0;
    }
    else {
LAB_141197467:
      if ((plVar14 == (longlong *)0x0) || (plVar17 = plVar14 + -2, plVar17 == (longlong *)0x0)) {
        if (plVar11 != (longlong *)0x0) goto LAB_1411975c4;
      }
      else if ((int)*plVar17 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar15 = 0xffffffffffffffff;
        do {
          uVar15 = uVar15 + 1;
        } while (*(char *)((longlong)plVar14 + uVar15) != '\0');
        iVar3 = (int)uVar15;
        if (0 < iVar3) {
          iVar2 = iVar3;
        }
        piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
        piVar10[1] = iVar2;
        *piVar10 = -1;
        plVar11 = (longlong *)(piVar10 + 4);
        piVar10[2] = 0;
        *(undefined1 *)plVar11 = 0;
        local_res18 = plVar11;
        FUN_142ef7ba0(plVar11,plVar14,(longlong)iVar3);
        if (*piVar10 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar3 == -1) || (iVar3 <= piVar10[1])) {
          *piVar10 = 1;
          if (iVar3 != -1) goto LAB_141197523;
          if (plVar11 == (longlong *)0x0) {
            uVar15 = 0;
          }
          else {
            uVar15 = 0xffffffffffffffff;
            do {
              uVar15 = uVar15 + 1;
            } while (*(char *)((longlong)plVar11 + uVar15) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar10[1],uVar15 & 0xffffffff);
          *piVar10 = 1;
LAB_141197523:
          *(undefined1 *)((longlong)plVar11 + (longlong)iVar3) = 0;
        }
        iVar3 = (int)uVar15;
        if ((iVar3 < 0) || (piVar10[1] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,uVar15 & 0xffffffff);
        }
        piVar10[2] = iVar3;
        if (*pplVar18 != (longlong *)0x0) {
          FUN_14019f2c0(*pplVar18 + -2);
        }
        *pplVar18 = plVar11;
      }
      else {
        if ((int)*plVar17 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *(int *)plVar17 = (int)*plVar17 + 1;
        UNLOCK();
        if (*pplVar18 != (longlong *)0x0) {
          FUN_14019f2c0(*pplVar18 + -2);
        }
        *pplVar18 = plVar14;
        plVar13 = local_res20;
      }
    }
LAB_1411975d1:
    plVar14 = local_b8;
    if (plVar13 != (longlong *)0x0) {
      FUN_14019f2c0(plVar13 + -2);
      plVar14 = local_b8;
    }
  }
  FUN_14019a260(param_1 + 0x77,param_1 + 0x66);
  pIVar1 = (IUnknown *)param_1[0x5e];
  if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_e8);
  if (DAT_143a8b8d8 == 8) {
    if ((short)local_e8 == 8) {
      local_e8 = (uint)local_e8._2_2_ << 0x10;
      if (uStack_e0 != 0) {
        (*DAT_143ad5990)(uStack_e0 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_e8);
      if (iVar3 < 0) goto LAB_141197a1e;
    }
    local_e8 = CONCAT22(local_e8._2_2_,8);
    if (DAT_143a8b8e0 == 0) {
      uVar16 = 0;
    }
    else {
      uVar16 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    uStack_e0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar16);
  }
  else {
    if (((short)local_e8 == 8) && (local_e8 = (uint)local_e8._2_2_ << 0x10, uStack_e0 != 0)) {
      (*DAT_143ad5990)(uStack_e0 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_e8,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_141197a1e:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  plVar11 = (longlong *)FUN_1401a5780(&local_b8,param_1[0x77]);
  local_res18 = (longlong *)((ulonglong)local_res18 & 0xffffffff00000000);
  if ((undefined8 *)*plVar11 == (undefined8 *)0x0) {
    uVar7 = 0;
  }
  else {
    uVar7 = *(undefined8 *)*plVar11;
  }
  local_78 = local_e8;
  uStack_74 = uStack_e4;
  uStack_70 = (undefined4)uStack_e0;
  uStack_6c = uStack_e0._4_4_;
  local_68 = local_d8;
  local_res20 = plVar11;
  iVar3 = (**(code **)(*(longlong *)pIVar1 + 0xb0))(pIVar1,uVar7,&local_78,&local_res18);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143297250);
  }
  iVar3 = (int)local_res18;
  thunk_FUN_1401be120(plVar11);
  if ((short)local_e8 == 8) {
    local_e8 = local_e8 & 0xffff0000;
    if (uStack_e0 != 0) {
      (*DAT_143ad5990)(uStack_e0 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_e8);
  }
  if (0x54 < iVar3) {
    local_res18 = (longlong *)param_1[0x5e];
    if (local_res18 != (longlong *)0x0) {
      (**(code **)(*local_res18 + 8))();
    }
    FUN_1429eb300(param_1 + 0x77,&local_res18,0x54);
    pIVar1 = (IUnknown *)param_1[0x5e];
    if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_d0);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_d0 == 8) {
        local_d0 = (uint)local_d0._2_2_ << 0x10;
        if (uStack_c8 != 0) {
          (*DAT_143ad5990)(uStack_c8 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_d0);
        if (iVar3 < 0) goto LAB_141197a26;
      }
      local_d0 = CONCAT22(local_d0._2_2_,8);
      if (DAT_143a8b8e0 == 0) {
        uVar16 = 0;
      }
      else {
        uVar16 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      uStack_c8 = FUN_1401a5fa0(DAT_143a8b8e0,uVar16);
    }
    else {
      if (((short)local_d0 == 8) && (local_d0 = (uint)local_d0._2_2_ << 0x10, uStack_c8 != 0)) {
        (*DAT_143ad5990)(uStack_c8 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_d0,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_141197a26:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    plVar11 = (longlong *)FUN_1401a5780(&local_b0,param_1[0x77]);
    local_res18 = (longlong *)((ulonglong)local_res18 & 0xffffffff00000000);
    if ((undefined8 *)*plVar11 == (undefined8 *)0x0) {
      uVar7 = 0;
    }
    else {
      uVar7 = *(undefined8 *)*plVar11;
    }
    local_58 = local_d0;
    uStack_54 = uStack_cc;
    uStack_50 = (undefined4)uStack_c8;
    uStack_4c = uStack_c8._4_4_;
    local_48 = local_c0;
    local_res20 = plVar11;
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0xb0))(pIVar1,uVar7,&local_58,&local_res18);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143297250);
    }
    iVar3 = (int)local_res18;
    thunk_FUN_1401be120(plVar11);
    if ((short)local_d0 == 8) {
      local_d0 = local_d0 & 0xffff0000;
      if (uStack_c8 != 0) {
        (*DAT_143ad5990)(uStack_c8 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_d0);
    }
  }
  *(int *)(param_1 + 0x78) = -(iVar3 / 2);
  if ((*plVar14 == 0) || (*(int *)(*plVar14 + -8) < 1)) {
    piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x12);
    piVar10[1] = 1;
    *piVar10 = -1;
    plVar14 = (longlong *)(piVar10 + 4);
    piVar10[2] = 0;
    *(undefined1 *)plVar14 = 0;
    *(undefined1 *)plVar14 = DAT_143271f04;
    local_res18 = plVar14;
    if (*piVar10 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar10[1] < 1) {
      FUN_142e54290(0x90,piVar10[1],1);
    }
    *piVar10 = 1;
    *(undefined1 *)((longlong)piVar10 + 0x11) = 0;
    if (piVar10[1] + 1 < 2) {
      FUN_142e54290(0x9c,1);
    }
    piVar10[2] = 1;
    if (param_1[0x69] != 0) {
      FUN_14019f2c0(param_1[0x69] + -0x10);
    }
    param_1[0x69] = (longlong)plVar14;
  }
  *(undefined1 *)(param_1 + 0x62) = 0;
  if (DAT_143acaa00 != 0) {
    FUN_14119c3c0();
  }
  if (DAT_143aca9f8 != 0) {
    FUN_14119a960();
  }
  if (DAT_143acaa08 != 0) {
    FUN_1411a00d0();
  }
  (**(code **)(*param_1 + 0x90))(param_1,0);
  return;
}



//===========================================================
// FUN_141195060 @ 141195060   (4172 bytes)
//===========================================================

void FUN_141195060(longlong param_1)

{
  longlong lVar1;
  IUnknown *pIVar2;
  longlong lVar3;
  bool bVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  wchar_t *pwVar7;
  char cVar8;
  int iVar9;
  int iVar10;
  undefined4 uVar11;
  int *piVar12;
  undefined8 uVar13;
  longlong *plVar14;
  uint uVar15;
  uint uVar16;
  uint uVar17;
  undefined8 local_res18;
  wchar_t *local_res20;
  undefined8 in_stack_ffffffffffffff18;
  wchar_t *local_c8;
  longlong *local_c0;
  undefined8 local_b8;
  longlong *local_b0;
  undefined8 local_a8;
  longlong lStack_a0;
  undefined8 local_98;
  undefined4 local_88;
  undefined4 uStack_84;
  longlong lStack_80;
  undefined8 local_78;
  longlong local_68;
  IUnknown *local_60;
  longlong *local_58;
  wchar_t *local_50;
  wchar_t *local_48;
  longlong *local_40;
  
  uVar11 = (undefined4)((ulonglong)in_stack_ffffffffffffff18 >> 0x20);
  uVar16 = 0;
  local_res18 = (IUnknown *)((ulonglong)local_res18 & 0xffffffff00000000);
  FUN_142bf7e40();
  FUN_142bf6230(param_1,&local_50);
  if (local_50 == (wchar_t *)0x0) goto LAB_141196052;
  FUN_14090ead0(&local_48,DAT_143acaa30);
  if (local_48 != (wchar_t *)0x0) {
    local_res20 = local_48;
    (**(code **)(*(longlong *)local_48 + 8))();
    FUN_14090f750(&local_60,&local_res20,L"backgrnd_detail");
    local_res18 = local_60;
    if (local_60 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_60 + 8))();
    }
    FUN_142aa2ae0(&local_a8,&local_res18);
    pIVar2 = local_60;
    if (local_60 == (IUnknown *)0x0) {
LAB_141196094:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res18 = (IUnknown *)((ulonglong)local_res18 & 0xffffffff00000000);
    iVar9 = (**(code **)(*(longlong *)local_60 + 0x108))(local_60,&local_res18);
    if (iVar9 < 0) {
      _com_issue_errorex(iVar9,pIVar2,(_GUID *)&DAT_14327ac98);
    }
    pIVar2 = local_60;
    iVar9 = -(int)local_res18;
    if (local_60 == (IUnknown *)0x0) goto LAB_141196094;
    local_res18 = (IUnknown *)((ulonglong)local_res18 & 0xffffffff00000000);
    iVar10 = (**(code **)(*(longlong *)local_60 + 0xf8))(local_60,&local_res18);
    if (iVar10 < 0) {
      _com_issue_errorex(iVar10,pIVar2,(_GUID *)&DAT_14327ac98);
    }
    iVar10 = -(int)local_res18;
    local_b8 = local_60;
    if (local_60 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_60 + 8))();
    }
    local_c8 = local_50;
    if (local_50 != (wchar_t *)0x0) {
      (**(code **)(*(longlong *)local_50 + 8))();
    }
    FUN_142aa1590(&local_c8,&local_b8,iVar10,iVar9,CONCAT44(uVar11,0xff));
    pwVar7 = local_50;
    local_res20 = local_50;
    if (local_50 != (wchar_t *)0x0) {
      (**(code **)(*(longlong *)local_50 + 8))(local_50);
    }
    local_res18 = (IUnknown *)CONCAT44(local_res18._4_4_,*(undefined4 *)(param_1 + 0x3c0));
    local_b0 = &local_b8;
    local_b8 = (IUnknown *)0x0;
    FUN_14019a260(&local_b8,param_1 + 0x3b8);
    local_c8 = (wchar_t *)0x0;
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,0x22);
    piVar12[1] = 8;
    *piVar12 = -1;
    local_c8 = (wchar_t *)(piVar12 + 4);
    piVar12[2] = 0;
    *local_c8 = L'\0';
    uVar6 = u_charName_14337fa78._12_4_;
    uVar5 = u_charName_14337fa78._8_4_;
    uVar11 = u_charName_14337fa78._4_4_;
    *(undefined4 *)local_c8 = u_charName_14337fa78._0_4_;
    piVar12[5] = uVar11;
    piVar12[6] = uVar5;
    piVar12[7] = uVar6;
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 8) {
      FUN_142e54290(0x90,piVar12[1],8);
    }
    *piVar12 = 1;
    local_c8[8] = L'\0';
    if (piVar12[1] + 1 < 9) {
      FUN_142e54290(0x9c,8);
    }
    piVar12[2] = 0x10;
    local_c0 = *(longlong **)(param_1 + 0x2f0);
    if (local_c0 != (longlong *)0x0) {
      (**(code **)(*local_c0 + 8))();
    }
    lVar1 = param_1 + 0x2e0;
    FUN_1411a0e00(&local_res20,lVar1,&local_c0,&local_c8,&local_b8,0,(int)local_res18);
    local_b0 = &local_b8;
    local_c8 = (wchar_t *)0x0;
    uVar13 = FUN_14019ba10(&local_c8,&DAT_143274298,*(undefined4 *)(param_1 + 0x338));
    local_b8 = (IUnknown *)0x0;
    FUN_14019a260(&local_b8,uVar13);
    local_res18 = (IUnknown *)0x0;
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,0x1c);
    piVar12[1] = 5;
    *piVar12 = -1;
    local_res18 = (IUnknown *)(piVar12 + 4);
    piVar12[2] = 0;
    *(wchar_t *)local_res18 = L'\0';
    *(undefined8 *)local_res18 = u_level_1432830d8._0_8_;
    *(wchar_t *)(piVar12 + 6) = u_level_1432830d8[4];
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 5) {
      FUN_142e54290(0x90,piVar12[1],5);
    }
    *piVar12 = 1;
    *(wchar_t *)((longlong)local_res18 + 10) = L'\0';
    if (piVar12[1] + 1 < 6) {
      FUN_142e54290(0x9c,5);
    }
    piVar12[2] = 10;
    local_c0 = *(longlong **)(param_1 + 0x2e8);
    if (local_c0 != (longlong *)0x0) {
      (**(code **)(*local_c0 + 8))();
    }
    FUN_1411a0e00(&local_res20,lVar1,&local_c0,&local_res18,&local_b8,0,0);
    if (local_c8 != (wchar_t *)0x0) {
      FUN_14019f2c0(local_c8 + -8);
    }
    local_b0 = &local_b8;
    local_c8 = (wchar_t *)0x0;
    uVar13 = FUN_14019ba10(&local_c8,&DAT_143274298,*(undefined4 *)(param_1 + 0x340));
    local_b8 = (IUnknown *)0x0;
    FUN_14019a260(&local_b8,uVar13);
    local_res18 = (IUnknown *)0x0;
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,0x18);
    piVar12[1] = 3;
    *piVar12 = -1;
    local_res18 = (IUnknown *)(piVar12 + 4);
    piVar12[2] = 0;
    *(undefined2 *)local_res18 = 0;
    *(int *)local_res18 = DAT_1432993e8;
    *(undefined2 *)(piVar12 + 5) = DAT_1432993ec;
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 3) {
      FUN_142e54290(0x90,piVar12[1],3);
    }
    *piVar12 = 1;
    *(undefined2 *)((longlong)local_res18 + 6) = 0;
    if (piVar12[1] + 1 < 4) {
      FUN_142e54290(0x9c,3);
    }
    piVar12[2] = 6;
    local_c0 = *(longlong **)(param_1 + 0x2e8);
    if (local_c0 != (longlong *)0x0) {
      (**(code **)(*local_c0 + 8))();
    }
    FUN_1411a0e00(&local_res20,lVar1,&local_c0,&local_res18,&local_b8,0,0);
    if (local_c8 != (wchar_t *)0x0) {
      FUN_14019f2c0(local_c8 + -8);
    }
    local_b0 = &local_b8;
    local_b8 = (IUnknown *)0x0;
    FUN_14019a260(&local_b8,param_1 + 0x348);
    local_res18 = (IUnknown *)0x0;
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,0x24);
    piVar12[1] = 9;
    *piVar12 = -1;
    local_res18 = (IUnknown *)(piVar12 + 4);
    piVar12[2] = 0;
    *(wchar_t *)local_res18 = L'\0';
    uVar6 = u_guildName_14338a8b0._12_4_;
    uVar5 = u_guildName_14338a8b0._8_4_;
    uVar11 = u_guildName_14338a8b0._4_4_;
    *(undefined4 *)local_res18 = u_guildName_14338a8b0._0_4_;
    piVar12[5] = uVar11;
    piVar12[6] = uVar5;
    piVar12[7] = uVar6;
    *(wchar_t *)(piVar12 + 8) = u_guildName_14338a8b0[8];
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 9) {
      FUN_142e54290(0x90,piVar12[1],9);
    }
    *piVar12 = 1;
    *(wchar_t *)((longlong)local_res18 + 0x12) = L'\0';
    if (piVar12[1] + 1 < 10) {
      FUN_142e54290(0x9c,9);
    }
    piVar12[2] = 0x12;
    local_c0 = *(longlong **)(param_1 + 0x2e8);
    if (local_c0 != (longlong *)0x0) {
      (**(code **)(*local_c0 + 8))();
    }
    FUN_1411a0e00(&local_res20,lVar1,&local_c0,&local_res18,&local_b8,0,0);
    FUN_1429fbeb0(&local_58,0x26);
    uVar13 = FUN_1408a9e40(&local_68,0x1800);
    FUN_1429fa1b0(&local_40,0xffe5a7c1,0xc,0,uVar13);
    if (local_68 != 0) {
      FUN_14019f2c0(local_68 + -0x10);
    }
    local_res18 = (IUnknown *)local_48;
    if (local_48 != (wchar_t *)0x0) {
      (**(code **)(*(longlong *)local_48 + 8))();
    }
    uVar11 = FUN_140910eb0(&local_res18,L"labelOffset",0xffffffd5);
    local_res18 = (IUnknown *)CONCAT44(local_res18._4_4_,uVar11);
    local_a8 = (longlong **)&local_b8;
    local_b0 = (longlong *)FUN_1408a9e40(&local_b8,0x17f1);
    local_c8 = (wchar_t *)0x0;
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,0x1c);
    piVar12[1] = 5;
    *piVar12 = -1;
    local_c8 = (wchar_t *)(piVar12 + 4);
    piVar12[2] = 0;
    *local_c8 = L'\0';
    *(undefined8 *)local_c8 = u_level_1432830d8._0_8_;
    *(wchar_t *)(piVar12 + 6) = u_level_1432830d8[4];
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 5) {
      FUN_142e54290(0x90,piVar12[1],5);
    }
    *piVar12 = 1;
    local_c8[5] = L'\0';
    if (piVar12[1] + 1 < 6) {
      FUN_142e54290(0x9c,5);
    }
    piVar12[2] = 10;
    local_c0 = local_58;
    if (local_58 != (longlong *)0x0) {
      (**(code **)(*local_58 + 8))();
    }
    FUN_1411a0e00(&local_res20,lVar1,&local_c0,&local_c8,local_b0,1,(int)local_res18);
    local_a8 = &local_b0;
    local_b8 = (IUnknown *)FUN_1408a9e40(&local_b0,0x17f2);
    local_c8 = (wchar_t *)0x0;
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,0x20);
    piVar12[1] = 7;
    *piVar12 = -1;
    local_c8 = (wchar_t *)(piVar12 + 4);
    piVar12[2] = 0;
    *local_c8 = L'\0';
    *(undefined8 *)local_c8 = u_jobName_14337fa68._0_8_;
    piVar12[6] = u_jobName_14337fa68._8_4_;
    *(wchar_t *)(piVar12 + 7) = u_jobName_14337fa68[6];
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 7) {
      FUN_142e54290(0x90,piVar12[1],7);
    }
    *piVar12 = 1;
    local_c8[7] = L'\0';
    if (piVar12[1] + 1 < 8) {
      FUN_142e54290(0x9c,7);
    }
    piVar12[2] = 0xe;
    local_c0 = local_58;
    if (local_58 != (longlong *)0x0) {
      (**(code **)(*local_58 + 8))();
    }
    FUN_1411a0e00(&local_res20,lVar1,&local_c0,&local_c8,local_b8,1,(int)local_res18);
    local_a8 = &local_b0;
    local_b8 = (IUnknown *)FUN_1408a9e40(&local_b0,0x17f3);
    local_c8 = (wchar_t *)0x0;
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,0x18);
    piVar12[1] = 3;
    *piVar12 = -1;
    local_c8 = (wchar_t *)(piVar12 + 4);
    piVar12[2] = 0;
    *local_c8 = L'\0';
    *(int *)local_c8 = DAT_1432993e8;
    *(undefined2 *)(piVar12 + 5) = DAT_1432993ec;
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 3) {
      FUN_142e54290(0x90,piVar12[1],3);
    }
    *piVar12 = 1;
    local_c8[3] = L'\0';
    if (piVar12[1] + 1 < 4) {
      FUN_142e54290(0x9c,3);
    }
    piVar12[2] = 6;
    local_c0 = local_58;
    if (local_58 != (longlong *)0x0) {
      (**(code **)(*local_58 + 8))();
    }
    FUN_1411a0e00(&local_res20,lVar1,&local_c0,&local_c8,local_b8,1,(int)local_res18);
    local_a8 = &local_b0;
    local_b8 = (IUnknown *)FUN_1408a9e40(&local_b0,0x17f4);
    local_c8 = (wchar_t *)0x0;
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,0x24);
    piVar12[1] = 9;
    *piVar12 = -1;
    local_c8 = (wchar_t *)(piVar12 + 4);
    piVar12[2] = 0;
    *local_c8 = L'\0';
    uVar6 = u_guildName_14338a8b0._12_4_;
    uVar5 = u_guildName_14338a8b0._8_4_;
    uVar11 = u_guildName_14338a8b0._4_4_;
    *(undefined4 *)local_c8 = u_guildName_14338a8b0._0_4_;
    piVar12[5] = uVar11;
    piVar12[6] = uVar5;
    piVar12[7] = uVar6;
    *(wchar_t *)(piVar12 + 8) = u_guildName_14338a8b0[8];
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 9) {
      FUN_142e54290(0x90,piVar12[1],9);
    }
    *piVar12 = 1;
    local_c8[9] = L'\0';
    if (piVar12[1] + 1 < 10) {
      FUN_142e54290(0x9c,9);
    }
    piVar12[2] = 0x12;
    local_c0 = local_40;
    if (local_40 != (longlong *)0x0) {
      (**(code **)(*local_40 + 8))();
    }
    plVar14 = (longlong *)local_b8;
    FUN_1411a0e00(&local_res20,lVar1,&local_c0,&local_c8,local_b8,1,(int)local_res18);
    uVar11 = (undefined4)((ulonglong)plVar14 >> 0x20);
    pIVar2 = *(IUnknown **)(param_1 + 0x2f8);
    plVar14 = (longlong *)local_res18;
    uVar15 = uVar16;
    if (pIVar2 == (IUnknown *)0x0) {
LAB_141195bfb:
      bVar4 = false;
    }
    else {
      (*DAT_143262a20)(&local_a8);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_a8 == 8) {
          local_a8 = (longlong **)((ulonglong)local_a8._2_6_ << 0x10);
          if (lStack_a0 != 0) {
            (*DAT_143ad5990)(lStack_a0 + -4);
          }
        }
        else {
          iVar9 = (*DAT_143262a18)(&local_a8);
          if (iVar9 < 0) goto LAB_141196084;
        }
        local_a8 = (longlong **)CONCAT62(local_a8._2_6_,8);
        if (DAT_143a8b8e0 != 0) {
          uVar15 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        lStack_a0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar15);
      }
      else {
        if (((short)local_a8 == 8) &&
           (local_a8 = (longlong **)((ulonglong)local_a8._2_6_ << 0x10), lStack_a0 != 0)) {
          (*DAT_143ad5990)(lStack_a0 + -4);
        }
        iVar9 = (*DAT_143262a28)(&local_a8,&DAT_143a8b8d8);
        if (iVar9 < 0) {
LAB_141196084:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar9);
        }
      }
      local_res18 = (IUnknown *)CONCAT44(local_res18._4_4_,1);
      local_c0 = (longlong *)0x0;
      local_88 = (uint)local_a8;
      uStack_84 = local_a8._4_4_;
      lStack_80 = lStack_a0;
      local_78 = local_98;
      iVar9 = (**(code **)(*(longlong *)pIVar2 + 0x240))(pIVar2,&local_88,&local_c0);
      if (iVar9 < 0) {
        _com_issue_errorex(iVar9,pIVar2,(_GUID *)&DAT_14327fcb0);
      }
      uVar15 = 7;
      plVar14 = local_c0;
      if (local_c0 == (longlong *)0x0) goto LAB_141195bfb;
      bVar4 = true;
    }
    uVar17 = uVar15;
    if ((uVar15 & 2) != 0) {
      uVar17 = uVar15 & 0xfffffffd;
      local_res18 = (IUnknown *)(CONCAT44(local_res18._4_4_,uVar15) & 0xfffffffffffffffd);
      if (plVar14 != (longlong *)0x0) {
        (**(code **)(*plVar14 + 0x10))();
      }
    }
    if ((uVar17 & 1) != 0) {
      if ((short)local_a8 == 8) {
        local_a8 = (longlong **)((ulonglong)local_a8 & 0xffffffffffff0000);
        if (lStack_a0 != 0) {
          (*DAT_143ad5990)(lStack_a0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_a8);
      }
    }
    if (bVar4) {
      pIVar2 = *(IUnknown **)(param_1 + 0x2f8);
      if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_a8);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_a8 == 8) {
          local_a8 = (longlong **)((ulonglong)local_a8._2_6_ << 0x10);
          if (lStack_a0 != 0) {
            (*DAT_143ad5990)(lStack_a0 + -4);
          }
        }
        else {
          iVar9 = (*DAT_143262a18)(&local_a8);
          if (iVar9 < 0) goto LAB_14119609f;
        }
        local_a8 = (longlong **)CONCAT62(local_a8._2_6_,8);
        if (DAT_143a8b8e0 == 0) {
          lStack_a0 = FUN_1401a5fa0(0,0);
        }
        else {
          lStack_a0 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
      }
      else {
        if (((short)local_a8 == 8) &&
           (local_a8 = (longlong **)((ulonglong)local_a8._2_6_ << 0x10), lStack_a0 != 0)) {
          (*DAT_143ad5990)(lStack_a0 + -4);
        }
        iVar9 = (*DAT_143262a28)(&local_a8,&DAT_143a8b8d8);
        if (iVar9 < 0) {
LAB_14119609f:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar9);
        }
      }
      local_c0 = (longlong *)0x0;
      local_88 = (uint)local_a8;
      uStack_84 = local_a8._4_4_;
      lStack_80 = lStack_a0;
      local_78 = local_98;
      iVar9 = (**(code **)(*(longlong *)pIVar2 + 0x240))(pIVar2,&local_88,&local_c0);
      if (iVar9 < 0) {
        _com_issue_errorex(iVar9,pIVar2,(_GUID *)&DAT_14327fcb0);
      }
      local_b8 = (IUnknown *)local_c0;
      FUN_142aa14a0(&local_b8);
      if ((short)local_a8 == 8) {
        local_a8 = (longlong **)((ulonglong)local_a8 & 0xffffffffffff0000);
        if (lStack_a0 != 0) {
          (*DAT_143ad5990)(lStack_a0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_a8);
      }
      FUN_1411a2da0(&local_res18,*(undefined4 *)(param_1 + 0x3c8));
      if (local_res18 != (IUnknown *)0x0) {
        local_c0 = (longlong *)0x0;
        FUN_141adbce0(lVar1,&local_b8,L"badge",&local_c0);
        local_a8 = &local_c0;
        local_c0 = (longlong *)local_res18;
        if (local_res18 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_res18 + 8))();
        }
        lVar3 = *(longlong *)(param_1 + 0x2f8);
        if (lVar3 == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(&local_88);
        if (DAT_143a8b8d8 == 8) {
          if ((short)local_88 == 8) {
            local_88 = (uint)local_88._2_2_ << 0x10;
            if (lStack_80 != 0) {
              (*DAT_143ad5990)(lStack_80 + -4);
            }
          }
          else {
            iVar9 = (*DAT_143262a18)(&local_88);
            if (iVar9 < 0) goto LAB_14119608c;
          }
          local_88 = CONCAT22(local_88._2_2_,8);
          uVar15 = uVar16;
          if (DAT_143a8b8e0 != 0) {
            uVar15 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
          }
          lStack_80 = FUN_1401a5fa0(DAT_143a8b8e0,uVar15);
        }
        else {
          if (((short)local_88 == 8) && (local_88 = (uint)local_88._2_2_ << 0x10, lStack_80 != 0)) {
            (*DAT_143ad5990)(lStack_80 + -4);
          }
          iVar9 = (*DAT_143262a28)(&local_88,&DAT_143a8b8d8);
          if (iVar9 < 0) {
LAB_14119608c:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar9);
          }
        }
        uVar13 = FUN_140ee7c90(lVar3,&local_b0,&local_88);
        FUN_142aa1590(uVar13,&local_c0,(ulonglong)local_b8 & 0xffffffff,local_b8._4_4_,
                      CONCAT44(uVar11,0xff));
        if ((short)local_88 == 8) {
          local_88 = local_88 & 0xffff0000;
          if (lStack_80 != 0) {
            (*DAT_143ad5990)(lStack_80 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_88);
        }
      }
      if (local_res18 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_res18 + 0x10))();
      }
    }
    if ((*(char *)(param_1 + 0x310) != '\0') || (uVar15 = 1, *(int *)(DAT_143aa84a0 + 0x23a8) != 0))
    {
      uVar15 = uVar16;
    }
    FUN_142aa2010(lVar1,L"upPop",uVar15);
    FUN_142aa2010(lVar1,L"downPop",uVar15);
    FUN_142aa2010(lVar1,L"trade",*(char *)(param_1 + 0x310) == '\0');
    uVar15 = uVar16;
    if ((*(char *)(param_1 + 0x310) == '\0') &&
       ((cVar8 = FUN_1413b8e10(), cVar8 == '\0' || (cVar8 = FUN_1413b8e30(), cVar8 != '\0')))) {
      uVar15 = 1;
    }
    FUN_142aa2010(lVar1,L"party",uVar15);
    FUN_142aa2010(lVar1,L"showPet",*(int *)(param_1 + 0x350) != 0);
    uVar15 = 1;
    if ((*(char *)(param_1 + 0x310) == '\0') && (*(char *)(param_1 + 0x3c4) != '\0')) {
      uVar15 = uVar16;
    }
    FUN_142aa2010(lVar1,L"showCitizenship",uVar15);
    FUN_142aa2150(lVar1,L"showPet",DAT_143acaa00 == 0);
    if (local_40 != (longlong *)0x0) {
      (**(code **)(*local_40 + 0x10))();
    }
    if (local_58 != (longlong *)0x0) {
      (**(code **)(*local_58 + 0x10))();
    }
    if (pwVar7 != (wchar_t *)0x0) {
      (**(code **)(*(longlong *)pwVar7 + 0x10))(pwVar7);
    }
    if (local_60 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_60 + 0x10))();
    }
  }
  if (local_48 != (wchar_t *)0x0) {
    (**(code **)(*(longlong *)local_48 + 0x10))();
  }
LAB_141196052:
  if (local_50 != (wchar_t *)0x0) {
    (**(code **)(*(longlong *)local_50 + 0x10))();
  }
  return;
}



//===========================================================
// FUN_141197a40 @ 141197a40   (4440 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001411984ae) */

void FUN_141197a40(longlong *param_1)

{
  longlong *plVar1;
  IUnknown *pIVar2;
  undefined8 *puVar3;
  byte bVar4;
  short sVar5;
  undefined4 uVar6;
  int iVar7;
  int iVar8;
  longlong lVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  int *piVar14;
  longlong *plVar15;
  int *piVar16;
  undefined8 *puVar17;
  uint uVar18;
  ulonglong uVar19;
  longlong lVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  longlong lVar23;
  longlong *plVar24;
  undefined1 *local_res10;
  undefined1 *local_res18;
  int *local_res20;
  undefined4 local_1c8;
  undefined4 uStack_1c4;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined4 local_1b0;
  undefined4 uStack_1ac;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined4 local_190;
  int *local_188;
  longlong *local_180;
  undefined1 local_178 [8];
  undefined8 *local_170;
  longlong local_168;
  undefined8 *local_160;
  undefined4 local_158;
  undefined4 local_154;
  longlong *local_150;
  undefined1 local_148 [8];
  undefined8 *local_140;
  undefined1 local_138 [8];
  undefined1 local_130 [8];
  longlong *local_128;
  longlong *local_120;
  int *local_118;
  longlong local_110;
  undefined1 local_108 [8];
  undefined1 local_100 [8];
  undefined1 local_f8 [8];
  undefined8 *local_f0;
  undefined8 *local_e0;
  int *local_d8;
  uint local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 local_b8;
  uint local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  undefined4 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined4 local_68;
  longlong local_60 [4];
  
  lVar23 = DAT_143aa84a0;
  local_198 = DAT_143aa84a0;
  if (DAT_143aa84a0 == 0) {
    return;
  }
  lVar9 = FUN_142cbe730(DAT_143aa84a0);
  if (lVar9 == 0) {
    return;
  }
  local_168 = FUN_142cbe730(lVar23);
  piVar14 = DAT_143aa8518;
  local_188 = DAT_143aa8518;
  if (DAT_143aa8518 == (int *)0x0) {
    return;
  }
  if (param_1[100] != 0) {
    FUN_140d2d420(param_1 + 99);
  }
  piVar16 = (int *)0x0;
  local_res20 = (int *)0x0;
  FUN_141adbce0(param_1 + 0x5c,&local_158,L"avatar",&local_res20);
  FUN_140d2d420(param_1 + 99);
  lVar9 = FUN_140d2d280(0);
  param_1[100] = lVar9;
  if (lVar9 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar9 = param_1[100];
  }
  local_res10 = local_138;
  puVar10 = (undefined8 *)FUN_142bf6010(param_1,&local_120);
  uVar11 = FUN_140eca8f0(local_138,*puVar10);
  local_res18 = local_130;
  puVar10 = (undefined8 *)FUN_142bf6010(param_1,&local_128);
  uVar12 = FUN_140eca980(local_130,*puVar10);
  uVar13 = FUN_140f80130(piVar14 + 0x40);
  FUN_140f7f030(lVar9,uVar13,5,uVar12,uVar11,0,local_158,local_154,100,0,0,0,0,0);
  if (local_128 != (longlong *)0x0) {
    (**(code **)(*local_128 + 0x10))();
  }
  if (local_120 != (longlong *)0x0) {
    (**(code **)(*local_120 + 0x10))();
  }
  uVar6 = FUN_14276df20(piVar14);
  *(undefined4 *)(param_1 + 0x65) = uVar6;
  lVar9 = FUN_14276df30(piVar14);
  local_118 = (int *)0x0;
  uVar22 = 0xffffffffffffffff;
  if (lVar9 != 0) {
    uVar21 = 0xffffffffffffffff;
    do {
      uVar21 = uVar21 + 1;
    } while (*(char *)(lVar9 + uVar21) != '\0');
    iVar8 = (int)uVar21;
    iVar7 = 0;
    if (0 < iVar8) {
      iVar7 = iVar8;
    }
    piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
    piVar14[1] = iVar7;
    *piVar14 = -1;
    piVar16 = piVar14 + 4;
    piVar14[2] = 0;
    *(undefined1 *)piVar16 = 0;
    local_118 = piVar16;
    FUN_142ef7ba0(piVar16,lVar9,(longlong)iVar8);
    if (*piVar14 != -1) {
      FUN_142e52dd0();
    }
    if ((iVar8 == -1) || (iVar8 <= piVar14[1])) {
      *piVar14 = 1;
      if (iVar8 != -1) goto LAB_141197c9a;
      if (piVar16 == (int *)0x0) {
        uVar21 = 0;
      }
      else {
        uVar21 = 0xffffffffffffffff;
        do {
          uVar21 = uVar21 + 1;
        } while (*(char *)((longlong)piVar16 + uVar21) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar14[1],uVar21 & 0xffffffff);
      *piVar14 = 1;
LAB_141197c9a:
      *(undefined1 *)((longlong)iVar8 + (longlong)piVar16) = 0;
    }
    iVar7 = (int)uVar21;
    if ((iVar7 < 0) || (piVar14[1] + 1 <= iVar7)) {
      FUN_142e54290(0x9c,uVar21 & 0xffffffff);
    }
    piVar14[2] = iVar7;
  }
  if (param_1[0x66] != 0) {
    FUN_14019f2c0(param_1[0x66] + -0x10);
  }
  lVar9 = local_168;
  param_1[0x66] = (longlong)piVar16;
  uVar6 = FUN_1401ba9d0(local_168 + 0x27,*(undefined4 *)(local_168 + 0x2f));
  *(undefined4 *)(param_1 + 0x67) = uVar6;
  sVar5 = FUN_1401ab420(lVar9 + 0x33,*(undefined4 *)(lVar9 + 0x37));
  *(int *)((longlong)param_1 + 0x33c) = (int)sVar5;
  uVar6 = FUN_1401ba9d0(lVar9 + 0xb3,*(undefined4 *)(lVar9 + 0xbb));
  *(undefined4 *)(param_1 + 0x68) = uVar6;
  plVar15 = (longlong *)FUN_142cc0410(lVar23,&local_110);
  if (param_1[0x69] != 0) {
    FUN_14019f2c0(param_1[0x69] + -0x10);
    param_1[0x69] = 0;
  }
  param_1[0x69] = *plVar15;
  *plVar15 = 0;
  if (local_110 != 0) {
    FUN_14019f2c0(local_110 + -0x10);
  }
  plVar15 = param_1 + 0x71;
  lVar23 = param_1[0x72];
  lVar20 = *plVar15;
  if (lVar20 != lVar23) {
    do {
      FUN_1401abd80(lVar20);
      lVar20 = lVar20 + 0x10;
    } while (lVar20 != lVar23);
    lVar20 = *plVar15;
  }
  param_1[0x72] = lVar20;
  plVar24 = (longlong *)(lVar9 + 0x1b0);
  lVar23 = 0x20;
  do {
    puVar10 = (undefined8 *)*plVar24;
    if (puVar10 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)puVar10[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      puVar10[1] = puVar10[1] + 1;
      UNLOCK();
    }
    if (puVar10 != (undefined8 *)0x0) {
      FUN_1416db0f0(local_178,puVar10);
      if (local_170 != (undefined8 *)0x0) {
        puVar17 = (undefined8 *)FUN_140193080();
        local_140 = puVar17;
        if (puVar17 != (undefined8 *)0x0) {
          if (0xfffff < (ulonglong)puVar17[1]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          puVar17[1] = puVar17[1] + 1;
          UNLOCK();
        }
        puVar3 = local_140;
        puVar17 = local_170;
        if (local_140 == (undefined8 *)0x0) {
          if (local_170 != (undefined8 *)0x0) {
            if (0xffffe < local_170[1] - 1) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar1 = puVar17 + 1;
            lVar9 = *plVar1;
            *plVar1 = *plVar1 + -1;
            UNLOCK();
            if (((int)lVar9 == 1) && (local_170 != (undefined8 *)0x0)) {
              (**(code **)*local_170)(local_170,1);
            }
            local_170 = (undefined8 *)0x0;
          }
        }
        else {
          lVar9 = param_1[0x72];
          if (lVar9 == param_1[0x73]) {
            FUN_1411a0440(plVar15,lVar9,local_148);
          }
          else {
            *(undefined8 **)(lVar9 + 8) = local_140;
            if (0xfffff < (ulonglong)local_140[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            puVar3[1] = puVar3[1] + 1;
            UNLOCK();
            param_1[0x72] = param_1[0x72] + 0x10;
          }
          puVar17 = local_140;
          if (local_140 != (undefined8 *)0x0) {
            if (0xffffe < local_140[1] - 1) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar1 = puVar17 + 1;
            lVar9 = *plVar1;
            *plVar1 = *plVar1 + -1;
            UNLOCK();
            if ((int)lVar9 == 1) {
              (**(code **)*local_140)(local_140,1);
            }
            local_140 = (undefined8 *)0x0;
          }
          puVar17 = local_170;
          if (local_170 != (undefined8 *)0x0) {
            if (0xffffe < local_170[1] - 1) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar1 = puVar17 + 1;
            lVar9 = *plVar1;
            *plVar1 = *plVar1 + -1;
            UNLOCK();
            if (((int)lVar9 == 1) && (local_170 != (undefined8 *)0x0)) {
              (**(code **)*local_170)(local_170,1);
            }
            local_170 = (undefined8 *)0x0;
          }
        }
      }
      if (puVar10 != (undefined8 *)0x0) {
        if (0xffffe < puVar10[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = puVar10 + 1;
        lVar9 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar9 == 1) {
          (**(code **)*puVar10)(puVar10,1);
        }
      }
    }
    plVar24 = plVar24 + 2;
    lVar23 = lVar23 + -1;
  } while (lVar23 != 0);
  FUN_14019a260(param_1 + 0x77,param_1 + 0x66);
  pIVar2 = (IUnknown *)param_1[0x5e];
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)();
  if (DAT_143a8b8d8 == 8) {
    if ((short)local_1c8 == 8) {
      local_1c8 = (uint)local_1c8._2_2_ << 0x10;
      if (uStack_1c0 != 0) {
        (*DAT_143ad5990)(uStack_1c0 + -4);
      }
    }
    else {
      iVar7 = (*DAT_143262a18)(&local_1c8);
      if (iVar7 < 0) goto LAB_141198b81;
    }
    local_1c8 = CONCAT22(local_1c8._2_2_,8);
    if (DAT_143a8b8e0 == 0) {
      uVar18 = 0;
    }
    else {
      uVar18 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    uStack_1c0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar18);
  }
  else {
    if (((short)local_1c8 == 8) && (local_1c8 = (uint)local_1c8._2_2_ << 0x10, uStack_1c0 != 0)) {
      (*DAT_143ad5990)(uStack_1c0 + -4);
    }
    iVar7 = (*DAT_143262a28)(&local_1c8,&DAT_143a8b8d8);
    if (iVar7 < 0) {
LAB_141198b81:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar7);
    }
  }
  plVar15 = (longlong *)FUN_1401a5780(local_108,param_1[0x77]);
  uVar21 = 0;
  local_res10 = (undefined1 *)((ulonglong)local_res10 & 0xffffffff00000000);
  uVar19 = uVar21;
  if ((ulonglong *)*plVar15 != (ulonglong *)0x0) {
    uVar19 = *(ulonglong *)*plVar15;
  }
  local_c8 = local_1c8;
  uStack_c4 = uStack_1c4;
  uStack_c0 = (undefined4)uStack_1c0;
  uStack_bc = uStack_1c0._4_4_;
  local_b8 = local_1b8;
  local_180 = plVar15;
  iVar7 = (**(code **)(*(longlong *)pIVar2 + 0xb0))(pIVar2,uVar19,&local_c8,&local_res10);
  if (iVar7 < 0) {
    _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_143297250);
  }
  iVar7 = (int)local_res10;
  thunk_FUN_1401be120(plVar15);
  if ((short)local_1c8 == 8) {
    local_1c8 = local_1c8 & 0xffff0000;
    if (uStack_1c0 != 0) {
      (*DAT_143ad5990)(uStack_1c0 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1c8);
  }
  if (0x54 < iVar7) {
    local_150 = (longlong *)param_1[0x5e];
    if (local_150 != (longlong *)0x0) {
      (**(code **)(*local_150 + 8))();
    }
    FUN_1429eb300(param_1 + 0x77,&local_150,0x54);
    pIVar2 = (IUnknown *)param_1[0x5e];
    if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_1b0);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_1b0 == 8) {
        local_1b0 = (uint)local_1b0._2_2_ << 0x10;
        if (uStack_1a8 != 0) {
          (*DAT_143ad5990)(uStack_1a8 + -4);
        }
      }
      else {
        iVar7 = (*DAT_143262a18)(&local_1b0);
        if (iVar7 < 0) goto LAB_141198b89;
      }
      local_1b0 = CONCAT22(local_1b0._2_2_,8);
      uVar19 = uVar21;
      if (DAT_143a8b8e0 != 0) {
        uVar19 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_1a8 = FUN_1401a5fa0(DAT_143a8b8e0,uVar19);
    }
    else {
      if (((short)local_1b0 == 8) && (local_1b0 = (uint)local_1b0._2_2_ << 0x10, uStack_1a8 != 0)) {
        (*DAT_143ad5990)(uStack_1a8 + -4);
      }
      iVar7 = (*DAT_143262a28)(&local_1b0,&DAT_143a8b8d8);
      if (iVar7 < 0) {
LAB_141198b89:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
    }
    plVar15 = (longlong *)FUN_1401a5780(local_100,param_1[0x77]);
    local_res18 = (undefined1 *)((ulonglong)local_res18 & 0xffffffff00000000);
    if ((ulonglong *)*plVar15 != (ulonglong *)0x0) {
      uVar21 = *(ulonglong *)*plVar15;
    }
    local_a8 = local_1b0;
    uStack_a4 = uStack_1ac;
    uStack_a0 = (undefined4)uStack_1a8;
    uStack_9c = uStack_1a8._4_4_;
    local_98 = local_1a0;
    local_180 = plVar15;
    iVar7 = (**(code **)(*(longlong *)pIVar2 + 0xb0))(pIVar2,uVar21,&local_a8,&local_res18);
    if (iVar7 < 0) {
      _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_143297250);
    }
    iVar7 = (int)local_res18;
    thunk_FUN_1401be120(plVar15);
    if ((short)local_1b0 == 8) {
      local_1b0 = local_1b0 & 0xffff0000;
      if (uStack_1a8 != 0) {
        (*DAT_143ad5990)(uStack_1a8 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_1b0);
    }
  }
  *(int *)(param_1 + 0x78) = -(iVar7 / 2);
  if ((param_1[0x69] == 0) || (*(int *)(param_1[0x69] + -8) < 1)) {
    piVar16 = (int *)FUN_14019b600(&DAT_143ad6a30,0x12);
    piVar16[1] = 1;
    *piVar16 = -1;
    piVar14 = piVar16 + 4;
    piVar16[2] = 0;
    *(undefined1 *)piVar14 = 0;
    *(undefined1 *)piVar14 = DAT_143271f04;
    local_d8 = piVar14;
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar16[1] < 1) {
      FUN_142e54290(0x90,piVar16[1],1);
    }
    *piVar16 = 1;
    *(undefined1 *)((longlong)piVar16 + 0x11) = 0;
    if (piVar16[1] + 1 < 2) {
      FUN_142e54290(0x9c,1);
    }
    piVar16[2] = 1;
    if (param_1[0x69] != 0) {
      FUN_14019f2c0(param_1[0x69] + -0x10);
    }
    param_1[0x69] = (longlong)piVar14;
  }
  local_88 = 0;
  local_80 = 0;
  local_78 = 0;
  uStack_70 = 0;
  local_68 = 0;
  local_60[1] = 0;
  *(undefined4 *)(param_1 + 0x6a) = 0;
  if (param_1[0x6b] != 0) {
    FUN_14019f2c0(param_1[0x6b] + -0x10);
  }
  local_80 = 0;
  param_1[0x6b] = 0;
  param_1[0x6c] = 0;
  param_1[0x6d] = 0;
  *(undefined4 *)(param_1 + 0x6e) = 0;
  if ((param_1[0x70] - 1U < 999) || (param_1[0x70] == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (param_1 + 0x6f == local_60) {
    FUN_142e52d50(0x45c,1);
  }
  FUN_1401abd80(param_1 + 0x6f);
  param_1[0x70] = 0;
  plVar15 = (longlong *)FUN_1427703d0(local_188,0);
  local_180 = plVar15;
  if (plVar15 != (longlong *)0x0) {
    puVar10 = (undefined8 *)FUN_141ebdad0(plVar15);
    if (puVar10 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)puVar10[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      puVar10[1] = puVar10[1] + 1;
      UNLOCK();
    }
    if (puVar10 != (undefined8 *)0x0) {
      uVar6 = FUN_14019a5d0(puVar10 + 4);
      *(undefined4 *)(param_1 + 0x6a) = uVar6;
      lVar23 = (longlong)puVar10 + 0x4d;
      piVar14 = (int *)0x0;
      local_188 = (int *)0x0;
      if (lVar23 != 0) {
        uVar21 = 0xffffffffffffffff;
        do {
          uVar21 = uVar21 + 1;
        } while (*(char *)(lVar23 + uVar21) != '\0');
        iVar8 = (int)uVar21;
        iVar7 = 0;
        if (0 < iVar8) {
          iVar7 = iVar8;
        }
        piVar16 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
        piVar16[1] = iVar7;
        *piVar16 = -1;
        piVar14 = piVar16 + 4;
        piVar16[2] = 0;
        *(undefined1 *)piVar14 = 0;
        local_188 = piVar14;
        FUN_142ef7ba0(piVar14,lVar23,(longlong)iVar8);
        if (*piVar16 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar8 == -1) || (iVar8 <= piVar16[1])) {
          *piVar16 = 1;
          if (iVar8 != -1) goto LAB_1411985c0;
          if (piVar14 == (int *)0x0) {
            uVar21 = 0;
          }
          else {
            uVar21 = 0xffffffffffffffff;
            do {
              uVar21 = uVar21 + 1;
            } while (*(char *)((longlong)piVar14 + uVar21) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar16[1],uVar21 & 0xffffffff);
          *piVar16 = 1;
LAB_1411985c0:
          *(undefined1 *)((longlong)piVar14 + (longlong)iVar8) = 0;
        }
        iVar7 = (int)uVar21;
        if ((iVar7 < 0) || (piVar16[1] + 1 <= iVar7)) {
          FUN_142e54290(0x9c,uVar21 & 0xffffffff);
        }
        piVar16[2] = iVar7;
        plVar15 = local_180;
      }
      if (param_1[0x6b] != 0) {
        FUN_14019f2c0(param_1[0x6b] + -0x10);
      }
      param_1[0x6b] = (longlong)piVar14;
      bVar4 = FUN_1401b0050((longlong)puVar10 + 0x5a,*(undefined4 *)((longlong)puVar10 + 0x5e));
      *(uint *)(param_1 + 0x6c) = (uint)bVar4;
      sVar5 = FUN_1401ab420((longlong)puVar10 + 0x62,*(undefined4 *)((longlong)puVar10 + 0x66));
      *(int *)((longlong)param_1 + 0x364) = (int)sVar5;
      bVar4 = FUN_1401b0050((longlong)puVar10 + 0x6a,*(undefined4 *)((longlong)puVar10 + 0x6e));
      *(uint *)(param_1 + 0x6d) = (uint)bVar4;
      iVar7 = FUN_1402537a0(0);
      puVar17 = *(undefined8 **)(local_168 + ((longlong)iVar7 + 0x3b) * 0x10);
      local_e0 = puVar17;
      if (puVar17 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar17[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar17[1] = puVar17[1] + 1;
        UNLOCK();
      }
      if (puVar17 != (undefined8 *)0x0) {
        FUN_1416db0f0(local_f8,local_e0);
        local_160 = (undefined8 *)0x0;
        if (local_f0 != (undefined8 *)0x0) {
          puVar17 = (undefined8 *)FUN_140193080(local_f0);
          if (puVar17 != (undefined8 *)0x0) {
            if (0xfffff < (ulonglong)puVar17[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            puVar17[1] = puVar17[1] + 1;
            UNLOCK();
          }
          local_160 = puVar17;
          if (puVar17 != (undefined8 *)0x0) {
            if ((param_1[0x70] - 1U < 999) || (param_1[0x70] == -1)) {
              FUN_142e52ed0(0x447);
            }
            if (0xfffff < (ulonglong)puVar17[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            puVar17[1] = puVar17[1] + 1;
            UNLOCK();
            puVar3 = (undefined8 *)param_1[0x70];
            param_1[0x70] = (longlong)puVar17;
            if (puVar3 != (undefined8 *)0x0) {
              if (0xffffe < puVar3[1] - 1) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar24 = puVar3 + 1;
              lVar23 = *plVar24;
              *plVar24 = *plVar24 + -1;
              UNLOCK();
              if ((int)lVar23 == 1) {
                (**(code **)*puVar3)(puVar3,1);
              }
            }
          }
        }
        puVar17 = local_160;
        if (local_160 != (undefined8 *)0x0) {
          if (0xffffe < local_160[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar24 = puVar17 + 1;
          lVar23 = *plVar24;
          *plVar24 = *plVar24 + -1;
          UNLOCK();
          if ((int)lVar23 == 1) {
            (**(code **)*local_160)(local_160,1);
          }
        }
        puVar17 = local_f0;
        if (local_f0 != (undefined8 *)0x0) {
          if (0xffffe < local_f0[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar24 = puVar17 + 1;
          lVar23 = *plVar24;
          *plVar24 = *plVar24 + -1;
          UNLOCK();
          if (((int)lVar23 == 1) && (local_f0 != (undefined8 *)0x0)) {
            (**(code **)*local_f0)(local_f0,1);
          }
        }
      }
      puVar17 = local_e0;
      if (local_e0 != (undefined8 *)0x0) {
        if (0xffffe < local_e0[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar24 = puVar17 + 1;
        lVar23 = *plVar24;
        *plVar24 = *plVar24 + -1;
        UNLOCK();
        if ((int)lVar23 == 1) {
          (**(code **)*local_e0)(local_e0,1);
        }
      }
    }
    if (puVar10 != (undefined8 *)0x0) {
      if (0xffffe < puVar10[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar24 = puVar10 + 1;
      lVar23 = *plVar24;
      *plVar24 = *plVar24 + -1;
      UNLOCK();
      if ((int)lVar23 == 1) {
        (**(code **)*puVar10)(puVar10,1);
      }
    }
  }
  lVar23 = local_198;
  param_1[0x75] = param_1[0x74];
  iVar7 = 1;
  do {
    iVar8 = FUN_1402c90f0(lVar23,iVar7);
    uVar6 = FUN_1402c9150(lVar23,iVar7);
    if (iVar8 == 1) {
      *(undefined4 *)(param_1 + 0x79) = uVar6;
LAB_14119890b:
      *(undefined1 *)((longlong)param_1 + 0x3c4) = 0;
    }
    else if (iVar8 != 0) goto LAB_14119890b;
    local_198 = CONCAT44(uVar6,iVar8);
    local_190 = FUN_1402c92a0(lVar23,iVar7);
    plVar24 = (longlong *)param_1[0x75];
    if (plVar24 == (longlong *)param_1[0x76]) {
      FUN_1401cf0e0(param_1 + 0x74,plVar24,&local_198);
    }
    else {
      *plVar24 = local_198;
      *(undefined4 *)(plVar24 + 1) = local_190;
      param_1[0x75] = param_1[0x75] + 0xc;
    }
    iVar7 = iVar7 + 1;
  } while (iVar7 < 3);
  *(undefined1 *)(param_1 + 0x62) = 1;
  if (param_1[0x61] == 0) goto LAB_141198aba;
  lVar23 = FUN_1402b0250(*(undefined4 *)((longlong)param_1 + 0x33c),0);
  piVar14 = (int *)0x0;
  local_res20 = (int *)0x0;
  if (lVar23 != 0) {
    uVar21 = 0xffffffffffffffff;
    do {
      uVar21 = uVar21 + 1;
    } while (*(char *)(lVar23 + uVar21) != '\0');
    iVar8 = (int)uVar21;
    iVar7 = 0;
    if (0 < iVar8) {
      iVar7 = iVar8;
    }
    piVar16 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
    piVar16[1] = iVar7;
    *piVar16 = -1;
    piVar14 = piVar16 + 4;
    piVar16[2] = 0;
    *(undefined1 *)piVar14 = 0;
    local_res20 = piVar14;
    FUN_142ef7ba0(piVar14,lVar23,(longlong)iVar8);
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar8 == -1) || (iVar8 <= piVar16[1])) {
      *piVar16 = 1;
      if (iVar8 != -1) goto LAB_141198a22;
      if (piVar14 == (int *)0x0) {
        uVar21 = 0;
      }
      else {
        do {
          uVar22 = uVar22 + 1;
        } while (*(char *)((longlong)piVar14 + uVar22) != '\0');
        uVar21 = uVar22 & 0xffffffff;
      }
    }
    else {
      FUN_142e54290(0x90,piVar16[1],uVar21 & 0xffffffff);
      *piVar16 = 1;
LAB_141198a22:
      *(undefined1 *)((longlong)iVar8 + (longlong)piVar14) = 0;
    }
    iVar7 = (int)uVar21;
    if ((iVar7 < 0) || (piVar16[1] + 1 <= iVar7)) {
      FUN_142e54290(0x9c,uVar21 & 0xffffffff);
    }
    piVar16[2] = iVar7;
  }
  lVar23 = param_1[0x61];
  if (lVar23 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar23 = param_1[0x61];
  }
  FUN_14019a260(lVar23 + 0x11b0,&local_res20);
  lVar23 = param_1[0x61];
  if (lVar23 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar23 = param_1[0x61];
  }
  FUN_14019a260(lVar23 + 0x11c0,&local_res20);
  if (piVar14 != (int *)0x0) {
    FUN_14019f2c0(piVar14 + -4);
  }
LAB_141198aba:
  if (DAT_143acaa00 != 0) {
    if (plVar15 == (longlong *)0x0) {
      FUN_142bf3f70();
      if (DAT_143acaa00 != 0) {
        (*(code *)**(undefined8 **)(DAT_143acaa00 + 8))((undefined8 *)(DAT_143acaa00 + 8),1);
      }
      FUN_141196560(param_1);
    }
    else {
      FUN_14119c3c0();
    }
  }
  if (DAT_143aca9f8 != 0) {
    FUN_14119a960();
  }
  if (DAT_143acaa08 != 0) {
    FUN_1411a00d0();
  }
  (**(code **)(*param_1 + 0x90))(param_1,0);
  return;
}



//===========================================================
// FUN_1411a00d0 @ 1411a00d0   (160 bytes)
//===========================================================

void FUN_1411a00d0(longlong *param_1)

{
  longlong lVar1;
  
  if (DAT_143ac8908 == 0) {
    return;
  }
  for (lVar1 = *(longlong *)(DAT_143ac8908 + 0x3a0); lVar1 != *(longlong *)(DAT_143ac8908 + 0x3a8);
      lVar1 = lVar1 + 0xc) {
  }
  *(undefined4 *)(param_1 + 0x6d) = 0;
  if (param_1[0x66] != 0) {
    FUN_1416ee330();
    lVar1 = param_1[0x66];
    if (lVar1 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar1 = param_1[0x66];
    }
    FUN_1416ee2d0(lVar1,0);
  }
                    /* WARNING: Could not recover jumptable at 0x0001411a0163. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(*param_1 + 0x90))(param_1,0);
  return;
}



//===========================================================
// FUN_1402c90f0 @ 1402c90f0   (88 bytes)
//===========================================================

ulonglong FUN_1402c90f0(undefined8 param_1,undefined4 param_2)

{
  ulonglong uVar1;
  longlong local_res18 [2];
  
  FUN_1402c8870(local_res18,0,param_2);
  uVar1 = FUN_140729fb0(510000,local_res18[0],0);
  if (local_res18[0] != 0) {
    FUN_14019f2c0(local_res18[0] + -0x10);
    return uVar1 & 0xffffffff;
  }
  return uVar1;
}



//===========================================================
// FUN_1402c9150 @ 1402c9150   (91 bytes)
//===========================================================

ulonglong FUN_1402c9150(undefined8 param_1,undefined4 param_2)

{
  ulonglong uVar1;
  longlong local_res18 [2];
  
  FUN_1402c8870(local_res18,1,param_2);
  uVar1 = FUN_140729fb0(510000,local_res18[0],0);
  if (local_res18[0] != 0) {
    FUN_14019f2c0(local_res18[0] + -0x10);
    return uVar1 & 0xffffffff;
  }
  return uVar1;
}



//===========================================================
// FUN_1402c92a0 @ 1402c92a0   (91 bytes)
//===========================================================

ulonglong FUN_1402c92a0(undefined8 param_1,undefined4 param_2)

{
  ulonglong uVar1;
  longlong local_res18 [2];
  
  FUN_1402c8870(local_res18,2,param_2);
  uVar1 = FUN_140729fb0(510000,local_res18[0],0);
  if (local_res18[0] != 0) {
    FUN_14019f2c0(local_res18[0] + -0x10);
    return uVar1 & 0xffffffff;
  }
  return uVar1;
}


