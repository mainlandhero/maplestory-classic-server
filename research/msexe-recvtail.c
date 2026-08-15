
//===========================================================
// FUN_1406e9530 @ 1406e9530   (677 bytes)
//===========================================================

undefined4 FUN_1406e9530(longlong param_1,longlong param_2,undefined4 *param_3,int param_4)

{
  longlong *plVar1;
  int iVar2;
  undefined8 *puVar3;
  undefined2 uVar4;
  uint uVar5;
  longlong lVar6;
  undefined8 *puVar7;
  uint uVar8;
  int iVar9;
  longlong lVar10;
  uint uVar11;
  
  if (param_3 != (undefined4 *)0x0) {
    *param_3 = *(undefined4 *)(param_1 + 8);
  }
  lVar6 = *(longlong *)(param_2 + 8);
  if (lVar6 == 0) {
    FUN_142e52ed0(0x428,0);
    lVar6 = *(longlong *)(param_2 + 8);
  }
  lVar10 = *(longlong *)(lVar6 + 0x28);
  uVar8 = *(uint *)(lVar6 + 0x20);
  if (*(int *)(param_1 + 8) == 0) {
    if (*(uint *)(param_1 + 0x18) < 4) {
      uVar11 = 4 - *(uint *)(param_1 + 0x18);
      if (uVar8 < uVar11) {
        uVar11 = uVar8;
      }
      FUN_1406ea710(param_1,lVar10,(ulonglong)uVar11);
      if (3 < *(uint *)(param_1 + 0x18)) {
        uVar4 = FUN_1406e8b80(param_1);
        *(undefined2 *)(param_1 + 0x1c) = uVar4;
        uVar5 = FUN_1406e8b80(param_1);
        if (param_4 == 0) {
          uVar5 = uVar5 & 0xffff;
        }
        else {
          uVar5 = (uint)*(ushort *)(param_1 + 0x1c) ^ uVar5 & 0xffff;
        }
        *(uint *)(param_1 + 0x20) = uVar5;
      }
      uVar8 = uVar8 - uVar11;
      if (uVar8 != 0) {
        lVar10 = lVar10 + (ulonglong)uVar11;
        goto LAB_1406e95f4;
      }
LAB_1406e95e1:
      FUN_1406f19d0(param_2);
      goto LAB_1406e97b9;
    }
LAB_1406e95f4:
    if (*(uint *)(param_1 + 0x20) < 0xff00) {
      *(undefined4 *)(param_1 + 8) = 1;
    }
    else {
      uVar11 = 8 - *(int *)(param_1 + 0x18);
      if (uVar8 < uVar11) {
        uVar11 = uVar8;
      }
      FUN_1406ea710(param_1,lVar10,(ulonglong)uVar11);
      if (7 < *(uint *)(param_1 + 0x18)) {
        uVar5 = FUN_1406e8c20(param_1);
        if (param_4 != 0) {
          uVar5 = uVar5 ^ *(ushort *)(param_1 + 0x1c);
        }
        *(uint *)(param_1 + 0x20) = uVar5;
        *(int *)(param_1 + 0x24) = *(int *)(param_1 + 0x24) + -4;
        *(int *)(param_1 + 0x18) = *(int *)(param_1 + 0x18) + -4;
        *(undefined4 *)(param_1 + 8) = 1;
      }
      uVar8 = uVar8 - uVar11;
      if (uVar8 == 0) goto LAB_1406e95e1;
      lVar10 = lVar10 + (ulonglong)uVar11;
    }
  }
  uVar11 = (*(int *)(param_1 + 0x20) - *(int *)(param_1 + 0x18)) + 4;
  if (uVar8 < uVar11) {
    uVar11 = uVar8;
  }
  FUN_1406ea710(param_1,lVar10,uVar11);
  if (*(int *)(param_1 + 0x20) + 4U <= *(uint *)(param_1 + 0x18)) {
    *(undefined4 *)(param_1 + 8) = 2;
  }
  iVar9 = uVar8 - uVar11;
  if (iVar9 == 0) {
    puVar7 = (undefined8 *)0x0;
  }
  else {
    lVar6 = *(longlong *)(param_2 + 8);
    if (lVar6 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar6 = *(longlong *)(param_2 + 8);
    }
    lVar10 = lVar6;
    if (lVar6 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar10 = *(longlong *)(param_2 + 8);
    }
    iVar2 = *(int *)(lVar10 + 0x20);
    puVar7 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x40);
    if (puVar7 == (undefined8 *)0x0) {
      puVar7 = (undefined8 *)0x0;
    }
    else {
      lVar10 = *(longlong *)(lVar6 + 0x28);
      puVar7[3] = 0;
      puVar7[1] = 0;
      puVar7[2] = 0;
      *puVar7 = &PTR_FUN_143296c70;
      puVar7[7] = 0;
      *(int *)(puVar7 + 4) = iVar9;
      puVar7[5] = (ulonglong)(uint)(iVar2 - iVar9) + lVar10;
      FUN_1406f13e0(puVar7 + 6,lVar6);
    }
  }
  if ((*(longlong *)(param_2 + 8) - 1U < 999) || (*(longlong *)(param_2 + 8) == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (puVar7 != (undefined8 *)0x0) {
    if (0xfffff < (ulonglong)puVar7[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    puVar7[1] = puVar7[1] + 1;
    UNLOCK();
  }
  puVar3 = *(undefined8 **)(param_2 + 8);
  *(undefined8 **)(param_2 + 8) = puVar7;
  if (puVar3 != (undefined8 *)0x0) {
    if (0xffffe < puVar3[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = puVar3 + 1;
    lVar6 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar6 == 1) {
      (**(code **)*puVar3)(puVar3,1);
    }
  }
LAB_1406e97b9:
  return *(undefined4 *)(param_1 + 8);
}



//===========================================================
// FUN_1406e88d0 @ 1406e88d0   (266 bytes)
//===========================================================

undefined4 * FUN_1406e88d0(undefined4 *param_1,longlong param_2)

{
  uint uVar1;
  ulonglong *puVar2;
  undefined8 uVar3;
  longlong lVar4;
  
  *param_1 = 2;
  uVar3 = 0;
  *(undefined8 *)(param_1 + 4) = 0;
  param_1[1] = *(undefined4 *)(param_2 + 4);
  param_1[2] = 2;
  uVar1 = *(uint *)(param_2 + 0x18);
  lVar4 = *(longlong *)(param_2 + 0x10);
  if (lVar4 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar4 = *(longlong *)(param_2 + 0x10);
    if (lVar4 != 0) goto LAB_1406e8928;
  }
  else {
LAB_1406e8928:
    if (*(int *)(lVar4 + -8) != 0) goto LAB_1406e8946;
  }
  FUN_142e54290(0xbc,0,0);
  lVar4 = *(longlong *)(param_2 + 0x10);
LAB_1406e8946:
  if (*(longlong *)(param_1 + 4) != 0) {
    thunk_FUN_140205820(*(longlong *)(param_1 + 4) + -8,0);
    *(undefined8 *)(param_1 + 4) = 0;
  }
  if (uVar1 != 0) {
    puVar2 = (ulonglong *)FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar1 + 8);
    if (puVar2 == (ulonglong *)0x0) {
      *(undefined8 *)(param_1 + 4) = 0;
    }
    else {
      *(ulonglong **)(param_1 + 4) = puVar2 + 1;
      if (puVar2 + 1 != (ulonglong *)0x0) {
        *puVar2 = (ulonglong)uVar1;
        uVar3 = *(undefined8 *)(param_1 + 4);
      }
    }
  }
  FUN_142ef7ba0(uVar3,lVar4,*(undefined4 *)(param_2 + 0x18));
  param_1[6] = *(undefined4 *)(param_2 + 0x18);
  *(undefined2 *)(param_1 + 7) = *(undefined2 *)(param_2 + 0x1c);
  param_1[8] = *(undefined4 *)(param_2 + 0x20);
  param_1[9] = *(undefined4 *)(param_2 + 0x24);
  *(undefined4 *)(param_2 + 8) = 0;
  *(undefined4 *)(param_2 + 0x18) = 0;
  *(undefined4 *)(param_2 + 0x24) = 0;
  return param_1;
}



//===========================================================
// FUN_1406e9810 @ 1406e9810   (5 bytes)
//===========================================================

undefined2 FUN_1406e9810(longlong param_1)

{
  return *(undefined2 *)(param_1 + 0x20);
}


