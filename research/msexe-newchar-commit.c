
//===========================================================
// FUN_141b3fb10 @ 141b3fb10   (33 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */

void FUN_141b3fb10(longlong param_1)

{
  if (((*(int *)(param_1 + 0x238) == 0) && (*(int *)(param_1 + 0xd0) == 5)) && (DAT_143aca358 != 0))
  {
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  return;
}



//===========================================================
// FUN_141b3fd40 @ 141b3fd40   (457 bytes)
//===========================================================

void FUN_141b3fd40(longlong param_1,char param_2)

{
  undefined1 *puVar1;
  int iVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  undefined1 local_18 [8];
  longlong local_10;
  
  plVar6 = *(longlong **)(param_1 + 0x270);
  if (plVar6 != (longlong *)0x0) {
    (**(code **)(*plVar6 + 0x138))(plVar6,3);
  }
  if (param_2 != '\0') {
    lVar3 = FUN_14019b780(&DAT_143ad68a0,0x2c8);
    lVar4 = 0;
    if (lVar3 != 0) {
      lVar4 = FUN_1416087d0(lVar3,param_1,0,0,1);
    }
    lVar3 = lVar4 + 0x18;
    if (lVar4 == 0) {
      lVar3 = 0;
    }
    if (lVar3 == 0) {
      local_10 = 0;
    }
    else {
      local_10 = lVar3 + -0x18;
      if (local_10 != 0) {
        if (0xfffff < *(ulonglong *)(lVar3 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar3 + 8) = *(longlong *)(lVar3 + 8) + 1;
        UNLOCK();
      }
    }
    lVar4 = local_10;
    puVar1 = (undefined1 *)(param_1 + 0x268);
    if ((*(longlong *)(param_1 + 0x270) - 1U < 999) || (*(longlong *)(param_1 + 0x270) == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (puVar1 == local_18) {
      FUN_142e52d50(0x45c,1);
    }
    if (lVar4 != 0) {
      if (0xfffff < *(ulonglong *)(lVar4 + 0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar4 + 0x20) = *(longlong *)(lVar4 + 0x20) + 1;
      UNLOCK();
      lVar4 = local_10;
    }
    FUN_14117cd70(puVar1);
    *(longlong *)(param_1 + 0x270) = lVar4;
    if (lVar4 != 0) {
      if (0xffffe < *(longlong *)(lVar4 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar6 = (longlong *)(lVar4 + 0x20);
      lVar4 = *plVar6;
      *plVar6 = *plVar6 + -1;
      UNLOCK();
      if (((int)lVar4 == 1) &&
         (puVar5 = (undefined8 *)(local_10 + 0x18), puVar5 != (undefined8 *)0x0)) {
        (**(code **)*puVar5)(puVar5,1);
      }
    }
    plVar6 = *(longlong **)(param_1 + 0x270);
    if (plVar6 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar6 = *(longlong **)(param_1 + 0x270);
    }
    iVar2 = (**(code **)(*plVar6 + 0x130))();
    if ((iVar2 == 3) && (*(longlong *)(param_1 + 0x270) != 0)) {
      FUN_142bf3f70();
      FUN_14117cd70(puVar1);
    }
  }
  return;
}



//===========================================================
// FUN_141b3df70 @ 141b3df70   (140 bytes)
//===========================================================

void FUN_141b3df70(longlong param_1,uint param_2,int param_3)

{
  int *piVar1;
  int iVar2;
  
  if ((-1 < (int)param_2) && (param_2 < 4)) {
    if (0 < param_3) {
      iVar2 = 0;
      for (piVar1 = (int *)(param_1 + 0x220); piVar1 != (int *)(param_1 + 0x230);
          piVar1 = piVar1 + 1) {
        iVar2 = iVar2 + *piVar1;
      }
      if (0x19 - iVar2 < param_3) {
        return;
      }
    }
    param_1 = param_1 + (longlong)(int)param_2 * 4;
    param_3 = *(int *)(param_1 + 0x220) + param_3;
    if (param_3 < 4) {
      param_3 = 4;
    }
    iVar2 = 0xc;
    if (param_3 < 0xc) {
      iVar2 = param_3;
    }
    *(int *)(param_1 + 0x220) = iVar2;
    if (DAT_143aca358 != (longlong *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x000141b3dff4. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)(*DAT_143aca358 + 0x90))(DAT_143aca358,0);
      return;
    }
  }
  return;
}



//===========================================================
// FUN_141b28bf0 @ 141b28bf0   (130 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b28bf0(longlong param_1,undefined4 param_2)

{
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  if (*(int *)(param_1 + 0xd4) == 0) {
    FUN_1406ed520(local_468,0x7b);
    FUN_1415d01c0(local_468);
    *(undefined4 *)(param_1 + 0xb8) = param_2;
    *(undefined4 *)(param_1 + 0xd4) = 1;
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_141b391e0 @ 141b391e0   (629 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b391e0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int *piVar5;
  undefined1 auStack_4a8 [32];
  wchar_t *local_488;
  longlong local_480;
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  local_488 = (wchar_t *)0x0;
  piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,0x44);
  piVar5[1] = 0x19;
  *piVar5 = -1;
  local_488 = (wchar_t *)(piVar5 + 4);
  piVar5[2] = 0;
  *local_488 = L'\0';
  uVar1 = u_goToNexonAuthPageToVerify_1433fe810._8_8_;
  *(undefined8 *)local_488 = u_goToNexonAuthPageToVerify_1433fe810._0_8_;
  *(undefined8 *)(piVar5 + 6) = uVar1;
  uVar4 = u_goToNexonAuthPageToVerify_1433fe810._28_4_;
  uVar3 = u_goToNexonAuthPageToVerify_1433fe810._24_4_;
  uVar2 = u_goToNexonAuthPageToVerify_1433fe810._20_4_;
  piVar5[8] = u_goToNexonAuthPageToVerify_1433fe810._16_4_;
  piVar5[9] = uVar2;
  piVar5[10] = uVar3;
  piVar5[0xb] = uVar4;
  uVar4 = u_goToNexonAuthPageToVerify_1433fe810._44_4_;
  uVar3 = u_goToNexonAuthPageToVerify_1433fe810._40_4_;
  uVar2 = u_goToNexonAuthPageToVerify_1433fe810._36_4_;
  piVar5[0xc] = u_goToNexonAuthPageToVerify_1433fe810._32_4_;
  piVar5[0xd] = uVar2;
  piVar5[0xe] = uVar3;
  piVar5[0xf] = uVar4;
  *(wchar_t *)(piVar5 + 0x10) = u_goToNexonAuthPageToVerify_1433fe810[0x18];
  if (*piVar5 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar5[1] < 0x19) {
    FUN_142e54290(0x90,piVar5[1],0x19);
  }
  *piVar5 = 1;
  local_488[0x19] = L'\0';
  if (piVar5[1] + 1 < 0x1a) {
    FUN_142e54290(0x9c,0x19);
  }
  piVar5[2] = 0x32;
  FUN_141b4a840(&local_488,param_1 + 0x140);
  FUN_1406e9050(param_2,&local_480);
  FUN_1429e4fa0(local_480,0,0);
  local_488 = (wchar_t *)0x0;
  piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,0x34);
  piVar5[1] = 0x11;
  *piVar5 = -1;
  local_488 = (wchar_t *)(piVar5 + 4);
  piVar5[2] = 0;
  *local_488 = L'\0';
  uVar4 = u_clickOKIfVerified_1433fe848._12_4_;
  uVar3 = u_clickOKIfVerified_1433fe848._8_4_;
  uVar2 = u_clickOKIfVerified_1433fe848._4_4_;
  *(undefined4 *)local_488 = u_clickOKIfVerified_1433fe848._0_4_;
  piVar5[5] = uVar2;
  piVar5[6] = uVar3;
  piVar5[7] = uVar4;
  uVar4 = u_clickOKIfVerified_1433fe848._28_4_;
  uVar3 = u_clickOKIfVerified_1433fe848._24_4_;
  uVar2 = u_clickOKIfVerified_1433fe848._20_4_;
  piVar5[8] = u_clickOKIfVerified_1433fe848._16_4_;
  piVar5[9] = uVar2;
  piVar5[10] = uVar3;
  piVar5[0xb] = uVar4;
  *(wchar_t *)(piVar5 + 0xc) = u_clickOKIfVerified_1433fe848[0x10];
  if (*piVar5 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar5[1] < 0x11) {
    FUN_142e54290(0x90,piVar5[1],0x11);
  }
  *piVar5 = 1;
  local_488[0x11] = L'\0';
  if (piVar5[1] + 1 < 0x12) {
    FUN_142e54290(0x9c,0x11);
  }
  piVar5[2] = 0x22;
  FUN_141b4a840(&local_488,param_1 + 0x140);
  if (*(int *)(param_1 + 0xd4) == 0) {
    FUN_1406ed520(local_478,0x7b);
    FUN_1415d01c0(local_478);
    *(undefined4 *)(param_1 + 0xb8) = 6;
    *(undefined4 *)(param_1 + 0xd4) = 1;
    FUN_1406ed610(local_478);
  }
  if (local_480 != 0) {
    FUN_14019f2c0(local_480 + -0x10);
  }
  return;
}


