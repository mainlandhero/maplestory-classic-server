
//===========================================================
// FUN_14039f600 @ 14039f600   (463 bytes)
//===========================================================

longlong * FUN_14039f600(undefined8 param_1,longlong *param_2,ulonglong param_3,longlong param_4)

{
  int iVar1;
  longlong lVar2;
  undefined *puVar3;
  longlong lVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  longlong *plVar7;
  int iVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *local_res20;
  longlong *local_30;
  longlong *local_28 [2];
  
  uVar9 = param_3 & 0xffffffff;
  iVar8 = (int)param_3;
  if (param_4 != 0) {
    local_res20 = (longlong *)CONCAT44(local_res20._4_4_,iVar8);
    uVar10 = ((((param_3 & 0xff ^ 0xcbf29ce484222325) * 0x100000001b3 ^ param_3 >> 8 & 0xff) *
               0x100000001b3 ^ uVar9 >> 0x10 & 0xff) * 0x100000001b3 ^ uVar9 >> 0x18) *
             0x100000001b3;
    plVar6 = (longlong *)
             ((*(ulonglong *)(param_4 + 0x30) & uVar10) * 0x10 + *(longlong *)(param_4 + 0x18));
    lVar4 = plVar6[1];
    lVar2 = *(longlong *)(param_4 + 8);
    if (lVar4 != lVar2) {
      iVar1 = *(int *)(lVar4 + 0x10);
      while (iVar8 != iVar1) {
        if (lVar4 == *plVar6) goto LAB_14039f6ec;
        lVar4 = *(longlong *)(lVar4 + 8);
        iVar1 = *(int *)(lVar4 + 0x10);
      }
      if (lVar4 == 0) {
        lVar4 = lVar2;
      }
      if (lVar4 != lVar2) {
        plVar6 = (longlong *)FUN_140415e70(param_4,&local_res20,param_3,uVar10,0);
        plVar6 = (longlong *)*plVar6;
        if (plVar6 != (longlong *)0x0) {
          (**(code **)(*plVar6 + 8))(plVar6);
          *param_2 = (longlong)plVar6;
          return param_2;
        }
      }
    }
  }
LAB_14039f6ec:
  FUN_14039e630(param_1,&local_30,uVar9);
  puVar3 = PTR_DAT_143a45980;
  if (local_30 == (longlong *)0x0) {
    *param_2 = 0;
  }
  else if (iVar8 - 9100000U < 10000) {
    *param_2 = (longlong)local_30;
  }
  else {
    local_res20 = local_30;
    (**(code **)(*local_30 + 8))(local_30);
    puVar5 = (undefined8 *)FUN_14090f200(local_28,&local_res20,puVar3);
    plVar6 = (longlong *)*puVar5;
    plVar7 = (longlong *)0x0;
    if (plVar6 != (longlong *)0x0) {
      *puVar5 = 0;
      plVar7 = plVar6;
    }
    if (local_28[0] != (longlong *)0x0) {
      (**(code **)(*local_28[0] + 0x10))();
    }
    if ((param_4 != 0) && (plVar7 != (longlong *)0x0)) {
      local_res20 = plVar7;
      (**(code **)(*plVar7 + 8))(plVar7);
      FUN_140382a00(param_4,uVar9,&local_res20);
    }
    *param_2 = (longlong)plVar7;
    (**(code **)(*local_30 + 0x10))(local_30);
  }
  return param_2;
}



//===========================================================
// FUN_14039e630 @ 14039e630   (4015 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014039ee7c) */
/* WARNING: Removing unreachable block (ram,0x00014039e981) */
/* WARNING: Removing unreachable block (ram,0x00014039f2c4) */

undefined8 * FUN_14039e630(undefined8 param_1,undefined8 *param_2,int param_3)

{
  longlong lVar1;
  longlong *plVar2;
  IUnknown *pIVar3;
  IUnknown *pIVar4;
  int iVar5;
  int iVar6;
  longlong *******ppppppplVar7;
  undefined8 uVar8;
  longlong *******ppppppplVar9;
  int *piVar10;
  longlong *******ppppppplVar11;
  undefined8 *puVar12;
  longlong *******ppppppplVar13;
  longlong ******pppppplVar14;
  uint uVar15;
  longlong lVar16;
  longlong ******local_res20;
  short local_138;
  undefined2 uStack_136;
  undefined4 uStack_134;
  undefined8 *puStack_130;
  undefined8 local_128;
  longlong ******local_120;
  short local_118;
  ushort uStack_116;
  undefined4 uStack_114;
  undefined8 uStack_110;
  undefined8 local_108;
  short local_f8;
  undefined2 uStack_f6;
  undefined4 uStack_f4;
  undefined8 *puStack_f0;
  undefined8 local_e8;
  IUnknown *local_d8;
  uint local_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined8 local_c0;
  longlong ******local_b8 [2];
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  uint local_88;
  longlong ******local_80;
  longlong ******local_78 [2];
  undefined8 local_68;
  undefined8 *puStack_60;
  undefined8 local_58;
  undefined4 local_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 local_38;
  
  ppppppplVar9 = (longlong *******)0x0;
  uVar15 = 0;
  local_88 = 0;
  if ((param_3 - 1000000U < 1000000) || (iVar6 = param_3 / 10000, iVar6 < 4)) {
    lVar16 = FUN_140cccda0(DAT_143ac0188,&local_f8);
    puVar12 = puStack_f0;
    lVar16 = *(longlong *)(lVar16 + 8);
    if (puStack_f0 != (undefined8 *)0x0) {
      if (0xffffe < puStack_f0[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar2 = puVar12 + 1;
      lVar1 = *plVar2;
      *plVar2 = *plVar2 + -1;
      UNLOCK();
      if (((int)lVar1 == 1) && (puStack_f0 != (undefined8 *)0x0)) {
        (**(code **)*puStack_f0)(puStack_f0,1);
        puStack_f0 = (undefined8 *)0x0;
      }
      else {
        puStack_f0 = (undefined8 *)0x0;
      }
    }
    uVar15 = 1;
    if (lVar16 == 0) {
      ppppppplVar13 = &local_120;
      uVar15 = 2;
    }
    else {
      ppppppplVar9 = *(longlong ********)(lVar16 + 0x20);
      local_res20 = (longlong ******)ppppppplVar9;
      if (ppppppplVar9 != (longlong *******)0x0) {
        (*(code *)(*ppppppplVar9)[1])(ppppppplVar9);
      }
      ppppppplVar13 = &local_res20;
    }
    *param_2 = ppppppplVar9;
    *ppppppplVar13 = (longlong ******)0x0;
    if (((uVar15 & 2) != 0) &&
       (uVar15 = uVar15 & 0xfffffffd | 4, local_88 = uVar15,
       (longlong *******)local_120 != (longlong *******)0x0)) {
      (*(code *)(*local_120)[2])();
    }
    if ((uVar15 & 1) == 0) {
      return param_2;
    }
    if ((longlong *******)local_res20 == (longlong *******)0x0) {
      return param_2;
    }
    (*(code *)(*local_res20)[2])();
    return param_2;
  }
  local_80 = (longlong ******)0x0;
  ppppppplVar13 = ppppppplVar9;
  if (param_3 - 5000000U < 1000000) {
    local_res20 = (longlong ******)0x0;
    iVar5 = FUN_1401b1040(param_3);
    if (iVar5 == 3) {
      FUN_1401c21c0(&local_res20,PTR_u_Item_Pet__07d_img_143a46ff8,param_3);
      pIVar3 = DAT_143add058;
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_138);
      if (DAT_143a8b8d8 == 8) {
        if (local_138 == 8) {
          local_138 = 0;
          if (puStack_130 != (undefined8 *)0x0) {
            (*DAT_143ad5990)((longlong)puStack_130 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_138);
          if (iVar6 < 0) goto LAB_14039f581;
        }
        local_138 = 8;
        ppppppplVar11 = ppppppplVar9;
        if (DAT_143a8b8e0 != 0) {
          ppppppplVar11 = (longlong *******)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        puStack_130 = (undefined8 *)FUN_1401a5fa0(DAT_143a8b8e0,ppppppplVar11);
      }
      else {
        if ((local_138 == 8) && (local_138 = 0, puStack_130 != (undefined8 *)0x0)) {
          (*DAT_143ad5990)((longlong)puStack_130 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14039f581:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      (*DAT_143262a20)(&local_f8);
      if (DAT_143a8b8d8 == 8) {
        if (local_f8 == 8) {
          local_f8 = 0;
          if (puStack_f0 != (undefined8 *)0x0) {
            (*DAT_143ad5990)((longlong)puStack_f0 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_f8);
          if (iVar6 < 0) goto LAB_14039f589;
        }
        local_f8 = 8;
        ppppppplVar11 = ppppppplVar9;
        if (DAT_143a8b8e0 != 0) {
          ppppppplVar11 = (longlong *******)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        puStack_f0 = (undefined8 *)FUN_1401a5fa0(DAT_143a8b8e0,ppppppplVar11);
      }
      else {
        if ((local_f8 == 8) && (local_f8 = 0, puStack_f0 != (undefined8 *)0x0)) {
          (*DAT_143ad5990)((longlong)puStack_f0 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_f8,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14039f589:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      ppppppplVar11 = (longlong *******)local_res20;
      ppppppplVar7 = (longlong *******)FUN_1401a5890(&local_d8,local_res20);
      local_120 = (longlong ******)ppppppplVar7;
      (*DAT_143262a20)(&local_d0);
      if (*ppppppplVar7 != (longlong ******)0x0) {
        ppppppplVar9 = (longlong *******)**ppppppplVar7;
      }
      local_a8 = CONCAT44(uStack_134,CONCAT22(uStack_136,local_138));
      uStack_a0 = puStack_130;
      local_98 = local_128;
      local_68 = CONCAT44(uStack_f4,CONCAT22(uStack_f6,local_f8));
      puStack_60 = puStack_f0;
      local_58 = local_e8;
      iVar6 = (**(code **)(*(longlong *)pIVar3 + 0x48))
                        (pIVar3,ppppppplVar9,&local_68,&local_a8,&local_d0);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar3,(_GUID *)&DAT_1432743e8);
      }
      local_118 = (short)local_d0;
      uStack_116 = (ushort)(local_d0 >> 0x10);
      uStack_114 = uStack_cc;
      uStack_110 = (undefined8 *)CONCAT44(uStack_c4,uStack_c8);
      local_108 = local_c0;
      local_d0 = (uint)uStack_116 << 0x10;
      uVar15 = 0x20;
      FUN_1401be120(ppppppplVar7);
      uVar8 = FUN_1409339d0(local_b8,&local_118);
      FUN_1401a5040(local_78,uVar8);
      if ((longlong *******)local_78[0] != (longlong *******)0x0) {
        local_80 = local_78[0];
        ppppppplVar13 = (longlong *******)local_78[0];
      }
      if ((longlong *******)local_b8[0] != (longlong *******)0x0) {
        (*(code *)(*local_b8[0])[2])();
      }
      if (local_118 == 8) {
        local_118 = 0;
        if (uStack_110 != (undefined8 *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_110 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_118);
      }
      if (local_f8 == 8) {
        local_f8 = 0;
        if (puStack_f0 != (undefined8 *)0x0) {
          (*DAT_143ad5990)((longlong)puStack_f0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_f8);
      }
      if (local_138 == 8) {
        local_138 = 0;
        if (puStack_130 != (undefined8 *)0x0) {
          (*DAT_143ad5990)((longlong)puStack_130 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_138);
      }
    }
    else {
      FUN_1401c21c0(&local_res20,PTR_u_Item_Cash__04d_img_143a47000,iVar6);
      pIVar3 = DAT_143add058;
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_118);
      if (DAT_143a8b8d8 == 8) {
        if (local_118 == 8) {
          local_118 = 0;
          if (uStack_110 != (undefined8 *)0x0) {
            (*DAT_143ad5990)((longlong)uStack_110 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_118);
          if (iVar6 < 0) goto LAB_14039f59b;
        }
        local_118 = 8;
        ppppppplVar11 = ppppppplVar9;
        if (DAT_143a8b8e0 != 0) {
          ppppppplVar11 = (longlong *******)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_110 = (undefined8 *)FUN_1401a5fa0(DAT_143a8b8e0,ppppppplVar11);
      }
      else {
        if ((local_118 == 8) && (local_118 = 0, uStack_110 != (undefined8 *)0x0)) {
          (*DAT_143ad5990)((longlong)uStack_110 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_118,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14039f59b:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      (*DAT_143262a20)(&local_138);
      if (DAT_143a8b8d8 == 8) {
        if (local_138 == 8) {
          local_138 = 0;
          if (puStack_130 != (undefined8 *)0x0) {
            (*DAT_143ad5990)((longlong)puStack_130 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_138);
          if (iVar6 < 0) goto LAB_14039f5a3;
        }
        local_138 = 8;
        ppppppplVar11 = ppppppplVar9;
        if (DAT_143a8b8e0 != 0) {
          ppppppplVar11 = (longlong *******)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        puStack_130 = (undefined8 *)FUN_1401a5fa0(DAT_143a8b8e0,ppppppplVar11);
      }
      else {
        if ((local_138 == 8) && (local_138 = 0, puStack_130 != (undefined8 *)0x0)) {
          (*DAT_143ad5990)((longlong)puStack_130 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14039f5a3:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      ppppppplVar7 = (longlong *******)FUN_1401a5890(local_b8,local_res20);
      local_120 = (longlong ******)ppppppplVar7;
      (*DAT_143262a20)(&local_d0);
      ppppppplVar11 = ppppppplVar9;
      if (*ppppppplVar7 != (longlong ******)0x0) {
        ppppppplVar11 = (longlong *******)**ppppppplVar7;
      }
      local_68 = CONCAT44(uStack_114,CONCAT22(uStack_116,local_118));
      puStack_60 = uStack_110;
      local_58 = local_108;
      local_f8 = local_138;
      uStack_f4 = uStack_134;
      puStack_f0 = puStack_130;
      local_e8 = local_128;
      iVar6 = (**(code **)(*(longlong *)pIVar3 + 0x48))
                        (pIVar3,ppppppplVar11,&local_f8,&local_68,&local_d0);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar3,(_GUID *)&DAT_1432743e8);
      }
      local_a8 = CONCAT44(uStack_cc,local_d0);
      uStack_a0 = (undefined8 *)CONCAT44(uStack_c4,uStack_c8);
      local_98 = local_c0;
      local_d0 = local_d0 & 0xffff0000;
      FUN_1401be120(ppppppplVar7);
      uVar8 = FUN_1409339d0(local_78,&local_a8);
      FUN_1401a5040(&local_d8,uVar8);
      if ((longlong *******)local_78[0] != (longlong *******)0x0) {
        (*(code *)(*local_78[0])[2])();
      }
      if ((short)local_a8 == 8) {
        local_a8 = local_a8 & 0xffffffffffff0000;
        if (uStack_a0 != (undefined8 *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_a0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_a8);
      }
      if (local_138 == 8) {
        local_138 = 0;
        if (puStack_130 != (undefined8 *)0x0) {
          (*DAT_143ad5990)((longlong)puStack_130 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_138);
      }
      if (local_118 == 8) {
        local_118 = 0;
        if (uStack_110 != (undefined8 *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_110 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_118);
      }
      FUN_1401c21c0(&local_res20,PTR_u__08d_143a482b0,param_3);
      pIVar3 = local_d8;
      ppppppplVar11 = (longlong *******)local_res20;
      if (local_d8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      ppppppplVar7 = (longlong *******)FUN_1401a5890(&local_f8,local_res20);
      local_b8[0] = (longlong ******)ppppppplVar7;
      (*DAT_143262a20)(&local_a8);
      if (*ppppppplVar7 != (longlong ******)0x0) {
        ppppppplVar9 = (longlong *******)**ppppppplVar7;
      }
      iVar6 = (**(code **)(*(longlong *)pIVar3 + 0x28))(pIVar3,ppppppplVar9,&local_a8);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar3,(_GUID *)&DAT_143272478);
      }
      local_d0 = (uint)local_a8;
      uStack_cc = local_a8._4_4_;
      uStack_c8 = (undefined4)uStack_a0;
      uStack_c4 = uStack_a0._4_4_;
      local_c0 = local_98;
      local_a8 = local_a8 & 0xffffffffffff0000;
      uVar15 = 0xc0;
      FUN_1401be120(ppppppplVar7);
      uVar8 = FUN_1409339d0(&local_120,&local_d0);
      FUN_1401a5040(local_b8,uVar8);
      if ((longlong *******)local_b8[0] != (longlong *******)0x0) {
        local_80 = local_b8[0];
        ppppppplVar13 = (longlong *******)local_b8[0];
      }
      if ((longlong *******)local_120 != (longlong *******)0x0) {
        (*(code *)(*local_120)[2])();
      }
      if ((short)local_d0 == 8) {
        local_d0 = local_d0 & 0xffff0000;
        if (CONCAT44(uStack_c4,uStack_c8) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_c4,uStack_c8) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_d0);
      }
      if (local_d8 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_d8 + 0x10))();
      }
    }
    if (ppppppplVar11 != (longlong *******)0x0) {
      FUN_1401bebb0(ppppppplVar11 + -2);
    }
LAB_14039f49f:
    ppppppplVar9 = ppppppplVar13;
    if (ppppppplVar13 != (longlong *******)0x0) {
      local_res20 = (longlong ******)ppppppplVar13;
      (*(code *)(*ppppppplVar13)[1])(ppppppplVar13);
      ppppppplVar11 = &local_res20;
      uVar15 = uVar15 | 8;
      goto LAB_14039f397;
    }
  }
  else {
    if (((param_3 - 2000000U < 1000000) || (param_3 - 3000000U < 1000000)) ||
       (param_3 - 4000000U < 1000000)) {
      puVar12 = (undefined8 *)FUN_1409180f0(&local_res20,param_3);
      ppppppplVar9 = (longlong *******)*puVar12;
      if (ppppppplVar9 != (longlong *******)0x0) {
        *puVar12 = 0;
        ppppppplVar13 = ppppppplVar9;
        local_80 = (longlong ******)ppppppplVar9;
      }
      if ((longlong *******)local_res20 != (longlong *******)0x0) {
        (*(code *)(*local_res20)[2])();
        uVar15 = 0;
      }
      goto LAB_14039f49f;
    }
    local_d8 = (IUnknown *)0x0;
    if (iVar6 == 0x38e) {
      FUN_1401c21c0(&local_d8,PTR_u_Item_Special_0910_img__d_143a46130);
      pIVar3 = DAT_143add058;
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_138);
      if (DAT_143a8b8d8 == 8) {
        if (local_138 == 8) {
          local_138 = 0;
          if (puStack_130 != (undefined8 *)0x0) {
            (*DAT_143ad5990)((longlong)puStack_130 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_138);
          if (iVar6 < 0) goto LAB_14039f5c1;
        }
        local_138 = 8;
        ppppppplVar11 = ppppppplVar9;
        if (DAT_143a8b8e0 != 0) {
          ppppppplVar11 = (longlong *******)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        puStack_130 = (undefined8 *)FUN_1401a5fa0(DAT_143a8b8e0,ppppppplVar11);
      }
      else {
        if ((local_138 == 8) && (local_138 = 0, puStack_130 != (undefined8 *)0x0)) {
          (*DAT_143ad5990)((longlong)puStack_130 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14039f5c1:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      (*DAT_143262a20)(&local_118);
      if (DAT_143a8b8d8 == 8) {
        if (local_118 == 8) {
          local_118 = 0;
          if (uStack_110 != (undefined8 *)0x0) {
            (*DAT_143ad5990)((longlong)uStack_110 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_118);
          if (iVar6 < 0) goto LAB_14039f5c9;
        }
        local_118 = 8;
        if (DAT_143a8b8e0 != 0) {
          ppppppplVar9 = (longlong *******)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_110 = (undefined8 *)FUN_1401a5fa0(DAT_143a8b8e0,ppppppplVar9);
      }
      else {
        if ((local_118 == 8) && (local_118 = 0, uStack_110 != (undefined8 *)0x0)) {
          (*DAT_143ad5990)((longlong)uStack_110 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_118,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_14039f5c9:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      ppppppplVar9 = (longlong *******)FUN_14019b780(&DAT_143ad68a0,0x18);
      pIVar4 = local_d8;
      local_120 = (longlong ******)ppppppplVar9;
      if (ppppppplVar9 == (longlong *******)0x0) {
        local_res20 = (longlong ******)(longlong *******)0x0;
      }
      else {
        ppppppplVar9[1] = (longlong ******)0x0;
        *(undefined4 *)(ppppppplVar9 + 2) = 1;
        local_res20 = (longlong ******)ppppppplVar9;
        if (local_d8 == (IUnknown *)0x0) {
          *ppppppplVar9 = (longlong ******)0x0;
        }
        else {
          lVar16 = -1;
          do {
            lVar16 = lVar16 + 1;
          } while (*(short *)(local_d8 + lVar16 * 2) != 0);
          uVar15 = (int)lVar16 + 1;
          piVar10 = (int *)(*DAT_143ad5980)((ulonglong)uVar15 * 2 + 4);
          if (piVar10 == (int *)0x0) {
            *ppppppplVar9 = (longlong ******)0x0;
LAB_14039f5d1:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x8007000e);
          }
          *piVar10 = (int)lVar16 * 2;
          pppppplVar14 = (longlong ******)(piVar10 + 1);
          FUN_142ef7ba0(pppppplVar14,pIVar4,(ulonglong)uVar15 * 2);
          *ppppppplVar9 = pppppplVar14;
          if (pppppplVar14 == (longlong ******)0x0) goto LAB_14039f5d1;
        }
      }
      pppppplVar14 = (longlong ******)0x0;
      if ((longlong *******)local_res20 == (longlong *******)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      local_120 = (longlong ******)&local_res20;
      (*DAT_143262a20)(&local_a8);
      if ((longlong *******)local_res20 != (longlong *******)0x0) {
        pppppplVar14 = (longlong ******)*local_res20;
      }
      local_68 = CONCAT44(uStack_134,CONCAT22(uStack_136,local_138));
      puStack_60 = puStack_130;
      local_58 = local_128;
      local_48 = CONCAT22(uStack_116,local_118);
      uStack_44 = uStack_114;
      uStack_40 = (undefined4)uStack_110;
      uStack_3c = uStack_110._4_4_;
      local_38 = local_108;
      iVar6 = (**(code **)(*(longlong *)pIVar3 + 0x48))
                        (pIVar3,pppppplVar14,&local_48,&local_68,&local_a8);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar3,(_GUID *)&DAT_1432743e8);
      }
      local_d0 = (uint)local_a8;
      uStack_cc = local_a8._4_4_;
      uStack_c8 = (undefined4)uStack_a0;
      uStack_c4 = uStack_a0._4_4_;
      local_c0 = local_98;
      local_a8 = local_a8 & 0xffffffffffff0000;
      uVar15 = 0x100;
      FUN_1401be120(&local_res20);
      uVar8 = FUN_1409339d0(&local_f8,&local_d0);
      FUN_1401a5040(&local_120,uVar8);
      if ((longlong *******)local_120 != (longlong *******)0x0) {
        local_80 = local_120;
        ppppppplVar13 = (longlong *******)local_120;
      }
      plVar2 = (longlong *)CONCAT44(uStack_f4,CONCAT22(uStack_f6,local_f8));
      if (plVar2 != (longlong *)0x0) {
        (**(code **)(*plVar2 + 0x10))();
      }
      if ((short)local_d0 == 8) {
        local_d0 = local_d0 & 0xffff0000;
        if (CONCAT44(uStack_c4,uStack_c8) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_c4,uStack_c8) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_d0);
      }
      if (local_118 == 8) {
        local_118 = 0;
        if (uStack_110 != (undefined8 *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_110 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_118);
      }
      if (local_138 == 8) {
        local_138 = 0;
        if (puStack_130 != (undefined8 *)0x0) {
          (*DAT_143ad5990)((longlong)puStack_130 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_138);
      }
      if (pIVar4 != (IUnknown *)0x0) {
        FUN_1401bebb0(pIVar4 + -0x10);
      }
      goto LAB_14039f49f;
    }
  }
  ppppppplVar13 = (longlong *******)0x0;
  ppppppplVar11 = &local_120;
  uVar15 = uVar15 | 0x10;
LAB_14039f397:
  *param_2 = ppppppplVar13;
  *ppppppplVar11 = (longlong ******)0x0;
  if (((uVar15 & 0x10) != 0) &&
     (uVar15 = uVar15 & 0xffffffef | 4, local_88 = uVar15,
     (longlong *******)local_120 != (longlong *******)0x0)) {
    (*(code *)(*local_120)[2])();
  }
  if (((uVar15 & 8) != 0) && ((longlong *******)local_res20 != (longlong *******)0x0)) {
    (*(code *)(*local_res20)[2])();
  }
  if (ppppppplVar9 != (longlong *******)0x0) {
    (*(code *)(*ppppppplVar9)[2])(ppppppplVar9);
  }
  return param_2;
}



//===========================================================
// FUN_14090f200 @ 14090f200   (413 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_14090f200(longlong *param_1,longlong *param_2,undefined8 param_3,undefined8 param_4)

{
  longlong *plVar1;
  longlong *plVar2;
  int iVar3;
  longlong lVar4;
  longlong *local_res20;
  longlong local_70;
  short local_68 [4];
  longlong local_60;
  undefined4 local_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 local_38;
  
  if (*param_2 == 0) {
    *param_1 = 0;
    param_2 = (longlong *)*param_2;
  }
  else {
    FUN_14090e210(local_68,param_2,param_3,param_4,0);
    local_70 = 0;
    local_48 = _DAT_143a8b8d8;
    uStack_44 = uRam0000000143a8b8dc;
    uStack_40 = (undefined4)DAT_143a8b8e0;
    uStack_3c = DAT_143a8b8e0._4_4_;
    local_38 = DAT_143a8b8e8;
    FUN_140930ec0(&local_res20,local_68,&local_48);
    plVar2 = local_res20;
    *param_1 = 0;
    if (local_res20 == (longlong *)0x0) {
      plVar1 = (longlong *)*param_1;
      if (plVar1 != (longlong *)0x0) {
        *param_1 = 0;
        (**(code **)(*plVar1 + 0x10))(plVar1);
      }
      iVar3 = -0x7fffbffe;
    }
    else {
      (**(code **)(*local_res20 + 8))();
      local_70 = 0;
      iVar3 = (**(code **)*plVar2)(plVar2,&DAT_143272478,&local_70);
      lVar4 = 0;
      if (-1 < iVar3) {
        lVar4 = local_70;
      }
      if ((longlong *)*param_1 != (longlong *)0x0) {
        (**(code **)(*(longlong *)*param_1 + 0x10))();
      }
      *param_1 = lVar4;
    }
    if (plVar2 != (longlong *)0x0) {
      (**(code **)(*plVar2 + 0x10))(plVar2);
    }
    if (((iVar3 + 0x80000000U & 0x80000000) == 0) && (iVar3 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 0x10))();
    }
    if (local_68[0] == 8) {
      local_68[0] = 0;
      if (local_60 != 0) {
        (*DAT_143ad5990)(local_60 + -4);
      }
    }
    else {
      (*DAT_143262a18)(local_68);
    }
    param_2 = (longlong *)*param_2;
  }
  if (param_2 != (longlong *)0x0) {
    (**(code **)(*param_2 + 0x10))();
  }
  return param_1;
}



//===========================================================
// FUN_140382a00 @ 140382a00   (214 bytes)
//===========================================================

void FUN_140382a00(longlong param_1,uint param_2,longlong *param_3)

{
  uint uVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *plVar4;
  uint local_res10 [2];
  longlong *local_res18;
  undefined1 local_18 [16];
  
  plVar4 = (longlong *)
           ((*(ulonglong *)(param_1 + 0x30) &
            (((((ulonglong)(param_2 & 0xff) ^ 0xcbf29ce484222325) * 0x100000001b3 ^
              (ulonglong)((int)param_2 >> 8 & 0xff)) * 0x100000001b3 ^
             (ulonglong)((int)param_2 >> 0x10 & 0xff)) * 0x100000001b3 ^
            (ulonglong)((int)param_2 >> 0x18 & 0xff)) * 0x100000001b3) * 0x10 +
           *(longlong *)(param_1 + 0x18));
  lVar3 = plVar4[1];
  lVar2 = *(longlong *)(param_1 + 8);
  local_res10[0] = param_2;
  local_res18 = param_3;
  if (lVar3 != lVar2) {
    uVar1 = *(uint *)(lVar3 + 0x10);
    while (param_2 != uVar1) {
      if (lVar3 == *plVar4) goto LAB_140382aab;
      lVar3 = *(longlong *)(lVar3 + 8);
      uVar1 = *(uint *)(lVar3 + 0x10);
    }
    if (lVar3 == 0) {
      lVar3 = lVar2;
    }
    if (lVar3 != lVar2) goto LAB_140382ac1;
  }
LAB_140382aab:
  FUN_1403f1da0(param_1,local_18,local_res10,param_3);
LAB_140382ac1:
  if ((longlong *)*param_3 != (longlong *)0x0) {
    (**(code **)(*(longlong *)*param_3 + 0x10))();
  }
  return;
}



//===========================================================
// FUN_140415e70 @ 140415e70   (147 bytes)
//===========================================================

longlong FUN_140415e70(longlong param_1,byte *param_2)

{
  int iVar1;
  longlong lVar2;
  longlong *plVar3;
  
  plVar3 = (longlong *)
           ((*(ulonglong *)(param_1 + 0x30) &
            (((((ulonglong)*param_2 ^ 0xcbf29ce484222325) * 0x100000001b3 ^ (ulonglong)param_2[1]) *
              0x100000001b3 ^ (ulonglong)param_2[2]) * 0x100000001b3 ^ (ulonglong)param_2[3]) *
            0x100000001b3) * 0x10 + *(longlong *)(param_1 + 0x18));
  lVar2 = plVar3[1];
  if (lVar2 != *(longlong *)(param_1 + 8)) {
    iVar1 = *(int *)(lVar2 + 0x10);
    while( true ) {
      if (*(int *)param_2 == iVar1) {
        return lVar2 + 0x18;
      }
      if (lVar2 == *plVar3) break;
      lVar2 = *(longlong *)(lVar2 + 8);
      iVar1 = *(int *)(lVar2 + 0x10);
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ed308c("invalid unordered_map<K, T> key");
}



//===========================================================
// FUN_1401a5890 @ 1401a5890   (236 bytes)
//===========================================================

undefined8 * FUN_1401a5890(undefined8 *param_1,longlong param_2)

{
  uint uVar1;
  undefined8 *puVar2;
  int *piVar3;
  longlong lVar4;
  
  puVar2 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
  if (puVar2 == (undefined8 *)0x0) {
    puVar2 = (undefined8 *)0x0;
  }
  else {
    puVar2[1] = 0;
    *(undefined4 *)(puVar2 + 2) = 1;
    if (param_2 != 0) {
      lVar4 = -1;
      do {
        lVar4 = lVar4 + 1;
      } while (*(short *)(param_2 + lVar4 * 2) != 0);
      uVar1 = (int)lVar4 + 1;
      piVar3 = (int *)(*DAT_143ad5980)((ulonglong)uVar1 * 2 + 4);
      if (piVar3 == (int *)0x0) {
        *puVar2 = 0;
      }
      else {
        *piVar3 = (int)lVar4 * 2;
        piVar3 = piVar3 + 1;
        FUN_142ef7ba0(piVar3,param_2,(ulonglong)uVar1 * 2);
        *puVar2 = piVar3;
        if (piVar3 != (int *)0x0) goto LAB_1401a5940;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    *puVar2 = 0;
  }
LAB_1401a5940:
  *param_1 = puVar2;
  if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x8007000e);
  }
  return param_1;
}



//===========================================================
// FUN_142ef3ac0 @ 142ef3ac0   (16 bytes)
//===========================================================

void FUN_142ef3ac0(undefined8 param_1)

{
  (*(code *)PTR_FUN_1432630d8)(param_1,0);
  return;
}



//===========================================================
// FUN_14019b780 @ 14019b780   (374 bytes)
//===========================================================

void FUN_14019b780(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  int iVar6;
  undefined8 uVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar7 = 0x80;
  if (param_2 < 0x21) {
    uVar10 = (uint)(0x10 < param_2);
LAB_14019b7de:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x10;
      goto LAB_14019b825;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x20;
      goto LAB_14019b825;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b825;
    }
  }
  else {
    if (0x40 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x81) {
        uVar10 = 3;
      }
      goto LAB_14019b7de;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x40;
LAB_14019b825:
  lVar11 = (longlong)(int)uVar10;
  lVar8 = lVar11 * 0x10 + param_1;
  plVar1 = (longlong *)(lVar8 + 0x28);
  LOCK();
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar4 == 0) {
LAB_14019b889:
    *(undefined4 *)(lVar8 + 0x30) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar4 = *plVar1;
      if (lVar4 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar4 == 0) goto LAB_14019b889;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar7,iVar6);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar6;
    puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  }
  piVar2 = (int *)(param_1 + 0x14 + lVar11 * 4);
  *piVar2 = *piVar2 + 1;
  *(undefined8 *)(param_1 + 0x68 + lVar11 * 8) = *puVar5;
  *piVar9 = *piVar9 + -1;
  if (*piVar9 == 0) {
    *plVar1 = 0;
  }
  return;
}



//===========================================================
// FUN_142ef7ba0 @ 142ef7ba0   (1379 bytes)
//===========================================================

undefined8 * FUN_142ef7ba0(undefined8 *param_1,undefined8 *param_2,ulonglong param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [32];
  undefined1 auVar4 [32];
  undefined1 auVar5 [32];
  undefined1 auVar6 [32];
  undefined1 uVar7;
  undefined2 uVar8;
  undefined4 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  undefined8 *puVar23;
  undefined1 (*pauVar24) [32];
  undefined1 (*pauVar25) [32];
  undefined8 *puVar26;
  undefined1 (*pauVar27) [32];
  undefined1 (*pauVar28) [32];
  ulonglong uVar29;
  longlong lVar30;
  ulonglong uVar31;
  undefined8 uVar32;
  undefined8 uVar33;
  
  puVar23 = param_1;
  switch(param_3) {
  case 0:
    return puVar23;
  case 1:
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    return puVar23;
  case 2:
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    return puVar23;
  case 3:
    uVar7 = *(undefined1 *)((longlong)param_2 + 2);
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    *(undefined1 *)((longlong)param_1 + 2) = uVar7;
    return puVar23;
  case 4:
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    return puVar23;
  case 5:
    uVar7 = *(undefined1 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined1 *)((longlong)param_1 + 4) = uVar7;
    return puVar23;
  case 6:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    return puVar23;
  case 7:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    uVar7 = *(undefined1 *)((longlong)param_2 + 6);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    *(undefined1 *)((longlong)param_1 + 6) = uVar7;
    return puVar23;
  case 8:
    *param_1 = *param_2;
    return puVar23;
  case 9:
    uVar7 = *(undefined1 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined1 *)(param_1 + 1) = uVar7;
    return puVar23;
  case 10:
    uVar8 = *(undefined2 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    return puVar23;
  case 0xb:
    uVar8 = *(undefined2 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 10);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    *(undefined1 *)((longlong)param_1 + 10) = uVar7;
    return puVar23;
  case 0xc:
    uVar9 = *(undefined4 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    return puVar23;
  case 0xd:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined1 *)((longlong)param_1 + 0xc) = uVar7;
    return puVar23;
  case 0xe:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    return puVar23;
  case 0xf:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xe);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    *(undefined1 *)((longlong)param_1 + 0xe) = uVar7;
    return puVar23;
  }
  if (param_3 < 0x21) {
    uVar10 = param_2[1];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x10));
    uVar11 = *puVar26;
    uVar32 = puVar26[1];
    *param_1 = *param_2;
    param_1[1] = uVar10;
    param_1 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    *param_1 = uVar11;
    param_1[1] = uVar32;
    return puVar23;
  }
  if ((param_2 < param_1) && (param_1 < (undefined8 *)((longlong)param_2 + param_3))) {
    lVar30 = (longlong)param_2 - (longlong)param_1;
    puVar23 = (undefined8 *)((longlong)param_1 + lVar30 + (param_3 - 0x10));
    uVar10 = *puVar23;
    uVar11 = puVar23[1];
    puVar26 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    uVar29 = param_3 - 0x10;
    puVar23 = puVar26;
    uVar32 = uVar10;
    uVar33 = uVar11;
    if (((ulonglong)puVar26 & 0xf) != 0) {
      puVar23 = (undefined8 *)((ulonglong)puVar26 & 0xfffffffffffffff0);
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
      *puVar26 = uVar10;
      *(undefined8 *)((longlong)param_1 + (param_3 - 8)) = uVar11;
      uVar29 = (longlong)puVar23 - (longlong)param_1;
    }
    uVar31 = uVar29 >> 7;
    if (uVar31 != 0) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar26 = puVar23;
      while( true ) {
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x10);
        uVar10 = puVar1[1];
        puVar23 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x20);
        uVar11 = *puVar23;
        uVar32 = puVar23[1];
        puVar23 = puVar26 + -0x10;
        puVar26[-2] = *puVar1;
        puVar26[-1] = uVar10;
        puVar26[-4] = uVar11;
        puVar26[-3] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x30);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x40);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        uVar31 = uVar31 - 1;
        puVar26[-6] = *puVar1;
        puVar26[-5] = uVar10;
        puVar26[-8] = uVar11;
        puVar26[-7] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x50);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x60);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        puVar26[-10] = *puVar1;
        puVar26[-9] = uVar10;
        puVar26[-0xc] = uVar11;
        puVar26[-0xb] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x70);
        uVar10 = *puVar1;
        uVar11 = puVar1[1];
        uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
        uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
        if (uVar31 == 0) break;
        puVar26[-0xe] = uVar10;
        puVar26[-0xd] = uVar11;
        *puVar23 = uVar32;
        puVar26[-0xf] = uVar33;
        puVar26 = puVar23;
      }
      puVar26[-0xe] = uVar10;
      puVar26[-0xd] = uVar11;
      uVar29 = uVar29 & 0x7f;
    }
    for (uVar31 = uVar29 >> 4; uVar31 != 0; uVar31 = uVar31 - 1) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar23 = puVar23 + -2;
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
    }
    if ((uVar29 & 0xf) != 0) {
      uVar10 = param_2[1];
      *param_1 = *param_2;
      param_1[1] = uVar10;
    }
    *puVar23 = uVar32;
    puVar23[1] = uVar33;
    return param_1;
  }
  if (DAT_143a8b918 < 3) {
    if ((param_3 < 0x801) || (((byte)DAT_143ae2c20 & 2) == 0)) {
      if (0x80 < param_3) {
        lVar30 = ((ulonglong)param_1 & 0xf) - 0x10;
        param_1 = (undefined8 *)((longlong)param_1 - lVar30);
        param_2 = (undefined8 *)((longlong)param_2 - lVar30);
        param_3 = param_3 + lVar30;
        if (0x80 < param_3) {
          do {
            uVar10 = param_2[1];
            uVar11 = param_2[2];
            uVar32 = param_2[3];
            uVar33 = param_2[4];
            uVar12 = param_2[5];
            uVar13 = param_2[6];
            uVar14 = param_2[7];
            *param_1 = *param_2;
            param_1[1] = uVar10;
            param_1[2] = uVar11;
            param_1[3] = uVar32;
            param_1[4] = uVar33;
            param_1[5] = uVar12;
            param_1[6] = uVar13;
            param_1[7] = uVar14;
            uVar10 = param_2[9];
            uVar11 = param_2[10];
            uVar32 = param_2[0xb];
            uVar33 = param_2[0xc];
            uVar12 = param_2[0xd];
            uVar13 = param_2[0xe];
            uVar14 = param_2[0xf];
            param_1[8] = param_2[8];
            param_1[9] = uVar10;
            param_1[10] = uVar11;
            param_1[0xb] = uVar32;
            param_1[0xc] = uVar33;
            param_1[0xd] = uVar12;
            param_1[0xe] = uVar13;
            param_1[0xf] = uVar14;
            param_1 = param_1 + 0x10;
            param_2 = param_2 + 0x10;
            param_3 = param_3 - 0x80;
          } while (0x7f < param_3);
        }
      }
                    /* WARNING: Could not recover jumptable at 0x000142ef80b6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      puVar23 = (undefined8 *)
                (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                          *(uint *)(&DAT_143c47088 + (param_3 + 0xf >> 4) * 4)))();
      return puVar23;
    }
  }
  else if (((param_3 < 0x2001) || (0x180000 < param_3)) || (((byte)DAT_143ae2c20 & 2) == 0)) {
    uVar10 = *param_2;
    uVar11 = param_2[1];
    uVar32 = param_2[2];
    uVar33 = param_2[3];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x20));
    uVar12 = *puVar26;
    uVar13 = puVar26[1];
    uVar14 = puVar26[2];
    uVar15 = puVar26[3];
    if (0x100 < param_3) {
      lVar30 = ((ulonglong)param_1 & 0x1f) - 0x20;
      pauVar24 = (undefined1 (*) [32])((longlong)param_1 - lVar30);
      pauVar27 = (undefined1 (*) [32])((longlong)param_2 - lVar30);
      param_3 = param_3 + lVar30;
      if (0x100 < param_3) {
        if (0x180000 < param_3) {
          do {
            uVar29 = param_3;
            pauVar28 = pauVar27;
            pauVar25 = pauVar24;
            auVar3 = pauVar28[1];
            auVar4 = pauVar28[2];
            auVar5 = pauVar28[3];
            auVar6 = vmovntdq_avx(*pauVar28);
            *pauVar25 = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[1] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[2] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[3] = auVar3;
            auVar3 = pauVar28[5];
            auVar4 = pauVar28[6];
            auVar5 = pauVar28[7];
            auVar6 = vmovntdq_avx(pauVar28[4]);
            pauVar25[4] = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[5] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[6] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[7] = auVar3;
            pauVar24 = pauVar25 + 8;
            pauVar27 = pauVar28 + 8;
            param_3 = uVar29 - 0x100;
          } while (0xff < uVar29 - 0x100);
          uVar31 = uVar29 - 0xe1 & 0xffffffffffffffe0;
          switch(uVar29) {
          case 0x1e1:
          case 0x1e2:
          case 0x1e3:
          case 0x1e4:
          case 0x1e5:
          case 0x1e6:
          case 0x1e7:
          case 0x1e8:
          case 0x1e9:
          case 0x1ea:
          case 0x1eb:
          case 0x1ec:
          case 0x1ed:
          case 0x1ee:
          case 0x1ef:
          case 0x1f0:
          case 0x1f1:
          case 0x1f2:
          case 499:
          case 500:
          case 0x1f5:
          case 0x1f6:
          case 0x1f7:
          case 0x1f8:
          case 0x1f9:
          case 0x1fa:
          case 0x1fb:
          case 0x1fc:
          case 0x1fd:
          case 0x1fe:
          case 0x1ff:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(*pauVar28 + uVar31));
            *(undefined1 (*) [32])(*pauVar25 + uVar31) = auVar3;
          case 0x1c1:
          case 0x1c2:
          case 0x1c3:
          case 0x1c4:
          case 0x1c5:
          case 0x1c6:
          case 0x1c7:
          case 0x1c8:
          case 0x1c9:
          case 0x1ca:
          case 0x1cb:
          case 0x1cc:
          case 0x1cd:
          case 0x1ce:
          case 0x1cf:
          case 0x1d0:
          case 0x1d1:
          case 0x1d2:
          case 0x1d3:
          case 0x1d4:
          case 0x1d5:
          case 0x1d6:
          case 0x1d7:
          case 0x1d8:
          case 0x1d9:
          case 0x1da:
          case 0x1db:
          case 0x1dc:
          case 0x1dd:
          case 0x1de:
          case 0x1df:
          case 0x1e0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[1] + uVar31));
            *(undefined1 (*) [32])(pauVar25[1] + uVar31) = auVar3;
          case 0x1a1:
          case 0x1a2:
          case 0x1a3:
          case 0x1a4:
          case 0x1a5:
          case 0x1a6:
          case 0x1a7:
          case 0x1a8:
          case 0x1a9:
          case 0x1aa:
          case 0x1ab:
          case 0x1ac:
          case 0x1ad:
          case 0x1ae:
          case 0x1af:
          case 0x1b0:
          case 0x1b1:
          case 0x1b2:
          case 0x1b3:
          case 0x1b4:
          case 0x1b5:
          case 0x1b6:
          case 0x1b7:
          case 0x1b8:
          case 0x1b9:
          case 0x1ba:
          case 0x1bb:
          case 0x1bc:
          case 0x1bd:
          case 0x1be:
          case 0x1bf:
          case 0x1c0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[2] + uVar31));
            *(undefined1 (*) [32])(pauVar25[2] + uVar31) = auVar3;
          case 0x181:
          case 0x182:
          case 0x183:
          case 0x184:
          case 0x185:
          case 0x186:
          case 0x187:
          case 0x188:
          case 0x189:
          case 0x18a:
          case 0x18b:
          case 0x18c:
          case 0x18d:
          case 0x18e:
          case 399:
          case 400:
          case 0x191:
          case 0x192:
          case 0x193:
          case 0x194:
          case 0x195:
          case 0x196:
          case 0x197:
          case 0x198:
          case 0x199:
          case 0x19a:
          case 0x19b:
          case 0x19c:
          case 0x19d:
          case 0x19e:
          case 0x19f:
          case 0x1a0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[3] + uVar31));
            *(undefined1 (*) [32])(pauVar25[3] + uVar31) = auVar3;
          case 0x161:
          case 0x162:
          case 0x163:
          case 0x164:
          case 0x165:
          case 0x166:
          case 0x167:
          case 0x168:
          case 0x169:
          case 0x16a:
          case 0x16b:
          case 0x16c:
          case 0x16d:
          case 0x16e:
          case 0x16f:
          case 0x170:
          case 0x171:
          case 0x172:
          case 0x173:
          case 0x174:
          case 0x175:
          case 0x176:
          case 0x177:
          case 0x178:
          case 0x179:
          case 0x17a:
          case 0x17b:
          case 0x17c:
          case 0x17d:
          case 0x17e:
          case 0x17f:
          case 0x180:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[4] + uVar31));
            *(undefined1 (*) [32])(pauVar25[4] + uVar31) = auVar3;
          case 0x141:
          case 0x142:
          case 0x143:
          case 0x144:
          case 0x145:
          case 0x146:
          case 0x147:
          case 0x148:
          case 0x149:
          case 0x14a:
          case 0x14b:
          case 0x14c:
          case 0x14d:
          case 0x14e:
          case 0x14f:
          case 0x150:
          case 0x151:
          case 0x152:
          case 0x153:
          case 0x154:
          case 0x155:
          case 0x156:
          case 0x157:
          case 0x158:
          case 0x159:
          case 0x15a:
          case 0x15b:
          case 0x15c:
          case 0x15d:
          case 0x15e:
          case 0x15f:
          case 0x160:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[5] + uVar31));
            *(undefined1 (*) [32])(pauVar25[5] + uVar31) = auVar3;
          case 0x121:
          case 0x122:
          case 0x123:
          case 0x124:
          case 0x125:
          case 0x126:
          case 0x127:
          case 0x128:
          case 0x129:
          case 0x12a:
          case 299:
          case 300:
          case 0x12d:
          case 0x12e:
          case 0x12f:
          case 0x130:
          case 0x131:
          case 0x132:
          case 0x133:
          case 0x134:
          case 0x135:
          case 0x136:
          case 0x137:
          case 0x138:
          case 0x139:
          case 0x13a:
          case 0x13b:
          case 0x13c:
          case 0x13d:
          case 0x13e:
          case 0x13f:
          case 0x140:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[6] + uVar31));
            *(undefined1 (*) [32])(pauVar25[6] + uVar31) = auVar3;
          default:
            puVar26 = (undefined8 *)(pauVar25[-1] + uVar29);
            *puVar26 = uVar12;
            puVar26[1] = uVar13;
            puVar26[2] = uVar14;
            puVar26[3] = uVar15;
          case 0x100:
            *param_1 = uVar10;
            param_1[1] = uVar11;
            param_1[2] = uVar32;
            param_1[3] = uVar33;
            return puVar23;
          }
        }
        do {
          uVar10 = *(undefined8 *)(*pauVar27 + 8);
          uVar11 = *(undefined8 *)(*pauVar27 + 0x10);
          uVar32 = *(undefined8 *)(*pauVar27 + 0x18);
          uVar33 = *(undefined8 *)pauVar27[1];
          uVar12 = *(undefined8 *)(pauVar27[1] + 8);
          uVar13 = *(undefined8 *)(pauVar27[1] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[1] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[2];
          uVar16 = *(undefined8 *)(pauVar27[2] + 8);
          uVar17 = *(undefined8 *)(pauVar27[2] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[2] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[3];
          uVar20 = *(undefined8 *)(pauVar27[3] + 8);
          uVar21 = *(undefined8 *)(pauVar27[3] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[3] + 0x18);
          *(undefined8 *)*pauVar24 = *(undefined8 *)*pauVar27;
          *(undefined8 *)(*pauVar24 + 8) = uVar10;
          *(undefined8 *)(*pauVar24 + 0x10) = uVar11;
          *(undefined8 *)(*pauVar24 + 0x18) = uVar32;
          *(undefined8 *)pauVar24[1] = uVar33;
          *(undefined8 *)(pauVar24[1] + 8) = uVar12;
          *(undefined8 *)(pauVar24[1] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[1] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[2] = uVar15;
          *(undefined8 *)(pauVar24[2] + 8) = uVar16;
          *(undefined8 *)(pauVar24[2] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[2] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[3] = uVar19;
          *(undefined8 *)(pauVar24[3] + 8) = uVar20;
          *(undefined8 *)(pauVar24[3] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[3] + 0x18) = uVar22;
          uVar10 = *(undefined8 *)(pauVar27[4] + 8);
          uVar11 = *(undefined8 *)(pauVar27[4] + 0x10);
          uVar32 = *(undefined8 *)(pauVar27[4] + 0x18);
          uVar33 = *(undefined8 *)pauVar27[5];
          uVar12 = *(undefined8 *)(pauVar27[5] + 8);
          uVar13 = *(undefined8 *)(pauVar27[5] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[5] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[6];
          uVar16 = *(undefined8 *)(pauVar27[6] + 8);
          uVar17 = *(undefined8 *)(pauVar27[6] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[6] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[7];
          uVar20 = *(undefined8 *)(pauVar27[7] + 8);
          uVar21 = *(undefined8 *)(pauVar27[7] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[7] + 0x18);
          *(undefined8 *)pauVar24[4] = *(undefined8 *)pauVar27[4];
          *(undefined8 *)(pauVar24[4] + 8) = uVar10;
          *(undefined8 *)(pauVar24[4] + 0x10) = uVar11;
          *(undefined8 *)(pauVar24[4] + 0x18) = uVar32;
          *(undefined8 *)pauVar24[5] = uVar33;
          *(undefined8 *)(pauVar24[5] + 8) = uVar12;
          *(undefined8 *)(pauVar24[5] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[5] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[6] = uVar15;
          *(undefined8 *)(pauVar24[6] + 8) = uVar16;
          *(undefined8 *)(pauVar24[6] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[6] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[7] = uVar19;
          *(undefined8 *)(pauVar24[7] + 8) = uVar20;
          *(undefined8 *)(pauVar24[7] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[7] + 0x18) = uVar22;
          pauVar24 = pauVar24 + 8;
          pauVar27 = pauVar27 + 8;
          param_3 = param_3 - 0x100;
        } while (0xff < param_3);
      }
    }
                    /* WARNING: Could not recover jumptable at 0x000142ef7e12. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    puVar23 = (undefined8 *)
              (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                        *(uint *)(&DAT_143c47040 + (param_3 + 0x1f >> 5) * 4)))();
    return puVar23;
  }
  for (; param_3 != 0; param_3 = param_3 - 1) {
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    param_2 = (undefined8 *)((longlong)param_2 + 1);
    param_1 = (undefined8 *)((longlong)param_1 + 1);
  }
  return puVar23;
}



//===========================================================
// FUN_1401be120 @ 1401be120   (114 bytes)
//===========================================================

void FUN_1401be120(undefined8 *param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  
  plVar2 = (longlong *)*param_1;
  if (plVar2 != (longlong *)0x0) {
    LOCK();
    plVar1 = plVar2 + 2;
    lVar3 = *plVar1;
    *(int *)plVar1 = (int)*plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      if (*plVar2 != 0) {
        (*DAT_143ad5990)(*plVar2 + -4);
        *plVar2 = 0;
      }
      if (plVar2[1] != 0) {
        FUN_14019b4e0();
        plVar2[1] = 0;
      }
      thunk_FUN_140205820(plVar2,0x18);
    }
    *param_1 = 0;
  }
  return;
}



//===========================================================
// FUN_142e541f0 @ 142e541f0   (146 bytes)
//===========================================================

void FUN_142e541f0(undefined4 param_1,undefined8 param_2)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18 [2];
  longlong local_res20;
  
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    FUN_142e5cd30("LogCallStack4",&DAT_1434997dc,local_res18,&DAT_1434997f8,local_res8,"Info1",
                  &local_res10,&local_res20);
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1401bebb0 @ 1401bebb0   (347 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1401bebb0(int *param_1)

{
  int iVar1;
  code *UNRECOVERED_JUMPTABLE;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  int *piVar5;
  uint uVar6;
  longlong lVar7;
  bool bVar8;
  
  if (0x100000 < *param_1 + 1U) {
    FUN_142e52dd0(0x23);
  }
  LOCK();
  iVar1 = *param_1;
  *param_1 = *param_1 + -1;
  UNLOCK();
  if (1 < iVar1) {
    return;
  }
  if (*param_1 != 0) {
    FUN_142e52dd0(0x32);
  }
  pvVar2 = Self;
  UNRECOVERED_JUMPTABLE = DAT_143ad5530;
  uVar3 = *(ulonglong *)(param_1 + -2);
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x61) {
    uVar6 = (uint)(0x40 < uVar3);
  }
  else {
    if (uVar3 < 0xa1) {
      uVar6 = 2;
      goto LAB_1401bec24;
    }
    uVar6 = 0xffffffff;
    if (uVar3 < 0x121) {
      uVar6 = 3;
    }
  }
  if ((int)uVar6 < 0) {
    uVar4 = (*DAT_143ad5538)();
                    /* WARNING: Could not recover jumptable at 0x0001401bec98. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(uVar4,0,param_1 + -2);
    return;
  }
LAB_1401bec24:
  uVar3 = (ulonglong)uVar6;
  lVar7 = (ulonglong)uVar6 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad69a8 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad69a8 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_1401becc5:
    *(undefined4 *)(&DAT_143ad69b0 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad69a8 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad69a8 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad69a8 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_1401becc5;
        if (*(void **)(&DAT_143ad69a8 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad69b0 + lVar7) = *(int *)(&DAT_143ad69b0 + lVar7) + 1;
  }
  piVar5 = (int *)(&DAT_143ad69b0 + lVar7);
  *(undefined8 *)param_1 = *(undefined8 *)(&DAT_143ad69e8 + uVar3 * 8);
  *(int **)(&DAT_143ad69e8 + uVar3 * 8) = param_1;
  _DAT_143ad6a28 = *(undefined8 *)param_1;
  *(int *)(&DAT_143ad6994 + uVar3 * 4) = *(int *)(&DAT_143ad6994 + uVar3 * 4) + -1;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *(undefined8 *)(&DAT_143ad69a8 + lVar7) = 0;
  }
  return;
}



//===========================================================
// _com_issue_errorex @ 142ef3ad0   (184 bytes)
//===========================================================

/* Library Function - Single Match
    void __cdecl _com_issue_errorex(long,struct IUnknown * __ptr64,struct _GUID const & __ptr64)
   
   Libraries: Visual Studio 2017 Release, Visual Studio 2019 Release */

void __cdecl _com_issue_errorex(long param_1,IUnknown *param_2,_GUID *param_3)

{
  int iVar1;
  undefined8 local_res10;
  undefined8 local_res20;
  
  local_res10 = 0;
  if ((param_2 != (IUnknown *)0x0) &&
     (iVar1 = (*(code *)PTR_FUN_1432630d8)(param_2,&DAT_1434a1968,&local_res20), -1 < iVar1)) {
    iVar1 = (*(code *)PTR_FUN_1432630d8)(local_res20,param_3);
    (*(code *)PTR_FUN_1432630d8)();
    if ((iVar1 == 0) && (iVar1 = (*DAT_1432629c0)(0,&local_res10), iVar1 != 0)) {
      local_res10 = 0;
    }
  }
  (*(code *)PTR_FUN_1432630d8)(param_1,local_res10);
  return;
}



//===========================================================
// FUN_1401a5fa0 @ 1401a5fa0   (120 bytes)
//===========================================================

int * FUN_1401a5fa0(longlong param_1,uint param_2)

{
  int *piVar1;
  
  piVar1 = (int *)(*DAT_143ad5980)();
  if (piVar1 == (int *)0x0) {
    return (int *)0x0;
  }
  *piVar1 = param_2 * 2;
  if (param_1 != 0) {
    FUN_142ef7ba0(piVar1 + 1,param_1,(ulonglong)param_2 * 2);
  }
  *(undefined2 *)((longlong)piVar1 + (ulonglong)param_2 * 2 + 4) = 0;
  return piVar1 + 1;
}



//===========================================================
// FUN_1401b1040 @ 1401b1040   (84 bytes)
//===========================================================

undefined8 FUN_1401b1040(int param_1)

{
  switch(param_1 / 1000000) {
  case 1:
  case 6:
    return 1;
  case 2:
  case 3:
  case 4:
    goto switchD_1401b106f_caseD_2;
  case 5:
    if (param_1 - 5000000U < 10000) {
      return 3;
    }
switchD_1401b106f_caseD_2:
    return 2;
  default:
    return 0;
  }
}



//===========================================================
// FUN_140cccda0 @ 140cccda0   (4905 bytes)
//===========================================================

longlong FUN_140cccda0(longlong param_1,longlong param_2,uint param_3)

{
  longlong lVar1;
  longlong **pplVar2;
  undefined *puVar3;
  IUnknown *pIVar4;
  undefined2 uVar5;
  int iVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  longlong *plVar9;
  longlong lVar10;
  undefined8 uVar11;
  undefined8 *puVar12;
  undefined1 *puVar13;
  longlong lVar14;
  longlong *plVar15;
  longlong *plVar16;
  longlong *plVar17;
  uint uVar18;
  ulonglong uVar19;
  uint local_res18 [2];
  undefined8 *local_res20;
  IUnknown *local_328;
  short *local_320;
  short local_318;
  undefined6 uStack_316;
  longlong lStack_310;
  undefined8 local_308;
  undefined4 local_300;
  undefined4 uStack_2fc;
  undefined8 uStack_2f8;
  undefined8 local_2f0;
  longlong *local_2e8;
  short *local_2e0;
  longlong *local_2d8;
  char *local_2d0;
  uint local_2c8;
  undefined4 uStack_2c4;
  undefined4 uStack_2c0;
  undefined4 uStack_2bc;
  undefined8 local_2b8;
  longlong *local_2b0;
  longlong *local_2a8;
  longlong *local_2a0;
  longlong *local_298;
  longlong *local_290;
  longlong *local_288;
  longlong *local_280;
  IUnknown *local_278;
  IUnknown *local_270;
  IUnknown *local_268;
  longlong *local_260;
  IUnknown *local_258;
  IUnknown *local_250;
  IUnknown *local_248;
  IUnknown *local_240;
  undefined8 local_238;
  IUnknown *local_230;
  IUnknown *local_228;
  IUnknown *local_220;
  IUnknown *local_218;
  uint local_210;
  undefined4 uStack_20c;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined8 local_200;
  undefined *local_1f8;
  longlong local_1f0;
  uint local_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined8 local_1d8;
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
  uint local_1a0;
  undefined4 uStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined8 local_190;
  undefined1 local_188 [8];
  longlong *local_180;
  undefined1 local_178 [8];
  undefined1 local_170 [8];
  undefined1 local_168 [8];
  undefined1 local_160 [8];
  longlong *local_158;
  undefined1 local_150 [8];
  longlong *local_148;
  longlong *local_140;
  short local_138 [4];
  longlong local_130;
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
  uint local_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined8 local_e0;
  uint local_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 local_c8;
  uint local_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined8 local_b0;
  uint local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  undefined8 local_88;
  longlong lStack_80;
  undefined8 local_78;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  undefined1 local_48 [16];
  
  uVar19 = (ulonglong)(int)param_3;
  if ((int)param_3 < 1) {
    *(undefined8 *)(param_2 + 8) = 0;
    return param_2;
  }
  local_res18[0] = param_3;
  if (*(longlong *)(param_1 + 0x28) != 0) {
    for (lVar14 = *(longlong *)
                   (*(longlong *)(param_1 + 0x28) +
                   (uVar19 % (ulonglong)*(uint *)(param_1 + 0x30)) * 8); lVar14 != 0;
        lVar14 = *(longlong *)(lVar14 + 8)) {
      if (*(uint *)(lVar14 + 0x10) == param_3) {
        lVar1 = lVar14 + 0x18;
        if (lVar1 != 0) {
          uVar8 = (*DAT_143262db0)();
          lVar10 = *(longlong *)(lVar14 + 0x20);
          if (lVar10 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar10 = *(longlong *)(lVar14 + 0x20);
          }
          *(undefined4 *)(lVar10 + 0x58) = uVar8;
          lVar10 = *(longlong *)(lVar14 + 0x20);
          if (lVar10 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar10 = *(longlong *)(lVar14 + 0x20);
          }
          if (*(int *)(lVar10 + 0x70) != 0) {
            uVar11 = FUN_140cf1560();
            uVar5 = FUN_141779500(uVar11);
            FUN_1403e2a80(&local_320,param_3,uVar5);
            pIVar4 = DAT_143add058;
            if ((local_320 != (short *)0x0) && (*local_320 != 0)) {
              if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              (*DAT_143262a20)(&local_300);
              iVar6 = FUN_14023c4c0(&local_300,&DAT_143a8b8d8);
              if (iVar6 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar6);
              }
              (*DAT_143262a20)(&local_318);
              iVar6 = FUN_14023c4c0(&local_318,&DAT_143a8b8d8);
              if (iVar6 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar6);
              }
              uVar11 = FUN_1401a5890(&local_2b0,local_320);
              uVar11 = FUN_140403ec0(pIVar4,&local_2c8,uVar11,&local_318,&local_300);
              uVar11 = FUN_1409339d0(&local_2a8,uVar11);
              FUN_1401a5040(&local_2d8,uVar11);
              if (local_2a8 != (longlong *)0x0) {
                (**(code **)(*local_2a8 + 0x10))();
              }
              if ((short)local_2c8 == 8) {
                local_2c8 = local_2c8 & 0xffff0000;
                if (CONCAT44(uStack_2bc,uStack_2c0) != 0) {
                  (*DAT_143ad5990)(CONCAT44(uStack_2bc,uStack_2c0) + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_2c8);
              }
              if (local_318 == 8) {
                local_318 = 0;
                if (lStack_310 != 0) {
                  (*DAT_143ad5990)(lStack_310 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_318);
              }
              if ((short)local_300 == 8) {
                local_300 = local_300 & 0xffff0000;
                if (uStack_2f8 != 0) {
                  (*DAT_143ad5990)(uStack_2f8 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_300);
              }
              if (local_2d8 != (longlong *)0x0) {
                lVar14 = FUN_140ceb300(lVar1);
                FUN_140ceb060(lVar14 + 0x78,&local_2d8);
                (**(code **)(*local_2d8 + 0x10))(local_2d8);
              }
            }
            if (local_320 != (short *)0x0) {
              FUN_1401bebb0(local_320 + -8);
            }
          }
          FUN_140ce7c70(param_2,lVar1);
          return param_2;
        }
        break;
      }
    }
  }
  local_res20 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xb0);
  plVar16 = (longlong *)0x0;
  plVar9 = plVar16;
  if (local_res20 != (undefined8 *)0x0) {
    plVar9 = (longlong *)FUN_140ce7cc0(local_res20);
  }
  local_180 = plVar9;
  if (plVar9 != (longlong *)0x0) {
    if (0xfffff < (ulonglong)plVar9[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    plVar9[1] = plVar9[1] + 1;
    UNLOCK();
    uVar19 = (ulonglong)local_res18[0];
  }
  FUN_1403e18a0(&local_2e0,uVar19 & 0xffffffff);
  if ((local_2e0 == (short *)0x0) || (*local_2e0 == 0)) {
    *(undefined8 *)(param_2 + 8) = 0;
    if (local_2e0 != (short *)0x0) {
      FUN_1401bebb0(local_2e0 + -8);
    }
LAB_140cce00a:
    if (plVar9 != (longlong *)0x0) {
      if (0xffffe < plVar9[1] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar16 = plVar9 + 1;
      lVar14 = *plVar16;
      *plVar16 = *plVar16 + -1;
      UNLOCK();
      if ((int)lVar14 == 1) {
        (**(code **)*plVar9)(plVar9,1);
      }
    }
    return param_2;
  }
  FUN_140ce6090(uVar19 & 0xffffffff);
  uVar11 = FUN_14090de10(local_138,local_2e0);
  uVar11 = FUN_1409339d0(&local_140,uVar11);
  FUN_1401a5040(&local_2a0,uVar11);
  if (plVar9 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  plVar15 = (longlong *)plVar9[4];
  plVar17 = local_2a0;
  if (plVar15 != local_2a0) {
    plVar9[4] = (longlong)local_2a0;
    local_2a0 = (longlong *)0x0;
    plVar17 = plVar16;
    if (plVar15 != (longlong *)0x0) {
      (**(code **)(*plVar15 + 0x10))();
      plVar17 = (longlong *)0x0;
    }
  }
  plVar15 = (longlong *)plVar9[4];
  local_148 = plVar15;
  if (plVar15 != (longlong *)0x0) {
    (**(code **)(*plVar15 + 8))(plVar15);
  }
  if (plVar17 != (longlong *)0x0) {
    (**(code **)(*plVar17 + 0x10))(plVar17);
  }
  if (local_140 != (longlong *)0x0) {
    (**(code **)(*local_140 + 0x10))();
  }
  if (local_138[0] == 8) {
    local_138[0] = 0;
    if (local_130 != 0) {
      (*DAT_143ad5990)(local_130 + -4);
    }
  }
  else {
    (*DAT_143262a18)(local_138);
  }
  puVar3 = PTR_DAT_143a45980;
  if (plVar15 == (longlong *)0x0) {
    *(undefined8 *)(param_2 + 8) = 0;
    if (local_2e0 != (short *)0x0) {
      FUN_1401bebb0(local_2e0 + -8);
    }
    goto LAB_140cce00a;
  }
  local_298 = plVar15;
  (**(code **)(*plVar15 + 8))(plVar15);
  FUN_14090f200(&local_328,&local_298,puVar3);
  pIVar4 = local_328;
  if (local_328 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar12 = (undefined8 *)FUN_1401a5890(local_178,PTR_u_islot_143a47658);
  local_res20 = puVar12;
  (*DAT_143262a20)(&local_120);
  plVar15 = plVar16;
  if ((undefined8 *)*puVar12 != (undefined8 *)0x0) {
    plVar15 = *(longlong **)*puVar12;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar4 + 0x28))(pIVar4,plVar15,&local_120);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar4,(_GUID *)&DAT_143272478);
  }
  local_1e8 = local_120;
  uStack_1e4 = uStack_11c;
  uStack_1e0 = uStack_118;
  uStack_1dc = uStack_114;
  local_1d8 = local_110;
  local_120 = local_120 & 0xffff0000;
  FUN_1401be120(puVar12);
  uVar11 = FUN_1401a5cf0(&local_290,&local_1e8);
  if (plVar9 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_140360880(plVar9 + 5,uVar11);
  uVar18 = (uint)uVar19;
  if (local_290 != (longlong *)0x0) {
    LOCK();
    plVar15 = local_290 + 2;
    lVar14 = *plVar15;
    *(int *)plVar15 = (int)*plVar15 + -1;
    UNLOCK();
    if (((int)lVar14 == 1) && (local_290 != (longlong *)0x0)) {
      if (*local_290 != 0) {
        (*DAT_143ad5990)(*local_290 + -4);
        *local_290 = 0;
      }
      if (local_290[1] != 0) {
        FUN_14019b4e0();
        local_290[1] = 0;
      }
      thunk_FUN_140205820(local_290,0x18);
    }
    local_290 = (longlong *)0x0;
    uVar18 = local_res18[0];
  }
  if ((short)local_1e8 == 8) {
    local_1e8 = local_1e8 & 0xffff0000;
    if (CONCAT44(uStack_1dc,uStack_1e0) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1dc,uStack_1e0) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1e8);
  }
  pIVar4 = local_328;
  if (local_328 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar12 = (undefined8 *)FUN_1401a5890(local_170,PTR_u_vslot_143a47660);
  local_res20 = puVar12;
  (*DAT_143262a20)(&local_108);
  plVar15 = plVar16;
  if ((undefined8 *)*puVar12 != (undefined8 *)0x0) {
    plVar15 = *(longlong **)*puVar12;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar4 + 0x28))(pIVar4,plVar15,&local_108);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar4,(_GUID *)&DAT_143272478);
  }
  local_1d0 = local_108;
  uStack_1cc = uStack_104;
  uStack_1c8 = uStack_100;
  uStack_1c4 = uStack_fc;
  local_1c0 = local_f8;
  local_108 = local_108 & 0xffff0000;
  FUN_1401be120(puVar12);
  uVar11 = FUN_1401a5cf0(&local_288,&local_1d0);
  if (plVar9 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_140360880(plVar9 + 6,uVar11);
  if (local_288 != (longlong *)0x0) {
    LOCK();
    plVar15 = local_288 + 2;
    lVar14 = *plVar15;
    *(int *)plVar15 = (int)*plVar15 + -1;
    UNLOCK();
    if (((int)lVar14 == 1) && (local_288 != (longlong *)0x0)) {
      if (*local_288 != 0) {
        (*DAT_143ad5990)(*local_288 + -4);
        *local_288 = 0;
      }
      if (local_288[1] != 0) {
        FUN_14019b4e0();
        local_288[1] = 0;
      }
      thunk_FUN_140205820(local_288,0x18);
    }
    local_288 = (longlong *)0x0;
    uVar18 = local_res18[0];
  }
  if ((short)local_1d0 == 8) {
    local_1d0 = local_1d0 & 0xffff0000;
    if (CONCAT44(uStack_1c4,uStack_1c8) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1c4,uStack_1c8) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1d0);
  }
  pIVar4 = local_328;
  if (local_328 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar12 = (undefined8 *)FUN_1401a5890(local_168,PTR_DAT_143a47668);
  local_res20 = puVar12;
  (*DAT_143262a20)(&local_f0);
  plVar15 = plVar16;
  if ((undefined8 *)*puVar12 != (undefined8 *)0x0) {
    plVar15 = *(longlong **)*puVar12;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar4 + 0x28))(pIVar4,plVar15,&local_f0);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar4,(_GUID *)&DAT_143272478);
  }
  local_1b8 = local_f0;
  uStack_1b4 = uStack_ec;
  uStack_1b0 = uStack_e8;
  uStack_1ac = uStack_e4;
  local_1a8 = local_e0;
  local_f0 = local_f0 & 0xffff0000;
  FUN_1401be120(puVar12);
  uVar11 = FUN_1401a5cf0(&local_280,&local_1b8);
  if (plVar9 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_140360880(plVar9 + 0xd,uVar11);
  if (local_280 != (longlong *)0x0) {
    LOCK();
    plVar15 = local_280 + 2;
    lVar14 = *plVar15;
    *(int *)plVar15 = (int)*plVar15 + -1;
    UNLOCK();
    if (((int)lVar14 == 1) && (local_280 != (longlong *)0x0)) {
      if (*local_280 != 0) {
        (*DAT_143ad5990)(*local_280 + -4);
        *local_280 = 0;
      }
      if (local_280[1] != 0) {
        FUN_14019b4e0();
        local_280[1] = 0;
      }
      thunk_FUN_140205820(local_280,0x18);
    }
    local_280 = (longlong *)0x0;
    uVar18 = local_res18[0];
  }
  if ((short)local_1b8 == 8) {
    local_1b8 = local_1b8 & 0xffff0000;
    if (CONCAT44(uStack_1ac,uStack_1b0) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1ac,uStack_1b0) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1b8);
  }
  puVar3 = PTR_u_weekly_143a45bd0;
  local_278 = local_328;
  if (local_328 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_328 + 8))();
  }
  iVar6 = FUN_140910eb0(&local_278,puVar3,0);
  uVar8 = 0;
  if (plVar9 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  *(uint *)(plVar9 + 0xe) = (uint)(iVar6 != 0);
  puVar3 = PTR_u_invisibleFace_143a476e8;
  local_270 = local_328;
  if (local_328 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_328 + 8))();
  }
  iVar6 = FUN_140910eb0(&local_270,puVar3,0);
  *(uint *)(plVar9 + 0x10) = (uint)(iVar6 != 0);
  puVar3 = PTR_u_extendFrame_143a45bc0;
  local_268 = local_328;
  if (local_328 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_328 + 8))();
  }
  iVar6 = FUN_140910eb0(&local_268,puVar3,0);
  pIVar4 = local_328;
  *(uint *)((longlong)plVar9 + 0x84) = (uint)(iVar6 != 0);
  if (local_328 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar12 = (undefined8 *)FUN_1401a5890(local_160,PTR_u_vehicleDefaultFrame_143a476f0);
  local_res20 = puVar12;
  (*DAT_143262a20)(&local_d8);
  plVar15 = plVar16;
  if ((undefined8 *)*puVar12 != (undefined8 *)0x0) {
    plVar15 = *(longlong **)*puVar12;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar4 + 0x28))(pIVar4,plVar15,&local_d8);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar4,(_GUID *)&DAT_143272478);
  }
  local_1a0 = local_d8;
  uStack_19c = uStack_d4;
  uStack_198 = uStack_d0;
  uStack_194 = uStack_cc;
  local_190 = local_c8;
  local_d8 = local_d8 & 0xffff0000;
  FUN_1401be120(puVar12);
  uVar11 = FUN_1409339d0(&local_158,&local_1a0);
  FUN_1401a5040(&local_260,uVar11);
  plVar15 = (longlong *)plVar9[0x11];
  plVar17 = local_260;
  if (plVar15 != local_260) {
    plVar9[0x11] = (longlong)local_260;
    local_260 = (longlong *)0x0;
    plVar17 = plVar16;
    if (plVar15 != (longlong *)0x0) {
      (**(code **)(*plVar15 + 0x10))();
      plVar17 = (longlong *)0x0;
    }
  }
  if (plVar17 != (longlong *)0x0) {
    (**(code **)(*plVar17 + 0x10))(plVar17);
  }
  if (local_158 != (longlong *)0x0) {
    (**(code **)(*local_158 + 0x10))();
  }
  if ((short)local_1a0 == 8) {
    local_1a0 = local_1a0 & 0xffff0000;
    if (CONCAT44(uStack_194,uStack_198) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_194,uStack_198) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1a0);
  }
  local_220 = local_328;
  if (local_328 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_328 + 8))();
  }
  uVar7 = FUN_140910eb0(&local_220,L"partsQuestID",0);
  *(undefined4 *)(plVar9 + 0x12) = uVar7;
  local_228 = local_328;
  if (local_328 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_328 + 8))();
  }
  uVar7 = FUN_140910eb0(&local_228,L"partsCount",0);
  *(undefined4 *)((longlong)plVar9 + 0x94) = uVar7;
  local_218 = local_328;
  if (local_328 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_328 + 8))();
  }
  uVar7 = FUN_140910eb0(&local_218,L"removeEar",0);
  *(undefined4 *)(plVar9 + 0x13) = uVar7;
  local_238 = 0;
  local_230 = local_328;
  if (local_328 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_328 + 8))();
  }
  FUN_140911cf0(&local_2d0,&local_230,&DAT_14336b740,&local_238);
  if ((local_2d0 != (char *)0x0) && (*local_2d0 != '\0')) {
    local_1f8 = &DAT_14336b740;
    local_1f0 = 0;
    FUN_14019a260(&local_1f0,&local_2d0);
    FUN_140ce7680(plVar9 + 0x14,local_48,&local_1f8);
    if (local_1f0 != 0) {
      FUN_14019f2c0(local_1f0 + -0x10);
    }
  }
  pIVar4 = local_328;
  if ((int)uVar18 / 10000 - 0x79U < 9) {
    *(undefined4 *)(plVar9 + 7) = 1;
LAB_140ccd962:
    if (local_328 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    puVar12 = (undefined8 *)FUN_1401a5890(local_150,PTR_u_afterImage_143a47670);
    local_res20 = puVar12;
    (*DAT_143262a20)(&local_c0);
    plVar15 = plVar16;
    if ((undefined8 *)*puVar12 != (undefined8 *)0x0) {
      plVar15 = *(longlong **)*puVar12;
    }
    iVar6 = (**(code **)(*(longlong *)pIVar4 + 0x28))(pIVar4,plVar15,&local_c0);
    if (iVar6 < 0) {
      _com_issue_errorex(iVar6,pIVar4,(_GUID *)&DAT_143272478);
    }
    local_210 = local_c0;
    uStack_20c = uStack_bc;
    uStack_208 = uStack_b8;
    uStack_204 = uStack_b4;
    local_200 = local_b0;
    local_c0 = local_c0 & 0xffff0000;
    FUN_1401be120(puVar12);
    if ((short)local_210 == 8) {
      FUN_14023b420(&local_2e8,&local_210);
      pplVar2 = (longlong **)(plVar9 + 8);
      if (pplVar2 != &local_2e8) {
        FUN_1401be120(pplVar2);
        *pplVar2 = local_2e8;
        if (local_2e8 != (longlong *)0x0) {
          LOCK();
          *(int *)(local_2e8 + 2) = (int)local_2e8[2] + 1;
          UNLOCK();
        }
      }
      if (local_2e8 != (longlong *)0x0) {
        LOCK();
        plVar15 = local_2e8 + 2;
        lVar14 = *plVar15;
        *(int *)plVar15 = (int)*plVar15 + -1;
        UNLOCK();
        if ((int)lVar14 == 1) {
          if (*local_2e8 != 0) {
            (*DAT_143ad5990)(*local_2e8 + -4);
            *local_2e8 = 0;
          }
          if (local_2e8[1] != 0) {
            FUN_14019b4e0();
            local_2e8[1] = 0;
          }
          thunk_FUN_140205820(local_2e8,0x18);
        }
      }
    }
    puVar3 = PTR_u_walk_143a47678;
    local_258 = local_328;
    if (local_328 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_328 + 8))();
    }
    uVar8 = FUN_140910eb0(&local_258,puVar3,0);
    *(undefined4 *)((longlong)plVar9 + 0x4c) = uVar8;
    puVar3 = PTR_u_stand_143a47370;
    local_250 = local_328;
    if (local_328 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_328 + 8))();
    }
    uVar8 = FUN_140910eb0(&local_250,puVar3,0);
    *(undefined4 *)(plVar9 + 10) = uVar8;
    puVar3 = PTR_u_attack_143a47680;
    local_248 = local_328;
    if (local_328 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_328 + 8))();
    }
    uVar8 = FUN_140910eb0(&local_248,puVar3,0);
    *(undefined4 *)((longlong)plVar9 + 0x54) = uVar8;
    puVar3 = PTR_u_attackSpeed_143a45bb8;
    local_240 = local_328;
    if (local_328 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_328 + 8))();
    }
    uVar8 = FUN_140910eb0(&local_240,puVar3,6);
    *(undefined4 *)(plVar9 + 9) = uVar8;
    if ((short)local_210 == 8) {
      local_210 = local_210 & 0xffff0000;
      if (CONCAT44(uStack_204,uStack_208) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_204,uStack_208) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_210);
    }
  }
  else {
    iVar6 = (int)uVar18 / 100000;
    if (iVar6 == 0xd) {
      *(undefined4 *)(plVar9 + 7) = 1;
      goto LAB_140ccd962;
    }
    if (iVar6 == 0xe) {
      *(undefined4 *)(plVar9 + 7) = 2;
      goto LAB_140ccd962;
    }
    if (iVar6 == 0xf) {
      uVar8 = 3;
    }
    *(undefined4 *)(plVar9 + 7) = uVar8;
    if (iVar6 == 0xf) goto LAB_140ccd962;
  }
  puVar13 = (undefined1 *)FUN_140ced550(param_1 + 0x10);
  if ((*(longlong *)(puVar13 + 8) - 1U < 999) || (*(longlong *)(puVar13 + 8) == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (puVar13 == local_188) {
    FUN_142e52d50(0x45c,1);
  }
  if (0xfffff < (ulonglong)plVar9[1]) {
    FUN_142e541f0(0x30f);
  }
  LOCK();
  plVar9[1] = plVar9[1] + 1;
  UNLOCK();
  FUN_140cf8af0(puVar13);
  *(longlong **)(puVar13 + 8) = plVar9;
  lVar14 = FUN_140cf1890(param_1 + 0x28,local_res18,local_188);
  plVar9[0xc] = lVar14;
  uVar8 = (*DAT_143262db0)();
  *(undefined4 *)(plVar9 + 0xb) = uVar8;
  if ((int)plVar9[0xe] != 0) {
    uVar11 = FUN_140cf1560();
    uVar5 = FUN_141779500(uVar11);
    FUN_1403e2a80(&local_320,local_res18[0],uVar5);
    pIVar4 = DAT_143add058;
    if ((local_320 != (short *)0x0) && (*local_320 != 0)) {
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_318);
      if (DAT_143a8b8d8 == 8) {
        if (local_318 == 8) {
          local_318 = 0;
          if (lStack_310 != 0) {
            (*DAT_143ad5990)(lStack_310 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_318);
          if (iVar6 < 0) goto LAB_140cce072;
        }
        local_318 = 8;
        plVar15 = plVar16;
        if (DAT_143a8b8e0 != 0) {
          plVar15 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_310 = FUN_1401a5fa0(DAT_143a8b8e0,plVar15);
      }
      else {
        if ((local_318 == 8) && (local_318 = 0, lStack_310 != 0)) {
          (*DAT_143ad5990)(lStack_310 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_318,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_140cce072:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      (*DAT_143262a20)(&local_300);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_300 == 8) {
          local_300 = (uint)local_300._2_2_ << 0x10;
          if (uStack_2f8 != 0) {
            (*DAT_143ad5990)(uStack_2f8 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_300);
          if (iVar6 < 0) goto LAB_140cce07a;
        }
        local_300 = CONCAT22(local_300._2_2_,8);
        plVar15 = plVar16;
        if (DAT_143a8b8e0 != 0) {
          plVar15 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_2f8 = FUN_1401a5fa0(DAT_143a8b8e0,plVar15);
      }
      else {
        if (((short)local_300 == 8) && (local_300 = (uint)local_300._2_2_ << 0x10, uStack_2f8 != 0))
        {
          (*DAT_143ad5990)(uStack_2f8 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_300,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_140cce07a:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      puVar12 = (undefined8 *)FUN_1401a5890(&local_2a8,local_320);
      local_res20 = puVar12;
      (*DAT_143262a20)(&local_a8);
      if ((undefined8 *)*puVar12 != (undefined8 *)0x0) {
        plVar16 = *(longlong **)*puVar12;
      }
      local_88 = CONCAT62(uStack_316,local_318);
      lStack_80 = lStack_310;
      local_78 = local_308;
      local_68 = local_300;
      uStack_64 = uStack_2fc;
      uStack_60 = (undefined4)uStack_2f8;
      uStack_5c = uStack_2f8._4_4_;
      local_58 = local_2f0;
      iVar6 = (**(code **)(*(longlong *)pIVar4 + 0x48))
                        (pIVar4,plVar16,&local_68,&local_88,&local_a8);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar4,(_GUID *)&DAT_1432743e8);
      }
      local_2c8 = local_a8;
      uStack_2c4 = uStack_a4;
      uStack_2c0 = uStack_a0;
      uStack_2bc = uStack_9c;
      local_2b8 = local_98;
      local_a8 = local_a8 & 0xffff0000;
      FUN_1401be120(puVar12);
      uVar11 = FUN_1409339d0(&local_2b0,&local_2c8);
      FUN_1401a5040(&local_2d8,uVar11);
      if (local_2b0 != (longlong *)0x0) {
        (**(code **)(*local_2b0 + 0x10))();
      }
      if ((short)local_2c8 == 8) {
        local_2c8 = local_2c8 & 0xffff0000;
        if (CONCAT44(uStack_2bc,uStack_2c0) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_2bc,uStack_2c0) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_2c8);
      }
      if ((short)local_300 == 8) {
        local_300 = local_300 & 0xffff0000;
        if (uStack_2f8 != 0) {
          (*DAT_143ad5990)(uStack_2f8 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_300);
      }
      if (local_318 == 8) {
        local_318 = 0;
        if (lStack_310 != 0) {
          (*DAT_143ad5990)(lStack_310 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_318);
      }
      if ((local_2d8 != (longlong *)0x0) && ((longlong *)plVar9[0xf] != local_2d8)) {
        (**(code **)(*local_2d8 + 8))(local_2d8);
        plVar16 = (longlong *)plVar9[0xf];
        plVar9[0xf] = (longlong)local_2d8;
        if (plVar16 != (longlong *)0x0) {
          (**(code **)(*plVar16 + 0x10))();
        }
      }
      if (local_2d8 != (longlong *)0x0) {
        (**(code **)(*local_2d8 + 0x10))(local_2d8);
      }
    }
    if (local_320 != (short *)0x0) {
      FUN_1401bebb0(local_320 + -8);
    }
  }
  if (plVar9[0xf] == 0) {
    FUN_140ceb060(plVar9 + 0xf,plVar9 + 4);
  }
  *(longlong **)(param_2 + 8) = plVar9;
  local_180 = (longlong *)0x0;
  if (local_2d0 != (char *)0x0) {
    FUN_14019f2c0(local_2d0 + -0x10);
  }
  if (local_328 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_328 + 0x10))();
  }
  (**(code **)(*local_148 + 0x10))();
  if (local_2e0 == (short *)0x0) {
    return param_2;
  }
  FUN_1401bebb0(local_2e0 + -8);
  return param_2;
}



//===========================================================
// FUN_1409180f0 @ 1409180f0   (195 bytes)
//===========================================================

longlong FUN_1409180f0(longlong param_1,undefined4 param_2)

{
  longlong lVar1;
  longlong local_res8;
  longlong *local_res18;
  longlong local_res20;
  longlong *local_28 [2];
  
  local_res8 = param_1;
  FUN_140917d80(&local_res20);
  FUN_14090ead0(local_28,local_res20);
  local_res8 = 0;
  FUN_1401c21c0(&local_res8,L"%0*d",8,param_2);
  local_res18 = local_28[0];
  if (local_28[0] != (longlong *)0x0) {
    (**(code **)(*local_28[0] + 8))(local_28[0]);
  }
  lVar1 = local_res8;
  FUN_14090f200(param_1,&local_res18,local_res8);
  if (lVar1 != 0) {
    FUN_1401bebb0(lVar1 + -0x10);
  }
  if (local_28[0] != (longlong *)0x0) {
    (**(code **)(*local_28[0] + 0x10))(local_28[0]);
  }
  if (local_res20 != 0) {
    FUN_1401bebb0(local_res20 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_1409339d0 @ 1409339d0   (68 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_1409339d0(undefined8 param_1)

{
  undefined4 local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 local_18;
  
  local_28 = _DAT_143a8b8d8;
  uStack_24 = uRam0000000143a8b8dc;
  uStack_20 = (undefined4)DAT_143a8b8e0;
  uStack_1c = DAT_143a8b8e0._4_4_;
  local_18 = DAT_143a8b8e8;
  FUN_140930ec0(_DAT_143a8b8d8,DAT_143a8b8e8,&local_28);
  return param_1;
}



//===========================================================
// FUN_1401a5040 @ 1401a5040   (195 bytes)
//===========================================================

longlong * FUN_1401a5040(longlong *param_1,longlong *param_2)

{
  longlong *plVar1;
  int iVar2;
  longlong lVar3;
  longlong local_res8;
  
  *param_1 = 0;
  param_2 = (longlong *)*param_2;
  if (param_2 == (longlong *)0x0) {
    plVar1 = (longlong *)*param_1;
    if (plVar1 != (longlong *)0x0) {
      *param_1 = 0;
      (**(code **)(*plVar1 + 0x10))();
    }
    iVar2 = -0x7fffbffe;
  }
  else {
    (**(code **)(*param_2 + 8))(param_2);
    local_res8 = 0;
    iVar2 = (**(code **)*param_2)(param_2,&DAT_143272478,&local_res8);
    lVar3 = 0;
    if (-1 < iVar2) {
      lVar3 = local_res8;
    }
    if ((longlong *)*param_1 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_1 + 0x10))();
    }
    *param_1 = lVar3;
  }
  if (param_2 != (longlong *)0x0) {
    (**(code **)(*param_2 + 0x10))(param_2);
  }
  if (((iVar2 + 0x80000000U & 0x80000000) == 0) && (iVar2 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar2);
  }
  return param_1;
}



//===========================================================
// FUN_1401c21c0 @ 1401c21c0   (42 bytes)
//===========================================================

undefined8
FUN_1401c21c0(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 local_res18;
  undefined8 local_res20;
  
  local_res18 = param_3;
  local_res20 = param_4;
  FUN_1401c2300(param_1,param_2,&local_res18);
  return param_1;
}



//===========================================================
// FUN_140930ec0 @ 140930ec0   (10963 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

IUnknown * FUN_140930ec0(IUnknown *param_1,short *param_2,undefined8 param_3)

{
  int iVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  IUnknown *pIVar5;
  short *psVar6;
  undefined8 uVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  IUnknown *pIVar10;
  short *psVar11;
  longlong *plVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  IUnknown *pIVar15;
  IUnknown **ppIVar16;
  IUnknown *pIVar17;
  IUnknown *pIVar18;
  undefined8 ****ppppuVar19;
  undefined2 *puVar20;
  IUnknown *pIVar21;
  longlong *plVar22;
  longlong *plVar23;
  longlong lVar24;
  uint uVar25;
  undefined2 *puVar26;
  undefined1 auStack_8d8 [32];
  uint *local_8b8;
  uint local_8a8;
  IUnknown **local_8a0;
  IUnknown *local_898;
  IUnknown *local_890;
  IUnknown *local_888;
  IUnknown *local_880;
  longlong local_878;
  longlong local_870;
  longlong local_868;
  IUnknown *local_860;
  IUnknown *local_858;
  longlong local_850;
  IUnknown *local_848;
  short local_840;
  undefined6 uStack_83e;
  longlong lStack_838;
  undefined8 local_830;
  undefined4 local_828;
  undefined4 uStack_824;
  undefined8 uStack_820;
  undefined8 local_818;
  short local_810;
  undefined6 uStack_80e;
  longlong lStack_808;
  undefined8 local_800;
  undefined4 local_7f8;
  undefined4 uStack_7f4;
  undefined8 uStack_7f0;
  undefined8 local_7e8;
  IUnknown *local_7e0;
  longlong *local_7d8;
  IUnknown *local_7d0;
  IUnknown *local_7c8;
  IUnknown *local_7c0;
  short *local_7b8;
  short *local_7b0;
  longlong *local_7a8;
  short *local_7a0;
  longlong local_798;
  longlong local_790;
  longlong local_788;
  IUnknown *local_780;
  longlong *local_770;
  IUnknown *local_768;
  IUnknown *local_760;
  IUnknown *local_758;
  uint local_750;
  undefined4 uStack_74c;
  undefined4 uStack_748;
  undefined4 uStack_744;
  undefined8 local_740;
  IUnknown *local_738;
  IUnknown *local_730;
  IUnknown *local_728;
  longlong *local_720;
  longlong *local_718;
  undefined1 local_710 [8];
  longlong *local_708;
  longlong *local_700;
  longlong *local_6f8;
  undefined1 local_6f0 [8];
  undefined1 local_6e8 [8];
  longlong local_6e0;
  longlong *local_6d8;
  undefined1 local_6d0 [8];
  undefined1 local_6c8 [8];
  longlong local_6c0;
  undefined1 local_6b8 [8];
  undefined1 local_6b0 [8];
  longlong *local_6a8;
  longlong *local_6a0;
  undefined1 local_698 [8];
  longlong *local_690;
  undefined1 local_688 [8];
  undefined1 local_680 [8];
  longlong local_678;
  undefined1 local_670 [8];
  undefined1 local_668 [8];
  longlong *local_660;
  longlong *local_658;
  IUnknown *local_650;
  undefined1 local_648 [8];
  undefined1 local_640 [8];
  longlong local_638;
  IUnknown *local_630;
  IUnknown *local_628;
  longlong *local_620;
  IUnknown *local_618;
  IUnknown *local_610;
  uint local_608;
  undefined4 uStack_604;
  undefined4 uStack_600;
  undefined4 uStack_5fc;
  undefined8 local_5f8;
  uint local_5f0;
  undefined4 uStack_5ec;
  undefined4 uStack_5e8;
  undefined4 uStack_5e4;
  undefined8 local_5e0;
  short local_5d8 [4];
  longlong local_5d0;
  short local_5c0 [4];
  longlong local_5b8;
  short local_5a8 [4];
  longlong local_5a0;
  short local_590 [4];
  longlong local_588;
  short local_578 [4];
  longlong local_570;
  short local_560 [4];
  longlong local_558;
  short local_548 [4];
  longlong local_540;
  short local_530 [4];
  longlong local_528;
  short local_518 [4];
  longlong local_510;
  short local_500 [4];
  longlong local_4f8;
  longlong *local_4e8;
  longlong *local_4e0;
  longlong *local_4d8;
  longlong *local_4d0;
  longlong *local_4c8;
  longlong *local_4c0;
  char *local_4b8;
  undefined1 local_4b0 [8];
  longlong *local_4a8;
  longlong *local_4a0;
  longlong *local_498;
  char *local_490;
  undefined1 local_488 [8];
  char *local_480;
  longlong *local_478;
  longlong *local_470;
  IUnknown *local_468;
  IUnknown *local_460;
  longlong *local_458;
  longlong *local_450;
  longlong *local_448;
  uint local_440;
  undefined4 uStack_43c;
  undefined4 uStack_438;
  undefined4 uStack_434;
  undefined8 local_430;
  short local_428 [4];
  longlong local_420;
  short local_410 [4];
  longlong local_408;
  short local_3f8 [4];
  longlong local_3f0;
  short local_3e0 [4];
  longlong local_3d8;
  short local_3c8 [4];
  longlong local_3c0;
  uint local_3b0;
  undefined4 uStack_3ac;
  undefined4 uStack_3a8;
  undefined4 uStack_3a4;
  undefined8 local_3a0;
  uint local_398;
  undefined4 uStack_394;
  undefined4 uStack_390;
  undefined4 uStack_38c;
  undefined8 local_388;
  short local_380 [4];
  longlong local_378;
  short local_368 [4];
  longlong local_360;
  short local_350 [4];
  longlong local_348;
  short local_338 [4];
  longlong local_330;
  short local_320 [4];
  longlong local_318;
  short local_308 [4];
  longlong local_300;
  short local_2f0 [4];
  longlong local_2e8;
  short local_2d8 [4];
  longlong local_2d0;
  short local_2c0 [4];
  longlong local_2b8;
  short local_2a8 [4];
  longlong local_2a0;
  short local_290 [4];
  longlong local_288;
  short local_278 [4];
  longlong local_270;
  short local_260 [4];
  longlong local_258;
  short local_248 [4];
  longlong local_240;
  short local_230 [4];
  longlong local_228;
  short local_218 [4];
  longlong local_210;
  short local_200 [4];
  longlong local_1f8;
  short local_1e8 [4];
  longlong local_1e0;
  short local_1d0 [4];
  longlong local_1c8;
  IUnknown *local_1b8;
  longlong *local_1b0;
  longlong *local_1a8;
  longlong *local_1a0;
  undefined8 local_188;
  longlong lStack_180;
  undefined8 local_178;
  uint local_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined8 local_158;
  undefined8 local_148;
  longlong lStack_140;
  undefined8 local_138;
  uint local_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined8 local_118;
  undefined8 ***local_108 [2];
  undefined8 local_f8;
  ulonglong local_f0;
  undefined8 ***local_e8 [2];
  undefined8 local_d8;
  ulonglong local_d0;
  undefined8 ***local_c8 [2];
  undefined8 local_b8;
  ulonglong local_b0;
  undefined1 local_a8 [32];
  undefined1 local_88 [32];
  undefined1 local_68 [32];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_8d8;
  pIVar10 = (IUnknown *)0x0;
  local_8a8 = 0;
  if ((*param_2 - 9U & 0xfffb) == 0) {
    pIVar18 = *(IUnknown **)(param_2 + 4);
  }
  else {
    pIVar18 = pIVar10;
    if (((*param_2 + 0xbff7U & 0xfffb) == 0) && (*(undefined8 **)(param_2 + 4) != (undefined8 *)0x0)
       ) {
      pIVar18 = (IUnknown *)**(undefined8 **)(param_2 + 4);
    }
  }
  if (pIVar18 == (IUnknown *)0x0) {
    *(undefined8 *)param_1 = 0;
  }
  else {
    local_898 = pIVar18;
    local_858 = param_1;
    local_628 = param_1;
    (**(code **)(*(longlong *)pIVar18 + 8))(pIVar18);
    (**(code **)(*(longlong *)pIVar18 + 8))(pIVar18);
    local_848 = (IUnknown *)0x0;
    iVar2 = (*(code *)**(undefined8 **)pIVar18)(pIVar18,&DAT_143303020,&local_848);
    pIVar15 = pIVar10;
    if (-1 < iVar2) {
      pIVar15 = local_848;
    }
    local_858 = pIVar15;
    (**(code **)(*(longlong *)pIVar18 + 0x10))(pIVar18);
    if (((iVar2 + 0x80000000U & 0x80000000) == 0) && (iVar2 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
    iVar2 = 0;
    uVar25 = 0;
    if (pIVar15 != (IUnknown *)0x0) {
      local_850 = 0;
      iVar3 = (**(code **)(*(longlong *)pIVar15 + 0x40))(pIVar15,&local_850);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar15,(_GUID *)&DAT_143303020);
      }
      lVar24 = local_850;
      pIVar5 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
      pIVar21 = pIVar10;
      if (pIVar5 != (IUnknown *)0x0) {
        *(longlong *)(pIVar5 + 8) = 0;
        *(undefined4 *)(pIVar5 + 0x10) = 1;
        *(longlong *)pIVar5 = lVar24;
        pIVar21 = pIVar5;
      }
      pIVar5 = DAT_143add058;
      local_768 = pIVar21;
      if (pIVar21 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_810);
      if (DAT_143a8b8d8 == 8) {
        if (local_810 == 8) {
          local_810 = 0;
          if (lStack_808 != 0) {
            (*DAT_143ad5990)(lStack_808 + -4);
          }
        }
        else {
          iVar3 = (*DAT_143262a18)(&local_810);
          if (iVar3 < 0) goto LAB_140933831;
        }
        local_810 = 8;
        if (DAT_143a8b8e0 == 0) {
          lStack_808 = FUN_1401a5fa0(0,0);
        }
        else {
          lStack_808 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
      }
      else {
        if ((local_810 == 8) && (local_810 = 0, lStack_808 != 0)) {
          (*DAT_143ad5990)(lStack_808 + -4);
        }
        iVar3 = (*DAT_143262a28)(&local_810,&DAT_143a8b8d8);
        if (iVar3 < 0) {
LAB_140933831:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
      }
      (*DAT_143262a20)(&local_7f8);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_7f8 == 8) {
          local_7f8 = (uint)local_7f8._2_2_ << 0x10;
          if (uStack_7f0 != 0) {
            (*DAT_143ad5990)(uStack_7f0 + -4);
          }
        }
        else {
          iVar3 = (*DAT_143262a18)(&local_7f8);
          if (iVar3 < 0) goto LAB_140933803;
        }
        local_7f8 = CONCAT22(local_7f8._2_2_,8);
        pIVar17 = pIVar10;
        if (DAT_143a8b8e0 != 0) {
          pIVar17 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_7f0 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar17);
      }
      else {
        if (((short)local_7f8 == 8) && (local_7f8 = (uint)local_7f8._2_2_ << 0x10, uStack_7f0 != 0))
        {
          (*DAT_143ad5990)(uStack_7f0 + -4);
        }
        iVar3 = (*DAT_143262a28)(&local_7f8,&DAT_143a8b8d8);
        if (iVar3 < 0) {
LAB_140933803:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
      }
      LOCK();
      *(int *)(pIVar21 + 0x10) = *(int *)(pIVar21 + 0x10) + 1;
      UNLOCK();
      local_8a0 = &local_860;
      local_860 = pIVar21;
      (*DAT_143262a20)(&local_440);
      pIVar17 = pIVar10;
      if (local_860 != (IUnknown *)0x0) {
        pIVar17 = *(IUnknown **)local_860;
      }
      local_188 = CONCAT62(uStack_80e,local_810);
      lStack_180 = lStack_808;
      local_178 = local_800;
      local_168 = local_7f8;
      uStack_164 = uStack_7f4;
      uStack_160 = (undefined4)uStack_7f0;
      uStack_15c = uStack_7f0._4_4_;
      local_158 = local_7e8;
      local_8b8 = &local_440;
      iVar3 = (**(code **)(*(longlong *)pIVar5 + 0x48))(pIVar5,pIVar17,&local_168,&local_188);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar5,(_GUID *)&DAT_1432743e8);
      }
      local_750 = local_440;
      uStack_74c = uStack_43c;
      uStack_748 = uStack_438;
      uStack_744 = uStack_434;
      local_740 = local_430;
      local_440 = local_440 & 0xffff0000;
      local_8a8 = 6;
      FUN_1401be120(&local_860);
      pIVar5 = (IUnknown *)CONCAT44(uStack_744,uStack_748);
      pIVar17 = pIVar5;
      if (((((short)local_750 - 9U & 0xfffb) != 0) &&
          (pIVar17 = pIVar10, ((short)local_750 + 0xbff7U & 0xfffb) == 0)) &&
         (pIVar5 != (IUnknown *)0x0)) {
        pIVar17 = *(IUnknown **)pIVar5;
      }
      if (pIVar17 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar17 + 8))(pIVar17);
        pIVar5 = (IUnknown *)CONCAT44(uStack_744,uStack_748);
      }
      if (pIVar18 != pIVar17) {
        local_898 = pIVar17;
        (**(code **)(*(longlong *)pIVar18 + 0x10))(pIVar18);
        pIVar5 = (IUnknown *)CONCAT44(uStack_744,uStack_748);
        pIVar17 = pIVar10;
      }
      if (pIVar17 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar17 + 0x10))(pIVar17);
        pIVar5 = (IUnknown *)CONCAT44(uStack_744,uStack_748);
      }
      if ((short)local_750 == 8) {
        local_750 = local_750 & 0xffff0000;
        if (pIVar5 != (IUnknown *)0x0) {
          (*DAT_143ad5990)(pIVar5 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_750);
      }
      if ((short)local_7f8 == 8) {
        local_7f8 = local_7f8 & 0xffff0000;
        if (uStack_7f0 != 0) {
          (*DAT_143ad5990)(uStack_7f0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_7f8);
      }
      if (local_810 == 8) {
        local_810 = 0;
        if (lStack_808 != 0) {
          (*DAT_143ad5990)(lStack_808 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_810);
      }
      LOCK();
      pIVar18 = pIVar21 + 0x10;
      iVar3 = *(int *)pIVar18;
      *(int *)pIVar18 = *(int *)pIVar18 + -1;
      UNLOCK();
      pIVar18 = local_898;
      uVar25 = local_8a8;
      if (iVar3 == 1) {
        if (*(longlong *)pIVar21 != 0) {
          (*DAT_143ad5990)(*(longlong *)pIVar21 + -4);
          *(longlong *)pIVar21 = 0;
        }
        if (*(longlong *)(pIVar21 + 8) != 0) {
          FUN_14019b4e0();
          *(longlong *)(pIVar21 + 8) = 0;
        }
        thunk_FUN_140205820(pIVar21,0x18);
        pIVar18 = local_898;
        uVar25 = local_8a8;
      }
    }
    if (pIVar15 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar15 + 0x10))(pIVar15);
    }
    local_890 = (IUnknown *)0x0;
    if (pIVar18 == (IUnknown *)0x0) {
      iVar3 = -0x7fffbffe;
    }
    else {
      (**(code **)(*(longlong *)pIVar18 + 8))(pIVar18);
      local_738 = (IUnknown *)0x0;
      iVar3 = (*(code *)**(undefined8 **)pIVar18)(pIVar18,&DAT_14327ac98,&local_738);
      local_890 = pIVar10;
      if (-1 < iVar3) {
        local_890 = local_738;
      }
    }
    pIVar15 = local_890;
    if (pIVar18 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar18 + 0x10))(pIVar18);
    }
    if (((iVar3 + 0x80000000U & 0x80000000) == 0) && (iVar3 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    if (pIVar15 != (IUnknown *)0x0) {
      local_730 = (IUnknown *)0x0;
      iVar3 = (**(code **)(*(longlong *)pIVar15 + 0xf0))(pIVar15,&local_730);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar15,(_GUID *)&DAT_14327ac98);
      }
      pIVar21 = local_730;
      local_1b8 = local_730;
      if (local_730 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1401bb8d0(&local_7e0,L"source");
      local_8a0 = &local_7e0;
      (*DAT_143262a20)(&local_3b0);
      pIVar5 = pIVar10;
      if (local_7e0 != (IUnknown *)0x0) {
        pIVar5 = *(IUnknown **)local_7e0;
      }
      iVar3 = (**(code **)(*(longlong *)pIVar21 + 0x28))(pIVar21,pIVar5,&local_3b0);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar21,(_GUID *)&DAT_143272478);
      }
      local_608 = local_3b0;
      uStack_604 = uStack_3ac;
      uStack_600 = uStack_3a8;
      uStack_5fc = uStack_3a4;
      local_5f8 = local_3a0;
      local_3b0 = local_3b0 & 0xffff0000;
      uVar25 = uVar25 | 0x18;
      local_8a8 = uVar25;
      FUN_1401be120(&local_7e0);
      puVar26 = &DAT_143278568;
      if ((short)local_608 == 8) {
        puVar26 = (undefined2 *)CONCAT44(uStack_5fc,uStack_600);
      }
      FUN_1401bb8d0(&local_720,puVar26);
      if ((short)local_608 == 8) {
        local_608 = local_608 & 0xffff0000;
        if (CONCAT44(uStack_5fc,uStack_600) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_5fc,uStack_600) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_608);
      }
      (**(code **)(*(longlong *)pIVar21 + 0x10))(pIVar21);
      pIVar21 = DAT_143add058;
      if (((local_720 != (longlong *)0x0) && (*local_720 != 0)) && (1 < *(uint *)(*local_720 + -4)))
      {
        if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(&local_840);
        if (DAT_143a8b8d8 == 8) {
          if (local_840 == 8) {
            local_840 = 0;
            if (lStack_838 != 0) {
              (*DAT_143ad5990)(lStack_838 + -4);
            }
          }
          else {
            iVar3 = (*DAT_143262a18)(&local_840);
            if (iVar3 < 0) goto LAB_14093384c;
          }
          local_840 = 8;
          if (DAT_143a8b8e0 == 0) {
            lStack_838 = FUN_1401a5fa0(0,0);
          }
          else {
            lStack_838 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
        }
        else {
          if ((local_840 == 8) && (local_840 = 0, lStack_838 != 0)) {
            (*DAT_143ad5990)(lStack_838 + -4);
          }
          iVar3 = (*DAT_143262a28)(&local_840,&DAT_143a8b8d8);
          if (iVar3 < 0) {
LAB_14093384c:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
        }
        (*DAT_143262a20)(&local_828);
        if (DAT_143a8b8d8 == 8) {
          if ((short)local_828 == 8) {
            local_828 = (uint)local_828._2_2_ << 0x10;
            if (uStack_820 != 0) {
              (*DAT_143ad5990)(uStack_820 + -4);
            }
          }
          else {
            iVar3 = (*DAT_143262a18)(&local_828);
            if (iVar3 < 0) goto LAB_14093380b;
          }
          local_828 = CONCAT22(local_828._2_2_,8);
          pIVar18 = pIVar10;
          if (DAT_143a8b8e0 != 0) {
            pIVar18 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
          uStack_820 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar18);
        }
        else {
          if (((short)local_828 == 8) &&
             (local_828 = (uint)local_828._2_2_ << 0x10, uStack_820 != 0)) {
            (*DAT_143ad5990)(uStack_820 + -4);
          }
          iVar3 = (*DAT_143262a28)(&local_828,&DAT_143a8b8d8);
          if (iVar3 < 0) {
LAB_14093380b:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
        }
        local_7d8 = local_720;
        LOCK();
        *(int *)(local_720 + 2) = (int)local_720[2] + 1;
        UNLOCK();
        local_780 = (IUnknown *)&local_7d8;
        (*DAT_143262a20)(&local_398);
        pIVar18 = pIVar10;
        if (local_7d8 != (longlong *)0x0) {
          pIVar18 = (IUnknown *)*local_7d8;
        }
        local_148 = CONCAT62(uStack_83e,local_840);
        lStack_140 = lStack_838;
        local_138 = local_830;
        local_128 = local_828;
        uStack_124 = uStack_824;
        uStack_120 = (undefined4)uStack_820;
        uStack_11c = uStack_820._4_4_;
        local_118 = local_818;
        local_8b8 = &local_398;
        iVar3 = (**(code **)(*(longlong *)pIVar21 + 0x48))(pIVar21,pIVar18,&local_128,&local_148);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar21,(_GUID *)&DAT_1432743e8);
        }
        local_5f0 = local_398;
        uStack_5ec = uStack_394;
        uStack_5e8 = uStack_390;
        uStack_5e4 = uStack_38c;
        local_5e0 = local_388;
        local_398 = local_398 & 0xffff0000;
        uVar25 = local_8a8 | 0x20;
        FUN_1401be120(&local_7d8);
        pIVar18 = (IUnknown *)CONCAT44(uStack_5e4,uStack_5e8);
        pIVar15 = pIVar18;
        if (((((short)local_5f0 - 9U & 0xfffb) != 0) &&
            (pIVar15 = pIVar10, ((short)local_5f0 + 0xbff7U & 0xfffb) == 0)) &&
           (pIVar18 != (IUnknown *)0x0)) {
          pIVar15 = *(IUnknown **)pIVar18;
        }
        local_780 = (IUnknown *)0x0;
        if (pIVar15 == (IUnknown *)0x0) {
          iVar3 = -0x7fffbffe;
        }
        else {
          local_728 = (IUnknown *)0x0;
          iVar3 = (*(code *)**(undefined8 **)pIVar15)(pIVar15,&DAT_14327ac98,&local_728);
          local_780 = pIVar10;
          if (-1 < iVar3) {
            local_780 = local_728;
          }
          pIVar18 = (IUnknown *)CONCAT44(uStack_5e4,uStack_5e8);
        }
        pIVar21 = local_780;
        if (((iVar3 + 0x80000000U & 0x80000000) == 0) && (iVar3 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0();
        }
        if ((short)local_5f0 == 8) {
          local_5f0 = local_5f0 & 0xffff0000;
          if (pIVar18 != (IUnknown *)0x0) {
            (*DAT_143ad5990)(pIVar18 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_5f0);
        }
        if ((short)local_828 == 8) {
          local_828 = local_828 & 0xffff0000;
          if (uStack_820 != 0) {
            (*DAT_143ad5990)(uStack_820 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_828);
        }
        if (local_840 == 8) {
          local_840 = 0;
          if (lStack_838 != 0) {
            (*DAT_143ad5990)(lStack_838 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_840);
        }
        pIVar15 = local_890;
        pIVar18 = local_898;
        if (pIVar21 != (IUnknown *)0x0) {
          FUN_14092ee90(local_890,pIVar21);
          (**(code **)(*(longlong *)pIVar21 + 0x10))(pIVar21);
          pIVar18 = local_898;
        }
      }
      FUN_1401be120(&local_720);
      local_718 = (longlong *)0x0;
      iVar3 = (**(code **)(*(longlong *)pIVar15 + 0xf0))(pIVar15,&local_718);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar15,(_GUID *)&DAT_14327ac98);
      }
      plVar8 = local_718;
      local_1b0 = local_718;
      uVar25 = uVar25 | 0x40;
      local_8a8 = uVar25;
      if (local_718 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1401bb8d0(local_710,L"_outlink");
      psVar6 = (short *)FUN_1401e4330(plVar8,local_380,local_710);
      if (*psVar6 == 8) {
        puVar26 = *(undefined2 **)(psVar6 + 4);
      }
      else {
        puVar26 = &DAT_143278568;
      }
      FUN_1401bb8d0(&local_6f8,puVar26);
      if (local_380[0] == 8) {
        local_380[0] = 0;
        if (local_378 != 0) {
          (*DAT_143ad5990)(local_378 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_380);
      }
      (**(code **)(*plVar8 + 0x10))(plVar8);
      pIVar21 = DAT_143add058;
      if (((local_6f8 != (longlong *)0x0) && (*local_6f8 != 0)) && (1 < *(uint *)(*local_6f8 + -4)))
      {
        if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(local_578);
        iVar3 = FUN_14023c4c0(local_578,&DAT_143a8b8d8);
        if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
        (*DAT_143262a20)(local_590);
        iVar3 = FUN_14023c4c0(local_590,param_3);
        if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
        local_708 = local_6f8;
        LOCK();
        *(int *)(local_6f8 + 2) = (int)local_6f8[2] + 1;
        UNLOCK();
        local_8b8 = (uint *)local_578;
        uVar7 = FUN_140403ec0(pIVar21,local_368,&local_708,local_590);
        local_700 = (longlong *)0x0;
        iVar3 = FUN_140360cd0(&local_700,uVar7);
        if (((iVar3 + 0x80000000U & 0x80000000) == 0) && (iVar3 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
        if (local_368[0] == 8) {
          local_368[0] = 0;
          if (local_360 != 0) {
            (*DAT_143ad5990)(local_360 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_368);
        }
        if (local_590[0] == 8) {
          local_590[0] = 0;
          if (local_588 != 0) {
            (*DAT_143ad5990)(local_588 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_590);
        }
        if (local_578[0] == 8) {
          local_578[0] = 0;
          if (local_570 != 0) {
            (*DAT_143ad5990)(local_570 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_578);
        }
        plVar8 = local_700;
        pIVar15 = local_890;
        pIVar18 = local_898;
        uVar25 = local_8a8;
        if (local_700 != (longlong *)0x0) {
          FUN_14092ee90(local_890,local_700);
          (**(code **)(*plVar8 + 0x10))(plVar8);
          pIVar18 = local_898;
          uVar25 = local_8a8;
        }
      }
      FUN_1401be120(&local_6f8);
      local_770 = (longlong *)0x0;
      iVar3 = (**(code **)(*(longlong *)pIVar15 + 0xf0))(pIVar15,&local_770);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar15,(_GUID *)&DAT_14327ac98);
      }
      plVar8 = local_770;
      local_1a8 = local_770;
      uVar25 = uVar25 | 0x80;
      local_8a8 = uVar25;
      if (local_770 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1401bb8d0(local_6f0,L"_inlink");
      psVar6 = (short *)FUN_1401e4330(plVar8,local_3c8,local_6f0);
      if (*psVar6 == 8) {
        puVar26 = *(undefined2 **)(psVar6 + 4);
      }
      else {
        puVar26 = &DAT_143278568;
      }
      FUN_1401bb8d0(&local_7a8,puVar26);
      if (local_3c8[0] == 8) {
        local_3c8[0] = 0;
        if (local_3c0 != 0) {
          (*DAT_143ad5990)(local_3c0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_3c8);
      }
      (**(code **)(*plVar8 + 0x10))(plVar8);
      if (((local_7a8 != (longlong *)0x0) && (*local_7a8 != 0)) && (1 < *(uint *)(*local_7a8 + -4)))
      {
        plVar8 = (longlong *)FUN_14092e200(pIVar15,&local_4e8);
        lVar24 = *plVar8;
        if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        FUN_1401bb8d0(local_6e8,L"_filepath");
        psVar6 = (short *)FUN_1401e4330(lVar24,local_338,local_6e8);
        if (*psVar6 == 8) {
          puVar26 = *(undefined2 **)(psVar6 + 4);
        }
        else {
          puVar26 = &DAT_143278568;
        }
        FUN_1401bb8d0(&local_7c0,puVar26);
        if (local_338[0] == 8) {
          local_338[0] = 0;
          if (local_330 != 0) {
            (*DAT_143ad5990)(local_330 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_338);
        }
        if (local_4e8 != (longlong *)0x0) {
          (**(code **)(*local_4e8 + 0x10))();
        }
        local_888 = (IUnknown *)0x0;
        puVar9 = (undefined8 *)FUN_14092df20(&local_7c0);
        FUN_14022d860(&local_6e0,*puVar9,0xffffffff);
        lVar24 = local_6e0;
        iVar3 = iVar2;
        if (local_6e0 != 0) {
          iVar3 = *(int *)(local_6e0 + -8);
        }
        iVar1 = iVar3 + -1;
        if (local_6e0 == 0) {
          FUN_142e52dd0(199,iVar1);
        }
        else {
          iVar2 = *(int *)(local_6e0 + -8);
        }
        if ((iVar1 < 0) || (iVar2 <= iVar1)) {
          pIVar18 = pIVar10;
          if (lVar24 != 0) {
            pIVar18 = (IUnknown *)(ulonglong)*(uint *)(lVar24 + -8);
          }
          FUN_142e54290(0xcc,iVar1,pIVar18);
        }
        if (*(char *)((longlong)iVar3 + -1 + lVar24) == '/') {
          local_7d0 = local_7c0;
          if (local_7c0 != (IUnknown *)0x0) {
            LOCK();
            *(int *)(local_7c0 + 0x10) = *(int *)(local_7c0 + 0x10) + 1;
            UNLOCK();
          }
          local_618 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
          if (local_618 != (IUnknown *)0x0) {
            pIVar10 = (IUnknown *)FUN_1401bb660(local_618,&local_7d0,&local_7a8);
          }
          if (pIVar10 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x8007000e);
          }
          FUN_1401be120(&local_7d0);
          LOCK();
          *(int *)(pIVar10 + 0x10) = *(int *)(pIVar10 + 0x10) + 1;
          UNLOCK();
          local_7d0 = pIVar10;
          local_618 = pIVar10;
          FUN_1401be120(&local_7d0);
          LOCK();
          *(int *)(pIVar10 + 0x10) = *(int *)(pIVar10 + 0x10) + 1;
          UNLOCK();
          LOCK();
          *(int *)(pIVar10 + 0x10) = *(int *)(pIVar10 + 0x10) + 1;
          UNLOCK();
          local_888 = pIVar10;
          local_460 = pIVar10;
          FUN_1401be120(&local_460);
          ppIVar16 = &local_618;
        }
        else {
          plVar8 = (longlong *)FUN_1401bc490(&local_7c0,&local_468,&DAT_1432ac608);
          local_7c8 = (IUnknown *)*plVar8;
          if (local_7c8 != (IUnknown *)0x0) {
            LOCK();
            *(int *)(local_7c8 + 0x10) = *(int *)(local_7c8 + 0x10) + 1;
            UNLOCK();
            uVar25 = local_8a8;
          }
          local_8a8 = uVar25 | 0x200;
          local_610 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
          if (local_610 != (IUnknown *)0x0) {
            pIVar10 = (IUnknown *)FUN_1401bb660(local_610,&local_7c8,&local_7a8);
          }
          if (pIVar10 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x8007000e);
          }
          FUN_1401be120(&local_7c8);
          LOCK();
          *(int *)(pIVar10 + 0x10) = *(int *)(pIVar10 + 0x10) + 1;
          UNLOCK();
          LOCK();
          *(int *)(pIVar10 + 0x10) = *(int *)(pIVar10 + 0x10) + 1;
          UNLOCK();
          local_888 = pIVar10;
          local_7c8 = pIVar10;
          local_610 = pIVar10;
          FUN_1401be120(&local_610);
          FUN_1401be120(&local_7c8);
          ppIVar16 = &local_468;
        }
        FUN_1401be120(ppIVar16);
        pIVar18 = DAT_143add058;
        if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(local_548);
        iVar2 = FUN_14023c4c0(local_548,&DAT_143a8b8d8);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
        (*DAT_143262a20)(local_560);
        iVar2 = FUN_14023c4c0(local_560,&DAT_143a8b8d8);
        pIVar21 = local_888;
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
        local_760 = local_888;
        if (pIVar10 != (IUnknown *)0x0) {
          LOCK();
          *(int *)(pIVar10 + 0x10) = *(int *)(pIVar10 + 0x10) + 1;
          UNLOCK();
        }
        local_8b8 = (uint *)local_548;
        uVar7 = FUN_140403ec0(pIVar18,local_320,&local_760,local_560);
        uVar7 = FUN_14092dfa0(uVar7,0,0);
        FUN_140924200(&local_470,uVar7);
        if (local_320[0] == 8) {
          local_320[0] = 0;
          if (local_318 != 0) {
            (*DAT_143ad5990)(local_318 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_320);
        }
        if (local_560[0] == 8) {
          local_560[0] = 0;
          if (local_558 != 0) {
            (*DAT_143ad5990)(local_558 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_560);
        }
        if (local_548[0] == 8) {
          local_548[0] = 0;
          if (local_540 != 0) {
            (*DAT_143ad5990)(local_540 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_548);
        }
        pIVar15 = local_890;
        if (local_470 != (longlong *)0x0) {
          FUN_14092ee90(local_890,local_470);
          (**(code **)(*local_470 + 0x10))(local_470);
        }
        if (local_6e0 != 0) {
          FUN_14019f2c0(local_6e0 + -0x10);
        }
        pIVar10 = local_888;
        if (pIVar21 != (IUnknown *)0x0) {
          LOCK();
          pIVar21 = pIVar21 + 0x10;
          iVar2 = *(int *)pIVar21;
          *(int *)pIVar21 = *(int *)pIVar21 + -1;
          UNLOCK();
          pIVar15 = local_890;
          if (iVar2 == 1) {
            if (*(longlong *)local_888 != 0) {
              (*DAT_143ad5990)(*(longlong *)local_888 + -4);
              *(longlong *)pIVar10 = 0;
            }
            if (*(longlong *)(pIVar10 + 8) != 0) {
              FUN_14019b4e0();
              *(longlong *)(pIVar10 + 8) = 0;
            }
            thunk_FUN_140205820(pIVar10,0x18);
            pIVar15 = local_890;
          }
        }
        FUN_1401be120(&local_7c0);
        pIVar18 = local_898;
      }
      FUN_1401be120(&local_7a8);
      local_6d8 = (longlong *)0x0;
      iVar2 = (**(code **)(*(longlong *)pIVar15 + 0xf0))(pIVar15,&local_6d8);
      if (iVar2 < 0) {
        _com_issue_errorex(iVar2,pIVar15,(_GUID *)&DAT_14327ac98);
      }
      plVar8 = local_6d8;
      local_1a0 = local_6d8;
      if (local_6d8 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1401bb8d0(local_6d0,L"_binlink");
      psVar6 = (short *)FUN_1401e4330(plVar8,local_308,local_6d0);
      if (*psVar6 == 8) {
        puVar26 = *(undefined2 **)(psVar6 + 4);
      }
      else {
        puVar26 = &DAT_143278568;
      }
      FUN_1403edf80(&local_7a0,puVar26,0xffffffff);
      if (local_308[0] == 8) {
        local_308[0] = 0;
        if (local_300 != 0) {
          (*DAT_143ad5990)(local_300 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_308);
      }
      (**(code **)(*plVar8 + 0x10))(plVar8);
      psVar6 = local_7a0;
      if ((local_7a0 != (short *)0x0) && (*local_7a0 != 0)) {
        plVar8 = (longlong *)FUN_14092e200(pIVar15,&local_478);
        lVar24 = *plVar8;
        if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        FUN_1401bb8d0(local_6c8,L"_filepath");
        psVar11 = (short *)FUN_1401e4330(lVar24,local_2f0,local_6c8);
        if (*psVar11 == 8) {
          puVar26 = *(undefined2 **)(psVar11 + 4);
        }
        else {
          puVar26 = &DAT_143278568;
        }
        FUN_1401bb8d0(&local_878,puVar26);
        if (local_2f0[0] == 8) {
          local_2f0[0] = 0;
          if (local_2e8 != 0) {
            (*DAT_143ad5990)(local_2e8 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_2f0);
        }
        if (local_478 != (longlong *)0x0) {
          (**(code **)(*local_478 + 0x10))();
        }
        plVar8 = (longlong *)FUN_14092df20(&local_878);
        lVar24 = -1;
        do {
          lVar24 = lVar24 + 1;
        } while (*(short *)(*plVar8 + lVar24 * 2) != 0);
        local_b8 = 0;
        local_b0 = 7;
        local_c8[0] = (undefined8 ****)0x0;
        FUN_1401e6270(local_c8);
        FUN_140924110(local_a8,&DAT_14330310c,0);
        FUN_140933a20(local_c8,local_a8);
        FID_conflict__Tidy_deallocate(local_a8);
        ppppuVar19 = local_c8;
        if (7 < local_b0) {
          ppppuVar19 = (undefined8 ****)local_c8[0];
        }
        FUN_1401bb8d0(&local_6c0,ppppuVar19);
        FUN_1401be120(&local_878);
        local_878 = local_6c0;
        if (local_6c0 != 0) {
          LOCK();
          *(int *)(local_6c0 + 0x10) = *(int *)(local_6c0 + 0x10) + 1;
          UNLOCK();
          pIVar15 = local_890;
          pIVar18 = local_898;
          psVar6 = local_7a0;
        }
        FUN_1401be120(&local_6c0);
        if (7 < local_b0) {
          if (0xfff < local_b0 * 2 + 2) {
            if (0x1f < (ulonglong)((longlong)local_c8[0] + (-8 - (longlong)local_c8[0][-1]))) {
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(local_c8[0][-1],local_b0 * 2 + 0x29);
            }
          }
          thunk_FUN_140205820();
        }
        pIVar10 = DAT_143add058;
        if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(local_500);
        iVar2 = FUN_14023c4c0(local_500,&DAT_143a8b8d8);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
        (*DAT_143262a20)(local_518);
        iVar2 = FUN_14023c4c0(local_518,&DAT_143a8b8d8);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
        local_788 = local_6c0;
        if (local_6c0 != 0) {
          LOCK();
          *(int *)(local_6c0 + 0x10) = *(int *)(local_6c0 + 0x10) + 1;
          UNLOCK();
          pIVar15 = local_890;
          pIVar18 = local_898;
          psVar6 = local_7a0;
        }
        local_8b8 = (uint *)local_500;
        uVar7 = FUN_140403ec0(pIVar10,local_2d8,&local_788,local_518);
        uVar7 = FUN_14092dfa0(uVar7,0,0);
        FUN_140924290(&local_448,uVar7);
        if (local_2d8[0] == 8) {
          local_2d8[0] = 0;
          if (local_2d0 != 0) {
            (*DAT_143ad5990)(local_2d0 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_2d8);
        }
        if (local_518[0] == 8) {
          local_518[0] = 0;
          if (local_510 != 0) {
            (*DAT_143ad5990)(local_510 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_518);
        }
        if (local_500[0] == 8) {
          local_500[0] = 0;
          if (local_4f8 != 0) {
            (*DAT_143ad5990)(local_4f8 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_500);
        }
        if (local_448 != (longlong *)0x0) {
          uVar7 = FUN_14092d470(&local_7a0,5);
          FUN_1401bb8d0(local_6b8,uVar7);
          uVar7 = FUN_1401e4330(local_448,local_2c0,local_6b8);
          uVar7 = FUN_14092dfa0(uVar7,0,0);
          FUN_140924200(&local_6a8,uVar7);
          if (local_2c0[0] == 8) {
            local_2c0[0] = 0;
            if (local_2b8 != 0) {
              (*DAT_143ad5990)(local_2b8 + -4);
            }
          }
          else {
            (*DAT_143262a18)(local_2c0);
          }
          plVar8 = local_6a8;
          if (local_6a8 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          plVar12 = (longlong *)FUN_14092e200(local_6a8,&local_450);
          lVar24 = *plVar12;
          if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          FUN_1401bb8d0(local_6b0,L"_inlink");
          psVar11 = (short *)FUN_1401e4330(lVar24,local_2a8,local_6b0);
          if (*psVar11 == 8) {
            puVar26 = *(undefined2 **)(psVar11 + 4);
          }
          else {
            puVar26 = &DAT_143278568;
          }
          FUN_14022d860(&local_480,puVar26,0xffffffff);
          if (local_2a8[0] == 8) {
            local_2a8[0] = 0;
            if (local_2a0 != 0) {
              (*DAT_143ad5990)(local_2a0 + -4);
            }
          }
          else {
            (*DAT_143262a18)(local_2a8);
          }
          if (local_450 != (longlong *)0x0) {
            (**(code **)(*local_450 + 0x10))();
          }
          plVar12 = plVar8;
          if ((local_480 != (char *)0x0) && (*local_480 != '\0')) {
            uVar7 = FUN_1401a5780(local_488,local_480);
            uVar7 = FUN_1401e4330(local_448,local_290,uVar7);
            uVar7 = FUN_14092dfa0(uVar7,0,0);
            FUN_140924200(&local_6a0,uVar7);
            plVar23 = local_6a0;
            plVar22 = local_6a0;
            if (plVar8 != local_6a0) {
              local_6a8 = local_6a0;
              local_6a0 = (longlong *)0x0;
              plVar22 = (longlong *)0x0;
              plVar12 = plVar23;
              if (plVar8 != (longlong *)0x0) {
                (**(code **)(*plVar8 + 0x10))();
                plVar22 = (longlong *)0x0;
              }
            }
            if (plVar22 != (longlong *)0x0) {
              (**(code **)(*plVar22 + 0x10))(plVar22);
            }
            if (local_290[0] == 8) {
              local_290[0] = 0;
              if (local_288 != 0) {
                (*DAT_143ad5990)(local_288 + -4);
              }
            }
            else {
              (*DAT_143262a18)(local_290);
            }
          }
          if (local_480 != (char *)0x0) {
            FUN_14019f2c0(local_480 + -0x10);
          }
          if (plVar12 != (longlong *)0x0) {
            FUN_14092ee90(pIVar15,plVar12);
            plVar8 = (longlong *)FUN_14092e200(pIVar15,&local_4e0);
            lVar24 = *plVar8;
            if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            FUN_1401bb8d0(local_698,L"origin");
            uVar7 = FUN_1401e4330(lVar24,local_278,local_698);
            puVar9 = (undefined8 *)FUN_14092dfa0(uVar7,0,0);
            local_620 = (longlong *)0x0;
            if (puVar9 == (undefined8 *)0x0) {
              iVar2 = -0x7fffbffe;
            }
            else {
              local_690 = (longlong *)0x0;
              iVar2 = (**(code **)*puVar9)(puVar9,&DAT_143273488,&local_690);
              local_620 = (longlong *)0x0;
              if (-1 < iVar2) {
                local_620 = local_690;
              }
            }
            plVar8 = local_620;
            if (((iVar2 + 0x80000000U & 0x80000000) == 0) && (iVar2 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0();
            }
            if (local_278[0] == 8) {
              local_278[0] = 0;
              if (local_270 != 0) {
                (*DAT_143ad5990)(local_270 + -4);
              }
            }
            else {
              (*DAT_143262a18)(local_278);
            }
            if (local_4e0 != (longlong *)0x0) {
              (**(code **)(*local_4e0 + 0x10))();
            }
            if (plVar8 == (longlong *)0x0) {
              FUN_14092e360(pIVar15,0);
              uVar4 = 0;
            }
            else {
              uVar4 = FUN_140319f60(plVar8);
              FUN_14092e360(pIVar15,uVar4);
              uVar4 = FUN_140319fa0(plVar8);
            }
            FUN_14092e3a0(pIVar15,uVar4);
            if (plVar8 != (longlong *)0x0) {
              (**(code **)(*plVar8 + 0x10))(plVar8);
            }
          }
          if (plVar12 != (longlong *)0x0) {
            (**(code **)(*plVar12 + 0x10))(plVar12);
          }
        }
        if (local_448 != (longlong *)0x0) {
          (**(code **)(*local_448 + 0x10))(local_448);
        }
        FUN_1401be120(&local_878);
      }
      if (psVar6 != (short *)0x0) {
        FUN_1401bebb0(psVar6 + -8);
      }
    }
    if (pIVar15 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar15 + 0x10))(pIVar15);
    }
    FUN_140924440(&local_758,&local_898);
    pIVar10 = local_758;
    if (local_758 != (IUnknown *)0x0) {
      plVar8 = (longlong *)FUN_14092e2c0(local_758,&local_4d8);
      lVar24 = *plVar8;
      if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1401bb8d0(local_688,L"_binlink");
      psVar6 = (short *)FUN_1401e4330(lVar24,local_260,local_688);
      if (*psVar6 == 8) {
        puVar26 = *(undefined2 **)(psVar6 + 4);
      }
      else {
        puVar26 = &DAT_143278568;
      }
      FUN_1403edf80(&local_7b0,puVar26,0xffffffff);
      if (local_260[0] == 8) {
        local_260[0] = 0;
        if (local_258 != 0) {
          (*DAT_143ad5990)(local_258 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_260);
      }
      if (local_4d8 != (longlong *)0x0) {
        (**(code **)(*local_4d8 + 0x10))();
      }
      psVar6 = local_7b0;
      if ((local_7b0 != (short *)0x0) && (*local_7b0 != 0)) {
        plVar8 = (longlong *)FUN_14092e2c0(pIVar10,&local_4d0);
        lVar24 = *plVar8;
        if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        FUN_1401bb8d0(local_680,L"_filepath");
        psVar11 = (short *)FUN_1401e4330(lVar24,local_248,local_680);
        if (*psVar11 == 8) {
          puVar26 = *(undefined2 **)(psVar11 + 4);
        }
        else {
          puVar26 = &DAT_143278568;
        }
        FUN_1401bb8d0(&local_870,puVar26);
        if (local_248[0] == 8) {
          local_248[0] = 0;
          if (local_240 != 0) {
            (*DAT_143ad5990)(local_240 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_248);
        }
        if (local_4d0 != (longlong *)0x0) {
          (**(code **)(*local_4d0 + 0x10))();
        }
        plVar8 = (longlong *)FUN_14092df20(&local_870);
        lVar24 = -1;
        do {
          lVar24 = lVar24 + 1;
        } while (*(short *)(*plVar8 + lVar24 * 2) != 0);
        local_f8 = 0;
        local_f0 = 7;
        local_108[0] = (undefined8 ****)0x0;
        FUN_1401e6270(local_108);
        FUN_140924110(local_88,&DAT_14330310c,0);
        FUN_140933a20(local_108,local_88);
        FID_conflict__Tidy_deallocate(local_88);
        ppppuVar19 = local_108;
        if (7 < local_f0) {
          ppppuVar19 = (undefined8 ****)local_108[0];
        }
        FUN_1401bb8d0(&local_678,ppppuVar19);
        FUN_1401be120(&local_870);
        local_870 = local_678;
        if (local_678 != 0) {
          LOCK();
          *(int *)(local_678 + 0x10) = *(int *)(local_678 + 0x10) + 1;
          UNLOCK();
          pIVar18 = local_898;
          psVar6 = local_7b0;
          pIVar10 = local_758;
        }
        FUN_1401be120(&local_678);
        if (7 < local_f0) {
          deallocate(local_108,local_108[0],local_f0 + 1);
        }
        pIVar15 = DAT_143add058;
        if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(local_5c0);
        iVar2 = FUN_14023c4c0(local_5c0,&DAT_143a8b8d8);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
        (*DAT_143262a20)(local_5d8);
        iVar2 = FUN_14023c4c0(local_5d8,&DAT_143a8b8d8);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
        local_790 = local_678;
        if (local_678 != 0) {
          LOCK();
          *(int *)(local_678 + 0x10) = *(int *)(local_678 + 0x10) + 1;
          UNLOCK();
          pIVar18 = local_898;
          psVar6 = local_7b0;
          pIVar10 = local_758;
        }
        local_8b8 = (uint *)local_5c0;
        uVar7 = FUN_140403ec0(pIVar15,local_230,&local_790,local_5d8);
        uVar7 = FUN_14092dfa0(uVar7,0,0);
        FUN_140924290(&local_4c8,uVar7);
        if (local_230[0] == 8) {
          local_230[0] = 0;
          if (local_228 != 0) {
            (*DAT_143ad5990)(local_228 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_230);
        }
        if (local_5d8[0] == 8) {
          local_5d8[0] = 0;
          if (local_5d0 != 0) {
            (*DAT_143ad5990)(local_5d0 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_5d8);
        }
        if (local_5c0[0] == 8) {
          local_5c0[0] = 0;
          if (local_5b8 != 0) {
            (*DAT_143ad5990)(local_5b8 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_5c0);
        }
        if (local_4c8 != (longlong *)0x0) {
          uVar7 = FUN_14092d470(&local_7b0,5);
          FUN_1401bb8d0(local_670,uVar7);
          uVar7 = FUN_1401e4330(local_4c8,local_218,local_670);
          uVar7 = FUN_14092dfa0(uVar7,0,0);
          FUN_1409243b0(&local_660,uVar7);
          if (local_218[0] == 8) {
            local_218[0] = 0;
            if (local_210 != 0) {
              (*DAT_143ad5990)(local_210 + -4);
            }
          }
          else {
            (*DAT_143262a18)(local_218);
          }
          plVar8 = local_660;
          if (local_660 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          plVar12 = (longlong *)FUN_14092e2c0(local_660,&local_4c0);
          lVar24 = *plVar12;
          if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          FUN_1401bb8d0(local_668,L"_inlink");
          psVar11 = (short *)FUN_1401e4330(lVar24,local_200,local_668);
          if (*psVar11 == 8) {
            puVar26 = *(undefined2 **)(psVar11 + 4);
          }
          else {
            puVar26 = &DAT_143278568;
          }
          FUN_14022d860(&local_4b8,puVar26,0xffffffff);
          if (local_200[0] == 8) {
            local_200[0] = 0;
            if (local_1f8 != 0) {
              (*DAT_143ad5990)(local_1f8 + -4);
            }
          }
          else {
            (*DAT_143262a18)(local_200);
          }
          if (local_4c0 != (longlong *)0x0) {
            (**(code **)(*local_4c0 + 0x10))();
          }
          plVar12 = plVar8;
          if ((local_4b8 != (char *)0x0) && (*local_4b8 != '\0')) {
            uVar7 = FUN_1401a5780(local_4b0,local_4b8);
            uVar7 = FUN_1401e4330(local_4c8,local_1e8,uVar7);
            uVar7 = FUN_14092dfa0(uVar7,0,0);
            FUN_1409243b0(&local_658,uVar7);
            plVar23 = local_658;
            if (plVar8 != local_658) {
              local_660 = local_658;
              local_658 = (longlong *)0x0;
              plVar12 = plVar23;
              plVar23 = (longlong *)0x0;
              if (plVar8 != (longlong *)0x0) {
                (**(code **)(*plVar8 + 0x10))();
                plVar23 = (longlong *)0x0;
              }
            }
            if (plVar23 != (longlong *)0x0) {
              (**(code **)(*plVar23 + 0x10))(plVar23);
            }
            if (local_1e8[0] == 8) {
              local_1e8[0] = 0;
              if (local_1e0 != 0) {
                (*DAT_143ad5990)(local_1e0 + -4);
              }
            }
            else {
              (*DAT_143262a18)(local_1e8);
            }
          }
          if (local_4b8 != (char *)0x0) {
            FUN_14019f2c0(local_4b8 + -0x10);
          }
          if ((plVar12 != (longlong *)0x0) &&
             (iVar2 = (**(code **)(*(longlong *)pIVar10 + 200))(pIVar10,plVar12), iVar2 < 0)) {
            _com_issue_errorex(iVar2,pIVar10,(_GUID *)&DAT_14329d348);
          }
          if (plVar12 != (longlong *)0x0) {
            (**(code **)(*plVar12 + 0x10))(plVar12);
          }
        }
        if (local_4c8 != (longlong *)0x0) {
          (**(code **)(*local_4c8 + 0x10))(local_4c8);
        }
        FUN_1401be120(&local_870);
      }
      if (psVar6 != (short *)0x0) {
        FUN_1401bebb0(psVar6 + -8);
      }
    }
    if (pIVar10 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar10 + 0x10))(pIVar10);
    }
    local_880 = (IUnknown *)0x0;
    if (pIVar18 == (IUnknown *)0x0) {
      iVar2 = -0x7fffbffe;
    }
    else {
      (**(code **)(*(longlong *)pIVar18 + 8))(pIVar18);
      local_650 = (IUnknown *)0x0;
      iVar2 = (*(code *)**(undefined8 **)pIVar18)(pIVar18,&DAT_143299f18,&local_650);
      local_880 = (IUnknown *)0x0;
      if (-1 < iVar2) {
        local_880 = local_650;
      }
    }
    pIVar10 = local_880;
    if (pIVar18 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar18 + 0x10))(pIVar18);
    }
    if (((iVar2 + 0x80000000U & 0x80000000) == 0) && (iVar2 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
    if (pIVar10 != (IUnknown *)0x0) {
      plVar8 = (longlong *)FUN_14092e260(pIVar10,&local_4a8);
      lVar24 = *plVar8;
      if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1401bb8d0(local_648,L"_binlink");
      psVar6 = (short *)FUN_1401e4330(lVar24,local_1d0,local_648);
      puVar26 = &DAT_143278568;
      if (*psVar6 == 8) {
        puVar20 = *(undefined2 **)(psVar6 + 4);
      }
      else {
        puVar20 = &DAT_143278568;
      }
      FUN_1403edf80(&local_7b8,puVar20,0xffffffff);
      if (local_1d0[0] == 8) {
        local_1d0[0] = 0;
        if (local_1c8 != 0) {
          (*DAT_143ad5990)(local_1c8 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_1d0);
      }
      if (local_4a8 != (longlong *)0x0) {
        (**(code **)(*local_4a8 + 0x10))();
      }
      psVar6 = local_7b8;
      if ((local_7b8 != (short *)0x0) && (*local_7b8 != 0)) {
        plVar8 = (longlong *)FUN_14092e260(pIVar10,&local_458);
        lVar24 = *plVar8;
        if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        FUN_1401bb8d0(local_640,L"_filepath");
        psVar11 = (short *)FUN_1401e4330(lVar24,local_428,local_640);
        if (*psVar11 == 8) {
          puVar20 = *(undefined2 **)(psVar11 + 4);
        }
        else {
          puVar20 = &DAT_143278568;
        }
        FUN_1401bb8d0(&local_868,puVar20);
        if (local_428[0] == 8) {
          local_428[0] = 0;
          if (local_420 != 0) {
            (*DAT_143ad5990)(local_420 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_428);
        }
        if (local_458 != (longlong *)0x0) {
          (**(code **)(*local_458 + 0x10))();
        }
        plVar8 = (longlong *)FUN_14092df20(&local_868);
        lVar24 = -1;
        do {
          lVar24 = lVar24 + 1;
        } while (*(short *)(*plVar8 + lVar24 * 2) != 0);
        local_d8 = 0;
        local_d0 = 7;
        local_e8[0] = (undefined8 ****)0x0;
        FUN_1401e6270(local_e8);
        FUN_140924110(local_68,&DAT_14330310c,0);
        FUN_140933a20(local_e8,local_68);
        FID_conflict__Tidy_deallocate(local_68);
        ppppuVar19 = local_e8;
        if (7 < local_d0) {
          ppppuVar19 = (undefined8 ****)local_e8[0];
        }
        FUN_1401bb8d0(&local_638,ppppuVar19);
        FUN_1401be120(&local_868);
        local_868 = local_638;
        if (local_638 != 0) {
          LOCK();
          *(int *)(local_638 + 0x10) = *(int *)(local_638 + 0x10) + 1;
          UNLOCK();
          pIVar18 = local_898;
          pIVar10 = local_880;
          psVar6 = local_7b8;
        }
        FUN_1401be120(&local_638);
        if (7 < local_d0) {
          deallocate(local_e8,local_e8[0],local_d0 + 1);
        }
        pIVar15 = DAT_143add058;
        if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(local_530);
        iVar2 = FUN_14023c4c0(local_530,&DAT_143a8b8d8);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
        (*DAT_143262a20)(local_5a8);
        iVar2 = FUN_14023c4c0(local_5a8,&DAT_143a8b8d8);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar2);
        }
        local_798 = local_638;
        if (local_638 != 0) {
          LOCK();
          *(int *)(local_638 + 0x10) = *(int *)(local_638 + 0x10) + 1;
          UNLOCK();
          pIVar18 = local_898;
          pIVar10 = local_880;
          psVar6 = local_7b8;
        }
        local_8b8 = (uint *)local_530;
        uVar7 = FUN_140403ec0(pIVar15,local_410,&local_798,local_5a8);
        uVar7 = FUN_14092dfa0(uVar7,0,0);
        FUN_140924290(&local_4a0,uVar7);
        if (local_410[0] == 8) {
          local_410[0] = 0;
          if (local_408 != 0) {
            (*DAT_143ad5990)(local_408 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_410);
        }
        if (local_5a8[0] == 8) {
          local_5a8[0] = 0;
          if (local_5a0 != 0) {
            (*DAT_143ad5990)(local_5a0 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_5a8);
        }
        if (local_530[0] == 8) {
          local_530[0] = 0;
          if (local_528 != 0) {
            (*DAT_143ad5990)(local_528 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_530);
        }
        if (local_4a0 != (longlong *)0x0) {
          uVar7 = FUN_14092d470(&local_7b8,5);
          FUN_1401bb8d0(&local_850,uVar7);
          uVar7 = FUN_1401e4330(local_4a0,local_3f8,&local_850);
          uVar7 = FUN_14092dfa0(uVar7,0,0);
          FUN_140924320(&local_630,uVar7);
          if (local_3f8[0] == 8) {
            local_3f8[0] = 0;
            if (local_3f0 != 0) {
              (*DAT_143ad5990)(local_3f0 + -4);
            }
          }
          else {
            (*DAT_143262a18)(local_3f8);
          }
          pIVar15 = local_630;
          if (local_630 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          plVar8 = (longlong *)FUN_14092e260(local_630,&local_498);
          lVar24 = *plVar8;
          if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          FUN_1401bb8d0(&local_848,L"_inlink");
          psVar11 = (short *)FUN_1401e4330(lVar24,local_3e0,&local_848);
          if (*psVar11 == 8) {
            puVar26 = *(undefined2 **)(psVar11 + 4);
          }
          FUN_14022d860(&local_490,puVar26,0xffffffff);
          if (local_3e0[0] == 8) {
            local_3e0[0] = 0;
            if (local_3d8 != 0) {
              (*DAT_143ad5990)(local_3d8 + -4);
            }
          }
          else {
            (*DAT_143262a18)(local_3e0);
          }
          if (local_498 != (longlong *)0x0) {
            (**(code **)(*local_498 + 0x10))();
          }
          pIVar21 = pIVar15;
          if ((local_490 != (char *)0x0) && (*local_490 != '\0')) {
            uVar7 = FUN_1401a5780(&local_768,local_490);
            uVar7 = FUN_1401e4330(local_4a0,local_350,uVar7);
            uVar7 = FUN_14092dfa0(uVar7,0,0);
            FUN_140924320(&local_858,uVar7);
            pIVar5 = local_858;
            if (pIVar15 != local_858) {
              local_630 = local_858;
              pIVar21 = local_858;
              pIVar5 = (IUnknown *)0x0;
              if (pIVar15 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)pIVar15 + 0x10))();
                pIVar5 = (IUnknown *)0x0;
              }
            }
            if (pIVar5 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)pIVar5 + 0x10))(pIVar5);
            }
            if (local_350[0] == 8) {
              local_350[0] = 0;
              if (local_348 != 0) {
                (*DAT_143ad5990)(local_348 + -4);
              }
            }
            else {
              (*DAT_143262a18)(local_350);
            }
          }
          if (local_490 != (char *)0x0) {
            FUN_14019f2c0(local_490 + -0x10);
          }
          if (pIVar21 != (IUnknown *)0x0) {
            uVar7 = FUN_14092e320(pIVar21);
            iVar2 = (**(code **)(*(longlong *)pIVar10 + 0x28))(pIVar10,uVar7);
            if (iVar2 < 0) {
              _com_issue_errorex(iVar2,pIVar10,(_GUID *)&DAT_143299f18);
            }
            uVar7 = FUN_14092e320(pIVar21);
            uVar13 = FUN_14092e1c0(pIVar21);
            uVar14 = FUN_14092e1c0(pIVar10);
            FUN_142ef7ba0(uVar14,uVar13,uVar7);
          }
          if (pIVar21 != (IUnknown *)0x0) {
            (**(code **)(*(longlong *)pIVar21 + 0x10))(pIVar21);
          }
        }
        if (local_4a0 != (longlong *)0x0) {
          (**(code **)(*local_4a0 + 0x10))(local_4a0);
        }
        FUN_1401be120(&local_868);
      }
      if (psVar6 != (short *)0x0) {
        FUN_1401bebb0(psVar6 + -8);
      }
    }
    if (pIVar10 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar10 + 0x10))(pIVar10);
    }
    *(IUnknown **)local_628 = pIVar18;
    param_1 = local_628;
  }
  return param_1;
}



//===========================================================
// FUN_14090e210 @ 14090e210   (90 bytes)
//===========================================================

undefined8 FUN_14090e210(undefined8 param_1,longlong *param_2,undefined8 param_3,undefined8 param_4)

{
  (*DAT_143262a20)();
  param_2 = (longlong *)*param_2;
  if (param_2 != (longlong *)0x0) {
    (**(code **)(*param_2 + 0x28))(param_2,param_3,param_1,param_4,1);
  }
  return param_1;
}



//===========================================================
// FUN_1403f1da0 @ 1403f1da0   (732 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_1403f1da0(float *param_1,longlong *param_2,byte *param_3,longlong *param_4)

{
  float *pfVar1;
  int iVar2;
  ulonglong uVar3;
  longlong *plVar4;
  longlong *plVar5;
  longlong *plVar6;
  longlong lVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *plVar11;
  float fVar12;
  
  uVar9 = (((((ulonglong)*param_3 ^ 0xcbf29ce484222325) * 0x100000001b3 ^ (ulonglong)param_3[1]) *
            0x100000001b3 ^ (ulonglong)param_3[2]) * 0x100000001b3 ^ (ulonglong)param_3[3]) *
          0x100000001b3;
  plVar6 = *(longlong **)
            (*(longlong *)(param_1 + 6) + 8 + (uVar9 & *(ulonglong *)(param_1 + 0xc)) * 0x10);
  pfVar1 = param_1 + 2;
  plVar11 = *(longlong **)pfVar1;
  if (plVar6 != plVar11) {
    iVar2 = (int)plVar6[2];
    plVar11 = plVar6;
    while( true ) {
      if (*(int *)param_3 == iVar2) {
        *param_2 = (longlong)plVar11;
        *(undefined1 *)(param_2 + 1) = 0;
        return param_2;
      }
      if (plVar11 ==
          *(longlong **)
           (*(longlong *)(param_1 + 6) + (uVar9 & *(ulonglong *)(param_1 + 0xc)) * 0x10)) break;
      plVar11 = (longlong *)plVar11[1];
      iVar2 = (int)plVar11[2];
    }
  }
  if (*(longlong *)(param_1 + 4) == 0x7ffffffffffffff) {
                    /* WARNING: Subroutine does not return */
    FUN_142ed3068("unordered_map/set too long");
  }
  plVar6 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x20,param_3,param_4,pfVar1,0);
  *(undefined4 *)(plVar6 + 2) = *(undefined4 *)param_3;
  param_4 = (longlong *)*param_4;
  plVar6[3] = (longlong)param_4;
  if (param_4 != (longlong *)0x0) {
    (**(code **)(*param_4 + 8))();
  }
  lVar7 = *(longlong *)(param_1 + 4);
  uVar3 = *(ulonglong *)(param_1 + 0xe);
  if (*param_1 < (float)(lVar7 + 1) / (float)uVar3) {
    fVar12 = ceilf((float)(lVar7 + 1) / *param_1);
    lVar7 = 0;
    if ((_DAT_143275074 <= fVar12) && (fVar12 = fVar12 - _DAT_143275074, fVar12 < _DAT_143275074)) {
      lVar7 = -0x8000000000000000;
    }
    uVar8 = 8;
    if (8 < (ulonglong)((longlong)fVar12 + lVar7)) {
      uVar8 = (longlong)fVar12 + lVar7;
    }
    uVar10 = uVar3;
    if ((uVar3 < uVar8) && ((0x1ff < uVar3 || (uVar10 = uVar3 * 8, uVar3 * 8 < uVar8)))) {
      uVar10 = uVar8;
    }
    FUN_14040fef0(param_1,uVar10);
    plVar4 = *(longlong **)
              (*(longlong *)(param_1 + 6) + 8 + (uVar9 & *(ulonglong *)(param_1 + 0xc)) * 0x10);
    plVar11 = *(longlong **)pfVar1;
    if (plVar4 != plVar11) {
      iVar2 = (int)plVar4[2];
      plVar11 = plVar4;
      while ((int)plVar6[2] != iVar2) {
        if (plVar11 ==
            *(longlong **)
             (*(longlong *)(param_1 + 6) + (uVar9 & *(ulonglong *)(param_1 + 0xc)) * 0x10))
        goto LAB_1403f1ff3;
        plVar11 = (longlong *)plVar11[1];
        iVar2 = (int)plVar11[2];
      }
      plVar11 = (longlong *)*plVar11;
    }
LAB_1403f1ff3:
    lVar7 = *(longlong *)(param_1 + 4);
  }
  plVar4 = (longlong *)plVar11[1];
  *(longlong *)(param_1 + 4) = lVar7 + 1;
  *plVar6 = (longlong)plVar11;
  plVar6[1] = (longlong)plVar4;
  *plVar4 = (longlong)plVar6;
  plVar11[1] = (longlong)plVar6;
  lVar7 = *(longlong *)(param_1 + 6);
  uVar9 = uVar9 & *(ulonglong *)(param_1 + 0xc);
  plVar5 = *(longlong **)(lVar7 + uVar9 * 0x10);
  if (plVar5 == *(longlong **)pfVar1) {
    *(longlong **)(lVar7 + uVar9 * 0x10) = plVar6;
  }
  else {
    if (plVar5 == plVar11) {
      *(longlong **)(lVar7 + uVar9 * 0x10) = plVar6;
      goto LAB_1403f2063;
    }
    if (*(longlong **)(lVar7 + 8 + uVar9 * 0x10) != plVar4) goto LAB_1403f2063;
  }
  *(longlong **)(lVar7 + 8 + uVar9 * 0x10) = plVar6;
LAB_1403f2063:
  *param_2 = (longlong)plVar6;
  *(undefined1 *)(param_2 + 1) = 1;
  return param_2;
}



//===========================================================
// FUN_142ed308c @ 142ed308c   (34 bytes)
//===========================================================

void FUN_142ed308c(undefined8 param_1)

{
  undefined1 local_28 [40];
  
  FUN_1411334d0(local_28,param_1);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_28,(ThrowInfo *)&DAT_143a3bb80);
}



//===========================================================
// FUN_14019d350 @ 14019d350   (105 bytes)
//===========================================================

longlong * FUN_14019d350(longlong param_1,ulonglong param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong *plVar3;
  
  if (0xc7fffff < param_2) {
    FUN_142e541f0(0x3a);
  }
  pcVar1 = DAT_143ad5528;
  uVar2 = (*DAT_143ad5538)();
  plVar3 = (longlong *)(*pcVar1)(uVar2,0,param_1 + 8);
  if (plVar3 != (longlong *)0x0) {
    *plVar3 = param_1;
    return plVar3 + 1;
  }
  return (longlong *)0x0;
}



//===========================================================
// FUN_14019d3c0 @ 14019d3c0   (144 bytes)
//===========================================================

undefined8 * FUN_14019d3c0(longlong param_1,longlong param_2)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  
  puVar3 = (undefined8 *)FUN_14019d350((param_1 + 8) * param_2 + 8,param_1 * param_2);
  *puVar3 = 0;
  puVar1 = puVar3 + 2;
  puVar3[1] = param_1;
  param_2 = param_2 + -1;
  puVar3 = puVar1;
  if (param_2 == 0) {
    *puVar1 = 0;
    return puVar1;
  }
  do {
    puVar2 = (undefined8 *)((longlong)puVar3 + param_1 + 8);
    *puVar3 = puVar2;
    puVar2[-1] = param_1;
    param_2 = param_2 + -1;
    puVar3 = puVar2;
  } while (param_2 != 0);
  *puVar2 = 0;
  return puVar1;
}



//===========================================================
// thunk_FUN_140205820 @ 142ef3bb8   (5 bytes)
//===========================================================

void thunk_FUN_140205820(undefined8 param_1)

{
  FUN_14019bb50(&DAT_143ad68a0,param_1);
  return;
}



//===========================================================
// FUN_14019b4e0 @ 14019b4e0   (288 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019b4e0(undefined8 *param_1)

{
  code *pcVar1;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  int *piVar6;
  longlong lVar7;
  bool bVar8;
  
  pvVar2 = Self;
  pcVar1 = DAT_143ad5530;
  if (param_1 == (undefined8 *)0x0) {
    return;
  }
  uVar3 = param_1[-1];
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x21) {
    uVar5 = (ulonglong)(0x10 < uVar3);
  }
  else {
    if (uVar3 < 0x41) {
      uVar5 = 2;
      goto LAB_14019b52b;
    }
    uVar5 = 0xffffffff;
    if (uVar3 < 0x81) {
      uVar5 = 3;
    }
  }
  if ((int)uVar5 < 0) {
    uVar4 = (*DAT_143ad5538)();
    (*pcVar1)(uVar4,0,param_1 + -1);
    return;
  }
LAB_14019b52b:
  lVar7 = uVar5 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad68c8 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad68c8 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019b5b5:
    *(undefined4 *)(&DAT_143ad68d0 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad68c8 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad68c8 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad68c8 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019b5b5;
        if (*(void **)(&DAT_143ad68c8 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad68d0 + lVar7) = *(int *)(&DAT_143ad68d0 + lVar7) + 1;
  }
  piVar6 = (int *)(&DAT_143ad68d0 + lVar7);
  *param_1 = *(undefined8 *)(&DAT_143ad6908 + uVar5 * 8);
  *(undefined8 **)(&DAT_143ad6908 + uVar5 * 8) = param_1;
  _DAT_143ad6948 = *param_1;
  *(int *)(&DAT_143ad68b4 + uVar5 * 4) = *(int *)(&DAT_143ad68b4 + uVar5 * 4) + -1;
  *piVar6 = *piVar6 + -1;
  if (*piVar6 == 0) {
    *(undefined8 *)(&DAT_143ad68c8 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_14091a3e0 @ 14091a3e0   (62 bytes)
//===========================================================

uint FUN_14091a3e0(undefined8 *param_1)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  ulonglong uVar4;
  
  pbVar3 = (byte *)*param_1;
  if (pbVar3 != (byte *)0x0) {
    uVar2 = 0x811c9dc5;
    if (*(uint *)(pbVar3 + -8) != 0) {
      uVar4 = (ulonglong)*(uint *)(pbVar3 + -8);
      do {
        bVar1 = *pbVar3;
        pbVar3 = pbVar3 + 1;
        uVar2 = (bVar1 ^ uVar2) * 0x1000193;
        uVar4 = uVar4 - 1;
      } while (uVar4 != 0);
    }
    return uVar2;
  }
  return 0x811c9dc5;
}



//===========================================================
// FUN_142e5cd30 @ 142e5cd30   (764 bytes)
//===========================================================

void FUN_142e5cd30(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8)

{
  undefined8 uVar1;
  undefined1 local_84;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  longlong local_38;
  longlong local_30;
  longlong local_28;
  longlong local_20;
  
  uVar1 = FUN_142a1d8a0(&local_20);
  local_80 = 0;
  FUN_140198700(&local_80,uVar1,local_84);
  local_58 = 0;
  uVar1 = FUN_14019ba10(&local_58,&DAT_143272338,param_1);
  local_78 = 0;
  FUN_14019a260(&local_78,uVar1);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_78);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  local_50 = 0;
  uVar1 = FUN_14019ba10(&local_50,&DAT_143272338,param_2);
  local_70 = 0;
  FUN_14019a260(&local_70,uVar1);
  if (local_50 != 0) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_70);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  FUN_1408bc140(&local_48,param_3);
  FUN_1401a1c50(&local_80,&local_48);
  if (local_48 != 0) {
    FUN_14019f2c0(local_48 + -0x10);
  }
  local_40 = 0;
  uVar1 = FUN_14019ba10(&local_40,&DAT_143272338,param_4);
  local_68 = 0;
  FUN_14019a260(&local_68,uVar1);
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_68);
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  FUN_1408bc020(&local_38,param_5);
  FUN_1401a1c50(&local_80,&local_38);
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
  local_30 = 0;
  uVar1 = FUN_14019ba10(&local_30,&DAT_143272338,param_6);
  local_60 = 0;
  FUN_14019a260(&local_60,uVar1);
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_60);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  FUN_1408bc280(&local_28,param_7);
  FUN_1401a1c50(&local_80,&local_28);
  if (local_28 != 0) {
    FUN_14019f2c0(local_28 + -0x10);
  }
  FUN_140198700(&local_80,param_8,local_84);
  FUN_142a1ec10(&local_80);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  if (local_20 != 0) {
    FUN_14019f2c0(local_20 + -0x10);
  }
  return;
}



//===========================================================
// FUN_14019f2c0 @ 14019f2c0   (345 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019f2c0(int *param_1)

{
  int iVar1;
  code *UNRECOVERED_JUMPTABLE;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  int *piVar5;
  uint uVar6;
  longlong lVar7;
  bool bVar8;
  
  if (0x100000 < *param_1 + 1U) {
    FUN_142e52dd0(0x23);
  }
  LOCK();
  iVar1 = *param_1;
  *param_1 = *param_1 + -1;
  UNLOCK();
  if (1 < iVar1) {
    return;
  }
  if (*param_1 != 0) {
    FUN_142e52dd0(0x32);
  }
  pvVar2 = Self;
  UNRECOVERED_JUMPTABLE = DAT_143ad5530;
  uVar3 = *(ulonglong *)(param_1 + -2);
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x39) {
    uVar6 = (uint)(0x28 < uVar3);
  }
  else {
    if (uVar3 < 0x59) {
      uVar6 = 2;
      goto LAB_14019f332;
    }
    uVar6 = 0xffffffff;
    if (uVar3 < 0x99) {
      uVar6 = 3;
    }
  }
  if ((int)uVar6 < 0) {
    uVar4 = (*DAT_143ad5538)();
                    /* WARNING: Could not recover jumptable at 0x00014019f3a6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(uVar4,0,param_1 + -2);
    return;
  }
LAB_14019f332:
  uVar3 = (ulonglong)uVar6;
  lVar7 = (ulonglong)uVar6 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad6a58 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019f3d5:
    *(undefined4 *)(&DAT_143ad6a60 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad6a58 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad6a58 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019f3d5;
        if (*(void **)(&DAT_143ad6a58 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad6a60 + lVar7) = *(int *)(&DAT_143ad6a60 + lVar7) + 1;
  }
  piVar5 = (int *)(&DAT_143ad6a60 + lVar7);
  *(undefined8 *)param_1 = *(undefined8 *)(&DAT_143ad6a98 + uVar3 * 8);
  *(int **)(&DAT_143ad6a98 + uVar3 * 8) = param_1;
  _DAT_143ad6ad8 = *(undefined8 *)param_1;
  *(int *)(&DAT_143ad6a44 + uVar3 * 4) = *(int *)(&DAT_143ad6a44 + uVar3 * 4) + -1;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *(undefined8 *)(&DAT_143ad6a58 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_142e559e0 @ 142e559e0   (127 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_142e559e0(void)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  
  iVar2 = FUN_14090d160(0x116,1);
  if (iVar2 != 0) {
    cVar1 = FUN_14090d340(0x11a);
    if (cVar1 != '\0') {
      _DAT_00000000 = 1;
    }
    if (DAT_143ae1514 < 3) {
      DAT_143ae1514 = DAT_143ae1514 + 1;
      return 1;
    }
    uVar3 = (*DAT_143262db0)();
    cVar1 = FUN_1408fcaa0(DAT_143ae1548,1800000,uVar3);
    if (cVar1 != '\0') {
      DAT_143ae1548 = uVar3;
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_140194c60 @ 140194c60   (6267 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 * FUN_140194c60(undefined8 *param_1)

{
  code *pcVar1;
  undefined8 uVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  longlong *plVar7;
  int *piVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulonglong *puVar11;
  undefined4 *puVar12;
  longlong lVar13;
  longlong lVar14;
  char *pcVar15;
  undefined8 uVar16;
  int iVar17;
  int *piVar18;
  int *piVar19;
  int *piVar20;
  ulonglong uVar21;
  undefined8 *puVar22;
  int iVar23;
  int *piVar24;
  undefined1 auStack_aa8 [32];
  int **local_a88;
  int **local_a80;
  undefined8 local_a78;
  undefined8 local_a70;
  undefined8 local_a68;
  int *local_a58;
  longlong local_a50;
  undefined8 *local_a48;
  ulonglong local_a40;
  int local_a38;
  ulonglong local_a30;
  longlong local_a28;
  undefined8 *local_a20;
  undefined4 local_a18 [2];
  undefined8 local_a10;
  undefined8 uStack_a08;
  undefined8 local_a00;
  undefined8 uStack_9f8;
  undefined8 local_9e8;
  undefined1 local_9e0 [4];
  undefined4 local_9dc;
  longlong local_9c8;
  undefined4 local_9bc;
  undefined8 local_9b8;
  undefined4 local_9ac;
  undefined4 local_8d8;
  undefined8 local_8d4;
  undefined8 uStack_8cc;
  undefined8 local_8c4;
  undefined8 uStack_8bc;
  undefined8 local_8b4;
  undefined8 uStack_8ac;
  undefined8 local_8a4;
  undefined8 uStack_89c;
  undefined8 local_894;
  undefined8 uStack_88c;
  undefined8 local_884;
  undefined8 uStack_87c;
  undefined8 local_874;
  undefined8 uStack_86c;
  undefined8 local_864;
  undefined8 uStack_85c;
  undefined8 local_854;
  undefined8 uStack_84c;
  undefined1 local_838 [48];
  undefined4 local_808;
  undefined8 local_7a0;
  longlong local_798;
  undefined8 local_740;
  int *local_368 [34];
  undefined4 local_258 [6];
  undefined4 local_240;
  undefined1 local_23c [516];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_aa8;
  piVar24 = (int *)0x0;
  iVar3 = 0;
  local_a58 = (int *)0x0;
  local_a48 = param_1;
  local_a20 = param_1;
  if (DAT_143aa8288 == '\0') {
    local_a50 = 0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a50,"Not Init\r\n");
    lVar14 = *plVar7;
    piVar18 = piVar24;
    if (lVar14 == 0) goto LAB_140194d86;
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    param_1 = local_a20;
    if (iVar4 == 0) goto LAB_140194d86;
    if (0 < iVar4) {
      iVar3 = iVar4;
    }
    piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
    piVar8[1] = iVar3;
    *piVar8 = -1;
    piVar18 = piVar8 + 4;
    piVar8[2] = 0;
    *(char *)piVar18 = '\0';
    local_a58 = piVar18;
    FUN_142ef7ba0(piVar18,lVar14,piVar20);
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
      *piVar8 = 1;
      if (iVar4 != -1) goto LAB_140194d5f;
      piVar20 = (int *)0xffffffffffffffff;
      if (piVar18 != (int *)0x0) {
        do {
          piVar24 = (int *)((longlong)piVar20 + 1);
          piVar20 = piVar24;
        } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar8[1],iVar4);
      *piVar8 = 1;
LAB_140194d5f:
      *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      piVar24 = piVar20;
    }
    iVar3 = (int)piVar24;
    if ((iVar3 < 0) || (piVar8[1] + 1 <= iVar3)) {
      FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
    }
    piVar8[2] = iVar3;
    param_1 = local_a20;
LAB_140194d86:
    if (local_a50 != 0) {
      FUN_14019f2c0(local_a50 + -0x10);
    }
    *param_1 = piVar18;
    return param_1;
  }
  FUN_142ef8250(local_838,0,0x4d0);
  local_808 = 0x10001f;
  (*DAT_143262840)(local_838);
  local_a50 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a50,&DAT_143271d00);
  lVar14 = *plVar7;
  piVar18 = piVar24;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    piVar18 = (int *)0x0;
    if (iVar4 != 0) {
      iVar6 = 0;
      if (0 < iVar4) {
        iVar6 = iVar4;
      }
      piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      piVar8[1] = iVar6;
      *piVar8 = -1;
      piVar18 = piVar8 + 4;
      piVar8[2] = 0;
      *(char *)piVar18 = '\0';
      local_a58 = piVar18;
      FUN_142ef7ba0(piVar18,lVar14,piVar20);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar4 != -1) goto LAB_140194ea0;
        piVar20 = piVar24;
        if (piVar18 != (int *)0x0) {
          piVar20 = (int *)0xffffffffffffffff;
          do {
            piVar20 = (int *)((longlong)piVar20 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar20) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],iVar4);
        *piVar8 = 1;
LAB_140194ea0:
        *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      }
      iVar4 = (int)piVar20;
      if ((iVar4 < 0) || (piVar8[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,(ulonglong)piVar20 & 0xffffffff);
      }
      piVar8[2] = iVar4;
    }
  }
  if (local_a50 != 0) {
    FUN_14019f2c0(local_a50 + -0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Major:%d Minor:%d\r\n",1);
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar4;
    if (iVar4 != 0) {
      piVar19 = piVar24;
      if (piVar18 == (int *)0x0) goto LAB_14019509b;
      if ((char)*piVar18 != '\0') {
        iVar6 = piVar18[-2];
        for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar18 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140194f9f:
          if (iVar3 < iVar17) {
            iVar3 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          local_a58 = piVar20;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar20 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar18[-2] + 1;
            iVar23 = iVar3 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
            puVar12[2] = piVar18[-2];
            *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
            iVar3 = piVar18[-2];
            goto LAB_140194f9f;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar20 == (int *)0x0) {
          iVar3 = 0;
        }
        else {
          iVar3 = piVar20[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar4);
        goto LAB_140195140;
      }
      if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019509b:
        if (iVar3 < iVar4) {
          iVar3 = iVar4;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
        puVar12[1] = iVar3;
        *puVar12 = 0xffffffff;
        piVar20 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        local_a58 = piVar20;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
          iVar3 = piVar18[-2];
          goto LAB_14019509b;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar20,lVar14,piVar8);
      if (piVar20[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
        piVar20[-4] = 1;
        if (iVar4 != -1) goto LAB_14019511d;
        if (piVar20 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar20[-3],iVar4);
        piVar20[-4] = 1;
LAB_14019511d:
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar20[-2] = iVar3;
    }
  }
LAB_140195140:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Call stack:\r\n");
  lVar14 = *plVar7;
  piVar18 = piVar20;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar3 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar3;
    if (iVar3 != 0) {
      iVar4 = 0;
      piVar19 = piVar24;
      if (piVar20 == (int *)0x0) goto LAB_14019530e;
      if ((char)*piVar20 != '\0') {
        iVar6 = piVar20[-2];
        for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar20 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140195212:
          if (iVar4 < iVar17) {
            iVar4 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          local_a58 = piVar18;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar20[-2] + 1;
            iVar23 = iVar4 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
            puVar12[2] = piVar20[-2];
            *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
            iVar4 = piVar20[-2];
            goto LAB_140195212;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar18 == (int *)0x0) {
          iVar4 = 0;
        }
        else {
          iVar4 = piVar18[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar3);
        goto LAB_1401953b3;
      }
      if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_14019530e:
        if (iVar4 < iVar3) {
          iVar4 = iVar3;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
        puVar12[1] = iVar4;
        *puVar12 = 0xffffffff;
        piVar18 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar18 = '\0';
        local_a58 = piVar18;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
          iVar4 = piVar20[-2];
          goto LAB_14019530e;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar18,lVar14,piVar8);
      if (piVar18[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
        piVar18[-4] = 1;
        if (iVar3 != -1) goto LAB_140195390;
        if (piVar18 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar18[-3],iVar3);
        piVar18[-4] = 1;
LAB_140195390:
        *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar18[-2] = iVar3;
    }
  }
LAB_1401953b3:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Address   Frame\r\n");
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 == 0) goto LAB_140195630;
  iVar3 = *(int *)(lVar14 + -8);
  piVar8 = (int *)(longlong)iVar3;
  if (iVar3 == 0) goto LAB_140195630;
  iVar4 = 0;
  piVar19 = piVar24;
  if (piVar18 == (int *)0x0) goto LAB_14019558b;
  if ((char)*piVar18 != '\0') {
    iVar6 = piVar18[-2];
    for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
    }
    piVar24 = piVar18 + -4;
    if (piVar24 == (int *)0x0) {
LAB_14019548f:
      if (iVar4 < iVar17) {
        iVar4 = iVar17;
      }
      puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
      puVar12[1] = iVar4;
      *puVar12 = 0xffffffff;
      piVar20 = puVar12 + 4;
      local_a58 = piVar20;
      if (piVar24 == (int *)0x0) {
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        lVar14 = local_a50;
      }
      else {
        iVar17 = piVar18[-2] + 1;
        iVar23 = iVar4 + 1;
        if (iVar23 < iVar17) {
          FUN_142e54290(0x5c,iVar17,iVar23);
          iVar17 = iVar23;
        }
        FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
        puVar12[2] = piVar18[-2];
        *(char *)((longlong)iVar4 + (longlong)piVar20) = '\0';
        FUN_14019f2c0(piVar24);
        lVar14 = local_a50;
      }
    }
    else {
      if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
        iVar4 = piVar18[-2];
        goto LAB_14019548f;
      }
      if (*piVar24 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar24 = -1;
    }
    if (piVar20 == (int *)0x0) {
      iVar4 = 0;
    }
    else {
      iVar4 = piVar20[-2];
    }
    FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar20),lVar14,piVar8);
    FUN_14019c870(&local_a58,iVar6 + iVar3);
    goto LAB_140195630;
  }
  if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019558b:
    if (iVar4 < iVar3) {
      iVar4 = iVar3;
    }
    puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
    puVar12[1] = iVar4;
    *puVar12 = 0xffffffff;
    piVar20 = puVar12 + 4;
    puVar12[2] = 0;
    *(char *)piVar20 = '\0';
    local_a58 = piVar20;
    if (piVar19 != (int *)0x0) {
      FUN_14019f2c0(piVar19);
    }
  }
  else {
    if ((1 < *piVar19) || (piVar18[-3] < iVar3)) {
      iVar4 = piVar18[-2];
      goto LAB_14019558b;
    }
    if (*piVar19 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar19 = -1;
  }
  FUN_142ef7ba0(piVar20,lVar14,piVar8);
  if (piVar20[-4] != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar3 == -1) || (iVar3 <= piVar20[-3])) {
    piVar20[-4] = 1;
    if (iVar3 != -1) goto LAB_14019560d;
    if (piVar20 != (int *)0x0) {
      piVar24 = (int *)0xffffffffffffffff;
      do {
        piVar24 = (int *)((longlong)piVar24 + 1);
      } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
    }
  }
  else {
    FUN_142e54290(0x90,piVar20[-3],iVar3);
    piVar20[-4] = 1;
LAB_14019560d:
    *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
    piVar24 = piVar8;
  }
  iVar3 = (int)piVar24;
  if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
    FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
  }
  piVar20[-2] = iVar3;
LAB_140195630:
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  FUN_142ef8250(local_9e0,0,0x100);
  uVar2 = DAT_143aa8260;
  uVar16 = DAT_143aa8258;
  pcVar1 = DAT_143aa8250;
  local_9e8 = local_740;
  local_9dc = 3;
  local_9b8 = local_7a0;
  local_9ac = 3;
  local_9c8 = local_798;
  local_9bc = 3;
  uVar9 = (*DAT_143ad5440)();
  uVar10 = (*DAT_143ad5408)();
  local_a68 = 0;
  local_a70 = uVar2;
  local_a78 = uVar16;
  local_a80 = (int **)0x0;
  local_a88 = (int **)local_838;
  iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  do {
    if ((iVar3 == 0) || (piVar24 = (int *)0x0, local_9c8 == 0)) {
      *local_a20 = piVar20;
      return local_a20;
    }
    local_a30 = 0;
    puVar11 = (ulonglong *)FUN_14019ba10(&local_a30,"%016X  %016X  ",local_9e8);
    uVar21 = *puVar11;
    piVar18 = piVar20;
    local_a40 = uVar21;
    if (uVar21 != 0) {
      iVar3 = *(int *)(uVar21 - 8);
      piVar8 = (int *)(longlong)iVar3;
      if (iVar3 != 0) {
        iVar4 = 0;
        piVar19 = piVar24;
        if (piVar20 == (int *)0x0) goto LAB_1401958b6;
        if ((char)*piVar20 != '\0') {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar20 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401957c2:
            if (iVar4 < iVar17) {
              iVar4 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
            puVar12[1] = iVar4;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            local_a58 = piVar18;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar18 = '\0';
              uVar21 = local_a40;
            }
            else {
              iVar17 = piVar20[-2] + 1;
              iVar23 = iVar4 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
              puVar12[2] = piVar20[-2];
              *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
              FUN_14019f2c0(piVar24);
              uVar21 = local_a40;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
              iVar4 = piVar20[-2];
              goto LAB_1401957c2;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          if (piVar18 == (int *)0x0) {
            iVar4 = 0;
          }
          else {
            iVar4 = piVar18[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),uVar21,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar3);
          goto LAB_14019595b;
        }
        if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_1401958b6:
          if (iVar4 < iVar3) {
            iVar4 = iVar3;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar18 = '\0';
          local_a58 = piVar18;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
            iVar4 = piVar20[-2];
            goto LAB_1401958b6;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar18,uVar21,piVar8);
        if (piVar18[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
          piVar18[-4] = 1;
          if (iVar3 != -1) goto LAB_140195938;
          if (piVar18 != (int *)0x0) {
            piVar24 = (int *)0xffffffffffffffff;
            do {
              piVar24 = (int *)((longlong)piVar24 + 1);
            } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar18[-3],iVar3);
          piVar18[-4] = 1;
LAB_140195938:
          *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
          piVar24 = piVar8;
        }
        iVar3 = (int)piVar24;
        if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
        }
        piVar18[-2] = iVar3;
      }
    }
LAB_14019595b:
    piVar24 = (int *)0x0;
    if (local_a30 != 0) {
      FUN_14019f2c0(local_a30 - 0x10);
    }
    local_258[0] = 0x20;
    local_240 = 0x200;
    local_a50 = 0;
    FUN_142ef8250(local_368,0,0x104);
    iVar3 = 0;
    local_a40 = local_a40 & 0xffffffff00000000;
    local_a30 = local_a30 & 0xffffffff00000000;
    local_a18[0] = 0x28;
    local_a10 = 0;
    local_a00 = 0;
    uStack_9f8 = 0;
    uStack_a08 = 0xffffffff;
    if (DAT_143aa8280 == 0) {
      local_8d8 = 0x94;
      local_8d4 = 0;
      uStack_8cc = 0;
      local_8c4 = 0;
      uStack_8bc = 0;
      local_8b4 = 0;
      uStack_8ac = 0;
      local_8a4 = 0;
      uStack_89c = 0;
      local_894 = 0;
      uStack_88c = 0;
      local_884 = 0;
      uStack_87c = 0;
      local_874 = 0;
      uStack_86c = 0;
      local_864 = 0;
      uStack_85c = 0;
      local_854 = 0;
      uStack_84c = 0;
      (*DAT_143262820)(&local_8d8);
      if (uStack_8cc._4_4_ == 2) {
        DAT_143aa8280 = (*DAT_143ad5408)();
      }
      else {
        iVar4 = (*DAT_143ad5410)();
        DAT_143aa8280 = (longlong)iVar4;
      }
      if (DAT_143aa8280 == 0) {
        local_a28 = 0;
        plVar7 = (longlong *)FUN_14019ba10(&local_a28,"m_hProcess is Null\r\n");
        puVar22 = (undefined8 *)*plVar7;
        piVar20 = piVar18;
        local_a48 = puVar22;
        if (puVar22 == (undefined8 *)0x0) goto LAB_140196462;
        iVar4 = *(int *)(puVar22 + -1);
        piVar8 = (int *)(longlong)iVar4;
        if (iVar4 == 0) goto LAB_140196462;
        piVar19 = piVar24;
        if (piVar18 == (int *)0x0) goto LAB_1401963be;
        if ((char)*piVar18 != '\0') {
          iVar6 = piVar18[-2];
          for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar18 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401962c5:
            if (iVar3 < iVar17) {
              iVar3 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar20 = puVar12 + 4;
            local_a58 = piVar20;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar20 = '\0';
              puVar22 = local_a48;
            }
            else {
              iVar17 = piVar18[-2] + 1;
              iVar23 = iVar3 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
              puVar12[2] = piVar18[-2];
              *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
              FUN_14019f2c0(piVar24);
              puVar22 = local_a48;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
              iVar3 = piVar18[-2];
              goto LAB_1401962c5;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          iVar3 = 0;
          if (piVar20 != (int *)0x0) {
            iVar3 = piVar20[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),puVar22,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
          goto LAB_140196462;
        }
        if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_1401963be:
          if (iVar3 < iVar4) {
            iVar3 = iVar4;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar20 = '\0';
          local_a58 = piVar20;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
            iVar3 = piVar18[-2];
            goto LAB_1401963be;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar20,puVar22,piVar8);
        if (piVar20[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
          piVar20[-4] = 1;
          if (iVar4 == -1) {
            piVar18 = (int *)0xffffffffffffffff;
            if (piVar20 != (int *)0x0) {
              do {
                piVar24 = (int *)((longlong)piVar18 + 1);
                piVar18 = piVar24;
              } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
            }
LAB_140196443:
            iVar3 = (int)piVar24;
            if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
              FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
            }
            piVar20[-2] = iVar3;
LAB_140196462:
            if (local_a28 != 0) {
              FUN_14019f2c0(local_a28 + -0x10);
            }
            *local_a20 = piVar20;
            return local_a20;
          }
        }
        else {
          FUN_142e54290(0x90,piVar20[-3],iVar4);
          piVar20[-4] = 1;
        }
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
        goto LAB_140196443;
      }
    }
    iVar4 = (*DAT_143aa8268)(DAT_143aa8280,local_9e8,&local_a50,local_258);
    local_a38 = iVar4;
    if (iVar4 == 0) {
      uVar5 = (*DAT_143262838)();
      local_a48 = (undefined8 *)0x0;
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"_SymGetLineFromAddr Error : %x ",uVar5);
      lVar14 = *plVar7;
      local_a28 = lVar14;
      if (lVar14 != 0) {
        iVar6 = *(int *)(lVar14 + -8);
        piVar20 = (int *)(longlong)iVar6;
        iVar4 = local_a38;
        if (iVar6 != 0) {
          piVar8 = piVar24;
          if (piVar18 == (int *)0x0) goto LAB_140195c6d;
          if ((char)*piVar18 != '\0') {
            iVar4 = piVar18[-2];
            for (iVar17 = piVar18[-3]; iVar17 < iVar4 + iVar6; iVar17 = iVar17 * 2) {
            }
            piVar24 = piVar18 + -4;
            if (piVar24 == (int *)0x0) {
LAB_140195b6f:
              if (iVar3 < iVar17) {
                iVar3 = iVar17;
              }
              puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
              puVar12[1] = iVar3;
              *puVar12 = 0xffffffff;
              piVar8 = puVar12 + 4;
              local_a58 = piVar8;
              if (piVar24 == (int *)0x0) {
                puVar12[2] = 0;
                *(char *)piVar8 = '\0';
                lVar14 = local_a28;
              }
              else {
                iVar17 = piVar18[-2] + 1;
                iVar23 = iVar3 + 1;
                if (iVar23 < iVar17) {
                  FUN_142e54290(0x5c,iVar17,iVar23);
                  iVar17 = iVar23;
                }
                FUN_142ef7ba0(piVar8,piVar18,(longlong)iVar17);
                puVar12[2] = piVar18[-2];
                *(char *)((longlong)iVar3 + (longlong)piVar8) = '\0';
                FUN_14019f2c0(piVar24);
                lVar14 = local_a28;
              }
            }
            else {
              if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
                iVar3 = piVar18[-2];
                goto LAB_140195b6f;
              }
              if (*piVar24 != 1) {
                FUN_142e52dd0(0x74);
              }
              *piVar24 = -1;
              piVar8 = piVar18;
            }
            if (piVar8 == (int *)0x0) {
              iVar3 = 0;
            }
            else {
              iVar3 = piVar8[-2];
            }
            FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar8),lVar14,piVar20);
            FUN_14019c870(&local_a58,iVar4 + iVar6);
            iVar4 = local_a38;
            goto LAB_140195d1d;
          }
          if ((piVar18 == (int *)0x0) || (piVar8 = piVar18 + -4, piVar8 == (int *)0x0)) {
LAB_140195c6d:
            if (iVar3 < iVar6) {
              iVar3 = iVar6;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            local_a58 = piVar18;
            if (piVar8 != (int *)0x0) {
              FUN_14019f2c0(piVar8);
            }
          }
          else {
            if ((1 < *piVar8) || (piVar18[-3] < iVar6)) {
              iVar3 = piVar18[-2];
              goto LAB_140195c6d;
            }
            if (*piVar8 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar8 = -1;
          }
          FUN_142ef7ba0(piVar18,lVar14,piVar20);
          if (piVar18[-4] != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar6 == -1) || (iVar6 <= piVar18[-3])) {
            piVar18[-4] = 1;
            if (iVar6 != -1) goto LAB_140195cf6;
            if (piVar18 != (int *)0x0) {
              piVar24 = (int *)0xffffffffffffffff;
              do {
                piVar24 = (int *)((longlong)piVar24 + 1);
              } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar18[-3],iVar6);
            piVar18[-4] = 1;
LAB_140195cf6:
            *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
            piVar24 = piVar20;
          }
          iVar3 = (int)piVar24;
          if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
            FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
          }
          piVar18[-2] = iVar3;
          iVar4 = local_a38;
        }
      }
LAB_140195d1d:
      if (local_a48 != (undefined8 *)0x0) {
        FUN_14019f2c0(local_a48 + -2);
      }
    }
    else if (DAT_143aa8270 != (code *)0x0) {
      (*DAT_143aa8270)(DAT_143aa8280,local_9e8,&local_a50,local_a18);
    }
    local_a80 = &local_a58;
    local_a88 = (int **)&local_a30;
    iVar6 = FUN_140194a90(local_9e8,local_368,0x104,&local_a40);
    local_a48 = (undefined8 *)0x0;
    iVar3 = 0;
    if ((int)uStack_a08 == -1) {
      if (iVar4 == 0) {
        local_a88 = local_368;
        plVar7 = (longlong *)
                 FUN_14019ba10(&local_a48,"%04X:%08X [%s]",local_a40 & 0xffffffff,
                               local_a30 & 0xffffffff);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
      else {
        local_a88 = local_368;
        plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs()+%X [%s]",local_23c,local_a50);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
    }
    else {
      local_a80 = local_368;
      local_a88 = (int **)CONCAT44(local_a88._4_4_,(int)uStack_a08);
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs() %hs(%lu) [%s]",local_23c,local_a00);
      lVar14 = *plVar7;
      piVar20 = local_a58;
      if (lVar14 != 0) {
        iVar4 = *(int *)(lVar14 + -8);
        if (iVar4 != 0) {
          if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
            uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
            FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar4);
            piVar20 = local_a58;
          }
          else {
            iVar17 = local_a58[-2];
            for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
            }
            lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
            piVar20 = local_a58;
            iVar23 = iVar3;
            if (local_a58 != (int *)0x0) {
              iVar23 = local_a58[-2];
            }
            FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar17 + iVar4);
          }
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    if (iVar6 == 0) {
      if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
        pcVar15 = (char *)FUN_14019bd40(&local_a58,0xc);
        *(undefined8 *)pcVar15 = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)(pcVar15 + 8) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,0xc);
        piVar20 = local_a58;
      }
      else {
        iVar4 = piVar20[-2];
        for (iVar6 = piVar20[-3]; iVar6 < iVar4 + 0xc; iVar6 = iVar6 * 2) {
        }
        lVar14 = FUN_14019bd40(&local_a58,iVar6,1);
        piVar20 = local_a58;
        iVar6 = iVar3;
        if (local_a58 != (int *)0x0) {
          iVar6 = local_a58[-2];
        }
        *(undefined8 *)(iVar6 + lVar14) = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)((longlong)iVar6 + 8 + lVar14) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,iVar4 + 0xc);
      }
    }
    local_a48 = (undefined8 *)0x0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a48,&DAT_143271d00);
    lVar14 = *plVar7;
    if (lVar14 != 0) {
      iVar4 = *(int *)(lVar14 + -8);
      if (iVar4 != 0) {
        if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
          uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
          FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar4);
          piVar20 = local_a58;
        }
        else {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          lVar13 = FUN_14019bd40(&local_a58,iVar17,1);
          piVar20 = local_a58;
          if (local_a58 != (int *)0x0) {
            iVar3 = local_a58[-2];
          }
          FUN_142ef7ba0(iVar3 + lVar13,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    uVar2 = DAT_143aa8260;
    uVar16 = DAT_143aa8258;
    pcVar1 = DAT_143aa8250;
    uVar9 = (*DAT_143ad5440)();
    uVar10 = (*DAT_143ad5408)();
    local_a68 = 0;
    local_a70 = uVar2;
    local_a78 = uVar16;
    local_a80 = (int **)0x0;
    local_a88 = (int **)local_838;
    iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  } while( true );
}


