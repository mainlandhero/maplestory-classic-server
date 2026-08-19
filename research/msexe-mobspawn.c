
//===========================================================
// FUN_141d33630 @ 141d33630   (1581 bytes)
//===========================================================

void FUN_141d33630(longlong param_1,undefined8 param_2)

{
  undefined8 *puVar1;
  int *piVar2;
  char cVar3;
  byte bVar4;
  char cVar5;
  uint uVar6;
  undefined4 uVar7;
  int iVar8;
  longlong *plVar9;
  longlong lVar10;
  longlong lVar11;
  undefined8 uVar12;
  longlong *plVar13;
  undefined8 *puVar14;
  undefined ***local_res20;
  undefined1 local_d8 [4];
  uint local_d4;
  uint local_d0;
  undefined4 local_cc;
  undefined1 local_c8 [8];
  longlong *local_c0;
  undefined1 local_b8 [8];
  longlong *local_b0;
  undefined8 local_a8;
  undefined1 local_a0 [8];
  longlong *local_98;
  undefined1 local_90 [8];
  longlong local_88;
  undefined **local_80;
  undefined1 *local_78;
  undefined ***local_48;
  
  cVar3 = FUN_1406e8ae0(param_2);
  local_80 = &PTR_LAB_143409208;
  local_78 = &LAB_140c93a40;
  local_48 = &local_80;
  local_res20 = &local_80;
  uVar6 = FUN_1406e8c20(param_2);
  if (uVar6 == 0) {
    uVar6 = FUN_1406e8c20(param_2);
    if (uVar6 == (uVar6 / 0xb2) * 0xb2) goto LAB_141d336d3;
  }
  else if (uVar6 != (uVar6 / 0xb2) * 0xb2) goto LAB_141d336d3;
  if (local_48 == (undefined ***)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ed3024();
  }
  (*(code *)(*local_48)[2])();
LAB_141d336d3:
  if (local_48 != (undefined ***)0x0) {
    (*(code *)(*local_48)[4])
              (local_48,CONCAT71((int7)((ulonglong)&local_80 >> 8),local_48 != &local_80));
  }
  local_d0 = uVar6;
  bVar4 = FUN_1406e8ae0(param_2);
  local_d4 = (uint)bVar4;
  uVar7 = FUN_1406e8c20(param_2);
  local_cc = uVar7;
  plVar9 = (longlong *)FUN_141d2efc0(param_1,uVar6);
  if (plVar9 == (longlong *)0x0) {
    uVar12 = FUN_140495990(uVar7);
    plVar9 = (longlong *)FUN_141d3a540(uVar12);
    if ((plVar9 == (longlong *)0x0) || (plVar9 == (longlong *)0xfffffffffffffff0)) {
      local_c0 = (longlong *)0x0;
    }
    else {
      local_c0 = plVar9;
      if (0xfffff < (ulonglong)plVar9[3]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar9[3] = plVar9[3] + 1;
      UNLOCK();
    }
    local_a8 = FUN_141d4edf0(param_1 + 0x38,local_c8);
    FUN_141d4f320(param_1 + 0x68,&local_d0,&local_a8);
    if (local_c0 != (longlong *)0x0) {
      puVar14 = (undefined8 *)local_c0[7];
      if (puVar14 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar14[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar14[1] = puVar14[1] + 1;
        UNLOCK();
      }
      plVar13 = local_c0;
      puVar1 = puVar14 + 4;
      FUN_1401d3510(puVar1,&local_res20);
      if (0xffffe < plVar13[3] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar13 = plVar13 + 3;
      lVar11 = *plVar13;
      *plVar13 = *plVar13 + -1;
      UNLOCK();
      if ((int)lVar11 == 1) {
        puVar14[6] = 0;
        plVar13 = local_c0 + 2;
        if (plVar13 != (longlong *)0x0) {
          (**(code **)*plVar13)(plVar13,1);
        }
      }
      if (puVar1 != (undefined8 *)0x0) {
        piVar2 = (int *)(puVar14 + 5);
        *piVar2 = *piVar2 + -1;
        if (*piVar2 == 0) {
          *puVar1 = 0;
        }
      }
      if (puVar14 != (undefined8 *)0x0) {
        if (0xffffe < puVar14[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar13 = puVar14 + 1;
        lVar11 = *plVar13;
        *plVar13 = *plVar13 + -1;
        UNLOCK();
        if ((int)lVar11 == 1) {
          (**(code **)*puVar14)(puVar14,1);
        }
      }
    }
    FUN_141d4f990(DAT_143aa84a0,local_cc);
    FUN_141c543e0(plVar9,1);
    cVar5 = FUN_1406e8ae0(param_2);
    if (cVar5 != '\0') {
      FUN_141cc9410(plVar9,param_2);
    }
    FUN_141c76190(plVar9,param_2,local_d4);
    (**(code **)(*plVar9 + 0x38))(plVar9,uVar6,param_2);
    lVar10 = FUN_141c54e50(plVar9,local_90);
    lVar11 = *(longlong *)(lVar10 + 8);
    if (lVar11 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar11 = *(longlong *)(lVar10 + 8);
    }
    lVar10 = local_88;
    iVar8 = *(int *)(lVar11 + 0x4c0);
    if (local_88 != 0) {
      puVar14 = (undefined8 *)(local_88 + -0x28);
      if (0xffffe < *(longlong *)(local_88 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar13 = (longlong *)(lVar10 + -0x20);
      lVar11 = *plVar13;
      *plVar13 = *plVar13 + -1;
      UNLOCK();
      if ((int)lVar11 == 1) {
        if ((local_88 != 0) && (*(longlong *)(local_88 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_88 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_88 + -0x10) + 4) != 0);
        }
        if (puVar14 != (undefined8 *)0x0) {
          (**(code **)*puVar14)(puVar14,1);
        }
      }
    }
    if (iVar8 != 0) {
      FUN_141cc1940(plVar9,0);
    }
  }
  else {
    FUN_141c543e0(plVar9,1);
    cVar5 = FUN_1406e8ae0(param_2);
    if (cVar5 != '\0') {
      FUN_141cc9410(plVar9,param_2);
    }
    FUN_141c76190(plVar9,param_2,local_d4);
  }
  iVar8 = FUN_141c55b00(plVar9);
  if (iVar8 != 0) {
    iVar8 = FUN_141cc20b0(plVar9);
    if (iVar8 == 0) {
      if (plVar9 == (longlong *)0xfffffffffffffff0) {
        local_b0 = (longlong *)0x0;
      }
      else {
        local_b0 = plVar9;
        if (plVar9 != (longlong *)0x0) {
          if (0xfffff < (ulonglong)plVar9[3]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          plVar9[3] = plVar9[3] + 1;
          UNLOCK();
        }
      }
      plVar13 = local_b0;
      FUN_141d4f5b0(param_1 + 0xa8,&local_d0,local_b8);
      if (plVar13 != (longlong *)0x0) {
        puVar14 = (undefined8 *)plVar13[7];
        if (puVar14 != (undefined8 *)0x0) {
          if (0xfffff < (ulonglong)puVar14[1]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          puVar14[1] = puVar14[1] + 1;
          UNLOCK();
          plVar13 = local_b0;
        }
        puVar1 = puVar14 + 4;
        FUN_1401d3510(puVar1,local_d8);
        if (0xffffe < plVar13[3] - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar13 = plVar13 + 3;
        lVar11 = *plVar13;
        *plVar13 = *plVar13 + -1;
        UNLOCK();
        if ((int)lVar11 == 1) {
          puVar14[6] = 0;
          plVar13 = local_b0 + 2;
          if (plVar13 != (longlong *)0x0) {
            (**(code **)*plVar13)(plVar13,1);
          }
        }
        if (puVar1 != (undefined8 *)0x0) {
          piVar2 = (int *)(puVar14 + 5);
          *piVar2 = *piVar2 + -1;
          if (*piVar2 == 0) {
            *puVar1 = 0;
          }
        }
        if (puVar14 != (undefined8 *)0x0) {
          if (0xffffe < puVar14[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar13 = puVar14 + 1;
          lVar11 = *plVar13;
          *plVar13 = *plVar13 + -1;
          UNLOCK();
          if ((int)lVar11 == 1) {
            (**(code **)*puVar14)(puVar14,1);
          }
        }
      }
    }
    else {
      if ((*(longlong *)(param_1 + 0xa0) - 1U < 999) || (*(longlong *)(param_1 + 0xa0) == -1)) {
        FUN_142e52ed0(0x447);
      }
      local_98 = (longlong *)0x0;
      if ((plVar9 != (longlong *)0xfffffffffffffff0) &&
         (local_98 = plVar9, plVar9 != (longlong *)0x0)) {
        if (0xfffff < (ulonglong)plVar9[3]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        plVar9[3] = plVar9[3] + 1;
        UNLOCK();
      }
      uVar12 = *(undefined8 *)(param_1 + 0xa0);
      *(longlong **)(param_1 + 0xa0) = local_98;
      local_98 = (longlong *)uVar12;
      FUN_140f08f00(local_a0);
    }
  }
  if ((DAT_143aa8518 != 0) && (lVar11 = FUN_141c54dd0(plVar9), lVar11 != 0)) {
    uVar12 = FUN_141c54dd0(plVar9);
    cVar5 = FUN_14047a660(uVar12);
    lVar11 = DAT_143aa8518;
    if (cVar5 != '\0') {
      uVar7 = FUN_141c54eb0(plVar9);
      FUN_1429029f0(lVar11,uVar7);
    }
  }
  iVar8 = FUN_141c54f00(plVar9);
  if ((iVar8 != 0) && (iVar8 = FUN_141c54fc0(plVar9), iVar8 == 0)) {
    *(int *)(param_1 + 0xd0) = *(int *)(param_1 + 0xd0) + 1;
  }
  FUN_141c55ab0(plVar9,cVar3 != '\0');
  FUN_141c89180(plVar9);
  FUN_142e10390(plVar9);
  lVar11 = FUN_141892840();
  if (lVar11 != 0) {
    plVar13 = (longlong *)FUN_141892840();
    (**(code **)(*plVar13 + 0x108))(plVar13,plVar9);
  }
  return;
}


