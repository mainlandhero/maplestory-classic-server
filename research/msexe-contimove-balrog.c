
//===========================================================
// FUN_140d6b130 @ 140d6b130   (3167 bytes)
//===========================================================

void FUN_140d6b130(longlong param_1)

{
  IUnknown *pIVar1;
  longlong *plVar2;
  int iVar3;
  longlong *plVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  IUnknown *local_res8;
  IUnknown *local_res10;
  longlong *local_res18;
  longlong *local_res20;
  undefined8 in_stack_fffffffffffffe18;
  uint uVar7;
  undefined8 in_stack_fffffffffffffe20;
  uint uVar8;
  undefined8 in_stack_fffffffffffffe30;
  undefined4 uVar9;
  short local_168;
  undefined6 uStack_166;
  longlong lStack_160;
  undefined8 local_158;
  short local_150;
  undefined6 uStack_14e;
  longlong lStack_148;
  undefined8 local_140;
  short local_138;
  undefined6 uStack_136;
  longlong lStack_130;
  undefined8 local_128;
  short local_120;
  undefined6 uStack_11e;
  longlong lStack_118;
  undefined8 local_110;
  uint local_108;
  undefined4 uStack_104;
  int iStack_100;
  undefined4 uStack_fc;
  undefined8 local_f8;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 local_c8;
  longlong lStack_c0;
  undefined8 local_b8;
  undefined8 local_a8;
  longlong lStack_a0;
  undefined8 local_98;
  undefined8 local_88;
  longlong lStack_80;
  undefined8 local_78;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  IUnknown **local_48;
  short *local_40;
  
  uVar7 = (uint)((ulonglong)in_stack_fffffffffffffe18 >> 0x20);
  uVar8 = (uint)((ulonglong)in_stack_fffffffffffffe20 >> 0x20);
  uVar9 = (undefined4)((ulonglong)in_stack_fffffffffffffe30 >> 0x20);
  pIVar1 = *(IUnknown **)(param_1 + 0x60);
  if ((pIVar1 == (IUnknown *)0x0) || (*(int *)(param_1 + 0x20) != 1)) {
    return;
  }
  iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x80))
                    (pIVar1,*(undefined4 *)(param_1 + 0x24),*(undefined4 *)(param_1 + 0x2c));
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273780);
  }
  local_48 = &local_res8;
  uVar6 = 0;
  local_res8 = (IUnknown *)0x0;
  local_40 = &local_168;
  lStack_160 = 0;
  local_res10 = (IUnknown *)0x0;
  local_res18 = *(longlong **)(param_1 + 0x60);
  if (local_res18 != (longlong *)0x0) {
    (**(code **)(*local_res18 + 8))();
  }
  uVar5 = uVar6;
  if (*(ulonglong **)(param_1 + 0x50) != (ulonglong *)0x0) {
    uVar5 = **(ulonglong **)(param_1 + 0x50);
  }
  plVar4 = (longlong *)
           FUN_140dc12e0(&local_res20,uVar5,*(int *)(param_1 + 0x38) != 0,&local_res18,
                         (ulonglong)uVar7 << 0x20,(ulonglong)uVar8 << 0x20,&local_res10,
                         CONCAT44(uVar9,0xc0061378),0xff,0,&local_168,0,0,0,0,&local_res8,0,0,0);
  plVar2 = *(longlong **)(param_1 + 0x58);
  if (plVar2 != (longlong *)*plVar4) {
    *(longlong **)(param_1 + 0x58) = (longlong *)*plVar4;
    *plVar4 = 0;
    if (plVar2 != (longlong *)0x0) {
      (**(code **)(*plVar2 + 0x10))();
    }
  }
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))();
  }
  pIVar1 = *(IUnknown **)(param_1 + 0x58);
  if (pIVar1 != (IUnknown *)0x0) {
    local_res8 = (IUnknown *)0x0;
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x208))(pIVar1,&local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    pIVar1 = local_res8;
    local_res10 = local_res8;
    if (local_res8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar3 = (**(code **)(*(longlong *)local_res8 + 0x80))(local_res8,0,0);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273780);
    }
    (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
    if (*(int *)(param_1 + 0x20) == 0) {
      return;
    }
    pIVar1 = *(IUnknown **)(param_1 + 0x58);
    if (pIVar1 != (IUnknown *)0x0) {
      local_res8 = (IUnknown *)0x0;
      iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x208))(pIVar1,&local_res8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
      pIVar1 = local_res8;
      local_res10 = local_res8;
      if (local_res8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      iVar3 = (**(code **)(*(longlong *)local_res8 + 0x80))(local_res8,0,0);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273780);
      }
      (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
      pIVar1 = *(IUnknown **)(param_1 + 0x58);
      if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_res8 = (IUnknown *)0x0;
      iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x208))(pIVar1,&local_res8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
      pIVar1 = local_res8;
      local_res10 = local_res8;
      if (local_res8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_120);
      if (DAT_143a8b8d8 == 8) {
        if (local_120 == 8) {
          local_120 = 0;
          if (lStack_118 != 0) {
            (*DAT_143ad5990)(lStack_118 + -4);
          }
        }
        else {
          iVar3 = (*DAT_143262a18)(&local_120);
          if (iVar3 < 0) goto LAB_140d6bd12;
        }
        local_120 = 8;
        uVar5 = uVar6;
        if (DAT_143a8b8e0 != 0) {
          uVar5 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_118 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
      }
      else {
        if ((local_120 == 8) && (local_120 = 0, lStack_118 != 0)) {
          (*DAT_143ad5990)(lStack_118 + -4);
        }
        iVar3 = (*DAT_143262a28)(&local_120,&DAT_143a8b8d8);
        if (iVar3 < 0) {
LAB_140d6bd12:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
      }
      (*DAT_143262a20)(&local_138);
      if (DAT_143a8b8d8 == 8) {
        if (local_138 == 8) {
          local_138 = 0;
          if (lStack_130 != 0) {
            (*DAT_143ad5990)(lStack_130 + -4);
          }
        }
        else {
          iVar3 = (*DAT_143262a18)(&local_138);
          if (iVar3 < 0) goto LAB_140d6bd1a;
        }
        local_138 = 8;
        uVar5 = uVar6;
        if (DAT_143a8b8e0 != 0) {
          uVar5 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_130 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
      }
      else {
        if ((local_138 == 8) && (local_138 = 0, lStack_130 != 0)) {
          (*DAT_143ad5990)(lStack_130 + -4);
        }
        iVar3 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
        if (iVar3 < 0) {
LAB_140d6bd1a:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
      }
      (*DAT_143262a20)(&local_150);
      if (DAT_143a8b8d8 == 8) {
        if (local_150 == 8) {
          local_150 = 0;
          if (lStack_148 != 0) {
            (*DAT_143ad5990)(lStack_148 + -4);
          }
        }
        else {
          iVar3 = (*DAT_143262a18)(&local_150);
          if (iVar3 < 0) goto LAB_140d6bd22;
        }
        local_150 = 8;
        uVar5 = uVar6;
        if (DAT_143a8b8e0 != 0) {
          uVar5 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_148 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
      }
      else {
        if ((local_150 == 8) && (local_150 = 0, lStack_148 != 0)) {
          (*DAT_143ad5990)(lStack_148 + -4);
        }
        iVar3 = (*DAT_143262a28)(&local_150,&DAT_143a8b8d8);
        if (iVar3 < 0) {
LAB_140d6bd22:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
      }
      (*DAT_143262a20)(&local_168);
      if (DAT_143a8b8d8 == 8) {
        if (local_168 == 8) {
          local_168 = 0;
          if (lStack_160 != 0) {
            (*DAT_143ad5990)(lStack_160 + -4);
          }
        }
        else {
          iVar3 = (*DAT_143262a18)(&local_168);
          if (iVar3 < 0) goto LAB_140d6bd2a;
        }
        local_168 = 8;
        uVar5 = uVar6;
        if (DAT_143a8b8e0 != 0) {
          uVar5 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_160 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
      }
      else {
        if ((local_168 == 8) && (local_168 = 0, lStack_160 != 0)) {
          (*DAT_143ad5990)(lStack_160 + -4);
        }
        iVar3 = (*DAT_143262a28)(&local_168,&DAT_143a8b8d8);
        if (iVar3 < 0) {
LAB_140d6bd2a:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
      }
      if (*(longlong *)(param_1 + 0x58) == 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      iStack_100 = FUN_140458d90();
      iStack_100 = *(int *)(param_1 + 0x34) * 1000 + iStack_100;
      local_108 = CONCAT22(local_108._2_2_,3);
      local_e8 = CONCAT62(uStack_11e,local_120);
      uStack_e0 = lStack_118;
      local_d8 = local_110;
      local_c8 = CONCAT62(uStack_136,local_138);
      lStack_c0 = lStack_130;
      local_b8 = local_128;
      local_a8 = CONCAT62(uStack_14e,local_150);
      lStack_a0 = lStack_148;
      local_98 = local_140;
      local_88 = CONCAT62(uStack_166,local_168);
      lStack_80 = lStack_160;
      local_78 = local_158;
      local_68 = CONCAT44(uStack_104,local_108);
      uStack_60 = CONCAT44(uStack_fc,iStack_100);
      local_58 = local_f8;
      iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x140))
                        (pIVar1,0xff,0xff,&local_68,&local_88,&local_a8,&local_c8,&local_e8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273488);
      }
      if ((short)local_108 == 8) {
        local_108 = local_108 & 0xffff0000;
        if (CONCAT44(uStack_fc,iStack_100) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_fc,iStack_100) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_108);
      }
      if (local_168 == 8) {
        local_168 = 0;
        if (lStack_160 != 0) {
          (*DAT_143ad5990)(lStack_160 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_168);
      }
      if (local_150 == 8) {
        local_150 = 0;
        if (lStack_148 != 0) {
          (*DAT_143ad5990)(lStack_148 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_150);
      }
      if (local_138 == 8) {
        local_138 = 0;
        if (lStack_130 != 0) {
          (*DAT_143ad5990)(lStack_130 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_138);
      }
      if (local_120 == 8) {
        local_120 = 0;
        if (lStack_118 != 0) {
          (*DAT_143ad5990)(lStack_118 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_120);
      }
      (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
      pIVar1 = *(IUnknown **)(param_1 + 0x60);
      if (pIVar1 != (IUnknown *)0x0) {
        iVar3 = 100;
        if (*(int *)(param_1 + 0x38) != 0) {
          iVar3 = -100;
        }
        iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x80))
                          (pIVar1,iVar3 + *(int *)(param_1 + 0x24),*(int *)(param_1 + 0x2c) + -100);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273780);
        }
        pIVar1 = *(IUnknown **)(param_1 + 0x60);
        if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(&local_168);
        if (DAT_143a8b8d8 == 8) {
          if (local_168 == 8) {
            local_168 = 0;
            if (lStack_160 != 0) {
              (*DAT_143ad5990)(lStack_160 + -4);
            }
          }
          else {
            iVar3 = (*DAT_143262a18)(&local_168);
            if (iVar3 < 0) goto LAB_140d6bd32;
          }
          local_168 = 8;
          uVar5 = uVar6;
          if (DAT_143a8b8e0 != 0) {
            uVar5 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
          lStack_160 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
        }
        else {
          if ((local_168 == 8) && (local_168 = 0, lStack_160 != 0)) {
            (*DAT_143ad5990)(lStack_160 + -4);
          }
          iVar3 = (*DAT_143262a28)(&local_168,&DAT_143a8b8d8);
          if (iVar3 < 0) {
LAB_140d6bd32:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
        }
        (*DAT_143262a20)(&local_150);
        if (DAT_143a8b8d8 == 8) {
          if (local_150 == 8) {
            local_150 = 0;
            if (lStack_148 != 0) {
              (*DAT_143ad5990)(lStack_148 + -4);
            }
          }
          else {
            iVar3 = (*DAT_143262a18)(&local_150);
            if (iVar3 < 0) goto LAB_140d6bd3a;
          }
          local_150 = 8;
          uVar5 = uVar6;
          if (DAT_143a8b8e0 != 0) {
            uVar5 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
          lStack_148 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
        }
        else {
          if ((local_150 == 8) && (local_150 = 0, lStack_148 != 0)) {
            (*DAT_143ad5990)(lStack_148 + -4);
          }
          iVar3 = (*DAT_143262a28)(&local_150,&DAT_143a8b8d8);
          if (iVar3 < 0) {
LAB_140d6bd3a:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
        }
        (*DAT_143262a20)(&local_138);
        if (DAT_143a8b8d8 == 8) {
          if (local_138 == 8) {
            local_138 = 0;
            if (lStack_130 != 0) {
              (*DAT_143ad5990)(lStack_130 + -4);
            }
          }
          else {
            iVar3 = (*DAT_143262a18)(&local_138);
            if (iVar3 < 0) goto LAB_140d6bd42;
          }
          local_138 = 8;
          uVar5 = uVar6;
          if (DAT_143a8b8e0 != 0) {
            uVar5 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
          lStack_130 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
        }
        else {
          if ((local_138 == 8) && (local_138 = 0, lStack_130 != 0)) {
            (*DAT_143ad5990)(lStack_130 + -4);
          }
          iVar3 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
          if (iVar3 < 0) {
LAB_140d6bd42:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
        }
        (*DAT_143262a20)(&local_120);
        if (DAT_143a8b8d8 == 8) {
          if (local_120 == 8) {
            local_120 = 0;
            if (lStack_118 != 0) {
              (*DAT_143ad5990)(lStack_118 + -4);
            }
          }
          else {
            iVar3 = (*DAT_143262a18)(&local_120);
            if (iVar3 < 0) goto LAB_140d6bd4a;
          }
          local_120 = 8;
          if (DAT_143a8b8e0 != 0) {
            uVar6 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
          lStack_118 = FUN_1401a5fa0(DAT_143a8b8e0,uVar6);
        }
        else {
          if ((local_120 == 8) && (local_120 = 0, lStack_118 != 0)) {
            (*DAT_143ad5990)(lStack_118 + -4);
          }
          iVar3 = (*DAT_143262a28)(&local_120,&DAT_143a8b8d8);
          if (iVar3 < 0) {
LAB_140d6bd4a:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
        }
        if (*(longlong *)(param_1 + 0x58) == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        iStack_100 = FUN_140458d90();
        iStack_100 = *(int *)(param_1 + 0x34) * 1000 + iStack_100;
        local_108 = CONCAT22(local_108._2_2_,3);
        local_68 = CONCAT62(uStack_166,local_168);
        uStack_60 = lStack_160;
        local_58 = local_158;
        local_88 = CONCAT62(uStack_14e,local_150);
        lStack_80 = lStack_148;
        local_78 = local_140;
        local_a8 = CONCAT62(uStack_136,local_138);
        lStack_a0 = lStack_130;
        local_98 = local_128;
        local_c8 = CONCAT62(uStack_11e,local_120);
        lStack_c0 = lStack_118;
        local_b8 = local_110;
        local_e8 = CONCAT44(uStack_104,local_108);
        uStack_e0 = CONCAT44(uStack_fc,iStack_100);
        local_d8 = local_f8;
        iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x140))
                          (pIVar1,*(undefined4 *)(param_1 + 0x24),*(undefined4 *)(param_1 + 0x2c),
                           &local_e8,&local_c8,&local_a8,&local_88,&local_68);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273488);
        }
        if ((short)local_108 == 8) {
          local_108 = local_108 & 0xffff0000;
          if (CONCAT44(uStack_fc,iStack_100) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_fc,iStack_100) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_108);
        }
        if (local_120 == 8) {
          local_120 = 0;
          if (lStack_118 != 0) {
            (*DAT_143ad5990)(lStack_118 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_120);
        }
        if (local_138 == 8) {
          local_138 = 0;
          if (lStack_130 != 0) {
            (*DAT_143ad5990)(lStack_130 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_138);
        }
        if (local_150 == 8) {
          local_150 = 0;
          if (lStack_148 != 0) {
            (*DAT_143ad5990)(lStack_148 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_150);
        }
        if (local_168 == 8) {
          local_168 = 0;
          if (lStack_160 == 0) {
            return;
          }
          (*DAT_143ad5990)(lStack_160 + -4);
          return;
        }
        (*DAT_143262a18)(&local_168);
        return;
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_140d6bdb0 @ 140d6bdb0   (1776 bytes)
//===========================================================

void FUN_140d6bdb0(longlong param_1)

{
  IUnknown *pIVar1;
  bool bVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  IUnknown *pIVar6;
  undefined8 local_res8;
  IUnknown *local_res10;
  IUnknown *local_res18;
  IUnknown *local_res20;
  short local_158;
  undefined6 uStack_156;
  longlong lStack_150;
  undefined8 local_148;
  short local_140;
  undefined6 uStack_13e;
  longlong lStack_138;
  undefined8 local_130;
  short local_128;
  undefined6 uStack_126;
  longlong lStack_120;
  undefined8 local_118;
  short local_110;
  undefined6 uStack_10e;
  longlong lStack_108;
  undefined8 local_100;
  uint local_f8;
  undefined4 uStack_f4;
  int iStack_f0;
  undefined4 uStack_ec;
  undefined8 local_e8;
  undefined8 local_d8;
  longlong lStack_d0;
  undefined8 local_c8;
  undefined8 local_b8;
  longlong lStack_b0;
  undefined8 local_a8;
  undefined8 local_98;
  longlong lStack_90;
  undefined8 local_88;
  undefined8 local_78;
  longlong lStack_70;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  int iStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  uVar5 = 0;
  local_res10 = (IUnknown *)((ulonglong)local_res10 & 0xffffffff00000000);
  pIVar1 = *(IUnknown **)(param_1 + 0x58);
  pIVar6 = local_res20;
  uVar4 = uVar5;
  if ((pIVar1 == (IUnknown *)0x0) || (*(int *)(param_1 + 0x20) != 1)) {
LAB_140d6be83:
    bVar2 = true;
  }
  else {
    local_res18 = (IUnknown *)0x0;
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x208))(pIVar1,&local_res18);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    pIVar6 = local_res18;
    local_res20 = local_res18;
    uVar4 = 3;
    local_res10 = (IUnknown *)CONCAT44(local_res10._4_4_,3);
    if (local_res18 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8 = (IUnknown *)((ulonglong)local_res8._4_4_ << 0x20);
    iVar3 = (**(code **)(*(longlong *)local_res18 + 0x40))(local_res18,&local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar6,(_GUID *)&DAT_143273780);
    }
    if ((int)local_res8 == 0) goto LAB_140d6be83;
    bVar2 = false;
  }
  if (((uVar4 & 1) != 0) && (pIVar6 != (IUnknown *)0x0)) {
    (**(code **)(*(longlong *)pIVar6 + 0x10))(pIVar6);
  }
  if (!bVar2) {
    pIVar1 = *(IUnknown **)(param_1 + 0x58);
    if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8 = (IUnknown *)0x0;
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x208))(pIVar1,&local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    pIVar1 = local_res8;
    local_res10 = local_res8;
    if (local_res8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar3 = (**(code **)(*(longlong *)local_res8 + 0x80))(local_res8,0xff,0xff);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273780);
    }
    (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
    pIVar1 = *(IUnknown **)(param_1 + 0x58);
    if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8 = (IUnknown *)0x0;
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x208))(pIVar1,&local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    pIVar1 = local_res8;
    local_res10 = local_res8;
    if (local_res8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_110);
    if (DAT_143a8b8d8 == 8) {
      if (local_110 == 8) {
        local_110 = 0;
        if (lStack_108 != 0) {
          (*DAT_143ad5990)(lStack_108 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_110);
        if (iVar3 < 0) goto LAB_140d6c453;
      }
      local_110 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_108 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_110 == 8) && (local_110 = 0, lStack_108 != 0)) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_110,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6c453:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_128);
    if (DAT_143a8b8d8 == 8) {
      if (local_128 == 8) {
        local_128 = 0;
        if (lStack_120 != 0) {
          (*DAT_143ad5990)(lStack_120 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_128);
        if (iVar3 < 0) goto LAB_140d6c45b;
      }
      local_128 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_128 == 8) && (local_128 = 0, lStack_120 != 0)) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_128,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6c45b:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_140);
    if (DAT_143a8b8d8 == 8) {
      if (local_140 == 8) {
        local_140 = 0;
        if (lStack_138 != 0) {
          (*DAT_143ad5990)(lStack_138 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_140);
        if (iVar3 < 0) goto LAB_140d6c463;
      }
      local_140 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_138 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_140 == 8) && (local_140 = 0, lStack_138 != 0)) {
        (*DAT_143ad5990)(lStack_138 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_140,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6c463:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_158);
    if (DAT_143a8b8d8 == 8) {
      if (local_158 == 8) {
        local_158 = 0;
        if (lStack_150 != 0) {
          (*DAT_143ad5990)(lStack_150 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_158);
        if (iVar3 < 0) goto LAB_140d6c46b;
      }
      local_158 = 8;
      if (DAT_143a8b8e0 != 0) {
        uVar5 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_150 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
    }
    else {
      if ((local_158 == 8) && (local_158 = 0, lStack_150 != 0)) {
        (*DAT_143ad5990)(lStack_150 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_158,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6c46b:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    pIVar6 = *(IUnknown **)(param_1 + 0x58);
    if (pIVar6 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8 = (IUnknown *)((ulonglong)local_res8 & 0xffffffff00000000);
    iVar3 = (**(code **)(*(longlong *)pIVar6 + 0xb0))(pIVar6,&local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar6,(_GUID *)&DAT_143273488);
    }
    iStack_f0 = *(int *)(param_1 + 0x34) * 1000 + (int)local_res8;
    local_f8 = CONCAT22(local_f8._2_2_,3);
    local_d8 = CONCAT62(uStack_10e,local_110);
    lStack_d0 = lStack_108;
    local_c8 = local_100;
    local_b8 = CONCAT62(uStack_126,local_128);
    lStack_b0 = lStack_120;
    local_a8 = local_118;
    local_98 = CONCAT62(uStack_13e,local_140);
    lStack_90 = lStack_138;
    local_88 = local_130;
    local_78 = CONCAT62(uStack_156,local_158);
    lStack_70 = lStack_150;
    local_68 = local_148;
    local_58 = local_f8;
    uStack_54 = uStack_f4;
    uStack_4c = uStack_ec;
    local_48 = local_e8;
    iStack_50 = iStack_f0;
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x140))
                      (pIVar1,0,0,&local_58,&local_78,&local_98,&local_b8,&local_d8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273488);
    }
    if ((short)local_f8 == 8) {
      local_f8 = local_f8 & 0xffff0000;
      if (CONCAT44(uStack_ec,iStack_f0) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_ec,iStack_f0) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_f8);
    }
    if (local_158 == 8) {
      local_158 = 0;
      if (lStack_150 != 0) {
        (*DAT_143ad5990)(lStack_150 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_158);
    }
    if (local_140 == 8) {
      local_140 = 0;
      if (lStack_138 != 0) {
        (*DAT_143ad5990)(lStack_138 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_140);
    }
    if (local_128 == 8) {
      local_128 = 0;
      if (lStack_120 != 0) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_128);
    }
    if (local_110 == 8) {
      local_110 = 0;
      if (lStack_108 != 0) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_110);
    }
    (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
  }
  return;
}


