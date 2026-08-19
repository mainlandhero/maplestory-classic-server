
//===========================================================
// FUN_140495990 @ 140495990   (1668 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000140495e69) */
/* WARNING: Removing unreachable block (ram,0x000140495d96) */

longlong FUN_140495990(uint param_1)

{
  longlong lVar1;
  int iVar2;
  IUnknown *pIVar3;
  int *piVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined8 *puVar7;
  IUnknown *pIVar8;
  ulonglong uVar9;
  longlong lVar10;
  IUnknown *pIVar11;
  uint local_res8 [2];
  IUnknown *local_res10;
  longlong local_res18;
  longlong local_res20;
  undefined4 local_108;
  undefined4 uStack_104;
  undefined8 uStack_100;
  undefined8 local_f8;
  short local_f0;
  undefined6 uStack_ee;
  longlong lStack_e8;
  undefined8 local_e0;
  IUnknown *local_d8;
  IUnknown *local_d0;
  ulonglong local_c8;
  uint local_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined8 local_b0;
  longlong *local_a8;
  IUnknown *local_a0;
  uint local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 local_88;
  undefined8 local_78;
  longlong lStack_70;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  uVar9 = (ulonglong)(int)param_1;
  pIVar8 = (IUnknown *)0x0;
  local_res20 = 0;
  local_res8[0] = param_1;
  if (DAT_143a42350 != 0) {
    for (lVar6 = *(longlong *)(DAT_143a42350 + (uVar9 % (ulonglong)DAT_143a42358) * 8); lVar6 != 0;
        lVar6 = *(longlong *)(lVar6 + 8)) {
      if (*(uint *)(lVar6 + 0x10) == param_1) {
        if ((lVar6 != -0x18) &&
           (lVar6 = *(longlong *)(lVar6 + 0x20), local_res20 = lVar6, lVar6 != 0))
        goto LAB_140495f96;
        break;
      }
    }
  }
  lVar6 = local_res20;
  local_res18 = 0;
  FUN_1401c21c0(&local_res18,PTR_DAT_143a46b30,param_1);
  pIVar11 = DAT_143add058;
  local_d0 = (IUnknown *)0x0;
  local_d8 = DAT_143add058;
  if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_f0);
  if (DAT_143a8b8d8 == 8) {
    if (local_f0 == 8) {
      local_f0 = 0;
      if (lStack_e8 != 0) {
        (*DAT_143ad5990)(lStack_e8 + -4);
      }
    }
    else {
      iVar2 = (*DAT_143262a18)(&local_f0);
      if (iVar2 < 0) goto LAB_140495fff;
    }
    local_f0 = 8;
    pIVar3 = pIVar8;
    if (DAT_143a8b8e0 != 0) {
      pIVar3 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_e8 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar3);
  }
  else {
    if ((local_f0 == 8) && (local_f0 = 0, lStack_e8 != 0)) {
      (*DAT_143ad5990)(lStack_e8 + -4);
    }
    iVar2 = (*DAT_143262a28)(&local_f0,&DAT_143a8b8d8);
    if (iVar2 < 0) {
LAB_140495fff:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
  }
  (*DAT_143262a20)(&local_108);
  if (DAT_143a8b8d8 == 8) {
    if ((short)local_108 == 8) {
      local_108 = (uint)local_108._2_2_ << 0x10;
      if (uStack_100 != 0) {
        (*DAT_143ad5990)(uStack_100 + -4);
      }
    }
    else {
      iVar2 = (*DAT_143262a18)(&local_108);
      if (iVar2 < 0) goto LAB_140496007;
    }
    local_108 = CONCAT22(local_108._2_2_,8);
    pIVar3 = pIVar8;
    if (DAT_143a8b8e0 != 0) {
      pIVar3 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    uStack_100 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar3);
  }
  else {
    if (((short)local_108 == 8) && (local_108 = (uint)local_108._2_2_ << 0x10, uStack_100 != 0)) {
      (*DAT_143ad5990)(uStack_100 + -4);
    }
    iVar2 = (*DAT_143262a28)(&local_108,&DAT_143a8b8d8);
    if (iVar2 < 0) {
LAB_140496007:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
  }
  pIVar3 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
  lVar1 = local_res18;
  local_a0 = pIVar3;
  if (pIVar3 == (IUnknown *)0x0) {
    local_res10 = (IUnknown *)0x0;
  }
  else {
    *(undefined8 *)(pIVar3 + 8) = 0;
    *(undefined4 *)(pIVar3 + 0x10) = 1;
    local_res10 = pIVar3;
    if (local_res18 != 0) {
      lVar10 = -1;
      do {
        lVar10 = lVar10 + 1;
      } while (*(short *)(local_res18 + lVar10 * 2) != 0);
      local_c8 = (ulonglong)((int)lVar10 + 1);
      piVar4 = (int *)(*DAT_143ad5980)(local_c8 * 2 + 4);
      if (piVar4 == (int *)0x0) {
        *(undefined8 *)pIVar3 = 0;
      }
      else {
        *piVar4 = (int)lVar10 * 2;
        piVar4 = piVar4 + 1;
        FUN_142ef7ba0(piVar4,lVar1,local_c8 * 2);
        *(int **)pIVar3 = piVar4;
        pIVar11 = local_d8;
        if (piVar4 != (int *)0x0) goto LAB_140495c6b;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    *(undefined8 *)pIVar3 = 0;
  }
LAB_140495c6b:
  if (local_res10 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x8007000e);
  }
  local_a0 = (IUnknown *)&local_res10;
  (*DAT_143262a20)(&local_98);
  pIVar3 = pIVar8;
  if (local_res10 != (IUnknown *)0x0) {
    pIVar3 = *(IUnknown **)local_res10;
  }
  local_78 = CONCAT62(uStack_ee,local_f0);
  lStack_70 = lStack_e8;
  local_68 = local_e0;
  local_58 = local_108;
  uStack_54 = uStack_104;
  uStack_50 = (undefined4)uStack_100;
  uStack_4c = uStack_100._4_4_;
  local_48 = local_f8;
  iVar2 = (**(code **)(*(longlong *)pIVar11 + 0x48))(pIVar11,pIVar3,&local_58,&local_78,&local_98);
  if (iVar2 < 0) {
    _com_issue_errorex(iVar2,pIVar11,(_GUID *)&DAT_1432743e8);
  }
  local_c0 = local_98;
  uStack_bc = uStack_94;
  uStack_b8 = uStack_90;
  uStack_b4 = uStack_8c;
  local_b0 = local_88;
  local_98 = local_98 & 0xffff0000;
  FUN_1401be120(&local_res10);
  uVar5 = FUN_1409339d0(&local_a8,&local_c0);
  FUN_1401a5040(&local_d8,uVar5);
  if (local_d8 != (IUnknown *)0x0) {
    local_d0 = local_d8;
    pIVar8 = local_d8;
  }
  if (local_a8 != (longlong *)0x0) {
    (**(code **)(*local_a8 + 0x10))();
  }
  if ((short)local_c0 == 8) {
    local_c0 = local_c0 & 0xffff0000;
    if (CONCAT44(uStack_b4,uStack_b8) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_b4,uStack_b8) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_c0);
  }
  if ((short)local_108 == 8) {
    local_108 = local_108 & 0xffff0000;
    if (uStack_100 != 0) {
      (*DAT_143ad5990)(uStack_100 + -4);
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
  if (pIVar8 == (IUnknown *)0x0) {
    if (lVar1 != 0) {
      FUN_1401bebb0(lVar1 + -0x10);
    }
    lVar6 = 0;
  }
  else {
    local_res10 = pIVar8;
    if (pIVar8 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar8 + 8))(pIVar8);
    }
    iVar2 = FUN_14047d990(local_res8[0]);
    if (iVar2 == 0) {
      local_res10 = (IUnknown *)FUN_1418039d0(0x22000006);
      puVar7 = (undefined8 *)FUN_1404a2410(&local_res20,&local_res10,local_res8);
      FUN_141804870(&DAT_143271f04,0x81f,0x22000006,*puVar7);
      if (local_res20 != 0) {
        FUN_14019f2c0(local_res20 + -0x10);
      }
    }
    if (DAT_143a42350 == 0) {
      uVar9 = (ulonglong)local_res8[0];
    }
    else {
      uVar9 = (ulonglong)(int)local_res8[0];
      for (lVar10 = *(longlong *)(DAT_143a42350 + (uVar9 % (ulonglong)DAT_143a42358) * 8);
          lVar10 != 0; lVar10 = *(longlong *)(lVar10 + 8)) {
        if (*(uint *)(lVar10 + 0x10) == local_res8[0]) {
          if (lVar10 != -0x18) {
            lVar6 = *(longlong *)(lVar10 + 0x20);
          }
          break;
        }
      }
    }
    if (pIVar8 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar8 + 0x10))(pIVar8);
      uVar9 = (ulonglong)local_res8[0];
    }
    if (lVar1 != 0) {
      FUN_1401bebb0(lVar1 + -0x10);
      uVar9 = (ulonglong)local_res8[0];
    }
LAB_140495f96:
    lVar1 = DAT_143aa9c78;
    if (DAT_143aa9c78 != 0) {
      local_res10 = (IUnknown *)0x0;
      uVar5 = FUN_14019ba10(&local_res10,"Mob/%07d.img",uVar9 & 0xffffffff);
      FUN_1402170a0(lVar1,2,uVar5);
      if (local_res10 != (IUnknown *)0x0) {
        FUN_14019f2c0(local_res10 + -0x10);
      }
    }
  }
  return lVar6;
}



//===========================================================
// FUN_141c8a730 @ 141c8a730   (116 bytes)
//===========================================================

longlong FUN_141c8a730(longlong param_1)

{
  int iVar1;
  longlong lVar2;
  
  if ((*(longlong *)(param_1 + 0xe90) == 0) ||
     (lVar2 = *(longlong *)(*(longlong *)(param_1 + 0xe90) + 0x40), lVar2 == 0)) {
    if (*(longlong *)(param_1 + 0xa20) == 0) {
      lVar2 = FUN_14047a100(*(undefined8 *)(param_1 + 0x3a8));
    }
    else {
      lVar2 = *(longlong *)(*(longlong *)(param_1 + 0xa20) + 8);
    }
  }
  iVar1 = *(int *)(param_1 + 0x8c0);
  if ((iVar1 != 0) && (iVar1 != 100)) {
    lVar2 = SUB168(SEXT816(-0x5c28f5c28f5c28f5) * SEXT816(iVar1 * lVar2),8) + iVar1 * lVar2;
    lVar2 = (lVar2 >> 6) - (lVar2 >> 0x3f);
  }
  return lVar2;
}


