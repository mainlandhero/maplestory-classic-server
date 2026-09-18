
//===========================================================
// FUN_1411992f0 @ 1411992f0   (778 bytes)
//===========================================================

void FUN_1411992f0(longlong param_1)

{
  longlong *plVar1;
  byte bVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  undefined8 *puVar6;
  longlong *plVar7;
  longlong lVar8;
  int *piVar9;
  int *local_res8;
  longlong *local_res18;
  longlong *local_res20;
  longlong *local_58;
  longlong *local_50;
  longlong *local_48;
  longlong local_40 [2];
  undefined8 *local_30;
  
  FUN_1417104f0();
  if (*(longlong *)(param_1 + 0x11b8) != 0) {
    puVar6 = (undefined8 *)FUN_140193080();
    local_30 = puVar6;
    if (puVar6 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)puVar6[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      puVar6[1] = puVar6[1] + 1;
      UNLOCK();
    }
    puVar6 = local_30;
    if (local_30 != (undefined8 *)0x0) {
      FUN_14090ead0(&local_res18,*(undefined8 *)(param_1 + 0x11a8));
      if (local_res18 != (longlong *)0x0) {
        lVar8 = *(longlong *)(param_1 + 0x11b8);
        if (lVar8 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar8 = *(longlong *)(param_1 + 0x11b8);
        }
        uVar3 = FUN_14019a5d0(lVar8 + 0x20);
        *(undefined4 *)(param_1 + 0x11c0) = uVar3;
        plVar7 = (longlong *)FUN_140398ba0(DAT_143aa8328,local_40,uVar3);
        plVar1 = (longlong *)(param_1 + 0x11c8);
        if (*plVar1 != 0) {
          FUN_14019f2c0();
          *plVar1 = 0;
        }
        *plVar1 = *plVar7;
        *plVar7 = 0;
        if (local_40[0] != 0) {
          FUN_14019f2c0(local_40[0] + -0x10);
        }
        bVar2 = FUN_1401b0050((longlong)puVar6 + 0x16e,*(undefined4 *)((longlong)puVar6 + 0x172));
        *(uint *)(param_1 + 0x11d0) = (uint)bVar2;
        lVar8 = FUN_140388c60(DAT_143aa8328,*(undefined4 *)(param_1 + 0x11c0));
        if (lVar8 != 0) {
          *(int *)(param_1 + 0x11d0) = *(int *)(param_1 + 0x11d0) + *(int *)(lVar8 + 0x78);
        }
        FUN_1429fbeb0(&local_48,2);
        local_res8 = (int *)0x0;
        piVar9 = (int *)FUN_14019b600(&DAT_143ad6a30,0x18);
        piVar9[1] = 7;
        *piVar9 = -1;
        local_res8 = piVar9 + 4;
        piVar9[2] = 0;
        *(undefined1 *)local_res8 = 0;
        *local_res8 = s_posName_14337d938._0_4_;
        *(undefined2 *)(piVar9 + 5) = s_posName_14337d938._4_2_;
        *(char *)((longlong)piVar9 + 0x16) = s_posName_14337d938[6];
        if (*piVar9 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar9[1] < 7) {
          FUN_142e54290(0x90,piVar9[1],7);
        }
        *piVar9 = 1;
        *(undefined1 *)((longlong)local_res8 + 7) = 0;
        if (piVar9[1] + 1 < 8) {
          FUN_142e54290(0x9c,7);
        }
        piVar9[2] = 7;
        local_res20 = local_res18;
        if (local_res18 != (longlong *)0x0) {
          (**(code **)(*local_res18 + 8))();
        }
        FUN_14090fcc0(&local_50,&local_res20,&local_res8);
        if (local_res8 != (int *)0x0) {
          FUN_14019f2c0(local_res8 + -4);
        }
        if (local_50 != (longlong *)0x0) {
          iVar4 = FUN_140319f60();
          iVar5 = FUN_141711e00(param_1);
          local_58 = local_48;
          if (local_48 != (longlong *)0x0) {
            (**(code **)(*local_48 + 8))();
          }
          FUN_1429eb300(plVar1,&local_58,(-5 - iVar4) + iVar5);
        }
        if (local_50 != (longlong *)0x0) {
          (**(code **)(*local_50 + 0x10))();
        }
        if (local_48 != (longlong *)0x0) {
          (**(code **)(*local_48 + 0x10))();
        }
      }
      if (local_res18 != (longlong *)0x0) {
        (**(code **)(*local_res18 + 0x10))();
      }
    }
    if (puVar6 != (undefined8 *)0x0) {
      if (0xffffe < puVar6[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = puVar6 + 1;
      lVar8 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar8 == 1) {
        (**(code **)*local_30)(local_30,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_141199600 @ 141199600   (1803 bytes)
//===========================================================

void FUN_141199600(longlong param_1,int param_2,int param_3)

{
  longlong *plVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  ulonglong uVar6;
  int *piVar7;
  undefined8 uVar8;
  longlong ***local_res8;
  undefined8 in_stack_ffffffffffffff18;
  undefined4 uVar9;
  longlong ***local_a8;
  longlong *local_a0;
  longlong ***local_98;
  longlong **local_90;
  longlong ***local_88;
  longlong *local_80;
  longlong ***local_78;
  longlong **local_70;
  longlong local_68;
  longlong ****local_60;
  longlong ***local_58;
  longlong *local_50;
  longlong ***local_48;
  longlong ***local_40;
  int local_38;
  int local_34;
  longlong ***local_30;
  
  uVar9 = (undefined4)((ulonglong)in_stack_ffffffffffffff18 >> 0x20);
  FUN_141710fc0();
  iVar2 = (**(code **)(*(longlong *)(param_1 + 8) + 0x88))();
  if (iVar2 != 0) {
    FUN_141710af0(param_1,&local_88,0);
    if (local_88 != (longlong ***)0x0) {
      FUN_14090ead0(&local_78,*(undefined8 *)(param_1 + 0x11a8));
      if (local_78 != (longlong ***)0x0) {
        local_res8 = local_78;
        (*(code *)(*local_78)[1])();
        FUN_14090f750(&local_50,&local_res8,L"backgrnd");
        local_a0 = local_50;
        if (local_50 != (longlong *)0x0) {
          (**(code **)(*local_50 + 8))();
        }
        local_98 = local_88;
        if (local_88 != (longlong ***)0x0) {
          (*(code *)(*local_88)[1])();
        }
        FUN_142aa1590(&local_98,&local_a0,param_2,param_3,CONCAT44(uVar9,0xff));
        local_40 = local_78;
        if (local_78 != (longlong ***)0x0) {
          (*(code *)(*local_78)[1])();
        }
        local_30 = local_88;
        local_38 = param_2;
        local_34 = param_3;
        if (local_88 != (longlong ***)0x0) {
          (*(code *)(*local_88)[1])();
        }
        FUN_1429fbeb0(&local_70,2);
        uVar6 = FUN_1408a9e40(&local_68,0x1800);
        FUN_1429fa1b0(&local_58,0xff4a6aaf,0xb,0,uVar6);
        if (local_68 != 0) {
          FUN_14019f2c0(local_68 + -0x10);
        }
        local_a8 = (longlong ***)&local_98;
        local_98 = (longlong ***)local_70;
        if ((longlong ***)local_70 != (longlong ***)0x0) {
          (*(code *)(*local_70)[1])();
        }
        local_48 = (longlong ***)&local_a0;
        local_a0 = (longlong *)0x0;
        FUN_14019a260(&local_a0,param_1 + 0x11c8);
        local_res8 = (longlong ***)0x0;
        piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,0x18);
        piVar7[1] = 7;
        *piVar7 = -1;
        local_res8 = (longlong ***)(piVar7 + 4);
        piVar7[2] = 0;
        *(undefined1 *)local_res8 = 0;
        *(undefined4 *)local_res8 = s_posName_14337d938._0_4_;
        *(undefined2 *)(piVar7 + 5) = s_posName_14337d938._4_2_;
        *(char *)((longlong)piVar7 + 0x16) = s_posName_14337d938[6];
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar7[1] < 7) {
          FUN_142e54290(0x90,piVar7[1],7);
        }
        *piVar7 = 1;
        *(undefined1 *)((longlong)local_res8 + 7) = 0;
        if (piVar7[1] + 1 < 8) {
          FUN_142e54290(0x9c,7);
        }
        piVar7[2] = 7;
        FUN_1411a2390(&local_40,&local_res8,&local_a0,&local_98);
        local_48 = (longlong ***)&local_98;
        local_98 = (longlong ***)local_70;
        if ((longlong ***)local_70 != (longlong ***)0x0) {
          (*(code *)(*local_70)[1])();
        }
        local_60 = (longlong ****)&local_a0;
        local_a8 = (longlong ***)0x0;
        uVar8 = FUN_14019ba10(&local_a8,&DAT_143274298,*(undefined4 *)(param_1 + 0x11d0));
        local_a0 = (longlong *)0x0;
        FUN_14019a260(&local_a0,uVar8);
        local_res8 = (longlong ***)0x0;
        piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,0x19);
        piVar7[1] = 8;
        *piVar7 = -1;
        local_res8 = (longlong ***)(piVar7 + 4);
        piVar7[2] = 0;
        *(undefined1 *)local_res8 = 0;
        *local_res8 = (longlong **)s_posLevel_14338a990._0_8_;
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar7[1] < 8) {
          FUN_142e54290(0x90,piVar7[1],8);
        }
        *piVar7 = 1;
        *(undefined1 *)(local_res8 + 1) = 0;
        if (piVar7[1] + 1 < 9) {
          FUN_142e54290(0x9c,8);
        }
        piVar7[2] = 8;
        FUN_1411a2390(&local_40,&local_res8,&local_a0,&local_98);
        if (local_a8 != (longlong ***)0x0) {
          FUN_14019f2c0(local_a8 + -2);
        }
        local_60 = &local_a8;
        local_a8 = local_58;
        if (local_58 != (longlong ***)0x0) {
          (*(code *)(*local_58)[1])();
        }
        local_48 = (longlong ***)&local_98;
        uVar8 = FUN_1408a9e40(&local_98,0x17f5);
        local_res8 = (longlong ***)0x0;
        piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,0x1e);
        piVar7[1] = 0xd;
        *piVar7 = -1;
        local_res8 = (longlong ***)(piVar7 + 4);
        piVar7[2] = 0;
        *(undefined1 *)local_res8 = 0;
        *local_res8 = (longlong **)s_posLevelLabel_14338a9a0._0_8_;
        piVar7[6] = s_posLevelLabel_14338a9a0._8_4_;
        *(char *)(piVar7 + 7) = s_posLevelLabel_14338a9a0[0xc];
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar7[1] < 0xd) {
          FUN_142e54290(0x90,piVar7[1],0xd);
        }
        *piVar7 = 1;
        *(undefined1 *)((longlong)local_res8 + 0xd) = 0;
        if (piVar7[1] + 1 < 0xe) {
          FUN_142e54290(0x9c,0xd);
        }
        piVar7[2] = 0xd;
        FUN_1411a2390(&local_40,&local_res8,uVar8,&local_a8);
        local_res8 = (longlong ***)0x0;
        piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,0x18);
        piVar7[1] = 7;
        *piVar7 = -1;
        local_res8 = (longlong ***)(piVar7 + 4);
        piVar7[2] = 0;
        *(undefined1 *)local_res8 = 0;
        *(undefined4 *)local_res8 = s_posIcon_1433804f0._0_4_;
        *(undefined2 *)(piVar7 + 5) = s_posIcon_1433804f0._4_2_;
        *(char *)((longlong)piVar7 + 0x16) = s_posIcon_1433804f0[6];
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar7[1] < 7) {
          FUN_142e54290(0x90,piVar7[1],7);
        }
        *piVar7 = 1;
        *(undefined1 *)((longlong)local_res8 + 7) = 0;
        if (piVar7[1] + 1 < 8) {
          FUN_142e54290(0x9c,7);
        }
        piVar7[2] = 7;
        local_a8 = local_78;
        if (local_78 != (longlong ***)0x0) {
          (*(code *)(*local_78)[1])();
        }
        FUN_14090fcc0(&local_80,&local_a8,&local_res8);
        if (local_res8 != (longlong ***)0x0) {
          FUN_14019f2c0(local_res8 + -2);
        }
        uVar6 = uVar6 & 0xffffffff00000000;
        FUN_1403a0810(DAT_143aa8328,&local_90,*(undefined4 *)(param_1 + 0x11c0),1,uVar6,0,0,0,0xff,0
                      ,0);
        plVar1 = local_80;
        uVar9 = (undefined4)(uVar6 >> 0x20);
        if ((local_80 != (longlong *)0x0) && ((longlong ***)local_90 != (longlong ***)0x0)) {
          uVar3 = FUN_140404280();
          iVar2 = FUN_140319f60(plVar1);
          plVar1 = local_80;
          if ((local_80 == (longlong *)0x0) || ((longlong ***)local_90 == (longlong ***)0x0)) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          iVar4 = FUN_140404240();
          iVar5 = FUN_140319fa0(plVar1);
          local_res8 = (longlong ***)local_90;
          if ((longlong ***)local_90 != (longlong ***)0x0) {
            (*(code *)(*local_90)[1])();
          }
          local_a8 = local_88;
          if (local_88 != (longlong ***)0x0) {
            (*(code *)(*local_88)[1])();
          }
          FUN_142aa1590(&local_a8,&local_res8,(param_2 - (uVar3 >> 1)) + iVar2,
                        (param_3 - iVar4) + iVar5,CONCAT44(uVar9,0xff));
        }
        if ((longlong ***)local_90 != (longlong ***)0x0) {
          (*(code *)(*local_90)[2])();
        }
        if (local_80 != (longlong *)0x0) {
          (**(code **)(*local_80 + 0x10))(local_80);
        }
        if (local_58 != (longlong ***)0x0) {
          (*(code *)(*local_58)[2])();
        }
        if ((longlong ***)local_70 != (longlong ***)0x0) {
          (*(code *)(*local_70)[2])();
        }
        if (local_30 != (longlong ***)0x0) {
          (*(code *)(*local_30)[2])();
        }
        if (local_40 != (longlong ***)0x0) {
          (*(code *)(*local_40)[2])();
        }
        if (local_50 != (longlong *)0x0) {
          (**(code **)(*local_50 + 0x10))();
        }
      }
      if (local_78 != (longlong ***)0x0) {
        (*(code *)(*local_78)[2])();
      }
    }
    if (local_88 != (longlong ***)0x0) {
      (*(code *)(*local_88)[2])();
    }
  }
  return;
}



//===========================================================
// FUN_141199d20 @ 141199d20   (702 bytes)
//===========================================================

void FUN_141199d20(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  int iVar3;
  undefined4 uVar4;
  undefined8 *puVar5;
  int *piVar6;
  int *local_res8;
  undefined8 local_res10;
  undefined8 local_38;
  longlong lStack_30;
  undefined8 local_28;
  
  if (*(longlong *)(param_1 + 0x11b8) != 0) {
    iVar3 = (**(code **)(*(longlong *)(param_1 + 8) + 0x88))(param_1 + 8);
    if (iVar3 != 0) {
      puVar5 = (undefined8 *)FUN_140193080(*(undefined8 *)(param_1 + 0x11b8));
      if (puVar5 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar5[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar5[1] = puVar5[1] + 1;
        UNLOCK();
      }
      if (puVar5 != (undefined8 *)0x0) {
        uVar4 = (**(code **)(*(longlong *)(param_1 + 8) + 0x90))(param_1 + 8);
        local_res10 = CONCAT44(local_res10._4_4_,uVar4);
        iVar3 = (**(code **)(*(longlong *)(param_1 + 8) + 0x98))(param_1 + 8);
        local_res10 = CONCAT44(iVar3 + 0x14,(undefined4)local_res10);
        local_res10 = FUN_142aa2de0(local_res10,param_1 + 0x78);
        local_38 = 1;
        lStack_30 = 0;
        local_28 = 0;
        local_res8 = (int *)0x0;
        piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
        piVar6[1] = 0;
        *piVar6 = -1;
        local_res8 = piVar6 + 4;
        piVar6[2] = 0;
        *(undefined1 *)local_res8 = 0;
        if (*piVar6 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar6[1] < 0) {
          FUN_142e54290(0x90,piVar6[1],0);
        }
        *piVar6 = 1;
        *(undefined1 *)local_res8 = 0;
        if (piVar6[1] + 1 < 1) {
          FUN_142e54290(0x9c,0);
        }
        piVar6[2] = 0;
        FUN_142656500(param_1 + 0x78,local_res10 & 0xffffffff,local_res10._4_4_,puVar5,0,0,0,0,0,0,0
                      ,0,0,0,0,0,0,0,0,0,&local_res8,0,0,0,0,&local_38);
        if (lStack_30 != 0) {
          FUN_14019f2c0(lStack_30 + -0x10);
        }
      }
      if (puVar5 != (undefined8 *)0x0) {
        if (0xffffe < puVar5[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = puVar5 + 1;
        lVar2 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar2 == 1) {
          (**(code **)*puVar5)(puVar5,1);
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_141199ff0 @ 141199ff0   (57 bytes)
//===========================================================

void FUN_141199ff0(longlong param_1,int param_2)

{
  FUN_141711090();
  if (param_2 == 0) {
    FUN_142645170(param_1 + 0x70);
  }
  FUN_14112c0b0(param_1 + -8,param_2 == 0);
  return;
}



//===========================================================
// FUN_14119a250 @ 14119a250   (790 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014119a3af) */

void FUN_14119a250(longlong param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong *plVar2;
  undefined4 uVar3;
  int *piVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  int iVar7;
  int iVar8;
  ulonglong uVar9;
  longlong lVar10;
  ulonglong uVar11;
  longlong *local_res8;
  longlong *local_res18;
  longlong *local_res20;
  longlong *local_48;
  undefined1 local_40 [8];
  
  uVar11 = 0xffffffffffffffff;
  lVar10 = -1;
  do {
    lVar10 = lVar10 + 1;
  } while (L"/backgrnd"[lVar10] != L'\0');
  FUN_14040ea40(&DAT_143acaa38,&local_res20);
  plVar6 = local_res20;
  local_res8 = (longlong *)0x0;
  plVar2 = local_res8;
  if ((local_res20 == (longlong *)0x0) || (plVar1 = local_res20 + -2, plVar1 == (longlong *)0x0))
  goto LAB_14119a417;
  if ((int)*plVar1 != -1) {
    if ((int)*plVar1 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *(int *)plVar1 = (int)*plVar1 + 1;
    UNLOCK();
    if (local_res8 != (longlong *)0x0) {
      FUN_1401bebb0(local_res8 + -2);
    }
    local_res8 = plVar6;
    plVar6 = local_res20;
    plVar2 = local_res8;
    goto LAB_14119a417;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  uVar9 = 0xffffffffffffffff;
  do {
    uVar9 = uVar9 + 1;
  } while (*(short *)((longlong)local_res20 + uVar9 * 2) != 0);
  iVar7 = (int)uVar9;
  iVar8 = 0;
  if (0 < iVar7) {
    iVar8 = iVar7;
  }
  piVar4 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar8 * 2 + 0x12));
  piVar4[1] = iVar8;
  *piVar4 = -1;
  plVar2 = (longlong *)(piVar4 + 4);
  piVar4[2] = 0;
  *(undefined2 *)plVar2 = 0;
  local_48 = plVar2;
  FUN_142ef7ba0(plVar2,local_res20,(longlong)iVar7 * 2);
  if (*piVar4 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar7 == -1) || (iVar7 <= piVar4[1])) {
    *piVar4 = 1;
    if (iVar7 != -1) goto LAB_14119a365;
    if (plVar2 == (longlong *)0x0) {
      uVar9 = 0;
    }
    else {
      do {
        uVar11 = uVar11 + 1;
      } while (*(short *)((longlong)plVar2 + uVar11 * 2) != 0);
      uVar9 = uVar11 & 0xffffffff;
    }
  }
  else {
    FUN_142e54290(0x90,piVar4[1],uVar9 & 0xffffffff);
    *piVar4 = 1;
LAB_14119a365:
    *(undefined2 *)((longlong)plVar2 + (longlong)iVar7 * 2) = 0;
  }
  iVar8 = (int)uVar9;
  if ((iVar8 < 0) || (piVar4[1] + 1 <= iVar8)) {
    FUN_142e54290(0x9c,uVar9 & 0xffffffff);
  }
  piVar4[2] = iVar8 * 2;
  if (local_res8 != (longlong *)0x0) {
    FUN_1401bebb0(local_res8 + -2);
  }
LAB_14119a417:
  local_res8 = plVar2;
  FUN_142748650(param_1,param_2,&local_res8,0);
  FUN_142bfa600(param_1,1);
  FUN_141aa3af0(param_1 + 0x2e0,param_1,0,0);
  FUN_141ac3370(param_1 + 0x2e0,DAT_143acaa38,0,0,0,1,0,0);
  FUN_14090ead0(&local_res18,DAT_143acaa38);
  if (local_res18 != (longlong *)0x0) {
    local_res8 = local_res18;
    (**(code **)(*local_res18 + 8))();
    uVar3 = FUN_140910ca0(&local_res8,"entryMargin",0);
    *(undefined4 *)(param_1 + 0x300) = uVar3;
    local_res8 = local_res18;
    if (local_res18 != (longlong *)0x0) {
      (**(code **)(*local_res18 + 8))();
    }
    uVar3 = FUN_140910ca0(&local_res8,"entryCount",0);
    *(undefined4 *)(param_1 + 0x304) = uVar3;
    local_48 = (longlong *)0x0;
    puVar5 = (undefined8 *)FUN_141adbce0(param_1 + 0x2e0,local_40,L"entryList_lt",&local_48);
    *(undefined8 *)(param_1 + 0x308) = *puVar5;
    FUN_14119a960(param_1);
    if (DAT_143ac8908 != 0) {
      FUN_141198bf0();
    }
  }
  if (local_res18 != (longlong *)0x0) {
    (**(code **)(*local_res18 + 0x10))();
  }
  if (plVar6 != (longlong *)0x0) {
    FUN_1401bebb0(plVar6 + -2);
  }
  return;
}



//===========================================================
// thunk_FUN_142748b10 @ 14119a570   (5 bytes)
//===========================================================

void thunk_FUN_142748b10(void)

{
  FUN_142748b10();
  return;
}



//===========================================================
// FUN_14119a7a0 @ 14119a7a0   (224 bytes)
//===========================================================

void FUN_14119a7a0(longlong *param_1,int param_2,undefined4 param_3,undefined4 param_4)

{
  longlong *plVar1;
  longlong lVar2;
  int iVar3;
  undefined8 *puVar4;
  undefined1 local_18 [8];
  longlong local_10;
  
  FUN_141ada1c0(param_1 + 0x5c,local_18,L"entryScroll");
  if (local_10 != 0) {
    iVar3 = FUN_141710ad0();
    if (iVar3 == param_2) {
      *(undefined1 *)(param_1 + 0x62) = 1;
      (**(code **)(*param_1 + 0x90))(param_1,0);
    }
  }
  FUN_142bf5d40(param_1,param_2,param_3,param_4);
  lVar2 = local_10;
  if (local_10 != 0) {
    if (0xffffe < *(longlong *)(local_10 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      puVar4 = (undefined8 *)(local_10 + 0x18);
      if (local_10 == 0) {
        puVar4 = (undefined8 *)0x0;
      }
      if (puVar4 != (undefined8 *)0x0) {
        (**(code **)*puVar4)(puVar4,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_14119a890 @ 14119a890   (101 bytes)
//===========================================================

void FUN_14119a890(longlong param_1,undefined4 param_2)

{
  char cVar1;
  
  FUN_142748c20();
  cVar1 = FUN_142aa1a20(param_1 + 0x2e0,&DAT_1432a3130,param_2);
  if ((cVar1 != '\0') && (DAT_143aca9f8 != 0)) {
    FUN_142bf3f70();
    if (DAT_143aca9f8 != 0) {
      (*(code *)**(undefined8 **)(DAT_143aca9f8 + 8))(DAT_143aca9f8 + 8,1);
    }
  }
  return;
}


