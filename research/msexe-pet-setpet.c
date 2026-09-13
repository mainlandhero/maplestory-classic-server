
//===========================================================
// FUN_1427707e0 @ 1427707e0   (259 bytes)
//===========================================================

void FUN_1427707e0(longlong param_1,int param_2,longlong param_3)

{
  undefined8 uVar1;
  longlong lVar2;
  longlong lVar3;
  ulonglong uVar4;
  bool bVar5;
  
  if (param_2 == 0) {
    if (param_3 == 0) {
      lVar2 = FUN_142867730(param_1 + 0x11d8);
      lVar3 = FUN_142770440(param_1);
      bVar5 = *(longlong *)(lVar2 + 8) == lVar3;
      uVar1 = FUN_142867730(param_1 + 0x11d8,0);
      FUN_14286e2b0(uVar1,0);
    }
    else {
      FUN_141ec2e50(param_3);
      FUN_141ecde00(param_3);
      uVar1 = FUN_142867730(param_1 + 0x11d8,0);
      FUN_14286e2b0(uVar1,param_3);
      lVar2 = FUN_142770440(param_1);
      bVar5 = param_3 == lVar2;
    }
    if (((bVar5) && (lVar2 = *(longlong *)(param_1 + 0x11d8), lVar2 != 0)) &&
       (*(uint *)(lVar2 + -8) != 0)) {
      for (uVar4 = lVar2 + -0x10 + (ulonglong)*(uint *)(lVar2 + -8) * 0x10; uVar4 != 0;
          uVar4 = uVar4 - 0x10) {
        if (*(longlong *)(uVar4 + 8) != 0) {
          FUN_141ec6630();
        }
        if (uVar4 <= *(ulonglong *)(param_1 + 0x11d8)) break;
      }
    }
    if (DAT_143acda08 != 0) {
      FUN_1414beaa0();
    }
  }
  return;
}



//===========================================================
// FUN_141ecde00 @ 141ecde00   (483 bytes)
//===========================================================

void FUN_141ecde00(longlong *param_1)

{
  IUnknown *pIVar1;
  bool bVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  longlong lVar6;
  undefined8 uVar7;
  int iVar8;
  int local_res8 [2];
  
  iVar5 = 0;
  cVar3 = FUN_1409d6150(param_1 + 0xc6);
  iVar8 = iVar5;
  if ((cVar3 == '\0') && (param_1[0x24] != 0)) {
    lVar6 = param_1[0x23] + -0x20;
    if (param_1[0x23] == 0) {
      lVar6 = 0;
    }
    if (lVar6 != 0) {
      cVar3 = FUN_142826340();
      if (cVar3 == '\0') {
        cVar3 = FUN_140f80830(param_1[0x24] + 0x100);
        if (cVar3 == '\0') {
          cVar3 = FUN_140f80860(param_1[0x24] + 0x100);
          if (cVar3 == '\0') {
            iVar4 = (**(code **)(*(longlong *)param_1[0x24] + 0x50))();
            if (iVar4 == 0) {
              cVar3 = FUN_142d0f360(DAT_143aa84a0);
              if (cVar3 != '\0') goto LAB_141ecdf15;
            }
            cVar3 = FUN_1409bd2f0(lVar6);
            if (cVar3 == '\0') {
              lVar6 = FUN_141892840();
              if (lVar6 == 0) {
LAB_141ecdeeb:
                bVar2 = false;
              }
              else {
                uVar7 = FUN_141892840();
                cVar3 = FUN_14183a640(uVar7);
                if (cVar3 == '\0') goto LAB_141ecdeeb;
                bVar2 = true;
              }
              iVar4 = (**(code **)(*(longlong *)param_1[0x24] + 0x50))();
              if ((iVar4 != 0) || (!bVar2)) {
                iVar4 = FUN_142cc1e40(DAT_143aa84a0);
                if (iVar4 == 0) {
                  iVar8 = 1;
                }
              }
            }
          }
        }
      }
    }
  }
LAB_141ecdf15:
  pIVar1 = (IUnknown *)param_1[0x79];
  if (pIVar1 != (IUnknown *)0x0) {
    local_res8[0] = 0;
    iVar4 = (**(code **)(*(longlong *)pIVar1 + 0x2b0))(pIVar1,local_res8);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    iVar4 = 1;
    if (local_res8[0] != 0) goto LAB_141ecdf56;
  }
  iVar4 = iVar5;
LAB_141ecdf56:
  if (iVar4 != iVar8) {
    pIVar1 = (IUnknown *)param_1[0x79];
    if (pIVar1 != (IUnknown *)0x0) {
      iVar5 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1,iVar8);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar1,(_GUID *)&DAT_14327fcb0);
      }
    }
    FUN_14159b0a0(param_1[8],iVar8);
    (**(code **)(*param_1 + 0x18))(param_1,iVar8,0);
    FUN_141ecaf00(param_1);
    if (iVar8 == 0) {
      if (param_1[0x20] != 0) {
        FUN_140dd7fc0(DAT_143abfdf0,param_1[0x20],0);
      }
      param_1[0x20] = 0;
    }
  }
  return;
}



//===========================================================
// FUN_141ebdad0 @ 141ebdad0   (135 bytes)
//===========================================================

undefined8 FUN_141ebdad0(longlong param_1)

{
  int iVar1;
  undefined4 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  
  if (*(longlong **)(param_1 + 0x120) != (longlong *)0x0) {
    iVar1 = (**(code **)(**(longlong **)(param_1 + 0x120) + 0x50))();
    if (iVar1 != 0) {
      lVar3 = FUN_142cbe730(DAT_143aa84a0);
      if (lVar3 != 0) {
        uVar4 = FUN_142cbe730(DAT_143aa84a0);
        uVar2 = FUN_140230cb0(uVar4,5,*(undefined8 *)(param_1 + 0x150));
        lVar3 = FUN_1402e3e30(uVar4,5,uVar2);
        if (lVar3 != 0) {
          uVar4 = FUN_140192f80(lVar3);
          return uVar4;
        }
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_141ed3540 @ 141ed3540   (457 bytes)
//===========================================================

longlong FUN_141ed3540(int param_1)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined1 local_18 [8];
  longlong local_10;
  
  local_10 = 0;
  if (DAT_143a88bf8 != 0) {
    for (lVar3 = *(longlong *)
                  (DAT_143a88bf8 + ((ulonglong)(longlong)param_1 % (ulonglong)DAT_143a88c00) * 8);
        lVar3 != 0; lVar3 = *(longlong *)(lVar3 + 8)) {
      if (*(int *)(lVar3 + 0x10) == param_1) {
        if (local_18 == (undefined1 *)(lVar3 + 0x18)) {
          FUN_142e52d50(0x45c);
        }
        lVar2 = *(longlong *)(lVar3 + 0x20);
        if (lVar2 != 0) {
          if (0xfffff < *(ulonglong *)(lVar2 + -0x20)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar2 + -0x20) = *(longlong *)(lVar2 + -0x20) + 1;
          UNLOCK();
        }
        lVar3 = *(longlong *)(lVar3 + 0x20);
        local_10 = lVar3;
        if (lVar3 != 0) goto LAB_141ed3678;
        break;
      }
    }
  }
  lVar3 = local_10;
  if ((DAT_143ad3164 != '\0') && (FUN_141ed8f70(param_1), DAT_143a88bf8 != 0)) {
    for (lVar2 = *(longlong *)
                  (DAT_143a88bf8 + ((ulonglong)(longlong)param_1 % (ulonglong)DAT_143a88c00) * 8);
        lVar2 != 0; lVar2 = *(longlong *)(lVar2 + 8)) {
      if (*(int *)(lVar2 + 0x10) == param_1) {
        if (local_18 == (undefined1 *)(lVar2 + 0x18)) {
          FUN_142e52d50(0x45c,CONCAT71((int7)((ulonglong)local_18 >> 8),1));
        }
        lVar3 = *(longlong *)(lVar2 + 0x20);
        if (lVar3 != 0) {
          if (0xfffff < *(ulonglong *)(lVar3 + -0x20)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar3 + -0x20) = *(longlong *)(lVar3 + -0x20) + 1;
          UNLOCK();
        }
        lVar3 = *(longlong *)(lVar2 + 0x20);
        local_10 = lVar3;
        break;
      }
    }
  }
LAB_141ed3678:
  if (lVar3 == 0) {
    lVar2 = 0;
  }
  else {
    puVar4 = (undefined8 *)(lVar3 + -0x28);
    if (0xffffe < *(longlong *)(lVar3 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    lVar2 = local_10;
    LOCK();
    plVar1 = (longlong *)(lVar3 + -0x20);
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      if (*(longlong *)(local_10 + -0x10) != 0) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_10 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_10 + -0x10) + 4) != 0);
      }
      if (puVar4 != (undefined8 *)0x0) {
        (**(code **)*puVar4)(puVar4,1);
      }
    }
  }
  return lVar2;
}


