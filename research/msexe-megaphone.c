
//===========================================================
// FUN_142db9320 @ 142db9320   (218 bytes)
//===========================================================

void FUN_142db9320(undefined8 param_1,undefined8 *param_2)

{
  longlong *plVar1;
  int *piVar2;
  int iVar3;
  longlong lVar4;
  longlong *plVar5;
  longlong *local_18;
  longlong *local_10;
  
  local_10 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x58);
  if (local_10 == (longlong *)0x0) {
    local_10 = (longlong *)0x0;
  }
  else {
    *local_10 = 0;
    local_10[1] = 0;
    *(undefined4 *)(local_10 + 1) = 1;
    *(undefined4 *)((longlong)local_10 + 0xc) = 1;
    *local_10 = (longlong)&PTR_FUN_143300d78;
    FUN_1408d6690(local_10 + 2);
  }
  local_18 = local_10 + 2;
  FUN_142dbe0d0(param_2,&local_18);
  plVar5 = local_10;
  if (local_10 != (longlong *)0x0) {
    LOCK();
    plVar1 = local_10 + 1;
    lVar4 = *plVar1;
    *(int *)plVar1 = (int)*plVar1 + -1;
    UNLOCK();
    if ((int)lVar4 == 1) {
      (**(code **)*local_10)(local_10);
      LOCK();
      piVar2 = (int *)((longlong)plVar5 + 0xc);
      iVar3 = *piVar2;
      *piVar2 = *piVar2 + -1;
      UNLOCK();
      if (iVar3 == 1) {
        (**(code **)(*local_10 + 8))();
      }
    }
  }
  FUN_1408d6760(*param_2,param_1);
  return;
}



//===========================================================
// FUN_1408da090 @ 1408da090   (366 bytes)
//===========================================================

void FUN_1408da090(int *param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong lVar2;
  char cVar3;
  int *piVar4;
  undefined8 *puVar5;
  int local_res8;
  undefined4 uStackX_c;
  undefined1 local_28 [8];
  undefined8 *local_20;
  
  FUN_1406e9170(param_2,&local_res8,4);
  *param_1 = local_res8;
  if (local_res8 == 1) {
    cVar3 = FUN_1406e8ae0(param_2);
    if (cVar3 != '\0') {
      piVar4 = (int *)FUN_140303530(local_28,param_2);
      if ((*(longlong *)(param_1 + 4) - 1U < 999) || (*(longlong *)(param_1 + 4) == -1)) {
        FUN_142e52ed0(0x447);
      }
      if (param_1 + 2 == piVar4) {
        FUN_142e52d50(0x45c,1);
      }
      lVar2 = *(longlong *)(piVar4 + 2);
      if (lVar2 != 0) {
        if (0xfffff < *(ulonglong *)(lVar2 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar2 + 8) = *(longlong *)(lVar2 + 8) + 1;
        UNLOCK();
      }
      FUN_1401abd80(param_1 + 2);
      puVar5 = local_20;
      *(undefined8 *)(param_1 + 4) = *(undefined8 *)(piVar4 + 2);
      if (local_20 != (undefined8 *)0x0) {
        if (0xffffe < local_20[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = puVar5 + 1;
        lVar2 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if (((int)lVar2 == 1) && (local_20 != (undefined8 *)0x0)) {
          (**(code **)*local_20)(local_20,1);
        }
      }
      puVar5 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
      if (*(longlong *)(param_1 + 6) != 0) {
        FUN_14019f2c0();
        param_1[6] = 0;
        param_1[7] = 0;
      }
      *(undefined8 *)(param_1 + 6) = *puVar5;
      *puVar5 = 0;
      if (CONCAT44(uStackX_c,local_res8) != 0) {
        FUN_14019f2c0(CONCAT44(uStackX_c,local_res8) + -0x10);
      }
    }
  }
  return;
}



//===========================================================
// FUN_140417eb0 @ 140417eb0   (13 bytes)
//===========================================================

undefined4 FUN_140417eb0(int param_1)

{
  return CONCAT31((int3)(param_1 - 0x4d7484U >> 8),param_1 - 0x4d7484U < 100);
}



//===========================================================
// FUN_141a50140 @ 141a50140   (1196 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

int FUN_141a50140(longlong param_1)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  longlong lVar4;
  char *pcVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  longlong *plVar9;
  longlong lVar10;
  int iVar11;
  int iVar12;
  char *pcVar13;
  bool bVar14;
  undefined1 auStack_4e8 [32];
  undefined4 local_4c8;
  undefined4 local_4c0;
  undefined4 local_4b8;
  undefined4 local_4b0;
  undefined4 local_4a8;
  undefined4 local_4a0;
  undefined4 local_498;
  char *local_488;
  longlong local_480;
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4e8;
  lVar4 = *(longlong *)(param_1 + 0x1408);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar4 = *(longlong *)(param_1 + 0x1408);
  }
  iVar11 = 0;
  iVar12 = 0;
  iVar2 = iVar11;
  if (*(longlong *)(lVar4 + 0xc0) != 0) {
    iVar2 = *(int *)(*(longlong *)(lVar4 + 0xc0) + -8);
  }
  local_488 = (char *)0x0;
  lVar10 = 0xc0;
  if (iVar2 < 1) {
    lVar10 = 200;
  }
  FUN_14019a260(&local_488,lVar10 + lVar4);
  if (((local_488 != (char *)0x0) && (*local_488 != '\0')) &&
     (lVar4 = FUN_142ef83e0(&DAT_143273878,(int)local_488[(longlong)*(int *)(local_488 + -8) + -1]),
     lVar4 != 0)) {
    pcVar5 = (char *)FUN_14019bd40(&local_488,0,1);
    iVar2 = iVar12;
    if (local_488 != (char *)0x0) {
      iVar2 = *(int *)(local_488 + -8);
    }
    for (pcVar13 = pcVar5 + (longlong)iVar2 + -2; pcVar5 <= pcVar13; pcVar13 = pcVar13 + -1) {
      lVar4 = FUN_142ef83e0(&DAT_143273878,(int)*pcVar13);
      if (lVar4 == 0) {
        if (pcVar5 <= pcVar13) {
          pcVar13[1] = '\0';
          FUN_14019c870(&local_488,(int)(pcVar13 + 1) - (int)pcVar5);
          goto LAB_141a502cd;
        }
        break;
      }
    }
    pcVar5 = local_488;
    if (*(int *)(local_488 + -0x10) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (*(int *)(pcVar5 + -0xc) < 0) {
      FUN_142e54290(0x90,*(int *)(pcVar5 + -0xc),0);
    }
    pcVar5[-0x10] = '\x01';
    pcVar5[-0xf] = '\0';
    pcVar5[-0xe] = '\0';
    pcVar5[-0xd] = '\0';
    *local_488 = '\0';
    if (*(int *)(pcVar5 + -0xc) + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    pcVar5[-8] = '\0';
    pcVar5[-7] = '\0';
    pcVar5[-6] = '\0';
    pcVar5[-5] = '\0';
    if (local_488 != (char *)0x0) {
      FUN_14019f2c0(local_488 + -0x10);
      local_488 = (char *)0x0;
    }
  }
LAB_141a502cd:
  puVar6 = (undefined8 *)FUN_1401d12a0(&local_488,0);
  if (((char *)*puVar6 == (char *)0x0) || (*(char *)*puVar6 == '\0')) {
    iVar12 = 1;
  }
  if (local_488 != (char *)0x0) {
    FUN_14019f2c0(local_488 + -0x10);
  }
  if (iVar12 != 0) {
    return 0;
  }
  lVar4 = *(longlong *)(param_1 + 0x1408);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar4 = *(longlong *)(param_1 + 0x1408);
  }
  cVar1 = FUN_1416aae20(lVar4);
  if (cVar1 == '\0') {
    uVar7 = FUN_1408a9e40(&local_488,2999);
    local_498 = 0;
    local_4a0 = 0;
    local_4a8 = 3;
    local_4b0 = 0;
    local_4b8 = 0;
    local_4c0 = 0xffffffff;
    local_4c8 = 0;
    iVar2 = FUN_142a269c0(uVar7,0,0,1);
    if (iVar2 == 7) {
      return 0;
    }
  }
  lVar4 = DAT_143aa84a0;
  if (DAT_143aa84a0 == 0) {
    return 0;
  }
  iVar2 = FUN_142cc42d0(DAT_143aa84a0,500,0);
  if (iVar2 == 0) {
    uVar7 = FUN_1408a9e40(&local_488,0xf8);
    local_4a0 = 0;
    local_4a8 = 0;
    local_4b0 = 0;
    local_4b8 = 0;
    local_4c0 = 0;
    local_4c8 = 0;
    FUN_142a26280(uVar7,0,0,1);
    return 0;
  }
  FUN_1406ed520(local_478,0x116);
  uVar3 = FUN_1429e3ef0();
  FUN_1406ed9d0(local_478,uVar3);
  FUN_1406ed940(local_478,*(undefined2 *)(param_1 + 0x29c));
  FUN_1406ed9d0(local_478,*(undefined4 *)(param_1 + 0x2a0));
  lVar10 = *(longlong *)(param_1 + 0x1408);
  if (lVar10 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar10 = *(longlong *)(param_1 + 0x1408);
  }
  uVar7 = FUN_1416aae10(lVar10);
  lVar10 = *(longlong *)(param_1 + 0x1408);
  if (lVar10 == 0) {
    FUN_142e52ed0(0x431);
    lVar10 = *(longlong *)(param_1 + 0x1408);
  }
  plVar9 = (longlong *)(lVar10 + 0xc0);
  if ((*plVar9 == 0) || (*(int *)(*plVar9 + -8) < 1)) {
    plVar9 = (longlong *)(lVar10 + 200);
  }
  local_480 = 0;
  FUN_14019a260(&local_480,plVar9);
  iVar2 = FUN_1408beba0(DAT_143ac2f58,&local_480);
  if (iVar2 != 0) {
    uVar7 = FUN_1408a9e40(&local_488,0xb5);
    local_4a0 = 0;
    local_4a8 = 0;
    local_4b0 = 0;
    local_4b8 = 0;
    local_4c0 = 0;
    local_4c8 = 0;
    FUN_142a26280(uVar7,0,0,1);
    goto LAB_141a505c7;
  }
  bVar14 = false;
  if (*(longlong *)(param_1 + 0x13e8) == 0) {
    if (*(longlong *)(param_1 + 0x13f8) != 0) {
      iVar2 = FUN_1416899e0();
      goto LAB_141a5056a;
    }
  }
  else {
    uVar8 = FUN_141060430(param_1 + 0x13e0);
    iVar2 = FUN_141690380(uVar8);
LAB_141a5056a:
    bVar14 = iVar2 != 0;
  }
  FUN_1406edc80(local_478,&local_480);
  FUN_1406ed840(local_478,bVar14);
  FUN_1408d9f50(uVar7,local_478);
  FUN_1415d01c0(local_478);
  FUN_142cc4430(lVar4,1);
  FUN_1429f14c0(PTR_u_UseShopItem_143a48488,100);
  iVar11 = 1;
LAB_141a505c7:
  if (local_480 != 0) {
    FUN_14019f2c0(local_480 + -0x10);
  }
  FUN_1406ed610(local_478);
  return iVar11;
}


