
//===========================================================
// FUN_14271a080 @ 14271a080   (1475 bytes)
//===========================================================

void FUN_14271a080(longlong param_1)

{
  longlong lVar1;
  undefined4 uVar2;
  IUnknown *pIVar3;
  int iVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  int *piVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  longlong *plVar11;
  undefined4 uVar12;
  undefined8 *puVar13;
  undefined4 uVar14;
  ulonglong local_res8;
  int *local_res18;
  IUnknown *local_res20;
  undefined8 in_stack_ffffffffffffff38;
  uint uVar15;
  undefined4 uVar16;
  undefined8 in_stack_ffffffffffffff48;
  uint uVar18;
  ulonglong uVar17;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined1 local_60 [8];
  longlong local_58;
  undefined1 local_50 [8];
  longlong local_48;
  undefined1 local_40 [8];
  longlong local_38;
  
  uVar15 = (uint)((ulonglong)in_stack_ffffffffffffff38 >> 0x20);
  uVar18 = (uint)((ulonglong)in_stack_ffffffffffffff48 >> 0x20);
  FUN_141aa3af0(param_1 + 0x13f8,param_1,0,0);
  uVar6 = FUN_1401a5890(&local_res8,L"UI/UIWindow2.img/UserInfo/exception/backgrnd");
  puVar9 = (undefined8 *)0x0;
  uVar17 = (ulonglong)uVar18 << 0x20;
  FUN_142bfac60(param_1,uVar6,0,0,1,0,(ulonglong)uVar15 << 0x20,0xff,uVar17);
  local_78 = 0;
  uVar17 = uVar17 & 0xffffffff00000000;
  lVar7 = FUN_141aa81d0(param_1 + 0x13f8,local_60,L"UI/UIWindow2.img/UserInfo/exception/BtMeso",
                        0x7d1,0,0,0,0xff,uVar17,0,0,0,0,0,1,&local_78);
  lVar7 = *(longlong *)(lVar7 + 8);
  if ((*(longlong *)(param_1 + 0x250) - 1U < 999) || (*(longlong *)(param_1 + 0x250) == -1)) {
    FUN_142e52ed0(0x447);
  }
  puVar10 = (undefined8 *)(lVar7 + 0x18);
  if (lVar7 == 0) {
    puVar10 = puVar9;
  }
  puVar13 = puVar9;
  if ((puVar10 != (undefined8 *)0x0) && (puVar13 = puVar10 + -3, puVar13 != (undefined8 *)0x0)) {
    if (0xfffff < (ulonglong)puVar10[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    puVar10[1] = puVar10[1] + 1;
    UNLOCK();
  }
  lVar7 = *(longlong *)(param_1 + 0x250);
  *(undefined8 **)(param_1 + 0x250) = puVar13;
  if (lVar7 != 0) {
    if (0xffffe < *(longlong *)(lVar7 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar11 = (longlong *)(lVar7 + 0x20);
    lVar1 = *plVar11;
    *plVar11 = *plVar11 + -1;
    UNLOCK();
    if (((int)lVar1 == 1) && (puVar10 = (undefined8 *)(lVar7 + 0x18), puVar10 != (undefined8 *)0x0))
    {
      (**(code **)*puVar10)(puVar10,1);
    }
  }
  lVar7 = local_58;
  if (local_58 != 0) {
    if (0xffffe < *(longlong *)(local_58 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar11 = (longlong *)(lVar7 + 0x20);
    lVar7 = *plVar11;
    *plVar11 = *plVar11 + -1;
    UNLOCK();
    if ((int)lVar7 == 1) {
      puVar10 = (undefined8 *)(local_58 + 0x18);
      if (local_58 == 0) {
        puVar10 = puVar9;
      }
      if (puVar10 != (undefined8 *)0x0) {
        (**(code **)*puVar10)(puVar10,1);
      }
    }
  }
  local_70 = 0;
  uVar17 = uVar17 & 0xffffffff00000000;
  FUN_141aa81d0(param_1 + 0x13f8,local_50,L"UI/UIWindow2.img/UserInfo/exception/BtRegist",0x7d2,0,0,
                0,0xff,uVar17,0,0,0,0,0,1,&local_70);
  lVar7 = local_48;
  if (local_48 != 0) {
    if (0xffffe < *(longlong *)(local_48 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar11 = (longlong *)(lVar7 + 0x20);
    lVar7 = *plVar11;
    *plVar11 = *plVar11 + -1;
    UNLOCK();
    if ((int)lVar7 == 1) {
      puVar10 = (undefined8 *)(local_48 + 0x18);
      if (local_48 == 0) {
        puVar10 = puVar9;
      }
      if (puVar10 != (undefined8 *)0x0) {
        (**(code **)*puVar10)(puVar10,1);
      }
    }
  }
  local_68 = 0;
  uVar16 = 0;
  FUN_141aa81d0(param_1 + 0x13f8,local_40,L"UI/UIWindow2.img/UserInfo/exception/BtDelete",0x7d3,0,0,
                0,0xff,uVar17 & 0xffffffff00000000,0,0,0,0,0,1,&local_68);
  lVar7 = local_38;
  if (local_38 != 0) {
    if (0xffffe < *(longlong *)(local_38 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar11 = (longlong *)(lVar7 + 0x20);
    lVar7 = *plVar11;
    *plVar11 = *plVar11 + -1;
    UNLOCK();
    if ((int)lVar7 == 1) {
      puVar10 = (undefined8 *)(local_38 + 0x18);
      if (local_38 == 0) {
        puVar10 = puVar9;
      }
      if (puVar10 != (undefined8 *)0x0) {
        (**(code **)*puVar10)(puVar10,1);
      }
    }
  }
  uVar14 = 0x98;
  local_res18 = (int *)0x0;
  piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,0x3e);
  piVar8[1] = 0x2d;
  *piVar8 = -1;
  local_res18 = piVar8 + 4;
  piVar8[2] = 0;
  *(undefined1 *)local_res18 = 0;
  uVar6 = s_UI_UIWindow2_img_UserInfo_except_14347d990._8_8_;
  *(undefined8 *)local_res18 = s_UI_UIWindow2_img_UserInfo_except_14347d990._0_8_;
  *(undefined8 *)(piVar8 + 6) = uVar6;
  uVar2 = s_UI_UIWindow2_img_UserInfo_except_14347d990._28_4_;
  uVar5 = s_UI_UIWindow2_img_UserInfo_except_14347d990._24_4_;
  uVar12 = s_UI_UIWindow2_img_UserInfo_except_14347d990._20_4_;
  piVar8[8] = s_UI_UIWindow2_img_UserInfo_except_14347d990._16_4_;
  piVar8[9] = uVar12;
  piVar8[10] = uVar5;
  piVar8[0xb] = uVar2;
  *(undefined8 *)(piVar8 + 0xc) = s_UI_UIWindow2_img_UserInfo_except_14347d990._32_8_;
  piVar8[0xe] = s_UI_UIWindow2_img_UserInfo_except_14347d990._40_4_;
  *(char *)(piVar8 + 0xf) = s_UI_UIWindow2_img_UserInfo_except_14347d990[0x2c];
  if (*piVar8 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar8[1] < 0x2d) {
    FUN_142e54290(0x90,piVar8[1],0x2d);
  }
  *piVar8 = 1;
  *(undefined1 *)((longlong)local_res18 + 0x2d) = 0;
  if (piVar8[1] + 1 < 0x2e) {
    FUN_142e54290(0x9c);
  }
  piVar8[2] = 0x2d;
  FUN_14090faf0(&local_res20,&local_res18);
  if (local_res18 != (int *)0x0) {
    FUN_14019f2c0(local_res18 + -4);
  }
  pIVar3 = local_res20;
  uVar12 = 0x1b;
  if (local_res20 != (IUnknown *)0x0) {
    local_res8 = local_res8 & 0xffffffff00000000;
    iVar4 = (**(code **)(*(longlong *)local_res20 + 0x40))(local_res20,&local_res8);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar3,(_GUID *)&DAT_143273780);
    }
    pIVar3 = local_res20;
    uVar14 = (undefined4)local_res8;
    if (local_res20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8 = local_res8 & 0xffffffff00000000;
    iVar4 = (**(code **)(*(longlong *)local_res20 + 0x50))(local_res20,&local_res8);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar3,(_GUID *)&DAT_143273780);
    }
    uVar12 = (undefined4)local_res8;
  }
  if (local_res20 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_res20 + 0x10))(local_res20);
  }
  uVar5 = FUN_140910dc0("UI/UIWindow2.img/UserInfo/exception/maxLenth",0);
  *(undefined4 *)(param_1 + 0x13d0) = uVar5;
  local_res8 = FUN_14019b780(&DAT_143ad68a0,0x108);
  if (local_res8 != 0) {
    puVar9 = (undefined8 *)FUN_1416ed1a0(local_res8);
  }
  FUN_140d2cd60(param_1 + 0x278,puVar9);
  plVar11 = *(longlong **)(param_1 + 0x280);
  if (plVar11 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
    plVar11 = *(longlong **)(param_1 + 0x280);
  }
  (**(code **)(*plVar11 + 0x80))(plVar11,param_1,1000,1,5,uVar14,CONCAT44(uVar16,uVar12),0x88,0);
  lVar7 = *(longlong *)(param_1 + 0x280);
  if (lVar7 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar7 = *(longlong *)(param_1 + 0x280);
  }
  iVar4 = FUN_142bf7d70(param_1);
  *(int *)(lVar7 + 0x78) = iVar4 + -0x1b;
  FUN_142645170(param_1 + 0x298);
  FUN_14271bcd0(param_1);
  return;
}



//===========================================================
// FUN_14271a690 @ 14271a690   (2338 bytes)
//===========================================================

void FUN_14271a690(longlong param_1)

{
  char *pcVar1;
  int iVar2;
  undefined8 *puVar3;
  longlong lVar4;
  char *pcVar5;
  undefined8 uVar6;
  int *piVar7;
  char *pcVar8;
  int *piVar9;
  IUnknown *pIVar10;
  int *piVar11;
  int iVar12;
  uint uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  int local_res18 [2];
  int local_res20;
  undefined8 in_stack_fffffffffffffe98;
  undefined4 uVar16;
  uint *in_stack_fffffffffffffea0;
  undefined4 uVar17;
  char *local_138;
  undefined4 local_130;
  undefined4 uStack_12c;
  undefined8 uStack_128;
  undefined8 local_120;
  short local_118;
  undefined6 uStack_116;
  longlong lStack_110;
  undefined8 local_108;
  short local_100;
  undefined6 uStack_fe;
  longlong lStack_f8;
  undefined8 local_f0;
  undefined8 *local_e8;
  IUnknown *local_e0;
  IUnknown *local_d8;
  ulonglong local_d0;
  longlong local_c8;
  longlong local_c0;
  undefined1 local_b8 [8];
  longlong *local_b0;
  undefined8 **local_a8;
  undefined8 *local_a0;
  undefined8 local_98;
  longlong lStack_90;
  undefined8 local_88;
  undefined8 local_78;
  longlong lStack_70;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  FUN_142bf7e40();
  FUN_142bf6230(param_1,&local_e0);
  piVar11 = (int *)0x0;
  local_138 = (char *)0x0;
  if ((DAT_143aa8518 != 0) && (*(longlong *)(param_1 + 0x13f0) != 0)) {
    iVar2 = FUN_142bf7d70(param_1);
    iVar2 = iVar2 + -0x21;
    lVar4 = *(longlong *)(param_1 + 0x280);
    local_res20 = iVar2;
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = *(longlong *)(param_1 + 0x280);
    }
    uVar14 = (ulonglong)*(int *)(lVar4 + 0x80);
    uVar15 = uVar14;
    while( true ) {
      uVar16 = (undefined4)((ulonglong)in_stack_fffffffffffffe98 >> 0x20);
      iVar12 = (int)uVar15;
      lVar4 = *(longlong *)(param_1 + 0x280);
      local_res18[0] = iVar12;
      local_d0 = uVar14;
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = *(longlong *)(param_1 + 0x280);
      }
      pIVar10 = local_e0;
      if ((*(int *)(lVar4 + 0x80) + 10 <= iVar12) ||
         ((ulonglong)(*(longlong *)(param_1 + 0x13e0) - *(longlong *)(param_1 + 0x13d8) >> 2) <=
          (ulonglong)(longlong)iVar12)) break;
      uVar17 = (undefined4)((ulonglong)in_stack_fffffffffffffea0 >> 0x20);
      if (iVar12 == *(int *)(param_1 + 0x13c8)) {
        if (local_e0 == (IUnknown *)0x0) {
LAB_14271af8a:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        in_stack_fffffffffffffea0 = (uint *)CONCAT44(uVar17,0xfffdc100);
LAB_14271a7c1:
        iVar2 = (**(code **)(*(longlong *)local_e0 + 0x170))
                          (local_e0,10,(iVar12 - *(int *)(lVar4 + 0x80)) * 0xd + 0x1e,iVar2,
                           CONCAT44(uVar16,10),in_stack_fffffffffffffea0);
        if (iVar2 < 0) {
          _com_issue_errorex(iVar2,pIVar10,(_GUID *)&DAT_14327ac98);
        }
      }
      else if (iVar12 == *(int *)(param_1 + 0x13c4)) {
        if (local_e0 == (IUnknown *)0x0) goto LAB_14271af8a;
        in_stack_fffffffffffffea0 = (uint *)CONCAT44(uVar17,0xfffde480);
        goto LAB_14271a7c1;
      }
      if (*(int *)(*(longlong *)(param_1 + 0x13d8) + uVar14 * 4) == 0x7fffffff) {
        puVar3 = (undefined8 *)FUN_1408a9e40(&local_c8,0xa4d);
        if (local_138 != (char *)0x0) {
          FUN_14019f2c0(local_138 + -0x10);
        }
        local_138 = (char *)*puVar3;
        *puVar3 = 0;
        lVar4 = local_c8;
      }
      else {
        puVar3 = (undefined8 *)FUN_14039a960(DAT_143aa8328,&local_c0);
        if (local_138 != (char *)0x0) {
          FUN_14019f2c0(local_138 + -0x10);
        }
        local_138 = (char *)*puVar3;
        *puVar3 = 0;
        lVar4 = local_c0;
      }
      if (lVar4 != 0) {
        FUN_14019f2c0(lVar4 + -0x10);
      }
      iVar2 = 0;
      if (((local_138 != (char *)0x0) && (*local_138 != '\0')) &&
         (lVar4 = FUN_142ef83e0(&DAT_143273878,
                                (int)local_138[(longlong)*(int *)(local_138 + -8) + -1]), lVar4 != 0
         )) {
        pcVar5 = (char *)FUN_14019bd40(&local_138,0,1);
        if (local_138 != (char *)0x0) {
          iVar2 = *(int *)(local_138 + -8);
        }
        pcVar8 = pcVar5 + (longlong)iVar2 + -2;
        if (pcVar8 < pcVar5) {
LAB_14271a935:
          pcVar5 = local_138;
          if (*(int *)(local_138 + -0x10) != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (*(int *)(pcVar5 + -0xc) < 0) {
            FUN_142e54290(0x90,*(int *)(pcVar5 + -0xc),0);
          }
          pcVar5[-0x10] = '\x01';
          pcVar5[-0xf] = '\0';
          pcVar5[-0xe] = '\0';
          pcVar5[-0xd] = '\0';
          *local_138 = '\0';
          if (*(int *)(pcVar5 + -0xc) + 1 < 1) {
            FUN_142e54290(0x9c,0);
          }
          pcVar5[-8] = '\0';
          pcVar5[-7] = '\0';
          pcVar5[-6] = '\0';
          pcVar5[-5] = '\0';
          if (local_138 != (char *)0x0) {
            FUN_14019f2c0(local_138 + -0x10);
            local_138 = (char *)0x0;
          }
        }
        else {
          while (lVar4 = FUN_142ef83e0(&DAT_143273878,(int)*pcVar8), pcVar1 = local_138, lVar4 != 0)
          {
            pcVar8 = pcVar8 + -1;
            if (pcVar8 < pcVar5) goto LAB_14271a935;
          }
          if (pcVar8 < pcVar5) goto LAB_14271a935;
          pcVar8[1] = '\0';
          uVar13 = (int)(pcVar8 + 1) - (int)pcVar5;
          if (*(int *)(local_138 + -0x10) != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((uVar13 == 0xffffffff) || ((int)uVar13 <= *(int *)(pcVar1 + -0xc))) {
            pcVar1[-0x10] = '\x01';
            pcVar1[-0xf] = '\0';
            pcVar1[-0xe] = '\0';
            pcVar1[-0xd] = '\0';
            if (uVar13 == 0xffffffff) {
              piVar9 = piVar11;
              if (pcVar1 != (char *)0x0) {
                piVar9 = (int *)0xffffffffffffffff;
                do {
                  piVar9 = (int *)((longlong)piVar9 + 1);
                } while (pcVar1[(longlong)piVar9] != '\0');
              }
              goto LAB_14271aa7b;
            }
          }
          else {
            FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),uVar13);
            pcVar1[-0x10] = '\x01';
            pcVar1[-0xf] = '\0';
            pcVar1[-0xe] = '\0';
            pcVar1[-0xd] = '\0';
          }
          local_138[(int)uVar13] = '\0';
          piVar9 = (int *)(ulonglong)uVar13;
LAB_14271aa7b:
          iVar2 = (int)piVar9;
          if ((iVar2 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar2)) {
            FUN_142e54290(0x9c,(ulonglong)piVar9 & 0xffffffff);
          }
          *(int *)(pcVar1 + -8) = iVar2;
        }
      }
      FUN_1401d12a0(&local_138,0);
      iVar2 = *(int *)(param_1 + 0x13d0);
      if (0 < iVar2) {
        uVar6 = FUN_1429fbeb0(local_b8,0x2a);
        FUN_1429eb300(&local_138,uVar6,iVar2);
      }
      pIVar10 = local_e0;
      local_d8 = local_e0;
      if (local_e0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_100);
      if (DAT_143a8b8d8 == 8) {
        if (local_100 == 8) {
          local_100 = 0;
          if (lStack_f8 != 0) {
            (*DAT_143ad5990)(lStack_f8 + -4);
          }
        }
        else {
          iVar2 = (*DAT_143262a18)(&local_100);
          if (iVar2 < 0) goto LAB_14271afb0;
        }
        local_100 = 8;
        if (DAT_143a8b8e0 == 0) {
          lStack_f8 = FUN_1401a5fa0(0,0);
        }
        else {
          lStack_f8 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
      }
      else {
        if ((local_100 == 8) && (local_100 = 0, lStack_f8 != 0)) {
          (*DAT_143ad5990)(lStack_f8 + -4);
        }
        iVar2 = (*DAT_143262a28)(&local_100,&DAT_143a8b8d8);
        if (iVar2 < 0) {
LAB_14271afb0:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
      }
      (*DAT_143262a20)(&local_118);
      if (DAT_143a8b8d8 == 8) {
        if (local_118 == 8) {
          local_118 = 0;
          if (lStack_110 != 0) {
            (*DAT_143ad5990)(lStack_110 + -4);
          }
        }
        else {
          iVar2 = (*DAT_143262a18)(&local_118);
          if (iVar2 < 0) goto LAB_14271afa8;
        }
        local_118 = 8;
        if (DAT_143a8b8e0 == 0) {
          lStack_110 = FUN_1401a5fa0(0,0);
        }
        else {
          lStack_110 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
      }
      else {
        if ((local_118 == 8) && (local_118 = 0, lStack_110 != 0)) {
          (*DAT_143ad5990)(lStack_110 + -4);
        }
        iVar2 = (*DAT_143262a28)(&local_118,&DAT_143a8b8d8);
        if (iVar2 < 0) {
LAB_14271afa8:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
      }
      (*DAT_143262a20)(&local_130);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_130 == 8) {
          local_130 = (uint)local_130._2_2_ << 0x10;
          if (uStack_128 != 0) {
            (*DAT_143ad5990)(uStack_128 + -4);
          }
        }
        else {
          iVar2 = (*DAT_143262a18)(&local_130);
          if (iVar2 < 0) goto LAB_14271afa0;
        }
        local_130 = CONCAT22(local_130._2_2_,8);
        if (DAT_143a8b8e0 == 0) {
          uStack_128 = FUN_1401a5fa0(0,0);
        }
        else {
          uStack_128 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
      }
      else {
        if (((short)local_130 == 8) && (local_130 = (uint)local_130._2_2_ << 0x10, uStack_128 != 0))
        {
          (*DAT_143ad5990)(uStack_128 + -4);
        }
        iVar2 = (*DAT_143262a28)(&local_130,&DAT_143a8b8d8);
        if (iVar2 < 0) {
LAB_14271afa0:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
      }
      puVar3 = (undefined8 *)FUN_1429fbeb0(&local_b0,0x2a);
      pcVar5 = local_138;
      in_stack_fffffffffffffe98 = *puVar3;
      local_a8 = &local_e8;
      puVar3 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
      local_a0 = puVar3;
      if (puVar3 == (undefined8 *)0x0) {
        local_e8 = (undefined8 *)0x0;
      }
      else {
        puVar3[1] = 0;
        *(undefined4 *)(puVar3 + 2) = 1;
        if (pcVar5 == (char *)0x0) {
          *puVar3 = 0;
          local_e8 = puVar3;
          pIVar10 = local_d8;
        }
        else {
          uVar15 = (ulonglong)in_stack_fffffffffffffea0 & 0xffffffff00000000;
          iVar2 = (*DAT_1432627f8)(0xfde9,0,pcVar5,0xffffffff,0,uVar15);
          uVar16 = (undefined4)(uVar15 >> 0x20);
          uVar15 = (ulonglong)(longlong)(iVar2 * 2) >> 1;
          iVar2 = (int)uVar15;
          uVar13 = iVar2 - 1;
          piVar7 = (int *)(*DAT_143ad5980)((uVar15 & 0xffffffff) * 2 + 4);
          piVar9 = piVar11;
          if (piVar7 != (int *)0x0) {
            *piVar7 = uVar13 * 2;
            *(undefined2 *)((longlong)(piVar7 + 1) + (ulonglong)uVar13 * 2) = 0;
            piVar9 = piVar7 + 1;
          }
          (*DAT_1432627f8)(0xfde9,0,pcVar5,0xffffffff,piVar9,CONCAT44(uVar16,iVar2));
          *puVar3 = piVar9;
          local_e8 = puVar3;
          pIVar10 = local_d8;
          iVar12 = local_res18[0];
        }
      }
      if (local_e8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      local_d8 = (IUnknown *)&local_e8;
      lVar4 = *(longlong *)(param_1 + 0x280);
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = *(longlong *)(param_1 + 0x280);
      }
      local_res18[0] = 0;
      piVar9 = piVar11;
      if (local_e8 != (undefined8 *)0x0) {
        piVar9 = (int *)*local_e8;
      }
      local_98 = CONCAT62(uStack_fe,local_100);
      lStack_90 = lStack_f8;
      local_88 = local_f0;
      local_78 = CONCAT62(uStack_116,local_118);
      lStack_70 = lStack_110;
      local_68 = local_108;
      local_58 = local_130;
      uStack_54 = uStack_12c;
      uStack_50 = (undefined4)uStack_128;
      uStack_4c = uStack_128._4_4_;
      local_48 = local_120;
      in_stack_fffffffffffffea0 = &local_58;
      iVar2 = (**(code **)(*(longlong *)pIVar10 + 0x1a8))
                        (pIVar10,0xc,(iVar12 - *(int *)(lVar4 + 0x80)) * 0xd + 0x1e,piVar9,
                         in_stack_fffffffffffffe98,in_stack_fffffffffffffea0,&local_78,&local_98,
                         local_res18);
      if (iVar2 < 0) {
        _com_issue_errorex(iVar2,pIVar10,(_GUID *)&DAT_14327ac98);
      }
      FUN_1401be120(&local_e8);
      if (local_b0 != (longlong *)0x0) {
        (**(code **)(*local_b0 + 0x10))();
      }
      if ((short)local_130 == 8) {
        local_130 = local_130 & 0xffff0000;
        if (uStack_128 != 0) {
          (*DAT_143ad5990)(uStack_128 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_130);
      }
      if (local_118 == 8) {
        local_118 = 0;
        if (lStack_110 != 0) {
          (*DAT_143ad5990)(lStack_110 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_118);
      }
      if (local_100 == 8) {
        local_100 = 0;
        if (lStack_f8 != 0) {
          (*DAT_143ad5990)(lStack_f8 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_100);
      }
      uVar15 = (ulonglong)(iVar12 + 1);
      uVar14 = local_d0 + 1;
      iVar2 = local_res20;
    }
  }
  if (local_138 != (char *)0x0) {
    FUN_14019f2c0(local_138 + -0x10);
  }
  if (local_e0 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_e0 + 0x10))();
  }
  return;
}



//===========================================================
// FUN_14271a650 @ 14271a650   (49 bytes)
//===========================================================

void FUN_14271a650(longlong param_1)

{
  if (*(longlong *)(param_1 + 0x290) != 0) {
    FUN_142bf3f70();
    FUN_1423c85e0(param_1 + 0x288);
    return;
  }
  return;
}



//===========================================================
// thunk_FUN_142bf5fe0 @ 14271b330   (5 bytes)
//===========================================================

void thunk_FUN_142bf5fe0(void)

{
  FUN_142bf5fe0();
  return;
}


