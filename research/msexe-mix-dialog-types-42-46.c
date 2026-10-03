
//===========================================================
// FUN_141f765d0 @ 141f765d0   (842 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141f765d0(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  int *piVar2;
  undefined8 *puVar3;
  byte bVar4;
  undefined4 uVar5;
  longlong lVar6;
  undefined4 **ppuVar7;
  undefined4 *puVar8;
  longlong *plVar9;
  undefined8 *puVar10;
  uint uVar11;
  longlong lVar12;
  undefined1 local_res10 [16];
  uint local_res20;
  undefined4 *local_98;
  longlong local_90;
  longlong local_88;
  longlong local_78;
  longlong *local_68;
  undefined4 local_60;
  ulonglong local_5c;
  undefined8 uStack_54;
  undefined8 local_4c;
  undefined4 *local_40;
  undefined4 local_38;
  
  local_res20 = 0;
  local_5c = _DAT_14327cda0;
  uStack_54 = _UNK_14327cda8;
  local_4c = 0;
  local_40 = (undefined4 *)0x0;
  local_38 = 0;
  local_60 = 0;
  bVar4 = FUN_1406e8ae0(param_3);
  local_5c = (ulonglong)bVar4;
  local_38 = param_2;
  uVar5 = FUN_1406e8c20(param_3);
  if (DAT_143aa84f8 != 0) {
    lVar6 = FUN_141e768f0(DAT_143aa84f8,uVar5);
    if ((lVar6 == 0) || (lVar6 == -0x10)) {
      local_78 = 0;
    }
    else {
      local_78 = lVar6;
      if (0xfffff < *(ulonglong *)(lVar6 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar6 + 0x18) = *(longlong *)(lVar6 + 0x18) + 1;
      UNLOCK();
    }
    if (local_78 == 0) {
      puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
      puVar8[1] = 0;
      *puVar8 = 0xffffffff;
      local_98 = puVar8 + 4;
      puVar8[2] = 0;
      *(undefined1 *)local_98 = 0;
      *puVar8 = 1;
      *(undefined1 *)local_98 = 0;
      puVar8[2] = 0;
      ppuVar7 = &local_98;
      uVar11 = 2;
    }
    else {
      ppuVar7 = (undefined4 **)FUN_141e39b10(local_78,&local_90);
      uVar11 = 1;
    }
    local_res20 = uVar11;
    if (local_40 != (undefined4 *)0x0) {
      FUN_14019f2c0(local_40 + -4);
      local_40 = (undefined4 *)0x0;
    }
    local_40 = *ppuVar7;
    *ppuVar7 = (undefined4 *)0x0;
    if (((uVar11 & 2) != 0) &&
       (uVar11 = uVar11 & 0xfffffffd, local_res20 = uVar11, local_98 != (undefined4 *)0x0)) {
      FUN_14019f2c0(local_98 + -4);
    }
    if (((uVar11 & 1) != 0) && (local_90 != 0)) {
      FUN_14019f2c0(local_90 + -0x10);
    }
    local_88 = FUN_14019b780(&DAT_143ad68a0,0x390);
    lVar6 = 0;
    if (local_88 != 0) {
      lVar6 = FUN_1410dcc60(local_88);
    }
    lVar12 = lVar6 + 0x18;
    if (lVar6 == 0) {
      lVar12 = 0;
    }
    if (lVar12 == 0) {
      local_68 = (longlong *)0x0;
    }
    else {
      local_68 = (longlong *)(lVar12 + -0x18);
      if (local_68 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar12 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar12 + 8) = *(longlong *)(lVar12 + 8) + 1;
        UNLOCK();
      }
    }
    plVar9 = local_68;
    if (local_68 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_1410dd230(plVar9,&local_60);
    if (plVar9 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar9 + 0x130))(plVar9);
    if (0xffffe < plVar9[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar9 = plVar9 + 4;
    lVar6 = *plVar9;
    *plVar9 = *plVar9 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (plVar9 = local_68 + 3, plVar9 != (longlong *)0x0)) {
      (**(code **)*plVar9)(plVar9,1);
    }
    if (local_78 != 0) {
      puVar3 = *(undefined8 **)(local_78 + 0x38);
      if (puVar3 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar3[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar3[1] = puVar3[1] + 1;
        UNLOCK();
      }
      lVar6 = local_78;
      puVar1 = puVar3 + 4;
      FUN_1401d3510(puVar1,local_res10);
      if (0xffffe < *(longlong *)(lVar6 + 0x18) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar9 = (longlong *)(lVar6 + 0x18);
      lVar6 = *plVar9;
      *plVar9 = *plVar9 + -1;
      UNLOCK();
      if ((int)lVar6 == 1) {
        puVar3[6] = 0;
        puVar10 = (undefined8 *)(local_78 + 0x10);
        if (puVar10 != (undefined8 *)0x0) {
          (**(code **)*puVar10)(puVar10,1);
        }
      }
      if (puVar1 != (undefined8 *)0x0) {
        piVar2 = (int *)(puVar3 + 5);
        *piVar2 = *piVar2 + -1;
        if (*piVar2 == 0) {
          *puVar1 = 0;
        }
      }
      if (puVar3 != (undefined8 *)0x0) {
        if (0xffffe < puVar3[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar9 = puVar3 + 1;
        lVar6 = *plVar9;
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if ((int)lVar6 == 1) {
          (**(code **)*puVar3)(puVar3,1);
        }
      }
    }
  }
  if (local_40 != (undefined4 *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  return;
}



//===========================================================
// FUN_141f76920 @ 141f76920   (855 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141f76920(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  int *piVar2;
  undefined8 *puVar3;
  byte bVar4;
  undefined1 uVar5;
  undefined4 uVar6;
  longlong lVar7;
  undefined4 **ppuVar8;
  undefined4 *puVar9;
  longlong *plVar10;
  undefined8 *puVar11;
  uint uVar12;
  longlong lVar13;
  undefined1 local_res10 [16];
  uint local_res20;
  undefined4 *local_98;
  longlong local_90;
  longlong local_88;
  longlong local_78;
  longlong *local_68;
  undefined4 local_60;
  ulonglong local_5c;
  undefined8 uStack_54;
  ulonglong local_4c;
  undefined4 *local_40;
  undefined4 local_38;
  
  local_res20 = 0;
  local_5c = _DAT_14327cda0;
  uStack_54 = _UNK_14327cda8;
  local_4c = 0;
  local_40 = (undefined4 *)0x0;
  local_38 = 0;
  local_60 = 1;
  bVar4 = FUN_1406e8ae0(param_3);
  local_5c = CONCAT44(local_5c._4_4_,(uint)bVar4);
  uVar5 = FUN_1406e8ae0(param_3);
  local_4c = (ulonglong)CONCAT14(uVar5,(undefined4)local_4c);
  local_5c = local_5c & 0xffffffff;
  local_38 = param_2;
  uVar6 = FUN_1406e8c20(param_3);
  if (DAT_143aa84f8 != 0) {
    lVar7 = FUN_141e768f0(DAT_143aa84f8,uVar6);
    if ((lVar7 == 0) || (lVar7 == -0x10)) {
      local_78 = 0;
    }
    else {
      local_78 = lVar7;
      if (0xfffff < *(ulonglong *)(lVar7 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar7 + 0x18) = *(longlong *)(lVar7 + 0x18) + 1;
      UNLOCK();
    }
    if (local_78 == 0) {
      puVar9 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
      puVar9[1] = 0;
      *puVar9 = 0xffffffff;
      local_98 = puVar9 + 4;
      puVar9[2] = 0;
      *(undefined1 *)local_98 = 0;
      *puVar9 = 1;
      *(undefined1 *)local_98 = 0;
      puVar9[2] = 0;
      ppuVar8 = &local_98;
      uVar12 = 2;
    }
    else {
      ppuVar8 = (undefined4 **)FUN_141e39b10(local_78,&local_90);
      uVar12 = 1;
    }
    local_res20 = uVar12;
    if (local_40 != (undefined4 *)0x0) {
      FUN_14019f2c0(local_40 + -4);
      local_40 = (undefined4 *)0x0;
    }
    local_40 = *ppuVar8;
    *ppuVar8 = (undefined4 *)0x0;
    if (((uVar12 & 2) != 0) &&
       (uVar12 = uVar12 & 0xfffffffd, local_res20 = uVar12, local_98 != (undefined4 *)0x0)) {
      FUN_14019f2c0(local_98 + -4);
    }
    if (((uVar12 & 1) != 0) && (local_90 != 0)) {
      FUN_14019f2c0(local_90 + -0x10);
    }
    local_88 = FUN_14019b780(&DAT_143ad68a0,0x390);
    lVar7 = 0;
    if (local_88 != 0) {
      lVar7 = FUN_1410dcc60(local_88);
    }
    lVar13 = lVar7 + 0x18;
    if (lVar7 == 0) {
      lVar13 = 0;
    }
    if (lVar13 == 0) {
      local_68 = (longlong *)0x0;
    }
    else {
      local_68 = (longlong *)(lVar13 + -0x18);
      if (local_68 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
        UNLOCK();
      }
    }
    plVar10 = local_68;
    if (local_68 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_1410dd230(plVar10,&local_60);
    if (plVar10 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar10 + 0x130))(plVar10);
    if (0xffffe < plVar10[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar10 = plVar10 + 4;
    lVar7 = *plVar10;
    *plVar10 = *plVar10 + -1;
    UNLOCK();
    if (((int)lVar7 == 1) && (plVar10 = local_68 + 3, plVar10 != (longlong *)0x0)) {
      (**(code **)*plVar10)(plVar10,1);
    }
    if (local_78 != 0) {
      puVar3 = *(undefined8 **)(local_78 + 0x38);
      if (puVar3 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar3[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar3[1] = puVar3[1] + 1;
        UNLOCK();
      }
      lVar7 = local_78;
      puVar1 = puVar3 + 4;
      FUN_1401d3510(puVar1,local_res10);
      if (0xffffe < *(longlong *)(lVar7 + 0x18) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar10 = (longlong *)(lVar7 + 0x18);
      lVar7 = *plVar10;
      *plVar10 = *plVar10 + -1;
      UNLOCK();
      if ((int)lVar7 == 1) {
        puVar3[6] = 0;
        puVar11 = (undefined8 *)(local_78 + 0x10);
        if (puVar11 != (undefined8 *)0x0) {
          (**(code **)*puVar11)(puVar11,1);
        }
      }
      if (puVar1 != (undefined8 *)0x0) {
        piVar2 = (int *)(puVar3 + 5);
        *piVar2 = *piVar2 + -1;
        if (*piVar2 == 0) {
          *puVar1 = 0;
        }
      }
      if (puVar3 != (undefined8 *)0x0) {
        if (0xffffe < puVar3[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar10 = puVar3 + 1;
        lVar7 = *plVar10;
        *plVar10 = *plVar10 + -1;
        UNLOCK();
        if ((int)lVar7 == 1) {
          (**(code **)*puVar3)(puVar3,1);
        }
      }
    }
  }
  if (local_40 != (undefined4 *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  return;
}



//===========================================================
// FUN_141f76c80 @ 141f76c80   (867 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141f76c80(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  int *piVar2;
  undefined8 *puVar3;
  byte bVar4;
  undefined4 uVar5;
  longlong lVar6;
  undefined4 **ppuVar7;
  undefined4 *puVar8;
  longlong *plVar9;
  undefined8 *puVar10;
  longlong lVar11;
  uint uVar12;
  undefined1 local_res10 [16];
  uint local_res20;
  undefined4 *local_98;
  longlong local_90;
  longlong local_88;
  longlong local_78;
  longlong *local_68;
  undefined4 local_60;
  undefined8 local_5c;
  undefined8 uStack_54;
  undefined8 local_4c;
  undefined4 *local_40;
  undefined4 local_38;
  
  local_res20 = 0;
  local_5c = _DAT_14327cda0;
  uStack_54 = _UNK_14327cda8;
  local_4c = 0;
  local_40 = (undefined4 *)0x0;
  local_38 = 0;
  uVar12 = 2;
  local_60 = 2;
  bVar4 = FUN_1406e8ae0(param_3);
  local_5c = CONCAT44(local_5c._4_4_,(uint)bVar4);
  bVar4 = FUN_1406e8ae0(param_3);
  uStack_54 = CONCAT44(uStack_54._4_4_,(uint)bVar4);
  uVar5 = FUN_1406e8c20(param_3);
  local_4c = CONCAT44(local_4c._4_4_,uVar5);
  local_5c = CONCAT44(2,(undefined4)local_5c);
  local_38 = param_2;
  uVar5 = FUN_1406e8c20(param_3);
  if (DAT_143aa84f8 != 0) {
    lVar6 = FUN_141e768f0(DAT_143aa84f8,uVar5);
    if ((lVar6 == 0) || (lVar6 == -0x10)) {
      local_78 = 0;
    }
    else {
      local_78 = lVar6;
      if (0xfffff < *(ulonglong *)(lVar6 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar6 + 0x18) = *(longlong *)(lVar6 + 0x18) + 1;
      UNLOCK();
    }
    if (local_78 == 0) {
      puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
      puVar8[1] = 0;
      *puVar8 = 0xffffffff;
      local_98 = puVar8 + 4;
      puVar8[2] = 0;
      *(undefined1 *)local_98 = 0;
      *puVar8 = 1;
      *(undefined1 *)local_98 = 0;
      puVar8[2] = 0;
      ppuVar7 = &local_98;
    }
    else {
      ppuVar7 = (undefined4 **)FUN_141e39b10(local_78,&local_90);
      uVar12 = 1;
    }
    local_res20 = uVar12;
    if (local_40 != (undefined4 *)0x0) {
      FUN_14019f2c0(local_40 + -4);
      local_40 = (undefined4 *)0x0;
    }
    local_40 = *ppuVar7;
    *ppuVar7 = (undefined4 *)0x0;
    if (((uVar12 & 2) != 0) &&
       (uVar12 = uVar12 & 0xfffffffd, local_res20 = uVar12, local_98 != (undefined4 *)0x0)) {
      FUN_14019f2c0(local_98 + -4);
    }
    if (((uVar12 & 1) != 0) && (local_90 != 0)) {
      FUN_14019f2c0(local_90 + -0x10);
    }
    local_88 = FUN_14019b780(&DAT_143ad68a0,0x390);
    lVar6 = 0;
    if (local_88 != 0) {
      lVar6 = FUN_1410dcc60(local_88);
    }
    lVar11 = lVar6 + 0x18;
    if (lVar6 == 0) {
      lVar11 = 0;
    }
    if (lVar11 == 0) {
      local_68 = (longlong *)0x0;
    }
    else {
      local_68 = (longlong *)(lVar11 + -0x18);
      if (local_68 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar11 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar11 + 8) = *(longlong *)(lVar11 + 8) + 1;
        UNLOCK();
      }
    }
    plVar9 = local_68;
    if (local_68 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_1410dd230(plVar9,&local_60);
    if (plVar9 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar9 + 0x130))(plVar9);
    if (0xffffe < plVar9[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar9 = plVar9 + 4;
    lVar6 = *plVar9;
    *plVar9 = *plVar9 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (plVar9 = local_68 + 3, plVar9 != (longlong *)0x0)) {
      (**(code **)*plVar9)(plVar9,1);
    }
    if (local_78 != 0) {
      puVar3 = *(undefined8 **)(local_78 + 0x38);
      if (puVar3 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar3[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar3[1] = puVar3[1] + 1;
        UNLOCK();
      }
      lVar6 = local_78;
      puVar1 = puVar3 + 4;
      FUN_1401d3510(puVar1,local_res10);
      if (0xffffe < *(longlong *)(lVar6 + 0x18) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar9 = (longlong *)(lVar6 + 0x18);
      lVar6 = *plVar9;
      *plVar9 = *plVar9 + -1;
      UNLOCK();
      if ((int)lVar6 == 1) {
        puVar3[6] = 0;
        puVar10 = (undefined8 *)(local_78 + 0x10);
        if (puVar10 != (undefined8 *)0x0) {
          (**(code **)*puVar10)(puVar10,1);
        }
      }
      if (puVar1 != (undefined8 *)0x0) {
        piVar2 = (int *)(puVar3 + 5);
        *piVar2 = *piVar2 + -1;
        if (*piVar2 == 0) {
          *puVar1 = 0;
        }
      }
      if (puVar3 != (undefined8 *)0x0) {
        if (0xffffe < puVar3[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar9 = puVar3 + 1;
        lVar6 = *plVar9;
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if ((int)lVar6 == 1) {
          (**(code **)*puVar3)(puVar3,1);
        }
      }
    }
  }
  if (local_40 != (undefined4 *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  return;
}



//===========================================================
// FUN_141f76ff0 @ 141f76ff0   (858 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141f76ff0(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  int *piVar2;
  undefined8 *puVar3;
  byte bVar4;
  undefined4 uVar5;
  longlong lVar6;
  undefined4 **ppuVar7;
  undefined4 *puVar8;
  longlong *plVar9;
  undefined8 *puVar10;
  uint uVar11;
  longlong lVar12;
  undefined1 local_res10 [16];
  uint local_res20;
  undefined4 *local_98;
  longlong local_90;
  longlong local_88;
  longlong local_78;
  longlong *local_68;
  undefined4 local_60;
  undefined8 local_5c;
  undefined8 uStack_54;
  undefined8 local_4c;
  undefined4 *local_40;
  undefined4 local_38;
  
  local_res20 = 0;
  local_5c = _DAT_14327cda0;
  uStack_54 = _UNK_14327cda8;
  local_4c = 0;
  local_40 = (undefined4 *)0x0;
  local_38 = 0;
  local_60 = 3;
  bVar4 = FUN_1406e8ae0(param_3);
  local_5c = CONCAT44(local_5c._4_4_,(uint)bVar4);
  bVar4 = FUN_1406e8ae0(param_3);
  uStack_54 = CONCAT44(uStack_54._4_4_,(uint)bVar4);
  local_5c = CONCAT44(1,(undefined4)local_5c);
  local_38 = param_2;
  uVar5 = FUN_1406e8c20(param_3);
  if (DAT_143aa84f8 != 0) {
    lVar6 = FUN_141e768f0(DAT_143aa84f8,uVar5);
    if ((lVar6 == 0) || (lVar6 == -0x10)) {
      local_78 = 0;
    }
    else {
      local_78 = lVar6;
      if (0xfffff < *(ulonglong *)(lVar6 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar6 + 0x18) = *(longlong *)(lVar6 + 0x18) + 1;
      UNLOCK();
    }
    if (local_78 == 0) {
      puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
      puVar8[1] = 0;
      *puVar8 = 0xffffffff;
      local_98 = puVar8 + 4;
      puVar8[2] = 0;
      *(undefined1 *)local_98 = 0;
      *puVar8 = 1;
      *(undefined1 *)local_98 = 0;
      puVar8[2] = 0;
      ppuVar7 = &local_98;
      uVar11 = 2;
    }
    else {
      ppuVar7 = (undefined4 **)FUN_141e39b10(local_78,&local_90);
      uVar11 = 1;
    }
    local_res20 = uVar11;
    if (local_40 != (undefined4 *)0x0) {
      FUN_14019f2c0(local_40 + -4);
      local_40 = (undefined4 *)0x0;
    }
    local_40 = *ppuVar7;
    *ppuVar7 = (undefined4 *)0x0;
    if (((uVar11 & 2) != 0) &&
       (uVar11 = uVar11 & 0xfffffffd, local_res20 = uVar11, local_98 != (undefined4 *)0x0)) {
      FUN_14019f2c0(local_98 + -4);
    }
    if (((uVar11 & 1) != 0) && (local_90 != 0)) {
      FUN_14019f2c0(local_90 + -0x10);
    }
    local_88 = FUN_14019b780(&DAT_143ad68a0,0x390);
    lVar6 = 0;
    if (local_88 != 0) {
      lVar6 = FUN_1410dcc60(local_88);
    }
    lVar12 = lVar6 + 0x18;
    if (lVar6 == 0) {
      lVar12 = 0;
    }
    if (lVar12 == 0) {
      local_68 = (longlong *)0x0;
    }
    else {
      local_68 = (longlong *)(lVar12 + -0x18);
      if (local_68 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar12 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar12 + 8) = *(longlong *)(lVar12 + 8) + 1;
        UNLOCK();
      }
    }
    plVar9 = local_68;
    if (local_68 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_1410dd230(plVar9,&local_60);
    if (plVar9 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar9 + 0x130))(plVar9);
    if (0xffffe < plVar9[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar9 = plVar9 + 4;
    lVar6 = *plVar9;
    *plVar9 = *plVar9 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (plVar9 = local_68 + 3, plVar9 != (longlong *)0x0)) {
      (**(code **)*plVar9)(plVar9,1);
    }
    if (local_78 != 0) {
      puVar3 = *(undefined8 **)(local_78 + 0x38);
      if (puVar3 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar3[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar3[1] = puVar3[1] + 1;
        UNLOCK();
      }
      lVar6 = local_78;
      puVar1 = puVar3 + 4;
      FUN_1401d3510(puVar1,local_res10);
      if (0xffffe < *(longlong *)(lVar6 + 0x18) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar9 = (longlong *)(lVar6 + 0x18);
      lVar6 = *plVar9;
      *plVar9 = *plVar9 + -1;
      UNLOCK();
      if ((int)lVar6 == 1) {
        puVar3[6] = 0;
        puVar10 = (undefined8 *)(local_78 + 0x10);
        if (puVar10 != (undefined8 *)0x0) {
          (**(code **)*puVar10)(puVar10,1);
        }
      }
      if (puVar1 != (undefined8 *)0x0) {
        piVar2 = (int *)(puVar3 + 5);
        *piVar2 = *piVar2 + -1;
        if (*piVar2 == 0) {
          *puVar1 = 0;
        }
      }
      if (puVar3 != (undefined8 *)0x0) {
        if (0xffffe < puVar3[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar9 = puVar3 + 1;
        lVar6 = *plVar9;
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if ((int)lVar6 == 1) {
          (**(code **)*puVar3)(puVar3,1);
        }
      }
    }
  }
  if (local_40 != (undefined4 *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  return;
}



//===========================================================
// FUN_141f77350 @ 141f77350   (869 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141f77350(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  int *piVar2;
  undefined8 *puVar3;
  byte bVar4;
  undefined4 uVar5;
  longlong lVar6;
  undefined4 **ppuVar7;
  undefined4 *puVar8;
  longlong *plVar9;
  undefined8 *puVar10;
  uint uVar11;
  longlong lVar12;
  undefined1 local_res10 [16];
  uint local_res20;
  undefined4 *local_98;
  longlong local_90;
  longlong local_88;
  longlong local_78;
  longlong *local_68;
  undefined4 local_60;
  undefined8 local_5c;
  undefined8 uStack_54;
  undefined8 local_4c;
  undefined4 *local_40;
  undefined4 local_38;
  
  local_res20 = 0;
  local_5c = _DAT_14327cda0;
  uStack_54 = _UNK_14327cda8;
  local_4c = 0;
  local_40 = (undefined4 *)0x0;
  local_38 = 0;
  local_60 = 4;
  bVar4 = FUN_1406e8ae0(param_3);
  local_5c = CONCAT44(local_5c._4_4_,(uint)bVar4);
  bVar4 = FUN_1406e8ae0(param_3);
  uStack_54 = CONCAT44(uStack_54._4_4_,(uint)bVar4);
  uVar5 = FUN_1406e8c20(param_3);
  uStack_54 = CONCAT44(uVar5,(undefined4)uStack_54);
  local_5c = CONCAT44(1,(undefined4)local_5c);
  local_38 = param_2;
  uVar5 = FUN_1406e8c20(param_3);
  if (DAT_143aa84f8 != 0) {
    lVar6 = FUN_141e768f0(DAT_143aa84f8,uVar5);
    if ((lVar6 == 0) || (lVar6 == -0x10)) {
      local_78 = 0;
    }
    else {
      local_78 = lVar6;
      if (0xfffff < *(ulonglong *)(lVar6 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar6 + 0x18) = *(longlong *)(lVar6 + 0x18) + 1;
      UNLOCK();
    }
    if (local_78 == 0) {
      puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
      puVar8[1] = 0;
      *puVar8 = 0xffffffff;
      local_98 = puVar8 + 4;
      puVar8[2] = 0;
      *(undefined1 *)local_98 = 0;
      *puVar8 = 1;
      *(undefined1 *)local_98 = 0;
      puVar8[2] = 0;
      ppuVar7 = &local_98;
      uVar11 = 2;
    }
    else {
      ppuVar7 = (undefined4 **)FUN_141e39b10(local_78,&local_90);
      uVar11 = 1;
    }
    local_res20 = uVar11;
    if (local_40 != (undefined4 *)0x0) {
      FUN_14019f2c0(local_40 + -4);
      local_40 = (undefined4 *)0x0;
    }
    local_40 = *ppuVar7;
    *ppuVar7 = (undefined4 *)0x0;
    if (((uVar11 & 2) != 0) &&
       (uVar11 = uVar11 & 0xfffffffd, local_res20 = uVar11, local_98 != (undefined4 *)0x0)) {
      FUN_14019f2c0(local_98 + -4);
    }
    if (((uVar11 & 1) != 0) && (local_90 != 0)) {
      FUN_14019f2c0(local_90 + -0x10);
    }
    local_88 = FUN_14019b780(&DAT_143ad68a0,0x390);
    lVar6 = 0;
    if (local_88 != 0) {
      lVar6 = FUN_1410dcc60(local_88);
    }
    lVar12 = lVar6 + 0x18;
    if (lVar6 == 0) {
      lVar12 = 0;
    }
    if (lVar12 == 0) {
      local_68 = (longlong *)0x0;
    }
    else {
      local_68 = (longlong *)(lVar12 + -0x18);
      if (local_68 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar12 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar12 + 8) = *(longlong *)(lVar12 + 8) + 1;
        UNLOCK();
      }
    }
    plVar9 = local_68;
    if (local_68 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    FUN_1410dd230(plVar9,&local_60);
    if (plVar9 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar9 + 0x130))(plVar9);
    if (0xffffe < plVar9[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar9 = plVar9 + 4;
    lVar6 = *plVar9;
    *plVar9 = *plVar9 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (plVar9 = local_68 + 3, plVar9 != (longlong *)0x0)) {
      (**(code **)*plVar9)(plVar9,1);
    }
    if (local_78 != 0) {
      puVar3 = *(undefined8 **)(local_78 + 0x38);
      if (puVar3 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar3[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar3[1] = puVar3[1] + 1;
        UNLOCK();
      }
      lVar6 = local_78;
      puVar1 = puVar3 + 4;
      FUN_1401d3510(puVar1,local_res10);
      if (0xffffe < *(longlong *)(lVar6 + 0x18) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar9 = (longlong *)(lVar6 + 0x18);
      lVar6 = *plVar9;
      *plVar9 = *plVar9 + -1;
      UNLOCK();
      if ((int)lVar6 == 1) {
        puVar3[6] = 0;
        puVar10 = (undefined8 *)(local_78 + 0x10);
        if (puVar10 != (undefined8 *)0x0) {
          (**(code **)*puVar10)(puVar10,1);
        }
      }
      if (puVar1 != (undefined8 *)0x0) {
        piVar2 = (int *)(puVar3 + 5);
        *piVar2 = *piVar2 + -1;
        if (*piVar2 == 0) {
          *puVar1 = 0;
        }
      }
      if (puVar3 != (undefined8 *)0x0) {
        if (0xffffe < puVar3[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar9 = puVar3 + 1;
        lVar6 = *plVar9;
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if ((int)lVar6 == 1) {
          (**(code **)*puVar3)(puVar3,1);
        }
      }
    }
  }
  if (local_40 != (undefined4 *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  return;
}



//===========================================================
// FUN_1410de8c0 @ 1410de8c0   (206 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1410de8c0(longlong param_1,undefined1 param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  FUN_1406ed520(local_468,0xf3);
  FUN_1406ed9d0(local_468,*(undefined4 *)(param_1 + 0x2c8));
  iVar1 = *(int *)(param_1 + 0x2a0);
  if (iVar1 == 0) {
    uVar2 = 0x42;
  }
  else if (iVar1 == 1) {
    uVar2 = 0x43;
  }
  else if (iVar1 == 2) {
    uVar2 = 0x44;
  }
  else if (iVar1 == 3) {
    uVar2 = 0x45;
  }
  else if (iVar1 == 4) {
    uVar2 = 0x46;
  }
  else {
    uVar2 = 0;
  }
  FUN_1406ed840(local_468,uVar2);
  FUN_1406ed840(local_468,param_2);
  FUN_1415d3990(DAT_143ac18a0,local_468);
  FUN_1406ed610(local_468);
  return;
}



//===========================================================
// FUN_1410de9a0 @ 1410de9a0   (209 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1410de9a0(longlong param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  if (*(char *)(param_1 + 800) != '\0') {
    *(undefined1 *)(param_1 + 800) = 0;
    FUN_1406ed520(local_468,0xf3);
    FUN_1406ed9d0(local_468,*(undefined4 *)(param_1 + 0x2c8));
    FUN_1406ed840(local_468,0x47);
    iVar1 = *(int *)(param_1 + 0x2a0);
    if (iVar1 == 0) {
      uVar2 = 0x42;
    }
    else if (iVar1 == 1) {
      uVar2 = 0x43;
    }
    else if (iVar1 == 2) {
      uVar2 = 0x44;
    }
    else if (iVar1 == 3) {
      uVar2 = 0x45;
    }
    else if (iVar1 == 4) {
      uVar2 = 0x46;
    }
    else {
      uVar2 = 0;
    }
    FUN_1406ed840(local_468,uVar2);
    FUN_1415d3990(DAT_143ac18a0,local_468);
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_1410dea80 @ 1410dea80   (403 bytes)
//===========================================================

void FUN_1410dea80(longlong *param_1,char param_2)

{
  IUnknown *pIVar1;
  int iVar2;
  undefined8 uVar3;
  uint local_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 local_68;
  short local_60 [4];
  longlong lStack_58;
  undefined8 local_50;
  undefined1 local_48 [8];
  longlong lStack_40;
  undefined8 local_38;
  uint local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 local_18;
  
  if (param_2 == '\0') {
    FUN_1410de9a0();
    uVar3 = 7;
  }
  else {
    pIVar1 = (IUnknown *)param_1[0x61];
    if (pIVar1 != (IUnknown *)0x0) {
      iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1,1);
      if (iVar2 < 0) {
        _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
      pIVar1 = (IUnknown *)param_1[0x61];
      if (pIVar1 != (IUnknown *)0x0) {
        FUN_1404a7700(local_60,&DAT_143a8b8d8);
        FUN_1404a7700(&local_78,&DAT_143a8b8d8);
        lStack_40 = lStack_58;
        local_38 = local_50;
        local_28 = local_78;
        uStack_24 = uStack_74;
        uStack_20 = uStack_70;
        uStack_1c = uStack_6c;
        local_18 = local_68;
        iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x280))(pIVar1,0,&local_28,local_48);
        if (iVar2 < 0) {
          _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
        }
        if ((short)local_78 == 8) {
          local_78 = local_78 & 0xffff0000;
          if (CONCAT44(uStack_6c,uStack_70) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_6c,uStack_70) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_78);
        }
        if (local_60[0] == 8) {
          local_60[0] = 0;
          if (lStack_58 != 0) {
            (*DAT_143ad5990)(lStack_58 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_60);
        }
        *(undefined1 *)((longlong)param_1 + 0x321) = 1;
        return;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    FUN_1410de9a0();
    uVar3 = 6;
  }
                    /* WARNING: Could not recover jumptable at 0x0001410dec02. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(*param_1 + 0x138))(param_1,uVar3);
  return;
}


