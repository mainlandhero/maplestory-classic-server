
//===========================================================
// FUN_141183ec0 @ 141183ec0   (1136 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014118426f) */

void FUN_141183ec0(undefined8 param_1)

{
  int *piVar1;
  longlong lVar2;
  int *piVar3;
  char cVar4;
  int iVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  int *piVar9;
  int *piVar10;
  int iVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  int *local_res10;
  ulonglong local_res18;
  int *local_res20;
  longlong local_68;
  int *local_60;
  longlong local_58;
  undefined4 local_50 [2];
  longlong local_48;
  longlong lStack_40;
  
  lVar2 = DAT_143aa84a0;
  if (DAT_143aa84a0 == 0) {
    return;
  }
  iVar5 = FUN_1406e8c20();
  FUN_1406e9170(param_1,&local_res10,4);
  uVar13 = 0;
  uVar15 = 0;
  local_60 = (int *)0x0;
  uVar14 = 0xffffffffffffffff;
  uVar12 = uVar13;
  switch(local_res10._0_4_) {
  case 0:
    FUN_1406e9170(param_1,&local_res18,4);
    uVar12 = local_res18 & 0xffffffff;
    if ((int)local_res18 == 0) {
LAB_141183f50:
      DAT_143a872a0 = iVar5;
      FUN_1411574a0(0x1b);
      FUN_1411571c0(0x1b,1);
    }
    goto LAB_141183f6d;
  case 1:
    FUN_1406e9170(param_1,&local_res18,4);
    uVar12 = local_res18 & 0xffffffff;
  case 2:
    FUN_141183cc0();
    FUN_1411574a0(0x1b);
    FUN_142cc4430(lVar2,0);
    break;
  case 3:
    uVar12 = uVar15;
    if (iVar5 == DAT_143a872a0) {
      local_50[0] = 0xffffffff;
      local_48 = 0;
      lStack_40 = 0;
      FUN_140426220(local_50,param_1);
      FUN_141183b20(local_50);
      if (lStack_40 != 0) {
        FUN_14019f2c0(lStack_40 + -0x10);
      }
      if (local_48 != 0) {
        FUN_14019f2c0(local_48 + -0x10);
      }
    }
    break;
  case 4:
    FUN_141184360(param_1);
    uVar12 = uVar15;
    break;
  case 5:
    FUN_1406e9170(param_1,&local_res18,4);
    uVar12 = local_res18 & 0xffffffff;
    FUN_142cc4430(lVar2,0);
    break;
  case 6:
    cVar4 = FUN_1406e8ae0(param_1);
    uVar6 = FUN_1406e8c20(param_1);
    FUN_1406e9050(param_1,&local_68);
    local_res18 = 0;
    FUN_14019a260(&local_res18,&local_68);
    FUN_1411838e0(iVar5,cVar4 != '\0',uVar6,&local_res18);
    uVar12 = uVar15;
    if (local_68 != 0) {
      FUN_14019f2c0(local_68 + -0x10);
    }
    break;
  case 7:
    FUN_1406e9170(param_1,&local_res18,4);
    uVar12 = local_res18 & 0xffffffff;
    if ((int)local_res18 == 0) {
      FUN_141183cc0();
      goto LAB_141183f50;
    }
LAB_141183f6d:
    FUN_142cc4430(lVar2,0);
    break;
  case 8:
    uVar12 = uVar15;
    if (iVar5 == DAT_143a872a0) {
      FUN_1406e9050(param_1,&local_res18);
      uVar12 = local_res18;
      local_50[0] = 0xffffffff;
      local_48 = 0;
      lStack_40 = 0;
      uVar7 = FUN_1408a9f20("SID_MAPLECHAT_DECLINED");
      FUN_14019ba10(&lStack_40,uVar7,uVar12);
      FUN_141183b20(local_50);
      if (lStack_40 != 0) {
        FUN_14019f2c0(lStack_40 + -0x10);
      }
      if (local_48 != 0) {
        FUN_14019f2c0(local_48 + -0x10);
      }
      uVar12 = uVar15;
      if (local_res18 != 0) {
        FUN_14019f2c0(local_res18 - 0x10);
      }
    }
  }
  puVar8 = (undefined8 *)FUN_141189c50(&local_58,uVar12);
  piVar1 = (int *)*puVar8;
  *puVar8 = 0;
  local_60 = piVar1;
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  piVar10 = piVar1;
  if ((piVar1 == (int *)0x0) || ((char)*piVar1 == '\0')) goto LAB_141184306;
  local_res20 = (int *)0x0;
  piVar9 = piVar1 + -4;
  piVar3 = local_res20;
  if (piVar9 != (int *)0x0) {
    if (*piVar9 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar12 = uVar14;
      do {
        uVar12 = uVar12 + 1;
      } while (*(char *)((longlong)piVar1 + uVar12) != '\0');
      iVar11 = (int)uVar12;
      iVar5 = 0;
      if (0 < iVar11) {
        iVar5 = iVar11;
      }
      piVar9 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
      piVar9[1] = iVar5;
      *piVar9 = -1;
      piVar3 = piVar9 + 4;
      piVar9[2] = 0;
      *(char *)piVar3 = '\0';
      local_res10 = piVar3;
      FUN_142ef7ba0(piVar3,piVar1,(longlong)iVar11);
      if (*piVar9 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar9[1])) {
        *piVar9 = 1;
        if (iVar11 != -1) goto LAB_14118422d;
        if (piVar3 != (int *)0x0) {
          do {
            uVar14 = uVar14 + 1;
          } while (*(char *)((longlong)piVar3 + uVar14) != '\0');
          uVar13 = uVar14 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar9[1],uVar12 & 0xffffffff);
        *piVar9 = 1;
LAB_14118422d:
        *(char *)((longlong)iVar11 + (longlong)piVar3) = '\0';
        uVar13 = uVar12;
      }
      iVar5 = (int)uVar13;
      if ((iVar5 < 0) || (piVar9[1] + 1 <= iVar5)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar9[2] = iVar5;
      if (local_res20 != (int *)0x0) {
        FUN_14019f2c0(local_res20 + -4);
      }
    }
    else {
      if (*piVar9 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar9 = *piVar9 + 1;
      UNLOCK();
      piVar10 = local_60;
      piVar3 = piVar1;
      if (local_res20 != (int *)0x0) {
        FUN_14019f2c0(local_res20 + -4);
        piVar10 = local_60;
      }
    }
  }
  local_res20 = piVar3;
  FUN_142a26280(&local_res20,0,0,1,0,0,0,0,0,0);
LAB_141184306:
  if (piVar10 != (int *)0x0) {
    FUN_14019f2c0(piVar10 + -4);
  }
  return;
}



//===========================================================
// FUN_141188700 @ 141188700   (971 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x0001411888b0) */

void FUN_141188700(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  longlong lVar6;
  int *piVar7;
  int iVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  undefined1 auStack_4e8 [32];
  int *local_4c8;
  undefined4 local_4c0 [2];
  int *local_4b8;
  undefined1 local_4b0 [8];
  longlong local_4a8;
  int *local_4a0;
  int **local_498;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_4e8;
  FUN_141ad7ec0(param_1 + 0x2e8,local_4b0,L"chatinput");
  lVar2 = local_4a8;
  puVar11 = (undefined8 *)0x0;
  puVar10 = (undefined8 *)0xffffffffffffffff;
  if ((local_4a8 == 0) || (iVar4 = (**(code **)(*(longlong *)(local_4a8 + 8) + 0x78))(), iVar4 == 0)
     ) goto LAB_141188a54;
  lVar6 = *(longlong *)(lVar2 + 0xc0);
  iVar4 = 0;
  if (lVar6 != 0) {
    iVar4 = *(int *)(lVar6 + -8);
  }
  local_4b8 = (int *)0x0;
  lVar6 = 0xc0;
  if (iVar4 < 1) {
    lVar6 = 200;
  }
  FUN_14019a260(&local_4b8,lVar6 + lVar2);
  piVar7 = local_4b8;
  if ((local_4b8 != (int *)0x0) && ((char)*local_4b8 != '\0')) {
    local_4c8 = (int *)0x0;
    piVar5 = local_4b8 + -4;
    piVar3 = local_4c8;
    if (piVar5 != (int *)0x0) {
      if (*piVar5 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        puVar9 = puVar10;
        do {
          puVar9 = (undefined8 *)((longlong)puVar9 + 1);
        } while (*(char *)((longlong)piVar7 + (longlong)puVar9) != '\0');
        iVar8 = (int)puVar9;
        iVar4 = 0;
        if (0 < iVar8) {
          iVar4 = iVar8;
        }
        piVar5 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
        piVar5[1] = iVar4;
        *piVar5 = -1;
        piVar3 = piVar5 + 4;
        piVar5[2] = 0;
        *(char *)piVar3 = '\0';
        local_4a0 = piVar3;
        FUN_142ef7ba0(piVar3,piVar7,(longlong)iVar8);
        if (*piVar5 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar8 == -1) || (iVar8 <= piVar5[1])) {
          *piVar5 = 1;
          if (iVar8 != -1) goto LAB_14118886c;
          puVar9 = puVar11;
          if (piVar3 != (int *)0x0) {
            do {
              puVar10 = (undefined8 *)((longlong)puVar10 + 1);
              puVar9 = puVar10;
            } while (*(char *)((longlong)piVar3 + (longlong)puVar10) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar5[1],(ulonglong)puVar9 & 0xffffffff);
          *piVar5 = 1;
LAB_14118886c:
          *(char *)((longlong)iVar8 + (longlong)piVar3) = '\0';
        }
        iVar4 = (int)puVar9;
        if ((iVar4 < 0) || (piVar5[1] + 1 <= iVar4)) {
          FUN_142e54290(0x9c,(ulonglong)puVar9 & 0xffffffff);
        }
        piVar5[2] = iVar4;
        if (local_4c8 != (int *)0x0) {
          FUN_14019f2c0(local_4c8 + -4);
        }
      }
      else {
        if (*piVar5 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar5 = *piVar5 + 1;
        UNLOCK();
        if (local_4c8 != (int *)0x0) {
          FUN_14019f2c0(local_4c8 + -4);
        }
        local_4c8 = piVar7;
        piVar7 = local_4b8;
        piVar3 = local_4c8;
      }
    }
    local_4c8 = piVar3;
    local_498 = &local_4c8;
    if ((((DAT_143a872a0 < 0) && (DAT_143aca618 == 0)) ||
        (iVar4 = FUN_1415c0460(&DAT_143aca848,&local_4c8,1), iVar4 == 0)) ||
       ((DAT_143aa84a0 == 0 || (iVar4 = FUN_142cc42d0(DAT_143aa84a0,500,0), iVar4 == 0)))) {
      if (local_4c8 != (int *)0x0) {
        FUN_14019f2c0(local_4c8 + -4);
      }
    }
    else {
      FUN_1406ed520(local_488,0x1fd);
      local_4c0[0] = 3;
      FUN_1406ede20(local_488,local_4c0,4);
      FUN_1406edc80(local_488,&local_4c8);
      FUN_1415d01c0(local_488);
      FUN_1406ed610(local_488);
      if (local_4c8 != (int *)0x0) {
        FUN_14019f2c0(local_4c8 + -4);
      }
      if (local_4a8 == 0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_1416a7070(local_4a8,&DAT_1434b2af1);
    }
    puVar10 = (undefined8 *)(local_4a8 + 8);
    if (local_4a8 == 0) {
      puVar10 = puVar11;
    }
    FUN_142c0bf50(DAT_143abfdf8,puVar10,0);
  }
  if (piVar7 != (int *)0x0) {
    FUN_14019f2c0(piVar7 + -4);
  }
LAB_141188a54:
  lVar2 = local_4a8;
  if (local_4a8 != 0) {
    if (0xffffe < *(longlong *)(local_4a8 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      puVar10 = (undefined8 *)(local_4a8 + 0x18);
      if (local_4a8 == 0) {
        puVar10 = puVar11;
      }
      if (puVar10 != (undefined8 *)0x0) {
        (**(code **)*puVar10)(puVar10,1);
      }
    }
  }
  return;
}


