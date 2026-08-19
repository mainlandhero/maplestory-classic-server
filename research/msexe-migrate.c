
//===========================================================
// FUN_14108cae0 @ 14108cae0   (84 bytes)
//===========================================================

undefined4 * FUN_14108cae0(uint param_1)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  
  if (param_1 != 0) {
    cVar1 = *(char *)((longlong)DAT_143ac9890[1] + 0x19);
    puVar3 = (undefined8 *)DAT_143ac9890[1];
    puVar2 = DAT_143ac9890;
    while (puVar4 = puVar3, cVar1 == '\0') {
      if (*(uint *)(puVar4 + 4) < param_1) {
        puVar3 = (undefined8 *)puVar4[2];
        puVar4 = puVar2;
      }
      else {
        puVar3 = (undefined8 *)*puVar4;
      }
      cVar1 = *(char *)((longlong)puVar3 + 0x19);
      puVar2 = puVar4;
    }
    if (((*(char *)((longlong)puVar2 + 0x19) == '\0') && (*(uint *)(puVar2 + 4) <= param_1)) &&
       (puVar2 != DAT_143ac9890)) {
      return (undefined4 *)puVar2[5];
    }
  }
  return &DAT_143ac98a0;
}



//===========================================================
// FUN_1408414d0 @ 1408414d0   (745 bytes)
//===========================================================

void FUN_1408414d0(undefined4 param_1,int param_2)

{
  undefined8 uVar1;
  longlong *plVar2;
  undefined4 *puVar3;
  wchar_t *pwVar4;
  char *pcVar5;
  longlong *plVar6;
  longlong *local_res18;
  longlong *local_res20;
  longlong *local_78;
  short local_70 [4];
  longlong local_68;
  longlong *local_58;
  longlong *local_50;
  longlong *local_48;
  short local_40 [4];
  longlong local_38;
  longlong *local_28;
  
  FUN_14083f1d0();
  if (param_2 == 0) {
    DAT_143ac2040 = param_1;
    DAT_143ac2044 = param_2;
    return;
  }
  DAT_143ac2040 = param_1;
  DAT_143ac2044 = param_2;
  uVar1 = FUN_14090de10(local_70,L"Etc/SpecialServerInfo.img");
  uVar1 = FUN_1409339d0(&local_res18,uVar1);
  FUN_1401a5040(&local_58,uVar1);
  if (local_res18 != (longlong *)0x0) {
    (**(code **)(*local_res18 + 0x10))();
  }
  if (local_70[0] == 8) {
    local_70[0] = 0;
    if (local_68 != 0) {
      (*DAT_143ad5990)(local_68 + -4);
    }
  }
  else {
    (*DAT_143262a18)(local_70);
  }
  if (local_58 == (longlong *)0x0) goto LAB_14084177b;
  if (param_2 == 0) {
LAB_1408415b8:
    pcVar5 = "";
  }
  else if (param_2 == 1) {
    pcVar5 = "normal";
  }
  else if (param_2 == 2) {
    pcVar5 = "reboot";
  }
  else if (param_2 == 3) {
    pcVar5 = "burning";
  }
  else {
    if (param_2 != 4) goto LAB_1408415b8;
    pcVar5 = "challenge";
  }
  uVar1 = FUN_1401a5780(&local_res20,pcVar5);
  uVar1 = FUN_1401e4330(local_58,local_40,uVar1);
  uVar1 = FUN_1409339d0(&local_78,uVar1);
  FUN_1401a5040(&local_50,uVar1);
  if (local_78 != (longlong *)0x0) {
    (**(code **)(*local_78 + 0x10))();
  }
  if (local_40[0] == 8) {
    local_40[0] = 0;
    if (local_38 != 0) {
      (*DAT_143ad5990)(local_38 + -4);
    }
  }
  else {
    (*DAT_143262a18)(local_40);
  }
  local_res18 = local_50;
  if (local_50 != (longlong *)0x0) {
    (**(code **)(*local_50 + 8))(local_50);
  }
  FUN_14083f3a0(&local_res18);
  plVar2 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
  plVar6 = (longlong *)0x0;
  local_28 = plVar2;
  if (plVar2 != (longlong *)0x0) {
    plVar2[1] = 0;
    *(undefined4 *)(plVar2 + 2) = 1;
    puVar3 = (undefined4 *)(*DAT_143ad5980)(0x12);
    if (puVar3 == (undefined4 *)0x0) {
      *plVar2 = 0;
LAB_1408417b0:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    *puVar3 = 0xc;
    pwVar4 = (wchar_t *)(puVar3 + 1);
    *(undefined8 *)pwVar4 = u_common_1432a34b0._0_8_;
    puVar3[3] = u_common_1432a34b0._8_4_;
    *(wchar_t *)(puVar3 + 4) = u_common_1432a34b0[6];
    *plVar2 = (longlong)pwVar4;
    plVar6 = plVar2;
    if (pwVar4 == (wchar_t *)0x0) goto LAB_1408417b0;
  }
  local_res18 = plVar6;
  if (plVar6 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x8007000e);
  }
  uVar1 = FUN_1401e4330(local_58,local_70,&local_res18);
  uVar1 = FUN_1409339d0(&local_48,uVar1);
  FUN_1401a5040(&local_78,uVar1);
  if (local_48 != (longlong *)0x0) {
    (**(code **)(*local_48 + 0x10))();
  }
  if (local_70[0] == 8) {
    local_70[0] = 0;
    if (local_68 != 0) {
      (*DAT_143ad5990)(local_68 + -4);
    }
  }
  else {
    (*DAT_143262a18)(local_70);
  }
  local_res20 = local_78;
  if (local_78 != (longlong *)0x0) {
    (**(code **)(*local_78 + 8))(local_78);
  }
  FUN_140840b20(&local_res20);
  if (local_78 != (longlong *)0x0) {
    (**(code **)(*local_78 + 0x10))(local_78);
  }
  if (local_50 != (longlong *)0x0) {
    (**(code **)(*local_50 + 0x10))(local_50);
  }
LAB_14084177b:
  if (local_58 != (longlong *)0x0) {
    (**(code **)(*local_58 + 0x10))(local_58);
  }
  return;
}



//===========================================================
// FUN_140842250 @ 140842250   (20 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140842250(undefined4 *param_1)

{
  _DAT_143ac2160 = *param_1;
  FUN_140843f20(&DAT_143ac2160);
  return;
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



//===========================================================
// FUN_1429f14c0 @ 1429f14c0   (1599 bytes)
//===========================================================

void FUN_1429f14c0(longlong param_1,undefined4 param_2,int param_3)

{
  undefined *puVar1;
  longlong lVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  undefined4 *puVar6;
  int *piVar7;
  int iVar8;
  int iVar9;
  int *piVar10;
  int *piVar11;
  int *piVar12;
  int iVar13;
  int *piVar14;
  int *local_78;
  undefined8 local_70;
  int *local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined4 local_50;
  
  puVar1 = PTR_u_Sound_Game_img__143a46f00;
  piVar10 = (int *)0x0;
  local_78 = (int *)0x0;
  piVar14 = (int *)0xffffffffffffffff;
  piVar11 = piVar14;
  if (PTR_u_Sound_Game_img__143a46f00 != (undefined *)0x0) {
    do {
      piVar11 = (int *)((longlong)piVar11 + 1);
    } while (*(short *)(PTR_u_Sound_Game_img__143a46f00 + (longlong)piVar11 * 2) != 0);
    iVar3 = (int)piVar11;
    iVar4 = 0;
    if (0 < iVar3) {
      iVar4 = iVar3;
    }
    piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar4 * 2 + 0x12));
    piVar5[1] = iVar4;
    *piVar5 = -1;
    piVar10 = piVar5 + 4;
    piVar5[2] = 0;
    *(short *)piVar10 = 0;
    local_78 = piVar10;
    FUN_142ef7ba0(piVar10,puVar1,(longlong)iVar3 * 2);
    if (*piVar5 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar3 == -1) || (iVar3 <= piVar5[1])) {
      *piVar5 = 1;
      if (iVar3 != -1) goto LAB_1429f1591;
      piVar11 = piVar14;
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        do {
          piVar11 = (int *)((longlong)piVar11 + 1);
        } while (*(short *)((longlong)piVar10 + (longlong)piVar11 * 2) != 0);
      }
    }
    else {
      FUN_142e54290(0x90,piVar5[1],(ulonglong)piVar11 & 0xffffffff);
      *piVar5 = 1;
LAB_1429f1591:
      *(short *)((longlong)iVar3 * 2 + (longlong)piVar10) = 0;
    }
    iVar4 = (int)piVar11;
    if ((iVar4 < 0) || (piVar5[1] + 1 <= iVar4)) {
      FUN_142e54290(0x9c,(ulonglong)piVar11 & 0xffffffff);
    }
    piVar5[2] = iVar4 * 2;
  }
  piVar11 = (int *)0x0;
  piVar5 = piVar14;
  piVar12 = piVar11;
  if (param_1 != 0) {
    do {
      piVar12 = (int *)((longlong)piVar5 + 1);
      piVar5 = piVar12;
    } while (*(short *)(param_1 + (longlong)piVar12 * 2) != 0);
  }
  iVar4 = (int)piVar12;
  piVar5 = piVar10;
  if (iVar4 != 0) {
    iVar3 = 0;
    piVar7 = piVar11;
    if (piVar10 == (int *)0x0) goto LAB_1429f1778;
    if ((short)*piVar10 != 0) {
      iVar9 = (int)((ulonglong)(longlong)piVar10[-2] >> 1) + iVar4;
      for (iVar3 = piVar10[-3]; iVar3 < iVar9; iVar3 = iVar3 * 2) {
      }
      piVar11 = piVar10 + -4;
      if (piVar11 == (int *)0x0) {
        iVar13 = 0;
LAB_1429f166a:
        if (iVar13 < iVar3) {
          iVar13 = iVar3;
        }
        puVar6 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar13 * 2 + 0x12));
        puVar6[1] = iVar13;
        *puVar6 = 0xffffffff;
        piVar5 = puVar6 + 4;
        local_78 = piVar5;
        if (piVar11 == (int *)0x0) {
          puVar6[2] = 0;
          *(short *)piVar5 = 0;
        }
        else {
          iVar3 = (piVar10[-2] & 0xfffffffeU) + 2;
          iVar8 = iVar13 * 2 + 2;
          if (iVar8 < iVar3) {
            FUN_142e54290(0x5c,iVar3,iVar8);
            iVar3 = iVar8;
          }
          FUN_142ef7ba0(piVar5,piVar10,(longlong)iVar3);
          puVar6[2] = piVar10[-2];
          *(short *)((longlong)piVar5 + (longlong)iVar13 * 2) = 0;
          FUN_1401bebb0(piVar11);
        }
      }
      else {
        if ((1 < *piVar11) || (piVar10[-3] < iVar3)) {
          iVar13 = (int)((ulonglong)(longlong)piVar10[-2] >> 1);
          goto LAB_1429f166a;
        }
        if (*piVar11 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar11 = -1;
      }
      iVar3 = 0;
      if (piVar5 != (int *)0x0) {
        iVar3 = (int)((ulonglong)(longlong)piVar5[-2] >> 1);
      }
      FUN_142ef7ba0((short *)((longlong)piVar5 + (longlong)iVar3 * 2),param_1,(longlong)iVar4 * 2);
      FUN_1401bd8a0(&local_78,iVar9);
      goto LAB_1429f182b;
    }
    if ((piVar10 == (int *)0x0) || (piVar7 = piVar10 + -4, piVar7 == (int *)0x0)) {
LAB_1429f1778:
      if (iVar3 < iVar4) {
        iVar3 = iVar4;
      }
      puVar6 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar3 * 2 + 0x12));
      puVar6[1] = iVar3;
      *puVar6 = 0xffffffff;
      piVar5 = puVar6 + 4;
      puVar6[2] = 0;
      *(short *)piVar5 = 0;
      local_78 = piVar5;
      if (piVar7 != (int *)0x0) {
        FUN_1401bebb0(piVar7);
      }
    }
    else {
      if ((1 < *piVar7) || (piVar10[-3] < iVar4)) {
        iVar3 = (int)((ulonglong)(longlong)piVar10[-2] >> 1);
        goto LAB_1429f1778;
      }
      if (*piVar7 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar7 = -1;
    }
    FUN_142ef7ba0(piVar5,param_1,(longlong)iVar4 * 2);
    if (piVar5[-4] != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar4 == -1) || (iVar4 <= piVar5[-3])) {
      piVar5[-4] = 1;
      if (iVar4 != -1) goto LAB_1429f1801;
      piVar10 = piVar14;
      if (piVar5 != (int *)0x0) {
        do {
          piVar11 = (int *)((longlong)piVar10 + 1);
          piVar10 = piVar11;
        } while (*(short *)((longlong)piVar5 + (longlong)piVar11 * 2) != 0);
      }
    }
    else {
      FUN_142e54290(0x90,piVar5[-3],(ulonglong)piVar12 & 0xffffffff);
      piVar5[-4] = 1;
LAB_1429f1801:
      *(short *)((longlong)piVar5 + (longlong)iVar4 * 2) = 0;
      piVar11 = piVar12;
    }
    iVar4 = (int)piVar11;
    if ((iVar4 < 0) || (piVar5[-3] + 1 <= iVar4)) {
      FUN_142e54290(0x9c,(ulonglong)piVar11 & 0xffffffff);
    }
    piVar5[-2] = iVar4 * 2;
  }
LAB_1429f182b:
  if (param_3 < 1) {
    FUN_142085330(DAT_143abfea0,piVar5,param_2,0,0,0,3,0,0,4);
    goto LAB_1429f1adc;
  }
  piVar12 = (int *)0x0;
  local_70 = 0;
  local_68 = (int *)0x0;
  local_60 = 100;
  local_58 = 0;
  local_50 = 4;
  iVar4 = FUN_142c4a030(DAT_143ac1898);
  piVar10 = local_68;
  lVar2 = DAT_143abfea0;
  *(int *)(DAT_143abfea0 + 0x108) = *(int *)(DAT_143abfea0 + 0x108) + 1;
  local_70 = CONCAT44(*(undefined4 *)(lVar2 + 0x108),iVar4 + param_3);
  iVar3 = 0;
  piVar11 = piVar12;
  iVar4 = iVar3;
  if (local_68 != (int *)0x0) {
    piVar11 = local_68;
    iVar4 = (int)((ulonglong)(longlong)local_68[-2] >> 1);
  }
  iVar9 = iVar3;
  if (piVar5 != (int *)0x0) {
    piVar12 = piVar5;
    iVar9 = (int)((ulonglong)(longlong)piVar5[-2] >> 1);
  }
  if (((iVar4 == iVar9) && (iVar4 != 0)) && (piVar11 != (int *)0x0)) {
    if (piVar12 == (int *)0x0) goto LAB_1429f1a63;
    iVar4 = memcmp(piVar11,piVar12,(longlong)iVar4 * 2);
    piVar11 = local_68;
    if (iVar4 != 0) goto LAB_1429f1913;
  }
  else {
LAB_1429f1913:
    if ((piVar12 == (int *)0x0) || (piVar11 = piVar12 + -4, piVar11 == (int *)0x0)) {
LAB_1429f1a63:
      piVar11 = local_68;
      if (piVar10 != (int *)0x0) {
        FUN_1401bebb0(piVar10 + -4);
        local_68 = (int *)0x0;
        piVar11 = local_68;
      }
    }
    else {
      if (*piVar11 != -1) {
        if (*piVar11 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar11 = *piVar11 + 1;
        UNLOCK();
        piVar5 = local_78;
        piVar11 = piVar12;
        if (local_68 != (int *)0x0) {
          FUN_1401bebb0(local_68 + -4);
          piVar5 = local_78;
        }
        goto LAB_1429f1a75;
      }
      FUN_142e52d50(0xcb,0xffffff01);
      piVar10 = piVar14;
      do {
        piVar10 = (int *)((longlong)piVar10 + 1);
      } while (*(short *)((longlong)piVar12 + (longlong)piVar10 * 2) != 0);
      iVar4 = (int)piVar10;
      if (0 < iVar4) {
        iVar3 = iVar4;
      }
      piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar3 * 2 + 0x12));
      piVar7[1] = iVar3;
      *piVar7 = -1;
      piVar11 = piVar7 + 4;
      piVar7[2] = 0;
      *(short *)piVar11 = 0;
      FUN_142ef7ba0(piVar11,piVar12,(longlong)iVar4 * 2);
      if (*piVar7 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar7[1])) {
        *piVar7 = 1;
        if (iVar4 != -1) goto LAB_1429f19c9;
        if (piVar11 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          do {
            piVar14 = (int *)((longlong)piVar14 + 1);
          } while (*(short *)((longlong)piVar11 + (longlong)piVar14 * 2) != 0);
          piVar10 = (int *)((ulonglong)piVar14 & 0xffffffff);
        }
      }
      else {
        FUN_142e54290(0x90,piVar7[1],(ulonglong)piVar10 & 0xffffffff);
        *piVar7 = 1;
LAB_1429f19c9:
        *(short *)((longlong)piVar11 + (longlong)iVar4 * 2) = 0;
      }
      iVar4 = (int)piVar10;
      if ((iVar4 < 0) || (piVar7[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,(ulonglong)piVar10 & 0xffffffff);
      }
      piVar7[2] = iVar4 * 2;
      if (local_68 != (int *)0x0) {
        FUN_1401bebb0(local_68 + -4);
      }
    }
  }
LAB_1429f1a75:
  local_68 = piVar11;
  local_60 = CONCAT44(local_60._4_4_,param_2);
  FUN_14208eab0(DAT_143abfea0,&local_70);
  if (local_68 != (int *)0x0) {
    FUN_1401bebb0(local_68 + -4);
  }
LAB_1429f1adc:
  if (piVar5 != (int *)0x0) {
    FUN_1401bebb0(piVar5 + -4);
  }
  return;
}


