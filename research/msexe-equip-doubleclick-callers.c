
//===========================================================
// FUN_141784fa0 @ 141784fa0   (8725 bytes)
//===========================================================

undefined8 FUN_141784fa0(longlong *param_1)

{
  longlong lVar1;
  undefined1 uVar2;
  char cVar3;
  short sVar4;
  int iVar5;
  int iVar6;
  undefined4 uVar7;
  longlong lVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  undefined4 *puVar11;
  longlong *plVar12;
  int *piVar13;
  longlong lVar14;
  int iVar15;
  int **ppiVar16;
  int *piVar17;
  longlong *plVar18;
  longlong *plVar19;
  uint uVar20;
  uint uVar21;
  longlong *plVar22;
  int *local_res10;
  int *local_res18;
  undefined8 local_res20;
  undefined8 *in_stack_fffffffffffffee8;
  uint in_stack_fffffffffffffef0;
  ulonglong in_stack_fffffffffffffef8;
  ulonglong in_stack_ffffffffffffff00;
  uint uVar23;
  int *local_d8;
  undefined8 *local_d0;
  longlong local_c8;
  undefined8 *local_c0;
  int *local_b8 [2];
  undefined1 local_a8 [8];
  undefined8 *local_a0;
  undefined1 local_98 [8];
  undefined8 *local_90;
  int **local_88;
  int **local_80;
  longlong *local_78;
  undefined1 local_70 [8];
  longlong *local_68;
  undefined1 local_60 [8];
  longlong *local_58;
  undefined1 local_50 [8];
  undefined8 *local_48;
  
  iVar5 = FUN_1417da100(0);
  if (iVar5 == 0) {
    return 0;
  }
  if (((DAT_143aa8520 != (longlong *)0x0) &&
      (iVar5 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a548),
      iVar5 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
    return 0;
  }
  FUN_141892890();
  plVar18 = (longlong *)param_1[8];
  plVar12 = (longlong *)0x0;
  uVar21 = 0;
  if ((plVar18 == (longlong *)0x0) ||
     (iVar5 = (**(code **)(*plVar18 + 0xd0))(plVar18,&PTR_PTR_143a8a928), lVar14 = DAT_143aa8518,
     piVar17 = DAT_143aa84a0, iVar5 == 0)) {
    plVar18 = (longlong *)param_1[8];
LAB_141786ba3:
    if ((plVar18 == (longlong *)0x0) ||
       (iVar5 = (**(code **)(*plVar18 + 0xd0))(plVar18,&PTR_PTR_143a8a5d0), iVar5 == 0)) {
      plVar18 = (longlong *)param_1[8];
    }
    else {
      plVar18 = (longlong *)param_1[8];
      plVar22 = plVar18 + -1;
      if (plVar18 == (longlong *)0x0) {
        plVar22 = plVar12;
      }
      if (plVar22 != (longlong *)0x0) {
        local_res18 = DAT_143aa84a0;
        if (DAT_143aa84a0 == (int *)0x0) {
          return 0;
        }
        uVar9 = FUN_142cbe730();
        lVar14 = DAT_143aa8518;
        if (DAT_143aa8518 == 0) {
          return 0;
        }
        local_d8 = (int *)0x0;
        iVar5 = FUN_140255650(*(undefined4 *)((longlong)param_1 + 0x34),(int)param_1[6]);
        if (iVar5 == 0) {
          return 0;
        }
        lVar8 = FUN_14022f570(&local_c8,uVar9,*(undefined4 *)((longlong)param_1 + 0x34),
                              (int)param_1[6]);
        puVar10 = local_c0;
        lVar8 = *(longlong *)(lVar8 + 8);
        if (local_c0 != (undefined8 *)0x0) {
          if (0xffffe < local_c0[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar18 = puVar10 + 1;
          lVar1 = *plVar18;
          *plVar18 = *plVar18 + -1;
          UNLOCK();
          if (((int)lVar1 == 1) && (local_c0 != (undefined8 *)0x0)) {
            (**(code **)*local_c0)(local_c0,1);
          }
          local_c0 = (undefined8 *)0x0;
        }
        if (lVar8 == 0) {
          return 0;
        }
        if (((int)param_1[6] == 2) &&
           (iVar5 = FUN_141787410(param_1,lVar14,lVar8,local_res18,&local_d8), iVar5 == 0)) {
          return 0;
        }
        uVar7 = FUN_14019a5d0(lVar8 + 0x20);
        local_res10 = (int *)CONCAT44(local_res10._4_4_,uVar7);
        cVar3 = FUN_141892d00();
        if ((cVar3 == '\0') ||
           (cVar3 = FUN_140419bd0((ulonglong)local_res10 & 0xffffffff), cVar3 == '\0')) {
          local_88 = &local_res10;
          local_80 = &local_res18;
          local_78 = param_1;
          iVar5 = FUN_14178dc90(&local_88);
          if (iVar5 != 0) {
            return 1;
          }
          iVar5 = FUN_142cc42d0(local_res18,200,0);
          if (iVar5 == 0) {
            return 0;
          }
          uVar9 = FUN_14178c840(param_1,(ulonglong)local_res10 & 0xffffffff);
          return uVar9;
        }
        uVar9 = FUN_1408a9e40(local_b8,0x12b2);
        FUN_1415eca30(uVar9,0xb);
        piVar17 = local_b8[0];
LAB_141786cf9:
        if (piVar17 == (int *)0x0) {
          return 0;
        }
        FUN_14019f2c0(piVar17 + -4);
        return 0;
      }
    }
    if ((plVar18 == (longlong *)0x0) ||
       (iVar5 = (**(code **)(*plVar18 + 0xd0))(plVar18,&PTR_PTR_143a87d48), iVar5 == 0)) {
      plVar18 = (longlong *)param_1[8];
    }
    else {
      plVar18 = (longlong *)param_1[8];
      plVar22 = plVar18 + -1;
      if (plVar18 == (longlong *)0x0) {
        plVar22 = plVar12;
      }
      if (plVar22 != (longlong *)0x0) {
        if ((plVar18 == (longlong *)0x0) ||
           (iVar5 = (**(code **)(*plVar18 + 0xd0))(plVar18,&PTR_PTR_143a87d48), iVar5 == 0)) {
          FUN_142cbe730(DAT_143aa84a0);
        }
        else {
          plVar18 = (longlong *)(param_1[8] + -8);
          if (param_1[8] == 0) {
            plVar18 = plVar12;
          }
          FUN_142cbe730(DAT_143aa84a0);
          if ((plVar18 != (longlong *)0x0) &&
             (iVar5 = FUN_140255740(*(int *)((longlong)plVar18 + 0x11fc)), iVar5 != 0)) {
            return 0;
          }
        }
        goto LAB_141786de4;
      }
    }
    if ((plVar18 == (longlong *)0x0) ||
       (iVar5 = (**(code **)(*plVar18 + 0xd0))(plVar18,&PTR_PTR_143a8a968), iVar5 == 0)) {
      plVar18 = (longlong *)param_1[8];
    }
    else {
      plVar18 = (longlong *)param_1[8];
      plVar22 = plVar18 + -1;
      if (plVar18 == (longlong *)0x0) {
        plVar22 = plVar12;
      }
      if (plVar22 != (longlong *)0x0) {
        if (DAT_143aa84a0 == (int *)0x0) {
          return 0;
        }
        uVar9 = FUN_142cbe730();
        lVar14 = FUN_1402e3cd0(uVar9,&local_d8,1,*(undefined4 *)((longlong)param_1 + 0x34));
        puVar10 = local_d0;
        lVar14 = *(longlong *)(lVar14 + 8);
        if (local_d0 != (undefined8 *)0x0) {
          if (0xffffe < local_d0[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar18 = puVar10 + 1;
          lVar8 = *plVar18;
          *plVar18 = *plVar18 + -1;
          UNLOCK();
          if (((int)lVar8 == 1) && (local_d0 != (undefined8 *)0x0)) {
            (**(code **)*local_d0)(local_d0,1);
          }
          local_d0 = (undefined8 *)0x0;
        }
        if (lVar14 == 0) {
          return 0;
        }
        lVar14 = FUN_140192f00(lVar14);
        if (lVar14 == 0) {
          return 0;
        }
        plVar18 = (longlong *)param_1[8];
        plVar22 = plVar12;
        if (((plVar18 != (longlong *)0x0) &&
            (iVar5 = (**(code **)(*plVar18 + 0xd0))(plVar18,&PTR_PTR_143a8a968), iVar5 != 0)) &&
           (plVar22 = (longlong *)(param_1[8] + -8), param_1[8] == 0)) {
          plVar22 = plVar12;
        }
        FUN_1423c4440(plVar22,*(undefined4 *)((longlong)param_1 + 0x34),lVar14);
        FUN_1429edb20(PTR_u_DragEnd_143a47bd8);
        return 1;
      }
    }
    if ((plVar18 != (longlong *)0x0) &&
       (iVar5 = (**(code **)(*plVar18 + 0xd0))(plVar18,&PTR_PTR_143a8ad18), iVar5 != 0)) {
      plVar18 = (longlong *)(param_1[8] + -8);
      if (param_1[8] == 0) {
        plVar18 = plVar12;
      }
      if (plVar18 != (longlong *)0x0) {
        FUN_1426e9e70(DAT_143aceac0,(int)param_1[7],0);
        return 1;
      }
    }
    if ((((int)param_1[6] != 1) && ((int)param_1[6] != 6)) ||
       ((-1 < *(int *)((longlong)param_1 + 0x34) ||
        (iVar5 = (**(code **)(*(longlong *)param_1[8] + 0x50))(), iVar5 == 0)))) {
LAB_141786de4:
      uVar7 = FUN_141d5dd80(param_1[8]);
      cVar3 = FUN_141d5ddd0(uVar7);
      if (cVar3 == '\0') {
        return 0;
      }
      uVar9 = (**(code **)(*param_1 + 0x20))(param_1,uVar7);
      return uVar9;
    }
    lVar14 = FUN_142cbe730(DAT_143aa84a0);
    if ((int)param_1[6] == 1) {
      lVar8 = FUN_1402e3cd0(lVar14,local_98,1,*(undefined4 *)((longlong)param_1 + 0x34));
      puVar10 = local_90;
      plVar18 = *(longlong **)(lVar8 + 8);
      if (local_90 != (undefined8 *)0x0) {
        if (0xffffe < local_90[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = puVar10 + 1;
        lVar8 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if (((int)lVar8 == 1) && (local_90 != (undefined8 *)0x0)) {
          (**(code **)*local_90)(local_90,1);
        }
        local_90 = (undefined8 *)0x0;
      }
      if (plVar18 == (longlong *)0x0) {
        return 0;
      }
      iVar5 = *(int *)((longlong)param_1 + 0x34);
      sVar4 = FUN_1401ab420(lVar14 + 0x33,*(undefined4 *)(lVar14 + 0x37));
      uVar2 = *(undefined1 *)(lVar14 + 0x1a6);
      cVar3 = FUN_1402df070(uVar2,0);
      if (((cVar3 == '\0') && (cVar3 = (**(code **)(*plVar18 + 0x2c0))(plVar18), cVar3 != '\0')) &&
         (cVar3 = FUN_1402df080((int)sVar4,-iVar5), cVar3 != '\0')) {
        cVar3 = (**(code **)(*plVar18 + 0x2c0))(plVar18);
        if (cVar3 == '\0') {
          return 0;
        }
        cVar3 = (**(code **)(*plVar18 + 0x2d8))(plVar18,uVar2,0);
        if (cVar3 != '\0') {
          return 0;
        }
        iVar5 = FUN_1402df060(-iVar5);
        iVar15 = -iVar5;
        if (((0x1f < iVar15 - 3000U) && (0x1f < iVar15 - 0xc1cU)) && (0x1f < iVar15 - 0xc80U)) {
          return 0;
        }
        goto LAB_141787132;
      }
      iVar5 = FUN_14022fae0(lVar14,(int)param_1[6],0);
    }
    else {
      iVar5 = FUN_14022fae0(lVar14,(int)param_1[6],0);
    }
    if (iVar5 < 1) {
      return 0;
    }
LAB_141787132:
    lVar14 = FUN_1402e3cd0(lVar14,local_50,(int)param_1[6],iVar5);
    puVar10 = local_48;
    lVar14 = *(longlong *)(lVar14 + 8);
    if (local_48 != (undefined8 *)0x0) {
      if (0xffffe < local_48[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar18 = puVar10 + 1;
      lVar8 = *plVar18;
      *plVar18 = *plVar18 + -1;
      UNLOCK();
      if (((int)lVar8 == 1) && (local_48 != (undefined8 *)0x0)) {
        (**(code **)*local_48)(local_48,1);
      }
    }
    if (lVar14 != 0) {
      return 0;
    }
    FUN_1429edb20(PTR_u_DragEnd_143a47bd8);
    uVar9 = FUN_1417dd7e0((int)param_1[6],*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
    return uVar9;
  }
  plVar18 = (longlong *)param_1[8];
  plVar22 = plVar18 + -1;
  if (plVar18 == (longlong *)0x0) {
    plVar22 = plVar12;
  }
  if (plVar22 == (longlong *)0x0) goto LAB_141786ba3;
  local_c8 = DAT_143ac8240;
  local_res10 = (int *)FUN_142cbe730(DAT_143aa84a0);
  lVar8 = FUN_1402e3cd0(local_res10,local_a8,(int)param_1[6],
                        *(undefined4 *)((longlong)param_1 + 0x34));
  puVar10 = local_a0;
  plVar18 = *(longlong **)(lVar8 + 8);
  if (local_a0 != (undefined8 *)0x0) {
    if (0xffffe < local_a0[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar22 = puVar10 + 1;
    lVar8 = *plVar22;
    *plVar22 = *plVar22 + -1;
    UNLOCK();
    if (((int)lVar8 == 1) && (local_a0 != (undefined8 *)0x0)) {
      (**(code **)*local_a0)(local_a0,1);
    }
    local_a0 = (undefined8 *)0x0;
  }
  if (plVar18 == (longlong *)0x0) {
    return 0;
  }
  local_res20 = 0;
  iVar5 = FUN_14019a5d0(plVar18 + 4);
  cVar3 = FUN_141892d00();
  if ((cVar3 != '\0') && (cVar3 = FUN_140419bd0(iVar5), cVar3 != '\0')) {
    uVar9 = FUN_1408a9e40(&local_res10,0x12b2);
    FUN_1415eca30(uVar9,0xb);
    piVar17 = local_res10;
    goto LAB_141786cf9;
  }
  if (((piVar17 != (int *)0x0) && (lVar14 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
    iVar5 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a540);
    uVar21 = (uint)((ulonglong)in_stack_fffffffffffffee8 >> 0x20);
    plVar22 = plVar12;
    if (iVar5 != 0) {
      plVar22 = DAT_143aa8520;
    }
    if ((DAT_143aa8520 == (longlong *)0x0) ||
       (iVar5 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a87100),
       plVar19 = DAT_143aa8520, iVar5 == 0)) {
      plVar19 = plVar12;
    }
    if (plVar22 != (longlong *)0x0) {
      uVar9 = FUN_142889070(lVar14);
      cVar3 = FUN_14214e740(plVar18,uVar9);
      if (cVar3 == '\0') {
        return 1;
      }
      uVar7 = *(undefined4 *)((longlong)param_1 + 0x34);
      lVar14 = param_1[6];
      local_68 = plVar18;
      if (0xfffff < (ulonglong)plVar18[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar18[1] = plVar18[1] + 1;
      UNLOCK();
      FUN_14214b5d0(plVar22,local_70,(int)lVar14,uVar7,(ulonglong)uVar21 << 0x20,0,1);
      return 1;
    }
    if (plVar19 == (longlong *)0x0) {
      return 1;
    }
    uVar9 = FUN_142889070(lVar14);
    cVar3 = FUN_1410877d0(plVar18,uVar9);
    if (cVar3 == '\0') {
      return 1;
    }
    uVar7 = *(undefined4 *)((longlong)param_1 + 0x34);
    lVar14 = param_1[6];
    local_58 = plVar18;
    if (0xfffff < (ulonglong)plVar18[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    plVar18[1] = plVar18[1] + 1;
    UNLOCK();
    FUN_141085440(plVar19,local_60,(int)lVar14,uVar7,(ulonglong)uVar21 << 0x20,0,1);
    return 1;
  }
  switch((int)param_1[6]) {
  case 1:
  case 6:
    if (DAT_143aa8520 != (longlong *)0x0) {
      iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a968);
      if ((iVar15 != 0) && (DAT_143aa8520 != (longlong *)0x0)) {
        return 0;
      }
      if (DAT_143aa8520 != (longlong *)0x0) {
        iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a790);
        if ((iVar15 != 0) && (DAT_143aa8520 != (longlong *)0x0)) {
          return 0;
        }
        if (DAT_143aa8520 != (longlong *)0x0) {
          iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a9a0);
          if ((iVar15 != 0) && (DAT_143aa8520 != (longlong *)0x0)) {
            return 0;
          }
          if (DAT_143aa8520 != (longlong *)0x0) {
            iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a9a8);
            if ((iVar15 != 0) && (DAT_143aa8520 != (longlong *)0x0)) {
              return 0;
            }
            if (DAT_143aa8520 != (longlong *)0x0) {
              iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a970);
              if ((iVar15 != 0) && (DAT_143aa8520 != (longlong *)0x0)) {
                return 0;
              }
              if (((DAT_143aa8520 != (longlong *)0x0) &&
                  (iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))
                                      (DAT_143aa8520 + 1,&PTR_PTR_143a8a978), iVar15 != 0)) &&
                 (DAT_143aa8520 != (longlong *)0x0)) {
                return 0;
              }
            }
          }
        }
      }
    }
    if ((iVar5 - 1500000U < 10000) || (iVar5 - 0x170a70U < 10000)) {
      puVar10 = (undefined8 *)(**(code **)(*plVar18 + 0x80))(plVar18,&local_res10);
      FUN_142d4d9e0(piVar17,*puVar10,iVar5);
      return 0;
    }
    iVar15 = FUN_142d44b20(piVar17,plVar18,0);
    if (iVar15 != 0) {
      iVar6 = FUN_1417da2a0((int)param_1[6],*(undefined4 *)((longlong)param_1 + 0x34),iVar15);
      if (iVar6 == 0) {
        return 0;
      }
      FUN_1429edb20(PTR_u_DragEnd_143a47bd8);
      FUN_1417e4fc0(iVar15,plVar18);
      FUN_1417e9350(iVar5);
      return 1;
    }
    if (99999 < iVar5 - 1800000U) {
      return 0;
    }
    uVar9 = 0x4fc;
LAB_141785406:
    ppiVar16 = &local_res10;
    break;
  case 2:
    in_stack_fffffffffffffee8 = &local_res20;
    iVar15 = FUN_141787410(param_1,lVar14,plVar18,piVar17,in_stack_fffffffffffffee8);
    if (iVar15 == 0) {
      return 0;
    }
    local_res18 = (int *)FUN_1408f6690();
    iVar15 = (*DAT_143ad5648)(plVar18 + 8,&local_res18);
    uVar20 = (uint)(in_stack_fffffffffffffef8 >> 0x20);
    uVar23 = (uint)(in_stack_ffffffffffffff00 >> 0x20);
    if (iVar15 < 1) {
      local_res18 = (int *)0x0;
      puVar10 = (undefined8 *)FUN_1408a9e40(&local_c8,0x14d9);
      uVar9 = FUN_14019ba10(&local_res18,*puVar10);
      local_res10 = (int *)0x0;
      FUN_14019a260(&local_res10,uVar9);
      FUN_142a26280(&local_res10,0,0,1,(ulonglong)in_stack_fffffffffffffee8 & 0xffffffff00000000,0,
                    (ulonglong)uVar20 << 0x20,(ulonglong)uVar23 << 0x20,0,0);
      piVar17 = local_res18;
      if (local_c8 != 0) {
        FUN_14019f2c0(local_c8 + -0x10);
        piVar17 = local_res18;
      }
      goto LAB_141786cf9;
    }
    if (iVar5 - 2000000U < 1000000) {
      iVar15 = FUN_14038e0b0(DAT_143aa8328,iVar5);
      if (((iVar15 != 0) && (lVar14 != 0)) && (iVar15 = FUN_140f82630(lVar14 + 0x100), iVar15 != 0))
      {
        uVar9 = FUN_1408a9e40(&local_res10,0x12b3);
        FUN_1415eca30(uVar9,0xb);
        piVar17 = local_res10;
        goto LAB_141786cf9;
      }
      iVar15 = FUN_14038e130(DAT_143aa8328,iVar5);
      if (((iVar15 != 0) && (lVar14 != 0)) && (iVar15 = FUN_140f80f60(lVar14 + 0x100), iVar15 != 0))
      {
        uVar9 = FUN_1408a9e40(&local_res10,0x12b3);
        FUN_1415eca30(uVar9,0xb);
        piVar17 = local_res10;
        goto LAB_141786cf9;
      }
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if (iVar15 != 0) {
      uVar9 = FUN_140431e10();
      lVar8 = FUN_140431e20(uVar9,iVar5);
      if (lVar8 != 0) {
        if (DAT_143aa8520 != (longlong *)0x0) {
          return 0;
        }
        FUN_142cd7830(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,0);
        return 0;
      }
    }
    if (iVar5 - 0x26bdf9U < 2) {
      FUN_140caf430(iVar5);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (iVar15 = FUN_140416fc0(iVar5), iVar15 != 0)) {
      if (DAT_143aa8520 == (longlong *)0x0) {
        local_res10 = (int *)FUN_14019b780(&DAT_143ad68a0,0x1408);
LAB_141785717:
        if (local_res10 == (int *)0x0) {
          return 0;
        }
        FUN_1423b05c0(local_res10,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      uVar9 = FUN_1408a9e40(&local_res10,0x86);
      FUN_1415eca30(uVar9,0xb);
      piVar17 = local_res10;
      goto LAB_141786cf9;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (iVar15 = FUN_140416ff0(iVar5), iVar15 != 0)) {
      if (DAT_143aa8520 == (longlong *)0x0) {
        local_res10 = (int *)FUN_14019b780(&DAT_143ad68a0,0x1408);
        goto LAB_141785717;
      }
      uVar9 = FUN_1408a9e40(&local_res10,0x86);
      FUN_1415eca30(uVar9,0xb);
      piVar17 = local_res10;
      goto LAB_141786cf9;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (cVar3 = FUN_140417c00(iVar5), cVar3 != '\0')) {
      FUN_142cc88b0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (iVar15 = FUN_1404169b0(iVar5), iVar15 != 0)) {
      FUN_142cc8ab0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,500,0);
    if ((iVar15 == 0) || (9999 < iVar5 - 0x216ab0U)) {
      iVar15 = FUN_142cc42d0(piVar17,500,0);
      if ((iVar15 != 0) && (iVar5 - 0x1ef9b0U < 10000)) {
        FUN_142d4be40(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,500,0);
      if ((iVar15 != 0) && (iVar5 - 2100000U < 10000)) {
        FUN_142ccb0f0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 != 0) && (iVar5 - 0x205940U < 10000)) {
        FUN_142ccb350(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 != 0) && (iVar5 - 0x227c20U < 10000)) {
        FUN_142ccb950(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,500,0);
      if ((iVar15 != 0) && (iVar5 - 0x22a330U < 10000)) {
        FUN_142cebd90(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 != 0) &&
         (((iVar5 - 0x22ca40U < 10000 || (iVar5 - 0x22f150U < 10000)) || (iVar5 - 0x55c120U < 10000)
          ))) {
        FUN_142cd4ff0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 != 0) && (iVar15 = FUN_14178e0a0(iVar5), iVar15 != 0)) {
LAB_14178682b:
        FUN_142cd51a0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 != 0) && (iVar15 = FUN_14178e040(iVar5), iVar15 != 0)) {
LAB_141786849:
        FUN_142cd5680(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 != 0) && (iVar15 = FUN_14178e010(iVar5), iVar15 != 0)) {
        uVar7 = *(undefined4 *)((longlong)param_1 + 0x34);
LAB_141785b38:
        FUN_142cd5b30(piVar17,uVar7,iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 != 0) && (iVar5 - 0x2206f0U < 10000)) {
        FUN_142cd6100(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 == 0) || (9999 < iVar5 - 0x233f70U)) {
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar5 - 0x236680U < 10000)) {
          FUN_142cd6b30(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && ((iVar5 - 0x532910U < 10000 || (iVar5 - 0x2477f0U < 10000)))) {
          FUN_142cd70e0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) &&
           ((((iVar5 == 0x252f2b || (iVar5 == 0x282825)) || (iVar5 == 0x282ca2)) ||
            (iVar5 == 0x251fb2)))) {
          FUN_142dc8d60(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,0,0);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar15 = FUN_140416d60(iVar5), iVar15 != 0)) {
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar15 = FUN_140416ea0(iVar5), iVar15 != 0)) goto LAB_141785cbe;
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar5 - 0x264cb0U < 10000)) {
          FUN_142ccc260(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar5 - 0x2673c0U < 10000)) {
          FUN_142971430(lVar14,&local_88,iVar5,*(undefined4 *)((longlong)param_1 + 0x34));
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if (((iVar15 != 0) && (DAT_143aa8328 != 0)) &&
           (cVar3 = FUN_1403c4260(DAT_143aa8328,iVar5), cVar3 != '\0')) {
          FUN_142cd8270(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) &&
           (((((iVar5 - 0x2c24c8U < 1000 || (iVar5 - 0x2c3080U < 1000)) ||
              ((iVar5 - 0x2c1910U < 1000 ||
               ((iVar5 - 0x2c1cf8U < 1000 || (iVar5 - 0x2c28b0U < 1000)))))) ||
             (iVar5 - 0x2c2c98U < 1000)) || (iVar5 - 0x26c1e0U < 10000)))) {
          FUN_142dc8100(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,0,0);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        lVar14 = local_c8;
        if ((iVar15 != 0) && (iVar5 - 0x258960U < 10000)) {
          if (local_c8 == 0) {
            return 0;
          }
          FUN_142386950(local_c8,0);
          FUN_14238b470(lVar14,1,*(undefined4 *)((longlong)param_1 + 0x34));
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar5 - 0x25b070U < 10000)) {
          if (DAT_143ac8240 == 0) {
            return 0;
          }
          FUN_142386950(DAT_143ac8240,0);
          FUN_142387ff0(DAT_143ac8240,1,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,
                        (ulonglong)in_stack_fffffffffffffee8 & 0xffffffffffffff00);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar5 - 0x278530U < 1000)) {
          if (DAT_143ac8240 == 0) {
            return 0;
          }
          FUN_142386950(DAT_143ac8240,0);
          FUN_142388080(DAT_143ac8240,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar5 - 0x278918U < 1000)) {
          if (DAT_143ac8240 == 0) {
            return 0;
          }
          FUN_142386950(DAT_143ac8240,0);
          FUN_1423880c0(DAT_143ac8240,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,500,0);
        if ((iVar15 != 0) && (iVar15 = FUN_14041b2d0(iVar5), iVar15 != 0)) {
          if (DAT_143ac8240 == 0) {
            return 0;
          }
          FUN_142386950(DAT_143ac8240,0);
          FUN_142388020(DAT_143ac8240,1,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,
                        (ulonglong)in_stack_fffffffffffffee8 & 0xffffffffffffff00);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,500,0);
        if ((iVar15 != 0) && (iVar5 - 0x295df0U < 2)) {
          FUN_142386950(DAT_143ac8240,0);
          uVar7 = (undefined4)param_1[6];
LAB_1417868ed:
          FUN_142387340(DAT_143ac8240,1,*(undefined4 *)((longlong)param_1 + 0x34),uVar7,
                        (ulonglong)in_stack_fffffffffffffee8 & 0xffffffffffffff00);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,500,0);
        if ((iVar15 != 0) && (iVar5 - 0x29a81bU < 2)) {
          FUN_142386950(DAT_143ac8240,0);
          uVar7 = (undefined4)param_1[6];
LAB_14178692b:
          FUN_142387310(DAT_143ac8240,1,*(undefined4 *)((longlong)param_1 + 0x34),uVar7,
                        (ulonglong)in_stack_fffffffffffffee8 & 0xffffffffffffff00);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,500,0);
        if ((iVar15 != 0) && ((iVar5 - 0x1f40c8U < 100 || (iVar5 - 0x1f412cU < 5)))) {
          if (DAT_143aa8520 != (longlong *)0x0) {
            uVar9 = FUN_1408a9e40(&local_res10,0x86);
            FUN_1415eca30(uVar9,0xb);
            if (local_res10 == (int *)0x0) {
              return 0;
            }
            FUN_14019f2c0(local_res10 + -4);
            return 0;
          }
          if (DAT_143ac8240 == 0) {
            return 0;
          }
          FUN_142386950(DAT_143ac8240,0);
          FUN_142388050(DAT_143ac8240,1,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,
                        (ulonglong)in_stack_fffffffffffffee8 & 0xffffffffffffff00);
          return 0;
        }
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 != 0) && (iVar15 = FUN_14043cf00(iVar5), iVar15 != 0)) goto LAB_141786533;
        iVar15 = FUN_142cc42d0(piVar17,200,0);
        if ((iVar15 == 0) || (999 < iVar5 - 0x26ba10U)) {
          iVar15 = FUN_142cc42d0(piVar17,500,0);
          if ((iVar15 != 0) && (cVar3 = FUN_140418e20(iVar5), cVar3 != '\0')) {
            if (DAT_143ac8240 == 0) {
              return 0;
            }
            FUN_142386950(DAT_143ac8240,0);
            FUN_142388100(DAT_143ac8240,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
            return 0;
          }
          iVar15 = FUN_142cc42d0(piVar17,200,0);
          if ((iVar15 != 0) && (iVar5 - 0x2429d0U < 10000)) {
            FUN_142cd6920(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
            return 0;
          }
          iVar15 = FUN_142cc42d0(piVar17,200,0);
          if (iVar15 == 0) {
            return 0;
          }
          if (5 < iVar5 - 0x1e8c55U) {
            return 0;
          }
          FUN_142d4ffb0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
          return 0;
        }
        iVar5 = FUN_142d1d0f0();
        if (iVar5 == 0) {
          iVar5 = FUN_14090d160(0xd9,0);
          if (iVar5 == 0) {
            if (DAT_143ac8240 != 0) {
              FUN_142386950(DAT_143ac8240,0);
            }
            if (DAT_143aa8520 == (longlong *)0x0) {
              local_res10 = (int *)FUN_14019b780(&DAT_143ad68a0,0x1478);
              if (local_res10 != (int *)0x0) {
                plVar12 = (longlong *)FUN_141414f50(local_res10);
              }
              FUN_141416550(plVar12,*(undefined4 *)((longlong)param_1 + 0x34));
              return 0;
            }
            uVar9 = FUN_1408a9e40(&local_res10,0x86);
            FUN_1415eca30(uVar9,0xb);
            piVar17 = local_res10;
          }
          else {
            uVar9 = FUN_1408a9e40(&local_res10,0x12ef);
            FUN_1415eca30(uVar9,0xb);
            piVar17 = local_res10;
          }
        }
        else {
          uVar9 = FUN_1408a9e40(&local_res10,0x12ee);
          FUN_1415eca30(uVar9,0xb);
          piVar17 = local_res10;
        }
        goto LAB_141786cf9;
      }
      uVar9 = 0xb91;
      goto LAB_141785406;
    }
    local_res18 = (int *)0x0;
    iVar15 = (*DAT_1432627f0)(0xfde9,0,&DAT_143278568,0xffffffff,0,0,0,0);
    uVar20 = iVar15 - 1;
    plVar22 = (longlong *)(ulonglong)uVar20;
    plVar18 = plVar12;
    if ((local_res18 == (int *)0x0) ||
       (plVar18 = (longlong *)(local_res18 + -4), plVar18 == (longlong *)0x0)) {
LAB_14178585b:
      if ((int)uVar21 < (int)uVar20) {
        uVar21 = uVar20;
      }
      local_res10 = (int *)CONCAT44(local_res10._4_4_,uVar21);
      puVar11 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(int)(uVar21 + 0x11));
      puVar11[1] = (int)local_res10;
      *puVar11 = 0xffffffff;
      local_res18 = puVar11 + 4;
      puVar11[2] = 0;
      *(undefined1 *)local_res18 = 0;
      if (plVar18 != (longlong *)0x0) {
        FUN_14019f2c0(plVar18);
      }
    }
    else {
      if ((1 < (int)*plVar18) || (local_res18[-3] < (int)uVar20)) {
        uVar21 = local_res18[-2];
        goto LAB_14178585b;
      }
      if ((int)*plVar18 != 1) {
        FUN_142e52dd0(0x74);
      }
      *(int *)plVar18 = -1;
    }
    (*DAT_1432627f0)(0xfde9,0,&DAT_143278568,0xffffffff,local_res18,iVar15,0,0);
    piVar13 = local_res18;
    if (local_res18[-4] != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((uVar20 != 0xffffffff) && (piVar13[-3] < (int)uVar20)) {
      FUN_142e54290(0x90,piVar13[-3],plVar22);
    }
    piVar13[-4] = 1;
    if (uVar20 == 0xffffffff) {
      plVar22 = (longlong *)0xffffffffffffffff;
      if (piVar13 != (int *)0x0) {
        do {
          plVar22 = (longlong *)((longlong)plVar22 + 1);
        } while (*(char *)((longlong)plVar22 + (longlong)piVar13) != '\0');
        goto LAB_14178593f;
      }
LAB_141785944:
      iVar15 = (int)plVar12;
      if (iVar15 < piVar13[-3] + 1) goto LAB_141785960;
    }
    else {
      *(undefined1 *)((longlong)(int)uVar20 + (longlong)local_res18) = 0;
LAB_14178593f:
      plVar12 = plVar22;
      if (-1 < (int)plVar22) goto LAB_141785944;
    }
    iVar15 = (int)plVar12;
    FUN_142e54290(0x9c,(ulonglong)plVar12 & 0xffffffff,piVar13[-3]);
LAB_141785960:
    piVar13[-2] = iVar15;
    FUN_142d4e120(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,&local_res18);
    return 0;
  case 3:
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    uVar7 = (undefined4)((ulonglong)in_stack_fffffffffffffee8 >> 0x20);
    if ((iVar15 != 0) && (iVar15 = FUN_14178e070(iVar5), iVar15 != 0)) {
LAB_141786b74:
      FUN_142cd2e20(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,0,
                    CONCAT44(uVar7,0xffffffff));
      return 0;
    }
    if (iVar5 - 0x2e8628U < 1000) {
      FUN_142cd7a20(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
      return 0;
    }
    if (iVar5 - 3700000U < 10000) {
      FUN_142d4dd10(piVar17,iVar5,*(undefined4 *)((longlong)param_1 + 0x34));
      return 0;
    }
    if (iVar5 - 0x2fc290U < 10000) {
      FUN_142dda640(piVar17,iVar5);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 == 0) || (iVar15 = FUN_14043cf00(iVar5), iVar15 == 0)) {
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if ((iVar15 == 0) || (iVar15 = FUN_140387120(DAT_143aa8328,local_res10,iVar5), iVar15 == 0)) {
        if (9999 < iVar5 - 0x2f7470U) {
          return 0;
        }
        FUN_142d4fa60(piVar17,iVar5,*(undefined4 *)((longlong)param_1 + 0x34));
        return 0;
      }
      goto LAB_141786579;
    }
    goto LAB_141786533;
  case 4:
    iVar15 = FUN_14276e0e0(lVar14);
    if (iVar15 != 0) {
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,500,0);
    if ((iVar15 != 0) && (iVar5 - 0x3e4180U < 10000)) {
      FUN_142d4abc0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
    }
    if ((iVar5 - 0x3f7a00U < 10000) && (999 < iVar5 - 0x3f81d0U)) {
      FUN_142ce33b0(piVar17,iVar5);
      return 0;
    }
    if (iVar5 - 0x440de5U < 2) {
      FUN_142ce3470(piVar17,iVar5);
      return 0;
    }
    if (iVar5 - 0x426030U < 10000) {
      FUN_142ce3590(piVar17,iVar5);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (iVar5 - 0x404138U < 1000)) {
      FUN_142cd7260(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (iVar5 - 0x404520U < 1000)) {
      FUN_142cd7620(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
      return 0;
    }
    if ((iVar5 - 0x406460U < 1000) && (iVar5 != 0x406514)) {
      FUN_142ce3750(piVar17,iVar5);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (iVar5 - 0x3fa110U < 10000)) {
      FUN_142dadff0(piVar17,iVar5,*(undefined4 *)((longlong)param_1 + 0x34));
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (iVar5 - 0x41eb00U < 10000)) {
      FUN_142cd3e20(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 == 0) || (iVar15 = FUN_14043cf00(iVar5), iVar15 == 0)) {
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if (iVar15 == 0) {
        return 0;
      }
      iVar15 = FUN_140387120(DAT_143aa8328,local_res10,iVar5);
      if (iVar15 == 0) {
        return 0;
      }
LAB_141786579:
      FUN_14178c1d0(param_1,iVar5);
      return 0;
    }
LAB_141786533:
    FUN_142cd3f50(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
    return 0;
  case 5:
    if (DAT_143aa8520 != (longlong *)0x0) {
      iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a878a0);
      if ((iVar15 != 0) && (DAT_143aa8520 != (longlong *)0x0)) {
        return 0;
      }
      if (DAT_143aa8520 != (longlong *)0x0) {
        iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a970);
        if ((iVar15 != 0) && (DAT_143aa8520 != (longlong *)0x0)) {
          return 0;
        }
        if (((DAT_143aa8520 != (longlong *)0x0) &&
            (iVar15 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a8a978),
            iVar15 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
          return 0;
        }
      }
    }
    iVar15 = FUN_142cc42d0(piVar17,500,0);
    if (iVar15 == 0) {
      return 0;
    }
    uVar7 = FUN_140418910(iVar5);
    local_res10 = (int *)CONCAT44(local_res10._4_4_,uVar7);
    iVar15 = FUN_14038ac80(DAT_143aa8328,iVar5);
    if (iVar15 == 0) {
      iVar15 = (**(code **)(*plVar18 + 0x88))(plVar18);
      if (iVar15 == 3) {
        local_res18 = (int *)FUN_1408f6690();
        uVar9 = FUN_140192f80(plVar18);
        iVar15 = FUN_1402cf680(uVar9,0);
        if (iVar15 == 0) {
          uVar9 = FUN_140192f80(plVar18);
          iVar15 = FUN_1402cf680(uVar9,&local_res18);
          if (iVar15 != 0) {
            return 0;
          }
        }
      }
      if (((int)local_res10 == 8) && (iVar15 = (**(code **)(*plVar18 + 0x38))(plVar18), iVar15 != 0)
         ) {
        uVar9 = FUN_1408a9e40(&local_c8,0xcba);
        in_stack_ffffffffffffff00 = in_stack_ffffffffffffff00 & 0xffffffff00000000;
        in_stack_fffffffffffffef8 = in_stack_fffffffffffffef8 & 0xffffffff00000000;
        in_stack_fffffffffffffef0 = 0xffffffff;
        in_stack_fffffffffffffee8 =
             (undefined8 *)((ulonglong)in_stack_fffffffffffffee8 & 0xffffffff00000000);
        iVar15 = FUN_142a269c0(uVar9,0,0,1,in_stack_fffffffffffffee8,0xffffffff,
                               in_stack_fffffffffffffef8,in_stack_ffffffffffffff00,3,0,0);
        if (iVar15 != 6) {
          return 0;
        }
      }
      iVar15 = (**(code **)(*plVar18 + 0x38))(plVar18);
      if (((iVar15 != 0) && (iVar15 = (**(code **)(*plVar18 + 0x98))(plVar18), 1 < iVar15)) &&
         (iVar15 = (**(code **)(*plVar18 + 0x28))(plVar18), iVar15 == 0)) {
        uVar9 = FUN_1408a9e40(local_b8,0x134b);
        in_stack_ffffffffffffff00 = in_stack_ffffffffffffff00 & 0xffffffff00000000;
        in_stack_fffffffffffffef8 = in_stack_fffffffffffffef8 & 0xffffffff00000000;
        in_stack_fffffffffffffef0 = 0xffffffff;
        in_stack_fffffffffffffee8 =
             (undefined8 *)((ulonglong)in_stack_fffffffffffffee8 & 0xffffffff00000000);
        iVar15 = FUN_142a269c0(uVar9,0,0,1,in_stack_fffffffffffffee8,0xffffffff,
                               in_stack_fffffffffffffef8,in_stack_ffffffffffffff00,3,0,0);
        if (iVar15 == 7) {
          return 0;
        }
      }
    }
    iVar15 = FUN_140418910(iVar5);
    if (iVar15 != 0) {
      FUN_142d4b500(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,plVar18[7]);
      return 0;
    }
    if (iVar5 == 0x4f0a60) {
      iVar5 = FUN_142387930(DAT_143ac8240);
      if (iVar5 != 0) {
        return 0;
      }
      FUN_142387810(DAT_143ac8240,1,*(undefined4 *)((longlong)param_1 + 0x34),0);
      return 0;
    }
    iVar15 = FUN_142cc42d0(piVar17,200,0);
    if ((iVar15 != 0) && (iVar5 - 0x56ab80U < 10000)) {
LAB_141785cbe:
      FUN_142ccba90(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
      return 0;
    }
    iVar15 = FUN_1404187a0(iVar5);
    if (iVar15 == 0) {
      iVar15 = FUN_140418870(iVar5);
      uVar7 = (undefined4)((ulonglong)in_stack_fffffffffffffee8 >> 0x20);
      if (iVar15 == 0) {
        return 0;
      }
      iVar15 = FUN_14178e070(iVar5);
      if (iVar15 == 0) {
        FUN_142d4b460(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5);
        return 0;
      }
      iVar15 = FUN_142cc42d0(piVar17,200,0);
      if (iVar15 == 0) {
        return 0;
      }
      goto LAB_141786b74;
    }
    if (iVar5 - 0x53f048U < 1000) {
      FUN_142d19cf0(piVar17);
      return 0;
    }
    iVar15 = FUN_14178e0a0(iVar5);
    if (iVar15 != 0) goto LAB_14178682b;
    iVar15 = FUN_14178e040(iVar5);
    if (iVar15 != 0) goto LAB_141786849;
    iVar15 = FUN_14178e010(iVar5);
    if (iVar15 != 0) {
      uVar7 = *(undefined4 *)((longlong)param_1 + 0x34);
      goto LAB_141785b38;
    }
    iVar15 = FUN_142cc42d0(piVar17,500,0);
    if (iVar15 == 0) {
      return 0;
    }
    if (DAT_143ac8240 == 0) {
LAB_141786a8e:
      local_res10 = (int *)0x0;
      piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
      piVar13[1] = 0;
      *piVar13 = -1;
      local_res10 = piVar13 + 4;
      piVar13[2] = 0;
      *(undefined1 *)local_res10 = 0;
      if (*piVar13 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar13[1] < 0) {
        FUN_142e54290(0x90,piVar13[1],0);
      }
      *piVar13 = 1;
      *(undefined1 *)local_res10 = 0;
      if (piVar13[1] + 1 < 1) {
        FUN_142e54290(0x9c,0);
      }
      piVar13[2] = 0;
      FUN_142ccc3a0(piVar17,*(undefined4 *)((longlong)param_1 + 0x34),iVar5,0,&local_res10,
                    in_stack_fffffffffffffef0 & 0xffffff00,0,
                    in_stack_ffffffffffffff00 & 0xffffffff00000000);
      return 0;
    }
    iVar15 = FUN_140417ed0(iVar5);
    if (iVar15 == 0x50) {
      FUN_142386950(DAT_143ac8240,0);
      FUN_142387370(DAT_143ac8240,1,*(undefined4 *)((longlong)param_1 + 0x34));
      return 0;
    }
    if (iVar5 - 0x295df0U < 2) {
      FUN_142386950(DAT_143ac8240,0);
      uVar7 = 2;
      goto LAB_1417868ed;
    }
    if (iVar5 - 0x29a81bU < 2) {
      FUN_142386950(DAT_143ac8240,0);
      uVar7 = 2;
      goto LAB_14178692b;
    }
    iVar15 = FUN_140417ed0(iVar5);
    if (iVar15 == 0x4d) {
      FUN_142386950(DAT_143ac8240,0);
      FUN_142387420(DAT_143ac8240,1,*(undefined4 *)((longlong)param_1 + 0x34),0,
                    (ulonglong)in_stack_fffffffffffffee8 & 0xffffffffffffff00);
      return 0;
    }
    iVar15 = FUN_1404187a0(iVar5);
    if (iVar15 == 0x5a) {
      if ((lVar14 == 0) || (iVar15 = FUN_14276e0e0(lVar14), iVar15 == 0)) {
        FUN_142386950(DAT_143ac8240,5);
        FUN_142387450(DAT_143ac8240,iVar5,1,*(undefined4 *)((longlong)param_1 + 0x34),
                      (ulonglong)in_stack_fffffffffffffee8 & 0xffffffff00000000,
                      in_stack_fffffffffffffef0 & 0xffffff00);
        return 0;
      }
LAB_1417869a2:
      uVar9 = FUN_1408a9e40(&local_d8,0x14cd);
      FUN_140d84b70(uVar9,0);
      return 0;
    }
    iVar15 = FUN_1404187a0(iVar5);
    if (iVar15 == 0x61) {
      FUN_142267d00(iVar5,*(undefined4 *)((longlong)param_1 + 0x34));
      return 0;
    }
    iVar15 = FUN_140419390(iVar5);
    if (iVar15 == 0) goto LAB_141786a8e;
    if ((lVar14 != 0) && (iVar15 = FUN_14276e0e0(lVar14), iVar15 != 0)) goto LAB_1417869a2;
    if (DAT_143aca978 == 0) {
      iVar15 = FUN_140417ed0(iVar5);
      FUN_142386950(DAT_143ac8240,(iVar15 == 0x5c) + '\x04');
      FUN_1423874a0(DAT_143ac8240,iVar5,1,*(undefined4 *)((longlong)param_1 + 0x34),
                    (ulonglong)in_stack_fffffffffffffee8 & 0xffffffff00000000,
                    in_stack_fffffffffffffef0 & 0xffffff00);
      return 0;
    }
    uVar9 = 0x152b;
    ppiVar16 = &local_d8;
    break;
  default:
    goto switchD_14178529c_default;
  }
  uVar9 = FUN_1408a9e40(ppiVar16,uVar9);
  FUN_142a26280(uVar9,0,0,1,(ulonglong)in_stack_fffffffffffffee8 & 0xffffffff00000000,0,
                in_stack_fffffffffffffef8 & 0xffffffff00000000,
                in_stack_ffffffffffffff00 & 0xffffffff00000000,0,0);
switchD_14178529c_default:
  return 0;
}



//===========================================================
// FUN_1416fb130 @ 1416fb130   (1736 bytes)
//===========================================================

undefined8 FUN_1416fb130(longlong *param_1,longlong param_2,longlong param_3)

{
  longlong *plVar1;
  undefined8 *puVar2;
  int *piVar3;
  uint uVar4;
  undefined8 *puVar5;
  char cVar6;
  undefined1 uVar7;
  int iVar8;
  int iVar9;
  undefined4 uVar10;
  int iVar11;
  undefined8 uVar12;
  longlong lVar13;
  longlong lVar14;
  undefined1 *puVar15;
  undefined8 *puVar16;
  longlong *plVar17;
  bool bVar18;
  undefined4 local_res8 [2];
  undefined1 local_c0;
  undefined1 local_bf [3];
  int local_bc;
  int local_b8;
  int local_b0 [2];
  uint local_a8;
  int local_a4;
  longlong *local_98;
  undefined1 local_90 [8];
  longlong local_88;
  undefined1 local_80 [8];
  undefined1 local_78 [8];
  undefined1 local_70 [8];
  undefined1 local_68 [8];
  undefined1 local_60 [8];
  undefined8 *local_58;
  undefined1 local_50 [8];
  undefined8 *local_48;
  undefined1 local_40 [24];
  
  local_res8[0] = 0;
  if ((int)param_1[0x1a] != 0) {
    uVar12 = FUN_1408a9e40(local_b0,0x11cb);
    FUN_140d84b70(uVar12,0);
    return 0;
  }
  iVar8 = (**(code **)(*param_1 + 0x50))();
  if (iVar8 == 0) {
    return 0;
  }
  if (param_2 == 0) {
    return 0;
  }
  if (param_3 == 0) {
    return 0;
  }
  if (DAT_143aa84a0 == 0) {
    return 0;
  }
  lVar13 = FUN_142cbe730();
  iVar8 = FUN_141787900(param_2);
  iVar9 = FUN_141787910(param_2);
  if (iVar9 < 1) {
    return 0;
  }
  if (iVar8 - 3U < 2) {
    return 0;
  }
  local_b0[0] = iVar9;
  lVar14 = FUN_14022f820(local_60,lVar13,iVar9,iVar8);
  puVar5 = local_58;
  plVar17 = *(longlong **)(lVar14 + 8);
  if (local_58 != (undefined8 *)0x0) {
    if (0xffffe < local_58[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = puVar5 + 1;
    lVar14 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar14 == 1) && (local_58 != (undefined8 *)0x0)) {
      (**(code **)*local_58)(local_58,1);
    }
    local_58 = (undefined8 *)0x0;
  }
  FUN_1402e3cd0(lVar13,local_50,*(undefined4 *)((longlong)param_1 + 0xe4),(int)param_1[0x1d]);
  puVar5 = local_48;
  if (local_48 != (undefined8 *)0x0) {
    if (0xffffe < local_48[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = puVar5 + 1;
    lVar14 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar14 == 1) && (local_48 != (undefined8 *)0x0)) {
      (**(code **)*local_48)(local_48,1);
    }
  }
  if (plVar17 == (longlong *)0x0) {
    return 0;
  }
  FUN_1429edb20(PTR_u_DragEnd_143a47bd8);
  uVar12 = DAT_143aa8328;
  if ((iVar8 != 1) && (iVar8 != 6)) {
    if ((iVar8 != 2) && (iVar8 != 5)) {
      return 0;
    }
    uVar12 = FUN_1417deab0(iVar8,iVar9,*(undefined4 *)((longlong)param_1 + 0xe4),(int)param_1[0x1d],
                           0,0xf,1);
    return uVar12;
  }
  iVar9 = (int)param_1[0x1d];
  plVar1 = plVar17 + 4;
  uVar10 = FUN_1401b0340(plVar1);
  iVar11 = FUN_140389c10(uVar12,uVar10);
  if ((iVar11 != 0) || (local_bc = 0, plVar17[7] != 0)) {
    local_bc = 1;
  }
  local_a8 = -iVar9;
  uVar4 = local_a8 - 0x4b0;
  local_a4 = FUN_1402536d0(local_a8);
  if ((local_bc == 0) && (iVar9 + 0x83U < 0x1f)) {
    return 0;
  }
  iVar9 = FUN_140253450(iVar9);
  local_b8 = iVar9;
  if (0xd < uVar4) {
    uVar7 = *(undefined1 *)(lVar13 + 0x19);
    uVar10 = FUN_14019a5d0(plVar1);
    iVar11 = FUN_140253980(uVar10,iVar9,uVar7,0);
    if (iVar11 == 0) {
      return 0;
    }
LAB_1416fb7ad:
    iVar8 = FUN_1417da2a0(iVar8,local_b0[0],iVar9);
    if (iVar8 == 0) {
      return 0;
    }
    uVar10 = FUN_14019a5d0(plVar1);
    FUN_1417e9350(uVar10);
    if ((local_bc != 0) && ((local_a8 < 0x20 || (local_a4 != 0)))) {
      FUN_141157a70(1);
    }
    return 1;
  }
  if (0xd < iVar9 - 0x4b0U) {
    return 0;
  }
  uVar10 = FUN_14019a5d0(plVar1);
  iVar11 = FUN_140253980(uVar10,iVar9,2,0);
  if (iVar11 == 0) {
    return 0;
  }
  if ((plVar17[7] == 0) ||
     (iVar11 = (**(code **)(*plVar17 + 0x38))(plVar17), uVar12 = DAT_143aa8328, iVar11 == 0))
  goto LAB_1416fb7ad;
  uVar10 = FUN_14019a5d0(plVar17 + 4);
  iVar11 = FUN_14038ac80(uVar12,uVar10);
  if (iVar11 != 0) goto LAB_1416fb7ad;
  bVar18 = DAT_143aa8518 == 0;
  if (bVar18) {
    local_88 = 0;
    puVar15 = local_90;
  }
  else {
    puVar15 = (undefined1 *)FUN_1427be040(DAT_143aa8518,local_40);
  }
  local_98 = *(longlong **)(puVar15 + 8);
  *(undefined8 *)(puVar15 + 8) = 0;
  if ((bVar18) && (local_88 != 0)) {
    puVar5 = *(undefined8 **)(local_88 + 0x38);
    if (puVar5 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)puVar5[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      puVar5[1] = puVar5[1] + 1;
      UNLOCK();
    }
    lVar13 = local_88;
    puVar2 = puVar5 + 4;
    FUN_1401d3510(puVar2,local_res8);
    if (0xffffe < *(longlong *)(lVar13 + 0x18) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar17 = (longlong *)(lVar13 + 0x18);
    lVar13 = *plVar17;
    *plVar17 = *plVar17 + -1;
    UNLOCK();
    if ((int)lVar13 == 1) {
      puVar5[6] = 0;
      puVar16 = (undefined8 *)(local_88 + 0x10);
      if (puVar16 != (undefined8 *)0x0) {
        (**(code **)*puVar16)(puVar16,1);
      }
    }
    if (puVar2 != (undefined8 *)0x0) {
      piVar3 = (int *)(puVar5 + 5);
      *piVar3 = *piVar3 + -1;
      if (*piVar3 == 0) {
        *puVar2 = 0;
      }
    }
    if (puVar5 != (undefined8 *)0x0) {
      if (0xffffe < puVar5[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar17 = puVar5 + 1;
      lVar13 = *plVar17;
      *plVar17 = *plVar17 + -1;
      UNLOCK();
      if ((int)lVar13 == 1) {
        (**(code **)*puVar5)(puVar5,1);
      }
    }
  }
  plVar17 = local_98;
  if (!bVar18) {
    FUN_140ce88a0(local_40);
  }
  if (plVar17 == (longlong *)0x0) {
    uVar12 = 0x115d;
    puVar15 = local_80;
LAB_1416fb5da:
    uVar12 = FUN_1408a9e40(puVar15,uVar12);
  }
  else {
    cVar6 = FUN_140dac980(plVar17);
    if (cVar6 == '\0') {
      uVar7 = (**(code **)(*plVar17 + 0x20))(plVar17);
      uVar10 = FUN_14019a5d0(plVar1);
      iVar9 = FUN_140253330(uVar10,uVar7);
      if (iVar9 != 0) {
        puVar5 = (undefined8 *)plVar17[7];
        if (puVar5 != (undefined8 *)0x0) {
          if (0xfffff < (ulonglong)puVar5[1]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          puVar5[1] = puVar5[1] + 1;
          UNLOCK();
          plVar17 = local_98;
        }
        puVar2 = puVar5 + 4;
        FUN_1401d3510(puVar2,local_bf);
        if (0xffffe < plVar17[3] - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar17 = plVar17 + 3;
        lVar13 = *plVar17;
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if ((int)lVar13 == 1) {
          puVar5[6] = 0;
          plVar17 = local_98 + 2;
          if (plVar17 != (longlong *)0x0) {
            (**(code **)*plVar17)(plVar17,1);
          }
        }
        if (puVar2 != (undefined8 *)0x0) {
          piVar3 = (int *)(puVar5 + 5);
          *piVar3 = *piVar3 + -1;
          if (*piVar3 == 0) {
            *puVar2 = 0;
          }
        }
        iVar9 = local_b8;
        if (puVar5 != (undefined8 *)0x0) {
          if (0xffffe < puVar5[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar17 = puVar5 + 1;
          lVar13 = *plVar17;
          *plVar17 = *plVar17 + -1;
          UNLOCK();
          iVar9 = local_b8;
          if ((int)lVar13 == 1) {
            (**(code **)*puVar5)(puVar5,1);
            iVar9 = local_b8;
          }
        }
        goto LAB_1416fb7ad;
      }
      uVar12 = 0x115b;
      puVar15 = local_68;
      goto LAB_1416fb5da;
    }
    cVar6 = FUN_140db5600();
    if (cVar6 != '\0') {
      uVar12 = 0x115c;
      puVar15 = local_78;
      goto LAB_1416fb5da;
    }
    cVar6 = FUN_140db5620(plVar17);
    if (cVar6 == '\0') goto LAB_1416fb5ea;
    uVar12 = FUN_1408a9f20("SID_SYNCROID_EQUIP_BLOCK");
    uVar12 = FUN_140196ed0(local_70,uVar12,0xffffffff);
  }
  FUN_140d84b70(uVar12,0);
LAB_1416fb5ea:
  if (plVar17 == (longlong *)0x0) {
    return 0;
  }
  puVar5 = (undefined8 *)plVar17[7];
  if (puVar5 != (undefined8 *)0x0) {
    if (0xfffff < (ulonglong)puVar5[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    puVar5[1] = puVar5[1] + 1;
    UNLOCK();
    plVar17 = local_98;
  }
  puVar2 = puVar5 + 4;
  FUN_1401d3510(puVar2,&local_c0);
  if (0xffffe < plVar17[3] - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar17 = plVar17 + 3;
  lVar13 = *plVar17;
  *plVar17 = *plVar17 + -1;
  UNLOCK();
  if ((int)lVar13 == 1) {
    puVar5[6] = 0;
    plVar17 = local_98 + 2;
    if (plVar17 != (longlong *)0x0) {
      (**(code **)*plVar17)(plVar17,1);
    }
  }
  if (puVar2 != (undefined8 *)0x0) {
    piVar3 = (int *)(puVar5 + 5);
    *piVar3 = *piVar3 + -1;
    if (*piVar3 == 0) {
      *puVar2 = 0;
    }
  }
  if (puVar5 == (undefined8 *)0x0) {
    return 0;
  }
  if (0xffffe < puVar5[1] - 1) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar17 = puVar5 + 1;
  lVar13 = *plVar17;
  *plVar17 = *plVar17 + -1;
  UNLOCK();
  if ((int)lVar13 != 1) {
    return 0;
  }
  (**(code **)*puVar5)(puVar5,1);
  return 0;
}


