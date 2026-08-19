
//===========================================================
// FUN_142699710 @ 142699710   (2734 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142699f80) */
/* WARNING: Removing unreachable block (ram,0x000142699c60) */
/* WARNING: Removing unreachable block (ram,0x000142699927) */
/* WARNING: Removing unreachable block (ram,0x000142699ac0) */
/* WARNING: Removing unreachable block (ram,0x000142699df0) */
/* WARNING: Removing unreachable block (ram,0x00014269a101) */
/* WARNING: Type propagation algorithm not settling */

void FUN_142699710(undefined8 param_1,undefined8 param_2,int param_3,int param_4,int param_5,
                  int param_6,undefined8 *param_7)

{
  int *piVar1;
  undefined8 uVar2;
  int *piVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  int **ppiVar9;
  int iVar10;
  int iVar11;
  int *local_a8;
  int *local_a0;
  int *local_98;
  int *local_90;
  int *local_88;
  int *local_80;
  int *local_78;
  int *local_70;
  longlong local_68 [2];
  int *local_58;
  int *local_50;
  int *local_48;
  int *local_40;
  int *local_38;
  
  if ((param_5 < 1) && (param_4 < 1)) {
    return;
  }
  iVar11 = ((param_5 - param_3) - param_4) - param_6;
  FUN_14019ba10(param_2,*param_7);
  if (((param_4 < 1) && (iVar11 == 0)) && (param_6 == 0)) {
    local_98 = (int *)0x0;
    FUN_14019a260(&local_98,param_2);
    FUN_14269a1d0(param_1,0x28,&local_98,0x3e9,0,0);
    return;
  }
  uVar7 = 0;
  local_68[1] = 0;
  FUN_14019a260(local_68 + 1,param_2);
  FUN_14269a1d0(param_1,0x28,local_68 + 1,0x3e9,0,0);
  local_68[0] = 0;
  uVar2 = FUN_14019ba10(local_68,&DAT_143479558,param_3);
  local_98 = (int *)0x0;
  FUN_14019a260(&local_98,uVar2);
  if (local_68[0] != 0) {
    FUN_14019f2c0(local_68[0] + -0x10);
  }
  piVar3 = local_98;
  local_70 = (int *)0x0;
  iVar10 = 0;
  piVar1 = local_70;
  if ((local_98 != (int *)0x0) && (piVar4 = local_98 + -4, piVar4 != (int *)0x0)) {
    if (*piVar4 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar8 = 0xffffffffffffffff;
      do {
        uVar8 = uVar8 + 1;
      } while (*(char *)((longlong)piVar3 + uVar8) != '\0');
      iVar5 = (int)uVar8;
      iVar6 = iVar10;
      if (0 < iVar5) {
        iVar6 = iVar5;
      }
      piVar3 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      piVar3[1] = iVar6;
      *piVar3 = -1;
      piVar1 = piVar3 + 4;
      piVar3[2] = 0;
      *(undefined1 *)piVar1 = 0;
      local_58 = piVar1;
      FUN_142ef7ba0(piVar1,local_98,(longlong)iVar5);
      if (*piVar3 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar5 == -1) || (iVar5 <= piVar3[1])) {
        *piVar3 = 1;
        if (iVar5 != -1) goto LAB_1426998e5;
        uVar8 = uVar7;
        if (piVar1 != (int *)0x0) {
          uVar8 = 0xffffffffffffffff;
          do {
            uVar8 = uVar8 + 1;
          } while (*(char *)((longlong)piVar1 + uVar8) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar3[1],uVar8 & 0xffffffff);
        *piVar3 = 1;
LAB_1426998e5:
        *(undefined1 *)((longlong)piVar1 + (longlong)iVar5) = 0;
      }
      iVar6 = (int)uVar8;
      if ((iVar6 < 0) || (piVar3[1] + 1 <= iVar6)) {
        FUN_142e54290(0x9c,uVar8 & 0xffffffff);
      }
      piVar3[2] = iVar6;
      if (local_70 != (int *)0x0) {
        FUN_14019f2c0(local_70 + -4);
      }
    }
    else {
      if (*piVar4 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar4 = *piVar4 + 1;
      UNLOCK();
      if (local_70 != (int *)0x0) {
        FUN_14019f2c0(local_70 + -4);
      }
      local_70 = piVar3;
      piVar1 = local_70;
    }
  }
  local_70 = piVar1;
  FUN_14269ac40(param_1,0x28,&local_70);
  local_a8 = (int *)0x0;
  if (0 < param_4) {
    FUN_14019ba10(&local_a8,&DAT_143479560,param_4);
    piVar3 = local_a8;
    local_90 = (int *)0x0;
    piVar1 = local_90;
    if ((local_a8 != (int *)0x0) && (piVar4 = local_a8 + -4, piVar4 != (int *)0x0)) {
      if (*piVar4 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar8 = 0xffffffffffffffff;
        do {
          uVar8 = uVar8 + 1;
        } while (*(char *)((longlong)piVar3 + uVar8) != '\0');
        iVar5 = (int)uVar8;
        iVar6 = iVar10;
        if (0 < iVar5) {
          iVar6 = iVar5;
        }
        piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
        piVar4[1] = iVar6;
        *piVar4 = -1;
        piVar1 = piVar4 + 4;
        piVar4[2] = 0;
        *(undefined1 *)piVar1 = 0;
        local_50 = piVar1;
        FUN_142ef7ba0(piVar1,piVar3,(longlong)iVar5);
        if (*piVar4 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar5 == -1) || (iVar5 <= piVar4[1])) {
          *piVar4 = 1;
          if (iVar5 != -1) goto LAB_142699a7e;
          uVar8 = uVar7;
          if (piVar1 != (int *)0x0) {
            uVar8 = 0xffffffffffffffff;
            do {
              uVar8 = uVar8 + 1;
            } while (*(char *)((longlong)piVar1 + uVar8) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar4[1],uVar8 & 0xffffffff);
          *piVar4 = 1;
LAB_142699a7e:
          *(undefined1 *)((longlong)piVar1 + (longlong)iVar5) = 0;
        }
        iVar6 = (int)uVar8;
        if ((iVar6 < 0) || (piVar4[1] + 1 <= iVar6)) {
          FUN_142e54290(0x9c,uVar8 & 0xffffffff);
        }
        piVar4[2] = iVar6;
        if (local_90 != (int *)0x0) {
          FUN_14019f2c0(local_90 + -4);
        }
      }
      else {
        if (*piVar4 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar4 = *piVar4 + 1;
        UNLOCK();
        if (local_90 != (int *)0x0) {
          FUN_14019f2c0(local_90 + -4);
        }
        local_90 = piVar3;
        piVar1 = local_90;
      }
    }
    local_90 = piVar1;
    FUN_14269ac40(param_1,0x2c,&local_90);
  }
  if (iVar11 < 0) {
    FUN_14019ba10(&local_a8,&DAT_1433de688,iVar11);
    piVar1 = local_a8;
    local_88 = (int *)0x0;
    if ((local_a8 != (int *)0x0) && (piVar3 = local_a8 + -4, piVar3 != (int *)0x0)) {
      if (*piVar3 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar8 = 0xffffffffffffffff;
        do {
          uVar8 = uVar8 + 1;
        } while (*(char *)((longlong)piVar1 + uVar8) != '\0');
        iVar6 = (int)uVar8;
        iVar11 = iVar10;
        if (0 < iVar6) {
          iVar11 = iVar6;
        }
        piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
        piVar4[1] = iVar11;
        *piVar4 = -1;
        piVar3 = piVar4 + 4;
        piVar4[2] = 0;
        *(undefined1 *)piVar3 = 0;
        local_48 = piVar3;
        FUN_142ef7ba0(piVar3,piVar1,(longlong)iVar6);
        if (*piVar4 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar6 == -1) || (iVar6 <= piVar4[1])) {
          *piVar4 = 1;
          if (iVar6 != -1) goto LAB_142699c1e;
          uVar8 = uVar7;
          if (piVar3 != (int *)0x0) {
            uVar8 = 0xffffffffffffffff;
            do {
              uVar8 = uVar8 + 1;
            } while (*(char *)((longlong)piVar3 + uVar8) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar4[1],uVar8 & 0xffffffff);
          *piVar4 = 1;
LAB_142699c1e:
          *(undefined1 *)((longlong)piVar3 + (longlong)iVar6) = 0;
        }
        iVar11 = (int)uVar8;
        if ((iVar11 < 0) || (piVar4[1] + 1 <= iVar11)) {
          FUN_142e54290(0x9c,uVar8 & 0xffffffff);
        }
        piVar4[2] = iVar11;
        if (local_88 != (int *)0x0) {
          FUN_14019f2c0(local_88 + -4);
        }
        ppiVar9 = &local_88;
        uVar2 = 0x2b;
        local_88 = piVar3;
        goto LAB_142699e58;
      }
      if (*piVar3 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar3 = *piVar3 + 1;
      UNLOCK();
      if (local_88 != (int *)0x0) {
        FUN_14019f2c0(local_88 + -4);
      }
      local_88 = piVar1;
    }
    ppiVar9 = &local_88;
    uVar2 = 0x2b;
LAB_142699e58:
    FUN_14269ac40(param_1,uVar2,ppiVar9,0xffffffff);
  }
  else if (0 < iVar11) {
    FUN_14019ba10(&local_a8,&DAT_143479560,iVar11);
    piVar3 = local_a8;
    local_80 = (int *)0x0;
    piVar1 = local_80;
    if ((local_a8 != (int *)0x0) && (piVar4 = local_a8 + -4, piVar4 != (int *)0x0)) {
      if (*piVar4 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar8 = 0xffffffffffffffff;
        do {
          uVar8 = uVar8 + 1;
        } while (*(char *)((longlong)piVar3 + uVar8) != '\0');
        iVar6 = (int)uVar8;
        iVar11 = iVar10;
        if (0 < iVar6) {
          iVar11 = iVar6;
        }
        piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
        piVar4[1] = iVar11;
        *piVar4 = -1;
        piVar1 = piVar4 + 4;
        piVar4[2] = 0;
        *(undefined1 *)piVar1 = 0;
        local_40 = piVar1;
        FUN_142ef7ba0(piVar1,piVar3,(longlong)iVar6);
        if (*piVar4 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar6 == -1) || (iVar6 <= piVar4[1])) {
          *piVar4 = 1;
          if (iVar6 != -1) goto LAB_142699dae;
          uVar8 = uVar7;
          if (piVar1 != (int *)0x0) {
            uVar8 = 0xffffffffffffffff;
            do {
              uVar8 = uVar8 + 1;
            } while (*(char *)((longlong)piVar1 + uVar8) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar4[1],uVar8 & 0xffffffff);
          *piVar4 = 1;
LAB_142699dae:
          *(undefined1 *)((longlong)piVar1 + (longlong)iVar6) = 0;
        }
        iVar11 = (int)uVar8;
        if ((iVar11 < 0) || (piVar4[1] + 1 <= iVar11)) {
          FUN_142e54290(0x9c,uVar8 & 0xffffffff);
        }
        piVar4[2] = iVar11;
        if (local_80 != (int *)0x0) {
          FUN_14019f2c0(local_80 + -4);
        }
      }
      else {
        if (*piVar4 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar4 = *piVar4 + 1;
        UNLOCK();
        if (local_80 != (int *)0x0) {
          FUN_14019f2c0(local_80 + -4);
        }
        local_80 = piVar3;
        piVar1 = local_80;
      }
    }
    local_80 = piVar1;
    ppiVar9 = &local_80;
    uVar2 = 0x1d;
    goto LAB_142699e58;
  }
  if (0 < param_6) {
    FUN_14019ba10(&local_a8,&DAT_143479560,param_6);
    piVar3 = local_a8;
    local_78 = (int *)0x0;
    piVar1 = local_78;
    if ((local_a8 != (int *)0x0) && (piVar4 = local_a8 + -4, piVar4 != (int *)0x0)) {
      if (*piVar4 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar8 = 0xffffffffffffffff;
        do {
          uVar8 = uVar8 + 1;
        } while (*(char *)((longlong)piVar3 + uVar8) != '\0');
        iVar6 = (int)uVar8;
        iVar11 = iVar10;
        if (0 < iVar6) {
          iVar11 = iVar6;
        }
        piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
        piVar4[1] = iVar11;
        *piVar4 = -1;
        piVar1 = piVar4 + 4;
        piVar4[2] = 0;
        *(undefined1 *)piVar1 = 0;
        local_38 = piVar1;
        FUN_142ef7ba0(piVar1,piVar3,(longlong)iVar6);
        if (*piVar4 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar6 == -1) || (iVar6 <= piVar4[1])) {
          *piVar4 = 1;
          if (iVar6 != -1) goto LAB_142699f3e;
          uVar8 = uVar7;
          if (piVar1 != (int *)0x0) {
            uVar8 = 0xffffffffffffffff;
            do {
              uVar8 = uVar8 + 1;
            } while (*(char *)((longlong)piVar1 + uVar8) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar4[1],uVar8 & 0xffffffff);
          *piVar4 = 1;
LAB_142699f3e:
          *(undefined1 *)((longlong)piVar1 + (longlong)iVar6) = 0;
        }
        iVar11 = (int)uVar8;
        if ((iVar11 < 0) || (piVar4[1] + 1 <= iVar11)) {
          FUN_142e54290(0x9c,uVar8 & 0xffffffff);
        }
        piVar4[2] = iVar11;
        if (local_78 != (int *)0x0) {
          FUN_14019f2c0(local_78 + -4);
        }
      }
      else {
        if (*piVar4 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar4 = *piVar4 + 1;
        UNLOCK();
        if (local_78 != (int *)0x0) {
          FUN_14019f2c0(local_78 + -4);
        }
        local_78 = piVar3;
        piVar1 = local_78;
      }
    }
    local_78 = piVar1;
    FUN_14269ac40(param_1,0x1b,&local_78);
  }
  FUN_14019ba10(&local_a8,&DAT_1432841a8);
  piVar3 = local_a8;
  local_a0 = (int *)0x0;
  piVar1 = local_a0;
  if ((local_a8 == (int *)0x0) || (piVar4 = local_a8 + -4, piVar4 == (int *)0x0))
  goto LAB_14269a164;
  if (*piVar4 != -1) {
    if (*piVar4 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar4 = *piVar4 + 1;
    UNLOCK();
    if (local_a0 != (int *)0x0) {
      FUN_14019f2c0(local_a0 + -4);
    }
    local_a0 = piVar3;
    piVar3 = local_a8;
    piVar1 = local_a0;
    goto LAB_14269a164;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  uVar8 = 0xffffffffffffffff;
  do {
    uVar8 = uVar8 + 1;
  } while (*(char *)((longlong)piVar3 + uVar8) != '\0');
  iVar11 = (int)uVar8;
  if (0 < iVar11) {
    iVar10 = iVar11;
  }
  piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar10 + 0x11));
  piVar4[1] = iVar10;
  *piVar4 = -1;
  piVar1 = piVar4 + 4;
  piVar4[2] = 0;
  *(undefined1 *)piVar1 = 0;
  local_58 = piVar1;
  FUN_142ef7ba0(piVar1,piVar3,(longlong)iVar11);
  if (*piVar4 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar11 == -1) || (iVar11 <= piVar4[1])) {
    *piVar4 = 1;
    if (iVar11 != -1) goto LAB_14269a0bf;
    if (piVar1 != (int *)0x0) {
      uVar7 = 0xffffffffffffffff;
      do {
        uVar7 = uVar7 + 1;
      } while (*(char *)((longlong)piVar1 + uVar7) != '\0');
    }
  }
  else {
    FUN_142e54290(0x90,piVar4[1],uVar8 & 0xffffffff);
    *piVar4 = 1;
LAB_14269a0bf:
    *(undefined1 *)((longlong)piVar1 + (longlong)iVar11) = 0;
    uVar7 = uVar8;
  }
  iVar11 = (int)uVar7;
  if ((iVar11 < 0) || (piVar4[1] + 1 <= iVar11)) {
    FUN_142e54290(0x9c,uVar7 & 0xffffffff);
  }
  piVar4[2] = iVar11;
  if (local_a0 != (int *)0x0) {
    FUN_14019f2c0(local_a0 + -4);
  }
LAB_14269a164:
  local_a0 = piVar1;
  FUN_14269ac40(param_1,0x28,&local_a0);
  if (piVar3 != (int *)0x0) {
    FUN_14019f2c0(piVar3 + -4);
  }
  if (local_98 != (int *)0x0) {
    FUN_14019f2c0(local_98 + -4);
  }
  return;
}



//===========================================================
// FUN_14269a1d0 @ 14269a1d0   (1368 bytes)
//===========================================================

void FUN_14269a1d0(longlong param_1,int param_2,longlong *param_3,int param_4,int param_5,
                  int param_6)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  longlong lVar4;
  undefined8 *puVar5;
  int *piVar6;
  undefined8 uVar7;
  longlong lVar8;
  ulonglong uVar9;
  int *piVar10;
  undefined8 *local_res8;
  int local_res10;
  longlong *local_res18;
  int local_res20;
  ulonglong in_stack_fffffffffffffed0;
  ulonglong in_stack_fffffffffffffed8;
  undefined8 in_stack_fffffffffffffee0;
  undefined8 **in_stack_fffffffffffffee8;
  undefined4 uVar11;
  undefined4 local_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 **local_c8;
  int local_c0;
  undefined8 *local_b8;
  IUnknown *local_b0;
  longlong local_a8;
  undefined1 local_a0 [8];
  undefined1 local_98 [8];
  undefined8 **local_90;
  undefined8 *local_88;
  undefined8 ***local_80;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  
  local_res10 = param_2;
  local_res18 = param_3;
  local_res20 = param_4;
  if (*(int *)(param_1 + 0xa0) == 0x22) {
    if (*param_3 != 0) {
      FUN_14019f2c0(*param_3 + -0x10);
    }
    return;
  }
  FUN_142699220(param_1,&local_b0,param_2);
  iVar3 = *(int *)(param_1 + 0xa0);
  iVar1 = 0;
  do {
    piVar10 = (int *)(param_1 + 0xa8 + (longlong)iVar3 * 0x20);
    piVar10[2] = param_2;
    lVar8 = *param_3;
    if (lVar8 == 0) {
      iVar3 = -1;
    }
    else {
      lVar4 = FUN_142ef6a98(iVar1 + lVar8,&DAT_143295a70);
      iVar3 = -1;
      if (lVar4 != 0) {
        iVar3 = (int)lVar4 - (int)lVar8;
      }
    }
    puVar5 = (undefined8 *)FUN_14019ce60(param_3,&local_a8,iVar1,iVar3);
    if (*(longlong *)(piVar10 + 4) != 0) {
      FUN_14019f2c0(*(longlong *)(piVar10 + 4) + -0x10);
      piVar10[4] = 0;
      piVar10[5] = 0;
    }
    uVar9 = 0;
    *(undefined8 *)(piVar10 + 4) = *puVar5;
    *puVar5 = 0;
    if (local_a8 != 0) {
      FUN_14019f2c0(local_a8 + -0x10);
    }
    local_c0 = iVar3 + 2;
    if (local_b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_e0);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_e0 == 8) {
        local_e0 = (uint)local_e0._2_2_ << 0x10;
        if (uStack_d8 != (int *)0x0) {
          (*DAT_143ad5990)(uStack_d8 + -1);
        }
      }
      else {
        iVar1 = (*DAT_143262a18)(&local_e0);
        if (iVar1 < 0) goto LAB_14269a70d;
      }
      lVar8 = DAT_143a8b8e0;
      local_e0 = CONCAT22(local_e0._2_2_,8);
      if (DAT_143a8b8e0 != 0) {
        uVar9 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      piVar6 = (int *)(*DAT_143ad5980)((ulonglong)((int)uVar9 + 1) * 2 + 4);
      if (piVar6 == (int *)0x0) {
        uStack_d8 = (int *)0x0;
      }
      else {
        *piVar6 = (int)uVar9 * 2;
        piVar6 = piVar6 + 1;
        if (lVar8 != 0) {
          FUN_142ef7ba0(piVar6,lVar8,uVar9 * 2);
        }
        *(undefined2 *)((longlong)piVar6 + uVar9 * 2) = 0;
        uStack_d8 = piVar6;
      }
    }
    else {
      if (((short)local_e0 == 8) &&
         (local_e0 = (uint)local_e0._2_2_ << 0x10, uStack_d8 != (int *)0x0)) {
        (*DAT_143ad5990)(uStack_d8 + -1);
      }
      iVar1 = (*DAT_143262a28)(&local_e0,&DAT_143a8b8d8);
      if (iVar1 < 0) {
LAB_14269a70d:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar1);
      }
    }
    lVar8 = *(longlong *)(piVar10 + 4);
    puVar5 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
    local_88 = puVar5;
    if (puVar5 == (undefined8 *)0x0) {
      local_b8 = (undefined8 *)0x0;
    }
    else {
      puVar5[1] = 0;
      *(undefined4 *)(puVar5 + 2) = 1;
      if (lVar8 == 0) {
        *puVar5 = 0;
        local_b8 = puVar5;
      }
      else {
        in_stack_fffffffffffffed0 = in_stack_fffffffffffffed0 & 0xffffffff00000000;
        iVar1 = (*DAT_1432627f8)(0xfde9,0,lVar8,0xffffffff,0,in_stack_fffffffffffffed0);
        uVar2 = (undefined4)(in_stack_fffffffffffffed0 >> 0x20);
        iVar1 = (int)((ulonglong)(longlong)(iVar1 * 2) >> 1);
        local_res8 = (undefined8 *)CONCAT44(local_res8._4_4_,iVar1 + -1);
        piVar6 = (int *)(*DAT_143ad5980)();
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          *piVar6 = (int)local_res8 * 2;
          piVar6 = piVar6 + 1;
          *(undefined2 *)((longlong)piVar6 + ((ulonglong)local_res8 & 0xffffffff) * 2) = 0;
        }
        in_stack_fffffffffffffed0 = CONCAT44(uVar2,iVar1);
        (*DAT_1432627f8)(0xfde9,0,lVar8,0xffffffff,piVar6,in_stack_fffffffffffffed0);
        *puVar5 = piVar6;
        local_b8 = puVar5;
      }
    }
    if (local_b8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    local_c8 = &local_b8;
    local_res8 = (undefined8 *)((ulonglong)local_res8 & 0xffffffff00000000);
    local_68 = local_e0;
    uStack_64 = uStack_dc;
    uStack_60 = (undefined4)uStack_d8;
    uStack_5c = uStack_d8._4_4_;
    local_58 = local_d0;
    iVar1 = (**(code **)(*(longlong *)local_b0 + 0xb0))(local_b0,*local_b8,&local_68,&local_res8);
    if (iVar1 < 0) {
      _com_issue_errorex(iVar1,local_b0,(_GUID *)&DAT_143297250);
    }
    iVar1 = (int)local_res8;
    FUN_1401be120(&local_b8);
    *piVar10 = *piVar10 + iVar1;
    if ((short)local_e0 == 8) {
      local_e0 = local_e0 & 0xffff0000;
      if (uStack_d8 != (int *)0x0) {
        (*DAT_143ad5990)(uStack_d8 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_e0);
    }
    uVar11 = (undefined4)((ulonglong)in_stack_fffffffffffffee8 >> 0x20);
    uVar2 = (undefined4)((ulonglong)in_stack_fffffffffffffee0 >> 0x20);
    piVar10[6] = local_res20;
    piVar10[7] = param_5;
    if (param_5 == 0) {
      local_res8 = (undefined8 *)((ulonglong)local_res8 & 0xffffffff00000000);
      iVar1 = (**(code **)(*(longlong *)local_b0 + 0x30))(local_b0,&local_res8);
      if (iVar1 < 0) {
        _com_issue_errorex(iVar1,local_b0,(_GUID *)&DAT_143297250);
      }
      uVar2 = (int)local_res8;
    }
    else {
      local_res8 = (undefined8 *)0x0;
      if (*(int *)(param_1 + 0x9c) == 1) {
        local_c8 = &local_res8;
        uVar7 = FUN_142699220(param_1,local_a0,piVar10[2]);
        in_stack_fffffffffffffee8 = &local_res8;
        in_stack_fffffffffffffee0 = CONCAT44(uVar2,1);
        in_stack_fffffffffffffed8 = 0;
        in_stack_fffffffffffffed0 = in_stack_fffffffffffffed0 & 0xffffffff00000000;
        uVar2 = FUN_1426a7040(param_1,0,0,*(undefined8 *)(piVar10 + 4),uVar7,
                              in_stack_fffffffffffffed0,0,in_stack_fffffffffffffee0,
                              in_stack_fffffffffffffee8);
      }
      else {
        local_80 = &local_c8;
        local_c8 = (undefined8 **)0x0;
        local_90 = &local_res8;
        in_stack_fffffffffffffed0 = FUN_142699220(param_1,local_98,piVar10[2]);
        in_stack_fffffffffffffee8 = (undefined8 **)CONCAT44(uVar11,1);
        in_stack_fffffffffffffee0 = 0;
        in_stack_fffffffffffffed8 = in_stack_fffffffffffffed8 & 0xffffffff00000000;
        uVar2 = FUN_1426a6b20(param_1,10,*(int *)(param_1 + 0x40) + -0x14,0,
                              *(undefined8 *)(piVar10 + 4),in_stack_fffffffffffffed0,
                              in_stack_fffffffffffffed8,0,in_stack_fffffffffffffee8,&local_c8,0,0,0,
                              &local_res8);
      }
    }
    *(undefined4 *)((longlong)*(int *)(param_1 + 0xa0) * 0x20 + 0xac + param_1) = uVar2;
    lVar8 = (longlong)*(int *)(param_1 + 0xa0) * 0x20;
    iVar1 = 4;
    if (piVar10[7] != 0) {
      iVar1 = 0;
    }
    *(int *)(param_1 + 0x3c) = *(int *)(param_1 + 0x3c) + iVar1 + *(int *)(lVar8 + 0xac + param_1);
    if (iVar3 == -1) {
      piVar10 = (int *)(lVar8 + 0xac + param_1);
      *piVar10 = *piVar10 + param_6;
      *(int *)(param_1 + 0x3c) = *(int *)(param_1 + 0x3c) + param_6;
      *(int *)(param_1 + 0xa0) = *(int *)(param_1 + 0xa0) + 1;
      (**(code **)(*(longlong *)local_b0 + 0x10))(local_b0);
      if (*local_res18 == 0) {
        return;
      }
      FUN_14019f2c0(*local_res18 + -0x10);
      return;
    }
    iVar3 = *(int *)(param_1 + 0xa0) + 1;
    *(int *)(param_1 + 0xa0) = iVar3;
    param_3 = local_res18;
    param_2 = local_res10;
    iVar1 = local_c0;
  } while( true );
}



//===========================================================
// FUN_1417e2c90 @ 1417e2c90   (3471 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001417e34ff) */

undefined8 FUN_1417e2c90(longlong param_1,longlong param_2,uint param_3,int param_4)

{
  bool bVar1;
  undefined1 uVar2;
  char cVar3;
  ushort uVar4;
  undefined4 uVar5;
  int iVar6;
  undefined4 uVar7;
  longlong *plVar8;
  longlong lVar9;
  undefined8 *puVar10;
  undefined4 *puVar11;
  undefined8 uVar12;
  longlong lVar13;
  int iVar14;
  int iVar15;
  int *piVar16;
  int *piVar17;
  int iVar18;
  int *piVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  int *piVar22;
  int *piVar23;
  uint uVar24;
  undefined8 uVar25;
  int iVar26;
  longlong lVar27;
  undefined8 local_res8;
  longlong local_res10;
  uint local_res18;
  int local_res20;
  int *local_a8;
  int *local_a0;
  int *local_98;
  uint local_90;
  int local_8c;
  longlong *local_88;
  longlong local_80;
  longlong *local_78;
  int local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50 [2];
  
  local_90 = 0;
  if (param_1 == 0) {
    return 0;
  }
  if (param_2 == 0) {
    return 0;
  }
  if (DAT_143aa84a0 == 0) {
    return 0;
  }
  local_res10 = param_2;
  local_res18 = param_3;
  local_res20 = param_4;
  FUN_142cbe730();
  plVar8 = (longlong *)FUN_140192f00(param_2);
  uVar12 = DAT_143aa8328;
  param_1 = param_1 + 0x20;
  local_88 = plVar8;
  local_80 = param_1;
  uVar5 = FUN_14019a5d0(param_1);
  local_70 = FUN_14038db20(uVar12,uVar5);
  uVar5 = FUN_14019a5d0(param_1);
  iVar6 = FUN_1417da1a0(plVar8,uVar5,param_3);
  uVar12 = DAT_143aa8328;
  if (iVar6 == 0) {
    return 0;
  }
  uVar5 = FUN_14019a5d0(plVar8 + 4);
  lVar9 = FUN_140388c60(uVar12,uVar5);
  if (lVar9 == 0) {
    return 0;
  }
  if (*(int *)(lVar9 + 0x124) != 0) {
    return 0;
  }
  if (*(int *)(lVar9 + 0x128) != 0) {
    return 0;
  }
  uVar2 = FUN_1403e8d40(plVar8);
  local_res8 = CONCAT71(local_res8._1_7_,uVar2);
  if ((param_3 & 2) != 0) {
    piVar19 = (int *)0x0;
    local_98 = (int *)0x0;
    lVar9 = FUN_1417ea580((longlong)plVar8 + 0x62);
    iVar6 = 0;
    piVar23 = piVar19;
    do {
      uVar24 = (uint)piVar23;
      if (lVar9 == 0) break;
      lVar9 = lVar9 / 1000;
      uVar24 = uVar24 + 1;
      piVar23 = (int *)(ulonglong)uVar24;
      iVar6 = iVar6 + 1;
    } while (iVar6 < 4);
    if (uVar24 == 0) {
      puVar10 = (undefined8 *)FUN_1408a9e40(local_50,0xf65);
      uVar24 = 1;
      local_90 = 1;
    }
    else {
      puVar10 = (undefined8 *)FUN_1408a9e40(&local_58,0xf67);
      uVar24 = 2;
      local_90 = 2;
    }
    piVar23 = (int *)*puVar10;
    *puVar10 = 0;
    local_98 = piVar23;
    if (((uVar24 & 2) != 0) && (uVar24 = uVar24 & 0xfffffffd, local_90 = uVar24, local_58 != 0)) {
      FUN_14019f2c0(local_58 + -0x10);
    }
    if (((uVar24 & 1) != 0) && (local_50[0] != 0)) {
      FUN_14019f2c0(local_50[0] + -0x10);
    }
    uVar12 = DAT_143aa8328;
    piVar16 = piVar23;
    if ((char)local_res8 != '\0') {
      uVar5 = FUN_14019a5d0(plVar8 + 4);
      uVar5 = FUN_14038be30(uVar12,uVar5);
      cVar3 = FUN_1403e8c10(uVar5);
      if (cVar3 == '\0') {
        puVar10 = (undefined8 *)FUN_1408a9e40(&local_60,0xf66);
        piVar22 = (int *)*puVar10;
        local_a0 = piVar22;
        if (piVar22 != (int *)0x0) {
          iVar6 = piVar22[-2];
          uVar21 = (ulonglong)iVar6;
          if (iVar6 != 0) {
            if (piVar23 == (int *)0x0) {
LAB_1417e300b:
              iVar18 = 0;
LAB_1417e300e:
              if (iVar18 < iVar6) {
                iVar18 = iVar6;
              }
              puVar11 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar18 + 0x11));
              puVar11[1] = iVar18;
              *puVar11 = 0xffffffff;
              piVar16 = puVar11 + 4;
              puVar11[2] = 0;
              *(char *)piVar16 = '\0';
              local_98 = piVar16;
              if (piVar19 != (int *)0x0) {
                FUN_14019f2c0(piVar19);
              }
            }
            else {
              if ((char)*piVar23 != '\0') {
                iVar18 = piVar23[-2];
                for (iVar14 = piVar23[-3]; iVar14 < iVar18 + iVar6; iVar14 = iVar14 * 2) {
                }
                piVar19 = piVar23 + -4;
                if (piVar19 == (int *)0x0) {
                  iVar15 = 0;
LAB_1417e2f13:
                  if (iVar15 < iVar14) {
                    iVar15 = iVar14;
                  }
                  puVar11 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
                  puVar11[1] = iVar15;
                  *puVar11 = 0xffffffff;
                  piVar16 = puVar11 + 4;
                  local_98 = piVar16;
                  if (piVar19 == (int *)0x0) {
                    puVar11[2] = 0;
                    *(char *)piVar16 = '\0';
                    piVar22 = local_a0;
                  }
                  else {
                    iVar14 = piVar23[-2] + 1;
                    iVar26 = iVar15 + 1;
                    if (iVar26 < iVar14) {
                      FUN_142e54290(0x5c,iVar14,iVar26);
                      iVar14 = iVar26;
                    }
                    FUN_142ef7ba0(piVar16,piVar23,(longlong)iVar14);
                    puVar11[2] = piVar23[-2];
                    *(char *)((longlong)iVar15 + (longlong)piVar16) = '\0';
                    FUN_14019f2c0(piVar19);
                    piVar22 = local_a0;
                  }
                }
                else {
                  if ((1 < *piVar19) || (piVar23[-3] < iVar14)) {
                    iVar15 = piVar23[-2];
                    goto LAB_1417e2f13;
                  }
                  if (*piVar19 != 1) {
                    FUN_142e52dd0(0x74);
                  }
                  *piVar19 = -1;
                }
                if (piVar16 == (int *)0x0) {
                  iVar14 = 0;
                }
                else {
                  iVar14 = piVar16[-2];
                }
                FUN_142ef7ba0((char *)((longlong)iVar14 + (longlong)piVar16),piVar22,uVar21);
                FUN_14019c870(&local_98,iVar18 + iVar6);
                goto LAB_1417e30b2;
              }
              if ((piVar23 == (int *)0x0) || (piVar19 = piVar23 + -4, piVar19 == (int *)0x0))
              goto LAB_1417e300b;
              if ((1 < *piVar19) || (piVar23[-3] < iVar6)) {
                iVar18 = piVar23[-2];
                goto LAB_1417e300e;
              }
              if (*piVar19 != 1) {
                FUN_142e52dd0(0x74);
              }
              *piVar19 = -1;
            }
            FUN_142ef7ba0(piVar16,piVar22,uVar21);
            if (piVar16[-4] != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar6 == -1) || (iVar6 <= piVar16[-3])) {
              piVar16[-4] = 1;
              if (iVar6 != -1) goto LAB_1417e308f;
              if (piVar16 == (int *)0x0) {
                uVar21 = 0;
              }
              else {
                uVar21 = 0xffffffffffffffff;
                do {
                  uVar21 = uVar21 + 1;
                } while (*(char *)((longlong)piVar16 + uVar21) != '\0');
              }
            }
            else {
              FUN_142e54290(0x90,piVar16[-3],iVar6);
              piVar16[-4] = 1;
LAB_1417e308f:
              *(char *)(uVar21 + (longlong)piVar16) = '\0';
            }
            iVar6 = (int)uVar21;
            if ((iVar6 < 0) || (piVar16[-3] + 1 <= iVar6)) {
              FUN_142e54290(0x9c,uVar21 & 0xffffffff);
            }
            piVar16[-2] = iVar6;
          }
        }
LAB_1417e30b2:
        if (local_60 != 0) {
          FUN_14019f2c0(local_60 + -0x10);
        }
      }
    }
    piVar23 = (int *)0x0;
    plVar8 = (longlong *)FUN_1408a9e40(&local_68,0xf68);
    piVar19 = (int *)*plVar8;
    local_a0 = piVar19;
    if (piVar19 != (int *)0x0) {
      iVar6 = piVar19[-2];
      piVar22 = (int *)(longlong)iVar6;
      if (iVar6 != 0) {
        iVar18 = 0;
        piVar17 = piVar23;
        if (piVar16 == (int *)0x0) goto LAB_1417e3270;
        if ((char)*piVar16 == '\0') {
          if ((piVar16 == (int *)0x0) || (piVar17 = piVar16 + -4, piVar17 == (int *)0x0)) {
LAB_1417e3270:
            if (iVar18 < iVar6) {
              iVar18 = iVar6;
            }
            puVar11 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar18 + 0x11));
            puVar11[1] = iVar18;
            *puVar11 = 0xffffffff;
            piVar16 = puVar11 + 4;
            puVar11[2] = 0;
            *(char *)piVar16 = '\0';
            local_98 = piVar16;
            if (piVar17 != (int *)0x0) {
              FUN_14019f2c0(piVar17);
            }
          }
          else {
            if ((1 < *piVar17) || (piVar16[-3] < iVar6)) {
              iVar18 = piVar16[-2];
              goto LAB_1417e3270;
            }
            if (*piVar17 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar17 = -1;
          }
          FUN_142ef7ba0(piVar16,piVar19,piVar22);
          if (piVar16[-4] != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar6 == -1) || (iVar6 <= piVar16[-3])) {
            piVar16[-4] = 1;
            if (iVar6 != -1) goto LAB_1417e32f1;
            if (piVar16 != (int *)0x0) {
              piVar23 = (int *)0xffffffffffffffff;
              do {
                piVar23 = (int *)((longlong)piVar23 + 1);
              } while (*(char *)((longlong)piVar16 + (longlong)piVar23) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar16[-3],iVar6);
            piVar16[-4] = 1;
LAB_1417e32f1:
            *(char *)((longlong)piVar22 + (longlong)piVar16) = '\0';
            piVar23 = piVar22;
          }
          iVar6 = (int)piVar23;
          if ((iVar6 < 0) || (piVar16[-3] + 1 <= iVar6)) {
            FUN_142e54290(0x9c,(ulonglong)piVar23 & 0xffffffff);
          }
          piVar16[-2] = iVar6;
          goto LAB_1417e3316;
        }
        iVar14 = piVar16[-2];
        for (iVar15 = piVar16[-3]; iVar15 < iVar14 + iVar6; iVar15 = iVar15 * 2) {
        }
        piVar23 = piVar16 + -4;
        if (piVar23 == (int *)0x0) {
LAB_1417e317f:
          if (iVar18 < iVar15) {
            iVar18 = iVar15;
          }
          puVar11 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar18 + 0x11));
          puVar11[1] = iVar18;
          *puVar11 = 0xffffffff;
          piVar17 = puVar11 + 4;
          local_98 = piVar17;
          if (piVar23 == (int *)0x0) {
            puVar11[2] = 0;
            *(char *)piVar17 = '\0';
            piVar19 = local_a0;
          }
          else {
            iVar15 = piVar16[-2] + 1;
            iVar26 = iVar18 + 1;
            if (iVar26 < iVar15) {
              FUN_142e54290(0x5c,iVar15,iVar26);
              iVar15 = iVar26;
            }
            FUN_142ef7ba0(piVar17,piVar16,(longlong)iVar15);
            puVar11[2] = piVar16[-2];
            *(char *)((longlong)iVar18 + (longlong)piVar17) = '\0';
            FUN_14019f2c0(piVar23);
            piVar19 = local_a0;
          }
        }
        else {
          if ((1 < *piVar23) || (piVar16[-3] < iVar15)) {
            iVar18 = piVar16[-2];
            goto LAB_1417e317f;
          }
          if (*piVar23 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar23 = -1;
          piVar17 = piVar16;
        }
        if (piVar17 == (int *)0x0) {
          iVar18 = 0;
        }
        else {
          iVar18 = piVar17[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar18 + (longlong)piVar17),piVar19,piVar22);
        FUN_14019c870(&local_98,iVar14 + iVar6);
        piVar16 = piVar17;
      }
    }
LAB_1417e3316:
    if (local_68 != 0) {
      FUN_14019f2c0(local_68 + -0x10);
    }
    param_1 = local_80;
    iVar6 = FUN_14019a5d0(local_80);
    if ((iVar6 - 0x1f42f1U < 0x10) && ((0xe005U >> (iVar6 - 0x1f42f1U & 0x1f) & 1) != 0)) {
      puVar10 = (undefined8 *)FUN_1408a9e40(&local_a8,0xf69);
      if (piVar16 != (int *)0x0) {
        FUN_14019f2c0(piVar16 + -4);
      }
      piVar16 = (int *)*puVar10;
      *puVar10 = 0;
LAB_1417e33ec:
      local_98 = piVar16;
      if (local_a8 != (int *)0x0) {
        FUN_14019f2c0(local_a8 + -4);
      }
    }
    else {
      iVar6 = FUN_14019a5d0(param_1);
      if ((iVar6 - 0x1f431aU & 0xfffffffd) == 0) {
        puVar10 = (undefined8 *)FUN_1408a9e40(&local_a8,0xf6a);
        if (piVar16 != (int *)0x0) {
          FUN_14019f2c0(piVar16 + -4);
        }
        piVar16 = (int *)*puVar10;
        *puVar10 = 0;
        goto LAB_1417e33ec;
      }
    }
    uVar21 = 0;
    local_a0 = (int *)0x0;
    piVar23 = piVar16;
    piVar19 = local_a0;
    if ((piVar16 != (int *)0x0) && (piVar22 = piVar16 + -4, piVar22 != (int *)0x0)) {
      if (*piVar22 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar20 = 0xffffffffffffffff;
        do {
          uVar20 = uVar20 + 1;
        } while (*(char *)((longlong)piVar16 + uVar20) != '\0');
        iVar18 = (int)uVar20;
        iVar6 = 0;
        if (0 < iVar18) {
          iVar6 = iVar18;
        }
        piVar22 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
        piVar22[1] = iVar6;
        *piVar22 = -1;
        piVar19 = piVar22 + 4;
        piVar22[2] = 0;
        *(char *)piVar19 = '\0';
        local_a8 = piVar19;
        FUN_142ef7ba0(piVar19,piVar16,(longlong)iVar18);
        if (*piVar22 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar18 == -1) || (iVar18 <= piVar22[1])) {
          *piVar22 = 1;
          if (iVar18 != -1) goto LAB_1417e34be;
          if (piVar19 != (int *)0x0) {
            uVar21 = 0xffffffffffffffff;
            do {
              uVar21 = uVar21 + 1;
            } while (*(char *)((longlong)piVar19 + uVar21) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar22[1],uVar20 & 0xffffffff);
          *piVar22 = 1;
LAB_1417e34be:
          *(char *)((longlong)iVar18 + (longlong)piVar19) = '\0';
          uVar21 = uVar20;
        }
        iVar6 = (int)uVar21;
        if ((iVar6 < 0) || (piVar22[1] + 1 <= iVar6)) {
          FUN_142e54290(0x9c,uVar21 & 0xffffffff);
        }
        piVar22[2] = iVar6;
        param_1 = local_80;
        if (local_a0 != (int *)0x0) {
          FUN_14019f2c0(local_a0 + -4);
          param_1 = local_80;
        }
      }
      else {
        if (*piVar22 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar22 = *piVar22 + 1;
        UNLOCK();
        piVar23 = local_98;
        piVar19 = piVar16;
        if (local_a0 != (int *)0x0) {
          FUN_14019f2c0(local_a0 + -4);
          piVar23 = local_98;
        }
      }
    }
    local_a0 = piVar19;
    iVar6 = FUN_142a269c0(&local_a0,0,0,1,0,0xffffffff,0,0,3,0,0);
    if (iVar6 != 6) {
      if (piVar23 == (int *)0x0) {
        return 0;
      }
      FUN_14019f2c0(piVar23 + -4);
      return 0;
    }
    plVar8 = local_88;
    param_3 = local_res18;
    if (piVar23 != (int *)0x0) {
      FUN_14019f2c0(piVar23 + -4);
      plVar8 = local_88;
      param_3 = local_res18;
    }
  }
  uVar12 = DAT_143aa8328;
  uVar25 = 1;
  local_8c = 0;
  uVar5 = FUN_14019a5d0(param_1);
  lVar9 = local_res10 + 0x20;
  uVar7 = FUN_14019a5d0(lVar9);
  iVar6 = FUN_14038da60(uVar12,uVar7,uVar5,&local_8c);
  uVar12 = DAT_143aa8328;
  if (iVar6 == 0) {
    if ((param_3 & 4) == 0) {
      return 0;
    }
    local_88 = (longlong *)0x0;
    puVar10 = (undefined8 *)FUN_140398ba0(DAT_143aa8328,&local_68,local_8c);
    uVar12 = *puVar10;
    puVar10 = (undefined8 *)FUN_1408a9e40(&local_a8,0x104b);
    uVar12 = FUN_14019ba10(&local_88,*puVar10,uVar12);
    local_res8 = 0;
    FUN_14019a260(&local_res8,uVar12);
    FUN_142a26280(&local_res8,0,0,1,0,0,0,0,0,0);
    if (local_a8 != (int *)0x0) {
      FUN_14019f2c0(local_a8 + -4);
    }
    if (local_68 != 0) {
      FUN_14019f2c0(local_68 + -0x10);
    }
    if (local_88 == (longlong *)0x0) {
      return 0;
    }
    FUN_14019f2c0(local_88 + -2);
    return 0;
  }
  uVar5 = FUN_14019a5d0(plVar8 + 4);
  uVar7 = FUN_14019a5d0(param_1);
  iVar6 = FUN_14038e290(uVar12,uVar7,uVar5);
  if (iVar6 == 0) {
    if ((param_3 & 4) == 0) {
      return 0;
    }
    uVar12 = 0xbb5;
    goto LAB_1417e370b;
  }
  lVar13 = (longlong)plVar8 + 0x15a;
  uVar4 = FUN_1401ab420(lVar13,*(undefined4 *)((longlong)plVar8 + 0x15e));
  uVar5 = FUN_14019a5d0(lVar9);
  uVar7 = FUN_14019a5d0(local_80);
  iVar6 = FUN_1404174b0(uVar7,uVar5,uVar4 >> 3 & 1);
  uVar12 = DAT_143aa8328;
  lVar27 = local_res10;
  if (iVar6 != 0) {
    uVar2 = FUN_1401b0050((longlong)local_88 + 0x16e,*(undefined4 *)((longlong)local_88 + 0x172));
    uVar5 = FUN_14019a5d0(local_80);
    uVar7 = FUN_14019a5d0(lVar9);
    iVar6 = FUN_14038d940(uVar12,uVar7,uVar5,uVar2);
    lVar27 = local_res10;
    if (iVar6 == 0) {
      if ((param_3 & 4) == 0) {
        return 0;
      }
      uVar12 = 0x1049;
      goto LAB_1417e370b;
    }
    iVar6 = FUN_1403c3c00(DAT_143aa8328,local_res10);
    plVar8 = local_88;
    if (iVar6 != 0) {
      if ((param_3 & 4) == 0) {
        return 0;
      }
      uVar12 = 0x104a;
      goto LAB_1417e370b;
    }
  }
  uVar24 = FUN_1401ab420(lVar13,*(undefined4 *)((longlong)plVar8 + 0x15e));
  if (((uVar24 >> 3 & 1) == 0) || (local_70 != 0)) {
    uVar5 = FUN_14019a5d0(plVar8 + 4);
    uVar5 = FUN_140253130(uVar5);
    uVar7 = FUN_14019a5d0(plVar8 + 4);
    uVar5 = FUN_140253d30(uVar7,uVar5);
    iVar6 = FUN_1402534a0(uVar5);
    if ((iVar6 != 0) ||
       ((local_8c != 0 ||
        (uVar24 = FUN_1401ab420(lVar13,*(undefined4 *)((longlong)plVar8 + 0x15e)),
        (uVar24 >> 3 & 1) != 0)))) {
      uVar12 = DAT_143aa8328;
      uVar5 = FUN_14019a5d0(plVar8 + 4);
      iVar6 = FUN_140389c10(uVar12,uVar5);
      lVar9 = local_80;
      if (iVar6 == 0) {
        iVar6 = FUN_14019a5d0(local_80);
        if ((iVar6 - 0x1f42f1U < 0x10) && ((0xe005U >> (iVar6 - 0x1f42f1U & 0x1f) & 1) != 0)) {
          bVar1 = true;
        }
        else {
          bVar1 = false;
        }
        lVar13 = FUN_1417ea580((longlong)plVar8 + 0x62);
        if ((lVar13 == 0) && (bVar1)) {
          return 0;
        }
        uVar5 = FUN_14019a5d0(lVar9);
        cVar3 = FUN_1403e8670(uVar5);
        if ((cVar3 != '\0') && (cVar3 = thunk_FUN_1403e84b0(lVar27), cVar3 == '\0')) {
          if ((param_3 & 4) == 0) {
            return 0;
          }
          uVar12 = FUN_1408a9e40(&local_res8,0xb28);
          FUN_140d84b70(uVar12,0);
          return 0;
        }
        iVar6 = FUN_14019a5d0(lVar9);
        if ((iVar6 != 0x1f42f5) || (cVar3 = FUN_1403e84b0(lVar27), cVar3 != '\0')) {
          uVar12 = DAT_143aa8328;
          uVar5 = FUN_14019a5d0(lVar9);
          FUN_14039f600(uVar12,&local_78,uVar5,0);
          if (local_78 != (longlong *)0x0) {
            if (((-1 < local_res20) || ((char)local_res8 == '\0')) ||
               (iVar6 = (**(code **)(*plVar8 + 0x28))(plVar8), iVar6 != 0)) goto LAB_1417e3a05;
            if ((param_3 & 4) != 0) {
              uVar12 = FUN_1408a9e40(&local_res8,0xf64);
              FUN_142a26280(uVar12,0,0,1,0,0,0,0,0,0);
            }
          }
          uVar25 = 0;
LAB_1417e3a05:
          if (local_78 != (longlong *)0x0) {
            (**(code **)(*local_78 + 0x10))();
            return uVar25;
          }
          return uVar25;
        }
        goto LAB_1417e3960;
      }
    }
    if ((param_3 & 4) == 0) {
      return 0;
    }
    uVar12 = 0xb26;
  }
  else {
LAB_1417e3960:
    if ((param_3 & 4) == 0) {
      return 0;
    }
    uVar12 = 0xb27;
  }
LAB_1417e370b:
  uVar12 = FUN_1408a9e40(&local_res8,uVar12);
  FUN_142a26280(uVar12,0,0,1,0,0,0,0,0,0);
  return 0;
}



//===========================================================
// FUN_1403e8d40 @ 1403e8d40   (183 bytes)
//===========================================================

undefined4 FUN_1403e8d40(longlong param_1)

{
  undefined8 uVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  
  cVar2 = FUN_140841850();
  uVar1 = DAT_143aa8328;
  if (((cVar2 == '\0') && (param_1 != 0)) &&
     (iVar3 = FUN_14038c1f0(DAT_143aa8328,param_1), iVar3 == 0)) {
    uVar4 = FUN_1401b0340(param_1 + 0x20);
    iVar3 = FUN_14038c340(uVar1,uVar4);
    if (iVar3 == 0) {
      iVar3 = FUN_14038be30(uVar1,uVar4);
      if (iVar3 == 0) {
        iVar3 = FUN_14038ce90(uVar1,uVar4);
        if (iVar3 != 0) {
          return 0;
        }
        iVar3 = FUN_14038aae0(uVar1,uVar4);
        if (iVar3 != 0) {
          return 0;
        }
      }
      cVar2 = FUN_1401b0050(param_1 + 0x1a6,*(undefined4 *)(param_1 + 0x1aa));
      if (cVar2 == -1) {
        return 1;
      }
    }
  }
  return 0;
}


