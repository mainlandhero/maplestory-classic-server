
//===========================================================
// FUN_142105340 @ 142105340   (11226 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142105e3a) */
/* WARNING: Removing unreachable block (ram,0x000142105c1c) */
/* WARNING: Removing unreachable block (ram,0x000142106b1e) */
/* WARNING: Removing unreachable block (ram,0x00014210682e) */
/* WARNING: Removing unreachable block (ram,0x000142106d75) */

undefined8 *
FUN_142105340(undefined8 *param_1,int param_2,uint param_3,uint param_4,undefined8 param_5,
             longlong *param_6,uint param_7,undefined4 param_8,undefined4 param_9,int param_10,
             uint *param_11,undefined4 param_12,longlong param_13)

{
  char cVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  long lVar4;
  int iVar5;
  int iVar6;
  undefined8 uVar7;
  longlong *plVar8;
  longlong lVar9;
  IUnknown *pIVar10;
  IUnknown *pIVar11;
  IUnknown **ppIVar12;
  longlong *plVar13;
  undefined8 *puVar14;
  IUnknown *pIVar15;
  IUnknown *pIVar16;
  uint uVar17;
  uint uVar18;
  IUnknown *pIVar19;
  uint *in_stack_fffffffffffffe48;
  short *psVar20;
  ulonglong uVar21;
  ulonglong in_stack_fffffffffffffe50;
  ulonglong uVar22;
  undefined8 in_stack_fffffffffffffe58;
  undefined4 uVar24;
  undefined4 *puVar23;
  ulonglong in_stack_fffffffffffffe60;
  undefined8 in_stack_fffffffffffffe68;
  undefined4 uVar25;
  ulonglong in_stack_fffffffffffffe70;
  uint local_178;
  undefined4 uStack_174;
  undefined8 uStack_170;
  undefined8 local_168;
  IUnknown *local_160;
  uint local_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 local_148;
  uint local_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined8 local_130;
  short local_128;
  undefined6 uStack_126;
  longlong lStack_120;
  undefined8 local_118;
  IUnknown *local_110;
  IUnknown *local_108;
  undefined8 local_100;
  uint local_f8;
  IUnknown *local_f0;
  uint local_e8;
  uint local_e0;
  undefined4 uStack_dc;
  IUnknown *local_d8;
  undefined4 local_d0;
  undefined4 uStack_cc;
  undefined8 uStack_c8;
  undefined8 local_c0;
  IUnknown *local_b8;
  int local_b0;
  undefined4 uStack_ac;
  uint local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  IUnknown *local_88;
  longlong *local_80;
  ulonglong local_78;
  longlong lStack_70;
  undefined8 local_68;
  uint *local_58;
  longlong *local_50;
  undefined8 *local_48;
  longlong *local_40;
  
  uVar24 = (undefined4)((ulonglong)in_stack_fffffffffffffe58 >> 0x20);
  uVar25 = (undefined4)((ulonglong)in_stack_fffffffffffffe68 >> 0x20);
  local_108 = (IUnknown *)CONCAT44(local_108._4_4_,param_4);
  local_50 = param_6;
  local_40 = param_6;
  local_e0 = param_7;
  local_58 = param_11;
  local_100 = (IUnknown *)((ulonglong)local_100 & 0xffffffff00000000);
  param_1[3] = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  *param_1 = &PTR_FUN_14342f8e8;
  local_f8 = param_4;
  local_b0 = param_2;
  local_48 = param_1;
  FUN_1402c23d0(param_1 + 4,param_5,0x3e0);
  *(int *)((longlong)param_1 + 0x9c) = param_2;
  *(uint *)(param_1 + 0x14) = param_3;
  *(uint *)((longlong)param_1 + 0xa4) = param_7;
  *(undefined4 *)(param_1 + 0x15) = param_12;
  param_1[0x16] = 0;
  FUN_14019a260(param_1 + 0x16,param_6);
  param_1[0x17] = 0;
  param_1[0x18] = 0;
  param_1[0x19] = 0;
  param_1[0x1a] = 0;
  param_1[0x1b] = 0;
  param_1[0x1c] = 0xffffffffffffffff;
  *(undefined4 *)(param_1 + 0x1d) = param_8;
  *(undefined4 *)((longlong)param_1 + 0xec) = param_9;
  param_1[0x1e] = 0;
  *(undefined1 *)(param_1 + 0x1f) = 0;
  *(undefined4 *)((longlong)param_1 + 0xfc) = 1;
  *(undefined2 *)(param_1 + 0x20) = 0;
  *(undefined8 *)((longlong)param_1 + 0x104) = 0;
  *(undefined8 *)((longlong)param_1 + 0x10c) = 0;
  *(undefined2 *)((longlong)param_1 + 0x114) = 0;
  *(undefined1 *)(param_1 + 0x23) = 0;
  *(undefined4 *)((longlong)param_1 + 0x11c) = 0;
  local_100 = (IUnknown *)CONCAT44(local_100._4_4_,param_4);
  *local_58 = param_4;
  plVar8 = DAT_143aa84a0;
  pIVar19 = (IUnknown *)0x0;
  local_88 = (IUnknown *)0x0;
  local_e8 = param_4;
  uVar7 = FUN_142cbe730(DAT_143aa84a0);
  (**(code **)(*plVar8 + 0x30))(plVar8);
  uVar2 = (**(code **)(*plVar8 + 0xa8))(plVar8);
  uVar3 = (**(code **)(*plVar8 + 0xb0))(plVar8);
  uVar17 = local_e8;
  uVar18 = local_f8;
  switch(local_b0) {
  case 1:
  case 6:
    lVar9 = FUN_1403c84b0(DAT_143aa8328);
    if ((lVar9 == 0) || (*(int *)(lVar9 + 0x3fc) == 0)) {
      (*DAT_143262a20)(&local_128);
      if (DAT_143a8b8d8 == 8) {
        if (local_128 == 8) {
          local_128 = 0;
          if (lStack_120 != 0) {
            (*DAT_143ad5990)(lStack_120 + -4);
          }
        }
        else {
          iVar5 = (*DAT_143262a18)(&local_128);
          if (iVar5 < 0) goto LAB_142107d88;
        }
        local_128 = 8;
        if (DAT_143a8b8e0 == 0) {
          uVar17 = 0;
        }
        else {
          uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
      }
      else {
        if ((local_128 == 8) && (local_128 = 0, lStack_120 != 0)) {
          (*DAT_143ad5990)(lStack_120 + -4);
        }
        iVar5 = (*DAT_143262a28)(&local_128,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_142107d88:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
      }
      plVar8 = (longlong *)
               FUN_141028310(&local_110,0x20,0x20,&local_128,
                             (ulonglong)in_stack_fffffffffffffe48 & 0xffffffffffffff00);
      pIVar16 = (IUnknown *)*plVar8;
      pIVar15 = (IUnknown *)0x0;
      if (pIVar16 != (IUnknown *)0x0) {
        *plVar8 = 0;
        pIVar19 = pIVar16;
        local_88 = pIVar16;
      }
      if (local_110 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_110 + 0x10))();
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
      pIVar16 = DAT_143add058;
      if (pIVar19 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_178 = CONCAT22(local_178._2_2_,3);
      uStack_170 = (longlong *)CONCAT44(uStack_170._4_4_,0xff);
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
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
          iVar5 = (*DAT_143262a18)(&local_128);
          if (iVar5 < 0) goto LAB_142107d90;
        }
        local_128 = 8;
        pIVar11 = pIVar15;
        if (DAT_143a8b8e0 != 0) {
          pIVar11 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar11);
      }
      else {
        if ((local_128 == 8) && (local_128 = 0, lStack_120 != 0)) {
          (*DAT_143ad5990)(lStack_120 + -4);
        }
        iVar5 = (*DAT_143262a28)(&local_128,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_142107d90:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
      }
      (*DAT_143262a20)(&local_d0);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_d0 == 8) {
          local_d0 = (uint)local_d0._2_2_ << 0x10;
          if (uStack_c8 != 0) {
            (*DAT_143ad5990)(uStack_c8 + -4);
          }
        }
        else {
          iVar5 = (*DAT_143262a18)(&local_d0);
          if (iVar5 < 0) goto LAB_142107d98;
        }
        local_d0 = CONCAT22(local_d0._2_2_,8);
        pIVar11 = pIVar15;
        if (DAT_143a8b8e0 != 0) {
          pIVar11 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_c8 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar11);
      }
      else {
        if (((short)local_d0 == 8) && (local_d0 = (uint)local_d0._2_2_ << 0x10, uStack_c8 != 0)) {
          (*DAT_143ad5990)(uStack_c8 + -4);
        }
        iVar5 = (*DAT_143262a28)(&local_d0,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_142107d98:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
      }
      pIVar10 = (IUnknown *)FUN_1401a5890(&local_110,PTR_u_UI_BuffIcon_img_IconBase_0_143a47120);
      local_160 = pIVar10;
      (*DAT_143262a20)(&local_140);
      pIVar11 = pIVar15;
      if (*(undefined8 **)pIVar10 != (undefined8 *)0x0) {
        pIVar11 = (IUnknown *)**(undefined8 **)pIVar10;
      }
      local_78 = CONCAT62(uStack_126,local_128);
      lStack_70 = lStack_120;
      local_68 = local_118;
      local_a8 = local_d0;
      uStack_a4 = uStack_cc;
      uStack_a0 = (undefined4)uStack_c8;
      uStack_9c = uStack_c8._4_4_;
      local_98 = local_c0;
      iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x48))
                        (pIVar16,pIVar11,&local_a8,&local_78,&local_140);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_1432743e8);
      }
      local_158 = local_140;
      uStack_154 = uStack_13c;
      uStack_150 = uStack_138;
      uStack_14c = uStack_134;
      local_148 = local_130;
      local_140 = local_140 & 0xffff0000;
      uVar17 = 0x10;
      FUN_1401be120(pIVar10);
      uVar7 = FUN_1409339d0(&local_100,&local_158);
      puVar14 = (undefined8 *)FUN_1403ee040(&local_108,uVar7);
      local_a8 = local_178;
      uStack_a4 = uStack_174;
      uStack_a0 = (undefined4)uStack_170;
      uStack_9c = uStack_170._4_4_;
      local_98 = local_168;
      in_stack_fffffffffffffe48 = &local_a8;
      iVar5 = (**(code **)(*(longlong *)pIVar19 + 0x128))
                        (pIVar19,0,0,*puVar14,in_stack_fffffffffffffe48);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar19,(_GUID *)&DAT_14327ac98);
      }
      if (local_108 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_108 + 0x10))();
      }
      if (local_100 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_100 + 0x10))();
      }
      if ((short)local_158 == 8) {
        local_158 = local_158 & 0xffff0000;
        if (CONCAT44(uStack_14c,uStack_150) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_158);
      }
      if ((short)local_d0 == 8) {
        local_d0 = local_d0 & 0xffff0000;
        if (uStack_c8 != 0) {
          (*DAT_143ad5990)(uStack_c8 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_d0);
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
      if ((short)local_178 == 8) {
        local_178 = local_178 & 0xffff0000;
        if (uStack_170 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_170 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_178);
      }
      if (*(int *)((longlong)param_1 + 0xec) != 0) {
        FUN_14039f600(DAT_143aa8328,&local_108,*(int *)((longlong)param_1 + 0xec),0);
        pIVar16 = local_108;
        local_b8 = (IUnknown *)0x0;
        if (local_108 == (IUnknown *)0x0) {
LAB_142105d3d:
          *(undefined4 *)((longlong)param_1 + 0xec) = 0;
        }
        else {
          pIVar11 = (IUnknown *)FUN_1401a5890(&local_110,PTR_u_iconRaw_143a45dd0);
          local_160 = pIVar11;
          (*DAT_143262a20)(&local_140);
          uVar7 = 0;
          if (*(undefined8 **)pIVar11 != (undefined8 *)0x0) {
            uVar7 = **(undefined8 **)pIVar11;
          }
          iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x28))(pIVar16,uVar7,&local_140);
          if (iVar5 < 0) {
            _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_143272478);
          }
          local_158 = local_140;
          uStack_154 = uStack_13c;
          uStack_150 = uStack_138;
          uStack_14c = uStack_134;
          local_148 = local_130;
          local_140 = local_140 & 0xffff0000;
          uVar17 = 0x30;
          FUN_1401be120(pIVar11);
          uVar7 = FUN_1409339d0(&local_f0,&local_158);
          FUN_1403ee040(&local_100,uVar7);
          if (local_100 != (IUnknown *)0x0) {
            local_b8 = local_100;
            pIVar15 = local_100;
          }
          if (local_f0 != (IUnknown *)0x0) {
            (**(code **)(*(longlong *)local_f0 + 0x10))();
          }
          if ((short)local_158 == 8) {
            local_158 = local_158 & 0xffff0000;
            if (CONCAT44(uStack_14c,uStack_150) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_158);
          }
          if (pIVar15 == (IUnknown *)0x0) goto LAB_142105d3d;
          local_178 = CONCAT22(local_178._2_2_,3);
          uStack_170 = (longlong *)CONCAT44(uStack_170._4_4_,0xff);
          iVar5 = FUN_140404240(pIVar15);
          iVar5 = (0x20 - iVar5) / 2;
          if (iVar5 < 0) {
            iVar5 = 0;
          }
          iVar6 = FUN_140404280(pIVar15);
          iVar6 = (0x20 - iVar6) / 2;
          if (iVar6 < 0) {
            iVar6 = 0;
          }
          local_a8 = local_178;
          uStack_a4 = uStack_174;
          uStack_a0 = (undefined4)uStack_170;
          uStack_9c = uStack_170._4_4_;
          local_98 = local_168;
          in_stack_fffffffffffffe48 = &local_a8;
          iVar5 = (**(code **)(*(longlong *)pIVar19 + 0x128))
                            (pIVar19,iVar6,iVar5,pIVar15,in_stack_fffffffffffffe48);
          if (iVar5 < 0) {
            _com_issue_errorex(iVar5,pIVar19,(_GUID *)&DAT_14327ac98);
          }
          if ((short)local_178 == 8) {
            local_178 = local_178 & 0xffff0000;
            if (uStack_170 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)uStack_170 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_178);
          }
        }
        if (pIVar15 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)pIVar15 + 0x10))(pIVar15);
        }
        if (local_108 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_108 + 0x10))(local_108);
        }
      }
      pIVar15 = (IUnknown *)0x0;
      FUN_14039f600(DAT_143aa8328,&local_e0,param_3,0);
      local_d8 = (IUnknown *)0x0;
      pIVar16 = (IUnknown *)CONCAT44(uStack_dc,local_e0);
      plVar8 = (longlong *)0x0;
      if (pIVar16 != (IUnknown *)0x0) {
        pIVar11 = (IUnknown *)FUN_1401a5890(&local_f0,PTR_u_iconRaw_143a45dd0);
        local_160 = pIVar11;
        (*DAT_143262a20)(&local_140);
        uVar7 = 0;
        if (*(undefined8 **)pIVar11 != (undefined8 *)0x0) {
          uVar7 = **(undefined8 **)pIVar11;
        }
        lVar4 = (**(code **)(*(longlong *)pIVar16 + 0x28))(pIVar16,uVar7,&local_140);
        if (lVar4 < 0) {
          _com_issue_errorex(lVar4,pIVar16,(_GUID *)&DAT_143272478);
        }
        local_158 = local_140;
        uStack_154 = uStack_13c;
        uStack_150 = uStack_138;
        uStack_14c = uStack_134;
        local_148 = local_130;
        local_140 = local_140 & 0xffff0000;
        uVar17 = uVar17 | 0x40;
        FUN_1401be120(pIVar11);
        uVar7 = FUN_1409339d0(&local_110,&local_158);
        FUN_1403ee040(&local_b8,uVar7);
        if (local_b8 != (IUnknown *)0x0) {
          local_d8 = local_b8;
          pIVar15 = local_b8;
        }
        if (local_110 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_110 + 0x10))();
        }
        if ((short)local_158 == 8) {
          local_158 = local_158 & 0xffff0000;
          if (CONCAT44(uStack_14c,uStack_150) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_158);
        }
        if (pIVar15 != (IUnknown *)0x0) {
          local_178 = CONCAT22(local_178._2_2_,3);
          uStack_170 = (longlong *)CONCAT44(uStack_170._4_4_,0xff);
          iVar5 = FUN_140404240(pIVar15);
          iVar5 = (0x20 - iVar5) / 2;
          if (iVar5 < 0) {
            iVar5 = 0;
          }
          iVar6 = FUN_140404280(pIVar15);
          iVar6 = (0x20 - iVar6) / 2;
          if (iVar6 < 0) {
            iVar6 = 0;
          }
          local_a8 = local_178;
          uStack_a4 = uStack_174;
          uStack_a0 = (undefined4)uStack_170;
          uStack_9c = uStack_170._4_4_;
          local_98 = local_168;
          in_stack_fffffffffffffe48 = &local_a8;
          iVar5 = (**(code **)(*(longlong *)pIVar19 + 0x128))
                            (pIVar19,iVar6,iVar5,pIVar15,in_stack_fffffffffffffe48);
          if (iVar5 < 0) {
            _com_issue_errorex(iVar5,pIVar19,(_GUID *)&DAT_14327ac98);
          }
          if ((short)local_178 == 8) {
            local_178 = local_178 & 0xffff0000;
            if (uStack_170 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)uStack_170 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_178);
          }
        }
        plVar8 = (longlong *)CONCAT44(uStack_dc,local_e0);
      }
      if (local_b0 == 1) {
        FUN_14039e630(DAT_143aa8328,&local_110,param_3);
        pIVar16 = local_110;
        if (local_110 == (IUnknown *)0x0) {
          local_108 = (IUnknown *)0x0;
          ppIVar12 = &local_108;
          uVar17 = uVar17 | 4;
        }
        else {
          pIVar11 = (IUnknown *)FUN_1401a5890(&local_f0,PTR_u_spec_143a46448);
          local_160 = pIVar11;
          (*DAT_143262a20)(&local_140);
          uVar7 = 0;
          if (*(undefined8 **)pIVar11 != (undefined8 *)0x0) {
            uVar7 = **(undefined8 **)pIVar11;
          }
          lVar4 = (**(code **)(*(longlong *)pIVar16 + 0x28))(pIVar16,uVar7,&local_140);
          if (lVar4 < 0) {
            _com_issue_errorex(lVar4,pIVar16,(_GUID *)&DAT_143272478);
          }
          local_158 = local_140;
          uStack_154 = uStack_13c;
          uStack_150 = uStack_138;
          uStack_14c = uStack_134;
          local_148 = local_130;
          local_140 = local_140 & 0xffff0000;
          FUN_1401be120(pIVar11);
          local_100 = (IUnknown *)(CONCAT44(local_100._4_4_,uVar17) | 0x81);
          ppIVar12 = (IUnknown **)FUN_1409339d0(&local_b8,&local_158);
          uVar17 = uVar17 | 0x83;
        }
        pIVar16 = local_108;
        local_100 = (IUnknown *)CONCAT44(local_100._4_4_,uVar17);
        FUN_1401a5040(&local_160,ppIVar12);
        uVar18 = uVar17;
        if ((uVar17 & 4) != 0) {
          uVar18 = uVar17 & 0xfffffffb;
          local_100 = (IUnknown *)(CONCAT44(local_100._4_4_,uVar17) & 0xfffffffffffffffb);
          if (pIVar16 != (IUnknown *)0x0) {
            (**(code **)(*(longlong *)pIVar16 + 0x10))(pIVar16);
          }
        }
        uVar17 = uVar18;
        if ((uVar18 & 2) != 0) {
          uVar17 = uVar18 & 0xfffffffd;
          local_100 = (IUnknown *)(CONCAT44(local_100._4_4_,uVar18) & 0xfffffffffffffffd);
          if (local_b8 != (IUnknown *)0x0) {
            (**(code **)(*(longlong *)local_b8 + 0x10))();
          }
        }
        if ((uVar17 & 1) != 0) {
          uVar17 = uVar17 & 0xfffffffe;
          if ((short)local_158 == 8) {
            local_158 = local_158 & 0xffff0000;
            if (CONCAT44(uStack_14c,uStack_150) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_158);
          }
        }
        pIVar16 = local_160;
        if (local_160 != (IUnknown *)0x0) {
          pIVar11 = (IUnknown *)FUN_1401a5890(&local_108,PTR_u_time_143a464e0);
          local_100 = pIVar11;
          (*DAT_143262a20)(&local_140);
          uVar7 = 0;
          if (*(undefined8 **)pIVar11 != (undefined8 *)0x0) {
            uVar7 = **(undefined8 **)pIVar11;
          }
          lVar4 = (**(code **)(*(longlong *)pIVar16 + 0x28))(pIVar16,uVar7,&local_140);
          if (lVar4 < 0) {
            _com_issue_errorex(lVar4,pIVar16,(_GUID *)&DAT_143272478);
          }
          local_178 = local_140;
          uStack_174 = uStack_13c;
          uStack_170 = (longlong *)CONCAT44(uStack_134,uStack_138);
          local_168 = local_130;
          local_140 = local_140 & 0xffff0000;
          uVar17 = uVar17 | 0x100;
          FUN_1401be120(pIVar11);
          local_e8 = FUN_14022ee40(&local_178,local_f8);
          if ((short)local_178 == 8) {
            local_178 = local_178 & 0xffff0000;
            if (uStack_170 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)uStack_170 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_178);
          }
        }
        FUN_14039f600(DAT_143aa8328,&local_b0,param_3,0);
        pIVar11 = (IUnknown *)CONCAT44(uStack_ac,local_b0);
        if (pIVar11 == (IUnknown *)0x0) {
          uVar18 = 0;
          plVar8 = (longlong *)0x0;
        }
        else {
          pIVar10 = (IUnknown *)FUN_1401a5890(&local_f0,PTR_u_noShadow_143a48a48);
          local_b8 = pIVar10;
          (*DAT_143262a20)(&local_140);
          uVar7 = 0;
          if (*(undefined8 **)pIVar10 != (undefined8 *)0x0) {
            uVar7 = **(undefined8 **)pIVar10;
          }
          iVar5 = (**(code **)(*(longlong *)pIVar11 + 0x28))(pIVar11,uVar7,&local_140);
          if (iVar5 < 0) {
            _com_issue_errorex(iVar5,pIVar11,(_GUID *)&DAT_143272478);
          }
          local_158 = local_140;
          uStack_154 = uStack_13c;
          uStack_150 = uStack_138;
          uStack_14c = uStack_134;
          local_148 = local_130;
          local_140 = local_140 & 0xffff0000;
          FUN_1401be120(pIVar10);
          local_100 = (IUnknown *)(CONCAT44(local_100._4_4_,uVar17) | 0x208);
          iVar5 = FUN_14022ee40(&local_158,0);
          uVar18 = (uint)(iVar5 != 0);
          plVar8 = (longlong *)CONCAT44(uStack_ac,local_b0);
          uVar17 = 0x208;
        }
        *(uint *)((longlong)param_1 + 0x104) = uVar18;
        if ((uVar17 & 8) != 0) {
          if ((short)local_158 == 8) {
            local_158 = local_158 & 0xffff0000;
            if (CONCAT44(uStack_14c,uStack_150) == 0) goto LAB_142106260;
            (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
          }
          else {
            (*DAT_143262a18)(&local_158);
          }
          plVar8 = (longlong *)CONCAT44(uStack_ac,local_b0);
        }
LAB_142106260:
        if (plVar8 != (longlong *)0x0) {
          (**(code **)(*plVar8 + 0x10))(plVar8);
        }
        if (pIVar16 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)pIVar16 + 0x10))(pIVar16);
        }
        if (local_110 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_110 + 0x10))();
        }
        plVar8 = (longlong *)CONCAT44(uStack_dc,local_e0);
      }
      else if (local_b0 == 6) {
        *(undefined4 *)((longlong)param_1 + 0x104) = 1;
      }
      *(undefined4 *)((longlong)param_1 + 0xfc) = 2;
      if (pIVar15 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar15 + 0x10))(pIVar15);
        plVar8 = (longlong *)CONCAT44(uStack_dc,local_e0);
      }
      uVar17 = local_e8;
      uVar18 = local_f8;
      if (plVar8 != (longlong *)0x0) {
        (**(code **)(*plVar8 + 0x10))(plVar8);
        uVar17 = local_e8;
        uVar18 = local_f8;
      }
    }
    else {
      in_stack_fffffffffffffe48 = (uint *)&local_100;
      plVar8 = (longlong *)
               FUN_142109b80(param_1,&local_110,*(int *)(lVar9 + 0x3fc),&local_108,
                             in_stack_fffffffffffffe48);
      pIVar16 = (IUnknown *)*plVar8;
      if (pIVar16 != (IUnknown *)0x0) {
        *plVar8 = 0;
        pIVar19 = pIVar16;
        local_88 = pIVar16;
      }
      if (local_110 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_110 + 0x10))();
      }
      uVar17 = (uint)local_100;
      uVar18 = (uint)local_108;
    }
    break;
  case 2:
  case 5:
  case 8:
    if (local_e0 != 0) {
      param_3 = local_e0;
    }
    in_stack_fffffffffffffe48 = (uint *)&local_100;
    plVar8 = (longlong *)
             FUN_142109b80(param_1,&local_110,param_3,&local_108,in_stack_fffffffffffffe48);
    pIVar16 = (IUnknown *)*plVar8;
    if (pIVar16 != (IUnknown *)0x0) {
      *plVar8 = 0;
      pIVar19 = pIVar16;
      local_88 = pIVar16;
    }
    if (local_110 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_110 + 0x10))();
    }
    plVar8 = *(longlong **)(param_13 + 0x38);
    if ((plVar8 == (longlong *)0x0) ||
       (local_e0 = param_3, cVar1 = (**(code **)(*plVar8 + 0x10))(plVar8,&local_e0), cVar1 == '\0'))
    {
      iVar5 = (int)param_3 / 10000;
      if (iVar5 == 8000) {
        iVar5 = (int)param_3 / 100;
      }
      if ((((iVar5 - 800000U < 100) || (param_3 == 0x56c90be)) || (param_3 == 0x56c90bf)) ||
         ((param_3 == 0x56c90c0 || (param_3 == 0x56c90c1)))) {
        *(undefined4 *)((longlong)param_1 + 0xfc) = 1;
        uVar17 = (uint)local_100;
        uVar18 = (uint)local_108;
      }
      else {
        iVar5 = FUN_1407b3df0(DAT_143aa84d0,uVar7,param_3,0);
        if ((0 < iVar5) || (cVar1 = FUN_1407eba30(param_3,uVar2,uVar3), cVar1 != '\0'))
        goto LAB_142105618;
        *(undefined4 *)((longlong)param_1 + 0xfc) = 3;
        uVar17 = (uint)local_100;
        uVar18 = (uint)local_108;
      }
    }
    else {
LAB_142105618:
      *(undefined4 *)((longlong)param_1 + 0xfc) = 0;
      uVar17 = (uint)local_100;
      uVar18 = (uint)local_108;
    }
    break;
  case 3:
  case 7:
    (*DAT_143262a20)(&local_158);
    iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
    if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    puVar14 = (undefined8 *)
              FUN_141028310(&local_160,0x20,0x20,&local_158,
                            (ulonglong)in_stack_fffffffffffffe48 & 0xffffffffffffff00);
    pIVar16 = (IUnknown *)*puVar14;
    pIVar15 = (IUnknown *)0x0;
    if (pIVar16 != (IUnknown *)0x0) {
      *puVar14 = 0;
      pIVar19 = pIVar16;
      local_88 = pIVar16;
    }
    if (local_160 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_160 + 0x10))();
    }
    if ((short)local_158 == 8) {
      local_158 = local_158 & 0xffff0000;
      if (CONCAT44(uStack_14c,uStack_150) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_158);
    }
    pIVar16 = DAT_143add058;
    if (pIVar19 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_128 = 3;
    lStack_120 = CONCAT44(lStack_120._4_4_,0xff);
    if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_178);
    iVar5 = FUN_14023c4c0(&local_178,&DAT_143a8b8d8);
    if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    (*DAT_143262a20)(&local_158);
    iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
    if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    uVar7 = FUN_1401a5890(&local_160,PTR_u_UI_BuffIcon_img_IconBase_0_143a47120);
    uVar7 = FUN_140403ec0(pIVar16,&local_140,uVar7,&local_158,&local_178);
    uVar7 = FUN_1409339d0(&local_f0,uVar7);
    puVar14 = (undefined8 *)FUN_1403ee040(&local_d8,uVar7);
    psVar20 = &local_128;
    FUN_140ceffd0(pIVar19,0,0,*puVar14,psVar20);
    if (local_d8 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_d8 + 0x10))();
    }
    if (local_f0 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_f0 + 0x10))();
    }
    if ((short)local_140 == 8) {
      local_140 = local_140 & 0xffff0000;
      if (CONCAT44(uStack_134,uStack_138) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_140);
    }
    if ((short)local_158 == 8) {
      local_158 = local_158 & 0xffff0000;
      if (CONCAT44(uStack_14c,uStack_150) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_158);
    }
    if ((short)local_178 == 8) {
      local_178 = local_178 & 0xffff0000;
      if (uStack_170 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_170 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_178);
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
    local_108 = (IUnknown *)0x0;
    if (local_b0 == 7) {
      uVar24 = 0;
      in_stack_fffffffffffffe50 = in_stack_fffffffffffffe50 & 0xffffffffffffff00;
      in_stack_fffffffffffffe48 = (uint *)((ulonglong)psVar20 & 0xffffffff00000000);
      puVar14 = (undefined8 *)
                FUN_1403a0810(DAT_143aa8328,&local_160,local_e0,0,in_stack_fffffffffffffe48,
                              in_stack_fffffffffffffe50,0,
                              in_stack_fffffffffffffe60 & 0xffffffffffffff00,CONCAT44(uVar25,0xff),
                              in_stack_fffffffffffffe70 & 0xffffffff00000000,0);
      pIVar16 = (IUnknown *)*puVar14;
      if (pIVar16 != (IUnknown *)0x0) {
        *puVar14 = 0;
        pIVar15 = pIVar16;
        local_108 = pIVar16;
      }
      if (local_160 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_160 + 0x10))();
      }
      if (local_f8 == 0) {
        *(undefined4 *)((longlong)param_1 + 0x104) = 1;
      }
LAB_1421068e4:
      if (pIVar15 != (IUnknown *)0x0) {
        local_158 = CONCAT22(local_158._2_2_,3);
        uStack_150 = 0xff;
        iVar5 = FUN_140404240(pIVar15);
        iVar5 = (0x20 - iVar5) / 2;
        if (iVar5 < 0) {
          iVar5 = 0;
        }
        iVar6 = FUN_140404280(pIVar15);
        iVar6 = (0x20 - iVar6) / 2;
        if (iVar6 < 0) {
          iVar6 = 0;
        }
        in_stack_fffffffffffffe48 = &local_158;
        FUN_140ceffd0(pIVar19,iVar6,iVar5,pIVar15,in_stack_fffffffffffffe48);
        if ((short)local_158 == 8) {
          local_158 = local_158 & 0xffff0000;
          if (CONCAT44(uStack_14c,uStack_150) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_158);
        }
      }
    }
    else {
      if ((0xc < param_3) || ((0x1290U >> (param_3 & 0x1f) & 1) == 0)) {
        local_110 = (IUnknown *)0x0;
        puVar14 = (undefined8 *)FUN_1408a9d20(&local_160,0x9e7);
        FUN_1401c21c0(&local_110,*puVar14,param_3);
        if (local_160 != (IUnknown *)0x0) {
          FUN_1401bebb0(local_160 + -0x10);
        }
        pIVar16 = DAT_143add058;
        if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(&local_178);
        iVar5 = FUN_14023c4c0(&local_178,&DAT_143a8b8d8);
        if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
        (*DAT_143262a20)(&local_158);
        iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
        pIVar11 = local_110;
        if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
        uVar7 = FUN_1401a5890(&local_160,local_110);
        in_stack_fffffffffffffe48 = &local_178;
        uVar7 = FUN_140403ec0(pIVar16,&local_140,uVar7,&local_158,in_stack_fffffffffffffe48);
        uVar7 = FUN_1409339d0(&local_f0,uVar7);
        FUN_1403ee040(&local_d8,uVar7);
        if (local_d8 != (IUnknown *)0x0) {
          local_108 = local_d8;
          pIVar15 = local_d8;
        }
        if (local_f0 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_f0 + 0x10))();
        }
        if ((short)local_140 == 8) {
          local_140 = local_140 & 0xffff0000;
          if (CONCAT44(uStack_134,uStack_138) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_140);
        }
        if ((short)local_158 == 8) {
          local_158 = local_158 & 0xffff0000;
          if (CONCAT44(uStack_14c,uStack_150) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_158);
        }
        if ((short)local_178 == 8) {
          local_178 = local_178 & 0xffff0000;
          if (uStack_170 != (longlong *)0x0) {
            (*DAT_143ad5990)((longlong)uStack_170 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_178);
        }
        if (pIVar11 != (IUnknown *)0x0) {
          FUN_1401bebb0(pIVar11 + -0x10);
        }
        goto LAB_1421068e4;
      }
      local_e0 = 0;
      uVar2 = FUN_142127400(param_3);
      in_stack_fffffffffffffe48 = &local_e0;
      puVar14 = (undefined8 *)
                FUN_142109b80(param_1,&local_160,uVar2,&local_e0,in_stack_fffffffffffffe48);
      pIVar16 = (IUnknown *)*puVar14;
      if (pIVar19 != pIVar16) {
        *puVar14 = 0;
        local_88 = pIVar16;
        (**(code **)(*(longlong *)pIVar19 + 0x10))();
        pIVar19 = pIVar16;
      }
      if (local_160 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_160 + 0x10))();
      }
    }
    *(undefined4 *)((longlong)param_1 + 0xfc) = 1;
    uVar17 = local_e8;
    uVar18 = local_f8;
    if (pIVar15 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar15 + 0x10))(pIVar15);
      uVar17 = local_e8;
      uVar18 = local_f8;
    }
    break;
  case 4:
    (*DAT_143262a20)(&local_158);
    iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
    if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    puVar14 = (undefined8 *)
              FUN_141028310(&local_160,0x20,0x20,&local_158,
                            (ulonglong)in_stack_fffffffffffffe48 & 0xffffffffffffff00);
    pIVar16 = (IUnknown *)*puVar14;
    if (pIVar16 != (IUnknown *)0x0) {
      *puVar14 = 0;
      pIVar19 = pIVar16;
      local_88 = pIVar16;
    }
    if (local_160 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_160 + 0x10))();
    }
    if ((short)local_158 == 8) {
      local_158 = local_158 & 0xffff0000;
      if (CONCAT44(uStack_14c,uStack_150) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_158);
    }
    local_f0 = (IUnknown *)0x0;
    local_110 = (IUnknown *)0x0;
    puVar14 = (undefined8 *)FUN_1408a9d20(&local_160,0xc52);
    FUN_1401c21c0(&local_110,*puVar14,param_3);
    if (local_160 != (IUnknown *)0x0) {
      FUN_1401bebb0(local_160 + -0x10);
    }
    pIVar16 = DAT_143add058;
    if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_178);
    iVar5 = FUN_14023c4c0(&local_178,&DAT_143a8b8d8);
    if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    (*DAT_143262a20)(&local_158);
    iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
    pIVar15 = local_110;
    if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    uVar7 = FUN_1401a5890(&local_160,local_110);
    in_stack_fffffffffffffe48 = &local_178;
    uVar7 = FUN_140403ec0(pIVar16,&local_140,uVar7,&local_158,in_stack_fffffffffffffe48);
    uVar7 = FUN_1409339d0(&local_b8,uVar7);
    FUN_1403ee040(&local_d8,uVar7);
    pIVar16 = (IUnknown *)0x0;
    if (local_d8 != (IUnknown *)0x0) {
      local_f0 = local_d8;
      pIVar16 = local_d8;
    }
    if (local_b8 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_b8 + 0x10))();
    }
    if ((short)local_140 == 8) {
      local_140 = local_140 & 0xffff0000;
      if (CONCAT44(uStack_134,uStack_138) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_140);
    }
    if ((short)local_158 == 8) {
      local_158 = local_158 & 0xffff0000;
      if (CONCAT44(uStack_14c,uStack_150) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_158);
    }
    if ((short)local_178 == 8) {
      local_178 = local_178 & 0xffff0000;
      if (uStack_170 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_170 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_178);
    }
    if (pIVar16 != (IUnknown *)0x0) {
      if (pIVar19 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_158 = CONCAT22(local_158._2_2_,3);
      uStack_150 = 0xff;
      iVar5 = FUN_140404240(pIVar16);
      iVar5 = (0x20 - iVar5) / 2;
      if (iVar5 < 0) {
        iVar5 = 0;
      }
      iVar6 = FUN_140404280(pIVar16);
      iVar6 = (0x20 - iVar6) / 2;
      if (iVar6 < 0) {
        iVar6 = 0;
      }
      in_stack_fffffffffffffe48 = &local_158;
      FUN_140ceffd0(pIVar19,iVar6,iVar5,pIVar16,in_stack_fffffffffffffe48);
      if ((short)local_158 == 8) {
        local_158 = local_158 & 0xffff0000;
        if (CONCAT44(uStack_14c,uStack_150) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_158);
      }
    }
    if (pIVar15 != (IUnknown *)0x0) {
      FUN_1401bebb0(pIVar15 + -0x10);
    }
    uVar17 = local_e8;
    uVar18 = local_f8;
    if (pIVar16 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar16 + 0x10))(pIVar16);
      uVar17 = local_e8;
      uVar18 = local_f8;
    }
    break;
  case 9:
    if ((param_3 & 0xffff) - 100 < 0xb9) {
      iVar5 = (int)param_3 >> 0x10;
    }
    else {
      iVar5 = 0;
    }
    FUN_1409181c0(&local_110,param_3 & 0xffff,iVar5,L"icon");
    pIVar16 = DAT_143add058;
    if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_178);
    iVar5 = FUN_14023c4c0(&local_178,&DAT_143a8b8d8);
    if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    (*DAT_143262a20)(&local_158);
    iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
    if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    uVar7 = FUN_1401a5890(&local_160,local_110);
    in_stack_fffffffffffffe48 = &local_178;
    uVar7 = FUN_140403ec0(pIVar16,&local_140,uVar7,&local_158,in_stack_fffffffffffffe48);
    uVar7 = FUN_1409339d0(&local_f0,uVar7);
    FUN_1403ee040(&local_d8,uVar7);
    if (local_d8 != (IUnknown *)0x0) {
      local_88 = local_d8;
      pIVar19 = local_d8;
    }
    if (local_f0 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_f0 + 0x10))();
    }
    if ((short)local_140 == 8) {
      local_140 = local_140 & 0xffff0000;
      if (CONCAT44(uStack_134,uStack_138) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_140);
    }
    if ((short)local_158 == 8) {
      local_158 = local_158 & 0xffff0000;
      if (CONCAT44(uStack_14c,uStack_150) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_158);
    }
    if ((short)local_178 == 8) {
      local_178 = local_178 & 0xffff0000;
      if (uStack_170 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_170 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_178);
    }
    *(undefined4 *)((longlong)param_1 + 0xfc) = 1;
    uVar17 = local_e8;
    uVar18 = local_f8;
    if (local_110 != (IUnknown *)0x0) {
      FUN_1401bebb0(local_110 + -0x10);
      uVar17 = local_e8;
      uVar18 = local_f8;
    }
  }
  lVar9 = DAT_143add050;
  local_d8 = (IUnknown *)0x0;
  if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_d0);
  iVar5 = FUN_14023c4c0(&local_d0,&DAT_143a8b8d8);
  if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar5);
  }
  (*DAT_143262a20)(&local_128);
  iVar5 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
  if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar5);
  }
  (*DAT_143262a20)(&local_178);
  iVar5 = FUN_14023c4c0(&local_178,&DAT_143a8b8d8);
  if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar5);
  }
  local_158 = CONCAT22(local_158._2_2_,3);
  uStack_150 = 0;
  uVar7 = CONCAT44(uVar24,0xc006156c);
  psVar20 = (short *)(in_stack_fffffffffffffe50 & 0xffffffff00000000);
  in_stack_fffffffffffffe48 = (uint *)((ulonglong)in_stack_fffffffffffffe48 & 0xffffffff00000000);
  plVar13 = (longlong *)
            FUN_140d8d300(lVar9,&local_160,0,0,in_stack_fffffffffffffe48,psVar20,uVar7,&local_158,
                          &local_178,&local_128,&local_d0);
  uVar2 = (undefined4)((ulonglong)uVar7 >> 0x20);
  plVar8 = (longlong *)param_1[0x17];
  if (plVar8 != (longlong *)*plVar13) {
    param_1[0x17] = (longlong *)*plVar13;
    *plVar13 = 0;
    if (plVar8 != (longlong *)0x0) {
      (**(code **)(*plVar8 + 0x10))();
    }
  }
  if (local_160 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_160 + 0x10))();
  }
  if ((short)local_158 == 8) {
    local_158 = local_158 & 0xffff0000;
    if (CONCAT44(uStack_14c,uStack_150) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_158);
  }
  if ((short)local_178 == 8) {
    local_178 = local_178 & 0xffff0000;
    if (uStack_170 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)uStack_170 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_178);
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
  if ((short)local_d0 == 8) {
    local_d0 = local_d0 & 0xffff0000;
    if (uStack_c8 != 0) {
      (*DAT_143ad5990)(uStack_c8 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_d0);
  }
  pIVar16 = (IUnknown *)param_1[0x17];
  if (pIVar16 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  uStack_170 = (longlong *)FUN_142c137a0(DAT_143abfdf8,&local_160,2);
  uStack_170 = (longlong *)*uStack_170;
  local_178 = CONCAT22(local_178._2_2_,0xd);
  if (uStack_170 != (longlong *)0x0) {
    (**(code **)(*uStack_170 + 8))();
  }
  local_a8 = local_178;
  uStack_a4 = uStack_174;
  uStack_a0 = (undefined4)uStack_170;
  uStack_9c = uStack_170._4_4_;
  local_98 = local_168;
  iVar5 = (**(code **)(*(longlong *)pIVar16 + 200))(pIVar16,&local_a8);
  if (iVar5 < 0) {
    _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_143273488);
  }
  if ((short)local_178 == 8) {
    local_178 = local_178 & 0xffff0000;
    if (uStack_170 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)uStack_170 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_178);
  }
  if (local_160 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_160 + 0x10))();
  }
  pIVar16 = (IUnknown *)param_1[0x17];
  if (pIVar16 != (IUnknown *)0x0) {
    iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x200))(pIVar16,0xd2ffffff);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_14327fcb0);
    }
    if (pIVar19 != (IUnknown *)0x0) {
      lVar9 = param_1[0x17];
      if (lVar9 == 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_140);
      iVar5 = FUN_14023c4c0(&local_140,&DAT_143a8b8d8);
      if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar5);
      }
      (*DAT_143262a20)(&local_d0);
      iVar5 = FUN_14023c4c0(&local_d0,&DAT_143a8b8d8);
      if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar5);
      }
      local_128 = 3;
      lStack_120 = CONCAT44(lStack_120._4_4_,0x40);
      local_178 = CONCAT22(local_178._2_2_,3);
      uStack_170 = (longlong *)CONCAT44(uStack_170._4_4_,0xd2);
      local_158 = CONCAT22(local_158._2_2_,3);
      uStack_150 = 500;
      puVar23 = &local_d0;
      psVar20 = &local_128;
      in_stack_fffffffffffffe48 = &local_178;
      FUN_140d5ec50(lVar9,&local_78,pIVar19,&local_158,in_stack_fffffffffffffe48,psVar20,puVar23,
                    &local_140);
      uVar2 = (undefined4)((ulonglong)puVar23 >> 0x20);
      if ((short)local_78 == 8) {
        local_78 = local_78 & 0xffffffffffff0000;
        if (lStack_70 != 0) {
          (*DAT_143ad5990)(lStack_70 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_78);
      }
      if ((short)local_158 == 8) {
        local_158 = local_158 & 0xffff0000;
        if (CONCAT44(uStack_14c,uStack_150) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_158);
      }
      if ((short)local_178 == 8) {
        local_178 = local_178 & 0xffff0000;
        if (uStack_170 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_170 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_178);
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
      if ((short)local_d0 == 8) {
        local_d0 = local_d0 & 0xffff0000;
        if (uStack_c8 != 0) {
          (*DAT_143ad5990)(uStack_c8 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_d0);
      }
      if ((short)local_140 == 8) {
        local_140 = local_140 & 0xffff0000;
        if (CONCAT44(uStack_134,uStack_138) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_140);
      }
    }
    pIVar16 = (IUnknown *)param_1[0x17];
    if (pIVar16 != (IUnknown *)0x0) {
      iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x350))(pIVar16,1);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_14327fcb0);
      }
      lVar9 = DAT_143add050;
      if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_128);
      iVar5 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
      if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar5);
      }
      (*DAT_143262a20)(&local_178);
      iVar5 = FUN_14023c4c0(&local_178,&DAT_143a8b8d8);
      if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar5);
      }
      (*DAT_143262a20)(&local_158);
      iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
      if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar5);
      }
      local_140 = CONCAT22(local_140._2_2_,3);
      uStack_138 = 0;
      uVar7 = CONCAT44(uVar2,0xc006156c);
      uVar22 = (ulonglong)psVar20 & 0xffffffff00000000;
      uVar21 = (ulonglong)in_stack_fffffffffffffe48 & 0xffffffff00000000;
      plVar13 = (longlong *)
                FUN_140d8d300(lVar9,&local_160,0,0,uVar21,uVar22,uVar7,&local_140,&local_158,
                              &local_178,&local_128);
      uVar2 = (undefined4)((ulonglong)uVar7 >> 0x20);
      plVar8 = (longlong *)param_1[0x18];
      if (plVar8 != (longlong *)*plVar13) {
        param_1[0x18] = (longlong *)*plVar13;
        *plVar13 = 0;
        if (plVar8 != (longlong *)0x0) {
          (**(code **)(*plVar8 + 0x10))();
        }
      }
      if (local_160 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_160 + 0x10))();
      }
      if ((short)local_140 == 8) {
        local_140 = local_140 & 0xffff0000;
        if (CONCAT44(uStack_134,uStack_138) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_140);
      }
      if ((short)local_158 == 8) {
        local_158 = local_158 & 0xffff0000;
        if (CONCAT44(uStack_14c,uStack_150) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_158);
      }
      if ((short)local_178 == 8) {
        local_178 = local_178 & 0xffff0000;
        if (uStack_170 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_170 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_178);
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
      pIVar16 = (IUnknown *)param_1[0x18];
      if (pIVar16 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      puVar14 = (undefined8 *)FUN_142c137a0(DAT_143abfdf8,&local_160,2);
      uStack_170 = (longlong *)*puVar14;
      local_178 = CONCAT22(local_178._2_2_,0xd);
      if (uStack_170 != (longlong *)0x0) {
        (**(code **)(*uStack_170 + 8))();
      }
      local_a8 = local_178;
      uStack_a4 = uStack_174;
      uStack_a0 = (undefined4)uStack_170;
      uStack_9c = uStack_170._4_4_;
      local_98 = local_168;
      iVar5 = (**(code **)(*(longlong *)pIVar16 + 200))(pIVar16,&local_a8);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_143273488);
      }
      if ((short)local_178 == 8) {
        local_178 = local_178 & 0xffff0000;
        if (uStack_170 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_170 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_178);
      }
      if (local_160 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_160 + 0x10))();
      }
      pIVar16 = (IUnknown *)param_1[0x18];
      if (pIVar16 != (IUnknown *)0x0) {
        iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x200))(pIVar16,0xd2ffffff);
        if (iVar5 < 0) {
          _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_14327fcb0);
        }
        pIVar16 = (IUnknown *)param_1[0x18];
        if (pIVar16 != (IUnknown *)0x0) {
          iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x350))(pIVar16,1);
          if (iVar5 < 0) {
            _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_14327fcb0);
          }
          lVar9 = DAT_143add050;
          if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          (*DAT_143262a20)(&local_128);
          iVar5 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
          if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar5);
          }
          (*DAT_143262a20)(&local_178);
          iVar5 = FUN_14023c4c0(&local_178,&DAT_143a8b8d8);
          if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar5);
          }
          (*DAT_143262a20)(&local_158);
          iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
          if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar5);
          }
          local_140 = CONCAT22(local_140._2_2_,3);
          uStack_138 = 0;
          uVar7 = CONCAT44(uVar2,0xc006156c);
          uVar22 = uVar22 & 0xffffffff00000000;
          uVar21 = uVar21 & 0xffffffff00000000;
          plVar13 = (longlong *)
                    FUN_140d8d300(lVar9,&local_160,0,0,uVar21,uVar22,uVar7,&local_140,&local_158,
                                  &local_178,&local_128);
          uVar2 = (undefined4)((ulonglong)uVar7 >> 0x20);
          plVar8 = (longlong *)param_1[0x19];
          if (plVar8 != (longlong *)*plVar13) {
            param_1[0x19] = (longlong *)*plVar13;
            *plVar13 = 0;
            if (plVar8 != (longlong *)0x0) {
              (**(code **)(*plVar8 + 0x10))();
            }
          }
          if (local_160 != (IUnknown *)0x0) {
            (**(code **)(*(longlong *)local_160 + 0x10))();
          }
          if ((short)local_140 == 8) {
            local_140 = local_140 & 0xffff0000;
            if (CONCAT44(uStack_134,uStack_138) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_140);
          }
          if ((short)local_158 == 8) {
            local_158 = local_158 & 0xffff0000;
            if (CONCAT44(uStack_14c,uStack_150) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_158);
          }
          if ((short)local_178 == 8) {
            local_178 = local_178 & 0xffff0000;
            if (uStack_170 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)uStack_170 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_178);
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
          pIVar16 = (IUnknown *)param_1[0x19];
          if (pIVar16 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          puVar14 = (undefined8 *)FUN_142c137a0(DAT_143abfdf8,&local_160,2);
          uStack_170 = (longlong *)*puVar14;
          local_178 = CONCAT22(local_178._2_2_,0xd);
          if (uStack_170 != (longlong *)0x0) {
            (**(code **)(*uStack_170 + 8))();
          }
          local_a8 = local_178;
          uStack_a4 = uStack_174;
          uStack_a0 = (undefined4)uStack_170;
          uStack_9c = uStack_170._4_4_;
          local_98 = local_168;
          iVar5 = (**(code **)(*(longlong *)pIVar16 + 200))(pIVar16,&local_a8);
          if (iVar5 < 0) {
            _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_143273488);
          }
          if ((short)local_178 == 8) {
            local_178 = local_178 & 0xffff0000;
            if (uStack_170 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)uStack_170 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_178);
          }
          if (local_160 != (IUnknown *)0x0) {
            (**(code **)(*(longlong *)local_160 + 0x10))();
          }
          pIVar16 = (IUnknown *)param_1[0x19];
          if (pIVar16 != (IUnknown *)0x0) {
            iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x200))(pIVar16,0xd2ffffff);
            if (iVar5 < 0) {
              _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_14327fcb0);
            }
            if (param_1[0x19] != 0) {
              FUN_1418b39b0(param_1[0x19],1);
              lVar9 = DAT_143add050;
              if (0x7ffffffd < uVar18 - 1) {
LAB_142107a20:
                FUN_14090f440(&local_80,L"UI/SkillIcon.img/Disable/0");
                if (local_80 != (longlong *)0x0) {
                  pIVar16 = (IUnknown *)param_1[0x19];
                  if (pIVar16 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x80004003);
                  }
                  local_178 = CONCAT22(local_178._2_2_,3);
                  uStack_170 = (longlong *)CONCAT44(uStack_170._4_4_,0xfffffffe);
                  local_110 = (IUnknown *)0x0;
                  local_a8 = local_178;
                  uStack_a4 = uStack_174;
                  uStack_a0 = 0xfffffffe;
                  uStack_9c = uStack_170._4_4_;
                  local_98 = local_168;
                  iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x268))(pIVar16,&local_a8,&local_110);
                  if (iVar5 < 0) {
                    _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_14327fcb0);
                  }
                  if (local_110 != (IUnknown *)0x0) {
                    (**(code **)(*(longlong *)local_110 + 0x10))();
                  }
                  if ((short)local_178 == 8) {
                    local_178 = local_178 & 0xffff0000;
                    if (uStack_170 != (longlong *)0x0) {
                      (*DAT_143ad5990)((longlong)uStack_170 + -4);
                    }
                  }
                  else {
                    (*DAT_143262a18)(&local_178);
                  }
                  lVar9 = param_1[0x19];
                  if (lVar9 == 0) {
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x80004003);
                  }
                  (*DAT_143262a20)(&local_d0);
                  iVar5 = FUN_14023c4c0(&local_d0,&DAT_143a8b8d8);
                  if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(iVar5);
                  }
                  (*DAT_143262a20)(&local_128);
                  iVar5 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
                  if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(iVar5);
                  }
                  local_178 = CONCAT22(local_178._2_2_,3);
                  uStack_170 = (longlong *)CONCAT44(uStack_170._4_4_,0x40);
                  local_158 = CONCAT22(local_158._2_2_,3);
                  uStack_150 = 0xd2;
                  local_140 = CONCAT22(local_140._2_2_,3);
                  uStack_138 = 500;
                  FUN_140d5ec50(lVar9,&local_78,local_80,&local_140,&local_158,&local_178,&local_128
                                ,&local_d0);
                  if ((short)local_78 == 8) {
                    local_78 = local_78 & 0xffffffffffff0000;
                    if (lStack_70 != 0) {
                      (*DAT_143ad5990)(lStack_70 + -4);
                    }
                  }
                  else {
                    (*DAT_143262a18)(&local_78);
                  }
                  if ((short)local_140 == 8) {
                    local_140 = local_140 & 0xffff0000;
                    if (CONCAT44(uStack_134,uStack_138) != 0) {
                      (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
                    }
                  }
                  else {
                    (*DAT_143262a18)(&local_140);
                  }
                  if ((short)local_158 == 8) {
                    local_158 = local_158 & 0xffff0000;
                    if (CONCAT44(uStack_14c,uStack_150) != 0) {
                      (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
                    }
                  }
                  else {
                    (*DAT_143262a18)(&local_158);
                  }
                  if ((short)local_178 == 8) {
                    local_178 = local_178 & 0xffff0000;
                    if (uStack_170 != (longlong *)0x0) {
                      (*DAT_143ad5990)((longlong)uStack_170 + -4);
                    }
                  }
                  else {
                    (*DAT_143262a18)(&local_178);
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
                  if ((short)local_d0 == 8) {
                    local_d0 = local_d0 & 0xffff0000;
                    if (uStack_c8 != 0) {
                      (*DAT_143ad5990)(uStack_c8 + -4);
                    }
                  }
                  else {
                    (*DAT_143262a18)(&local_d0);
                  }
                  pIVar16 = (IUnknown *)param_1[0x19];
                  if (pIVar16 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x80004003);
                  }
                  iVar5 = (**(code **)(*(longlong *)pIVar16 + 0x2b8))(pIVar16,0);
                  if (iVar5 < 0) {
                    _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_14327fcb0);
                  }
                }
                *(bool *)(param_1 + 0x23) = param_10 != 0;
                if (param_10 == 0) {
                  if (0 < (int)uVar17) {
                    *(int *)(param_1 + 0x22) = (int)(uVar17 + ((int)uVar17 >> 0x1f & 0xfU)) >> 4;
                  }
                }
                else {
                  *(uint *)((longlong)param_1 + 0x11c) = uVar17;
                }
                *(int *)(param_1 + 0x22) = (int)(uVar17 + ((int)uVar17 >> 0x1f & 0xfU)) >> 4;
                *local_58 = uVar18;
                if (local_80 != (longlong *)0x0) {
                  (**(code **)(*local_80 + 0x10))();
                }
                if (pIVar19 != (IUnknown *)0x0) {
                  (**(code **)(*(longlong *)pIVar19 + 0x10))(pIVar19);
                }
                if (*local_50 != 0) {
                  FUN_14019f2c0(*local_50 + -0x10);
                }
                return param_1;
              }
              if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              (*DAT_143262a20)(&local_128);
              iVar5 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              (*DAT_143262a20)(&local_178);
              iVar5 = FUN_14023c4c0(&local_178,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              (*DAT_143262a20)(&local_158);
              iVar5 = FUN_14023c4c0(&local_158,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              local_140 = CONCAT22(local_140._2_2_,3);
              uStack_138 = 0;
              uVar7 = FUN_140d8d300(lVar9,&local_160,0,0,uVar21 & 0xffffffff00000000,
                                    uVar22 & 0xffffffff00000000,CONCAT44(uVar2,0xc006156c),
                                    &local_140,&local_158,&local_178,&local_128);
              FUN_140cb04e0(param_1 + 0x1a,uVar7);
              if (local_160 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_160 + 0x10))();
              }
              if ((short)local_140 == 8) {
                local_140 = local_140 & 0xffff0000;
                if (CONCAT44(uStack_134,uStack_138) != 0) {
                  (*DAT_143ad5990)(CONCAT44(uStack_134,uStack_138) + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_140);
              }
              if ((short)local_158 == 8) {
                local_158 = local_158 & 0xffff0000;
                if (CONCAT44(uStack_14c,uStack_150) != 0) {
                  (*DAT_143ad5990)(CONCAT44(uStack_14c,uStack_150) + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_158);
              }
              if ((short)local_178 == 8) {
                local_178 = local_178 & 0xffff0000;
                if (uStack_170 != (longlong *)0x0) {
                  (*DAT_143ad5990)((longlong)uStack_170 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_178);
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
              pIVar16 = (IUnknown *)param_1[0x1a];
              if (pIVar16 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              puVar14 = (undefined8 *)FUN_142c137a0(DAT_143abfdf8,&local_160,2);
              uStack_170 = (longlong *)*puVar14;
              local_178 = CONCAT22(local_178._2_2_,0xd);
              if (uStack_170 != (longlong *)0x0) {
                (**(code **)(*uStack_170 + 8))();
              }
              local_a8 = local_178;
              uStack_a4 = uStack_174;
              uStack_a0 = (undefined4)uStack_170;
              uStack_9c = uStack_170._4_4_;
              local_98 = local_168;
              iVar5 = (**(code **)(*(longlong *)pIVar16 + 200))(pIVar16,&local_a8);
              if (iVar5 < 0) {
                _com_issue_errorex(iVar5,pIVar16,(_GUID *)&DAT_143273488);
              }
              if ((short)local_178 == 8) {
                local_178 = local_178 & 0xffff0000;
                if (uStack_170 != (longlong *)0x0) {
                  (*DAT_143ad5990)((longlong)uStack_170 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_178);
              }
              if (local_160 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_160 + 0x10))();
              }
              if (param_1[0x1a] != 0) {
                FUN_140eeba60(param_1[0x1a],0xd2ffffff);
                if (param_1[0x1a] != 0) {
                  FUN_1418b39b0(param_1[0x1a],1);
                  goto LAB_142107a20;
                }
              }
            }
          }
        }
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_1403a0810 @ 1403a0810   (3665 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x0001403a0f97) */
/* WARNING: Removing unreachable block (ram,0x0001403a0cb2) */
/* WARNING: Removing unreachable block (ram,0x0001403a0e57) */
/* WARNING: Removing unreachable block (ram,0x0001403a1299) */
/* WARNING: Removing unreachable block (ram,0x0001403a1162) */
/* WARNING: Removing unreachable block (ram,0x0001403a142e) */

undefined8 *
FUN_1403a0810(undefined8 param_1,undefined8 *param_2,ulonglong param_3,int param_4,int param_5,
             byte param_6,undefined8 param_7,byte param_8,undefined4 param_9,int param_10,
             IUnknown **param_11)

{
  longlong *plVar1;
  ulonglong uVar2;
  int iVar3;
  int iVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  IUnknown *pIVar7;
  int *piVar8;
  longlong *plVar9;
  ulonglong *puVar10;
  IUnknown **ppIVar11;
  IUnknown *pIVar12;
  undefined *puVar13;
  longlong lVar14;
  IUnknown *pIVar15;
  uint uVar16;
  IUnknown *pIVar17;
  longlong lVar18;
  undefined1 auStack_1e8 [32];
  char local_1c8;
  IUnknown *local_1c0;
  IUnknown *local_1b8;
  IUnknown *local_1b0;
  longlong *local_1a8;
  uint local_1a0;
  uint local_198 [2];
  IUnknown *local_190;
  IUnknown *local_188;
  IUnknown *local_180;
  int local_178;
  undefined4 uStack_174;
  IUnknown *local_170;
  longlong *local_168;
  uint local_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined8 local_150;
  ulonglong local_148;
  IUnknown *local_140;
  IUnknown *local_138;
  ulonglong local_130;
  IUnknown **local_128;
  uint local_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined8 local_110;
  uint local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 local_f8;
  undefined8 *local_f0;
  int *local_e8;
  longlong *local_e0;
  IUnknown *local_d8;
  IUnknown *local_d0;
  uint local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 local_b8;
  IUnknown *local_b0;
  uint local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  IUnknown *local_90;
  uint local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_78;
  undefined1 local_70 [16];
  ushort local_60;
  undefined1 local_5e;
  int local_58;
  ulonglong local_50;
  
  local_50 = DAT_143a8b908 ^ (ulonglong)auStack_1e8;
  local_198[0] = (uint)param_3;
  local_170 = (IUnknown *)CONCAT44(local_170._4_4_,param_9);
  local_128 = param_11;
  pIVar15 = (IUnknown *)0x0;
  local_1a0 = 0;
  local_190 = (IUnknown *)0x0;
  local_140 = (IUnknown *)0x0;
  local_1c8 = '\0';
  lVar14 = -1;
  pIVar17 = pIVar15;
  local_178 = param_4;
  local_f0 = param_2;
  if (param_11 == (IUnknown **)0x0) {
LAB_1403a09a3:
    puVar5 = (undefined8 *)FUN_14039f600(param_1,&local_180,param_3,0);
    pIVar12 = (IUnknown *)*puVar5;
    if (pIVar12 != (IUnknown *)0x0) {
      *puVar5 = 0;
      pIVar17 = pIVar12;
      local_140 = pIVar12;
    }
    if (local_180 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_180 + 0x10))();
    }
    puVar5 = local_f0;
    if (pIVar17 == (IUnknown *)0x0) {
      *local_f0 = 0;
      if (pIVar15 == (IUnknown *)0x0) {
        return local_f0;
      }
      (**(code **)(*(longlong *)pIVar15 + 0x10))(pIVar15);
      return puVar5;
    }
  }
  else {
    local_60 = ((ushort)param_6 << 2 | (ushort)param_4 & 1 | local_60 & 0xfff8) & 0xfff7 |
               ((ushort)param_8 << 2 | (ushort)param_5 & 1) * 2;
    local_5e = (undefined1)param_9;
    local_58 = param_10;
    FUN_140382ae0(param_11,&local_1b0,param_3,&local_60);
    pIVar12 = local_1b0;
    if (local_1b0 != (IUnknown *)0x0) {
      pIVar7 = *(IUnknown **)local_1b0;
      if (pIVar7 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar7 + 8))(pIVar7);
        pIVar15 = pIVar7;
        local_190 = pIVar7;
      }
      pIVar12 = *(IUnknown **)(pIVar12 + 8);
      if (pIVar12 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar12 + 8))(pIVar12);
        pIVar17 = pIVar12;
        local_140 = pIVar12;
      }
      local_1c8 = '\x01';
    }
    plVar9 = local_1a8;
    if (local_1a8 != (longlong *)0x0) {
      LOCK();
      plVar1 = local_1a8 + 1;
      lVar18 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -1;
      UNLOCK();
      pIVar15 = local_190;
      pIVar17 = local_140;
      if ((int)lVar18 == 1) {
        (**(code **)*local_1a8)(local_1a8);
        LOCK();
        piVar8 = (int *)((longlong)plVar9 + 0xc);
        iVar4 = *piVar8;
        *piVar8 = *piVar8 + -1;
        UNLOCK();
        pIVar15 = local_190;
        pIVar17 = local_140;
        if (iVar4 == 1) {
          (**(code **)(*local_1a8 + 8))();
          pIVar15 = local_190;
          pIVar17 = local_140;
        }
      }
    }
    if (pIVar17 == (IUnknown *)0x0) {
      param_3 = (ulonglong)local_198[0];
      goto LAB_1403a09a3;
    }
  }
  if (pIVar15 != (IUnknown *)0x0) goto LAB_1403a1471;
  local_180 = pIVar17;
  (**(code **)(*(longlong *)pIVar17 + 8))(pIVar17);
  iVar3 = FUN_140910ca0(&local_180,"useIconSubIdx",0);
  iVar4 = local_178;
  if ((iVar3 == 0) || (param_10 < 1)) {
    if (param_5 == 0) {
      puVar13 = PTR_u_iconRaw_143a45dd0;
      if (local_178 != 0) {
        puVar13 = PTR_u_icon_143a45db8;
      }
      local_1b0 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
      if (local_1b0 == (IUnknown *)0x0) {
        local_1c0 = (IUnknown *)0x0;
      }
      else {
        local_1c0 = (IUnknown *)FUN_14023b360(local_1b0,puVar13);
      }
      pIVar12 = (IUnknown *)0x0;
      if (local_1c0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      local_1b0 = (IUnknown *)&local_1c0;
      (*DAT_143262a20)(&local_120);
      pIVar7 = pIVar12;
      if (local_1c0 != (IUnknown *)0x0) {
        pIVar7 = *(IUnknown **)local_1c0;
      }
      iVar4 = (**(code **)(*(longlong *)pIVar17 + 0x28))(pIVar17,pIVar7,&local_120);
      if (iVar4 < 0) {
        _com_issue_errorex(iVar4,pIVar17,(_GUID *)&DAT_143272478);
      }
      local_160 = local_120;
      uStack_15c = uStack_11c;
      uStack_158 = uStack_118;
      uStack_154 = uStack_114;
      local_150 = local_110;
      local_120 = local_120 & 0xffff0000;
      FUN_1401be120(&local_1c0);
      plVar9 = (longlong *)FUN_1409339d0(&local_168,&local_160);
      plVar9 = (longlong *)*plVar9;
      if (plVar9 == (longlong *)0x0) {
        iVar4 = -0x7fffbffe;
      }
      else {
        (**(code **)(*plVar9 + 8))(plVar9);
        local_1b8 = (IUnknown *)0x0;
        iVar4 = (**(code **)*plVar9)(plVar9,&DAT_14327ac98,&local_1b8);
        if (-1 < iVar4) {
          pIVar12 = local_1b8;
        }
      }
      if (plVar9 != (longlong *)0x0) {
        (**(code **)(*plVar9 + 0x10))(plVar9);
      }
      if (((iVar4 + 0x80000000U & 0x80000000) == 0) && (iVar4 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar4);
      }
      if (pIVar12 == (IUnknown *)0x0) {
        pIVar12 = pIVar15;
      }
      pIVar15 = pIVar12;
      local_190 = pIVar15;
      if (local_168 != (longlong *)0x0) {
        (**(code **)(*local_168 + 0x10))();
      }
      if ((short)local_160 == 8) {
        local_160 = local_160 & 0xffff0000;
        if (CONCAT44(uStack_154,uStack_158) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_154,uStack_158) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_160);
      }
    }
    else {
      puVar13 = PTR_u_iconRawD_143a45dd8;
      if (local_178 != 0) {
        puVar13 = PTR_u_iconD_143a45dc0;
      }
      local_1b0 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
      if (local_1b0 == (IUnknown *)0x0) {
        local_1c0 = (IUnknown *)0x0;
      }
      else {
        local_1c0 = (IUnknown *)FUN_14023b360(local_1b0,puVar13);
      }
      pIVar12 = (IUnknown *)0x0;
      if (local_1c0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      local_1b0 = (IUnknown *)&local_1c0;
      (*DAT_143262a20)(&local_120);
      pIVar7 = pIVar12;
      if (local_1c0 != (IUnknown *)0x0) {
        pIVar7 = *(IUnknown **)local_1c0;
      }
      iVar3 = (**(code **)(*(longlong *)pIVar17 + 0x28))(pIVar17,pIVar7,&local_120);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar17,(_GUID *)&DAT_143272478);
      }
      local_160 = local_120;
      uStack_15c = uStack_11c;
      uStack_158 = uStack_118;
      uStack_154 = uStack_114;
      local_150 = local_110;
      local_120 = local_120 & 0xffff0000;
      FUN_1401be120(&local_1c0);
      plVar9 = (longlong *)FUN_1409339d0(&local_168,&local_160);
      plVar9 = (longlong *)*plVar9;
      pIVar7 = pIVar12;
      if (plVar9 == (longlong *)0x0) {
        iVar3 = -0x7fffbffe;
      }
      else {
        (**(code **)(*plVar9 + 8))(plVar9);
        local_1b8 = (IUnknown *)0x0;
        iVar3 = (**(code **)*plVar9)(plVar9,&DAT_14327ac98,&local_1b8);
        if (-1 < iVar3) {
          pIVar7 = local_1b8;
        }
      }
      if (plVar9 != (longlong *)0x0) {
        (**(code **)(*plVar9 + 0x10))(plVar9);
      }
      if (((iVar3 + 0x80000000U & 0x80000000) == 0) && (iVar3 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
      if (pIVar7 == (IUnknown *)0x0) {
        pIVar7 = pIVar15;
      }
      pIVar15 = pIVar7;
      local_190 = pIVar15;
      if (local_168 != (longlong *)0x0) {
        (**(code **)(*local_168 + 0x10))();
      }
      if ((short)local_160 == 8) {
        local_160 = local_160 & 0xffff0000;
        if (CONCAT44(uStack_154,uStack_158) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_154,uStack_158) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_160);
      }
      if (pIVar15 == (IUnknown *)0x0) {
        puVar13 = PTR_u_iconRaw_143a45dd0;
        if (iVar4 != 0) {
          puVar13 = PTR_u_icon_143a45db8;
        }
        local_1b0 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
        local_188 = pIVar12;
        if (local_1b0 != (IUnknown *)0x0) {
          local_188 = (IUnknown *)FUN_14023b360(local_1b0,puVar13);
        }
        if (local_188 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x8007000e);
        }
        local_1b0 = (IUnknown *)&local_188;
        (*DAT_143262a20)(&local_c8);
        if (local_188 != (IUnknown *)0x0) {
          pIVar12 = *(IUnknown **)local_188;
        }
        iVar4 = (**(code **)(*(longlong *)pIVar17 + 0x28))(pIVar17,pIVar12,&local_c8);
        if (iVar4 < 0) {
          _com_issue_errorex(iVar4,pIVar17,(_GUID *)&DAT_143272478);
        }
        local_108 = local_c8;
        uStack_104 = uStack_c4;
        uStack_100 = uStack_c0;
        uStack_fc = uStack_bc;
        local_f8 = local_b8;
        local_c8 = local_c8 & 0xffff0000;
        FUN_1401be120(&local_188);
        uVar6 = FUN_1409339d0(&local_d8,&local_108);
        FUN_1403ee040(&local_d0,uVar6);
        if (local_d0 != (IUnknown *)0x0) {
          local_190 = local_d0;
          pIVar15 = local_d0;
        }
        if (local_d8 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_d8 + 0x10))();
        }
        if ((short)local_108 == 8) {
          local_108 = local_108 & 0xffff0000;
          if (CONCAT44(uStack_fc,uStack_100) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_fc,uStack_100) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_108);
        }
      }
    }
    goto LAB_1403a1471;
  }
  if (local_178 == 0) {
    local_130 = 0;
    uVar16 = 4;
    puVar10 = &local_130;
    puVar13 = PTR_u_iconRaw_d_143a45de8;
  }
  else {
    local_148 = 0;
    uVar16 = 2;
    puVar10 = &local_148;
    puVar13 = PTR_u_icon_d_143a45df0;
  }
  local_180 = (IUnknown *)0x0;
  local_1a0 = uVar16;
  uVar6 = FUN_1401c21c0(puVar10,puVar13,param_10);
  FUN_1401c1fb0(&local_180,uVar6);
  if (((uVar16 & 4) != 0) && (uVar16 = uVar16 & 0xfffffffb, local_1a0 = uVar16, local_130 != 0)) {
    FUN_1401bebb0(local_130 - 0x10);
  }
  if (((uVar16 & 2) != 0) && (uVar16 = uVar16 & 0xfffffffd, local_148 != 0)) {
    FUN_1401bebb0(local_148 - 0x10);
  }
  if (iVar4 == 0) {
    local_b0 = (IUnknown *)0x0;
    uVar16 = uVar16 | 0x10;
    ppIVar11 = &local_b0;
    puVar13 = PTR_u_iconRawD_d_143a45de0;
  }
  else {
    local_1b8 = (IUnknown *)0x0;
    uVar16 = uVar16 | 8;
    ppIVar11 = &local_1b8;
    puVar13 = PTR_u_iconD_d_143a45dc8;
  }
  local_148 = 0;
  local_1a0 = uVar16;
  uVar6 = FUN_1401c21c0(ppIVar11,puVar13,param_10);
  FUN_1401c1fb0(&local_148,uVar6);
  if (((uVar16 & 0x10) != 0) &&
     (uVar16 = uVar16 & 0xffffffef, local_1a0 = uVar16, local_b0 != (IUnknown *)0x0)) {
    FUN_1401bebb0(local_b0 + -0x10);
  }
  if (((uVar16 & 8) != 0) && (local_1b8 != (IUnknown *)0x0)) {
    FUN_1401bebb0(local_1b8 + -0x10);
  }
  uVar2 = local_148;
  pIVar12 = local_180;
  if (param_5 == 0) {
LAB_1403a0d03:
    pIVar7 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
    local_188 = (IUnknown *)0x0;
    local_1b0 = pIVar7;
    if (pIVar7 != (IUnknown *)0x0) {
      *(longlong *)(pIVar7 + 8) = 0;
      *(undefined4 *)(pIVar7 + 0x10) = 1;
      local_188 = pIVar7;
      if (pIVar12 == (IUnknown *)0x0) {
        *(longlong *)pIVar7 = 0;
      }
      else {
        do {
          lVar14 = lVar14 + 1;
        } while (*(short *)(pIVar12 + lVar14 * 2) != 0);
        local_e0 = (longlong *)(ulonglong)((int)lVar14 + 1);
        piVar8 = (int *)(*DAT_143ad5980)((longlong)local_e0 * 2 + 4);
        if (piVar8 == (int *)0x0) {
          *(longlong *)pIVar7 = 0;
LAB_1403a161b:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x8007000e);
        }
        *piVar8 = (int)lVar14 * 2;
        piVar8 = piVar8 + 1;
        FUN_142ef7ba0(piVar8,pIVar12,(longlong)local_e0 * 2);
        *(int **)pIVar7 = piVar8;
        if (piVar8 == (int *)0x0) goto LAB_1403a161b;
      }
    }
    lVar14 = 0;
    if (local_188 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    local_1b0 = (IUnknown *)&local_188;
    (*DAT_143262a20)(&local_c8);
    if (local_188 != (IUnknown *)0x0) {
      lVar14 = *(longlong *)local_188;
    }
    iVar4 = (**(code **)(*(longlong *)pIVar17 + 0x28))(pIVar17,lVar14,&local_c8);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar17,(_GUID *)&DAT_143272478);
    }
    local_108 = local_c8;
    uStack_104 = uStack_c4;
    uStack_100 = uStack_c0;
    uStack_fc = uStack_bc;
    local_f8 = local_b8;
    local_c8 = local_c8 & 0xffff0000;
    FUN_1401be120(&local_188);
    uVar6 = FUN_1409339d0(&local_d0,&local_108);
    FUN_1403ee040(&local_d8,uVar6);
    if (local_d8 != (IUnknown *)0x0) {
      local_190 = local_d8;
      pIVar15 = local_d8;
    }
    if (local_d0 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_d0 + 0x10))();
    }
    if ((short)local_108 == 8) {
      local_108 = local_108 & 0xffff0000;
      if (CONCAT44(uStack_fc,uStack_100) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_fc,uStack_100) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_108);
    }
    if (pIVar15 == (IUnknown *)0x0) {
      puVar13 = PTR_u_iconRawD_143a45dd8;
      if (local_178 != 0) {
        puVar13 = PTR_u_iconD_143a45dc0;
      }
      local_1b0 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
      if (local_1b0 == (IUnknown *)0x0) {
        local_1c0 = (IUnknown *)0x0;
      }
      else {
        local_1c0 = (IUnknown *)FUN_14023b360(local_1b0,puVar13);
      }
      lVar14 = 0;
      if (local_1c0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      local_1b0 = (IUnknown *)&local_1c0;
      (*DAT_143262a20)(&local_120);
      if (local_1c0 != (IUnknown *)0x0) {
        lVar14 = *(longlong *)local_1c0;
      }
      iVar4 = (**(code **)(*(longlong *)pIVar17 + 0x28))(pIVar17,lVar14,&local_120);
      if (iVar4 < 0) {
        _com_issue_errorex(iVar4,pIVar17,(_GUID *)&DAT_143272478);
      }
      local_160 = local_120;
      uStack_15c = uStack_11c;
      uStack_158 = uStack_118;
      uStack_154 = uStack_114;
      local_150 = local_110;
      local_120 = local_120 & 0xffff0000;
      FUN_1401be120(&local_1c0);
      uVar6 = FUN_1409339d0(&local_168,&local_160);
      FUN_1403ee040(&local_178,uVar6);
      pIVar7 = (IUnknown *)CONCAT44(uStack_174,local_178);
      if (pIVar7 != (IUnknown *)0x0) {
        pIVar15 = pIVar7;
        local_190 = pIVar7;
      }
      if (local_168 != (longlong *)0x0) {
        (**(code **)(*local_168 + 0x10))();
      }
      if ((short)local_160 == 8) {
        local_160 = local_160 & 0xffff0000;
        if (CONCAT44(uStack_154,uStack_158) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_154,uStack_158) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_160);
      }
    }
  }
  else {
    pIVar7 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
    local_1b0 = pIVar7;
    if (pIVar7 == (IUnknown *)0x0) {
      local_138 = (IUnknown *)0x0;
    }
    else {
      *(undefined8 *)(pIVar7 + 8) = 0;
      *(undefined4 *)(pIVar7 + 0x10) = 1;
      lVar18 = lVar14;
      local_138 = pIVar7;
      if (uVar2 == 0) {
        *(undefined8 *)pIVar7 = 0;
      }
      else {
        do {
          lVar18 = lVar18 + 1;
        } while (*(short *)(uVar2 + lVar18 * 2) != 0);
        local_130 = (ulonglong)((int)lVar18 + 1);
        piVar8 = (int *)(*DAT_143ad5980)(local_130 * 2 + 4);
        if (piVar8 == (int *)0x0) {
          *(undefined8 *)pIVar7 = 0;
LAB_1403a160d:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x8007000e);
        }
        *piVar8 = (int)lVar18 * 2;
        local_e8 = piVar8 + 1;
        FUN_142ef7ba0(local_e8,uVar2,local_130 * 2);
        *(int **)pIVar7 = local_e8;
        if (local_e8 == (int *)0x0) goto LAB_1403a160d;
      }
    }
    if (local_138 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    local_1b0 = (IUnknown *)&local_138;
    (*DAT_143262a20)(&local_88);
    uVar6 = 0;
    if (local_138 != (IUnknown *)0x0) {
      uVar6 = *(undefined8 *)local_138;
    }
    iVar4 = (**(code **)(*(longlong *)pIVar17 + 0x28))(pIVar17,uVar6,&local_88);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar17,(_GUID *)&DAT_143272478);
    }
    local_a8 = local_88;
    uStack_a4 = uStack_84;
    uStack_a0 = uStack_80;
    uStack_9c = uStack_7c;
    local_98 = local_78;
    local_88 = local_88 & 0xffff0000;
    FUN_1401be120(&local_138);
    uVar6 = FUN_1409339d0(&local_e0,&local_a8);
    FUN_1403ee040(&local_90,uVar6);
    if (local_90 != (IUnknown *)0x0) {
      local_190 = local_90;
      pIVar15 = local_90;
    }
    if (local_e0 != (longlong *)0x0) {
      (**(code **)(*local_e0 + 0x10))();
    }
    if ((short)local_a8 == 8) {
      local_a8 = local_a8 & 0xffff0000;
      if (CONCAT44(uStack_9c,uStack_a0) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_9c,uStack_a0) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_a8);
    }
    if (pIVar15 == (IUnknown *)0x0) goto LAB_1403a0d03;
  }
  if (uVar2 != 0) {
    FUN_1401bebb0(uVar2 - 0x10);
  }
  if (pIVar12 != (IUnknown *)0x0) {
    FUN_1401bebb0(pIVar12 + -0x10);
  }
LAB_1403a1471:
  ppIVar11 = local_128;
  if (param_8 == 0) {
    if (((local_1c8 == '\0') && (local_128 != (IUnknown **)0x0)) && (pIVar15 != (IUnknown *)0x0)) {
      local_1c0 = pIVar17;
      (**(code **)(*(longlong *)pIVar17 + 8))(pIVar17);
      local_1b0 = (IUnknown *)&local_1c0;
      local_170 = pIVar15;
      (**(code **)(*(longlong *)pIVar15 + 8))(pIVar15);
      local_128 = &local_170;
      plVar9 = (longlong *)FUN_1403f1aa0(ppIVar11 + 8,local_70,local_198);
      puVar5 = (undefined8 *)FUN_1403fc250(*plVar9 + 0x18,&local_60);
      pIVar7 = local_170;
      pIVar12 = (IUnknown *)*puVar5;
      if (pIVar12 != local_170) {
        if (local_170 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_170 + 8))(local_170);
          pIVar12 = (IUnknown *)*puVar5;
        }
        *puVar5 = pIVar7;
        if (pIVar12 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)pIVar12 + 0x10))();
        }
      }
      local_1b8 = local_1c0;
      if (local_1c0 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_1c0 + 8))();
      }
      FUN_140382a00(ppIVar11,local_198[0],&local_1b8);
      if (local_170 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_170 + 0x10))();
      }
      if (local_1c0 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_1c0 + 0x10))();
      }
    }
    puVar5 = local_f0;
    *local_f0 = pIVar15;
    local_190 = (IUnknown *)0x0;
    (**(code **)(*(longlong *)pIVar17 + 0x10))(pIVar17);
  }
  else {
    local_1b8 = pIVar15;
    if (pIVar15 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar15 + 8))(pIVar15);
    }
    puVar5 = local_f0;
    FUN_140410d90(local_f0,&local_1b8,1,(ulonglong)local_170 & 0xffffffff);
    (**(code **)(*(longlong *)pIVar17 + 0x10))(pIVar17);
    if (pIVar15 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar15 + 0x10))(pIVar15);
    }
  }
  return puVar5;
}



//===========================================================
// FUN_14117d4e0 @ 14117d4e0   (5599 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x00014117d661) */

void FUN_14117d4e0(longlong param_1,undefined8 param_2)

{
  int *piVar1;
  IUnknown *pIVar2;
  int *piVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  int iVar7;
  int *piVar8;
  longlong *plVar9;
  undefined8 uVar10;
  undefined8 *puVar11;
  longlong *plVar12;
  longlong *plVar13;
  undefined8 *puVar14;
  IUnknown *pIVar15;
  ulonglong uVar16;
  longlong lVar17;
  longlong lVar18;
  IUnknown *pIVar19;
  undefined1 auStack_3b8 [32];
  undefined8 local_398;
  undefined8 local_390;
  uint local_388;
  ulonglong local_380;
  longlong *local_378;
  longlong *local_370;
  int *local_368;
  longlong *local_360;
  int local_358;
  int local_354;
  short local_350;
  undefined6 uStack_34e;
  longlong lStack_348;
  undefined8 local_340;
  undefined4 local_338;
  undefined4 uStack_334;
  undefined8 uStack_330;
  undefined8 local_328;
  longlong *local_320;
  int *local_318;
  longlong local_310;
  longlong *local_308;
  longlong *local_300;
  longlong *local_2f8;
  longlong *local_2f0;
  longlong *local_2e8;
  longlong *local_2e0;
  longlong *local_2d8;
  longlong *local_2d0;
  IUnknown *local_2c8;
  IUnknown *local_2c0;
  longlong *local_2b8;
  longlong *local_2b0;
  uint local_2a8;
  undefined4 uStack_2a4;
  undefined4 uStack_2a0;
  undefined4 uStack_29c;
  undefined8 local_298;
  uint local_290;
  undefined4 uStack_28c;
  undefined4 uStack_288;
  undefined4 uStack_284;
  undefined8 local_280;
  uint local_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined8 local_268;
  longlong local_260;
  longlong local_258;
  longlong local_250;
  longlong local_248;
  longlong local_240;
  longlong *local_238;
  longlong *local_230;
  longlong *local_228;
  longlong *local_220;
  longlong *local_218;
  longlong *local_210;
  longlong *local_208;
  longlong *local_200;
  longlong *local_1f8;
  longlong *local_1f0;
  undefined1 local_1e8 [8];
  longlong *local_1e0;
  undefined1 local_1d8 [8];
  longlong *local_1d0;
  undefined1 local_1c8 [8];
  longlong *local_1c0;
  longlong *local_1b8;
  longlong *local_1b0;
  longlong *local_1a8;
  longlong *local_1a0;
  longlong *local_198;
  longlong *local_190;
  uint local_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 local_178;
  uint local_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined8 local_160;
  uint local_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 local_148;
  undefined1 local_140 [8];
  longlong local_138;
  undefined1 local_130 [8];
  longlong local_128;
  int *local_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  longlong *local_d0;
  undefined8 local_c8;
  longlong lStack_c0;
  undefined8 local_b8;
  uint local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined4 local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_3b8;
  lVar17 = -1;
  do {
    lVar17 = lVar17 + 1;
  } while (L"/backgrnd"[lVar17] != L'\0');
  FUN_14040ea40(&DAT_143aca7c0,&local_318);
  piVar8 = local_318;
  local_368 = (int *)0x0;
  piVar3 = local_368;
  if ((local_318 != (int *)0x0) && (piVar1 = local_318 + -4, piVar1 != (int *)0x0)) {
    if (*piVar1 == -1) {
      FUN_142e52d50(0xcb,1);
      uVar16 = 0xffffffffffffffff;
      do {
        uVar16 = uVar16 + 1;
      } while (*(short *)((longlong)piVar8 + uVar16 * 2) != 0);
      iVar7 = (int)uVar16;
      iVar4 = 0;
      if (0 < iVar7) {
        iVar4 = iVar7;
      }
      piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar4 * 2 + 0x12));
      piVar8[1] = iVar4;
      *piVar8 = -1;
      piVar3 = piVar8 + 4;
      piVar8[2] = 0;
      *(undefined2 *)piVar3 = 0;
      local_120 = piVar3;
      FUN_142ef7ba0(piVar3,local_318,(longlong)iVar7 * 2);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar7 != -1) goto LAB_14117d615;
        if (piVar3 == (int *)0x0) {
          uVar16 = 0;
        }
        else {
          uVar16 = 0xffffffffffffffff;
          do {
            uVar16 = uVar16 + 1;
          } while (*(short *)((longlong)piVar3 + uVar16 * 2) != 0);
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],uVar16 & 0xffffffff);
        *piVar8 = 1;
LAB_14117d615:
        *(undefined2 *)((longlong)iVar7 * 2 + (longlong)piVar3) = 0;
      }
      iVar4 = (int)uVar16;
      if ((iVar4 < 0) || (piVar8[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,uVar16 & 0xffffffff);
      }
      piVar8[2] = iVar4 * 2;
      if (local_368 != (int *)0x0) {
        FUN_1401bebb0(local_368 + -4);
      }
    }
    else {
      if (*piVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      if (local_368 != (int *)0x0) {
        FUN_1401bebb0(local_368 + -4);
      }
      local_368 = piVar8;
      piVar3 = local_368;
    }
  }
  local_368 = piVar3;
  puVar14 = (undefined8 *)0x0;
  FUN_142748650(param_1,param_2,&local_368,0);
  lVar17 = param_1 + 0x2e0;
  FUN_141aa3af0(lVar17,param_1,0,0);
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390._0_1_ = 1;
  local_398._0_4_ = 0;
  FUN_141ac3370(lVar17,DAT_143aca7c0,0,0);
  lVar18 = -1;
  do {
    lVar18 = lVar18 + 1;
  } while (L"/letterIcon"[lVar18] != L'\0');
  FUN_14040ea40(&DAT_143aca7c0,&local_260);
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390 = CONCAT71(local_390._1_7_,1);
  local_398 = (uint *)((ulonglong)local_398._4_4_ << 0x20);
  FUN_141ac3370(lVar17,local_260,0,0);
  if (local_260 != 0) {
    FUN_1401bebb0(local_260 + -0x10);
  }
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390._0_1_ = 1;
  local_398 = (uint *)((ulonglong)local_398 & 0xffffffff00000000);
  FUN_141ac3370(lVar17,DAT_143aca7c8,0,0);
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390._0_1_ = 1;
  local_398 = (uint *)((ulonglong)local_398 & 0xffffffff00000000);
  FUN_141ac3370(lVar17,DAT_143aca7d0,0,0);
  lVar18 = -1;
  do {
    lVar18 = lVar18 + 1;
  } while ((&DAT_1433741f8)[lVar18] != 0);
  FUN_14040ea40(&DAT_143aca7d0,&local_258,&DAT_1433741f8);
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390 = CONCAT71(local_390._1_7_,1);
  local_398 = (uint *)((ulonglong)local_398 & 0xffffffff00000000);
  FUN_141ac3370(lVar17,local_258,0,0);
  if (local_258 != 0) {
    FUN_1401bebb0(local_258 + -0x10);
  }
  lVar18 = -1;
  do {
    lVar18 = lVar18 + 1;
  } while (L"/font"[lVar18] != L'\0');
  FUN_14040ea40(&DAT_143aca7d0,&local_250,L"/font");
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390 = CONCAT71(local_390._1_7_,1);
  local_398 = (uint *)((ulonglong)local_398 & 0xffffffff00000000);
  FUN_141ac3370(lVar17,local_250,0,0);
  if (local_250 != 0) {
    FUN_1401bebb0(local_250 + -0x10);
  }
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390._0_1_ = 1;
  local_398 = (uint *)((ulonglong)local_398 & 0xffffffff00000000);
  FUN_141ac3370(lVar17,DAT_143aca7d8,0,0);
  lVar18 = -1;
  do {
    lVar18 = lVar18 + 1;
  } while ((&DAT_1433741f8)[lVar18] != 0);
  FUN_14040ea40(&DAT_143aca7d8,&local_248,&DAT_1433741f8);
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390 = CONCAT71(local_390._1_7_,1);
  local_398 = (uint *)((ulonglong)local_398 & 0xffffffff00000000);
  FUN_141ac3370(lVar17,local_248,0,0);
  if (local_248 != 0) {
    FUN_1401bebb0(local_248 + -0x10);
  }
  lVar18 = -1;
  do {
    lVar18 = lVar18 + 1;
  } while (L"/font"[lVar18] != L'\0');
  FUN_14040ea40(&DAT_143aca7d8,&local_240);
  local_380 = 0;
  local_388 = local_388 & 0xffffff00;
  local_390 = CONCAT71(local_390._1_7_,1);
  local_398 = (uint *)((ulonglong)local_398 & 0xffffffff00000000);
  FUN_141ac3370(lVar17,local_240,0,0);
  if (local_240 != 0) {
    FUN_1401bebb0(local_240 + -0x10);
  }
  plVar9 = (longlong *)FUN_141adaca0(lVar17,&local_238,L"notice");
  plVar13 = *(longlong **)(param_1 + 0x25c8);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x25c8) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_238 != (longlong *)0x0) {
    (**(code **)(*local_238 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_230,L"mailTitle");
  plVar13 = *(longlong **)(param_1 + 0x2e8);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x2e8) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_230 != (longlong *)0x0) {
    (**(code **)(*local_230 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_228,L"itemTimeLeft");
  plVar13 = *(longlong **)(param_1 + 0x2f0);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x2f0) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_228 != (longlong *)0x0) {
    (**(code **)(*local_228 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_220,L"mailTitleSelected");
  plVar13 = *(longlong **)(param_1 + 0x2f8);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x2f8) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_220 != (longlong *)0x0) {
    (**(code **)(*local_220 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_1a8,L"itemTimeLeftSelected");
  plVar13 = *(longlong **)(param_1 + 0x300);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x300) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_1a8 != (longlong *)0x0) {
    (**(code **)(*local_1a8 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_1b0,L"detailItemCountNum");
  plVar13 = *(longlong **)(param_1 + 0x308);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x308) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_1b0 != (longlong *)0x0) {
    (**(code **)(*local_1b0 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_1b8,L"detailItemCountNumStroke");
  plVar13 = *(longlong **)(param_1 + 0x310);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x310) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_1b8 != (longlong *)0x0) {
    (**(code **)(*local_1b8 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_1c0,L"detailItemName");
  plVar13 = *(longlong **)(param_1 + 0x318);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x318) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_1c0 != (longlong *)0x0) {
    (**(code **)(*local_1c0 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_198,L"detailItemCount");
  plVar13 = *(longlong **)(param_1 + 800);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 800) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_198 != (longlong *)0x0) {
    (**(code **)(*local_198 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_1a0,L"detailSenderReceived");
  plVar13 = *(longlong **)(param_1 + 0x328);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x328) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_1a0 != (longlong *)0x0) {
    (**(code **)(*local_1a0 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_190,L"detailSenderName");
  plVar13 = *(longlong **)(param_1 + 0x330);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x330) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_190 != (longlong *)0x0) {
    (**(code **)(*local_190 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_218,L"detailReceivedDate");
  plVar13 = *(longlong **)(param_1 + 0x338);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x338) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_218 != (longlong *)0x0) {
    (**(code **)(*local_218 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adbfc0(lVar17,&local_210,L"detailBody");
  plVar13 = *(longlong **)(param_1 + 0x340);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x340) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_210 != (longlong *)0x0) {
    (**(code **)(*local_210 + 0x10))();
  }
  uVar10 = FUN_141ada1c0(lVar17,local_140,L"itemList");
  FUN_141182780(param_1 + 0x2598,uVar10);
  lVar18 = local_138;
  if (local_138 != 0) {
    if (0xffffe < *(longlong *)(local_138 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar13 = (longlong *)(lVar18 + 0x20);
    lVar18 = *plVar13;
    *plVar13 = *plVar13 + -1;
    UNLOCK();
    if ((int)lVar18 == 1) {
      puVar11 = (undefined8 *)(local_138 + 0x18);
      if (local_138 == 0) {
        puVar11 = puVar14;
      }
      if (puVar11 != (undefined8 *)0x0) {
        (**(code **)*puVar11)(puVar11,1);
      }
    }
  }
  uVar10 = FUN_141ada1c0(lVar17,local_130,L"detailBody");
  FUN_141182780(param_1 + 0x25a8,uVar10);
  lVar18 = local_128;
  if (local_128 != 0) {
    if (0xffffe < *(longlong *)(local_128 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar13 = (longlong *)(lVar18 + 0x20);
    lVar18 = *plVar13;
    *plVar13 = *plVar13 + -1;
    UNLOCK();
    if ((int)lVar18 == 1) {
      puVar11 = (undefined8 *)(local_128 + 0x18);
      if (local_128 == 0) {
        puVar11 = puVar14;
      }
      if (puVar11 != (undefined8 *)0x0) {
        (**(code **)*puVar11)(puVar11,1);
      }
    }
  }
  plVar9 = (longlong *)FUN_141adb9c0(lVar17,&local_208,L"list");
  plVar13 = *(longlong **)(param_1 + 0x25d0);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x25d0) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_208 != (longlong *)0x0) {
    (**(code **)(*local_208 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adb9c0(lVar17,&local_200,L"select");
  plVar13 = *(longlong **)(param_1 + 0x25d8);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x25d8) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_200 != (longlong *)0x0) {
    (**(code **)(*local_200 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adb9c0(lVar17,&local_1f8,L"closed");
  plVar13 = *(longlong **)(param_1 + 0x25e0);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x25e0) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_1f8 != (longlong *)0x0) {
    (**(code **)(*local_1f8 + 0x10))();
  }
  plVar9 = (longlong *)FUN_141adb9c0(lVar17,&local_1f0,L"open");
  plVar13 = *(longlong **)(param_1 + 0x25e8);
  if (plVar13 != (longlong *)*plVar9) {
    *(longlong **)(param_1 + 0x25e8) = (longlong *)*plVar9;
    *plVar9 = 0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
    }
  }
  if (local_1f0 != (longlong *)0x0) {
    (**(code **)(*local_1f0 + 0x10))();
  }
  pIVar19 = DAT_143add058;
  if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_350);
  if (DAT_143a8b8d8 == 8) {
    if (local_350 == 8) {
      local_350 = 0;
      if (lStack_348 != 0) {
        (*DAT_143ad5990)(lStack_348 + -4);
      }
    }
    else {
      iVar4 = (*DAT_143262a18)(&local_350);
      if (iVar4 < 0) goto LAB_14117ea78;
    }
    local_350 = 8;
    puVar11 = puVar14;
    if (DAT_143a8b8e0 != 0) {
      puVar11 = (undefined8 *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_348 = FUN_1401a5fa0(DAT_143a8b8e0,puVar11);
  }
  else {
    if ((local_350 == 8) && (local_350 = 0, lStack_348 != 0)) {
      (*DAT_143ad5990)(lStack_348 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_350,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14117ea78:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
  }
  (*DAT_143262a20)(&local_338);
  if (DAT_143a8b8d8 == 8) {
    if ((short)local_338 == 8) {
      local_338 = (uint)local_338._2_2_ << 0x10;
      if (uStack_330 != 0) {
        (*DAT_143ad5990)(uStack_330 + -4);
      }
    }
    else {
      iVar4 = (*DAT_143262a18)(&local_338);
      if (iVar4 < 0) goto LAB_14117ea80;
    }
    local_338 = CONCAT22(local_338._2_2_,8);
    puVar11 = puVar14;
    if (DAT_143a8b8e0 != 0) {
      puVar11 = (undefined8 *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    uStack_330 = FUN_1401a5fa0(DAT_143a8b8e0,puVar11);
  }
  else {
    if (((short)local_338 == 8) && (local_338 = (uint)local_338._2_2_ << 0x10, uStack_330 != 0)) {
      (*DAT_143ad5990)(uStack_330 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_338,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14117ea80:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
  }
  local_310 = 0;
  puVar11 = (undefined8 *)
            FUN_14019ba10(&local_310,PTR_s_Item_Special_0900_img__08d_143a449a0,0x895441);
  puVar11 = (undefined8 *)FUN_1401a5780(local_1e8,*puVar11);
  local_370 = puVar11;
  (*DAT_143262a20)(&local_188);
  if ((undefined8 *)*puVar11 != (undefined8 *)0x0) {
    puVar14 = *(undefined8 **)*puVar11;
  }
  local_c8 = CONCAT62(uStack_34e,local_350);
  lStack_c0 = lStack_348;
  local_b8 = local_340;
  local_a8 = local_338;
  uStack_a4 = uStack_334;
  uStack_a0 = (undefined4)uStack_330;
  uStack_9c = uStack_330._4_4_;
  local_98 = local_328;
  local_398 = &local_188;
  iVar4 = (**(code **)(*(longlong *)pIVar19 + 0x48))(pIVar19,puVar14,&local_a8,&local_c8);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,pIVar19,(_GUID *)&DAT_1432743e8);
  }
  local_290 = local_188;
  uStack_28c = uStack_184;
  uStack_288 = uStack_180;
  uStack_284 = uStack_17c;
  local_280 = local_178;
  local_188 = local_188 & 0xffff0000;
  thunk_FUN_1401be120(puVar11);
  uVar10 = FUN_1409339d0(&local_1e0,&local_290);
  FUN_1401a5040(&local_2c8,uVar10);
  if (local_1e0 != (longlong *)0x0) {
    (**(code **)(*local_1e0 + 0x10))();
  }
  if ((short)local_290 == 8) {
    local_290 = local_290 & 0xffff0000;
    if (CONCAT44(uStack_284,uStack_288) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_284,uStack_288) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_290);
  }
  if (local_310 != 0) {
    FUN_14019f2c0(local_310 + -0x10);
  }
  if ((short)local_338 == 8) {
    local_338 = local_338 & 0xffff0000;
    if (uStack_330 != 0) {
      (*DAT_143ad5990)(uStack_330 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_338);
  }
  if (local_350 == 8) {
    local_350 = 0;
    if (lStack_348 != 0) {
      (*DAT_143ad5990)(lStack_348 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_350);
  }
  pIVar19 = local_2c8;
  if (local_2c8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plVar12 = (longlong *)FUN_1401a5890(local_1d8,PTR_u_iconRaw_143a45dd0);
  local_370 = plVar12;
  (*DAT_143262a20)(&local_170);
  plVar9 = (longlong *)0x0;
  plVar13 = plVar9;
  if ((undefined8 *)*plVar12 != (undefined8 *)0x0) {
    plVar13 = *(longlong **)*plVar12;
  }
  iVar4 = (**(code **)(*(longlong *)pIVar19 + 0x28))(pIVar19,plVar13,&local_170);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,pIVar19,(_GUID *)&DAT_143272478);
  }
  local_2a8 = local_170;
  uStack_2a4 = uStack_16c;
  uStack_2a0 = uStack_168;
  uStack_29c = uStack_164;
  local_298 = local_160;
  local_170 = local_170 & 0xffff0000;
  thunk_FUN_1401be120(plVar12);
  uVar10 = FUN_1409339d0(&local_1d0,&local_2a8);
  FUN_1401a5040(&local_2c0,uVar10);
  if (local_1d0 != (longlong *)0x0) {
    (**(code **)(*local_1d0 + 0x10))();
  }
  if ((short)local_2a8 == 8) {
    local_2a8 = local_2a8 & 0xffff0000;
    if (CONCAT44(uStack_29c,uStack_2a0) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_29c,uStack_2a0) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_2a8);
  }
  pIVar15 = local_2c0;
  if (local_2c0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  local_110 = 0;
  uStack_108 = 0;
  local_100 = 0;
  uStack_f8 = 0;
  local_f0 = 0;
  uStack_e8 = 0;
  local_e0 = 0;
  uStack_d8 = 0;
  FUN_142f121cc(1,&local_110,10);
  local_88 = local_110;
  uStack_80 = uStack_108;
  local_78 = local_100;
  uStack_70 = uStack_f8;
  local_68 = (undefined4)local_f0;
  uStack_64 = local_f0._4_4_;
  uStack_60 = (undefined4)uStack_e8;
  uStack_5c = uStack_e8._4_4_;
  local_58 = (undefined4)local_e0;
  uStack_54 = local_e0._4_4_;
  uStack_50 = (undefined4)uStack_d8;
  uStack_4c = uStack_d8._4_4_;
  plVar12 = (longlong *)FUN_1401a5890(local_1c8,&local_88);
  local_370 = plVar12;
  (*DAT_143262a20)(&local_158);
  plVar13 = plVar9;
  if ((undefined8 *)*plVar12 != (undefined8 *)0x0) {
    plVar13 = *(longlong **)*plVar12;
  }
  iVar4 = (**(code **)(*(longlong *)pIVar15 + 0x28))(pIVar15,plVar13,&local_158);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,pIVar15,(_GUID *)&DAT_143272478);
  }
  local_278 = local_158;
  uStack_274 = uStack_154;
  uStack_270 = uStack_150;
  uStack_26c = uStack_14c;
  local_268 = local_148;
  local_158 = local_158 & 0xffff0000;
  thunk_FUN_1401be120(plVar12);
  plVar13 = (longlong *)FUN_1409339d0(&local_2b0,&local_278);
  plVar13 = (longlong *)*plVar13;
  local_370 = plVar13;
  if (plVar13 == (longlong *)0x0) {
    iVar4 = -0x7fffbffe;
  }
  else {
    (**(code **)(*plVar13 + 8))(plVar13);
    local_308 = (longlong *)0x0;
    iVar4 = (**(code **)*plVar13)(plVar13,&DAT_14327ac98,&local_308);
    if (-1 < iVar4) {
      plVar9 = local_308;
    }
  }
  if (plVar13 != (longlong *)0x0) {
    (**(code **)(*plVar13 + 0x10))(plVar13);
  }
  if (((iVar4 + 0x80000000U & 0x80000000) == 0) && (iVar4 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar4);
  }
  plVar13 = *(longlong **)(param_1 + 0x25f0);
  plVar12 = (longlong *)0x0;
  if (plVar13 != plVar9) {
    *(longlong **)(param_1 + 0x25f0) = plVar9;
    plVar9 = plVar12;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
      plVar9 = (longlong *)0x0;
    }
  }
  if (plVar9 != (longlong *)0x0) {
    (**(code **)(*plVar9 + 0x10))(plVar9);
  }
  if (local_2b0 != (longlong *)0x0) {
    (**(code **)(*local_2b0 + 0x10))();
  }
  if ((short)local_278 == 8) {
    local_278 = local_278 & 0xffff0000;
    if (CONCAT44(uStack_26c,uStack_270) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_26c,uStack_270) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_278);
  }
  plVar13 = *(longlong **)(param_1 + 0x25f0);
  if (plVar13 != (longlong *)0x0) {
    (**(code **)(*plVar13 + 0x100))(plVar13,0);
    plVar13 = *(longlong **)(param_1 + 0x25f0);
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x110))(plVar13,0);
      if (*(longlong *)(param_1 + 0x25d0) != 0) {
        uVar5 = FUN_142bf7d80(param_1);
        uVar6 = FUN_142bf7d70(param_1);
        local_380 = local_380 & 0xffffffff00000000;
        local_388 = 0;
        local_390 = 0;
        local_398 = (uint *)CONCAT44(local_398._4_4_,1);
        FUN_141aa5a30(lVar17,&local_2b8,uVar6,uVar5);
        plVar13 = local_2b8;
        if (local_2b8 != (longlong *)0x0) {
          if (*(longlong **)(param_1 + 0x25c0) != local_2b8) {
            (**(code **)(*local_2b8 + 8))(local_2b8);
            plVar9 = *(longlong **)(param_1 + 0x25c0);
            *(longlong **)(param_1 + 0x25c0) = plVar13;
            if (plVar9 != (longlong *)0x0) {
              (**(code **)(*plVar9 + 0x10))();
            }
          }
          pIVar2 = *(IUnknown **)(param_1 + 0x25d0);
          if (pIVar2 == (IUnknown *)0x0) {
LAB_14117eaa7:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          local_358 = 0;
          iVar4 = (**(code **)(*(longlong *)pIVar2 + 0xf8))(pIVar2,&local_358);
          if (iVar4 < 0) {
            _com_issue_errorex(iVar4,pIVar2,(_GUID *)&DAT_14327ac98);
          }
          *(int *)(param_1 + 0x2600) = -local_358;
          pIVar2 = *(IUnknown **)(param_1 + 0x25d0);
          if (pIVar2 == (IUnknown *)0x0) goto LAB_14117eaa7;
          local_354 = 0;
          iVar4 = (**(code **)(*(longlong *)pIVar2 + 0x108))(pIVar2,&local_354);
          if (iVar4 < 0) {
            _com_issue_errorex(iVar4,pIVar2,(_GUID *)&DAT_14327ac98);
          }
          *(int *)(param_1 + 0x2604) = -local_354;
          FUN_14090ead0(&local_320,DAT_143aca7d0);
          local_300 = local_320;
          if (local_320 != (longlong *)0x0) {
            (**(code **)(*local_320 + 8))();
          }
          iVar7 = FUN_140910eb0(&local_300,&DAT_143386c38,0);
          iVar4 = 1;
          if (1 < iVar7) {
            iVar4 = iVar7;
          }
          *(int *)(param_1 + 0x260c) = iVar4;
          local_2f8 = local_320;
          if (local_320 != (longlong *)0x0) {
            (**(code **)(*local_320 + 8))();
          }
          uVar5 = FUN_140910eb0(&local_2f8,L"space_x",0);
          *(undefined4 *)(param_1 + 0x2614) = uVar5;
          local_2f0 = local_320;
          if (local_320 != (longlong *)0x0) {
            (**(code **)(*local_320 + 8))();
          }
          uVar5 = FUN_140910eb0(&local_2f0,L"space_y",0);
          *(undefined4 *)(param_1 + 0x2618) = uVar5;
          if (*(longlong *)(param_1 + 0x25d0) == 0) {
LAB_14117ea9c:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          iVar4 = FUN_140404280();
          *(int *)(param_1 + 0x25f8) = iVar4 + *(int *)(param_1 + 0x2614);
          if (*(longlong *)(param_1 + 0x25d0) == 0) goto LAB_14117ea9c;
          iVar4 = FUN_140404240();
          *(int *)(param_1 + 0x25fc) = iVar4 + *(int *)(param_1 + 0x2618);
          FUN_14090ead0(&local_360,DAT_143aca7d8);
          local_2e8 = local_360;
          if (local_360 != (longlong *)0x0) {
            (**(code **)(*local_360 + 8))();
          }
          iVar7 = FUN_140910eb0(&local_2e8,&DAT_143386c38,0);
          iVar4 = 1;
          if (1 < iVar7) {
            iVar4 = iVar7;
          }
          *(int *)(param_1 + 0x2610) = iVar4;
          local_2e0 = local_360;
          if (local_360 != (longlong *)0x0) {
            (**(code **)(*local_360 + 8))();
          }
          uVar5 = FUN_140910ca0(&local_2e0,"detailBodyWidth",0);
          *(undefined4 *)(param_1 + 0x2648) = uVar5;
          local_2d8 = local_360;
          if (local_360 != (longlong *)0x0) {
            (**(code **)(*local_360 + 8))();
          }
          uVar5 = FUN_140910ca0(&local_2d8,"detailBodyHeight",0);
          *(undefined4 *)(param_1 + 0x264c) = uVar5;
          local_2d0 = local_360;
          if (local_360 != (longlong *)0x0) {
            (**(code **)(*local_360 + 8))();
          }
          uVar5 = FUN_140910ca0(&local_2d0,"detailBodyLineSpace",0);
          *(undefined4 *)(param_1 + 0x2650) = uVar5;
          local_378 = local_360;
          if (local_360 != (longlong *)0x0) {
            (**(code **)(*local_360 + 8))();
          }
          uVar5 = FUN_140910ca0(&local_378,"detailItemNameWidth",0);
          *(undefined4 *)(param_1 + 0x2654) = uVar5;
          local_d0 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x80);
          if (local_d0 != (longlong *)0x0) {
            uVar5 = *(undefined4 *)(param_1 + 0x2650);
            uVar6 = *(undefined4 *)(param_1 + 0x2648);
            local_d0[3] = 0;
            local_d0[1] = 0;
            local_d0[2] = 0;
            *local_d0 = (longlong)&PTR_FUN_14336e7a8;
            local_d0[7] = 0;
            local_d0[8] = 0;
            local_d0[9] = 0;
            local_d0[10] = 0;
            local_d0[0xb] = 0;
            local_d0[0xc] = 0;
            local_d0[0xd] = 0;
            local_d0[0xe] = 0;
            local_d0[0xf] = 0;
            *(undefined4 *)((longlong)local_d0 + 0x24) = uVar6;
            *(undefined4 *)(local_d0 + 4) = 0;
            local_d0[5] = 0;
            *(undefined4 *)(local_d0 + 6) = uVar5;
            plVar12 = local_d0;
          }
          if ((*(longlong *)(param_1 + 0x2660) - 1U < 999) ||
             (*(longlong *)(param_1 + 0x2660) == -1)) {
            FUN_142e52ed0(0x447);
          }
          if (plVar12 != (longlong *)0x0) {
            if (0xfffff < (ulonglong)plVar12[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            plVar12[1] = plVar12[1] + 1;
            UNLOCK();
            pIVar15 = local_2c0;
            pIVar19 = local_2c8;
          }
          local_118 = *(undefined8 *)(param_1 + 0x2660);
          *(longlong **)(param_1 + 0x2660) = plVar12;
          FUN_140ed6f40(&local_120);
          FUN_1411820e0(param_1);
          FUN_142ced8f0(DAT_143aa84a0,param_1);
          if (local_360 != (longlong *)0x0) {
            (**(code **)(*local_360 + 0x10))();
          }
          if (local_320 != (longlong *)0x0) {
            (**(code **)(*local_320 + 0x10))();
          }
        }
        if (local_2b8 != (longlong *)0x0) {
          (**(code **)(*local_2b8 + 0x10))(local_2b8);
        }
      }
      (**(code **)(*(longlong *)pIVar15 + 0x10))(pIVar15);
      (**(code **)(*(longlong *)pIVar19 + 0x10))(pIVar19);
      if (local_318 != (int *)0x0) {
        FUN_1401bebb0(local_318 + -4);
      }
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_1417b5c50 @ 1417b5c50   (1535 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

longlong **
FUN_1417b5c50(longlong param_1,longlong **param_2,int param_3,undefined4 param_4,int param_5)

{
  IUnknown *pIVar1;
  longlong *plVar2;
  longlong lVar3;
  int iVar4;
  longlong **pplVar5;
  int *piVar6;
  undefined8 uVar7;
  longlong *plVar8;
  longlong *plVar9;
  uint uVar10;
  uint uVar11;
  ulonglong uVar12;
  undefined1 auStack_178 [32];
  longlong *local_158;
  int local_150;
  uint local_148;
  longlong *local_140;
  longlong **local_138;
  longlong **local_130;
  longlong **local_128;
  uint local_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined8 local_110;
  uint local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 local_f8;
  longlong **local_f0;
  longlong **local_e8;
  longlong local_e0;
  IUnknown *local_d8;
  IUnknown *local_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined4 local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_178;
  local_140 = (longlong *)CONCAT44(local_140._4_4_,param_4);
  local_148 = 0;
  local_e0 = 0;
  if ((param_5 < 1) && (param_5 = 0, 0x31 < param_3)) {
    if (param_3 < 100) {
      param_5 = 1;
    }
    else {
      param_5 = (999 < param_3) + 2;
    }
  }
  local_130 = param_2;
  local_e8 = param_2;
  FUN_14019ba10(&local_e0,PTR_DAT_143a44e20,param_5 + 9000000);
  pIVar1 = *(IUnknown **)(param_1 + 0x70);
  if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  pplVar5 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x18);
  lVar3 = local_e0;
  local_130 = pplVar5;
  if (pplVar5 == (longlong **)0x0) {
    local_138 = (longlong **)0x0;
  }
  else {
    pplVar5[1] = (longlong *)0x0;
    *(undefined4 *)(pplVar5 + 2) = 1;
    local_138 = pplVar5;
    if (local_e0 == 0) {
      *pplVar5 = (longlong *)0x0;
      param_4 = local_140._0_4_;
    }
    else {
      local_150 = 0;
      local_158 = (longlong *)0x0;
      iVar4 = (*DAT_1432627f8)(0xfde9,0,local_e0,0xffffffff);
      uVar12 = (ulonglong)(longlong)(iVar4 * 2) >> 1;
      iVar4 = (int)uVar12;
      uVar10 = iVar4 - 1;
      piVar6 = (int *)(*DAT_143ad5980)((uVar12 & 0xffffffff) * 2 + 4);
      if (piVar6 == (int *)0x0) {
        plVar9 = (longlong *)0x0;
      }
      else {
        *piVar6 = uVar10 * 2;
        plVar9 = (longlong *)(piVar6 + 1);
        *(undefined2 *)((longlong)plVar9 + (ulonglong)uVar10 * 2) = 0;
      }
      local_158 = plVar9;
      local_150 = iVar4;
      (*DAT_1432627f8)(0xfde9,0,lVar3,0xffffffff);
      *pplVar5 = plVar9;
      param_4 = local_140._0_4_;
    }
  }
  plVar9 = (longlong *)0x0;
  if (local_138 == (longlong **)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x8007000e);
  }
  local_130 = (longlong **)&local_138;
  (*DAT_143262a20)(&local_108);
  plVar8 = plVar9;
  if (local_138 != (longlong **)0x0) {
    plVar8 = *local_138;
  }
  iVar4 = (**(code **)(*(longlong *)pIVar1 + 0x28))(pIVar1,plVar8,&local_108);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,pIVar1,(_GUID *)&DAT_143272478);
  }
  local_120 = local_108;
  uStack_11c = uStack_104;
  uStack_118 = uStack_100;
  uStack_114 = uStack_fc;
  local_110 = local_f8;
  local_108 = local_108 & 0xffff0000;
  FUN_1401be120(&local_138);
  uVar7 = FUN_1409339d0(&local_f0,&local_120);
  FUN_1401a5040(&local_d8,uVar7);
  if ((longlong ***)local_f0 != (longlong ***)0x0) {
    (*(code *)(*local_f0)[2])();
  }
  if ((short)local_120 == 8) {
    local_120 = local_120 & 0xffff0000;
    if (CONCAT44(uStack_114,uStack_118) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_114,uStack_118) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_120);
  }
  if (local_d8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  FUN_1401bb8d0(&local_140,PTR_u_iconRaw_143a45dd0);
  local_130 = &local_140;
  (*DAT_143262a20)(&local_120);
  plVar8 = plVar9;
  if (local_140 != (longlong *)0x0) {
    plVar8 = (longlong *)*local_140;
  }
  iVar4 = (**(code **)(*(longlong *)local_d8 + 0x28))(local_d8,plVar8,&local_120);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,local_d8,(_GUID *)&DAT_143272478);
  }
  local_108 = local_120;
  uStack_104 = uStack_11c;
  uStack_100 = uStack_118;
  uStack_fc = uStack_114;
  local_f8 = local_110;
  local_120 = local_120 & 0xffff0000;
  FUN_1401be120(&local_140);
  uVar7 = FUN_1409339d0(&local_128,&local_108);
  FUN_1401a5040(&local_d0,uVar7);
  if ((longlong ***)local_128 != (longlong ***)0x0) {
    (*(code *)(*local_128)[2])();
  }
  if ((short)local_108 == 8) {
    local_108 = local_108 & 0xffff0000;
    if (CONCAT44(uStack_fc,uStack_100) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_fc,uStack_100) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_108);
  }
  if (local_d0 == (IUnknown *)0x0) {
    local_140 = (longlong *)0x0;
    pplVar5 = &local_140;
    uVar10 = 0x34;
  }
  else {
    local_c8 = 0;
    uStack_c0 = 0;
    local_b8 = 0;
    uStack_b0 = 0;
    local_a8 = 0;
    uStack_a0 = 0;
    local_98 = 0;
    uStack_90 = 0;
    FUN_142f121cc(param_4,&local_c8,10);
    local_88 = local_c8;
    uStack_80 = uStack_c0;
    local_78 = local_b8;
    uStack_70 = uStack_b0;
    local_68 = (undefined4)local_a8;
    uStack_64 = local_a8._4_4_;
    uStack_60 = (undefined4)uStack_a0;
    uStack_5c = uStack_a0._4_4_;
    local_58 = (undefined4)local_98;
    uStack_54 = local_98._4_4_;
    uStack_50 = (undefined4)uStack_90;
    uStack_4c = uStack_90._4_4_;
    FUN_1401bb8d0(&local_138,&local_88);
    local_128 = (longlong **)&local_138;
    (*DAT_143262a20)(&local_108);
    if (local_138 != (longlong **)0x0) {
      plVar9 = *local_138;
    }
    iVar4 = (**(code **)(*(longlong *)local_d0 + 0x28))(local_d0,plVar9,&local_108);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,local_d0,(_GUID *)&DAT_143272478);
    }
    local_120 = local_108;
    uStack_11c = uStack_104;
    uStack_118 = uStack_100;
    uStack_114 = uStack_fc;
    local_110 = local_f8;
    local_108 = local_108 & 0xffff0000;
    FUN_1401be120(&local_138);
    local_148 = 0x71;
    pplVar5 = (longlong **)FUN_1409339d0(&local_130,&local_120);
    uVar10 = 0x73;
  }
  plVar8 = local_140;
  *param_2 = (longlong *)0x0;
  plVar9 = *pplVar5;
  local_148 = uVar10;
  if (plVar9 == (longlong *)0x0) {
    plVar2 = *param_2;
    if (plVar2 != (longlong *)0x0) {
      *param_2 = (longlong *)0x0;
      (**(code **)(*plVar2 + 0x10))();
    }
    iVar4 = -0x7fffbffe;
  }
  else {
    (**(code **)(*plVar9 + 8))(plVar9);
    local_f0 = (longlong **)0x0;
    iVar4 = (**(code **)*plVar9)(plVar9,&DAT_14327ac98,&local_f0);
    local_128 = (longlong **)(longlong ***)0x0;
    if (-1 < iVar4) {
      local_128 = local_f0;
    }
    if (*local_e8 != (longlong *)0x0) {
      (**(code **)(**local_e8 + 0x10))();
    }
    *local_e8 = (longlong *)local_128;
  }
  if (plVar9 != (longlong *)0x0) {
    (**(code **)(*plVar9 + 0x10))(plVar9);
  }
  if (((iVar4 + 0x80000000U & 0x80000000) == 0) && (iVar4 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar4);
  }
  uVar11 = uVar10 | 8;
  if (((uVar10 & 4) != 0) &&
     (uVar11 = uVar10 & 0xfffffffb | 8, local_148 = uVar11, plVar8 != (longlong *)0x0)) {
    (**(code **)(*plVar8 + 0x10))(plVar8);
  }
  if (((uVar11 & 2) != 0) &&
     (uVar11 = uVar11 & 0xfffffffd, local_148 = uVar11, local_130 != (longlong **)0x0)) {
    (*(code *)(*local_130)[2])();
  }
  if ((uVar11 & 1) != 0) {
    if ((short)local_120 == 8) {
      local_120 = local_120 & 0xffff0000;
      if (CONCAT44(uStack_114,uStack_118) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_114,uStack_118) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_120);
    }
  }
  if (local_d0 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_d0 + 0x10))(local_d0);
  }
  (**(code **)(*(longlong *)local_d8 + 0x10))(local_d8);
  if (lVar3 != 0) {
    FUN_14019f2c0(lVar3 + -0x10);
  }
  return local_e8;
}



//===========================================================
// FUN_14119a960 @ 14119a960   (1206 bytes)
//===========================================================

void FUN_14119a960(longlong *param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  int iVar3;
  longlong lVar4;
  longlong lVar5;
  longlong *plVar6;
  undefined8 *puVar7;
  int iVar8;
  longlong lVar9;
  longlong lVar10;
  int iVar11;
  undefined1 local_98 [8];
  longlong local_90;
  undefined1 local_88 [8];
  longlong *local_80;
  longlong *local_70;
  undefined8 *local_60;
  undefined1 local_58 [8];
  undefined8 *local_50;
  
  lVar4 = DAT_143ac8908;
  if (DAT_143ac8908 != 0) {
    plVar2 = param_1 + 0x5d;
    lVar9 = param_1[0x5e];
    for (lVar10 = *plVar2; lVar10 != lVar9; lVar10 = lVar10 + 0x10) {
      plVar6 = *(longlong **)(lVar10 + 8);
      local_70 = plVar6;
      if (plVar6 != (longlong *)0x0) {
        if (0xfffff < (ulonglong)plVar6[4]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        plVar6[4] = plVar6[4] + 1;
        UNLOCK();
      }
      plVar1 = local_70;
      if (plVar6 != (longlong *)0x0) {
        (**(code **)(*local_70 + 0x18))(local_70);
        if (0xffffe < plVar1[4] - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = plVar1 + 4;
        lVar5 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if (((int)lVar5 == 1) && (plVar6 = local_70 + 3, plVar6 != (longlong *)0x0)) {
          (**(code **)*plVar6)(plVar6,1);
        }
        local_70 = (longlong *)0x0;
      }
      plVar6 = local_70;
      if (local_70 != (longlong *)0x0) {
        if (0xffffe < local_70[4] - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar6 = plVar6 + 4;
        lVar5 = *plVar6;
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (((int)lVar5 == 1) && (plVar6 = local_70 + 3, plVar6 != (longlong *)0x0)) {
          (**(code **)*plVar6)(plVar6,1);
        }
        local_70 = (longlong *)0x0;
      }
    }
    lVar10 = param_1[0x5e];
    lVar9 = *plVar2;
    if (lVar9 != lVar10) {
      do {
        FUN_1411a2c30(lVar9);
        lVar9 = lVar9 + 0x10;
      } while (lVar9 != lVar10);
      lVar9 = *plVar2;
    }
    param_1[0x5e] = lVar9;
    lVar10 = *(longlong *)(lVar4 + 0x388);
    lVar4 = *(longlong *)(lVar4 + 0x390);
    if (lVar10 != lVar4) {
      iVar11 = 0x1f;
      iVar3 = 10000;
      do {
        puVar7 = *(undefined8 **)(lVar10 + 8);
        local_60 = puVar7;
        if (puVar7 != (undefined8 *)0x0) {
          if (0xfffff < (ulonglong)puVar7[1]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          puVar7[1] = puVar7[1] + 1;
          UNLOCK();
        }
        lVar5 = FUN_14019b780(&DAT_143ad68a0,0x11d8);
        lVar9 = 0;
        if (lVar5 != 0) {
          local_50 = local_60;
          if (puVar7 != (undefined8 *)0x0) {
            if (0xfffff < (ulonglong)puVar7[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            puVar7[1] = puVar7[1] + 1;
            UNLOCK();
          }
          lVar9 = FUN_1411991f0(lVar5,local_58);
        }
        lVar5 = lVar9 + 0x18;
        if (lVar9 == 0) {
          lVar5 = 0;
        }
        if (lVar5 == 0) {
          local_80 = (longlong *)0x0;
        }
        else {
          local_80 = (longlong *)(lVar5 + -0x18);
          if (local_80 != (longlong *)0x0) {
            if (0xfffff < *(ulonglong *)(lVar5 + 8)) {
              FUN_142e541f0();
            }
            LOCK();
            *(longlong *)(lVar5 + 8) = *(longlong *)(lVar5 + 8) + 1;
            UNLOCK();
          }
        }
        plVar6 = local_80;
        if (local_80 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
        }
        (**(code **)(*plVar6 + 0x10))(plVar6,param_1,iVar3,0,0,0xc6,0x23,0);
        FUN_1417113f0(plVar6,5,iVar11);
        lVar9 = param_1[0x5e];
        if (lVar9 == param_1[0x5f]) {
          FUN_1411a0690(plVar2,lVar9,local_88);
        }
        else {
          *(longlong **)(lVar9 + 8) = plVar6;
          if (0xfffff < (ulonglong)plVar6[4]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          plVar6[4] = plVar6[4] + 1;
          UNLOCK();
          param_1[0x5e] = param_1[0x5e] + 0x10;
        }
        plVar6 = local_80;
        iVar11 = iVar11 + 0x23;
        if (local_80 != (longlong *)0x0) {
          if (0xffffe < local_80[4] - 1U) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar6 = plVar6 + 4;
          lVar9 = *plVar6;
          *plVar6 = *plVar6 + -1;
          UNLOCK();
          if (((int)lVar9 == 1) && (plVar6 = local_80 + 3, plVar6 != (longlong *)0x0)) {
            (**(code **)*plVar6)(plVar6,1);
          }
          local_80 = (longlong *)0x0;
        }
        puVar7 = local_60;
        if (local_60 != (undefined8 *)0x0) {
          if (0xffffe < local_60[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar6 = puVar7 + 1;
          lVar9 = *plVar6;
          *plVar6 = *plVar6 + -1;
          UNLOCK();
          if ((int)lVar9 == 1) {
            (**(code **)*local_60)(local_60,1);
          }
          local_60 = (undefined8 *)0x0;
        }
        lVar10 = lVar10 + 0x10;
        iVar3 = iVar3 + 1;
      } while (lVar10 != lVar4);
    }
    FUN_141ada1c0(param_1 + 0x5c,local_98,L"entryScroll");
    if (local_90 != 0) {
      FUN_1416ee2d0(local_90,0);
      iVar3 = *(int *)((longlong)param_1 + 0x304);
      iVar8 = (int)(param_1[0x5e] - *plVar2 >> 4);
      iVar11 = 0;
      if (iVar3 < iVar8) {
        iVar11 = (iVar8 - iVar3) + 1;
      }
      if (local_90 == 0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_1416ee330(local_90,iVar11);
      if (local_90 == 0) {
        FUN_142e52ed0(0x431,0);
      }
      (**(code **)(*(longlong *)(local_90 + 8) + 0x70))((longlong *)(local_90 + 8),iVar3 < iVar8);
    }
    *(undefined1 *)(param_1 + 0x62) = 1;
    (**(code **)(*param_1 + 0x90))(param_1,0);
    lVar4 = local_90;
    if (local_90 != 0) {
      if (0xffffe < *(longlong *)(local_90 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar2 = (longlong *)(lVar4 + 0x20);
      lVar4 = *plVar2;
      *plVar2 = *plVar2 + -1;
      UNLOCK();
      if ((int)lVar4 == 1) {
        puVar7 = (undefined8 *)(local_90 + 0x18);
        if (local_90 == 0) {
          puVar7 = (undefined8 *)0x0;
        }
        if (puVar7 != (undefined8 *)0x0) {
          (**(code **)*puVar7)(puVar7,1);
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_1403d4760 @ 1403d4760   (386 bytes)
//===========================================================

undefined8 * FUN_1403d4760(undefined8 param_1,undefined8 *param_2)

{
  undefined *puVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  undefined8 *local_res20;
  IUnknown *local_58;
  longlong *local_50;
  undefined8 **local_48;
  uint local_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined8 local_30;
  uint local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 local_18;
  
  FUN_1403a7c40(param_1,&local_58);
  puVar1 = PTR_u_icon_143a45db8;
  if (local_58 == (IUnknown *)0x0) {
    *param_2 = 0;
  }
  else {
    local_48 = (undefined8 **)FUN_14019b780(&DAT_143ad68a0,0x18);
    puVar4 = (undefined8 *)0x0;
    local_res20 = puVar4;
    if (local_48 != (undefined8 **)0x0) {
      local_res20 = (undefined8 *)FUN_14023b360(local_48,puVar1);
    }
    if (local_res20 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    local_48 = &local_res20;
    (*DAT_143262a20)(&local_28);
    if (local_res20 != (undefined8 *)0x0) {
      puVar4 = (undefined8 *)*local_res20;
    }
    iVar2 = (**(code **)(*(longlong *)local_58 + 0x28))(local_58,puVar4,&local_28);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,local_58,(_GUID *)&DAT_143272478);
    }
    local_40 = local_28;
    uStack_3c = uStack_24;
    uStack_38 = uStack_20;
    uStack_34 = uStack_1c;
    local_30 = local_18;
    local_28 = local_28 & 0xffff0000;
    FUN_1401be120(&local_res20);
    uVar3 = FUN_1409339d0(&local_50,&local_40);
    FUN_1403ee040(param_2,uVar3);
    if (local_50 != (longlong *)0x0) {
      (**(code **)(*local_50 + 0x10))();
    }
    if ((short)local_40 == 8) {
      local_40 = local_40 & 0xffff0000;
      if (CONCAT44(uStack_34,uStack_38) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_34,uStack_38) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_40);
    }
    (**(code **)(*(longlong *)local_58 + 0x10))(local_58);
  }
  return param_2;
}


