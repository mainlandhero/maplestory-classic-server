
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



//===========================================================
// FUN_1406e8f10 @ 1406e8f10   (146 bytes)
//===========================================================

undefined8 FUN_1406e8f10(longlong param_1)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  longlong lVar4;
  undefined1 local_30 [40];
  
  iVar1 = *(int *)(param_1 + 0x18);
  iVar2 = *(int *)(param_1 + 0x24);
  lVar4 = *(longlong *)(param_1 + 0x10);
  if (lVar4 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar4 = *(longlong *)(param_1 + 0x10);
    if (lVar4 != 0) goto LAB_1406e8f4b;
  }
  else {
LAB_1406e8f4b:
    if (*(int *)(lVar4 + -8) != 0) goto LAB_1406e8f60;
  }
  FUN_142e54290(0xbc,0,0);
LAB_1406e8f60:
  if ((uint)(iVar1 - iVar2) < 8) {
    FUN_1401bb8b0(local_30,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_30,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar3 = *(undefined8 *)((ulonglong)*(uint *)(param_1 + 0x24) + *(longlong *)(param_1 + 0x10));
  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) + 8;
  return uVar3;
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
// FUN_141b267c0 @ 141b267c0   (2321 bytes)
//===========================================================

undefined8 FUN_141b267c0(longlong param_1,int param_2,char param_3,longlong *param_4)

{
  longlong lVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  wchar_t *pwVar5;
  longlong local_48;
  longlong local_40;
  longlong *local_38;
  longlong *local_30;
  
  if (param_2 != 0) {
    FUN_14209c060(param_1,0);
  }
  switch(param_2 + 1) {
  case 0:
  case 7:
  case 9:
  case 10:
    local_38 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    uVar3 = FUN_1403edf80(&local_40,L"loginTroubleAskSupport",0xffffffff);
    FUN_141b4ac80(uVar3,&local_48,0);
  default:
    goto switchD_141b2681f_caseD_1;
  case 4:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"blockedID";
    break;
  case 5:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"incorrectPassword";
    break;
  case 6:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"notRegisteredID";
    break;
  case 8:
    FUN_141b2d290(param_1,0,0);
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"loginAlready";
    break;
  case 0xb:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"loginTimeout";
    break;
  case 0xc:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    goto LAB_141b26fff;
  case 0xe:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"blockedIPAddr";
    break;
  case 0xf:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    uVar3 = FUN_1403edf80(&local_40,L"notAdult",0xffffffff);
    FUN_141b4ac80(uVar3,&local_48,0);
    puVar4 = (undefined8 *)FUN_1408a9e40(&local_40,0x803);
    FUN_1429e4fa0(*puVar4,0,0);
    goto LAB_141b26d1b;
  case 0x10:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    uVar3 = FUN_1403edf80(&local_40,L"notRegisteredAccount",0xffffffff);
    FUN_141b4ac80(uVar3,&local_48,0);
    puVar4 = (undefined8 *)FUN_1408a9e40(&local_40,0x803);
    FUN_1429e4fa0(*puVar4,0,0);
    goto LAB_141b26d1b;
  case 0x12:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"accountNotVerified";
    break;
  case 0x14:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    uVar3 = FUN_1403edf80(&local_40,L"temporaryBlockedIPAddr",0xffffffff);
    FUN_141b4ac80(uVar3,&local_48,0);
    *(undefined4 *)(param_1 + 0xf0) = 1;
    goto LAB_141b26873;
  case 0x1b:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"systemErrorOTP";
    break;
  case 0x1d:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"expiredOTP";
    break;
  case 0x23:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"loginFailedServerIsFull";
    break;
  case 0x24:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"onOTPReissuing";
    break;
  case 0x25:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"onOTPReissuing";
    break;
  case 0x26:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"shutdownNotAdult";
    break;
  case 0x29:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"shutdownNotAdultSelective";
    break;
  case 0x2a:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"mapleIDConvertedToNexonEmailID";
    break;
  case 0x2e:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"signUpOTPforCoopPermission";
    break;
  case 0x33:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"shutdownNotAdultSelective";
    break;
  case 0x36:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"mustCreatePIC";
    break;
  case 0x40:
    uVar3 = FUN_1408a9e40(&local_40,0x128b);
    FUN_142a26280(uVar3,0,0,1,0,0,0,0,0,0);
    puVar4 = (undefined8 *)FUN_1408a9e40(&local_40,0x128a);
    FUN_1429e4fa0(*puVar4,0,0);
LAB_141b26d1b:
    if (local_40 != 0) {
      FUN_14019f2c0(local_40 + -0x10);
    }
    goto LAB_141b26873;
  case 0x41:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"overseasLoginBlockActivated";
    break;
  case 0x45:
  case 0x4a:
    uVar3 = FUN_1403edf80(&local_40,L"accountProtected",0xffffffff);
    FUN_141b49b80(0xc,uVar3,0);
    local_48 = 0;
    puVar4 = (undefined8 *)FUN_1408a9e40(&local_40,0xe);
    FUN_14019ba10(&local_48,*puVar4);
    if (local_40 != 0) {
      FUN_14019f2c0(local_40 + -0x10);
    }
    lVar1 = local_48;
    FUN_1429e4fa0(local_48,0,0);
    if (DAT_143ad2220 != 0) {
      FUN_141b5ff30();
    }
    if (lVar1 != 0) {
      FUN_14019f2c0(lVar1 + -0x10);
    }
    goto switchD_141b2681f_caseD_1;
  case 0x4f:
    FUN_141b2d290(param_1,0,0);
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"loginAlready";
    break;
  case 0x53:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"notVerifiedEmail";
    break;
  case 0x5d:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"inactiveMember";
    break;
  case 0x60:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"accountSuspended";
    break;
  case 99:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"accountBeingCanceled";
    break;
  case 0x65:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"cannotLoginToProtectAccount";
    break;
  case 0x68:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"cannotUseWorldIsFull";
    break;
  case 0x69:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"cannotProcessRequest";
    break;
  case 0x6e:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"accountWaitForSignUp";
    break;
  case 0x71:
    uVar3 = FUN_1403edf80(&local_40,L"blockMapleID",0xffffffff);
    iVar2 = FUN_141b49b80(8,uVar3,1,param_1 + 0x140);
    if (iVar2 == 6) {
      puVar4 = (undefined8 *)FUN_1408a9e40(&local_40,0x15);
      FUN_1429e4fa0(*puVar4,0,0);
      if (local_40 != 0) {
        FUN_14019f2c0(local_40 + -0x10);
      }
    }
    goto LAB_141b26873;
  case 0x7e:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"inactiveAccount";
    break;
  case 0x7f:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    pwVar5 = L"accountVerificationRequested";
    break;
  case 0x80:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
LAB_141b26fff:
    pwVar5 = L"notAdult";
    break;
  case 0x86:
    local_30 = &local_48;
    local_48 = 0;
    FUN_1401c1fb0(&local_48,param_4);
    uVar3 = FUN_1403edf80(&local_40,L"mapleIDConvertedToNexonEmailID",0xffffffff);
    FUN_141b4ac80(uVar3,&local_48,0);
    puVar4 = (undefined8 *)FUN_1408a9e40(&local_38,4);
    FUN_1429e4fa0(*puVar4,0,0);
    if (local_38 != (longlong *)0x0) {
      FUN_14019f2c0(local_38 + -2);
    }
    goto LAB_141b26873;
  }
  uVar3 = FUN_1403edf80(&local_40,pwVar5,0xffffffff);
  FUN_141b4ac80(uVar3,&local_48,0);
LAB_141b26873:
  if (param_3 != '\0') {
    *(undefined4 *)(param_1 + 0xf0) = 1;
  }
  if (*param_4 != 0) {
    FUN_1401bebb0(*param_4 + -0x10);
  }
  return 0;
switchD_141b2681f_caseD_1:
  if ((param_2 == 0) || (param_2 == 0xc)) {
    if (*param_4 != 0) {
      FUN_1401bebb0(*param_4 + -0x10);
    }
    return 1;
  }
  goto LAB_141b26873;
}


