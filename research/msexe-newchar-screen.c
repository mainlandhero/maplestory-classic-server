
//===========================================================
// FUN_141b2d290 @ 141b2d290   (507 bytes)
//===========================================================

void FUN_141b2d290(longlong param_1,int param_2,longlong param_3)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  IUnknown *pIVar5;
  int iVar6;
  int *piVar7;
  wchar_t *local_res8;
  
  if ((*(int *)(param_1 + 0xd0) <= DAT_143a886e0) || (*(int *)(param_1 + 0x238) != 0)) {
    return;
  }
  if ((DAT_143aa84a0 != 0) && (iVar6 = FUN_142ce3050(), iVar6 != 0)) {
    return;
  }
  if (param_2 != 0) {
    local_res8 = (wchar_t *)0x0;
    piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar7[1] = 0x14;
    local_res8 = (wchar_t *)(piVar7 + 4);
    *piVar7 = -1;
    piVar7[2] = 0;
    *local_res8 = L'\0';
    uVar1 = u_confirmReturnToLogin_1433fea40._8_8_;
    *(undefined8 *)local_res8 = u_confirmReturnToLogin_1433fea40._0_8_;
    *(undefined8 *)(piVar7 + 6) = uVar1;
    uVar4 = u_confirmReturnToLogin_1433fea40._28_4_;
    uVar3 = u_confirmReturnToLogin_1433fea40._24_4_;
    uVar2 = u_confirmReturnToLogin_1433fea40._20_4_;
    piVar7[8] = u_confirmReturnToLogin_1433fea40._16_4_;
    piVar7[9] = uVar2;
    piVar7[10] = uVar3;
    piVar7[0xb] = uVar4;
    *(undefined8 *)(piVar7 + 0xc) = u_confirmReturnToLogin_1433fea40._32_8_;
    if (*piVar7 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar7[1] < 0x14) {
      FUN_142e54290(0x90,piVar7[1],0x14);
    }
    *piVar7 = 1;
    local_res8[0x14] = L'\0';
    if (piVar7[1] + 1 < 0x15) {
      FUN_142e54290(0x9c,0x14);
    }
    piVar7[2] = 0x28;
    iVar6 = FUN_141b4a290(&local_res8,param_1 + 0x140);
    if (iVar6 == 0) {
      if (param_3 == 0) {
        return;
      }
      if (DAT_143abfdf8 == 0) {
        return;
      }
      FUN_142c0bf50(DAT_143abfdf8,param_3,0);
      return;
    }
  }
  pIVar5 = DAT_143add050;
  if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  iVar6 = (**(code **)(*(longlong *)DAT_143add050 + 0x80))(DAT_143add050);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar5,(_GUID *)&DAT_14327fcd0);
  }
  FUN_142c48ca0(DAT_143ac1898,0);
  if (DAT_143a886e0 == 1) {
    FUN_140f5ecb0();
    FUN_140f676b0();
  }
  else if (DAT_143a886e0 == 2) {
    FUN_141b45280(param_1 + 0x100);
  }
  *(undefined1 *)(param_1 + 0xdc) = 0xff;
  FUN_141b3f050(param_1,DAT_143a886e0,800);
  return;
}



//===========================================================
// FUN_141b256e0 @ 141b256e0   (840 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */

void FUN_141b256e0(longlong param_1)

{
  int iVar1;
  undefined8 *puVar2;
  char cVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined1 local_28 [8];
  undefined1 local_20 [32];
  
  uVar4 = FUN_1415e3d00();
  lVar5 = FUN_142c4eea0(uVar4);
  cVar3 = FUN_140d21310(lVar5);
  if (cVar3 != '\0') {
    FUN_140d21280(lVar5,2);
  }
  iVar1 = *(int *)(param_1 + 0xd0);
  if (iVar1 == 1) {
    cVar3 = FUN_141b44830();
    if ((cVar3 != '\0') && (cVar3 = FUN_140d21340(lVar5), cVar3 != '\0')) {
      uVar4 = FUN_1401bb5c0(local_28,lVar5 + 0x28);
      uVar6 = FUN_1401bb5c0(local_20,lVar5 + 0x18);
      FUN_141b2a660(param_1,uVar6,uVar4);
      FUN_140d21280(lVar5,3);
      if (*(longlong *)(lVar5 + 0x58) != 0) {
        puVar2 = *(undefined8 **)(lVar5 + 0x58);
        if (puVar2 != (undefined8 *)0x0) {
          (**(code **)*puVar2)(puVar2,1);
        }
        *(undefined8 *)(lVar5 + 0x58) = 0;
      }
    }
  }
  else if (iVar1 == 2) {
    if ((*(int *)(param_1 + 0xd4) == 0) && (cVar3 = FUN_141b3faf0(param_1), cVar3 == '\0')) {
      uVar4 = FUN_1415e3d00();
      cVar3 = FUN_142c4ee50(uVar4);
      if (cVar3 != '\0') {
        cVar3 = FUN_140d21370(lVar5);
        if (cVar3 == '\0') {
          cVar3 = FUN_140d213a0(lVar5);
          if (cVar3 != '\0') {
            FUN_141b2ba60(param_1,*(undefined4 *)(lVar5 + 0x30),*(undefined4 *)(lVar5 + 0x34),0,0);
            FUN_140d21280(lVar5,5);
          }
        }
        else {
          FUN_141b2b930(param_1,*(undefined4 *)(lVar5 + 0x30));
          FUN_140d21280(lVar5,4);
        }
      }
    }
  }
  else if (iVar1 == 3) {
    cVar3 = FUN_141b3faf0(param_1);
    if ((cVar3 == '\0') && (cVar3 = FUN_140d213d0(lVar5), cVar3 != '\0')) {
      FUN_141b3f050(param_1,4,600);
    }
  }
  else if ((iVar1 == 4) && (cVar3 = FUN_141b3faf0(param_1), cVar3 == '\0')) {
    uVar4 = FUN_1415e3d00();
    cVar3 = FUN_142c4ee80(uVar4);
    if ((cVar3 != '\0') && (cVar3 = FUN_140d213d0(lVar5), cVar3 != '\0')) {
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
  }
  return;
}



//===========================================================
// FUN_141b39490 @ 141b39490   (1081 bytes)
//===========================================================

void FUN_141b39490(longlong param_1,undefined8 param_2)

{
  undefined1 uVar1;
  undefined1 uVar2;
  char cVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  longlong *plVar7;
  longlong lVar8;
  longlong local_res8;
  longlong *local_res18;
  longlong *local_res20;
  longlong local_58 [2];
  longlong *local_48;
  
  if (DAT_143aca790 != 0) {
    FUN_141179940();
  }
  uVar1 = FUN_1406e8ae0(param_2);
  switch(uVar1) {
  case 0:
    FUN_1406e8ae0(param_2);
    FUN_141b3f050(param_1,5,0x14a);
    break;
  case 0x14:
    local_res18 = &local_res8;
    local_res8 = 0;
    uVar4 = FUN_1403edf80(local_58,L"incorrectPIC",0xffffffffffffffff);
    FUN_141b4ac80(uVar4,&local_res8,param_1 + 0x140);
    break;
  case 0x39:
    local_res18 = &local_res8;
    local_res8 = 0;
    uVar4 = FUN_1403edf80(local_58,L"incorrectPICWarningOverCount",0xffffffffffffffff);
    FUN_141b4ac80(uVar4,&local_res8,param_1 + 0x140);
    break;
  case 0x3a:
    local_res18 = &local_res8;
    local_res8 = 0;
    uVar4 = FUN_1403edf80(local_58,L"incorrectPICCloseByOverCount",0xffffffffffffffff);
    FUN_141b4ac80(uVar4,&local_res8,param_1 + 0x140);
    FUN_141b2d290(param_1,0,0);
    break;
  case 0x45:
    cVar3 = FUN_1406e8ae0(param_2);
    FUN_1406e8ae0(param_2);
    uVar1 = FUN_1406e8ae0(param_2);
    uVar2 = FUN_1406e8ae0(param_2);
    if (cVar3 != '\0') {
      local_res18 = &local_res8;
      local_res8 = 0;
      uVar4 = FUN_1403edf80(local_58,L"antimacroTextMismatch",0xffffffff);
      FUN_141b4ac80(uVar4,&local_res8,param_1 + 0x140);
    }
    FUN_141029040(&local_res20,param_2);
    local_res18 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x340);
    lVar5 = 0;
    if (local_res18 != (longlong *)0x0) {
      lVar5 = FUN_14130a540(local_res18,param_1,0x3c);
    }
    lVar8 = lVar5 + 0x18;
    if (lVar5 == 0) {
      lVar8 = 0;
    }
    if (lVar8 == 0) {
      local_48 = (longlong *)0x0;
    }
    else {
      local_48 = (longlong *)(lVar8 + -0x18);
      if (local_48 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar8 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar8 + 8) = *(longlong *)(lVar8 + 8) + 1;
        UNLOCK();
      }
    }
    plVar7 = local_48;
    if (local_48 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_res18 = local_res20;
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 8))();
    }
    FUN_14130cbc0(plVar7,&local_res18,uVar1,uVar2);
    if (plVar7 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar7 + 0x130))(plVar7);
    if (0xffffe < plVar7[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar7 = plVar7 + 4;
    lVar5 = *plVar7;
    *plVar7 = *plVar7 + -1;
    UNLOCK();
    if (((int)lVar5 == 1) && (plVar7 = local_48 + 3, plVar7 != (longlong *)0x0)) {
      (**(code **)*plVar7)(plVar7,1);
    }
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 0x10))();
    }
    break;
  case 0x46:
    local_res18 = &local_res8;
    local_res8 = 0;
    uVar4 = FUN_1403edf80(local_58,L"cannotProcessRequest",0xffffffffffffffff);
    FUN_141b4ac80(uVar4,&local_res8,param_1 + 0x140);
    break;
  case 0x47:
    cVar3 = FUN_1406e8ae0(param_2);
    local_res8 = 0;
    local_res18 = &local_res8;
    if (cVar3 == '\0') {
      uVar4 = FUN_1403edf80(local_58,L"antimacroTextFailTooMuch",0xffffffffffffffff);
      FUN_141b4ac80(uVar4,&local_res8,param_1 + 0x140);
      local_res8 = 0;
      puVar6 = (undefined8 *)FUN_1408a9e40(local_58,3);
      FUN_14019ba10(&local_res8,*puVar6);
      if (local_58[0] != 0) {
        FUN_14019f2c0(local_58[0] + -0x10);
      }
      lVar5 = local_res8;
      FUN_1429e4fa0(local_res8,0,0);
      *(undefined4 *)(param_1 + 0xf0) = 1;
      if (lVar5 != 0) {
        FUN_14019f2c0(lVar5 + -0x10);
      }
    }
    else {
      uVar4 = FUN_1403edf80(local_58,L"antimacroTextMismatch",0xffffffffffffffff);
      FUN_141b4ac80(uVar4,&local_res8,param_1 + 0x140);
      FUN_141b2d290(param_1,0,0);
    }
    break;
  case 0x48:
    local_res18 = &local_res8;
    local_res8 = 0;
    uVar4 = FUN_1403edf80(local_58,L"maxCharCountTomorrow",0xffffffffffffffff);
    FUN_141b4ac80(uVar4,&local_res8,param_1 + 0x140);
  }
  *(undefined4 *)(param_1 + 0xd4) = 0;
  return;
}


