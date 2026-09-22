
//===========================================================
// FUN_140426220 @ 140426220   (198 bytes)
//===========================================================

void FUN_140426220(uint *param_1,undefined8 param_2)

{
  byte bVar1;
  undefined8 *puVar2;
  longlong local_res8;
  
  bVar1 = FUN_1406e8ae0(param_2);
  *param_1 = (uint)bVar1;
  puVar2 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
  if (*(longlong *)(param_1 + 2) != 0) {
    FUN_14019f2c0();
    param_1[2] = 0;
    param_1[3] = 0;
  }
  *(undefined8 *)(param_1 + 2) = *puVar2;
  *puVar2 = 0;
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  puVar2 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
  if (*(longlong *)(param_1 + 4) != 0) {
    FUN_14019f2c0();
    param_1[4] = 0;
    param_1[5] = 0;
  }
  *(undefined8 *)(param_1 + 4) = *puVar2;
  *puVar2 = 0;
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141184360 @ 141184360   (1254 bytes)
//===========================================================

void FUN_141184360(undefined8 param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined4 *puVar4;
  longlong lVar5;
  int *piVar6;
  longlong *plVar7;
  undefined8 uVar8;
  int *piVar9;
  longlong **pplVar10;
  ulonglong uVar11;
  undefined4 *puVar12;
  int *piVar13;
  int iVar14;
  int iVar15;
  int *piVar16;
  int **ppiVar17;
  bool bVar18;
  undefined4 uVar19;
  int *local_res10;
  longlong *local_78;
  longlong *plStack_70;
  longlong *local_68;
  longlong *local_60;
  longlong *plStack_58;
  longlong *local_50;
  undefined4 local_48 [2];
  longlong local_40;
  longlong alStack_38 [2];
  
  uVar8 = DAT_143aa84a0;
  bVar18 = true;
  puVar12 = &DAT_143aca880;
  puVar4 = &DAT_143aca880;
  do {
    if (puVar4[1] != 0) {
      bVar18 = false;
      break;
    }
    puVar4 = puVar4 + 8;
  } while (puVar4 != &DAT_143aca940);
  local_78 = (longlong *)0x0;
  plStack_70 = (longlong *)0x0;
  piVar9 = (int *)0x0;
  local_68 = (longlong *)0x0;
  local_60 = (longlong *)0x0;
  plStack_58 = (longlong *)0x0;
  local_50 = (longlong *)0x0;
  if (!bVar18) {
    piVar9 = &DAT_143aca884;
    do {
      iVar15 = *piVar9;
      local_res10 = (int *)0x0;
      FUN_14019a260(&local_res10,piVar9 + 1);
      FUN_140425ed0(piVar9 + -1,param_1);
      if (iVar15 == 0) {
        if (*piVar9 != 0) {
          if (plStack_70 == local_68) {
            ppiVar17 = (int **)(piVar9 + 1);
            pplVar10 = &local_78;
            plVar7 = plStack_70;
LAB_141184592:
            FUN_1401ba060(pplVar10,plVar7,ppiVar17);
          }
          else {
            *plStack_70 = 0;
            FUN_14019a260(plStack_70,piVar9 + 1);
            plStack_70 = plStack_70 + 1;
          }
        }
      }
      else if (*piVar9 == 0) {
        if (plStack_58 == local_50) {
          ppiVar17 = &local_res10;
          pplVar10 = &local_60;
          plVar7 = plStack_58;
          goto LAB_141184592;
        }
        *plStack_58 = 0;
        FUN_14019a260(plStack_58,&local_res10);
        plStack_58 = plStack_58 + 1;
      }
      if (local_res10 != (int *)0x0) {
        FUN_14019f2c0(local_res10 + -4);
      }
      bVar18 = piVar9 != &DAT_143aca924;
      piVar9 = piVar9 + 8;
      plVar7 = local_78;
      plVar1 = plStack_70;
      plVar2 = plStack_58;
    } while (bVar18);
    goto joined_r0x0001411845ca;
  }
  do {
    FUN_140425ed0(puVar12,param_1);
    puVar12 = puVar12 + 8;
  } while (puVar12 != &DAT_143aca940);
  lVar5 = FUN_142cb9610(uVar8);
  local_res10 = (int *)0x0;
  piVar13 = piVar9;
  if (lVar5 != 0) {
    piVar16 = (int *)0xffffffffffffffff;
    do {
      piVar16 = (int *)((longlong)piVar16 + 1);
    } while (*(char *)(lVar5 + (longlong)piVar16) != '\0');
    iVar14 = (int)piVar16;
    iVar15 = 0;
    if (0 < iVar14) {
      iVar15 = iVar14;
    }
    piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
    piVar6[1] = iVar15;
    *piVar6 = -1;
    piVar13 = piVar6 + 4;
    piVar6[2] = 0;
    *(undefined1 *)piVar13 = 0;
    local_res10 = piVar13;
    FUN_142ef7ba0(piVar13,lVar5,(longlong)iVar14);
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar14 == -1) || (iVar14 <= piVar6[1])) {
      *piVar6 = 1;
      if (iVar14 != -1) goto LAB_141184483;
      piVar16 = piVar9;
      if (piVar13 != (int *)0x0) {
        piVar16 = (int *)0xffffffffffffffff;
        do {
          piVar16 = (int *)((longlong)piVar16 + 1);
        } while (*(char *)((longlong)piVar13 + (longlong)piVar16) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar6[1],(ulonglong)piVar16 & 0xffffffff);
      *piVar6 = 1;
LAB_141184483:
      *(undefined1 *)((longlong)iVar14 + (longlong)piVar13) = 0;
    }
    iVar15 = (int)piVar16;
    if ((iVar15 < 0) || (piVar6[1] + 1 <= iVar15)) {
      FUN_142e54290(0x9c,(ulonglong)piVar16 & 0xffffffff);
    }
    piVar6[2] = iVar15;
  }
  if (plStack_70 == local_68) {
    FUN_1401d63a0(&local_78,plStack_70,&local_res10);
    piVar9 = local_res10;
  }
  else {
    *plStack_70 = (longlong)piVar13;
    plStack_70 = plStack_70 + 1;
  }
  plVar7 = local_78;
  plVar1 = plStack_70;
  plVar2 = plStack_58;
  if (piVar9 != (int *)0x0) {
    FUN_14019f2c0(piVar9 + -4);
    plVar7 = local_78;
    plVar1 = plStack_70;
    plVar2 = plStack_58;
  }
joined_r0x0001411845ca:
  for (; plVar3 = plStack_70, bVar18 = plVar7 != plStack_70, plStack_70 = plVar1,
      plStack_58 = plVar2, plVar1 = local_60, bVar18; plVar7 = plVar7 + 1) {
    local_res10 = (int *)0x0;
    FUN_14019a260(&local_res10,plVar7);
    local_48[0] = 0xffffffff;
    local_40 = 0;
    alStack_38[0] = 0;
    uVar8 = FUN_1408a9f20("SID_MAPLECHAT_USERENTER");
    piVar9 = local_res10;
    FUN_14019ba10(alStack_38,uVar8,local_res10);
    FUN_141183b20(local_48);
    if (alStack_38[0] != 0) {
      FUN_14019f2c0(alStack_38[0] + -0x10);
    }
    if (local_40 != 0) {
      FUN_14019f2c0(local_40 + -0x10);
    }
    if (piVar9 != (int *)0x0) {
      FUN_14019f2c0(piVar9 + -4);
    }
    plVar1 = plStack_70;
    plVar2 = plStack_58;
    plStack_70 = plVar3;
  }
  for (; plVar1 != plVar2; plVar1 = plVar1 + 1) {
    local_res10 = (int *)0x0;
    FUN_14019a260(&local_res10,plVar1);
    local_48[0] = 0xffffffff;
    local_40 = 0;
    alStack_38[0] = 0;
    uVar8 = FUN_1408a9f20("SID_MAPLECHAT_USERLEAVE");
    piVar9 = local_res10;
    FUN_14019ba10(alStack_38,uVar8,local_res10);
    FUN_141183b20(local_48);
    if (alStack_38[0] != 0) {
      FUN_14019f2c0(alStack_38[0] + -0x10);
    }
    if (local_40 != 0) {
      FUN_14019f2c0(local_40 + -0x10);
    }
    if (piVar9 != (int *)0x0) {
      FUN_14019f2c0(piVar9 + -4);
    }
  }
  uVar19 = FUN_141184970();
  plVar7 = DAT_143aca618;
  if (DAT_143aca618 != (longlong *)0x0) {
    FUN_141187b90(DAT_143aca618);
    uVar19 = FUN_141187eb0(plVar7,(int)plVar7[0x5e]);
    if (DAT_143aca618 != (longlong *)0x0) {
      uVar19 = (**(code **)(*DAT_143aca618 + 0x90))(DAT_143aca618,0);
    }
  }
  plVar1 = plStack_58;
  plVar7 = local_60;
  if (local_60 != (longlong *)0x0) {
    for (; plVar7 != plVar1; plVar7 = plVar7 + 1) {
      if (*plVar7 != 0) {
        uVar19 = FUN_14019f2c0(*plVar7 + -0x10);
      }
    }
    uVar11 = ((longlong)local_50 - (longlong)local_60 >> 3) * 8;
    plVar7 = local_60;
    if (0xfff < uVar11) {
      plVar7 = (longlong *)local_60[-1];
      if (0x1f < (ulonglong)((longlong)local_60 + (-8 - (longlong)plVar7))) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(uVar19,uVar11 + 0x27);
      }
    }
    thunk_FUN_140205820(plVar7);
    uVar19 = 0;
    local_60 = (longlong *)0x0;
    plStack_58 = (longlong *)0x0;
    local_50 = (longlong *)0x0;
  }
  plVar1 = plStack_70;
  plVar7 = local_78;
  if (local_78 != (longlong *)0x0) {
    for (; plVar7 != plVar1; plVar7 = plVar7 + 1) {
      if (*plVar7 != 0) {
        uVar19 = FUN_14019f2c0(*plVar7 + -0x10);
      }
    }
    uVar11 = (longlong)local_68 - (longlong)local_78 & 0xfffffffffffffff8;
    plVar7 = local_78;
    if (0xfff < uVar11) {
      plVar7 = (longlong *)local_78[-1];
      if (0x1f < (ulonglong)((longlong)local_78 + (-8 - (longlong)plVar7))) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(uVar19,uVar11 + 0x27);
      }
    }
    thunk_FUN_140205820(plVar7);
  }
  return;
}



//===========================================================
// FUN_141183cc0 @ 141183cc0   (471 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141183cc0(void)

{
  longlong *plVar1;
  int *piVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong *plVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  undefined1 auStackY_138 [32];
  undefined4 local_e8 [2];
  undefined8 auStack_e0 [23];
  ulonglong local_28;
  
  lVar9 = DAT_143aca838;
  local_28 = DAT_143a8b908 ^ (ulonglong)auStackY_138;
  DAT_143a872a0 = 0xffffffff;
  for (lVar8 = DAT_143aca830; lVar8 != lVar9; lVar8 = lVar8 + 0x18) {
    if (*(longlong *)(lVar8 + 0x10) != 0) {
      FUN_14019f2c0(*(longlong *)(lVar8 + 0x10) + -0x10);
    }
    if (*(longlong *)(lVar8 + 8) != 0) {
      FUN_14019f2c0(*(longlong *)(lVar8 + 8) + -0x10);
    }
  }
  DAT_143aca838 = DAT_143aca830;
  FUN_142ef8250(local_e8,0,0xc0);
  _eh_vector_constructor_iterator_
            (local_e8,0x20,6,(_func_void_void_ptr *)&LAB_14118a5f0,FUN_14118a800);
  lVar9 = 0;
  lVar8 = 6;
  do {
    *(undefined4 *)((longlong)&DAT_143aca880 + lVar9) = *(undefined4 *)((longlong)local_e8 + lVar9);
    *(undefined4 *)((longlong)&DAT_143aca884 + lVar9) =
         *(undefined4 *)((longlong)local_e8 + lVar9 + 4);
    if (*(longlong *)((longlong)&DAT_143aca888 + lVar9) != 0) {
      FUN_14019f2c0(*(longlong *)((longlong)&DAT_143aca888 + lVar9) + -0x10);
    }
    *(undefined8 *)((longlong)&DAT_143aca888 + lVar9) =
         *(undefined8 *)((longlong)auStack_e0 + lVar9);
    *(undefined8 *)((longlong)auStack_e0 + lVar9) = 0;
    uVar4 = *(undefined8 *)((longlong)auStack_e0 + lVar9 + 8);
    uVar5 = *(undefined8 *)((longlong)auStack_e0 + lVar9 + 0x10);
    *(undefined8 *)((longlong)auStack_e0 + lVar9 + 8) = 0;
    *(undefined8 *)((longlong)auStack_e0 + lVar9 + 0x10) = 0;
    *(undefined8 *)((longlong)&DAT_143aca890 + lVar9) = uVar4;
    plVar6 = *(longlong **)((longlong)&DAT_143aca898 + lVar9);
    *(undefined8 *)((longlong)&DAT_143aca898 + lVar9) = uVar5;
    if (plVar6 != (longlong *)0x0) {
      LOCK();
      plVar1 = plVar6 + 1;
      lVar7 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -1;
      UNLOCK();
      if ((int)lVar7 == 1) {
        (**(code **)*plVar6)(plVar6);
        LOCK();
        piVar2 = (int *)((longlong)plVar6 + 0xc);
        iVar3 = *piVar2;
        *piVar2 = *piVar2 + -1;
        UNLOCK();
        if (iVar3 == 1) {
          (**(code **)(*plVar6 + 8))(plVar6);
        }
      }
    }
    lVar9 = lVar9 + 0x20;
    lVar8 = lVar8 + -1;
  } while (lVar8 != 0);
  _eh_vector_destructor_iterator_(local_e8,0x20,6,FUN_14118a800);
  _DAT_143aca7e0 = 0;
  _DAT_143aca7e8 = 0;
  _DAT_143aca7f0 = 0;
  return;
}



//===========================================================
// FUN_141183b20 @ 141183b20   (410 bytes)
//===========================================================

void FUN_141183b20(int *param_1)

{
  int *piVar1;
  int *piVar2;
  longlong *plVar3;
  int iVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  
  iVar4 = *param_1;
  if (-1 < iVar4) {
    if ((ulonglong)(longlong)iVar4 < 6) {
      puVar5 = &DAT_143aca880 + (longlong)iVar4 * 8;
    }
    else {
      puVar5 = &DAT_143aca940;
    }
    if (DAT_143aa84a0 == 0) {
      return;
    }
    iVar4 = FUN_142d01050(DAT_143aa84a0,*puVar5);
    if (iVar4 != 0) {
      return;
    }
  }
  piVar2 = DAT_143aca838;
  if (DAT_143aca838 == DAT_143aca840) {
    FUN_14118a0a0(&DAT_143aca830,DAT_143aca838,param_1);
  }
  else {
    *DAT_143aca838 = *param_1;
    piVar1 = piVar2 + 2;
    piVar1[0] = 0;
    piVar1[1] = 0;
    FUN_14019a260(piVar1,param_1 + 2);
    piVar2 = piVar2 + 4;
    piVar2[0] = 0;
    piVar2[1] = 0;
    FUN_14019a260(piVar2,param_1 + 4);
    DAT_143aca838 = DAT_143aca838 + 6;
  }
  plVar3 = DAT_143aca618;
  if (DAT_143aca618 != (longlong *)0x0) {
    FUN_141188f60(DAT_143aca618,(int)(((longlong)DAT_143aca838 - DAT_143aca830) / 0x18) + -1);
    FUN_141189b30(plVar3,1);
    if (*(int *)((longlong)plVar3 + 0x2dc) == 1) {
      iVar4 = *(int *)((longlong)plVar3 + 0x394);
      lVar6 = ((longlong)DAT_143aca838 - DAT_143aca830) / 6 +
              ((longlong)DAT_143aca838 - DAT_143aca830 >> 0x3f);
      iVar7 = (int)(lVar6 >> 2) - (int)(lVar6 >> 0x3f);
      FUN_142aa2720(plVar3 + 0x5d,L"newchat_enabled",iVar4 < iVar7);
      FUN_142aa2720(plVar3 + 0x5d,L"newchat_disabled",iVar7 <= iVar4);
    }
    (**(code **)(*plVar3 + 0x90))(plVar3,0);
  }
  return;
}


