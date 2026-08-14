
//===========================================================
// FUN_1408aa850 @ 1408aa850   (661 bytes)
//===========================================================

void FUN_1408aa850(longlong *param_1,int param_2,int param_3)

{
  longlong **pplVar1;
  longlong *plVar2;
  uint uVar3;
  int *piVar4;
  longlong *plVar5;
  undefined *puVar6;
  int *piVar7;
  longlong lVar8;
  int iVar9;
  int iVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong *local_res20;
  
  puVar6 = (&PTR_PTR_143a563f8)[param_3];
  lVar8 = *(longlong *)(puVar6 + (longlong)param_2 * 8) + 1;
  piVar7 = (int *)0x0;
  if (lVar8 != 0) {
    uVar11 = 0xffffffffffffffff;
    do {
      uVar11 = uVar11 + 1;
    } while (*(char *)(uVar11 + lVar8) != '\0');
    iVar9 = (int)uVar11;
    iVar10 = 0;
    if (0 < iVar9) {
      iVar10 = iVar9;
    }
    piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar10 + 0x11));
    piVar4[1] = iVar10;
    *piVar4 = -1;
    piVar7 = piVar4 + 4;
    piVar4[2] = 0;
    *(undefined1 *)piVar7 = 0;
    FUN_142ef7ba0(piVar7,lVar8,(longlong)iVar9);
    if (*piVar4 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar9 == -1) || (iVar9 <= piVar4[1])) {
      *piVar4 = 1;
      if (iVar9 != -1) goto LAB_1408aa917;
      if (piVar7 == (int *)0x0) {
        uVar11 = 0;
      }
      else {
        uVar11 = 0xffffffffffffffff;
        do {
          uVar11 = uVar11 + 1;
        } while (*(char *)((longlong)piVar7 + uVar11) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar4[1],uVar11 & 0xffffffff);
      *piVar4 = 1;
LAB_1408aa917:
      *(undefined1 *)((longlong)piVar7 + (longlong)iVar9) = 0;
    }
    iVar10 = (int)uVar11;
    if ((iVar10 < 0) || (piVar4[1] + 1 <= iVar10)) {
      FUN_142e54290(0x9c,uVar11 & 0xffffffff);
    }
    piVar4[2] = iVar10;
  }
  if (*param_1 != 0) {
    FUN_14019f2c0(*param_1 + -0x10);
  }
  *param_1 = (longlong)piVar7;
  FUN_1408aadb0(param_1,&DAT_1432ba470,0x10,(longlong)**(char **)(puVar6 + (longlong)param_2 * 8));
  puVar6 = (undefined *)*param_1;
  if (puVar6 == (undefined *)0x0) {
    puVar6 = &DAT_1432780f0;
    iVar10 = 0;
  }
  else {
    iVar10 = *(int *)(puVar6 + -8);
  }
  FUN_1408abf30(param_1 + 1,puVar6,puVar6 + iVar10);
  lVar8 = param_1[1];
  plVar5 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
  local_res20 = plVar5;
  if (plVar5 == (longlong *)0x0) {
    plVar5 = (longlong *)0x0;
  }
  else {
    plVar5[1] = 0;
    *(undefined4 *)(plVar5 + 2) = 1;
    if (lVar8 != 0) {
      lVar12 = -1;
      do {
        lVar12 = lVar12 + 1;
      } while (*(short *)(lVar8 + lVar12 * 2) != 0);
      uVar3 = (int)lVar12 + 1;
      piVar7 = (int *)(*DAT_143ad5980)((ulonglong)uVar3 * 2 + 4);
      if (piVar7 == (int *)0x0) {
        *plVar5 = 0;
      }
      else {
        *piVar7 = (int)lVar12 * 2;
        piVar7 = piVar7 + 1;
        FUN_142ef7ba0(piVar7,lVar8,(ulonglong)uVar3 * 2);
        *plVar5 = (longlong)piVar7;
        if (piVar7 != (int *)0x0) goto LAB_1408aaa59;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    *plVar5 = 0;
  }
LAB_1408aaa59:
  if (plVar5 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x8007000e);
  }
  pplVar1 = (longlong **)(param_1 + 2);
  if (pplVar1 != &local_res20) {
    FUN_1401be120(pplVar1);
    *pplVar1 = plVar5;
    LOCK();
    *(int *)(plVar5 + 2) = (int)plVar5[2] + 1;
    UNLOCK();
  }
  LOCK();
  plVar2 = plVar5 + 2;
  lVar8 = *plVar2;
  *(int *)plVar2 = (int)*plVar2 + -1;
  UNLOCK();
  if ((int)lVar8 == 1) {
    if (*plVar5 != 0) {
      (*DAT_143ad5990)(*plVar5 + -4);
      *plVar5 = 0;
    }
    if (plVar5[1] != 0) {
      FUN_14019b4e0();
      plVar5[1] = 0;
    }
    thunk_FUN_140205820(plVar5,0x18);
  }
  return;
}


