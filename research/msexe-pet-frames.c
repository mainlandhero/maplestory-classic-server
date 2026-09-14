
//===========================================================
// FUN_141ecaa40 @ 141ecaa40   (448 bytes)
//===========================================================

void FUN_141ecaa40(longlong param_1,undefined8 param_2,uint *param_3)

{
  IUnknown *pIVar1;
  int iVar2;
  uint local_48;
  uint uStack_44;
  undefined8 uStack_40;
  undefined8 local_38;
  uint local_28;
  uint uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 local_18;
  
  if (*(longlong *)(param_1 + 0x3d8) != 0) {
    if ((short)*param_3 == 8) {
      *(short *)param_3 = 0;
      if (*(longlong *)(param_3 + 2) != 0) {
        (*DAT_143ad5990)(*(longlong *)(param_3 + 2) + -4);
      }
    }
    else {
      (*DAT_143262a18)(param_3);
    }
    return;
  }
  pIVar1 = *(IUnknown **)(param_1 + 0x3b8);
  if (pIVar1 != (IUnknown *)0x0) {
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x20))(pIVar1);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14336fcd0);
    }
    pIVar1 = *(IUnknown **)(param_1 + 0x3b8);
    if (pIVar1 != (IUnknown *)0x0) {
      local_48 = *param_3;
      uStack_44 = param_3[1];
      uStack_40 = *(longlong **)(param_3 + 2);
      local_38 = *(undefined8 *)(param_3 + 4);
      iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x40))(pIVar1,&local_48);
      if (iVar2 < 0) {
        _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14336fcd0);
      }
      pIVar1 = *(IUnknown **)(param_1 + 0x3c8);
      if (pIVar1 != (IUnknown *)0x0) {
        uStack_40 = *(longlong **)(param_1 + 0x3b8);
        local_48 = CONCAT22(local_48._2_2_,0xd);
        if (uStack_40 != (longlong *)0x0) {
          (**(code **)(*uStack_40 + 8))();
        }
        local_28 = local_48;
        uStack_24 = uStack_44;
        uStack_20 = (undefined4)uStack_40;
        uStack_1c = uStack_40._4_4_;
        local_18 = local_38;
        iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x238))(pIVar1,&local_28);
        if (iVar2 < 0) {
          _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
        }
        if ((short)local_48 == 8) {
          local_48 = local_48 & 0xffff0000;
          if (uStack_40 != (longlong *)0x0) {
            (*DAT_143ad5990)((longlong)uStack_40 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_48);
        }
        if ((short)*param_3 == 8) {
          *(short *)param_3 = 0;
          if (*(longlong *)(param_3 + 2) != 0) {
            (*DAT_143ad5990)(*(longlong *)(param_3 + 2) + -4);
          }
        }
        else {
          (*DAT_143262a18)(param_3);
        }
        return;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_141eca710 @ 141eca710   (805 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141eca85a) */

void FUN_141eca710(longlong param_1)

{
  code *pcVar1;
  longlong *plVar2;
  IUnknown *pIVar3;
  bool bVar4;
  char cVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  longlong lVar10;
  undefined8 uVar11;
  longlong lVar12;
  longlong *plVar13;
  longlong *plVar14;
  longlong *plVar15;
  bool bVar16;
  undefined1 local_res18 [8];
  undefined1 local_res20 [8];
  short local_98 [4];
  longlong *local_90;
  uint local_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 local_70;
  uint local_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined8 local_58;
  
  if (*(longlong *)(param_1 + 0x3d8) != 0) {
    return;
  }
  iVar6 = FUN_14019a5d0(param_1 + 0x3a0);
  if (iVar6 != 0) {
    return;
  }
  iVar6 = FUN_14019a5d0(param_1 + 0x370);
  if (iVar6 != 0) {
    return;
  }
  lVar12 = DAT_143aa8518;
  if (*(longlong *)(param_1 + 0x120) != 0) {
    lVar12 = *(longlong *)(param_1 + 0x120);
  }
  lVar10 = FUN_141892840();
  plVar15 = (longlong *)0x0;
  if (lVar10 == 0) {
LAB_141eca794:
    bVar4 = false;
  }
  else {
    uVar11 = FUN_141892840();
    cVar5 = FUN_141bc0990(uVar11);
    bVar4 = true;
    if (cVar5 == '\0') goto LAB_141eca794;
  }
  plVar14 = (longlong *)(*(longlong *)(param_1 + 0x118) + -0x20);
  if (*(longlong *)(param_1 + 0x118) == 0) {
    plVar14 = plVar15;
  }
  if (plVar14 == (longlong *)0x0) {
    return;
  }
  plVar13 = plVar15;
  if ((lVar12 == 0) || (*(int *)(lVar12 + 0x3fd0) == 0)) {
LAB_141eca962:
    (*DAT_143262a20)(&local_68);
    iVar6 = FUN_14023c4c0(&local_68,&DAT_143ad4a38);
    if (bVar4) {
      if (iVar6 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar6);
      }
      lVar12 = (**(code **)(*(longlong *)(param_1 + 8) + 0x30))
                         ((longlong *)(param_1 + 8),local_res18);
      iVar6 = *(int *)(lVar12 + 4) * 10 + -0x3ffccbb0;
    }
    else {
      if (iVar6 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar6);
      }
      iVar7 = FUN_1409c6ce0(plVar14);
      iVar8 = FUN_1409c6cc0(plVar14);
      iVar9 = FUN_1409c5080(plVar14);
      iVar6 = 3;
      if (iVar9 != 0) {
        iVar6 = 8;
      }
      iVar6 = iVar6 + (iVar8 * 3000 - iVar7) * 10 + -0x3fff8ada;
    }
  }
  else {
    plVar13 = (longlong *)(lVar12 + 8);
    if (bVar4) {
      pcVar1 = *(code **)(*(longlong *)(param_1 + 8) + 0x30);
      lVar10 = (**(code **)(*plVar13 + 0x30))(plVar13,local_res18);
      iVar6 = *(int *)(lVar10 + 4);
      lVar10 = (*pcVar1)(param_1 + 8,local_res20);
      bVar16 = *(int *)(lVar10 + 4) == iVar6;
    }
    else {
      uVar11 = (**(code **)(*plVar13 + 0x50))();
      iVar6 = FUN_1409c6cc0(uVar11);
      iVar7 = FUN_1409c6cc0(plVar14);
      bVar16 = iVar7 == iVar6;
    }
    plVar13 = (longlong *)0x0;
    if (!bVar16) goto LAB_141eca962;
    plVar2 = *(longlong **)(lVar12 + 0x3fd8);
    plVar13 = plVar15;
    if (plVar2 != (longlong *)0x0) {
      (**(code **)(*plVar2 + 8))(plVar2);
      plVar13 = plVar2;
    }
    if (plVar13 == (longlong *)0x0) goto LAB_141eca962;
    pIVar3 = *(IUnknown **)(param_1 + 0x3c8);
    if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_68);
    iVar6 = (**(code **)(*(longlong *)pIVar3 + 0x230))(pIVar3,&local_68);
    if (iVar6 < 0) {
      _com_issue_errorex(iVar6,pIVar3,(_GUID *)&DAT_14327fcb0);
    }
    local_80 = local_68;
    uStack_7c = uStack_64;
    uStack_78 = (undefined4)uStack_60;
    uStack_74 = uStack_60._4_4_;
    local_70 = local_58;
    local_98[0] = 0xd;
    local_90 = plVar13;
    (**(code **)(*plVar13 + 8))(plVar13);
    cVar5 = FUN_140dba180(&local_80,local_98);
    if (local_98[0] == 8) {
      local_98[0] = 0;
      if (local_90 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)local_90 + -4);
      }
    }
    else {
      (*DAT_143262a18)(local_98);
    }
    if ((short)local_80 == 8) {
      local_80 = local_80 & 0xffff0000;
      if (CONCAT44(uStack_74,uStack_78) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_74,uStack_78) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_80);
    }
    if (cVar5 != '\0') goto LAB_141eca9fd;
    local_68 = CONCAT22(local_68._2_2_,0xd);
    uStack_60 = plVar13;
    (**(code **)(*plVar13 + 8))(plVar13);
    iVar6 = 1;
  }
  FUN_141ecaa40(param_1,iVar6,&local_68);
LAB_141eca9fd:
  if (plVar13 != (longlong *)0x0) {
    (**(code **)(*plVar13 + 0x10))(plVar13);
  }
  return;
}



//===========================================================
// FUN_1409bd0e0 @ 1409bd0e0   (24 bytes)
//===========================================================

bool FUN_1409bd0e0(void)

{
  longlong lVar1;
  
  lVar1 = FUN_1409dc2b0();
  return lVar1 != 0;
}


