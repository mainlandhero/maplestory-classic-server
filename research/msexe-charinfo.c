
//===========================================================
// FUN_141e75800 @ 141e75800   (666 bytes)
//===========================================================

ulonglong FUN_141e75800(longlong param_1,int param_2,undefined8 param_3)

{
  byte *pbVar1;
  undefined8 *puVar2;
  byte bVar3;
  uint uVar4;
  undefined4 uVar5;
  ulonglong uVar6;
  undefined8 *puVar7;
  longlong lVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  undefined8 *puVar11;
  longlong lVar12;
  int iVar13;
  uint local_res10 [2];
  
  if (param_2 == 0xbe) {
    bVar3 = FUN_1406e8ae0(param_3);
    uVar6 = FUN_14040d460(param_1 + 0x40);
    if (bVar3 == 0) {
      return uVar6;
    }
    uVar6 = (ulonglong)bVar3;
    do {
      local_res10[0] = FUN_1406e8c20(param_3);
      uVar10 = FUN_140409670(param_1 + 0x40,local_res10);
      uVar6 = uVar6 - 1;
    } while (uVar6 != 0);
    return uVar10;
  }
  if (param_2 != 0x44f) {
    if (param_2 != 0x450) {
      if (param_2 == 0x451) {
        uVar6 = FUN_141e75f50(param_1,param_3);
        return uVar6;
      }
      if (param_2 != 0x452) {
        if (param_2 - 0x453U < 0x14) {
          FUN_141e75aa0(param_1);
        }
        if (param_2 - 0x467U != 0) {
          return (ulonglong)(param_2 - 0x467U);
        }
        if (param_2 != 0x467) {
          return 0;
        }
        uVar6 = FUN_141e78110(param_3);
        return uVar6;
      }
      uVar6 = FUN_141e76c60(param_1,param_3);
      return uVar6;
    }
    uVar6 = FUN_1406e8c20(param_3);
    uVar10 = uVar6 & 0xffffffff;
    if (*(longlong *)(param_1 + 8) == 0) {
      return uVar6;
    }
    iVar13 = (int)uVar6;
    uVar6 = uVar10 / *(uint *)(param_1 + 0x10);
    lVar12 = *(longlong *)
              (*(longlong *)(param_1 + 8) + (uVar10 % (ulonglong)*(uint *)(param_1 + 0x10)) * 8);
    while( true ) {
      if (lVar12 == 0) {
        return uVar6;
      }
      if (*(int *)(lVar12 + 0x10) == iVar13) break;
      lVar12 = *(longlong *)(lVar12 + 8);
    }
    if (*(longlong *)(lVar12 + 0x18) == 0) {
      return uVar6;
    }
    pbVar1 = (byte *)(*(longlong *)(lVar12 + 0x18) + 0x38);
    *pbVar1 = *pbVar1 & 0xfe;
    if (*pbVar1 != 0) {
      return uVar6;
    }
    uVar6 = FUN_141e77500(param_1 + 0x20);
    lVar12 = *(longlong *)(param_1 + 8);
    if (lVar12 == 0) {
      return uVar6;
    }
    uVar6 = uVar10 % (ulonglong)*(uint *)(param_1 + 0x10);
    puVar11 = *(undefined8 **)(lVar12 + uVar6 * 8);
    if (puVar11 == (undefined8 *)0x0) {
      return uVar10 / *(uint *)(param_1 + 0x10);
    }
    puVar2 = (undefined8 *)puVar11[1];
    if (*(int *)(puVar11 + 2) == iVar13) {
      *(undefined8 **)(lVar12 + uVar6 * 8) = puVar2;
      uVar6 = (**(code **)*puVar11)(puVar11,1);
      *(int *)(param_1 + 0x14) = *(int *)(param_1 + 0x14) + -1;
      return uVar6;
    }
    if (puVar2 == (undefined8 *)0x0) {
      return 0;
    }
    do {
      puVar7 = puVar2;
      puVar2 = (undefined8 *)puVar7[1];
      if (*(int *)(puVar7 + 2) == iVar13) {
        puVar11[1] = puVar2;
        uVar6 = 0;
        if (puVar7 != (undefined8 *)0x0) {
          uVar6 = (**(code **)*puVar7)(puVar7,1);
        }
        *(int *)(param_1 + 0x14) = *(int *)(param_1 + 0x14) + -1;
        return uVar6;
      }
      puVar11 = puVar7;
    } while (puVar2 != (undefined8 *)0x0);
    return 0;
  }
  uVar4 = FUN_1406e8c20(param_3);
  if (*(longlong *)(param_1 + 8) != 0) {
    for (lVar12 = *(longlong *)
                   (*(longlong *)(param_1 + 8) +
                   ((ulonglong)uVar4 % (ulonglong)*(uint *)(param_1 + 0x10)) * 8); lVar12 != 0;
        lVar12 = *(longlong *)(lVar12 + 8)) {
      if (*(uint *)(lVar12 + 0x10) == uVar4) {
        uVar6 = *(ulonglong *)(lVar12 + 0x18);
        if (uVar6 != 0) {
          *(byte *)(uVar6 + 0x38) = *(byte *)(uVar6 + 0x38) | 1;
          return uVar6;
        }
        break;
      }
    }
  }
  local_res10[0] = uVar4;
  lVar8 = FUN_141e77060(param_1 + 0x20);
  uVar5 = FUN_1406e8c20(param_3);
  uVar9 = FUN_141e77b70(uVar5,0);
  uVar9 = FUN_141e4c620(uVar9,0);
  FUN_141e76e90(lVar8 + 0x20,uVar9);
  *(undefined1 *)(lVar8 + 0x38) = 1;
  *(undefined8 *)(lVar8 + 0x30) = *(undefined8 *)(param_1 + 0x30);
  FUN_141e77160(param_1 + 8,local_res10);
  lVar12 = *(longlong *)(lVar8 + 0x28);
  if (lVar12 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar12 = *(longlong *)(lVar8 + 0x28);
  }
  uVar6 = FUN_141e36b20(lVar12,uVar4,param_3);
  return uVar6;
}



//===========================================================
// FUN_141e75aa0 @ 141e75aa0   (533 bytes)
//===========================================================

void FUN_141e75aa0(longlong param_1,undefined4 param_2,undefined8 param_3)

{
  longlong lVar1;
  bool bVar2;
  uint uVar3;
  longlong lVar4;
  undefined1 local_18 [8];
  longlong local_10;
  
  lVar4 = 0;
  uVar3 = FUN_1406e8c20(param_3);
  if (*(longlong *)(param_1 + 8) != 0) {
    for (lVar1 = *(longlong *)
                  (*(longlong *)(param_1 + 8) +
                  ((ulonglong)uVar3 % (ulonglong)*(uint *)(param_1 + 0x10)) * 8); lVar1 != 0;
        lVar1 = *(longlong *)(lVar1 + 8)) {
      if (*(uint *)(lVar1 + 0x10) == uVar3) {
        if (*(longlong *)(lVar1 + 0x18) != 0) {
          lVar4 = *(longlong *)(*(longlong *)(lVar1 + 0x18) + 0x28);
          local_10 = lVar4;
          if (lVar4 != 0) {
            if (0xfffff < *(ulonglong *)(lVar4 + 0x18)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar4 + 0x18) = *(longlong *)(lVar4 + 0x18) + 1;
            UNLOCK();
          }
          bVar2 = true;
          goto LAB_141e75b04;
        }
        break;
      }
    }
  }
  bVar2 = false;
LAB_141e75b04:
  if (bVar2) {
    FUN_1401d33f0(local_18);
  }
  if (lVar4 != 0) {
    switch(param_2) {
    case 0x453:
      FUN_141e421f0(lVar4,param_3);
      break;
    case 0x454:
      FUN_141e433a0(lVar4,param_3);
      break;
    case 0x455:
      FUN_141e43bb0(lVar4,param_3);
      break;
    case 0x456:
      FUN_141e43bd0(lVar4,param_3);
      break;
    case 0x457:
      FUN_141e43c50(lVar4,param_3);
      break;
    case 0x458:
      FUN_141e43cd0(lVar4,param_3);
      break;
    case 0x459:
      FUN_141e43d40(lVar4,param_3);
      break;
    case 0x45a:
      FUN_141e43e10(lVar4,param_3);
      break;
    case 0x45b:
      FUN_141e44a00(lVar4,param_3);
      break;
    case 0x45c:
      FUN_141e44a60(lVar4,param_3);
      break;
    case 0x45d:
      FUN_141e44ab0(lVar4,param_3);
      break;
    case 0x45e:
      FUN_141e44af0(lVar4,param_3);
      break;
    case 0x45f:
      FUN_141e432c0(lVar4,param_3);
      break;
    case 0x460:
      FUN_141e44b10(lVar4,param_3);
      break;
    case 0x461:
      FUN_141e44d00(lVar4,param_3);
      break;
    case 0x462:
      FUN_141e44d50(lVar4,param_3);
      break;
    case 0x463:
      FUN_141e44dc0(lVar4,param_3);
      break;
    case 0x464:
      FUN_141e44e10(lVar4,param_3);
      break;
    case 0x465:
      FUN_141e430c0(lVar4,param_3);
      break;
    case 0x466:
      FUN_141e42ff0(lVar4,param_3);
    }
  }
  return;
}



//===========================================================
// FUN_141e78110 @ 141e78110   (215 bytes)
//===========================================================

void FUN_141e78110(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  byte bVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined4 local_38;
  longlong local_34;
  undefined4 local_2c;
  undefined4 local_28;
  
  puVar4 = (undefined8 *)FUN_1408f66e0(&local_48);
  uVar1 = *puVar4;
  uVar2 = puVar4[1];
  local_34 = 0;
  bVar3 = FUN_1406e8ae0(param_1);
  if (bVar3 != 0) {
    uVar7 = (ulonglong)bVar3;
    do {
      FUN_1402eeef0(&local_38,param_1);
      lVar5 = FUN_141e77b70(local_38,0);
      if (lVar5 != 0) {
        lVar6 = FUN_141e8b020(lVar5 + 0x160);
        FUN_14019a260(lVar6,&local_34);
        *(undefined4 *)(lVar6 + 8) = local_2c;
        *(undefined4 *)(lVar6 + 0xc) = local_28;
        local_48 = uVar1;
        uStack_40 = uVar2;
        FUN_141e86bc0(lVar5,&local_48);
      }
      uVar7 = uVar7 - 1;
    } while (uVar7 != 0);
  }
  if (local_34 != 0) {
    FUN_14019f2c0(local_34 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141e76c60 @ 141e76c60   (394 bytes)
//===========================================================

void FUN_141e76c60(longlong param_1,undefined8 param_2)

{
  longlong *plVar1;
  undefined4 uVar2;
  char cVar3;
  int iVar4;
  longlong lVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined8 uVar8;
  
  iVar4 = FUN_1406e8c20(param_2);
  DAT_143ad2d30 = (uint)(iVar4 != 0);
  uVar7 = *(ulonglong *)(param_1 + 0x28);
  if (iVar4 == 0) {
    while (uVar6 = uVar7, uVar6 != 0) {
      uVar7 = *(ulonglong *)(uVar6 + 8);
      if ((uVar7 != 0) && (uVar7 < 0x10001)) {
        FUN_142e52ed0(0x33e);
        uVar7 = *(ulonglong *)(uVar6 + 8);
      }
      if ((*(longlong *)(uVar6 + 0x28) != 0) && (lVar5 = FUN_141892840(), lVar5 != 0)) {
        uVar8 = FUN_141892840();
        cVar3 = FUN_1418826a0(uVar8);
        if (cVar3 != '\0') {
          lVar5 = *(longlong *)(uVar6 + 0x28);
          if (lVar5 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar5 = *(longlong *)(uVar6 + 0x28);
          }
          FUN_141e64690(lVar5);
          lVar5 = FUN_142df7280(DAT_143ac18d8);
          uVar2 = *(undefined4 *)(lVar5 + 0xc);
          lVar5 = FUN_14019b780(&DAT_143ad68a0,0x90);
          if (lVar5 == 0) {
            uVar8 = 0;
          }
          else {
            uVar8 = FUN_140d13c80(lVar5,*(undefined8 *)(uVar6 + 0x28));
          }
          FUN_140d13cc0(uVar8,uVar2);
          lVar5 = *(longlong *)(uVar6 + 0x28);
          if (lVar5 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar5 = *(longlong *)(uVar6 + 0x28);
          }
          FUN_141e64580(lVar5,uVar8);
        }
      }
    }
  }
  else {
    while (uVar7 != 0) {
      uVar6 = *(ulonglong *)(uVar7 + 8);
      if ((uVar6 != 0) && (uVar6 < 0x10001)) {
        FUN_142e52ed0(0x33e);
        uVar6 = *(ulonglong *)(uVar7 + 8);
      }
      plVar1 = (longlong *)(uVar7 + 0x28);
      uVar7 = uVar6;
      if (*plVar1 != 0) {
        FUN_141e64690();
      }
    }
  }
  return;
}



//===========================================================
// FUN_141e75f50 @ 141e75f50   (460 bytes)
//===========================================================

void FUN_141e75f50(longlong param_1,undefined8 param_2)

{
  char cVar1;
  uint uVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  uint local_res18 [4];
  
  cVar1 = FUN_1406e8ae0(param_2);
  uVar2 = FUN_1406e8c20(param_2);
  lVar4 = *(longlong *)(param_1 + 8);
  local_res18[0] = uVar2;
  if (cVar1 == '\0') {
    if (lVar4 != 0) {
      for (lVar4 = *(longlong *)
                    (lVar4 + ((ulonglong)uVar2 % (ulonglong)*(uint *)(param_1 + 0x10)) * 8);
          lVar4 != 0; lVar4 = *(longlong *)(lVar4 + 8)) {
        if (*(uint *)(lVar4 + 0x10) == uVar2) {
          lVar4 = *(longlong *)(lVar4 + 0x18);
          if (lVar4 == 0) {
            return;
          }
          if ((*(byte *)(lVar4 + 0x38) & 2) == 0) {
            return;
          }
          *(byte *)(lVar4 + 0x38) = *(byte *)(lVar4 + 0x38) & 0xfd;
          lVar6 = *(longlong *)(lVar4 + 0x28);
          if (lVar6 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar6 = *(longlong *)(lVar4 + 0x28);
          }
          FUN_141e397b0(lVar6,0);
          if (*(char *)(lVar4 + 0x38) != '\0') {
            return;
          }
          FUN_141e77500(param_1 + 0x20,*(undefined8 *)(lVar4 + 0x30));
          FUN_141e77700(param_1 + 8,local_res18);
          return;
        }
      }
    }
  }
  else {
    if (lVar4 != 0) {
      for (lVar4 = *(longlong *)
                    (lVar4 + ((ulonglong)uVar2 % (ulonglong)*(uint *)(param_1 + 0x10)) * 8);
          lVar4 != 0; lVar4 = *(longlong *)(lVar4 + 8)) {
        if (*(uint *)(lVar4 + 0x10) == uVar2) {
          lVar4 = *(longlong *)(lVar4 + 0x18);
          if (lVar4 != 0) {
            if ((*(byte *)(lVar4 + 0x38) & 2) != 0) {
              return;
            }
            *(byte *)(lVar4 + 0x38) = *(byte *)(lVar4 + 0x38) | 2;
            goto LAB_141e760ca;
          }
          break;
        }
      }
    }
    lVar4 = FUN_141e77060(param_1 + 0x20);
    uVar3 = FUN_1406e8c20(param_2);
    uVar5 = FUN_141e77b70(uVar3,0);
    uVar5 = FUN_141e4c620(uVar5,0);
    FUN_141e76e90(lVar4 + 0x20,uVar5);
    *(undefined1 *)(lVar4 + 0x38) = 2;
    *(undefined8 *)(lVar4 + 0x30) = *(undefined8 *)(param_1 + 0x30);
    FUN_141e77160(param_1 + 8,local_res18);
    lVar6 = *(longlong *)(lVar4 + 0x28);
    if (lVar6 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar6 = *(longlong *)(lVar4 + 0x28);
    }
    FUN_141e36b20(lVar6,uVar2,param_2);
LAB_141e760ca:
    lVar6 = *(longlong *)(lVar4 + 0x28);
    if (lVar6 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar6 = *(longlong *)(lVar4 + 0x28);
    }
    FUN_141e397b0(lVar6,1);
  }
  return;
}



//===========================================================
// FUN_141e36b20 @ 141e36b20   (11342 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

void FUN_141e36b20(longlong *param_1,undefined4 param_2,longlong *param_3)

{
  undefined2 *puVar1;
  code *pcVar2;
  longlong *plVar3;
  IUnknown *pIVar4;
  undefined4 *puVar5;
  undefined *puVar6;
  char *pcVar7;
  char cVar8;
  byte bVar9;
  undefined1 uVar10;
  short sVar11;
  undefined4 uVar12;
  int iVar13;
  undefined4 uVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  longlong *plVar18;
  undefined8 uVar19;
  longlong *plVar20;
  longlong lVar21;
  ulonglong uVar22;
  undefined8 *puVar23;
  longlong **pplVar24;
  uint uVar25;
  uint uVar26;
  longlong *plVar27;
  undefined1 *puVar28;
  uint uVar29;
  longlong *plVar30;
  undefined4 uVar31;
  longlong *plVar32;
  undefined8 uStack_860;
  undefined1 auStack_858 [32];
  int *local_838;
  undefined8 *local_830;
  uint *local_828;
  longlong *local_820;
  undefined8 *local_818;
  undefined8 *local_810;
  longlong **local_808;
  undefined4 local_800 [8];
  longlong local_7e0;
  undefined4 local_7d8 [2];
  undefined1 local_7d0 [8];
  undefined4 local_7c8 [4];
  IUnknown *local_7b8;
  IUnknown *local_7b0;
  longlong *local_7a8;
  longlong *local_7a0;
  uint local_798;
  longlong **local_790;
  short local_788;
  undefined6 uStack_786;
  longlong lStack_780;
  undefined8 local_778;
  short local_770;
  undefined6 uStack_76e;
  longlong lStack_768;
  undefined8 local_760;
  short local_758;
  undefined6 uStack_756;
  longlong lStack_750;
  undefined8 local_748;
  short local_740;
  undefined6 uStack_73e;
  longlong lStack_738;
  undefined8 local_730;
  short local_728;
  undefined6 uStack_726;
  longlong lStack_720;
  undefined8 local_718;
  short local_710;
  undefined6 uStack_70e;
  longlong lStack_708;
  undefined8 local_700;
  short local_6f8;
  undefined6 uStack_6f6;
  longlong lStack_6f0;
  undefined8 local_6e8;
  undefined4 local_6e0;
  undefined4 uStack_6dc;
  undefined8 uStack_6d8;
  undefined8 local_6d0;
  short local_6c8;
  undefined6 uStack_6c6;
  longlong lStack_6c0;
  undefined8 local_6b8;
  short local_6b0;
  undefined6 uStack_6ae;
  longlong lStack_6a8;
  undefined8 local_6a0;
  undefined4 local_698;
  undefined4 uStack_694;
  undefined8 uStack_690;
  undefined8 local_688;
  short local_680;
  undefined6 uStack_67e;
  longlong lStack_678;
  undefined8 local_670;
  short local_668;
  undefined6 uStack_666;
  longlong lStack_660;
  undefined8 local_658;
  longlong *local_650;
  IUnknown *local_648;
  longlong *local_640;
  undefined4 local_638;
  undefined4 local_634;
  int local_630;
  int local_62c;
  undefined4 local_628;
  int local_624;
  int local_620 [2];
  char *local_618;
  int local_610;
  int local_60c;
  longlong *local_608;
  longlong *local_600;
  longlong *local_5f8;
  longlong *local_5f0;
  longlong *local_5e8;
  longlong *local_5e0;
  longlong *local_5d8;
  longlong *local_5d0;
  longlong *local_5c8;
  longlong *local_5c0;
  longlong *local_5b8;
  undefined8 local_5b0;
  longlong local_5a8 [2];
  longlong *local_598;
  longlong *local_590;
  longlong *local_588;
  longlong *local_580;
  IUnknown *local_578;
  longlong *local_570;
  uint local_568;
  undefined4 uStack_564;
  undefined4 uStack_560;
  undefined4 uStack_55c;
  undefined8 local_558;
  uint local_550;
  undefined4 uStack_54c;
  undefined4 uStack_548;
  undefined4 uStack_544;
  undefined8 local_540;
  uint local_538;
  undefined4 uStack_534;
  undefined8 uStack_530;
  undefined8 local_528;
  uint local_520;
  undefined4 uStack_51c;
  undefined8 uStack_518;
  undefined8 local_510;
  uint local_508;
  undefined4 uStack_504;
  undefined4 uStack_500;
  undefined4 uStack_4fc;
  undefined8 local_4f8;
  short local_4f0 [4];
  longlong lStack_4e8;
  undefined8 local_4e0;
  short local_4d8 [4];
  longlong lStack_4d0;
  undefined8 local_4c8;
  short local_4c0 [4];
  longlong lStack_4b8;
  undefined8 local_4b0;
  uint local_4a8;
  undefined4 uStack_4a4;
  undefined8 uStack_4a0;
  undefined8 local_498;
  uint local_490;
  undefined4 uStack_48c;
  undefined4 uStack_488;
  undefined4 uStack_484;
  undefined8 local_480;
  short local_478 [4];
  longlong lStack_470;
  undefined8 local_468;
  short local_460 [4];
  longlong lStack_458;
  undefined8 local_450;
  short local_448 [4];
  longlong lStack_440;
  undefined8 local_438;
  short local_430 [4];
  longlong lStack_428;
  undefined8 local_420;
  uint local_418;
  undefined4 uStack_414;
  undefined4 uStack_410;
  undefined4 uStack_40c;
  undefined8 local_408;
  undefined1 local_3f8 [8];
  longlong *local_3f0;
  undefined1 local_3e8 [8];
  longlong *local_3e0;
  longlong *local_3d8;
  undefined8 local_3d0;
  longlong *local_3c8;
  undefined1 local_3c0 [8];
  undefined1 local_3b8 [8];
  undefined8 local_3b0;
  longlong **local_3a8;
  longlong local_3a0;
  undefined1 local_398 [16];
  undefined1 local_388 [8];
  longlong local_380;
  uint local_378;
  undefined4 uStack_374;
  undefined4 uStack_370;
  undefined4 uStack_36c;
  undefined8 local_368;
  longlong local_358;
  longlong lStack_350;
  undefined8 local_348;
  undefined8 local_338;
  longlong lStack_330;
  undefined8 local_328;
  undefined8 local_318;
  longlong lStack_310;
  undefined8 local_308;
  undefined8 local_2f8;
  longlong lStack_2f0;
  undefined8 local_2e8;
  uint local_2d8;
  undefined4 uStack_2d4;
  undefined4 uStack_2d0;
  undefined4 uStack_2cc;
  undefined8 local_2c8;
  longlong local_2b8;
  longlong lStack_2b0;
  undefined8 local_2a8;
  undefined8 local_298;
  longlong lStack_290;
  undefined8 local_288;
  undefined8 local_278;
  longlong lStack_270;
  undefined8 local_268;
  undefined8 local_258;
  longlong lStack_250;
  undefined8 local_248;
  uint local_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined8 local_228;
  undefined8 local_218;
  longlong lStack_210;
  undefined8 local_208;
  uint local_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined8 local_1e8;
  longlong local_1d8;
  longlong lStack_1d0;
  undefined8 local_1c8;
  uint local_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined8 local_1a8;
  uint local_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined8 local_188;
  undefined8 local_178;
  longlong lStack_170;
  undefined8 local_168;
  undefined8 local_158;
  longlong lStack_150;
  undefined8 local_148;
  longlong local_138;
  longlong lStack_130;
  undefined8 local_128;
  uint local_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 local_108;
  uint local_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined8 local_e8;
  longlong local_d8;
  longlong lStack_d0;
  undefined8 local_c8;
  uint local_b8 [2];
  longlong lStack_b0;
  undefined8 local_a8;
  undefined8 local_98;
  longlong lStack_90;
  undefined8 local_88;
  int local_78 [2];
  longlong lStack_70;
  undefined8 local_68;
  undefined8 local_58;
  longlong lStack_50;
  undefined8 local_48;
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)&local_7b8;
  plVar32 = (longlong *)0x0;
  uVar31 = 0;
  local_798 = 0;
  *(undefined4 *)(param_1 + 0x32) = param_2;
  uStack_860 = 0x141e36b7e;
  local_640 = param_3;
  FUN_141e3ada0();
  uStack_860 = 0x141e36b86;
  sVar11 = FUN_1406e8b80(param_3);
  *(int *)(param_1 + 0x7e) = (int)sVar11;
  uStack_860 = 0x141e36b99;
  sVar11 = FUN_1406e8b80(param_3);
  *(int *)((longlong)param_1 + 0x3f4) = (int)sVar11;
  uStack_860 = 0x141e36bac;
  uVar12 = FUN_1406e8c20(param_3);
  *(undefined4 *)(param_1 + 0xb5) = uVar12;
  uStack_860 = 0x141e36bbc;
  uVar12 = FUN_1406e8c20(param_3);
  *(undefined4 *)((longlong)param_1 + 0x5ac) = uVar12;
  uStack_860 = 0x141e36bd9;
  FUN_1409d4040(param_1 + 0x78,param_1 + 0x7e);
  uStack_860 = 0x141e36be1;
  cVar8 = FUN_1406e8ae0(param_3);
  *(uint *)((longlong)param_1 + 0x1ac) = (uint)(cVar8 != '\0');
  uStack_860 = 0x141e36bf9;
  bVar9 = FUN_1406e8ae0(param_3);
  *(uint *)(param_1 + 0x34) = (uint)bVar9;
  lVar21 = DAT_143ac18d8;
  uStack_860 = 0x141e36c13;
  sVar11 = FUN_1406e8b80(param_3);
  uStack_860 = 0x141e36c1e;
  local_650 = (longlong *)FUN_142df6c50(lVar21,(int)sVar11);
  uStack_860 = 0x141e36c35;
  sVar11 = FUN_1406e8b80(param_3);
  *(int *)((longlong)param_1 + 0x174) = (int)sVar11;
  uStack_860 = 0x141e36c42;
  sVar11 = FUN_1406e8b80(param_3);
  *(int *)(param_1 + 0x2f) = (int)sVar11;
  uStack_860 = 0x141e36c55;
  sVar11 = FUN_1406e8b80(param_3);
  *(int *)((longlong)param_1 + 0x17c) = (int)sVar11;
  uStack_860 = 0x141e36c68;
  sVar11 = FUN_1406e8b80(param_3);
  *(int *)(param_1 + 0x30) = (int)sVar11;
  if ((param_1[0x33] == 0) || (*(int *)(param_1[0x33] + 0x2c) == 0)) {
    uVar10 = 0;
  }
  else {
    uVar10 = 1;
  }
  *(undefined1 *)((longlong)param_1 + 0x1a9) = uVar10;
  uStack_860 = 0x141e36c99;
  plVar18 = (longlong *)FUN_142b56090();
  plVar30 = plVar32;
  if (plVar18 == (longlong *)0x0) {
    iVar13 = -0x7fffbffe;
  }
  else {
    local_570 = (longlong *)0x0;
    uStack_860 = 0x141e36cc3;
    iVar13 = (**(code **)plVar18[4])(plVar18 + 4,&DAT_143273488,&local_570);
    if (-1 < iVar13) {
      plVar30 = local_570;
    }
  }
  if (((iVar13 + 0x80000000U & 0x80000000) == 0) && (iVar13 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    uStack_860 = 0x141e396e1;
    FUN_142ef3ac0();
  }
  plVar27 = (longlong *)param_1[0x2c];
  if ((plVar27 != plVar30) &&
     (param_1[0x2c] = (longlong)plVar30, plVar30 = plVar32, plVar27 != (longlong *)0x0)) {
    uStack_860 = 0x141e36d17;
    (**(code **)(*plVar27 + 0x10))();
  }
  if (plVar30 != (longlong *)0x0) {
    uStack_860 = 0x141e36d26;
    (**(code **)(*plVar30 + 0x10))(plVar30);
  }
  lVar21 = param_1[0x33];
  plVar30 = plVar32;
  if (lVar21 != 0) {
    if ((*(int *)(lVar21 + 0x28) != 0) &&
       (plVar30 = (longlong *)0x0, *(int *)((longlong)param_1 + 0x1ac) != 0)) {
      plVar30 = (longlong *)0x1;
    }
    if (0 < (int)*(uint *)(lVar21 + 0x30)) {
      plVar30 = (longlong *)(ulonglong)*(uint *)(lVar21 + 0x30);
    }
  }
  local_830 = (undefined8 *)((longlong)param_1 + 0x17c);
  uStack_860 = 0x141e36d7d;
  local_838 = (int *)((longlong)param_1 + 0x174);
  FUN_142b53fc0(plVar18,param_1 + 1,plVar30,*(undefined4 *)(lVar21 + 0x34));
  pcVar2 = *(code **)(*plVar18 + 0x118);
  lVar21 = param_1[0x34];
  uStack_860 = 0x141e36d9c;
  uVar12 = FUN_14019a5d0(param_1 + 0x78);
  uStack_860 = 0x141e36dab;
  uVar14 = FUN_14019a5d0(param_1 + 0x7b);
  local_820 = local_650;
  local_828 = (uint *)CONCAT44(local_828._4_4_,(int)lVar21);
  local_830 = (undefined8 *)((ulonglong)local_830 & 0xffffffff00000000);
  local_838 = (int *)((ulonglong)local_838 & 0xffffffff00000000);
  uStack_860 = 0x141e36dd2;
  (*pcVar2)(plVar18,0,uVar14,uVar12);
  uStack_860 = 0x141e36dda;
  uVar12 = FUN_1409c6d00(plVar18);
  plVar18 = local_640;
  *(undefined4 *)(param_1 + 0x34) = uVar12;
  uStack_860 = 0x141e36df1;
  bVar9 = FUN_1406e8ae0(local_640);
  *(uint *)(param_1 + 0x4e) = (uint)bVar9;
  if (*(int *)(param_1[0x33] + 0x38) != 0) {
    local_7b0 = (IUnknown *)0x0;
    local_7b8 = (IUnknown *)0x0;
    local_648 = (IUnknown *)0x0;
    if (DAT_143ad48a0 == (code *)0x0) {
      iVar13 = -0x7ffbfe10;
LAB_141e395ef:
      uStack_860 = 0x141e39603;
      FUN_1401a59c0(&local_418,iVar13,0,0);
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39616;
      _CxxThrowException(&local_418,(ThrowInfo *)&DAT_143a3b0c0);
    }
    uStack_860 = 0x141e36e4f;
    iVar13 = (*DAT_143ad48a0)(PTR_u_Shape2D_Vector2D_143a479e8,&DAT_143273488,&local_648,0);
    pIVar4 = local_7b0;
    puVar6 = PTR_u_Shape2D_Vector2D_143a479e8;
    if (iVar13 < 0) goto LAB_141e395ef;
    if ((local_7b0 != (IUnknown *)0x0) && (local_7b0 = (IUnknown *)0x0, pIVar4 != (IUnknown *)0x0))
    {
      uStack_860 = 0x141e36e7f;
      (**(code **)(*(longlong *)pIVar4 + 0x10))(pIVar4);
    }
    if (local_7b0 != (IUnknown *)0x0) {
      uStack_860 = 0x141e36e8e;
      (**(code **)(*(longlong *)local_7b0 + 0x10))();
    }
    local_7b0 = (IUnknown *)0x0;
    if (DAT_143ad48a0 == (code *)0x0) {
      iVar13 = -0x7ffbfe10;
LAB_141e3961c:
      uStack_860 = 0x141e39630;
      FUN_1401a59c0(&local_418,iVar13,0,0);
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39643;
      _CxxThrowException(&local_418,(ThrowInfo *)&DAT_143a3b0c0);
    }
    uStack_860 = 0x141e36eb6;
    iVar13 = (*DAT_143ad48a0)(puVar6,&DAT_143273488,&local_7b0,0);
    pIVar4 = local_7b8;
    puVar6 = PTR_u_Shape2D_Vector2D_143a479e8;
    if (iVar13 < 0) goto LAB_141e3961c;
    if ((local_7b8 != (IUnknown *)0x0) && (local_7b8 = (IUnknown *)0x0, pIVar4 != (IUnknown *)0x0))
    {
      uStack_860 = 0x141e36ee6;
      (**(code **)(*(longlong *)pIVar4 + 0x10))(pIVar4);
    }
    if (local_7b8 != (IUnknown *)0x0) {
      uStack_860 = 0x141e36ef5;
      (**(code **)(*(longlong *)local_7b8 + 0x10))();
    }
    local_7b8 = (IUnknown *)0x0;
    if (DAT_143ad48a0 == (code *)0x0) {
      iVar13 = -0x7ffbfe10;
LAB_141e39649:
      uStack_860 = 0x141e3965d;
      FUN_1401a59c0(&local_418,iVar13,0,0);
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39670;
      _CxxThrowException(&local_418,(ThrowInfo *)&DAT_143a3b0c0);
    }
    uStack_860 = 0x141e36f1d;
    iVar13 = (*DAT_143ad48a0)(puVar6,&DAT_143273488,&local_7b8,0);
    pIVar4 = local_648;
    if (iVar13 < 0) goto LAB_141e39649;
    if (local_648 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39718;
      FUN_142ef3ac0(0x80004003);
    }
    local_550 = CONCAT22(local_550._2_2_,3);
    uStack_548 = 2000;
    local_1f8 = local_550;
    uStack_1f4 = uStack_54c;
    uStack_1f0 = 2000;
    uStack_1ec = uStack_544;
    local_1e8 = local_540;
    uStack_860 = 0x141e36f7a;
    iVar13 = (**(code **)(*(longlong *)local_648 + 0x160))(local_648,0,&local_1f8);
    if (iVar13 < 0) {
      uStack_860 = 0x141e36f8f;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_143273488);
    }
    if ((short)local_550 == 8) {
      local_550 = local_550 & 0xffff0000;
      if (CONCAT44(uStack_544,uStack_548) != 0) {
        uStack_860 = 0x141e36fb8;
        (*DAT_143ad5990)(CONCAT44(uStack_544,uStack_548) + -4);
      }
    }
    else {
      uStack_860 = 0x141e36fc7;
      (*DAT_143262a18)(&local_550);
    }
    pIVar4 = local_7b0;
    if (local_7b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e3970d;
      FUN_142ef3ac0(0x80004003);
    }
    local_538 = CONCAT22(local_538._2_2_,0xd);
    uStack_530 = local_648;
    if (local_648 != (IUnknown *)0x0) {
      uStack_860 = 0x141e36ffa;
      (**(code **)(*(longlong *)local_648 + 8))();
    }
    local_378 = local_538;
    uStack_374 = uStack_534;
    uStack_370 = (undefined4)uStack_530;
    uStack_36c = uStack_530._4_4_;
    local_368 = local_528;
    uStack_860 = 0x141e3702c;
    iVar13 = (**(code **)(*(longlong *)pIVar4 + 200))(pIVar4,&local_378);
    if (iVar13 < 0) {
      uStack_860 = 0x141e37041;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_143273488);
    }
    if ((short)local_538 == 8) {
      local_538 = local_538 & 0xffff0000;
      if (uStack_530 != (IUnknown *)0x0) {
        uStack_860 = 0x141e3706a;
        (*DAT_143ad5990)(uStack_530 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37079;
      (*DAT_143262a18)(&local_538);
    }
    pIVar4 = local_7b0;
    if (local_7b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39702;
      FUN_142ef3ac0(0x80004003);
    }
    uStack_860 = 0x141e37094;
    (*DAT_143262a20)(&local_6b0);
    if (DAT_143a8b8d8 == 8) {
      if (local_6b0 == 8) {
        local_6b0 = 0;
        if (lStack_6a8 != 0) {
          uStack_860 = 0x141e370ca;
          (*DAT_143ad5990)(lStack_6a8 + -4);
        }
      }
      else {
        uStack_860 = 0x141e370d9;
        iVar13 = (*DAT_143262a18)(&local_6b0);
        if (iVar13 < 0) goto LAB_141e39671;
      }
      local_6b0 = 8;
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e37103;
      lStack_6a8 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if ((local_6b0 == 8) && (local_6b0 = 0, lStack_6a8 != 0)) {
        uStack_860 = 0x141e37177;
        (*DAT_143ad5990)(lStack_6a8 + -4);
      }
      uStack_860 = 0x141e3718b;
      iVar13 = (*DAT_143262a28)(&local_6b0,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e39671:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e39678;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e37117;
    (*DAT_143262a20)(&local_6c8);
    if (DAT_143a8b8d8 == 8) {
      if (local_6c8 == 8) {
        local_6c8 = 0;
        if (lStack_6c0 != 0) {
          uStack_860 = 0x141e3714d;
          (*DAT_143ad5990)(lStack_6c0 + -4);
        }
      }
      else {
        uStack_860 = 0x141e371a5;
        iVar13 = (*DAT_143262a18)(&local_6c8);
        if (iVar13 < 0) goto LAB_141e39679;
      }
      local_6c8 = 8;
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e371cf;
      lStack_6c0 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if ((local_6c8 == 8) && (local_6c8 = 0, lStack_6c0 != 0)) {
        uStack_860 = 0x141e37237;
        (*DAT_143ad5990)(lStack_6c0 + -4);
      }
      uStack_860 = 0x141e3724b;
      iVar13 = (*DAT_143262a28)(&local_6c8,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e39679:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e39680;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e371e0;
    (*DAT_143262a20)(&local_758);
    if (DAT_143a8b8d8 == 8) {
      if (local_758 == 8) {
        local_758 = 0;
        if (lStack_750 != 0) {
          uStack_860 = 0x141e3720d;
          (*DAT_143ad5990)(lStack_750 + -4);
        }
      }
      else {
        uStack_860 = 0x141e3725f;
        iVar13 = (*DAT_143262a18)(&local_758);
        if (iVar13 < 0) goto LAB_141e39681;
      }
      local_758 = 8;
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e37286;
      lStack_750 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if ((local_758 == 8) && (local_758 = 0, lStack_750 != 0)) {
        uStack_860 = 0x141e372ee;
        (*DAT_143ad5990)(lStack_750 + -4);
      }
      uStack_860 = 0x141e372ff;
      iVar13 = (*DAT_143262a28)(&local_758,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e39681:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e39688;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e37297;
    (*DAT_143262a20)(&local_6f8);
    if (DAT_143a8b8d8 == 8) {
      if (local_6f8 == 8) {
        local_6f8 = 0;
        if (lStack_6f0 != 0) {
          uStack_860 = 0x141e372cd;
          (*DAT_143ad5990)(lStack_6f0 + -4);
        }
      }
      else {
        uStack_860 = 0x141e37316;
        iVar13 = (*DAT_143262a18)(&local_6f8);
        if (iVar13 < 0) goto LAB_141e39689;
      }
      local_6f8 = 8;
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e37340;
      lStack_6f0 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if ((local_6f8 == 8) && (local_6f8 = 0, lStack_6f0 != 0)) {
        uStack_860 = 0x141e373b4;
        (*DAT_143ad5990)(lStack_6f0 + -4);
      }
      uStack_860 = 0x141e373c8;
      iVar13 = (*DAT_143262a28)(&local_6f8,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e39689:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e39690;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e37354;
    (*DAT_143262a20)(&local_6e0);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_6e0 == 8) {
        local_6e0 = (uint)local_6e0._2_2_ << 0x10;
        if (uStack_6d8 != 0) {
          uStack_860 = 0x141e3738a;
          (*DAT_143ad5990)(uStack_6d8 + -4);
        }
      }
      else {
        uStack_860 = 0x141e373e2;
        iVar13 = (*DAT_143262a18)(&local_6e0);
        if (iVar13 < 0) goto LAB_141e39691;
      }
      local_6e0 = CONCAT22(local_6e0._2_2_,8);
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e3740c;
      uStack_6d8 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if (((short)local_6e0 == 8) && (local_6e0 = (uint)local_6e0._2_2_ << 0x10, uStack_6d8 != 0)) {
        uStack_860 = 0x141e37555;
        (*DAT_143ad5990)(uStack_6d8 + -4);
      }
      uStack_860 = 0x141e37569;
      iVar13 = (*DAT_143262a28)(&local_6e0,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e39691:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e39698;
        FUN_142ef3ac0(iVar13);
      }
    }
    local_358 = CONCAT62(uStack_6ae,local_6b0);
    lStack_350 = lStack_6a8;
    local_348 = local_6a0;
    local_338 = CONCAT62(uStack_6c6,local_6c8);
    lStack_330 = lStack_6c0;
    local_328 = local_6b8;
    local_318 = CONCAT62(uStack_756,local_758);
    lStack_310 = lStack_750;
    local_308 = local_748;
    local_2f8 = CONCAT62(uStack_6f6,local_6f8);
    lStack_2f0 = lStack_6f0;
    local_2e8 = local_6e8;
    local_2d8 = local_6e0;
    uStack_2d4 = uStack_6dc;
    uStack_2d0 = (undefined4)uStack_6d8;
    uStack_2cc = uStack_6d8._4_4_;
    local_2c8 = local_6d0;
    local_820 = &local_358;
    local_828 = (uint *)&local_338;
    local_830 = &local_318;
    local_838 = (int *)&local_2f8;
    uStack_860 = 0x141e374ed;
    iVar13 = (**(code **)(*(longlong *)pIVar4 + 0x140))(pIVar4,5,0,&local_2d8);
    if (iVar13 < 0) {
      uStack_860 = 0x141e37502;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_143273488);
    }
    if ((short)local_6e0 == 8) {
      local_6e0 = local_6e0 & 0xffff0000;
      if (uStack_6d8 != 0) {
        uStack_860 = 0x141e3752b;
        (*DAT_143ad5990)(uStack_6d8 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37583;
      (*DAT_143262a18)(&local_6e0);
    }
    if (local_6f8 == 8) {
      local_6f8 = 0;
      if (lStack_6f0 != 0) {
        uStack_860 = 0x141e375ac;
        (*DAT_143ad5990)(lStack_6f0 + -4);
      }
    }
    else {
      uStack_860 = 0x141e375bb;
      (*DAT_143262a18)(&local_6f8);
    }
    if (local_758 == 8) {
      local_758 = 0;
      if (lStack_750 != 0) {
        uStack_860 = 0x141e375db;
        (*DAT_143ad5990)(lStack_750 + -4);
      }
    }
    else {
      uStack_860 = 0x141e375e7;
      (*DAT_143262a18)(&local_758);
    }
    if (local_6c8 == 8) {
      local_6c8 = 0;
      if (lStack_6c0 != 0) {
        uStack_860 = 0x141e37610;
        (*DAT_143ad5990)(lStack_6c0 + -4);
      }
    }
    else {
      uStack_860 = 0x141e3761f;
      (*DAT_143262a18)(&local_6c8);
    }
    if (local_6b0 == 8) {
      local_6b0 = 0;
      if (lStack_6a8 != 0) {
        uStack_860 = 0x141e37648;
        (*DAT_143ad5990)(lStack_6a8 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37657;
      (*DAT_143262a18)(&local_6b0);
    }
    pIVar4 = local_7b8;
    if (local_7b8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e396f7;
      FUN_142ef3ac0(0x80004003);
    }
    local_830 = (undefined8 *)CONCAT44(local_830._4_4_,100);
    local_838 = (int *)((ulonglong)local_838 & 0xffffffff00000000);
    uStack_860 = 0x141e3768b;
    iVar13 = (**(code **)(*(longlong *)local_7b8 + 0x150))(local_7b8,local_7b0,100);
    if (iVar13 < 0) {
      uStack_860 = 0x141e376a0;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_143273488);
    }
    pIVar4 = local_7b8;
    if (local_7b8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e396ec;
      FUN_142ef3ac0(0x80004003);
    }
    uStack_860 = 0x141e376b7;
    (*DAT_143262a20)(&local_788);
    if (DAT_143a8b8d8 == 8) {
      if (local_788 == 8) {
        local_788 = 0;
        if (lStack_780 != 0) {
          uStack_860 = 0x141e376e4;
          (*DAT_143ad5990)(lStack_780 + -4);
        }
      }
      else {
        uStack_860 = 0x141e376f0;
        iVar13 = (*DAT_143262a18)(&local_788);
        if (iVar13 < 0) goto LAB_141e39699;
      }
      local_788 = 8;
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e37717;
      lStack_780 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if ((local_788 == 8) && (local_788 = 0, lStack_780 != 0)) {
        uStack_860 = 0x141e3777f;
        (*DAT_143ad5990)(lStack_780 + -4);
      }
      uStack_860 = 0x141e37790;
      iVar13 = (*DAT_143262a28)(&local_788,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e39699:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e396a0;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e37728;
    (*DAT_143262a20)(&local_668);
    if (DAT_143a8b8d8 == 8) {
      if (local_668 == 8) {
        local_668 = 0;
        if (lStack_660 != 0) {
          uStack_860 = 0x141e3775e;
          (*DAT_143ad5990)(lStack_660 + -4);
        }
      }
      else {
        uStack_860 = 0x141e377a7;
        iVar13 = (*DAT_143262a18)(&local_668);
        if (iVar13 < 0) goto LAB_141e396a1;
      }
      local_668 = 8;
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e377d1;
      lStack_660 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if ((local_668 == 8) && (local_668 = 0, lStack_660 != 0)) {
        uStack_860 = 0x141e37845;
        (*DAT_143ad5990)(lStack_660 + -4);
      }
      uStack_860 = 0x141e37859;
      iVar13 = (*DAT_143262a28)(&local_668,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e396a1:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e396a8;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e377e5;
    (*DAT_143262a20)(&local_728);
    if (DAT_143a8b8d8 == 8) {
      if (local_728 == 8) {
        local_728 = 0;
        if (lStack_720 != 0) {
          uStack_860 = 0x141e3781b;
          (*DAT_143ad5990)(lStack_720 + -4);
        }
      }
      else {
        uStack_860 = 0x141e37873;
        iVar13 = (*DAT_143262a18)(&local_728);
        if (iVar13 < 0) goto LAB_141e396a9;
      }
      local_728 = 8;
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e3789d;
      lStack_720 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if ((local_728 == 8) && (local_728 = 0, lStack_720 != 0)) {
        uStack_860 = 0x141e37911;
        (*DAT_143ad5990)(lStack_720 + -4);
      }
      uStack_860 = 0x141e37925;
      iVar13 = (*DAT_143262a28)(&local_728,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e396a9:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e396b0;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e378b1;
    (*DAT_143262a20)(&local_680);
    if (DAT_143a8b8d8 == 8) {
      if (local_680 == 8) {
        local_680 = 0;
        if (lStack_678 != 0) {
          uStack_860 = 0x141e378e7;
          (*DAT_143ad5990)(lStack_678 + -4);
        }
      }
      else {
        uStack_860 = 0x141e3793f;
        iVar13 = (*DAT_143262a18)(&local_680);
        if (iVar13 < 0) goto LAB_141e396b1;
      }
      local_680 = 8;
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e37969;
      lStack_678 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if ((local_680 == 8) && (local_680 = 0, lStack_678 != 0)) {
        uStack_860 = 0x141e379dd;
        (*DAT_143ad5990)(lStack_678 + -4);
      }
      uStack_860 = 0x141e379f1;
      iVar13 = (*DAT_143262a28)(&local_680,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e396b1:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e396b8;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e3797d;
    (*DAT_143262a20)(&local_698);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_698 == 8) {
        local_698 = (uint)local_698._2_2_ << 0x10;
        if (uStack_690 != 0) {
          uStack_860 = 0x141e379b3;
          (*DAT_143ad5990)(uStack_690 + -4);
        }
      }
      else {
        uStack_860 = 0x141e37a0b;
        iVar13 = (*DAT_143262a18)(&local_698);
        if (iVar13 < 0) goto LAB_141e396b9;
      }
      local_698 = CONCAT22(local_698._2_2_,8);
      plVar27 = plVar32;
      if (DAT_143a8b8e0 != 0) {
        plVar27 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      uStack_860 = 0x141e37a35;
      uStack_690 = FUN_1401a5fa0(DAT_143a8b8e0,plVar27);
    }
    else {
      if (((short)local_698 == 8) && (local_698 = (uint)local_698._2_2_ << 0x10, uStack_690 != 0)) {
        uStack_860 = 0x141e37b9c;
        (*DAT_143ad5990)(uStack_690 + -4);
      }
      uStack_860 = 0x141e37bb0;
      iVar13 = (*DAT_143262a28)(&local_698,&DAT_143a8b8d8);
      if (iVar13 < 0) {
LAB_141e396b9:
                    /* WARNING: Subroutine does not return */
        uStack_860 = 0x141e396c0;
        FUN_142ef3ac0(iVar13);
      }
    }
    uStack_860 = 0x141e37a49;
    uVar12 = FUN_14019a5d0(param_1 + 0x78);
    uStack_860 = 0x141e37a58;
    uVar14 = FUN_14019a5d0(param_1 + 0x7b);
    local_2b8 = CONCAT62(uStack_786,local_788);
    lStack_2b0 = lStack_780;
    local_2a8 = local_778;
    local_298 = CONCAT62(uStack_666,local_668);
    lStack_290 = lStack_660;
    local_288 = local_658;
    local_278 = CONCAT62(uStack_726,local_728);
    lStack_270 = lStack_720;
    local_268 = local_718;
    local_258 = CONCAT62(uStack_67e,local_680);
    lStack_250 = lStack_678;
    local_248 = local_670;
    local_238 = local_698;
    uStack_234 = uStack_694;
    uStack_230 = (undefined4)uStack_690;
    uStack_22c = uStack_690._4_4_;
    local_228 = local_688;
    local_820 = &local_2b8;
    local_828 = (uint *)&local_298;
    local_830 = &local_278;
    local_838 = (int *)&local_258;
    uStack_860 = 0x141e37b34;
    iVar13 = (**(code **)(*(longlong *)pIVar4 + 0x140))(pIVar4,uVar14,uVar12,&local_238);
    if (iVar13 < 0) {
      uStack_860 = 0x141e37b49;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_143273488);
    }
    if ((short)local_698 == 8) {
      local_698 = local_698 & 0xffff0000;
      if (uStack_690 != 0) {
        uStack_860 = 0x141e37b72;
        (*DAT_143ad5990)(uStack_690 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37bca;
      (*DAT_143262a18)(&local_698);
    }
    if (local_680 == 8) {
      local_680 = 0;
      if (lStack_678 != 0) {
        uStack_860 = 0x141e37bf3;
        (*DAT_143ad5990)(lStack_678 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37c02;
      (*DAT_143262a18)(&local_680);
    }
    if (local_728 == 8) {
      local_728 = 0;
      if (lStack_720 != 0) {
        uStack_860 = 0x141e37c2b;
        (*DAT_143ad5990)(lStack_720 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37c3a;
      (*DAT_143262a18)(&local_728);
    }
    if (local_668 == 8) {
      local_668 = 0;
      if (lStack_660 != 0) {
        uStack_860 = 0x141e37c63;
        (*DAT_143ad5990)(lStack_660 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37c72;
      (*DAT_143262a18)(&local_668);
    }
    if (local_788 == 8) {
      local_788 = 0;
      if (lStack_780 != 0) {
        uStack_860 = 0x141e37c92;
        (*DAT_143ad5990)(lStack_780 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37c9e;
      (*DAT_143262a18)(&local_788);
    }
    uStack_860 = 0x141e37cad;
    uVar19 = (**(code **)(param_1[1] + 0x50))(param_1 + 1);
    local_578 = local_7b8;
    if (local_7b8 != (IUnknown *)0x0) {
      uStack_860 = 0x141e37cc6;
      (**(code **)(*(longlong *)local_7b8 + 8))();
    }
    uStack_860 = 0x141e37cd6;
    FUN_1409c6db0(uVar19,&local_578);
    if (local_7b8 != (IUnknown *)0x0) {
      uStack_860 = 0x141e37ce6;
      (**(code **)(*(longlong *)local_7b8 + 0x10))();
    }
    if (local_7b0 != (IUnknown *)0x0) {
      uStack_860 = 0x141e37cf6;
      (**(code **)(*(longlong *)local_7b0 + 0x10))();
    }
    if (local_648 != (IUnknown *)0x0) {
      uStack_860 = 0x141e37d09;
      (**(code **)(*(longlong *)local_648 + 0x10))();
    }
  }
  uStack_860 = 0x141e37d16;
  plVar20 = (longlong *)FUN_142b56090();
  plVar27 = plVar32;
  if (plVar20 == (longlong *)0x0) {
    iVar13 = -0x7fffbffe;
  }
  else {
    local_580 = (longlong *)0x0;
    uStack_860 = 0x141e37d40;
    iVar13 = (**(code **)plVar20[4])(plVar20 + 4,&DAT_143273488,&local_580);
    if (-1 < iVar13) {
      plVar27 = local_580;
    }
  }
  if (((iVar13 + 0x80000000U & 0x80000000) == 0) && (iVar13 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    uStack_860 = 0x141e3971e;
    FUN_142ef3ac0();
  }
  plVar3 = (longlong *)param_1[0x2d];
  if ((plVar3 != plVar27) &&
     (param_1[0x2d] = (longlong)plVar27, plVar27 = plVar32, plVar3 != (longlong *)0x0)) {
    uStack_860 = 0x141e37d94;
    (**(code **)(*plVar3 + 0x10))();
  }
  if (plVar27 != (longlong *)0x0) {
    uStack_860 = 0x141e37da3;
    (**(code **)(*plVar27 + 0x10))(plVar27);
  }
  local_830 = (undefined8 *)((longlong)param_1 + 0x17c);
  local_838 = (int *)((longlong)param_1 + 0x174);
  uStack_860 = 0x141e37dd8;
  FUN_142b53fc0(plVar20,param_1 + 1,plVar30,*(undefined4 *)(param_1[0x33] + 0x34));
  local_820 = (longlong *)0x0;
  local_828 = (uint *)((ulonglong)local_828 & 0xffffffff00000000);
  local_830 = (undefined8 *)((ulonglong)local_830 & 0xffffffff00000000);
  local_838 = (int *)((ulonglong)local_838 & 0xffffffff00000000);
  uStack_860 = 0x141e37e00;
  (**(code **)(*plVar20 + 0x118))(plVar20,0,0,0);
  pIVar4 = DAT_143add050;
  if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    uStack_860 = 0x141e396db;
    FUN_142ef3ac0(0x80004003);
  }
  uStack_860 = 0x141e37e1d;
  (*DAT_143262a20)(&local_710);
  if (DAT_143a8b8d8 == 8) {
    if (local_710 == 8) {
      local_710 = 0;
      if (lStack_708 != 0) {
        uStack_860 = 0x141e37e4f;
        (*DAT_143ad5990)(lStack_708 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37e5e;
      iVar13 = (*DAT_143262a18)(&local_710);
      if (iVar13 < 0) goto LAB_141e3971f;
    }
    local_710 = 8;
    if (DAT_143a8b8e0 == 0) {
      uStack_860 = 0x141e37e86;
      lStack_708 = FUN_1401a5fa0(0,0);
    }
    else {
      uStack_860 = 0x141e37e99;
      lStack_708 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
  }
  else {
    if ((local_710 == 8) && (local_710 = 0, lStack_708 != 0)) {
      uStack_860 = 0x141e37eca;
      (*DAT_143ad5990)(lStack_708 + -4);
    }
    uStack_860 = 0x141e37ede;
    iVar13 = (*DAT_143262a28)(&local_710,&DAT_143a8b8d8);
    if (iVar13 < 0) {
LAB_141e3971f:
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39726;
      FUN_142ef3ac0(iVar13);
    }
  }
  uStack_860 = 0x141e37ef5;
  (*DAT_143262a20)(&local_740);
  if (DAT_143a8b8d8 == 8) {
    if (local_740 == 8) {
      local_740 = 0;
      if (lStack_738 != 0) {
        uStack_860 = 0x141e37f25;
        (*DAT_143ad5990)(lStack_738 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37f31;
      iVar13 = (*DAT_143262a18)(&local_740);
      if (iVar13 < 0) goto LAB_141e396c1;
    }
    local_740 = 8;
    plVar30 = plVar32;
    if (DAT_143a8b8e0 != 0) {
      plVar30 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    uStack_860 = 0x141e37f58;
    lStack_738 = FUN_1401a5fa0(DAT_143a8b8e0,plVar30);
  }
  else {
    if ((local_740 == 8) && (local_740 = 0, lStack_738 != 0)) {
      uStack_860 = 0x141e37fba;
      (*DAT_143ad5990)(lStack_738 + -4);
    }
    uStack_860 = 0x141e37fcb;
    iVar13 = (*DAT_143262a28)(&local_740,&DAT_143a8b8d8);
    if (iVar13 < 0) {
LAB_141e396c1:
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e396c8;
      FUN_142ef3ac0(iVar13);
    }
  }
  uStack_860 = 0x141e37f69;
  (*DAT_143262a20)(&local_770);
  if (DAT_143a8b8d8 == 8) {
    if (local_770 == 8) {
      local_770 = 0;
      if (lStack_768 != 0) {
        uStack_860 = 0x141e37f96;
        (*DAT_143ad5990)(lStack_768 + -4);
      }
    }
    else {
      uStack_860 = 0x141e37fdf;
      iVar13 = (*DAT_143262a18)(&local_770);
      if (iVar13 < 0) goto LAB_141e396c9;
    }
    local_770 = 8;
    plVar30 = plVar32;
    if (DAT_143a8b8e0 != 0) {
      plVar30 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    uStack_860 = 0x141e38006;
    lStack_768 = FUN_1401a5fa0(DAT_143a8b8e0,plVar30);
  }
  else {
    if ((local_770 == 8) && (local_770 = 0, lStack_768 != 0)) {
      uStack_860 = 0x141e3818d;
      (*DAT_143ad5990)(lStack_768 + -4);
    }
    uStack_860 = 0x141e3819e;
    iVar13 = (*DAT_143262a28)(&local_770,&DAT_143a8b8d8);
    if (iVar13 < 0) {
LAB_141e396c9:
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e396d0;
      FUN_142ef3ac0(iVar13);
    }
  }
  local_568 = CONCAT22(local_568._2_2_,3);
  uStack_560 = 0;
  local_590 = (longlong *)0x0;
  local_218 = CONCAT62(uStack_70e,local_710);
  lStack_210 = lStack_708;
  local_208 = local_700;
  local_58 = CONCAT62(uStack_73e,local_740);
  lStack_50 = lStack_738;
  local_48 = local_730;
  local_1d8 = CONCAT62(uStack_76e,local_770);
  lStack_1d0 = lStack_768;
  local_1c8 = local_760;
  local_1b8 = local_568;
  uStack_1b4 = uStack_564;
  uStack_1b0 = 0;
  uStack_1ac = uStack_55c;
  local_1a8 = local_558;
  local_808 = &local_590;
  local_810 = &local_218;
  local_818 = &local_58;
  local_820 = &local_1d8;
  local_828 = &local_1b8;
  local_830 = (undefined8 *)((ulonglong)local_830 & 0xffffffff00000000);
  local_838 = (int *)((ulonglong)local_838 & 0xffffffff00000000);
  uStack_860 = 0x141e380ed;
  iVar13 = (**(code **)(*(longlong *)pIVar4 + 0x168))(pIVar4,0,0,0);
  if (iVar13 < 0) {
    uStack_860 = 0x141e38102;
    _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_14327fcd0);
  }
  uVar29 = 2;
  local_798 = 2;
  plVar30 = param_1 + 0x47;
  plVar27 = (longlong *)*plVar30;
  plVar20 = local_590;
  if (plVar27 != local_590) {
    *plVar30 = (longlong)local_590;
    plVar20 = plVar32;
    if (plVar27 != (longlong *)0x0) {
      uStack_860 = 0x141e38134;
      (**(code **)(*plVar27 + 0x10))();
      plVar20 = (longlong *)0x0;
    }
  }
  if (plVar20 != (longlong *)0x0) {
    uStack_860 = 0x141e38143;
    (**(code **)(*plVar20 + 0x10))(plVar20);
  }
  if ((short)local_568 == 8) {
    local_568 = local_568 & 0xffff0000;
    if (CONCAT44(uStack_55c,uStack_560) != 0) {
      uStack_860 = 0x141e3816c;
      (*DAT_143ad5990)(CONCAT44(uStack_55c,uStack_560) + -4);
    }
  }
  else {
    uStack_860 = 0x141e381b8;
    (*DAT_143262a18)(&local_568);
  }
  if (local_770 == 8) {
    local_770 = 0;
    if (lStack_768 != 0) {
      uStack_860 = 0x141e381d8;
      (*DAT_143ad5990)(lStack_768 + -4);
    }
  }
  else {
    uStack_860 = 0x141e381e4;
    (*DAT_143262a18)(&local_770);
  }
  if (local_740 == 8) {
    local_740 = 0;
    if (lStack_738 != 0) {
      uStack_860 = 0x141e38207;
      (*DAT_143ad5990)(lStack_738 + -4);
    }
  }
  else {
    uStack_860 = 0x141e38213;
    (*DAT_143262a18)(&local_740);
  }
  if (local_710 == 8) {
    local_710 = 0;
    if (lStack_708 != 0) {
      uStack_860 = 0x141e3823c;
      (*DAT_143ad5990)(lStack_708 + -4);
    }
  }
  else {
    uStack_860 = 0x141e3824b;
    (*DAT_143262a18)(&local_710);
  }
  pIVar4 = (IUnknown *)*plVar30;
  if (pIVar4 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    uStack_860 = 0x141e395e9;
    FUN_142ef3ac0(0x80004003);
  }
  uStack_518 = (longlong *)param_1[0x2c];
  local_520 = CONCAT22(local_520._2_2_,0xd);
  if (uStack_518 != (longlong *)0x0) {
    uStack_860 = 0x141e3827f;
    (**(code **)(*uStack_518 + 8))();
  }
  local_198 = local_520;
  uStack_194 = uStack_51c;
  uStack_190 = (undefined4)uStack_518;
  uStack_18c = uStack_518._4_4_;
  local_188 = local_510;
  uStack_860 = 0x141e382b1;
  iVar13 = (**(code **)(*(longlong *)pIVar4 + 200))(pIVar4,&local_198);
  if (iVar13 < 0) {
    uStack_860 = 0x141e382c6;
    _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_143273488);
  }
  if ((short)local_520 == 8) {
    local_520 = local_520 & 0xffff0000;
    if (uStack_518 != (longlong *)0x0) {
      uStack_860 = 0x141e382ef;
      (*DAT_143ad5990)((longlong)uStack_518 + -4);
    }
  }
  else {
    uStack_860 = 0x141e382fe;
    (*DAT_143262a18)(&local_520);
  }
  pIVar4 = (IUnknown *)*plVar30;
  if (pIVar4 == (IUnknown *)0x0) {
LAB_141e39788:
                    /* WARNING: Subroutine does not return */
    uStack_860 = 0x141e39792;
    FUN_142ef3ac0(0x80004003);
  }
  uStack_860 = 0x141e3831d;
  iVar13 = (**(code **)(*(longlong *)pIVar4 + 0x200))(pIVar4,0xffffffff);
  if (iVar13 < 0) {
    uStack_860 = 0x141e38332;
    _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_14327fcb0);
  }
  pIVar4 = DAT_143add050;
  if (*(int *)(param_1[0x33] + 0x1dc) == 0) {
    if ((longlong *)param_1[0x4d] != (longlong *)0x0) {
      uStack_860 = 0x141e389e6;
      (**(code **)(*(longlong *)param_1[0x4d] + 0x10))();
    }
    param_1[0x4d] = 0;
  }
  else {
    if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39787;
      FUN_142ef3ac0(0x80004003);
    }
    uStack_860 = 0x141e38364;
    (*DAT_143262a20)(local_4c0);
    uStack_860 = 0x141e38377;
    iVar13 = FUN_14023c4c0(local_4c0,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e3972e;
      FUN_142ef3ac0(iVar13);
    }
    uStack_860 = 0x141e3838c;
    (*DAT_143262a20)(local_4d8);
    uStack_860 = 0x141e3839f;
    iVar13 = FUN_14023c4c0(local_4d8,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39736;
      FUN_142ef3ac0(iVar13);
    }
    uStack_860 = 0x141e383b4;
    (*DAT_143262a20)(local_4f0);
    uStack_860 = 0x141e383c7;
    iVar13 = FUN_14023c4c0(local_4f0,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e3973e;
      FUN_142ef3ac0(iVar13);
    }
    local_508 = CONCAT22(local_508._2_2_,3);
    uStack_500 = 0;
    local_598 = (longlong *)0x0;
    lStack_170 = lStack_4b8;
    local_168 = local_4b0;
    lStack_150 = lStack_4d0;
    local_148 = local_4c8;
    lStack_130 = lStack_4e8;
    local_128 = local_4e0;
    local_118 = local_508;
    uStack_114 = uStack_504;
    uStack_110 = 0;
    uStack_10c = uStack_4fc;
    local_108 = local_4f8;
    local_808 = &local_598;
    local_810 = &local_178;
    local_818 = &local_158;
    local_820 = &local_138;
    local_828 = &local_118;
    local_830 = (undefined8 *)((ulonglong)local_830 & 0xffffffff00000000);
    local_838 = (int *)((ulonglong)local_838 & 0xffffffff00000000);
    uStack_860 = 0x141e384bb;
    iVar13 = (**(code **)(*(longlong *)pIVar4 + 0x168))(pIVar4,0,0,0);
    if (iVar13 < 0) {
      uStack_860 = 0x141e384d0;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_14327fcd0);
    }
    uVar29 = 6;
    local_798 = 6;
    plVar30 = (longlong *)param_1[0x4d];
    plVar27 = local_598;
    if (plVar30 != local_598) {
      param_1[0x4d] = (longlong)local_598;
      plVar27 = plVar32;
      if (plVar30 != (longlong *)0x0) {
        uStack_860 = 0x141e38502;
        (**(code **)(*plVar30 + 0x10))();
        plVar27 = (longlong *)0x0;
      }
    }
    if (plVar27 != (longlong *)0x0) {
      uStack_860 = 0x141e38511;
      (**(code **)(*plVar27 + 0x10))(plVar27);
    }
    if ((short)local_508 == 8) {
      local_508 = local_508 & 0xffff0000;
      if (CONCAT44(uStack_4fc,uStack_500) != 0) {
        uStack_860 = 0x141e3853a;
        (*DAT_143ad5990)(CONCAT44(uStack_4fc,uStack_500) + -4);
      }
    }
    else {
      uStack_860 = 0x141e38549;
      (*DAT_143262a18)(&local_508);
    }
    if (local_4f0[0] == 8) {
      local_4f0[0] = 0;
      if (lStack_4e8 != 0) {
        uStack_860 = 0x141e38572;
        (*DAT_143ad5990)(lStack_4e8 + -4);
      }
    }
    else {
      uStack_860 = 0x141e38581;
      (*DAT_143262a18)(local_4f0);
    }
    if (local_4d8[0] == 8) {
      local_4d8[0] = 0;
      if (lStack_4d0 != 0) {
        uStack_860 = 0x141e385aa;
        (*DAT_143ad5990)(lStack_4d0 + -4);
      }
    }
    else {
      uStack_860 = 0x141e385b9;
      (*DAT_143262a18)(local_4d8);
    }
    if (local_4c0[0] == 8) {
      local_4c0[0] = 0;
      if (lStack_4b8 != 0) {
        uStack_860 = 0x141e385e2;
        (*DAT_143ad5990)(lStack_4b8 + -4);
      }
    }
    else {
      uStack_860 = 0x141e385f1;
      (*DAT_143262a18)(local_4c0);
    }
    pIVar4 = (IUnknown *)param_1[0x4d];
    if (pIVar4 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e3977c;
      FUN_142ef3ac0(0x80004003);
    }
    uStack_4a0 = (longlong *)param_1[0x2c];
    local_4a8 = CONCAT22(local_4a8._2_2_,0xd);
    if (uStack_4a0 != (longlong *)0x0) {
      uStack_860 = 0x141e38629;
      (**(code **)(*uStack_4a0 + 8))();
    }
    local_f8 = local_4a8;
    uStack_f4 = uStack_4a4;
    uStack_f0 = (undefined4)uStack_4a0;
    uStack_ec = uStack_4a0._4_4_;
    local_e8 = local_498;
    uStack_860 = 0x141e3865b;
    iVar13 = (**(code **)(*(longlong *)pIVar4 + 200))(pIVar4,&local_f8);
    if (iVar13 < 0) {
      uStack_860 = 0x141e38670;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_143273488);
    }
    if ((short)local_4a8 == 8) {
      local_4a8 = local_4a8 & 0xffff0000;
      if (uStack_4a0 != (longlong *)0x0) {
        uStack_860 = 0x141e38699;
        (*DAT_143ad5990)((longlong)uStack_4a0 + -4);
      }
    }
    else {
      uStack_860 = 0x141e386a8;
      (*DAT_143262a18)(&local_4a8);
    }
    pIVar4 = (IUnknown *)param_1[0x4d];
    if (pIVar4 == (IUnknown *)0x0) goto LAB_141e39788;
    uStack_860 = 0x141e386cb;
    iVar13 = (**(code **)(*(longlong *)pIVar4 + 0x200))(pIVar4,0xffffffff);
    if (iVar13 < 0) {
      uStack_860 = 0x141e386e0;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_14327fcb0);
    }
    pIVar4 = (IUnknown *)param_1[0x4d];
    if (pIVar4 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39771;
      FUN_142ef3ac0(0x80004003);
    }
    uStack_860 = 0x141e386fe;
    (*DAT_143262a20)(local_430);
    uStack_860 = 0x141e38711;
    iVar13 = FUN_14023c4c0(local_430,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39746;
      FUN_142ef3ac0(iVar13);
    }
    uStack_860 = 0x141e38726;
    (*DAT_143262a20)(local_448);
    uStack_860 = 0x141e38739;
    iVar13 = FUN_14023c4c0(local_448,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e3974e;
      FUN_142ef3ac0(iVar13);
    }
    uStack_860 = 0x141e3874e;
    (*DAT_143262a20)(local_460);
    uStack_860 = 0x141e38761;
    iVar13 = FUN_14023c4c0(local_460,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39756;
      FUN_142ef3ac0(iVar13);
    }
    uStack_860 = 0x141e38776;
    (*DAT_143262a20)(local_478);
    uStack_860 = 0x141e38789;
    iVar13 = FUN_14023c4c0(local_478,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e3975e;
      FUN_142ef3ac0(iVar13);
    }
    uStack_860 = 0x141e3879e;
    (*DAT_143262a20)(&local_490);
    uStack_860 = 0x141e387b1;
    iVar13 = FUN_14023c4c0(&local_490,&DAT_143a8b8d8);
    if (iVar13 < 0) {
                    /* WARNING: Subroutine does not return */
      uStack_860 = 0x141e39766;
      FUN_142ef3ac0(iVar13);
    }
    lStack_d0 = lStack_428;
    local_c8 = local_420;
    lStack_b0 = lStack_440;
    local_a8 = local_438;
    lStack_90 = lStack_458;
    local_88 = local_450;
    lStack_70 = lStack_470;
    local_68 = local_468;
    local_418 = local_490;
    uStack_414 = uStack_48c;
    uStack_410 = uStack_488;
    uStack_40c = uStack_484;
    local_408 = local_480;
    local_820 = &local_d8;
    local_828 = local_b8;
    local_830 = &local_98;
    local_838 = local_78;
    uStack_860 = 0x141e3889b;
    iVar13 = (**(code **)(*(longlong *)pIVar4 + 0x140))(pIVar4,0xfffffff2,0xfffffff0,&local_418);
    if (iVar13 < 0) {
      uStack_860 = 0x141e388b0;
      _com_issue_errorex(iVar13,pIVar4,(_GUID *)&DAT_143273488);
    }
    if ((short)local_490 == 8) {
      local_490 = local_490 & 0xffff0000;
      if (CONCAT44(uStack_484,uStack_488) != 0) {
        uStack_860 = 0x141e388d9;
        (*DAT_143ad5990)(CONCAT44(uStack_484,uStack_488) + -4);
      }
    }
    else {
      uStack_860 = 0x141e388e8;
      (*DAT_143262a18)(&local_490);
    }
    if (local_478[0] == 8) {
      local_478[0] = 0;
      if (lStack_470 != 0) {
        uStack_860 = 0x141e38911;
        (*DAT_143ad5990)(lStack_470 + -4);
      }
    }
    else {
      uStack_860 = 0x141e38920;
      (*DAT_143262a18)(local_478);
    }
    if (local_460[0] == 8) {
      local_460[0] = 0;
      if (lStack_458 != 0) {
        uStack_860 = 0x141e38949;
        (*DAT_143ad5990)(lStack_458 + -4);
      }
    }
    else {
      uStack_860 = 0x141e38958;
      (*DAT_143262a18)(local_460);
    }
    if (local_448[0] == 8) {
      local_448[0] = 0;
      if (lStack_440 != 0) {
        uStack_860 = 0x141e38981;
        (*DAT_143ad5990)(lStack_440 + -4);
      }
    }
    else {
      uStack_860 = 0x141e38990;
      (*DAT_143262a18)(local_448);
    }
    if (local_430[0] == 8) {
      local_430[0] = 0;
      if (lStack_428 != 0) {
        uStack_860 = 0x141e389b9;
        (*DAT_143ad5990)(lStack_428 + -4);
      }
    }
    else {
      uStack_860 = 0x141e389c8;
      (*DAT_143262a18)(local_430);
    }
    uStack_860 = 0x141e389d1;
    FUN_141e46f50(param_1);
  }
  uStack_860 = 0x141e389f7;
  FUN_141e4a5d0(param_1);
  uStack_860 = 0x141e38a01;
  (**(code **)(*param_1 + 0x68))(param_1);
  uStack_860 = 0x141e38a09;
  FUN_141e60680(param_1);
  if ((int)param_1[0x4e] != 0) {
    uStack_860 = 0x141e38a1c;
    FUN_141e4b7e0(param_1);
  }
  uStack_860 = 0x141e38a24;
  FUN_141e4aa10(param_1);
  *(undefined8 *)((longlong)param_1 + 0x274) = 0;
  if ((int *)param_1[0x33] != (int *)0x0) {
    if (*(int *)param_1[0x33] == 1300000) {
      uVar31 = 0xffffffec;
    }
    *(undefined4 *)(param_1 + 0x4f) = uVar31;
  }
  uStack_860 = 0x141e38a62;
  FUN_141e6d010(param_1,param_1 + 0x78);
  if ((int)param_1[0x4e] == 0) {
    if (param_1[0x25] != 0) {
      uStack_860 = 0x141e38a95;
      FUN_140f8abf0(param_1[0x25],0);
      lVar21 = param_1[0x25];
      if (lVar21 == 0) {
        uStack_860 = 0x141e38aae;
        FUN_142e52ed0(0x431,0);
        lVar21 = param_1[0x25];
      }
      uStack_860 = 0x141e38abd;
      FUN_140f8aee0(lVar21,0);
    }
  }
  else {
    uStack_860 = 0x141e38a77;
    FUN_141e3b110(param_1,0);
    uStack_860 = 0x141e38a7f;
    FUN_141e681f0(param_1);
  }
  uStack_860 = 0x141e38ac5;
  uVar12 = FUN_1406e8c20(plVar18);
  uStack_860 = 0x141e38acf;
  FUN_141e51de0(param_1,uVar12);
  uStack_860 = 0x141e38ad7;
  uVar12 = FUN_1406e8c20(plVar18);
  *(undefined4 *)((longlong)param_1 + 0x4a4) = uVar12;
  uStack_860 = 0x141e38ae7;
  uVar10 = FUN_1406e8ae0(plVar18);
  *(undefined1 *)(param_1 + 0x95) = uVar10;
  uStack_860 = 0x141e38af7;
  uVar12 = FUN_1406e8c20(plVar18);
  *(undefined4 *)((longlong)param_1 + 0x4ac) = uVar12;
  uStack_860 = 0x141e38b12;
  FUN_1406e9170(plVar18,(longlong)param_1 + 0x4b4,8);
  if ((longlong *)param_1[0xa5] != (longlong *)0x0) {
    uStack_860 = 0x141e38b26;
    (**(code **)(*(longlong *)param_1[0xa5] + 0x10))();
  }
  param_1[0xa5] = 0;
  uStack_860 = 0x141e38b37;
  iVar13 = FUN_1406e8c20(plVar18);
  *(int *)(param_1 + 0xa4) = iVar13;
  if (iVar13 == 0) {
    *(undefined4 *)(param_1 + 0xa4) = *(undefined4 *)(param_1[0x33] + 0x2a4);
    plVar30 = (longlong *)param_1[0xa8];
    plVar27 = *(longlong **)(param_1[0x33] + 0x2a8);
    if (plVar30 != plVar27) {
      if (plVar27 != (longlong *)0x0) {
        uStack_860 = 0x141e38b7b;
        (**(code **)(*plVar27 + 8))(plVar27);
        plVar30 = (longlong *)param_1[0xa8];
      }
      param_1[0xa8] = (longlong)plVar27;
      if (plVar30 != (longlong *)0x0) {
        uStack_860 = 0x141e38b96;
        (**(code **)(*plVar30 + 0x10))();
      }
    }
  }
  if ((int)param_1[0xa4] - 1U < 3) {
    uStack_860 = 0x141e38bae;
    FUN_141e5caa0(param_1);
    if ((int)param_1[0xa4] == 2) {
      uStack_860 = 0x141e38bc1;
      FUN_141e5a3d0(param_1);
    }
    else if (2 < (int)param_1[0xa4]) {
      uStack_860 = 0x141e38bcd;
      FUN_141e5ad20(param_1);
    }
  }
  puVar5 = (undefined4 *)param_1[0x33];
  if (puVar5[0x11] == 0) {
    uStack_860 = 0x141e38be2;
    cVar8 = FUN_140841a90(*puVar5);
    if (cVar8 == '\0') {
      uStack_860 = 0x141e38bf3;
      iVar13 = FUN_141e84310(puVar5,1);
      if (iVar13 < 0) goto LAB_141e38c18;
      uStack_860 = 0x141e38c05;
      lVar21 = FUN_141e6eff0(puVar5 + 0x4c,iVar13);
      if (*(int *)(lVar21 + 0x48) == 0) goto LAB_141e38c18;
    }
  }
  uStack_860 = 0x141e38c18;
  FUN_141e4bdc0(param_1,0,0);
LAB_141e38c18:
  uStack_860 = 0x141e38c1d;
  lVar21 = FUN_141892840();
  if (lVar21 != 0) {
    uStack_860 = 0x141e38c27;
    uVar19 = FUN_141892840();
    uStack_860 = 0x141e38c39;
    iVar13 = FUN_141bbe5f0(uVar19,*(undefined4 *)param_1[0x33]);
    if (iVar13 == 2) {
      uStack_860 = 0x141e38c4e;
      FUN_141e4f2a0(param_1,0,0,0);
    }
  }
  uStack_860 = 0x141e38c56;
  iVar13 = FUN_1406e8c20(plVar18);
  if ((*(int *)((longlong)param_1 + 0x294) == 0) && (iVar13 != 0)) {
    uStack_860 = 0x141e38c6f;
    FUN_141e57510(param_1,iVar13);
  }
  uStack_860 = 0x141e38c7e;
  FUN_1406e9050(plVar18,&local_618);
  puVar28 = auStack_858;
  if ((local_618 != (char *)0x0) && (puVar28 = auStack_858, *local_618 != '\0')) {
    local_5a8[1] = 0;
    local_3b0 = 0;
    local_7a8 = local_5a8;
    local_5a8[0] = 0;
    local_5b0 = 0;
    local_830 = (undefined8 *)((ulonglong)local_830 & 0xffffffff00000000);
    local_838 = (int *)0x0;
    uStack_860 = 0x141e38cf1;
    iVar13 = (*DAT_1432627f8)(0xfde9,0,local_618,0xffffffff);
    pcVar7 = local_618;
    uVar22 = (longlong)(iVar13 * 2) + 0xf;
    if (uVar22 <= (ulonglong)(longlong)(iVar13 * 2)) {
      uVar22 = 0xffffffffffffff0;
    }
    uStack_860 = 0x141e38d12;
    lVar21 = -(uVar22 & 0xfffffffffffffff0);
    puVar1 = (undefined2 *)((longlong)&local_7b8 + lVar21);
    if (local_618 == (char *)0x0) {
      if (puVar1 != (undefined2 *)0x0) {
        *puVar1 = 0;
      }
    }
    else {
      *(undefined4 *)((longlong)&local_830 + lVar21) = 0x100000;
      *(undefined2 **)((longlong)&local_838 + lVar21) = puVar1;
      *(undefined8 *)(auStack_858 + lVar21 + -8) = 0x141e38d53;
      (*DAT_1432627f8)(0xfde9,0,pcVar7,0xffffffff);
    }
    *(undefined4 *)((longlong)local_7c8 + lVar21) = 0;
    local_7d0[lVar21] = 0;
    *(undefined4 *)((longlong)local_7d8 + lVar21) = 0;
    *(longlong **)((longlong)&local_7e0 + lVar21) = local_5a8 + 1;
    *(undefined4 *)((longlong)local_800 + lVar21 + 0x18) = 0;
    *(undefined4 *)((longlong)local_800 + lVar21 + 0x10) = 0;
    *(undefined4 *)((longlong)local_800 + lVar21 + 8) = 0;
    *(undefined4 *)((longlong)local_800 + lVar21) = 0;
    *(undefined1 **)((longlong)local_800 + lVar21 + -8) = local_3b8;
    *(undefined4 *)((longlong)&local_810 + lVar21) = 0;
    *(undefined4 *)((longlong)&local_818 + lVar21) = 0xff;
    *(undefined4 *)((longlong)&local_820 + lVar21) = 0;
    *(longlong **)((longlong)&local_828 + lVar21) = local_5a8;
    *(undefined4 *)((longlong)&local_830 + lVar21) = 0;
    *(undefined4 *)((longlong)&local_838 + lVar21) = 0;
    *(undefined8 *)(auStack_858 + lVar21 + -8) = 0x141e38dd9;
    FUN_140dc12e0(&local_5c0,puVar1,0,&local_5b0);
    local_5b8 = local_5c0;
    if (local_5c0 != (longlong *)0x0) {
      pcVar2 = *(code **)(*local_5c0 + 8);
      *(undefined8 *)(auStack_858 + lVar21 + -8) = 0x141e38df3;
      (*pcVar2)();
    }
    *(undefined8 *)(auStack_858 + lVar21 + -8) = 0x141e38e03;
    FUN_141e5b5e0(param_1,&local_5b8);
    puVar28 = auStack_858 + lVar21;
    if (local_5c0 != (longlong *)0x0) {
      pcVar2 = *(code **)(*local_5c0 + 0x10);
      *(undefined8 *)(auStack_858 + lVar21 + -8) = 0x141e38e16;
      (*pcVar2)();
      puVar28 = auStack_858 + lVar21;
    }
  }
  lVar21 = param_1[0x33];
  if (*(int *)(lVar21 + 0xd0) == 1) {
    *(undefined8 *)(puVar28 + -8) = 0x141e38e3d;
    plVar18 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x98);
    plVar30 = plVar32;
    local_7a8 = plVar18;
    if (plVar18 != (longlong *)0x0) {
      *(undefined8 *)(puVar28 + -8) = 0x141e38e4e;
      plVar30 = (longlong *)FUN_141e70ce0(plVar18);
    }
    if ((param_1[0x88] - 1U < 999) || (param_1[0x88] == -1)) {
      *(undefined8 *)(puVar28 + -8) = 0x141e38e7a;
      FUN_142e52ed0(0x447);
    }
    if (plVar30 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar30[1]) {
        *(undefined8 *)(puVar28 + -8) = 0x141e38e96;
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar30[1] = plVar30[1] + 1;
      UNLOCK();
      uVar29 = local_798;
    }
    local_380 = param_1[0x88];
    param_1[0x88] = (longlong)plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e38ec1;
    FUN_141e6e010(local_388);
    plVar30 = (longlong *)param_1[0x88];
    if (plVar30 == (longlong *)0x0) {
      *(undefined8 *)(puVar28 + -8) = 0x141e38eda;
      FUN_142e52ed0(0x431,0);
      plVar30 = (longlong *)param_1[0x88];
    }
    pcVar2 = *(code **)(*plVar30 + 8);
    uVar19 = *(undefined8 *)(param_1[0x33] + 0xe0);
    *(undefined8 *)(puVar28 + -8) = 0x141e38efe;
    (*pcVar2)(plVar30,uVar19,param_1);
    lVar21 = param_1[0x33];
  }
  plVar30 = *(longlong **)(lVar21 + 0xc0);
  if (plVar30 != (longlong *)0x0) {
    pcVar2 = *(code **)(*plVar30 + 8);
    local_3c8 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e38f26;
    (*pcVar2)(plVar30);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_5c8 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e38f37;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e38f4e;
    iVar13 = FUN_140910ca0(&local_5c8,&DAT_14329ce58,0);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_62c = iVar13;
    local_608 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e38f67;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e38f7e;
    iVar15 = FUN_140910ca0(&local_608,&DAT_14329ce54,0);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_630 = iVar15;
    local_588 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e38f97;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e38fae;
    iVar16 = FUN_140910ca0(&local_588,&DAT_143273024,0);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_5d8 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e38fc0;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e38fd7;
    iVar17 = FUN_140910ca0(&local_5d8,&DAT_143273074,0);
    uVar25 = iVar16 - iVar13 >> 0x1f;
    uVar26 = iVar17 - iVar15 >> 0x1f;
    pcVar2 = *(code **)(*plVar30 + 8);
    local_5d0 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e39004;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e3901b;
    local_634 = FUN_140910ca0(&local_5d0,"scrollWidth",0);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_7a0 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e3902e;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e39042;
    local_638 = FUN_140910ca0(&local_7a0,"scrollTime",0);
    *(undefined8 *)(puVar28 + -8) = 0x141e39059;
    lVar21 = FUN_14019b780(&DAT_143ad68a0,0x290);
    plVar18 = plVar32;
    local_3a0 = lVar21;
    if (lVar21 != 0) {
      local_3a8 = &local_600;
      local_600 = (longlong *)param_1[0x47];
      if (local_600 != (longlong *)0x0) {
        pcVar2 = *(code **)(*local_600 + 8);
        *(undefined8 *)(puVar28 + -8) = 0x141e39094;
        (*pcVar2)();
      }
      local_790 = &local_5f8;
      pcVar2 = *(code **)(*plVar30 + 8);
      local_5f8 = plVar30;
      *(undefined8 *)(puVar28 + -8) = 0x141e390b0;
      (*pcVar2)(plVar30);
      *(undefined8 *)(puVar28 + -8) = 0x141e390c2;
      puVar23 = (undefined8 *)FUN_1409397b0(param_1 + 1,&local_3d8);
      uVar29 = uVar29 | 1;
      uVar19 = *puVar23;
      local_798 = uVar29;
      *(undefined8 *)(puVar28 + -8) = 0x141e390d8;
      uVar19 = FUN_140eca980(local_3c0,uVar19);
      *(longlong ***)(puVar28 + 0x58) = &local_600;
      *(undefined4 *)(puVar28 + 0x50) = 0xc006149f;
      *(undefined4 *)(puVar28 + 0x48) = 0;
      *(undefined4 *)(puVar28 + 0x40) = local_638;
      *(undefined4 *)(puVar28 + 0x38) = local_634;
      *(uint *)(puVar28 + 0x30) = (iVar17 - iVar15 ^ uVar26) - uVar26;
      *(uint *)(puVar28 + 0x28) = (iVar16 - iVar13 ^ uVar25) - uVar25;
      *(int *)(puVar28 + 0x20) = local_630;
      iVar13 = local_62c;
      *(undefined8 *)(puVar28 + -8) = 0x141e39132;
      plVar18 = (longlong *)FUN_141a24a70(lVar21,uVar19,&local_5f8,iVar13);
    }
    if ((param_1[0x8a] - 1U < 999) || (param_1[0x8a] == -1)) {
      *(undefined8 *)(puVar28 + -8) = 0x141e3915e;
      FUN_142e52ed0(0x447);
    }
    plVar27 = plVar18 + 3;
    if (plVar18 == (longlong *)0x0) {
      plVar27 = plVar32;
    }
    plVar18 = plVar32;
    if ((plVar27 != (longlong *)0x0) &&
       (local_3e0 = plVar27 + -3, plVar18 = local_3e0, local_3e0 != (longlong *)0x0)) {
      if (0xfffff < (ulonglong)plVar27[1]) {
        *(undefined8 *)(puVar28 + -8) = 0x141e39192;
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar27[1] = plVar27[1] + 1;
      UNLOCK();
      plVar30 = local_3c8;
      plVar18 = local_3e0;
      uVar29 = local_798;
    }
    local_3e0 = (longlong *)param_1[0x8a];
    param_1[0x8a] = (longlong)plVar18;
    *(undefined8 *)(puVar28 + -8) = 0x141e391d0;
    FUN_141e6f560(local_3e8);
    if (((uVar29 & 1) != 0) && (local_3d8 != (longlong *)0x0)) {
      pcVar2 = *(code **)(*local_3d8 + 0x10);
      *(undefined8 *)(puVar28 + -8) = 0x141e391e9;
      (*pcVar2)();
    }
    pcVar2 = *(code **)(*plVar30 + 0x10);
    *(undefined8 *)(puVar28 + -8) = 0x141e391f3;
    (*pcVar2)(plVar30);
    lVar21 = param_1[0x33];
  }
  plVar30 = *(longlong **)(lVar21 + 200);
  if (plVar30 != (longlong *)0x0) {
    pcVar2 = *(code **)(*plVar30 + 8);
    local_650 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e39224;
    (*pcVar2)(plVar30);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_5f0 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e39235;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e3924c;
    iVar13 = FUN_140910ca0(&local_5f0,&DAT_14329ce58,0);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_60c = iVar13;
    local_5e8 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e39266;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e3927d;
    iVar15 = FUN_140910ca0(&local_5e8,&DAT_14329ce54,0);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_620[0] = iVar15;
    local_5e0 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e39296;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e392ad;
    iVar16 = FUN_140910ca0(&local_5e0,&DAT_143273024,0);
    pcVar2 = *(code **)(*plVar30 + 8);
    local_640 = plVar30;
    *(undefined8 *)(puVar28 + -8) = 0x141e392c0;
    (*pcVar2)(plVar30);
    *(undefined8 *)(puVar28 + -8) = 0x141e392d7;
    iVar17 = FUN_140910ca0(&local_640,&DAT_143273074,0);
    uVar29 = iVar16 - iVar13 >> 0x1f;
    local_624 = (iVar16 - iVar13 ^ uVar29) - uVar29;
    uVar29 = iVar17 - iVar15 >> 0x1f;
    local_610 = (iVar17 - iVar15 ^ uVar29) - uVar29;
    local_628 = 0xc006149f;
    *(undefined8 *)(puVar28 + -8) = 0x141e39313;
    puVar23 = (undefined8 *)FUN_1409397b0(param_1 + 1,&local_7a8);
    local_3d0 = *puVar23;
    *(longlong **)(puVar28 + 0x40) = param_1 + 0x47;
    *(undefined4 **)(puVar28 + 0x38) = &local_628;
    *(int **)(puVar28 + 0x30) = &local_610;
    *(int **)(puVar28 + 0x28) = &local_624;
    *(int **)(puVar28 + 0x20) = local_620;
    *(undefined8 *)(puVar28 + -8) = 0x141e39374;
    plVar18 = (longlong *)FUN_141e6dcb0(local_398,&local_3d0,&local_650,&local_60c);
    if ((param_1[0x8c] - 1U < 999) || (param_1[0x8c] == -1)) {
      *(undefined8 *)(puVar28 + -8) = 0x141e393a0;
      FUN_142e52ed0(0x447);
    }
    if (param_1 + 0x8b == plVar18) {
      *(undefined8 *)(puVar28 + -8) = 0x141e393b1;
      FUN_142e52d50(0x45c,1);
    }
    lVar21 = plVar18[1];
    if (lVar21 != 0) {
      if (0xfffff < *(ulonglong *)(lVar21 + 0x20)) {
        *(undefined8 *)(puVar28 + -8) = 0x141e393d1;
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar21 + 0x20) = *(longlong *)(lVar21 + 0x20) + 1;
      UNLOCK();
      plVar30 = local_650;
    }
    *(undefined8 *)(puVar28 + -8) = 0x141e393e5;
    FUN_141e6f5e0(param_1 + 0x8b);
    param_1[0x8c] = plVar18[1];
    *(undefined8 *)(puVar28 + -8) = 0x141e393f9;
    FUN_141e6f5e0(local_398);
    if (local_7a8 != (longlong *)0x0) {
      pcVar2 = *(code **)(*local_7a8 + 0x10);
      *(undefined8 *)(puVar28 + -8) = 0x141e39409;
      (*pcVar2)();
    }
    pcVar2 = *(code **)(*plVar30 + 0x10);
    *(undefined8 *)(puVar28 + -8) = 0x141e39413;
    (*pcVar2)(plVar30);
  }
  *(undefined8 *)(puVar28 + -8) = 0x141e39419;
  lVar21 = FUN_141892840();
  if (lVar21 != 0) {
    *(undefined8 *)(puVar28 + -8) = 0x141e39427;
    uVar19 = FUN_141892840();
    *(undefined8 *)(puVar28 + -8) = 0x141e3942f;
    cVar8 = FUN_1418826a0(uVar19);
    if ((cVar8 != '\0') && (DAT_143ac18d8 != 0)) {
      *(undefined8 *)(puVar28 + -8) = 0x141e39456;
      pplVar24 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x90);
      plVar30 = plVar32;
      local_790 = pplVar24;
      if (pplVar24 != (longlong **)0x0) {
        *(undefined8 *)(puVar28 + -8) = 0x141e3946a;
        plVar30 = (longlong *)FUN_140d13c80(pplVar24,param_1);
      }
      *(undefined8 *)(puVar28 + -8) = 0x141e3947e;
      lVar21 = FUN_142df7280(DAT_143ac18d8);
      uVar12 = *(undefined4 *)(lVar21 + 0xc);
      *(undefined8 *)(puVar28 + -8) = 0x141e39489;
      FUN_140d13cc0(plVar30,uVar12);
      if (DAT_143ad2d30 == 0) {
        if ((param_1[0xaf] - 1U < 999) || (param_1[0xaf] == -1)) {
          *(undefined8 *)(puVar28 + -8) = 0x141e394ba;
          FUN_142e52ed0(0x447);
        }
        plVar18 = plVar30 + 5;
        if (plVar30 == (longlong *)0x0) {
          plVar18 = plVar32;
        }
        local_3f0 = plVar32;
        if ((plVar18 != (longlong *)0x0) && (local_3f0 = plVar18 + -5, local_3f0 != (longlong *)0x0)
           ) {
          if (0xfffff < (ulonglong)plVar18[1]) {
            *(undefined8 *)(puVar28 + -8) = 0x141e394f1;
            FUN_142e541f0(0x30f);
          }
          LOCK();
          plVar18[1] = plVar18[1] + 1;
          UNLOCK();
        }
        lVar21 = param_1[0xaf];
        param_1[0xaf] = (longlong)local_3f0;
        local_3f0 = (longlong *)lVar21;
        *(undefined8 *)(puVar28 + -8) = 0x141e39520;
        FUN_141e6f660(local_3f8);
        lVar21 = param_1[0xaf];
        if (lVar21 == 0) {
          *(undefined8 *)(puVar28 + -8) = 0x141e3953a;
          FUN_142e52ed0(0x431,0);
          lVar21 = param_1[0xaf];
        }
        *(undefined8 *)(puVar28 + -8) = 0x141e39547;
        FUN_140d13270(lVar21);
      }
    }
  }
  pcVar2 = *(code **)(*param_1 + 0x38);
  *(undefined8 *)(puVar28 + -8) = 0x141e39551;
  (*pcVar2)(param_1);
  lVar21 = param_1[0x33];
  if (*(char *)(lVar21 + 0x2e1) != '\0') {
    *(undefined4 *)(puVar28 + 0x28) = *(undefined4 *)(lVar21 + 0x2fc);
    *(undefined4 *)(puVar28 + 0x20) = *(undefined4 *)(lVar21 + 0x2f8);
    uVar12 = *(undefined4 *)(lVar21 + 0x2f4);
    uVar31 = *(undefined4 *)(lVar21 + 0x2f0);
    *(undefined8 *)(puVar28 + -8) = 0x141e39593;
    FUN_141e51b90(param_1,lVar21 + 0x2e8,uVar31,uVar12);
  }
  if (local_618 != (char *)0x0) {
    *(undefined8 *)(puVar28 + -8) = 0x141e395a9;
    FUN_14019f2c0(local_618 + -0x10);
  }
  *(undefined8 *)(puVar28 + -8) = 0x141e395b9;
  return;
}



//===========================================================
// FUN_141e421f0 @ 141e421f0   (3569 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141e4291f) */

void FUN_141e421f0(longlong *param_1,undefined8 param_2)

{
  IUnknown **ppIVar1;
  longlong *plVar2;
  char cVar3;
  char cVar4;
  short sVar5;
  int iVar6;
  uint uVar7;
  undefined4 uVar8;
  longlong lVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 *puVar12;
  longlong lVar13;
  ulonglong uVar14;
  IUnknown *pIVar15;
  undefined4 *puVar16;
  IUnknown *pIVar17;
  IUnknown *pIVar18;
  int iVar19;
  longlong *plVar20;
  ulonglong uVar21;
  IUnknown *pIVar22;
  uint uVar23;
  IUnknown *pIVar24;
  IUnknown *pIVar25;
  longlong *local_res8;
  undefined8 local_res10;
  longlong *local_res18;
  IUnknown *local_res20;
  longlong **in_stack_fffffffffffffdc8;
  ulonglong in_stack_fffffffffffffdd0;
  uint in_stack_fffffffffffffde0;
  longlong **pplVar26;
  IUnknown *local_1e0;
  IUnknown *local_1d8;
  longlong *local_1d0;
  undefined2 *local_1c8;
  undefined8 local_1c0;
  undefined4 local_1b8;
  undefined4 uStack_1b4;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  short local_1a0;
  undefined6 uStack_19e;
  longlong lStack_198;
  undefined8 local_190;
  undefined8 *local_188;
  longlong local_180;
  IUnknown *local_178;
  IUnknown *local_170;
  uint local_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined8 local_158;
  uint local_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined8 local_140;
  uint local_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 local_128;
  uint local_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined8 local_110;
  undefined1 local_108 [8];
  longlong *local_100;
  undefined1 local_f8 [8];
  undefined8 local_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined8 local_e0;
  uint local_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 local_c8;
  uint local_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined8 local_b0;
  uint local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  undefined8 local_88;
  longlong lStack_80;
  undefined8 local_78;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  
  if ((char)param_1[0x35] != '\0') {
    if ((int)param_1[0x2e] == 0) {
      return;
    }
    *(undefined4 *)(param_1 + 0x2e) = 0;
    return;
  }
  local_res8 = param_1;
  local_res10 = param_2;
  cVar3 = FUN_1406e8ae0(param_2);
  iVar19 = (int)cVar3;
  cVar4 = FUN_1406e8ae0(param_2);
  local_res18._0_4_ = (uint)cVar4;
  iVar6 = FUN_1406e8c20(param_2);
  pIVar24 = (IUnknown *)0x0;
  uVar23 = 0;
  if (iVar19 == -1) {
    if ((uint)local_res18 == 0xffffffff) goto LAB_141e42f4a;
    uVar10 = FUN_142cbe730(DAT_143aa84a0);
    lVar13 = param_1[0x33];
    uVar11 = (**(code **)(param_1[1] + 0x30))(param_1 + 1,&local_1c0);
    in_stack_fffffffffffffdc8 = &local_res18;
    FUN_141e842c0(lVar13,&local_res8,uVar10,uVar11,in_stack_fffffffffffffdc8);
    if (((-1 < (int)(uint)local_res18) && (local_res8 != (longlong *)0x0)) &&
       ((uint)local_res18 < *(uint *)(local_res8 + -1))) {
      local_res20 = (IUnknown *)local_res8[(int)(uint)local_res18];
      if (local_res20 != (IUnknown *)0x0) {
        LOCK();
        *(int *)(local_res20 + 0x10) = *(int *)(local_res20 + 0x10) + 1;
        UNLOCK();
      }
      FUN_141e3b510(param_1,&local_res20);
    }
    param_2 = local_res10;
    if (local_res8 == (longlong *)0x0) goto LAB_141e42f4a;
    plVar2 = local_res8 + local_res8[-1];
    for (plVar20 = local_res8; plVar20 < plVar2; plVar20 = plVar20 + 1) {
      FUN_1401be120(plVar20);
    }
    pIVar17 = (IUnknown *)(local_res8 + -1);
  }
  else {
    if (cVar3 < '\0') goto LAB_141e42f4a;
    lVar13 = *(longlong *)(param_1[0x33] + 0x108);
    uVar7 = uVar23;
    if (lVar13 != 0) {
      uVar7 = *(uint *)(lVar13 + -8);
    }
    if (uVar7 <= iVar19 - 3U) goto LAB_141e42f4a;
    lVar13 = param_1[0x49];
    if (lVar13 != 0) {
      lVar9 = FUN_141e6cfb0(param_1[0x33] + 0x108,iVar19 + -3);
      if ((*(int *)(lVar9 + 0x14) != 0) ||
         (pIVar17 = (IUnknown *)0x1, *(int *)((longlong)param_1 + 0x294) != 0)) {
        pIVar17 = pIVar24;
      }
      FUN_140d1a5b0(lVar13,pIVar17);
    }
    FUN_141e39c90(param_1);
    *(int *)((longlong)param_1 + 0x1a4) = iVar19;
    lVar13 = *(longlong *)(param_1[0x33] + 0x108);
    if (lVar13 != 0) {
      uVar23 = *(uint *)(lVar13 + -8);
    }
    if (uVar23 - *(int *)(param_1[0x33] + 0x128) <= iVar19 - 3U) {
      *(undefined4 *)(param_1 + 0x39) = 1;
    }
    (**(code **)(*param_1 + 0x68))(param_1);
    param_1[0x85] = 0;
    (**(code **)(*param_1 + 0x78))(param_1,0,0);
    if (iVar6 != 0) {
      FUN_141e4c6b0(param_1,1,iVar6);
    }
    uVar8 = (undefined4)((ulonglong)in_stack_fffffffffffffdc8 >> 0x20);
    local_1e0 = (IUnknown *)0x0;
    iVar19 = iVar19 + -3;
    local_res20 = (IUnknown *)CONCAT44(local_res20._4_4_,iVar19);
    if ((int)param_1[0x51] < 0) {
      uVar10 = FUN_142cbe730(DAT_143aa84a0);
      uVar11 = FUN_141e6cfb0(param_1[0x33] + 0x108,iVar19);
      in_stack_fffffffffffffdd0 = (**(code **)(param_1[1] + 0x30))(param_1 + 1,&local_1c0);
      in_stack_fffffffffffffdc8 = (longlong **)CONCAT44(uVar8,*(undefined4 *)param_1[0x33]);
      puVar12 = (undefined8 *)
                FUN_141e83d60(uVar11,&local_1d0,uVar10,&local_res18,in_stack_fffffffffffffdc8,
                              in_stack_fffffffffffffdd0);
      pIVar22 = (IUnknown *)*puVar12;
      *puVar12 = 0;
      local_1e0 = pIVar22;
      if (local_1d0 != (longlong *)0x0) {
        plVar2 = local_1d0 + local_1d0[-1];
        for (plVar20 = local_1d0; plVar20 < plVar2; plVar20 = plVar20 + 1) {
          FUN_1401be120(plVar20);
        }
        thunk_FUN_140205820(local_1d0 + -1,0);
      }
LAB_141e424e3:
      if (((-1 < (int)(uint)local_res18) && (pIVar22 != (IUnknown *)0x0)) &&
         ((uint)local_res18 < *(uint *)(pIVar22 + -8))) {
        local_1c8 = *(undefined2 **)(pIVar22 + (longlong)(int)(uint)local_res18 * 8);
        if (local_1c8 != (undefined2 *)0x0) {
          LOCK();
          *(int *)(local_1c8 + 8) = *(int *)(local_1c8 + 8) + 1;
          UNLOCK();
          pIVar22 = local_1e0;
        }
        FUN_141e3b510(param_1,&local_1c8);
      }
    }
    else {
      lVar13 = FUN_141e6eff0(param_1[0x33] + 0x130);
      lVar13 = FUN_141e6cfb0(lVar13 + 0x40,iVar19);
      ppIVar1 = (IUnknown **)(lVar13 + 8);
      pIVar22 = pIVar24;
      if (((&local_1e0 != ppIVar1) && (*ppIVar1 != (IUnknown *)0x0)) &&
         (uVar23 = *(uint *)(*ppIVar1 + -8), uVar21 = (ulonglong)uVar23, pIVar22 = (IUnknown *)0x0,
         uVar23 != 0)) {
        lVar13 = FUN_14019b780(&DAT_143ad68a0);
        pIVar22 = (IUnknown *)(lVar13 + 8);
        if (lVar13 == 0) {
          pIVar22 = pIVar24;
        }
        *(ulonglong *)(pIVar22 + -8) = uVar21;
        pIVar18 = *ppIVar1;
        pIVar17 = pIVar18 + uVar21 * 8;
        pIVar25 = pIVar22;
        if (pIVar18 < pIVar17) {
          do {
            lVar13 = *(longlong *)pIVar18;
            pIVar18 = pIVar18 + 8;
            *(longlong *)pIVar25 = lVar13;
            if (lVar13 != 0) {
              LOCK();
              *(int *)(lVar13 + 0x10) = *(int *)(lVar13 + 0x10) + 1;
              UNLOCK();
            }
            pIVar25 = pIVar25 + 8;
          } while (pIVar18 < pIVar17);
          uVar21 = (ulonglong)*(uint *)(pIVar22 + -8);
        }
        local_1e0 = pIVar22;
        if ((int)uVar21 != 0) {
          uVar14 = FUN_1407386b0(&DAT_143ac1ab0);
          local_res18._0_4_ = (uint)((uVar14 & 0xffffffff) % uVar21);
          goto LAB_141e424e3;
        }
      }
      local_res18._0_4_ = 0xffffffff;
    }
    if ((int)param_1[0x39] != 0) {
      local_170 = (IUnknown *)0x0;
      local_180 = 0;
      FUN_1401c21c0(&local_180,PTR_DAT_143a46b38,*(undefined4 *)param_1[0x33]);
      pIVar17 = DAT_143add058;
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_1a0);
      if (DAT_143a8b8d8 == 8) {
        if (local_1a0 == 8) {
          local_1a0 = 0;
          if (lStack_198 != 0) {
            (*DAT_143ad5990)(lStack_198 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_1a0);
          if (iVar6 < 0) goto LAB_141e42fca;
        }
        local_1a0 = 8;
        pIVar18 = pIVar24;
        if (DAT_143a8b8e0 != 0) {
          pIVar18 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_198 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar18);
      }
      else {
        if ((local_1a0 == 8) && (local_1a0 = 0, lStack_198 != 0)) {
          (*DAT_143ad5990)(lStack_198 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_1a0,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_141e42fca:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      (*DAT_143262a20)(&local_1b8);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_1b8 == 8) {
          local_1b8 = (uint)local_1b8._2_2_ << 0x10;
          if (uStack_1b0 != 0) {
            (*DAT_143ad5990)(uStack_1b0 + -4);
          }
        }
        else {
          iVar6 = (*DAT_143262a18)(&local_1b8);
          if (iVar6 < 0) goto LAB_141e42fd2;
        }
        local_1b8 = CONCAT22(local_1b8._2_2_,8);
        pIVar18 = pIVar24;
        if (DAT_143a8b8e0 != 0) {
          pIVar18 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_1b0 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar18);
      }
      else {
        if (((short)local_1b8 == 8) && (local_1b8 = (uint)local_1b8._2_2_ << 0x10, uStack_1b0 != 0))
        {
          (*DAT_143ad5990)(uStack_1b0 + -4);
        }
        iVar6 = (*DAT_143262a28)(&local_1b8,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_141e42fd2:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
      lVar13 = local_180;
      puVar12 = (undefined8 *)FUN_1401a5890(local_108,local_180);
      (*DAT_143262a20)(&local_f0);
      pIVar18 = pIVar24;
      if ((undefined8 *)*puVar12 != (undefined8 *)0x0) {
        pIVar18 = *(IUnknown **)*puVar12;
      }
      local_88 = CONCAT62(uStack_19e,local_1a0);
      lStack_80 = lStack_198;
      local_78 = local_190;
      local_68 = local_1b8;
      uStack_64 = uStack_1b4;
      uStack_60 = (undefined4)uStack_1b0;
      uStack_5c = uStack_1b0._4_4_;
      local_58 = local_1a8;
      in_stack_fffffffffffffdc8 = (longlong **)&local_f0;
      iVar6 = (**(code **)(*(longlong *)pIVar17 + 0x48))
                        (pIVar17,pIVar18,&local_68,&local_88,in_stack_fffffffffffffdc8);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar17,(_GUID *)&DAT_1432743e8);
      }
      local_150 = (uint)local_f0;
      uStack_14c = local_f0._4_4_;
      uStack_148 = uStack_e8;
      uStack_144 = uStack_e4;
      local_140 = local_e0;
      local_f0._0_4_ = (uint)local_f0 & 0xffff0000;
      FUN_1401be120(puVar12);
      uVar10 = FUN_1409339d0(&local_100,&local_150);
      FUN_1401a5040(&local_178,uVar10);
      if (local_100 != (longlong *)0x0) {
        (**(code **)(*local_100 + 0x10))();
      }
      if ((short)local_150 == 8) {
        local_150 = local_150 & 0xffff0000;
        if (CONCAT44(uStack_144,uStack_148) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_144,uStack_148) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_150);
      }
      if ((short)local_1b8 == 8) {
        local_1b8 = local_1b8 & 0xffff0000;
        if (uStack_1b0 != 0) {
          (*DAT_143ad5990)(uStack_1b0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_1b8);
      }
      if (local_1a0 == 8) {
        local_1a0 = 0;
        if (lStack_198 != 0) {
          (*DAT_143ad5990)(lStack_198 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_1a0);
      }
      pIVar17 = local_178;
      pIVar18 = pIVar24;
      if (local_178 != (IUnknown *)0x0) {
        local_188 = (undefined8 *)
                    FUN_141e6cfb0(param_1[0x33] + 0x108,(ulonglong)local_res20 & 0xffffffff);
        local_188 = (undefined8 *)*local_188;
        if (local_188 != (undefined8 *)0x0) {
          LOCK();
          *(int *)(local_188 + 2) = *(int *)(local_188 + 2) + 1;
          UNLOCK();
          pIVar18 = local_170;
          lVar13 = local_180;
          pIVar17 = local_178;
          pIVar22 = local_1e0;
        }
        (*DAT_143262a20)(&local_d8);
        pIVar25 = pIVar24;
        if (local_188 != (undefined8 *)0x0) {
          pIVar25 = (IUnknown *)*local_188;
        }
        iVar6 = (**(code **)(*(longlong *)pIVar17 + 0x28))(pIVar17,pIVar25,&local_d8);
        if (iVar6 < 0) {
          _com_issue_errorex(iVar6,pIVar17,(_GUID *)&DAT_143272478);
        }
        local_138 = local_d8;
        uStack_134 = uStack_d4;
        uStack_130 = uStack_d0;
        uStack_12c = uStack_cc;
        local_128 = local_c8;
        local_d8 = local_d8 & 0xffff0000;
        FUN_1401be120(&local_188);
        uVar10 = FUN_1409339d0(&local_1d8,&local_138);
        FUN_1401a5040(&local_res20,uVar10);
        if (local_res20 != (IUnknown *)0x0) {
          local_170 = local_res20;
          pIVar18 = local_res20;
        }
        if (local_1d8 != (IUnknown *)0x0) {
          (*(code *)(*(longlong **)local_1d8)[2])();
        }
        if ((short)local_138 == 8) {
          local_138 = local_138 & 0xffff0000;
          if (CONCAT44(uStack_12c,uStack_130) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_12c,uStack_130) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_138);
        }
        if (pIVar18 != (IUnknown *)0x0) {
          pIVar15 = (IUnknown *)FUN_1401a5890(local_f8,PTR_u_effect_143a45d10);
          local_res20 = pIVar15;
          (*DAT_143262a20)(&local_c0);
          pIVar25 = pIVar24;
          if (*(undefined8 **)pIVar15 != (undefined8 *)0x0) {
            pIVar25 = (IUnknown *)**(undefined8 **)pIVar15;
          }
          iVar6 = (**(code **)(*(longlong *)pIVar18 + 0x28))(pIVar18,pIVar25,&local_c0);
          if (iVar6 < 0) {
            _com_issue_errorex(iVar6,pIVar18,(_GUID *)&DAT_143272478);
          }
          local_168 = local_c0;
          uStack_164 = uStack_bc;
          uStack_160 = uStack_b8;
          uStack_15c = uStack_b4;
          local_158 = local_b0;
          local_c0 = local_c0 & 0xffff0000;
          FUN_1401be120(pIVar15);
          if ((short)local_168 == 8) {
            local_1c8 = (undefined2 *)CONCAT44(uStack_15c,uStack_160);
            local_res20 = (IUnknown *)0x0;
            lVar9 = 0;
            sVar5 = 8;
            if (local_1c8 != (undefined2 *)0x0) goto LAB_141e42a22;
          }
          else {
            local_1c8 = &DAT_143278568;
LAB_141e42a22:
            local_1d8 = (IUnknown *)0xffffffffffffffff;
            do {
              local_1d8 = local_1d8 + 1;
            } while (local_1c8[(longlong)local_1d8] != 0);
            iVar6 = 0;
            if (0 < (int)local_1d8) {
              iVar6 = (int)local_1d8;
            }
            puVar16 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar6 * 2 + 0x12));
            pIVar25 = local_1d8;
            puVar16[1] = iVar6;
            *puVar16 = 0xffffffff;
            local_res20 = (IUnknown *)(puVar16 + 4);
            puVar16[2] = 0;
            *(short *)local_res20 = 0;
            iVar6 = (int)local_1d8;
            local_1d8 = (IUnknown *)((longlong)iVar6 * 2);
            FUN_142ef7ba0(local_res20,local_1c8,local_1d8);
            if (*(int *)(local_res20 + -0x10) != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar6 == -1) || (iVar6 <= *(int *)(local_res20 + -0xc))) {
              *(undefined4 *)(local_res20 + -0x10) = 1;
              if (iVar6 != -1) goto LAB_141e42ade;
              pIVar25 = pIVar24;
              if (local_res20 != (IUnknown *)0x0) {
                pIVar25 = (IUnknown *)0xffffffffffffffff;
                do {
                  pIVar25 = pIVar25 + 1;
                } while (*(short *)(local_res20 + (longlong)pIVar25 * 2) != 0);
              }
            }
            else {
              FUN_142e54290(0x90,*(int *)(local_res20 + -0xc),(ulonglong)pIVar25 & 0xffffffff);
              *(undefined4 *)(local_res20 + -0x10) = 1;
LAB_141e42ade:
              *(undefined2 *)(local_1d8 + (longlong)local_res20) = 0;
            }
            iVar6 = (int)pIVar25;
            if ((iVar6 < 0) || (*(int *)(local_res20 + -0xc) + 1 <= iVar6)) {
              FUN_142e54290(0x9c,(ulonglong)pIVar25 & 0xffffffff);
            }
            *(int *)(local_res20 + -8) = iVar6 * 2;
            lVar9 = CONCAT44(uStack_15c,uStack_160);
            sVar5 = (short)local_168;
          }
          if (sVar5 == 8) {
            local_168 = local_168 & 0xffff0000;
            if (lVar9 != 0) {
              (*DAT_143ad5990)(lVar9 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_168);
          }
          uVar10 = DAT_143abfdf0;
          if ((local_res20 != (IUnknown *)0x0) && (*(short *)local_res20 != 0)) {
            pIVar25 = (IUnknown *)(local_res8[0x2c] + -0x20);
            if (local_res8[0x2c] == 0) {
              pIVar25 = pIVar24;
            }
            iVar6 = FUN_1409c6ce0(pIVar25);
            pIVar25 = (IUnknown *)(local_res8[0x2c] + -0x20);
            if (local_res8[0x2c] == 0) {
              pIVar25 = pIVar24;
            }
            iVar19 = FUN_1409c6cc0(pIVar25);
            in_stack_fffffffffffffdd0 = in_stack_fffffffffffffdd0 & 0xffffffff00000000;
            in_stack_fffffffffffffdc8 =
                 (longlong **)((ulonglong)in_stack_fffffffffffffdc8 & 0xffffffff00000000);
            FUN_140e18ff0(uVar10,local_res20,(iVar19 * 3000 - iVar6) * 10 + -0x3fff8ada,0,
                          in_stack_fffffffffffffdc8,in_stack_fffffffffffffdd0,0,
                          in_stack_fffffffffffffde0 & 0xffffff00);
            param_1 = local_res8;
          }
          puVar12 = (undefined8 *)FUN_1401a5890(&local_1c0,L"imitateFace");
          (*DAT_143262a20)(&local_a8);
          pIVar25 = pIVar24;
          if ((undefined8 *)*puVar12 != (undefined8 *)0x0) {
            pIVar25 = *(IUnknown **)*puVar12;
          }
          iVar6 = (**(code **)(*(longlong *)pIVar18 + 0x28))(pIVar18,pIVar25,&local_a8);
          if (iVar6 < 0) {
            _com_issue_errorex(iVar6,pIVar18,(_GUID *)&DAT_143272478);
          }
          local_120 = local_a8;
          uStack_11c = uStack_a4;
          uStack_118 = uStack_a0;
          uStack_114 = uStack_9c;
          local_110 = local_98;
          local_a8 = local_a8 & 0xffff0000;
          FUN_1401be120(puVar12);
          iVar6 = FUN_14022ee40(&local_120,0);
          if ((short)local_120 == 8) {
            local_120 = local_120 & 0xffff0000;
            if (CONCAT44(uStack_114,uStack_118) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_114,uStack_118) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_120);
          }
          if ((iVar6 != 0) && (*(int *)((longlong)param_1 + 0x294) == 0)) {
            FUN_140d2d420(param_1 + 0x22);
            uVar8 = (undefined4)(in_stack_fffffffffffffdd0 >> 0x20);
            lVar9 = FUN_140d2d280(0);
            param_1[0x23] = lVar9;
            local_1c0 = FUN_1410a2950(param_1 + 0x22);
            pplVar26 = &local_res8;
            local_res8 = (longlong *)param_1[0x47];
            if (local_res8 != (longlong *)0x0) {
              (**(code **)(*local_res8 + 8))();
            }
            local_1d8 = (IUnknown *)&local_1d0;
            local_1d0 = (longlong *)param_1[0x2c];
            if (local_1d0 != (longlong *)0x0) {
              (**(code **)(*local_1d0 + 8))();
            }
            if (param_1[0x47] == 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            iVar6 = FUN_140d1a410();
            uVar10 = FUN_140f80130(DAT_143aa8518 + 0x100);
            in_stack_fffffffffffffdd0 = CONCAT44(uVar8,0xffffffff);
            in_stack_fffffffffffffdc8 = &local_res8;
            FUN_140f7f030(local_1c0,uVar10,0x13 - (uint)(iVar6 != 0),&local_1d0,
                          in_stack_fffffffffffffdc8,in_stack_fffffffffffffdd0,0xfffffffc,0xfffffffb,
                          100,0,0,0,0,0,pplVar26);
            FUN_141e4a5d0(param_1);
          }
          if (local_res20 != (IUnknown *)0x0) {
            FUN_1401bebb0(local_res20 + -0x10);
          }
        }
      }
      if (pIVar17 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar17 + 0x10))(pIVar17);
      }
      if (lVar13 != 0) {
        FUN_1401bebb0(lVar13 + -0x10);
      }
      if (pIVar18 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar18 + 0x10))(pIVar18);
      }
    }
    param_2 = local_res10;
    if (pIVar22 == (IUnknown *)0x0) goto LAB_141e42f4a;
    pIVar17 = pIVar22 + -8;
    pIVar18 = pIVar22 + *(longlong *)pIVar17 * 8;
    for (; pIVar22 < pIVar18; pIVar22 = pIVar22 + 8) {
      FUN_1401be120(pIVar22);
    }
  }
  thunk_FUN_140205820(pIVar17,0);
  param_2 = local_res10;
LAB_141e42f4a:
  uVar8 = FUN_141892890();
  if (((*(char *)((longlong)param_1 + 0x1a9) == '\0') && (*(int *)(param_1[0x33] + 0x28) != 0)) &&
     (*(int *)((longlong)param_1 + 0x1ac) != 0)) {
    pIVar17 = (IUnknown *)(param_1[0x2c] + -0x20);
    if (param_1[0x2c] == 0) {
      pIVar17 = pIVar24;
    }
    uVar10 = FUN_14099fe20(pIVar17);
    FUN_141d598b0(uVar10,param_2,0,uVar8,(ulonglong)in_stack_fffffffffffffdc8 & 0xffffffff00000000,
                  in_stack_fffffffffffffdd0 & 0xffffffffffffff00);
  }
  *(undefined4 *)(param_1 + 0x2e) = 0;
  return;
}



//===========================================================
// FUN_141e433a0 @ 141e433a0   (2038 bytes)
//===========================================================

void FUN_141e433a0(longlong *param_1,undefined8 param_2)

{
  longlong *plVar1;
  IUnknown *pIVar2;
  byte bVar3;
  int iVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  longlong lVar7;
  uint uVar8;
  uint uVar9;
  undefined8 local_res8 [2];
  undefined8 local_res18;
  longlong *local_res20;
  short local_c8;
  undefined2 uStack_c6;
  undefined4 uStack_c4;
  longlong lStack_c0;
  undefined8 local_b8;
  short local_b0;
  undefined2 uStack_ae;
  undefined4 uStack_ac;
  longlong lStack_a8;
  undefined8 local_a0;
  longlong *local_98;
  longlong *local_90;
  longlong **local_88;
  short *local_80;
  short *local_78;
  longlong **local_70;
  undefined8 local_68;
  longlong lStack_60;
  undefined8 local_58;
  undefined8 local_48;
  longlong lStack_40;
  undefined8 local_38;
  
  bVar3 = FUN_1406e8ae0(param_2);
  *(uint *)(param_1 + 0x4e) = (uint)bVar3;
  *(undefined4 *)(param_1 + 0x45) = 0xffffffff;
  (**(code **)(*param_1 + 0x68))(param_1);
  FUN_141593a20(param_1[8]);
  if ((int)param_1[0x4e] != 0) {
    FUN_141e4b7e0(param_1);
    FUN_141e5caa0(param_1);
    local_70 = (longlong **)local_res8;
    uVar9 = 0;
    local_res8[0] = 0;
    local_78 = &local_c8;
    lStack_c0 = 0;
    local_80 = (short *)&local_res18;
    local_res18 = 0;
    local_88 = &local_res20;
    local_res20 = (longlong *)param_1[0x2c];
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 8))();
    }
    puVar5 = (undefined8 *)FUN_1408a9d20(&local_98,0xbae);
    plVar6 = (longlong *)
             FUN_140dc12e0(&local_90,*puVar5,0,&local_res20,0,0,&local_res18,0xc00614a4,0xff,0,
                           &local_c8,0,0,0,0,local_res8,0,0,0);
    plVar1 = (longlong *)param_1[0x4a];
    if (plVar1 != (longlong *)*plVar6) {
      param_1[0x4a] = *plVar6;
      *plVar6 = 0;
      if (plVar1 != (longlong *)0x0) {
        (**(code **)(*plVar1 + 0x10))();
      }
    }
    if (local_90 != (longlong *)0x0) {
      (**(code **)(*local_90 + 0x10))();
    }
    if (local_98 != (longlong *)0x0) {
      FUN_1401bebb0(local_98 + -2);
    }
    pIVar2 = (IUnknown *)param_1[0x4a];
    if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_c8);
    if (DAT_143a8b8d8 == 8) {
      if (local_c8 == 8) {
        local_c8 = 0;
        if (lStack_c0 != 0) {
          (*DAT_143ad5990)(lStack_c0 + -4);
        }
      }
      else {
        iVar4 = (*DAT_143262a18)(&local_c8);
        if (iVar4 < 0) goto LAB_141e43b81;
      }
      local_c8 = 8;
      uVar8 = uVar9;
      if (DAT_143a8b8e0 != 0) {
        uVar8 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_c0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar8);
    }
    else {
      if ((local_c8 == 8) && (local_c8 = 0, lStack_c0 != 0)) {
        (*DAT_143ad5990)(lStack_c0 + -4);
      }
      iVar4 = (*DAT_143262a28)(&local_c8,&DAT_143a8b8d8);
      if (iVar4 < 0) {
LAB_141e43b81:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar4);
      }
    }
    (*DAT_143262a20)(&local_b0);
    if (DAT_143a8b8d8 == 8) {
      if (local_b0 == 8) {
        local_b0 = 0;
        if (lStack_a8 != 0) {
          (*DAT_143ad5990)(lStack_a8 + -4);
        }
      }
      else {
        iVar4 = (*DAT_143262a18)(&local_b0);
        if (iVar4 < 0) goto LAB_141e43b89;
      }
      local_b0 = 8;
      if (DAT_143a8b8e0 != 0) {
        uVar9 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_a8 = FUN_1401a5fa0(DAT_143a8b8e0,uVar9);
    }
    else {
      if ((local_b0 == 8) && (local_b0 = 0, lStack_a8 != 0)) {
        (*DAT_143ad5990)(lStack_a8 + -4);
      }
      iVar4 = (*DAT_143262a28)(&local_b0,&DAT_143a8b8d8);
      if (iVar4 < 0) {
LAB_141e43b89:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar4);
      }
    }
    local_48 = CONCAT44(uStack_c4,CONCAT22(uStack_c6,local_c8));
    lStack_40 = lStack_c0;
    local_38 = local_b8;
    local_68 = CONCAT44(uStack_ac,CONCAT22(uStack_ae,local_b0));
    lStack_60 = lStack_a8;
    local_58 = local_a0;
    iVar4 = (**(code **)(*(longlong *)pIVar2 + 0x280))(pIVar2,0,&local_68,&local_48);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar2,(_GUID *)&DAT_14327fcb0);
    }
    if (local_b0 == 8) {
      local_b0 = 0;
      if (lStack_a8 != 0) {
        (*DAT_143ad5990)(lStack_a8 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_b0);
    }
    if (local_c8 == 8) {
      local_c8 = 0;
      if (lStack_c0 != 0) {
        (*DAT_143ad5990)(lStack_c0 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_c8);
    }
    FUN_141e60680(param_1);
    FUN_141e3b110(param_1,0);
    FUN_141e681f0(param_1);
    return;
  }
  if ((longlong *)param_1[0x48] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x48] + 0x10))();
  }
  uVar9 = 0;
  param_1[0x48] = 0;
  if ((longlong *)param_1[0x49] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x49] + 0x10))();
  }
  param_1[0x49] = 0;
  if ((longlong *)param_1[0xa5] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xa5] + 0x10))();
  }
  param_1[0xa5] = 0;
  if ((longlong *)param_1[0x4b] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x4b] + 0x10))();
  }
  param_1[0x4b] = 0;
  if ((longlong *)param_1[0x99] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x99] + 0x10))();
  }
  param_1[0x99] = 0;
  if ((longlong *)param_1[0x9a] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x9a] + 0x10))();
  }
  param_1[0x9a] = 0;
  if ((longlong *)param_1[0x98] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x98] + 0x10))();
  }
  param_1[0x98] = 0;
  (**(code **)(*param_1 + 0x18))(param_1,0,0);
  local_88 = (longlong **)local_res8;
  local_res8[0] = 0;
  local_80 = &local_c8;
  lStack_c0 = 0;
  local_78 = (short *)&local_res18;
  local_res18 = 0;
  local_70 = &local_res20;
  local_res20 = (longlong *)param_1[0x2c];
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 8))();
  }
  puVar5 = (undefined8 *)FUN_1408a9d20(&local_90,0xbaf);
  plVar6 = (longlong *)
           FUN_140dc12e0(&local_98,*puVar5,0,&local_res20,0,0,&local_res18,0xc00614a4,0xff,0,
                         &local_c8,0,0,0,0,local_res8,0,0,0);
  plVar1 = (longlong *)param_1[0x4a];
  if (plVar1 != (longlong *)*plVar6) {
    param_1[0x4a] = *plVar6;
    *plVar6 = 0;
    if (plVar1 != (longlong *)0x0) {
      (**(code **)(*plVar1 + 0x10))();
    }
  }
  if (local_98 != (longlong *)0x0) {
    (**(code **)(*local_98 + 0x10))();
  }
  if (local_90 != (longlong *)0x0) {
    FUN_1401bebb0(local_90 + -2);
  }
  pIVar2 = (IUnknown *)param_1[0x4a];
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_b0);
  if (DAT_143a8b8d8 == 8) {
    if (local_b0 == 8) {
      local_b0 = 0;
      if (lStack_a8 != 0) {
        (*DAT_143ad5990)(lStack_a8 + -4);
      }
    }
    else {
      iVar4 = (*DAT_143262a18)(&local_b0);
      if (iVar4 < 0) goto LAB_141e43b71;
    }
    local_b0 = 8;
    uVar8 = uVar9;
    if (DAT_143a8b8e0 != 0) {
      uVar8 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_a8 = FUN_1401a5fa0(DAT_143a8b8e0,uVar8);
  }
  else {
    if ((local_b0 == 8) && (local_b0 = 0, lStack_a8 != 0)) {
      (*DAT_143ad5990)(lStack_a8 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_b0,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_141e43b71:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
  }
  (*DAT_143262a20)(&local_c8);
  if (DAT_143a8b8d8 == 8) {
    if (local_c8 == 8) {
      local_c8 = 0;
      if (lStack_c0 != 0) {
        (*DAT_143ad5990)(lStack_c0 + -4);
      }
    }
    else {
      iVar4 = (*DAT_143262a18)(&local_c8);
      if (iVar4 < 0) goto LAB_141e43b79;
    }
    local_c8 = 8;
    if (DAT_143a8b8e0 != 0) {
      uVar9 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_c0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar9);
  }
  else {
    if ((local_c8 == 8) && (local_c8 = 0, lStack_c0 != 0)) {
      (*DAT_143ad5990)(lStack_c0 + -4);
    }
    iVar4 = (*DAT_143262a28)(&local_c8,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_141e43b79:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
  }
  local_68 = CONCAT44(uStack_ac,CONCAT22(uStack_ae,local_b0));
  lStack_60 = lStack_a8;
  local_58 = local_a0;
  local_48 = CONCAT44(uStack_c4,CONCAT22(uStack_c6,local_c8));
  lStack_40 = lStack_c0;
  local_38 = local_b8;
  iVar4 = (**(code **)(*(longlong *)pIVar2 + 0x280))(pIVar2,0,&local_48,&local_68);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,pIVar2,(_GUID *)&DAT_14327fcb0);
  }
  if (local_c8 == 8) {
    local_c8 = 0;
    if (lStack_c0 != 0) {
      (*DAT_143ad5990)(lStack_c0 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_c8);
  }
  if (local_b0 == 8) {
    local_b0 = 0;
    if (lStack_a8 != 0) {
      (*DAT_143ad5990)(lStack_a8 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_b0);
  }
  FUN_141e60d10(param_1);
  if (param_1[0x25] != 0) {
    FUN_140f8abf0(param_1[0x25],(int)param_1[0x4e]);
    lVar7 = param_1[0x25];
    if (lVar7 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar7 = param_1[0x25];
    }
    FUN_140f8aee0(lVar7,(int)param_1[0x4e]);
  }
  return;
}



//===========================================================
// FUN_141e42ff0 @ 141e42ff0   (195 bytes)
//===========================================================

void FUN_141e42ff0(longlong *param_1,undefined8 param_2)

{
  char cVar1;
  char cVar2;
  undefined4 uVar3;
  longlong local_res18;
  
  FUN_1406e9050(param_2,&local_res18);
  uVar3 = FUN_1406e8c20(param_2);
  cVar1 = FUN_1406e8ae0(param_2);
  cVar2 = FUN_1406e8ae0(param_2);
  if (cVar2 != '\0') {
    *(undefined4 *)((longlong)param_1 + 0x1a4) = 0xffffffff;
    *(undefined4 *)(param_1 + 0x39) = 0;
    *(undefined1 *)(param_1 + 0x35) = 0;
    FUN_140d2d420(param_1 + 0x22);
    param_1[0x85] = 0;
    (**(code **)(*param_1 + 0x78))(param_1,0,0);
  }
  FUN_141e5af80(param_1,&local_res18,uVar3,cVar1 != '\0');
  if (local_res18 != 0) {
    FUN_14019f2c0(local_res18 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141e430c0 @ 141e430c0   (494 bytes)
//===========================================================

void FUN_141e430c0(longlong param_1,undefined8 param_2)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 *puVar3;
  char cVar4;
  int iVar5;
  undefined4 uVar6;
  longlong *plVar7;
  int iVar8;
  bool bVar9;
  longlong local_68;
  longlong local_60;
  undefined4 local_58;
  undefined1 local_54;
  longlong *local_50;
  longlong *local_48;
  
  cVar4 = FUN_1406e8ae0(param_2);
  bVar9 = cVar4 != '\0';
  iVar5 = FUN_1406e8c20(param_2);
  iVar8 = 0;
  if (0 < iVar5) {
    do {
      local_60 = 0;
      local_58 = 0;
      local_54 = 0;
      plVar7 = (longlong *)FUN_1406e9050(param_2,&local_68);
      lVar1 = *plVar7;
      *plVar7 = 0;
      local_60 = lVar1;
      if (local_68 != 0) {
        FUN_14019f2c0(local_68 + -0x10);
      }
      uVar6 = FUN_1406e8c20(param_2);
      lVar2 = *(longlong *)(param_1 + 0x1d0);
      local_58 = uVar6;
      local_54 = bVar9;
      if (*(longlong *)(param_1 + 0x1d8) == 0x7ffffffffffffff) {
                    /* WARNING: Subroutine does not return */
        FUN_142ed3068("list too long");
      }
      local_48 = (longlong *)0x0;
      local_50 = (longlong *)(param_1 + 0x1d0);
      plVar7 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x20);
      plVar7[2] = 0;
      local_48 = plVar7;
      FUN_14019a260(plVar7 + 2,&local_60);
      *(undefined4 *)(plVar7 + 3) = uVar6;
      *(bool *)((longlong)plVar7 + 0x1c) = bVar9;
      *(longlong *)(param_1 + 0x1d8) = *(longlong *)(param_1 + 0x1d8) + 1;
      puVar3 = *(undefined8 **)(lVar2 + 8);
      *plVar7 = lVar2;
      plVar7[1] = (longlong)puVar3;
      local_48 = (longlong *)0x0;
      *(longlong **)(lVar2 + 8) = plVar7;
      *puVar3 = plVar7;
      if (lVar1 != 0) {
        FUN_14019f2c0(lVar1 + -0x10);
      }
      iVar8 = iVar8 + 1;
    } while (iVar8 < iVar5);
  }
  if (*(longlong *)(param_1 + 0x1d8) != 0) {
    FUN_141e5af80(param_1,**(longlong **)(param_1 + 0x1d0) + 0x10,
                  *(undefined4 *)(**(longlong **)(param_1 + 0x1d0) + 0x18),bVar9);
    plVar7 = (longlong *)**(longlong **)(param_1 + 0x1d0);
    lVar1 = *plVar7;
    *(longlong *)(param_1 + 0x1d8) = *(longlong *)(param_1 + 0x1d8) + -1;
    *(longlong *)plVar7[1] = lVar1;
    *(longlong *)(lVar1 + 8) = plVar7[1];
    if (plVar7[2] != 0) {
      FUN_14019f2c0(plVar7[2] + -0x10);
    }
    thunk_FUN_140205820(plVar7,0x20);
  }
  return;
}



//===========================================================
// FUN_141e43bd0 @ 141e43bd0   (110 bytes)
//===========================================================

void FUN_141e43bd0(longlong param_1,undefined8 param_2)

{
  undefined1 uVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  
  if (*(int *)(*(longlong *)(param_1 + 0x198) + 0x1dc) != 0) {
    uVar1 = FUN_1406e8ae0(param_2);
    uVar2 = FUN_1406e8ae0(param_2);
    uVar3 = FUN_1406e8ae0(param_2);
    FUN_141e485a0(param_1,uVar3,uVar2,uVar1);
  }
  return;
}



//===========================================================
// FUN_141e43c50 @ 141e43c50   (117 bytes)
//===========================================================

void FUN_141e43c50(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  
  if (*(int *)(*(longlong *)(param_1 + 0x198) + 0x34) != 0) {
    uVar4 = FUN_142b590f0(*(undefined8 *)(param_1 + 0x168));
    uVar1 = FUN_1406e8c20(param_2);
    uVar2 = FUN_1406e8c20(param_2);
    uVar3 = FUN_1406e8c20(param_2);
    FUN_142b55ff0(uVar4,uVar1,uVar2,uVar3);
  }
  return;
}



//===========================================================
// FUN_141e43cd0 @ 141e43cd0   (96 bytes)
//===========================================================

void FUN_141e43cd0(longlong *param_1,undefined8 param_2)

{
  undefined4 uVar1;
  longlong lVar2;
  
  uVar1 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x9d) = uVar1;
  lVar2 = FUN_142b590f0(param_1[0x2c]);
  if (lVar2 != 0) {
    FUN_1409c6d20(lVar2);
    uVar1 = FUN_1409c6d00(lVar2);
    FUN_141e39b90(param_1,uVar1,0);
  }
                    /* WARNING: Could not recover jumptable at 0x000141e43d2c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(*param_1 + 0x68))(param_1);
  return;
}



//===========================================================
// FUN_141e43d40 @ 141e43d40   (188 bytes)
//===========================================================

void FUN_141e43d40(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  char cVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int iVar5;
  longlong lVar6;
  
  uVar3 = FUN_1406e8c20(param_2);
  uVar4 = FUN_1406e8c20(param_2);
  cVar2 = FUN_1406e8ae0(param_2);
  if ((*(int *)(*(longlong *)(param_1 + 0x198) + 0x1f0) != 0) &&
     (*(longlong *)(param_1 + 0x128) != 0)) {
    uVar1 = FUN_142b590f0(*(undefined8 *)(param_1 + 0x168));
    *(undefined1 *)(param_1 + 0x1a8) = 0;
    if ((cVar2 != '\0') && (iVar5 = FUN_1409c5080(uVar1), iVar5 == 0)) {
      return;
    }
    *(bool *)(param_1 + 0x1a8) = cVar2 != '\0';
    lVar6 = *(longlong *)(param_1 + 0x128);
    if (lVar6 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar6 = *(longlong *)(param_1 + 0x128);
    }
    FUN_140f832e0(lVar6,uVar3,uVar4);
  }
  return;
}



//===========================================================
// FUN_141e43e10 @ 141e43e10   (3042 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e43e10(longlong param_1,undefined8 param_2)

{
  undefined2 *puVar1;
  code *pcVar2;
  IUnknown *pIVar3;
  longlong lVar4;
  char cVar5;
  int iVar6;
  int iVar7;
  undefined4 uVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  int *piVar12;
  int *piVar13;
  undefined4 *puVar14;
  longlong lVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined1 *puVar18;
  uint uVar19;
  uint uVar20;
  undefined8 uStack_210;
  undefined1 auStack_208 [32];
  undefined8 local_1e8;
  int local_1e0 [2];
  longlong local_1d8;
  ulonglong local_1d0;
  undefined8 local_1c8;
  undefined4 local_1c0 [2];
  longlong local_1b8;
  undefined8 local_1b0;
  undefined4 local_1a8 [2];
  longlong local_1a0;
  longlong local_198 [2];
  undefined4 local_188 [2];
  undefined1 local_180 [8];
  undefined4 local_178 [4];
  int *local_168;
  longlong **local_160;
  short local_158;
  undefined2 uStack_156;
  undefined4 uStack_154;
  longlong lStack_150;
  undefined8 local_148;
  short local_140;
  undefined2 uStack_13e;
  undefined4 uStack_13c;
  longlong lStack_138;
  undefined8 local_130;
  IUnknown *local_128;
  IUnknown *local_120;
  IUnknown *local_118;
  longlong local_110;
  longlong local_108;
  undefined8 local_100;
  longlong *local_f8;
  int local_f0 [2];
  longlong ***local_e8;
  IUnknown **local_e0;
  undefined8 local_d8;
  longlong lStack_d0;
  undefined8 local_c8;
  undefined8 local_b8;
  longlong lStack_b0;
  undefined8 local_a8;
  IUnknown local_98 [8];
  undefined8 local_90;
  int **local_88;
  short *local_80;
  IUnknown *local_78 [7];
  undefined8 local_40;
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)&local_168;
  uStack_210 = 0x141e43e61;
  FUN_1406e9050(param_2,&local_110);
  uStack_210 = 0x141e43e6a;
  iVar6 = FUN_1406e8c20(param_2);
  uStack_210 = 0x141e43e75;
  cVar5 = FUN_1406e8ae0(param_2);
  puVar18 = auStack_208;
  if ((*(int *)(*(longlong *)(param_1 + 0x198) + 0x1f0) == 0) ||
     (puVar18 = auStack_208, *(longlong *)(param_1 + 0x128) == 0)) goto LAB_141e4498e;
  uStack_210 = 0x141e43ea8;
  uVar9 = FUN_142b590f0(*(undefined8 *)(param_1 + 0x168));
  *(undefined1 *)(param_1 + 0x1a8) = 0;
  if (cVar5 != '\0') {
    uStack_210 = 0x141e43ebb;
    iVar7 = FUN_1409c5080(uVar9);
    puVar18 = auStack_208;
    if (iVar7 == 0) goto LAB_141e4498e;
  }
  lVar15 = local_110;
  *(bool *)(param_1 + 0x1a8) = cVar5 != '\0';
  piVar12 = (int *)0x0;
  local_108 = 0;
  uStack_210 = 0x141e43ee2;
  puVar10 = (undefined8 *)FUN_1408a9e40(&local_128,0x115e);
  uStack_210 = 0x141e43ef2;
  FUN_14019ba10(&local_108,*puVar10,lVar15);
  if (local_128 != (IUnknown *)0x0) {
    uStack_210 = 0x141e43f05;
    FUN_14019f2c0(local_128 + -0x10);
  }
  local_128 = (IUnknown *)&local_100;
  local_100 = 0;
  local_120 = local_98;
  local_90 = 0;
  local_160 = &local_f8;
  lVar15 = *(longlong *)(param_1 + 0x128);
  if (lVar15 == 0) {
    uStack_210 = 0x141e43f44;
    FUN_142e52ed0(0x431,0);
    lVar15 = *(longlong *)(param_1 + 0x128);
  }
  local_f8 = *(longlong **)(lVar15 + 0xd58);
  if (local_f8 != (longlong *)0x0) {
    uStack_210 = 0x141e43f61;
    (**(code **)(*local_f8 + 8))();
  }
  local_168 = local_f0;
  uStack_210 = 0x141e43f77;
  uVar9 = FUN_1409397b0(param_1 + 8,local_f0);
  lVar15 = *(longlong *)(param_1 + 0x128);
  if (lVar15 == 0) {
    uStack_210 = 0x141e43f92;
    FUN_142e52ed0(0x431,0);
    lVar15 = *(longlong *)(param_1 + 0x128);
  }
  lVar4 = local_108;
  uVar8 = *(undefined4 *)(lVar15 + 0xe5c);
  uVar20 = 0;
  if (local_108 == 0) {
    iVar7 = 2;
  }
  else {
    local_1e0[0] = 0;
    local_1e8 = 0;
    uStack_210 = 0x141e43fce;
    iVar7 = (*DAT_1432627f8)(0xfde9,0,local_108,0xffffffff);
    iVar7 = iVar7 * 2;
  }
  uVar16 = (longlong)iVar7 + 0xf;
  if (uVar16 <= (ulonglong)(longlong)iVar7) {
    uVar16 = 0xffffffffffffff0;
  }
  uStack_210 = 0x141e43ff1;
  lVar15 = -(uVar16 & 0xfffffffffffffff0);
  puVar1 = (undefined2 *)((longlong)&local_168 + lVar15);
  if (lVar4 == 0) {
    if (puVar1 != (undefined2 *)0x0) {
      *puVar1 = 0;
    }
  }
  else {
    *(undefined4 *)((longlong)local_1e0 + lVar15) = 0x100000;
    *(undefined2 **)((longlong)local_1e0 + lVar15 + -8) = puVar1;
    *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44030;
    (*DAT_1432627f8)(0xfde9,0,lVar4,0xffffffff);
  }
  *(undefined4 *)((longlong)local_178 + lVar15) = 0;
  local_180[lVar15] = 0;
  *(undefined4 *)((longlong)local_188 + lVar15) = 0;
  *(undefined8 **)((longlong)local_198 + lVar15 + 8) = &local_100;
  *(undefined4 *)((longlong)local_198 + lVar15) = 0;
  *(undefined4 *)((longlong)&local_1a0 + lVar15) = 0;
  *(undefined4 *)((longlong)local_1a8 + lVar15) = 0;
  *(undefined4 *)((longlong)&local_1b0 + lVar15) = 0;
  *(IUnknown **)((longlong)&local_1b8 + lVar15) = local_98;
  *(undefined4 *)((longlong)local_1c0 + lVar15) = 0;
  *(undefined4 *)((longlong)&local_1c8 + lVar15) = 0xff;
  *(undefined4 *)((longlong)&local_1d0 + lVar15) = 0xffffffff;
  *(longlong ***)((longlong)&local_1d8 + lVar15) = &local_f8;
  *(undefined4 *)((longlong)local_1e0 + lVar15) = 0;
  *(undefined4 *)((longlong)local_1e0 + lVar15 + -8) = 0;
  *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e440ac;
  FUN_140dc12e0(&local_118,puVar1,uVar8,uVar9);
  lVar17 = *(longlong *)(param_1 + 0x128);
  if (lVar17 == 0) {
    *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e440c5;
    FUN_142e52ed0(0x431,0);
    lVar17 = *(longlong *)(param_1 + 0x128);
  }
  local_160 = (longlong **)0x0;
  *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e440dd;
  FUN_14019a260(&local_160,&local_110);
  *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e440e6;
  uVar8 = FUN_1402b2d60(&local_160);
  *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e440f9;
  FUN_140f8d6a0(lVar17,uVar8,0,6);
  lVar17 = *(longlong *)(param_1 + 0x128);
  if (lVar17 == 0) {
    *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44111;
    FUN_142e52ed0(0x431,0);
    lVar17 = *(longlong *)(param_1 + 0x128);
  }
  *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44125;
  FUN_140f8ca70(lVar17,0,0,0);
  if (0 < iVar6) {
    lVar17 = *(longlong *)(param_1 + 0x128);
    if (lVar17 == 0) {
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44142;
      FUN_142e52ed0(0x431,0);
      lVar17 = *(longlong *)(param_1 + 0x128);
    }
    *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44158;
    FUN_140f8ca70(lVar17,1,iVar6);
  }
  pIVar3 = local_118;
  if (local_118 != (IUnknown *)0x0) {
    if (iVar6 < 1) {
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e444b2;
      (*DAT_143262a20)(&local_158);
      if (DAT_143a8b8d8 == 8) {
        if (local_158 == 8) {
          local_158 = 0;
          if (lStack_150 != 0) {
            *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e444e2;
            (*DAT_143ad5990)(lStack_150 + -4);
          }
        }
        else {
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e444ee;
          iVar6 = (*DAT_143262a18)(&local_158);
          if (iVar6 < 0) goto LAB_141e449e6;
        }
        local_158 = 8;
        piVar13 = piVar12;
        if (DAT_143a8b8e0 != 0) {
          piVar13 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44515;
        lStack_150 = FUN_1401a5fa0(DAT_143a8b8e0,piVar13);
      }
      else {
        if ((local_158 == 8) && (local_158 = 0, lStack_150 != 0)) {
          lVar17 = lStack_150 + -4;
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44570;
          (*DAT_143ad5990)(lVar17);
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44581;
        iVar6 = (*DAT_143262a28)(&local_158,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_141e449e6:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_208 + lVar15 + -8) = &UNK_141e449ed;
          FUN_142ef3ac0(iVar6);
        }
      }
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44523;
      (*DAT_143262a20)(&local_140);
      if (DAT_143a8b8d8 == 8) {
        if (local_140 == 8) {
          local_140 = 0;
          if (lStack_138 != 0) {
            *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44550;
            (*DAT_143ad5990)(lStack_138 + -4);
          }
        }
        else {
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44595;
          iVar6 = (*DAT_143262a18)(&local_140);
          if (iVar6 < 0) goto LAB_141e449ee;
        }
        local_140 = 8;
        piVar13 = piVar12;
        if (DAT_143a8b8e0 != 0) {
          piVar13 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e445bc;
        lStack_138 = FUN_1401a5fa0(DAT_143a8b8e0,piVar13);
      }
      else {
        if ((local_140 == 8) && (local_140 = 0, lStack_138 != 0)) {
          lVar17 = lStack_138 + -4;
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44662;
          (*DAT_143ad5990)(lVar17);
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44673;
        iVar6 = (*DAT_143262a28)(&local_140,&DAT_143a8b8d8);
        if (iVar6 < 0) {
LAB_141e449ee:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_208 + lVar15 + -8) = &UNK_141e449f5;
          FUN_142ef3ac0(iVar6);
        }
      }
      local_b8 = CONCAT44(uStack_154,CONCAT22(uStack_156,local_158));
      lStack_b0 = lStack_150;
      local_a8 = local_148;
      local_d8 = CONCAT44(uStack_13c,CONCAT22(uStack_13e,local_140));
      lStack_d0 = lStack_138;
      local_c8 = local_130;
      pcVar2 = *(code **)(*(longlong *)pIVar3 + 0x280);
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4460c;
      iVar6 = (*pcVar2)(pIVar3,0,&local_d8,&local_b8);
      if (iVar6 < 0) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44621;
        _com_issue_errorex(iVar6,pIVar3,(_GUID *)&DAT_14327fcb0);
      }
      if (local_140 == 8) {
        local_140 = 0;
        if (lStack_138 != 0) {
          lVar17 = lStack_138 + -4;
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44641;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4468a;
        (*DAT_143262a18)(&local_140);
      }
      if (local_158 == 8) {
        local_158 = 0;
        if (lStack_150 != 0) {
          lVar17 = lStack_150 + -4;
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e446aa;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e446b6;
        (*DAT_143262a18)(&local_158);
      }
      uVar9 = DAT_143abfdf0;
      local_e0 = local_78;
      local_40 = 0;
      local_e8 = &local_160;
      local_160 = (longlong **)0x0;
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e446f3;
      piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
      piVar13[1] = 0;
      *piVar13 = -1;
      local_160 = (longlong **)(piVar13 + 4);
      piVar13[2] = 0;
      *(undefined1 *)local_160 = 0;
      if (*piVar13 != -1) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44726;
        FUN_142e52dd0(0x8b);
      }
      iVar6 = piVar13[1];
      if (iVar6 < 0) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4473a;
        FUN_142e54290(0x90,iVar6,0);
      }
      *piVar13 = 1;
      *(undefined1 *)local_160 = 0;
      if (piVar13[1] + 1 < 1) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44760;
        FUN_142e54290(0x9c,0);
      }
      piVar13[2] = 0;
      local_168 = (int *)0x0;
      local_88 = &local_168;
      *(undefined4 *)((longlong)local_1e0 + lVar15) = 0;
      *(undefined8 *)((longlong)local_1e0 + lVar15 + -8) = 0;
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44797;
      iVar6 = (*DAT_1432627f8)(0xfde9,0,&DAT_1434b2af1,0xffffffff);
      iVar6 = (int)((ulonglong)(longlong)(iVar6 * 2) >> 1);
      uVar19 = iVar6 - 1;
      piVar13 = piVar12;
      if ((local_168 == (int *)0x0) || (piVar13 = local_168 + -4, piVar13 == (int *)0x0)) {
LAB_141e447e7:
        if ((int)uVar20 < (int)uVar19) {
          uVar20 = uVar19;
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44805;
        puVar14 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(int)(uVar20 * 2 + 0x12));
        puVar14[1] = uVar20;
        *puVar14 = 0xffffffff;
        local_168 = puVar14 + 4;
        puVar14[2] = 0;
        *(undefined2 *)local_168 = 0;
        if (piVar13 != (int *)0x0) {
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44830;
          FUN_1401bebb0(piVar13);
        }
      }
      else {
        if ((1 < *piVar13) || (local_168[-3] < (int)uVar19)) {
          uVar20 = (uint)((ulonglong)(longlong)local_168[-2] >> 1);
          goto LAB_141e447e7;
        }
        if (*piVar13 != 1) {
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e447d0;
          FUN_142e52dd0(0x74);
        }
        *piVar13 = -1;
      }
      *(int *)((longlong)local_1e0 + lVar15) = iVar6;
      *(int **)((longlong)local_1e0 + lVar15 + -8) = local_168;
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44858;
      (*DAT_1432627f8)(0xfde9,0,&DAT_1434b2af1,0xffffffff);
      piVar13 = local_168;
      if (local_168[-4] != -1) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4486e;
        FUN_142e52dd0(0x8b);
      }
      if ((uVar19 != 0xffffffff) && (iVar6 = piVar13[-3], iVar6 < (int)uVar19)) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44887;
        FUN_142e54290(0x90,iVar6,(int *)(ulonglong)uVar19);
      }
      piVar13[-4] = 1;
      if (uVar19 == 0xffffffff) {
        if (piVar13 != (int *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (*(short *)((longlong)piVar13 + (longlong)piVar12 * 2) != 0);
        }
      }
      else {
        *(undefined2 *)((longlong)local_168 + (longlong)(int)uVar19 * 2) = 0;
        piVar12 = (int *)(ulonglong)uVar19;
      }
      iVar6 = (int)piVar12;
      if ((iVar6 < 0) || (piVar13[-3] + 1 <= iVar6)) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e448d9;
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      piVar13[-2] = iVar6 * 2;
      local_80 = &local_158;
      lStack_150 = 0;
      local_120 = (IUnknown *)0x0;
      local_128 = local_118;
      if (local_118 != (IUnknown *)0x0) {
        pcVar2 = *(code **)(*(longlong *)local_118 + 8);
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44905;
        (*pcVar2)();
      }
      *(IUnknown ***)((longlong)local_198 + lVar15 + 8) = local_78;
      *(undefined4 *)((longlong)local_198 + lVar15) = 0xffffffff;
      *(longlong ****)((longlong)&local_1a0 + lVar15) = &local_160;
      *(undefined4 *)((longlong)local_1a8 + lVar15) = 0;
      *(undefined4 *)((longlong)&local_1b0 + lVar15) = 0;
      *(undefined4 *)((longlong)&local_1b8 + lVar15) = 0;
      *(undefined4 *)((longlong)local_1c0 + lVar15) = 0;
      *(undefined4 *)((longlong)&local_1c8 + lVar15) = 0;
      *(int ***)((longlong)&local_1d0 + lVar15) = &local_168;
      *(short **)((longlong)&local_1d8 + lVar15) = &local_158;
      *(undefined4 *)((longlong)local_1e0 + lVar15) = 0;
      *(undefined4 *)((longlong)local_1e0 + lVar15 + -8) = 1000;
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4496e;
      FUN_140dd6110(uVar9,&local_128,0,&local_120);
    }
    else {
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44179;
      (*DAT_143262a20)(&local_140);
      if (DAT_143a8b8d8 == 8) {
        if (local_140 == 8) {
          local_140 = 0;
          if (lStack_138 != 0) {
            *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e441a9;
            (*DAT_143ad5990)(lStack_138 + -4);
          }
        }
        else {
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e441b5;
          iVar7 = (*DAT_143262a18)(&local_140);
          if (iVar7 < 0) goto LAB_141e449d6;
        }
        local_140 = 8;
        piVar13 = piVar12;
        if (DAT_143a8b8e0 != 0) {
          piVar13 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e441dc;
        lStack_138 = FUN_1401a5fa0(DAT_143a8b8e0,piVar13);
      }
      else {
        if ((local_140 == 8) && (local_140 = 0, lStack_138 != 0)) {
          lVar17 = lStack_138 + -4;
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44237;
          (*DAT_143ad5990)(lVar17);
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44248;
        iVar7 = (*DAT_143262a28)(&local_140,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_141e449d6:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_208 + lVar15 + -8) = &UNK_141e449dd;
          FUN_142ef3ac0(iVar7);
        }
      }
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e441ea;
      (*DAT_143262a20)(&local_158);
      if (DAT_143a8b8d8 == 8) {
        if (local_158 == 8) {
          local_158 = 0;
          if (lStack_150 != 0) {
            *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44217;
            (*DAT_143ad5990)(lStack_150 + -4);
          }
        }
        else {
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4425c;
          iVar7 = (*DAT_143262a18)(&local_158);
          if (iVar7 < 0) goto LAB_141e449de;
        }
        local_158 = 8;
        if (DAT_143a8b8e0 != 0) {
          piVar12 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44283;
        lStack_150 = FUN_1401a5fa0(DAT_143a8b8e0,piVar12);
      }
      else {
        if ((local_158 == 8) && (local_158 = 0, lStack_150 != 0)) {
          lVar17 = lStack_150 + -4;
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4432c;
          (*DAT_143ad5990)(lVar17);
        }
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4433d;
        iVar7 = (*DAT_143262a28)(&local_158,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_141e449de:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(auStack_208 + lVar15 + -8) = &UNK_141e449e5;
          FUN_142ef3ac0(iVar7);
        }
      }
      local_d8 = CONCAT44(uStack_13c,CONCAT22(uStack_13e,local_140));
      lStack_d0 = lStack_138;
      local_c8 = local_130;
      local_b8 = CONCAT44(uStack_154,CONCAT22(uStack_156,local_158));
      lStack_b0 = lStack_150;
      local_a8 = local_148;
      pcVar2 = *(code **)(*(longlong *)pIVar3 + 0x280);
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e442d6;
      iVar7 = (*pcVar2)(pIVar3,0x20,&local_b8,&local_d8);
      if (iVar7 < 0) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e442eb;
        _com_issue_errorex(iVar7,pIVar3,(_GUID *)&DAT_14327fcb0);
      }
      if (local_158 == 8) {
        local_158 = 0;
        if (lStack_150 != 0) {
          lVar17 = lStack_150 + -4;
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4430b;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44354;
        (*DAT_143262a18)(&local_158);
      }
      if (local_140 == 8) {
        local_140 = 0;
        if (lStack_138 != 0) {
          lVar17 = lStack_138 + -4;
          *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44374;
          (*DAT_143ad5990)(lVar17);
        }
      }
      else {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44380;
        (*DAT_143262a18)(&local_140);
      }
      uVar9 = DAT_143abfdf0;
      local_e8 = &local_160;
      local_160 = (longlong **)0x0;
      local_e0 = &local_128;
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e443b8;
      uVar11 = FUN_1403edf80(&local_128,&DAT_143278568,0xffffffff);
      local_168 = (int *)0x0;
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e443d0;
      piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
      piVar12[1] = 0;
      *piVar12 = -1;
      local_168 = piVar12 + 4;
      piVar12[2] = 0;
      *(undefined1 *)local_168 = 0;
      if (*piVar12 != -1) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44403;
        FUN_142e52dd0(0x8b);
      }
      iVar7 = piVar12[1];
      if (iVar7 < 0) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44417;
        FUN_142e54290(0x90,iVar7,0);
      }
      *piVar12 = 1;
      *(undefined1 *)local_168 = 0;
      if (piVar12[1] + 1 < 1) {
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4443d;
        FUN_142e54290(0x9c,0);
      }
      piVar12[2] = 0;
      local_120 = local_118;
      if (local_118 != (IUnknown *)0x0) {
        pcVar2 = *(code **)(*(longlong *)local_118 + 8);
        *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e44454;
        (*pcVar2)();
      }
      *(longlong ****)((longlong)local_198 + lVar15) = &local_160;
      *(undefined4 *)((longlong)&local_1a0 + lVar15) = 0;
      *(undefined4 *)((longlong)local_1a8 + lVar15) = 0;
      *(undefined8 *)((longlong)&local_1b0 + lVar15) = 0;
      *(undefined4 *)((longlong)&local_1b8 + lVar15) = 0;
      *(undefined4 *)((longlong)local_1c0 + lVar15) = 0;
      *(undefined8 *)((longlong)&local_1c8 + lVar15) = uVar11;
      *(undefined4 *)((longlong)&local_1d0 + lVar15) = 0;
      *(undefined4 *)((longlong)&local_1d8 + lVar15) = 0;
      *(undefined4 *)((longlong)local_1e0 + lVar15) = 0;
      *(undefined4 *)((longlong)local_1e0 + lVar15 + -8) = 0;
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e444a3;
      FUN_140dd7ab0(uVar9,&local_120,iVar6,&local_168);
    }
    if (local_118 != (IUnknown *)0x0) {
      pcVar2 = *(code **)(*(longlong *)local_118 + 0x10);
      *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4497e;
      (*pcVar2)();
    }
  }
  puVar18 = auStack_208 + lVar15;
  if (lVar4 != 0) {
    *(undefined8 *)(auStack_208 + lVar15 + -8) = 0x141e4498d;
    FUN_14019f2c0(lVar4 + -0x10);
    puVar18 = auStack_208 + lVar15;
  }
LAB_141e4498e:
  if (local_110 != 0) {
    *(undefined8 *)(puVar18 + -8) = 0x141e449a0;
    FUN_14019f2c0(local_110 + -0x10);
  }
  *(undefined8 *)(puVar18 + -8) = 0x141e449b0;
  return;
}



//===========================================================
// FUN_141e44a00 @ 141e44a00   (80 bytes)
//===========================================================

void FUN_141e44a00(undefined8 param_1,undefined8 param_2)

{
  char cVar1;
  char cVar2;
  
  cVar1 = FUN_1406e8ae0(param_2);
  cVar2 = FUN_1406e8ae0(param_2);
  FUN_141e4bdc0(param_1,cVar1 != '\0',cVar2 != '\0');
  return;
}



//===========================================================
// FUN_141e44a60 @ 141e44a60   (70 bytes)
//===========================================================

void FUN_141e44a60(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  
  uVar1 = FUN_1406e8c20(param_2);
  uVar2 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x4a4) = uVar2;
  FUN_141e51de0(param_1,uVar1);
  return;
}



//===========================================================
// FUN_141e44ab0 @ 141e44ab0   (55 bytes)
//===========================================================

void FUN_141e44ab0(longlong param_1,undefined8 param_2)

{
  undefined1 uVar1;
  undefined4 uVar2;
  
  uVar1 = FUN_1406e8ae0(param_2);
  *(undefined1 *)(param_1 + 0x4a8) = uVar1;
  uVar2 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x4ac) = uVar2;
  return;
}



//===========================================================
// FUN_141e44af0 @ 141e44af0   (24 bytes)
//===========================================================

void FUN_141e44af0(longlong param_1,undefined8 param_2)

{
  FUN_1406e9170(param_2,param_1 + 0x4b4,8);
  return;
}



//===========================================================
// FUN_141e44b10 @ 141e44b10   (477 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

void FUN_141e44b10(undefined8 param_1,undefined8 param_2)

{
  undefined2 *puVar1;
  code *pcVar2;
  longlong lVar3;
  char *pcVar4;
  int iVar5;
  ulonglong uVar6;
  undefined1 *puVar7;
  undefined8 uStack_100;
  undefined1 auStack_f8 [32];
  undefined8 local_d8;
  undefined4 local_d0 [2];
  longlong local_c8;
  undefined4 local_c0 [6];
  longlong local_a8;
  undefined4 local_a0 [8];
  longlong local_80;
  undefined4 local_78 [2];
  undefined1 local_70 [8];
  undefined4 local_68 [4];
  longlong *local_58;
  char *local_50;
  longlong *local_48;
  undefined8 local_40;
  longlong local_38 [2];
  undefined1 local_28 [8];
  undefined8 local_20;
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)&local_58;
  uStack_100 = 0x141e44b4d;
  FUN_1406e9050(param_2,&local_50);
  if ((local_50 == (char *)0x0) || (*local_50 == '\0')) {
    local_58 = (longlong *)0x0;
    uStack_100 = 0x141e44cbb;
    FUN_141e5b5e0(param_1,&local_58);
    puVar7 = auStack_f8;
  }
  else {
    local_40 = 0;
    local_20 = 0;
    local_58 = local_38;
    local_38[0] = 0;
    local_38[1] = 0;
    local_d0[0] = 0;
    local_d8 = 0;
    uStack_100 = 0x141e44ba9;
    iVar5 = (*DAT_1432627f8)(0xfde9,0,local_50,0xffffffff);
    pcVar4 = local_50;
    uVar6 = (longlong)(iVar5 * 2) + 0xf;
    if (uVar6 <= (ulonglong)(longlong)(iVar5 * 2)) {
      uVar6 = 0xffffffffffffff0;
    }
    uStack_100 = 0x141e44bca;
    lVar3 = -(uVar6 & 0xfffffffffffffff0);
    puVar1 = (undefined2 *)((longlong)&local_58 + lVar3);
    if (local_50 == (char *)0x0) {
      if (puVar1 != (undefined2 *)0x0) {
        *puVar1 = 0;
      }
    }
    else {
      *(undefined4 *)((longlong)local_d0 + lVar3) = 0x100000;
      *(undefined2 **)((longlong)local_d0 + lVar3 + -8) = puVar1;
      *(undefined8 *)(auStack_f8 + lVar3 + -8) = 0x141e44c08;
      (*DAT_1432627f8)(0xfde9,0,pcVar4,0xffffffff);
    }
    *(undefined4 *)((longlong)local_68 + lVar3) = 0;
    local_70[lVar3] = 0;
    *(undefined4 *)((longlong)local_78 + lVar3) = 0;
    *(undefined8 **)((longlong)&local_80 + lVar3) = &local_40;
    *(undefined4 *)((longlong)local_a0 + lVar3 + 0x18) = 0;
    *(undefined4 *)((longlong)local_a0 + lVar3 + 0x10) = 0;
    *(undefined4 *)((longlong)local_a0 + lVar3 + 8) = 0;
    *(undefined4 *)((longlong)local_a0 + lVar3) = 0;
    *(undefined1 **)((longlong)&local_a8 + lVar3) = local_28;
    *(undefined4 *)((longlong)local_c0 + lVar3 + 0x10) = 0;
    *(undefined4 *)((longlong)local_c0 + lVar3 + 8) = 0xff;
    *(undefined4 *)((longlong)local_c0 + lVar3) = 0;
    *(longlong **)((longlong)&local_c8 + lVar3) = local_38;
    *(undefined4 *)((longlong)local_d0 + lVar3) = 0;
    *(undefined4 *)((longlong)local_d0 + lVar3 + -8) = 0;
    *(undefined8 *)(auStack_f8 + lVar3 + -8) = 0x141e44c75;
    FUN_140dc12e0(&local_58,puVar1,0,local_38 + 1);
    local_48 = local_58;
    if (local_58 != (longlong *)0x0) {
      pcVar2 = *(code **)(*local_58 + 8);
      *(undefined8 *)(auStack_f8 + lVar3 + -8) = 0x141e44c89;
      (*pcVar2)();
    }
    *(undefined8 *)(auStack_f8 + lVar3 + -8) = 0x141e44c96;
    FUN_141e5b5e0(param_1,&local_48);
    puVar7 = auStack_f8 + lVar3;
    if (local_58 != (longlong *)0x0) {
      pcVar2 = *(code **)(*local_58 + 0x10);
      *(undefined8 *)(auStack_f8 + lVar3 + -8) = 0x141e44ca6;
      (*pcVar2)();
      puVar7 = auStack_f8 + lVar3;
    }
  }
  if (local_50 != (char *)0x0) {
    *(undefined8 *)(puVar7 + -8) = 0x141e44cce;
    FUN_14019f2c0(local_50 + -0x10);
  }
  *(undefined8 *)(puVar7 + -8) = 0x141e44cdb;
  return;
}



//===========================================================
// FUN_141e44d00 @ 141e44d00   (62 bytes)
//===========================================================

void FUN_141e44d00(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  
  uVar1 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x520) = uVar1;
  uVar1 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x524) = uVar1;
  FUN_141e5caa0(param_1);
  return;
}



//===========================================================
// FUN_141e44d50 @ 141e44d50   (103 bytes)
//===========================================================

void FUN_141e44d50(longlong param_1,undefined8 param_2)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar1 = FUN_1406e8c20(param_2);
  iVar2 = FUN_1406e8c20(param_2);
  if (iVar1 != *(int *)(param_1 + 0x1ec)) {
    iVar3 = FUN_1429e3ef0();
    *(int *)(param_1 + 0x1e0) = iVar3;
    *(int *)(param_1 + 0x1e4) = iVar3 + iVar2;
    *(undefined4 *)(param_1 + 0x1e8) = *(undefined4 *)(param_1 + 0x1ec);
    *(int *)(param_1 + 0x1ec) = iVar1;
  }
  return;
}



//===========================================================
// FUN_141e44dc0 @ 141e44dc0   (67 bytes)
//===========================================================

void FUN_141e44dc0(undefined8 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  
  uVar1 = FUN_1406e8c20(param_2);
  uVar2 = FUN_1406e8c20(param_2);
  FUN_141e45140(param_1,uVar1,uVar2);
  return;
}



//===========================================================
// FUN_141e44e10 @ 141e44e10   (733 bytes)
//===========================================================

void FUN_141e44e10(longlong param_1,undefined8 param_2)

{
  longlong *plVar1;
  int *piVar2;
  undefined8 *puVar3;
  longlong *plVar4;
  undefined8 uVar5;
  IUnknown *pIVar6;
  longlong lVar7;
  char cVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  int iVar13;
  undefined8 uVar14;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  longlong *local_c0;
  longlong local_b8;
  longlong *local_b0;
  longlong *local_a8;
  longlong *local_a0;
  uint local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 local_88;
  undefined8 *local_80;
  undefined8 *local_78;
  IUnknown *local_70;
  longlong **local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  FUN_1406e9050(param_2,&local_b8);
  uVar9 = FUN_1406e8c20(param_2);
  uVar10 = thunk_FUN_1406e8c20(param_2);
  uVar11 = thunk_FUN_1406e8c20(param_2);
  uVar12 = thunk_FUN_1406e8c20(param_2);
  plVar4 = *(longlong **)(param_1 + 0x238);
  if (plVar4 != (longlong *)0x0) {
    (**(code **)(*plVar4 + 8))(plVar4);
    (**(code **)(*plVar4 + 0x10))(plVar4);
    local_b0 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
    local_a0 = (longlong *)0x0;
    if (local_b0 != (longlong *)0x0) {
      *local_b0 = 0;
      local_b0[1] = 0;
      *(undefined4 *)(local_b0 + 1) = 1;
      *(undefined4 *)((longlong)local_b0 + 0xc) = 1;
      *local_b0 = (longlong)&PTR_FUN_143413df8;
      local_b0[2] = 0;
      local_a0 = local_b0;
    }
    local_a8 = local_a0 + 2;
    puVar3 = (undefined8 *)(param_1 + 0x598);
    FUN_141e6e660(puVar3,&local_a8);
    plVar4 = local_a0;
    if (local_a0 != (longlong *)0x0) {
      LOCK();
      plVar1 = local_a0 + 1;
      lVar7 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -1;
      UNLOCK();
      if ((int)lVar7 == 1) {
        (**(code **)*local_a0)(local_a0);
        LOCK();
        piVar2 = (int *)((longlong)plVar4 + 0xc);
        iVar13 = *piVar2;
        *piVar2 = *piVar2 + -1;
        UNLOCK();
        if (iVar13 == 1) {
          (**(code **)(*local_a0 + 8))();
        }
      }
    }
    uVar5 = *puVar3;
    local_80 = &local_d8;
    local_d8 = 0;
    FUN_14019a260(&local_d8,&local_b8);
    local_78 = &local_c8;
    pIVar6 = *(IUnknown **)(param_1 + 0x238);
    local_70 = pIVar6;
    if (pIVar6 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (**(code **)(*(longlong *)pIVar6 + 8))(pIVar6);
    local_98 = CONCAT22(local_98._2_2_,3);
    uStack_90 = 0;
    local_d0 = 0;
    local_58 = local_98;
    uStack_54 = uStack_94;
    uStack_50 = 0;
    uStack_4c = uStack_8c;
    local_48 = local_88;
    iVar13 = (**(code **)(*(longlong *)pIVar6 + 0x240))(pIVar6,&local_58,&local_d0);
    if (iVar13 < 0) {
      _com_issue_errorex(iVar13,pIVar6,(_GUID *)&DAT_14327fcb0);
    }
    local_c8 = local_d0;
    local_68 = &local_c0;
    local_c0 = *(longlong **)(param_1 + 0x238);
    if (local_c0 != (longlong *)0x0) {
      (**(code **)(*local_c0 + 8))();
    }
    uVar14 = FUN_1409397b0(param_1 + 8,&local_b0);
    cVar8 = FUN_1403228d0(uVar5,uVar14,&local_c0,&local_c8,&local_d8,uVar9,uVar10,uVar11,uVar12);
    if ((short)local_98 == 8) {
      local_98 = local_98 & 0xffff0000;
      if (CONCAT44(uStack_8c,uStack_90) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_8c,uStack_90) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_98);
    }
    if (pIVar6 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar6 + 0x10))(pIVar6);
    }
    if (cVar8 == '\0') {
      *puVar3 = 0;
      plVar4 = *(longlong **)(param_1 + 0x5a0);
      *(undefined8 *)(param_1 + 0x5a0) = 0;
      if (plVar4 != (longlong *)0x0) {
        LOCK();
        plVar1 = plVar4 + 1;
        lVar7 = *plVar1;
        *(int *)plVar1 = (int)*plVar1 + -1;
        UNLOCK();
        if ((int)lVar7 == 1) {
          (**(code **)*plVar4)(plVar4);
          LOCK();
          piVar2 = (int *)((longlong)plVar4 + 0xc);
          iVar13 = *piVar2;
          *piVar2 = *piVar2 + -1;
          UNLOCK();
          if (iVar13 == 1) {
            (**(code **)(*plVar4 + 8))(plVar4);
          }
        }
      }
    }
  }
  if (local_b8 != 0) {
    FUN_14019f2c0(local_b8 + -0x10);
  }
  return;
}



//===========================================================
// FUN_1402eeef0 @ 1402eeef0   (139 bytes)
//===========================================================

void FUN_1402eeef0(undefined4 *param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined8 *puVar2;
  longlong local_res8;
  
  uVar1 = FUN_1406e8c20(param_2);
  *param_1 = uVar1;
  puVar2 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
  if (*(longlong *)(param_1 + 1) != 0) {
    FUN_14019f2c0();
    *(undefined8 *)(param_1 + 1) = 0;
  }
  *(undefined8 *)(param_1 + 1) = *puVar2;
  *puVar2 = 0;
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  uVar1 = FUN_1406e8c20(param_2);
  param_1[3] = uVar1;
  uVar1 = FUN_1406e8c20(param_2);
  param_1[4] = uVar1;
  return;
}



//===========================================================
// FUN_142d1caf0 @ 142d1caf0   (458 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142d1caf0(longlong param_1,undefined4 param_2,undefined1 param_3)

{
  longlong lVar1;
  int iVar2;
  undefined4 uVar3;
  int *piVar4;
  undefined1 auStack_4a8 [32];
  int *local_488 [2];
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  if (((*(int *)(param_1 + 0x2338) == 0) && (*(int *)(param_1 + 0x2330) == 0)) &&
     (lVar1 = *(longlong *)(param_1 + 0x2358), lVar1 != 0)) {
    iVar2 = FUN_1401ba9d0(lVar1 + 0x5b,*(undefined4 *)(lVar1 + 99));
    if (0 < iVar2) {
      iVar2 = FUN_1429e3ef0();
      if (499 < iVar2 - *(int *)(param_1 + 0x2334)) {
        FUN_1406ed520(local_478,0x1fc);
        uVar3 = FUN_1429e3ef0();
        FUN_1406ed9d0(local_478,uVar3);
        FUN_1406ed9d0(local_478,param_2);
        local_488[0] = (int *)0x0;
        piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
        piVar4[1] = 0;
        *piVar4 = -1;
        local_488[0] = piVar4 + 4;
        piVar4[2] = 0;
        *(undefined1 *)local_488[0] = 0;
        if (*piVar4 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar4[1] < 0) {
          FUN_142e54290(0x90,piVar4[1],0);
        }
        *piVar4 = 1;
        *(undefined1 *)local_488[0] = 0;
        if (piVar4[1] + 1 < 1) {
          FUN_142e54290(0x9c,0);
        }
        piVar4[2] = 0;
        FUN_1406edc80(local_478,local_488);
        if (local_488[0] != (int *)0x0) {
          FUN_14019f2c0(local_488[0] + -4);
        }
        FUN_1406ed840(local_478,param_3);
        FUN_1415d01c0(local_478);
        *(undefined4 *)(param_1 + 0x2330) = 1;
        uVar3 = FUN_1429e3ef0();
        *(undefined4 *)(param_1 + 0x2334) = uVar3;
        uVar3 = FUN_1429e3ef0();
        FUN_142e54b20(uVar3);
        uVar3 = FUN_1429e3ef0();
        FUN_142e54f40(uVar3);
        FUN_1406ed610(local_478);
      }
    }
  }
  return;
}



//===========================================================
// FUN_142d1ccc0 @ 142d1ccc0   (288 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142d1ccc0(longlong param_1,undefined8 param_2,undefined1 param_3)

{
  longlong lVar1;
  int iVar2;
  undefined4 uVar3;
  undefined1 auStack_498 [32];
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  if (((*(int *)(param_1 + 0x2338) == 0) && (*(int *)(param_1 + 0x2330) == 0)) &&
     (lVar1 = *(longlong *)(param_1 + 0x2358), lVar1 != 0)) {
    iVar2 = FUN_1401ba9d0(lVar1 + 0x5b,*(undefined4 *)(lVar1 + 99));
    if (0 < iVar2) {
      iVar2 = FUN_1429e3ef0();
      if (499 < iVar2 - *(int *)(param_1 + 0x2334)) {
        FUN_1406ed520(local_478,0x1fc);
        uVar3 = FUN_1429e3ef0();
        FUN_1406ed9d0(local_478,uVar3);
        FUN_1406ed9d0(local_478,0);
        FUN_1406edc80(local_478,param_2);
        FUN_1406ed840(local_478,param_3);
        FUN_1415d01c0(local_478);
        *(undefined4 *)(param_1 + 0x2330) = 1;
        uVar3 = FUN_1429e3ef0();
        *(undefined4 *)(param_1 + 0x2334) = uVar3;
        uVar3 = FUN_1429e3ef0();
        FUN_142e54b20(uVar3);
        uVar3 = FUN_1429e3ef0();
        FUN_142e54f40(uVar3);
        FUN_1406ed610(local_478);
      }
    }
  }
  return;
}


