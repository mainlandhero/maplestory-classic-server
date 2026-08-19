
//===========================================================
// FUN_142797be0 @ 142797be0   (848 bytes)
//===========================================================

void FUN_142797be0(longlong param_1,undefined8 param_2)

{
  longlong *plVar1;
  int *piVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined1 *puVar5;
  void *pvVar6;
  int iVar7;
  ulonglong uVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  undefined1 *puVar11;
  longlong *plVar12;
  undefined1 *puVar13;
  undefined1 local_48 [8];
  undefined1 *local_40;
  undefined8 *local_30;
  
  local_40 = (undefined1 *)0x0;
  puVar5 = *(undefined1 **)(param_1 + 0x1200);
  puVar11 = (undefined1 *)0x0;
  while (puVar5 != (undefined1 *)0x0) {
    uVar8 = *(ulonglong *)(puVar5 + -0x20);
    if ((uVar8 != 0) && (uVar8 < 0x10001)) {
      FUN_142e52ed0(0x33e);
      uVar8 = *(ulonglong *)(puVar5 + -0x20);
    }
    puVar13 = (undefined1 *)0x0;
    if (uVar8 != 0) {
      puVar13 = (undefined1 *)(uVar8 + 0x28);
    }
    if ((puVar11 + -1 < (undefined1 *)0x3e7) || (puVar11 == (undefined1 *)0xffffffffffffffff)) {
      FUN_142e52ed0(0x447,puVar11);
    }
    if (local_48 == puVar5) {
      FUN_142e52d50(0x45c,1);
    }
    lVar3 = *(longlong *)(puVar5 + 8);
    if (lVar3 != 0) {
      if (0xfffff < *(ulonglong *)(lVar3 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar3 + 0x18) = *(longlong *)(lVar3 + 0x18) + 1;
      UNLOCK();
      puVar11 = local_40;
    }
    if (puVar11 != (undefined1 *)0x0) {
      puVar4 = *(undefined8 **)(puVar11 + 0x38);
      local_30 = puVar4;
      if (puVar4 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar4[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar4[1] = puVar4[1] + 1;
        UNLOCK();
      }
      puVar4 = local_30;
      pvVar6 = Self;
      plVar12 = local_30 + 4;
      LOCK();
      lVar3 = *plVar12;
      if (lVar3 == 0) {
        *plVar12 = (longlong)Self;
      }
      UNLOCK();
      if (lVar3 == 0) {
LAB_142797d31:
        *(undefined4 *)(puVar4 + 5) = 1;
      }
      else {
        if ((void *)*plVar12 != pvVar6) {
          while( true ) {
            pvVar6 = Self;
            LOCK();
            lVar3 = *plVar12;
            if (lVar3 == 0) {
              *plVar12 = (longlong)Self;
            }
            UNLOCK();
            if (lVar3 == 0) goto LAB_142797d31;
            if ((void *)*plVar12 == pvVar6) break;
            (*DAT_143262828)(0);
          }
        }
        *(int *)(puVar4 + 5) = *(int *)(puVar4 + 5) + 1;
      }
      puVar11 = local_40;
      if (0xffffe < *(longlong *)(local_40 + 0x18) - 1U) {
        FUN_142e541f0(0x31e);
      }
      puVar10 = local_30;
      LOCK();
      plVar1 = (longlong *)(puVar11 + 0x18);
      lVar3 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar3 == 1) {
        local_30[6] = 0;
        puVar9 = (undefined8 *)(local_40 + 0x10);
        if (puVar9 != (undefined8 *)0x0) {
          (**(code **)*puVar9)(puVar9,1);
        }
      }
      piVar2 = (int *)(puVar4 + 5);
      *piVar2 = *piVar2 + -1;
      if (*piVar2 == 0) {
        *plVar12 = 0;
      }
      if (puVar10 != (undefined8 *)0x0) {
        if (0xffffe < puVar10[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = puVar10 + 1;
        lVar3 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if ((int)lVar3 == 1) {
          (**(code **)*local_30)(local_30,1);
        }
        local_30 = (undefined8 *)0x0;
      }
    }
    puVar11 = *(undefined1 **)(puVar5 + 8);
    puVar5 = puVar13;
    local_40 = puVar11;
    if ((puVar11 != (undefined1 *)0x0) &&
       (iVar7 = FUN_1407f5ce0(*(undefined4 *)(puVar11 + 0x30c)), iVar7 != 0)) {
      FUN_1420dd920(puVar11,param_2);
      FUN_1420dd220(puVar11,1);
    }
  }
  if (puVar11 != (undefined1 *)0x0) {
    puVar4 = *(undefined8 **)(puVar11 + 0x38);
    if (puVar4 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)puVar4[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      puVar4[1] = puVar4[1] + 1;
      UNLOCK();
    }
    pvVar6 = Self;
    plVar12 = puVar4 + 4;
    LOCK();
    lVar3 = *plVar12;
    if (lVar3 == 0) {
      *plVar12 = (longlong)Self;
    }
    UNLOCK();
    if (lVar3 == 0) {
LAB_142797e91:
      *(undefined4 *)(puVar4 + 5) = 1;
    }
    else {
      if ((void *)*plVar12 != pvVar6) {
        while( true ) {
          pvVar6 = Self;
          LOCK();
          lVar3 = *plVar12;
          if (lVar3 == 0) {
            *plVar12 = (longlong)Self;
          }
          UNLOCK();
          if (lVar3 == 0) goto LAB_142797e91;
          if ((void *)*plVar12 == pvVar6) break;
          (*DAT_143262828)(0);
        }
      }
      *(int *)(puVar4 + 5) = *(int *)(puVar4 + 5) + 1;
    }
    puVar11 = local_40;
    if (0xffffe < *(longlong *)(local_40 + 0x18) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(puVar11 + 0x18);
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      puVar4[6] = 0;
      puVar10 = (undefined8 *)(local_40 + 0x10);
      if (puVar10 != (undefined8 *)0x0) {
        (**(code **)*puVar10)(puVar10,1);
      }
    }
    if (plVar12 != (longlong *)0x0) {
      piVar2 = (int *)(puVar4 + 5);
      *piVar2 = *piVar2 + -1;
      if (*piVar2 == 0) {
        *plVar12 = 0;
      }
    }
    if (puVar4 != (undefined8 *)0x0) {
      if (0xffffe < puVar4[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar12 = puVar4 + 1;
      lVar3 = *plVar12;
      *plVar12 = *plVar12 + -1;
      UNLOCK();
      if ((int)lVar3 == 1) {
        (**(code **)*puVar4)(puVar4,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1429b6c90 @ 1429b6c90   (156 bytes)
//===========================================================

undefined8 FUN_1429b6c90(longlong param_1,uint param_2)

{
  longlong lVar1;
  uint uVar2;
  
  if (DAT_143ac1b90 != 0) {
    if ((*(longlong *)(param_1 + 0x10) != 0) && (uVar2 = FUN_14276df20(), uVar2 == param_2)) {
      return *(undefined8 *)(param_1 + 0x10);
    }
    if (*(longlong *)(param_1 + 0xf8) != 0) {
      for (lVar1 = *(longlong *)
                    (*(longlong *)(param_1 + 0xf8) +
                    ((ulonglong)param_2 % (ulonglong)*(uint *)(param_1 + 0x100)) * 8); lVar1 != 0;
          lVar1 = *(longlong *)(lVar1 + 8)) {
        if (*(uint *)(lVar1 + 0x10) == param_2) {
          if (lVar1 == -0x18) {
            return 0;
          }
          if (*(longlong *)(lVar1 + 0x20) == 0) {
            return 0;
          }
          return *(undefined8 *)(*(longlong *)(lVar1 + 0x20) + 0x28);
        }
      }
    }
  }
  return 0;
}


