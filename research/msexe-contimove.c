
//===========================================================
// FUN_140d6cc90 @ 140d6cc90   (226 bytes)
//===========================================================

void FUN_140d6cc90(longlong param_1,int param_2,undefined8 param_3)

{
  undefined1 uVar1;
  char cVar2;
  
  if (param_2 == 0x1bf) {
    FUN_140d6cda0(param_1 + -0x18,param_3);
    return;
  }
  if (param_2 == 0x1c0) {
    uVar1 = FUN_1406e8ae0(param_3);
    cVar2 = FUN_1406e8ae0(param_3);
    switch(uVar1) {
    case 0:
    case 1:
    case 6:
      if (*(int *)(param_1 + 0xf8) == 0) {
        FUN_140d6aba0(param_1 + 0xd8);
      }
      break;
    case 3:
    case 4:
      if ((*(int *)(param_1 + 0xf8) == 1) && (cVar2 == '\x01')) {
        FUN_140d6b130(param_1 + 0xd8);
      }
    case 2:
    case 5:
      if (*(int *)(param_1 + 0xf8) == 0) {
        FUN_140d6a610(param_1 + 0xd8);
        return;
      }
    }
    return;
  }
  FUN_141820080(param_1);
  return;
}



//===========================================================
// FUN_140d6cda0 @ 140d6cda0   (556 bytes)
//===========================================================

void FUN_140d6cda0(longlong param_1,undefined8 param_2)

{
  longlong *plVar1;
  int *piVar2;
  int iVar3;
  longlong lVar4;
  undefined1 uVar5;
  char cVar6;
  undefined8 *puVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  undefined1 local_res18 [8];
  undefined8 **local_res20;
  undefined8 *local_80;
  undefined8 *local_78;
  longlong local_70;
  longlong *local_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 uStack_48;
  undefined8 local_40;
  undefined4 local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 local_18;
  undefined4 local_10;
  
  uVar5 = FUN_1406e8ae0(param_2);
  switch(uVar5) {
  case 8:
    cVar6 = FUN_1406e8ae0(param_2);
    if (cVar6 == '\x02') {
      FUN_140d6a610(param_1 + 0xf0);
      return;
    }
    break;
  case 10:
    cVar6 = FUN_1406e8ae0(param_2);
    if (cVar6 == '\x04') {
      FUN_140d6b130(param_1 + 0xf0);
      FUN_14208a6d0(DAT_143abfea0,&local_70);
      if (local_70 != 0) {
        local_50 = 1;
        puVar9 = (undefined8 *)0x0;
        uStack_48 = 0;
        local_40 = 0;
        local_res20 = &local_80;
        puVar7 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
        if (puVar7 != (undefined8 *)0x0) {
          *puVar7 = 0;
          puVar7[1] = 0;
          *(undefined4 *)(puVar7 + 1) = 1;
          *(undefined4 *)((longlong)puVar7 + 0xc) = 1;
          *puVar7 = &PTR_LAB_14336d918;
          puVar7[2] = &PTR_LAB_14336d908;
          puVar9 = puVar7;
        }
        local_80 = puVar9 + 2;
        local_60 = 0;
        uStack_58 = 0;
        local_78 = puVar9;
        uVar8 = FUN_1408a9d20(local_res18,0x58d);
        local_28 = (undefined4)local_50;
        uStack_24 = local_50._4_4_;
        uStack_20 = (undefined4)uStack_48;
        uStack_1c = uStack_48._4_4_;
        local_18 = local_40;
        local_10 = 0;
        FUN_140fe01b0(local_70,uVar8,4,&local_80,&local_28,0,0);
      }
      if (local_68 != (longlong *)0x0) {
        LOCK();
        plVar1 = local_68 + 1;
        lVar4 = *plVar1;
        *(int *)plVar1 = (int)*plVar1 + -1;
        UNLOCK();
        if ((int)lVar4 == 1) {
          (**(code **)*local_68)(local_68);
          LOCK();
          piVar2 = (int *)((longlong)local_68 + 0xc);
          iVar3 = *piVar2;
          *piVar2 = *piVar2 + -1;
          UNLOCK();
          if (iVar3 == 1) {
            (**(code **)(*local_68 + 8))(local_68);
            return;
          }
        }
      }
    }
    else if (cVar6 == '\x05') {
      FUN_140d6bdb0(param_1 + 0xf0);
      return;
    }
    break;
  case 0xc:
    cVar6 = FUN_1406e8ae0(param_2);
    if (cVar6 == '\x06') {
      FUN_140d6aba0(param_1 + 0xf0);
    }
  }
  return;
}



//===========================================================
// FUN_140d6aba0 @ 140d6aba0   (1402 bytes)
//===========================================================

void FUN_140d6aba0(longlong param_1)

{
  IUnknown *pIVar1;
  IUnknown *pIVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  int local_res8 [2];
  short local_138;
  undefined6 uStack_136;
  longlong lStack_130;
  undefined8 local_128;
  short local_120;
  undefined6 uStack_11e;
  longlong lStack_118;
  undefined8 local_110;
  short local_108;
  undefined6 uStack_106;
  longlong lStack_100;
  undefined8 local_f8;
  short local_f0;
  undefined6 uStack_ee;
  longlong lStack_e8;
  undefined8 local_e0;
  uint local_d8;
  undefined4 uStack_d4;
  int iStack_d0;
  undefined4 uStack_cc;
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
  undefined8 local_58;
  longlong lStack_50;
  undefined8 local_48;
  uint local_38;
  undefined4 uStack_34;
  int iStack_30;
  undefined4 uStack_2c;
  undefined8 local_28;
  
  if ((*(longlong *)(param_1 + 0x58) != 0) && (*(int *)(param_1 + 0x20) == 0)) {
    FUN_1429f20d0(PTR_u_Whistle_143a48fd0,100);
    pIVar1 = *(IUnknown **)(param_1 + 0x60);
    if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x80))
                      (pIVar1,*(undefined4 *)(param_1 + 0x28),*(undefined4 *)(param_1 + 0x2c));
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273780);
    }
    pIVar1 = *(IUnknown **)(param_1 + 0x60);
    if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_f0);
    uVar5 = 0;
    if (DAT_143a8b8d8 == 8) {
      if (local_f0 == 8) {
        local_f0 = 0;
        if (lStack_e8 != 0) {
          (*DAT_143ad5990)(lStack_e8 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_f0);
        if (iVar3 < 0) goto LAB_140d6b0eb;
      }
      local_f0 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_e8 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_f0 == 8) && (local_f0 = 0, lStack_e8 != 0)) {
        (*DAT_143ad5990)(lStack_e8 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_f0,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6b0eb:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_108);
    if (DAT_143a8b8d8 == 8) {
      if (local_108 == 8) {
        local_108 = 0;
        if (lStack_100 != 0) {
          (*DAT_143ad5990)(lStack_100 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_108);
        if (iVar3 < 0) goto LAB_140d6b0f3;
      }
      local_108 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_100 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_108 == 8) && (local_108 = 0, lStack_100 != 0)) {
        (*DAT_143ad5990)(lStack_100 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_108,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6b0f3:
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
        if (iVar3 < 0) goto LAB_140d6b0fb;
      }
      local_120 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_118 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_120 == 8) && (local_120 = 0, lStack_118 != 0)) {
        (*DAT_143ad5990)(lStack_118 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_120,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6b0fb:
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
        if (iVar3 < 0) goto LAB_140d6b103;
      }
      local_138 = 8;
      if (DAT_143a8b8e0 != 0) {
        uVar5 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_130 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
    }
    else {
      if ((local_138 == 8) && (local_138 = 0, lStack_130 != 0)) {
        (*DAT_143ad5990)(lStack_130 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6b103:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    pIVar2 = *(IUnknown **)(param_1 + 0x58);
    if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8[0] = 0;
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 0xb0))(pIVar2,local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
    }
    iStack_d0 = *(int *)(param_1 + 0x34) * 1000 + local_res8[0];
    local_d8 = CONCAT22(local_d8._2_2_,3);
    local_b8 = CONCAT62(uStack_ee,local_f0);
    lStack_b0 = lStack_e8;
    local_a8 = local_e0;
    local_98 = CONCAT62(uStack_106,local_108);
    lStack_90 = lStack_100;
    local_88 = local_f8;
    local_78 = CONCAT62(uStack_11e,local_120);
    lStack_70 = lStack_118;
    local_68 = local_110;
    local_58 = CONCAT62(uStack_136,local_138);
    lStack_50 = lStack_130;
    local_48 = local_128;
    local_38 = local_d8;
    uStack_34 = uStack_d4;
    uStack_2c = uStack_cc;
    local_28 = local_c8;
    iStack_30 = iStack_d0;
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x140))
                      (pIVar1,*(undefined4 *)(param_1 + 0x24),*(undefined4 *)(param_1 + 0x2c),
                       &local_38,&local_58,&local_78,&local_98,&local_b8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273488);
    }
    if ((short)local_d8 == 8) {
      local_d8 = local_d8 & 0xffff0000;
      if (CONCAT44(uStack_cc,iStack_d0) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_cc,iStack_d0) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_d8);
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
    if (local_108 == 8) {
      local_108 = 0;
      if (lStack_100 != 0) {
        (*DAT_143ad5990)(lStack_100 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_108);
    }
    if (local_f0 == 8) {
      local_f0 = 0;
      if (lStack_e8 != 0) {
        (*DAT_143ad5990)(lStack_e8 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_f0);
    }
  }
  return;
}



//===========================================================
// FUN_140d6a610 @ 140d6a610   (1402 bytes)
//===========================================================

void FUN_140d6a610(longlong param_1)

{
  IUnknown *pIVar1;
  IUnknown *pIVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  int local_res8 [2];
  short local_138;
  undefined6 uStack_136;
  longlong lStack_130;
  undefined8 local_128;
  short local_120;
  undefined6 uStack_11e;
  longlong lStack_118;
  undefined8 local_110;
  short local_108;
  undefined6 uStack_106;
  longlong lStack_100;
  undefined8 local_f8;
  short local_f0;
  undefined6 uStack_ee;
  longlong lStack_e8;
  undefined8 local_e0;
  uint local_d8;
  undefined4 uStack_d4;
  int iStack_d0;
  undefined4 uStack_cc;
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
  undefined8 local_58;
  longlong lStack_50;
  undefined8 local_48;
  uint local_38;
  undefined4 uStack_34;
  int iStack_30;
  undefined4 uStack_2c;
  undefined8 local_28;
  
  if ((*(longlong *)(param_1 + 0x58) != 0) && (*(int *)(param_1 + 0x20) == 0)) {
    FUN_1429f20d0(PTR_u_Whistle_143a48fd0,100);
    pIVar1 = *(IUnknown **)(param_1 + 0x60);
    if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x80))
                      (pIVar1,*(undefined4 *)(param_1 + 0x24),*(undefined4 *)(param_1 + 0x2c));
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273780);
    }
    pIVar1 = *(IUnknown **)(param_1 + 0x60);
    if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_f0);
    uVar5 = 0;
    if (DAT_143a8b8d8 == 8) {
      if (local_f0 == 8) {
        local_f0 = 0;
        if (lStack_e8 != 0) {
          (*DAT_143ad5990)(lStack_e8 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_f0);
        if (iVar3 < 0) goto LAB_140d6ab5b;
      }
      local_f0 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_e8 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_f0 == 8) && (local_f0 = 0, lStack_e8 != 0)) {
        (*DAT_143ad5990)(lStack_e8 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_f0,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6ab5b:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_108);
    if (DAT_143a8b8d8 == 8) {
      if (local_108 == 8) {
        local_108 = 0;
        if (lStack_100 != 0) {
          (*DAT_143ad5990)(lStack_100 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_108);
        if (iVar3 < 0) goto LAB_140d6ab63;
      }
      local_108 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_100 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_108 == 8) && (local_108 = 0, lStack_100 != 0)) {
        (*DAT_143ad5990)(lStack_100 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_108,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6ab63:
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
        if (iVar3 < 0) goto LAB_140d6ab6b;
      }
      local_120 = 8;
      uVar4 = uVar5;
      if (DAT_143a8b8e0 != 0) {
        uVar4 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_118 = FUN_1401a5fa0(DAT_143a8b8e0,uVar4);
    }
    else {
      if ((local_120 == 8) && (local_120 = 0, lStack_118 != 0)) {
        (*DAT_143ad5990)(lStack_118 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_120,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6ab6b:
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
        if (iVar3 < 0) goto LAB_140d6ab73;
      }
      local_138 = 8;
      if (DAT_143a8b8e0 != 0) {
        uVar5 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_130 = FUN_1401a5fa0(DAT_143a8b8e0,uVar5);
    }
    else {
      if ((local_138 == 8) && (local_138 = 0, lStack_130 != 0)) {
        (*DAT_143ad5990)(lStack_130 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140d6ab73:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    pIVar2 = *(IUnknown **)(param_1 + 0x58);
    if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8[0] = 0;
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 0xb0))(pIVar2,local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
    }
    iStack_d0 = *(int *)(param_1 + 0x34) * 1000 + local_res8[0];
    local_d8 = CONCAT22(local_d8._2_2_,3);
    local_b8 = CONCAT62(uStack_ee,local_f0);
    lStack_b0 = lStack_e8;
    local_a8 = local_e0;
    local_98 = CONCAT62(uStack_106,local_108);
    lStack_90 = lStack_100;
    local_88 = local_f8;
    local_78 = CONCAT62(uStack_11e,local_120);
    lStack_70 = lStack_118;
    local_68 = local_110;
    local_58 = CONCAT62(uStack_136,local_138);
    lStack_50 = lStack_130;
    local_48 = local_128;
    local_38 = local_d8;
    uStack_34 = uStack_d4;
    uStack_2c = uStack_cc;
    local_28 = local_c8;
    iStack_30 = iStack_d0;
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x140))
                      (pIVar1,*(undefined4 *)(param_1 + 0x28),*(undefined4 *)(param_1 + 0x2c),
                       &local_38,&local_58,&local_78,&local_98,&local_b8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143273488);
    }
    if ((short)local_d8 == 8) {
      local_d8 = local_d8 & 0xffff0000;
      if (CONCAT44(uStack_cc,iStack_d0) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_cc,iStack_d0) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_d8);
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
    if (local_108 == 8) {
      local_108 = 0;
      if (lStack_100 != 0) {
        (*DAT_143ad5990)(lStack_100 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_108);
    }
    if (local_f0 == 8) {
      local_f0 = 0;
      if (lStack_e8 != 0) {
        (*DAT_143ad5990)(lStack_e8 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_f0);
    }
  }
  return;
}



//===========================================================
// FUN_140d69b90 @ 140d69b90   (2673 bytes)
//===========================================================

void FUN_140d69b90(longlong param_1,longlong *param_2)

{
  longlong **pplVar1;
  longlong lVar2;
  undefined *puVar3;
  int iVar4;
  undefined4 uVar5;
  IUnknown *pIVar6;
  longlong *plVar7;
  IUnknown *pIVar8;
  longlong *plVar9;
  undefined2 *puVar10;
  IUnknown *pIVar11;
  IUnknown *pIVar12;
  IUnknown *local_res18;
  longlong *local_res20;
  IUnknown *local_1e8;
  longlong *local_1e0;
  undefined1 *local_1d8;
  uint local_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined8 local_1c0;
  uint local_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined8 local_1a8;
  longlong *local_1a0;
  uint local_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined8 local_188;
  undefined1 local_180 [8];
  IUnknown **local_178;
  uint local_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined8 local_160;
  uint local_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined8 local_140;
  undefined1 local_138 [8];
  undefined8 local_130;
  uint local_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined8 local_118;
  uint local_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined8 local_100;
  uint local_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined8 local_e8;
  uint local_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined8 local_d0;
  undefined1 local_c8 [8];
  longlong *local_c0;
  undefined1 local_b8 [8];
  uint local_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined8 local_a0;
  uint local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 local_88;
  uint local_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 local_70;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  uint local_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined8 local_40;
  
  pIVar11 = (IUnknown *)*param_2;
  if (pIVar11 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  pIVar6 = (IUnknown *)FUN_1401a5890(local_c8,PTR_u_shipObj_143a47d60);
  local_res18 = pIVar6;
  (*DAT_143262a20)(&local_b0);
  pIVar12 = (IUnknown *)0x0;
  pIVar8 = pIVar12;
  if (*(longlong **)pIVar6 != (longlong *)0x0) {
    pIVar8 = (IUnknown *)**(longlong **)pIVar6;
  }
  iVar4 = (**(code **)(*(longlong *)pIVar11 + 0x28))(pIVar11,pIVar8,&local_b0);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,pIVar11,(_GUID *)&DAT_143272478);
  }
  local_128 = local_b0;
  uStack_124 = uStack_ac;
  uStack_120 = uStack_a8;
  uStack_11c = uStack_a4;
  local_118 = local_a0;
  local_b0 = local_b0 & 0xffff0000;
  FUN_1401be120(pIVar6);
  plVar7 = (longlong *)FUN_1409339d0(&local_c0,&local_128);
  local_1e8 = (IUnknown *)0x0;
  plVar7 = (longlong *)*plVar7;
  local_1e0 = plVar7;
  if (plVar7 == (longlong *)0x0) {
    iVar4 = -0x7fffbffe;
  }
  else {
    (**(code **)(*plVar7 + 8))(plVar7);
    local_res18 = (IUnknown *)0x0;
    iVar4 = (**(code **)*plVar7)(plVar7,&DAT_143272478,&local_res18);
    local_1e8 = pIVar12;
    if (-1 < iVar4) {
      local_1e8 = local_res18;
    }
  }
  if (plVar7 != (longlong *)0x0) {
    (**(code **)(*plVar7 + 0x10))(plVar7);
  }
  if (((iVar4 + 0x80000000U & 0x80000000) == 0) && (iVar4 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar4);
  }
  if (local_c0 != (longlong *)0x0) {
    (**(code **)(*local_c0 + 0x10))();
  }
  if ((short)local_128 == 8) {
    local_128 = local_128 & 0xffff0000;
    if (CONCAT44(uStack_11c,uStack_120) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_11c,uStack_120) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_128);
  }
  if (local_1e8 == (IUnknown *)0x0) {
    if ((longlong *)*param_2 == (longlong *)0x0) {
      return;
    }
    (**(code **)(*(longlong *)*param_2 + 0x10))();
    return;
  }
  plVar7 = (longlong *)FUN_1401a5890(local_b8,PTR_u_shipKind_143a47d68);
  local_res20 = plVar7;
  (*DAT_143262a20)(&local_98);
  pIVar11 = pIVar12;
  if ((longlong *)*plVar7 != (longlong *)0x0) {
    pIVar11 = *(IUnknown **)*plVar7;
  }
  iVar4 = (**(code **)(*(longlong *)local_1e8 + 0x28))(local_1e8,pIVar11,&local_98);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,local_1e8,(_GUID *)&DAT_143272478);
  }
  local_110 = local_98;
  uStack_10c = uStack_94;
  uStack_108 = uStack_90;
  uStack_104 = uStack_8c;
  local_100 = local_88;
  local_98 = local_98 & 0xffff0000;
  FUN_1401be120(plVar7);
  uVar5 = FUN_14022ee40(&local_110,0);
  *(undefined4 *)(param_1 + 0x20) = uVar5;
  if ((short)local_110 == 8) {
    local_110 = local_110 & 0xffff0000;
    if (CONCAT44(uStack_104,uStack_108) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_104,uStack_108) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_110);
  }
  plVar7 = (longlong *)FUN_1401a5890(local_138,PTR_DAT_143a47a98);
  local_res20 = plVar7;
  (*DAT_143262a20)(&local_80);
  pIVar11 = pIVar12;
  if ((longlong *)*plVar7 != (longlong *)0x0) {
    pIVar11 = *(IUnknown **)*plVar7;
  }
  iVar4 = (**(code **)(*(longlong *)local_1e8 + 0x28))(local_1e8,pIVar11,&local_80);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,local_1e8,(_GUID *)&DAT_143272478);
  }
  local_f8 = local_80;
  uStack_f4 = uStack_7c;
  uStack_f0 = uStack_78;
  uStack_ec = uStack_74;
  local_e8 = local_70;
  local_80 = local_80 & 0xffff0000;
  FUN_1401be120(plVar7);
  uVar5 = FUN_14022ee40(&local_f8,0);
  *(undefined4 *)(param_1 + 0x24) = uVar5;
  if ((short)local_f8 == 8) {
    local_f8 = local_f8 & 0xffff0000;
    if (CONCAT44(uStack_ec,uStack_f0) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_ec,uStack_f0) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_f8);
  }
  plVar7 = (longlong *)FUN_1401a5890(local_180,PTR_DAT_143a47aa0);
  local_res20 = plVar7;
  (*DAT_143262a20)(&local_68);
  pIVar11 = pIVar12;
  if ((longlong *)*plVar7 != (longlong *)0x0) {
    pIVar11 = *(IUnknown **)*plVar7;
  }
  iVar4 = (**(code **)(*(longlong *)local_1e8 + 0x28))(local_1e8,pIVar11,&local_68);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,local_1e8,(_GUID *)&DAT_143272478);
  }
  local_e0 = local_68;
  uStack_dc = uStack_64;
  uStack_d8 = uStack_60;
  uStack_d4 = uStack_5c;
  local_d0 = local_58;
  local_68 = local_68 & 0xffff0000;
  FUN_1401be120(plVar7);
  uVar5 = FUN_14022ee40(&local_e0,0);
  *(undefined4 *)(param_1 + 0x2c) = uVar5;
  if ((short)local_e0 == 8) {
    local_e0 = local_e0 & 0xffff0000;
    if (CONCAT44(uStack_d4,uStack_d8) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_d4,uStack_d8) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_e0);
  }
  plVar7 = (longlong *)FUN_1401a5890(&local_1a0,PTR_DAT_143a47d78);
  local_res20 = plVar7;
  (*DAT_143262a20)(&local_50);
  pIVar11 = pIVar12;
  if ((longlong *)*plVar7 != (longlong *)0x0) {
    pIVar11 = *(IUnknown **)*plVar7;
  }
  iVar4 = (**(code **)(*(longlong *)local_1e8 + 0x28))(local_1e8,pIVar11,&local_50);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,local_1e8,(_GUID *)&DAT_143272478);
  }
  local_1d0 = local_50;
  uStack_1cc = uStack_4c;
  uStack_1c8 = uStack_48;
  uStack_1c4 = uStack_44;
  local_1c0 = local_40;
  local_50 = local_50 & 0xffff0000;
  FUN_1401be120(plVar7);
  uVar5 = FUN_14022ee40(&local_1d0,0);
  *(undefined4 *)(param_1 + 0x38) = uVar5;
  if ((short)local_1d0 == 8) {
    local_1d0 = local_1d0 & 0xffff0000;
    if (CONCAT44(uStack_1c4,uStack_1c8) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1c4,uStack_1c8) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1d0);
  }
  plVar7 = (longlong *)FUN_1401a5890(&local_1d8,PTR_u_tMove_143a47d70);
  local_res20 = plVar7;
  (*DAT_143262a20)(&local_198);
  pIVar11 = pIVar12;
  if ((longlong *)*plVar7 != (longlong *)0x0) {
    pIVar11 = *(IUnknown **)*plVar7;
  }
  iVar4 = (**(code **)(*(longlong *)local_1e8 + 0x28))(local_1e8,pIVar11,&local_198);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,local_1e8,(_GUID *)&DAT_143272478);
  }
  local_1b8 = local_198;
  uStack_1b4 = uStack_194;
  uStack_1b0 = uStack_190;
  uStack_1ac = uStack_18c;
  local_1a8 = local_188;
  local_198 = local_198 & 0xffff0000;
  FUN_1401be120(plVar7);
  uVar5 = FUN_14022ee40(&local_1b8,0);
  *(undefined4 *)(param_1 + 0x34) = uVar5;
  if ((short)local_1b8 == 8) {
    local_1b8 = local_1b8 & 0xffff0000;
    if (CONCAT44(uStack_1ac,uStack_1b0) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1ac,uStack_1b0) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1b8);
  }
  plVar7 = (longlong *)FUN_1401a5890(&local_178,PTR_u_shipObj_143a47d60);
  local_res20 = plVar7;
  (*DAT_143262a20)(&local_170);
  pIVar11 = pIVar12;
  if ((longlong *)*plVar7 != (longlong *)0x0) {
    pIVar11 = *(IUnknown **)*plVar7;
  }
  iVar4 = (**(code **)(*(longlong *)local_1e8 + 0x28))(local_1e8,pIVar11,&local_170);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,local_1e8,(_GUID *)&DAT_143272478);
  }
  local_150 = local_170;
  uStack_14c = uStack_16c;
  uStack_148 = uStack_168;
  uStack_144 = uStack_164;
  local_140 = local_160;
  local_170 = local_170 & 0xffff0000;
  FUN_1401be120(plVar7);
  puVar10 = &DAT_143278568;
  if ((short)local_150 == 8) {
    puVar10 = (undefined2 *)CONCAT44(uStack_144,uStack_148);
  }
  FUN_1401a5890(&local_res20,puVar10);
  pplVar1 = (longlong **)(param_1 + 0x50);
  if (pplVar1 != &local_res20) {
    FUN_1401be120(pplVar1);
    *pplVar1 = local_res20;
    if (local_res20 == (longlong *)0x0) goto LAB_140d6a24d;
    LOCK();
    *(int *)(local_res20 + 2) = (int)local_res20[2] + 1;
    UNLOCK();
  }
  plVar7 = local_res20;
  if (local_res20 != (longlong *)0x0) {
    LOCK();
    plVar9 = local_res20 + 2;
    lVar2 = *plVar9;
    *(int *)plVar9 = (int)*plVar9 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      if (*local_res20 != 0) {
        (*DAT_143ad5990)(*local_res20 + -4);
        *plVar7 = 0;
      }
      if (plVar7[1] != 0) {
        FUN_14019b4e0();
        plVar7[1] = 0;
      }
      thunk_FUN_140205820(plVar7,0x18);
    }
  }
LAB_140d6a24d:
  if ((short)local_150 == 8) {
    local_150 = local_150 & 0xffff0000;
    if (CONCAT44(uStack_144,uStack_148) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_144,uStack_148) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_150);
  }
  puVar3 = PTR_u_Shape2D_Vector2D_143a479e8;
  plVar7 = *(longlong **)(param_1 + 0x60);
  plVar9 = (longlong *)0x0;
  if (plVar7 != (longlong *)0x0) {
    *(undefined8 *)(param_1 + 0x60) = 0;
    (**(code **)(*plVar7 + 0x10))();
    plVar9 = *(longlong **)(param_1 + 0x60);
  }
  if (plVar9 != (longlong *)0x0) {
    (**(code **)(*plVar9 + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x60) = 0;
  if (DAT_143ad48a0 == (code *)0x0) {
    iVar4 = -0x7ffbfe10;
  }
  else {
    iVar4 = (*DAT_143ad48a0)(puVar3,&DAT_143273488,param_1 + 0x60,0);
    if (-1 < iVar4) {
      if (*(int *)(param_1 + 0x20) == 0) {
        pIVar8 = (IUnknown *)FUN_1401a5890(&local_res20,PTR_DAT_143a47d80);
        local_res18 = pIVar8;
        (*DAT_143262a20)(&local_170);
        pIVar11 = pIVar12;
        if (*(longlong **)pIVar8 != (longlong *)0x0) {
          pIVar11 = (IUnknown *)**(longlong **)pIVar8;
        }
        iVar4 = (**(code **)(*(longlong *)local_1e8 + 0x28))(local_1e8,pIVar11,&local_170);
        if (iVar4 < 0) {
          _com_issue_errorex(iVar4,local_1e8,(_GUID *)&DAT_143272478);
        }
        local_1b8 = local_170;
        uStack_1b4 = uStack_16c;
        uStack_1b0 = uStack_168;
        uStack_1ac = uStack_164;
        local_1a8 = local_160;
        local_170 = local_170 & 0xffff0000;
        FUN_1401be120(pIVar8);
        uVar5 = FUN_14022ee40(&local_1b8,0);
        *(undefined4 *)(param_1 + 0x28) = uVar5;
        if ((short)local_1b8 == 8) {
          local_1b8 = local_1b8 & 0xffff0000;
          if (CONCAT44(uStack_1ac,uStack_1b0) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_1ac,uStack_1b0) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_1b8);
        }
        pIVar8 = (IUnknown *)FUN_1401a5890(&local_1d8,PTR_DAT_143a47aa8);
        local_res18 = pIVar8;
        (*DAT_143262a20)(&local_198);
        pIVar11 = pIVar12;
        if (*(longlong **)pIVar8 != (longlong *)0x0) {
          pIVar11 = (IUnknown *)**(longlong **)pIVar8;
        }
        iVar4 = (**(code **)(*(longlong *)local_1e8 + 0x28))(local_1e8,pIVar11,&local_198);
        if (iVar4 < 0) {
          _com_issue_errorex(iVar4,local_1e8,(_GUID *)&DAT_143272478);
        }
        local_1d0 = local_198;
        uStack_1cc = uStack_194;
        uStack_1c8 = uStack_190;
        uStack_1c4 = uStack_18c;
        local_1c0 = local_188;
        local_198 = local_198 & 0xffff0000;
        FUN_1401be120(pIVar8);
        uVar5 = FUN_14022ee40(&local_1d0,0);
        *(undefined4 *)(param_1 + 0x30) = uVar5;
        if ((short)local_1d0 == 8) {
          local_1d0 = local_1d0 & 0xffff0000;
          if (CONCAT44(uStack_1c4,uStack_1c8) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_1c4,uStack_1c8) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_1d0);
        }
        pIVar11 = *(IUnknown **)(param_1 + 0x60);
        if (pIVar11 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        iVar4 = (**(code **)(*(longlong *)pIVar11 + 0x80))
                          (pIVar11,*(undefined4 *)(param_1 + 0x28),*(undefined4 *)(param_1 + 0x2c));
        if (iVar4 < 0) {
          _com_issue_errorex(iVar4,pIVar11,(_GUID *)&DAT_143273780);
        }
        local_178 = &local_res18;
        local_res18 = (IUnknown *)0x0;
        local_1d8 = local_138;
        local_130 = 0;
        iVar4 = *(int *)(param_1 + 0x30);
        local_res20 = (longlong *)0x0;
        local_1e0 = *(longlong **)(param_1 + 0x60);
        if (local_1e0 != (longlong *)0x0) {
          (**(code **)(*local_1e0 + 8))();
        }
        if (*pplVar1 != (longlong *)0x0) {
          pIVar12 = (IUnknown *)**pplVar1;
        }
        plVar9 = (longlong *)
                 FUN_140dc12e0(&local_1a0,pIVar12,*(int *)(param_1 + 0x38) != 0,&local_1e0,0,0,
                               &local_res20,iVar4 * 30000 + -0x3fffff9c,0xff,0,local_138,0,0,0,0,
                               &local_res18,0,0,0);
        plVar7 = *(longlong **)(param_1 + 0x58);
        if (plVar7 != (longlong *)*plVar9) {
          *(longlong **)(param_1 + 0x58) = (longlong *)*plVar9;
          *plVar9 = 0;
          if (plVar7 != (longlong *)0x0) {
            (**(code **)(*plVar7 + 0x10))();
          }
        }
        if (local_1a0 != (longlong *)0x0) {
          (**(code **)(*local_1a0 + 0x10))();
        }
      }
      else {
        *(int *)(param_1 + 0x40) = *(int *)(param_1 + 0x24) + -0x32;
        *(int *)(param_1 + 0x44) = *(int *)(param_1 + 0x24) + 0x32;
        *(int *)(param_1 + 0x4c) = *(int *)(param_1 + 0x2c) + -100;
        *(int *)(param_1 + 0x48) = *(int *)(param_1 + 0x2c) + 100;
      }
      (**(code **)(*(longlong *)local_1e8 + 0x10))(local_1e8);
      if ((longlong *)*param_2 != (longlong *)0x0) {
        (**(code **)(*(longlong *)*param_2 + 0x10))();
      }
      return;
    }
  }
  FUN_1401a59c0(&local_170,iVar4,0,0);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(&local_170,(ThrowInfo *)&DAT_143a3b0c0);
}



//===========================================================
// FUN_140d6cb50 @ 140d6cb50   (211 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_140d6cb50(longlong param_1)

{
  undefined4 uVar1;
  undefined1 auStack_498 [32];
  longlong *local_478 [2];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  FUN_141818ae0();
  *(undefined4 *)(param_1 + 0x158) = 0;
  local_478[0] = *(longlong **)(param_1 + 0x78);
  if (local_478[0] != (longlong *)0x0) {
    (**(code **)(*local_478[0] + 8))();
  }
  FUN_140d69b90(param_1 + 0xf0,local_478);
  if (*(uint *)(param_1 + 0x110) < 2) {
    FUN_1406ed520(local_468,0x34d);
    uVar1 = FUN_1402fa540(*(longlong *)(param_1 + 0xa8) + 0x80);
    FUN_1406ed9d0(local_468,uVar1);
    FUN_1406ed840(local_468,*(undefined1 *)(param_1 + 0x110));
    FUN_1415d01c0(local_468);
    FUN_1406ed610(local_468);
  }
  return;
}


