
//===========================================================
// FUN_141b25f30 @ 141b25f30   (1706 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b25f30(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  byte bVar2;
  undefined4 uVar3;
  int iVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  undefined8 uVar8;
  uint uVar9;
  longlong lVar10;
  undefined1 auStack_4e8 [32];
  undefined4 local_4c8;
  undefined4 local_4c0;
  undefined4 local_4b8;
  undefined4 local_4b0;
  undefined4 local_4a8;
  undefined4 local_4a0;
  undefined8 local_498;
  longlong local_490;
  undefined1 local_488 [12];
  undefined1 local_47c [203];
  undefined1 local_3b1 [36];
  longlong local_38d;
  undefined1 local_352 [794];
  longlong local_38;
  ulonglong local_30;
  
  local_30 = DAT_143a8b908 ^ (ulonglong)auStack_4e8;
  iVar4 = (int)param_2;
  if (iVar4 < 0x5f5) {
    if (iVar4 == 0x5f4) {
      FUN_141b39490(param_1 + -0x18,param_3);
      return;
    }
    switch(iVar4) {
    case 0:
      FUN_141b2dd00(param_1 + -0x18,param_3);
      break;
    default:
      goto switchD_141b25f9b_caseD_1;
    case 0xb:
      FUN_141b2fac0(param_1 + -0x18,param_3);
      break;
    case 0xc:
      if (*(int *)(param_1 + 0xb8) == 2) {
        iVar4 = FUN_1406e8c20(param_3);
        if (iVar4 == 0xfd) {
          iVar4 = *(int *)(param_1 + 0x16c);
        }
        else {
          *(int *)(param_1 + 0x16c) = iVar4;
        }
        if (((iVar4 != 0xfe) && (iVar4 != 0xff)) && (DAT_143ad2230 != 0)) {
          FUN_141b65b10();
        }
      }
      break;
    case 0xd:
      FUN_1406e9170(param_3,&local_38,8);
      FUN_142cb6330(DAT_143aa84a0,&local_38,8);
      break;
    case 0xe:
      uVar3 = FUN_1406e8c20(param_3);
      FUN_142e17ed0(uVar3);
      uVar3 = FUN_1406e8c20(param_3);
      FUN_142cb8f60(DAT_143aa84a0,uVar3);
      if (DAT_143ad2101 == '\0') {
        DAT_143ad2101 = '\x01';
        FUN_1406ed520(local_488,0x96);
        FUN_142e571b0(local_488);
        FUN_142e56810(local_488);
        FUN_142e5c2e0(local_488);
        FUN_1415d01c0(local_488);
        FUN_142e13d90();
        FUN_1406ed610(local_488);
      }
      break;
    case 0xf:
      FUN_141b30230(param_1 + -0x18,param_3);
      break;
    case 0x10:
      FUN_141b307b0(param_1 + -0x18,param_3);
      break;
    case 0x11:
      FUN_141b36f60(param_1 + -0x18,param_3);
      break;
    case 0x12:
      FUN_141b2ee90(param_1 + -0x18,param_3);
      break;
    case 0x13:
      FUN_141b2f5f0(param_1 + -0x18,param_3);
      break;
    case 0x14:
      FUN_141b33f30(param_1 + -0x18,param_3);
      break;
    case 0x15:
      FUN_141b36a10(param_1 + -0x18,param_3);
      break;
    case 0x16:
      FUN_141b34970(param_1 + -0x18,param_3);
      break;
    case 0x17:
      FUN_141b359e0(param_1 + -0x18,param_3);
      break;
    case 0x18:
      FUN_141b365f0(param_1 + -0x18,param_3);
      break;
    case 0x23:
      break;
    case 0x25:
      FUN_141b38240(param_1 + -0x18,param_3);
      break;
    case 0x26:
      FUN_141b38d90(param_1 + -0x18,param_3);
      break;
    case 0x27:
      FUN_141b391e0(param_1 + -0x18,param_3);
      break;
    case 0x29:
      cVar1 = FUN_1406e8ae0(param_3);
      FUN_142cb6150(DAT_143aa84a0,cVar1 != '\0');
      break;
    case 0x2b:
      uVar6 = FUN_142c4eea0(DAT_143ac1898);
      FUN_140d21280(uVar6,2);
      break;
    case 0x34:
      bVar2 = FUN_1406e8ae0(param_3);
      *(uint *)(param_1 + 0x168) = (uint)bVar2;
      if (DAT_143aca790 != (longlong *)0x0) {
        FUN_141179940();
      }
      break;
    case 0x35:
      FUN_141b28cc0(param_1 + -0x18,param_3);
      break;
    case 0x36:
      *(undefined4 *)(param_1 + 0xbc) = 0;
      *(undefined4 *)(param_1 + 0x168) = 3;
      local_498 = 0;
      FUN_14019a260(&local_498,param_1 + 0x178);
      FUN_141b2cd50(param_1 + -0x18,&local_498,0);
      break;
    case 0x37:
      FUN_141b3a180(param_1 + -0x18,param_3);
      break;
    case 0x38:
      FUN_141b3a2f0(param_1 + -0x18,param_3);
      break;
    case 0x39:
      cVar1 = FUN_1406e8ae0(param_3);
      *(undefined4 *)(param_1 + 0xbc) = 0;
      if (cVar1 != '\0') {
        *(undefined4 *)(param_1 + 0xd4) = 0xbba;
        FUN_141b3a560(param_1 + -0x18);
      }
      break;
    case 0x45:
      bVar2 = FUN_1406e8ae0(param_3);
      uVar9 = 0;
      lVar5 = *(longlong *)(param_1 + 0xe8);
      lVar10 = 0;
      while( true ) {
        if ((lVar5 == 0) || (*(uint *)(lVar5 + -8) <= uVar9)) goto LAB_141b26354;
        if ((int)uVar9 < 0) {
          FUN_142e54290(0xbc,uVar9);
          lVar5 = *(longlong *)(param_1 + 0xe8);
        }
        if (*(uint *)(lVar5 + lVar10) == (uint)bVar2) break;
        uVar9 = uVar9 + 1;
        lVar10 = lVar10 + 0x68;
      }
      FUN_1408e4210((uint *)(lVar5 + lVar10) + 0xe,param_3);
LAB_141b26354:
      if (((DAT_143aa8520 != 0) &&
          (iVar4 = (**(code **)(*(longlong *)(DAT_143aa8520 + 8) + 0xd0))
                             ((longlong *)(DAT_143aa8520 + 8),&PTR_PTR_143a886f8), iVar4 != 0)) &&
         (DAT_143aa8520 != 0)) {
        FUN_141b6e570();
      }
      break;
    case 0x46:
      FUN_141b29170(param_1 + -0x18,param_3);
      break;
    case 0x47:
      if (*(longlong *)(param_1 + 0x1d0) == 0) {
        uVar6 = FUN_1408a9e40(&local_38,0x124b);
        local_4a0 = 0;
        local_4a8 = 0;
        local_4b0 = 0;
        local_4b8 = 0;
        local_4c0 = 0;
        local_4c8 = 0;
        FUN_142a26280(uVar6,0,param_1 + 0x128,1);
      }
      break;
    case 0x48:
      if ((*(int *)(param_1 + 0xb8) == 4) &&
         (iVar4 = FUN_1406e8c20(param_3), iVar4 == *(int *)(param_1 + 0x1a8))) {
        FUN_14108e7f0(local_488);
        FUN_1403094b0(local_488,param_3);
        FUN_14108c210(local_488);
        uVar3 = FUN_14108cd40();
        *(undefined4 *)(param_1 + 0xd0) = uVar3;
        if (DAT_143aca790 != (longlong *)0x0) {
          (**(code **)(*DAT_143aca790 + 0x90))(DAT_143aca790,0);
        }
        if (*(longlong *)(param_1 + 0x1e0) != 0) {
          uVar6 = *(undefined8 *)(param_1 + 0x1e0);
          local_38 = 0;
          puVar7 = (undefined8 *)FUN_1408a9e40(&local_490,0x124a);
          uVar8 = FUN_14019ba10(&local_38,*puVar7,local_47c);
          local_498 = 0;
          FUN_14019a260(&local_498,uVar8);
          local_4c8 = 0;
          FUN_141e2c6b0(uVar6,&local_498,0,3);
          if (local_490 != 0) {
            FUN_14019f2c0(local_490 + -0x10);
          }
          if (local_38 != 0) {
            FUN_14019f2c0(local_38 + -0x10);
          }
        }
        FUN_1401d5120(local_352);
        if (local_38d != 0) {
          thunk_FUN_140205820(local_38d,0xc);
        }
        FUN_141b43570(local_3b1);
      }
      break;
    case 0x4a:
      uVar3 = FUN_1406e8c20(param_3);
      FUN_142cb8400(DAT_143aa84a0,uVar3);
      break;
    case 0x50:
      FUN_141b38590(param_1 + -0x18,param_3);
      break;
    case 0x5f:
      FUN_141b3acf0(param_1 + -0x18,param_3);
    }
  }
  else {
    if (iVar4 == 0x5f5) {
      FUN_141b39940(param_1 + -0x18,param_3);
      return;
    }
    if (iVar4 == 0x61b) {
      FUN_141b3a9e0(param_1 + -0x18,param_3);
      return;
    }
    if (iVar4 == 0x620) {
      FUN_141b29c90(param_1 + -0x18,param_3);
      return;
    }
    if (iVar4 == 0xfbe) {
      FUN_141b299a0(param_1 + -0x18,param_3);
      return;
    }
switchD_141b25f9b_caseD_1:
    if (iVar4 - 0x1a0U < 4) {
      FUN_142097ee0(param_1,param_2,param_3);
    }
    else if (iVar4 - 0x51U < 0x1f) {
      FUN_141b82b00(param_1,param_2,param_3);
    }
  }
  return;
}



//===========================================================
// FUN_1406e9050 @ 1406e9050   (242 bytes)
//===========================================================

undefined8 *
FUN_1406e9050(longlong param_1,undefined8 *param_2,undefined8 param_3,undefined8 param_4)

{
  ushort uVar1;
  longlong lVar2;
  ushort *puVar3;
  uint uVar4;
  undefined1 local_48 [16];
  undefined1 local_38 [32];
  
  *param_2 = 0;
  uVar4 = *(int *)(param_1 + 0x18) - *(int *)(param_1 + 0x24);
  lVar2 = *(longlong *)(param_1 + 0x10);
  if (lVar2 == 0) {
    FUN_142e52d50(0xd0,1,param_3,param_4,1);
    lVar2 = *(longlong *)(param_1 + 0x10);
    if (lVar2 != 0) goto LAB_1406e90a4;
  }
  else {
LAB_1406e90a4:
    if (*(int *)(lVar2 + -8) != 0) goto LAB_1406e90bc;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e90bc:
  puVar3 = (ushort *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  if (uVar4 < 2) {
    FUN_1401bb8b0(local_48,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_48,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar1 = *puVar3;
  if (uVar4 < uVar1 + 2) {
    FUN_1401bb8b0(local_38,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_38,(ThrowInfo *)&DAT_143a3b118);
  }
  FUN_1401d66b0(param_2,puVar3 + 1);
  *(int *)(param_1 + 0x24) = *(int *)(param_1 + 0x24) + uVar1 + 2;
  return param_2;
}



//===========================================================
// FUN_1406e8ae0 @ 1406e8ae0   (145 bytes)
//===========================================================

undefined1 FUN_1406e8ae0(longlong param_1)

{
  undefined1 uVar1;
  int iVar2;
  int iVar3;
  longlong lVar4;
  undefined1 local_30 [40];
  
  iVar2 = *(int *)(param_1 + 0x18);
  iVar3 = *(int *)(param_1 + 0x24);
  lVar4 = *(longlong *)(param_1 + 0x10);
  if (lVar4 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar4 = *(longlong *)(param_1 + 0x10);
    if (lVar4 != 0) goto LAB_1406e8b1b;
  }
  else {
LAB_1406e8b1b:
    if (*(int *)(lVar4 + -8) != 0) goto LAB_1406e8b30;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e8b30:
  if (iVar2 == iVar3) {
    FUN_1401bb8b0(local_30,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_30,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar1 = *(undefined1 *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) + 1;
  return uVar1;
}


