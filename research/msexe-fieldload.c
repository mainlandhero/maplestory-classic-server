
//===========================================================
// FUN_1415d60e0 @ 1415d60e0   (22 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */

void FUN_1415d60e0(void)

{
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}



//===========================================================
// FUN_1415d6a50 @ 1415d6a50   (23 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */

void FUN_1415d6a50(void)

{
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}



//===========================================================
// FUN_1403999e0 @ 1403999e0   (1765 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000140399df7) */

undefined8 * FUN_1403999e0(undefined8 param_1,undefined8 *param_2,uint param_3,undefined8 param_4)

{
  undefined *puVar1;
  longlong lVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  int ***pppiVar6;
  undefined8 uVar7;
  int *piVar8;
  longlong *plVar9;
  IUnknown *pIVar10;
  IUnknown *pIVar11;
  int **ppiVar12;
  longlong lVar13;
  IUnknown *pIVar14;
  int ***local_res8;
  undefined8 *local_res10;
  uint local_res18 [2];
  undefined8 local_res20;
  IUnknown *local_118;
  undefined4 local_110;
  undefined4 uStack_10c;
  undefined8 uStack_108;
  undefined8 local_100;
  short local_f8;
  undefined6 uStack_f6;
  longlong lStack_f0;
  undefined8 local_e8;
  longlong local_e0;
  IUnknown *local_d8;
  int ***local_d0;
  IUnknown *local_c8;
  IUnknown *local_c0;
  uint local_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 *local_98;
  uint local_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 local_80;
  undefined8 local_78;
  longlong lStack_70;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  local_98 = &local_res20;
  local_res10 = param_2;
  local_res18[0] = param_3;
  local_res20 = param_4;
  local_a0 = param_1;
  FUN_1403fcd90(&local_a0,&local_res8);
  if ((local_res8 == (int ***)0x0) || (*(char *)local_res8 == '\0')) {
    if (local_res8 != (int ***)0x0) {
      FUN_14019f2c0(local_res8 + -2);
    }
    pIVar11 = (IUnknown *)0x0;
    local_e0 = 0;
    FUN_140338540(DAT_143ac0178,local_res18);
    FUN_1401c21c0(&local_e0,PTR_u_Map_Map_Map_d__09d_img_143a45d78,
                  (ulonglong)local_res18[0] / 100000000);
    pIVar14 = DAT_143add058;
    uVar5 = 0;
    local_c0 = (IUnknown *)0x0;
    local_118 = DAT_143add058;
    if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_f8);
    if (DAT_143a8b8d8 == 8) {
      if (local_f8 == 8) {
        local_f8 = 0;
        if (lStack_f0 != 0) {
          (*DAT_143ad5990)(lStack_f0 + -4);
        }
      }
      else {
        iVar4 = (*DAT_143262a18)(&local_f8);
        if (iVar4 < 0) goto LAB_14039a0a8;
      }
      local_f8 = 8;
      pIVar10 = pIVar11;
      if (DAT_143a8b8e0 != 0) {
        pIVar10 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      lStack_f0 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar10);
    }
    else {
      if ((local_f8 == 8) && (local_f8 = 0, lStack_f0 != 0)) {
        (*DAT_143ad5990)(lStack_f0 + -4);
      }
      iVar4 = (*DAT_143262a28)(&local_f8,&DAT_143a8b8d8);
      if (iVar4 < 0) {
LAB_14039a0a8:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar4);
      }
    }
    (*DAT_143262a20)(&local_110);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_110 == 8) {
        local_110 = (uint)local_110._2_2_ << 0x10;
        if (uStack_108 != 0) {
          (*DAT_143ad5990)(uStack_108 + -4);
        }
      }
      else {
        iVar4 = (*DAT_143262a18)(&local_110);
        if (iVar4 < 0) goto LAB_14039a0b0;
      }
      local_110 = CONCAT22(local_110._2_2_,8);
      pIVar10 = pIVar11;
      if (DAT_143a8b8e0 != 0) {
        pIVar10 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_108 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar10);
    }
    else {
      if (((short)local_110 == 8) && (local_110 = (uint)local_110._2_2_ << 0x10, uStack_108 != 0)) {
        (*DAT_143ad5990)(uStack_108 + -4);
      }
      iVar4 = (*DAT_143262a28)(&local_110,&DAT_143a8b8d8);
      if (iVar4 < 0) {
LAB_14039a0b0:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar4);
      }
    }
    pppiVar6 = (int ***)FUN_14019b780(&DAT_143ad68a0,0x18);
    lVar2 = local_e0;
    local_d0 = pppiVar6;
    if (pppiVar6 == (int ***)0x0) {
      local_res8 = (int ***)0x0;
    }
    else {
      pppiVar6[1] = (int **)0x0;
      *(int *)(pppiVar6 + 2) = 1;
      local_res8 = pppiVar6;
      if (local_e0 == 0) {
        *pppiVar6 = (int **)0x0;
      }
      else {
        lVar13 = -1;
        do {
          lVar13 = lVar13 + 1;
        } while (*(short *)(local_e0 + lVar13 * 2) != 0);
        uVar3 = (int)lVar13 + 1;
        piVar8 = (int *)(*DAT_143ad5980)((ulonglong)uVar3 * 2 + 4);
        if (piVar8 == (int *)0x0) {
          *pppiVar6 = (int **)0x0;
LAB_14039a0b8:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x8007000e);
        }
        *piVar8 = (int)lVar13 * 2;
        ppiVar12 = (int **)(piVar8 + 1);
        FUN_142ef7ba0(ppiVar12,lVar2,(ulonglong)uVar3 * 2);
        *pppiVar6 = ppiVar12;
        pIVar14 = local_118;
        if (ppiVar12 == (int **)0x0) goto LAB_14039a0b8;
      }
    }
    if (local_res8 == (int ***)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    local_d0 = (int ***)&local_res8;
    (*DAT_143262a20)(&local_90);
    pIVar10 = pIVar11;
    if (local_res8 != (int ***)0x0) {
      pIVar10 = (IUnknown *)*local_res8;
    }
    local_78 = CONCAT62(uStack_f6,local_f8);
    lStack_70 = lStack_f0;
    local_68 = local_e8;
    local_58 = local_110;
    uStack_54 = uStack_10c;
    uStack_50 = (undefined4)uStack_108;
    uStack_4c = uStack_108._4_4_;
    local_48 = local_100;
    iVar4 = (**(code **)(*(longlong *)pIVar14 + 0x48))
                      (pIVar14,pIVar10,&local_58,&local_78,&local_90);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar14,(_GUID *)&DAT_1432743e8);
    }
    local_b8 = local_90;
    uStack_b4 = uStack_8c;
    uStack_b0 = uStack_88;
    uStack_ac = uStack_84;
    local_a8 = local_80;
    local_90 = local_90 & 0xffff0000;
    FUN_1401be120(&local_res8);
    uVar7 = FUN_1409339d0(&local_d8,&local_b8);
    FUN_1401a5040(&local_118,uVar7);
    if (local_118 != (IUnknown *)0x0) {
      local_c0 = local_118;
      pIVar11 = local_118;
    }
    if (local_d8 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_d8 + 0x10))();
    }
    if ((short)local_b8 == 8) {
      local_b8 = local_b8 & 0xffff0000;
      if (CONCAT44(uStack_ac,uStack_b0) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_ac,uStack_b0) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_b8);
    }
    if ((short)local_110 == 8) {
      local_110 = local_110 & 0xffff0000;
      if (uStack_108 != 0) {
        (*DAT_143ad5990)(uStack_108 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_110);
    }
    if (local_f8 == 8) {
      local_f8 = 0;
      if (lStack_f0 != 0) {
        (*DAT_143ad5990)(lStack_f0 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_f8);
    }
    puVar1 = PTR_DAT_143a45980;
    if (pIVar11 != (IUnknown *)0x0) {
      local_d8 = pIVar11;
      if (pIVar11 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar11 + 8))(pIVar11);
      }
      FUN_14090f200(&local_c8,&local_d8,puVar1);
      uVar5 = 0;
      if (local_c8 != (IUnknown *)0x0) {
        local_res8 = (int ***)0x0;
        piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x14);
        piVar8[1] = 1;
        *piVar8 = -1;
        local_res8 = (int ***)(piVar8 + 4);
        piVar8[2] = 0;
        *(undefined2 *)local_res8 = 0;
        *(undefined2 *)local_res8 = DAT_14327fe1c;
        if (*piVar8 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar8[1] < 1) {
          FUN_142e54290(0x90,piVar8[1],1);
        }
        *piVar8 = 1;
        *(undefined2 *)((longlong)local_res8 + 2) = 0;
        if (piVar8[1] + 1 < 2) {
          FUN_142e54290(0x9c,1);
        }
        piVar8[2] = 2;
        puVar1 = PTR_u_link_143a46190;
        local_118 = local_c8;
        if (local_c8 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_c8 + 8))();
        }
        plVar9 = (longlong *)FUN_140912780(&local_d0,&local_118,puVar1,&local_res8);
        uVar5 = 0;
        if (*plVar9 != 0) {
          uVar5 = FUN_142f11880();
        }
        if ((int ****)local_d0 != (int ****)0x0) {
          FUN_1401bebb0(local_d0 + -2);
        }
      }
      if (local_c8 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_c8 + 0x10))();
      }
    }
    uVar3 = local_res18[0];
    if (uVar5 != 0) {
      uVar3 = uVar5;
    }
    if (pIVar11 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar11 + 0x10))(pIVar11);
    }
    if (lVar2 != 0) {
      FUN_1401bebb0(lVar2 + -0x10);
    }
    FUN_1403fcd90(&local_a0,&local_res8,uVar3);
    *local_res10 = local_res8;
    param_2 = local_res10;
  }
  else {
    *param_2 = local_res8;
  }
  return param_2;
}



//===========================================================
// FUN_142d97e30 @ 142d97e30   (551 bytes)
//===========================================================

void FUN_142d97e30(longlong param_1,undefined4 param_2,longlong *param_3,undefined4 param_4,
                  undefined4 param_5,int param_6,undefined4 param_7,undefined1 param_8)

{
  longlong lVar1;
  char cVar2;
  longlong lVar3;
  undefined8 *puVar4;
  longlong *plVar5;
  longlong *plVar6;
  undefined8 local_res8;
  undefined4 local_res10;
  longlong *local_res18;
  
  plVar5 = (longlong *)**(longlong **)(param_1 + 0x2c10);
  local_res10 = param_2;
  local_res18 = param_3;
  if (plVar5 != *(longlong **)(param_1 + 0x2c10)) {
    do {
      lVar1 = plVar5[3];
      if (lVar1 != 0) {
        if (0xfffff < *(ulonglong *)(lVar1 + 0x20)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar1 + 0x20) = *(longlong *)(lVar1 + 0x20) + 1;
        UNLOCK();
      }
      if (lVar1 != 0) {
        local_res8 = 0;
        FUN_14019a260(&local_res8,param_3);
        cVar2 = FUN_141811ee0(lVar1,local_res10,&local_res8,param_4,param_5);
        if (cVar2 == '\0') {
          plVar6 = (longlong *)*plVar5;
        }
        else {
          thunk_FUN_142bf3f70(lVar1,param_7,param_8);
          plVar6 = (longlong *)*plVar5;
          *(longlong **)plVar5[1] = plVar6;
          *(longlong *)(*plVar5 + 8) = plVar5[1];
          *(longlong *)(param_1 + 0x2c18) = *(longlong *)(param_1 + 0x2c18) + -1;
          FUN_140cbb030(plVar5 + 2);
          thunk_FUN_140205820(plVar5,0x20);
          plVar5 = plVar6;
          if (plVar6 != (longlong *)*(longlong *)(param_1 + 0x2c10)) {
            do {
              lVar3 = plVar5[3];
              if (lVar3 == 0) {
                FUN_142e52ed0(0x431,0);
                lVar3 = plVar5[3];
              }
              FUN_141811f60(lVar3);
              plVar5 = (longlong *)*plVar5;
            } while (plVar5 != (longlong *)*(longlong *)(param_1 + 0x2c10));
          }
          if (param_6 == 0) {
            if (0xffffe < *(longlong *)(lVar1 + 0x20) - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar5 = (longlong *)(lVar1 + 0x20);
            lVar3 = *plVar5;
            *plVar5 = *plVar5 + -1;
            UNLOCK();
            if (((int)lVar3 == 1) &&
               (puVar4 = (undefined8 *)(lVar1 + 0x18), puVar4 != (undefined8 *)0x0)) {
              (**(code **)*puVar4)(puVar4,1);
            }
            break;
          }
        }
        if (0xffffe < *(longlong *)(lVar1 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar5 = (longlong *)(lVar1 + 0x20);
        lVar3 = *plVar5;
        *plVar5 = *plVar5 + -1;
        UNLOCK();
        plVar5 = plVar6;
        if (((int)lVar3 == 1) &&
           (puVar4 = (undefined8 *)(lVar1 + 0x18), puVar4 != (undefined8 *)0x0)) {
          (**(code **)*puVar4)(puVar4,1);
        }
      }
    } while (plVar5 != *(longlong **)(param_1 + 0x2c10));
  }
  if (*param_3 != 0) {
    FUN_14019f2c0(*param_3 + -0x10);
  }
  return;
}



//===========================================================
// FUN_142da05c0 @ 142da05c0   (31 bytes)
//===========================================================

bool FUN_142da05c0(longlong param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x31b4);
  *param_2 = *(undefined8 *)(param_1 + 0x31b8);
  *(undefined4 *)(param_1 + 0x31b4) = 0;
  return iVar1 != 0;
}


