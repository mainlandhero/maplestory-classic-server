
//===========================================================
// FUN_142656500 @ 142656500   (424 bytes)
//===========================================================

void FUN_142656500(undefined8 param_1,undefined4 param_2,undefined4 param_3,undefined8 param_4,
                  undefined4 param_5,undefined8 param_6,undefined4 param_7,undefined4 param_8,
                  undefined8 param_9,undefined4 param_10,undefined4 param_11,undefined4 param_12,
                  undefined4 param_13,undefined4 param_14,undefined8 param_15,undefined8 param_16,
                  undefined1 param_17,undefined8 param_18,undefined8 param_19,undefined1 param_20,
                  longlong *param_21,undefined4 param_22,undefined4 param_23,undefined8 param_24,
                  undefined1 param_25,undefined8 param_26)

{
  undefined8 local_18 [2];
  
  local_18[0] = 0;
  FUN_14019a260(local_18,param_21);
  FUN_14264f750(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_10,
                param_11,param_12,param_13,param_14,param_15,param_16,param_17,param_18,param_19,
                param_20,local_18,param_22,param_23,param_24,param_25,param_26);
  if (*param_21 != 0) {
    FUN_14019f2c0(*param_21 + -0x10);
  }
  return;
}



//===========================================================
// FUN_142664790 @ 142664790   (940 bytes)
//===========================================================

void FUN_142664790(longlong param_1,int param_2,int param_3,undefined8 param_4,undefined8 param_5,
                  longlong *param_6,undefined4 param_7,undefined4 param_8)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong *plVar3;
  longlong lVar4;
  longlong local_res8;
  undefined8 local_68;
  longlong *local_60;
  longlong *local_58;
  longlong *local_50;
  undefined8 local_48;
  longlong lStack_40;
  undefined8 local_38;
  
  if (((*(longlong *)(param_1 + 0x48) != 0) && (*(int *)(param_1 + 0x80) == param_2)) &&
     (*(int *)(param_1 + 0x84) == param_3)) {
    lVar4 = *param_6;
    goto LAB_142664b0c;
  }
  local_48 = 1;
  lStack_40 = 0;
  local_38 = 0;
  local_res8 = 0;
  local_60 = &local_res8;
  local_68 = 0;
  FUN_14019a260(&local_68,&local_res8);
  FUN_14264f750(param_1,0,0,param_4,0,0,0,0,0,param_7,0,0,0,0,0,0,0,0,0,0,&local_68,0,param_8,0,0,
                &local_48);
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  if (lStack_40 != 0) {
    FUN_14019f2c0(lStack_40 + -0x10);
  }
  plVar1 = *(longlong **)(param_1 + 0x48);
  *(undefined8 *)(param_1 + 0x48) = 0;
  plVar2 = *(longlong **)(param_1 + 0x50);
  local_60 = plVar1;
  local_58 = plVar2;
  if (plVar2 != (longlong *)0x0) {
    (**(code **)(*plVar2 + 8))(plVar2);
  }
  local_48 = 1;
  lStack_40 = 0;
  local_38 = 0;
  local_res8 = 0;
  FUN_14019a260(&local_res8,param_6);
  local_50 = &local_res8;
  local_68 = 0;
  FUN_14019a260(&local_68,&local_res8);
  FUN_14264f750(param_1,param_2,param_3,param_5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,&local_68,param_7,0,
                0,0,&local_48);
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  if (lStack_40 != 0) {
    FUN_14019f2c0(lStack_40 + -0x10);
  }
  plVar3 = *(longlong **)(param_1 + 0x68);
  if (plVar3 != plVar1) {
    if (plVar1 != (longlong *)0x0) {
      (**(code **)(*plVar1 + 8))(plVar1);
      plVar3 = *(longlong **)(param_1 + 0x68);
    }
    *(longlong **)(param_1 + 0x68) = plVar1;
    if (plVar3 != (longlong *)0x0) {
      (**(code **)(*plVar3 + 0x10))();
    }
  }
  if ((plVar2 == (longlong *)0x0) || (*(longlong *)(param_1 + 0x68) != 0)) {
    if (plVar2 != (longlong *)0x0) {
      plVar3 = *(longlong **)(param_1 + 0x70);
      if (plVar3 != plVar2) {
        if (plVar2 != (longlong *)0x0) {
          (**(code **)(*plVar2 + 8))(plVar2);
          plVar3 = *(longlong **)(param_1 + 0x70);
        }
        *(longlong **)(param_1 + 0x70) = plVar2;
        if (plVar3 != (longlong *)0x0) {
          (**(code **)(*plVar3 + 0x10))();
        }
      }
      if (*(longlong **)(param_1 + 0x50) != (longlong *)0x0) {
        (**(code **)(**(longlong **)(param_1 + 0x50) + 0x10))();
      }
      goto LAB_142664ad0;
    }
  }
  else {
    (**(code **)(*plVar2 + 8))(plVar2);
    plVar3 = *(longlong **)(param_1 + 0x68);
    *(longlong **)(param_1 + 0x68) = plVar2;
    if (plVar3 != (longlong *)0x0) {
      (**(code **)(*plVar3 + 0x10))();
    }
    if (*(longlong **)(param_1 + 0x50) != (longlong *)0x0) {
      (**(code **)(**(longlong **)(param_1 + 0x50) + 0x10))();
    }
LAB_142664ad0:
    *(undefined8 *)(param_1 + 0x50) = 0;
  }
  FUN_142664b50(param_1,param_2,param_3,0);
  if (plVar2 != (longlong *)0x0) {
    (**(code **)(*plVar2 + 0x10))(plVar2);
  }
  if (plVar1 != (longlong *)0x0) {
    (**(code **)(*plVar1 + 0x10))(plVar1);
  }
  lVar4 = *param_6;
LAB_142664b0c:
  if (lVar4 != 0) {
    FUN_14019f2c0(lVar4 + -0x10);
  }
  return;
}



//===========================================================
// FUN_142687b40 @ 142687b40   (1995 bytes)
//===========================================================

undefined8
FUN_142687b40(undefined8 param_1,undefined4 param_2,undefined4 param_3,longlong param_4,
             undefined8 param_5,undefined4 param_6,undefined8 param_7,undefined1 param_8,
             longlong param_9,undefined1 param_10)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined8 *puVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong *plVar11;
  longlong lVar12;
  int *piVar13;
  undefined8 uVar14;
  undefined4 *puVar15;
  uint uVar16;
  uint uVar17;
  int *piVar18;
  undefined8 in_stack_fffffffffffffe08;
  uint uVar19;
  undefined8 in_stack_fffffffffffffe18;
  undefined8 in_stack_fffffffffffffe20;
  undefined4 uVar20;
  undefined8 in_stack_fffffffffffffe28;
  uint uVar21;
  undefined8 in_stack_fffffffffffffe30;
  undefined4 uVar22;
  undefined8 in_stack_fffffffffffffe40;
  undefined4 uVar23;
  undefined8 in_stack_fffffffffffffe48;
  uint uVar24;
  int *local_148;
  int *local_140;
  undefined8 local_138;
  undefined4 *puStack_130;
  undefined8 local_128;
  int **local_120;
  undefined1 local_118 [8];
  undefined8 *local_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined4 local_d0;
  undefined4 *local_c8;
  undefined1 local_c0;
  undefined4 local_b8;
  longlong local_b0;
  undefined4 local_a8;
  undefined1 local_98 [8];
  longlong local_90;
  int local_88 [2];
  longlong local_80;
  undefined1 local_78 [8];
  longlong local_70;
  int *local_68;
  undefined8 local_60;
  
  if (*(longlong *)(param_4 + 8) == 0) {
    FUN_1401d1a30(param_4);
    FUN_1401d1a30(param_9);
  }
  else {
    if ((DAT_143aa84a0 != 0) && (DAT_143aa8328 != 0)) {
      FUN_14019a5d0(*(longlong *)(param_4 + 8) + 0x28);
      lVar9 = FUN_140d83bc0(param_4);
      if (((9999999 < *(int *)(lVar9 + 0x20) + 0xfb3b4c00U) ||
          (*(int *)(lVar9 + 0x20) + 0xfb2c09c0U < 1000000)) &&
         (lVar9 = FUN_140d83bc0(param_4), 999999 < *(int *)(lVar9 + 0x20) + 0xfb2c09c0U)) {
        lVar9 = FUN_140d83bc0(param_4);
        iVar5 = FUN_14019a5d0(lVar9 + 0x28);
        lVar9 = DAT_143aa8328;
        if (9999 < iVar5 - 9100000U) {
          lVar10 = FUN_140d83bc0(param_4);
          uVar6 = FUN_14019a5d0(lVar10 + 0x28);
          iVar5 = FUN_140389c10(lVar9,uVar6);
          if (iVar5 == 0) {
            FUN_1401d1a30(param_4);
            FUN_1401d1a30(param_9);
            return 0;
          }
        }
      }
      piVar13 = (int *)0x0;
      local_148 = (int *)0x0;
      lVar9 = *(longlong *)(param_4 + 8);
      if (lVar9 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar9 = *(longlong *)(param_4 + 8);
      }
      uVar20 = (undefined4)((ulonglong)in_stack_fffffffffffffe20 >> 0x20);
      uVar22 = (undefined4)((ulonglong)in_stack_fffffffffffffe30 >> 0x20);
      uVar23 = (undefined4)((ulonglong)in_stack_fffffffffffffe40 >> 0x20);
      uVar24 = (uint)((ulonglong)in_stack_fffffffffffffe48 >> 0x20);
      uVar21 = (uint)((ulonglong)in_stack_fffffffffffffe28 >> 0x20);
      uVar6 = (undefined4)((ulonglong)in_stack_fffffffffffffe18 >> 0x20);
      uVar19 = (uint)((ulonglong)in_stack_fffffffffffffe08 >> 0x20);
      piVar18 = piVar13;
      if (*(int *)(lVar9 + 0x7c) != 0) {
        plVar11 = (longlong *)FUN_142cc41c0(DAT_143aa84a0);
        lVar9 = 0;
        while( true ) {
          uVar20 = (undefined4)((ulonglong)in_stack_fffffffffffffe20 >> 0x20);
          uVar22 = (undefined4)((ulonglong)in_stack_fffffffffffffe30 >> 0x20);
          uVar23 = (undefined4)((ulonglong)in_stack_fffffffffffffe40 >> 0x20);
          uVar24 = (uint)((ulonglong)in_stack_fffffffffffffe48 >> 0x20);
          uVar21 = (uint)((ulonglong)in_stack_fffffffffffffe28 >> 0x20);
          uVar6 = (undefined4)((ulonglong)in_stack_fffffffffffffe18 >> 0x20);
          uVar19 = (uint)((ulonglong)in_stack_fffffffffffffe08 >> 0x20);
          lVar10 = *plVar11;
          piVar18 = local_148;
          if ((lVar10 == 0) || (uVar17 = (uint)piVar13, *(uint *)(lVar10 + -8) <= uVar17)) break;
          if ((int)uVar17 < 0) {
            FUN_142e54290(0xbc,piVar13);
            lVar10 = *plVar11;
          }
          piVar18 = (int *)(lVar9 + lVar10);
          if (piVar18 != (int *)0x0) {
            if (*(longlong *)(param_4 + 8) == 0) {
              FUN_142e52ed0(0x431,0);
            }
            iVar5 = FUN_14019a5d0();
            if (iVar5 == *piVar18) {
              lVar10 = 0;
              do {
                piVar18 = piVar18 + 1;
                if (*piVar18 == 0) break;
                lVar12 = *(longlong *)(param_4 + 8);
                if (lVar12 == 0) {
                  FUN_142e52ed0(0x431,0);
                  lVar12 = *(longlong *)(param_4 + 8);
                }
                uVar20 = (undefined4)((ulonglong)in_stack_fffffffffffffe20 >> 0x20);
                uVar22 = (undefined4)((ulonglong)in_stack_fffffffffffffe30 >> 0x20);
                uVar23 = (undefined4)((ulonglong)in_stack_fffffffffffffe40 >> 0x20);
                uVar24 = (uint)((ulonglong)in_stack_fffffffffffffe48 >> 0x20);
                uVar21 = (uint)((ulonglong)in_stack_fffffffffffffe28 >> 0x20);
                uVar6 = (undefined4)((ulonglong)in_stack_fffffffffffffe18 >> 0x20);
                uVar19 = (uint)((ulonglong)in_stack_fffffffffffffe08 >> 0x20);
                if (*piVar18 == *(int *)(lVar12 + 0x20)) {
                  lVar9 = *plVar11;
                  if (lVar9 == 0) {
                    uVar16 = 0;
                  }
                  else {
                    uVar16 = *(uint *)(lVar9 + -8);
                  }
                  if (((int)uVar17 < 0) || (uVar16 <= uVar17)) {
                    if (lVar9 == 0) {
                      uVar7 = 0;
                    }
                    else {
                      uVar7 = *(undefined4 *)(lVar9 + -8);
                    }
                    FUN_142e54290(0xbc,piVar13,uVar7);
                    lVar9 = *plVar11;
                  }
                  local_148 = (int *)((longlong)(int)uVar17 * 0x80 + lVar9);
                  piVar18 = local_148;
                  goto LAB_142687d9e;
                }
                lVar10 = lVar10 + 1;
              } while (lVar10 < 10);
            }
          }
          piVar13 = (int *)(ulonglong)(uVar17 + 1);
          lVar9 = lVar9 + 0x80;
        }
      }
LAB_142687d9e:
      lVar9 = *(longlong *)(param_4 + 8);
      if (lVar9 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar9 = *(longlong *)(param_4 + 8);
      }
      iVar5 = FUN_14019a5d0(lVar9 + 0x28);
      if (iVar5 - 9100000U < 10000) {
        lVar9 = *(longlong *)(param_4 + 8);
        local_90 = lVar9;
        if (lVar9 != 0) {
          if (0xfffff < *(ulonglong *)(lVar9 + 8)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar9 + 8) = *(longlong *)(lVar9 + 8) + 1;
          UNLOCK();
        }
        FUN_142679640(param_1,param_2,param_3,local_98,CONCAT44(uVar19,param_6),piVar18,param_7);
      }
      else {
        lVar9 = *(longlong *)(param_4 + 8);
        if (lVar9 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar9 = *(longlong *)(param_4 + 8);
        }
        FUN_14019a5d0(lVar9 + 0x28);
        local_140 = local_88;
        lVar9 = *(longlong *)(param_9 + 8);
        local_80 = lVar9;
        if (lVar9 != 0) {
          if (0xfffff < *(ulonglong *)(lVar9 + 8)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar9 + 8) = *(longlong *)(lVar9 + 8) + 1;
          UNLOCK();
        }
        lVar9 = *(longlong *)(param_4 + 8);
        local_70 = lVar9;
        if (lVar9 != 0) {
          if (0xfffff < *(ulonglong *)(lVar9 + 8)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar9 + 8) = *(longlong *)(lVar9 + 8) + 1;
          UNLOCK();
        }
        uVar7 = FUN_141059c70(local_78,local_88);
        lVar9 = *(longlong *)(param_4 + 8);
        if (lVar9 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar9 = *(longlong *)(param_4 + 8);
        }
        uVar8 = FUN_14019a5d0(lVar9 + 0x28);
        iVar5 = FUN_140253d30(uVar8,2);
        lVar10 = DAT_143aa8328;
        lVar9 = *(longlong *)(param_4 + 8);
        if (lVar9 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar9 = *(longlong *)(param_4 + 8);
        }
        uVar8 = FUN_14019a5d0(lVar9 + 0x28);
        lVar9 = (ulonglong)uVar19 << 0x20;
        FUN_1403d2200(lVar10,local_118,uVar8,0,lVar9);
        uVar8 = (undefined4)((ulonglong)lVar9 >> 0x20);
        if (iVar5 == 0xc) {
          local_138 = 1;
          puStack_130 = (undefined4 *)0x0;
          local_128 = 0;
          local_120 = &local_140;
          local_140 = (int *)0x0;
          piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
          piVar13[1] = 0;
          *piVar13 = -1;
          local_140 = piVar13 + 4;
          piVar13[2] = 0;
          *(undefined1 *)local_140 = 0;
          if (*piVar13 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar13[1] < 0) {
            FUN_142e54290(0x90,piVar13[1],0);
          }
          *piVar13 = 1;
          *(undefined1 *)local_140 = 0;
          if (piVar13[1] + 1 < 1) {
            FUN_142e54290(0x9c);
          }
          piVar13[2] = 0;
          lVar9 = *(longlong *)(param_4 + 8);
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431);
            lVar9 = *(longlong *)(param_4 + 8);
          }
          uVar1 = *(undefined4 *)(lVar9 + 0x20);
          uVar2 = *(undefined4 *)(lVar9 + 0x44);
          uVar3 = *(undefined4 *)(lVar9 + 0x54);
          uVar14 = FUN_140192f00(local_110);
          FUN_14264f750(param_1,param_2,param_3,uVar14,CONCAT44(uVar8,uVar3),0,CONCAT44(uVar6,uVar7)
                        ,CONCAT44(uVar20,uVar2),local_148,CONCAT44(uVar22,0xc),param_6,
                        CONCAT44(uVar23,uVar1),(ulonglong)uVar24 << 0x20,0,param_7,0,0,0,0,param_8,
                        &local_140,0,0,0,param_10,&local_138);
          puVar15 = puStack_130;
        }
        else {
          local_108 = 0;
          uStack_100 = 0;
          local_f8 = 0;
          local_f0 = 0;
          local_e8 = 0;
          uStack_e0 = 0;
          local_d8 = 0;
          local_d0 = 0;
          local_c8 = (undefined4 *)0x0;
          local_c0 = 1;
          local_b8 = 1;
          local_b0 = 0;
          local_a8 = 0;
          puVar15 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
          puVar15[1] = 0;
          *puVar15 = 0xffffffff;
          local_c8 = puVar15 + 4;
          puVar15[2] = 0;
          *(undefined1 *)local_c8 = 0;
          *puVar15 = 1;
          *(undefined1 *)local_c8 = 0;
          puVar15[2] = 0;
          local_f8 = *(undefined8 *)(param_4 + 8);
          local_f0 = CONCAT44(1,uVar7);
          uStack_100 = param_7;
          local_120 = &local_68;
          local_60 = 0;
          local_148 = (int *)0x0;
          piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
          piVar13[1] = 0;
          *piVar13 = -1;
          local_148 = piVar13 + 4;
          piVar13[2] = 0;
          *(undefined1 *)local_148 = 0;
          if (*piVar13 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar13[1] < 0) {
            FUN_142e54290(0x90,piVar13[1],0);
          }
          *piVar13 = 1;
          *(undefined1 *)local_148 = 0;
          if (piVar13[1] + 1 < 1) {
            FUN_142e54290(0x9c,0);
          }
          piVar13[2] = 0;
          FUN_142694130(param_1,param_2,param_3,local_110,&local_108,piVar18,CONCAT44(uVar6,param_6)
                        ,0,(ulonglong)uVar21 << 0x20,0,param_8,&local_148,&local_68,param_10);
          puVar15 = local_c8;
          if (local_b0 != 0) {
            FUN_14019f2c0(local_b0 + -0x10);
            puVar15 = local_c8;
          }
        }
        if (puVar15 != (undefined4 *)0x0) {
          FUN_14019f2c0(puVar15 + -4);
        }
        puVar4 = local_110;
        if (local_110 != (undefined8 *)0x0) {
          if (0xffffe < local_110[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar11 = puVar4 + 1;
          lVar9 = *plVar11;
          *plVar11 = *plVar11 + -1;
          UNLOCK();
          if (((int)lVar9 == 1) && (local_110 != (undefined8 *)0x0)) {
            (**(code **)*local_110)(local_110,1);
          }
        }
      }
      FUN_1401d1a30(param_4);
      FUN_1401d1a30(param_9);
      return 1;
    }
    FUN_1401d1a30(param_4);
    FUN_1401d1a30(param_9);
  }
  return 0;
}


