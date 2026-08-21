
//===========================================================
// FUN_141e4a5d0 @ 141e4a5d0   (1012 bytes)
//===========================================================

void FUN_141e4a5d0(longlong param_1)

{
  uint uVar1;
  longlong lVar2;
  IUnknown *pIVar3;
  char cVar4;
  int iVar5;
  int iVar6;
  long lVar7;
  longlong lVar8;
  undefined8 uVar9;
  IUnknown *pIVar10;
  undefined4 local_res8 [2];
  
  pIVar10 = *(IUnknown **)(param_1 + 0x238);
  lVar8 = *(longlong *)(param_1 + 0x160) + -0x20;
  if (*(longlong *)(param_1 + 0x160) == 0) {
    lVar8 = 0;
  }
  if (pIVar10 == (IUnknown *)0x0) goto LAB_141e4a9bb;
  iVar5 = FUN_1409c6ce0(lVar8);
  iVar6 = FUN_1409c6cc0(lVar8);
  iVar5 = (**(code **)(*(longlong *)pIVar10 + 0x198))
                    (pIVar10,(iVar6 * 3000 - iVar5) * 10 + -0x3fff8ad5);
  if (iVar5 < 0) {
    _com_issue_errorex(iVar5,pIVar10,(_GUID *)&DAT_14327fcb0);
  }
  lVar2 = *(longlong *)(param_1 + 0x198);
  if ((lVar2 != 0) && (iVar5 = *(int *)(lVar2 + 0x29c), -1 < iVar5)) {
    iVar6 = *(int *)(lVar2 + 0x2a0);
    if (iVar6 < 0) {
      iVar6 = FUN_1409c6ce0(lVar8);
    }
    pIVar10 = *(IUnknown **)(param_1 + 0x238);
    if (pIVar10 == (IUnknown *)0x0) goto LAB_141e4a9bb;
    iVar5 = (**(code **)(*(longlong *)pIVar10 + 0x198))
                      (pIVar10,(iVar5 * 3000 - iVar6) * 10 + -0x3fff8ad5);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar10,(_GUID *)&DAT_14327fcb0);
    }
  }
  iVar5 = *(int *)(param_1 + 0x5a8);
  if (-1 < iVar5) {
    iVar6 = *(int *)(param_1 + 0x5ac);
    if (iVar6 < 0) {
      iVar6 = FUN_1409c6ce0(lVar8);
    }
    pIVar10 = *(IUnknown **)(param_1 + 0x238);
    if (pIVar10 == (IUnknown *)0x0) goto LAB_141e4a9bb;
    iVar5 = (**(code **)(*(longlong *)pIVar10 + 0x198))
                      (pIVar10,(iVar5 * 3000 - iVar6) * 10 + -0x3fff8ad5);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar10,(_GUID *)&DAT_14327fcb0);
    }
  }
  if (*(uint **)(param_1 + 0x198) != (uint *)0x0) {
    uVar1 = **(uint **)(param_1 + 0x198);
    if (uVar1 < 0x1781fb) {
      if ((((uVar1 == 0x1781fa) || (uVar1 == 0x17815b)) || (uVar1 == 0x1781f8)) ||
         (uVar1 == 0x1781f9)) {
LAB_141e4a755:
        pIVar10 = *(IUnknown **)(param_1 + 0x238);
        if (pIVar10 == (IUnknown *)0x0) goto LAB_141e4a9bb;
        lVar7 = (**(code **)(*(longlong *)pIVar10 + 0x198))(pIVar10,0xc0000000);
        if (lVar7 < 0) goto LAB_141e4a77d;
      }
    }
    else {
      if (uVar1 == 0x1781fb) goto LAB_141e4a755;
      if ((uVar1 != 0x895902) && (uVar1 != 0x895906)) goto LAB_141e4a78b;
      pIVar10 = *(IUnknown **)(param_1 + 0x238);
      if (pIVar10 == (IUnknown *)0x0) goto LAB_141e4a9bb;
      iVar5 = FUN_1409c6ce0(lVar8);
      iVar6 = FUN_1409c6cc0(lVar8);
      lVar7 = (**(code **)(*(longlong *)pIVar10 + 0x198))
                        (pIVar10,(iVar6 * 3000 - iVar5) * 10 + -0x3fff8ad9);
      if (-1 < lVar7) goto LAB_141e4a78b;
LAB_141e4a77d:
      _com_issue_errorex(lVar7,pIVar10,(_GUID *)&DAT_14327fcb0);
    }
  }
LAB_141e4a78b:
  iVar5 = 0;
  if ((*(longlong *)(param_1 + 0x238) != 0) && (lVar8 = FUN_141892840(), lVar8 != 0)) {
    uVar9 = FUN_141892840();
    cVar4 = FUN_141bc0990(uVar9);
    if (cVar4 != '\0') {
      if (*(longlong *)(param_1 + 0x198) != 0) {
        iVar5 = *(int *)(*(longlong *)(param_1 + 0x198) + 0x90);
      }
      pIVar10 = *(IUnknown **)(param_1 + 0x238);
      if (pIVar10 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      lVar8 = (**(code **)(*(longlong *)(param_1 + 8) + 0x30))(param_1 + 8,local_res8);
      iVar5 = (**(code **)(*(longlong *)pIVar10 + 0x198))
                        (pIVar10,(*(int *)(lVar8 + 4) + iVar5) * 10 + -0x3ffccbb0);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar10,(_GUID *)&DAT_14327fcb0);
      }
    }
  }
  lVar8 = *(longlong *)(param_1 + 0x108);
  if (lVar8 != 0) {
    pIVar10 = *(IUnknown **)(param_1 + 0x238);
    if (pIVar10 == (IUnknown *)0x0) goto LAB_141e4a9bb;
    local_res8[0] = 0;
    iVar5 = (**(code **)(*(longlong *)pIVar10 + 400))(pIVar10,local_res8);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar10,(_GUID *)&DAT_14327fcb0);
    }
    FUN_140f81230(lVar8,local_res8[0]);
  }
  lVar8 = *(longlong *)(param_1 + 0x128);
  if (lVar8 != 0) {
    pIVar10 = *(IUnknown **)(param_1 + 0x238);
    if (pIVar10 == (IUnknown *)0x0) goto LAB_141e4a9bb;
    local_res8[0] = 0;
    iVar5 = (**(code **)(*(longlong *)pIVar10 + 400))(pIVar10,local_res8);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar10,(_GUID *)&DAT_14327fcb0);
    }
    FUN_140f81230(lVar8,local_res8[0]);
  }
  pIVar10 = *(IUnknown **)(param_1 + 0x268);
  if (pIVar10 != (IUnknown *)0x0) {
    pIVar3 = *(IUnknown **)(param_1 + 0x238);
    if (pIVar3 == (IUnknown *)0x0) {
LAB_141e4a9bb:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8[0] = 0;
    iVar5 = (**(code **)(*(longlong *)pIVar3 + 400))(pIVar3,local_res8);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_14327fcb0);
    }
    iVar5 = (**(code **)(*(longlong *)pIVar10 + 0x198))(pIVar10,local_res8[0]);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar10,(_GUID *)&DAT_14327fcb0);
    }
  }
  return;
}



//===========================================================
// FUN_141cb9fd0 @ 141cb9fd0   (2032 bytes)
//===========================================================

void FUN_141cb9fd0(longlong param_1)

{
  uint uVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  longlong lVar5;
  longlong lVar6;
  IUnknown *pIVar7;
  int local_res8 [2];
  
  pIVar7 = *(IUnknown **)(param_1 + 0x610);
  lVar6 = *(longlong *)(param_1 + 0x2b8) + -0x20;
  if (*(longlong *)(param_1 + 0x2b8) == 0) {
    lVar6 = 0;
  }
  if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
  iVar3 = FUN_141cbaa80();
  iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))(pIVar7,iVar3 + 1);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
  }
  if (*(int *)(*(longlong *)(param_1 + 0x3a8) + 0x2d4) != 0) {
    pIVar7 = *(IUnknown **)(param_1 + 0x610);
    if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
    local_res8[0] = 0;
    iVar3 = (**(code **)(*(longlong *)pIVar7 + 400))(pIVar7,local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
    }
    iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))
                      (pIVar7,local_res8[0] + *(int *)(*(longlong *)(param_1 + 0x3a8) + 0x2d4));
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
    }
  }
  uVar1 = *(uint *)(*(longlong *)(param_1 + 0x3a8) + 0x60);
  if (uVar1 < 0x866e13) {
    if (uVar1 == 0x866e12) {
switchD_141cba196_caseD_866e14:
      pIVar7 = *(IUnknown **)(param_1 + 0x610);
      if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
      iVar3 = FUN_1409c6ce0(lVar6);
      iVar4 = FUN_1409c6cc0(lVar6);
      iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))
                        (pIVar7,(iVar4 * 3000 - iVar3) * 10 + -0x3fff8add);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
      }
    }
    else if (uVar1 < 0x864703) {
      if (uVar1 == 0x864702) {
switchD_141cba15b_caseD_864716:
        pIVar7 = *(IUnknown **)(param_1 + 0x610);
        if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))
                          (pIVar7,(iVar4 * 3000 - iVar3) * 10 + -0x3fff8adb);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
        }
      }
      else if ((uVar1 == 0x7dbb93) || (uVar1 == 0x7dbbae)) {
        pIVar7 = *(IUnknown **)(param_1 + 0x610);
        if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
        iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))(pIVar7,0xc0000000);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
        }
      }
    }
    else {
      switch(uVar1) {
      case 0x864703:
      case 0x864707:
      case 0x864717:
      case 0x86471b:
      case 0x864767:
      case 0x86476b:
      case 0x86478d:
      case 0x864791:
switchD_141cba15b_caseD_864703:
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8adc;
        break;
      case 0x864704:
      case 0x864708:
      case 0x864718:
      case 0x86471c:
      case 0x864768:
      case 0x86476c:
      case 0x86478e:
      case 0x864792:
switchD_141cba15b_caseD_864704:
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8add;
        break;
      case 0x864705:
      case 0x864709:
      case 0x864719:
      case 0x86471d:
      case 0x864769:
      case 0x86476d:
      case 0x86478f:
      case 0x864793:
switchD_141cba15b_caseD_864705:
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8ade;
        break;
      case 0x864706:
      case 0x86470a:
      case 0x86471a:
      case 0x86471e:
      case 0x86476a:
      case 0x86476e:
      case 0x864790:
      case 0x864794:
switchD_141cba15b_caseD_864706:
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8adf;
        break;
      default:
        goto switchD_141cba15b_caseD_86470b;
      case 0x864716:
      case 0x864766:
        goto switchD_141cba15b_caseD_864716;
      }
LAB_141cba57e:
      FUN_140dbb710(lVar5,iVar3);
    }
  }
  else if (uVar1 < 0x86bc31) {
    if (uVar1 != 0x86bc30) {
      switch(uVar1) {
      case 0x866e13:
      case 0x866e1b:
      case 0x866e77:
      case 0x866edb:
        goto switchD_141cba15b_caseD_864703;
      case 0x866e14:
      case 0x866e1a:
      case 0x866e1c:
      case 0x866e76:
      case 0x866e78:
      case 0x866eda:
      case 0x866edc:
        goto switchD_141cba196_caseD_866e14;
      case 0x866e15:
      case 0x866e16:
      case 0x866e1d:
      case 0x866e1e:
      case 0x866e79:
      case 0x866e7a:
      case 0x866edd:
      case 0x866ede:
        goto switchD_141cba15b_caseD_864705;
      case 0x866e17:
      case 0x866e1f:
      case 0x866e7b:
      case 0x866edf:
switchD_141cba196_caseD_866e17:
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = ((iVar4 * 3000 - iVar3) + -0x6665ab0) * 10;
        break;
      case 0x866e18:
      case 0x866e20:
      case 0x866e7c:
      case 0x866ee0:
        goto switchD_141cba15b_caseD_864706;
      case 0x866e19:
      case 0x866e21:
      case 0x866e7d:
      case 0x866ee1:
switchD_141cba196_caseD_866e19:
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8ae1;
        break;
      case 0x866e22:
      case 0x866e8a:
      case 0x866ee6:
switchD_141cba196_caseD_866e22:
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8adb;
        break;
      default:
        goto switchD_141cba15b_caseD_86470b;
      }
      goto LAB_141cba57e;
    }
switchD_141cba1ca_caseD_86bc33:
    pIVar7 = *(IUnknown **)(param_1 + 0x610);
    if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
    iVar3 = FUN_1409c6cc0(lVar6);
    iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))(pIVar7,iVar3 * 30000 + -0x3ffff831);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
    }
  }
  else {
    if (uVar1 < 0x878024) {
      if (uVar1 == 0x878023) {
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8ad0;
      }
      else {
        switch(uVar1) {
        case 0x86bc31:
        case 0x86bc32:
        case 0x86bc34:
        case 0x86bc35:
        case 0x86bc36:
        case 0x86bc38:
        case 0x86bc39:
        case 0x86bc3b:
        case 0x86bc3c:
        case 0x86bc3d:
          lVar5 = *(longlong *)(param_1 + 0x610);
          if (lVar5 == 0) goto LAB_141cba7b6;
          iVar3 = FUN_1409c6cc0(lVar6);
          iVar3 = iVar3 * 30000 + -0x3ffff830;
          break;
        case 0x86bc33:
        case 0x86bc37:
        case 0x86bc3a:
          goto switchD_141cba1ca_caseD_86bc33;
        default:
          goto switchD_141cba15b_caseD_86470b;
        }
      }
      goto LAB_141cba57e;
    }
    if (uVar1 < 0x8dea54) {
      if (uVar1 == 0x8dea53) goto switchD_141cba15b_caseD_864716;
      if (uVar1 == 0x878368) {
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6cc0(lVar6);
        iVar3 = iVar3 * 30000 + -0x3fff8b49;
        goto LAB_141cba57e;
      }
    }
    else if (uVar1 < 0x960d30) {
      if (uVar1 == 0x960d2f) {
        lVar5 = *(longlong *)(param_1 + 0x610);
        if (lVar5 == 0) goto LAB_141cba7b6;
        iVar3 = FUN_1409c6ce0(lVar6);
        iVar4 = FUN_1409c6cc0(lVar6);
        iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8ad8;
        goto LAB_141cba57e;
      }
      switch(uVar1) {
      case 0x8dea54:
      case 0x8dea55:
        goto switchD_141cba15b_caseD_864716;
      case 0x8dea56:
      case 0x8dea57:
      case 0x8dea6a:
      case 0x8dea88:
        goto switchD_141cba15b_caseD_864706;
      case 0x8dea58:
      case 0x8dea59:
      case 0x8dea67:
      case 0x8dea68:
      case 0x8dea85:
      case 0x8dea86:
        goto switchD_141cba15b_caseD_864705;
      case 0x8dea5a:
      case 0x8dea5b:
        goto switchD_141cba15b_caseD_864704;
      case 0x8dea5c:
      case 0x8dea5d:
      case 0x8dea65:
      case 0x8dea83:
        goto switchD_141cba15b_caseD_864703;
      case 0x8dea64:
      case 0x8dea66:
      case 0x8dea82:
      case 0x8dea84:
        goto switchD_141cba196_caseD_866e14;
      case 0x8dea69:
      case 0x8dea87:
        goto switchD_141cba196_caseD_866e17;
      case 0x8dea6b:
      case 0x8dea89:
        goto switchD_141cba196_caseD_866e19;
      case 0x8dea6c:
        goto switchD_141cba196_caseD_866e22;
      }
    }
  }
switchD_141cba15b_caseD_86470b:
  if (*(int *)(*(longlong *)(param_1 + 0x3a8) + 0x2d0) != 0) {
    pIVar7 = *(IUnknown **)(param_1 + 0x610);
    if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
    iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))(pIVar7,0xc0000000);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
    }
  }
  if (*(int *)(*(longlong *)(param_1 + 0x3a8) + 0x60) == 0x7dbb8f) {
    iVar3 = FUN_1401ba9d0(param_1 + 1000,*(undefined4 *)(param_1 + 0x3f0));
    pIVar7 = *(IUnknown **)(param_1 + 0x610);
    if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
    if (iVar3 == 5) {
      iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))(pIVar7,0xc0041eb0);
    }
    else {
      local_res8[0] = 0;
      iVar3 = (**(code **)(*(longlong *)pIVar7 + 400))(pIVar7,local_res8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
      }
      if (local_res8[0] != -0x3ffbe150) goto LAB_141cba6a6;
      pIVar7 = *(IUnknown **)(param_1 + 0x610);
      if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
      iVar3 = FUN_1409c6ce0(lVar6);
      iVar4 = FUN_1409c6cc0(lVar6);
      iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))
                        (pIVar7,(iVar4 * 3000 - iVar3) * 10 + -0x3fff8ad9);
    }
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
    }
  }
LAB_141cba6a6:
  if (*(int *)(*(longlong *)(param_1 + 0x3a8) + 0x178) - 1U < 2) {
    pIVar7 = *(IUnknown **)(param_1 + 0x610);
    if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
    iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))(pIVar7,0xc0041f14);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
    }
  }
  cVar2 = FUN_1402b9220(*(undefined4 *)(*(longlong *)(param_1 + 0x3a8) + 0x60));
  if (cVar2 == '\0') {
    cVar2 = FUN_1402b9200(*(undefined4 *)(*(longlong *)(param_1 + 0x3a8) + 0x60));
    if (cVar2 == '\0') {
      return;
    }
    pIVar7 = *(IUnknown **)(param_1 + 0x610);
    if (pIVar7 == (IUnknown *)0x0) goto LAB_141cba7b6;
    iVar3 = FUN_1409c6ce0(lVar6);
    iVar4 = FUN_1409c6cc0(lVar6);
    iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8ae4;
  }
  else {
    pIVar7 = *(IUnknown **)(param_1 + 0x610);
    if (pIVar7 == (IUnknown *)0x0) {
LAB_141cba7b6:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar3 = FUN_1409c6ce0(lVar6);
    iVar4 = FUN_1409c6cc0(lVar6);
    iVar3 = (iVar4 * 3000 - iVar3) * 10 + -0x3fff8ada;
  }
  iVar3 = (**(code **)(*(longlong *)pIVar7 + 0x198))(pIVar7,iVar3);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar7,(_GUID *)&DAT_14327fcb0);
  }
  return;
}



//===========================================================
// FUN_142b56090 @ 142b56090   (142 bytes)
//===========================================================

undefined8 * FUN_142b56090(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  
  puVar1 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x868);
  if (puVar1 != (undefined8 *)0x0) {
    FUN_14094c5c0(puVar1);
    *puVar1 = &PTR_LAB_14348cac0;
    puVar1[4] = &PTR_FUN_14348cc00;
    *(undefined8 *)((longlong)puVar1 + 0x844) = 0;
    puVar2 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    if (puVar2 != (undefined8 *)0x0) {
      *puVar2 = 0;
      *(undefined4 *)(puVar2 + 1) = 100;
    }
    puVar1[0x10b] = puVar2;
    return puVar1;
  }
  return (undefined8 *)0x0;
}


