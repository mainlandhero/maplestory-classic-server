
//===========================================================
// FUN_1429e4fa0 @ 1429e4fa0   (1128 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1429e4fa0(int *param_1,int param_2,undefined4 param_3)

{
  code *pcVar1;
  IUnknown *pIVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  int *piVar6;
  char *pcVar7;
  int *piVar8;
  ulonglong uVar9;
  int iVar10;
  ulonglong uVar11;
  undefined1 auStack_2b8 [32];
  int *local_298;
  undefined8 local_290;
  int *local_288;
  undefined4 local_280 [2];
  longlong local_278 [2];
  int local_268 [68];
  undefined1 local_158 [272];
  ulonglong local_48;
  
  pIVar2 = DAT_143add050;
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_2b8;
  if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  local_288 = (int *)((ulonglong)local_288 & 0xffffffff00000000);
  iVar3 = (**(code **)(*(longlong *)DAT_143add050 + 0xf8))(DAT_143add050,&local_288);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcd0);
  }
  pcVar1 = DAT_143ad5758;
  if ((int)local_288 != 0) {
    uVar4 = FUN_142c49f00(DAT_143ac1898);
    (*pcVar1)(uVar4,6);
  }
  local_280[0] = 0x104;
  FUN_142ef8250(local_268,0,0x104);
  local_290 = local_280;
  local_298 = local_268;
  iVar3 = (*DAT_143262a80)(0x400,2,".html");
  if (iVar3 < 0) {
    (*DAT_143ad53f8)(0,local_158,0x104);
    FUN_142c4a9b0(local_158);
    FUN_142c4aa30(local_158);
    FUN_142c4a9f0(local_158);
    local_288 = (int *)0x0;
    puVar5 = (undefined8 *)FUN_1408a9e40(local_278,0x67d);
    FUN_14019ba10(&local_288,*puVar5,local_158,param_1);
    if (local_278[0] != 0) {
      FUN_14019f2c0(local_278[0] + -0x10);
    }
    piVar8 = local_288;
    (*DAT_143ad5690)(local_288,5);
    if (piVar8 == (int *)0x0) {
      return;
    }
    FUN_14019f2c0(piVar8 + -4);
    return;
  }
  piVar8 = (int *)0x0;
  local_288 = (int *)0x0;
  if (param_1 != (int *)0x0) {
    uVar9 = 0xffffffffffffffff;
    uVar11 = 0xffffffffffffffff;
    do {
      uVar11 = uVar11 + 1;
    } while (*(char *)((longlong)param_1 + uVar11) != '\0');
    iVar10 = (int)uVar11;
    iVar3 = 0;
    if (0 < iVar10) {
      iVar3 = iVar10;
    }
    piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
    piVar6[1] = iVar3;
    *piVar6 = -1;
    piVar8 = piVar6 + 4;
    piVar6[2] = 0;
    *(undefined1 *)piVar8 = 0;
    local_288 = piVar8;
    FUN_142ef7ba0(piVar8,param_1,(longlong)iVar10);
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar10 == -1) || (iVar10 <= piVar6[1])) {
      *piVar6 = 1;
      if (iVar10 != -1) goto LAB_1429e51e1;
      if (piVar8 == (int *)0x0) {
        uVar11 = 0;
      }
      else {
        do {
          uVar9 = uVar9 + 1;
        } while (*(char *)((longlong)piVar8 + uVar9) != '\0');
        uVar11 = uVar9 & 0xffffffff;
      }
    }
    else {
      FUN_142e54290(0x90,piVar6[1],uVar11 & 0xffffffff);
      *piVar6 = 1;
LAB_1429e51e1:
      *(undefined1 *)((longlong)iVar10 + (longlong)piVar8) = 0;
    }
    iVar3 = (int)uVar11;
    if ((iVar3 < 0) || (piVar6[1] + 1 <= iVar3)) {
      FUN_142e54290(0x9c,uVar11 & 0xffffffff);
    }
    piVar6[2] = iVar3;
  }
  pcVar7 = (char *)(*DAT_143262a88)(local_268);
  iVar3 = strcmp("chrome.exe",pcVar7);
  if (iVar3 == 0) {
    if (param_2 == 0) goto LAB_1429e52e4;
    local_298 = param_1;
    FUN_14019ba10(&local_288,
                  "--app=\"data:text/html,<html><body><script>window.resizeTo(%d,%d);window.location=\'%s\';</script></body></html>\""
                  ,param_2,param_3);
    piVar8 = local_288;
  }
  else {
    pcVar7 = (char *)(*DAT_143262a88)(local_268);
    iVar3 = strcmp("msedge.exe",pcVar7);
    if (iVar3 == 0) {
LAB_1429e52e4:
      pcVar7 = "--new-window %s";
    }
    else {
      pcVar7 = (char *)(*DAT_143262a88)(local_268);
      iVar3 = strcmp("whale.exe",pcVar7);
      if (iVar3 == 0) goto LAB_1429e52e4;
      pcVar7 = (char *)(*DAT_143262a88)(local_268);
      iVar3 = strcmp("firefox.exe",pcVar7);
      if (iVar3 != 0) goto LAB_1429e52f8;
      pcVar7 = "-new-window %s";
    }
    FUN_14019ba10(&local_288,pcVar7,param_1);
    piVar8 = local_288;
  }
LAB_1429e52f8:
  uVar4 = (*DAT_143262a88)(local_268);
  iVar3 = FUN_142f100b0("chrome.exe",uVar4);
  if (iVar3 != 0) {
    uVar4 = (*DAT_143262a88)(local_268);
    iVar3 = FUN_142f100b0("msedge.exe",uVar4);
    if (iVar3 != 0) {
      uVar4 = (*DAT_143262a88)(local_268);
      iVar3 = FUN_142f100b0("whale.exe",uVar4);
      if (iVar3 != 0) {
        uVar4 = (*DAT_143262a88)(local_268);
        iVar3 = FUN_142f100b0("firefox.exe",uVar4);
        if (iVar3 != 0) {
          uVar4 = (*DAT_143262a88)(local_268);
          iVar3 = FUN_142f100b0("iexplore.exe",uVar4);
          if (iVar3 != 0) {
            local_298 = (int *)0x0;
            pcVar7 = "iexplore";
            uVar4 = 0;
            goto LAB_1429e53dc;
          }
        }
      }
    }
  }
  uVar4 = FUN_142c49f00(DAT_143ac1898);
  local_298 = (int *)&DAT_1434b2af1;
  pcVar7 = (char *)local_268;
  param_1 = piVar8;
LAB_1429e53dc:
  local_290 = (undefined4 *)CONCAT44(local_290._4_4_,5);
  (*DAT_143262a70)(uVar4,&DAT_143287db8,pcVar7,param_1);
  if (piVar8 != (int *)0x0) {
    FUN_14019f2c0(piVar8 + -4);
  }
  return;
}


