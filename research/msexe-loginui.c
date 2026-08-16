
//===========================================================
// FUN_141129930 @ 141129930   (3114 bytes)
//===========================================================

void FUN_141129930(longlong param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int iVar4;
  longlong *plVar5;
  int *piVar6;
  longlong *plVar7;
  int *piVar8;
  longlong lVar9;
  uint uVar10;
  longlong *local_res8;
  int *local_res18;
  undefined4 local_res20;
  undefined4 local_res24;
  undefined8 in_stack_fffffffffffffe68;
  uint uVar15;
  longlong lVar11;
  undefined4 uVar16;
  undefined8 *puVar12;
  int *piVar13;
  IUnknown *pIVar14;
  undefined8 *puVar17;
  ulonglong uVar18;
  ulonglong in_stack_fffffffffffffe78;
  undefined8 *puVar19;
  undefined4 uVar21;
  undefined8 uVar20;
  undefined8 *puVar22;
  undefined4 uVar24;
  undefined8 uVar23;
  longlong *local_178;
  short local_170;
  undefined2 uStack_16e;
  undefined4 uStack_16c;
  undefined8 uStack_168;
  undefined8 local_160;
  undefined4 local_158;
  undefined4 uStack_154;
  undefined8 uStack_150;
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
  IUnknown *local_f8;
  undefined4 local_f0;
  undefined4 local_ec;
  longlong *local_e8;
  longlong **local_e0;
  uint local_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
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
  
  uVar15 = (uint)((ulonglong)in_stack_fffffffffffffe68 >> 0x20);
  puVar22 = &local_b8;
  FUN_142bf5270();
  lVar9 = param_1 + 0x240;
  FUN_141aa3af0(lVar9,param_1,0,0);
  uVar10 = 0;
  lVar11 = (ulonglong)uVar15 << 0x20;
  FUN_141ac3370(lVar9,DAT_143aca408,0,0,lVar11,1,in_stack_fffffffffffffe78 & 0xffffffffffffff00,0);
  uVar16 = (undefined4)((ulonglong)lVar11 >> 0x20);
  FUN_142aa2010(lVar9,L"login",0);
  local_res8 = (longlong *)0x0;
  FUN_141adbce0(lVar9,&local_res20,L"textPassword",&local_res8);
  plVar5 = (longlong *)FUN_141aa7120(lVar9,&local_res8,0xb6,0x1e,CONCAT44(uVar16,1));
  plVar7 = *(longlong **)(param_1 + 0x248);
  if (plVar7 != (longlong *)*plVar5) {
    *(longlong **)(param_1 + 0x248) = (longlong *)*plVar5;
    *plVar5 = 0;
    if (plVar7 != (longlong *)0x0) {
      (**(code **)(*plVar7 + 0x10))();
    }
  }
  if (local_res8 != (longlong *)0x0) {
    (**(code **)(*local_res8 + 0x10))();
  }
  pIVar14 = *(IUnknown **)(param_1 + 0x248);
  if (pIVar14 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_170);
  if (DAT_143a8b8d8 == 8) {
    if (local_170 == 8) {
      local_170 = 0;
      if (uStack_168 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_168 + -4);
      }
    }
    else {
      iVar4 = (*DAT_143262a18)(&local_170);
      if (iVar4 < 0) goto LAB_14112a513;
    }
    local_170 = 8;
    uVar15 = uVar10;
    if (DAT_143a8b8e0 != 0) {
      uVar15 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    uStack_168 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar15);
  }
  else {
    if ((local_170 == 8) && (local_170 = 0, uStack_168 != (longlong *)0x0)) {
      (*DAT_143ad5990)((longlong)uStack_168 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_170,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14112a513:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
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
      iVar4 = (*DAT_143262a18)(&local_110);
      if (iVar4 < 0) goto LAB_14112a51b;
    }
    local_110 = 8;
    uVar15 = uVar10;
    if (DAT_143a8b8e0 != 0) {
      uVar15 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_108 = FUN_1401a5fa0(DAT_143a8b8e0,uVar15);
  }
  else {
    if ((local_110 == 8) && (local_110 = 0, lStack_108 != 0)) {
      (*DAT_143ad5990)(lStack_108 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_110,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14112a51b:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
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
      iVar4 = (*DAT_143262a18)(&local_128);
      if (iVar4 < 0) goto LAB_14112a523;
    }
    local_128 = 8;
    uVar15 = uVar10;
    if (DAT_143a8b8e0 != 0) {
      uVar15 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar15);
  }
  else {
    if ((local_128 == 8) && (local_128 = 0, lStack_120 != 0)) {
      (*DAT_143ad5990)(lStack_120 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_128,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14112a523:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
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
      iVar4 = (*DAT_143262a18)(&local_140);
      if (iVar4 < 0) goto LAB_14112a52b;
    }
    local_140 = 8;
    uVar15 = uVar10;
    if (DAT_143a8b8e0 != 0) {
      uVar15 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_138 = FUN_1401a5fa0(DAT_143a8b8e0,uVar15);
  }
  else {
    if ((local_140 == 8) && (local_140 = 0, lStack_138 != 0)) {
      (*DAT_143ad5990)(lStack_138 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_140,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14112a52b:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
  }
  (*DAT_143262a20)(&local_158);
  if (DAT_143a8b8d8 == 8) {
    if ((short)local_158 == 8) {
      local_158 = (uint)local_158._2_2_ << 0x10;
      if (uStack_150 != 0) {
        (*DAT_143ad5990)(uStack_150 + -4);
      }
    }
    else {
      iVar4 = (*DAT_143262a18)(&local_158);
      if (iVar4 < 0) goto LAB_14112a533;
    }
    local_158 = CONCAT22(local_158._2_2_,8);
    uVar15 = uVar10;
    if (DAT_143a8b8e0 != 0) {
      uVar15 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    uStack_150 = FUN_1401a5fa0(DAT_143a8b8e0,uVar15);
  }
  else {
    if (((short)local_158 == 8) && (local_158 = (uint)local_158._2_2_ << 0x10, uStack_150 != 0)) {
      (*DAT_143ad5990)(uStack_150 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_158,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14112a533:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
  }
  local_b8 = CONCAT44(uStack_16c,CONCAT22(uStack_16e,local_170));
  lStack_b0 = (longlong)uStack_168;
  local_a8 = local_160;
  local_98 = CONCAT62(uStack_10e,local_110);
  lStack_90 = lStack_108;
  local_88 = local_100;
  local_78 = CONCAT62(uStack_126,local_128);
  lStack_70 = lStack_120;
  local_68 = local_118;
  local_58 = CONCAT62(uStack_13e,local_140);
  lStack_50 = lStack_138;
  local_48 = local_130;
  local_d8 = local_158;
  uStack_d4 = uStack_154;
  uStack_d0 = (undefined4)uStack_150;
  uStack_cc = uStack_150._4_4_;
  local_c8 = local_148;
  puVar19 = &local_98;
  puVar17 = &local_78;
  puVar12 = &local_58;
  iVar4 = (**(code **)(*(longlong *)pIVar14 + 0x90))
                    (pIVar14,local_res20,local_res24,&local_d8,puVar12,puVar17,puVar19,puVar22);
  uVar21 = (undefined4)((ulonglong)puVar19 >> 0x20);
  uVar24 = (undefined4)((ulonglong)puVar22 >> 0x20);
  uVar16 = (undefined4)((ulonglong)puVar12 >> 0x20);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,pIVar14,(_GUID *)&DAT_143372510);
  }
  if ((short)local_158 == 8) {
    local_158 = local_158 & 0xffff0000;
    if (uStack_150 != 0) {
      (*DAT_143ad5990)(uStack_150 + -4);
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
  if (local_170 == 8) {
    local_170 = 0;
    if (uStack_168 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)uStack_168 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_170);
  }
  FUN_1429fbeb0(&local_f8,0);
  local_res18 = (int *)0x0;
  piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,0x3a);
  piVar6[1] = 0x29;
  *piVar6 = -1;
  local_res18 = piVar6 + 4;
  piVar6[2] = 0;
  *(undefined1 *)local_res18 = 0;
  uVar20 = s_Please_press_the_Login_Button_to_143383888._8_8_;
  *(undefined8 *)local_res18 = s_Please_press_the_Login_Button_to_143383888._0_8_;
  *(undefined8 *)(piVar6 + 6) = uVar20;
  uVar3 = s_Please_press_the_Login_Button_to_143383888._28_4_;
  uVar2 = s_Please_press_the_Login_Button_to_143383888._24_4_;
  uVar1 = s_Please_press_the_Login_Button_to_143383888._20_4_;
  piVar6[8] = s_Please_press_the_Login_Button_to_143383888._16_4_;
  piVar6[9] = uVar1;
  piVar6[10] = uVar2;
  piVar6[0xb] = uVar3;
  *(undefined8 *)(piVar6 + 0xc) = s_Please_press_the_Login_Button_to_143383888._32_8_;
  *(char *)(piVar6 + 0xe) = s_Please_press_the_Login_Button_to_143383888[0x28];
  if (*piVar6 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar6[1] < 0x29) {
    FUN_142e54290(0x90,piVar6[1],0x29);
  }
  *piVar6 = 1;
  *(undefined1 *)((longlong)local_res18 + 0x29) = 0;
  if (piVar6[1] + 1 < 0x2a) {
    FUN_142e54290(0x9c);
  }
  pIVar14 = local_f8;
  piVar6[2] = 0x29;
  if (local_f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_170);
  if (DAT_143a8b8d8 == 8) {
    if (local_170 == 8) {
      local_170 = 0;
      if (uStack_168 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_168 + -4);
      }
    }
    else {
      iVar4 = (*DAT_143262a18)(&local_170);
      if (iVar4 < 0) goto LAB_14112a53b;
    }
    local_170 = 8;
    if (DAT_143a8b8e0 != 0) {
      uVar10 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    uStack_168 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar10);
  }
  else {
    if ((local_170 == 8) && (local_170 = 0, uStack_168 != (longlong *)0x0)) {
      (*DAT_143ad5990)((longlong)uStack_168 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_170,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14112a53b:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
  }
  piVar6 = local_res18;
  plVar7 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
  local_res8 = plVar7;
  if (plVar7 == (longlong *)0x0) {
    local_178 = (longlong *)0x0;
  }
  else {
    plVar7[1] = 0;
    *(undefined4 *)(plVar7 + 2) = 1;
    local_178 = plVar7;
    if (piVar6 == (int *)0x0) {
      *plVar7 = 0;
    }
    else {
      uVar18 = (ulonglong)puVar17 & 0xffffffff00000000;
      iVar4 = (*DAT_1432627f8)(0xfde9,0,piVar6,0xffffffff,0,uVar18);
      uVar16 = (undefined4)(uVar18 >> 0x20);
      iVar4 = (int)((ulonglong)(longlong)(iVar4 * 2) >> 1);
      uVar10 = iVar4 - 1;
      piVar8 = (int *)(*DAT_143ad5980)();
      if (piVar8 == (int *)0x0) {
        piVar8 = (int *)0x0;
      }
      else {
        *piVar8 = uVar10 * 2;
        piVar8 = piVar8 + 1;
        *(undefined2 *)((longlong)piVar8 + (ulonglong)uVar10 * 2) = 0;
      }
      piVar13 = piVar8;
      (*DAT_1432627f8)(0xfde9,0,piVar6,0xffffffff,piVar8,CONCAT44(uVar16,iVar4));
      uVar16 = (undefined4)((ulonglong)piVar13 >> 0x20);
      *plVar7 = (longlong)piVar8;
    }
  }
  lVar9 = param_1 + 0x240;
  if (local_178 != (longlong *)0x0) {
    local_e0 = &local_178;
    local_res8 = (longlong *)((ulonglong)local_res8 & 0xffffffff00000000);
    local_d8 = CONCAT22(uStack_16e,local_170);
    uStack_d4 = uStack_16c;
    uStack_d0 = (undefined4)uStack_168;
    uStack_cc = uStack_168._4_4_;
    local_c8 = local_160;
    iVar4 = (**(code **)(*(longlong *)pIVar14 + 0xb0))(pIVar14,*local_178,&local_d8,&local_res8);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar14,(_GUID *)&DAT_143297250);
    }
    uVar1 = local_res8._0_4_;
    thunk_FUN_1401be120(&local_178);
    *(undefined4 *)(param_1 + 0x260) = uVar1;
    if (local_170 == 8) {
      local_170 = 0;
      if (uStack_168 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_168 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_170);
    }
    uVar23 = CONCAT44(uVar24,local_res24);
    uVar20 = CONCAT44(uVar21,local_res20);
    uVar18 = 0;
    plVar5 = (longlong *)
             FUN_141aa5a30(lVar9,&local_res8,*(int *)(param_1 + 0x260) + 5,0x1e,CONCAT44(uVar16,1),0
                           ,uVar20,uVar23);
    uVar21 = (undefined4)((ulonglong)uVar23 >> 0x20);
    uVar16 = (undefined4)((ulonglong)uVar20 >> 0x20);
    plVar7 = *(longlong **)(param_1 + 0x250);
    if (plVar7 != (longlong *)*plVar5) {
      *(longlong **)(param_1 + 0x250) = (longlong *)*plVar5;
      *plVar5 = 0;
      if (plVar7 != (longlong *)0x0) {
        (**(code **)(*plVar7 + 0x10))();
      }
    }
    if (local_res8 != (longlong *)0x0) {
      (**(code **)(*local_res8 + 0x10))();
    }
    pIVar14 = *(IUnknown **)(param_1 + 0x250);
    if (pIVar14 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    uStack_168 = *(longlong **)(param_1 + 0x248);
    local_170 = 0xd;
    if (uStack_168 != (longlong *)0x0) {
      (**(code **)(*uStack_168 + 8))();
    }
    local_d8 = CONCAT22(uStack_16e,local_170);
    uStack_d4 = uStack_16c;
    uStack_d0 = (undefined4)uStack_168;
    uStack_cc = uStack_168._4_4_;
    local_c8 = local_160;
    iVar4 = (**(code **)(*(longlong *)pIVar14 + 0x238))(pIVar14,&local_d8);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar14,(_GUID *)&DAT_14327fcb0);
    }
    if (local_170 == 8) {
      local_170 = 0;
      if (uStack_168 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_168 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_170);
    }
    local_res8 = *(longlong **)(param_1 + 0x250);
    if (local_res8 != (longlong *)0x0) {
      (**(code **)(*local_res8 + 8))();
    }
    FUN_142aa18d0(&local_e8,&local_res8);
    local_178 = *(longlong **)(param_1 + 0x250);
    if (local_178 != (longlong *)0x0) {
      (**(code **)(*local_178 + 8))();
    }
    FUN_142aa1310(&local_178);
    pIVar14 = local_f8;
    local_178 = local_e8;
    if (local_e8 != (longlong *)0x0) {
      (**(code **)(*local_e8 + 8))();
    }
    uVar20 = CONCAT44(uVar16,0xff);
    FUN_142a0ff80(&local_178,0,0,&local_res18,pIVar14,uVar18 & 0xffffffff00000000,uVar20);
    uVar16 = (undefined4)((ulonglong)pIVar14 >> 0x20);
    uVar24 = (undefined4)((ulonglong)uVar20 >> 0x20);
    FUN_14112a9a0(param_1,1);
    local_178 = (longlong *)0x0;
    FUN_141adbce0(lVar9,&local_f0,L"textAccount",&local_178);
    plVar5 = (longlong *)
             FUN_141aa5a30(lVar9,&local_178,0xb6,0x1e,CONCAT44(uVar16,1),0,CONCAT44(uVar24,local_f0)
                           ,CONCAT44(uVar21,local_ec));
    plVar7 = *(longlong **)(param_1 + 600);
    if (plVar7 != (longlong *)*plVar5) {
      *(longlong **)(param_1 + 600) = (longlong *)*plVar5;
      *plVar5 = 0;
      if (plVar7 != (longlong *)0x0) {
        (**(code **)(*plVar7 + 0x10))();
      }
    }
    if (local_178 != (longlong *)0x0) {
      (**(code **)(*local_178 + 0x10))();
    }
    *(undefined1 *)(param_1 + 0x26c) = 0;
    if (local_e8 != (longlong *)0x0) {
      (**(code **)(*local_e8 + 0x10))();
    }
    if (local_res18 != (int *)0x0) {
      FUN_14019f2c0(local_res18 + -4);
    }
    if (local_f8 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_f8 + 0x10))();
    }
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x8007000e);
}



//===========================================================
// FUN_14112a570 @ 14112a570   (418 bytes)
//===========================================================

void FUN_14112a570(longlong param_1,undefined4 param_2)

{
  longlong *plVar1;
  char cVar2;
  int iVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  undefined1 local_18 [8];
  longlong local_10;
  
  lVar4 = FUN_14112b010();
  if (lVar4 != 0) {
    uVar5 = FUN_14112b010();
    cVar2 = FUN_141b3faf0(uVar5);
    if (cVar2 == '\0') {
      cVar2 = FUN_142aa1a20(param_1 + 0x240,L"login",param_2);
      if (cVar2 == '\0') {
        cVar2 = FUN_142aa1a20(param_1 + 0x240,L"login_saved",param_2);
        if (cVar2 == '\0') {
          cVar2 = FUN_142aa1a20(param_1 + 0x240,L"quit",param_2);
          if (cVar2 == '\0') {
            FUN_142bf5d70(param_1,param_2);
          }
          else {
            cVar2 = FUN_141b3fd10(uVar5);
            if (cVar2 != '\0') {
              FUN_141b2d4a0(uVar5,0,0);
            }
          }
        }
        else {
          FUN_141ad7170(param_1 + 0x240,local_18,L"check_saved");
          if (local_10 != 0) {
            iVar3 = FUN_141690380(local_10);
            if (local_10 == 0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_141690300(local_10,iVar3 == 0);
          }
          lVar4 = local_10;
          if (local_10 != 0) {
            if (0xffffe < *(longlong *)(local_10 + 0x20) - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar1 = (longlong *)(lVar4 + 0x20);
            lVar4 = *plVar1;
            *plVar1 = *plVar1 + -1;
            UNLOCK();
            if ((int)lVar4 == 1) {
              puVar6 = (undefined8 *)(local_10 + 0x18);
              if (local_10 == 0) {
                puVar6 = (undefined8 *)0x0;
              }
              if (puVar6 != (undefined8 *)0x0) {
                (**(code **)*puVar6)(puVar6,1);
              }
            }
          }
        }
      }
      else {
        cVar2 = FUN_141b3fd10(uVar5);
        if (cVar2 == '\0') {
          FUN_141b3f050(uVar5,4,600);
        }
        else {
          FUN_141b3ff10();
        }
      }
    }
  }
  return;
}


