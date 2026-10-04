
//===========================================================
// FUN_142146d90 @ 142146d90   (719 bytes)
//===========================================================

undefined8 * FUN_142146d90(undefined8 *param_1)

{
  longlong *plVar1;
  int iVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  ulonglong uVar7;
  longlong *plVar8;
  longlong lVar9;
  ulonglong *puVar10;
  longlong local_res10;
  code *pcVar11;
  
  FUN_141c3cd20();
  *param_1 = &PTR_LAB_143431cc8;
  param_1[1] = &PTR_LAB_143431ea0;
  param_1[3] = &PTR_FUN_143431f78;
  param_1[0x9c] = 0;
  param_1[0x9d] = 0;
  param_1[0x9e] = 0;
  param_1[0x9f] = 0;
  param_1[0xa0] = 0;
  puVar10 = param_1 + 0xa1;
  pcVar11 = FUN_14214ea80;
  _eh_vector_constructor_iterator_(puVar10,8,2,(_func_void_void_ptr *)&LAB_14214e940,FUN_14214ea80);
  param_1[0xa5] = 0;
  param_1[0xa6] = 0;
  FUN_142644810(param_1 + 0xa7);
  param_1[0x2cc] = 0;
  param_1[0x2cd] = 0;
  FUN_141aa3a20(param_1 + 0x2ce);
  FUN_142cb1020(DAT_143aa84a0,0,0xffffffff,0,(ulonglong)pcVar11 & 0xffffffff00000000);
  FUN_14177f0e0(param_1,DAT_143ad6ec0,1,0,1,4);
  lVar9 = 2;
  do {
    puVar10[2] = 0;
    uVar7 = *puVar10;
    if (uVar7 != 0) {
      uVar5 = uVar7 + *(longlong *)(uVar7 - 8) * 0x18;
      if (uVar7 < uVar5) {
        do {
          FUN_1401abd80(uVar7);
          uVar7 = uVar7 + 0x18;
        } while (uVar7 < uVar5);
        uVar7 = *puVar10;
      }
      thunk_FUN_140205820(uVar7 - 8,0);
      *puVar10 = 0;
    }
    puVar3 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0);
    if (puVar3 == (undefined8 *)0x0) {
      *puVar10 = 0;
    }
    else {
      *puVar10 = (ulonglong)(puVar3 + 1);
      if (puVar3 + 1 != (undefined8 *)0x0) {
        *puVar3 = 9;
        uVar5 = *puVar10;
        uVar7 = uVar5 + 0xd8;
        for (; uVar5 < uVar7; uVar5 = uVar5 + 0x18) {
          *(undefined8 *)(uVar5 + 8) = 0;
          *(undefined4 *)(uVar5 + 0x10) = 0xffffffff;
        }
      }
    }
    puVar10 = puVar10 + 1;
    lVar9 = lVar9 + -1;
  } while (lVar9 != 0);
  plVar8 = (longlong *)param_1[0xa6];
  if (plVar8 != (longlong *)0x0) {
    plVar1 = plVar8 + plVar8[-1];
    if (plVar8 < plVar1) {
      do {
        if (*plVar8 != 0) {
          FUN_14019f2c0(*plVar8 + -0x10);
        }
        plVar8 = plVar8 + 1;
      } while (plVar8 < plVar1);
      plVar8 = (longlong *)param_1[0xa6];
    }
    thunk_FUN_140205820(plVar8 + -1,0);
    param_1[0xa6] = 0;
  }
  puVar3 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
  if (puVar3 == (undefined8 *)0x0) {
    param_1[0xa6] = 0;
  }
  else {
    param_1[0xa6] = puVar3 + 1;
    if (puVar3 + 1 != (undefined8 *)0x0) {
      *puVar3 = 2;
      puVar6 = (undefined8 *)param_1[0xa6];
      puVar3 = puVar6 + 2;
      for (; puVar6 < puVar3; puVar6 = puVar6 + 1) {
        *puVar6 = 0;
      }
    }
  }
  iVar2 = FUN_142d9ee10(DAT_143aa84a0);
  if (iVar2 != 0) {
    uVar4 = FUN_1408a9e40(&local_res10,0x4e2);
    FUN_1415eca30(uVar4,0xb);
    if (local_res10 != 0) {
      FUN_14019f2c0(local_res10 + -0x10);
    }
    *(undefined4 *)(DAT_143aa84a0 + 0x30f8) = 1;
  }
  return param_1;
}



//===========================================================
// FUN_142147070 @ 142147070   (234 bytes)
//===========================================================

void FUN_142147070(undefined8 *param_1)

{
  *param_1 = &PTR_LAB_143431cc8;
  param_1[1] = &PTR_LAB_143431ea0;
  param_1[3] = &PTR_FUN_143431f78;
  *(undefined4 *)(DAT_143aa84a0 + 0x30f8) = 0;
  FUN_141aa3a60(param_1 + 0x2ce);
  FUN_140da2000(param_1 + 0x2cd);
  FUN_140da2090(param_1 + 0x2cc);
  FUN_142644d50(param_1 + 0xa7);
  FUN_14022ea80(param_1 + 0xa6);
  _eh_vector_destructor_iterator_(param_1 + 0xa1,8,2,FUN_14214ea80);
  if ((longlong *)param_1[0x9f] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x9f] + 0x10))();
  }
  if ((longlong *)param_1[0x9e] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x9e] + 0x10))();
  }
  if ((longlong *)param_1[0x9d] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x9d] + 0x10))();
  }
  FUN_140d835a0(param_1 + 0x9b);
  FUN_141c3cdf0(param_1);
  return;
}



//===========================================================
// FUN_142147160 @ 142147160   (148 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142147160(undefined8 param_1,int param_2)

{
  undefined1 auStack_498 [32];
  undefined4 local_478 [4];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  if (param_2 == 2) {
    FUN_1406ed520(local_468,0x17e);
    local_478[0] = 0xc;
    FUN_1406ede20(local_468,local_478,4);
    FUN_1415d01c0(local_468);
    FUN_1406ed610(local_468);
  }
  FUN_14177fef0(param_1,param_2);
  return;
}



//===========================================================
// FUN_142147200 @ 142147200   (2588 bytes)
//===========================================================

void FUN_142147200(longlong *param_1)

{
  undefined8 *puVar1;
  longlong *plVar2;
  IUnknown *pIVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  int iVar6;
  longlong *plVar7;
  longlong lVar8;
  longlong lVar9;
  longlong *plVar10;
  undefined8 uVar11;
  longlong lVar12;
  longlong *plVar13;
  longlong *local_res20;
  ulonglong in_stack_fffffffffffffdc8;
  undefined4 uVar14;
  uint in_stack_fffffffffffffdd8;
  undefined4 uVar15;
  longlong *local_208;
  undefined4 local_200;
  undefined4 uStack_1fc;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  short local_1e8;
  undefined6 uStack_1e6;
  longlong lStack_1e0;
  undefined8 local_1d8;
  longlong *local_1d0;
  int local_1c8;
  int local_1c4;
  int local_1c0;
  int iStack_1bc;
  undefined8 local_1b8;
  undefined8 local_1b0;
  int local_1a8;
  int local_1a4;
  int local_1a0;
  int iStack_19c;
  undefined8 local_198;
  longlong *local_190;
  longlong *local_188;
  undefined4 local_180;
  undefined4 local_17c;
  longlong *local_178;
  uint local_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined8 local_160;
  longlong *local_158;
  undefined1 local_150 [8];
  longlong *local_148;
  uint local_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined8 local_130;
  undefined8 local_128;
  longlong lStack_120;
  undefined8 local_118;
  uint local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 local_f8;
  longlong local_e8;
  undefined4 local_e0;
  undefined4 local_dc;
  longlong local_d0;
  undefined4 local_c4;
  undefined4 local_b4;
  undefined4 local_a8;
  longlong local_88;
  longlong local_70;
  longlong local_58;
  longlong *local_48;
  longlong *local_40;
  
  FUN_14090ead0(&local_res20,DAT_143ad6eb8);
  if (local_res20 != (longlong *)0x0) {
    plVar2 = param_1 + 0x2ce;
    FUN_141aa3af0(plVar2,param_1,0,0);
    lVar12 = 0;
    in_stack_fffffffffffffdc8 = in_stack_fffffffffffffdc8 & 0xffffffff00000000;
    FUN_141ac3370(plVar2,DAT_143ad6eb8,0,0,in_stack_fffffffffffffdc8,1,
                  in_stack_fffffffffffffdd8 & 0xffffff00,0);
    uVar14 = (undefined4)(in_stack_fffffffffffffdc8 >> 0x20);
    local_1d0 = (longlong *)0x0;
    FUN_141adbce0(plVar2,&local_1c0,L"editChat_lt",&local_1d0);
    local_208 = (longlong *)0x0;
    FUN_141adbce0(plVar2,&local_1c8,L"editChat_rb",&local_208);
    FUN_1416a2090(&local_e8);
    local_b4 = 0;
    local_c4 = 0xff555555;
    local_e0 = 2;
    local_dc = 2;
    plVar7 = (longlong *)FUN_1408a9d80(&local_158,0x1801);
    if (&local_d0 != plVar7) {
      FUN_1401be120(&local_d0);
      local_d0 = *plVar7;
      if (local_d0 != 0) {
        LOCK();
        *(int *)(local_d0 + 0x10) = *(int *)(local_d0 + 0x10) + 1;
        UNLOCK();
      }
    }
    if (local_158 != (longlong *)0x0) {
      LOCK();
      plVar7 = local_158 + 2;
      lVar9 = *plVar7;
      *(int *)plVar7 = (int)*plVar7 + -1;
      UNLOCK();
      if (((int)lVar9 == 1) && (local_158 != (longlong *)0x0)) {
        if (*local_158 != 0) {
          (*DAT_143ad5990)(*local_158 + -4);
          *local_158 = 0;
        }
        if (local_158[1] != 0) {
          FUN_14019b4e0();
          local_158[1] = 0;
        }
        thunk_FUN_140205820(local_158,0x18);
      }
    }
    lVar8 = FUN_14019b780(&DAT_143ad68a0,0x220);
    lVar9 = lVar12;
    if (lVar8 != 0) {
      lVar9 = FUN_1416a6bf0(lVar8);
    }
    if ((param_1[0x9c] - 1U < 999) || (param_1[0x9c] == -1)) {
      FUN_142e52ed0(0x447);
    }
    lVar8 = lVar9 + 0x18;
    if (lVar9 == 0) {
      lVar8 = lVar12;
    }
    if ((lVar8 != 0) && (lVar12 = lVar8 + -0x18, lVar12 != 0)) {
      if (0xfffff < *(ulonglong *)(lVar8 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar8 + 8) = *(longlong *)(lVar8 + 8) + 1;
      UNLOCK();
    }
    lVar9 = param_1[0x9c];
    param_1[0x9c] = lVar12;
    if (lVar9 != 0) {
      if (0xffffe < *(longlong *)(lVar9 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar7 = (longlong *)(lVar9 + 0x20);
      lVar12 = *plVar7;
      *plVar7 = *plVar7 + -1;
      UNLOCK();
      if (((int)lVar12 == 1) && (puVar1 = (undefined8 *)(lVar9 + 0x18), puVar1 != (undefined8 *)0x0)
         ) {
        (**(code **)*puVar1)(puVar1,1);
      }
    }
    local_a8 = 0x100;
    plVar7 = (longlong *)param_1[0x9c];
    if (plVar7 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar7 = (longlong *)param_1[0x9c];
    }
    plVar10 = &local_e8;
    uVar11 = CONCAT44(uVar14,iStack_1bc);
    (**(code **)(*plVar7 + 0x10))
              (plVar7,param_1,0,local_1c0,uVar11,local_1c8 - local_1c0,local_1c4 - iStack_1bc,
               plVar10);
    uVar15 = (undefined4)((ulonglong)plVar10 >> 0x20);
    uVar14 = (undefined4)((ulonglong)uVar11 >> 0x20);
    FUN_142bf7c90(param_1,param_1[0x9c]);
    local_1b8 = 0;
    FUN_141adbce0(plVar2,&local_1a0,L"chat_lt",&local_1b8);
    local_1b0 = 0;
    FUN_141adbce0(plVar2,&local_1a8,L"chat_rb",&local_1b0);
    uVar11 = CONCAT44(uVar14,local_1a4 - iStack_19c);
    (**(code **)(*param_1 + 0x1c0))(param_1,local_1a0,iStack_19c,local_1a8 - local_1a0,uVar11,0,5);
    uVar14 = (undefined4)((ulonglong)uVar11 >> 0x20);
    local_198 = 0;
    FUN_141adbce0(plVar2,&local_180,L"scrollBar_lt",&local_198);
    local_190 = local_res20;
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 8))();
    }
    uVar4 = FUN_140910ca0(&local_190,"scrollBarLength",0);
    local_188 = local_res20;
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 8))();
    }
    uVar5 = FUN_140910ca0(&local_188,"scrollBarWheelRange",0);
    lVar9 = FUN_14019b780(&DAT_143ad68a0,0x108);
    lVar12 = 0;
    if (lVar9 != 0) {
      lVar12 = FUN_1416ed1a0(lVar9);
    }
    if ((param_1[0x54] - 1U < 999) || (param_1[0x54] == -1)) {
      FUN_142e52ed0(0x447);
    }
    lVar9 = lVar12 + 0x18;
    if (lVar12 == 0) {
      lVar9 = 0;
    }
    if (lVar9 == 0) {
      lVar12 = 0;
    }
    else {
      lVar12 = lVar9 + -0x18;
      if (lVar12 != 0) {
        if (0xfffff < *(ulonglong *)(lVar9 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar9 + 8) = *(longlong *)(lVar9 + 8) + 1;
        UNLOCK();
      }
    }
    lVar9 = param_1[0x54];
    param_1[0x54] = lVar12;
    if (lVar9 != 0) {
      if (0xffffe < *(longlong *)(lVar9 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar7 = (longlong *)(lVar9 + 0x20);
      lVar12 = *plVar7;
      *plVar7 = *plVar7 + -1;
      UNLOCK();
      if (((int)lVar12 == 1) && (puVar1 = (undefined8 *)(lVar9 + 0x18), puVar1 != (undefined8 *)0x0)
         ) {
        (**(code **)*puVar1)(puVar1,1);
      }
    }
    plVar7 = (longlong *)param_1[0x54];
    if (plVar7 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar7 = (longlong *)param_1[0x54];
    }
    (**(code **)(*plVar7 + 0x80))
              (plVar7,param_1,1000,1,CONCAT44(uVar14,4),local_180,local_17c,CONCAT44(uVar15,uVar4),0
              );
    lVar12 = param_1[0x54];
    if (lVar12 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar12 = param_1[0x54];
    }
    *(undefined4 *)(lVar12 + 0x78) = uVar5;
    plVar10 = (longlong *)FUN_1429fbeb0(&local_1d0,0x26);
    plVar7 = (longlong *)param_1[0x9d];
    plVar13 = (longlong *)0x0;
    if (plVar7 != (longlong *)*plVar10) {
      param_1[0x9d] = *plVar10;
      *plVar10 = 0;
      if (plVar7 != (longlong *)0x0) {
        (**(code **)(*plVar7 + 0x10))();
      }
    }
    if (local_1d0 != (longlong *)0x0) {
      (**(code **)(*local_1d0 + 0x10))();
    }
    plVar10 = (longlong *)FUN_1429fbeb0(&local_178,0x59);
    plVar7 = (longlong *)param_1[0x9e];
    if (plVar7 != (longlong *)*plVar10) {
      param_1[0x9e] = *plVar10;
      *plVar10 = 0;
      if (plVar7 != (longlong *)0x0) {
        (**(code **)(*plVar7 + 0x10))();
      }
    }
    if (local_178 != (longlong *)0x0) {
      (**(code **)(*local_178 + 0x10))();
    }
    pIVar3 = DAT_143add058;
    if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_1e8);
    if (DAT_143a8b8d8 == 8) {
      if (local_1e8 == 8) {
        local_1e8 = 0;
        if (lStack_1e0 != 0) {
          (*DAT_143ad5990)(lStack_1e0 + -4);
        }
      }
      else {
        iVar6 = (*DAT_143262a18)(&local_1e8);
        if (iVar6 < 0) goto LAB_142147c05;
      }
      local_1e8 = 8;
      plVar7 = plVar13;
      if (DAT_143a8b8e0 != 0) {
        plVar7 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      lStack_1e0 = FUN_1401a5fa0(DAT_143a8b8e0,plVar7);
    }
    else {
      if ((local_1e8 == 8) && (local_1e8 = 0, lStack_1e0 != 0)) {
        (*DAT_143ad5990)(lStack_1e0 + -4);
      }
      iVar6 = (*DAT_143262a28)(&local_1e8,&DAT_143a8b8d8);
      if (iVar6 < 0) {
LAB_142147c05:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar6);
      }
    }
    (*DAT_143262a20)(&local_200);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_200 == 8) {
        local_200 = (uint)local_200._2_2_ << 0x10;
        if (uStack_1f8 != 0) {
          (*DAT_143ad5990)(uStack_1f8 + -4);
        }
      }
      else {
        iVar6 = (*DAT_143262a18)(&local_200);
        if (iVar6 < 0) goto LAB_142147c0d;
      }
      local_200 = CONCAT22(local_200._2_2_,8);
      plVar7 = plVar13;
      if (DAT_143a8b8e0 != 0) {
        plVar7 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_1f8 = FUN_1401a5fa0(DAT_143a8b8e0,plVar7);
    }
    else {
      if (((short)local_200 == 8) && (local_200 = (uint)local_200._2_2_ << 0x10, uStack_1f8 != 0)) {
        (*DAT_143ad5990)(uStack_1f8 + -4);
      }
      iVar6 = (*DAT_143262a28)(&local_200,&DAT_143a8b8d8);
      if (iVar6 < 0) {
LAB_142147c0d:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar6);
      }
    }
    plVar10 = (longlong *)FUN_1401a5890(local_150,PTR_u_UI_Basic_img_ItemNo_143a46b70);
    (*DAT_143262a20)(&local_140);
    plVar7 = plVar13;
    if ((undefined8 *)*plVar10 != (undefined8 *)0x0) {
      plVar7 = *(longlong **)*plVar10;
    }
    local_128 = CONCAT62(uStack_1e6,local_1e8);
    lStack_120 = lStack_1e0;
    local_118 = local_1d8;
    local_108 = local_200;
    uStack_104 = uStack_1fc;
    uStack_100 = (undefined4)uStack_1f8;
    uStack_fc = uStack_1f8._4_4_;
    local_f8 = local_1f0;
    iVar6 = (**(code **)(*(longlong *)pIVar3 + 0x48))
                      (pIVar3,plVar7,&local_108,&local_128,&local_140);
    if (iVar6 < 0) {
      _com_issue_errorex(iVar6,pIVar3,(_GUID *)&DAT_1432743e8);
    }
    local_170 = local_140;
    uStack_16c = uStack_13c;
    uStack_168 = uStack_138;
    uStack_164 = uStack_134;
    local_160 = local_130;
    local_140 = local_140 & 0xffff0000;
    FUN_1401be120(plVar10);
    plVar7 = (longlong *)FUN_1409339d0(&local_148,&local_170);
    plVar7 = (longlong *)*plVar7;
    plVar10 = plVar13;
    if (plVar7 == (longlong *)0x0) {
      iVar6 = -0x7fffbffe;
    }
    else {
      (**(code **)(*plVar7 + 8))(plVar7);
      local_208 = (longlong *)0x0;
      iVar6 = (**(code **)*plVar7)(plVar7,&DAT_143272478,&local_208);
      if (-1 < iVar6) {
        plVar10 = local_208;
      }
    }
    if (plVar7 != (longlong *)0x0) {
      (**(code **)(*plVar7 + 0x10))(plVar7);
    }
    if (((iVar6 + 0x80000000U & 0x80000000) == 0) && (iVar6 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar6);
    }
    plVar7 = (longlong *)param_1[0x9f];
    if ((plVar7 != plVar10) &&
       (param_1[0x9f] = (longlong)plVar10, plVar10 = plVar13, plVar7 != (longlong *)0x0)) {
      (**(code **)(*plVar7 + 0x10))();
    }
    if (plVar10 != (longlong *)0x0) {
      (**(code **)(*plVar10 + 0x10))(plVar10);
    }
    if (local_148 != (longlong *)0x0) {
      (**(code **)(*local_148 + 0x10))();
    }
    if ((short)local_170 == 8) {
      local_170 = local_170 & 0xffff0000;
      if (CONCAT44(uStack_164,uStack_168) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_164,uStack_168) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_170);
    }
    if ((short)local_200 == 8) {
      local_200 = local_200 & 0xffff0000;
      if (uStack_1f8 != 0) {
        (*DAT_143ad5990)(uStack_1f8 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_200);
    }
    if (local_1e8 == 8) {
      local_1e8 = 0;
      if (lStack_1e0 != 0) {
        (*DAT_143ad5990)(lStack_1e0 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_1e8);
    }
    uVar11 = FUN_142cbe730(DAT_143aa84a0);
    lVar12 = FUN_1401d35e0(uVar11);
    param_1[0xa5] = lVar12;
    if (DAT_143ac8240 != 0) {
      FUN_14238b3d0(DAT_143ac8240,0);
    }
    FUN_14214d790(param_1);
    FUN_142aa2010(plVar2,L"claim",0);
    if (local_40 != (longlong *)0x0) {
      (**(code **)(*local_40 + 0x10))();
    }
    if (local_48 != (longlong *)0x0) {
      (**(code **)(*local_48 + 0x10))();
    }
    if (local_58 != 0) {
      FUN_1401bebb0(local_58 + -0x10);
    }
    if (local_70 != 0) {
      thunk_FUN_140205820(local_70,0xc);
    }
    if (local_88 != 0) {
      thunk_FUN_140205820(local_88,0xc);
    }
    FUN_1401be120(&local_d0);
    if (local_e8 != 0) {
      FUN_14019f2c0(local_e8 + -0x10);
    }
  }
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))();
  }
  return;
}



//===========================================================
// FUN_142147c30 @ 142147c30   (127 bytes)
//===========================================================

undefined8 FUN_142147c30(undefined8 param_1,uint param_2,int param_3,longlong *param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_142bf5dd0();
  if ((int)uVar1 == 2) {
    if ((0x10e < (int)param_2) && (0x125 < param_3)) {
      return 0;
    }
  }
  else if ((int)uVar1 == 0) {
    return uVar1;
  }
  if (((param_4 == (longlong *)0x0) || (*param_4 == 0)) &&
     ((0x10e < param_2 || (uVar1 = 2, 0x1d < param_3 - 0x96U)))) {
    uVar1 = 1;
  }
  return uVar1;
}



//===========================================================
// FUN_142147cc0 @ 142147cc0   (779 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142147ed7) */

void FUN_142147cc0(longlong param_1,int param_2,int param_3)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong *plVar7;
  int *piVar8;
  ulonglong uVar9;
  int iVar10;
  ulonglong uVar11;
  int iVar12;
  bool bVar13;
  int aiStackX_18 [2];
  int *piStackX_20;
  int *piStack_48;
  int *piStack_40;
  
  if (param_3 < 0) {
    return;
  }
  bVar13 = false;
  iVar12 = 0;
  aiStackX_18[0] = 0;
  if (param_2 != 0xd) {
    return;
  }
  lVar5 = *(longlong *)(param_1 + 0x4d8);
  if (lVar5 == 0) {
LAB_142147d58:
    bVar1 = true;
  }
  else {
    if (*(longlong *)(lVar5 + 0xc0) != 0) {
      iVar12 = *(int *)(*(longlong *)(lVar5 + 0xc0) + -8);
    }
    piStack_48 = (int *)0x0;
    lVar6 = 0xc0;
    if (iVar12 < 1) {
      lVar6 = 200;
    }
    FUN_14019a260(&piStack_48,lVar6 + lVar5);
    bVar13 = true;
    if ((piStack_48 == (int *)0x0) || ((char)*piStack_48 == '\0')) goto LAB_142147d58;
    bVar1 = false;
  }
  if ((bVar13) && (piStack_48 != (int *)0x0)) {
    FUN_14019f2c0(piStack_48 + -4);
  }
  if (bVar1) {
    return;
  }
  lVar5 = *(longlong *)(param_1 + 0x4d8);
  if (lVar5 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar5 = *(longlong *)(param_1 + 0x4d8);
  }
  plVar7 = (longlong *)(lVar5 + 0xc0);
  if ((*plVar7 == 0) || (*(int *)(*plVar7 + -8) < 1)) {
    plVar7 = (longlong *)(lVar5 + 200);
  }
  piStack_40 = (int *)0x0;
  FUN_14019a260(&piStack_40,plVar7);
  piVar8 = piStack_40;
  aiStackX_18[0] = 1;
  piStackX_20 = (int *)0x0;
  piVar2 = piStackX_20;
  if ((piStack_40 == (int *)0x0) || (piVar3 = piStack_40 + -4, piVar3 == (int *)0x0))
  goto LAB_142147f40;
  if (*piVar3 != -1) {
    if (*piVar3 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar3 = *piVar3 + 1;
    UNLOCK();
    if (piStackX_20 != (int *)0x0) {
      FUN_14019f2c0(piStackX_20 + -4);
    }
    piStackX_20 = piVar8;
    piVar8 = piStack_40;
    piVar2 = piStackX_20;
    goto LAB_142147f40;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  uVar9 = 0xffffffffffffffff;
  uVar11 = 0xffffffffffffffff;
  do {
    uVar11 = uVar11 + 1;
  } while (*(char *)((longlong)piVar8 + uVar11) != '\0');
  iVar10 = (int)uVar11;
  iVar12 = 0;
  if (0 < iVar10) {
    iVar12 = iVar10;
  }
  piVar3 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar12 + 0x11));
  piVar3[1] = iVar12;
  *piVar3 = -1;
  piVar2 = piVar3 + 4;
  piVar3[2] = 0;
  *(char *)piVar2 = '\0';
  piStack_48 = piVar2;
  FUN_142ef7ba0(piVar2,piVar8,(longlong)iVar10);
  if (*piVar3 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar10 == -1) || (iVar10 <= piVar3[1])) {
    *piVar3 = 1;
    if (iVar10 != -1) goto LAB_142147e8a;
    if (piVar2 != (int *)0x0) {
      do {
        uVar9 = uVar9 + 1;
      } while (*(char *)((longlong)piVar2 + uVar9) != '\0');
      uVar11 = uVar9 & 0xffffffff;
      goto LAB_142147e8f;
    }
    iVar12 = 0;
  }
  else {
    FUN_142e54290(0x90,piVar3[1],uVar11 & 0xffffffff);
    *piVar3 = 1;
LAB_142147e8a:
    *(char *)((longlong)iVar10 + (longlong)piVar2) = '\0';
LAB_142147e8f:
    iVar12 = (int)uVar11;
  }
  if ((iVar12 < 0) || (piVar3[1] + 1 <= iVar12)) {
    FUN_142e54290(0x9c,iVar12);
  }
  piVar3[2] = iVar12;
  if (piStackX_20 != (int *)0x0) {
    FUN_14019f2c0(piStackX_20 + -4);
  }
LAB_142147f40:
  piStackX_20 = piVar2;
  FUN_141c3fcb0(param_1 + -8,&piStackX_20,aiStackX_18);
  lVar5 = *(longlong *)(param_1 + 0x4d8);
  if (lVar5 != 0) {
    if (aiStackX_18[0] != 0) {
      uVar4 = FUN_141060460(param_1 + 0x4d0);
      FUN_1416a7070(uVar4,0);
      lVar5 = *(longlong *)(param_1 + 0x4d8);
    }
    lVar6 = lVar5 + 8;
    if (lVar5 == 0) {
      lVar6 = 0;
    }
    FUN_142c0bf50(DAT_143abfdf8,lVar6,0);
  }
  if (piVar8 != (int *)0x0) {
    FUN_14019f2c0(piVar8 + -4);
  }
  return;
}



//===========================================================
// FUN_142147fe0 @ 142147fe0   (760 bytes)
//===========================================================

void FUN_142147fe0(longlong *param_1,int param_2,undefined4 param_3,int param_4,int param_5)

{
  undefined8 uVar1;
  int iVar2;
  int iVar3;
  longlong lVar4;
  int *piVar5;
  int *local_res8;
  undefined1 local_48 [8];
  undefined8 local_40;
  
  if (param_2 == 0x204) {
    FUN_142ced860(DAT_143aa84a0,1);
    FUN_142645170(param_1 + 0xa6);
    iVar2 = FUN_14214d2f0(param_1 + -1,param_4,param_5);
    if (iVar2 == 0) goto LAB_1421482a8;
    if (iVar2 < 1) {
      iVar2 = -iVar2;
      lVar4 = 0x508;
    }
    else {
      lVar4 = 0x500;
    }
    lVar4 = FUN_14214e8d0(lVar4 + (longlong)param_1,iVar2 + -1);
    if ((lVar4 == 0) || (*(int *)(lVar4 + 0x10) < 0)) goto LAB_1421482a8;
    local_40 = 0;
    local_res8 = (int *)0x0;
    piVar5 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar5[1] = 0;
    *piVar5 = -1;
    local_res8 = piVar5 + 4;
    piVar5[2] = 0;
    *(undefined1 *)local_res8 = 0;
    if (*piVar5 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar5[1] < 0) {
      FUN_142e54290(0x90,piVar5[1],0);
    }
    *piVar5 = 1;
    *(undefined1 *)local_res8 = 0;
    if (piVar5[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar5[2] = 0;
  }
  else {
    if (param_2 != 0x205) goto LAB_1421482a8;
    FUN_142ced860(DAT_143aa84a0,0);
    FUN_142645170(param_1 + 0xa6);
    iVar2 = FUN_14214d2f0(param_1 + -1,param_4,param_5);
    if (iVar2 == 0) goto LAB_1421482a8;
    if (iVar2 < 1) {
      iVar2 = -iVar2;
      lVar4 = 0x508;
    }
    else {
      lVar4 = 0x500;
    }
    lVar4 = FUN_14214e8d0(lVar4 + (longlong)param_1,iVar2 + -1);
    if ((lVar4 == 0) || (*(int *)(lVar4 + 0x10) < 0)) goto LAB_1421482a8;
    local_40 = 0;
    local_res8 = (int *)0x0;
    piVar5 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar5[1] = 0;
    *piVar5 = -1;
    local_res8 = piVar5 + 4;
    piVar5[2] = 0;
    *(undefined1 *)local_res8 = 0;
    if (*piVar5 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar5[1] < 0) {
      FUN_142e54290(0x90,piVar5[1],0);
    }
    *piVar5 = 1;
    *(undefined1 *)local_res8 = 0;
    if (piVar5[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar5[2] = 0;
  }
  uVar1 = *(undefined8 *)(lVar4 + 8);
  iVar2 = (**(code **)(*param_1 + 0x98))(param_1);
  iVar3 = (**(code **)(*param_1 + 0x90))(param_1);
  FUN_142694130(param_1 + 0xa6,param_4 + iVar3,iVar2 + param_5 + 0x14,uVar1,0,0,0,0,0,0,0,
                &local_res8,local_48,0);
LAB_1421482a8:
  FUN_142bf2520(param_1,param_2,param_3,param_4);
  return;
}



//===========================================================
// FUN_1421482e0 @ 1421482e0   (884 bytes)
//===========================================================

undefined8 FUN_1421482e0(longlong *param_1,int param_2,int param_3)

{
  int iVar1;
  int iVar2;
  longlong lVar3;
  int *piVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  uint uVar7;
  int *local_res8;
  undefined8 in_stack_ffffffffffffff68;
  undefined4 uVar8;
  undefined8 in_stack_ffffffffffffff70;
  undefined4 uVar9;
  undefined1 local_40 [8];
  undefined8 local_38;
  
  uVar8 = (undefined4)((ulonglong)in_stack_ffffffffffffff68 >> 0x20);
  uVar9 = (undefined4)((ulonglong)in_stack_ffffffffffffff70 >> 0x20);
  iVar1 = FUN_14214d4a0(param_1 + -1);
  if (iVar1 == 1) {
    puVar5 = (undefined8 *)param_1[0xa5];
    if ((puVar5 == (undefined8 *)0x0) || (*(int *)(puVar5 + -1) == 0)) {
      FUN_142e54290(0xbc,0,0);
      puVar5 = (undefined8 *)param_1[0xa5];
    }
    uVar6 = *puVar5;
  }
  else {
    if (iVar1 != 2) {
      if (iVar1 != 3) {
        if (iVar1 == 4) {
          iVar1 = FUN_141c3f8b0(param_1 + -1);
          puVar5 = (undefined8 *)FUN_141c3f9f0(param_1 + -1,&local_res8,1 - iVar1);
          uVar6 = *puVar5;
          iVar1 = (**(code **)(*param_1 + 0x98))(param_1);
          iVar2 = (**(code **)(*param_1 + 0x90))(param_1);
          FUN_1426455d0(param_1 + 0xa6,param_2 + 0x14 + iVar2,iVar1 + param_3 + 0x14,uVar6,
                        CONCAT44(uVar8,0xcc0e395a),CONCAT44(uVar9,4),1);
          if (local_res8 == (int *)0x0) {
            return 1;
          }
          FUN_14019f2c0(local_res8 + -4);
          return 1;
        }
        iVar1 = FUN_14214d2f0(param_1 + -1,param_2,param_3);
        if (iVar1 != 0) {
          if (iVar1 < 1) {
            iVar1 = -iVar1;
            lVar3 = 0x508;
          }
          else {
            lVar3 = 0x500;
          }
          lVar3 = FUN_14214e8d0(lVar3 + (longlong)param_1,iVar1 + -1);
          if ((lVar3 != 0) && (-1 < *(int *)(lVar3 + 0x10))) {
            local_38 = 0;
            local_res8 = (int *)0x0;
            piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
            piVar4[1] = 0;
            *piVar4 = -1;
            local_res8 = piVar4 + 4;
            piVar4[2] = 0;
            *(undefined1 *)local_res8 = 0;
            if (*piVar4 != -1) {
              FUN_142e52dd0(0x8b);
            }
            if (piVar4[1] < 0) {
              FUN_142e54290(0x90,piVar4[1],0);
            }
            *piVar4 = 1;
            *(undefined1 *)local_res8 = 0;
            if (piVar4[1] + 1 < 1) {
              FUN_142e54290(0x9c,0);
            }
            piVar4[2] = 0;
            uVar6 = *(undefined8 *)(lVar3 + 8);
            iVar1 = (**(code **)(*param_1 + 0x98))(param_1);
            iVar2 = (**(code **)(*param_1 + 0x90))(param_1);
            FUN_142694130(param_1 + 0xa6,param_2 + iVar2,iVar1 + param_3 + 0x14,uVar6,0,0,0,0,0,0,0,
                          &local_res8,local_40,0);
            return 1;
          }
        }
        FUN_142645170(param_1 + 0xa6);
        return 0;
      }
      uVar6 = FUN_142cb9610(DAT_143aa84a0);
      iVar1 = (**(code **)(*param_1 + 0x98))(param_1);
      iVar2 = (**(code **)(*param_1 + 0x90))(param_1);
      goto LAB_142148602;
    }
    lVar3 = param_1[0xa5];
    if (lVar3 == 0) {
      uVar7 = 0;
LAB_142148599:
      FUN_142e54290(0xbc,1,uVar7);
      lVar3 = param_1[0xa5];
    }
    else {
      uVar7 = *(uint *)(lVar3 + -8);
      if (uVar7 < 2) goto LAB_142148599;
    }
    uVar6 = *(undefined8 *)(lVar3 + 8);
  }
  iVar1 = (**(code **)(*param_1 + 0x98))(param_1);
  iVar2 = (**(code **)(*param_1 + 0x90))(param_1);
LAB_142148602:
  FUN_1426455d0(param_1 + 0xa6,param_2 + 0x14 + iVar2,iVar1 + param_3 + 0x14,uVar6,
                CONCAT44(uVar8,0xcc0e395a),CONCAT44(uVar9,4),1);
  return 1;
}



//===========================================================
// FUN_142148660 @ 142148660   (31 bytes)
//===========================================================

void FUN_142148660(longlong param_1)

{
  FUN_142bf2640();
  FUN_142645170(param_1 + 0x530);
  return;
}



//===========================================================
// FUN_142148690 @ 142148690   (938 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142148926) */

void FUN_142148690(longlong *param_1,undefined4 param_2)

{
  bool bVar1;
  int *piVar2;
  char cVar3;
  longlong lVar4;
  int *piVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong *plVar8;
  int *piVar9;
  ulonglong uVar10;
  int iVar11;
  int iVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  int local_res8 [2];
  int *local_res18;
  int *local_res20;
  int *local_48 [2];
  
  uVar14 = 0;
  local_res8[0] = 0;
  cVar3 = FUN_142aa1a20(param_1 + 0x2ce,L"trade",param_2);
  if (cVar3 != '\0') {
    FUN_14214c0a0(param_1);
    return;
  }
  cVar3 = FUN_142aa1a20(param_1 + 0x2ce,L"coin",param_2);
  if (cVar3 != '\0') {
    FUN_14214bc10(param_1);
    return;
  }
  cVar3 = FUN_142aa1a20(param_1 + 0x2ce,L"claim",param_2);
  if (cVar3 != '\0') {
    FUN_141c417f0(param_1);
    return;
  }
  cVar3 = FUN_142aa1a20(param_1 + 0x2ce,L"enter",param_2);
  if (cVar3 == '\0') {
    cVar3 = FUN_142aa1a20(param_1 + 0x2ce,L"reset",param_2);
    if (cVar3 != '\0') {
      (**(code **)(*param_1 + 0x138))(param_1,2);
      return;
    }
    FUN_14177fb00(param_1,param_2);
    return;
  }
  uVar13 = uVar14;
  if (param_1[0x9c] == 0) {
LAB_1421487bc:
    bVar1 = true;
  }
  else {
    lVar4 = FUN_141060460(param_1 + 0x9b);
    if (*(longlong *)(lVar4 + 0xc0) != 0) {
      uVar13 = (ulonglong)*(uint *)(*(longlong *)(lVar4 + 0xc0) + -8);
    }
    local_res20 = (int *)0x0;
    lVar7 = 0xc0;
    if ((int)uVar13 < 1) {
      lVar7 = 200;
    }
    FUN_14019a260(&local_res20,lVar7 + lVar4);
    uVar13 = 3;
    if ((local_res20 == (int *)0x0) || ((char)*local_res20 == '\0')) goto LAB_1421487bc;
    bVar1 = false;
  }
  if (((uVar13 & 1) != 0) && (local_res20 != (int *)0x0)) {
    FUN_14019f2c0(local_res20 + -4);
  }
  if (bVar1) {
    return;
  }
  lVar4 = FUN_141060460(param_1 + 0x9b);
  plVar8 = (longlong *)(lVar4 + 0xc0);
  if ((*plVar8 == 0) || (*(int *)(*plVar8 + -8) < 1)) {
    plVar8 = (longlong *)(lVar4 + 200);
  }
  local_48[0] = (int *)0x0;
  FUN_14019a260(local_48,plVar8);
  piVar9 = local_48[0];
  local_res8[0] = 1;
  local_res18 = (int *)0x0;
  piVar2 = local_res18;
  if ((local_48[0] == (int *)0x0) || (piVar5 = local_48[0] + -4, piVar5 == (int *)0x0))
  goto LAB_14214898c;
  if (*piVar5 != -1) {
    if (*piVar5 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar5 = *piVar5 + 1;
    UNLOCK();
    if (local_res18 != (int *)0x0) {
      FUN_14019f2c0(local_res18 + -4);
    }
    local_res18 = piVar9;
    piVar9 = local_48[0];
    piVar2 = local_res18;
    goto LAB_14214898c;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  uVar10 = 0xffffffffffffffff;
  uVar13 = 0xffffffffffffffff;
  do {
    uVar13 = uVar13 + 1;
  } while (*(char *)((longlong)piVar9 + uVar13) != '\0');
  iVar11 = (int)uVar13;
  iVar12 = 0;
  if (0 < iVar11) {
    iVar12 = iVar11;
  }
  piVar5 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar12 + 0x11));
  piVar5[1] = iVar12;
  *piVar5 = -1;
  piVar2 = piVar5 + 4;
  piVar5[2] = 0;
  *(char *)piVar2 = '\0';
  local_res20 = piVar2;
  FUN_142ef7ba0(piVar2,piVar9,(longlong)iVar11);
  if (*piVar5 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar11 == -1) || (iVar11 <= piVar5[1])) {
    *piVar5 = 1;
    if (iVar11 != -1) goto LAB_1421488dc;
    uVar13 = uVar14;
    if (piVar2 != (int *)0x0) {
      do {
        uVar10 = uVar10 + 1;
      } while (*(char *)((longlong)piVar2 + uVar10) != '\0');
      uVar13 = uVar10 & 0xffffffff;
    }
  }
  else {
    FUN_142e54290(0x90,piVar5[1],uVar13 & 0xffffffff);
    *piVar5 = 1;
LAB_1421488dc:
    *(char *)((longlong)piVar2 + (longlong)iVar11) = '\0';
  }
  iVar12 = (int)uVar13;
  if ((iVar12 < 0) || (piVar5[1] + 1 <= iVar12)) {
    FUN_142e54290(0x9c,uVar13 & 0xffffffff);
  }
  piVar5[2] = iVar12;
  if (local_res18 != (int *)0x0) {
    FUN_14019f2c0(local_res18 + -4);
  }
LAB_14214898c:
  local_res18 = piVar2;
  FUN_141c3fcb0(param_1,&local_res18,local_res8);
  if (local_res8[0] != 0) {
    uVar6 = FUN_141060460(param_1 + 0x9b);
    FUN_1416a7070(uVar6,0);
  }
  uVar13 = param_1[0x9c] + 8;
  if (param_1[0x9c] == 0) {
    uVar13 = uVar14;
  }
  FUN_142c0bf50(DAT_143abfdf8,uVar13,0);
  if (piVar9 != (int *)0x0) {
    FUN_14019f2c0(piVar9 + -4);
  }
  return;
}



//===========================================================
// FUN_142148a40 @ 142148a40   (7809 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142148a40(longlong param_1)

{
  undefined2 *puVar1;
  code *pcVar2;
  longlong lVar3;
  undefined4 uVar4;
  int iVar5;
  int iVar6;
  longlong lVar7;
  undefined4 *puVar8;
  IUnknown *pIVar9;
  int *piVar10;
  undefined8 *puVar11;
  IUnknown *pIVar12;
  longlong *plVar13;
  IUnknown *pIVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  longlong lVar17;
  ulonglong uVar18;
  longlong lVar19;
  longlong lVar20;
  short *psVar21;
  undefined1 *puVar22;
  undefined1 *puVar23;
  ulonglong uVar24;
  uint uVar25;
  undefined *puStack_1b0;
  undefined1 auStack_1a8 [32];
  IUnknown *pIStack_188;
  longlong alStack_180 [5];
  IUnknown *apIStack_158 [2];
  short sStack_148;
  undefined2 uStack_146;
  undefined4 uStack_144;
  undefined8 uStack_140;
  undefined8 uStack_138;
  short sStack_128;
  undefined2 uStack_126;
  undefined4 uStack_124;
  longlong lStack_120;
  undefined8 uStack_118;
  IUnknown *pIStack_110;
  short sStack_108;
  undefined6 uStack_106;
  longlong lStack_100;
  undefined8 uStack_f8;
  IUnknown *pIStack_f0;
  IUnknown *pIStack_e8;
  longlong lStack_e0;
  undefined4 *puStack_d8;
  IUnknown *pIStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  int iStack_a8;
  int iStack_a4;
  int iStack_a0;
  undefined4 uStack_9c;
  int iStack_98;
  undefined4 uStack_94;
  int iStack_90;
  undefined4 uStack_8c;
  int iStack_88;
  undefined4 uStack_84;
  IUnknown *pIStack_80;
  undefined8 uStack_78;
  longlong lStack_70;
  undefined8 uStack_68;
  undefined8 uStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  IUnknown *pIStack_38;
  ulonglong uStack_30;
  
  puVar22 = auStack_1a8;
  uStack_30 = DAT_143a8b908 ^ (ulonglong)apIStack_158;
  puStack_1b0 = (undefined *)0x142148a84;
  FUN_141c3d020();
  puStack_1b0 = (undefined *)0x142148a93;
  FUN_142bf6230(param_1,&pIStack_d0);
  puVar23 = auStack_1a8;
  if (pIStack_d0 == (IUnknown *)0x0) goto LAB_14214a779;
  puStack_1b0 = (undefined *)0x142148ab7;
  FUN_14090ead0(&pIStack_80,DAT_143ad6eb8);
  if (pIStack_80 != (IUnknown *)0x0) {
    apIStack_158[0] = pIStack_80;
    puStack_1b0 = (undefined *)0x142148ad2;
    (**(code **)(*(longlong *)pIStack_80 + 8))();
    puStack_1b0 = (undefined *)0x142148ae6;
    uVar4 = FUN_140910ca0((IUnknown *)apIStack_158,"nameTagWidth",0);
    pIStack_e8 = (IUnknown *)CONCAT44(pIStack_e8._4_4_,uVar4);
    apIStack_158[0] = (IUnknown *)0x0;
    puStack_1b0 = (undefined *)0x142148b0f;
    FUN_141adbce0(param_1 + 0x1670,&iStack_90,L"otherName",(IUnknown *)apIStack_158);
    apIStack_158[0] = (IUnknown *)0x0;
    puStack_1b0 = (undefined *)0x142148b31;
    FUN_141adbce0(param_1 + 0x1670,&iStack_a0,L"myName",(IUnknown *)apIStack_158);
    apIStack_158[0] = (IUnknown *)0x0;
    puStack_1b0 = (undefined *)0x142148b53;
    FUN_141adbce0(param_1 + 0x1670,&iStack_88,L"otherMeso",(IUnknown *)apIStack_158);
    apIStack_158[0] = (IUnknown *)0x0;
    puStack_1b0 = (undefined *)0x142148b75;
    FUN_141adbce0(param_1 + 0x1670,&iStack_98,L"myMeso",(IUnknown *)apIStack_158);
    apIStack_158[0] = (IUnknown *)0x0;
    puStack_1b0 = (undefined *)0x142148b97;
    FUN_141adbce0(param_1 + 0x1670,&iStack_a8,L"warning_lt",(IUnknown *)apIStack_158);
    lStack_e0 = 0;
    puStack_1b0 = (undefined *)0x142148ba9;
    lVar7 = FUN_142cb9610(DAT_143aa84a0);
    puStack_d8 = (undefined4 *)0x0;
    if (lVar7 != 0) {
      uVar24 = 0xffffffffffffffff;
      do {
        uVar24 = uVar24 + 1;
      } while (*(char *)(lVar7 + uVar24) != '\0');
      iVar6 = (int)uVar24;
      iVar5 = 0;
      if (0 < iVar6) {
        iVar5 = iVar6;
      }
      puStack_1b0 = (undefined *)0x142148bed;
      puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
      puVar8[1] = iVar5;
      *puVar8 = 0xffffffff;
      puStack_d8 = puVar8 + 4;
      puVar8[2] = 0;
      *(undefined1 *)puStack_d8 = 0;
      puStack_1b0 = (undefined *)0x142148c24;
      FUN_142ef7ba0(puStack_d8,lVar7,(longlong)iVar6);
      puVar8 = puStack_d8;
      if (puStack_d8[-4] != -1) {
        puStack_1b0 = (undefined *)0x142148c3d;
        FUN_142e52dd0(0x8b);
      }
      if ((iVar6 == -1) || (iVar6 <= (int)puVar8[-3])) {
        puVar8[-4] = 1;
        if (iVar6 != -1) goto LAB_142148c62;
        if (puVar8 == (undefined4 *)0x0) {
          uVar24 = 0;
        }
        else {
          uVar24 = 0xffffffffffffffff;
          do {
            uVar24 = uVar24 + 1;
          } while (*(char *)((longlong)puVar8 + uVar24) != '\0');
        }
      }
      else {
        puStack_1b0 = (undefined *)0x142148c5e;
        FUN_142e54290(0x90,puVar8[-3],uVar24 & 0xffffffff);
        puVar8[-4] = 1;
LAB_142148c62:
        *(undefined1 *)((longlong)iVar6 + (longlong)puStack_d8) = 0;
      }
      iVar5 = (int)uVar24;
      if ((iVar5 < 0) || (puVar8[-3] + 1 <= iVar5)) {
        puStack_1b0 = (undefined *)0x142148c89;
        FUN_142e54290(0x9c,uVar24 & 0xffffffff);
      }
      puVar8[-2] = iVar5;
      uVar4 = (int)pIStack_e8;
    }
    pIVar14 = (IUnknown *)0x0;
    apIStack_158[0] = *(IUnknown **)(param_1 + 0x4e8);
    if (apIStack_158[0] != (IUnknown *)0x0) {
      puStack_1b0 = (undefined *)0x142148ca7;
      (**(code **)(*(longlong *)apIStack_158[0] + 8))();
    }
    puStack_1b0 = (undefined *)0x142148cbb;
    FUN_1429eb300(&puStack_d8,(IUnknown *)apIStack_158,uVar4);
    pIVar12 = *(IUnknown **)(param_1 + 0x4e8);
    if (pIVar12 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      puStack_1b0 = &UNK_14214a81f;
      FUN_142ef3ac0(0x80004003);
    }
    puStack_1b0 = (undefined *)0x142148cd5;
    (*DAT_143262a20)(&sStack_128);
    if (DAT_143a8b8d8 == 8) {
      if (sStack_128 == 8) {
        sStack_128 = 0;
        if (lStack_120 != 0) {
          puStack_1b0 = (undefined *)0x142148d04;
          (*DAT_143ad5990)(lStack_120 + -4);
        }
      }
      else {
        puStack_1b0 = (undefined *)0x142148d40;
        iVar5 = (*DAT_143262a18)(&sStack_128);
        if (iVar5 < 0) goto LAB_14214a7c5;
      }
      sStack_128 = 8;
      pIVar9 = pIVar14;
      if (DAT_143a8b8e0 != 0) {
        pIVar9 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      puStack_1b0 = (undefined *)0x142148d66;
      lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar9);
    }
    else {
      if ((sStack_128 == 8) && (sStack_128 = 0, lStack_120 != 0)) {
        puStack_1b0 = (undefined *)0x142148e0c;
        (*DAT_143ad5990)(lStack_120 + -4);
      }
      puStack_1b0 = (undefined *)0x142148e1d;
      iVar5 = (*DAT_143262a28)(&sStack_128,&DAT_143a8b8d8);
      if (iVar5 < 0) {
LAB_14214a7c5:
                    /* WARNING: Subroutine does not return */
        puStack_1b0 = &UNK_14214a7cc;
        FUN_142ef3ac0(iVar5);
      }
    }
    puVar8 = puStack_d8;
    puStack_1b0 = (undefined *)0x142148d82;
    pIVar9 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
    pIStack_110 = pIVar9;
    if (pIVar9 != (IUnknown *)0x0) {
      *(undefined8 *)(pIVar9 + 8) = 0;
      *(int *)(pIVar9 + 0x10) = 1;
      if (puVar8 != (undefined4 *)0x0) {
        alStack_180[0]._0_4_ = 0;
        pIStack_188 = (IUnknown *)0x0;
        puStack_1b0 = (undefined *)0x142148dc2;
        iVar5 = (*DAT_1432627f8)(0xfde9,0,puVar8,0xffffffff);
        iVar5 = (int)((ulonglong)(longlong)(iVar5 * 2) >> 1);
        uVar25 = iVar5 - 1;
        puStack_1b0 = (undefined *)0x142148de1;
        piVar10 = (int *)(*DAT_143ad5980)();
        if (piVar10 == (int *)0x0) {
          pIVar14 = (IUnknown *)0x0;
        }
        else {
          *piVar10 = uVar25 * 2;
          pIVar14 = (IUnknown *)(piVar10 + 1);
          *(undefined2 *)(pIVar14 + (ulonglong)uVar25 * 2) = 0;
        }
        puStack_1b0 = (undefined *)0x142148e5a;
        pIStack_188 = pIVar14;
        alStack_180[0]._0_4_ = iVar5;
        (*DAT_1432627f8)(0xfde9,0,puVar8,0xffffffff);
      }
      *(IUnknown **)pIVar9 = pIVar14;
      pIVar14 = pIVar9;
    }
    apIStack_158[0] = pIVar14;
    if (pIVar14 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      puStack_1b0 = &UNK_14214a82a;
      FUN_142ef3ac0(0x8007000e);
    }
    pIStack_110 = (IUnknown *)((ulonglong)pIStack_110 & 0xffffffff00000000);
    sStack_148 = sStack_128;
    uStack_146 = uStack_126;
    uStack_144 = uStack_124;
    uStack_140 = lStack_120;
    uStack_138 = uStack_118;
    puStack_1b0 = (undefined *)0x142148eab;
    pIStack_f0 = (IUnknown *)apIStack_158;
    iVar5 = (**(code **)(*(longlong *)pIVar12 + 0xb0))
                      (pIVar12,*(undefined8 *)pIVar14,&sStack_148,&pIStack_110);
    if (iVar5 < 0) {
      puStack_1b0 = (undefined *)0x142148ec0;
      _com_issue_errorex(iVar5,pIVar12,(_GUID *)&DAT_143297250);
    }
    iVar5 = (int)pIStack_110;
    puStack_1b0 = (undefined *)0x142148ecd;
    FUN_1401be120((IUnknown *)apIStack_158);
    if (sStack_128 == 8) {
      sStack_128 = 0;
      if (lStack_120 != 0) {
        puStack_1b0 = (undefined *)0x142148eec;
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      puStack_1b0 = (undefined *)0x142148ef8;
      (*DAT_143262a18)(&sStack_128);
    }
    pIVar14 = pIStack_d0;
    if (pIStack_d0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      puStack_1b0 = &UNK_14214a7c4;
      FUN_142ef3ac0(0x80004003);
    }
    puStack_1b0 = (undefined *)0x142148f13;
    (*DAT_143262a20)(&sStack_148);
    if (DAT_143a8b8d8 == 8) {
      if (sStack_148 == 8) {
        sStack_148 = 0;
        if (uStack_140 != 0) {
          puStack_1b0 = (undefined *)0x142148f3b;
          (*DAT_143ad5990)(uStack_140 + -4);
        }
      }
      else {
        puStack_1b0 = (undefined *)0x142148f47;
        iVar6 = (*DAT_143262a18)(&sStack_148);
        if (iVar6 < 0) goto LAB_14214a82b;
      }
      sStack_148 = 8;
      if (DAT_143a8b8e0 == 0) {
        puStack_1b0 = (undefined *)0x142148f6b;
        uStack_140 = FUN_1401a5fa0(0,0);
      }
      else {
        puStack_1b0 = (undefined *)0x142148f7b;
        uStack_140 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
    }
    else {
      if ((sStack_148 == 8) && (sStack_148 = 0, uStack_140 != 0)) {
        puStack_1b0 = (undefined *)0x142148f9f;
        (*DAT_143ad5990)(uStack_140 + -4);
      }
      puStack_1b0 = (undefined *)0x142148fb0;
      iVar6 = (*DAT_143262a28)(&sStack_148,&DAT_143a8b8d8);
      if (iVar6 < 0) {
LAB_14214a82b:
                    /* WARNING: Subroutine does not return */
        puStack_1b0 = &UNK_14214a832;
        FUN_142ef3ac0(iVar6);
      }
    }
    puStack_1b0 = (undefined *)0x142148fc7;
    (*DAT_143262a20)(&sStack_108);
    if (DAT_143a8b8d8 == 8) {
      if (sStack_108 == 8) {
        sStack_108 = 0;
        if (lStack_100 != 0) {
          puStack_1b0 = (undefined *)0x142148ff3;
          (*DAT_143ad5990)(lStack_100 + -4);
        }
      }
      else {
        puStack_1b0 = (undefined *)0x142148fff;
        iVar6 = (*DAT_143262a18)(&sStack_108);
        if (iVar6 < 0) goto LAB_14214a7cd;
      }
      sStack_108 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar25 = 0;
      }
      else {
        uVar25 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      puStack_1b0 = (undefined *)0x142149025;
      lStack_100 = FUN_1401a5fa0(DAT_143a8b8e0,uVar25);
    }
    else {
      if ((sStack_108 == 8) && (sStack_108 = 0, lStack_100 != 0)) {
        puStack_1b0 = (undefined *)0x14214907f;
        (*DAT_143ad5990)(lStack_100 + -4);
      }
      puStack_1b0 = (undefined *)0x142149090;
      iVar6 = (*DAT_143262a28)(&sStack_108,&DAT_143a8b8d8);
      if (iVar6 < 0) {
LAB_14214a7cd:
                    /* WARNING: Subroutine does not return */
        puStack_1b0 = &UNK_14214a7d4;
        FUN_142ef3ac0(iVar6);
      }
    }
    puStack_1b0 = (undefined *)0x142149033;
    (*DAT_143262a20)(&sStack_128);
    if (DAT_143a8b8d8 == 8) {
      if (sStack_128 == 8) {
        sStack_128 = 0;
        if (lStack_120 != 0) {
          puStack_1b0 = (undefined *)0x14214905f;
          (*DAT_143ad5990)(lStack_120 + -4);
        }
      }
      else {
        puStack_1b0 = (undefined *)0x1421490a4;
        iVar6 = (*DAT_143262a18)(&sStack_128);
        if (iVar6 < 0) goto LAB_14214a7d5;
      }
      sStack_128 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar25 = 0;
      }
      else {
        uVar25 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      puStack_1b0 = (undefined *)0x1421490ca;
      lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar25);
    }
    else {
      if ((sStack_128 == 8) && (sStack_128 = 0, lStack_120 != 0)) {
        puStack_1b0 = (undefined *)0x14214910a;
        (*DAT_143ad5990)(lStack_120 + -4);
      }
      puStack_1b0 = (undefined *)0x14214911b;
      iVar6 = (*DAT_143262a28)(&sStack_128,&DAT_143a8b8d8);
      if (iVar6 < 0) {
LAB_14214a7d5:
                    /* WARNING: Subroutine does not return */
        puStack_1b0 = &UNK_14214a7dc;
        FUN_142ef3ac0(iVar6);
      }
    }
    uVar16 = *(undefined8 *)(param_1 + 0x4e8);
    if (puStack_d8 == (undefined4 *)0x0) {
      iVar6 = 2;
    }
    else {
      alStack_180[0]._0_4_ = 0;
      pIStack_188 = (IUnknown *)0x0;
      puStack_1b0 = (undefined *)0x142149141;
      iVar6 = (*DAT_1432627f8)(0xfde9,0,puStack_d8,0xffffffff);
      iVar6 = iVar6 * 2;
    }
    puVar8 = puStack_d8;
    uVar24 = (longlong)iVar6 + 0xf;
    if (uVar24 <= (ulonglong)(longlong)iVar6) {
      uVar24 = 0xffffffffffffff0;
    }
    puStack_1b0 = (undefined *)0x14214916e;
    lVar7 = -(uVar24 & 0xfffffffffffffff0);
    puVar1 = (undefined2 *)((longlong)apIStack_158 + lVar7);
    if (puStack_d8 == (undefined4 *)0x0) {
      if (puVar1 != (undefined2 *)0x0) {
        *puVar1 = 0;
      }
    }
    else {
      *(undefined4 *)((longlong)alStack_180 + lVar7) = 0x100000;
      *(undefined2 **)((longlong)alStack_180 + lVar7 + -8) = puVar1;
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421491a5;
      (*DAT_1432627f8)(0xfde9,0,puVar8,0xffffffff);
    }
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421491b1;
    puVar11 = (undefined8 *)FUN_1401a5890((IUnknown *)apIStack_158,puVar1);
    uVar25 = 0;
    pIVar9 = (IUnknown *)0x0;
    pIStack_110 = (IUnknown *)((ulonglong)pIStack_110 & 0xffffffff00000000);
    pcVar2 = *(code **)(*(longlong *)pIVar14 + 0x1a8);
    pIVar12 = pIVar9;
    if ((undefined8 *)*puVar11 != (undefined8 *)0x0) {
      pIVar12 = *(IUnknown **)*puVar11;
    }
    uStack_78 = CONCAT44(uStack_144,CONCAT22(uStack_146,sStack_148));
    lStack_70 = uStack_140;
    uStack_68 = uStack_138;
    uStack_58 = CONCAT62(uStack_106,sStack_108);
    lStack_50 = lStack_100;
    uStack_48 = uStack_f8;
    uStack_c8 = CONCAT44(uStack_124,CONCAT22(uStack_126,sStack_128));
    uStack_c0 = lStack_120;
    uStack_b8 = uStack_118;
    pIStack_f0 = (IUnknown *)puVar11;
    *(IUnknown ***)((longlong)alStack_180 + lVar7 + 0x18) = &pIStack_110;
    *(undefined8 **)((longlong)alStack_180 + lVar7 + 0x10) = &uStack_78;
    *(undefined8 **)((longlong)alStack_180 + lVar7 + 8) = &uStack_58;
    *(undefined8 **)((longlong)alStack_180 + lVar7) = &uStack_c8;
    *(undefined8 *)((longlong)alStack_180 + lVar7 + -8) = uVar16;
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214926f;
    iVar5 = (*pcVar2)(pIVar14,iStack_a0 - iVar5 / 2,uStack_9c,pIVar12);
    if (iVar5 < 0) {
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149284;
      _com_issue_errorex(iVar5,pIVar14,(_GUID *)&DAT_14327ac98);
    }
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214928d;
    FUN_1401be120(puVar11);
    if (sStack_128 == 8) {
      sStack_128 = 0;
      if (lStack_120 != 0) {
        lVar17 = lStack_120 + -4;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421492ad;
        (*DAT_143ad5990)(lVar17);
      }
    }
    else {
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421492b9;
      (*DAT_143262a18)(&sStack_128);
    }
    if (sStack_108 == 8) {
      sStack_108 = 0;
      if (lStack_100 != 0) {
        lVar17 = lStack_100 + -4;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421492d9;
        (*DAT_143ad5990)(lVar17);
      }
    }
    else {
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421492e5;
      (*DAT_143262a18)(&sStack_108);
    }
    if (sStack_148 == 8) {
      sStack_148 = 0;
      if (uStack_140 != 0) {
        lVar17 = uStack_140 + -4;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149305;
        (*DAT_143ad5990)(lVar17);
      }
    }
    else {
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149311;
      (*DAT_143262a18)(&sStack_148);
    }
    uVar16 = *(undefined8 *)(param_1 + 0x518);
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149325;
    FUN_1429e9d30(&lStack_e0,uVar16,1);
    pIVar14 = *(IUnknown **)(param_1 + 0x4e8);
    if (pIVar14 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      *(undefined **)(auStack_1a8 + lVar7 + -8) = &UNK_14214a8e0;
      FUN_142ef3ac0(0x80004003);
    }
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214933f;
    (*DAT_143262a20)(&sStack_148);
    if (DAT_143a8b8d8 == 8) {
      if (sStack_148 == 8) {
        sStack_148 = 0;
        if (uStack_140 != 0) {
          lVar17 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214936c;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149378;
        iVar5 = (*DAT_143262a18)(&sStack_148);
        if (iVar5 < 0) goto LAB_14214a7dd;
      }
      sStack_148 = 8;
      if (DAT_143a8b8e0 != 0) {
        uVar25 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421493a4;
      uStack_140 = FUN_1401a5fa0(DAT_143a8b8e0,uVar25);
    }
    else {
      if ((sStack_148 == 8) && (sStack_148 = 0, uStack_140 != 0)) {
        lVar17 = uStack_140 + -4;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149407;
        (*DAT_143ad5990)(lVar17);
      }
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149418;
      iVar5 = (*DAT_143262a28)(&sStack_148,&DAT_143a8b8d8);
      if (iVar5 < 0) {
LAB_14214a7dd:
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar7 + -8) = &UNK_14214a7e4;
        FUN_142ef3ac0(iVar5);
      }
    }
    lVar17 = lStack_e0;
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421493bd;
    pIVar12 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
    pIStack_f0 = pIVar12;
    if (pIVar12 != (IUnknown *)0x0) {
      *(undefined8 *)(pIVar12 + 8) = 0;
      *(undefined4 *)(pIVar12 + 0x10) = 1;
      pIVar9 = pIVar12;
      if (lVar17 == 0) {
        *(undefined8 *)pIVar12 = 0;
      }
      else {
        *(undefined4 *)((longlong)alStack_180 + lVar7) = 0;
        *(undefined8 *)((longlong)alStack_180 + lVar7 + -8) = 0;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149442;
        iVar5 = (*DAT_1432627f8)(0xfde9,0,lVar17,0xffffffff);
        iVar5 = (int)((ulonglong)(longlong)(iVar5 * 2) >> 1);
        uVar25 = iVar5 - 1;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149462;
        piVar10 = (int *)(*DAT_143ad5980)();
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          *piVar10 = uVar25 * 2;
          piVar10 = piVar10 + 1;
          *(undefined2 *)((longlong)piVar10 + (ulonglong)uVar25 * 2) = 0;
        }
        *(int *)((longlong)alStack_180 + lVar7) = iVar5;
        *(int **)((longlong)alStack_180 + lVar7 + -8) = piVar10;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421494a1;
        (*DAT_1432627f8)(0xfde9,0,lVar17,0xffffffff);
        *(int **)pIVar12 = piVar10;
      }
    }
    apIStack_158[0] = pIVar9;
    if (pIVar9 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      *(undefined **)(auStack_1a8 + lVar7 + -8) = &UNK_14214a83d;
      FUN_142ef3ac0(0x8007000e);
    }
    pIStack_110 = (IUnknown *)((ulonglong)pIStack_110 & 0xffffffff00000000);
    uStack_c8 = CONCAT44(uStack_144,CONCAT22(uStack_146,sStack_148));
    uStack_c0 = uStack_140;
    uStack_b8 = uStack_138;
    uVar16 = *(undefined8 *)pIVar9;
    pcVar2 = *(code **)(*(longlong *)pIVar14 + 0xb0);
    pIStack_f0 = (IUnknown *)apIStack_158;
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421494fe;
    iVar5 = (*pcVar2)(pIVar14,uVar16,&uStack_c8,&pIStack_110);
    if (iVar5 < 0) {
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149513;
      _com_issue_errorex(iVar5,pIVar14,(_GUID *)&DAT_143297250);
    }
    iVar5 = (int)pIStack_110;
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149520;
    FUN_1401be120((IUnknown *)apIStack_158);
    if (sStack_148 == 8) {
      sStack_148 = 0;
      if (uStack_140 != 0) {
        lVar17 = uStack_140 + -4;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149543;
        (*DAT_143ad5990)(lVar17);
      }
    }
    else {
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214954f;
      (*DAT_143262a18)(&sStack_148);
    }
    pIVar14 = pIStack_d0;
    uVar24 = 0;
    if (pIStack_d0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      *(undefined **)(auStack_1a8 + lVar7 + -8) = &UNK_14214a8d5;
      FUN_142ef3ac0(0x80004003);
    }
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214956c;
    (*DAT_143262a20)(&sStack_128);
    if (DAT_143a8b8d8 == 8) {
      if (sStack_128 == 8) {
        sStack_128 = 0;
        if (lStack_120 != 0) {
          lVar17 = lStack_120 + -4;
          *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149595;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421495a1;
        iVar6 = (*DAT_143262a18)(&sStack_128);
        if (iVar6 < 0) goto LAB_14214a83e;
      }
      sStack_128 = 8;
      if (DAT_143a8b8e0 == 0) {
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421495c6;
        lStack_120 = FUN_1401a5fa0(0,0);
      }
      else {
        uVar25 = *(uint *)(DAT_143a8b8e0 + -4);
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421495d6;
        lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar25 >> 1);
      }
    }
    else {
      if ((sStack_128 == 8) && (sStack_128 = 0, lStack_120 != 0)) {
        lVar17 = lStack_120 + -4;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421495fb;
        (*DAT_143ad5990)(lVar17);
      }
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214960c;
      iVar6 = (*DAT_143262a28)(&sStack_128,&DAT_143a8b8d8);
      if (iVar6 < 0) {
LAB_14214a83e:
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar7 + -8) = &UNK_14214a845;
        FUN_142ef3ac0(iVar6);
      }
    }
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149623;
    (*DAT_143262a20)(&sStack_108);
    if (DAT_143a8b8d8 == 8) {
      if (sStack_108 == 8) {
        sStack_108 = 0;
        if (lStack_100 != 0) {
          lVar17 = lStack_100 + -4;
          *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149650;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214965c;
        iVar6 = (*DAT_143262a18)(&sStack_108);
        if (iVar6 < 0) goto LAB_14214a7e5;
      }
      sStack_108 = 8;
      uVar18 = uVar24;
      if (DAT_143a8b8e0 != 0) {
        uVar18 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149683;
      lStack_100 = FUN_1401a5fa0(DAT_143a8b8e0,uVar18);
    }
    else {
      if ((sStack_108 == 8) && (sStack_108 = 0, lStack_100 != 0)) {
        lVar17 = lStack_100 + -4;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421496df;
        (*DAT_143ad5990)(lVar17);
      }
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421496f0;
      iVar6 = (*DAT_143262a28)(&sStack_108,&DAT_143a8b8d8);
      if (iVar6 < 0) {
LAB_14214a7e5:
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar7 + -8) = &UNK_14214a7ec;
        FUN_142ef3ac0(iVar6);
      }
    }
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149691;
    (*DAT_143262a20)(&sStack_148);
    if (DAT_143a8b8d8 == 8) {
      if (sStack_148 == 8) {
        sStack_148 = 0;
        if (uStack_140 != 0) {
          lVar17 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421496be;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149704;
        iVar6 = (*DAT_143262a18)(&sStack_148);
        if (iVar6 < 0) goto LAB_14214a7ed;
      }
      sStack_148 = 8;
      uVar18 = uVar24;
      if (DAT_143a8b8e0 != 0) {
        uVar18 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x14214972b;
      uStack_140 = FUN_1401a5fa0(DAT_143a8b8e0,uVar18);
    }
    else {
      if ((sStack_148 == 8) && (sStack_148 = 0, uStack_140 != 0)) {
        lVar17 = uStack_140 + -4;
        *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149763;
        (*DAT_143ad5990)(lVar17);
      }
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149774;
      iVar6 = (*DAT_143262a28)(&sStack_148,&DAT_143a8b8d8);
      if (iVar6 < 0) {
LAB_14214a7ed:
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar7 + -8) = &UNK_14214a7f4;
        FUN_142ef3ac0(iVar6);
      }
    }
    lVar17 = lStack_e0;
    uVar16 = *(undefined8 *)(param_1 + 0x4f0);
    if (lStack_e0 == 0) {
      iVar6 = 2;
    }
    else {
      *(undefined4 *)((longlong)alStack_180 + lVar7) = 0;
      *(undefined8 *)((longlong)alStack_180 + lVar7 + -8) = 0;
      *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x142149799;
      iVar6 = (*DAT_1432627f8)(0xfde9,0,lVar17,0xffffffff);
      iVar6 = iVar6 * 2;
    }
    lVar17 = lStack_e0;
    uVar18 = (longlong)iVar6 + 0xf;
    if (uVar18 <= (ulonglong)(longlong)iVar6) {
      uVar18 = 0xffffffffffffff0;
    }
    *(undefined8 *)(auStack_1a8 + lVar7 + -8) = 0x1421497c0;
    lVar3 = -(uVar18 & 0xfffffffffffffff0);
    puVar1 = (undefined2 *)((longlong)apIStack_158 + lVar3 + lVar7);
    if (lVar17 == 0) {
      if (puVar1 != (undefined2 *)0x0) {
        *puVar1 = 0;
      }
    }
    else {
      *(undefined4 *)((longlong)alStack_180 + lVar3 + lVar7) = 0x100000;
      *(undefined2 **)((longlong)alStack_180 + lVar3 + lVar7 + -8) = puVar1;
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421497f7;
      (*DAT_1432627f8)(0xfde9,0,lVar17,0xffffffff);
    }
    *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149803;
    pIVar12 = (IUnknown *)FUN_1401a5890((IUnknown *)apIStack_158,puVar1);
    pIStack_110 = (IUnknown *)((ulonglong)pIStack_110 & 0xffffffff00000000);
    pcVar2 = *(code **)(*(longlong *)pIVar14 + 0x1a8);
    uVar18 = uVar24;
    if (*(IUnknown **)pIVar12 != (IUnknown *)0x0) {
      uVar18 = *(ulonglong *)*(IUnknown **)pIVar12;
    }
    uStack_c8 = CONCAT44(uStack_124,CONCAT22(uStack_126,sStack_128));
    uStack_c0 = lStack_120;
    uStack_b8 = uStack_118;
    uStack_58 = CONCAT62(uStack_106,sStack_108);
    lStack_50 = lStack_100;
    uStack_48 = uStack_f8;
    uStack_78 = CONCAT44(uStack_144,CONCAT22(uStack_146,sStack_148));
    lStack_70 = uStack_140;
    uStack_68 = uStack_138;
    pIStack_f0 = pIVar12;
    *(IUnknown ***)((longlong)alStack_180 + lVar3 + lVar7 + 0x18) = &pIStack_110;
    *(undefined8 **)((longlong)alStack_180 + lVar3 + lVar7 + 0x10) = &uStack_c8;
    *(undefined8 **)((longlong)alStack_180 + lVar3 + lVar7 + 8) = &uStack_58;
    *(undefined8 **)((longlong)alStack_180 + lVar3 + lVar7) = &uStack_78;
    *(undefined8 *)((longlong)alStack_180 + lVar3 + lVar7 + -8) = uVar16;
    *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421498b8;
    iVar5 = (*pcVar2)(pIVar14,iStack_98 - iVar5,uStack_94,uVar18);
    if (iVar5 < 0) {
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421498cd;
      _com_issue_errorex(iVar5,pIVar14,(_GUID *)&DAT_14327ac98);
    }
    *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421498d6;
    FUN_1401be120(pIVar12);
    if (sStack_148 == 8) {
      sStack_148 = 0;
      if (uStack_140 != 0) {
        lVar17 = uStack_140 + -4;
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421498f6;
        (*DAT_143ad5990)(lVar17);
      }
    }
    else {
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149902;
      (*DAT_143262a18)(&sStack_148);
    }
    if (sStack_108 == 8) {
      sStack_108 = 0;
      if (lStack_100 != 0) {
        lVar17 = lStack_100 + -4;
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149922;
        (*DAT_143ad5990)(lVar17);
      }
    }
    else {
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x14214992e;
      (*DAT_143262a18)(&sStack_108);
    }
    if (sStack_128 == 8) {
      sStack_128 = 0;
      if (lStack_120 != 0) {
        lVar17 = lStack_120 + -4;
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x14214994e;
        (*DAT_143ad5990)(lVar17);
      }
    }
    else {
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x14214995a;
      (*DAT_143262a18)(&sStack_128);
    }
    lVar17 = *(longlong *)(param_1 + 0x530);
    uVar18 = uVar24;
    if ((lVar17 == 0) || (uVar18 = (ulonglong)*(uint *)(lVar17 + -8), *(uint *)(lVar17 + -8) == 0))
    {
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149984;
      FUN_142e54290(0xbc,0,uVar18);
      lVar17 = *(longlong *)(param_1 + 0x530);
    }
    *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149995;
    puVar11 = (undefined8 *)FUN_1408a9e40((IUnknown *)apIStack_158,0xa4d);
    lVar19 = lStack_e0;
    uVar16 = *puVar11;
    *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421499ac;
    FUN_14019ba10(lVar17," %s%s",lVar19,uVar16);
    if (apIStack_158[0] != (IUnknown *)0x0) {
      pIVar14 = apIStack_158[0] + -0x10;
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421499bf;
      FUN_14019f2c0(pIVar14);
    }
    *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421499c8;
    iVar5 = FUN_141c3f8a0(param_1);
    puVar22 = auStack_1a8 + lVar3 + lVar7;
    if (1 < iVar5) {
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421499d9;
      iVar5 = FUN_141c3f8b0(param_1);
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x1421499ee;
      FUN_141c3f9f0(param_1,&pIStack_110,1 - iVar5);
      apIStack_158[0] = *(IUnknown **)(param_1 + 0x4e8);
      if (apIStack_158[0] != (IUnknown *)0x0) {
        pcVar2 = *(code **)(*(longlong *)apIStack_158[0] + 8);
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149a05;
        (*pcVar2)();
      }
      uVar18 = (ulonglong)pIStack_e8 & 0xffffffff;
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149a17;
      FUN_1429eb300(&pIStack_110,(IUnknown *)apIStack_158,uVar18);
      pIVar14 = *(IUnknown **)(param_1 + 0x4e8);
      if (pIVar14 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar3 + lVar7 + -8) = &UNK_14214a89c;
        FUN_142ef3ac0(0x80004003);
      }
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149a31;
      (*DAT_143262a20)(&sStack_148);
      if (DAT_143a8b8d8 == 8) {
        if (sStack_148 == 8) {
          sStack_148 = 0;
          if (uStack_140 != 0) {
            lVar17 = uStack_140 + -4;
            *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149a5e;
            (*DAT_143ad5990)(lVar17);
          }
        }
        else {
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149a6a;
          iVar5 = (*DAT_143262a18)(&sStack_148);
          if (iVar5 < 0) goto LAB_14214a7f5;
        }
        sStack_148 = 8;
        uVar18 = uVar24;
        if (DAT_143a8b8e0 != 0) {
          uVar18 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149a96;
        uStack_140 = FUN_1401a5fa0(DAT_143a8b8e0,uVar18);
      }
      else {
        if ((sStack_148 == 8) && (sStack_148 = 0, uStack_140 != 0)) {
          lVar17 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149ae8;
          (*DAT_143ad5990)(lVar17);
        }
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149af9;
        iVar5 = (*DAT_143262a28)(&sStack_148,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_14214a7f5:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_1a8 + lVar3 + lVar7 + -8) = &UNK_14214a7fc;
          FUN_142ef3ac0(iVar5);
        }
      }
      pIVar12 = pIStack_110;
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149aa7;
      plVar13 = (longlong *)FUN_1401a5780((IUnknown *)apIStack_158,pIVar12);
      pIStack_e8 = (IUnknown *)((ulonglong)pIStack_e8 & 0xffffffff00000000);
      pcVar2 = *(code **)(*(longlong *)pIVar14 + 0xb0);
      uVar18 = uVar24;
      if ((ulonglong *)*plVar13 != (ulonglong *)0x0) {
        uVar18 = *(ulonglong *)*plVar13;
      }
      uStack_c8 = CONCAT44(uStack_144,CONCAT22(uStack_146,sStack_148));
      uStack_c0 = uStack_140;
      uStack_b8 = uStack_138;
      pIStack_f0 = (IUnknown *)plVar13;
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149b2e;
      iVar5 = (*pcVar2)(pIVar14,uVar18,&uStack_c8,&pIStack_e8);
      if (iVar5 < 0) {
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149b43;
        _com_issue_errorex(iVar5,pIVar14,(_GUID *)&DAT_143297250);
      }
      iVar5 = (int)pIStack_e8;
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149b4f;
      FUN_1401be120(plVar13);
      if (sStack_148 == 8) {
        sStack_148 = 0;
        if (uStack_140 != 0) {
          lVar17 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149b6f;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149b7b;
        (*DAT_143262a18)(&sStack_148);
      }
      pIVar14 = pIStack_d0;
      if (pIStack_d0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar3 + lVar7 + -8) = &UNK_14214a891;
        FUN_142ef3ac0(0x80004003);
      }
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149b96;
      (*DAT_143262a20)(&sStack_128);
      if (DAT_143a8b8d8 == 8) {
        if (sStack_128 == 8) {
          sStack_128 = 0;
          if (lStack_120 != 0) {
            lVar17 = lStack_120 + -4;
            *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149bbf;
            (*DAT_143ad5990)(lVar17);
          }
        }
        else {
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149bcb;
          iVar6 = (*DAT_143262a18)(&sStack_128);
          if (iVar6 < 0) goto LAB_14214a846;
        }
        sStack_128 = 8;
        if (DAT_143a8b8e0 == 0) {
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149bf0;
          lStack_120 = FUN_1401a5fa0(0,0);
        }
        else {
          uVar25 = *(uint *)(DAT_143a8b8e0 + -4);
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149c00;
          lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar25 >> 1);
        }
      }
      else {
        if ((sStack_128 == 8) && (sStack_128 = 0, lStack_120 != 0)) {
          lVar17 = lStack_120 + -4;
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149c25;
          (*DAT_143ad5990)(lVar17);
        }
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149c36;
        iVar6 = (*DAT_143262a28)(&sStack_128,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14214a846:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_1a8 + lVar3 + lVar7 + -8) = &UNK_14214a84d;
          FUN_142ef3ac0(iVar6);
        }
      }
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149c4d;
      (*DAT_143262a20)(&sStack_108);
      if (DAT_143a8b8d8 == 8) {
        if (sStack_108 == 8) {
          sStack_108 = 0;
          if (lStack_100 != 0) {
            lVar17 = lStack_100 + -4;
            *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149c7a;
            (*DAT_143ad5990)(lVar17);
          }
        }
        else {
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149c86;
          iVar6 = (*DAT_143262a18)(&sStack_108);
          if (iVar6 < 0) goto LAB_14214a7fd;
        }
        sStack_108 = 8;
        uVar18 = uVar24;
        if (DAT_143a8b8e0 != 0) {
          uVar18 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149cad;
        lStack_100 = FUN_1401a5fa0(DAT_143a8b8e0,uVar18);
      }
      else {
        if ((sStack_108 == 8) && (sStack_108 = 0, lStack_100 != 0)) {
          lVar17 = lStack_100 + -4;
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149d09;
          (*DAT_143ad5990)(lVar17);
        }
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149d1a;
        iVar6 = (*DAT_143262a28)(&sStack_108,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14214a7fd:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_1a8 + lVar3 + lVar7 + -8) = &UNK_14214a804;
          FUN_142ef3ac0(iVar6);
        }
      }
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149cbb;
      (*DAT_143262a20)(&sStack_148);
      if (DAT_143a8b8d8 == 8) {
        if (sStack_148 == 8) {
          sStack_148 = 0;
          if (uStack_140 != 0) {
            lVar17 = uStack_140 + -4;
            *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149ce8;
            (*DAT_143ad5990)(lVar17);
          }
        }
        else {
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149d2e;
          iVar6 = (*DAT_143262a18)(&sStack_148);
          if (iVar6 < 0) goto LAB_14214a805;
        }
        sStack_148 = 8;
        uVar18 = uVar24;
        if (DAT_143a8b8e0 != 0) {
          uVar18 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149d55;
        uStack_140 = FUN_1401a5fa0(DAT_143a8b8e0,uVar18);
      }
      else {
        if ((sStack_148 == 8) && (sStack_148 = 0, uStack_140 != 0)) {
          lVar17 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149d8e;
          (*DAT_143ad5990)(lVar17);
        }
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149d9f;
        iVar6 = (*DAT_143262a28)(&sStack_148,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14214a805:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_1a8 + lVar3 + lVar7 + -8) = &UNK_14214a80c;
          FUN_142ef3ac0(iVar6);
        }
      }
      pIVar12 = pIStack_110;
      uVar16 = *(undefined8 *)(param_1 + 0x4e8);
      if (pIStack_110 == (IUnknown *)0x0) {
        iVar6 = 2;
      }
      else {
        *(undefined4 *)((longlong)alStack_180 + lVar3 + lVar7) = 0;
        *(undefined8 *)((longlong)alStack_180 + lVar3 + lVar7 + -8) = 0;
        *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149dc4;
        iVar6 = (*DAT_1432627f8)(0xfde9,0,pIVar12,0xffffffff);
        iVar6 = iVar6 * 2;
      }
      pIVar12 = pIStack_110;
      uVar18 = (longlong)iVar6 + 0xf;
      if (uVar18 <= (ulonglong)(longlong)iVar6) {
        uVar18 = 0xffffffffffffff0;
      }
      *(undefined8 *)(auStack_1a8 + lVar3 + lVar7 + -8) = 0x142149deb;
      lVar17 = -(uVar18 & 0xfffffffffffffff0);
      puVar1 = (undefined2 *)((longlong)apIStack_158 + lVar17 + lVar3 + lVar7);
      if (pIVar12 == (IUnknown *)0x0) {
        if (puVar1 != (undefined2 *)0x0) {
          *puVar1 = 0;
        }
      }
      else {
        *(undefined4 *)((longlong)alStack_180 + lVar17 + lVar3 + lVar7) = 0x100000;
        *(undefined2 **)((longlong)alStack_180 + lVar17 + lVar3 + lVar7 + -8) = puVar1;
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149e22;
        (*DAT_1432627f8)(0xfde9,0,pIVar12,0xffffffff);
      }
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149e2e;
      plVar13 = (longlong *)FUN_1401a5890((IUnknown *)apIStack_158,puVar1);
      pIStack_e8 = (IUnknown *)((ulonglong)pIStack_e8 & 0xffffffff00000000);
      pcVar2 = *(code **)(*(longlong *)pIVar14 + 0x1a8);
      if ((ulonglong *)*plVar13 != (ulonglong *)0x0) {
        uVar24 = *(ulonglong *)*plVar13;
      }
      uStack_c8 = CONCAT44(uStack_124,CONCAT22(uStack_126,sStack_128));
      uStack_c0 = lStack_120;
      uStack_b8 = uStack_118;
      uStack_58 = CONCAT62(uStack_106,sStack_108);
      lStack_50 = lStack_100;
      uStack_48 = uStack_f8;
      uStack_78 = CONCAT44(uStack_144,CONCAT22(uStack_146,sStack_148));
      lStack_70 = uStack_140;
      uStack_68 = uStack_138;
      pIStack_f0 = (IUnknown *)plVar13;
      *(IUnknown ***)((longlong)alStack_180 + lVar17 + lVar3 + lVar7 + 0x18) = &pIStack_e8;
      *(undefined8 **)((longlong)alStack_180 + lVar17 + lVar3 + lVar7 + 0x10) = &uStack_c8;
      *(undefined8 **)((longlong)alStack_180 + lVar17 + lVar3 + lVar7 + 8) = &uStack_58;
      *(undefined8 **)((longlong)alStack_180 + lVar17 + lVar3 + lVar7) = &uStack_78;
      *(undefined8 *)((longlong)alStack_180 + lVar17 + lVar3 + lVar7 + -8) = uVar16;
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149ee9;
      iVar5 = (*pcVar2)(pIVar14,iStack_90 - iVar5 / 2,uStack_8c,uVar24);
      if (iVar5 < 0) {
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149efe;
        _com_issue_errorex(iVar5,pIVar14,(_GUID *)&DAT_14327ac98);
      }
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149f07;
      FUN_1401be120(plVar13);
      if (sStack_148 == 8) {
        sStack_148 = 0;
        if (uStack_140 != 0) {
          lVar19 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149f27;
          (*DAT_143ad5990)(lVar19);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149f33;
        (*DAT_143262a18)(&sStack_148);
      }
      if (sStack_108 == 8) {
        sStack_108 = 0;
        if (lStack_100 != 0) {
          lVar19 = lStack_100 + -4;
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149f53;
          (*DAT_143ad5990)(lVar19);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149f5f;
        (*DAT_143262a18)(&sStack_108);
      }
      if (sStack_128 == 8) {
        sStack_128 = 0;
        if (lStack_120 != 0) {
          lVar19 = lStack_120 + -4;
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149f7f;
          (*DAT_143ad5990)(lVar19);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149f8b;
        (*DAT_143262a18)(&sStack_128);
      }
      uVar16 = *(undefined8 *)(param_1 + 0x520);
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149fa4;
      FUN_1429e9d30(&lStack_e0,uVar16,1);
      lVar19 = *(longlong *)(param_1 + 0x4e8);
      if (lVar19 == 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = &UNK_14214a886;
        FUN_142ef3ac0(0x80004003);
      }
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149fbe;
      (*DAT_143262a20)();
      if (DAT_143a8b8d8 == 8) {
        if (sStack_148 == 8) {
          sStack_148 = 0;
          if (uStack_140 != 0) {
            lVar20 = uStack_140 + -4;
            *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149fec;
            (*DAT_143ad5990)(lVar20);
          }
        }
        else {
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x142149ff8;
          iVar5 = (*DAT_143262a18)(&sStack_148);
          if (iVar5 < 0) goto LAB_14214a80d;
        }
        sStack_148 = 8;
        if (DAT_143a8b8e0 == 0) {
          uVar25 = 0;
        }
        else {
          uVar25 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a025;
        uStack_140 = FUN_1401a5fa0(DAT_143a8b8e0,uVar25);
      }
      else {
        if ((sStack_148 == 8) && (sStack_148 = 0, uStack_140 != 0)) {
          lVar20 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a086;
          (*DAT_143ad5990)(lVar20);
        }
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a097;
        iVar5 = (*DAT_143262a28)(&sStack_148,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_14214a80d:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = &UNK_14214a814;
          FUN_142ef3ac0(iVar5);
        }
      }
      lVar20 = lStack_e0;
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a03e;
      pIVar14 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
      pIStack_f0 = pIVar14;
      if (pIVar14 != (IUnknown *)0x0) {
        *(IUnknown **)(pIVar14 + 8) = (IUnknown *)0x0;
        *(undefined4 *)(pIVar14 + 0x10) = 1;
        if (lVar20 == 0) {
          *(IUnknown **)pIVar14 = (IUnknown *)0x0;
        }
        else {
          *(undefined4 *)((longlong)alStack_180 + lVar17 + lVar3 + lVar7) = 0;
          *(undefined8 *)((longlong)alStack_180 + lVar17 + lVar3 + lVar7 + -8) = 0;
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a0c0;
          iVar5 = (*DAT_1432627f8)(0xfde9,0,lVar20,0xffffffff);
          iVar5 = (int)((ulonglong)(longlong)(iVar5 * 2) >> 1);
          uVar25 = iVar5 - 1;
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a0df;
          piVar10 = (int *)(*DAT_143ad5980)();
          if (piVar10 == (int *)0x0) {
            pIVar12 = (IUnknown *)0x0;
          }
          else {
            *piVar10 = uVar25 * 2;
            pIVar12 = (IUnknown *)(piVar10 + 1);
            *(undefined2 *)(pIVar12 + (ulonglong)uVar25 * 2) = 0;
          }
          *(int *)((longlong)alStack_180 + lVar17 + lVar3 + lVar7) = iVar5;
          *(IUnknown **)((longlong)alStack_180 + lVar17 + lVar3 + lVar7 + -8) = pIVar12;
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a11d;
          (*DAT_1432627f8)(0xfde9,0,lVar20,0xffffffff);
          *(IUnknown **)pIVar14 = pIVar12;
        }
      }
      apIStack_158[0] = pIVar14;
      if (pIVar14 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = &UNK_14214a858;
        FUN_142ef3ac0(0x8007000e);
      }
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a149;
      iVar5 = FUN_140ee5fb0(lVar19,(IUnknown *)apIStack_158,&sStack_148);
      if (sStack_148 == 8) {
        sStack_148 = 0;
        if (uStack_140 != 0) {
          lVar19 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a16e;
          (*DAT_143ad5990)(lVar19);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a17a;
        (*DAT_143262a18)(&sStack_148);
      }
      pIVar14 = pIStack_d0;
      if (pIStack_d0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = &UNK_14214a87b;
        FUN_142ef3ac0(0x80004003);
      }
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a197;
      (*DAT_143262a20)(&sStack_128);
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a1a7;
      iVar6 = FUN_14023c4c0(&sStack_128,&DAT_143a8b8d8);
      if (iVar6 < 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = &UNK_14214a860;
        FUN_142ef3ac0(iVar6);
      }
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a1b9;
      (*DAT_143262a20)(&sStack_108);
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a1c9;
      iVar6 = FUN_14023c4c0(&sStack_108,&DAT_143a8b8d8);
      if (iVar6 < 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = &UNK_14214a868;
        FUN_142ef3ac0(iVar6);
      }
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a1db;
      (*DAT_143262a20)(&sStack_148);
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a1eb;
      iVar6 = FUN_14023c4c0(&sStack_148,&DAT_143a8b8d8);
      lVar19 = lStack_e0;
      if (iVar6 < 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = &UNK_14214a870;
        FUN_142ef3ac0(iVar6);
      }
      uVar16 = *(undefined8 *)(param_1 + 0x4f0);
      if (lStack_e0 == 0) {
        iVar6 = 2;
      }
      else {
        *(undefined4 *)((longlong)alStack_180 + lVar17 + lVar3 + lVar7) = 0;
        *(undefined8 *)((longlong)alStack_180 + lVar17 + lVar3 + lVar7 + -8) = 0;
        *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a224;
        iVar6 = (*DAT_1432627f8)(0xfde9,0,lVar19,0xffffffff);
        iVar6 = iVar6 * 2;
      }
      lVar19 = lStack_e0;
      uVar24 = (longlong)iVar6 + 0xf;
      if (uVar24 <= (ulonglong)(longlong)iVar6) {
        uVar24 = 0xffffffffffffff0;
      }
      *(undefined8 *)(auStack_1a8 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a24b;
      lVar20 = -(uVar24 & 0xfffffffffffffff0);
      puVar1 = (undefined2 *)((longlong)apIStack_158 + lVar20 + lVar17 + lVar3 + lVar7);
      if (lVar19 == 0) {
        if (puVar1 != (undefined2 *)0x0) {
          *puVar1 = 0;
        }
      }
      else {
        *(undefined4 *)((longlong)alStack_180 + lVar20 + lVar17 + lVar3 + lVar7) = 0x100000;
        *(undefined2 **)((longlong)alStack_180 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = puVar1;
        *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a282;
        (*DAT_1432627f8)(0xfde9,0,lVar19,0xffffffff);
      }
      *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a28e;
      uVar15 = FUN_1401a5890((IUnknown *)apIStack_158,puVar1);
      *(short **)((longlong)alStack_180 + lVar20 + lVar17 + lVar3 + lVar7 + 0x10) = &sStack_128;
      *(short **)((longlong)alStack_180 + lVar20 + lVar17 + lVar3 + lVar7 + 8) = &sStack_108;
      *(short **)((longlong)alStack_180 + lVar20 + lVar17 + lVar3 + lVar7) = &sStack_148;
      *(undefined8 *)((longlong)alStack_180 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = uVar16;
      *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a2c9;
      FUN_140ee7710(pIVar14,iStack_88 - iVar5,uStack_84,uVar15);
      if (sStack_148 == 8) {
        sStack_148 = 0;
        if (uStack_140 != 0) {
          lVar19 = uStack_140 + -4;
          *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a2e9;
          (*DAT_143ad5990)(lVar19);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a2f5;
        (*DAT_143262a18)(&sStack_148);
      }
      if (sStack_108 == 8) {
        sStack_108 = 0;
        if (lStack_100 != 0) {
          lVar19 = lStack_100 + -4;
          *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a315;
          (*DAT_143ad5990)(lVar19);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a321;
        (*DAT_143262a18)(&sStack_108);
      }
      if (sStack_128 == 8) {
        sStack_128 = 0;
        if (lStack_120 != 0) {
          lVar19 = lStack_120 + -4;
          *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a341;
          (*DAT_143ad5990)(lVar19);
        }
      }
      else {
        *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a34d;
        (*DAT_143262a18)(&sStack_128);
      }
      *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a35b;
      uVar15 = FUN_141719be0(param_1 + 0x530,1);
      *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a36c;
      puVar11 = (undefined8 *)FUN_1408a9e40((IUnknown *)apIStack_158,0xa4d);
      lVar19 = lStack_e0;
      uVar16 = *puVar11;
      *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a383;
      FUN_14019ba10(uVar15," %s%s",lVar19,uVar16);
      if (apIStack_158[0] != (IUnknown *)0x0) {
        pIVar14 = apIStack_158[0] + -0x10;
        *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a396;
        FUN_14019f2c0(pIVar14);
      }
      puVar22 = auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7;
      if (pIStack_110 != (IUnknown *)0x0) {
        pIVar14 = pIStack_110 + -0x10;
        *(undefined8 *)(auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7 + -8) = 0x14214a3a9;
        FUN_14019f2c0(pIVar14);
        puVar22 = auStack_1a8 + lVar20 + lVar17 + lVar3 + lVar7;
      }
    }
    uVar24 = 0;
    apIStack_158[0] = pIStack_d0;
    if (pIStack_d0 != (IUnknown *)0x0) {
      pcVar2 = *(code **)(*(longlong *)pIStack_d0 + 8);
      *(undefined8 *)(puVar22 + -8) = 0x14214a3c0;
      (*pcVar2)();
    }
    *(undefined8 *)(puVar22 + -8) = 0x14214a3cd;
    FUN_14214ca90(param_1,(IUnknown *)apIStack_158);
    uVar18 = uVar24;
    while ((pIVar14 = pIStack_d0, lVar7 = *(longlong *)(param_1 + 0x1668), lVar7 != 0 &&
           (uVar25 = (uint)uVar24, uVar25 < *(uint *)(lVar7 + -8)))) {
      if ((int)uVar25 < 0) {
        *(undefined8 *)(puVar22 + -8) = 0x14214a407;
        FUN_142e54290(0xbc,uVar24);
        lVar7 = *(longlong *)(param_1 + 0x1668);
      }
      pIVar14 = pIStack_d0;
      piVar10 = (int *)(lVar7 + uVar18);
      if (*piVar10 == 0) {
        if (pIStack_d0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar22 + -8) = &UNK_14214a8ca;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar22 + -8) = 0x14214a4db;
        (*DAT_143262a20)(&sStack_128);
        *(undefined8 *)(puVar22 + -8) = 0x14214a4eb;
        iVar5 = FUN_14023c4c0(&sStack_128,&DAT_143a8b8d8);
        if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar22 + -8) = &UNK_14214a8bf;
          FUN_142ef3ac0(iVar5);
        }
        *(undefined8 *)(puVar22 + -8) = 0x14214a4fd;
        (*DAT_143262a20)(&sStack_108);
        *(undefined8 *)(puVar22 + -8) = 0x14214a50d;
        iVar5 = FUN_14023c4c0(&sStack_108,&DAT_143a8b8d8);
        if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar22 + -8) = &UNK_14214a8b7;
          FUN_142ef3ac0(iVar5);
        }
        *(undefined8 *)(puVar22 + -8) = 0x14214a51f;
        (*DAT_143262a20)(&sStack_148);
        *(undefined8 *)(puVar22 + -8) = 0x14214a52f;
        iVar5 = FUN_14023c4c0(&sStack_148,&DAT_143a8b8d8);
        if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar22 + -8) = &UNK_14214a8af;
          FUN_142ef3ac0(iVar5);
        }
        uVar16 = *(undefined8 *)(piVar10 + 6);
        uVar15 = *(undefined8 *)(piVar10 + 8);
        *(undefined8 *)(puVar22 + -8) = 0x14214a548;
        uVar15 = FUN_1401a5780((IUnknown *)apIStack_158,uVar15);
        iVar6 = iStack_a4 + piVar10[0xd];
        iVar5 = piVar10[0xc] + iStack_a8;
        *(short **)(puVar22 + 0x38) = &sStack_128;
        *(short **)(puVar22 + 0x30) = &sStack_108;
        *(short **)(puVar22 + 0x28) = &sStack_148;
        *(undefined8 *)(puVar22 + 0x20) = uVar16;
        *(undefined8 *)(puVar22 + -8) = 0x14214a587;
        FUN_140ee7710(pIVar14,iVar5,iVar6,uVar15);
        if (sStack_148 == 8) {
          sStack_148 = 0;
          if (uStack_140 != 0) {
            lVar7 = uStack_140 + -4;
            *(undefined8 *)(puVar22 + -8) = 0x14214a5a7;
            (*DAT_143ad5990)(lVar7);
          }
        }
        else {
          *(undefined8 *)(puVar22 + -8) = 0x14214a5b3;
          (*DAT_143262a18)(&sStack_148);
        }
        if (sStack_108 == 8) {
          sStack_108 = 0;
          if (lStack_100 != 0) {
            lVar7 = lStack_100 + -4;
            *(undefined8 *)(puVar22 + -8) = 0x14214a5d3;
            (*DAT_143ad5990)(lVar7);
          }
        }
        else {
          *(undefined8 *)(puVar22 + -8) = 0x14214a5df;
          (*DAT_143262a18)(&sStack_108);
        }
        if (sStack_128 == 8) {
          sStack_128 = 0;
          lVar7 = lStack_120;
          goto LAB_14214a5f0;
        }
        psVar21 = &sStack_128;
LAB_14214a605:
        *(undefined8 *)(puVar22 + -8) = 0x14214a60b;
        (*DAT_143262a18)(psVar21);
      }
      else if (*piVar10 == 2) {
        if (pIStack_d0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar22 + -8) = &UNK_14214a8a7;
          FUN_142ef3ac0(0x80004003);
        }
        sStack_148 = 3;
        uStack_140 = CONCAT44(uStack_140._4_4_,0xff);
        iVar6 = iStack_a4 + piVar10[0xd];
        iVar5 = iStack_a8 + piVar10[0xc];
        lVar7 = *(longlong *)pIStack_d0;
        uStack_c8 = CONCAT44(uStack_144,CONCAT22(uStack_146,3));
        uStack_c0 = CONCAT44(uStack_140._4_4_,0xff);
        uStack_b8 = uStack_138;
        *(undefined8 **)(puVar22 + 0x20) = &uStack_c8;
        uVar16 = *(undefined8 *)(piVar10 + 10);
        pcVar2 = *(code **)(lVar7 + 0x128);
        *(undefined8 *)(puVar22 + -8) = 0x14214a48d;
        iVar5 = (*pcVar2)(pIVar14,iVar5,iVar6,uVar16);
        if (iVar5 < 0) {
          *(undefined8 *)(puVar22 + -8) = 0x14214a4a2;
          _com_issue_errorex(iVar5,pIVar14,(_GUID *)&DAT_14327ac98);
        }
        if (sStack_148 != 8) {
          psVar21 = &sStack_148;
          goto LAB_14214a605;
        }
        sStack_148 = 0;
        lVar7 = uStack_140;
LAB_14214a5f0:
        if (lVar7 != 0) {
          *(undefined8 *)(puVar22 + -8) = 0x14214a5ff;
          (*DAT_143ad5990)(lVar7 + -4);
        }
      }
      uVar18 = uVar18 + 0x58;
      uVar24 = (ulonglong)(uVar25 + 1);
    }
    pIStack_e8 = pIStack_d0;
    if (pIStack_d0 != (IUnknown *)0x0) {
      pcVar2 = *(code **)(*(longlong *)pIStack_d0 + 8);
      *(undefined8 *)(puVar22 + -8) = 0x14214a631;
      (*pcVar2)(pIVar14);
    }
    if (*(int *)(param_1 + 0x500) != 0) {
      pIStack_f0 = (IUnknown *)apIStack_158;
      *(undefined8 *)(puVar22 + -8) = 0x14214a65a;
      uVar16 = FUN_1403edf80((IUnknown *)apIStack_158,L"myItemGrid_rb",0xffffffff);
      *(undefined8 *)(puVar22 + -8) = 0x14214a673;
      uVar15 = FUN_1403edf80(&pIStack_110,L"myItemGrid_lt",0xffffffff);
      *(undefined8 *)(puVar22 + -8) = 0x14214a68d;
      FUN_14214ea90(&pIStack_e8,param_1 + 0x1670,uVar15,uVar16);
      pIVar14 = pIStack_e8;
    }
    if (*(int *)(param_1 + 0x504) != 0) {
      pIStack_38 = (IUnknown *)apIStack_158;
      *(undefined8 *)(puVar22 + -8) = 0x14214a6c5;
      uVar16 = FUN_1403edf80((IUnknown *)apIStack_158,L"otherItemGrid_rb",0xffffffff);
      *(undefined8 *)(puVar22 + -8) = 0x14214a6de;
      uVar15 = FUN_1403edf80(&pIStack_110,L"otherItemGrid_lt",0xffffffff);
      *(undefined8 *)(puVar22 + -8) = 0x14214a6f1;
      FUN_14214ea90(&pIStack_e8,param_1 + 0x1670,uVar15,uVar16);
      pIVar14 = pIStack_e8;
    }
    *(undefined8 *)(puVar22 + -8) = 0x14214a6fd;
    iVar5 = FUN_141c3f8a0(param_1);
    if ((iVar5 < 2) || (uVar4 = 1, *(int *)(param_1 + 0x500) != 0)) {
      uVar4 = 0;
    }
    *(undefined8 *)(puVar22 + -8) = 0x14214a726;
    FUN_142aa2010(param_1 + 0x1670,L"trade",uVar4);
    if (pIVar14 != (IUnknown *)0x0) {
      pcVar2 = *(code **)(*(longlong *)pIVar14 + 0x10);
      *(undefined8 *)(puVar22 + -8) = 0x14214a735;
      (*pcVar2)(pIVar14);
    }
    if (puStack_d8 != (undefined4 *)0x0) {
      puVar8 = puStack_d8 + -4;
      *(undefined8 *)(puVar22 + -8) = 0x14214a74b;
      FUN_14019f2c0(puVar8);
    }
    if (lStack_e0 != 0) {
      lVar7 = lStack_e0 + -0x10;
      *(undefined8 *)(puVar22 + -8) = 0x14214a75e;
      FUN_14019f2c0(lVar7);
    }
  }
  puVar23 = puVar22;
  if (pIStack_80 != (IUnknown *)0x0) {
    pcVar2 = *(code **)(*(longlong *)pIStack_80 + 0x10);
    *(undefined8 *)(puVar22 + -8) = 0x14214a771;
    (*pcVar2)();
  }
LAB_14214a779:
  if (pIStack_d0 != (IUnknown *)0x0) {
    pcVar2 = *(code **)(*(longlong *)pIStack_d0 + 0x10);
    *(undefined8 *)(puVar23 + -8) = 0x14214a784;
    (*pcVar2)();
  }
  *(undefined8 *)(puVar23 + -8) = 0x14214a794;
  return;
}



//===========================================================
// FUN_14214a900 @ 14214a900   (179 bytes)
//===========================================================

void FUN_14214a900(longlong param_1,int param_2,undefined4 *param_3,undefined8 *param_4)

{
  int iVar1;
  undefined8 *puVar2;
  wchar_t *pwVar3;
  longlong *local_28;
  undefined8 local_20;
  undefined1 local_18 [16];
  
  FUN_14090ead0(&local_28,DAT_143ad6eb8);
  if (local_28 != (longlong *)0x0) {
    iVar1 = FUN_141c3f8b0(param_1);
    if (param_2 == iVar1) {
      *param_3 = 5;
      pwVar3 = L"myAvatar";
    }
    else {
      pwVar3 = L"otherAvatar";
    }
    local_20 = 0;
    puVar2 = (undefined8 *)FUN_141adbce0(param_1 + 0x1670,local_18,pwVar3,&local_20);
    *param_4 = *puVar2;
  }
  if (local_28 != (longlong *)0x0) {
    (**(code **)(*local_28 + 0x10))();
  }
  return;
}



//===========================================================
// FUN_14214a9c0 @ 14214a9c0   (181 bytes)
//===========================================================

void FUN_14214a9c0(longlong *param_1,int param_2,undefined8 param_3)

{
  char cVar1;
  longlong lVar2;
  int aiStackX_10 [2];
  
  if (param_2 == 0x10) {
    FUN_1406e9170(param_3,aiStackX_10,4);
    if (aiStackX_10[0] == 0) {
      FUN_14214ae50(param_1,param_3);
    }
    else if (aiStackX_10[0] == 1) {
      cVar1 = FUN_1406e8ae0(param_3);
      lVar2 = FUN_1406e8f10(param_3);
      param_1[(longlong)cVar1 + 0xa3] = lVar2;
      (**(code **)(*param_1 + 0x90))(param_1,0);
    }
    else if (aiStackX_10[0] == 2) {
      FUN_14214b090(param_1,param_3);
    }
    else if (aiStackX_10[0] == 6) {
      FUN_14214b3b0(param_1,param_3);
    }
    (**(code **)(*param_1 + 0x90))(param_1,0);
  }
  return;
}



//===========================================================
// FUN_14214a9c0 @ 14214a9c0   (181 bytes)
//===========================================================

void FUN_14214a9c0(longlong *param_1,int param_2,undefined8 param_3)

{
  char cVar1;
  longlong lVar2;
  int aiStackX_10 [2];
  
  if (param_2 == 0x10) {
    FUN_1406e9170(param_3,aiStackX_10,4);
    if (aiStackX_10[0] == 0) {
      FUN_14214ae50(param_1,param_3);
    }
    else if (aiStackX_10[0] == 1) {
      cVar1 = FUN_1406e8ae0(param_3);
      lVar2 = FUN_1406e8f10(param_3);
      param_1[(longlong)cVar1 + 0xa3] = lVar2;
      (**(code **)(*param_1 + 0x90))(param_1,0);
    }
    else if (aiStackX_10[0] == 2) {
      FUN_14214b090(param_1,param_3);
    }
    else if (aiStackX_10[0] == 6) {
      FUN_14214b3b0(param_1,param_3);
    }
    (**(code **)(*param_1 + 0x90))(param_1,0);
  }
  return;
}



//===========================================================
// FUN_14214a9c0 @ 14214a9c0   (181 bytes)
//===========================================================

void FUN_14214a9c0(longlong *param_1,int param_2,undefined8 param_3)

{
  char cVar1;
  longlong lVar2;
  int aiStackX_10 [2];
  
  if (param_2 == 0x10) {
    FUN_1406e9170(param_3,aiStackX_10,4);
    if (aiStackX_10[0] == 0) {
      FUN_14214ae50(param_1,param_3);
    }
    else if (aiStackX_10[0] == 1) {
      cVar1 = FUN_1406e8ae0(param_3);
      lVar2 = FUN_1406e8f10(param_3);
      param_1[(longlong)cVar1 + 0xa3] = lVar2;
      (**(code **)(*param_1 + 0x90))(param_1,0);
    }
    else if (aiStackX_10[0] == 2) {
      FUN_14214b090(param_1,param_3);
    }
    else if (aiStackX_10[0] == 6) {
      FUN_14214b3b0(param_1,param_3);
    }
    (**(code **)(*param_1 + 0x90))(param_1,0);
  }
  return;
}



//===========================================================
// FUN_14214aaa0 @ 14214aaa0   (265 bytes)
//===========================================================

void FUN_14214aaa0(longlong *param_1,int param_2,undefined8 param_3,int param_4)

{
  int iVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  
  iVar1 = FUN_141c3f8b0();
  lVar4 = DAT_143aa84a0;
  if (iVar1 == param_2) {
    if (DAT_143aa84a0 == 0) {
      return;
    }
    lVar2 = FUN_142cbe730(DAT_143aa84a0);
    if (lVar2 == 0) {
      return;
    }
    uVar3 = FUN_142cbe730(lVar4);
    if (param_4 == 9) {
      lVar4 = FUN_1401d35e0(uVar3);
      if ((param_1[0xa5] < lVar4) && (lVar4 - param_1[0xa5] < param_1[0xa4] - param_1[0xa3])) {
        (**(code **)(*param_1 + 0x138))(param_1,8);
        goto LAB_14214ab76;
      }
    }
    (**(code **)(*param_1 + 0x138))(param_1,8);
    if (param_4 - 1U < 0xf) {
LAB_14214ab76:
                    /* WARNING: Could not recover jumptable at 0x00014214ab89. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                *(uint *)(&DAT_14214ae08 + (longlong)(param_4 + -1) * 4)))
                (IMAGE_DOS_HEADER_140000000.e_magic +
                 *(uint *)(&DAT_14214ae08 + (longlong)(param_4 + -1) * 4));
      return;
    }
  }
  else {
    (**(code **)(*param_1 + 0x90))(param_1,0);
  }
  return;
}



//===========================================================
// FUN_14214ae50 @ 14214ae50   (472 bytes)
//===========================================================

void FUN_14214ae50(longlong *param_1,undefined8 param_2)

{
  longlong *plVar1;
  undefined1 *puVar2;
  char cVar3;
  char cVar4;
  undefined4 uVar5;
  longlong lVar6;
  uint uVar7;
  uint uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined1 local_28 [8];
  longlong *local_20;
  
  cVar3 = FUN_1406e8ae0(param_2);
  cVar4 = FUN_1406e8ae0(param_2);
  FUN_140303530(local_28,param_2);
  if (local_20 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  uVar5 = (**(code **)(*local_20 + 0x98))();
  uVar8 = (int)cVar4 - 1;
  lVar6 = param_1[(longlong)cVar3 + 0xa1];
  uVar9 = 0;
  uVar7 = 0;
  if (lVar6 != 0) {
    uVar7 = *(uint *)(lVar6 + -8);
  }
  if (((int)uVar8 < 0) || (uVar7 <= uVar8)) {
    uVar10 = 0;
    if (lVar6 != 0) {
      uVar10 = *(undefined4 *)(lVar6 + -8);
    }
    FUN_142e54290(0xbc,uVar8,uVar10);
    lVar6 = param_1[(longlong)cVar3 + 0xa1];
  }
  *(undefined4 *)((longlong)cVar4 * 0x18 + -8 + lVar6) = uVar5;
  lVar6 = param_1[(longlong)cVar3 + 0xa1];
  uVar7 = 0;
  if (lVar6 != 0) {
    uVar7 = *(uint *)(lVar6 + -8);
  }
  if (((int)uVar8 < 0) || (uVar7 <= uVar8)) {
    if (lVar6 != 0) {
      uVar9 = *(undefined4 *)(lVar6 + -8);
    }
    FUN_142e54290(0xbc,uVar8,uVar9);
    lVar6 = param_1[(longlong)cVar3 + 0xa1];
  }
  puVar2 = (undefined1 *)((longlong)cVar4 * 0x18 + -0x18 + lVar6);
  if ((*(longlong *)(puVar2 + 8) - 1U < 999) || (*(longlong *)(puVar2 + 8) == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (puVar2 == local_28) {
    FUN_142e52d50(0x45c,1);
  }
  plVar1 = local_20;
  if (local_20 != (longlong *)0x0) {
    if (0xfffff < (ulonglong)local_20[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    plVar1[1] = plVar1[1] + 1;
    UNLOCK();
  }
  FUN_1401abd80(puVar2);
  *(longlong **)(puVar2 + 8) = local_20;
  (**(code **)(*param_1 + 0x90))(param_1,0);
  plVar1 = local_20;
  if (local_20 != (longlong *)0x0) {
    if (0xffffe < local_20[1] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = plVar1 + 1;
    lVar6 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (local_20 != (longlong *)0x0)) {
      (**(code **)*local_20)(local_20,1);
    }
  }
  return;
}



//===========================================================
// FUN_14214b030 @ 14214b030   (79 bytes)
//===========================================================

void FUN_14214b030(longlong *param_1,undefined8 param_2)

{
  char cVar1;
  longlong lVar2;
  
  cVar1 = FUN_1406e8ae0(param_2);
  lVar2 = FUN_1406e8f10(param_2);
  param_1[(longlong)cVar1 + 0xa3] = lVar2;
                    /* WARNING: Could not recover jumptable at 0x00014214b078. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(*param_1 + 0x90))(param_1,0);
  return;
}



//===========================================================
// FUN_14214b090 @ 14214b090   (789 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14214b090(longlong *param_1)

{
  ulonglong uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  longlong lVar4;
  uint uVar5;
  longlong lVar6;
  ulonglong uVar7;
  int iVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  uint uVar11;
  undefined4 *puVar12;
  undefined4 *puVar13;
  uint uVar14;
  undefined4 *puVar15;
  undefined1 auStack_4b8 [32];
  longlong *local_498;
  undefined4 *local_490;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_4b8;
  puVar12 = (undefined4 *)0x0;
  local_490 = (undefined4 *)0x0;
  puVar9 = puVar12;
  puVar13 = puVar12;
  puVar15 = puVar12;
  local_498 = param_1;
  do {
    lVar4 = param_1[0xa2];
    uVar11 = 0;
    uVar5 = uVar11;
    if (lVar4 != 0) {
      uVar5 = *(uint *)(lVar4 + -8);
    }
    uVar14 = (uint)puVar15;
    if (uVar5 <= uVar14) {
      puVar10 = puVar12;
      if (lVar4 != 0) {
        puVar10 = (undefined4 *)(ulonglong)*(uint *)(lVar4 + -8);
      }
      FUN_142e54290(0xbc,puVar15,puVar10);
      lVar4 = param_1[0xa2];
    }
    if (*(longlong *)(lVar4 + 8 + (longlong)puVar13) != 0) {
      if (lVar4 != 0) {
        uVar11 = *(uint *)(lVar4 + -8);
      }
      if (uVar11 <= uVar14) {
        puVar10 = puVar12;
        if (lVar4 != 0) {
          puVar10 = (undefined4 *)(ulonglong)*(uint *)(lVar4 + -8);
        }
        FUN_142e54290(0xbc,puVar15,puVar10);
        lVar4 = param_1[0xa2];
      }
      lVar6 = *(longlong *)((longlong)puVar13 + lVar4 + 8);
      if (lVar6 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar6 = *(longlong *)((longlong)puVar13 + lVar4 + 8);
      }
      uVar2 = FUN_14019a5d0(lVar6 + 0x20);
      uVar3 = FUN_1403a1f90(DAT_143aa8328);
      uVar5 = 0;
      if (puVar9 == (undefined4 *)0x0) {
LAB_14214b1c2:
        iVar8 = 1;
        puVar15 = puVar12;
        if (puVar9 != (undefined4 *)0x0) {
LAB_14214b1d9:
          uVar1 = *(ulonglong *)(puVar9 + -4);
          uVar7 = ~uVar1;
          if (-1 < (longlong)uVar1) {
            uVar7 = uVar1;
          }
          if ((int)(uVar7 - 8 >> 3) == iVar8) goto LAB_14214b25a;
          if (puVar9 == (undefined4 *)0x0) {
            puVar15 = (undefined4 *)0x0;
          }
          else {
            puVar15 = (undefined4 *)(ulonglong)(uint)puVar9[-2];
          }
        }
        lVar4 = FUN_14019b780(&DAT_143ad68a0);
        puVar10 = (undefined4 *)(lVar4 + 8);
        if (lVar4 == 0) {
          puVar10 = puVar12;
        }
        if (puVar9 != (undefined4 *)0x0) {
          FUN_142ef7ba0(puVar10,puVar9,(longlong)puVar15 << 3);
          thunk_FUN_140205820(puVar9 + -2);
        }
        *(undefined4 **)(puVar10 + -2) = puVar15;
        puVar9 = puVar10;
        param_1 = local_498;
        local_490 = puVar10;
      }
      else {
        uVar5 = puVar9[-2];
        uVar1 = *(ulonglong *)(puVar9 + -4);
        uVar7 = ~uVar1;
        if (-1 < (longlong)uVar1) {
          uVar7 = uVar1;
        }
        if ((uint)(uVar7 - 8 >> 3) <= uVar5) {
          if (uVar5 == 0) goto LAB_14214b1c2;
          iVar8 = uVar5 * 2;
          goto LAB_14214b1d9;
        }
      }
LAB_14214b25a:
      *(longlong *)(puVar9 + -2) = *(longlong *)(puVar9 + -2) + 1;
      *(ulonglong *)(puVar9 + (longlong)(int)uVar5 * 2) = CONCAT44(uVar3,uVar2);
    }
    puVar15 = (undefined4 *)(ulonglong)(uVar14 + 1);
    puVar13 = puVar13 + 6;
    if (8 < (int)(uVar14 + 1)) {
      FUN_1406ed520(local_488,0x17e);
      local_498._0_4_ = 0x10;
      FUN_1406ede20(local_488,&local_498,4);
      local_498 = (longlong *)CONCAT44(local_498._4_4_,5);
      FUN_1406ede20(local_488,&local_498,4);
      if (puVar9 == (undefined4 *)0x0) {
        FUN_1406ed840(local_488,0);
      }
      else {
        uVar5 = puVar9[-2];
        FUN_1406ed840(local_488,uVar5 & 0xff);
        puVar13 = puVar9;
        if (0 < (int)uVar5) {
          do {
            uVar11 = (uint)puVar12;
            if ((uint)puVar9[-2] <= uVar11) {
              FUN_142e54290(0xbc,puVar12,puVar9[-2]);
            }
            FUN_1406ed9d0(local_488,*puVar13);
            if ((uint)puVar9[-2] <= uVar11) {
              FUN_142e54290(0xbc,puVar12,puVar9[-2]);
            }
            FUN_1406ed9d0(local_488,puVar13[1]);
            puVar12 = (undefined4 *)(ulonglong)(uVar11 + 1);
            puVar13 = puVar13 + 2;
          } while ((int)(uVar11 + 1) < (int)uVar5);
        }
      }
      FUN_1415d01c0(local_488);
      *(undefined4 *)((longlong)param_1 + 0x504) = 1;
      (**(code **)(*param_1 + 0x90))(param_1,0);
      FUN_1406ed610(local_488);
      if (puVar9 != (undefined4 *)0x0) {
        thunk_FUN_140205820(puVar9 + -2,0);
      }
      return;
    }
  } while( true );
}



//===========================================================
// FUN_14214b3b0 @ 14214b3b0   (534 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014214b506) */

void FUN_14214b3b0(longlong param_1)

{
  int *piVar1;
  int *piVar2;
  undefined8 *puVar3;
  int *piVar4;
  int *piVar5;
  ulonglong uVar6;
  int iVar7;
  int iVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  int *local_res8 [2];
  int *local_res18;
  int *local_res20;
  
  uVar10 = 0;
  local_res18 = (int *)0x0;
  puVar3 = (undefined8 *)FUN_1408a9e40(&local_res20,0x10f7);
  piVar1 = (int *)*puVar3;
  *puVar3 = 0;
  local_res18 = piVar1;
  if (local_res20 != (int *)0x0) {
    FUN_14019f2c0(local_res20 + -4);
  }
  local_res8[0] = (int *)0x0;
  piVar5 = piVar1;
  piVar2 = local_res8[0];
  if ((piVar1 == (int *)0x0) || (piVar4 = piVar1 + -4, piVar4 == (int *)0x0)) goto LAB_14214b56f;
  if (*piVar4 != -1) {
    if (*piVar4 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar4 = *piVar4 + 1;
    UNLOCK();
    piVar5 = local_res18;
    piVar2 = piVar1;
    if (local_res8[0] != (int *)0x0) {
      FUN_14019f2c0(local_res8[0] + -4);
      piVar5 = local_res18;
    }
    goto LAB_14214b56f;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  uVar6 = 0xffffffffffffffff;
  uVar9 = 0xffffffffffffffff;
  do {
    uVar9 = uVar9 + 1;
  } while (*(char *)((longlong)piVar1 + uVar9) != '\0');
  iVar7 = (int)uVar9;
  iVar8 = 0;
  if (0 < iVar7) {
    iVar8 = iVar7;
  }
  piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
  piVar4[1] = iVar8;
  *piVar4 = -1;
  piVar2 = piVar4 + 4;
  piVar4[2] = 0;
  *(undefined1 *)piVar2 = 0;
  local_res20 = piVar2;
  FUN_142ef7ba0(piVar2,piVar1,(longlong)iVar7);
  if (*piVar4 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar7 == -1) || (iVar7 <= piVar4[1])) {
    *piVar4 = 1;
    if (iVar7 != -1) goto LAB_14214b4bc;
    if (piVar2 != (int *)0x0) {
      do {
        uVar6 = uVar6 + 1;
      } while (*(char *)((longlong)piVar2 + uVar6) != '\0');
      uVar10 = uVar6 & 0xffffffff;
    }
  }
  else {
    FUN_142e54290(0x90,piVar4[1],uVar9 & 0xffffffff);
    *piVar4 = 1;
LAB_14214b4bc:
    *(undefined1 *)((longlong)piVar2 + (longlong)iVar7) = 0;
    uVar10 = uVar9;
  }
  iVar8 = (int)uVar10;
  if ((iVar8 < 0) || (piVar4[1] + 1 <= iVar8)) {
    FUN_142e54290(0x9c,uVar10 & 0xffffffff);
  }
  piVar4[2] = iVar8;
  if (local_res8[0] != (int *)0x0) {
    FUN_14019f2c0(local_res8[0] + -4);
  }
LAB_14214b56f:
  local_res8[0] = piVar2;
  FUN_142a26280(local_res8,0,param_1 + 0x240,1,0,0,0,0,0,0);
  if (piVar5 != (int *)0x0) {
    FUN_14019f2c0(piVar5 + -4);
  }
  return;
}



//===========================================================
// FUN_14214b5d0 @ 14214b5d0   (1358 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined4
FUN_14214b5d0(longlong param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
             undefined4 param_5,undefined4 param_6,char param_7)

{
  int iVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined1 auStack_5d8 [32];
  undefined4 local_5b8;
  undefined4 local_5b0;
  ulonglong local_5a8;
  ulonglong local_5a0;
  undefined4 local_598;
  undefined4 local_590;
  int local_588;
  int local_584;
  undefined4 local_580;
  int local_57c;
  undefined4 local_578;
  undefined4 local_574;
  undefined4 local_570;
  undefined4 local_56c;
  int local_568;
  undefined4 local_564;
  undefined1 local_550 [16];
  longlong *local_540;
  longlong *local_538;
  longlong local_530;
  undefined8 local_528;
  longlong *local_520;
  undefined8 local_518;
  undefined8 local_510;
  undefined8 local_508;
  undefined8 local_500;
  undefined1 *local_4f8;
  ulonglong local_4f0;
  ulonglong local_4e8;
  undefined1 *local_4e0;
  ulonglong local_4d8;
  ulonglong local_4d0;
  undefined1 *local_4c8;
  undefined8 local_4c0;
  ulonglong local_4b8;
  ulonglong local_4b0;
  undefined8 local_4a8;
  undefined8 local_4a0;
  undefined8 local_498;
  longlong local_490;
  undefined1 local_488 [8];
  undefined1 local_480 [8];
  undefined1 local_478 [1120];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_5d8;
  if ((*(int *)(param_1 + 0x500) != 0) || (iVar1 = FUN_141c3f8a0(param_1), iVar1 < 2)) {
    local_578 = 0;
    FUN_1401ab4e0(param_2);
    return local_578;
  }
  iVar1 = FUN_141087b40(param_2);
  if (iVar1 != 0) {
    local_574 = 0;
    FUN_1401ab4e0(param_2);
    return local_574;
  }
  local_588 = 0xffffffff;
  if (param_7 == '\0') {
    local_588 = FUN_14214d2f0(param_1,param_5,param_6);
  }
  else {
    local_588 = FUN_14214d6c0(param_1);
  }
  if ((local_588 < 1) || (9 < local_588)) {
LAB_14214b6fe:
    local_570 = 0;
    FUN_1401ab4e0(param_2);
    return local_570;
  }
  local_490 = param_1 + 0x508;
  lVar2 = FUN_14214e8d0(local_490,local_588 + -1);
  if (-1 < *(int *)(lVar2 + 0x10)) goto LAB_14214b6fe;
  local_518 = FUN_140caa510();
  iVar1 = FUN_142cc42d0(local_518,500,0);
  if (iVar1 == 0) {
    local_56c = 0;
    FUN_1401ab4e0(param_2);
    return local_56c;
  }
  uVar3 = FUN_1401a19e0(param_2);
  iVar1 = FUN_14214ece0(uVar3);
  if (iVar1 == 0) {
    local_508 = FUN_140caa4e0();
    local_510 = FUN_141087b50(param_2);
    iVar1 = FUN_1417ea500(local_508,local_510);
    if (iVar1 == 0) {
      local_57c = 0;
      goto LAB_14214b7c6;
    }
  }
  local_57c = 1;
LAB_14214b7c6:
  local_568 = local_57c;
  local_580 = 1;
  if (local_57c != 0) {
    local_540 = (longlong *)FUN_1401a19e0(param_2);
    (**(code **)(*local_540 + 0x98))(local_540);
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  uVar3 = FUN_141087b50(param_2);
  iVar1 = FUN_1403e1820(uVar3);
  if (iVar1 != 0) {
    halt_baddata();
  }
  local_538 = (longlong *)FUN_1401a19e0(param_2);
  local_584 = (**(code **)(*local_538 + 0x98))(local_538);
  if (local_584 < 2) {
    halt_baddata();
  }
  local_530 = FUN_14019a150(2000);
  if (local_530 == 0) {
    local_528 = 0;
  }
  else {
    local_528 = FUN_142a57d30(local_530,0,0,0);
  }
  local_500 = local_528;
  FUN_1410879e0(local_550,local_528,1);
  local_4a0 = FUN_141087b90(local_550);
  local_4f8 = local_478;
  local_4f0 = FUN_1410879b0(local_4f8,0);
  local_4e0 = local_480;
  local_4e8 = local_4f0;
  local_4b8 = local_4f0;
  local_4d8 = FUN_140ca6280(local_4e0,0);
  local_4c8 = local_488;
  local_4d0 = local_4d8;
  local_4b0 = local_4d8;
  local_4c0 = FUN_1408a9e40(local_4c8,0x527);
  local_590 = 0;
  local_598 = 0;
  local_5a0 = local_4b8;
  local_5a8 = local_4b0;
  local_5b0 = 0;
  local_5b8 = 1;
  local_4a8 = local_4c0;
  FUN_142a61900(local_4a0,3,0,local_4c0);
  local_498 = FUN_141087b90(local_550);
  local_5a0 = local_5a0 & 0xffffffff00000000;
  local_5a8 = local_5a8 & 0xffffffff00000000;
  local_5b0 = 10;
  local_5b8 = 0;
  FUN_142a62cc0(local_498,local_584,1,local_584);
  uVar3 = FUN_141087b90(local_550);
  FUN_142a5ee30(uVar3);
  local_520 = (longlong *)FUN_141087b90(local_550);
  iVar1 = (**(code **)(*local_520 + 0x130))(local_520);
  if (iVar1 != 1) {
    local_564 = 0;
    FUN_140d2cb10(local_550);
    FUN_1401ab4e0(param_2);
    return local_564;
  }
  uVar3 = FUN_141087b90(local_550);
  local_580 = FUN_142a643b0(uVar3);
  FUN_140d2cb10(local_550);
  halt_baddata();
}



//===========================================================
// FUN_14214bc10 @ 14214bc10   (1005 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14214bc10(longlong param_1)

{
  int iVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined1 auStack_5c8 [32];
  undefined4 local_5a8;
  undefined4 local_5a0;
  ulonglong local_598;
  undefined8 local_590;
  undefined4 local_588;
  undefined4 local_580;
  undefined1 local_578 [16];
  int local_568;
  longlong local_558;
  undefined8 local_550;
  longlong local_548;
  undefined8 local_540;
  longlong *local_538;
  undefined8 local_530;
  undefined8 local_528;
  undefined1 *local_520;
  undefined8 local_518;
  undefined8 local_510;
  undefined1 *local_508;
  ulonglong local_500;
  ulonglong local_4f8;
  undefined1 *local_4f0;
  undefined8 local_4e8;
  undefined8 local_4e0;
  ulonglong local_4d8;
  undefined8 local_4d0;
  undefined8 local_4c8;
  undefined8 local_4c0;
  undefined8 local_4b8;
  undefined1 *local_4b0;
  undefined8 local_4a8;
  undefined8 local_4a0;
  undefined1 local_498 [8];
  undefined1 local_490 [8];
  undefined1 local_488 [8];
  undefined1 local_480 [1128];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_5c8;
  if ((*(int *)(param_1 + 0x500) == 0) && (iVar1 = FUN_141c3f8a0(param_1), 1 < iVar1)) {
    local_550 = FUN_140caa510();
    iVar1 = FUN_142cc42d0(local_550,500,0);
    if (iVar1 != 0) {
      local_530 = FUN_142cbe730(local_550);
      local_4a0 = local_530;
      local_548 = FUN_14019a150(2000);
      if (local_548 == 0) {
        local_540 = 0;
      }
      else {
        local_540 = FUN_142a57d30(local_548,0,0,0);
      }
      local_528 = local_540;
      FUN_1410879e0(local_578,local_540,1);
      local_4c8 = FUN_141087b90(local_578);
      local_520 = local_480;
      local_518 = FUN_1410879b0(local_520,0);
      local_508 = local_488;
      local_510 = local_518;
      local_4e0 = local_518;
      local_500 = FUN_140ca6280(local_508,0);
      local_4f0 = local_498;
      local_4f8 = local_500;
      local_4d8 = local_500;
      local_4e8 = FUN_1408a9e40(local_4f0,0x1d7);
      local_580 = 0;
      local_588 = 0;
      local_590 = local_4e0;
      local_598 = local_4d8;
      local_5a0 = 0;
      local_5a8 = 1;
      local_4d0 = local_4e8;
      FUN_142a61900(local_4c8,4,0,local_4e8);
      local_4b8 = FUN_141087b90(local_578);
      uVar2 = FUN_1401d35e0(local_530);
      local_4c0 = FUN_141087970(499999999999,uVar2);
      local_590._0_4_ = 1;
      local_598 = local_598 & 0xffffffff00000000;
      local_5a0 = 0xc;
      local_5a8 = 0;
      FUN_142a62d60(local_4b8,1,1,local_4c0);
      uVar2 = FUN_141087b90(local_578);
      FUN_142a5ee30(uVar2);
      local_538 = (longlong *)FUN_141087b90(local_578);
      local_568 = (**(code **)(*local_538 + 0x130))(local_538);
      if (local_568 == 1) {
        uVar2 = FUN_141087b90(local_578);
        local_558 = FUN_142a643c0(uVar2);
        if (local_558 < 500000000000) {
          lVar3 = FUN_1401d35e0(local_530);
          if (local_558 <= lVar3) {
                    /* WARNING: Bad instruction - Truncating control flow here */
            halt_baddata();
          }
          local_4b0 = local_490;
          local_4a8 = FUN_1408a9e40(local_4b0,0x9d);
          local_580 = 0;
          local_588 = 0;
          local_590 = (ulonglong)local_590._4_4_ << 0x20;
          local_598 = local_598 & 0xffffffff00000000;
          local_5a0 = 0;
          local_5a8 = 0;
          FUN_142a26280(local_4a8,0,0,1);
          FUN_140d2cb10(local_578);
        }
        else {
          FUN_140d2cb10(local_578);
        }
      }
      else {
        FUN_140d2cb10(local_578);
      }
    }
  }
  return;
}



//===========================================================
// FUN_14214c0a0 @ 14214c0a0   (2235 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14214c0a0(longlong *param_1)

{
  char cVar1;
  uint uVar2;
  int iVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined1 auStack_6d8 [32];
  undefined4 *local_6b8;
  ulonglong local_6b0;
  uint local_6a8;
  undefined4 local_6a0;
  undefined4 local_698;
  undefined4 local_690;
  undefined4 local_688;
  int local_678;
  uint local_670;
  undefined1 local_668 [8];
  longlong local_660;
  undefined8 local_658;
  int local_650;
  undefined4 local_64c;
  undefined4 local_648 [2];
  undefined1 local_640 [8];
  undefined1 local_638 [16];
  undefined4 local_628;
  undefined4 local_624;
  undefined4 local_620;
  undefined4 local_61c;
  undefined4 local_618;
  undefined4 local_614;
  undefined4 local_610;
  int local_60c;
  int local_608;
  undefined4 local_604;
  undefined4 local_600 [4];
  undefined8 local_5f0;
  undefined8 local_5e8;
  longlong local_5e0;
  longlong *local_5d8;
  longlong *local_5d0;
  code *local_5c8;
  code *local_5c0;
  undefined1 *local_5b8;
  undefined8 local_5b0;
  code *local_5a8;
  longlong local_5a0;
  undefined8 local_598;
  undefined8 local_590;
  undefined8 local_588;
  undefined1 local_580 [8];
  undefined8 local_578;
  undefined1 *local_570;
  undefined8 local_568;
  undefined8 local_560;
  undefined8 local_558;
  undefined1 local_550 [8];
  undefined1 *local_548;
  undefined8 local_540;
  undefined1 *local_538;
  longlong *local_530;
  undefined8 local_528;
  code *local_520;
  longlong *local_518;
  longlong *local_510;
  undefined8 local_508;
  undefined4 local_500;
  undefined1 local_4f8 [8];
  undefined1 local_4f0 [8];
  undefined1 local_4e8 [8];
  undefined1 local_4e0 [8];
  undefined1 local_4d8 [16];
  undefined1 local_4c8 [28];
  undefined4 local_4ac;
  undefined4 local_47c;
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_6d8;
  local_5f0 = FUN_1408f6690();
  local_650 = 0;
  local_500 = 0;
  FUN_140196ed0(local_668,&DAT_1434b2af1,0xffffffff);
  local_670 = 0;
  do {
    uVar2 = FUN_14214ecc0(param_1 + 0xa2);
    if (uVar2 <= local_670) {
      iVar3 = FUN_1404bae40(local_668);
      if (iVar3 == 0) {
        FUN_141087bc0(local_668,&DAT_143271d00);
        local_560 = FUN_1408a9e40(local_550,0xcbe);
        local_558 = local_560;
        FUN_1401a1c50(local_668,local_560);
        FUN_140199470(local_550);
        FUN_14214e950(local_4c8);
        local_4ac = 1;
        local_47c = 1;
        local_548 = local_4f0;
        local_540 = FUN_1401bb5c0(local_548,local_668);
        local_60c = FUN_142a27970(local_540,local_4c8);
        cVar1 = FUN_141087de0();
        if ((cVar1 == '\0') || (local_60c != 6)) {
          FUN_140199470(local_668);
          return;
        }
      }
      local_530 = param_1 + 0x48;
      local_538 = local_4e8;
      if (local_650 == 0) {
        local_64c = 0x1d8;
      }
      else {
        local_64c = 0xc63;
      }
      local_528 = FUN_1408a9e40(local_538,local_64c);
      local_688 = 0;
      local_690 = 0;
      local_698 = 3;
      local_6a0 = 0;
      local_6a8 = 0;
      local_6b0 = CONCAT44(local_6b0._4_4_,0xffffffff);
      local_6b8 = (undefined4 *)((ulonglong)local_6b8 & 0xffffffff00000000);
      local_608 = FUN_142a269c0(local_528,0,local_530,1);
      if (local_608 != 6) {
        FUN_140199470(local_668);
        return;
      }
      *(undefined4 *)(param_1 + 0xa0) = 1;
      FUN_142aa2010(param_1 + 0x2ce,L"trade",0);
      FUN_142aa2010(param_1 + 0x2ce,L"coin",0);
      local_520 = *(code **)(*param_1 + 0x90);
      (*local_520)(param_1,0);
      FUN_141087980(local_640);
      for (local_678 = 0; local_678 < 9; local_678 = local_678 + 1) {
        local_518 = param_1 + 0xa1;
        uVar4 = FUN_14214e8d0(local_518,local_678);
        lVar5 = FUN_141087b50(uVar4);
        if (lVar5 != 0) {
          local_510 = param_1 + 0xa1;
          uVar4 = FUN_14214e8d0(local_510,local_678);
          lVar5 = FUN_1401a19e0(uVar4);
          local_648[0] = FUN_14019a5d0(lVar5 + 0x20);
          local_508 = FUN_140caa4e0();
          local_604 = local_648[0];
          local_600[0] = FUN_1403a1f90(local_508,local_648[0]);
          uVar4 = FUN_1410879a0(local_4e0,local_648,local_600);
          FUN_141087c80(local_640,uVar4,0xffffffff);
        }
      }
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
    local_5d0 = param_1 + 0xa2;
    uVar4 = FUN_14214e8d0(local_5d0,local_670);
    FUN_140232590(local_638,uVar4);
    iVar3 = FUN_141087b40(local_638);
    if (iVar3 == 0) {
      uVar4 = FUN_141087b50(local_638);
      cVar1 = FUN_1403e9110(uVar4);
      if (cVar1 != '\0') {
        local_650 = 1;
      }
      local_660 = 0;
      local_5d8 = (longlong *)FUN_1401a19e0(local_638);
      iVar3 = (**(code **)(*local_5d8 + 0x88))(local_5d8);
      if (iVar3 == 3) {
        uVar4 = FUN_141087b50(local_638);
        local_660 = FUN_140192f80(uVar4);
      }
      uVar4 = FUN_1401a19e0(local_638);
      iVar3 = FUN_141087e10(uVar4);
      if ((iVar3 != 0) || (local_660 != 0)) {
        FUN_1402bc5e0(&local_658);
        local_5c8 = DAT_143ad5648;
        lVar5 = FUN_1401a19e0(local_638);
        iVar3 = (*local_5c8)(lVar5 + 0x40,&local_5f0);
        if (iVar3 < 1) {
LAB_14214c2d4:
          local_5b8 = local_4d8;
          local_5b0 = FUN_1408a9e40(local_5b8,0x4ac);
          local_690 = 0;
          local_698 = 0;
          local_6a0 = 0;
          local_6a8 = 0;
          local_6b0 = local_6b0 & 0xffffffff00000000;
          local_6b8 = (undefined4 *)((ulonglong)local_6b8 & 0xffffffff00000000);
          FUN_142a26280(local_5b0,0,0,1);
          local_5a8 = *(code **)(*param_1 + 0x138);
          (*local_5a8)(param_1,2);
          FUN_140199470(&local_658);
          FUN_1401ab4e0(local_638);
          FUN_140199470(local_668);
          return;
        }
        if (local_660 != 0) {
          local_5c0 = DAT_143ad5648;
          iVar3 = (*DAT_143ad5648)(local_660 + 0x82,&local_5f0);
          if ((iVar3 < 1) || (iVar3 = FUN_1402cf680(local_660,0), iVar3 != 0)) goto LAB_14214c2d4;
        }
        if (local_660 == 0) {
          local_5e0 = FUN_1401a19e0(local_638);
          local_5e0 = local_5e0 + 0x40;
        }
        else {
          local_5e0 = local_660 + 0x82;
        }
        local_5a0 = local_5e0;
        local_6a8 = local_6a8 & 0xffffff00;
        local_6b0 = 0;
        local_6b8 = &local_628;
        FUN_1408f6cc0(local_5e0,&local_5f0,&local_620,&local_624);
        local_61c = local_628;
        local_618 = local_624;
        local_614 = local_620;
        local_598 = FUN_1408a9e40(local_580,0xcbc);
        local_590 = local_598;
        local_588 = FUN_140c2f770(local_598);
        local_6b8 = (undefined4 *)CONCAT44(local_6b8._4_4_,local_61c);
        FUN_14019ba10(&local_658,local_588,local_614,local_618);
        FUN_140199470(local_580);
        local_578 = FUN_140caa4e0();
        lVar5 = FUN_1401a19e0(local_638);
        local_610 = FUN_14019a5d0(lVar5 + 0x20);
        FUN_140398ba0(local_578,&local_5e8,local_610);
        local_570 = local_4f8;
        local_568 = FUN_1429fbeb0(local_570,0x28);
        FUN_1429eb300(&local_5e8,local_568,100);
        FUN_14019ba10(&local_658,"%s(#Corange#%s#k)\r\n",local_5e8,local_658);
        FUN_1401a1c50(local_668,&local_658);
        FUN_140199470(&local_5e8);
        FUN_140199470(&local_658);
      }
      FUN_1401ab4e0(local_638);
    }
    else {
      FUN_1401ab4e0(local_638);
    }
    local_670 = local_670 + 1;
  } while( true );
}



//===========================================================
// FUN_14214ca90 @ 14214ca90   (2131 bytes)
//===========================================================

void FUN_14214ca90(longlong param_1,longlong *param_2)

{
  int iVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong *plVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  undefined4 uVar8;
  int iVar9;
  undefined4 uVar10;
  uint uVar11;
  longlong *plVar12;
  longlong lVar13;
  undefined8 *puVar14;
  longlong *plVar15;
  longlong lVar16;
  longlong *plVar17;
  int iVar18;
  undefined4 uVar19;
  uint uVar20;
  ulonglong uVar21;
  undefined2 local_res18;
  undefined1 local_res1a;
  int local_res20;
  longlong **in_stack_fffffffffffffe58;
  undefined8 uVar22;
  ulonglong in_stack_fffffffffffffe60;
  ulonglong uVar23;
  uint in_stack_fffffffffffffe70;
  ulonglong uVar24;
  int local_120;
  longlong local_118;
  longlong *local_100;
  longlong *local_f8;
  longlong *local_f0;
  longlong *local_e8;
  undefined4 local_e0;
  longlong *local_d8;
  longlong *local_d0;
  undefined8 local_c8;
  longlong *local_c0;
  longlong *local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  longlong local_a0;
  longlong **local_98;
  longlong *local_90;
  undefined1 local_88 [8];
  longlong local_80;
  longlong local_70;
  undefined1 local_68 [40];
  
  FUN_14090ead0(&local_f8,DAT_143ad6eb8);
  if (local_f8 == (longlong *)0x0) {
    param_2 = (longlong *)*param_2;
  }
  else {
    local_d8 = local_f8;
    (**(code **)(*local_f8 + 8))();
    iVar5 = FUN_140910ca0(&local_d8,"itemGridWidth",0);
    local_d0 = local_f8;
    if (local_f8 != (longlong *)0x0) {
      (**(code **)(*local_f8 + 8))();
    }
    iVar6 = FUN_140910ca0(&local_d0,"itemGridHeight",0);
    uVar21 = 0;
    local_a0 = 0;
    do {
      lVar3 = local_a0;
      local_c8 = 0;
      FUN_141adbce0(param_1 + 0x1670,&local_a8,L"myItem",&local_c8);
      plVar17 = (longlong *)0x0;
      local_100 = (longlong *)0x0;
      local_120 = 0;
      local_118 = 0;
      iVar1 = (int)(uVar21 / 3);
      uVar20 = (uint)uVar21;
      do {
        local_res20 = (uVar20 + iVar1 * -3) * iVar5 + (int)local_a8;
        iVar18 = (int)((ulonglong)local_a8 >> 0x20) + iVar6 * iVar1;
        lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
        if (lVar13 == 0) {
          uVar7 = 0;
        }
        else {
          uVar7 = *(uint *)(lVar13 + -8);
        }
        if (uVar7 <= uVar20) {
          if (lVar13 == 0) {
            uVar10 = 0;
          }
          else {
            uVar10 = *(undefined4 *)(lVar13 + -8);
          }
          FUN_142e54290(0xbc,uVar21,uVar10);
          lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
        }
        uVar2 = DAT_143aa8328;
        if (*(longlong *)(lVar13 + 8 + lVar3 * 0x18) != 0) {
          if (lVar13 == 0) {
            uVar7 = 0;
          }
          else {
            uVar7 = *(uint *)(lVar13 + -8);
          }
          if (uVar7 <= uVar20) {
            if (lVar13 == 0) {
              uVar10 = 0;
            }
            else {
              uVar10 = *(undefined4 *)(lVar13 + -8);
            }
            FUN_142e54290(0xbc,uVar21,uVar10);
            lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
          }
          plVar15 = *(longlong **)(lVar13 + 8 + lVar3 * 0x18);
          if (plVar15 == (longlong *)0x0) {
            FUN_142e52ed0(0x431,0);
            plVar15 = *(longlong **)(lVar13 + 8 + lVar3 * 0x18);
          }
          uVar8 = (**(code **)(*plVar15 + 0x180))();
          uVar10 = 0;
          uVar24 = 0;
          in_stack_fffffffffffffe70 = in_stack_fffffffffffffe70 & 0xffffff00;
          uVar23 = 0;
          in_stack_fffffffffffffe60 = in_stack_fffffffffffffe60 & 0xffffffffffffff00;
          in_stack_fffffffffffffe58 =
               (longlong **)((ulonglong)in_stack_fffffffffffffe58 & 0xffffffff00000000);
          plVar12 = (longlong *)
                    FUN_1403a0810(uVar2,&local_90,uVar8,1,in_stack_fffffffffffffe58,
                                  in_stack_fffffffffffffe60,0,in_stack_fffffffffffffe70,0xff,0,0);
          plVar15 = (longlong *)*plVar12;
          plVar4 = plVar17;
          if ((plVar17 != plVar15) &&
             (*plVar12 = 0, plVar4 = plVar15, local_100 = plVar15, plVar17 != (longlong *)0x0)) {
            (**(code **)(*plVar17 + 0x10))();
          }
          plVar17 = plVar4;
          if (local_90 != (longlong *)0x0) {
            (**(code **)(*local_90 + 0x10))();
          }
          uVar2 = DAT_143aa8328;
          uVar8 = (undefined4)((ulonglong)in_stack_fffffffffffffe58 >> 0x20);
          if (plVar17 != (longlong *)0x0) {
            lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            uVar7 = 0;
            if (lVar13 != 0) {
              uVar7 = *(uint *)(lVar13 + -8);
            }
            if (uVar7 <= uVar20) {
              if (lVar13 != 0) {
                uVar10 = *(undefined4 *)(lVar13 + -8);
              }
              FUN_142e54290(0xbc,uVar21,uVar10);
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            }
            uVar10 = (undefined4)(in_stack_fffffffffffffe60 >> 0x20);
            lVar13 = *(longlong *)(lVar13 + 8 + lVar3 * 0x18);
            local_80 = lVar13;
            if (lVar13 != 0) {
              if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
              UNLOCK();
              plVar17 = local_100;
            }
            local_e0 = FUN_140396c50(uVar2,local_88);
            lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            if (lVar13 == 0) {
              uVar7 = 0;
            }
            else {
              uVar7 = *(uint *)(lVar13 + -8);
            }
            if (uVar7 <= uVar20) {
              if (lVar13 == 0) {
                uVar19 = 0;
              }
              else {
                uVar19 = *(undefined4 *)(lVar13 + -8);
              }
              FUN_142e54290(0xbc,uVar21,uVar19);
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            }
            plVar15 = *(longlong **)(lVar13 + 8 + lVar3 * 0x18);
            if (plVar15 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
              plVar15 = *(longlong **)(lVar13 + 8 + lVar3 * 0x18);
            }
            iVar9 = (**(code **)(*plVar15 + 0x88))();
            if (iVar9 == 1) {
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
              if (lVar13 == 0) {
                uVar7 = 0;
              }
              else {
                uVar7 = *(uint *)(lVar13 + -8);
              }
              if (uVar7 <= uVar20) {
                if (lVar13 == 0) {
                  uVar19 = 0;
                }
                else {
                  uVar19 = *(undefined4 *)(lVar13 + -8);
                }
                FUN_142e54290(0xbc,uVar21,uVar19);
              }
              lVar13 = FUN_140192f00();
              uVar2 = DAT_143aa8328;
              uVar19 = (undefined4)(uVar23 >> 0x20);
              if (lVar13 != 0) {
                local_f0 = (longlong *)*param_2;
                if (local_f0 != (longlong *)0x0) {
                  (**(code **)(*local_f0 + 8))();
                }
                uVar23 = CONCAT44(uVar19,iVar18 + 2);
                uVar22 = CONCAT44(uVar8,iVar18 + -0x21);
                FUN_1403d3580(uVar2,&local_f0,lVar13,(local_res20 - (uint)(local_120 == 0)) + -2,
                              uVar22,CONCAT44(uVar10,local_res20 + 0x1f),uVar23);
                uVar8 = (undefined4)((ulonglong)uVar22 >> 0x20);
              }
            }
            uVar2 = DAT_143aa8328;
            local_res18 = 1;
            local_res1a = 0;
            lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            if (lVar13 == 0) {
              uVar7 = 0;
            }
            else {
              uVar7 = *(uint *)(lVar13 + -8);
            }
            if (uVar7 <= uVar20) {
              if (lVar13 == 0) {
                uVar10 = 0;
              }
              else {
                uVar10 = *(undefined4 *)(lVar13 + -8);
              }
              FUN_142e54290(0xbc,uVar21,uVar10);
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            }
            lVar13 = *(longlong *)(lVar13 + 8 + lVar3 * 0x18);
            local_70 = lVar13;
            if (lVar13 != 0) {
              if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
              UNLOCK();
              plVar17 = local_100;
            }
            in_stack_fffffffffffffe70 = FUN_1403e5750();
            lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            if (lVar13 == 0) {
              uVar7 = 0;
            }
            else {
              uVar7 = *(uint *)(lVar13 + -8);
            }
            if (uVar7 <= uVar20) {
              if (lVar13 == 0) {
                uVar10 = 0;
              }
              else {
                uVar10 = *(undefined4 *)(lVar13 + -8);
              }
              FUN_142e54290(0xbc,uVar21,uVar10);
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            }
            in_stack_fffffffffffffe60 = *(ulonglong *)(lVar13 + 8 + lVar3 * 0x18);
            if (lVar13 == 0) {
              uVar7 = 0;
            }
            else {
              uVar7 = *(uint *)(lVar13 + -8);
            }
            if (uVar7 <= uVar20) {
              if (lVar13 == 0) {
                uVar10 = 0;
              }
              else {
                uVar10 = *(undefined4 *)(lVar13 + -8);
              }
              FUN_142e54290(0xbc,uVar21,uVar10);
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            }
            lVar16 = *(longlong *)(lVar13 + 8 + lVar3 * 0x18);
            if (lVar16 == 0) {
              FUN_142e52ed0(0x431,0);
              lVar16 = *(longlong *)(lVar13 + 8 + lVar3 * 0x18);
            }
            uVar10 = FUN_14019a5d0(lVar16 + 0x20);
            local_c0 = (longlong *)*param_2;
            if (local_c0 != (longlong *)0x0) {
              (**(code **)(*local_c0 + 8))();
            }
            uVar7 = 0;
            uVar23 = uVar23 & 0xffffffff00000000;
            in_stack_fffffffffffffe58 = (longlong **)CONCAT44(uVar8,iVar18);
            FUN_1403d37d0(uVar2,&local_c0,uVar10,local_res20,in_stack_fffffffffffffe58,
                          in_stack_fffffffffffffe60,uVar23,in_stack_fffffffffffffe70,0,local_e0,
                          uVar24 & 0xffffffff00000000,0,0,0xff,0,0,0,1,&local_res18);
            lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            if (lVar13 == 0) {
              uVar11 = 0;
            }
            else {
              uVar11 = *(uint *)(lVar13 + -8);
            }
            if (uVar11 <= uVar20) {
              if (lVar13 == 0) {
                uVar10 = 0;
              }
              else {
                uVar10 = *(undefined4 *)(lVar13 + -8);
              }
              FUN_142e54290(0xbc,uVar21,uVar10);
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            }
            lVar16 = *(longlong *)(lVar13 + 8 + lVar3 * 0x18);
            if (lVar16 == 0) {
              FUN_142e52ed0(0x431,0);
              lVar16 = *(longlong *)(lVar13 + 8 + lVar3 * 0x18);
            }
            iVar9 = FUN_14019a5d0(lVar16 + 0x20);
            if (((999999 < iVar9 - 2000000U) && (999999 < iVar9 - 3000000U)) &&
               (999999 < iVar9 - 4000000U)) {
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
              if (lVar13 == 0) {
                uVar11 = 0;
              }
              else {
                uVar11 = *(uint *)(lVar13 + -8);
              }
              if (uVar11 <= uVar20) {
                if (lVar13 == 0) {
                  uVar10 = 0;
                }
                else {
                  uVar10 = *(undefined4 *)(lVar13 + -8);
                }
                FUN_142e54290(0xbc,uVar21,uVar10);
                lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
              }
              lVar16 = *(longlong *)(lVar13 + 8 + lVar3 * 0x18);
              if (lVar16 == 0) {
                FUN_142e52ed0(0x431,0);
                lVar16 = *(longlong *)(lVar13 + 8 + lVar3 * 0x18);
              }
              uVar10 = FUN_14019a5d0(lVar16 + 0x20);
              iVar9 = FUN_140419af0(uVar10);
              if (iVar9 == 0) goto LAB_14214d225;
            }
            local_98 = &local_b8;
            local_b8 = *(longlong **)(param_1 + 0x4f8);
            if (local_b8 != (longlong *)0x0) {
              (**(code **)(*local_b8 + 8))();
            }
            lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            if (lVar13 == 0) {
              uVar11 = 0;
            }
            else {
              uVar11 = *(uint *)(lVar13 + -8);
            }
            if (uVar11 <= uVar20) {
              if (lVar13 == 0) {
                uVar10 = 0;
              }
              else {
                uVar10 = *(undefined4 *)(lVar13 + -8);
              }
              FUN_142e54290(0xbc,uVar21,uVar10);
              lVar13 = *(longlong *)(param_1 + 0x508 + local_118 * 8);
            }
            uVar10 = *(undefined4 *)(lVar13 + 0x10 + lVar3 * 0x18);
            local_e8 = (longlong *)*param_2;
            if (local_e8 != (longlong *)0x0) {
              (**(code **)(*local_e8 + 8))();
            }
            in_stack_fffffffffffffe70 = CONCAT31((int3)(in_stack_fffffffffffffe70 >> 8),1);
            in_stack_fffffffffffffe60 = in_stack_fffffffffffffe60 & 0xffffffff00000000;
            in_stack_fffffffffffffe58 = &local_b8;
            FUN_1429e7830(&local_e8,local_res20,iVar18 + -0xc,uVar10,in_stack_fffffffffffffe58,
                          in_stack_fffffffffffffe60,uVar23 & 0xffffffff00000000,
                          in_stack_fffffffffffffe70,uVar7 & 0xffffff00);
          }
        }
LAB_14214d225:
        local_b0 = 0;
        puVar14 = (undefined8 *)FUN_141adbce0(param_1 + 0x1670,local_68,L"otherItem",&local_b0);
        local_a8 = *puVar14;
        local_120 = local_120 + 1;
        local_118 = local_118 + 1;
      } while (local_118 < 2);
      if (plVar17 != (longlong *)0x0) {
        (**(code **)(*plVar17 + 0x10))(plVar17);
      }
      uVar21 = (ulonglong)(uVar20 + 1);
      local_a0 = local_a0 + 1;
    } while ((int)(uVar20 + 1) < 9);
    if (local_f8 != (longlong *)0x0) {
      (**(code **)(*local_f8 + 0x10))();
    }
    param_2 = (longlong *)*param_2;
  }
  if (param_2 != (longlong *)0x0) {
    (**(code **)(*param_2 + 0x10))();
  }
  return;
}



//===========================================================
// FUN_14214d2f0 @ 14214d2f0   (423 bytes)
//===========================================================

uint FUN_14214d2f0(longlong param_1,int param_2,int param_3)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  longlong *local_res20;
  longlong *local_48;
  undefined8 local_40;
  int local_38;
  int iStack_34;
  int local_30;
  int iStack_2c;
  
  FUN_14090ead0(&local_48,DAT_143ad6eb8);
  if (local_48 == (longlong *)0x0) {
    uVar5 = 0;
  }
  else {
    local_res20 = local_48;
    (**(code **)(*local_48 + 8))();
    iVar1 = FUN_140910ca0(&local_res20,"itemGridWidth",0);
    local_res20 = local_48;
    if (local_48 != (longlong *)0x0) {
      (**(code **)(*local_48 + 8))();
    }
    iVar2 = FUN_140910ca0(&local_res20,"itemGridHeight",0);
    local_res20 = (longlong *)CONCAT44(local_res20._4_4_,iVar2);
    uVar5 = 0;
    local_40 = 0;
    FUN_141adbce0(param_1 + 0x1670,&local_38,L"otherItemGrid_lt",&local_40);
    local_40 = 0;
    FUN_141adbce0(param_1 + 0x1670,&local_30,L"myItemGrid_lt",&local_40);
    uVar6 = uVar5;
    while( true ) {
      iVar3 = (int)uVar6 / 3 + ((int)uVar6 >> 0x1f) +
              (int)(((longlong)(int)uVar6 / 3 + ((longlong)(int)uVar6 >> 0x3f) & 0xffffffffU) >>
                   0x1f);
      iVar2 = iVar3 * iVar2;
      iVar7 = iStack_2c + iVar2;
      iVar4 = iVar1 * (uVar6 + iVar3 * -3);
      iVar3 = iVar4 + local_30;
      if ((((iVar3 <= param_2) && (param_2 < iVar3 + 0x20)) && (iVar7 <= param_3)) &&
         (param_3 < iVar7 + 0x20)) break;
      iVar2 = iVar2 + iStack_34;
      iVar4 = iVar4 + local_38;
      if (((iVar4 <= param_2) && (param_2 < iVar4 + 0x20)) &&
         ((iVar2 <= param_3 && (param_3 < iVar2 + 0x20)))) {
        uVar5 = ~uVar6;
        goto LAB_14214d469;
      }
      uVar6 = uVar6 + 1;
      if (8 < (int)uVar6) goto LAB_14214d469;
      iVar2 = (int)local_res20;
    }
    uVar5 = uVar6 + 1;
  }
LAB_14214d469:
  if (local_48 != (longlong *)0x0) {
    (**(code **)(*local_48 + 0x10))();
  }
  return uVar5;
}



//===========================================================
// FUN_14214d4a0 @ 14214d4a0   (523 bytes)
//===========================================================

undefined8 FUN_14214d4a0(longlong param_1,int param_2,int param_3)

{
  longlong lVar1;
  int iVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined8 uVar5;
  uint uVar6;
  undefined8 local_res8;
  int local_res20;
  int local_res24;
  int local_68;
  int local_64;
  int local_60;
  int local_5c;
  int local_58;
  int local_54;
  int local_50;
  int local_4c;
  int local_48;
  int local_44;
  int local_40;
  int local_3c;
  int local_38;
  int local_34;
  
  lVar1 = param_1 + 0x1670;
  uVar6 = 0;
  local_res8 = 0;
  FUN_141adbce0(lVar1,&local_res20,L"myMeso_lt",&local_res8);
  local_res8 = 0;
  FUN_141adbce0(lVar1,&local_68,L"myMeso_rb",&local_res8);
  if ((((local_res20 <= param_2) && (param_2 < local_68)) && (local_res24 <= param_3)) &&
     (param_3 < local_64)) {
    puVar3 = *(undefined8 **)(param_1 + 0x530);
    if ((puVar3 == (undefined8 *)0x0) || (*(int *)(puVar3 + -1) == 0)) {
      FUN_142e54290(0xbc,0,0);
      puVar3 = *(undefined8 **)(param_1 + 0x530);
    }
    if (((char *)*puVar3 != (char *)0x0) && (*(char *)*puVar3 != '\0')) {
      return 1;
    }
  }
  local_res8 = 0;
  FUN_141adbce0(lVar1,&local_60,L"otherMeso_lt",&local_res8);
  local_res8 = 0;
  FUN_141adbce0(lVar1,&local_58,L"otherMeso_rb",&local_res8);
  if ((((local_60 <= param_2) && (param_2 < local_58)) && (local_5c <= param_3)) &&
     (param_3 < local_54)) {
    lVar4 = *(longlong *)(param_1 + 0x530);
    if ((lVar4 == 0) || (uVar6 = *(uint *)(lVar4 + -8), uVar6 < 2)) {
      FUN_142e54290(0xbc,1,uVar6);
      lVar4 = *(longlong *)(param_1 + 0x530);
    }
    if ((*(char **)(lVar4 + 8) != (char *)0x0) && (**(char **)(lVar4 + 8) != '\0')) {
      return 2;
    }
  }
  local_res8 = 0;
  FUN_141adbce0(lVar1,&local_50,L"myName_lt",&local_res8);
  local_res8 = 0;
  FUN_141adbce0(lVar1,&local_48,L"myName_rb",&local_res8);
  if ((((param_2 < local_50) || (local_48 <= param_2)) || (param_3 < local_4c)) ||
     (local_44 <= param_3)) {
    local_res8 = 0;
    FUN_141adbce0(lVar1,&local_40,L"otherName_lt",&local_res8);
    local_res8 = 0;
    FUN_141adbce0(lVar1,&local_38,L"otherName_rb",&local_res8);
    if (((param_2 < local_40) || (local_38 <= param_2)) ||
       ((param_3 < local_3c ||
        ((local_34 <= param_3 || (iVar2 = FUN_141c3f8a0(param_1), iVar2 < 2)))))) {
      uVar5 = 0;
    }
    else {
      uVar5 = 4;
    }
  }
  else {
    uVar5 = 3;
  }
  return uVar5;
}



//===========================================================
// FUN_14214d6c0 @ 14214d6c0   (195 bytes)
//===========================================================

int FUN_14214d6c0(longlong param_1)

{
  uint uVar1;
  longlong lVar2;
  uint uVar3;
  longlong lVar4;
  undefined4 uVar5;
  
  lVar2 = *(longlong *)(param_1 + 0x508);
  uVar3 = 0;
  lVar4 = 0;
  do {
    if ((lVar2 == 0) || (*(uint *)(lVar2 + -8) <= uVar3)) {
      return -1;
    }
    if ((int)uVar3 < 0) {
      FUN_142e54290(0xbc,uVar3);
      lVar2 = *(longlong *)(param_1 + 0x508);
    }
    if (*(longlong *)(lVar4 + 8 + lVar2) == 0) {
      if (lVar2 == 0) {
        uVar1 = 0;
      }
      else {
        uVar1 = *(uint *)(lVar2 + -8);
      }
      if (((int)uVar3 < 0) || (uVar1 <= uVar3)) {
        if (lVar2 == 0) {
          uVar5 = 0;
        }
        else {
          uVar5 = *(undefined4 *)(lVar2 + -8);
        }
        FUN_142e54290(0xbc,uVar3,uVar5);
        lVar2 = *(longlong *)(param_1 + 0x508);
      }
      if (*(int *)(lVar4 + 0x10 + lVar2) < 0) {
        return uVar3 + 1;
      }
    }
    uVar3 = uVar3 + 1;
    lVar4 = lVar4 + 0x18;
  } while( true );
}



//===========================================================
// FUN_14214d790 @ 14214d790   (3990 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014214e5a1) */

void FUN_14214d790(longlong param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  IUnknown *pIVar3;
  undefined *puVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  int iVar7;
  uint uVar8;
  undefined8 *puVar9;
  int *piVar10;
  longlong *plVar11;
  int *piVar12;
  longlong lVar13;
  undefined8 uVar14;
  undefined8 *puVar15;
  longlong *plVar16;
  int *piVar17;
  uint uVar18;
  longlong lVar19;
  int *piVar20;
  ulonglong local_res10;
  longlong *local_res18;
  longlong *local_res20;
  uint *puVar21;
  longlong *local_288;
  undefined4 local_280;
  undefined4 uStack_27c;
  undefined8 uStack_278;
  undefined8 local_270;
  short local_268;
  undefined6 uStack_266;
  int *piStack_260;
  undefined8 local_258;
  short local_250;
  undefined6 uStack_24e;
  int *piStack_248;
  undefined8 local_240;
  undefined4 local_238;
  undefined4 uStack_234;
  undefined8 uStack_230;
  undefined8 local_228;
  short local_220;
  undefined6 uStack_21e;
  int *piStack_218;
  undefined8 local_210;
  short local_208;
  undefined6 uStack_206;
  int *piStack_200;
  undefined8 local_1f8;
  uint local_1f0;
  undefined4 uStack_1ec;
  undefined8 uStack_1e8;
  undefined8 local_1e0;
  uint local_1d8;
  undefined4 uStack_1d4;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  longlong *local_1c0;
  longlong *local_1b8;
  undefined1 local_1b0 [8];
  undefined1 local_1a8 [8];
  undefined1 local_1a0 [8];
  undefined1 local_198 [8];
  int local_190 [2];
  int local_188 [6];
  longlong *local_170;
  undefined8 local_168;
  int *piStack_160;
  undefined8 local_158;
  uint local_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined8 local_138;
  undefined8 local_128;
  int *piStack_120;
  undefined8 local_118;
  uint local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 local_f8;
  undefined8 local_e8;
  int *piStack_e0;
  undefined8 local_d8;
  uint local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 local_b8;
  undefined8 local_a8;
  int *piStack_a0;
  undefined8 local_98;
  uint local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_78;
  undefined1 local_68 [40];
  
  FUN_14090ead0(&local_res20,DAT_143ad6eb8);
  if (local_res20 != (longlong *)0x0) {
    local_1c0 = local_res20;
    (**(code **)(*local_res20 + 8))();
    uVar5 = FUN_140910ca0(&local_1c0,"colorWarningNormal",0);
    local_res18 = (longlong *)CONCAT44(local_res18._4_4_,uVar5);
    local_1b8 = local_res20;
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 8))();
    }
    uVar6 = FUN_140910ca0(&local_1b8,"colorWarningHighlight",0);
    local_res10 = CONCAT44(local_res10._4_4_,uVar6);
    plVar1 = (longlong *)(param_1 + 0x1660);
    FUN_140da2090(plVar1);
    puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x28);
    piVar12 = (int *)0x0;
    piVar17 = piVar12;
    piVar10 = piVar12;
    if (puVar9 == (undefined8 *)0x0) {
      *plVar1 = 0;
    }
    else {
      *plVar1 = (longlong)(puVar9 + 1);
      if (puVar9 + 1 != (undefined8 *)0x0) {
        *puVar9 = 4;
        puVar15 = (undefined8 *)*plVar1;
        puVar9 = puVar15 + 4;
        for (; puVar15 < puVar9; puVar15 = puVar15 + 1) {
          *puVar15 = 0;
        }
      }
    }
    do {
      lVar13 = *plVar1;
      uVar8 = 0;
      if (lVar13 != 0) {
        uVar8 = *(uint *)(lVar13 + -8);
      }
      uVar18 = (uint)piVar17;
      if (((int)uVar18 < 0) || (uVar8 <= uVar18)) {
        piVar20 = piVar12;
        if (lVar13 != 0) {
          piVar20 = (int *)(ulonglong)*(uint *)(lVar13 + -8);
        }
        FUN_142e54290(0xbc,piVar17,piVar20);
        lVar13 = *plVar1;
      }
      puVar4 = PTR_u_Canvas_Font_143a47cc0;
      plVar11 = (longlong *)((longlong)piVar10 + lVar13);
      plVar2 = (longlong *)*plVar11;
      plVar16 = (longlong *)0x0;
      if (plVar2 != (longlong *)0x0) {
        *plVar11 = 0;
        (**(code **)(*plVar2 + 0x10))();
        plVar16 = (longlong *)*plVar11;
      }
      if (plVar16 != (longlong *)0x0) {
        (**(code **)(*plVar16 + 0x10))();
      }
      *plVar11 = 0;
      if (DAT_143ad48a0 == (code *)0x0) {
        iVar7 = -0x7ffbfe10;
      }
      else {
        iVar7 = (*DAT_143ad48a0)(puVar4,&DAT_143297250,plVar11,0);
      }
      if (iVar7 < 0) {
        FUN_1401a59c0(local_68,iVar7,0,0);
                    /* WARNING: Subroutine does not return */
        _CxxThrowException(local_68,(ThrowInfo *)&DAT_143a3b0c0);
      }
      piVar17 = (int *)(ulonglong)(uVar18 + 1);
      piVar10 = piVar10 + 2;
    } while ((int)(uVar18 + 1) < 4);
    puVar9 = (undefined8 *)*plVar1;
    piVar17 = piVar12;
    if ((puVar9 == (undefined8 *)0x0) ||
       (piVar17 = (int *)(ulonglong)*(uint *)(puVar9 + -1), *(uint *)(puVar9 + -1) == 0)) {
      FUN_142e54290(0xbc,0,piVar17);
      puVar9 = (undefined8 *)*plVar1;
    }
    pIVar3 = (IUnknown *)*puVar9;
    if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_268);
    if (DAT_143a8b8d8 == 8) {
      if (local_268 == 8) {
        local_268 = 0;
        if (piStack_260 != (int *)0x0) {
          (*DAT_143ad5990)(piStack_260 + -1);
        }
      }
      else {
        iVar7 = (*DAT_143262a18)(&local_268);
        if (iVar7 < 0) goto LAB_14214e698;
      }
      lVar13 = DAT_143a8b8e0;
      local_268 = 8;
      piVar17 = piVar12;
      if (DAT_143a8b8e0 != 0) {
        piVar17 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      piVar10 = (int *)(*DAT_143ad5980)((ulonglong)((int)piVar17 + 1) * 2 + 4);
      piStack_260 = piVar12;
      if (piVar10 != (int *)0x0) {
        *piVar10 = (int)piVar17 * 2;
        piVar10 = piVar10 + 1;
        if (lVar13 != 0) {
          FUN_142ef7ba0(piVar10,lVar13,(longlong)piVar17 * 2);
        }
        *(undefined2 *)((longlong)piVar10 + (longlong)piVar17 * 2) = 0;
        piStack_260 = piVar10;
      }
    }
    else {
      if ((local_268 == 8) && (local_268 = 0, piStack_260 != (int *)0x0)) {
        (*DAT_143ad5990)(piStack_260 + -1);
      }
      iVar7 = (*DAT_143262a28)(&local_268,&DAT_143a8b8d8);
      if (iVar7 < 0) {
LAB_14214e698:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
    }
    (*DAT_143262a20)(&local_280);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_280 == 8) {
        local_280 = (uint)local_280._2_2_ << 0x10;
        if (uStack_278 != (int *)0x0) {
          (*DAT_143ad5990)(uStack_278 + -1);
        }
      }
      else {
        iVar7 = (*DAT_143262a18)(&local_280);
        if (iVar7 < 0) goto LAB_14214e6a0;
      }
      lVar13 = DAT_143a8b8e0;
      local_280 = CONCAT22(local_280._2_2_,8);
      piVar17 = piVar12;
      if (DAT_143a8b8e0 != 0) {
        piVar17 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      piVar10 = (int *)(*DAT_143ad5980)((ulonglong)((int)piVar17 + 1) * 2 + 4);
      uStack_278 = piVar12;
      if (piVar10 != (int *)0x0) {
        *piVar10 = (int)piVar17 * 2;
        piVar10 = piVar10 + 1;
        if (lVar13 != 0) {
          FUN_142ef7ba0(piVar10,lVar13,(longlong)piVar17 * 2);
        }
        *(undefined2 *)((longlong)piVar10 + (longlong)piVar17 * 2) = 0;
        uStack_278 = piVar10;
      }
    }
    else {
      if (((short)local_280 == 8) &&
         (local_280 = (uint)local_280._2_2_ << 0x10, uStack_278 != (int *)0x0)) {
        (*DAT_143ad5990)(uStack_278 + -1);
      }
      iVar7 = (*DAT_143262a28)(&local_280,&DAT_143a8b8d8);
      if (iVar7 < 0) {
LAB_14214e6a0:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
    }
    plVar11 = (longlong *)FUN_1408a9d80(local_1b0,0x1800);
    piVar17 = piVar12;
    if ((undefined8 *)*plVar11 != (undefined8 *)0x0) {
      piVar17 = *(int **)*plVar11;
    }
    local_168 = CONCAT62(uStack_266,local_268);
    piStack_160 = piStack_260;
    local_158 = local_258;
    local_148 = local_280;
    uStack_144 = uStack_27c;
    uStack_140 = (undefined4)uStack_278;
    uStack_13c = uStack_278._4_4_;
    local_138 = local_270;
    local_288 = plVar11;
    iVar7 = (**(code **)(*(longlong *)pIVar3 + 0x18))
                      (pIVar3,piVar17,0xc,uVar5,&local_148,&local_168);
    if (iVar7 < 0) {
      _com_issue_errorex(iVar7,pIVar3,(_GUID *)&DAT_143297250);
    }
    FUN_1401be120(plVar11);
    if ((short)local_280 == 8) {
      local_280 = local_280 & 0xffff0000;
      if (uStack_278 != (int *)0x0) {
        (*DAT_143ad5990)(uStack_278 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_280);
    }
    if (local_268 == 8) {
      local_268 = 0;
      if (piStack_260 != (int *)0x0) {
        (*DAT_143ad5990)(piStack_260 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_268);
    }
    lVar13 = *plVar1;
    piVar17 = piVar12;
    if ((lVar13 == 0) ||
       (piVar17 = (int *)(ulonglong)*(uint *)(lVar13 + -8), *(uint *)(lVar13 + -8) < 2)) {
      FUN_142e54290(0xbc,1,piVar17);
      lVar13 = *plVar1;
    }
    pIVar3 = *(IUnknown **)(lVar13 + 8);
    if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_250);
    if (DAT_143a8b8d8 == 8) {
      if (local_250 == 8) {
        local_250 = 0;
        if (piStack_248 != (int *)0x0) {
          (*DAT_143ad5990)(piStack_248 + -1);
        }
      }
      else {
        iVar7 = (*DAT_143262a18)(&local_250);
        if (iVar7 < 0) goto LAB_14214e6a8;
      }
      lVar13 = DAT_143a8b8e0;
      local_250 = 8;
      piVar17 = piVar12;
      if (DAT_143a8b8e0 != 0) {
        piVar17 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      piVar10 = (int *)(*DAT_143ad5980)((ulonglong)((int)piVar17 + 1) * 2 + 4);
      piStack_248 = piVar12;
      if (piVar10 != (int *)0x0) {
        *piVar10 = (int)piVar17 * 2;
        piVar10 = piVar10 + 1;
        if (lVar13 != 0) {
          FUN_142ef7ba0(piVar10,lVar13,(longlong)piVar17 * 2);
        }
        *(undefined2 *)((longlong)piVar10 + (longlong)piVar17 * 2) = 0;
        piStack_248 = piVar10;
      }
    }
    else {
      if ((local_250 == 8) && (local_250 = 0, piStack_248 != (int *)0x0)) {
        (*DAT_143ad5990)(piStack_248 + -1);
      }
      iVar7 = (*DAT_143262a28)(&local_250,&DAT_143a8b8d8);
      if (iVar7 < 0) {
LAB_14214e6a8:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
    }
    puVar4 = PTR_DAT_143a47cc8;
    local_1f0 = CONCAT22(local_1f0._2_2_,8);
    lVar19 = -1;
    lVar13 = lVar19;
    if (PTR_DAT_143a47cc8 == (undefined *)0x0) {
      uStack_1e8 = (int *)PTR_DAT_143a47cc8;
    }
    else {
      do {
        lVar13 = lVar13 + 1;
      } while (*(short *)(PTR_DAT_143a47cc8 + lVar13 * 2) != 0);
      uVar8 = (int)lVar13 + 1;
      uStack_1e8 = (int *)(*DAT_143ad5980)((ulonglong)uVar8 * 2 + 4);
      if (uStack_1e8 == (int *)0x0) {
        uStack_1e8 = (int *)0x0;
LAB_14214e6b6:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      *uStack_1e8 = (int)lVar13 * 2;
      uStack_1e8 = uStack_1e8 + 1;
      FUN_142ef7ba0(uStack_1e8,puVar4,(ulonglong)uVar8 * 2);
      if (uStack_1e8 == (int *)0x0) goto LAB_14214e6b6;
    }
    uVar14 = 0;
    plVar11 = (longlong *)FUN_1408a9d80(local_1a8,0x1800);
    if ((undefined8 *)*plVar11 != (undefined8 *)0x0) {
      uVar14 = *(undefined8 *)*plVar11;
    }
    local_128 = CONCAT62(uStack_24e,local_250);
    piStack_120 = piStack_248;
    local_118 = local_240;
    local_108 = local_1f0;
    uStack_104 = uStack_1ec;
    uStack_100 = (undefined4)uStack_1e8;
    uStack_fc = uStack_1e8._4_4_;
    local_f8 = local_1e0;
    local_288 = plVar11;
    iVar7 = (**(code **)(*(longlong *)pIVar3 + 0x18))
                      (pIVar3,uVar14,0xc,(ulonglong)local_res18 & 0xffffffff,&local_108,&local_128);
    if (iVar7 < 0) {
      _com_issue_errorex(iVar7,pIVar3,(_GUID *)&DAT_143297250);
    }
    FUN_1401be120(plVar11);
    if ((short)local_1f0 == 8) {
      local_1f0 = local_1f0 & 0xffff0000;
      if (uStack_1e8 != (int *)0x0) {
        (*DAT_143ad5990)(uStack_1e8 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_1f0);
    }
    piVar17 = (int *)0x0;
    if (local_250 == 8) {
      local_250 = 0;
      if (piStack_248 != (int *)0x0) {
        (*DAT_143ad5990)(piStack_248 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_250);
    }
    lVar13 = *plVar1;
    piVar10 = piVar17;
    if ((lVar13 == 0) ||
       (piVar10 = (int *)(ulonglong)*(uint *)(lVar13 + -8), *(uint *)(lVar13 + -8) < 3)) {
      FUN_142e54290(0xbc,2,piVar10);
      lVar13 = *plVar1;
    }
    pIVar3 = *(IUnknown **)(lVar13 + 0x10);
    if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_220);
    if (DAT_143a8b8d8 == 8) {
      if (local_220 == 8) {
        local_220 = 0;
        if (piStack_218 != (int *)0x0) {
          (*DAT_143ad5990)(piStack_218 + -1);
        }
      }
      else {
        iVar7 = (*DAT_143262a18)(&local_220);
        if (iVar7 < 0) goto LAB_14214e6c1;
      }
      lVar13 = DAT_143a8b8e0;
      local_220 = 8;
      piVar10 = piVar17;
      if (DAT_143a8b8e0 != 0) {
        piVar10 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      piVar12 = (int *)(*DAT_143ad5980)((ulonglong)((int)piVar10 + 1) * 2 + 4);
      piStack_218 = piVar17;
      if (piVar12 != (int *)0x0) {
        *piVar12 = (int)piVar10 * 2;
        piVar12 = piVar12 + 1;
        if (lVar13 != 0) {
          FUN_142ef7ba0(piVar12,lVar13,(longlong)piVar10 * 2);
        }
        *(undefined2 *)((longlong)piVar12 + (longlong)piVar10 * 2) = 0;
        piStack_218 = piVar12;
      }
    }
    else {
      if ((local_220 == 8) && (local_220 = 0, piStack_218 != (int *)0x0)) {
        (*DAT_143ad5990)(piStack_218 + -1);
      }
      iVar7 = (*DAT_143262a28)(&local_220,&DAT_143a8b8d8);
      if (iVar7 < 0) {
LAB_14214e6c1:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
    }
    (*DAT_143262a20)(&local_238);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_238 == 8) {
        local_238 = (uint)local_238._2_2_ << 0x10;
        if (uStack_230 != (int *)0x0) {
          (*DAT_143ad5990)(uStack_230 + -1);
        }
      }
      else {
        iVar7 = (*DAT_143262a18)(&local_238);
        if (iVar7 < 0) goto LAB_14214e6c9;
      }
      lVar13 = DAT_143a8b8e0;
      local_238 = CONCAT22(local_238._2_2_,8);
      piVar10 = piVar17;
      if (DAT_143a8b8e0 != 0) {
        piVar10 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      piVar12 = (int *)(*DAT_143ad5980)((ulonglong)((int)piVar10 + 1) * 2 + 4);
      uStack_230 = piVar17;
      if (piVar12 != (int *)0x0) {
        *piVar12 = (int)piVar10 * 2;
        piVar12 = piVar12 + 1;
        if (lVar13 != 0) {
          FUN_142ef7ba0(piVar12,lVar13,(longlong)piVar10 * 2);
        }
        *(undefined2 *)((longlong)piVar12 + (longlong)piVar10 * 2) = 0;
        uStack_230 = piVar12;
      }
    }
    else {
      if (((short)local_238 == 8) &&
         (local_238 = (uint)local_238._2_2_ << 0x10, uStack_230 != (int *)0x0)) {
        (*DAT_143ad5990)(uStack_230 + -1);
      }
      iVar7 = (*DAT_143262a28)(&local_238,&DAT_143a8b8d8);
      if (iVar7 < 0) {
LAB_14214e6c9:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
    }
    plVar11 = (longlong *)FUN_1408a9d80(local_1a0,0x1800);
    if ((undefined8 *)*plVar11 != (undefined8 *)0x0) {
      piVar17 = *(int **)*plVar11;
    }
    local_e8 = CONCAT62(uStack_21e,local_220);
    piStack_e0 = piStack_218;
    local_d8 = local_210;
    local_c8 = local_238;
    uStack_c4 = uStack_234;
    uStack_c0 = (undefined4)uStack_230;
    uStack_bc = uStack_230._4_4_;
    local_b8 = local_228;
    local_res18 = plVar11;
    iVar7 = (**(code **)(*(longlong *)pIVar3 + 0x18))
                      (pIVar3,piVar17,0xc,local_res10 & 0xffffffff,&local_c8,&local_e8);
    if (iVar7 < 0) {
      _com_issue_errorex(iVar7,pIVar3,(_GUID *)&DAT_143297250);
    }
    FUN_1401be120(plVar11);
    if ((short)local_238 == 8) {
      local_238 = local_238 & 0xffff0000;
      if (uStack_230 != (int *)0x0) {
        (*DAT_143ad5990)(uStack_230 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_238);
    }
    piVar17 = (int *)0x0;
    if (local_220 == 8) {
      local_220 = 0;
      if (piStack_218 != (int *)0x0) {
        (*DAT_143ad5990)(piStack_218 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_220);
    }
    lVar13 = *plVar1;
    piVar10 = piVar17;
    if ((lVar13 == 0) ||
       (piVar10 = (int *)(ulonglong)*(uint *)(lVar13 + -8), *(uint *)(lVar13 + -8) < 4)) {
      FUN_142e54290(0xbc,3,piVar10);
      lVar13 = *plVar1;
    }
    pIVar3 = *(IUnknown **)(lVar13 + 0x18);
    if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_208);
    if (DAT_143a8b8d8 == 8) {
      if (local_208 == 8) {
        local_208 = 0;
        if (piStack_200 != (int *)0x0) {
          (*DAT_143ad5990)(piStack_200 + -1);
        }
      }
      else {
        iVar7 = (*DAT_143262a18)(&local_208);
        if (iVar7 < 0) goto LAB_14214e6d1;
      }
      lVar13 = DAT_143a8b8e0;
      local_208 = 8;
      piVar10 = piVar17;
      if (DAT_143a8b8e0 != 0) {
        piVar10 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      piVar12 = (int *)(*DAT_143ad5980)((ulonglong)((int)piVar10 + 1) * 2 + 4);
      piStack_200 = piVar17;
      if (piVar12 != (int *)0x0) {
        *piVar12 = (int)piVar10 * 2;
        piVar12 = piVar12 + 1;
        if (lVar13 != 0) {
          FUN_142ef7ba0(piVar12,lVar13,(longlong)piVar10 * 2);
        }
        *(undefined2 *)((longlong)piVar12 + (longlong)piVar10 * 2) = 0;
        piStack_200 = piVar12;
      }
    }
    else {
      if ((local_208 == 8) && (local_208 = 0, piStack_200 != (int *)0x0)) {
        (*DAT_143ad5990)(piStack_200 + -1);
      }
      iVar7 = (*DAT_143262a28)(&local_208,&DAT_143a8b8d8);
      if (iVar7 < 0) {
LAB_14214e6d1:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
    }
    puVar4 = PTR_DAT_143a47cc8;
    local_1d8 = CONCAT22(local_1d8._2_2_,8);
    if (PTR_DAT_143a47cc8 == (undefined *)0x0) {
      uStack_1d0 = (int *)0x0;
    }
    else {
      do {
        lVar19 = lVar19 + 1;
      } while (*(short *)(PTR_DAT_143a47cc8 + lVar19 * 2) != 0);
      uVar8 = (int)lVar19 + 1;
      uStack_1d0 = (int *)(*DAT_143ad5980)((ulonglong)uVar8 * 2 + 4);
      if (uStack_1d0 == (int *)0x0) {
        uStack_1d0 = (int *)0x0;
LAB_14214e6df:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      *uStack_1d0 = (int)lVar19 * 2;
      uStack_1d0 = uStack_1d0 + 1;
      FUN_142ef7ba0(uStack_1d0,puVar4,(ulonglong)uVar8 * 2);
      if (uStack_1d0 == (int *)0x0) goto LAB_14214e6df;
    }
    uVar14 = 0;
    plVar11 = (longlong *)FUN_1408a9d80(local_198,0x1800);
    if ((undefined8 *)*plVar11 != (undefined8 *)0x0) {
      uVar14 = *(undefined8 *)*plVar11;
    }
    local_a8 = CONCAT62(uStack_206,local_208);
    piStack_a0 = piStack_200;
    local_98 = local_1f8;
    local_88 = local_1d8;
    uStack_84 = uStack_1d4;
    uStack_80 = (undefined4)uStack_1d0;
    uStack_7c = uStack_1d0._4_4_;
    local_78 = local_1c8;
    puVar9 = &local_a8;
    puVar21 = &local_88;
    local_res18 = plVar11;
    iVar7 = (**(code **)(*(longlong *)pIVar3 + 0x18))
                      (pIVar3,uVar14,0xc,local_res10 & 0xffffffff,puVar21,puVar9);
    if (iVar7 < 0) {
      _com_issue_errorex(iVar7,pIVar3,(_GUID *)&DAT_143297250);
    }
    FUN_1401be120(plVar11);
    if ((short)local_1d8 == 8) {
      local_1d8 = local_1d8 & 0xffff0000;
      if (uStack_1d0 != (int *)0x0) {
        (*DAT_143ad5990)(uStack_1d0 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_1d8);
    }
    if (local_208 == 8) {
      local_208 = 0;
      if (piStack_200 != (int *)0x0) {
        (*DAT_143ad5990)(piStack_200 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_208);
    }
    local_res10 = 0;
    FUN_141adbce0(param_1 + 0x1670,local_188,L"warning_lt",&local_res10);
    local_res18 = (longlong *)0x0;
    FUN_141adbce0(param_1 + 0x1670,local_190,L"warning_rb",&local_res18);
    FUN_140da2000(param_1 + 0x1668);
    local_288 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x80);
    if (local_288 == (longlong *)0x0) {
      local_170 = (longlong *)0x0;
    }
    else {
      local_288[3] = 0;
      local_288[1] = 0;
      local_288[2] = 0;
      *local_288 = (longlong)&PTR_FUN_14336e7a8;
      local_288[7] = 0;
      local_288[8] = 0;
      local_288[9] = 0;
      local_288[10] = 0;
      local_288[0xb] = 0;
      local_288[0xc] = 0;
      local_288[0xd] = 0;
      local_288[0xe] = 0;
      local_288[0xf] = 0;
      *(int *)((longlong)local_288 + 0x24) = local_190[0] - local_188[0];
      *(undefined4 *)(local_288 + 4) = 0;
      *(undefined4 *)((longlong)local_288 + 0x2c) = 0;
      *(undefined4 *)(local_288 + 6) = 0xfffffffc;
      *(undefined4 *)(local_288 + 5) = 0;
      local_170 = local_288;
      if (local_288 != (longlong *)0x0) {
        LOCK();
        local_288[1] = local_288[1] + 1;
        UNLOCK();
      }
    }
    plVar11 = local_170;
    if (local_170 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    uVar14 = FUN_1408a9e40(&local_288,0x1db);
    FUN_142a45580(plVar11,uVar14,param_1 + 0x1668,plVar1,(ulonglong)puVar21 & 0xffffffff00000000,
                  (ulonglong)puVar9 & 0xffffffff00000000,0,1,0,0,0,0,0,0,0);
    if (plVar11 != (longlong *)0x0) {
      if (0xffffe < plVar11[1] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = plVar11 + 1;
      lVar13 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar13 == 1) {
        (**(code **)*plVar11)(plVar11,1);
      }
    }
  }
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))();
  }
  return;
}



//===========================================================
// FUN_141c3d3e0 @ 141c3d3e0   (1021 bytes)
//===========================================================

void FUN_141c3d3e0(undefined8 param_1)

{
  longlong *plVar1;
  char cVar2;
  ushort uVar3;
  int iVar4;
  longlong *plVar5;
  undefined8 *puVar6;
  ulonglong uVar7;
  longlong *plVar8;
  uint local_res10 [2];
  longlong local_res18;
  longlong local_res20;
  
  FUN_1406e9170(param_1,local_res10,4);
  switch(local_res10[0]) {
  case 3:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        plVar1 = DAT_143aa8520, iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
      cVar2 = FUN_1406e8ae0(param_1);
      uVar7 = (ulonglong)cVar2;
      local_res10[0] = (uint)cVar2;
      if ((-1 < cVar2) && (local_res10[0] < 8)) {
        plVar8 = plVar1 + (uVar7 + 0xe) * 7;
        if ((int)*plVar8 != 0) {
          local_res18 = FUN_1418039d0(0x21000003);
          puVar6 = (undefined8 *)FUN_140ce7310(&local_res20,&local_res18,local_res10);
          FUN_1418049f0(&DAT_143271f04,0x1c8,0x21000003,*puVar6);
          if (local_res20 != 0) {
            FUN_14019f2c0(local_res20 + -0x10);
          }
          uVar7 = (ulonglong)local_res10[0];
        }
        (**(code **)(*plVar1 + 0x1c8))(plVar1,uVar7,param_1);
        iVar4 = FUN_1406e8c20(param_1);
        *(int *)plVar8 = iVar4;
        plVar5 = (longlong *)FUN_1406e9050(param_1,&local_res18);
        if (plVar8[1] != 0) {
          FUN_14019f2c0();
          plVar8[1] = 0;
        }
        plVar8[1] = *plVar5;
        *plVar5 = 0;
        if (local_res18 != 0) {
          FUN_14019f2c0(local_res18 + -0x10);
        }
        uVar3 = FUN_1406e8b80(param_1);
        *(uint *)(plVar8 + 2) = (uint)uVar3;
        *(int *)(plVar1 + 0x60) = (int)plVar1[0x60] + 1;
        (**(code **)(*plVar1 + 0x180))(plVar1,local_res10[0],param_1);
        return;
      }
    }
    break;
  case 4:
    FUN_141c3d980(param_1);
    return;
  case 5:
    FUN_141c3e110(param_1);
    return;
  case 6:
    FUN_141c3e360(param_1);
    return;
  default:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
      (**(code **)(*DAT_143aa8520 + 0x178))(DAT_143aa8520,local_res10[0],param_1);
    }
    break;
  case 8:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
                    /* WARNING: Could not recover jumptable at 0x000141c3d60b. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)(*DAT_143aa8520 + 0x198))(DAT_143aa8520,param_1);
      return;
    }
    break;
  case 0xb:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        plVar1 = DAT_143aa8520, iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
      cVar2 = FUN_1406e8ae0(param_1);
      uVar7 = (ulonglong)cVar2;
      local_res10[0] = (uint)cVar2;
      if ((-1 < cVar2) && (local_res10[0] < 8)) {
        if ((int)plVar1[(uVar7 + 0xe) * 7] == 0) {
          local_res18 = FUN_1418039d0(0x21000003);
          puVar6 = (undefined8 *)FUN_140ce7310(&local_res20,&local_res18,local_res10);
          FUN_1418049f0(&DAT_143271f04,0x204,0x21000003,*puVar6);
          if (local_res20 != 0) {
            FUN_14019f2c0(local_res20 + -0x10);
          }
          uVar7 = (ulonglong)local_res10[0];
        }
        (**(code **)(*plVar1 + 0x1c8))(plVar1,uVar7,param_1);
        return;
      }
    }
    break;
  case 0xc:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
                    /* WARNING: Could not recover jumptable at 0x000141c3d740. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)(*DAT_143aa8520 + 0x168))(DAT_143aa8520,param_1);
      return;
    }
    break;
  case 0xd:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
                    /* WARNING: Could not recover jumptable at 0x000141c3d78a. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)(*DAT_143aa8520 + 0x1a0))(DAT_143aa8520,param_1);
      return;
    }
  }
  return;
}



//===========================================================
// FUN_141c3d980 @ 141c3d980   (1809 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141c3de33) */
/* WARNING: Type propagation algorithm not settling */

void FUN_141c3d980(undefined8 param_1)

{
  int ****ppppiVar1;
  longlong *plVar2;
  int ****ppppiVar3;
  undefined4 uVar4;
  int iVar5;
  undefined8 *puVar6;
  int *piVar7;
  longlong lVar8;
  undefined8 uVar9;
  int ****ppppiVar10;
  int ****ppppiVar11;
  ulonglong uVar12;
  int iVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  undefined1 auStack_d8 [32];
  undefined4 local_b8;
  undefined4 local_b0;
  undefined4 local_a8;
  undefined4 local_a0;
  undefined4 local_98;
  undefined4 local_90;
  longlong local_88;
  int ****local_80;
  int ****local_78;
  int ****local_70;
  int local_68 [2];
  longlong local_60;
  longlong local_58;
  int local_50;
  int local_4c;
  undefined1 local_48 [2];
  undefined2 local_46;
  undefined2 local_42;
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_d8;
  if (DAT_143aa8520 != 0) {
    return;
  }
  FUN_1406e9170(param_1,local_68,4);
  FUN_1406e9170(param_1,&local_50,4);
  local_4c = local_50;
  if (local_68[0] == 0) {
    FUN_1406e9170(param_1,local_68,4);
    lVar8 = 0;
    if (local_50 == 1) {
      local_78 = (int ****)FUN_14019b780(&DAT_143ad68a0,0x1678);
      if (local_78 != (int ****)0x0) {
        lVar8 = FUN_142146d90(local_78);
      }
LAB_141c3df98:
      if (lVar8 != 0) goto LAB_141c3dfeb;
    }
    else {
      if (local_50 == 3) {
        local_78 = (int ****)FUN_14019b780(&DAT_143ad68a0,0x1b40);
        if (local_78 != (int ****)0x0) {
          lVar8 = FUN_141e93230(local_78);
        }
        goto LAB_141c3df98;
      }
      if (local_50 == 4) {
        local_78 = (int ****)FUN_14019b780(&DAT_143ad68a0,0x1860);
        if (local_78 != (int ****)0x0) {
          lVar8 = FUN_141c145f0(local_78);
        }
        goto LAB_141c3df98;
      }
    }
    local_78 = (int ****)FUN_1418039d0(0x21000003);
    puVar6 = (undefined8 *)FUN_141c428f0(&local_88,&local_78,&local_4c);
    FUN_1418049f0(&DAT_143271f04,0xdf,0x21000003,*puVar6);
    if (local_88 != 0) {
      FUN_14019f2c0(local_88 + -0x10);
    }
LAB_141c3dfeb:
    *(int *)(lVar8 + 0x304) = local_50;
    *(int *)(lVar8 + 0x308) = local_68[0];
    FUN_141c3ed00(lVar8,param_1);
    plVar2 = DAT_143aa84a0;
    if (DAT_143aa84a0 == (longlong *)0x0) {
      return;
    }
    if (DAT_143aa8518 == 0) {
      return;
    }
    iVar5 = FUN_14279c310();
    if (iVar5 == 0) {
      return;
    }
    uVar9 = FUN_1408a9e40(&local_78,0xdb8);
    FUN_1415eca30(uVar9,0xb);
    if (local_78 != (int ****)0x0) {
      FUN_14019f2c0(local_78 + -2);
    }
    FUN_142d50150(plVar2,0,0,1);
    return;
  }
  uVar12 = 0;
  ppppiVar10 = (int ****)0x0;
  local_80 = (int ****)0x0;
  switch(local_68[0]) {
  case 6:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_58,0x12d6);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    local_88 = local_58;
    break;
  default:
    goto switchD_141c3da16_caseD_7;
  case 8:
    local_60 = DAT_143aa84a0[0x708];
    local_60 = FUN_1408f63b0(&local_60,1);
    (*DAT_1432625b0)(&local_60,local_48);
    local_58 = 0;
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0xbb);
    uVar9 = FUN_14019ba10(&local_58,*puVar6,local_46,local_42);
    FUN_14019a260(&local_80,uVar9);
    if (local_88 != 0) {
      FUN_14019f2c0(local_88 + -0x10);
    }
    ppppiVar10 = local_80;
    if (local_58 != 0) {
      FUN_14019f2c0(local_58 + -0x10);
      ppppiVar10 = local_80;
    }
    goto LAB_141c3dcde;
  case 0xb:
    uVar4 = (**(code **)(*DAT_143aa84a0 + 0xa8))();
    uVar9 = FUN_14025b440(7,uVar4);
    FUN_14019a260(&local_80,uVar9);
    ppppiVar10 = local_80;
    goto LAB_141c3dcde;
  case 0xd:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1786);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0xe:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0xa7);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0xf:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x177f);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x11:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x175f);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x12:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1784);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x13:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1785);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x14:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x205);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x15:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1783);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x16:
  case 0x1c:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1782);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x19:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x177e);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x1a:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1780);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x1b:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1781);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x1e:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1e0);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x1f:
    if ((local_50 == 3) || (local_50 == 4)) {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1c6);
      ppppiVar10 = (int ****)*puVar6;
      *puVar6 = 0;
    }
    else {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1c5);
      ppppiVar10 = (int ****)*puVar6;
      *puVar6 = 0;
    }
    local_80 = ppppiVar10;
    if (local_88 != 0) {
      FUN_14019f2c0(local_88 + -0x10);
    }
    goto LAB_141c3dcde;
  }
  ppppiVar10 = local_80;
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
LAB_141c3dcde:
  if ((ppppiVar10 == (int ****)0x0) || (*(char *)ppppiVar10 == '\0')) {
switchD_141c3da16_caseD_7:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_78,0x1787);
    if (ppppiVar10 != (int ****)0x0) {
      FUN_14019f2c0(ppppiVar10 + -2);
    }
    ppppiVar10 = (int ****)*puVar6;
    *puVar6 = 0;
    local_80 = ppppiVar10;
    if (local_78 != (int ****)0x0) {
      FUN_14019f2c0(local_78 + -2);
    }
  }
  ppppiVar11 = ppppiVar10;
  if ((ppppiVar10 == (int ****)0x0) || (*(char *)ppppiVar10 == '\0')) goto LAB_141c3deec;
  local_70 = (int ****)0x0;
  ppppiVar1 = ppppiVar10 + -2;
  ppppiVar3 = local_70;
  if (ppppiVar1 != (int ****)0x0) {
    if (*(int *)ppppiVar1 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar15 = 0xffffffffffffffff;
      uVar14 = 0xffffffffffffffff;
      do {
        uVar14 = uVar14 + 1;
      } while (*(char *)((longlong)ppppiVar10 + uVar14) != '\0');
      iVar13 = (int)uVar14;
      iVar5 = 0;
      if (0 < iVar13) {
        iVar5 = iVar13;
      }
      piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
      piVar7[1] = iVar5;
      *piVar7 = -1;
      ppppiVar3 = (int ****)(piVar7 + 4);
      piVar7[2] = 0;
      *(char *)ppppiVar3 = '\0';
      local_78 = ppppiVar3;
      FUN_142ef7ba0(ppppiVar3,ppppiVar10,(longlong)iVar13);
      if (*piVar7 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar13 == -1) || (iVar13 <= piVar7[1])) {
        *piVar7 = 1;
        if (iVar13 != -1) goto LAB_141c3dded;
        if (ppppiVar3 != (int ****)0x0) {
          do {
            uVar15 = uVar15 + 1;
          } while (*(char *)((longlong)ppppiVar3 + uVar15) != '\0');
          uVar12 = uVar15 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar7[1],uVar14 & 0xffffffff);
        *piVar7 = 1;
LAB_141c3dded:
        *(char *)((longlong)ppppiVar3 + (longlong)iVar13) = '\0';
        uVar12 = uVar14;
      }
      iVar5 = (int)uVar12;
      if ((iVar5 < 0) || (piVar7[1] + 1 <= iVar5)) {
        FUN_142e54290(0x9c,uVar12 & 0xffffffff);
      }
      piVar7[2] = iVar5;
      if (local_70 != (int ****)0x0) {
        FUN_14019f2c0(local_70 + -2);
      }
    }
    else {
      if (*(int *)ppppiVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *(int *)ppppiVar1 = *(int *)ppppiVar1 + 1;
      UNLOCK();
      ppppiVar11 = local_80;
      ppppiVar3 = ppppiVar10;
      if (local_70 != (int ****)0x0) {
        FUN_14019f2c0(local_70 + -2);
        ppppiVar11 = local_80;
      }
    }
  }
  local_70 = ppppiVar3;
  local_78 = (int ****)&local_70;
  local_60 = 0;
  FUN_14019a260(&local_60,&local_70);
  local_90 = 0;
  local_98 = 0;
  local_a0 = 0;
  local_a8 = 0;
  local_b0 = 0;
  local_b8 = 0;
  FUN_142a26280(&local_60,0,0,1);
  if (local_70 != (int ****)0x0) {
    FUN_14019f2c0(local_70 + -2);
  }
LAB_141c3deec:
  if (ppppiVar11 != (int ****)0x0) {
    FUN_14019f2c0(ppppiVar11 + -2);
  }
  return;
}



//===========================================================
// FUN_141c3ed00 @ 141c3ed00   (616 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141c3ee3a) */

void FUN_141c3ed00(longlong *param_1,undefined8 param_2)

{
  char cVar1;
  ushort uVar2;
  undefined4 uVar3;
  int iVar4;
  longlong *plVar5;
  uint uVar6;
  longlong *plVar7;
  longlong local_res8;
  longlong local_50;
  undefined8 local_48;
  longlong local_40;
  undefined8 local_38;
  
  cVar1 = FUN_1406e8ae0(param_2);
  *(int *)((longlong)param_1 + 0x2fc) = (int)cVar1;
  cVar1 = FUN_1406e8ae0(param_2);
  *(int *)(param_1 + 0x5f) = (int)cVar1;
  if (param_1 + 0x62 != param_1 + 0x9a) {
    plVar5 = param_1 + 0x68;
    do {
      local_48 = 0;
      local_38 = 0;
      *(undefined4 *)(plVar5 + -6) = 0;
      if (plVar5[-5] != 0) {
        FUN_14019f2c0(plVar5[-5] + -0x10);
      }
      plVar5[-5] = 0;
      *(undefined4 *)(plVar5 + -4) = 0;
      if ((plVar5[-2] - 1U < 999) || (plVar5[-2] == -1)) {
        FUN_142e52ed0(0x447);
      }
      if (plVar5 + -3 == &local_50) {
        FUN_142e52d50(0x45c,1);
      }
      FUN_140d2d420(plVar5 + -3);
      plVar5[-2] = 0;
      if ((*plVar5 - 1U < 999) || (*plVar5 == -1)) {
        FUN_142e52ed0(0x447);
      }
      if (plVar5 + -1 == &local_40) {
        FUN_142e52d50(0x45c,1);
      }
      FUN_140301d60(plVar5 + -1);
      *plVar5 = 0;
      plVar7 = plVar5 + 1;
      plVar5 = plVar5 + 7;
    } while (plVar7 != param_1 + 0x9a);
  }
  cVar1 = FUN_1406e8ae0(param_2);
  while (-1 < cVar1) {
    uVar6 = (uint)cVar1;
    if (7 < (ulonglong)uVar6) break;
    (**(code **)(*param_1 + 0x1c8))(param_1,uVar6,param_2);
    plVar7 = param_1 + 0x62 + (ulonglong)uVar6 * 7;
    uVar3 = FUN_1406e8c20(param_2);
    *(undefined4 *)plVar7 = uVar3;
    plVar5 = (longlong *)FUN_1406e9050(param_2,&local_res8);
    if (plVar7[1] != 0) {
      FUN_14019f2c0(plVar7[1] + -0x10);
      plVar7[1] = 0;
    }
    plVar7[1] = *plVar5;
    *plVar5 = 0;
    if (local_res8 != 0) {
      FUN_14019f2c0(local_res8 + -0x10);
    }
    uVar2 = FUN_1406e8b80(param_2);
    *(uint *)(plVar7 + 2) = (uint)uVar2;
    *(int *)(param_1 + 0x60) = (int)param_1[0x60] + 1;
    cVar1 = FUN_1406e8ae0(param_2);
  }
  plVar5 = (longlong *)0x0;
  if ((DAT_143aa8520 != (longlong *)0x0) &&
     (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
     iVar4 != 0)) {
    plVar5 = DAT_143aa8520;
  }
                    /* WARNING: Could not recover jumptable at 0x000141c3ef61. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(*plVar5 + 0x188))(plVar5,param_2);
  return;
}



//===========================================================
// FUN_141c3ef70 @ 141c3ef70   (627 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141c3f172) */

void FUN_141c3ef70(longlong *param_1,undefined8 param_2)

{
  longlong lVar1;
  char cVar2;
  undefined8 *puVar3;
  longlong *plVar4;
  int local_res18 [2];
  undefined4 local_res20 [2];
  undefined8 local_a8;
  longlong local_a0;
  int local_98 [2];
  longlong local_90;
  undefined4 local_88;
  undefined1 local_80 [8];
  longlong local_78;
  undefined1 local_70 [8];
  longlong local_68;
  undefined4 local_60;
  undefined8 local_58;
  undefined4 local_50;
  longlong local_48 [3];
  undefined8 local_30;
  
  cVar2 = FUN_1406e8ae0(param_2);
  local_res18[0] = (int)cVar2;
  FUN_1406e9170(param_2,local_res20,4);
  if ((-1 < local_res18[0]) && ((ulonglong)(longlong)local_res18[0] < 8)) {
    plVar4 = param_1 + (longlong)local_res18[0] * 7 + 0x62;
    local_98[0] = (int)*plVar4;
    local_90 = 0;
    FUN_14019a260(&local_90,plVar4 + 1);
    local_88 = (undefined4)plVar4[2];
    lVar1 = plVar4[4];
    local_78 = lVar1;
    if (lVar1 != 0) {
      if (0xfffff < *(ulonglong *)(lVar1 + -0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar1 + -0x20) = *(longlong *)(lVar1 + -0x20) + 1;
      UNLOCK();
    }
    lVar1 = plVar4[6];
    local_68 = lVar1;
    if (lVar1 != 0) {
      if (0xfffff < *(ulonglong *)(lVar1 + -0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar1 + -0x20) = *(longlong *)(lVar1 + -0x20) + 1;
      UNLOCK();
    }
    if (local_98[0] == 0) {
      local_a8 = FUN_1418039d0(0x21000003);
      puVar3 = (undefined8 *)FUN_140ce7310(&local_a0,&local_a8,local_res18);
      FUN_1418049f0(&DAT_143271f04,0x1e8,0x21000003,*puVar3);
      if (local_a0 != 0) {
        FUN_14019f2c0(local_a0 + -0x10);
      }
    }
    local_60 = 0;
    local_58 = 0;
    local_50 = 0;
    local_48[1] = 0;
    local_30 = 0;
    plVar4 = param_1 + (longlong)local_res18[0] * 7 + 0x62;
    *(undefined4 *)plVar4 = 0;
    if (plVar4[1] != 0) {
      FUN_14019f2c0(plVar4[1] + -0x10);
    }
    local_58 = 0;
    plVar4[1] = 0;
    *(undefined4 *)(plVar4 + 2) = 0;
    if ((plVar4[4] - 1U < 999) || (plVar4[4] == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (plVar4 + 3 == local_48) {
      FUN_142e52d50(0x45c,1);
    }
    FUN_140d2d420(plVar4 + 3);
    plVar4[4] = 0;
    if ((plVar4[6] - 1U < 999) || (plVar4[6] == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (plVar4 + 5 == local_48 + 2) {
      FUN_142e52d50(0x45c,1);
    }
    FUN_140301d60(plVar4 + 5);
    plVar4[6] = 0;
    *(int *)(param_1 + 0x60) = (int)param_1[0x60] + -1;
    (**(code **)(*param_1 + 400))(param_1,local_res18[0],local_98,local_res20[0],param_2);
    FUN_140301d60(local_70);
    FUN_140d2d420(local_80);
    if (local_90 != 0) {
      FUN_14019f2c0(local_90 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_141c3e110 @ 141c3e110   (576 bytes)
//===========================================================

void FUN_141c3e110(undefined8 param_1)

{
  longlong *plVar1;
  undefined8 uVar2;
  char cVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  int iVar6;
  longlong lVar7;
  undefined8 *puVar8;
  longlong lVar9;
  undefined8 uVar10;
  int iVar11;
  int local_res10 [2];
  longlong local_res18;
  longlong local_res20;
  longlong local_48;
  undefined1 local_40 [8];
  longlong local_38;
  
  uVar2 = DAT_143aa84a0;
  uVar10 = 4;
  FUN_1406e9170(param_1,local_res10,4);
  uVar4 = FUN_1406e8c20(param_1);
  FUN_1406e9050(param_1,&local_res20);
  FUN_1406e8c20(param_1);
  uVar5 = FUN_1406e8c20(param_1);
  iVar11 = local_res10[0];
  if (DAT_143aa8520 == 0) {
    cVar3 = FUN_140426cb0(local_res10[0]);
    if (cVar3 == '\0') {
      iVar6 = FUN_142cb8770(uVar2);
    }
    else {
      iVar6 = FUN_142cb8820();
    }
    if (iVar6 == 0) {
      if ((iVar11 != 1) && (iVar11 != 2)) goto LAB_141c3e188;
      if ((*(int *)(DAT_143ac87a0 + 0x110) != 0) &&
         (iVar6 = FUN_142d01050(DAT_143aa84a0,uVar4), iVar6 == 0)) {
        local_res18 = FUN_14019b780(&DAT_143ad68a0,0x370);
        lVar7 = 0;
        if (local_res18 != 0) {
          lVar7 = FUN_141808b90(local_res18);
        }
        lVar9 = lVar7 + 0x18;
        if (lVar7 == 0) {
          lVar9 = 0;
        }
        if (lVar9 == 0) {
          local_48 = 0;
        }
        else {
          local_48 = lVar9 + -0x18;
          if (local_48 != 0) {
            if (0xfffff < *(ulonglong *)(lVar9 + 8)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar9 + 8) = *(longlong *)(lVar9 + 8) + 1;
            UNLOCK();
            iVar11 = local_res10[0];
          }
        }
        if (local_48 == 0) {
          FUN_142e52ed0(0x431,0);
        }
        local_res18 = 0;
        FUN_14019a260(&local_res18,&local_res20);
        FUN_14180fa70(local_48,&local_res18,uVar5,iVar11);
        local_38 = local_48;
        if (local_48 != 0) {
          if (0xfffff < *(ulonglong *)(local_48 + 0x20)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(local_48 + 0x20) = *(longlong *)(local_48 + 0x20) + 1;
          UNLOCK();
        }
        FUN_142d97880(uVar2,local_40);
        if (local_48 != 0) {
          if (0xffffe < *(longlong *)(local_48 + 0x20) - 1U) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar1 = (longlong *)(local_48 + 0x20);
          lVar7 = *plVar1;
          *plVar1 = *plVar1 + -1;
          UNLOCK();
          if (((int)lVar7 == 1) &&
             (puVar8 = (undefined8 *)(local_48 + 0x18), puVar8 != (undefined8 *)0x0)) {
            (**(code **)*puVar8)(puVar8,1);
          }
        }
        goto LAB_141c3e188;
      }
    }
  }
  else {
    uVar10 = 0xb;
  }
  FUN_141c3d820(uVar5,uVar10);
LAB_141c3e188:
  if (local_res20 != 0) {
    FUN_14019f2c0(local_res20 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141c3e360 @ 141c3e360   (1084 bytes)
//===========================================================

void FUN_141c3e360(undefined8 param_1)

{
  longlong lVar1;
  int iVar2;
  undefined4 uVar3;
  longlong *plVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong local_res10;
  longlong local_res18;
  longlong local_res20;
  
  FUN_1406e9170(param_1,&local_res10,4);
  if ((int)local_res10 == 0) {
    return;
  }
  local_res18 = 0;
  switch((int)local_res10) {
  case 1:
    plVar4 = (longlong *)FUN_1408a9e40(&local_res10,0x197);
    if (local_res18 != 0) {
      FUN_14019f2c0(local_res18 + -0x10);
    }
    local_res18 = *plVar4;
    *plVar4 = 0;
    lVar7 = local_res10;
    break;
  case 2:
    FUN_1406e9050(param_1,&local_res10);
    lVar7 = local_res10;
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res20,0x1c8);
    FUN_14019ba10(&local_res18,*puVar5,lVar7);
    lVar7 = local_res10;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
      lVar7 = local_res10;
    }
    break;
  case 3:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar2 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a540),
        iVar2 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
      (**(code **)(*DAT_143aa8520 + 0x138))(DAT_143aa8520,2);
    }
    FUN_1406e9050(param_1,&local_res10);
    lVar7 = local_res10;
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res20,0x1c9);
    FUN_14019ba10(&local_res18,*puVar5,lVar7);
    lVar7 = local_res10;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
      lVar7 = local_res10;
    }
    break;
  case 4:
    FUN_1406e9050(param_1,&local_res10);
    lVar7 = local_res10;
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res20,0x1ca);
    FUN_14019ba10(&local_res18,*puVar5,lVar7);
    lVar7 = local_res10;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
      lVar7 = local_res10;
    }
    break;
  case 5:
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res10,0x106e);
    FUN_14019ba10(&local_res18,*puVar5);
    lVar7 = local_res10;
    break;
  case 6:
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res10,0x106f);
    FUN_14019ba10(&local_res18,*puVar5);
    lVar7 = local_res10;
    break;
  case 7:
    FUN_1406e9050(param_1,&local_res10);
    lVar7 = local_res10;
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res20,0x1070);
    FUN_14019ba10(&local_res18,*puVar5,lVar7);
    lVar7 = local_res10;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
      lVar7 = local_res10;
    }
    break;
  case 8:
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res10,0x1071);
    FUN_14019ba10(&local_res18,*puVar5);
    lVar7 = local_res10;
    break;
  case 9:
    FUN_1406e9050(param_1,&local_res10);
    lVar7 = local_res10;
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res20,0x1072);
    FUN_14019ba10(&local_res18,*puVar5,lVar7);
    lVar7 = local_res10;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
      lVar7 = local_res10;
    }
    break;
  case 10:
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res10,0x1073);
    FUN_14019ba10(&local_res18,*puVar5);
    lVar7 = local_res10;
    break;
  case 0xb:
    FUN_1406e9050(param_1,&local_res10);
    lVar7 = local_res10;
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res20,0x1c8);
    FUN_14019ba10(&local_res18,*puVar5,lVar7);
    lVar7 = local_res10;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
      lVar7 = local_res10;
    }
    break;
  case 0xc:
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res10,0x1c7);
    FUN_14019ba10(&local_res18,*puVar5);
    lVar7 = local_res10;
    break;
  default:
    goto switchD_141c3e3b0_caseD_d;
  case 0xe:
    FUN_1406e9050(param_1,&local_res20);
    uVar3 = (**(code **)(*DAT_143aa84a0 + 0xa8))();
    uVar6 = FUN_14025b440(7,uVar3,1);
    local_res10 = 0;
    FUN_14019a260(&local_res10,uVar6);
    lVar1 = local_res10;
    FUN_14019ba10(&local_res18,local_res10,local_res20);
    lVar7 = local_res20;
    if (lVar1 != 0) {
      FUN_14019f2c0(lVar1 + -0x10);
      lVar7 = local_res20;
    }
    break;
  case 0xf:
    FUN_1406e9050(param_1,&local_res10);
    lVar7 = local_res10;
    puVar5 = (undefined8 *)FUN_1408a9e40(&local_res20,0x1cb);
    FUN_14019ba10(&local_res18,*puVar5,lVar7);
    lVar7 = local_res10;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
      lVar7 = local_res10;
    }
  }
  if (lVar7 != 0) {
    FUN_14019f2c0(lVar7 + -0x10);
  }
switchD_141c3e3b0_caseD_d:
  FUN_1415eca30(&local_res18,0xb);
  if (local_res18 != 0) {
    FUN_14019f2c0(local_res18 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141c3f300 @ 141c3f300   (1246 bytes)
//===========================================================

void FUN_141c3f300(longlong *param_1,undefined8 param_2)

{
  char *pcVar1;
  undefined1 uVar2;
  byte bVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 *puVar7;
  longlong ******pppppplVar8;
  undefined8 uVar9;
  uint uVar10;
  int *piVar11;
  uint uVar12;
  int *piVar13;
  longlong ******pppppplVar14;
  longlong lVar15;
  longlong ******pppppplVar16;
  undefined4 *local_res18;
  undefined4 **local_res20;
  longlong *****local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong *****local_68 [2];
  longlong *****local_58;
  longlong *****ppppplStack_50;
  longlong *****local_48;
  longlong *****ppppplStack_40;
  
  FUN_1406e9170(param_2,&local_res18,4);
  if ((int)local_res18 == 0) {
    bVar3 = FUN_1406e8ae0(param_2);
    pppppplVar16 = (longlong ******)0x0;
    if ((*(int *)((longlong)param_1 + 0x304) == 1) &&
       (uVar10 = (uint)bVar3, bVar3 = 2, uVar10 == *(uint *)(param_1 + 0x5f))) {
      bVar3 = 0;
    }
    uVar5 = (**(code **)(*param_1 + 0x1d0))(param_1,bVar3);
    FUN_1406e9050(param_2,&local_80);
    FUN_1406e9050(param_2,&local_res20);
    uVar6 = FUN_1406e8c20(param_2);
    FUN_1406e8c20(param_2);
    local_48 = (longlong *****)0x0;
    ppppplStack_40 = (longlong *****)0x0;
    pppppplVar8 = (longlong ******)FUN_14019b780(&DAT_143ad68a0,0x58);
    pppppplVar14 = pppppplVar16;
    local_68[0] = (longlong *****)pppppplVar8;
    if (pppppplVar8 != (longlong ******)0x0) {
      *pppppplVar8 = (longlong *****)0x0;
      pppppplVar8[1] = (longlong *****)0x0;
      *(undefined4 *)(pppppplVar8 + 1) = 1;
      *(undefined4 *)((longlong)pppppplVar8 + 0xc) = 1;
      *pppppplVar8 = (longlong *****)&PTR_FUN_143300d78;
      FUN_1408d6690(pppppplVar8 + 2);
      pppppplVar14 = pppppplVar8;
    }
    pppppplVar8 = pppppplVar14 + 2;
    local_48 = (longlong *****)pppppplVar8;
    ppppplStack_40 = (longlong *****)pppppplVar14;
    FUN_1408d6760(pppppplVar8,param_2);
    FUN_1408bee80(DAT_143ac2f58,&local_res20,0,1,0);
    iVar4 = FUN_142d01050(DAT_143aa84a0,uVar6);
    if (iVar4 == 0) {
      lVar15 = -1;
      do {
        pcVar1 = &DAT_143278815 + lVar15;
        lVar15 = lVar15 + 1;
      } while (*pcVar1 != '\0');
      FUN_1401abc80(&local_80,&local_70);
      if (local_res20 != (undefined4 **)0x0) {
        pppppplVar16 = (longlong ******)(ulonglong)*(uint *)(local_res20 + -1);
      }
      FUN_1401abc80(&local_70,&local_78,local_res20,pppppplVar16);
      if (local_70 != 0) {
        FUN_14019f2c0(local_70 + -0x10);
      }
      lVar15 = local_78;
      local_88 = (longlong *****)&local_58;
      if (pppppplVar14 != (longlong ******)0x0) {
        LOCK();
        *(int *)(pppppplVar14 + 1) = *(int *)(pppppplVar14 + 1) + 1;
        UNLOCK();
      }
      local_58 = (longlong *****)pppppplVar8;
      ppppplStack_50 = (longlong *****)pppppplVar14;
      uVar9 = FUN_1403ede40(local_68,local_78,0xffffffff);
      FUN_141c3fe50(param_1,uVar9,uVar5,&local_58);
      if (lVar15 != 0) {
        FUN_14019f2c0(lVar15 + -0x10);
      }
      if (pppppplVar14 != (longlong ******)0x0) {
        LOCK();
        pppppplVar16 = pppppplVar14 + 1;
        iVar4 = *(int *)pppppplVar16;
        *(int *)pppppplVar16 = *(int *)pppppplVar16 + -1;
        UNLOCK();
        if (iVar4 == 1) {
          (*(code *)**pppppplVar14)(pppppplVar14);
          LOCK();
          piVar13 = (int *)((longlong)pppppplVar14 + 0xc);
          iVar4 = *piVar13;
          *piVar13 = *piVar13 + -1;
          UNLOCK();
          if (iVar4 == 1) {
            (*(code *)(*pppppplVar14)[1])(pppppplVar14);
          }
        }
      }
      if (local_res20 != (undefined4 **)0x0) {
        FUN_14019f2c0(local_res20 + -2);
      }
    }
    else {
      if (pppppplVar14 != (longlong ******)0x0) {
        LOCK();
        pppppplVar16 = pppppplVar14 + 1;
        iVar4 = *(int *)pppppplVar16;
        *(int *)pppppplVar16 = *(int *)pppppplVar16 + -1;
        UNLOCK();
        if (iVar4 == 1) {
          (*(code *)**pppppplVar14)(pppppplVar14);
          LOCK();
          piVar13 = (int *)((longlong)pppppplVar14 + 0xc);
          iVar4 = *piVar13;
          *piVar13 = *piVar13 + -1;
          UNLOCK();
          if (iVar4 == 1) {
            (*(code *)(*pppppplVar14)[1])(pppppplVar14);
          }
        }
      }
      if (local_res20 != (undefined4 **)0x0) {
        FUN_14019f2c0(local_res20 + -2);
      }
    }
    goto LAB_141c3f7b4;
  }
  if ((int)local_res18 != 1) {
    return;
  }
  uVar2 = FUN_1406e8ae0(param_2);
  FUN_1406e9050(param_2,&local_70);
  local_78 = 0;
  local_res18 = (undefined4 *)0x0;
  FUN_14019a260(&local_res18,&local_70);
  FUN_141c414f0(param_1,&local_78,uVar2,&local_res18);
  lVar15 = local_78;
  local_res18 = (undefined4 *)0x0;
  if (local_78 != 0) {
    uVar10 = 0;
    piVar11 = (int *)0xffffffffffffffff;
    iVar4 = (*DAT_1432627f8)(0xfde9,0,local_78,0xffffffff,0,0);
    local_res20 = (undefined4 **)((ulonglong)(longlong)(iVar4 * 2) >> 1);
    uVar12 = (int)local_res20 - 1;
    piVar13 = (int *)0x0;
    if ((local_res18 == (undefined4 *)0x0) || (piVar13 = local_res18 + -4, piVar13 == (int *)0x0)) {
LAB_141c3f410:
      if ((int)uVar10 < (int)uVar12) {
        uVar10 = uVar12;
      }
      puVar7 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(int)(uVar10 * 2 + 0x12));
      puVar7[1] = uVar10;
      *puVar7 = 0xffffffff;
      local_res18 = puVar7 + 4;
      puVar7[2] = 0;
      *(undefined2 *)local_res18 = 0;
      if (piVar13 != (int *)0x0) {
        FUN_1401bebb0(piVar13);
      }
    }
    else {
      if ((1 < *piVar13) || ((int)local_res18[-3] < (int)uVar12)) {
        uVar10 = (uint)((ulonglong)(longlong)(int)local_res18[-2] >> 1);
        goto LAB_141c3f410;
      }
      if (*piVar13 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar13 = -1;
    }
    (*DAT_1432627f8)(0xfde9,0,lVar15,0xffffffff,local_res18,(int)local_res20);
    puVar7 = local_res18;
    if (local_res18[-4] != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((uVar12 != 0xffffffff) && ((int)puVar7[-3] < (int)uVar12)) {
      FUN_142e54290(0x90,puVar7[-3],(int *)(ulonglong)uVar12);
    }
    puVar7[-4] = 1;
    if (uVar12 == 0xffffffff) {
      piVar13 = (int *)0x0;
      if (puVar7 != (undefined4 *)0x0) {
        do {
          piVar11 = (int *)((longlong)piVar11 + 1);
          piVar13 = piVar11;
        } while (*(short *)((longlong)puVar7 + (longlong)piVar11 * 2) != 0);
      }
    }
    else {
      *(undefined2 *)((longlong)local_res18 + (longlong)(int)uVar12 * 2) = 0;
      piVar13 = (int *)(ulonglong)uVar12;
    }
    iVar4 = (int)piVar13;
    if ((iVar4 < 0) || (puVar7[-3] + 1 <= iVar4)) {
      FUN_142e54290(0x9c,(ulonglong)piVar13 & 0xffffffff);
    }
    puVar7[-2] = iVar4 * 2;
  }
  local_res20 = &local_res18;
  local_68[0] = (longlong *****)&local_48;
  local_48 = (longlong *****)0x0;
  ppppplStack_40 = (longlong *****)0x0;
  local_88 = (longlong *****)0x0;
  FUN_1401c1fb0(&local_88,&local_res18);
  FUN_141c3fe50(param_1,&local_88,2,&local_48);
  if (local_res18 != (undefined4 *)0x0) {
    FUN_1401bebb0(local_res18 + -4);
  }
  local_80 = local_70;
  if (lVar15 != 0) {
    FUN_14019f2c0(lVar15 + -0x10);
    local_80 = local_70;
  }
LAB_141c3f7b4:
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141c3f7f0 @ 141c3f7f0   (47 bytes)
//===========================================================

void FUN_141c3f7f0(longlong param_1,undefined8 param_2)

{
  undefined4 local_res8 [8];
  
  FUN_1406e9170(param_2,local_res8,4);
  *(undefined4 *)(param_1 + 0x308) = local_res8[0];
  return;
}



//===========================================================
// FUN_141c423d0 @ 141c423d0   (941 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141c4266a) */

void FUN_141c423d0(longlong *param_1,int param_2,undefined8 param_3)

{
  int iVar1;
  int *piVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong *plVar5;
  longlong *plVar6;
  ulonglong uVar7;
  undefined1 auStack_2f8 [32];
  undefined8 local_2d8;
  undefined4 local_2d0;
  undefined4 local_2c8;
  undefined4 local_2c0;
  undefined4 local_2b8;
  undefined4 local_2b0;
  undefined8 local_2a8;
  undefined1 local_2a0;
  undefined1 local_298;
  undefined1 local_290;
  int *local_288;
  longlong *local_280;
  undefined4 local_278 [2];
  undefined8 local_270;
  longlong *local_268;
  longlong *local_260;
  longlong *local_258;
  undefined1 local_250 [8];
  undefined1 *local_248;
  undefined **local_238;
  undefined8 local_230;
  undefined8 local_228;
  int *local_220;
  undefined1 local_218;
  undefined8 local_217;
  undefined8 local_20f;
  undefined8 local_207;
  undefined8 local_1ff;
  undefined8 uStack_1f7;
  undefined8 local_1ef;
  undefined8 uStack_1e7;
  undefined8 local_1df;
  undefined8 uStack_1d7;
  undefined8 local_1cf;
  undefined8 uStack_1c7;
  undefined8 local_1bf;
  undefined8 uStack_1b7;
  undefined8 local_1af;
  undefined8 uStack_1a7;
  undefined8 local_19f;
  undefined8 uStack_197;
  undefined8 local_18f;
  undefined8 uStack_187;
  undefined8 local_ff;
  undefined8 uStack_f7;
  undefined8 local_ef;
  undefined8 uStack_e7;
  undefined8 local_df;
  undefined8 uStack_d7;
  undefined8 local_cf;
  undefined8 uStack_c7;
  undefined8 local_bf;
  undefined8 uStack_b7;
  undefined8 local_af;
  undefined8 uStack_a7;
  undefined8 local_9f;
  undefined8 uStack_97;
  undefined8 local_8f;
  undefined8 uStack_87;
  undefined4 local_7f;
  undefined8 local_7b;
  undefined4 local_73;
  undefined1 local_6f;
  undefined8 local_6e;
  undefined4 local_66;
  undefined8 local_62;
  undefined4 local_5a;
  undefined1 local_56;
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_2f8;
  uVar7 = (ulonglong)param_2;
  local_220 = (int *)0x0;
  local_230 = 0;
  local_228 = 0;
  local_238 = &PTR_FUN_14327d968;
  local_6e = 0;
  local_218 = 0;
  local_217 = 0;
  local_20f = 0;
  local_207 = 0;
  local_7b = 0;
  local_73 = 0;
  local_6f = 0;
  local_1ff = 0;
  uStack_1f7 = 0;
  local_1ef = 0;
  uStack_1e7 = 0;
  local_1df = 0;
  uStack_1d7 = 0;
  local_1cf = 0;
  uStack_1c7 = 0;
  local_1bf = 0;
  uStack_1b7 = 0;
  local_1af = 0;
  uStack_1a7 = 0;
  local_19f = 0;
  uStack_197 = 0;
  local_18f = 0;
  uStack_187 = 0;
  local_7f = 0;
  local_ff = 0;
  uStack_f7 = 0;
  local_ef = 0;
  uStack_e7 = 0;
  local_df = 0;
  uStack_d7 = 0;
  local_cf = 0;
  uStack_c7 = 0;
  local_bf = 0;
  uStack_b7 = 0;
  local_af = 0;
  uStack_a7 = 0;
  local_9f = 0;
  uStack_97 = 0;
  local_8f = 0;
  uStack_87 = 0;
  local_62 = 0;
  local_5a = 0;
  local_56 = 0;
  local_66 = 0xffffffff;
  local_288 = (int *)0x0;
  piVar2 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  piVar2[1] = 0;
  *piVar2 = -1;
  local_288 = piVar2 + 4;
  piVar2[2] = 0;
  *(undefined1 *)local_288 = 0;
  if (*piVar2 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar2[1] < 0) {
    FUN_142e54290(0x90,piVar2[1],0);
  }
  *piVar2 = 1;
  *(undefined1 *)local_288 = 0;
  if (piVar2[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar2[2] = 0;
  FUN_1402ee8d0(&local_238,param_3,&local_288,0);
  local_278[0] = 4;
  local_270 = 0;
  (**(code **)(*param_1 + 0x170))(param_1,param_2,local_278,&local_270);
  if ((-1 < param_2) && (uVar7 < 8)) {
    FUN_140d2d420(param_1 + uVar7 * 7 + 0x65);
    lVar3 = FUN_140d2d280(0);
    param_1[uVar7 * 7 + 0x66] = lVar3;
    if (lVar3 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar3 = param_1[uVar7 * 7 + 0x66];
    }
    local_248 = local_250;
    uVar4 = FUN_142bf6010(param_1,local_250);
    plVar5 = (longlong *)FUN_142bf6010(param_1,&local_258);
    local_280 = (longlong *)0x0;
    plVar5 = (longlong *)*plVar5;
    local_268 = plVar5;
    if (plVar5 == (longlong *)0x0) {
      local_280 = (longlong *)0x0;
      iVar1 = -0x7fffbffe;
      plVar6 = local_280;
    }
    else {
      (**(code **)(*plVar5 + 8))(plVar5);
      local_260 = (longlong *)0x0;
      iVar1 = (**(code **)*plVar5)(plVar5,&DAT_143273488,&local_260);
      plVar6 = (longlong *)0x0;
      if (-1 < iVar1) {
        plVar6 = local_260;
      }
      if (local_280 != (longlong *)0x0) {
        (**(code **)(*local_280 + 0x10))();
      }
    }
    local_280 = plVar6;
    if (plVar5 != (longlong *)0x0) {
      (**(code **)(*plVar5 + 0x10))(plVar5);
    }
    if (((iVar1 + 0x80000000U & 0x80000000) == 0) && (iVar1 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar1);
    }
    local_290 = 0;
    local_298 = 0;
    local_2a0 = 0;
    local_2a8 = 0;
    local_2b0 = 0;
    local_2b8 = 100;
    local_2c0 = local_270._4_4_;
    local_2c8 = (undefined4)local_270;
    local_2d0 = 1;
    local_2d8 = uVar4;
    FUN_140f7f030(lVar3,&local_238,local_278[0],&local_280);
    if (local_258 != (longlong *)0x0) {
      (**(code **)(*local_258 + 0x10))();
    }
  }
  local_238 = &PTR_FUN_143273970;
  if (local_220 != (int *)0x0) {
    LOCK();
    local_220[2] = 0;
    local_220[3] = 0;
    UNLOCK();
    do {
    } while (local_220[1] != 0);
    if (local_220 != (int *)0x0) {
      LOCK();
      iVar1 = *local_220;
      *local_220 = *local_220 + -1;
      UNLOCK();
      if (iVar1 == 1) {
        thunk_FUN_140205820(local_220,0x10);
      }
    }
  }
  return;
}



//===========================================================
// FUN_141c3fcb0 @ 141c3fcb0   (273 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141c3fcb0(longlong param_1,longlong *param_2,undefined4 *param_3)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  undefined1 auStack_4a8 [32];
  undefined4 local_488 [2];
  longlong *local_480;
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  *param_3 = 1;
  local_480 = param_2;
  iVar2 = FUN_1415c0460(param_1 + 0x2b0,param_2,1);
  if (iVar2 == 0) {
    *param_3 = 0;
  }
  else {
    uVar3 = (*DAT_143262db0)();
    cVar1 = FUN_1408fc970(*(undefined4 *)(param_1 + 0x4d0),200,uVar3);
    if (cVar1 == '\0') {
      *param_3 = 0;
    }
    else {
      FUN_1406ed520(local_478,0x17e);
      local_488[0] = 8;
      FUN_1406ede20(local_478,local_488,4);
      uVar3 = FUN_1429e3ef0();
      FUN_1406ed9d0(local_478,uVar3);
      FUN_1406edc80(local_478,param_2);
      FUN_1415d01c0(local_478);
      uVar3 = (*DAT_143262db0)();
      *(undefined4 *)(param_1 + 0x4d0) = uVar3;
      FUN_1406ed610(local_478);
    }
  }
  if (*param_2 != 0) {
    FUN_14019f2c0(*param_2 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141c3d820 @ 141c3d820   (245 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141c3d820(undefined4 param_1,int param_2)

{
  undefined1 auStack_498 [32];
  undefined4 local_478 [2];
  int local_470 [2];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  local_470[0] = param_2;
  if (param_2 == 0) {
    FUN_1406ed520(local_468,0x17e);
    local_478[0] = 3;
    FUN_1406ede20(local_468,local_478,4);
    FUN_1406ed9d0(local_468,param_1);
    FUN_1406ed840(local_468,0);
    FUN_1406ed840(local_468,0);
    FUN_1415d01c0(local_468);
  }
  else {
    FUN_1406ed520(local_468,0x17e);
    local_478[0] = 6;
    FUN_1406ede20(local_468,local_478,4);
    FUN_1406ed9d0(local_468,param_1);
    FUN_1406ede20(local_468,local_470,4);
    FUN_1415d01c0(local_468);
  }
  FUN_1406ed610(local_468);
  return;
}



//===========================================================
// FUN_141c3ce70 @ 141c3ce70   (115 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141c3ce70(void)

{
  undefined1 auStack_498 [32];
  undefined4 auStack_478 [4];
  undefined1 auStack_468 [1104];
  ulonglong uStack_18;
  
  uStack_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  FUN_1406ed520(auStack_468,0x17e);
  auStack_478[0] = 0xc;
  FUN_1406ede20(auStack_468,auStack_478,4);
  FUN_1415d01c0(auStack_468);
  FUN_1406ed610(auStack_468);
  return;
}



//===========================================================
// FUN_141c3f8a0 @ 141c3f8a0   (7 bytes)
//===========================================================

undefined4 FUN_141c3f8a0(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x300);
}



//===========================================================
// FUN_141c3f8b0 @ 141c3f8b0   (7 bytes)
//===========================================================

undefined4 FUN_141c3f8b0(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x2f8);
}


