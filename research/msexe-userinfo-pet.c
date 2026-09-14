
//===========================================================
// FUN_1414be310 @ 1414be310   (1930 bytes)
//===========================================================

void FUN_1414be310(longlong param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  undefined4 uVar5;
  longlong lVar6;
  longlong lVar7;
  undefined8 *puVar8;
  longlong lVar9;
  longlong *plVar10;
  undefined8 uVar11;
  undefined8 *puVar12;
  undefined8 *puVar13;
  uint uVar14;
  undefined8 local_res20;
  undefined8 in_stack_ffffffffffffff18;
  ulonglong uVar15;
  longlong local_98;
  undefined1 local_90 [8];
  undefined1 local_88 [8];
  longlong local_80;
  undefined1 local_78 [8];
  longlong local_70;
  undefined1 local_68 [8];
  longlong local_60;
  undefined1 local_58 [8];
  longlong local_50;
  undefined1 local_48 [8];
  undefined8 *local_40;
  
  lVar7 = DAT_143aa84a0;
  uVar5 = (undefined4)((ulonglong)in_stack_ffffffffffffff18 >> 0x20);
  puVar13 = (undefined8 *)0x0;
  bVar1 = false;
  if (DAT_143aa84a0 == 0) {
    return;
  }
  lVar6 = FUN_142cbe730(DAT_143aa84a0);
  if (lVar6 == 0) {
    return;
  }
  FUN_142cbe730(lVar7);
  if (*(longlong **)(param_1 + 0x308) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x308) + 0x18))();
    FUN_140d948a0(param_1 + 0x300);
  }
  if (*(longlong **)(param_1 + 0x2f8) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x2f8) + 0x18))();
    FUN_140cbca60(param_1 + 0x2f0);
  }
  lVar7 = FUN_14019b780(&DAT_143ad68a0,0x108);
  puVar8 = puVar13;
  if (lVar7 != 0) {
    puVar8 = (undefined8 *)FUN_1416ed1a0(lVar7);
  }
  if ((*(longlong *)(param_1 + 0x308) - 1U < 999) || (*(longlong *)(param_1 + 0x308) == -1)) {
    FUN_142e52ed0(0x447);
  }
  puVar12 = puVar8 + 3;
  if (puVar8 == (undefined8 *)0x0) {
    puVar12 = puVar13;
  }
  puVar8 = puVar13;
  if ((puVar12 != (undefined8 *)0x0) && (puVar8 = puVar12 + -3, puVar8 != (undefined8 *)0x0)) {
    if (0xfffff < (ulonglong)puVar12[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    puVar12[1] = puVar12[1] + 1;
    UNLOCK();
  }
  lVar7 = *(longlong *)(param_1 + 0x308);
  *(undefined8 **)(param_1 + 0x308) = puVar8;
  if (lVar7 != 0) {
    if (0xffffe < *(longlong *)(lVar7 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar10 = (longlong *)(lVar7 + 0x20);
    lVar6 = *plVar10;
    *plVar10 = *plVar10 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (puVar8 = (undefined8 *)(lVar7 + 0x18), puVar8 != (undefined8 *)0x0)) {
      (**(code **)*puVar8)(puVar8,1);
    }
  }
  plVar10 = *(longlong **)(param_1 + 0x308);
  if (plVar10 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
    plVar10 = *(longlong **)(param_1 + 0x308);
  }
  uVar15 = 0;
  (**(code **)(*plVar10 + 0x80))(plVar10,param_1,0x7d1,1,5,0xfb,CONCAT44(uVar5,0x1b),0x79,0);
  lVar7 = *(longlong *)(param_1 + 0x308);
  if (lVar7 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar7 = *(longlong *)(param_1 + 0x308);
  }
  *(undefined4 *)(lVar7 + 0x78) = 0x8f;
  lVar7 = *(longlong *)(param_1 + 0x308);
  if (lVar7 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar7 = *(longlong *)(param_1 + 0x308);
  }
  lVar6 = FUN_1414be0e0(param_1,local_68);
  if (*(longlong *)(lVar6 + 8) == 0) {
    iVar2 = 0;
  }
  else {
    lVar9 = FUN_1414be0e0(param_1,local_78);
    bVar1 = true;
    lVar6 = *(longlong *)(lVar9 + 8);
    if (lVar6 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar6 = *(longlong *)(lVar9 + 8);
    }
    if (*(longlong *)(lVar6 + 0x28) == 0) {
      iVar2 = -2;
    }
    else {
      iVar2 = *(int *)(*(longlong *)(lVar6 + 0x28) + -8) + -2;
    }
  }
  FUN_1416ee330(lVar7,iVar2);
  lVar7 = local_70;
  if ((bVar1) && (local_70 != 0)) {
    puVar8 = (undefined8 *)(local_70 + -0x28);
    if (0xffffe < *(longlong *)(local_70 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar10 = (longlong *)(lVar7 + -0x20);
    lVar7 = *plVar10;
    *plVar10 = *plVar10 + -1;
    UNLOCK();
    if ((int)lVar7 == 1) {
      if ((local_70 != 0) && (*(longlong *)(local_70 + -0x10) != 0)) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_70 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_70 + -0x10) + 4) != 0);
      }
      if (puVar8 != (undefined8 *)0x0) {
        (**(code **)*puVar8)(puVar8,1);
      }
    }
    local_70 = 0;
  }
  lVar7 = local_60;
  if (local_60 != 0) {
    puVar8 = (undefined8 *)(local_60 + -0x28);
    if (0xffffe < *(longlong *)(local_60 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar10 = (longlong *)(lVar7 + -0x20);
    lVar7 = *plVar10;
    *plVar10 = *plVar10 + -1;
    UNLOCK();
    if ((int)lVar7 == 1) {
      if ((local_60 != 0) && (*(longlong *)(local_60 + -0x10) != 0)) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_60 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_60 + -0x10) + 4) != 0);
      }
      if (puVar8 != (undefined8 *)0x0) {
        (**(code **)*puVar8)(puVar8,1);
      }
    }
  }
  local_res20 = 0;
  lVar6 = FUN_141aa81d0(param_1 + 0x1458,local_58,L"UI/UIWindow2.img/UserInfo/pet/BtException",2000,
                        0,0,0,0xff,uVar15 & 0xffffffff00000000,0,0,0,0,0,1,&local_res20);
  lVar7 = param_1 + 0x2f0;
  if ((*(longlong *)(param_1 + 0x2f8) - 1U < 999) || (*(longlong *)(param_1 + 0x2f8) == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (lVar7 == lVar6) {
    FUN_142e52d50(0x45c,1);
  }
  lVar9 = *(longlong *)(lVar6 + 8);
  if (lVar9 != 0) {
    if (0xfffff < *(ulonglong *)(lVar9 + 0x20)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar9 + 0x20) = *(longlong *)(lVar9 + 0x20) + 1;
    UNLOCK();
  }
  FUN_140cbca60(lVar7);
  lVar9 = local_50;
  *(undefined8 *)(param_1 + 0x2f8) = *(undefined8 *)(lVar6 + 8);
  if (local_50 != 0) {
    if (0xffffe < *(longlong *)(local_50 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar10 = (longlong *)(lVar9 + 0x20);
    lVar6 = *plVar10;
    *plVar10 = *plVar10 + -1;
    UNLOCK();
    if ((int)lVar6 == 1) {
      puVar8 = (undefined8 *)(local_50 + 0x18);
      if (local_50 == 0) {
        puVar8 = puVar13;
      }
      if (puVar8 != (undefined8 *)0x0) {
        (**(code **)*puVar8)(puVar8,1);
      }
    }
  }
  FUN_1414be0e0(param_1,local_48);
  puVar13 = local_40;
  if (local_40 == (undefined8 *)0x0) {
    lVar7 = *(longlong *)(param_1 + 0x2f8);
    if (lVar7 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar7 = *(longlong *)(param_1 + 0x2f8);
    }
    (**(code **)(*(longlong *)(lVar7 + 8) + 0x70))((longlong *)(lVar7 + 8),0);
    if ((DAT_143acda20 != 0) && (FUN_142bf3f70(), DAT_143acda20 != 0)) {
      (*(code *)**(undefined8 **)(DAT_143acda20 + 8))((undefined8 *)(DAT_143acda20 + 8),1);
    }
    goto LAB_1414bea1d;
  }
  if (DAT_143aa8518 == 0) goto LAB_1414bea1d;
  iVar2 = FUN_142770370();
  if ((iVar2 < 1) || ((*(byte *)((longlong)puVar13 + 0x1c) & 8) == 0)) {
    lVar6 = FUN_141087b60(lVar7);
    uVar11 = 0;
  }
  else {
    lVar6 = FUN_141087b60(lVar7);
    uVar11 = 1;
  }
  (**(code **)(*(longlong *)(lVar6 + 8) + 0x70))((longlong *)(lVar6 + 8),uVar11);
  iVar2 = *(int *)(param_1 + 0x248);
  lVar7 = FUN_141087b60(lVar7);
  iVar3 = (**(code **)(*(longlong *)(lVar7 + 8) + 0x78))();
  if (iVar3 == 0) {
LAB_1414be950:
    if ((DAT_143acda20 != 0) && (FUN_142bf3f70(), DAT_143acda20 != 0)) {
      (*(code *)**(undefined8 **)(DAT_143acda20 + 8))((undefined8 *)(DAT_143acda20 + 8),1);
    }
  }
  else {
    if (*(longlong *)(param_1 + 0x2e8) == 0) {
      uVar14 = *(int *)(param_1 + 0x248) >> 4 & 0xf;
    }
    else {
      lVar7 = FUN_1414af8f0(param_1 + 0x2e0);
      uVar14 = *(uint *)(lVar7 + 0x90);
    }
    if (DAT_143acda20 == 0) {
LAB_1414be8de:
      uVar5 = (**(code **)(*(longlong *)(param_1 + 8) + 0x98))(param_1 + 8);
      iVar3 = (**(code **)(*(longlong *)(param_1 + 8) + 0x90))(param_1 + 8);
      if ((DAT_143acda20 == 0) && (local_80 = FUN_14019b780(&DAT_143ad68a0,0x1400), local_80 != 0))
      {
        FUN_142719d80(local_80,iVar3 + 0x10f,uVar5,uVar14);
      }
    }
    else {
      uVar4 = FUN_14271b980();
      if (uVar4 != uVar14) {
        if ((DAT_143acda20 != 0) && (FUN_142bf3f70(), DAT_143acda20 != 0)) {
          (*(code *)**(undefined8 **)(DAT_143acda20 + 8))((undefined8 *)(DAT_143acda20 + 8),1);
        }
        goto LAB_1414be8de;
      }
    }
    if ((iVar2 >> 2 & 1U) == 0) goto LAB_1414be950;
  }
  if (DAT_143aa8328 != 0) {
    if (((undefined4 *)*puVar13 == (undefined4 *)0x0) ||
       (iVar2 = FUN_14038a660(DAT_143aa8328,*(undefined4 *)*puVar13), iVar2 < 0)) {
      uVar11 = FUN_1403edf80(local_88,L"UI/UIWindow2.img/UserInfo/pet/backgrnd",0xffffffff);
      FUN_142bf86b0(param_1,uVar11,0,0,1,0);
    }
    else {
      local_98 = 0;
      puVar8 = (undefined8 *)
               FUN_1401c21c0(&local_98,L"UI/UIWindow2.img/UserInfo/pet/ActionPet/%d/0",iVar2);
      uVar11 = FUN_1403edf80(local_90,*puVar8,0xffffffff);
      FUN_142bf86b0(param_1,uVar11,0,0,0,0);
      if (local_98 != 0) {
        FUN_1401bebb0(local_98 + -0x10);
      }
    }
  }
LAB_1414bea1d:
  if (puVar13 != (undefined8 *)0x0) {
    puVar8 = puVar13 + -5;
    if (0xffffe < puVar13[-4] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar10 = puVar13 + -4;
    lVar7 = *plVar10;
    *plVar10 = *plVar10 + -1;
    UNLOCK();
    if ((int)lVar7 == 1) {
      if (local_40[-2] != 0) {
        LOCK();
        *(undefined8 *)(local_40[-2] + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(local_40[-2] + 4) != 0);
      }
      if (puVar8 != (undefined8 *)0x0) {
        (**(code **)*puVar8)(puVar8,1);
      }
    }
  }
  return;
}


