
//===========================================================
// FUN_142cb7e50 @ 142cb7e50   (9 bytes)
//===========================================================

void FUN_142cb7e50(longlong param_1)

{
  FUN_14019a260(param_1 + 0x48);
  return;
}



//===========================================================
// FUN_142cb8400 @ 142cb8400   (7 bytes)
//===========================================================

void FUN_142cb8400(longlong param_1,undefined4 param_2)

{
  *(undefined4 *)(param_1 + 0x22b8) = param_2;
  return;
}



//===========================================================
// FUN_142ce9780 @ 142ce9780   (7 bytes)
//===========================================================

void FUN_142ce9780(longlong param_1,undefined4 param_2)

{
  *(undefined4 *)(param_1 + 0x28e0) = param_2;
  return;
}



//===========================================================
// FUN_142cb6330 @ 142cb6330   (37 bytes)
//===========================================================

void FUN_142cb6330(longlong param_1,undefined1 *param_2,int param_3)

{
  longlong lVar1;
  
  lVar1 = (longlong)param_3;
  if (0 < param_3) {
    param_1 = param_1 - (longlong)param_2;
    do {
      param_2[param_1 + 0x2324] = *param_2;
      param_2 = param_2 + 1;
      lVar1 = lVar1 + -1;
    } while (lVar1 != 0);
  }
  return;
}



//===========================================================
// FUN_142cb8370 @ 142cb8370   (30 bytes)
//===========================================================

void FUN_142cb8370(longlong param_1)

{
  FUN_14019a260(param_1 + 0x22f8);
  FUN_141128960(4);
  return;
}



//===========================================================
// FUN_142cb83a0 @ 142cb83a0   (40 bytes)
//===========================================================

undefined8 * FUN_142cb83a0(longlong param_1,undefined8 *param_2)

{
  *param_2 = 0;
  FUN_14019a260(param_2,param_1 + 0x22f8);
  return param_2;
}



//===========================================================
// FUN_14112a720 @ 14112a720   (548 bytes)
//===========================================================

void FUN_14112a720(longlong param_1,undefined8 param_2)

{
  IUnknown *pIVar1;
  longlong *plVar2;
  char cVar3;
  int iVar4;
  longlong lVar5;
  undefined8 uVar6;
  char *local_res18;
  IUnknown *local_res20;
  longlong *local_78;
  IUnknown *local_70;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  uint local_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 local_38;
  
  lVar5 = FUN_14112b010();
  if (lVar5 == 0) {
    return;
  }
  uVar6 = FUN_14112b010();
  FUN_142bf7e40(param_1,param_2);
  lVar5 = DAT_143aa84a0;
  if (DAT_143aa84a0 == 0) {
    return;
  }
  FUN_1429fbeb0(&local_78,0);
  FUN_142cb83a0(lVar5,&local_res18);
  if ((local_res18 != (char *)0x0) && (*local_res18 != '\0')) {
    pIVar1 = *(IUnknown **)(param_1 + 600);
    if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    FUN_1404a7700(&local_68,&DAT_143a8b8d8);
    local_res20 = (IUnknown *)0x0;
    local_48 = local_68;
    uStack_44 = uStack_64;
    uStack_40 = uStack_60;
    uStack_3c = uStack_5c;
    local_38 = local_58;
    iVar4 = (**(code **)(*(longlong *)pIVar1 + 0x240))(pIVar1,&local_48,&local_res20);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    pIVar1 = local_res20;
    local_70 = local_res20;
    if ((short)local_68 == 8) {
      local_68 = local_68 & 0xffff0000;
      if (CONCAT44(uStack_5c,uStack_60) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_5c,uStack_60) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_68);
    }
    if (pIVar1 == (IUnknown *)0x0) goto LAB_14112a900;
    iVar4 = (**(code **)(*(longlong *)pIVar1 + 0x1b0))(pIVar1);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar1,(_GUID *)&DAT_14327ac98);
    }
    plVar2 = local_78;
    local_res20 = pIVar1;
    (**(code **)(*(longlong *)pIVar1 + 8))(pIVar1);
    FUN_142a0ff80(&local_res20,0,0,&local_res18,plVar2,0,0xff);
    (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
  }
  cVar3 = FUN_141b2a160(uVar6);
  if ((((cVar3 != '\0') && (FUN_142aa2010(param_1 + 0x240,L"login",1), DAT_143aa84a0 != 0)) &&
      (cVar3 = FUN_142cf42c0(), cVar3 != '\0')) && (*(char *)(param_1 + 0x26c) == '\0')) {
    *(undefined1 *)(param_1 + 0x26c) = 1;
    cVar3 = FUN_141b3fd10(uVar6);
    if (cVar3 != '\0') {
      FUN_141b3ff10(uVar6);
    }
  }
LAB_14112a900:
  if (local_res18 != (char *)0x0) {
    FUN_14019f2c0(local_res18 + -0x10);
  }
  if (local_78 != (longlong *)0x0) {
    (**(code **)(*local_78 + 0x10))();
  }
  return;
}


