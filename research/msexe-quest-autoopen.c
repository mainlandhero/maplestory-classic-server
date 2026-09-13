
//===========================================================
// FUN_142ce5e80 @ 142ce5e80   (238 bytes)
//===========================================================

void FUN_142ce5e80(undefined8 param_1,longlong *param_2,undefined4 param_3,int param_4,int param_5,
                  undefined4 param_6)

{
  undefined8 uVar1;
  longlong lVar2;
  uint uVar3;
  int iVar4;
  longlong lVar5;
  uint uVar6;
  ulonglong uVar7;
  undefined4 uVar8;
  longlong lVar9;
  
  uVar7 = (ulonglong)param_4;
  uVar1 = FUN_1408f6690();
  if (*param_2 == 0) {
    iVar4 = 0;
  }
  else {
    iVar4 = *(int *)(*param_2 + -8);
  }
  if (param_4 < iVar4) {
    lVar5 = 0;
    lVar9 = uVar7 * 4;
    do {
      if ((param_5 != -1) && (param_5 <= lVar5)) {
        return;
      }
      lVar2 = *param_2;
      if (lVar2 == 0) {
        uVar3 = 0;
      }
      else {
        uVar3 = *(uint *)(lVar2 + -8);
      }
      uVar6 = (uint)uVar7;
      if (((int)uVar6 < 0) || (uVar3 <= uVar6)) {
        if (lVar2 == 0) {
          uVar8 = 0;
        }
        else {
          uVar8 = *(undefined4 *)(lVar2 + -8);
        }
        FUN_142e54290(0xbc,uVar7 & 0xffffffff,uVar8);
        lVar2 = *param_2;
      }
      FUN_142ce5f80(param_1,*(undefined4 *)(lVar9 + lVar2),param_3,uVar1,param_6);
      uVar7 = (ulonglong)(uVar6 + 1);
      lVar5 = lVar5 + 1;
      lVar9 = lVar9 + 4;
    } while ((int)(uVar6 + 1) < iVar4);
  }
  return;
}



//===========================================================
// FUN_141f0e110 @ 141f0e110   (735 bytes)
//===========================================================

undefined8 * FUN_141f0e110(undefined8 *param_1,int param_2,undefined4 param_3)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 uVar3;
  char cVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  int iVar11;
  longlong lVar12;
  undefined8 uVar13;
  undefined8 *puVar14;
  longlong lVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 local_80;
  undefined8 local_78;
  longlong local_68;
  
  uVar17 = 0;
  param_1[3] = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  *param_1 = &PTR_FUN_143418c08;
  *(int *)(param_1 + 4) = param_2;
  *(undefined4 *)((longlong)param_1 + 0x24) = param_3;
  param_1[6] = 0;
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  lVar12 = FUN_142cbe730(DAT_143aa84a0);
  uVar13 = (**(code **)(*DAT_143aa84a0 + 0x30))();
  local_78 = FUN_142cbec90(DAT_143aa84a0);
  uVar5 = FUN_142cafb20(DAT_143aa84a0);
  uVar6 = FUN_142cb95e0(DAT_143aa84a0);
  uVar7 = FUN_142cb85b0(DAT_143aa84a0);
  uVar8 = FUN_142cb85d0(DAT_143aa84a0);
  uVar9 = thunk_FUN_1413b8e00(DAT_143aa84a0);
  puVar14 = (undefined8 *)FUN_142cf2680(DAT_143aa84a0);
  local_80 = *puVar14;
  if (DAT_143aa8518 != 0) {
    lVar15 = FUN_1428f74d0();
    lVar2 = local_68;
    uVar17 = *(undefined8 *)(lVar15 + 8);
    if (local_68 != 0) {
      puVar14 = (undefined8 *)(local_68 + -0x28);
      if (0xffffe < *(longlong *)(local_68 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = (longlong *)(lVar2 + -0x20);
      lVar2 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar2 == 1) {
        if ((local_68 != 0) && (*(longlong *)(local_68 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_68 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_68 + -0x10) + 4) != 0);
        }
        if (puVar14 != (undefined8 *)0x0) {
          (**(code **)*puVar14)(puVar14);
        }
      }
      local_68 = 0;
    }
  }
  uVar16 = FUN_1408f6690();
  if (*(longlong *)(lVar12 + 0x1273) != 0) {
    for (lVar2 = *(longlong *)
                  (*(longlong *)(lVar12 + 0x1273) +
                  ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(lVar12 + 0x127b)) * 8);
        lVar2 != 0; lVar2 = *(longlong *)(lVar2 + 8)) {
      if (*(int *)(lVar2 + 0x10) == param_2) {
        *(undefined4 *)(param_1 + 5) = 1;
        goto LAB_141f0e3cf;
      }
    }
  }
  cVar4 = FUN_1402e3340(lVar12,param_2);
  uVar3 = DAT_143aa9d98;
  if (cVar4 != '\0') {
    uVar10 = FUN_142cec650();
    iVar11 = FUN_14070fae0(uVar3,param_2,param_3,lVar12,uVar13,local_78,uVar10,uVar5,uVar6,uVar7,
                           uVar8,&local_80,uVar9,0,uVar16,0,1,uVar17);
    if (iVar11 != 0) {
      *(undefined4 *)(param_1 + 5) = 2;
      goto LAB_141f0e3cf;
    }
  }
  *(undefined4 *)(param_1 + 5) = 0;
LAB_141f0e3cf:
  FUN_141f10d90(param_1);
  return param_1;
}



//===========================================================
// FUN_141f0e4c0 @ 141f0e4c0   (2495 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141f0e4c0(longlong param_1,int param_2)

{
  longlong lVar1;
  IUnknown *pIVar2;
  longlong *plVar3;
  char cVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  longlong **pplVar10;
  longlong *plVar11;
  longlong *plVar12;
  undefined8 *puVar13;
  undefined4 uVar14;
  longlong *plVar15;
  uint uVar16;
  uint uVar17;
  undefined1 auStack_628 [32];
  undefined8 local_608;
  undefined8 local_600;
  longlong *local_5f8;
  undefined4 local_5f0;
  undefined4 local_5e8;
  longlong *local_5e0;
  undefined8 local_5d8;
  undefined2 local_5d0 [2];
  undefined2 local_5cc;
  longlong *local_5c8;
  undefined4 local_5c0;
  undefined4 uStack_5bc;
  int local_5b8;
  undefined4 uStack_5b4;
  undefined4 local_5b0;
  undefined4 uStack_5ac;
  longlong *local_5a8;
  undefined8 local_5a0;
  longlong *local_598;
  longlong local_590;
  uint local_588;
  undefined4 uStack_584;
  undefined4 uStack_580;
  undefined4 uStack_57c;
  undefined8 local_578;
  uint local_570;
  undefined4 uStack_56c;
  undefined4 uStack_568;
  undefined4 uStack_564;
  undefined8 local_560;
  longlong *local_558;
  uint local_550;
  undefined4 uStack_54c;
  undefined4 uStack_548;
  undefined4 uStack_544;
  undefined8 local_540;
  uint local_538;
  undefined4 uStack_534;
  undefined4 uStack_530;
  undefined4 uStack_52c;
  undefined8 local_528;
  undefined8 local_520;
  undefined8 uStack_518;
  undefined8 local_510;
  undefined8 uStack_508;
  undefined8 local_500;
  undefined8 uStack_4f8;
  undefined8 local_4f0;
  undefined8 uStack_4e8;
  undefined1 local_4e0 [24];
  undefined8 local_4c8;
  undefined8 uStack_4c0;
  undefined8 local_4b8;
  undefined8 uStack_4b0;
  undefined4 local_4a8;
  undefined4 uStack_4a4;
  undefined4 uStack_4a0;
  undefined4 uStack_49c;
  undefined4 local_498;
  undefined4 uStack_494;
  undefined4 uStack_490;
  undefined4 uStack_48c;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_628;
  plVar12 = (longlong *)0x0;
  local_5d8 = (undefined4 *)((ulonglong)local_5d8 & 0xffffffff00000000);
  if (DAT_143aa8518 == 0) {
    return;
  }
  if (DAT_143aa84a0 == (longlong *)0x0) {
    return;
  }
  local_5b8 = param_2;
  cVar4 = FUN_141b1f960(DAT_143abea80,1);
  if (cVar4 != '\0') {
    return;
  }
  (**(code **)(*(longlong *)(DAT_143aa8518 + 8) + 0x30))((longlong *)(DAT_143aa8518 + 8),local_5d0);
  uVar8 = FUN_142cbe730(DAT_143aa84a0);
  uVar9 = (**(code **)(*DAT_143aa84a0 + 0x30))();
  local_5a8 = (longlong *)FUN_142cbec90(DAT_143aa84a0);
  local_5c0 = FUN_142cb85b0(DAT_143aa84a0);
  local_5b0 = FUN_142cb85d0(DAT_143aa84a0);
  uVar5 = FUN_142cafb20(DAT_143aa84a0);
  local_5c8 = (longlong *)CONCAT44(local_5c8._4_4_,uVar5);
  if (DAT_143aa8518 == 0) {
    local_590 = 0;
    pplVar10 = &local_598;
    uVar16 = 2;
  }
  else {
    pplVar10 = (longlong **)FUN_1428f74d0(DAT_143aa8518,local_4e0);
    uVar16 = 1;
  }
  lVar1 = local_590;
  plVar15 = pplVar10[1];
  if ((uVar16 & 2) != 0) {
    local_5d8 = (undefined4 *)(CONCAT44(local_5d8._4_4_,uVar16) & 0xfffffffffffffffd);
    uVar16 = uVar16 & 0xfffffffd;
    if (local_590 != 0) {
      puVar13 = (undefined8 *)(local_590 + -0x28);
      if (0xffffe < *(longlong *)(local_590 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar11 = (longlong *)(lVar1 + -0x20);
      lVar1 = *plVar11;
      *plVar11 = *plVar11 + -1;
      UNLOCK();
      if ((int)lVar1 == 1) {
        if (*(longlong *)(local_590 + -0x10) != 0) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_590 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_590 + -0x10) + 4) != 0);
        }
        if (puVar13 != (undefined8 *)0x0) {
          (**(code **)*puVar13)(puVar13,1);
        }
      }
      uVar16 = (uint)local_5d8;
    }
  }
  if ((uVar16 & 1) != 0) {
    uVar16 = uVar16 & 0xfffffffe;
    FUN_140282280(local_4e0);
  }
  local_5e8 = local_5b0;
  local_5f0 = local_5c0;
  local_5f8 = local_5a8;
  local_608 = uVar8;
  local_600 = uVar9;
  local_5e0 = plVar15;
  iVar6 = FUN_140711d70(DAT_143aa9d98,*(undefined4 *)(param_1 + 0x20),
                        *(undefined4 *)(param_1 + 0x24),(ulonglong)local_5c8 & 0xffffffff);
  if ((iVar6 != 0) && ((*(int *)(param_1 + 0x28) == 1 || (iVar6 == 0xc)))) {
    iVar7 = FUN_140715460(DAT_143aa9d98,*(undefined4 *)(param_1 + 0x20));
    if (iVar7 != 0) {
      FUN_1406ed520(local_488,0x151);
      FUN_1406ed840(local_488,6);
      FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x20));
      FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x24));
      FUN_1406ed940(local_488,local_5d0[0]);
      FUN_1406ed940(local_488,local_5cc);
      FUN_1415d01c0(local_488);
      FUN_1406ed610(local_488);
      return;
    }
    FUN_141f18c20(param_1,iVar6);
    return;
  }
  if ((*(int *)(param_1 + 0x28) == 1) &&
     (iVar6 = FUN_1407158a0(DAT_143aa9d98,*(undefined4 *)(param_1 + 0x20)), iVar6 != 0)) {
    FUN_1406ed520(local_488,0x151);
    FUN_1406ed840(local_488,2);
    FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x20));
    FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x24));
    FUN_1406ed9d0(local_488,0xffffffff);
    FUN_1415d01c0(local_488);
    FUN_1406ed610(local_488);
    return;
  }
  if ((*(int *)(param_1 + 0x28) == 0) &&
     (iVar6 = FUN_140715180(DAT_143aa9d98,*(undefined4 *)(param_1 + 0x20)), iVar6 != 0)) {
    FUN_1406ed520(local_488,0x151);
    FUN_1406ed840(local_488,4);
    FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x20));
    FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x24));
    FUN_1406ed940(local_488,local_5d0[0]);
    FUN_1406ed940(local_488,local_5cc);
    FUN_1415d01c0(local_488);
    FUN_1406ed610(local_488);
    return;
  }
  if ((*(int *)(param_1 + 0x28) == 1) &&
     (iVar6 = FUN_1407152f0(DAT_143aa9d98,*(undefined4 *)(param_1 + 0x20)), iVar6 != 0)) {
    FUN_1406ed520(local_488,0x151);
    FUN_1406ed840(local_488,5);
    FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x20));
    FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x24));
    FUN_1406ed940(local_488,local_5d0[0]);
    FUN_1406ed940(local_488,local_5cc);
    FUN_1415d01c0(local_488);
    FUN_1406ed610(local_488);
    return;
  }
  local_5c8 = (longlong *)0x0;
  plVar15 = (longlong *)0x0;
  uVar5 = 0;
  if ((*(int *)(param_1 + 0x28) == 0) &&
     (iVar6 = FUN_140715800(DAT_143aa9d98,*(undefined4 *)(param_1 + 0x20)), plVar11 = plVar12,
     iVar6 != 0)) {
LAB_141f0ec6f:
    if (*(int *)(param_1 + 0x28) == 1) {
      iVar6 = FUN_141f15fb0(param_1,&local_5c8);
      if (iVar6 == 0x7fffffff) goto LAB_141f0ee00;
      FUN_1406ed520(local_488,0x151);
      FUN_1406ed840(local_488,2);
      FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x20));
      FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x24));
      iVar7 = FUN_1407155d0(DAT_143aa9d98,*(undefined4 *)(param_1 + 0x20));
      if (iVar7 == 0) {
        FUN_1406ed940(local_488,local_5d0[0]);
        FUN_1406ed940(local_488,local_5cc);
      }
      FUN_1406ed9d0(local_488,iVar6);
      FUN_1415d01c0(local_488);
    }
    else {
      FUN_1406ed520(local_488,0x151);
      FUN_1406ed840(local_488,1);
      FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x20));
      FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x24));
      iVar6 = FUN_1407155d0(DAT_143aa9d98,*(undefined4 *)(param_1 + 0x20));
      if (iVar6 == 0) {
        FUN_1406ed940(local_488,local_5d0[0]);
        FUN_1406ed940(local_488,local_5cc);
      }
      FUN_1406ed9d0(local_488,plVar11);
      FUN_1415d01c0(local_488);
    }
    FUN_1406ed610(local_488);
  }
  else {
    local_5a0 = 0;
    FUN_140734b40(&local_5b0,*(undefined4 *)(param_1 + 0x20),&local_5a0);
    pIVar2 = (IUnknown *)CONCAT44(uStack_5ac,local_5b0);
    if (pIVar2 == (IUnknown *)0x0) {
      local_5a8 = (longlong *)0x0;
      pplVar10 = &local_5a8;
      uVar16 = uVar16 | 0x10;
    }
    else {
      local_520 = 0;
      uStack_518 = 0;
      local_510 = 0;
      uStack_508 = 0;
      local_500 = 0;
      uStack_4f8 = 0;
      local_4f0 = 0;
      uStack_4e8 = 0;
      FUN_142f121cc(*(undefined4 *)(param_1 + 0x28),&local_520,10);
      local_4c8 = local_520;
      uStack_4c0 = uStack_518;
      local_4b8 = local_510;
      uStack_4b0 = uStack_508;
      local_4a8 = (undefined4)local_500;
      uStack_4a4 = local_500._4_4_;
      uStack_4a0 = (undefined4)uStack_4f8;
      uStack_49c = uStack_4f8._4_4_;
      local_498 = (undefined4)local_4f0;
      uStack_494 = local_4f0._4_4_;
      uStack_490 = (undefined4)uStack_4e8;
      uStack_48c = uStack_4e8._4_4_;
      FUN_1401bb8d0(&local_5c0,&local_4c8);
      local_5d8 = &local_5c0;
      (*DAT_143262a20)(&local_550);
      plVar11 = plVar12;
      if ((undefined8 *)CONCAT44(uStack_5bc,local_5c0) != (undefined8 *)0x0) {
        plVar11 = *(longlong **)CONCAT44(uStack_5bc,local_5c0);
      }
      iVar6 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,plVar11,&local_550);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar2,(_GUID *)&DAT_143272478);
      }
      local_588 = local_550;
      uStack_584 = uStack_54c;
      uStack_580 = uStack_548;
      uStack_57c = uStack_544;
      local_578 = local_540;
      local_550 = local_550 & 0xffff0000;
      FUN_1401be120(&local_5c0);
      local_5d8 = (undefined4 *)(CONCAT44(local_5d8._4_4_,uVar16) | 0x24);
      pplVar10 = (longlong **)FUN_1409339d0(&local_598,&local_588);
      uVar16 = uVar16 | 0x2c;
    }
    plVar3 = local_5a8;
    local_5d8 = (undefined4 *)CONCAT44(local_5d8._4_4_,uVar16);
    FUN_1401a5040(&local_558,pplVar10);
    plVar11 = *(longlong **)(param_1 + 0x30);
    if (plVar11 != local_558) {
      *(longlong **)(param_1 + 0x30) = local_558;
      local_558 = plVar12;
      if (plVar11 != (longlong *)0x0) {
        (**(code **)(*plVar11 + 0x10))();
        local_558 = (longlong *)0x0;
      }
    }
    if (local_558 != (longlong *)0x0) {
      (**(code **)(*local_558 + 0x10))(local_558);
    }
    uVar17 = uVar16;
    if ((uVar16 & 0x10) != 0) {
      uVar17 = uVar16 & 0xffffffef;
      local_5d8 = (undefined4 *)(CONCAT44(local_5d8._4_4_,uVar16) & 0xffffffffffffffef);
      if (plVar3 != (longlong *)0x0) {
        (**(code **)(*plVar3 + 0x10))(plVar3);
      }
    }
    uVar16 = uVar17;
    if ((uVar17 & 8) != 0) {
      uVar16 = uVar17 & 0xfffffff7;
      local_5d8 = (undefined4 *)(CONCAT44(local_5d8._4_4_,uVar17) & 0xfffffffffffffff7);
      if (local_598 != (longlong *)0x0) {
        (**(code **)(*local_598 + 0x10))();
      }
    }
    if ((uVar16 & 4) != 0) {
      if ((short)local_588 == 8) {
        local_588 = local_588 & 0xffff0000;
        if (CONCAT44(uStack_57c,uStack_580) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_57c,uStack_580) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_588);
      }
    }
    pIVar2 = *(IUnknown **)(param_1 + 0x30);
    if (pIVar2 == (IUnknown *)0x0) {
      if (local_5b8 != 0) goto LAB_141f0ec5e;
    }
    else {
      if (*(int *)(param_1 + 0x20) - 40000U < 1000) {
        uVar14 = 1;
      }
      else {
        uVar14 = uVar5;
        if (*(int *)(param_1 + 0x20) - 0x7563U < 0x1d) {
          uVar14 = 1;
          uVar5 = 1;
        }
      }
      FUN_1401bb8d0(&local_5b8,PTR_DAT_143a488f8);
      local_598 = (longlong *)&local_5b8;
      (*DAT_143262a20)(&local_538);
      if ((undefined8 *)CONCAT44(uStack_5b4,local_5b8) != (undefined8 *)0x0) {
        plVar12 = *(longlong **)CONCAT44(uStack_5b4,local_5b8);
      }
      iVar6 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,plVar12,&local_538);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar2,(_GUID *)&DAT_143272478);
      }
      local_570 = local_538;
      uStack_56c = uStack_534;
      uStack_568 = uStack_530;
      uStack_564 = uStack_52c;
      local_560 = local_528;
      local_538 = local_538 & 0xffff0000;
      FUN_1401be120(&local_5b8);
      iVar6 = FUN_14022ee40(&local_570,0);
      if ((short)local_570 == 8) {
        local_570 = local_570 & 0xffff0000;
        if (CONCAT44(uStack_564,uStack_568) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_564,uStack_568) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_570);
      }
      if (iVar6 == 0) {
        uVar16 = FUN_141f13360();
        plVar12 = local_5c8;
      }
      else {
        uVar16 = FUN_141f146a0(param_1,&local_5c8,uVar14,uVar5);
        plVar12 = local_5c8;
      }
      plVar15 = (longlong *)(ulonglong)uVar16;
      local_5c8 = plVar12;
      if (-1 < (int)uVar16) {
LAB_141f0ec5e:
        plVar11 = plVar15;
        if ((longlong *)CONCAT44(uStack_5ac,local_5b0) != (longlong *)0x0) {
          (**(code **)(*(longlong *)CONCAT44(uStack_5ac,local_5b0) + 0x10))();
        }
        goto LAB_141f0ec6f;
      }
    }
    if ((longlong *)CONCAT44(uStack_5ac,local_5b0) != (longlong *)0x0) {
      (**(code **)(*(longlong *)CONCAT44(uStack_5ac,local_5b0) + 0x10))();
    }
  }
LAB_141f0ee00:
  if (plVar12 != (longlong *)0x0) {
    plVar15 = plVar12 + -1;
    plVar11 = plVar12 + *plVar15 * 3;
    for (; plVar12 < plVar11; plVar12 = plVar12 + 3) {
      FUN_141f1bb60(plVar12 + 1);
      if (*plVar12 != 0) {
        FUN_14019f2c0(*plVar12 + -0x10);
      }
    }
    thunk_FUN_140205820(plVar15,0);
  }
  return;
}


