
//===========================================================
// FUN_140d734e0 @ 140d734e0   (617 bytes)
//===========================================================

void FUN_140d734e0(longlong param_1,int param_2,undefined8 param_3)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  longlong *plVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong local_28;
  longlong lStack_20;
  longlong local_18;
  
  if (param_2 == 0x5ad) {
    lVar10 = param_1 + -0x18;
    iVar2 = FUN_1406e8c20(param_3);
    iVar3 = FUN_1406e8c20(param_3);
    iVar4 = FUN_1406e8c20(param_3);
    if (((iVar2 < 0) || (iVar3 < 0)) || (iVar4 < 0)) {
      FUN_140d7c7f0(lVar10,2);
      FUN_140d73bf0(lVar10);
      return;
    }
    FUN_140d85bc0(lVar10,iVar2);
    lVar9 = DAT_143aa8360;
    if (*(longlong *)(param_1 + 0x50) != 0) {
      lVar9 = *(longlong *)(param_1 + 0x50);
    }
    lVar6 = 0;
    do {
      lVar7 = lVar6 + 1;
      if (*(char *)(lVar9 + lVar6) != (&DAT_14336df78)[lVar6]) goto LAB_140d736ae;
      lVar6 = lVar7;
    } while (lVar7 != 4);
    FUN_140d85bc0(lVar10,0);
LAB_140d736ae:
    FUN_140d85b70(lVar10,iVar3);
    if (*(longlong *)(param_1 + 0x90) != 0) {
      plVar5 = (longlong *)FUN_140d83c20(param_1 + 0x88);
      (**(code **)(*plVar5 + 0x90))(plVar5,0);
    }
    *(undefined1 *)(param_1 + 0x5c) = 0;
    if (*(int *)(param_1 + 0x108) == 1) {
      *(undefined4 *)(param_1 + 0x108) = 0;
      FUN_140d785f0(lVar10,0);
    }
    else if (*(int *)(param_1 + 0x108) == 6) {
      *(undefined4 *)(param_1 + 0x108) = 0;
      FUN_140d74a70(lVar10);
    }
    if (*(int *)(param_1 + 0x58) == 0) {
      return;
    }
    *(undefined4 *)(param_1 + 0x58) = 0;
    return;
  }
  if (param_2 == 0x5ae) {
    FUN_140d7dca0(param_1 + -0x18,param_3);
    return;
  }
  if (param_2 != 0x5b9) {
    if (param_2 != 0x5ba) {
      return;
    }
    iVar2 = FUN_1401c3050(param_3);
    if (iVar2 < 1) {
      return;
    }
    lVar10 = *(longlong *)(param_1 + 0xa0);
    if (lVar10 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar10 = *(longlong *)(param_1 + 0xa0);
    }
    FUN_1410db190(lVar10,iVar2);
    return;
  }
  cVar1 = FUN_1406e8ae0(param_3);
  if (cVar1 == '\0') {
    FUN_1401c2910(param_3,0);
    return;
  }
  local_28 = 0;
  lStack_20 = 0;
  local_18 = 0;
  FUN_1401c2df0(param_3,&local_28);
  if (local_28 == lStack_20) {
    if (local_28 == 0) {
      return;
    }
    uVar8 = local_18 - local_28 & 0xfffffffffffffffc;
    if (uVar8 < 0x1000) goto LAB_140d735d8;
    lVar10 = *(longlong *)(local_28 + -8);
  }
  else {
    if (local_28 == 0) {
      return;
    }
    uVar8 = local_18 - local_28 & 0xfffffffffffffffc;
    if (uVar8 < 0x1000) goto LAB_140d735d8;
    lVar10 = *(longlong *)(local_28 + -8);
  }
  if (0x1f < (local_28 - lVar10) - 8U) {
                    /* WARNING: Subroutine does not return */
    FUN_142f04804(lVar10,uVar8 + 0x27);
  }
LAB_140d735d8:
  thunk_FUN_140205820();
  return;
}



//===========================================================
// FUN_141072ec0 @ 141072ec0   (912 bytes)
//===========================================================

void FUN_141072ec0(longlong param_1,undefined4 param_2,undefined8 param_3)

{
  char cVar1;
  undefined1 uVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  longlong lVar6;
  longlong *plVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  char *local_res20;
  char *local_18;
  longlong local_10;
  
  uVar8 = DAT_143aa84a0;
  switch(param_2) {
  case 0x5ac:
    *(undefined4 *)(param_1 + 0x50) = 0;
    FUN_1406e9050(param_3,&local_res20);
    FUN_1429e5410(local_res20,0,0);
    local_18 = local_res20;
    goto LAB_1410730fc;
  case 0x5ad:
    FUN_141073ce0(param_1 + -0x18,param_3);
    break;
  case 0x5ae:
    FUN_141073f80(param_1 + -0x18,param_3);
    break;
  case 0x5af:
    uVar2 = FUN_1406e8ae0(param_3);
    FUN_142ce32e0(uVar8,uVar2);
    break;
  case 0x5b0:
    FUN_141045f00(param_1 + -0x18,param_3);
    break;
  case 0x5b1:
    FUN_14104a890(param_1 + -0x18,param_3);
    break;
  case 0x5b2:
    FUN_14107dfc0(param_1 + -0x18);
    break;
  case 0x5b3:
    lVar6 = FUN_141041990(param_1 + -0x18,&local_18,1);
    plVar7 = *(longlong **)(lVar6 + 8);
    if (plVar7 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar7 = *(longlong **)(lVar6 + 8);
    }
    (**(code **)(*plVar7 + 0x90))(plVar7,0);
    lVar6 = local_10;
    if (local_10 != 0) {
      if (0xffffe < *(longlong *)(local_10 + 0x20) - 1U) {
        FUN_142e541f0();
      }
      LOCK();
      plVar7 = (longlong *)(lVar6 + 0x20);
      lVar6 = *plVar7;
      *plVar7 = *plVar7 + -1;
      UNLOCK();
      if ((int)lVar6 == 1) {
        puVar9 = (undefined8 *)(local_10 + 0x18);
        if (local_10 == 0) {
          puVar9 = (undefined8 *)0x0;
        }
        if (puVar9 != (undefined8 *)0x0) {
          (**(code **)*puVar9)(puVar9,1);
        }
      }
    }
    break;
  case 0x5b4:
    FUN_14107d5c0(param_1 + -0x18,param_3);
    break;
  case 0x5b5:
    FUN_1406e9050(param_3,&local_18);
    uVar8 = DAT_143aa84a0;
    if ((local_18 != (char *)0x0) && (*local_18 != '\0')) {
      local_res20 = (char *)0x0;
      FUN_14019a260(&local_res20,&local_18);
      FUN_142cf07e0(uVar8,&local_res20);
    }
LAB_1410730fc:
    if (local_18 != (char *)0x0) {
      FUN_14019f2c0(local_18 + -0x10);
    }
    break;
  case 0x5b6:
    FUN_141073890(param_1 + -0x18,param_3);
    break;
  case 0x5b7:
    uVar2 = FUN_1406e8ae0(param_3);
    uVar3 = FUN_1406e8c20(param_3);
    FUN_141057360(param_1 + -0x18,uVar3,uVar2);
    break;
  case 0x5b8:
    FUN_1410739f0(param_1 + -0x18,param_3);
    break;
  case 0x5b9:
    FUN_14107dad0(param_1 + -0x18,param_3);
    break;
  case 0x5ba:
    iVar4 = FUN_1401c3050(param_3);
    if (0 < iVar4) {
      FUN_141041990(param_1 + -0x18,&local_18,0);
      if ((local_10 != 0) && (iVar5 = FUN_1416585e0(), iVar5 != 0)) {
        if (local_10 == 0) {
          FUN_142e52ed0(0x431,0);
        }
        FUN_1416586d0(local_10);
      }
      FUN_1410515b0(param_1 + -0x18,iVar4);
      lVar6 = local_10;
      if (local_10 != 0) {
        if (0xffffe < *(longlong *)(local_10 + 0x20) - 1U) {
          FUN_142e541f0();
        }
        LOCK();
        plVar7 = (longlong *)(lVar6 + 0x20);
        lVar6 = *plVar7;
        *plVar7 = *plVar7 + -1;
        UNLOCK();
        if ((int)lVar6 == 1) {
          puVar9 = (undefined8 *)(local_10 + 0x18);
          if (local_10 == 0) {
            puVar9 = (undefined8 *)0x0;
          }
          if (puVar9 != (undefined8 *)0x0) {
            (**(code **)*puVar9)(puVar9,1);
          }
        }
      }
    }
    break;
  case 0x5bb:
    FUN_14107dd10(param_1 + -0x18,param_3);
    break;
  case 0x5bc:
    FUN_141055960(param_1 + -0x18,param_3);
    break;
  case 0x5bf:
    cVar1 = FUN_1406e8ae0(param_3);
    if (cVar1 == '\x02') {
      uVar8 = 0x9c7;
LAB_141072fb6:
      uVar8 = FUN_1408a9e40(&local_res20,uVar8);
      FUN_142a39b00(uVar8,0,0,0);
    }
    else {
      if (cVar1 == '\x03') {
LAB_141072f8c:
        uVar2 = FUN_1406e8ae0(param_3);
        FUN_14107f5e0(param_1 + -0x18,uVar2);
        *(undefined4 *)(param_1 + 0x50) = 0;
        return;
      }
      if (cVar1 == '\x04') {
        uVar8 = 0x9c8;
        goto LAB_141072fb6;
      }
      if (cVar1 == '\x05') goto LAB_141072f8c;
    }
    *(undefined4 *)(param_1 + 0x50) = 0;
  }
  return;
}



//===========================================================
// FUN_141b82b00 @ 141b82b00   (1375 bytes)
//===========================================================

void FUN_141b82b00(longlong param_1,undefined4 param_2,undefined8 param_3)

{
  bool bVar1;
  IUnknown *pIVar2;
  char cVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  uint uVar7;
  undefined4 uVar8;
  longlong lVar9;
  ulonglong uVar10;
  uint local_res10 [4];
  char *local_res20;
  char *local_58;
  char **local_50;
  IUnknown *local_48;
  char ***local_40;
  
  switch(param_2) {
  case 0x51:
    FUN_141baa2c0(param_1 + -0x18,param_3);
    break;
  case 0x5c:
    (**(code **)(*(longlong *)(param_1 + -0x18) + 0x58))((longlong *)(param_1 + -0x18),param_3);
    break;
  case 0x5d:
    if ((DAT_143aa84a0 == 0) || (cVar3 = FUN_142d04b50(), cVar3 == '\0')) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    uVar7 = FUN_1406e8c20(param_3);
    if (0 < (int)uVar7) {
      uVar10 = (ulonglong)uVar7;
      do {
        FUN_1406e9050(param_3,&local_58);
        cVar3 = FUN_1406e8ae0(param_3);
        uVar8 = FUN_1406e8c20(param_3);
        local_res10[0] = FUN_1406e8c20(param_3);
        if ((local_res10[0] == 0) || (bVar1)) {
          FUN_141bad0b0(param_1 + -0x18,&local_58,cVar3 != '\0',1,uVar8);
        }
        else {
          local_res20 = (char *)0x0;
          FUN_14019a260(&local_res20,&local_58);
          FUN_141bc5080(param_1 + -0x18,&local_res20,local_res10[0],cVar3 != '\0',uVar8,1);
        }
        if (local_58 != (char *)0x0) {
          FUN_14019f2c0(local_58 + -0x10);
        }
        uVar10 = uVar10 - 1;
      } while (uVar10 != 0);
    }
    break;
  case 0x5e:
    uVar7 = FUN_1406e8c20(param_3);
    if (0 < (int)uVar7) {
      uVar10 = (ulonglong)uVar7;
      do {
        FUN_1406e9050(param_3,&local_res20);
        uVar8 = FUN_1406e8c20(param_3);
        if ((local_res20 != (char *)0x0) && (*local_res20 != '\0')) {
          FUN_141bc3b50(param_1 + -0x18,&local_res20,uVar8);
        }
        if (local_res20 != (char *)0x0) {
          FUN_14019f2c0(local_res20 + -0x10);
        }
        uVar10 = uVar10 - 1;
      } while (uVar10 != 0);
    }
    break;
  case 0x60:
    FUN_1406e9050(param_3,&local_res20);
    uVar8 = FUN_1406e8c20(param_3);
    if ((local_res20 != (char *)0x0) && (*local_res20 != '\0')) {
      FUN_141bc3840(param_1 + -0x18,&local_res20,uVar8);
    }
    if (local_res20 == (char *)0x0) {
      return;
    }
    local_58 = local_res20 + -0x10;
    goto LAB_141b82f33;
  case 0x61:
    FUN_1406e9050(param_3,&local_58);
    iVar4 = FUN_1406e8c20(param_3);
    local_res10[0] = (uint)(iVar4 != 0);
    if ((local_58 != (char *)0x0) && (*local_58 != '\0')) {
      local_res20 = (char *)0x0;
      FUN_14019a260(&local_res20,&local_58);
      local_50 = &local_res20;
      local_48 = (IUnknown *)0x0;
      lVar9 = FUN_1418af970(*(longlong *)(param_1 + 0x50) + 0x80,&local_res20,&local_48);
      pIVar2 = local_48;
      if ((lVar9 == 0) || (local_48 == (IUnknown *)0x0)) {
        if (local_48 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_48 + 0x10))(local_48);
        }
      }
      else {
        iVar4 = (**(code **)(*(longlong *)local_48 + 0x2b8))(local_48,iVar4 != 0);
        if (iVar4 < 0) {
          _com_issue_errorex(iVar4,pIVar2,(_GUID *)&DAT_14327fcb0);
        }
        (**(code **)(*(longlong *)pIVar2 + 0x10))(pIVar2);
      }
      if (local_res20 != (char *)0x0) {
        FUN_14019f2c0(local_res20 + -0x10);
      }
      FUN_141bdc620(*(longlong *)(param_1 + 0x50) + 0x158,&local_58,local_res10);
    }
    if (local_58 == (char *)0x0) {
      return;
    }
    local_58 = local_58 + -0x10;
    goto LAB_141b82f33;
  case 0x62:
    FUN_1406e9050(param_3,&local_58);
    uVar8 = FUN_1406e8c20(param_3);
    uVar6 = FUN_1406e8c20(param_3);
    uVar5 = FUN_1406e8c20(param_3);
    if ((local_58 != (char *)0x0) && (*local_58 != '\0')) {
      local_res20 = (char *)0x0;
      FUN_14019a260(&local_res20,&local_58);
      FUN_141bc43e0(param_1 + -0x18,&local_res20,uVar8,uVar6,uVar5);
    }
    if (local_58 == (char *)0x0) {
      return;
    }
    local_58 = local_58 + -0x10;
    goto LAB_141b82f33;
  case 99:
    FUN_1406e9050(param_3,&local_58);
    FUN_1406e9050(param_3,&local_res20);
    uVar8 = FUN_1406e8c20(param_3);
    uVar6 = FUN_1406e8c20(param_3);
    if ((((local_58 != (char *)0x0) && (*local_58 != '\0')) && (local_res20 != (char *)0x0)) &&
       (*local_res20 != '\0')) {
      local_40 = &local_50;
      local_50 = (char **)0x0;
      FUN_14019a260(&local_50,&local_res20);
      local_48 = (IUnknown *)0x0;
      FUN_14019a260(&local_48,&local_58);
      FUN_141bc4a60(param_1 + -0x18,&local_48,&local_50,uVar8,uVar6);
    }
    if (local_res20 != (char *)0x0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
    if (local_58 == (char *)0x0) {
      return;
    }
    local_58 = local_58 + -0x10;
LAB_141b82f33:
    FUN_14019f2c0(local_58);
    break;
  case 100:
    FUN_141bab920(param_1 + -0x18,param_3);
    break;
  case 0x65:
    FUN_141babe00(param_1 + -0x18,param_3);
    break;
  case 0x66:
    FUN_141bac240(param_1 + -0x18,param_3);
    break;
  case 0x67:
    FUN_141bac490(param_1 + -0x18,param_3);
    break;
  case 0x68:
    FUN_141bac5f0(param_1 + -0x18,param_3);
    break;
  case 0x69:
    FUN_141bac870(param_1 + -0x18,param_3);
    break;
  case 0x6a:
    FUN_141bacaf0(param_1 + -0x18,param_3);
    break;
  case 0x6b:
    FUN_141bace50(param_1 + -0x18,param_3);
    break;
  case 0x6c:
    uVar7 = FUN_1406e8c20(param_3);
    if (0 < (int)uVar7) {
      uVar10 = (ulonglong)uVar7;
      do {
        FUN_1406e9050(param_3,&local_res20);
        uVar8 = FUN_1406e8c20(param_3);
        if ((local_res20 != (char *)0x0) && (*local_res20 != '\0')) {
          FUN_141bc5430(param_1 + -0x18,&local_res20,uVar8);
        }
        if (local_res20 != (char *)0x0) {
          FUN_14019f2c0(local_res20 + -0x10);
        }
        uVar10 = uVar10 - 1;
      } while (uVar10 != 0);
    }
    break;
  case 0x6d:
    FUN_141bc5ae0(param_1 + -0x18,param_3);
    break;
  case 0x6e:
    FUN_141bc6610(param_1 + -0x18,param_3);
    break;
  case 0x6f:
    FUN_141ba5f20(param_1 + -0x18,0);
  }
  return;
}



//===========================================================
// FUN_141df5940 @ 141df5940   (1418 bytes)
//===========================================================

void FUN_141df5940(longlong *param_1,longlong param_2)

{
  uint uVar1;
  
  uVar1 = *(uint *)(param_2 + 0x10);
  if (uVar1 < 0x8002) {
    if (uVar1 == 0x8001) {
      (**(code **)(*param_1 + 0x150))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    switch(uVar1) {
    case 0x1001:
      (**(code **)(*param_1 + 8))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1002:
      (**(code **)(*param_1 + 0x20))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1003:
      (**(code **)(*param_1 + 0x28))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1004:
      (**(code **)(*param_1 + 0x30))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1005:
      (**(code **)(*param_1 + 0x38))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1006:
      (**(code **)(*param_1 + 0x40))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1007:
      (**(code **)(*param_1 + 0x48))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1008:
      (**(code **)(*param_1 + 0x50))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1009:
      (**(code **)(*param_1 + 0x58))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x100a:
      (**(code **)(*param_1 + 0x60))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x100b:
      (**(code **)(*param_1 + 0x68))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x100c:
      (**(code **)(*param_1 + 0x70))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1010:
      (**(code **)(*param_1 + 0x88))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1011:
      (**(code **)(*param_1 + 0x90))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1012:
      (**(code **)(*param_1 + 0x78))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1013:
      (**(code **)(*param_1 + 0x80))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1014:
      (**(code **)(*param_1 + 0x98))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1016:
      (**(code **)(*param_1 + 0xa0))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1017:
      (**(code **)(*param_1 + 0x10))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x1018:
      (**(code **)(*param_1 + 0x18))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
  }
  else if (uVar1 < 0x9004) {
    if (uVar1 == 0x9003) {
      (**(code **)(*param_1 + 200))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    switch(uVar1) {
    case 0x8002:
      (**(code **)(*param_1 + 0x158))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x8003:
      (**(code **)(*param_1 + 0x160))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x8004:
      (**(code **)(*param_1 + 0x168))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x8005:
      (**(code **)(*param_1 + 0x170))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x8008:
      (**(code **)(*param_1 + 0x178))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x8009:
      (**(code **)(*param_1 + 0x180))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x800c:
      (**(code **)(*param_1 + 0x188))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0x800d:
      (**(code **)(*param_1 + 400))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
  }
  else if (uVar1 < 0xa002) {
    if (uVar1 == 0xa001) {
      (**(code **)(*param_1 + 0xe8))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    if (uVar1 == 0x9004) {
      (**(code **)(*param_1 + 0xd0))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    if (uVar1 == 0x9006) {
      (**(code **)(*param_1 + 0xe0))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    if (uVar1 == 0x9007) {
      (**(code **)(*param_1 + 0xb8))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    if (uVar1 == 0x9008) {
      (**(code **)(*param_1 + 0xc0))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    if (uVar1 == 0x9009) {
      (**(code **)(*param_1 + 0xd8))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
  }
  else if (uVar1 < 0xa102) {
    if (uVar1 == 0xa101) {
      (**(code **)(*param_1 + 0x128))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    switch(uVar1) {
    case 0xa002:
      (**(code **)(*param_1 + 0xf0))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0xa003:
      (**(code **)(*param_1 + 0xf8))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0xa004:
      (**(code **)(*param_1 + 0x100))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0xa005:
      (**(code **)(*param_1 + 0x108))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0xa006:
      (**(code **)(*param_1 + 0x110))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0xa007:
      (**(code **)(*param_1 + 0x118))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    case 0xa008:
      (**(code **)(*param_1 + 0x120))(param_1,param_2);
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
  }
  else if (uVar1 < 0xa403) {
    if (uVar1 == 0xa402) {
      (**(code **)(*param_1 + 0x148))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    if (uVar1 == 0xa201) {
      (**(code **)(*param_1 + 0x130))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    if (uVar1 == 0xa301) {
      (**(code **)(*param_1 + 0x138))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
    if (uVar1 == 0xa401) {
      (**(code **)(*param_1 + 0x140))();
      *(undefined4 *)(param_2 + 0x14) = 1;
      return;
    }
  }
  else if (uVar1 == 0xb001) {
    (**(code **)(*param_1 + 0xa8))();
    *(undefined4 *)(param_2 + 0x14) = 1;
  }
  else if (uVar1 == 0xd001) {
    (**(code **)(*param_1 + 0xb0))();
    *(undefined4 *)(param_2 + 0x14) = 1;
    return;
  }
  return;
}



//===========================================================
// FUN_142097ee0 @ 142097ee0   (72 bytes)
//===========================================================

void FUN_142097ee0(longlong param_1,int param_2,undefined8 param_3)

{
  if (param_2 == 0x1a0) {
    FUN_142097f80(param_1 + -0x18,param_3);
    return;
  }
  if (param_2 == 0x1a1) {
    FUN_14209b070(param_1 + -0x18,param_3);
    return;
  }
  if (param_2 == 0x1a2) {
    FUN_14209b380(param_1 + -0x18,param_3);
    return;
  }
  if (param_2 == 0x1a3) {
    FUN_14209ad60(param_1 + -0x18,param_3);
    return;
  }
  return;
}


