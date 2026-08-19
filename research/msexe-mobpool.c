
//===========================================================
// FUN_141d30e80 @ 141d30e80   (770 bytes)
//===========================================================

void FUN_141d30e80(longlong param_1,int param_2,undefined8 param_3)

{
  longlong *plVar1;
  undefined1 uVar2;
  char cVar3;
  short sVar4;
  undefined4 uVar5;
  uint uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  int iVar10;
  longlong lVar11;
  ulonglong uVar12;
  undefined8 *puVar13;
  uint local_res10 [4];
  undefined8 local_res20;
  undefined8 in_stack_ffffffffffffff98;
  
  uVar7 = (undefined4)((ulonglong)in_stack_ffffffffffffff98 >> 0x20);
  switch(param_2) {
  case 0x3c6:
    FUN_141d33630(param_1,param_3);
    break;
  default:
    if (param_2 - 0x3d9U < 0x75) {
      FUN_141d32b30(param_1);
    }
    break;
  case 0x3d1:
    FUN_141d33c70(param_1,param_3);
    break;
  case 0x3d2:
    cVar3 = FUN_1406e8ae0(param_3);
    uVar6 = FUN_1406e8c20(param_3);
    if (cVar3 == '\0') {
      if (*(longlong *)(param_1 + 0x68) != 0) {
        for (lVar11 = *(longlong *)
                       (*(longlong *)(param_1 + 0x68) +
                       ((ulonglong)uVar6 % (ulonglong)*(uint *)(param_1 + 0x70)) * 8); lVar11 != 0;
            lVar11 = *(longlong *)(lVar11 + 8)) {
          if (*(uint *)(lVar11 + 0x10) == uVar6) {
            lVar11 = *(longlong *)(lVar11 + 0x18);
            plVar1 = *(longlong **)(lVar11 + 8);
            local_res10[0] = uVar6;
            iVar10 = (**(code **)(*plVar1 + 0x48))(plVar1);
            if (iVar10 == 0) {
              return;
            }
            (**(code **)(*plVar1 + 0x40))(plVar1,0);
            iVar10 = FUN_141c543c0(plVar1);
            if (iVar10 != 0) {
              return;
            }
            if (*(longlong **)(param_1 + 0xa0) == plVar1) {
              FUN_140f08f00(param_1 + 0x98);
            }
            FUN_141d51320(param_1 + 0x38,lVar11);
            FUN_141d51670(param_1 + 0x68,local_res10);
            FUN_141d51700(param_1 + 0xa8,local_res10);
            return;
          }
        }
      }
    }
    else {
      uVar2 = FUN_1406e8ae0(param_3);
      FUN_141d34a70(param_1,cVar3,uVar6,uVar2,param_3);
    }
    break;
  case 0x3d3:
    uVar8 = FUN_1406e8c20(param_3);
    sVar4 = FUN_1406e8b80(param_3);
    uVar9 = FUN_1406e8c20(param_3);
    uVar5 = FUN_1406e8c20(param_3);
    uVar2 = FUN_1406e8ae0(param_3);
    lVar11 = FUN_141d2efc0(param_1,uVar8);
    if (lVar11 != 0) {
      FUN_141cdf1e0(lVar11,(int)sVar4,uVar5,uVar2,CONCAT44(uVar7,uVar9));
    }
    break;
  case 0x3d4:
    FUN_141d34440(param_1,param_3);
    break;
  case 0x3d6:
    uVar6 = FUN_1406e8c20(param_3);
    if (0 < (int)uVar6) {
      uVar12 = (ulonglong)uVar6;
      do {
        uVar7 = FUN_1406e8c20(param_3);
        uVar8 = FUN_1406e8c20(param_3);
        cVar3 = FUN_1406e8ae0(param_3);
        local_res20 = 0;
        if (cVar3 != '\0') {
          uVar9 = FUN_1406e8c20(param_3);
          local_res20 = CONCAT44(local_res20._4_4_,uVar9);
          uVar9 = FUN_1406e8c20(param_3);
          local_res20 = CONCAT44(uVar9,(undefined4)local_res20);
        }
        uVar9 = FUN_1406e8c20(param_3);
        lVar11 = FUN_141d2efc0(param_1,uVar7);
        if (lVar11 != 0) {
          puVar13 = &local_res20;
          if (cVar3 == '\0') {
            puVar13 = (undefined8 *)0x0;
          }
          FUN_141c99730(lVar11,uVar8,puVar13,uVar9);
        }
        uVar12 = uVar12 - 1;
      } while (uVar12 != 0);
    }
    break;
  case 0x3d7:
    iVar10 = FUN_1406e8c20(param_3);
    cVar3 = FUN_1406e8ae0(param_3);
    if (iVar10 == 0) {
      lVar11 = FUN_141892840();
      if (lVar11 != 0) {
        FUN_1418300d0(lVar11,cVar3 != '\0');
      }
    }
    else {
      lVar11 = FUN_141d2efc0(param_1,iVar10);
      if (lVar11 != 0) {
        *(bool *)(lVar11 + 0x1100) = cVar3 != '\0';
      }
    }
    break;
  case 0x3d8:
    FUN_141d34830(param_1,param_3);
  }
  return;
}


