
//===========================================================
// FUN_141ec2690 @ 141ec2690   (873 bytes)
//===========================================================

void FUN_141ec2690(longlong *param_1)

{
  IUnknown *pIVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong *local_res8;
  uint *local_res10;
  undefined1 local_res18 [8];
  uint local_58;
  undefined4 uStack_54;
  undefined8 uStack_50;
  undefined8 local_48;
  uint local_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined8 local_28;
  
  if ((param_1[0x24] == 0) || (param_1[0x79] == 0)) {
    return;
  }
  local_res8 = (longlong *)0x0;
  FUN_141ec2a10(param_1,&local_res8);
  if (param_1[0x7d] == 0) {
    if (param_1[0x7b] == 0) {
      if (param_1[0x77] != 0) {
        local_res10 = &local_58;
        (*DAT_143262a20)(&local_58);
        iVar3 = FUN_14023c4c0(&local_58,&DAT_143ad4a38);
        if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
        pIVar1 = (IUnknown *)param_1[0x77];
        if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        local_res8 = (longlong *)((ulonglong)local_res8 & 0xffffffff00000000);
        iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x18))(pIVar1,&local_res8);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14336fcd0);
        }
        FUN_141ecaa40(param_1,(ulonglong)local_res8 & 0xffffffff,&local_58);
        FUN_141eca710(param_1);
      }
    }
    else {
      pIVar1 = (IUnknown *)param_1[0x79];
      if (pIVar1 == (IUnknown *)0x0) goto LAB_141ec29eb;
      iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x198))(pIVar1,1);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
      pIVar1 = (IUnknown *)param_1[0x79];
      if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      uStack_50 = (longlong *)param_1[0x7b];
      local_58 = CONCAT22(local_58._2_2_,0xd);
      if (uStack_50 != (longlong *)0x0) {
        (**(code **)(*uStack_50 + 8))();
      }
      local_38 = local_58;
      uStack_34 = uStack_54;
      uStack_30 = (undefined4)uStack_50;
      uStack_2c = uStack_50._4_4_;
      local_28 = local_48;
      iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x238))(pIVar1,&local_38);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
      if ((short)local_58 == 8) {
        local_58 = local_58 & 0xffff0000;
        if (uStack_50 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_50 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_58);
      }
    }
  }
  else {
    pIVar1 = (IUnknown *)param_1[0x79];
    if (pIVar1 == (IUnknown *)0x0) {
LAB_141ec29eb:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x198))(pIVar1,1);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    local_res8 = (longlong *)param_1[0x7b];
    if (local_res8 != (longlong *)0x0) {
      (**(code **)(*local_res8 + 8))();
    }
    local_res10 = (uint *)param_1[0x7d];
    if (local_res10 != (uint *)0x0) {
      (**(code **)(*(longlong *)local_res10 + 8))();
    }
    uVar4 = FUN_140f15810(local_res18,&local_res10,&local_res8,0);
    FUN_141ec2580(param_1,uVar4);
  }
  pIVar1 = (IUnknown *)param_1[0x79];
  if (pIVar1 != (IUnknown *)0x0) {
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x200))(pIVar1,0xffffffff);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    pIVar1 = (IUnknown *)param_1[0x79];
    if (pIVar1 != (IUnknown *)0x0) {
      local_res8 = (longlong *)((ulonglong)local_res8 & 0xffffffff00000000);
      iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x2b0))(pIVar1,&local_res8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
      if (param_1[0x24] == 0) {
        return;
      }
      if (((((int)local_res8 == 0) || (iVar3 = FUN_140f8abc0(param_1[0x24] + 0x100), iVar3 != 0)) ||
          (cVar2 = FUN_142826340(param_1[0x24]), cVar2 != '\0')) ||
         ((cVar2 = FUN_140f80830(param_1[0x24] + 0x100), cVar2 != '\0' ||
          (cVar2 = FUN_140f80860(param_1[0x24] + 0x100), cVar2 != '\0')))) {
        lVar5 = *param_1;
        uVar4 = 0;
      }
      else {
        lVar5 = *param_1;
        uVar4 = 1;
      }
      (**(code **)(lVar5 + 0x18))(param_1,uVar4,0);
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_14276e860 @ 14276e860   (27 bytes)
//===========================================================

void FUN_14276e860(longlong param_1)

{
  undefined8 uVar1;
  
  uVar1 = (**(code **)(*(longlong *)(param_1 + 8) + 0x48))(param_1 + 8);
  FUN_1409bd0e0(uVar1);
  return;
}



//===========================================================
// FUN_141ebe350 @ 141ebe350   (183 bytes)
//===========================================================

undefined8 FUN_141ebe350(longlong param_1,uint *param_2)

{
  uint uVar1;
  int iVar2;
  undefined8 uVar3;
  
  uVar1 = FUN_1401b0340(param_1 + 0x2d8);
  if (param_2 != (uint *)0x0) {
    *param_2 = uVar1 & 1;
  }
  switch((int)uVar1 >> 1) {
  default:
    uVar3 = 0;
    break;
  case 2:
    uVar3 = 1;
    break;
  case 3:
    uVar3 = 3;
    break;
  case 6:
    uVar3 = 4;
    break;
  case 0xb:
    uVar3 = 2;
    break;
  case 0xc:
    uVar3 = 5;
    break;
  case 0xd:
    uVar3 = 6;
    break;
  case 0xe:
    uVar3 = 7;
    break;
  case 0xf:
    uVar3 = 8;
  }
  iVar2 = FUN_1401b0340(param_1 + 0x308);
  if (-1 < iVar2) {
    uVar3 = FUN_1401b0340(param_1 + 0x308);
    return uVar3;
  }
  return uVar3;
}



//===========================================================
// FUN_141ec22f0 @ 141ec22f0   (641 bytes)
//===========================================================

void FUN_141ec22f0(longlong param_1,longlong *param_2,longlong *param_3,undefined4 param_4)

{
  IUnknown *pIVar1;
  int iVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  longlong *local_res8;
  longlong *local_res10;
  longlong *local_res18;
  longlong *local_88;
  undefined1 local_80 [8];
  uint local_78;
  undefined4 uStack_74;
  undefined8 uStack_70;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  local_res10 = param_2;
  local_res18 = param_3;
  if (*(longlong **)(param_1 + 0x120) == (longlong *)0x0) {
    if ((longlong *)*param_2 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_2 + 0x10))();
    }
  }
  else if (*(longlong *)(param_1 + 0x3c8) == 0) {
    if ((longlong *)*param_2 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_2 + 0x10))();
    }
  }
  else {
    iVar2 = (**(code **)(**(longlong **)(param_1 + 0x120) + 0x58))();
    iVar3 = FUN_1401b0340(param_1 + 0x308);
    if (-1 < iVar3) {
      FUN_141ebdf10(param_1);
      FUN_141ec87b0(param_1);
    }
    if (iVar2 == 0) {
      FUN_141593a20(*(undefined8 *)(param_1 + 0x40));
    }
    local_res8 = (longlong *)*param_3;
    if (local_res8 != (longlong *)0x0) {
      (**(code **)(*local_res8 + 8))();
    }
    FUN_141ec2a10(param_1,&local_res8);
    if (*(longlong *)(param_1 + 1000) == 0) {
      if (*(longlong *)(param_1 + 0x3d8) == 0) {
        uStack_70 = (longlong *)*param_2;
        local_78 = CONCAT22(local_78._2_2_,0xd);
        if (uStack_70 != (longlong *)0x0) {
          (**(code **)(*uStack_70 + 8))();
        }
        FUN_141ecaa40(param_1,param_4,&local_78);
      }
      else {
        if (*(longlong *)(param_1 + 0x3c8) == 0) goto LAB_141ec255d;
        FUN_140dbb710(*(longlong *)(param_1 + 0x3c8),param_4);
        pIVar1 = *(IUnknown **)(param_1 + 0x3c8);
        if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        uStack_70 = (longlong *)*param_2;
        local_78 = CONCAT22(local_78._2_2_,0xd);
        if (uStack_70 != (longlong *)0x0) {
          (**(code **)(*uStack_70 + 8))();
        }
        local_58 = local_78;
        uStack_54 = uStack_74;
        uStack_50 = (undefined4)uStack_70;
        uStack_4c = uStack_70._4_4_;
        local_48 = local_68;
        lVar4 = (**(code **)(*(longlong *)pIVar1 + 0x238))(pIVar1,&local_58);
        if (lVar4 < 0) {
          _com_issue_errorex(lVar4,pIVar1,(_GUID *)&DAT_14327fcb0);
        }
        if ((short)local_78 == 8) {
          local_78 = local_78 & 0xffff0000;
          if (uStack_70 != (longlong *)0x0) {
            (*DAT_143ad5990)((longlong)uStack_70 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_78);
        }
      }
    }
    else {
      pIVar1 = *(IUnknown **)(param_1 + 0x3c8);
      if (pIVar1 == (IUnknown *)0x0) {
LAB_141ec255d:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x198))(pIVar1,param_4);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
      local_res8 = (longlong *)*param_2;
      if (local_res8 != (longlong *)0x0) {
        (**(code **)(*local_res8 + 8))();
      }
      local_88 = *(longlong **)(param_1 + 1000);
      if (local_88 != (longlong *)0x0) {
        (**(code **)(*local_88 + 8))();
      }
      uVar5 = FUN_140f15810(local_80,&local_88,&local_res8,0);
      FUN_141ec2580(param_1,uVar5);
    }
    FUN_141ec2250(param_1,iVar2 != 0);
    if ((longlong *)*param_2 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_2 + 0x10))();
    }
  }
  if ((longlong *)*param_3 != (longlong *)0x0) {
    (**(code **)(*(longlong *)*param_3 + 0x10))();
  }
  return;
}


