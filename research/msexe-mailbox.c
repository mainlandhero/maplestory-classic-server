
//===========================================================
// FUN_142cb1020 @ 142cb1020   (12339 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142cb1020(longlong *param_1,int param_2,int param_3)

{
  longlong **pplVar1;
  uint uVar2;
  bool bVar3;
  undefined8 *puVar4;
  undefined4 uVar5;
  longlong **pplVar6;
  char cVar7;
  undefined4 uVar8;
  int iVar9;
  undefined4 uVar10;
  undefined8 uVar11;
  longlong *plVar12;
  longlong lVar13;
  longlong lVar14;
  int *piVar15;
  ulonglong uVar16;
  longlong **pplVar17;
  char *pcVar18;
  longlong lVar19;
  undefined8 *puVar20;
  undefined8 *puVar21;
  undefined8 *puVar22;
  int iVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  undefined1 auStack_578 [32];
  int local_558;
  undefined4 local_550;
  undefined4 local_548;
  undefined4 local_540;
  undefined4 local_538;
  undefined4 local_530;
  longlong **local_528;
  longlong *local_520;
  longlong *local_518;
  longlong local_510;
  longlong **local_508;
  undefined8 local_500;
  longlong **local_4f8;
  undefined8 local_4f0;
  ulonglong local_4e8;
  longlong *local_4e0 [3];
  longlong local_4c8 [4];
  longlong **local_4a8;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_578;
  local_510 = CONCAT44(local_510._4_4_,param_2);
  cVar7 = FUN_141b1e8f0(DAT_143abea80);
  if ((cVar7 != '\0') && (param_2 != 0xd5)) {
    uVar11 = FUN_141892840();
    cVar7 = FUN_14031f160(uVar11);
    if (cVar7 != '\0') {
      return;
    }
    uVar11 = FUN_1408a9e40(&local_518,0x11fb);
    FUN_1415eca30(uVar11,0xb);
    if (local_518 == (longlong *)0x0) {
      return;
    }
    FUN_14019f2c0(local_518 + -2);
    return;
  }
  uVar8 = (**(code **)(*param_1 + 0xa8))(param_1);
  plVar12 = (longlong *)FUN_141892840();
  local_518 = plVar12;
  if (plVar12 != (longlong *)0x0) {
    iVar9 = FUN_1418710f0(plVar12,param_2);
    if (iVar9 != 0) {
      param_2 = FUN_141870f30(plVar12,param_2);
      local_510 = CONCAT44(local_510._4_4_,param_2);
    }
    iVar9 = FUN_14182fd00(plVar12,param_2);
    if (iVar9 != 0) {
      uVar11 = FUN_141892840();
      cVar7 = FUN_14031f160(uVar11);
      if (cVar7 != '\0') {
        return;
      }
      uVar11 = FUN_1408a9e40(&local_518,0x12b4);
      FUN_1415eca30(uVar11,0xb);
      if (local_518 == (longlong *)0x0) {
        return;
      }
      FUN_14019f2c0(local_518 + -2);
      return;
    }
    iVar9 = FUN_141892c30(0x16);
    if ((iVar9 != 0) && (param_2 != 6)) {
      return;
    }
    iVar9 = FUN_141892c30(0x54);
    if ((iVar9 != 0) && (param_2 == 0x4e9)) {
      return;
    }
    cVar7 = (**(code **)(*plVar12 + 0x188))(plVar12,param_2);
    if (cVar7 != '\0') {
      return;
    }
  }
  if (param_2 == 0x4e9) {
    if ((char)param_1[0x1d] != '\0') {
      param_2 = 0x4ec;
    }
    local_510 = CONCAT44(local_510._4_4_,param_2);
  }
  cVar7 = FUN_14090d5e0(0x518,param_2);
  if (cVar7 != '\0') {
    return;
  }
  FUN_1411571c0(param_2,0);
  if (0x109 < param_2) {
    if (param_2 < 0x44f) {
      if (param_2 == 0x44e) {
        if ((DAT_143aceac0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x1488),
           local_528 != (longlong **)0x0)) {
          FUN_1426e6de0(local_528);
        }
        goto switchD_142cb2db3_caseD_11c;
      }
      switch(param_2) {
      case 0x10a:
        if ((DAT_143acf2b0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x3a0),
           local_528 != (longlong **)0x0)) {
          FUN_1426fcea0(local_528);
        }
        break;
      default:
        goto switchD_142cb121a_caseD_1;
      case 0x10d:
        if ((DAT_143acf2b8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x328),
           local_528 != (longlong **)0x0)) {
          FUN_142700420(local_528);
        }
        break;
      case 0x111:
        if ((DAT_143acfbc8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2e8),
           local_528 != (longlong **)0x0)) {
          FUN_1424a2360(local_528);
        }
        break;
      case 0x114:
        uVar11 = FUN_141892840();
        cVar7 = FUN_14031f350(uVar11);
        if (((cVar7 == '\0') && (DAT_143ad74d8 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x1488),
           local_528 != (longlong **)0x0)) {
          FUN_14222cf10(local_528);
        }
        break;
      case 0x115:
        uVar11 = FUN_141892840();
        cVar7 = FUN_14031f350(uVar11);
        if (((cVar7 == '\0') && (DAT_143ad74e0 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x308),
           local_528 != (longlong **)0x0)) {
          FUN_142234310(local_528);
        }
        break;
      case 0x117:
      case 0x118:
      case 0x119:
      case 0x11a:
        iVar9 = FUN_142747050(param_2);
        if (param_1[((longlong)iVar9 + 0x2a2) * 2] == 0) {
          local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x3c8);
          if (local_528 == (longlong **)0x0) {
            uVar11 = 0;
          }
          else {
            uVar11 = FUN_14221a750(local_528,param_2);
          }
          iVar9 = FUN_142747050(param_2);
          FUN_142d2e060(param_1 + (longlong)iVar9 * 2 + 0x543,uVar11);
        }
        break;
      case 0x11c:
        break;
      case 0x11d:
        goto switchD_142cb121a_caseD_2b;
      case 0x122:
        if ((DAT_143adb698 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x348),
           local_528 != (longlong **)0x0)) {
          FUN_14270bd10(local_528);
        }
        break;
      case 0x123:
        if ((DAT_143adb6a0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2e8),
           local_528 != (longlong **)0x0)) {
          FUN_14270d5e0(local_528);
        }
        break;
      case 0x124:
        if ((DAT_143ad4868 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x3b8),
           local_528 != (longlong **)0x0)) {
          FUN_14270ff80(local_528);
        }
        break;
      case 299:
        if ((DAT_143ad27c0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2f8),
           local_528 != (longlong **)0x0)) {
          FUN_141c3a0f0(local_528);
        }
      }
      goto switchD_142cb2db3_caseD_11c;
    }
    if (param_2 < 0x579) {
      if (param_2 == 0x578) {
        FUN_14139a200();
        goto switchD_142cb2db3_caseD_11c;
      }
      switch(param_2) {
      case 0x452:
        if ((*(int *)((longlong)param_1 + 0x24ac) == 0) && ((int)param_1[0x496] == 0)) {
          if ((DAT_143ad00e8 == 0) &&
             (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2f8),
             local_528 != (longlong **)0x0)) {
            FUN_142220b10(local_528);
          }
          iVar9 = FUN_142221c80(DAT_143ad00e8);
          if (iVar9 < 1) {
            local_510 = 0;
            puVar20 = (undefined8 *)FUN_1408a9e40(&local_520,0x130c);
            uVar11 = FUN_14019ba10(&local_510,*puVar20);
            local_518 = (longlong *)0x0;
            FUN_14019a260(&local_518,uVar11);
            local_530 = 0;
            local_538 = 0;
            local_540 = 0;
            local_548 = 0;
            local_550 = 0;
            local_558 = 0;
            FUN_142a26280(&local_518,0,0,1);
            if (local_520 != (longlong *)0x0) {
              FUN_14019f2c0(local_520 + -2);
            }
            if (local_510 != 0) {
              FUN_14019f2c0(local_510 + -0x10);
            }
            FUN_1419c2400(DAT_143acf0b8);
          }
        }
        break;
      default:
        goto switchD_142cb121a_caseD_1;
      case 0x456:
        plVar12 = DAT_143ad8718;
        if ((DAT_143ad8718 == (longlong *)0x0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2a0), plVar12 = (longlong *)0x0,
           local_528 != (longlong **)0x0)) {
          plVar12 = (longlong *)FUN_142486af0(local_528);
        }
        iVar9 = (**(code **)(*plVar12 + 0x130))();
        local_4a8 = (longlong **)0x0;
        local_528 = local_4e0;
        if ((DAT_143ad8718 != (longlong *)0x0) &&
           ((**(code **)DAT_143ad8718[1])(DAT_143ad8718 + 1,1), local_4a8 != (longlong **)0x0)) {
          local_520 = (longlong *)FUN_142ef86e8(0x143aa2f90,&DAT_143ae2c10);
          if (local_4a8 == (longlong **)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ed3024();
          }
          (*(code *)(*local_4a8)[2])(local_4a8,&local_520);
        }
        if (local_4a8 != (longlong **)0x0) {
          (*(code *)(*local_4a8)[4])(local_4a8,local_4a8 != local_4e0);
        }
        if (iVar9 == 1) {
          FUN_1406ed520(local_498,0x177);
          uVar11 = FUN_1408a9e40(&local_520,0x150c);
          FUN_1415eca30(uVar11,0xb);
          if (local_520 != (longlong *)0x0) {
            FUN_14019f2c0(local_520 + -2);
          }
          FUN_1406ed9d0(local_498,param_3);
          FUN_1415d01c0(local_498);
          FUN_1406ed610(local_498);
        }
        break;
      case 0x458:
        if ((DAT_143ad4878 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x498),
           local_528 != (longlong **)0x0)) {
          FUN_1424a5620(local_528,0);
        }
        FUN_1424aa330(DAT_143ad4878,param_3);
        FUN_1424a9cb0(DAT_143ad4878);
        break;
      case 0x459:
        if ((DAT_143ad87f8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2f8),
           local_528 != (longlong **)0x0)) {
          FUN_1424af270(local_528);
        }
        break;
      case 0x45c:
        if ((DAT_143ad7740 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x308),
           local_528 != (longlong **)0x0)) {
          FUN_1422e47e0(local_528);
        }
        break;
      case 0x461:
        if ((DAT_143aa8520 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x300),
           local_528 != (longlong **)0x0)) {
          FUN_1422569b0(local_528);
        }
        break;
      case 0x46f:
        if ((DAT_143ad85d8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2f0),
           local_528 != (longlong **)0x0)) {
          FUN_14243e0e0(local_528);
        }
        break;
      case 0x470:
        if ((DAT_143ad85e0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x388),
           local_528 != (longlong **)0x0)) {
          FUN_142440420(local_528);
        }
        break;
      case 0x471:
        if ((DAT_143ad85e8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x370),
           local_528 != (longlong **)0x0)) {
          FUN_1424467b0(local_528);
        }
        break;
      case 0x472:
        if ((DAT_143ad4870 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x380),
           local_528 != (longlong **)0x0)) {
          FUN_14244c080(local_528);
        }
        break;
      case 0x477:
        if ((DAT_143ad85f0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2f8),
           local_528 != (longlong **)0x0)) {
          FUN_1424503d0(local_528);
        }
        break;
      case 0x47f:
        if ((((DAT_143ad7ac8 == 0) || (FUN_142bf3f70(), DAT_143ad7ac8 == 0)) ||
            ((*(code *)**(undefined8 **)(DAT_143ad7ac8 + 8))((undefined8 *)(DAT_143ad7ac8 + 8),1),
            DAT_143ad7ac8 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x318),
           local_528 != (longlong **)0x0)) {
          FUN_14232ad40(local_528);
        }
        cVar7 = FUN_14232c3d0(DAT_143ad7ac8);
        if (((cVar7 == '\0') && (DAT_143ad7ac8 != 0)) && (FUN_142bf3f70(), DAT_143ad7ac8 != 0)) {
          (*(code *)**(undefined8 **)(DAT_143ad7ac8 + 8))((undefined8 *)(DAT_143ad7ac8 + 8),1);
        }
        break;
      case 0x482:
      case 0x51b:
      case 0x51c:
      case 0x51d:
      case 0x51e:
      case 0x51f:
      case 0x520:
      case 0x521:
        goto switchD_142cb121a_caseD_65;
      case 0x483:
        break;
      case 0x484:
        if ((DAT_143ace690 != 0) && (FUN_142bf3f70(), DAT_143ace690 != 0)) {
          (*(code *)**(undefined8 **)(DAT_143ace690 + 8))((undefined8 *)(DAT_143ace690 + 8),1);
        }
        cVar7 = FUN_14090d340(0x280);
        if (cVar7 == '\0') {
          lVar19 = FUN_141892840();
          if (lVar19 != 0) {
            uVar11 = FUN_141892840();
            iVar9 = FUN_141829fd0(uVar11);
            if ((((iVar9 == 0x80) || (iVar9 == 0x82)) || (iVar9 == 0x84)) || (iVar9 == 0xb3)) break;
          }
          if (((((int)param_1[0x496] == 0) && (*(int *)((longlong)param_1 + 0x24ac) == 0)) &&
              (((char)param_1[0x770] == '\0' && (cVar7 = FUN_1424fdd50(), cVar7 == '\0')))) &&
             (((cVar7 = FUN_1424fdd30(), cVar7 == '\0' ||
               (*(int *)((longlong)param_1 + 0x2274) != 0)) &&
              ((DAT_143ace690 == 0 &&
               (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x458),
               local_528 != (longlong **)0x0)))))) {
            FUN_1424f5380(local_528);
          }
        }
        break;
      case 0x488:
        if ((DAT_143ad7748 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x358),
           local_528 != (longlong **)0x0)) {
          FUN_1424932e0(local_528);
        }
        break;
      case 0x489:
        if ((((DAT_143ad7978 == 0) || (FUN_142bf3f70(), DAT_143ad7978 == 0)) ||
            ((*(code *)**(undefined8 **)(DAT_143ad7978 + 8))((undefined8 *)(DAT_143ad7978 + 8),1),
            DAT_143ad7978 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x250),
           local_528 != (longlong **)0x0)) {
          FUN_1422f6c80(local_528);
        }
        break;
      case 0x48e:
        cVar7 = FUN_1417793f0(&DAT_143adb138);
        if (((cVar7 != '\0') && (DAT_143adb130 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x350),
           local_528 != (longlong **)0x0)) {
          FUN_1425f6d80(local_528);
        }
        break;
      case 0x48f:
        lVar19 = FUN_141892840();
        if (lVar19 == 0) {
          FUN_142cb4530(param_1,0x48f,0);
        }
        else if ((DAT_143acfbc0 == 0) &&
                (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2f8),
                local_528 != (longlong **)0x0)) {
          FUN_14249d060(local_528);
        }
        break;
      case 0x490:
        if ((DAT_143acfbb8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x330),
           local_528 != (longlong **)0x0)) {
          FUN_14249a9b0(local_528);
        }
        break;
      case 0x493:
        cVar7 = FUN_1417793f0(&DAT_143ad8338);
        if (((cVar7 != '\0') && (DAT_143ad8330 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x308),
           local_528 != (longlong **)0x0)) {
          FUN_1423fdd50(local_528);
        }
        break;
      case 0x496:
        if ((DAT_143ad80d8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x360),
           local_528 != (longlong **)0x0)) {
          FUN_14236c7d0(local_528);
        }
        break;
      case 0x498:
        if ((((DAT_143acf5d8 == 0) || (FUN_142bf3f70(), DAT_143acf5d8 == 0)) ||
            ((*(code *)**(undefined8 **)(DAT_143acf5d8 + 8))((undefined8 *)(DAT_143acf5d8 + 8),1),
            DAT_143acf5d8 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x440),
           local_528 != (longlong **)0x0)) {
          FUN_142450c90(local_528);
        }
        break;
      case 0x49c:
        if ((DAT_143acf5e0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x300),
           local_528 != (longlong **)0x0)) {
          FUN_14245ff40(local_528);
        }
        break;
      case 0x49d:
        if ((DAT_143ad85f8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2e8),
           local_528 != (longlong **)0x0)) {
          FUN_1424603e0(local_528);
        }
        break;
      case 0x49e:
        FUN_142d20b10();
        break;
      case 0x49f:
        lVar19 = FUN_141892840();
        if (lVar19 != 0) {
          uVar11 = FUN_141892840();
          iVar9 = FUN_141829fd0(uVar11);
          if (((iVar9 == 0xb4) && (DAT_143ad78c0 == 0)) &&
             (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x318),
             local_528 != (longlong **)0x0)) {
            FUN_1422dd9a0(local_528);
          }
        }
        break;
      case 0x4a0:
        if ((DAT_143acf0c8 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x308),
           local_528 != (longlong **)0x0)) {
          FUN_1422de8d0(local_528);
        }
        break;
      case 0x4a8:
        lVar19 = FUN_141892840();
        if (lVar19 != 0) {
          uVar11 = FUN_141892840();
          iVar9 = FUN_141829fd0(uVar11);
          if (((iVar9 == 0xbe) && (DAT_143ace020 == 0)) &&
             (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,800),
             local_528 != (longlong **)0x0)) {
            FUN_1423679c0(local_528);
          }
        }
        break;
      case 0x4a9:
        lVar19 = FUN_141892840();
        if (lVar19 != 0) {
          uVar11 = FUN_141892840();
          iVar9 = FUN_141829fd0(uVar11);
          if (((iVar9 == 0xbe) && (DAT_143ace028 == 0)) &&
             (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x358),
             local_528 != (longlong **)0x0)) {
            FUN_142368a60(local_528);
          }
        }
        break;
      case 0x4b0:
        if ((DAT_143ad72a0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x368),
           local_528 != (longlong **)0x0)) {
          FUN_1421734d0(local_528);
        }
        break;
      case 0x4b9:
        if ((((DAT_143adb188 == 0) || (FUN_142bf3f70(), DAT_143adb188 == 0)) ||
            ((*(code *)**(undefined8 **)(DAT_143adb188 + 8))((undefined8 *)(DAT_143adb188 + 8),1),
            DAT_143adb188 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x1680),
           local_528 != (longlong **)0x0)) {
          FUN_14260c070(local_528);
        }
        break;
      case 0x4cc:
        if ((((DAT_143acfc70 == 0) || (FUN_142bf3f70(), DAT_143acfc70 == 0)) ||
            ((*(code *)**(undefined8 **)(DAT_143acfc70 + 8))((undefined8 *)(DAT_143acfc70 + 8),1),
            DAT_143acfc70 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x1a98),
           local_528 != (longlong **)0x0)) {
          FUN_1424b27b0(local_528);
        }
        break;
      case 0x4d2:
        if ((((DAT_143ad00b0 == 0) || (FUN_142bf3f70(), DAT_143ad00b0 == 0)) ||
            ((*(code *)**(undefined8 **)(DAT_143ad00b0 + 8))((undefined8 *)(DAT_143ad00b0 + 8),1),
            DAT_143ad00b0 == 0)) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x29c8),
           local_528 != (longlong **)0x0)) {
          FUN_14274db40(local_528);
        }
        break;
      case 0x4d6:
        if ((DAT_143ad3bb0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x308),
           local_528 != (longlong **)0x0)) {
          FUN_141fc9da0(local_528);
        }
        break;
      case 0x4e9:
        if ((char)param_1[0x48a] != '\0') {
          FUN_142cb4530(param_1,0x4ec,0);
          FUN_142cb4530(param_1,0x4e9,0);
          if ((DAT_143acdb40 == 0) &&
             (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x368),
             local_528 != (longlong **)0x0)) {
            FUN_1414ed1d0(local_528);
          }
          break;
        }
        uVar11 = 0x196;
        pplVar17 = &local_520;
        goto LAB_142cb1555;
      case 0x4ec:
        FUN_142cb4530(param_1,0x4ec,0);
        FUN_142cb4530(param_1,0x4e9,0);
        if ((DAT_143acdb48 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x310),
           local_528 != (longlong **)0x0)) {
          FUN_1414efe60(local_528);
        }
        break;
      case 0x501:
        FUN_141445ea0();
        break;
      case 0x50a:
        FUN_1414b1930();
        break;
      case 0x514:
        FUN_141490ea0();
        break;
      case 0x524:
        FUN_142580440();
        break;
      case 0x531:
        FUN_1413afa30();
        break;
      case 0x537:
        FUN_1425c59d0();
      }
    }
    else {
      switch(param_2) {
      case 0x57a:
        FUN_14139b830();
        break;
      default:
switchD_142cb121a_caseD_1:
        FUN_14139da10(param_2);
        local_500 = (longlong *)0x0;
        puVar20 = (undefined8 *)param_1[0x649];
        cVar7 = *(char *)((longlong)puVar20[1] + 0x19);
        puVar4 = puVar20;
        puVar22 = (undefined8 *)puVar20[1];
        while (cVar7 == '\0') {
          if (*(int *)(puVar22 + 4) < param_2) {
            puVar21 = (undefined8 *)puVar22[2];
            puVar22 = puVar4;
          }
          else {
            puVar21 = (undefined8 *)*puVar22;
          }
          puVar4 = puVar22;
          puVar22 = puVar21;
          cVar7 = *(char *)((longlong)puVar21 + 0x19);
        }
        if (((*(char *)((longlong)puVar4 + 0x19) != '\0') || (param_2 < *(int *)(puVar4 + 4))) ||
           (puVar4 == puVar20)) {
          FUN_141b1f960(DAT_143abea80,1);
        }
        break;
      case 0x587:
        FUN_14274c9a0();
        break;
      case 0x590:
        FUN_1414a92c0();
        break;
      case 0x591:
        FUN_1414a09c0();
        break;
      case 0x593:
        if ((*(int *)((longlong)param_1 + 0x24ac) == 0) && ((int)param_1[0x496] == 0)) {
          FUN_14135bbc0();
        }
        break;
      case 0x596:
        FUN_141382ae0();
        break;
      case 0x59e:
        FUN_141397570();
        break;
      case 0x5ab:
      case 0x625:
        FUN_1426fa840(param_2);
        break;
      case 0x5bf:
        FUN_141351270();
        break;
      case 0x5de:
        if ((DAT_143acbe00 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x348),
           local_528 != (longlong **)0x0)) {
          FUN_14134e830(local_528);
        }
        break;
      case 0x5df:
        iVar9 = 0;
        if (-1 < param_3) {
          iVar9 = param_3;
        }
        FUN_14273a990(iVar9);
        break;
      case 0x5e1:
        FUN_142619bb0();
        break;
      case 0x628:
        FUN_14118f820();
        break;
      case 0x64f:
        if ((DAT_143aca9a0 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x380),
           local_528 != (longlong **)0x0)) {
          FUN_141191b80(local_528);
        }
        break;
      case 0x651:
        if ((DAT_143aca128 == 0) &&
           (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x458),
           local_528 != (longlong **)0x0)) {
          FUN_1410e4be0(local_528);
        }
        break;
      case 0x652:
        break;
      }
    }
    goto switchD_142cb2db3_caseD_11c;
  }
  if (param_2 == 0x109) {
    if ((DAT_143ad86d8 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x308), local_528 != (longlong **)0x0)
       ) {
      FUN_142482c50(local_528);
    }
    goto switchD_142cb2db3_caseD_11c;
  }
  switch(param_2) {
  case 0:
    if ((DAT_143ac8240 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x4e0), local_528 != (longlong **)0x0)
       ) {
      FUN_142380260(local_528);
    }
    break;
  default:
    goto switchD_142cb121a_caseD_1;
  case 5:
    cVar7 = FUN_141a01990();
    if (cVar7 != '\0') {
      uVar11 = FUN_1408a9e40(&local_518,0x150f);
      FUN_1415eca30(uVar11,0xb);
      if (local_518 == (longlong *)0x0) {
        return;
      }
      FUN_14019f2c0(local_518 + -2);
      return;
    }
    if ((DAT_143acf0d8 == 0) &&
       (local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x3c8), local_520 != (longlong *)0x0))
    {
      FUN_1423b1900(local_520,CONCAT31(4,param_3 != -1) ^ 0x400);
    }
    break;
  case 6:
    if (param_1[0x577] == 0) {
      local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x438);
      lVar19 = 0;
      lVar14 = lVar19;
      if (local_520 != (longlong *)0x0) {
        lVar14 = FUN_1424d3b80(local_520,param_3);
      }
      if ((param_1[0x577] - 1U < 999) || (param_1[0x577] == -1)) {
        FUN_142e52ed0(0x447);
      }
      lVar13 = lVar14 + 0x18;
      if (lVar14 == 0) {
        lVar13 = lVar19;
      }
      if ((lVar13 != 0) && (lVar19 = lVar13 + -0x18, lVar19 != 0)) {
        if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
        UNLOCK();
      }
      lVar14 = param_1[0x577];
      param_1[0x577] = lVar19;
      if (lVar14 != 0) {
        if (0xffffe < *(longlong *)(lVar14 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = (longlong *)(lVar14 + 0x20);
        lVar19 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if (((int)lVar19 == 1) &&
           (puVar20 = (undefined8 *)(lVar14 + 0x18), puVar20 != (undefined8 *)0x0)) {
          (**(code **)*puVar20)(puVar20,1);
        }
      }
    }
    break;
  case 0x14:
    FUN_142cb4530(param_1,0xcb,0);
    break;
  case 0x16:
    if ((DAT_143ad81a8 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x18b0), local_528 != (longlong **)0x0
       )) {
      FUN_14238e130(local_528);
    }
    break;
  case 0x19:
    iVar9 = FUN_142d3c160(uVar8);
    if ((iVar9 == 0) ||
       ((lVar19 = param_1[0x46b], lVar19 != 0 &&
        (iVar9 = FUN_1401ba9d0(lVar19 + 0x27,*(undefined4 *)(lVar19 + 0x2f)), 9 < iVar9)))) {
      if (param_1[0x57d] == 0) {
        local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x3f0);
        lVar19 = 0;
        lVar14 = lVar19;
        if (local_520 != (longlong *)0x0) {
          lVar14 = FUN_14252f0c0(local_520);
        }
        if ((param_1[0x57d] - 1U < 999) || (param_1[0x57d] == -1)) {
          FUN_142e52ed0(0x447);
        }
        lVar13 = lVar14 + 0x18;
        if (lVar14 == 0) {
          lVar13 = lVar19;
        }
        if ((lVar13 != 0) && (lVar19 = lVar13 + -0x18, lVar19 != 0)) {
          if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
          UNLOCK();
        }
        lVar14 = param_1[0x57d];
        param_1[0x57d] = lVar19;
        if (lVar14 != 0) {
          if (0xffffe < *(longlong *)(lVar14 + 0x20) - 1U) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar12 = (longlong *)(lVar14 + 0x20);
          lVar19 = *plVar12;
          *plVar12 = *plVar12 + -1;
          UNLOCK();
          if (((int)lVar19 == 1) &&
             (puVar20 = (undefined8 *)(lVar14 + 0x18), puVar20 != (undefined8 *)0x0)) {
            (**(code **)*puVar20)(puVar20,1);
          }
        }
      }
      break;
    }
    uVar11 = 0xc73;
    pplVar17 = &local_518;
    goto LAB_142cb1555;
  case 0x1c:
    if (param_1[0x579] == 0) {
      local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x3f8);
      lVar19 = 0;
      lVar14 = lVar19;
      if (local_520 != (longlong *)0x0) {
        lVar14 = FUN_142498ca0(local_520);
      }
      if ((param_1[0x579] - 1U < 999) || (param_1[0x579] == -1)) {
        FUN_142e52ed0(0x447);
      }
      lVar13 = lVar14 + 0x18;
      if (lVar14 == 0) {
        lVar13 = lVar19;
      }
      if ((lVar13 != 0) && (lVar19 = lVar13 + -0x18, lVar19 != 0)) {
        if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
        UNLOCK();
      }
      lVar14 = param_1[0x579];
      param_1[0x579] = lVar19;
      if (lVar14 != 0) {
        if (0xffffe < *(longlong *)(lVar14 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = (longlong *)(lVar14 + 0x20);
        lVar19 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if (((int)lVar19 == 1) &&
           (puVar20 = (undefined8 *)(lVar14 + 0x18), puVar20 != (undefined8 *)0x0)) {
          (**(code **)*puVar20)(puVar20,1);
        }
      }
      *(int *)(DAT_143ac87a0 + 0x308) = (int)param_1[0x586];
    }
    break;
  case 0x1d:
    if (param_1[0x57b] == 0) {
      local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x2e0);
      lVar19 = 0;
      lVar14 = lVar19;
      if (local_520 != (longlong *)0x0) {
        lVar14 = FUN_142499660(local_520);
      }
      if ((param_1[0x57b] - 1U < 999) || (param_1[0x57b] == -1)) {
        FUN_142e52ed0(0x447);
      }
      lVar13 = lVar14 + 0x18;
      if (lVar14 == 0) {
        lVar13 = lVar19;
      }
      if ((lVar13 != 0) && (lVar19 = lVar13 + -0x18, lVar19 != 0)) {
        if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
        UNLOCK();
      }
      lVar14 = param_1[0x57b];
      param_1[0x57b] = lVar19;
      if (lVar14 != 0) {
        if (0xffffe < *(longlong *)(lVar14 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = (longlong *)(lVar14 + 0x20);
        lVar19 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if (((int)lVar19 == 1) &&
           (puVar20 = (undefined8 *)(lVar14 + 0x18), puVar20 != (undefined8 *)0x0)) {
          (**(code **)*puVar20)(puVar20,1);
        }
      }
      *(undefined1 *)((longlong)param_1 + 0x2c2c) = 1;
    }
    break;
  case 0x1e:
    if ((DAT_143ad8410 == 0) &&
       (local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x15b0), local_520 != (longlong *)0x0))
    {
      FUN_142417b70(local_520,0xffffffffffffffff);
    }
    break;
  case 0x23:
    if ((DAT_143ad74d0 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x25a0), local_528 != (longlong **)0x0
       )) {
      FUN_142222710(local_528);
    }
    break;
  case 0x28:
    lVar19 = 0;
    bVar3 = false;
    local_4e8 = local_4e8 & 0xffffffff00000000;
    if (param_1[0x573] == 0) {
      local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x14f0);
      lVar14 = lVar19;
      if (local_520 != (longlong *)0x0) {
        uVar8 = *(undefined4 *)((longlong)param_1 + 0x2c44);
        lVar14 = param_1[0x589];
        iVar9 = *(int *)((longlong)param_1 + 0x2c3c);
        lVar13 = param_1[0x587];
        uVar10 = FUN_141829f70(local_518);
        local_558 = iVar9;
        local_550 = (int)lVar14;
        local_548 = uVar8;
        lVar14 = FUN_142c235f0(local_520,&local_4e8,uVar10,(int)lVar13);
      }
      if ((param_1[0x573] - 1U < 999) || (param_1[0x573] == -1)) {
        FUN_142e52ed0(0x447);
      }
      lVar13 = lVar14 + 0x18;
      if (lVar14 == 0) {
        lVar13 = lVar19;
      }
      if ((lVar13 != 0) && (lVar19 = lVar13 + -0x18, lVar19 != 0)) {
        if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
        UNLOCK();
      }
      lVar14 = param_1[0x573];
      param_1[0x573] = lVar19;
      if (lVar14 != 0) {
        if (0xffffe < *(longlong *)(lVar14 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = (longlong *)(lVar14 + 0x20);
        lVar19 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if (((int)lVar19 == 1) &&
           (puVar20 = (undefined8 *)(lVar14 + 0x18), puVar20 != (undefined8 *)0x0)) {
          (**(code **)*puVar20)(puVar20,1);
        }
      }
      bVar3 = true;
      iVar9 = (int)local_4e8;
    }
    else {
      lVar19 = param_1[0x589];
      uVar8 = *(undefined4 *)((longlong)param_1 + 0x2c3c);
      lVar14 = param_1[0x587];
      uVar10 = FUN_141829f70(plVar12);
      local_558 = (int)lVar19;
      iVar9 = FUN_142d9ed00(param_1,uVar10,(int)lVar14,uVar8);
      local_4e8 = CONCAT44(local_4e8._4_4_,iVar9);
    }
    if (iVar9 == 0) {
      if ((bVar3) && (param_1[0x573] != 0)) {
        uVar11 = FUN_142d2ec50(param_1 + 0x572);
        FUN_142bf3f70(uVar11);
        FUN_142d3a370(param_1 + 0x572);
      }
      uVar11 = 0x100b;
      if ((int)param_1[0x589] < 1) {
        uVar11 = 0xac;
      }
      uVar11 = FUN_1408a9e40(&local_520,uVar11);
      local_530 = 0;
      local_538 = 0;
      local_540 = 0;
      local_548 = 0;
      local_550 = 0;
      local_558 = 0;
      FUN_142a26280(uVar11,0,0,1);
    }
    param_1[0x587] = 0;
    *(undefined4 *)(param_1 + 0x589) = 0xffffffff;
    *(undefined4 *)((longlong)param_1 + 0x2c44) = 999999999;
    param_2 = (int)local_510;
    break;
  case 0x29:
    if (((param_1[0x575] == 0) && (lVar19 = param_1[0x46b], lVar19 != 0)) &&
       (iVar9 = FUN_1401ba9d0(lVar19 + 0x27,*(undefined4 *)(lVar19 + 0x2f)), 9 < iVar9)) {
      uVar8 = FUN_1429e3ef0();
      *(undefined4 *)(param_1 + 0x58b) = uVar8;
      local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x1408);
      lVar19 = 0;
      lVar14 = lVar19;
      if (local_520 != (longlong *)0x0) {
        lVar14 = FUN_142c364c0(local_520);
      }
      if ((param_1[0x575] - 1U < 999) || (param_1[0x575] == -1)) {
        FUN_142e52ed0(0x447);
      }
      lVar13 = lVar14 + 0x18;
      if (lVar14 == 0) {
        lVar13 = lVar19;
      }
      if ((lVar13 != 0) && (lVar19 = lVar13 + -0x18, lVar19 != 0)) {
        if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
        UNLOCK();
      }
      lVar14 = param_1[0x575];
      param_1[0x575] = lVar19;
      if (lVar14 != 0) {
        if (0xffffe < *(longlong *)(lVar14 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = (longlong *)(lVar14 + 0x20);
        lVar19 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if (((int)lVar19 == 1) &&
           (puVar20 = (undefined8 *)(lVar14 + 0x18), puVar20 != (undefined8 *)0x0)) {
          (**(code **)*puVar20)(puVar20,1);
        }
      }
    }
    break;
  case 0x2a:
    uVar8 = (*DAT_143262db0)();
    cVar7 = FUN_1408fc970((int)param_1[0x6fa],200,uVar8);
    if (cVar7 != '\0') {
      *(undefined4 *)(param_1 + 0x6fa) = uVar8;
      lVar19 = DAT_143ad82a0;
      if (DAT_143ad82a0 == 0) {
        local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x1ba0);
        if (local_520 != (longlong *)0x0) {
          FUN_1423cf310(local_520);
        }
      }
      else {
        lVar14 = *(longlong *)(DAT_143ad82a0 + 0x19a8);
        if (lVar14 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar14 = *(longlong *)(lVar19 + 0x19a8);
        }
        if (0 < *(int *)(lVar14 + 0x9c)) {
          FUN_14170af70(lVar14,0,1);
        }
      }
    }
    break;
  case 0x2b:
  case 0x2c:
  case 0x2d:
  case 0x2e:
  case 0x40:
  case 0x56:
switchD_142cb121a_caseD_2b:
    iVar9 = FUN_142746e40(param_2);
    if (param_1[((longlong)iVar9 + 0x2af) * 2] == 0) {
      local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x3c8);
      if (local_528 == (longlong **)0x0) {
        uVar11 = 0;
      }
      else {
        uVar11 = FUN_14221a750(local_528,param_2);
      }
      iVar9 = FUN_142746e40(param_2);
      FUN_142d2e060(param_1 + (longlong)iVar9 * 2 + 0x55d,uVar11);
    }
    break;
  case 0x37:
    if ((DAT_143ad7ab8 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x14e8), local_528 != (longlong **)0x0
       )) {
      FUN_142320230(local_528);
    }
    break;
  case 0x48:
    if (param_1[0x57f] == 0) {
      uVar8 = FUN_1429e3ef0();
      *(undefined4 *)((longlong)param_1 + 0x2c5c) = uVar8;
      local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x370);
      lVar19 = 0;
      lVar14 = lVar19;
      if (local_520 != (longlong *)0x0) {
        lVar14 = FUN_1422eec00(local_520);
      }
      if ((param_1[0x57f] - 1U < 999) || (param_1[0x57f] == -1)) {
        FUN_142e52ed0(0x447);
      }
      lVar13 = lVar14 + 0x18;
      if (lVar14 == 0) {
        lVar13 = lVar19;
      }
      if ((lVar13 != 0) && (lVar19 = lVar13 + -0x18, lVar19 != 0)) {
        if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
        UNLOCK();
      }
      lVar14 = param_1[0x57f];
      param_1[0x57f] = lVar19;
      if (lVar14 != 0) {
        if (0xffffe < *(longlong *)(lVar14 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = (longlong *)(lVar14 + 0x20);
        lVar19 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if (((int)lVar19 == 1) &&
           (puVar20 = (undefined8 *)(lVar14 + 0x18), puVar20 != (undefined8 *)0x0)) {
          (**(code **)*puVar20)(puVar20,1);
        }
      }
    }
    break;
  case 0x4b:
    if (param_1[0x571] != 0) break;
    FUN_14029bf50(local_4e0);
    FUN_142d96c80(param_1);
    if ((local_4c8[0] != 0) && (uVar2 = *(uint *)(local_4c8[0] + -8), uVar2 != 0)) {
      uVar16 = FUN_1407386b0(&DAT_143ac1ab0);
      uVar16 = (uVar16 & 0xffffffff) % (ulonglong)uVar2;
      uVar24 = 0;
      uVar25 = uVar24;
      if ((local_4c8[0] == 0) ||
         (uVar25 = (ulonglong)*(uint *)(local_4c8[0] + -8),
         *(uint *)(local_4c8[0] + -8) <= (uint)uVar16)) {
        FUN_142e54290(0xbc,uVar16,uVar25);
      }
      local_510 = 0;
      FUN_14019a260(&local_510,local_4c8[0] + uVar16 * 8);
      local_520 = &local_510;
      local_508 = (longlong **)0x0;
      FUN_14019a260(&local_508,&local_510);
      local_500 = (longlong *)0x0;
      if (local_510 != 0) {
        FUN_14019f2c0(local_510 + -0x10);
      }
      local_518 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x1450);
      pplVar17 = local_508;
      if (local_518 != (longlong *)0x0) {
        local_4f8 = (longlong **)0x0;
        pplVar6 = local_4f8;
        if ((local_508 != (longlong **)0x0) &&
           (pplVar1 = local_508 + -2, pplVar1 != (longlong **)0x0)) {
          if (*(int *)pplVar1 == -1) {
            FUN_142e52d50(0xcb,0xffffff01);
            uVar25 = 0xffffffffffffffff;
            uVar16 = 0xffffffffffffffff;
            do {
              uVar16 = uVar16 + 1;
            } while (*(char *)((longlong)pplVar17 + uVar16) != '\0');
            iVar23 = (int)uVar16;
            iVar9 = 0;
            if (0 < iVar23) {
              iVar9 = iVar23;
            }
            piVar15 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
            piVar15[1] = iVar9;
            *piVar15 = -1;
            pplVar6 = (longlong **)(piVar15 + 4);
            piVar15[2] = 0;
            *(undefined1 *)pplVar6 = 0;
            local_520 = (longlong *)(longlong)iVar23;
            local_528 = pplVar6;
            FUN_142ef7ba0(pplVar6,pplVar17,local_520);
            if (*piVar15 != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar23 == -1) || (iVar23 <= piVar15[1])) {
              *piVar15 = 1;
              if (iVar23 != -1) goto LAB_142cb1e73;
              if (pplVar6 != (longlong **)0x0) {
                do {
                  uVar25 = uVar25 + 1;
                } while (*(char *)((longlong)pplVar6 + uVar25) != '\0');
                uVar24 = uVar25 & 0xffffffff;
              }
            }
            else {
              FUN_142e54290(0x90,piVar15[1],uVar16 & 0xffffffff);
              *piVar15 = 1;
LAB_142cb1e73:
              *(undefined1 *)((longlong)local_520 + (longlong)pplVar6) = 0;
              uVar24 = uVar16;
            }
            iVar9 = (int)uVar24;
            if ((iVar9 < 0) || (piVar15[1] + 1 <= iVar9)) {
              FUN_142e54290(0x9c,uVar24 & 0xffffffff);
            }
            piVar15[2] = iVar9;
            if (local_4f8 != (longlong **)0x0) {
              FUN_14019f2c0(local_4f8 + -2);
            }
          }
          else {
            if (*(int *)pplVar1 < 1) {
              FUN_142e52dd0(0xd2);
            }
            LOCK();
            *(int *)pplVar1 = *(int *)pplVar1 + 1;
            UNLOCK();
            if (local_4f8 != (longlong **)0x0) {
              FUN_14019f2c0(local_4f8 + -2);
            }
            local_4f8 = pplVar17;
            pplVar17 = local_508;
            pplVar6 = local_4f8;
          }
        }
        local_4f8 = pplVar6;
        local_4f0 = 0;
        uVar24 = FUN_142252850(local_518,&local_4f8);
      }
      FUN_142d2e130(param_1 + 0x570,uVar24);
      if (pplVar17 != (longlong **)0x0) {
        FUN_14019f2c0(pplVar17 + -2);
      }
    }
    FUN_14022ea80(local_4c8);
    break;
  case 0x50:
    if ((DAT_143ad7528 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x360), local_528 != (longlong **)0x0)
       ) {
      FUN_142237e60(local_528);
    }
    break;
  case 0x52:
    if ((DAT_143ad84b8 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x14b0), local_528 != (longlong **)0x0
       )) {
      FUN_14243b460(local_528);
    }
    break;
  case 0x5b:
    if ((DAT_143ac9e80 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x360), local_528 != (longlong **)0x0)
       ) {
      FUN_1410a7bd0(local_528);
    }
    break;
  case 0x5d:
    if ((DAT_143ad7620 == 0) &&
       (pplVar17 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x318), local_528 = pplVar17,
       pplVar17 != (longlong **)0x0)) {
      local_518 = (longlong *)0x0;
      pcVar18 = (char *)FUN_14019bd40(&local_518,0x1c,0);
      plVar12 = local_518;
      uVar5 = s_UI_UIWindow2_img_mapleMuseum_1434948b8._12_4_;
      uVar10 = s_UI_UIWindow2_img_mapleMuseum_1434948b8._8_4_;
      uVar8 = s_UI_UIWindow2_img_mapleMuseum_1434948b8._4_4_;
      *(undefined4 *)pcVar18 = s_UI_UIWindow2_img_mapleMuseum_1434948b8._0_4_;
      *(undefined4 *)(pcVar18 + 4) = uVar8;
      *(undefined4 *)(pcVar18 + 8) = uVar10;
      *(undefined4 *)(pcVar18 + 0xc) = uVar5;
      *(undefined8 *)(pcVar18 + 0x10) = s_UI_UIWindow2_img_mapleMuseum_1434948b8._16_8_;
      *(undefined4 *)(pcVar18 + 0x18) = s_UI_UIWindow2_img_mapleMuseum_1434948b8._24_4_;
      if ((int)local_518[-2] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (*(int *)((longlong)plVar12 + -0xc) < 0x1c) {
        FUN_142e54290(0x90,*(int *)((longlong)plVar12 + -0xc),0x1c);
      }
      *(undefined4 *)(plVar12 + -2) = 1;
      *(char *)((longlong)local_518 + 0x1c) = '\0';
      if (*(int *)((longlong)plVar12 + -0xc) + 1 < 0x1d) {
        FUN_142e54290(0x9c,0x1c);
      }
      *(undefined4 *)(plVar12 + -1) = 0x1c;
      FUN_14225c3a0(pplVar17,0x212,0xfa,&local_518);
    }
    break;
  case 0x5e:
    if ((DAT_143ace3e0 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x348), local_528 != (longlong **)0x0)
       ) {
      FUN_142285550(local_528);
    }
    break;
  case 0x5f:
    if ((param_1[0x571] != 0) || (cVar7 = FUN_1408c71c0(param_1 + 0x5b0), cVar7 != '\0')) break;
    uVar8 = (undefined4)param_1[0x5b0];
    local_510 = CONCAT44(local_510._4_4_,uVar8);
    uVar16 = 0;
    local_518 = (longlong *)0x0;
    FUN_14019a260(&local_518,param_1 + 0x5b1);
    local_528 = &local_518;
    local_508 = (longlong **)0x0;
    FUN_14019a260(&local_508,&local_518);
    local_500 = (longlong *)CONCAT44(uVar8,1);
    if (local_518 != (longlong *)0x0) {
      FUN_14019f2c0(local_518 + -2);
    }
    local_4e8 = FUN_14019b780(&DAT_143ad68a0,0x1450);
    pplVar17 = local_508;
    if (local_4e8 != 0) {
      local_4f8 = (longlong **)0x0;
      if ((local_508 != (longlong **)0x0) && (pplVar6 = local_508 + -2, pplVar6 != (longlong **)0x0)
         ) {
        if (*(int *)pplVar6 == -1) {
          FUN_142e52d50(0xcb,0xffffff01);
          uVar25 = 0xffffffffffffffff;
          uVar24 = 0xffffffffffffffff;
          do {
            uVar24 = uVar24 + 1;
          } while (*(char *)((longlong)pplVar17 + uVar24) != '\0');
          iVar23 = (int)uVar24;
          iVar9 = 0;
          if (0 < iVar23) {
            iVar9 = iVar23;
          }
          piVar15 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
          piVar15[1] = iVar9;
          *piVar15 = -1;
          pplVar6 = (longlong **)(piVar15 + 4);
          piVar15[2] = 0;
          *(undefined1 *)pplVar6 = 0;
          local_520 = (longlong *)(longlong)iVar23;
          local_528 = pplVar6;
          FUN_142ef7ba0(pplVar6,pplVar17,local_520);
          if (*piVar15 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar23 == -1) || (iVar23 <= piVar15[1])) {
            *piVar15 = 1;
            if (iVar23 != -1) goto LAB_142cb20b4;
            if (pplVar6 != (longlong **)0x0) {
              do {
                uVar25 = uVar25 + 1;
              } while (*(char *)((longlong)pplVar6 + uVar25) != '\0');
              uVar16 = uVar25 & 0xffffffff;
            }
          }
          else {
            FUN_142e54290(0x90,piVar15[1],uVar24 & 0xffffffff);
            *piVar15 = 1;
LAB_142cb20b4:
            *(undefined1 *)((longlong)local_520 + (longlong)pplVar6) = 0;
            uVar16 = uVar24;
          }
          iVar9 = (int)uVar16;
          if ((iVar9 < 0) || (piVar15[1] + 1 <= iVar9)) {
            FUN_142e54290(0x9c,uVar16 & 0xffffffff);
          }
          piVar15[2] = iVar9;
          if (local_4f8 != (longlong **)0x0) {
            FUN_14019f2c0(local_4f8 + -2);
          }
          uVar8 = (int)local_510;
          local_4f8 = pplVar6;
        }
        else {
          if (*(int *)pplVar6 < 1) {
            FUN_142e52dd0(0xd2);
          }
          LOCK();
          *(int *)pplVar6 = *(int *)pplVar6 + 1;
          UNLOCK();
          if (local_4f8 != (longlong **)0x0) {
            FUN_14019f2c0(local_4f8 + -2);
          }
          local_4f8 = pplVar17;
          pplVar17 = local_508;
        }
      }
      local_4f0 = CONCAT44(uVar8,1);
      uVar16 = FUN_142252850(local_4e8,&local_4f8);
    }
    FUN_142d2e130(param_1 + 0x570,uVar16);
    if (pplVar17 != (longlong **)0x0) {
      FUN_14019f2c0(pplVar17 + -2);
    }
    break;
  case 0x60:
    if ((DAT_143ad7798 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x330), local_528 != (longlong **)0x0)
       ) {
      FUN_1422809b0(local_528);
    }
    break;
  case 99:
    if ((DAT_143ad8360 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x1458), local_528 != (longlong **)0x0
       )) {
      FUN_1423fec30(local_528);
    }
    break;
  case 0x65:
  case 0x66:
switchD_142cb121a_caseD_65:
    iVar9 = FUN_142746fb0(param_2);
    if (param_1[((longlong)iVar9 + 0x2a5) * 2] == 0) {
      local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x3c8);
      if (local_528 == (longlong **)0x0) {
        uVar11 = 0;
      }
      else {
        uVar11 = FUN_14221a750(local_528,param_2);
      }
      iVar9 = FUN_142746fb0(param_2);
      FUN_142d2e060(param_1 + (longlong)iVar9 * 2 + 0x549,uVar11);
    }
    break;
  case 0x6a:
    if (DAT_143adb828 == 0) {
      FUN_1402e01c0(param_1[0x46b],&local_518,0x303c,&DAT_14327b87c);
      if ((local_518 == (longlong *)0x0) || ((char)*local_518 == '\0')) {
        if (local_518 != (longlong *)0x0) {
          FUN_14019f2c0(local_518 + -2);
        }
      }
      else {
        uVar8 = FUN_142f11b44();
        if (DAT_143ad2008 != 0) {
          uVar10 = FUN_140938ee0(DAT_143ad2008,uVar8);
          local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x1398);
          if (local_528 != (longlong **)0x0) {
            FUN_142725620(local_528,uVar8,uVar10);
          }
        }
        if (local_518 != (longlong *)0x0) {
          FUN_14019f2c0(local_518 + -2);
        }
      }
    }
    break;
  case 0x6c:
    if ((DAT_143ad7b40 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x308), local_528 != (longlong **)0x0)
       ) {
      FUN_142337af0(local_528);
    }
    break;
  case 0x6f:
    if ((DAT_143adb880 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x310), local_528 != (longlong **)0x0)
       ) {
      FUN_1427396e0(local_528);
    }
    break;
  case 0x74:
    if ((DAT_143ad8420 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x310), local_528 != (longlong **)0x0)
       ) {
      FUN_142429e50(local_528,param_3);
    }
    break;
  case 0x7f:
    if ((DAT_143ad7530 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x360), local_528 != (longlong **)0x0)
       ) {
      FUN_14223b8f0(local_528);
    }
    break;
  case 0x88:
    if (DAT_143ad7630 == 0) {
      pplVar17 = (longlong **)FUN_14019b780(&DAT_143ad68a0,800);
      local_528 = pplVar17;
      if (pplVar17 != (longlong **)0x0) {
        local_518 = (longlong *)0x0;
        pcVar18 = (char *)FUN_14019bd40(&local_518,0x1d,0);
        plVar12 = local_518;
        uVar5 = s_UI_UIWindow2_img_mapleMuseum2_1434948d8._12_4_;
        uVar10 = s_UI_UIWindow2_img_mapleMuseum2_1434948d8._8_4_;
        uVar8 = s_UI_UIWindow2_img_mapleMuseum2_1434948d8._4_4_;
        *(undefined4 *)pcVar18 = s_UI_UIWindow2_img_mapleMuseum2_1434948d8._0_4_;
        *(undefined4 *)(pcVar18 + 4) = uVar8;
        *(undefined4 *)(pcVar18 + 8) = uVar10;
        *(undefined4 *)(pcVar18 + 0xc) = uVar5;
        *(undefined8 *)(pcVar18 + 0x10) = s_UI_UIWindow2_img_mapleMuseum2_1434948d8._16_8_;
        *(undefined4 *)(pcVar18 + 0x18) = s_UI_UIWindow2_img_mapleMuseum2_1434948d8._24_4_;
        pcVar18[0x1c] = s_UI_UIWindow2_img_mapleMuseum2_1434948d8[0x1c];
        if ((int)local_518[-2] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (*(int *)((longlong)plVar12 + -0xc) < 0x1d) {
          FUN_142e54290(0x90,*(int *)((longlong)plVar12 + -0xc),0x1d);
        }
        *(undefined4 *)(plVar12 + -2) = 1;
        *(char *)((longlong)local_518 + 0x1d) = '\0';
        if (*(int *)((longlong)plVar12 + -0xc) + 1 < 0x1e) {
          FUN_142e54290(0x9c);
        }
        *(undefined4 *)(plVar12 + -1) = 0x1d;
        FUN_14225f7e0(pplVar17,0x212,0x133,&local_518);
      }
      if (*(longlong *)(DAT_143ad7630 + 0x300) == 0) {
        iVar9 = 0;
      }
      else {
        iVar9 = *(int *)(*(longlong *)(DAT_143ad7630 + 0x300) + -8);
      }
      if (iVar9 == *(int *)(DAT_143ad7630 + 0x2f0)) {
        local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x2e8);
        lVar19 = 0;
        if (local_528 != (longlong **)0x0) {
          lVar19 = FUN_14225e0d0(local_528);
        }
        lVar14 = lVar19 + 0x18;
        if (lVar19 == 0) {
          lVar14 = 0;
        }
        if (lVar14 == 0) {
          local_500 = (longlong *)0x0;
        }
        else {
          local_500 = (longlong *)(lVar14 + -0x18);
          if (local_500 != (longlong *)0x0) {
            if (0xfffff < *(ulonglong *)(lVar14 + 8)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar14 + 8) = *(longlong *)(lVar14 + 8) + 1;
            UNLOCK();
          }
        }
        plVar12 = local_500;
        if (local_500 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
        }
        iVar9 = (**(code **)(*plVar12 + 0x130))(plVar12);
        if (iVar9 == 2) {
          FUN_142cb4530(DAT_143aa84a0,0x88,0);
        }
        if (0xffffe < plVar12[4] - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar12 = plVar12 + 4;
        lVar19 = *plVar12;
        *plVar12 = *plVar12 + -1;
        UNLOCK();
        if (((int)lVar19 == 1) && (plVar12 = local_500 + 3, plVar12 != (longlong *)0x0)) {
          (**(code **)*plVar12)(plVar12,1);
        }
      }
    }
    break;
  case 0x91:
    if ((DAT_143ad89e0 == 0) &&
       (local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x578), local_520 != (longlong *)0x0))
    {
      FUN_142554df0(local_520,param_3);
    }
    break;
  case 0x92:
    uVar8 = FUN_141892a90();
    iVar9 = FUN_14031e2b0(uVar8);
    if (((iVar9 == 0) && (DAT_143adac70 == 0)) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,800), local_528 != (longlong **)0x0))
    {
      FUN_1425704b0(local_528);
    }
    break;
  case 0x96:
    local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x13f8);
    if (local_528 != (longlong **)0x0) {
      FUN_142303c70(local_528);
    }
    break;
  case 0x9a:
    if ((DAT_143ad77e0 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x310), local_528 != (longlong **)0x0)
       ) {
      FUN_1422b8250(local_528);
    }
    break;
  case 0xa4:
    if ((DAT_143ad81c8 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,1000), local_528 != (longlong **)0x0))
    {
      FUN_14239a0f0(local_528);
    }
    break;
  case 0xaa:
    if ((DAT_143ad1830 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x3a8), local_528 != (longlong **)0x0)
       ) {
      FUN_142377970(local_528);
    }
    break;
  case 0xab:
    if ((DAT_143ad1838 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x370), local_528 != (longlong **)0x0)
       ) {
      FUN_14237b640(local_528);
    }
    break;
  case 0xac:
    if ((DAT_143ad1840 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x328), local_528 != (longlong **)0x0)
       ) {
      FUN_14237d870(local_528);
    }
    break;
  case 0xad:
    if ((DAT_143adb880 == 0) && (DAT_143acedd8 == 0)) {
      if (DAT_143ad84b0 == (longlong *)0x0) {
        uVar11 = FUN_1408a9e40(&local_520,0x150c);
        FUN_1415eca30(uVar11,0xb);
        if (local_520 != (longlong *)0x0) {
          FUN_14019f2c0(local_520 + -2);
        }
        plVar12 = DAT_143ad84b0;
        if (DAT_143ad84b0 == (longlong *)0x0) {
          local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x370);
          if (local_528 == (longlong **)0x0) {
            plVar12 = (longlong *)0x0;
          }
          else {
            plVar12 = (longlong *)FUN_142433810(local_528);
          }
        }
        (**(code **)(*plVar12 + 0x130))(plVar12);
      }
      break;
    }
    uVar11 = 0x60;
    pplVar17 = &local_520;
LAB_142cb1555:
    uVar11 = FUN_1408a9e40(pplVar17,uVar11);
    local_530 = 0;
    local_538 = 0;
    local_540 = 0;
    local_548 = 0;
    local_550 = 0;
    local_558 = 0;
    FUN_142a26280(uVar11,0,0,1);
    break;
  case 0xb0:
    if (param_1[0x542] == 0) {
      local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x260);
      lVar14 = 0;
      lVar19 = lVar14;
      if (local_528 != (longlong **)0x0) {
        lVar19 = FUN_1423b7d60(local_528);
      }
      if ((param_1[0x542] - 1U < 999) || (param_1[0x542] == -1)) {
        FUN_142e52ed0(0x447);
      }
      lVar13 = lVar19 + 0x18;
      if (lVar19 == 0) {
        lVar13 = lVar14;
      }
      local_500 = (longlong *)lVar14;
      if ((lVar13 != 0) && (local_500 = (longlong *)(lVar13 + -0x18), local_500 != (longlong *)0x0))
      {
        if (0xfffff < *(ulonglong *)(lVar13 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar13 + 8) = *(longlong *)(lVar13 + 8) + 1;
        UNLOCK();
      }
      plVar12 = (longlong *)param_1[0x542];
      param_1[0x542] = (longlong)local_500;
      local_500 = plVar12;
      FUN_142d39ee0(&local_508);
    }
    break;
  case 0xb1:
    if ((DAT_143ad1848 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x360), local_528 != (longlong **)0x0)
       ) {
      FUN_14237e930(local_528);
    }
    break;
  case 0xb7:
    cVar7 = FUN_142df3630(param_1);
    if (cVar7 == '\0') {
      uVar11 = FUN_1408a9e40(&local_520,0x86);
      FUN_140d84b70(uVar11,0);
      return;
    }
    if ((DAT_143addd18 == 0) || (cVar7 = FUN_142c42dd0(), cVar7 != '\0')) {
      uVar11 = 0x123f;
    }
    else {
      cVar7 = FUN_142c42de0(DAT_143addd18);
      if (cVar7 == '\0') {
        if (DAT_143addd10 == 0) {
          local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x308);
          if (local_528 == (longlong **)0x0) {
            plVar12 = (longlong *)0x0;
          }
          else {
            plVar12 = (longlong *)FUN_142c40680(local_528);
          }
          (**(code **)(*plVar12 + 0x130))(plVar12);
        }
        break;
      }
      uVar11 = 0x122f;
    }
    uVar11 = FUN_1408a9e40(&local_520,uVar11);
    FUN_140d84b70(uVar11,0);
    break;
  case 0xc6:
    if ((DAT_143accdf0 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,800), local_528 != (longlong **)0x0))
    {
      FUN_142339b10(local_528);
    }
    break;
  case 0xcb:
    FUN_142cb4530(param_1,0x14);
    break;
  case 0xd2:
    local_558 = (uint)local_558._1_3_ << 8;
    iVar9 = FUN_142d9c490(param_1,0x3851,0,0);
    if (((iVar9 == 1) && (DAT_143ad7758 == 0)) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x348), local_528 != (longlong **)0x0)
       ) {
      FUN_14227b620(local_528);
    }
    break;
  case 0xd3:
    local_558 = (uint)local_558._1_3_ << 8;
    iVar9 = FUN_142d9c490(param_1,0x3851,0,0);
    if (iVar9 == 1) {
      FUN_142d24280();
    }
    break;
  case 0xd5:
    if (DAT_143ad81e8 == 0) {
      local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x4e0);
    }
    else {
      if (*(int *)(DAT_143ad81e8 + 0x2ec) == param_3) break;
      FUN_142bf3f70();
      if (DAT_143ad81e8 != 0) {
        (*(code *)**(undefined8 **)(DAT_143ad81e8 + 8))((undefined8 *)(DAT_143ad81e8 + 8),1);
      }
      local_520 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x4e0);
    }
    if (local_520 != (longlong *)0x0) {
      FUN_1423a16c0(local_520,param_3);
    }
    break;
  case 0xfb:
    if ((DAT_143ad8398 == 0) &&
       (local_528 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x380), local_528 != (longlong **)0x0)
       ) {
      FUN_142406c10(local_528);
    }
  }
switchD_142cb2db3_caseD_11c:
  if (DAT_143adb220 != 0) {
    FUN_14261c370(DAT_143adb220,param_2);
  }
  return;
}



//===========================================================
// FUN_142499b60 @ 142499b60   (204 bytes)
//===========================================================

void FUN_142499b60(longlong *param_1,int param_2)

{
  longlong lVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  undefined1 local_res8 [8];
  
  FUN_142748b60();
  if (param_2 != 0) {
    FUN_1429edb20(PTR_u_BtMouseOver_143a47d28);
    lVar1 = param_1[0x4b];
    uVar4 = FUN_1408a9e40(local_res8,0x534);
    iVar2 = (**(code **)(*param_1 + 0x98))(param_1);
    iVar3 = (**(code **)(*param_1 + 0x90))(param_1);
    FUN_142645c50(lVar1,iVar3 + -100,iVar2 + 0x14,uVar4,200,1,1,0,0xffffffff);
    return;
  }
  FUN_142645170(param_1[0x4b]);
  return;
}



//===========================================================
// FUN_142498ca0 @ 142498ca0   (181 bytes)
//===========================================================

undefined8 * FUN_142498ca0(undefined8 *param_1)

{
  FUN_142be3a20(param_1,0x1c,3,0x18,3,10,3,0x126,6);
  *param_1 = &PTR_LAB_143465fc0;
  param_1[1] = &PTR_LAB_143466138;
  param_1[3] = &PTR_FUN_143466210;
  param_1[0x46] = &PTR_LAB_143466218;
  param_1[0x5b] = &PTR_FUN_143466238;
  param_1[0x7d] = 0;
  param_1[0x7e] = 0;
  FUN_142748170(param_1,0x138,0x185,0x271a);
  return param_1;
}



//===========================================================
// FUN_14117f280 @ 14117f280   (1122 bytes)
//===========================================================

void FUN_14117f280(longlong param_1)

{
  IUnknown *pIVar1;
  longlong *plVar2;
  longlong *plVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  longlong lVar8;
  undefined8 *puVar9;
  longlong lVar10;
  uint uVar11;
  longlong *aplStackX_8 [2];
  longlong *plStackX_18;
  longlong *plStackX_20;
  longlong lStack_88;
  longlong *plStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 uStack_70;
  undefined8 uStack_68;
  uint uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 uStack_48;
  
  FUN_142bf7e40();
  pIVar1 = *(IUnknown **)(param_1 + 0x25c0);
  if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&uStack_78);
  uVar11 = 0;
  if (DAT_143a8b8d8 == 8) {
    if ((short)uStack_78 == 8) {
      uStack_78 = (uint)uStack_78._2_2_ << 0x10;
      if (uStack_70 != 0) {
        (*DAT_143ad5990)(uStack_70 + -4);
      }
    }
    else {
      iVar4 = (*DAT_143262a18)(&uStack_78);
      if (iVar4 < 0) goto LAB_14117f6d2;
    }
    uStack_78 = CONCAT22(uStack_78._2_2_,8);
    if (DAT_143a8b8e0 != 0) {
      uVar11 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    uStack_70 = FUN_1401a5fa0(DAT_143a8b8e0,uVar11);
  }
  else {
    if (((short)uStack_78 == 8) && (uStack_78 = (uint)uStack_78._2_2_ << 0x10, uStack_70 != 0)) {
      (*DAT_143ad5990)(uStack_70 + -4);
    }
    iVar4 = (*DAT_143262a28)(&uStack_78,&DAT_143a8b8d8);
    if (iVar4 < 0) {
LAB_14117f6d2:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar4);
    }
  }
  aplStackX_8[0] = (longlong *)0x0;
  uStack_58 = uStack_78;
  uStack_54 = uStack_74;
  uStack_50 = (undefined4)uStack_70;
  uStack_4c = uStack_70._4_4_;
  uStack_48 = uStack_68;
  iVar4 = (**(code **)(*(longlong *)pIVar1 + 0x240))(pIVar1,&uStack_58,aplStackX_8);
  if (iVar4 < 0) {
    _com_issue_errorex(iVar4,pIVar1,(_GUID *)&DAT_14327fcb0);
  }
  plVar3 = aplStackX_8[0];
  plStack_80 = aplStackX_8[0];
  if ((short)uStack_78 == 8) {
    uStack_78 = uStack_78 & 0xffff0000;
    if (uStack_70 != 0) {
      (*DAT_143ad5990)(uStack_70 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&uStack_78);
  }
  if (plVar3 == (longlong *)0x0) goto LAB_14117f6a6;
  aplStackX_8[0] = plVar3;
  (**(code **)(*plVar3 + 8))(plVar3);
  FUN_142aa14a0(aplStackX_8);
  if ((*(longlong *)(param_1 + 0x2638) == 0) ||
     (*(int *)(*(longlong *)(param_1 + 0x2638) + -8) == 0)) {
    plVar2 = *(longlong **)(param_1 + 0x25c8);
    if (plVar2 != (longlong *)0x0) {
      (**(code **)(*plVar2 + 0x2b8))(plVar2,1);
    }
    FUN_142aa21f0(param_1 + 0x2e0,L"claim",0);
    FUN_142aa21f0(param_1 + 0x2e0,L"refuse",0);
    FUN_142aa21f0(param_1 + 0x2e0,L"delete",0);
    FUN_141adbb50(param_1 + 0x2e0,aplStackX_8,L"empty");
    if (aplStackX_8[0] != (longlong *)0x0) {
      puVar9 = (undefined8 *)FUN_1408a9e40(&lStack_88,0x179b);
      uVar7 = *puVar9;
      if ((aplStackX_8[0] == (longlong *)0x0) ||
         (uVar5 = FUN_140319fa0(), aplStackX_8[0] == (longlong *)0x0)) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      uVar6 = FUN_140319f60();
      plStackX_18 = *(longlong **)(param_1 + 0x340);
      if (plStackX_18 != (longlong *)0x0) {
        (**(code **)(*plStackX_18 + 8))();
      }
      plStackX_20 = plVar3;
      (**(code **)(*plVar3 + 8))(plVar3);
      FUN_142a16240(&plStackX_20,&plStackX_18,uVar6,uVar5,0,uVar7,1,0xffffffff);
      if (lStack_88 != 0) {
        FUN_14019f2c0(lStack_88 + -0x10);
      }
    }
    if (aplStackX_8[0] != (longlong *)0x0) {
      (**(code **)(*aplStackX_8[0] + 0x10))();
    }
    goto LAB_14117f6a6;
  }
  plVar2 = *(longlong **)(param_1 + 0x25c8);
  if (plVar2 != (longlong *)0x0) {
    (**(code **)(*plVar2 + 0x2b8))(plVar2,0);
  }
  uVar11 = *(uint *)(param_1 + 0x2608);
  if (uVar11 == 0xffffffff) {
    FUN_142aa21f0(param_1 + 0x2e0,L"claim",0);
    FUN_142aa21f0(param_1 + 0x2e0,L"refuse",0);
LAB_14117f52c:
    uVar7 = 0;
LAB_14117f52f:
    FUN_142aa21f0(param_1 + 0x2e0,L"delete",uVar7);
  }
  else {
    lVar10 = *(longlong *)(param_1 + 0x2638);
    if ((lVar10 != 0) && (uVar11 < *(uint *)(lVar10 + -8))) {
      if ((int)uVar11 < 0) {
        FUN_142e54290(0xbc,uVar11);
        lVar10 = *(longlong *)(param_1 + 0x2638);
      }
      if (*(longlong *)(lVar10 + 8 + (longlong)(int)uVar11 * 0x10) == 0) goto LAB_14117f6a6;
      uVar7 = FUN_141182550(param_1 + 0x2638,*(undefined4 *)(param_1 + 0x2608));
      lVar8 = FUN_141182810(uVar7);
      lVar10 = param_1 + 0x2e0;
      if (*(int *)(lVar8 + 0x108) != 7) {
        FUN_142aa21f0(lVar10,L"claim",1);
        FUN_142aa21f0(lVar10,L"refuse",1);
        goto LAB_14117f52c;
      }
      FUN_142aa21f0(lVar10,L"claim",0);
      FUN_142aa21f0(lVar10,L"refuse",0);
      uVar7 = 1;
      goto LAB_14117f52f;
    }
  }
  aplStackX_8[0] = plVar3;
  (**(code **)(*plVar3 + 8))(plVar3);
  FUN_1411806d0(param_1,aplStackX_8);
  aplStackX_8[0] = plVar3;
  (**(code **)(*plVar3 + 8))(plVar3);
  FUN_141180f80(param_1,aplStackX_8);
LAB_14117f6a6:
  if (plVar3 != (longlong *)0x0) {
    (**(code **)(*plVar3 + 0x10))(plVar3);
  }
  return;
}



//===========================================================
// FUN_1411828e0 @ 1411828e0   (101 bytes)
//===========================================================

longlong FUN_1411828e0(longlong param_1,ulonglong param_2)

{
  FUN_141182690(param_1 + 0x28);
  *(undefined ***)(param_1 + 0x20) = &PTR_FUN_1433884c8;
  FUN_1401d5120(param_1);
  if ((param_2 & 1) != 0) {
    if ((param_2 & 4) == 0) {
      FUN_141182700();
      return param_1;
    }
    FUN_140128740(param_1,0x540);
  }
  return param_1;
}



//===========================================================
// FUN_141182840 @ 141182840   (40 bytes)
//===========================================================

longlong FUN_141182840(longlong param_1)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 8);
  if (lVar1 == 0) {
    FUN_142e52ed0(0x428,0);
    lVar1 = *(longlong *)(param_1 + 8);
  }
  return lVar1;
}



//===========================================================
// FUN_142cb4530 @ 142cb4530   (4718 bytes)
//===========================================================

void FUN_142cb4530(longlong param_1,int param_2)

{
  int iVar1;
  undefined8 uVar2;
  longlong lVar3;
  
  if (param_2 < 0x10a) {
    if (param_2 == 0x109) {
      if (DAT_143ad86d8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad86d8;
    }
    else {
      switch(param_2) {
      case 0:
        if (DAT_143ac8240 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ac8240;
        break;
      default:
        goto switchD_142cb4577_caseD_1;
      case 5:
        if (DAT_143acf0d8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143acf0d8;
        break;
      case 6:
        if (*(longlong *)(param_1 + 0x2bb8) == 0) {
          return;
        }
        uVar2 = FUN_142504f80(param_1 + 0x2bb0);
        FUN_142bf3f70(uVar2);
        FUN_142d3a060(param_1 + 0x2bb0);
        return;
      case 0xd:
        if (DAT_143ad20e8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad20e8;
        break;
      case 0x16:
        if (DAT_143ad81a8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad81a8;
        break;
      case 0x19:
        if (*(longlong *)(param_1 + 0x2be8) == 0) {
          return;
        }
        uVar2 = FUN_142d2ebc0(param_1 + 0x2be0);
        FUN_142bf3f70(uVar2);
        FUN_142d3a150(param_1 + 0x2be0);
        return;
      case 0x1c:
        if (*(longlong *)(param_1 + 0x2bc8) == 0) {
          return;
        }
        uVar2 = FUN_142d2eb60(param_1 + 0x2bc0);
        FUN_142bf3f70(uVar2);
        FUN_142d39fe0(param_1 + 0x2bc0);
        return;
      case 0x1d:
        *(undefined1 *)(param_1 + 0x2c2c) = 0;
        if (*(longlong *)(param_1 + 0x2bd8) == 0) {
          return;
        }
        uVar2 = FUN_142d2eb30(param_1 + 0x2bd0);
        FUN_142bf3f70(uVar2);
        FUN_142d39f60(param_1 + 0x2bd0);
        return;
      case 0x1e:
        if (DAT_143ad8410 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad8410;
        break;
      case 0x23:
        if (DAT_143ad74d0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad74d0;
        break;
      case 0x28:
        if (*(longlong *)(param_1 + 0x2b98) == 0) {
          return;
        }
        uVar2 = FUN_142d2ec50(param_1 + 0x2b90);
        FUN_142bf3f70(uVar2);
        FUN_142d3a370(param_1 + 0x2b90);
        return;
      case 0x29:
        if (*(longlong *)(param_1 + 0x2ba8) == 0) {
          return;
        }
        uVar2 = FUN_142d2ec20(param_1 + 0x2ba0);
        FUN_142bf3f70(uVar2);
        FUN_142d3a250(param_1 + 0x2ba0);
        return;
      case 0x2a:
        if (DAT_143ad82a0 == 0) {
          return;
        }
        iVar1 = FUN_1423e1040();
        if (iVar1 != 0) {
          return;
        }
        if (DAT_143ad82a0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad82a0;
        break;
      case 0x2b:
      case 0x2c:
      case 0x2d:
      case 0x2e:
      case 0x40:
      case 0x56:
switchD_142cb4577_caseD_2b:
        iVar1 = FUN_142746e40(param_2);
        if (*(longlong *)(param_1 + ((longlong)iVar1 + 0x2af) * 0x10) == 0) {
          return;
        }
        iVar1 = FUN_142746e40(param_2);
        uVar2 = FUN_142d2ea70((longlong)iVar1 * 0x10 + 0x2ae8 + param_1);
        FUN_142bf3f70(uVar2);
        iVar1 = FUN_142746e40(param_2);
        FUN_142d39ce0((longlong)iVar1 * 0x10 + 0x2ae8 + param_1);
        return;
      case 0x37:
        if (DAT_143ad7ab8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7ab8;
        break;
      case 0x48:
        if (*(longlong *)(param_1 + 0x2bf8) == 0) {
          return;
        }
        uVar2 = FUN_142d2ead0(param_1 + 0x2bf0);
        FUN_142bf3f70(uVar2);
        FUN_142d39de0(param_1 + 0x2bf0);
        return;
      case 0x4b:
        if (*(longlong *)(param_1 + 0x2b88) == 0) {
          return;
        }
        uVar2 = FUN_142d2eaa0(param_1 + 0x2b80);
        FUN_142bf3f70(uVar2);
        FUN_142d39d60(param_1 + 0x2b80);
        return;
      case 0x50:
        if (DAT_143ad7528 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7528;
        break;
      case 0x52:
        if (DAT_143ad84b8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad84b8;
        break;
      case 0x5b:
        if (DAT_143ac9e80 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ac9e80;
        break;
      case 0x5d:
        if (DAT_143ad7620 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7620;
        break;
      case 0x5e:
        if (DAT_143ace3e0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ace3e0;
        break;
      case 0x60:
        if (DAT_143ad7798 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7798;
        break;
      case 99:
        if (DAT_143ad8360 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad8360;
        break;
      case 0x65:
      case 0x66:
switchD_142cb4577_caseD_65:
        iVar1 = FUN_142746fb0(param_2);
        if (*(longlong *)(param_1 + ((longlong)iVar1 + 0x2a5) * 0x10) == 0) {
          return;
        }
        iVar1 = FUN_142746fb0(param_2);
        uVar2 = FUN_142d2ea70((longlong)iVar1 * 0x10 + 0x2a48 + param_1);
        FUN_142bf3f70(uVar2);
        iVar1 = FUN_142746fb0(param_2);
        FUN_142d39ce0((longlong)iVar1 * 0x10 + 0x2a48 + param_1);
        return;
      case 0x6a:
        if (DAT_143adb828 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143adb828;
        break;
      case 0x6c:
        if (DAT_143ad7b40 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7b40;
        break;
      case 0x6f:
        if (DAT_143adb880 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143adb880;
        break;
      case 0x74:
        if (DAT_143ad8420 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad8420;
        break;
      case 0x7f:
        if (DAT_143ad7530 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7530;
        break;
      case 0x88:
        if (DAT_143ad7630 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7630;
        break;
      case 0x8b:
        if (DAT_143ac8fd8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ac8fd8;
        break;
      case 0x91:
        if (DAT_143ad89e0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad89e0;
        break;
      case 0x92:
        if (DAT_143adac70 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143adac70;
        break;
      case 0x9a:
        if (DAT_143ad77e0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad77e0;
        break;
      case 0xa4:
        if (DAT_143ad81c8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad81c8;
        break;
      case 0xaa:
        if (DAT_143ad1830 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad1830;
        break;
      case 0xab:
        if (DAT_143ad1838 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad1838;
        break;
      case 0xac:
        if (DAT_143ad1840 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad1840;
        break;
      case 0xad:
        if (DAT_143ad84b0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad84b0;
        break;
      case 0xb0:
        if (*(longlong *)(param_1 + 0x2a10) == 0) {
          return;
        }
        uVar2 = FUN_142d2eb00(param_1 + 0x2a08);
        FUN_142bf3f70(uVar2);
        FUN_142d39ee0(param_1 + 0x2a08);
        return;
      case 0xb1:
        if (DAT_143ad1848 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad1848;
        break;
      case 0xb7:
        if (DAT_143addd10 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143addd10;
        break;
      case 0xc3:
        if (DAT_143ad20f0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad20f0;
        break;
      case 0xc4:
        if (DAT_143acedd8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143acedd8;
        break;
      case 0xc6:
        if (DAT_143accdf0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143accdf0;
        break;
      case 0xd2:
        if (DAT_143ad7758 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7758;
        break;
      case 0xd3:
        if (DAT_143ad7760 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7760;
        break;
      case 0xd4:
        if (DAT_143adb678 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143adb678;
        break;
      case 0xd5:
        if (DAT_143ad81e8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad81e8;
        break;
      case 0xf3:
        if (DAT_143acf0b0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143acf0b0;
        break;
      case 0xfb:
        if (DAT_143ad8398 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad8398;
      }
    }
  }
  else if (param_2 < 0x44f) {
    if (param_2 == 0x44e) {
      if (DAT_143aceac0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143aceac0;
    }
    else {
      switch(param_2) {
      case 0x10a:
        if (DAT_143acf2b0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143acf2b0;
        break;
      default:
        goto switchD_142cb4577_caseD_1;
      case 0x10d:
        if (DAT_143acf2b8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143acf2b8;
        break;
      case 0x111:
        if (DAT_143acfbc8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143acfbc8;
        break;
      case 0x114:
        if (DAT_143ad74d8 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad74d8;
        break;
      case 0x115:
        if (DAT_143ad74e0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad74e0;
        break;
      case 0x117:
      case 0x118:
      case 0x119:
        iVar1 = FUN_142747050(param_2);
        if (*(longlong *)(param_1 + ((longlong)iVar1 + 0x2a2) * 0x10) == 0) {
          return;
        }
        iVar1 = FUN_142747050(param_2);
        uVar2 = FUN_142d2ea70((longlong)iVar1 * 0x10 + 0x2a18 + param_1);
        FUN_142bf3f70(uVar2);
        iVar1 = FUN_142747050(param_2);
        FUN_142d39ce0((longlong)iVar1 * 0x10 + 0x2a18 + param_1);
        return;
      case 0x11c:
        if (DAT_143ad7940 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad7940;
        break;
      case 0x11d:
        goto switchD_142cb4577_caseD_2b;
      case 0x122:
        if (DAT_143adb698 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143adb698;
        break;
      case 0x123:
        if (DAT_143adb6a0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143adb6a0;
        break;
      case 0x124:
        if (DAT_143ad4868 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad4868;
        break;
      case 299:
        if (DAT_143ad27c0 == 0) {
          return;
        }
        FUN_142bf3f70();
        lVar3 = DAT_143ad27c0;
      }
    }
  }
  else if (param_2 < 0x579) {
    if (param_2 == 0x578) {
      FUN_14139a240();
      return;
    }
    switch(param_2) {
    case 0x452:
      if (DAT_143ad00e8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad00e8;
      break;
    default:
switchD_142cb4577_caseD_1:
      FUN_1411574a0(param_2,0);
      FUN_14139db80(param_2);
      FUN_142ca4590(param_1 + 0x3248,param_2);
      return;
    case 0x456:
      if (DAT_143ad8710 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad8710;
      break;
    case 0x457:
      if (DAT_143ad4668 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad4668;
      break;
    case 0x458:
      if (DAT_143ad4878 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad4878;
      break;
    case 0x459:
      if (DAT_143ad87f8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad87f8;
      break;
    case 0x45c:
      if (DAT_143ad7740 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad7740;
      break;
    case 0x46f:
      if (DAT_143ad85d8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad85d8;
      break;
    case 0x470:
      if (DAT_143ad85e0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad85e0;
      break;
    case 0x471:
      if (DAT_143ad85e8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad85e8;
      break;
    case 0x472:
      if (DAT_143ad4870 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad4870;
      break;
    case 0x477:
      if (DAT_143ad85f0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad85f0;
      break;
    case 0x47f:
      if (DAT_143ad7ac8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad7ac8;
      break;
    case 0x482:
    case 0x51b:
    case 0x51c:
    case 0x51d:
    case 0x51e:
    case 0x51f:
    case 0x520:
    case 0x521:
      goto switchD_142cb4577_caseD_65;
    case 0x488:
      if (DAT_143ad7748 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad7748;
      break;
    case 0x489:
      if (DAT_143ad7978 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad7978;
      break;
    case 0x48e:
      if (DAT_143adb130 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143adb130;
      break;
    case 0x48f:
      if (DAT_143acfbc0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acfbc0;
      break;
    case 0x490:
      if (DAT_143acfbb8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acfbb8;
      break;
    case 0x493:
      if (DAT_143ad8330 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad8330;
      break;
    case 0x496:
      if (DAT_143ad80d8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad80d8;
      break;
    case 0x498:
      if (DAT_143acf5d8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acf5d8;
      break;
    case 0x49c:
      if (DAT_143acf5e0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acf5e0;
      break;
    case 0x49d:
      if (DAT_143ad85f8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad85f8;
      break;
    case 0x49e:
      if (DAT_143ad78f8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad78f8;
      break;
    case 0x49f:
      if (DAT_143ad78c0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad78c0;
      break;
    case 0x4a0:
      if (DAT_143acf0c8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acf0c8;
      break;
    case 0x4a8:
      if (DAT_143ace020 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ace020;
      break;
    case 0x4a9:
      if (DAT_143ace028 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ace028;
      break;
    case 0x4b0:
      if (DAT_143ad72a0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad72a0;
      break;
    case 0x4b9:
      if (DAT_143adb188 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143adb188;
      break;
    case 0x4cc:
      if (DAT_143acfc70 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acfc70;
      break;
    case 0x4cd:
      if (DAT_143ad8878 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad8878;
      break;
    case 0x4d2:
      if (DAT_143ad00b0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad00b0;
      break;
    case 0x4d6:
      if (DAT_143ad3bb0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad3bb0;
      break;
    case 0x4d7:
      if (DAT_143ad3528 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143ad3528;
      break;
    case 0x4e9:
      if (DAT_143acdb40 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acdb40;
      break;
    case 0x4ec:
      if (DAT_143acdb48 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acdb48;
      break;
    case 0x4f4:
      if (DAT_143acd3e8 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acd3e8;
      break;
    case 0x501:
      FUN_141445ee0();
      return;
    case 0x50a:
      FUN_1414b1970();
      return;
    case 0x514:
      FUN_141490ee0();
      return;
    case 0x524:
      FUN_142580480();
      return;
    case 0x531:
      FUN_1413afa70();
      return;
    case 0x533:
    case 0x534:
      FUN_141428fa0(param_2);
      return;
    case 0x537:
      FUN_1425c5a10();
      return;
    }
  }
  else {
    switch(param_2) {
    case 0x57a:
      FUN_14139b870();
      return;
    default:
      goto switchD_142cb4577_caseD_1;
    case 0x581:
      FUN_142578ea0();
      return;
    case 0x587:
      FUN_14274c9e0();
      return;
    case 0x590:
      FUN_1414a9300();
      return;
    case 0x591:
      FUN_1414a0a00();
      return;
    case 0x593:
      if (DAT_143acbe70 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acbe70;
      break;
    case 0x596:
      FUN_141382b20();
      return;
    case 0x59e:
      FUN_1413975b0();
      return;
    case 0x5ab:
    case 0x625:
      FUN_1426fa890();
      return;
    case 0x5ad:
    case 0x620:
      FUN_1426fa8d0();
      return;
    case 0x5bf:
      FUN_1413512d0(1);
      return;
    case 0x5c3:
      if (DAT_143acd440 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acd440;
      break;
    case 0x5c5:
      FUN_14257b590();
      return;
    case 0x5de:
      if (DAT_143acbe00 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acbe00;
      break;
    case 0x5df:
      FUN_14273a9e0();
      return;
    case 0x5e1:
      if (DAT_143adb220 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143adb220;
      break;
    case 0x5e2:
      if (DAT_143adb228 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143adb228;
      break;
    case 0x624:
      if (DAT_143acbef0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143acbef0;
      break;
    case 0x628:
      FUN_14118f910();
      return;
    case 0x64f:
      if (DAT_143aca9a0 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143aca9a0;
      break;
    case 0x651:
      if (DAT_143aca128 == 0) {
        return;
      }
      FUN_142bf3f70();
      lVar3 = DAT_143aca128;
      break;
    case 0x652:
      goto switchD_142cb55af_caseD_652;
    }
  }
  if (lVar3 != 0) {
                    /* WARNING: Could not recover jumptable at 0x000142cb45b5. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)**(undefined8 **)(lVar3 + 8))(lVar3 + 8,1);
    return;
  }
switchD_142cb55af_caseD_652:
  return;
}


