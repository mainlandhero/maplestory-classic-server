
//===========================================================
// FUN_14085acd0 @ 14085acd0   (187 bytes)
//===========================================================

void FUN_14085acd0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  char cVar2;
  undefined4 uVar3;
  
  uVar1 = FUN_1406e8f10(param_2);
  *(undefined8 *)(param_1 + 8) = uVar1;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x10) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x14) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x18) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1c) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x20) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x24) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x28) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x2c) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x34) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x38) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x30) = uVar3;
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x48) = uVar3;
  cVar2 = FUN_1406e8ae0(param_2);
  *(bool *)(param_1 + 4) = cVar2 != '\0';
  return;
}



//===========================================================
// FUN_141d3a540 @ 141d3a540   (531 bytes)
//===========================================================

void FUN_141d3a540(longlong param_1)

{
  int iVar1;
  int iVar2;
  longlong lVar3;
  
  iVar1 = *(int *)(param_1 + 0x60);
  if (iVar1 == 0x878368) {
    lVar3 = FUN_14019b780(&DAT_143ad68a0,0x12d0);
    if (lVar3 != 0) {
      FUN_140ff87c0(lVar3,param_1);
    }
    return;
  }
  iVar2 = FUN_1402b3f30(iVar1);
  if (iVar2 != 0) {
    lVar3 = FUN_14019b780(&DAT_143ad68a0,0x12c0);
    if (lVar3 != 0) {
      FUN_141fd4a00(lVar3,param_1);
    }
    return;
  }
  iVar2 = FUN_1402b3e60(iVar1);
  if (iVar2 != 0) {
    lVar3 = FUN_14019b780(&DAT_143ad68a0,0x12d8);
    if (lVar3 != 0) {
      FUN_141fd6dc0(lVar3,param_1);
    }
    return;
  }
  if (((iVar1 != 0x7dbc02) && (iVar1 != 0x7dbc15)) && (iVar1 != 0x7dbc16)) {
    if (iVar1 == 0x7dbc13) {
      lVar3 = FUN_14019b780(&DAT_143ad68a0,0x1298);
      if (lVar3 != 0) {
        FUN_140ff7fa0(lVar3,param_1);
      }
      return;
    }
    if (*(char *)(param_1 + 0x84) != '\0') {
      if (*(char *)(param_1 + 0x81) != '\0') {
        lVar3 = FUN_14047c1d0(param_1);
        if (*(longlong *)(lVar3 + 8) != 0) {
          lVar3 = FUN_14019b780(&DAT_143ad68a0,0x1290);
          if (lVar3 != 0) {
            FUN_141fcf850(lVar3,param_1);
          }
          return;
        }
      }
      if (*(char *)(param_1 + 0x84) != '\0') {
        lVar3 = FUN_14019b780(&DAT_143ad68a0,0x1270);
        if (lVar3 != 0) {
          FUN_141fdb720(lVar3,param_1);
        }
        return;
      }
    }
    lVar3 = FUN_14019b780(&DAT_143ad68a0,0x1238);
    if (lVar3 != 0) {
      FUN_141c4cee0(lVar3,param_1);
    }
    return;
  }
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x1298);
  if (lVar3 != 0) {
    FUN_141fd4010(lVar3,param_1);
  }
  return;
}



//===========================================================
// FUN_141d2efc0 @ 141d2efc0   (152 bytes)
//===========================================================

longlong FUN_141d2efc0(longlong param_1,uint param_2)

{
  longlong lVar1;
  bool bVar2;
  longlong lVar3;
  undefined1 local_18 [8];
  longlong local_10;
  
  lVar3 = 0;
  if (*(longlong *)(param_1 + 0x68) != 0) {
    for (lVar1 = *(longlong *)
                  (*(longlong *)(param_1 + 0x68) +
                  ((ulonglong)param_2 % (ulonglong)*(uint *)(param_1 + 0x70)) * 8); lVar1 != 0;
        lVar1 = *(longlong *)(lVar1 + 8)) {
      if (*(uint *)(lVar1 + 0x10) == param_2) {
        lVar3 = *(longlong *)(*(longlong *)(lVar1 + 0x18) + 8);
        local_10 = lVar3;
        if (lVar3 != 0) {
          if (0xfffff < *(ulonglong *)(lVar3 + 0x18)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar3 + 0x18) = *(longlong *)(lVar3 + 0x18) + 1;
          UNLOCK();
        }
        bVar2 = true;
        goto LAB_141d2f004;
      }
    }
  }
  bVar2 = false;
LAB_141d2f004:
  if (bVar2) {
    FUN_140f08f00(local_18);
  }
  return lVar3;
}



//===========================================================
// FUN_141d22c10 @ 141d22c10   (145 bytes)
//===========================================================

void FUN_141d22c10(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    puVar3 = (undefined8 *)(lVar2 + -0x28);
    if (0xffffe < *(longlong *)(lVar2 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + -0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      lVar2 = *(longlong *)(param_1 + 8);
      if ((lVar2 != 0) && (*(longlong *)(lVar2 + -0x10) != 0)) {
        LOCK();
        *(undefined8 *)(*(longlong *)(lVar2 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(lVar2 + -0x10) + 4) != 0);
      }
      if (puVar3 != (undefined8 *)0x0) {
        (**(code **)*puVar3)(puVar3,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_141d0ed80 @ 141d0ed80   (313 bytes)
//===========================================================

undefined8 * FUN_141d0ed80(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  void *pvVar4;
  undefined8 *puVar5;
  int *piVar6;
  
  pvVar4 = Self;
  lVar3 = DAT_143ad2a70;
  plVar1 = (longlong *)(DAT_143ad2a70 + 0x18);
  LOCK();
  lVar2 = *plVar1;
  if (lVar2 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar2 == 0) {
LAB_141d0ede9:
    *(undefined4 *)(lVar3 + 0x20) = 1;
  }
  else if ((void *)*plVar1 == pvVar4) {
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  else {
    while( true ) {
      pvVar4 = Self;
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_141d0ede9;
      if ((void *)*plVar1 == pvVar4) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  piVar6 = (int *)(lVar3 + 0x20);
  puVar5 = *(undefined8 **)(lVar3 + 0x28);
  if (puVar5 == (undefined8 *)0x0) {
    puVar5 = (undefined8 *)FUN_14019d3c0(0x90);
    *(undefined8 **)(lVar3 + 0x28) = puVar5;
  }
  *(undefined8 *)(lVar3 + 0x28) = *puVar5;
  *piVar6 = *piVar6 + -1;
  if (*piVar6 == 0) {
    *plVar1 = 0;
  }
  *(undefined8 *)((longlong)puVar5 + 0x2c) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x34) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x3c) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x44) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x4c) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x54) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x5c) = 0;
  *(undefined8 *)((longlong)puVar5 + 100) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x6c) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x74) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x7c) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x84) = 0;
  puVar5[3] = 0;
  puVar5[1] = 0;
  puVar5[2] = 0;
  *puVar5 = &PTR_FUN_1434088a0;
  puVar5[4] = &PTR_LAB_1434088a8;
  *(undefined4 *)(puVar5 + 5) = 0;
  *(undefined1 *)((longlong)puVar5 + 0x2c) = 0;
  puVar5[6] = 1;
  puVar5[7] = 0;
  puVar5[8] = 0;
  puVar5[9] = 0;
  puVar5[10] = 0;
  puVar5[0xb] = 1;
  *(undefined4 *)(puVar5 + 0xc) = 0;
  puVar5[0xd] = 0;
  *(undefined4 *)(puVar5 + 0xe) = 0;
  puVar5[0xf] = 0x3ff0000000000000;
  *(undefined4 *)(puVar5 + 0x10) = 0;
  puVar5[0x11] = 0x3ff0000000000000;
  return puVar5;
}


