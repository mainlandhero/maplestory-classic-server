
//===========================================================
// FUN_140cc0b60 @ 140cc0b60   (2917 bytes)
//===========================================================

undefined8 FUN_140cc0b60(longlong param_1,int param_2,undefined4 param_3,longlong *param_4)

{
  char cVar1;
  IUnknown *pIVar2;
  int iVar3;
  undefined8 *puVar4;
  longlong *plVar5;
  longlong *plVar6;
  longlong *plVar7;
  undefined8 *puVar8;
  int *piVar9;
  longlong lVar10;
  undefined8 uVar11;
  longlong lVar12;
  longlong lVar13;
  undefined8 *puVar14;
  undefined8 *puVar15;
  uint uVar16;
  longlong *plVar17;
  undefined8 in_stack_fffffffffffffe78;
  int *piVar18;
  longlong **in_stack_fffffffffffffe80;
  ulonglong uVar19;
  undefined4 uVar20;
  uint local_168;
  int local_164;
  longlong local_160;
  longlong local_148;
  longlong *local_140;
  longlong *local_138;
  undefined8 *local_130;
  longlong *local_128;
  undefined8 local_120;
  longlong *local_118;
  undefined8 *local_110;
  longlong *local_108;
  undefined8 local_100;
  longlong *local_f8;
  undefined8 local_f0;
  longlong *local_e8;
  undefined8 *local_e0;
  longlong *local_d8;
  undefined8 local_d0;
  longlong *local_c8;
  longlong local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a8;
  undefined8 *local_a0;
  longlong *local_98;
  longlong *local_90;
  longlong *local_88;
  longlong *local_80;
  longlong *local_78;
  longlong *local_70;
  longlong *local_68;
  longlong *local_60;
  longlong *local_58;
  longlong *local_50;
  undefined8 *local_48;
  
  puVar14 = *(undefined8 **)(param_1 + 0x30);
  cVar1 = *(char *)((longlong)puVar14[1] + 0x19);
  puVar8 = puVar14;
  puVar15 = (undefined8 *)puVar14[1];
  while (cVar1 == '\0') {
    if (*(int *)(puVar15 + 4) < param_2) {
      puVar4 = (undefined8 *)puVar15[2];
      puVar15 = puVar8;
    }
    else {
      puVar4 = (undefined8 *)*puVar15;
    }
    puVar8 = puVar15;
    puVar15 = puVar4;
    cVar1 = *(char *)((longlong)puVar4 + 0x19);
  }
  if ((((*(char *)((longlong)puVar8 + 0x19) == '\0') && (*(int *)(puVar8 + 4) <= param_2)) &&
      (puVar8 != puVar14)) && (plVar17 = (longlong *)puVar8[5], plVar17 != (longlong *)puVar8[6])) {
    local_168 = 0;
    local_130 = puVar8;
    local_128 = plVar17;
    iVar3 = FUN_140cc3530(param_4);
    if (iVar3 != 0) {
      local_148 = 0;
      do {
        lVar10 = puVar8[5];
        local_164 = 0;
        if ((ulonglong)(longlong)(int)local_168 < (ulonglong)((puVar8[6] - lVar10) / 0x18)) {
          lVar13 = local_148 * 0x18;
          plVar17 = local_128;
          if (*(longlong *)(lVar13 + 8 + lVar10) - *(longlong *)(lVar13 + lVar10) >> 3 != 0) {
            local_160 = 0;
            do {
              uVar20 = (undefined4)((ulonglong)in_stack_fffffffffffffe78 >> 0x20);
              pIVar2 = *(IUnknown **)(local_160 + *(longlong *)(lVar13 + lVar10));
              if (pIVar2 == (IUnknown *)0x0) {
LAB_140cc1643:
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              local_140 = (longlong *)0x0;
              iVar3 = (**(code **)(*(longlong *)pIVar2 + 0xf0))(pIVar2,&local_140);
              if (iVar3 < 0) {
                _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327ac98);
              }
              plVar17 = local_140;
              local_98 = local_140;
              local_d0 = 0;
              local_c8 = local_140;
              if (local_140 != (longlong *)0x0) {
                (**(code **)(*local_140 + 8))(local_140);
              }
              plVar5 = (longlong *)FUN_140912780(&local_c0,&local_c8,L"islot",&local_d0);
              lVar10 = *plVar5;
              plVar5 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
              local_90 = plVar5;
              if (plVar5 == (longlong *)0x0) {
                plVar5 = (longlong *)0x0;
              }
              else {
                plVar5[1] = 0;
                *(undefined4 *)(plVar5 + 2) = 1;
                if (lVar10 == 0) {
                  *plVar5 = 0;
                }
                else {
                  lVar12 = -1;
                  do {
                    lVar12 = lVar12 + 1;
                  } while (*(short *)(lVar10 + lVar12 * 2) != 0);
                  piVar9 = (int *)(*DAT_143ad5980)();
                  if (piVar9 == (int *)0x0) {
                    *plVar5 = 0;
LAB_140cc1653:
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x8007000e);
                  }
                  *piVar9 = (int)lVar12 * 2;
                  piVar9 = piVar9 + 1;
                  FUN_142ef7ba0(piVar9,lVar10,(ulonglong)((int)lVar12 + 1) * 2);
                  *plVar5 = (longlong)piVar9;
                  if (piVar9 == (int *)0x0) goto LAB_140cc1653;
                }
              }
              local_88 = plVar5;
              if (plVar5 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x8007000e);
              }
              if (local_c0 != 0) {
                FUN_1401bebb0(local_c0 + -0x10);
              }
              local_120 = 0;
              local_118 = plVar17;
              if (plVar17 != (longlong *)0x0) {
                (**(code **)(*plVar17 + 8))(plVar17);
              }
              plVar6 = (longlong *)FUN_140912780(&local_b8,&local_118,L"vslot",&local_120);
              lVar10 = *plVar6;
              plVar7 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
              plVar6 = (longlong *)0x0;
              local_80 = plVar7;
              if (plVar7 != (longlong *)0x0) {
                plVar7[1] = 0;
                *(undefined4 *)(plVar7 + 2) = 1;
                plVar6 = plVar7;
                if (lVar10 == 0) {
                  *plVar7 = 0;
                }
                else {
                  lVar12 = -1;
                  do {
                    lVar12 = lVar12 + 1;
                  } while (*(short *)(lVar10 + lVar12 * 2) != 0);
                  piVar9 = (int *)(*DAT_143ad5980)();
                  if (piVar9 == (int *)0x0) {
                    *plVar7 = 0;
LAB_140cc1663:
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x8007000e);
                  }
                  *piVar9 = (int)lVar12 * 2;
                  piVar9 = piVar9 + 1;
                  FUN_142ef7ba0(piVar9,lVar10,(ulonglong)((int)lVar12 + 1) * 2);
                  *plVar7 = (longlong)piVar9;
                  if (piVar9 == (int *)0x0) goto LAB_140cc1663;
                }
              }
              local_78 = plVar6;
              if (plVar6 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x8007000e);
              }
              if (local_b8 != 0) {
                FUN_1401bebb0(local_b8 + -0x10);
              }
              lVar10 = *param_4;
              puVar14 = (undefined8 *)0x0;
              uVar16 = 0;
              if (lVar10 != 0) {
                uVar16 = *(uint *)(lVar10 + -8);
              }
              if (((int)local_168 < 0) || (uVar16 <= local_168)) {
                puVar8 = puVar14;
                if (lVar10 != 0) {
                  puVar8 = (undefined8 *)(ulonglong)*(uint *)(lVar10 + -8);
                }
                FUN_142e54290(0xbc,local_168,puVar8);
                lVar10 = *param_4;
              }
              puVar8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
              local_a0 = puVar8;
              if (puVar8 != (undefined8 *)0x0) {
                puVar8[1] = 0;
                *(undefined4 *)(puVar8 + 2) = 1;
                uVar19 = (ulonglong)in_stack_fffffffffffffe80 & 0xffffffff00000000;
                iVar3 = (*DAT_1432627f8)(0xfde9,0,&DAT_1434b2af1,0xffffffff,0,uVar19);
                uVar20 = (undefined4)(uVar19 >> 0x20);
                iVar3 = (int)((ulonglong)(longlong)(iVar3 * 2) >> 1);
                uVar16 = iVar3 - 1;
                piVar9 = (int *)(*DAT_143ad5980)();
                if (piVar9 == (int *)0x0) {
                  piVar9 = (int *)0x0;
                }
                else {
                  *piVar9 = uVar16 * 2;
                  piVar9 = piVar9 + 1;
                  *(undefined2 *)((longlong)piVar9 + (ulonglong)uVar16 * 2) = 0;
                }
                piVar18 = piVar9;
                (*DAT_1432627f8)(0xfde9,0,&DAT_1434b2af1,0xffffffff,piVar9,CONCAT44(uVar20,iVar3));
                uVar20 = (undefined4)((ulonglong)piVar18 >> 0x20);
                *puVar8 = piVar9;
                puVar14 = puVar8;
              }
              local_110 = puVar14;
              if (puVar14 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x8007000e);
              }
              local_108 = plVar17;
              if (plVar17 != (longlong *)0x0) {
                (**(code **)(*plVar17 + 8))(plVar17);
              }
              puVar8 = local_130;
              in_stack_fffffffffffffe80 = &local_108;
              in_stack_fffffffffffffe78 = CONCAT44(uVar20,param_3);
              FUN_140cc0300((longlong)(int)local_168 * 0xa8 + lVar10,*plVar5,*plVar6,
                            *(undefined8 *)(local_160 + *(longlong *)(lVar13 + local_130[5])),
                            in_stack_fffffffffffffe78,in_stack_fffffffffffffe80,&local_110,0);
              LOCK();
              plVar17 = plVar6 + 2;
              lVar10 = *plVar17;
              *(int *)plVar17 = (int)*plVar17 + -1;
              UNLOCK();
              if ((int)lVar10 == 1) {
                if (*plVar6 != 0) {
                  (*DAT_143ad5990)(*plVar6 + -4);
                  *plVar6 = 0;
                }
                if (plVar6[1] != 0) {
                  FUN_14019b4e0();
                  plVar6[1] = 0;
                }
                thunk_FUN_140205820(plVar6,0x18);
              }
              LOCK();
              plVar17 = plVar5 + 2;
              lVar10 = *plVar17;
              *(int *)plVar17 = (int)*plVar17 + -1;
              UNLOCK();
              if ((int)lVar10 == 1) {
                if (*plVar5 != 0) {
                  (*DAT_143ad5990)(*plVar5 + -4);
                  *plVar5 = 0;
                }
                if (plVar5[1] != 0) {
                  FUN_14019b4e0();
                  plVar5[1] = 0;
                }
                thunk_FUN_140205820(plVar5,0x18);
              }
              if (local_140 != (longlong *)0x0) {
                (**(code **)(*local_140 + 0x10))();
              }
              local_164 = local_164 + 1;
              local_160 = local_160 + 8;
              lVar10 = puVar8[5];
              plVar17 = local_128;
            } while ((ulonglong)(longlong)local_164 <
                     (ulonglong)
                     (*(longlong *)(lVar10 + 8 + lVar13) - *(longlong *)(lVar10 + lVar13) >> 3));
          }
        }
        else {
          lVar10 = *plVar17;
          if (plVar17[1] - lVar10 >> 3 != 0) {
            local_160 = 0;
            do {
              uVar20 = (undefined4)((ulonglong)in_stack_fffffffffffffe78 >> 0x20);
              pIVar2 = *(IUnknown **)(lVar10 + local_160);
              if (pIVar2 == (IUnknown *)0x0) goto LAB_140cc1643;
              local_138 = (longlong *)0x0;
              iVar3 = (**(code **)(*(longlong *)pIVar2 + 0xf0))(pIVar2,&local_138);
              if (iVar3 < 0) {
                _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327ac98);
              }
              plVar17 = local_138;
              local_70 = local_138;
              local_100 = 0;
              local_f8 = local_138;
              if (local_138 != (longlong *)0x0) {
                (**(code **)(*local_138 + 8))(local_138);
              }
              plVar5 = (longlong *)FUN_140912780(&local_b0,&local_f8,L"islot",&local_100);
              lVar10 = *plVar5;
              plVar5 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
              local_68 = plVar5;
              if (plVar5 == (longlong *)0x0) {
                plVar5 = (longlong *)0x0;
              }
              else {
                plVar5[1] = 0;
                *(undefined4 *)(plVar5 + 2) = 1;
                if (lVar10 == 0) {
                  *plVar5 = 0;
                }
                else {
                  lVar13 = -1;
                  do {
                    lVar13 = lVar13 + 1;
                  } while (*(short *)(lVar10 + lVar13 * 2) != 0);
                  piVar9 = (int *)(*DAT_143ad5980)();
                  if (piVar9 == (int *)0x0) {
                    *plVar5 = 0;
LAB_140cc1694:
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x8007000e);
                  }
                  *piVar9 = (int)lVar13 * 2;
                  piVar9 = piVar9 + 1;
                  FUN_142ef7ba0(piVar9,lVar10,(ulonglong)((int)lVar13 + 1) * 2);
                  *plVar5 = (longlong)piVar9;
                  if (piVar9 == (int *)0x0) goto LAB_140cc1694;
                }
              }
              local_60 = plVar5;
              if (plVar5 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x8007000e);
              }
              if (local_b0 != 0) {
                FUN_1401bebb0(local_b0 + -0x10);
              }
              local_f0 = 0;
              local_e8 = plVar17;
              if (plVar17 != (longlong *)0x0) {
                (**(code **)(*plVar17 + 8))(plVar17);
              }
              plVar6 = (longlong *)FUN_140912780(&local_a8,&local_e8,L"vslot",&local_f0);
              lVar10 = *plVar6;
              plVar7 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
              plVar6 = (longlong *)0x0;
              local_58 = plVar7;
              if (plVar7 != (longlong *)0x0) {
                plVar7[1] = 0;
                *(undefined4 *)(plVar7 + 2) = 1;
                plVar6 = plVar7;
                if (lVar10 == 0) {
                  *plVar7 = 0;
                }
                else {
                  lVar13 = -1;
                  do {
                    lVar13 = lVar13 + 1;
                  } while (*(short *)(lVar10 + lVar13 * 2) != 0);
                  piVar9 = (int *)(*DAT_143ad5980)();
                  if (piVar9 == (int *)0x0) {
                    *plVar7 = 0;
LAB_140cc16a4:
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x8007000e);
                  }
                  *piVar9 = (int)lVar13 * 2;
                  piVar9 = piVar9 + 1;
                  FUN_142ef7ba0(piVar9,lVar10,(ulonglong)((int)lVar13 + 1) * 2);
                  *plVar7 = (longlong)piVar9;
                  if (piVar9 == (int *)0x0) goto LAB_140cc16a4;
                }
              }
              local_50 = plVar6;
              if (plVar6 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x8007000e);
              }
              if (local_a8 != 0) {
                FUN_1401bebb0(local_a8 + -0x10);
              }
              lVar10 = *param_4;
              puVar14 = (undefined8 *)0x0;
              uVar16 = 0;
              if (lVar10 != 0) {
                uVar16 = *(uint *)(lVar10 + -8);
              }
              if (((int)local_168 < 0) || (uVar16 <= local_168)) {
                puVar8 = puVar14;
                if (lVar10 != 0) {
                  puVar8 = (undefined8 *)(ulonglong)*(uint *)(lVar10 + -8);
                }
                FUN_142e54290(0xbc,local_168,puVar8);
                lVar10 = *param_4;
              }
              puVar8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
              local_48 = puVar8;
              if (puVar8 != (undefined8 *)0x0) {
                puVar8[1] = 0;
                *(undefined4 *)(puVar8 + 2) = 1;
                uVar19 = (ulonglong)in_stack_fffffffffffffe80 & 0xffffffff00000000;
                iVar3 = (*DAT_1432627f8)(0xfde9,0,&DAT_1434b2af1,0xffffffff,0,uVar19);
                uVar20 = (undefined4)(uVar19 >> 0x20);
                iVar3 = (int)((ulonglong)(longlong)(iVar3 * 2) >> 1);
                uVar16 = iVar3 - 1;
                piVar9 = (int *)(*DAT_143ad5980)();
                if (piVar9 == (int *)0x0) {
                  piVar9 = (int *)0x0;
                }
                else {
                  *piVar9 = uVar16 * 2;
                  piVar9 = piVar9 + 1;
                  *(undefined2 *)((longlong)piVar9 + (ulonglong)uVar16 * 2) = 0;
                }
                piVar18 = piVar9;
                (*DAT_1432627f8)(0xfde9,0,&DAT_1434b2af1,0xffffffff,piVar9,CONCAT44(uVar20,iVar3));
                uVar20 = (undefined4)((ulonglong)piVar18 >> 0x20);
                *puVar8 = piVar9;
                puVar14 = puVar8;
              }
              local_e0 = puVar14;
              if (puVar14 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x8007000e);
              }
              local_d8 = plVar17;
              if (plVar17 != (longlong *)0x0) {
                (**(code **)(*plVar17 + 8))(plVar17);
              }
              plVar17 = local_128;
              in_stack_fffffffffffffe80 = &local_d8;
              in_stack_fffffffffffffe78 = CONCAT44(uVar20,param_3);
              FUN_140cc0300((longlong)(int)local_168 * 0xa8 + lVar10,*plVar5,*plVar6,
                            *(undefined8 *)(local_160 + *local_128),in_stack_fffffffffffffe78,
                            in_stack_fffffffffffffe80,&local_e0,0);
              LOCK();
              plVar7 = plVar6 + 2;
              lVar10 = *plVar7;
              *(int *)plVar7 = (int)*plVar7 + -1;
              UNLOCK();
              if ((int)lVar10 == 1) {
                if (*plVar6 != 0) {
                  (*DAT_143ad5990)(*plVar6 + -4);
                  *plVar6 = 0;
                }
                if (plVar6[1] != 0) {
                  FUN_14019b4e0();
                  plVar6[1] = 0;
                }
                thunk_FUN_140205820(plVar6,0x18);
              }
              LOCK();
              plVar6 = plVar5 + 2;
              lVar10 = *plVar6;
              *(int *)plVar6 = (int)*plVar6 + -1;
              UNLOCK();
              if ((int)lVar10 == 1) {
                if (*plVar5 != 0) {
                  (*DAT_143ad5990)(*plVar5 + -4);
                  *plVar5 = 0;
                }
                if (plVar5[1] != 0) {
                  FUN_14019b4e0();
                  plVar5[1] = 0;
                }
                thunk_FUN_140205820(plVar5,0x18);
              }
              if (local_138 != (longlong *)0x0) {
                (**(code **)(*local_138 + 0x10))();
              }
              local_164 = local_164 + 1;
              local_160 = local_160 + 8;
              lVar10 = *plVar17;
              puVar8 = local_130;
            } while ((ulonglong)(longlong)local_164 < (ulonglong)(plVar17[1] - lVar10 >> 3));
          }
        }
        local_168 = local_168 + 1;
        local_148 = local_148 + 1;
        uVar16 = FUN_140cc3530(param_4);
      } while (local_168 < uVar16);
    }
    uVar11 = 1;
  }
  else {
    uVar11 = 0;
  }
  return uVar11;
}



//===========================================================
// FUN_140388e30 @ 140388e30   (116 bytes)
//===========================================================

undefined8 FUN_140388e30(undefined8 param_1,undefined4 param_2)

{
  undefined *puVar1;
  int iVar2;
  undefined8 uVar3;
  longlong *local_res18;
  longlong *local_res20;
  
  FUN_14039f600(param_1,&local_res20,param_2,0);
  puVar1 = PTR_u_unchangeable_143a46518;
  if (local_res20 != (longlong *)0x0) {
    local_res18 = local_res20;
    (**(code **)(*local_res20 + 8))(local_res20);
    iVar2 = FUN_140910eb0(&local_res18,puVar1,0);
    if (iVar2 != 0) {
      uVar3 = 1;
      goto LAB_140388e88;
    }
  }
  uVar3 = 0;
LAB_140388e88:
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))(local_res20);
  }
  return uVar3;
}



//===========================================================
// FUN_142cf5c80 @ 142cf5c80   (96 bytes)
//===========================================================

undefined8 FUN_142cf5c80(longlong param_1)

{
  ulonglong uVar1;
  
  if (*(int *)(param_1 + 0x3288) == 0) {
    return 0;
  }
  if ((((*(int *)(param_1 + 0x2298) == 0) && (*(int *)(param_1 + 0x229c) == 0)) &&
      (*(int *)(param_1 + 0x2294) == 0)) &&
     ((uVar1 = FUN_1401b0340(param_1 + 0x22c8), (uVar1 & 0x10) == 0 &&
      (uVar1 = FUN_142d2e710(param_1 + 0x2210), (uVar1 & 0x20) == 0)))) {
    return 0;
  }
  return 1;
}



//===========================================================
// FUN_1402536d0 @ 1402536d0   (15 bytes)
//===========================================================

bool FUN_1402536d0(int param_1)

{
  return param_1 - 0x708U < 0x33;
}



//===========================================================
// FUN_14038cfa0 @ 14038cfa0   (160 bytes)
//===========================================================

undefined8 FUN_14038cfa0(undefined8 param_1,undefined4 param_2)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  longlong *local_res18;
  longlong *local_res20;
  
  FUN_14039f600(param_1,&local_res20,param_2,0);
  if (local_res20 == (longlong *)0x0) {
    uVar3 = 0;
  }
  else {
    local_res18 = local_res20;
    (**(code **)(*local_res20 + 8))(local_res20);
    iVar1 = FUN_140910eb0(&local_res18,L"expRate",0);
    local_res18 = local_res20;
    (**(code **)(*local_res20 + 8))(local_res20);
    iVar2 = FUN_140910eb0(&local_res18,L"bloodAllianceExpRate",0);
    if ((iVar1 < 1) && (iVar2 < 1)) {
      uVar3 = 0;
    }
    else {
      uVar3 = 1;
    }
  }
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))(local_res20);
  }
  return uVar3;
}



//===========================================================
// FUN_1417ea700 @ 1417ea700   (42 bytes)
//===========================================================

undefined8 FUN_1417ea700(int param_1)

{
  if (((0x1f < param_1 - 3000U) && (0x1f < param_1 - 0xc1cU)) && (0x1f < param_1 - 0xc80U)) {
    return 0;
  }
  return 1;
}


