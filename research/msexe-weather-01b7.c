
//===========================================================
// FUN_141853820 @ 141853820   (1001 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141853aed) */

void FUN_141853820(undefined8 param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  void *pvVar4;
  char cVar5;
  int iVar6;
  undefined8 *puVar7;
  int *piVar8;
  int *piVar9;
  int *piVar10;
  int iVar11;
  int iVar12;
  int *piVar13;
  int *piVar14;
  int *piVar15;
  int *piVar16;
  int local_res18;
  int *local_88;
  int *local_80;
  longlong local_78;
  longlong *local_70;
  undefined1 *local_68;
  int *local_60;
  int *local_50;
  undefined1 local_48 [8];
  int *local_40;
  
  iVar6 = FUN_1406e8c20(param_2);
  piVar14 = (int *)0x0;
  local_80 = (int *)0x0;
  local_res18 = 200;
  local_50 = (int *)0x0;
  piVar9 = piVar14;
  piVar15 = piVar14;
  if (iVar6 != 0) {
    puVar7 = (undefined8 *)FUN_1406e9050(param_2,&local_78);
    piVar9 = (int *)*puVar7;
    *puVar7 = 0;
    local_80 = piVar9;
    if (local_78 != 0) {
      FUN_14019f2c0(local_78 + -0x10);
    }
    local_res18 = FUN_1406e8c20(param_2);
    cVar5 = FUN_1406e8ae0(param_2);
    pvVar4 = Self;
    lVar3 = DAT_143ac9800;
    piVar15 = (int *)0x0;
    if (cVar5 != '\0') {
      plVar1 = (longlong *)(DAT_143ac9800 + 0x18);
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      local_70 = plVar1;
      if (lVar2 == 0) {
LAB_141853909:
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
          if (lVar2 == 0) goto LAB_141853909;
          if ((void *)*plVar1 == pvVar4) break;
          (*DAT_143262828)(0);
        }
        *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
      }
      piVar15 = (int *)(lVar3 + 0x20);
      puVar7 = *(undefined8 **)(lVar3 + 0x28);
      if (puVar7 == (undefined8 *)0x0) {
        puVar7 = (undefined8 *)FUN_14019d3c0(0xa0,0x10);
        *(undefined8 **)(lVar3 + 0x28) = puVar7;
      }
      *(undefined8 *)(lVar3 + 0x28) = *puVar7;
      *piVar15 = *piVar15 + -1;
      if (*piVar15 == 0) {
        *plVar1 = 0;
      }
      puVar7[5] = 0;
      puVar7[6] = 0;
      puVar7[7] = 0;
      puVar7[8] = 0;
      puVar7[9] = 0;
      puVar7[10] = 0;
      puVar7[0xb] = 0;
      puVar7[0xc] = 0;
      puVar7[0xd] = 0;
      puVar7[0xe] = 0;
      puVar7[0xf] = 0;
      puVar7[0x10] = 0;
      puVar7[0x11] = 0;
      puVar7[0x12] = 0;
      puVar7[0x13] = 0;
      puVar7[3] = 0;
      puVar7[2] = 0;
      *puVar7 = &PTR_FUN_1433c9ea0;
      puVar7[4] = &PTR_LAB_1433c9ea8;
      puVar7[1] = 1;
      piVar15 = (int *)(puVar7 + 5);
      local_50 = piVar15;
      if (piVar15 == (int *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_1406e9170(param_2,piVar15,0x78);
      piVar9 = local_80;
    }
  }
  local_68 = local_48;
  piVar16 = (int *)0x0;
  local_40 = piVar15;
  if (piVar15 != (int *)0x0) {
    if (0xfffff < *(ulonglong *)(piVar15 + -8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(piVar15 + -8) = *(longlong *)(piVar15 + -8) + 1;
    UNLOCK();
    piVar9 = local_80;
    piVar16 = local_50;
  }
  local_88 = (int *)0x0;
  piVar10 = piVar9;
  piVar15 = local_88;
  if ((piVar9 == (int *)0x0) || (piVar13 = piVar9 + -4, piVar13 == (int *)0x0)) goto LAB_141853b53;
  if (*piVar13 != -1) {
    if (*piVar13 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar13 = *piVar13 + 1;
    UNLOCK();
    piVar10 = local_80;
    piVar16 = local_50;
    piVar15 = piVar9;
    if (local_88 != (int *)0x0) {
      FUN_14019f2c0(local_88 + -4);
      piVar10 = local_80;
      piVar16 = local_50;
    }
    goto LAB_141853b53;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  piVar13 = (int *)0xffffffffffffffff;
  do {
    piVar13 = (int *)((longlong)piVar13 + 1);
  } while (*(char *)((longlong)piVar9 + (longlong)piVar13) != '\0');
  iVar11 = (int)piVar13;
  iVar12 = 0;
  if (0 < iVar11) {
    iVar12 = iVar11;
  }
  piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar12 + 0x11));
  piVar8[1] = iVar12;
  *piVar8 = -1;
  piVar15 = piVar8 + 4;
  piVar8[2] = 0;
  *(undefined1 *)piVar15 = 0;
  local_60 = piVar15;
  FUN_142ef7ba0(piVar15,piVar9,(longlong)iVar11);
  if (*piVar8 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar11 == -1) || (iVar11 <= piVar8[1])) {
    *piVar8 = 1;
    if (iVar11 != -1) goto LAB_141853aab;
    if (piVar15 != (int *)0x0) {
      piVar14 = (int *)0xffffffffffffffff;
      do {
        piVar14 = (int *)((longlong)piVar14 + 1);
      } while (*(char *)((longlong)piVar15 + (longlong)piVar14) != '\0');
    }
  }
  else {
    FUN_142e54290(0x90,piVar8[1],(ulonglong)piVar13 & 0xffffffff);
    *piVar8 = 1;
LAB_141853aab:
    *(undefined1 *)((longlong)piVar15 + (longlong)iVar11) = 0;
    piVar14 = piVar13;
  }
  iVar12 = (int)piVar14;
  if ((iVar12 < 0) || (piVar8[1] + 1 <= iVar12)) {
    FUN_142e54290(0x9c,(ulonglong)piVar14 & 0xffffffff);
  }
  piVar8[2] = iVar12;
  if (local_88 != (int *)0x0) {
    FUN_14019f2c0(local_88 + -4);
  }
LAB_141853b53:
  local_88 = piVar15;
  FUN_14185b1c0(param_1,iVar6,&local_88,0,local_res18 * 1000,local_48);
  if (piVar16 != (int *)0x0) {
    if (0xffffe < *(longlong *)(piVar16 + -8) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(piVar16 + -8);
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    piVar10 = local_80;
    if ((int)lVar3 == 1) {
      if (*(longlong *)(local_50 + -4) != 0) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_50 + -4) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_50 + -4) + 4) != 0);
      }
      (*(code *)**(undefined8 **)(piVar16 + -10))(piVar16 + -10,1);
      piVar10 = local_80;
    }
  }
  if (piVar10 != (int *)0x0) {
    FUN_14019f2c0(piVar10 + -4);
  }
  return;
}


