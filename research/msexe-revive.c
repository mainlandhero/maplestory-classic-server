
//===========================================================
// FUN_142903cd0 @ 142903cd0   (434 bytes)
//===========================================================

void FUN_142903cd0(longlong param_1,undefined8 param_2)

{
  char cVar1;
  char cVar2;
  char cVar3;
  char cVar4;
  uint uVar5;
  int iVar6;
  undefined4 uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  undefined8 uVar11;
  longlong lVar12;
  
  lVar12 = DAT_143aa84a0;
  if (DAT_143aa84a0 != 0) {
    uVar5 = FUN_1406e8c20(param_2);
    iVar6 = FUN_1406e8c20(param_2);
    uVar7 = FUN_1406e8c20(param_2);
    cVar1 = FUN_1406e8ae0(param_2);
    iVar8 = FUN_1406e8c20(param_2);
    iVar9 = FUN_1406e8c20(param_2);
    cVar2 = FUN_1406e8ae0(param_2);
    cVar3 = FUN_1406e8ae0(param_2);
    if (iVar6 != 9) {
      uVar11 = FUN_142cbecc0(lVar12);
      cVar4 = FUN_142122480(uVar11,1);
      lVar12 = FUN_142cbe730(lVar12);
      if (cVar4 == '\0') {
        uVar7 = 0xffffffff;
      }
      if ((uVar5 & 1) != 0) {
        *(int *)(param_1 + 0x4e10) = iVar6;
        *(bool *)(param_1 + 0x4e14) = cVar1 != '\0';
        *(int *)(param_1 + 0x4e18) = iVar8;
        *(int *)(param_1 + 0x4e1c) = iVar9;
        *(undefined1 *)(param_1 + 0x4e30) = 0;
        iVar10 = FUN_1401ba9d0(lVar12 + 0x5b,*(undefined4 *)(lVar12 + 99));
        if (iVar10 < 1) {
          uVar7 = FUN_142cb5cb0(DAT_143aa84a0,iVar6,uVar7,cVar1 != '\0',iVar8 * 1000,iVar9 * 1000,0,
                                cVar2 != '\0',cVar3 != '\0');
          if (DAT_143acaa78 != 0) {
            return;
          }
          uVar11 = 2;
        }
        else {
          uVar7 = 0;
          uVar11 = 0;
        }
        FUN_142903e90(param_1,uVar11,0,0,uVar7);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1411a2ea0 @ 1411a2ea0   (1084 bytes)
//===========================================================

int * FUN_1411a2ea0(int *param_1,undefined1 param_2)

{
  longlong *plVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  longlong lVar5;
  IUnknown *pIVar6;
  int iVar7;
  undefined8 uVar8;
  int *piVar9;
  int *piVar10;
  longlong *plVar11;
  int *piVar12;
  int *piVar13;
  uint uVar14;
  int *piVar15;
  uint local_res10 [2];
  int *local_res18;
  IUnknown *local_res20;
  IUnknown *local_78;
  longlong *local_70;
  short local_68 [4];
  int *local_60;
  
  uVar8 = FUN_141d5d8c0(local_68,&DAT_143271f04,&DAT_143271f04,0x19,0xffffffff);
  FUN_142bf1bb0(param_1,uVar8);
  piVar13 = (int *)0x0;
  DAT_143acaa78 = param_1;
  if (param_1 == (int *)0xfffffffffffffdcf) {
    DAT_143acaa78 = piVar13;
  }
  *(undefined ***)param_1 = &PTR_FUN_14338acf8;
  *(undefined ***)(param_1 + 2) = &PTR_LAB_14338ae28;
  *(undefined ***)(param_1 + 6) = &PTR_FUN_14338af00;
  FUN_141aa3a20(param_1 + 0x8e);
  *(undefined1 *)(param_1 + 0x90) = param_2;
  param_1[0x91] = 0;
  param_1[0x92] = 0;
  param_1[0x93] = 0;
  param_1[0x94] = 0;
  param_1[0x96] = 0;
  param_1[0x97] = 0;
  local_res18 = (int *)0x0;
  piVar9 = (int *)FUN_14019b600(&DAT_143ad6a30,0x27);
  piVar9[1] = 0x16;
  *piVar9 = -1;
  local_res18 = piVar9 + 4;
  piVar9[2] = 0;
  *(undefined1 *)local_res18 = 0;
  uVar4 = s_UI_Revive_img_backgrnd_14338af08._12_4_;
  uVar3 = s_UI_Revive_img_backgrnd_14338af08._8_4_;
  uVar2 = s_UI_Revive_img_backgrnd_14338af08._4_4_;
  *local_res18 = s_UI_Revive_img_backgrnd_14338af08._0_4_;
  piVar9[5] = uVar2;
  piVar9[6] = uVar3;
  piVar9[7] = uVar4;
  piVar9[8] = s_UI_Revive_img_backgrnd_14338af08._16_4_;
  *(undefined2 *)(piVar9 + 9) = s_UI_Revive_img_backgrnd_14338af08._20_2_;
  if (*piVar9 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar9[1] < 0x16) {
    FUN_142e54290(0x90,piVar9[1],0x16);
  }
  *piVar9 = 1;
  *(undefined1 *)((longlong)local_res18 + 0x16) = 0;
  if (piVar9[1] + 1 < 0x17) {
    FUN_142e54290(0x9c,0x16);
  }
  piVar9[2] = 0x16;
  FUN_14090f3b0(&local_res20,&local_res18);
  if (local_res18 != (int *)0x0) {
    FUN_14019f2c0(local_res18 + -4);
  }
  pIVar6 = local_res20;
  piVar9 = piVar13;
  piVar15 = piVar13;
  if (local_res20 != (IUnknown *)0x0) {
    local_res10[0] = 0;
    iVar7 = (**(code **)(*(longlong *)local_res20 + 0x98))(local_res20,local_res10);
    if (iVar7 < 0) {
      _com_issue_errorex(iVar7,pIVar6,(_GUID *)&DAT_14327ac98);
    }
    pIVar6 = local_res20;
    piVar9 = (int *)(ulonglong)local_res10[0];
    if (local_res20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res10[0] = 0;
    iVar7 = (**(code **)(*(longlong *)local_res20 + 0xa0))(local_res20,local_res10);
    if (iVar7 < 0) {
      _com_issue_errorex(iVar7,pIVar6,(_GUID *)&DAT_14327ac98);
    }
    piVar15 = (int *)(ulonglong)local_res10[0];
  }
  uVar14 = (uint)piVar15;
  FUN_142bf2d70(param_1,-((int)piVar9 / 2),-((int)uVar14 / 2),piVar9,uVar14,0x271a,1,0,1,4);
  (*DAT_143262a20)(local_68);
  if (DAT_143a8b8d8 == 8) {
    if (local_68[0] == 8) {
      local_68[0] = 0;
      if (local_60 != (int *)0x0) {
        (*DAT_143ad5990)(local_60 + -1);
      }
    }
    else {
      iVar7 = (*DAT_143262a18)(local_68);
      if (iVar7 < 0) goto LAB_1411a32d6;
    }
    lVar5 = DAT_143a8b8e0;
    local_68[0] = 8;
    piVar12 = piVar13;
    if (DAT_143a8b8e0 != 0) {
      piVar12 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    piVar10 = (int *)(*DAT_143ad5980)((ulonglong)((int)piVar12 + 1) * 2 + 4);
    local_60 = piVar13;
    if (piVar10 != (int *)0x0) {
      *piVar10 = (int)piVar12 * 2;
      piVar10 = piVar10 + 1;
      if (lVar5 != 0) {
        FUN_142ef7ba0(piVar10,lVar5,(longlong)piVar12 * 2);
      }
      *(undefined2 *)((longlong)piVar10 + (longlong)piVar12 * 2) = 0;
      local_60 = piVar10;
    }
  }
  else {
    if ((local_68[0] == 8) && (local_68[0] = 0, local_60 != (int *)0x0)) {
      (*DAT_143ad5990)(local_60 + -1);
    }
    iVar7 = (*DAT_143262a28)(local_68,&DAT_143a8b8d8);
    if (iVar7 < 0) {
LAB_1411a32d6:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar7);
    }
  }
  plVar11 = (longlong *)FUN_141028310(&local_70,piVar9,piVar15,local_68,uVar14 & 0xffffff00);
  plVar1 = *(longlong **)(param_1 + 0x4e);
  if (plVar1 != (longlong *)*plVar11) {
    *(longlong **)(param_1 + 0x4e) = (longlong *)*plVar11;
    *plVar11 = 0;
    if (plVar1 != (longlong *)0x0) {
      (**(code **)(*plVar1 + 0x10))();
    }
  }
  if (local_70 != (longlong *)0x0) {
    (**(code **)(*local_70 + 0x10))();
  }
  if (local_68[0] == 8) {
    local_68[0] = 0;
    if (local_60 != (int *)0x0) {
      (*DAT_143ad5990)(local_60 + -1);
    }
  }
  else {
    (*DAT_143262a18)(local_68);
  }
  local_78 = local_res20;
  if (local_res20 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_res20 + 8))();
  }
  local_70 = *(longlong **)(param_1 + 0x4e);
  if (local_70 != (longlong *)0x0) {
    (**(code **)(*local_70 + 8))();
  }
  FUN_142aa1590(&local_70,&local_78,0,0,0xff);
  if (local_res20 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_res20 + 0x10))();
  }
  return param_1;
}



//===========================================================
// FUN_1411a3440 @ 1411a3440   (2109 bytes)
//===========================================================

void FUN_1411a3440(longlong param_1)

{
  longlong *plVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  longlong *plVar5;
  int iVar6;
  undefined8 uVar7;
  longlong lVar8;
  undefined8 *puVar9;
  longlong *plVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong *plStackX_18;
  undefined8 uStackX_20;
  uint in_stack_fffffffffffffe38;
  undefined4 uVar13;
  undefined1 auStack_178 [8];
  longlong lStack_170;
  undefined1 auStack_168 [8];
  longlong lStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  longlong *plStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined8 uStack_138;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  longlong lStack_128;
  longlong lStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  longlong *plStack_f8;
  longlong lStack_f0;
  undefined **ppuStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  int *piStack_d0;
  undefined4 uStack_c8;
  int iStack_c4;
  int iStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  longlong lStack_b0;
  undefined8 uStack_a8;
  longlong lStack_a0;
  longlong lStack_98;
  undefined8 uStack_90;
  longlong lStack_88;
  longlong lStack_80;
  undefined8 uStack_78;
  longlong lStack_70;
  undefined1 auStack_68 [8];
  undefined1 auStack_60 [8];
  undefined1 auStack_58 [8];
  undefined1 auStack_50 [8];
  undefined1 auStack_48 [8];
  undefined1 auStack_40 [8];
  
  lVar11 = param_1 + 0x238;
  FUN_141aa3af0(lVar11,param_1,0,0);
  uVar13 = 0;
  FUN_141ac3370(lVar11,L"UI/Revive.img",0,0,0,1,in_stack_fffffffffffffe38 & 0xffffff00,0);
  FUN_141ad4500(lVar11,auStack_178,L"town");
  if (lStack_170 != 0) {
    FUN_1408a9e40(&lStack_128,0x17de);
    if (lStack_170 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    lVar8 = lStack_170;
    uVar7 = FUN_1429fa100(auStack_68,0xffffffff,10,0,0);
    uStackX_20 = 0;
    FUN_14019a260(&uStackX_20,&lStack_128);
    FUN_14168b1d0(lVar8,&uStackX_20,uVar7,0xff,0,0,1);
    if (lStack_170 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    lVar8 = lStack_170;
    uVar7 = FUN_1429fa100(auStack_60,0xffffd3b1,10,0,0);
    FUN_14168b510(lVar8,1,uVar7);
    if (lStack_170 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    lVar8 = lStack_170;
    uVar7 = FUN_1429fa100(auStack_58,0xffdadada,10,0,0);
    FUN_14168b510(lVar8,2,uVar7);
    if (DAT_143aa84a0 == (longlong *)0x0) {
LAB_1411a3608:
      uStack_138 = 0;
      FUN_141adbce0(lVar11,&uStack_130,L"town_center",&uStack_138);
      if (lStack_170 == 0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_1417113f0(lStack_170,uStack_130,uStack_12c,0);
    }
    else {
      lVar8 = (**(code **)(*DAT_143aa84a0 + 0x30))();
      iVar6 = FUN_1401ba9d0(lVar8 + 0x200,*(undefined4 *)(lVar8 + 0x208));
      if (iVar6 < 1) goto LAB_1411a3608;
    }
    if (lStack_128 != 0) {
      FUN_14019f2c0(lStack_128 + -0x10);
    }
  }
  FUN_141ad6240(lVar11,auStack_168,L"spot");
  if (lStack_160 != 0) {
    FUN_1408a9e40(&lStack_120,0x17df);
    if (lStack_160 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    lVar8 = lStack_160;
    uVar7 = FUN_1429fa100(auStack_50,0xffffffff,10,0,0);
    uStack_158 = 0;
    FUN_14019a260(&uStack_158,&lStack_120);
    FUN_14168b1d0(lVar8,&uStack_158,uVar7,0xff,0,0,1);
    if (lStack_160 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    lVar8 = lStack_160;
    uVar7 = FUN_1429fa100(auStack_48,0xffffd3b1,10,0,0);
    FUN_14168b510(lVar8,1,uVar7);
    if (lStack_160 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    lVar8 = lStack_160;
    uVar7 = FUN_1429fa100(auStack_40,0xffdadada,10,0,0);
    FUN_14168b510(lVar8,2,uVar7);
    if (DAT_143aa84a0 == (longlong *)0x0) {
LAB_1411a37be:
      FUN_142aa1c30(lVar11,L"spot",0);
    }
    else {
      lVar8 = (**(code **)(*DAT_143aa84a0 + 0x30))();
      iVar6 = FUN_1401ba9d0(lVar8 + 0x200,*(undefined4 *)(lVar8 + 0x208));
      if (iVar6 < 1) goto LAB_1411a37be;
    }
    if (lStack_120 != 0) {
      FUN_14019f2c0(lStack_120 + -0x10);
    }
  }
  uStack_118 = 0;
  FUN_141adbce0(lVar11,&uStack_108,L"msg_lt",&uStack_118);
  uStack_110 = 0;
  FUN_141adbce0(lVar11,&uStack_100,L"msg_rb",&uStack_110);
  uStack_140 = uStack_100;
  uStack_13c = uStack_fc;
  *(undefined4 *)(param_1 + 0x244) = uStack_108;
  *(undefined4 *)(param_1 + 0x248) = uStack_104;
  *(undefined4 *)(param_1 + 0x24c) = uStack_100;
  *(undefined4 *)(param_1 + 0x250) = uStack_fc;
  iVar6 = *(int *)(param_1 + 0x24c);
  iVar2 = *(int *)(param_1 + 0x244);
  iVar3 = *(int *)(param_1 + 0x250);
  iVar4 = *(int *)(param_1 + 0x248);
  FUN_1408a9e40(&lStack_f0,0x17dd);
  FUN_1429fa100(&plStack_f8,0xffffffff,0xd,1,0);
  plStack_148 = plStack_f8;
  if (plStack_f8 != (longlong *)0x0) {
    (**(code **)(*plStack_f8 + 8))();
  }
  plStackX_18 = (longlong *)0x0;
  puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x10);
  plVar1 = plStack_148;
  if (puVar9 == (undefined8 *)0x0) {
    plStackX_18 = (longlong *)0x0;
  }
  else {
    plStackX_18 = puVar9 + 1;
    if (plStackX_18 != (longlong *)0x0) {
      *puVar9 = 1;
      for (plVar10 = plStackX_18; plVar10 < puVar9 + 2; plVar10 = plVar10 + 1) {
        *plVar10 = 0;
      }
    }
  }
  plVar5 = plStackX_18;
  plVar10 = (longlong *)*plStackX_18;
  if (plVar10 != plStack_148) {
    if (plStack_148 != (longlong *)0x0) {
      (**(code **)(*plStack_148 + 8))(plStack_148);
      plVar10 = (longlong *)*plVar5;
    }
    *plVar5 = (longlong)plVar1;
    if (plVar10 != (longlong *)0x0) {
      (**(code **)(*plVar10 + 0x10))();
    }
  }
  _eh_vector_destructor_iterator_(&plStack_148,8,1,FUN_1406fc540);
  piStack_d0 = (int *)0x0;
  uStack_e0 = 0;
  uStack_d8 = 0;
  ppuStack_e8 = &PTR_FUN_14336e7a8;
  lStack_b0 = 0;
  uStack_a8 = 0;
  lStack_a0 = 0;
  lStack_98 = 0;
  uStack_90 = 0;
  lStack_88 = 0;
  lStack_80 = 0;
  uStack_78 = 0;
  lStack_70 = 0;
  uStack_c8 = 0;
  uStack_bc = 0;
  uStack_b8 = 2;
  uStack_150 = 0;
  iStack_c4 = iVar6 - iVar2;
  iStack_c0 = iVar3 - iVar4;
  FUN_14019a260(&uStack_150,&lStack_f0);
  FUN_142a45580(&ppuStack_e8,&uStack_150,param_1 + 600,&plStackX_18,0,0,0,CONCAT44(uVar13,1),0,0,1,0
                ,0,0,0);
  ppuStack_e8 = &PTR_FUN_14336e7a8;
  if (lStack_80 != 0) {
    uVar12 = (lStack_70 - lStack_80 >> 2) * 4;
    lVar11 = lStack_80;
    if (0xfff < uVar12) {
      uVar12 = uVar12 + 0x27;
      lVar11 = *(longlong *)(lStack_80 + -8);
      if (0x1f < (lStack_80 - lVar11) - 8U) goto LAB_1411a3c78;
    }
    thunk_FUN_140205820(lVar11,uVar12);
    lStack_80 = 0;
    uStack_78 = 0;
    lStack_70 = 0;
  }
  if (lStack_98 != 0) {
    uVar12 = (lStack_88 - lStack_98 >> 2) * 4;
    lVar11 = lStack_98;
    if (0xfff < uVar12) {
      uVar12 = uVar12 + 0x27;
      lVar11 = *(longlong *)(lStack_98 + -8);
      if (0x1f < (lStack_98 - lVar11) - 8U) goto LAB_1411a3c78;
    }
    thunk_FUN_140205820(lVar11,uVar12);
    lStack_98 = 0;
    uStack_90 = 0;
    lStack_88 = 0;
  }
  if (lStack_b0 != 0) {
    uVar12 = (lStack_a0 - lStack_b0 >> 2) * 4;
    if (0xfff < uVar12) {
      uVar12 = uVar12 + 0x27;
      lVar11 = *(longlong *)(lStack_b0 + -8);
      if (0x1f < (lStack_b0 - lVar11) - 8U) {
LAB_1411a3c78:
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(lVar11,uVar12);
      }
    }
    thunk_FUN_140205820();
    lStack_b0 = 0;
    uStack_a8 = 0;
    lStack_a0 = 0;
  }
  ppuStack_e8 = &PTR_FUN_143273970;
  if (piStack_d0 != (int *)0x0) {
    LOCK();
    piStack_d0[2] = 0;
    piStack_d0[3] = 0;
    UNLOCK();
    do {
    } while (piStack_d0[1] != 0);
    if (piStack_d0 != (int *)0x0) {
      LOCK();
      iVar6 = *piStack_d0;
      *piStack_d0 = *piStack_d0 + -1;
      UNLOCK();
      if (iVar6 == 1) {
        thunk_FUN_140205820(piStack_d0,0x10);
      }
      piStack_d0 = (int *)0x0;
    }
  }
  if (plStackX_18 != (longlong *)0x0) {
    plVar1 = plStackX_18 + plStackX_18[-1];
    for (plVar10 = plStackX_18; plVar10 < plVar1; plVar10 = plVar10 + 1) {
      if ((longlong *)*plVar10 != (longlong *)0x0) {
        (**(code **)(*(longlong *)*plVar10 + 0x10))();
      }
    }
    thunk_FUN_140205820(plStackX_18 + -1,0);
    plStackX_18 = (longlong *)0x0;
  }
  if (plStack_f8 != (longlong *)0x0) {
    (**(code **)(*plStack_f8 + 0x10))();
  }
  if (lStack_f0 != 0) {
    FUN_14019f2c0(lStack_f0 + -0x10);
  }
  lVar11 = lStack_160;
  if (lStack_160 != 0) {
    if (0xffffe < *(longlong *)(lStack_160 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar11 + 0x20);
    lVar11 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar11 == 1) {
      puVar9 = (undefined8 *)(lStack_160 + 0x18);
      if (lStack_160 == 0) {
        puVar9 = (undefined8 *)0x0;
      }
      if (puVar9 != (undefined8 *)0x0) {
        (**(code **)*puVar9)(puVar9,1);
      }
    }
    lStack_160 = 0;
  }
  lVar11 = lStack_170;
  if (lStack_170 != 0) {
    if (0xffffe < *(longlong *)(lStack_170 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar11 + 0x20);
    lVar11 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar11 == 1) {
      puVar9 = (undefined8 *)(lStack_170 + 0x18);
      if (lStack_170 == 0) {
        puVar9 = (undefined8 *)0x0;
      }
      if (puVar9 != (undefined8 *)0x0) {
        (**(code **)*puVar9)(puVar9,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1411a3c90 @ 1411a3c90   (437 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1411a3c90(longlong param_1,undefined4 param_2)

{
  longlong *plVar1;
  char cVar2;
  int iVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined1 auStack_4b8 [32];
  undefined1 local_498;
  undefined1 local_490;
  undefined1 local_488;
  undefined1 local_480;
  undefined1 local_478;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4b8;
  cVar2 = FUN_142aa1a20(param_1 + 0x238,L"town",param_2);
  plVar1 = DAT_143aa84a0;
  if (cVar2 == '\0') {
    cVar2 = FUN_142aa1b80(param_1 + 0x238,L"spot",param_2);
    if (cVar2 == '\0') {
      FUN_142bf5d70(param_1,param_2);
    }
    else if (DAT_143aa84a0 != (longlong *)0x0) {
      lVar4 = (**(code **)(*DAT_143aa84a0 + 0x30))();
      iVar3 = FUN_1401ba9d0(lVar4 + 0x200,*(undefined4 *)(lVar4 + 0x208));
      plVar1 = DAT_143aa84a0;
      if ((0 < iVar3) && (DAT_143aa84a0 != (longlong *)0x0)) {
        FUN_1406ed520(local_468,0x1e7);
        FUN_1406ed840(local_468,0);
        FUN_1406ed840(local_468,*(undefined1 *)(param_1 + 0x240));
        FUN_1415d01c0(local_468);
        FUN_142d0b290(plVar1,1,2000);
        FUN_142cb5dc0(DAT_143aa84a0);
        FUN_1406ed610(local_468);
      }
    }
  }
  else if (DAT_143aa84a0 != (longlong *)0x0) {
    lVar4 = FUN_141892840();
    if (lVar4 != 0) {
      uVar5 = FUN_141892840();
      iVar3 = FUN_142cc4350(plVar1,500);
      local_490 = iVar3 != 0;
      local_478 = 0;
      local_480 = 1;
      local_488 = 1;
      local_498 = 0;
      iVar3 = FUN_1418283f0(uVar5,0,0,0);
      if (iVar3 != 0) {
        FUN_142d0b290(plVar1,1,2000);
        FUN_142cb5dc0(DAT_143aa84a0);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1411a3e50 @ 1411a3e50   (162 bytes)
//===========================================================

void FUN_1411a3e50(longlong param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  
  FUN_142bf5630();
  iVar2 = (**(code **)(*(longlong *)(param_1 + 8) + 0x90))(param_1 + 8);
  iVar3 = FUN_142bf7d70(param_1);
  iVar4 = FUN_1410a4dc0();
  if (iVar2 < 0) {
    iVar2 = 0;
  }
  iVar1 = iVar4 - iVar3;
  if (iVar2 <= iVar4 - iVar3) {
    iVar1 = iVar2;
  }
  iVar2 = (**(code **)(*(longlong *)(param_1 + 8) + 0x98))(param_1 + 8);
  iVar3 = FUN_142bf7d80(param_1);
  iVar4 = FUN_1410a4dd0();
  if (iVar2 < 0) {
    iVar2 = 0;
  }
  iVar5 = iVar4 - iVar3;
  if (iVar2 <= iVar4 - iVar3) {
    iVar5 = iVar2;
  }
  FUN_142bf7a70(param_1,iVar1,iVar5);
  return;
}



//===========================================================
// FUN_1411a3f00 @ 1411a3f00   (79 bytes)
//===========================================================

ulonglong FUN_1411a3f00(undefined8 param_1,undefined8 param_2,undefined8 param_3,longlong *param_4)

{
  int iVar1;
  ulonglong uVar2;
  
  uVar2 = FUN_142bf5dd0();
  if (((int)uVar2 != 0) &&
     (((param_4 == (longlong *)0x0 || (*param_4 == 0)) &&
      (iVar1 = FUN_142bf8580(param_1), uVar2 = uVar2 & 0xffffffff, iVar1 != 0)))) {
    uVar2 = 1;
  }
  return uVar2;
}



//===========================================================
// FUN_1411a3f60 @ 1411a3f60   (499 bytes)
//===========================================================

void FUN_1411a3f60(longlong param_1)

{
  undefined4 **ppuVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  uint uVar4;
  undefined4 *puVar5;
  longlong *plVar6;
  undefined4 *puVar7;
  longlong lVar8;
  undefined4 *puVar9;
  undefined4 *local_res18;
  longlong *local_res20;
  undefined1 local_68 [8];
  undefined1 *local_60;
  undefined4 *local_58;
  undefined1 local_50 [8];
  undefined8 local_48;
  
  FUN_142bf7e40();
  FUN_142bf6230(param_1,&local_res20);
  if (local_res20 != (longlong *)0x0) {
    local_60 = local_50;
    puVar9 = (undefined4 *)0x0;
    local_48 = 0;
    uVar2 = *(undefined4 *)(param_1 + 0x248);
    uVar3 = *(undefined4 *)(param_1 + 0x244);
    ppuVar1 = (undefined4 **)(param_1 + 600);
    local_res18 = (undefined4 *)0x0;
    if ((&local_res18 != ppuVar1) && (*ppuVar1 != (undefined4 *)0x0)) {
      uVar4 = (*ppuVar1)[-2];
      if (uVar4 != 0) {
        lVar8 = FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar4 * 0x58 + 8);
        if (lVar8 != 0) {
          puVar9 = (undefined4 *)(lVar8 + 8);
        }
        *(ulonglong *)(puVar9 + -2) = (ulonglong)uVar4;
        puVar5 = *ppuVar1;
        puVar7 = puVar5;
        local_res18 = puVar9;
        while (puVar7 < puVar5 + (ulonglong)uVar4 * 0x16) {
          *puVar9 = *puVar7;
          puVar9[1] = puVar7[1];
          puVar9[2] = puVar7[2];
          puVar9[3] = puVar7[3];
          puVar9[4] = puVar7[4];
          plVar6 = *(longlong **)(puVar7 + 6);
          *(longlong **)(puVar9 + 6) = plVar6;
          local_58 = puVar9;
          if (plVar6 != (longlong *)0x0) {
            (**(code **)(*plVar6 + 8))();
          }
          *(undefined8 *)(puVar9 + 8) = 0;
          FUN_14019a260(puVar9 + 8,puVar7 + 8);
          plVar6 = *(longlong **)(puVar7 + 10);
          *(longlong **)(puVar9 + 10) = plVar6;
          if (plVar6 != (longlong *)0x0) {
            (**(code **)(*plVar6 + 8))();
          }
          puVar9[0xc] = puVar7[0xc];
          puVar9[0xd] = puVar7[0xd];
          puVar9[0xe] = puVar7[0xe];
          puVar9[0xf] = puVar7[0xf];
          puVar9[0x10] = puVar7[0x10];
          puVar9[0x11] = puVar7[0x11];
          puVar9[0x12] = puVar7[0x12];
          puVar9[0x13] = puVar7[0x13];
          puVar9[0x14] = puVar7[0x14];
          puVar9[0x15] = puVar7[0x15];
          puVar7 = puVar7 + 0x16;
          puVar9 = puVar9 + 0x16;
        }
      }
    }
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 8))();
    }
    FUN_142a4e7f0(local_68,&local_res18,uVar3,uVar2,local_50);
  }
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))();
  }
  return;
}



//===========================================================
// FUN_1411a43c0 @ 1411a43c0   (358 bytes)
//===========================================================

undefined8 * FUN_1411a43c0(undefined8 *param_1,ulonglong param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  IUnknown *pIVar4;
  int iVar5;
  undefined4 *puVar6;
  wchar_t *pwVar7;
  undefined8 *local_res8;
  
  *param_1 = &PTR_FUN_14338acf8;
  param_1[1] = &PTR_LAB_14338ae28;
  param_1[3] = &PTR_FUN_14338af00;
  pIVar4 = DAT_143add058;
  if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  local_res8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
  if (local_res8 != (undefined8 *)0x0) {
    local_res8[1] = 0;
    *(undefined4 *)(local_res8 + 2) = 1;
    puVar6 = (undefined4 *)(*DAT_143ad5980)(0x20);
    if (puVar6 == (undefined4 *)0x0) {
      *local_res8 = 0;
    }
    else {
      *puVar6 = 0x1a;
      uVar3 = u_UI_Revive_img_14338af20._12_4_;
      uVar2 = u_UI_Revive_img_14338af20._8_4_;
      uVar1 = u_UI_Revive_img_14338af20._4_4_;
      pwVar7 = (wchar_t *)(puVar6 + 1);
      *(undefined4 *)pwVar7 = u_UI_Revive_img_14338af20._0_4_;
      puVar6[2] = uVar1;
      puVar6[3] = uVar2;
      puVar6[4] = uVar3;
      *(undefined8 *)(puVar6 + 5) = u_UI_Revive_img_14338af20._16_8_;
      puVar6[7] = u_UI_Revive_img_14338af20._24_4_;
      *local_res8 = pwVar7;
      if (pwVar7 != (wchar_t *)0x0) {
        iVar5 = (**(code **)(*(longlong *)pIVar4 + 0x68))(pIVar4,pwVar7);
        if (iVar5 < 0) {
          _com_issue_errorex(iVar5,pIVar4,(_GUID *)&DAT_1432743e8);
        }
        thunk_FUN_1401be120(&local_res8);
        FUN_140da2000(param_1 + 0x4b);
        FUN_141aa3a60(param_1 + 0x47);
        DAT_143acaa78 = 0;
        FUN_142bf1ec0(param_1);
        if ((param_2 & 1) != 0) {
          thunk_FUN_140205820(param_1,0x260);
        }
        return param_1;
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x8007000e);
}



//===========================================================
// FUN_1411a32f0 @ 1411a32f0   (318 bytes)
//===========================================================

void FUN_1411a32f0(undefined8 *param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  IUnknown *pIVar4;
  int iVar5;
  undefined4 *puVar6;
  wchar_t *pwVar7;
  undefined8 *local_res8;
  
  *param_1 = &PTR_FUN_14338acf8;
  param_1[1] = &PTR_LAB_14338ae28;
  param_1[3] = &PTR_FUN_14338af00;
  pIVar4 = DAT_143add058;
  if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  local_res8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
  if (local_res8 != (undefined8 *)0x0) {
    local_res8[1] = 0;
    *(undefined4 *)(local_res8 + 2) = 1;
    puVar6 = (undefined4 *)(*DAT_143ad5980)(0x20);
    if (puVar6 == (undefined4 *)0x0) {
      *local_res8 = 0;
    }
    else {
      *puVar6 = 0x1a;
      uVar3 = u_UI_Revive_img_14338af20._12_4_;
      uVar2 = u_UI_Revive_img_14338af20._8_4_;
      uVar1 = u_UI_Revive_img_14338af20._4_4_;
      pwVar7 = (wchar_t *)(puVar6 + 1);
      *(undefined4 *)pwVar7 = u_UI_Revive_img_14338af20._0_4_;
      puVar6[2] = uVar1;
      puVar6[3] = uVar2;
      puVar6[4] = uVar3;
      *(undefined8 *)(puVar6 + 5) = u_UI_Revive_img_14338af20._16_8_;
      puVar6[7] = u_UI_Revive_img_14338af20._24_4_;
      *local_res8 = pwVar7;
      if (pwVar7 != (wchar_t *)0x0) {
        iVar5 = (**(code **)(*(longlong *)pIVar4 + 0x68))(pIVar4,pwVar7);
        if (iVar5 < 0) {
          _com_issue_errorex(iVar5,pIVar4,(_GUID *)&DAT_1432743e8);
        }
        thunk_FUN_1401be120(&local_res8);
        FUN_140da2000(param_1 + 0x4b);
        FUN_141aa3a60(param_1 + 0x47);
        DAT_143acaa78 = 0;
        FUN_142bf1ec0(param_1);
        return;
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x8007000e);
}



//===========================================================
// FUN_142cb5cb0 @ 142cb5cb0   (251 bytes)
//===========================================================

undefined4
FUN_142cb5cb0(longlong param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
             undefined8 param_5,undefined8 param_6,int param_7)

{
  bool bVar1;
  char cVar2;
  undefined4 uVar3;
  longlong lVar4;
  
  lVar4 = FUN_141892840();
  if (lVar4 == 0) {
    return 1;
  }
  cVar2 = FUN_14182ffb0(lVar4);
  if (cVar2 != '\0') {
    return 2;
  }
  FUN_141829fd0(lVar4);
  uVar3 = FUN_1429e3ef0();
  if (*(int *)(param_1 + 0x37b0) != 0) {
    cVar2 = FUN_1408fc980(*(int *)(param_1 + 0x37b0),uVar3);
    if (cVar2 == '\0') {
      bVar1 = true;
      goto LAB_142cb5d1c;
    }
  }
  bVar1 = false;
LAB_142cb5d1c:
  if ((*(char *)(param_1 + 0x37ac) != '\0') && (bVar1)) {
    return 4;
  }
  cVar2 = FUN_142d1d910();
  if (cVar2 != '\0') {
    FUN_142d1d4d0();
  }
  if (DAT_143acaa78 == 0) {
    lVar4 = FUN_14019b780(&DAT_143ad68a0,0x260);
    if (lVar4 != 0) {
      FUN_1411a2ea0(lVar4,param_7 != 0,param_3,param_4,lVar4);
    }
    if (DAT_143abfdf8 != 0) {
      FUN_142c180e0();
    }
    return 5;
  }
  return 0;
}



//===========================================================
// FUN_142cb5dc0 @ 142cb5dc0   (58 bytes)
//===========================================================

void FUN_142cb5dc0(void)

{
  if (DAT_143acaa78 != 0) {
    FUN_142bf3f70();
    if (DAT_143acaa78 != 0) {
                    /* WARNING: Could not recover jumptable at 0x000142cb5df2. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)**(undefined8 **)(DAT_143acaa78 + 8))(DAT_143acaa78 + 8,1);
      return;
    }
  }
  return;
}



//===========================================================
// FUN_1429376c0 @ 1429376c0   (578 bytes)
//===========================================================

void FUN_1429376c0(longlong param_1)

{
  longlong *plVar1;
  undefined8 *puVar2;
  longlong *plVar3;
  bool bVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  longlong lVar7;
  char cVar8;
  int iVar9;
  undefined4 uVar10;
  undefined8 uVar11;
  undefined8 *puVar12;
  longlong *plVar13;
  undefined8 *puVar14;
  longlong *plVar15;
  longlong *local_38;
  undefined8 uStack_30;
  longlong *local_28;
  uint uStack_20;
  undefined4 uStack_1c;
  
  iVar9 = FUN_140f810b0(param_1 + 0x100);
  lVar7 = DAT_143aa84a0;
  if ((((iVar9 != 0) && (DAT_143aa84a0 != 0)) &&
      (cVar8 = FUN_142d0b2e0(DAT_143aa84a0), cVar8 == '\0')) &&
     ((DAT_143acaa78 != 0 && (*(uint *)(param_1 + 0x4e10) < 9)))) {
    uVar11 = FUN_142cbecc0(lVar7);
    cVar8 = FUN_142122480(uVar11,1);
    uVar10 = 0xffffffff;
    if (cVar8 != '\0') {
      uVar10 = 0;
    }
    uVar10 = FUN_142cb5cb0(lVar7,*(undefined4 *)(param_1 + 0x4e10),uVar10,
                           *(undefined1 *)(param_1 + 0x4e14),*(int *)(param_1 + 0x4e18) * 1000,
                           *(int *)(param_1 + 0x4e1c) * 1000,1,0,0);
    if ((*(char *)(param_1 + 0x4e30) == '\0') && (DAT_143acaa78 == 0)) {
      *(undefined1 *)(param_1 + 0x4e30) = 1;
      plVar1 = (longlong *)(param_1 + 20000);
      puVar2 = (undefined8 *)*plVar1;
      cVar8 = *(char *)((longlong)puVar2[1] + 0x19);
      puVar5 = puVar2;
      puVar14 = (undefined8 *)puVar2[1];
      while (cVar8 == '\0') {
        if (*(int *)((longlong)puVar14 + 0x1c) < 1) {
          puVar12 = (undefined8 *)puVar14[2];
          puVar14 = puVar5;
        }
        else {
          puVar12 = (undefined8 *)*puVar14;
        }
        puVar5 = puVar14;
        puVar14 = puVar12;
        cVar8 = *(char *)((longlong)puVar12 + 0x19);
      }
      if (((*(char *)((longlong)puVar5 + 0x19) != '\0') || (1 < *(int *)((longlong)puVar5 + 0x1c)))
         || (puVar5 == puVar2)) {
        iVar9 = FUN_140f810b0(param_1 + 0x100);
        FUN_142903e90(param_1,1,iVar9 != 0,0,uVar10);
        plVar3 = (longlong *)*plVar1;
        plVar13 = (longlong *)plVar3[1];
        uStack_20 = 0;
        cVar8 = *(char *)((longlong)plVar13 + 0x19);
        plVar15 = plVar3;
        local_28 = plVar13;
        while (plVar6 = plVar13, cVar8 == '\0') {
          bVar4 = 0 < *(int *)((longlong)plVar6 + 0x1c);
          if (bVar4) {
            plVar13 = (longlong *)*plVar6;
            plVar15 = plVar6;
          }
          else {
            plVar13 = (longlong *)plVar6[2];
          }
          uStack_20 = (uint)bVar4;
          cVar8 = *(char *)((longlong)plVar13 + 0x19);
          local_28 = plVar6;
        }
        if ((*(char *)((longlong)plVar15 + 0x19) != '\0') ||
           (1 < *(int *)((longlong)plVar15 + 0x1c))) {
          if (*(longlong *)(param_1 + 0x4e28) == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
            FUN_14019f9d0();
          }
          uStack_30 = 0;
          local_38 = plVar1;
          plVar13 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
          *(undefined4 *)((longlong)plVar13 + 0x1c) = 1;
          *(undefined4 *)(plVar13 + 4) = 1;
          *plVar13 = (longlong)plVar3;
          plVar13[1] = (longlong)plVar3;
          plVar13[2] = (longlong)plVar3;
          *(undefined2 *)(plVar13 + 3) = 0;
          local_38 = local_28;
          uStack_30 = CONCAT44(uStack_1c,uStack_20);
          FUN_140816ab0(plVar1,&local_38,plVar13);
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_142903e90 @ 142903e90   (405 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142903e90(longlong *param_1,undefined4 param_2,undefined1 param_3,undefined1 param_4,
                  undefined4 param_5)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined4 *puVar7;
  undefined1 auStack_4d8 [32];
  undefined4 local_4b8 [4];
  undefined1 local_4a8 [1104];
  ulonglong local_58;
  
  local_58 = DAT_143a8b908 ^ (ulonglong)auStack_4d8;
  lVar4 = FUN_142cbe730(DAT_143aa84a0);
  uVar5 = (**(code **)(*param_1 + 0x48))(param_1);
  uVar6 = FUN_140878830(uVar5,5);
  puVar7 = (undefined4 *)FUN_1408f5120(uVar6);
  uVar1 = *puVar7;
  uVar5 = FUN_140878830(uVar5,5);
  puVar7 = (undefined4 *)FUN_1408f51a0(uVar5);
  uVar2 = *puVar7;
  FUN_1406ed520(local_4a8,0x2c6);
  local_4b8[0] = 1;
  FUN_1406ede20(local_4a8,local_4b8,4);
  FUN_1406ed9d0(local_4a8,param_2);
  FUN_1406ed9d0(local_4a8,param_3);
  FUN_1406ed9d0(local_4a8,param_4);
  FUN_1406ed9d0(local_4a8,param_5);
  uVar3 = FUN_140f810d0(param_1 + 0x20);
  FUN_1406ed9d0(local_4a8,uVar3);
  uVar3 = FUN_140f82790(param_1 + 0x20,0,0);
  FUN_1406ed9d0(local_4a8,uVar3);
  uVar3 = FUN_1401ba9d0(lVar4 + 0x5b,*(undefined4 *)(lVar4 + 99));
  FUN_1406ed9d0(local_4a8,uVar3);
  FUN_1406ed9d0(local_4a8,uVar1);
  FUN_1406ed9d0(local_4a8,uVar2);
  FUN_1406ed9d0(local_4a8,(int)param_1[0x9c2]);
  FUN_1415d01c0(local_4a8);
  FUN_1406ed610(local_4a8);
  return;
}



//===========================================================
// FUN_1418283f0 @ 1418283f0   (74 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */

void FUN_1418283f0(void)

{
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}



//===========================================================
// FUN_142cbe730 @ 142cbe730   (8 bytes)
//===========================================================

undefined8 FUN_142cbe730(longlong param_1)

{
  return *(undefined8 *)(param_1 + 0x2358);
}



//===========================================================
// FUN_142cbecc0 @ 142cbecc0   (46 bytes)
//===========================================================

longlong FUN_142cbecc0(longlong param_1)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 0x2380);
  if (lVar1 == 0) {
    FUN_142e52ed0(0x428,0);
    lVar1 = *(longlong *)(param_1 + 0x2380);
  }
  return lVar1;
}



//===========================================================
// FUN_142122480 @ 142122480   (278 bytes)
//===========================================================

ulonglong FUN_142122480(longlong param_1,char param_2)

{
  ulonglong uVar1;
  char cVar2;
  ulonglong in_RAX;
  longlong lVar3;
  undefined4 *puVar4;
  ulonglong uVar5;
  longlong lVar6;
  int iVar7;
  undefined4 local_88 [32];
  
  uVar1 = *(ulonglong *)(param_1 + 0x88);
  while( true ) {
    if (uVar1 == 0) {
      return in_RAX & 0xffffffffffffff00;
    }
    lVar3 = *(longlong *)(uVar1 + 8);
    if (lVar3 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar3 = *(longlong *)(uVar1 + 8);
    }
    iVar7 = *(int *)(lVar3 + 0xa0);
    if (*(int *)(lVar3 + 0x9c) == 1) {
      iVar7 = -iVar7;
    }
    lVar6 = 0x1f;
    puVar4 = local_88;
    do {
      *puVar4 = *(undefined4 *)(((lVar3 + 0x20) - (longlong)local_88) + (longlong)puVar4);
      puVar4 = puVar4 + 1;
      lVar6 = lVar6 + -1;
    } while (lVar6 != 0);
    cVar2 = FUN_1407eca00(iVar7,local_88);
    if (((cVar2 != '\0') && ((param_2 == '\0' || (cVar2 = FUN_1407ecbb0(iVar7), cVar2 == '\0')))) &&
       ((499 < -iVar7 - 0x1e922cU ||
        ((iVar7 + 0x1e9275U < 0x1a && ((0x3088041U >> (iVar7 + 0x1e9275U & 0x1f) & 1) != 0))))))
    break;
    uVar5 = *(ulonglong *)(uVar1 - 0x20);
    if ((uVar5 != 0) && (uVar5 < 0x10001)) {
      FUN_142e52ed0(0x33e);
      uVar5 = *(ulonglong *)(uVar1 - 0x20);
    }
    in_RAX = uVar5 + 0x28;
    uVar1 = 0;
    if (uVar5 != 0) {
      uVar1 = in_RAX;
    }
  }
  return CONCAT71((uint7)(uint3)(-iVar7 - 0x1e922cU >> 8),1);
}



//===========================================================
// FUN_141892840 @ 141892840   (66 bytes)
//===========================================================

longlong FUN_141892840(void)

{
  int iVar1;
  longlong lVar2;
  
  lVar2 = FUN_14209ee40();
  lVar2 = *(longlong *)(lVar2 + 8);
  if (lVar2 != 0) {
    iVar1 = (**(code **)(*(longlong *)(lVar2 + 8) + 0xd0))(lVar2 + 8,&PTR_PTR_143a87ec8);
    if (iVar1 != 0) {
      return lVar2;
    }
  }
  return 0;
}



//===========================================================
// FUN_142d0b290 @ 142d0b290   (71 bytes)
//===========================================================

void FUN_142d0b290(longlong param_1,char param_2,int param_3)

{
  int iVar1;
  
  *(char *)(param_1 + 0x37ac) = param_2;
  if (param_2 != '\0') {
    iVar1 = FUN_1429e3ef0();
    *(int *)(param_1 + 0x37b0) = iVar1 + param_3;
    return;
  }
  *(undefined4 *)(param_1 + 0x37b0) = 0;
  return;
}



//===========================================================
// FUN_142aa1a20 @ 142aa1a20   (160 bytes)
//===========================================================

bool FUN_142aa1a20(undefined8 param_1,undefined8 param_2,int param_3)

{
  longlong *plVar1;
  longlong lVar2;
  int iVar3;
  undefined8 *puVar4;
  bool bVar5;
  undefined1 local_18 [8];
  longlong local_10;
  
  FUN_141ad4500(param_1,local_18,param_2);
  if (local_10 == 0) {
    bVar5 = false;
  }
  else {
    iVar3 = FUN_141710ad0(local_10);
    bVar5 = iVar3 == param_3;
  }
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
  return bVar5;
}



//===========================================================
// FUN_142aa1b80 @ 142aa1b80   (160 bytes)
//===========================================================

bool FUN_142aa1b80(undefined8 param_1,undefined8 param_2,int param_3)

{
  longlong *plVar1;
  longlong lVar2;
  int iVar3;
  undefined8 *puVar4;
  bool bVar5;
  undefined1 local_18 [8];
  longlong local_10;
  
  FUN_141ad6240(param_1,local_18,param_2);
  if (local_10 == 0) {
    bVar5 = false;
  }
  else {
    iVar3 = FUN_141710ad0(local_10);
    bVar5 = iVar3 == param_3;
  }
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
  return bVar5;
}



//===========================================================
// FUN_142cc4350 @ 142cc4350   (50 bytes)
//===========================================================

undefined1 FUN_142cc4350(longlong param_1,undefined4 param_2)

{
  undefined1 uVar1;
  undefined4 uVar2;
  
  uVar2 = FUN_1429e3ef0();
  uVar1 = FUN_1408fc970(*(undefined4 *)(param_1 + 0x2334),param_2,uVar2);
  return uVar1;
}



//===========================================================
// FUN_1408fc980 @ 1408fc980   (8 bytes)
//===========================================================

bool FUN_1408fc980(int param_1,int param_2)

{
  return 0 < param_2 - param_1;
}



//===========================================================
// FUN_142d1d910 @ 142d1d910   (88 bytes)
//===========================================================

bool FUN_142d1d910(void)

{
  IUnknown *pIVar1;
  long lVar2;
  int local_res8 [8];
  
  pIVar1 = DAT_143add050;
  if (DAT_143add050 != (IUnknown *)0x0) {
    local_res8[0] = 0;
    lVar2 = (**(code **)(*(longlong *)DAT_143add050 + 0x90))(DAT_143add050,local_res8);
    if (lVar2 < 0) {
      _com_issue_errorex(lVar2,pIVar1,(_GUID *)&DAT_14327fcd0);
    }
    return local_res8[0] == 0;
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}


