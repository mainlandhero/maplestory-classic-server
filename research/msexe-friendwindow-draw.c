
//===========================================================
// FUN_142debab0 @ 142debab0   (318 bytes)
//===========================================================

uint FUN_142debab0(longlong *param_1,int param_2,int param_3)

{
  int iVar1;
  longlong lVar2;
  uint uVar3;
  uint uVar4;
  longlong lVar5;
  undefined4 uVar6;
  
  iVar1 = FUN_142deaf60();
  if (iVar1 == 0) {
    uVar4 = 0;
    lVar5 = 0;
    while( true ) {
      FUN_142deaf60();
      lVar2 = *param_1;
      if (lVar2 == 0) {
        iVar1 = 0;
      }
      else {
        iVar1 = *(int *)(lVar2 + -8);
      }
      if (iVar1 <= (int)uVar4) break;
      if (param_2 != 0) {
        if (lVar2 == 0) {
          uVar3 = 0;
        }
        else {
          uVar3 = *(uint *)(lVar2 + -8);
        }
        if (((int)uVar4 < 0) || (uVar3 <= uVar4)) {
          if (lVar2 == 0) {
            uVar6 = 0;
          }
          else {
            uVar6 = *(undefined4 *)(lVar2 + -8);
          }
          FUN_142e54290(0xbc,uVar4,uVar6);
          lVar2 = *param_1;
        }
        if (*(int *)(lVar5 + lVar2) == param_2) {
          return uVar4;
        }
      }
      if (param_3 != 0) {
        if (lVar2 == 0) {
          uVar3 = 0;
        }
        else {
          uVar3 = *(uint *)(lVar2 + -8);
        }
        if (((int)uVar4 < 0) || (uVar3 <= uVar4)) {
          if (lVar2 == 0) {
            uVar6 = 0;
          }
          else {
            uVar6 = *(undefined4 *)(lVar2 + -8);
          }
          FUN_142e54290(0xbc,uVar4,uVar6);
          lVar2 = *param_1;
        }
        if ((byte)(*(char *)(lVar5 + 0x11 + lVar2) - 5U) < 4) {
          if (lVar2 == 0) {
            uVar3 = 0;
          }
          else {
            uVar3 = *(uint *)(lVar2 + -8);
          }
          if (((int)uVar4 < 0) || (uVar3 <= uVar4)) {
            if (lVar2 == 0) {
              uVar6 = 0;
            }
            else {
              uVar6 = *(undefined4 *)(lVar2 + -8);
            }
            FUN_142e54290(0xbc,uVar4,uVar6);
            lVar2 = *param_1;
          }
          if (*(int *)(lVar5 + 0x28 + lVar2) == param_3) {
            return uVar4;
          }
        }
      }
      uVar4 = uVar4 + 1;
      lVar5 = lVar5 + 0x149;
    }
  }
  return 0xffffffff;
}



//===========================================================
// FUN_1411c54e0 @ 1411c54e0   (2168 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001411c5a6e) */
/* WARNING: Removing unreachable block (ram,0x0001411c5c33) */

void FUN_1411c54e0(longlong param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  undefined8 *puVar4;
  int *piVar5;
  longlong lVar6;
  int *piVar7;
  int *piVar8;
  int *piVar9;
  int iVar10;
  int iVar11;
  int *piVar12;
  ulonglong uVar13;
  int *piVar14;
  undefined4 local_res8;
  undefined4 local_res10;
  int *local_res18;
  int *local_res20;
  int *local_e8;
  int *local_e0;
  int *local_d8;
  longlong local_d0;
  undefined8 local_c8;
  int *local_c0;
  int *local_b8;
  int *piStack_b0;
  int *local_a8;
  undefined4 local_a0;
  int *local_98;
  undefined4 local_90;
  undefined8 local_88;
  undefined8 uStack_80;
  longlong local_78;
  int *local_70;
  int *local_68;
  
  piVar8 = (int *)0x0;
  local_c8 = 0;
  local_c0 = (int *)0x0;
  local_b8 = (int *)0x0;
  piStack_b0 = (int *)0x0;
  local_a8 = (int *)0x0;
  local_a0 = 0;
  local_98 = (int *)0x0;
  local_90 = 0;
  local_88 = 0;
  uStack_80 = 0;
  local_res8 = 0;
  piVar14 = (int *)0x0;
  local_res18 = (int *)0x0;
  piVar12 = (int *)0x0;
  local_e0 = (int *)0x0;
  lVar6 = *(longlong *)(param_1 + 0x290);
  if (lVar6 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar6 = *(longlong *)(param_1 + 0x290);
  }
  iVar1 = *(int *)(lVar6 + 0x26c);
  uVar13 = 0xffffffffffffffff;
  iVar11 = 0;
  piVar3 = piVar8;
  piVar2 = piVar12;
  piVar5 = piVar8;
  if (*(int *)(lVar6 + 0x268) != 0) {
    if (iVar1 != 0) goto LAB_1411c557b;
    iVar1 = FUN_1411bdd00(lVar6,*(int *)(lVar6 + 0x268),&local_c8);
    piVar7 = local_98;
    piVar9 = local_a8;
    piVar3 = piVar12;
    if (iVar1 == 0) goto LAB_1411c5cce;
    local_res10 = (undefined4)local_c8;
    if (local_c0 != (int *)0x0) {
      piVar5 = local_c0;
    }
    if ((piVar5 != (int *)0x0) && (piVar14 = piVar5 + -4, piVar14 != (int *)0x0)) {
      iVar1 = *piVar14;
      if (iVar1 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        piVar14 = (int *)0xffffffffffffffff;
        do {
          piVar14 = (int *)((longlong)piVar14 + 1);
        } while (*(char *)((longlong)piVar5 + (longlong)piVar14) != '\0');
        iVar10 = (int)piVar14;
        iVar1 = iVar11;
        if (0 < iVar10) {
          iVar1 = iVar10;
        }
        piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar1 + 0x11));
        piVar12[1] = iVar1;
        *piVar12 = -1;
        local_res18 = piVar12 + 4;
        piVar12[2] = 0;
        *(undefined1 *)local_res18 = 0;
        FUN_142ef7ba0(local_res18,piVar5,(longlong)iVar10);
        if (*piVar12 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= piVar12[1])) {
          *piVar12 = 1;
          if (iVar10 != -1) goto LAB_1411c578a;
          piVar14 = piVar8;
          if (local_res18 != (int *)0x0) {
            piVar14 = (int *)0xffffffffffffffff;
            do {
              piVar14 = (int *)((longlong)piVar14 + 1);
            } while (*(char *)((longlong)local_res18 + (longlong)piVar14) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar12[1],(ulonglong)piVar14 & 0xffffffff);
          *piVar12 = 1;
LAB_1411c578a:
          *(undefined1 *)((longlong)local_res18 + (longlong)iVar10) = 0;
        }
        iVar1 = (int)piVar14;
        if ((iVar1 < 0) || (piVar12[1] + 1 <= iVar1)) {
          FUN_142e54290(0x9c,(ulonglong)piVar14 & 0xffffffff);
        }
        piVar12[2] = iVar1;
        goto LAB_1411c57fd;
      }
      goto LAB_1411c57e0;
    }
    goto LAB_1411c57fd;
  }
  piVar7 = piVar8;
  piVar9 = piVar8;
  piVar14 = piVar8;
  if (iVar1 == 0) goto LAB_1411c5cce;
LAB_1411c557b:
  iVar1 = FUN_1411bded0(lVar6,iVar1,&local_c8);
  piVar7 = local_98;
  piVar9 = local_a8;
  piVar14 = piVar8;
  if (iVar1 == 0) goto LAB_1411c5cce;
  local_res8 = 1;
  local_res10 = local_c8._4_4_;
  if (piStack_b0 != (int *)0x0) {
    piVar5 = piStack_b0;
  }
  piVar2 = piVar8;
  if ((piVar5 != (int *)0x0) && (piVar14 = piVar5 + -4, piVar2 = piVar12, piVar14 != (int *)0x0)) {
    iVar1 = *piVar14;
    if (iVar1 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      piVar14 = (int *)0xffffffffffffffff;
      do {
        piVar14 = (int *)((longlong)piVar14 + 1);
      } while (*(char *)((longlong)piVar5 + (longlong)piVar14) != '\0');
      iVar10 = (int)piVar14;
      iVar1 = 0;
      if (0 < iVar10) {
        iVar1 = iVar10;
      }
      piVar2 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar1 + 0x11));
      piVar2[1] = iVar1;
      *piVar2 = -1;
      local_res18 = piVar2 + 4;
      piVar2[2] = 0;
      *(undefined1 *)local_res18 = 0;
      FUN_142ef7ba0(local_res18,piVar5,(longlong)iVar10);
      if (*piVar2 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= piVar2[1])) {
        *piVar2 = 1;
        if (iVar10 != -1) goto LAB_1411c565a;
        piVar14 = piVar8;
        if (local_res18 != (int *)0x0) {
          piVar14 = (int *)0xffffffffffffffff;
          do {
            piVar14 = (int *)((longlong)piVar14 + 1);
          } while (*(char *)((longlong)local_res18 + (longlong)piVar14) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar2[1],(ulonglong)piVar14 & 0xffffffff);
        *piVar2 = 1;
LAB_1411c565a:
        *(undefined1 *)((longlong)local_res18 + (longlong)iVar10) = 0;
      }
      iVar1 = (int)piVar14;
      if ((iVar1 < 0) || (piVar2[1] + 1 <= iVar1)) {
        FUN_142e54290(0x9c,(ulonglong)piVar14 & 0xffffffff);
      }
      piVar2[2] = iVar1;
      piVar2 = piVar12;
    }
    else {
LAB_1411c57e0:
      if (iVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar14 = *piVar14 + 1;
      UNLOCK();
      piVar2 = local_e0;
      local_res18 = piVar5;
    }
  }
LAB_1411c57fd:
  piVar14 = piVar8;
  if (local_b8 != (int *)0x0) {
    piVar14 = local_b8;
  }
  if ((piVar14 != (int *)0x0) && (piVar12 = piVar14 + -4, piVar12 != (int *)0x0)) {
    if (*piVar12 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      piVar12 = (int *)0xffffffffffffffff;
      do {
        piVar12 = (int *)((longlong)piVar12 + 1);
      } while (*(char *)((longlong)piVar14 + (longlong)piVar12) != '\0');
      iVar10 = (int)piVar12;
      iVar1 = iVar11;
      if (0 < iVar10) {
        iVar1 = iVar10;
      }
      piVar3 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar1 + 0x11));
      piVar3[1] = iVar1;
      *piVar3 = -1;
      piVar2 = piVar3 + 4;
      piVar3[2] = 0;
      *(undefined1 *)piVar2 = 0;
      FUN_142ef7ba0(piVar2,piVar14,(longlong)iVar10);
      if (*piVar3 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= piVar3[1])) {
        *piVar3 = 1;
        if (iVar10 != -1) goto LAB_1411c58bf;
        piVar12 = piVar8;
        if (piVar2 != (int *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (*(char *)((longlong)piVar2 + (longlong)piVar12) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar3[1],(ulonglong)piVar12 & 0xffffffff);
        *piVar3 = 1;
LAB_1411c58bf:
        *(undefined1 *)((longlong)piVar2 + (longlong)iVar10) = 0;
      }
      iVar1 = (int)piVar12;
      if ((iVar1 < 0) || (piVar3[1] + 1 <= iVar1)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      piVar3[2] = iVar1;
      local_e0 = piVar2;
    }
    else {
      if (*piVar12 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar12 = *piVar12 + 1;
      UNLOCK();
      piVar2 = piVar14;
      local_e0 = piVar14;
    }
  }
  local_d8 = (int *)0x0;
  puVar4 = (undefined8 *)FUN_1408a9e40(&local_78,0x3ed);
  FUN_14019ba10(&local_d8,*puVar4,local_res18);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  piVar12 = local_d8;
  local_res20 = (int *)0x0;
  piVar14 = local_res20;
  if ((local_d8 != (int *)0x0) && (piVar3 = local_d8 + -4, piVar3 != (int *)0x0)) {
    if (*piVar3 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      piVar3 = (int *)0xffffffffffffffff;
      do {
        piVar3 = (int *)((longlong)piVar3 + 1);
      } while (*(char *)((longlong)piVar12 + (longlong)piVar3) != '\0');
      iVar10 = (int)piVar3;
      iVar1 = iVar11;
      if (0 < iVar10) {
        iVar1 = iVar10;
      }
      piVar5 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar1 + 0x11));
      piVar5[1] = iVar1;
      *piVar5 = -1;
      piVar14 = piVar5 + 4;
      piVar5[2] = 0;
      *(undefined1 *)piVar14 = 0;
      local_d0 = (longlong)iVar10;
      local_70 = piVar14;
      FUN_142ef7ba0(piVar14,piVar12,local_d0);
      if (*piVar5 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= piVar5[1])) {
        *piVar5 = 1;
        if (iVar10 != -1) goto LAB_1411c5a24;
        piVar3 = piVar8;
        if (piVar14 != (int *)0x0) {
          piVar3 = (int *)0xffffffffffffffff;
          do {
            piVar3 = (int *)((longlong)piVar3 + 1);
          } while (*(char *)((longlong)piVar14 + (longlong)piVar3) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar5[1],(ulonglong)piVar3 & 0xffffffff);
        *piVar5 = 1;
LAB_1411c5a24:
        *(undefined1 *)((longlong)piVar14 + local_d0) = 0;
      }
      iVar1 = (int)piVar3;
      if ((iVar1 < 0) || (piVar5[1] + 1 <= iVar1)) {
        FUN_142e54290(0x9c,(ulonglong)piVar3 & 0xffffffff);
      }
      piVar5[2] = iVar1;
      if (local_res20 != (int *)0x0) {
        FUN_14019f2c0(local_res20 + -4);
      }
    }
    else {
      if (*piVar3 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar3 = *piVar3 + 1;
      UNLOCK();
      if (local_res20 != (int *)0x0) {
        FUN_14019f2c0(local_res20 + -4);
      }
      local_res20 = piVar12;
      piVar12 = local_d8;
      piVar2 = local_e0;
      piVar14 = local_res20;
    }
  }
  local_res20 = piVar14;
  iVar1 = FUN_142a269c0(&local_res20,0,0,1,0,0xffffffff,0,0,3,0,0);
  piVar3 = piVar2;
  if ((iVar1 == 6) && (lVar6 = FUN_141892840(), lVar6 != 0)) {
    local_d0 = FUN_141892840();
    local_e8 = (int *)0x0;
    piVar14 = local_e8;
    if ((piVar2 != (int *)0x0) && (piVar5 = piVar2 + -4, piVar5 != (int *)0x0)) {
      if (*piVar5 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        piVar5 = (int *)0xffffffffffffffff;
        do {
          piVar5 = (int *)((longlong)piVar5 + 1);
        } while (*(char *)((longlong)piVar2 + (longlong)piVar5) != '\0');
        iVar1 = (int)piVar5;
        if (0 < iVar1) {
          iVar11 = iVar1;
        }
        piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
        piVar7[1] = iVar11;
        *piVar7 = -1;
        piVar14 = piVar7 + 4;
        piVar7[2] = 0;
        *(undefined1 *)piVar14 = 0;
        local_res20 = (int *)(longlong)iVar1;
        local_68 = piVar14;
        FUN_142ef7ba0(piVar14,piVar2,local_res20);
        if (*piVar7 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar1 == -1) || (iVar1 <= piVar7[1])) {
          *piVar7 = 1;
          if (iVar1 != -1) goto LAB_1411c5be8;
          if (piVar14 != (int *)0x0) {
            do {
              uVar13 = uVar13 + 1;
            } while (*(char *)((longlong)piVar14 + uVar13) != '\0');
            piVar8 = (int *)(uVar13 & 0xffffffff);
          }
        }
        else {
          FUN_142e54290(0x90,piVar7[1],(ulonglong)piVar5 & 0xffffffff);
          *piVar7 = 1;
LAB_1411c5be8:
          *(undefined1 *)((longlong)piVar14 + (longlong)local_res20) = 0;
          piVar8 = piVar5;
        }
        iVar1 = (int)piVar8;
        if ((iVar1 < 0) || (piVar7[1] + 1 <= iVar1)) {
          FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
        }
        piVar7[2] = iVar1;
        if (local_e8 != (int *)0x0) {
          FUN_14019f2c0(local_e8 + -4);
        }
      }
      else {
        if (*piVar5 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar5 = *piVar5 + 1;
        UNLOCK();
        piVar12 = local_d8;
        piVar3 = local_e0;
        piVar14 = piVar2;
        if (local_e8 != (int *)0x0) {
          FUN_14019f2c0(local_e8 + -4);
          piVar12 = local_d8;
          piVar3 = local_e0;
        }
      }
    }
    local_e8 = piVar14;
    FUN_141829640(local_d0,local_res10,&local_e8,local_res8);
  }
  piVar7 = local_98;
  piVar9 = local_a8;
  piVar14 = local_res18;
  if (piVar12 != (int *)0x0) {
    FUN_14019f2c0(piVar12 + -4);
    piVar7 = local_98;
    piVar9 = local_a8;
  }
LAB_1411c5cce:
  if (piVar3 != (int *)0x0) {
    FUN_14019f2c0(piVar3 + -4);
  }
  if (piVar14 != (int *)0x0) {
    FUN_14019f2c0(piVar14 + -4);
  }
  if (piVar7 != (int *)0x0) {
    FUN_14019f2c0(piVar7 + -4);
  }
  if (piVar9 != (int *)0x0) {
    FUN_14019f2c0(piVar9 + -4);
  }
  if (piStack_b0 != (int *)0x0) {
    FUN_14019f2c0(piStack_b0 + -4);
  }
  if (local_b8 != (int *)0x0) {
    FUN_14019f2c0(local_b8 + -4);
  }
  if (local_c0 != (int *)0x0) {
    FUN_14019f2c0(local_c0 + -4);
  }
  return;
}



//===========================================================
// FUN_142d241a0 @ 142d241a0   (88 bytes)
//===========================================================

longlong FUN_142d241a0(longlong *param_1,uint param_2)

{
  undefined4 uVar1;
  uint uVar2;
  longlong lVar3;
  
  lVar3 = *param_1;
  uVar1 = 0;
  uVar2 = 0;
  if (lVar3 != 0) {
    uVar2 = *(uint *)(lVar3 + -8);
  }
  if (((int)param_2 < 0) || (uVar2 <= param_2)) {
    if (lVar3 != 0) {
      uVar1 = *(undefined4 *)(lVar3 + -8);
    }
    FUN_142e54290(0xbc,param_2,uVar1);
    lVar3 = *param_1;
  }
  return (longlong)(int)param_2 * 0x149 + lVar3;
}


