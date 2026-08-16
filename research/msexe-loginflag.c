
//===========================================================
// FUN_1415e5c20 @ 1415e5c20   (5194 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1415e5c20(longlong param_1,undefined8 param_2)

{
  char cVar1;
  IUnknown *pIVar2;
  byte ******ppppppbVar3;
  undefined4 uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  int iVar8;
  undefined8 *puVar9;
  undefined8 uVar10;
  longlong lVar11;
  longlong *plVar12;
  longlong *plVar13;
  byte *******pppppppbVar14;
  int *piVar15;
  short *psVar16;
  byte *******pppppppbVar17;
  byte ******ppppppbVar18;
  undefined2 *puVar19;
  longlong *plVar20;
  longlong *plVar21;
  void *pvVar22;
  byte *******pppppppbVar23;
  byte ******ppppppbVar24;
  ulonglong uVar25;
  void *pvVar26;
  longlong *plVar27;
  longlong *plVar28;
  code *pcVar29;
  bool bVar30;
  undefined1 auStack_468 [32];
  undefined8 local_448;
  int local_440;
  undefined8 local_438;
  undefined8 local_430;
  IUnknown *local_428;
  longlong *local_420;
  code *local_418 [2];
  undefined **local_408;
  void *local_400;
  int local_3f8;
  ulonglong local_3f0;
  longlong local_3e8;
  longlong lStack_3e0;
  longlong local_3d8;
  undefined8 local_3d0;
  undefined8 local_3c8;
  undefined8 *local_3c0;
  undefined4 local_3b8;
  short local_3a8 [4];
  longlong local_3a0;
  longlong local_390 [2];
  longlong *local_380;
  undefined **local_378;
  undefined1 local_370 [4];
  undefined4 local_36c;
  longlong *local_368;
  longlong *plStack_360;
  longlong *local_358;
  int local_350;
  undefined8 local_348;
  undefined8 local_340;
  longlong *local_338;
  longlong *local_330;
  longlong *local_328;
  longlong *local_320;
  longlong *local_318;
  longlong *local_310;
  longlong *local_308;
  longlong *local_300;
  longlong *local_2f8;
  int local_2f0;
  undefined4 uStack_2ec;
  undefined8 uStack_2e8;
  undefined1 local_2e0 [8];
  longlong *local_2d8;
  short local_2d0 [4];
  longlong local_2c8;
  longlong *local_2b8;
  code *local_2b0;
  longlong *local_2a8;
  undefined8 local_2a0;
  undefined8 uStack_298;
  undefined1 local_290 [80];
  byte *******local_240;
  byte ******ppppppbStack_238;
  byte ******local_230;
  byte ******ppppppbStack_228;
  ulonglong local_220 [2];
  undefined8 local_210;
  ulonglong local_208;
  ulonglong local_200 [2];
  undefined8 local_1f0;
  ulonglong local_1e8;
  longlong local_1e0 [2];
  undefined8 local_1d0;
  ulonglong local_1c8;
  longlong local_1c0 [2];
  undefined8 local_1b0;
  ulonglong local_1a8;
  undefined1 local_1a0;
  undefined7 uStack_19f;
  undefined8 local_190;
  ulonglong local_188;
  undefined1 local_180;
  undefined7 uStack_17f;
  undefined8 local_170;
  ulonglong local_168;
  byte *******local_160;
  byte ******ppppppbStack_158;
  undefined8 local_150;
  ulonglong uStack_148;
  char local_138 [47];
  undefined1 local_109;
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_468;
  if (*(char *)(param_1 + 0x150) != '\0') {
    return;
  }
  if (DAT_143ace3c8 == 0) {
    DAT_143ace3cc = thunk_FUN_1406efcc0(param_2);
    if (DAT_143ace3cc != 0) {
      if (DAT_143ace3cc < 0) {
        *(undefined1 *)(param_1 + 0x150) = 1;
        DAT_143ace3c8 = 2;
        (*DAT_143ad55c0)(PTR_s_Data_wz_143a87bc8);
        DAT_143ace3b8 = DAT_143ace3b0;
        *(undefined4 *)(param_1 + 0x14c) = 0;
        return;
      }
      uVar4 = FUN_1406e8c20(param_2);
      *(undefined4 *)(param_1 + 0x14c) = uVar4;
      FUN_1408bad40(&DAT_143ace3b0,(longlong)DAT_143ace3cc);
      goto LAB_1415e5d40;
    }
    *(undefined1 *)(param_1 + 0x150) = 1;
    DAT_143ace3c8 = 2;
    if (DAT_143ace3b0 == DAT_143ace3b8) {
      DAT_143ace3c8 = 2;
      return;
    }
  }
  else if (DAT_143ace3c8 == 1) {
LAB_1415e5d40:
    if (0x10000 < DAT_143ace3cc) {
      FUN_1406e9170(param_2,DAT_143ace3d0 + DAT_143ace3b0,0x10000);
      DAT_143ace3d0 = DAT_143ace3d0 + 0x10000;
      DAT_143ace3cc = DAT_143ace3cc + -0x10000;
      DAT_143ace3c8 = 1;
      return;
    }
    FUN_1406e9170(param_2,DAT_143ace3d0 + DAT_143ace3b0);
    DAT_143ace3c8 = 2;
    *(undefined1 *)(param_1 + 0x150) = 1;
    local_408 = &PTR_LAB_143272c88;
    pvVar26 = (void *)0x0;
    local_400 = (void *)0x0;
    local_3f0 = 0;
    local_3e8 = _DAT_143303fa0;
    lStack_3e0 = _UNK_143303fa8;
    local_3d8 = 0;
    local_3d0 = 0;
    local_3c8 = 0;
    local_3c0 = (undefined8 *)0x0;
    local_3b8 = 0;
    local_430 = 0;
    local_438 = 0;
    local_440 = 0x40000000;
    local_448 = (longlong *)((ulonglong)local_448._4_4_ << 0x20);
    FUN_1408e8c70(&local_408,PTR_s_Data_wz_143a87bc8,2,0x80);
    FUN_1401bdd80(&local_408,DAT_143ace3b0,DAT_143ace3b8 - DAT_143ace3b0);
    FUN_1401bca50(&local_408);
    local_408 = &PTR_LAB_143272c88;
    LOCK();
    bVar30 = local_400 == (void *)0x0;
    if (bVar30) {
      local_400 = Self;
    }
    UNLOCK();
    if (bVar30) {
LAB_1415e5ea7:
      local_3f8 = 1;
    }
    else {
      if (local_400 != Self) {
        while( true ) {
          LOCK();
          bVar30 = local_400 == (void *)0x0;
          if (bVar30) {
            local_400 = Self;
          }
          UNLOCK();
          if (bVar30) goto LAB_1415e5ea7;
          if (local_400 == Self) break;
          (*DAT_143262828)(0);
        }
      }
      local_3f8 = local_3f8 + 1;
    }
    LOCK();
    bVar30 = local_400 == (void *)0x0;
    if (bVar30) {
      local_400 = Self;
    }
    UNLOCK();
    if (bVar30) {
LAB_1415e5ef5:
      local_3f8 = 1;
    }
    else {
      if (local_400 != Self) {
        while( true ) {
          LOCK();
          bVar30 = local_400 == (void *)0x0;
          if (bVar30) {
            local_400 = Self;
          }
          UNLOCK();
          if (bVar30) goto LAB_1415e5ef5;
          if (local_400 == Self) break;
          (*DAT_143262828)(0);
        }
      }
      local_3f8 = local_3f8 + 1;
    }
    iVar8 = 1;
    pvVar22 = pvVar26;
    if (local_3d8 != 0) {
      iVar8 = (*DAT_143ad5558)();
      iVar5 = iVar8;
      if (iVar8 != 0) {
        local_3d8 = 0;
        local_340 = local_3f0;
        iVar5 = (*DAT_143ad55b0)(local_3e8,local_3f0 & 0xffffffff,(longlong)&local_340 + 4,0);
        local_340 = CONCAT44(local_340._4_4_,iVar5);
        pvVar22 = (void *)0x0;
        if ((iVar5 != -1) || (iVar6 = (*DAT_143ad5628)(), iVar5 = 0, iVar6 == 0))
        goto LAB_1415e5f77;
      }
      iVar8 = iVar5;
      uVar7 = (*DAT_143ad5628)();
      pvVar22 = (void *)(ulonglong)uVar7;
    }
LAB_1415e5f77:
    if (lStack_3e0 != 0) {
      iVar5 = (*DAT_143ad5400)();
      if ((iVar5 == 0) || (iVar8 == 0)) {
        iVar8 = 0;
        if ((int)pvVar22 == 0) {
          uVar7 = (*DAT_143ad5628)();
          pvVar22 = (void *)(ulonglong)uVar7;
          iVar8 = 0;
        }
      }
      else {
        iVar8 = 1;
      }
      lStack_3e0 = 0;
    }
    (*DAT_143ad5630)(pvVar22);
    local_3f8 = local_3f8 + -1;
    if (local_3f8 == 0) {
      local_400 = pvVar26;
    }
    iVar5 = (*DAT_143ad5628)();
    if (local_3e8 != -1) {
      if ((((local_3e8 != 0) && (iVar6 = (*DAT_143ad5400)(), iVar6 == 0)) || (iVar8 == 0)) &&
         (iVar5 == 0)) {
        iVar5 = (*DAT_143ad5628)();
      }
      local_3e8 = -1;
    }
    local_3b8 = 0;
    (*DAT_143ad5630)(iVar5);
    local_3f8 = local_3f8 + -1;
    if (local_3f8 == 0) {
      local_400 = pvVar26;
    }
    if (local_3c0 != (undefined8 *)0x0) {
      (**(code **)*local_3c0)(local_3c0,1);
    }
  }
  else if (DAT_143ace3c8 == 2) {
    return;
  }
  plVar27 = (longlong *)0x0;
  iVar8 = (int)DAT_143ace3b8 - (int)DAT_143ace3b0;
  local_378 = &PTR_FUN_143302f78;
  local_36c = 0;
  local_368 = (longlong *)0x0;
  plStack_360 = (longlong *)0x0;
  local_348 = 0;
  local_358 = (longlong *)0x0;
  local_350 = 0;
  uStack_2ec = 0;
  local_2f0 = iVar8;
  uStack_2e8 = FUN_14019b780(&DAT_143ad68a0,iVar8);
  puVar9 = (undefined8 *)FUN_14092d9d0(local_370);
  *puVar9 = CONCAT44(uStack_2ec,local_2f0);
  puVar9[1] = uStack_2e8;
  local_358 = local_368;
  local_350 = 0;
  local_348._0_4_ = (int)local_348 + iVar8;
  FUN_14092f860(&local_378,DAT_143ace3b0,DAT_143ace3b8 - DAT_143ace3b0);
  iVar8 = local_350;
  local_350 = 0;
  iVar5 = 0;
  if (local_348._4_4_ != 0) {
    iVar5 = local_348._4_4_;
  }
  local_348 = CONCAT44(iVar5,(int)local_348);
  plVar21 = local_368;
  if ((int)*local_368 == 0) {
    plVar13 = local_358;
    if (iVar8 != 0) {
      plVar20 = local_358;
      plVar21 = plStack_360;
      if (local_358 == (longlong *)0x0) goto LAB_1415e620b;
      do {
        if ((*(int *)((longlong)plVar20 + 4) == 0) && (plVar21 = plVar20, (int)*plVar20 != 0))
        goto LAB_1415e620b;
        uVar25 = plVar20[-3];
        if ((uVar25 != 0) && (uVar25 < 0x10001)) {
          FUN_142e52ed0(0x330);
          uVar25 = plVar20[-3];
        }
        plVar20 = plVar27;
        if (uVar25 != 0) {
          plVar20 = (longlong *)(uVar25 + 0x28);
        }
        plVar13 = local_358;
      } while (plVar20 != (longlong *)0x0);
    }
    while ((plVar21 = plStack_360, plVar13 != (longlong *)0x0 &&
           ((*(int *)((longlong)plVar13 + 4) != 0 || (plVar21 = plVar13, (int)*plVar13 == 0))))) {
      uVar25 = plVar13[-4];
      if ((uVar25 != 0) && (uVar25 < 0x10001)) {
        FUN_142e52ed0(0x33e);
        uVar25 = plVar13[-4];
      }
      plVar13 = plVar27;
      if (uVar25 != 0) {
        plVar13 = (longlong *)(uVar25 + 0x28);
      }
    }
  }
LAB_1415e620b:
  local_358 = plVar21;
  DAT_143ace3b8 = DAT_143ace3b0;
  local_380 = (longlong *)0x0;
  FUN_1401a5890(&local_338,&DAT_143278568);
  local_440 = 0;
  local_448 = (longlong *)((ulonglong)local_448 & 0xffffffff00000000);
  uVar10 = FUN_140a03890(local_290,&local_338,0,&local_378);
  FUN_1415e7980(&local_380,uVar10);
  FUN_140a03960(local_290);
  if (local_338 != (longlong *)0x0) {
    LOCK();
    plVar21 = local_338 + 2;
    lVar11 = *plVar21;
    *(int *)plVar21 = (int)*plVar21 + -1;
    UNLOCK();
    if (((int)lVar11 == 1) && (local_338 != (longlong *)0x0)) {
      if (*local_338 != 0) {
        (*DAT_143ad5990)(*local_338 + -4);
        *local_338 = 0;
      }
      if (local_338[1] != 0) {
        FUN_14019b4e0();
        local_338[1] = 0;
      }
      thunk_FUN_140205820(local_338,0x18);
    }
  }
  plVar21 = local_380;
  local_310 = (longlong *)0x0;
  local_2f8 = local_380;
  if (local_380 == (longlong *)0x0) {
    iVar8 = -0x7fffbffe;
  }
  else {
    (**(code **)(*local_380 + 8))(local_380);
    local_330 = (longlong *)0x0;
    iVar8 = (**(code **)*plVar21)(plVar21,&DAT_143272478,&local_330);
    local_310 = plVar27;
    if (-1 < iVar8) {
      local_310 = local_330;
    }
  }
  plVar13 = local_310;
  if (plVar21 != (longlong *)0x0) {
    (**(code **)(*plVar21 + 0x10))(plVar21);
  }
  if (((iVar8 + 0x80000000U & 0x80000000) == 0) && (iVar8 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar8);
  }
  local_2a0 = 0;
  uStack_298 = 0;
  local_328 = plVar13;
  if (plVar13 != (longlong *)0x0) {
    (**(code **)(*plVar13 + 8))(plVar13);
  }
  FUN_14035fdc0(local_390,&local_328,&DAT_143278568);
  do {
    if (local_390[0] == 0) {
      FUN_1403607a0(local_390);
      pIVar2 = DAT_143add058;
      if (DAT_143add058 != (IUnknown *)0x0) {
        iVar8 = (**(code **)(*(longlong *)DAT_143add058 + 0x80))(DAT_143add058,plVar13);
        if (iVar8 < 0) {
          _com_issue_errorex(iVar8,pIVar2,(_GUID *)&DAT_1432743e8);
        }
        if (plVar13 != (longlong *)0x0) {
          (**(code **)(*plVar13 + 0x10))(plVar13);
        }
        if (local_380 != (longlong *)0x0) {
          (**(code **)(*local_380 + 0x10))();
        }
        FUN_14092d170(&local_378);
        return;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    lVar11 = FUN_14035ff40(local_390);
    plVar21 = plVar13;
    if (*(short *)(lVar11 + 8) == 0xd) {
      lVar11 = FUN_14035ff40(local_390);
      local_320 = (longlong *)0x0;
      iVar8 = FUN_14092e730(&local_320,lVar11 + 8);
      plVar20 = local_320;
      if (((iVar8 + 0x80000000U & 0x80000000) == 0) && (iVar8 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar8);
      }
      plVar12 = (longlong *)FUN_14035ff40(local_390);
      if (plVar20 == (longlong *)0x0) {
        local_428 = (IUnknown *)0x0;
        iVar8 = FUN_140360cd0(&local_428,plVar12 + 1);
        pIVar2 = local_428;
        if (((iVar8 + 0x80000000U & 0x80000000) == 0) && (iVar8 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar8);
        }
        if (local_428 != (IUnknown *)0x0) {
          local_418[0] = (code *)0x0;
          iVar8 = (**(code **)(*(longlong *)local_428 + 0xf0))(local_428,local_418);
          if (iVar8 < 0) {
            _com_issue_errorex(iVar8,pIVar2,(_GUID *)&DAT_14327ac98);
          }
          pcVar29 = local_418[0];
          local_2b0 = local_418[0];
          if (local_418[0] == (code *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          uVar10 = FUN_1401a5890(local_2e0,L"_inlink");
          psVar16 = (short *)FUN_1401e4330(pcVar29,local_2d0,uVar10);
          if (*psVar16 == 8) {
            puVar19 = *(undefined2 **)(psVar16 + 4);
          }
          else {
            puVar19 = &DAT_143278568;
          }
          FUN_1401a5890(&local_420,puVar19);
          if (local_2d0[0] == 8) {
            local_2d0[0] = 0;
            if (local_2c8 != 0) {
              (*DAT_143ad5990)(local_2c8 + -4);
            }
          }
          else {
            (*DAT_143262a18)(local_2d0);
          }
          (**(code **)(*(longlong *)pcVar29 + 0x10))(pcVar29);
          plVar21 = local_310;
          if (((local_420 == (longlong *)0x0) || (*local_420 == 0)) ||
             (*(uint *)(*local_420 + -4) < 2)) {
            thunk_FUN_1401be120(&local_420);
            (**(code **)(*(longlong *)pIVar2 + 0x10))(pIVar2);
            plVar21 = plVar13;
          }
          else {
            if (plVar13 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            local_318 = local_420;
            LOCK();
            *(int *)(local_420 + 2) = (int)local_420[2] + 1;
            UNLOCK();
            FUN_1401e4330(local_310,local_3a8,&local_318);
            if (local_3a8[0] == 0xd) {
              local_308 = (longlong *)0x0;
              iVar8 = FUN_140360cd0(&local_308,local_3a8);
              plVar13 = local_308;
              pIVar2 = local_428;
              if (((iVar8 + 0x80000000U & 0x80000000) == 0) && (iVar8 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar8);
              }
              if (local_308 == (longlong *)0x0) {
                if (local_3a8[0] == 8) {
                  local_3a8[0] = 0;
                  if (local_3a0 != 0) {
                    (*DAT_143ad5990)(local_3a0 + -4);
                  }
                }
                else {
                  (*DAT_143262a18)(local_3a8);
                }
                thunk_FUN_1401be120(&local_420);
                (**(code **)(*(longlong *)local_428 + 0x10))();
              }
              else {
                iVar8 = (**(code **)(*(longlong *)local_428 + 0x150))(local_428,local_308);
                if (iVar8 < 0) {
                  _com_issue_errorex(iVar8,pIVar2,(_GUID *)&DAT_14327ac98);
                }
                local_300 = (longlong *)0x0;
                iVar8 = (**(code **)(*(longlong *)pIVar2 + 0xf0))(pIVar2,&local_300);
                if (iVar8 < 0) {
                  _com_issue_errorex(iVar8,pIVar2,(_GUID *)&DAT_14327ac98);
                }
                plVar20 = local_300;
                local_2b8 = local_300;
                if (local_300 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(0x80004003);
                }
                (**(code **)(*local_300 + 0x68))(local_300,L"_inlink");
                (**(code **)(*plVar20 + 0x10))(plVar20);
                (**(code **)(*plVar13 + 0x10))(plVar13);
                if (local_3a8[0] == 8) {
                  local_3a8[0] = 0;
                  if (local_3a0 != 0) {
                    (*DAT_143ad5990)(local_3a0 + -4);
                  }
                }
                else {
                  (*DAT_143262a18)(local_3a8);
                }
                thunk_FUN_1401be120(&local_420);
                (**(code **)(*(longlong *)pIVar2 + 0x10))(pIVar2);
              }
            }
            else {
              if (local_3a8[0] == 8) {
                local_3a8[0] = 0;
                if (local_3a0 != 0) {
                  (*DAT_143ad5990)(local_3a0 + -4);
                }
              }
              else {
                (*DAT_143262a18)(local_3a8);
              }
              thunk_FUN_1401be120(&local_420);
              (**(code **)(*(longlong *)local_428 + 0x10))();
            }
          }
        }
      }
      else {
        plVar12 = (longlong *)*plVar12;
        if (plVar12 == (longlong *)0x0) {
          plVar13 = (longlong *)0x0;
        }
        else {
          plVar13 = (longlong *)plVar12[1];
          if ((longlong *)plVar12[1] == (longlong *)0x0) {
            lVar11 = *plVar12;
            if (lVar11 == 0) {
              plVar12[1] = 0;
              plVar13 = plVar27;
            }
            else {
              local_430 = 0;
              local_438 = 0;
              local_440 = 0;
              local_448 = (longlong *)0x0;
              iVar8 = (*DAT_1432627f0)(0xfde9,0,lVar11,0xffffffff);
              plVar13 = (longlong *)FUN_14019b780(&DAT_143ad68a0,iVar8);
              local_430 = 0;
              local_438 = 0;
              local_448 = plVar13;
              local_440 = iVar8;
              (*DAT_1432627f0)(0xfde9,0,lVar11,0xffffffff);
              plVar12[1] = (longlong)plVar13;
            }
          }
        }
        local_230 = (byte ******)0x0;
        ppppppbStack_228 = (byte ******)0xf;
        local_240 = (byte *******)0x0;
        lVar11 = -1;
        do {
          lVar11 = lVar11 + 1;
        } while (*(char *)((longlong)plVar13 + lVar11) != '\0');
        FUN_1401d69c0(&local_240,plVar13);
        local_1a8 = 0xf;
        local_1b0 = 0;
        local_1c0[0] = 0;
        local_208 = 0xf;
        local_210 = 4;
        local_220[0] = 0x676d692e;
        pppppppbVar14 = (byte *******)FUN_1402b9b50(&local_1a0,&local_240,local_220,local_1c0);
        if (&local_240 != (byte ********)pppppppbVar14) {
          if ((byte ******)0xf < ppppppbStack_228) {
            ppppppbVar18 = (byte ******)((longlong)ppppppbStack_228 + 1);
            pppppppbVar17 = local_240;
            if ((byte ******)0xfff < ppppppbVar18) {
              ppppppbVar18 = ppppppbStack_228 + 5;
              pppppppbVar17 = (byte *******)local_240[-1];
              if ((byte *)0x1f < (byte *)((longlong)local_240 + (-8 - (longlong)pppppppbVar17)))
              goto LAB_1415e7019;
            }
            thunk_FUN_140205820(pppppppbVar17,ppppppbVar18);
          }
          local_240 = (byte *******)*pppppppbVar14;
          ppppppbStack_238 = pppppppbVar14[1];
          local_230 = pppppppbVar14[2];
          ppppppbStack_228 = pppppppbVar14[3];
          pppppppbVar14[2] = (byte ******)0x0;
          pppppppbVar14[3] = (byte ******)0xf;
          *(undefined1 *)pppppppbVar14 = 0;
        }
        if (0xf < local_188) {
          if (0xfff < local_188 + 1) {
            ppppppbVar18 = (byte ******)(local_188 + 0x28);
            pppppppbVar17 = *(byte ********)(CONCAT71(uStack_19f,local_1a0) + -8);
            if (0x1f < (CONCAT71(uStack_19f,local_1a0) - (longlong)pppppppbVar17) - 8U) {
LAB_1415e7019:
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(pppppppbVar17,ppppppbVar18);
            }
          }
          thunk_FUN_140205820();
        }
        local_190 = 0;
        local_188 = 0xf;
        local_1a0 = 0;
        if (0xf < local_208) {
          if (0xfff < local_208 + 1) {
            if (0x1f < (local_220[0] - *(longlong *)(local_220[0] - 8)) - 8) {
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(*(longlong *)(local_220[0] - 8),local_208 + 0x28);
            }
          }
          thunk_FUN_140205820();
        }
        local_210 = 0;
        local_208 = 0xf;
        local_220[0] = local_220[0] & 0xffffffffffffff00;
        if (0xf < local_1a8) {
          if (0xfff < local_1a8 + 1) {
            if (0x1f < (local_1c0[0] - *(longlong *)(local_1c0[0] + -8)) - 8U) {
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(*(longlong *)(local_1c0[0] + -8),local_1a8 + 0x28);
            }
          }
          thunk_FUN_140205820();
        }
        local_1c8 = 0xf;
        local_1d0 = 1;
        local_1e0[0] = 0x2f;
        local_1e8 = 0xf;
        local_1f0 = 1;
        local_200[0] = 0x5c;
        pppppppbVar14 = (byte *******)FUN_1402b9b50(&local_180,&local_240,local_200,local_1e0);
        if (&local_240 != (byte ********)pppppppbVar14) {
          if ((byte ******)0xf < ppppppbStack_228) {
            ppppppbVar18 = (byte ******)((longlong)ppppppbStack_228 + 1);
            pppppppbVar17 = local_240;
            if ((byte ******)0xfff < ppppppbVar18) {
              ppppppbVar18 = ppppppbStack_228 + 5;
              pppppppbVar17 = (byte *******)local_240[-1];
              if ((byte *)0x1f < (byte *)((longlong)local_240 + (-8 - (longlong)pppppppbVar17)))
              goto LAB_1415e702b;
            }
            thunk_FUN_140205820(pppppppbVar17,ppppppbVar18);
          }
          local_240 = (byte *******)*pppppppbVar14;
          ppppppbStack_238 = pppppppbVar14[1];
          local_230 = pppppppbVar14[2];
          ppppppbStack_228 = pppppppbVar14[3];
          pppppppbVar14[2] = (byte ******)0x0;
          pppppppbVar14[3] = (byte ******)0xf;
          *(undefined1 *)pppppppbVar14 = 0;
        }
        if (0xf < local_168) {
          if (0xfff < local_168 + 1) {
            ppppppbVar18 = (byte ******)(local_168 + 0x28);
            pppppppbVar17 = *(byte ********)(CONCAT71(uStack_17f,local_180) + -8);
            if (0x1f < (CONCAT71(uStack_17f,local_180) - (longlong)pppppppbVar17) - 8U) {
LAB_1415e702b:
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(pppppppbVar17,ppppppbVar18);
            }
          }
          thunk_FUN_140205820();
        }
        local_170 = 0;
        local_168 = 0xf;
        local_180 = 0;
        if (0xf < local_1e8) {
          if (0xfff < local_1e8 + 1) {
            if (0x1f < (local_200[0] - *(longlong *)(local_200[0] - 8)) - 8) {
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(*(longlong *)(local_200[0] - 8),local_1e8 + 0x28);
            }
          }
          thunk_FUN_140205820();
        }
        local_1f0 = 0;
        local_1e8 = 0xf;
        local_200[0] = local_200[0] & 0xffffffffffffff00;
        if (0xf < local_1c8) {
          if (0xfff < local_1c8 + 1) {
            if (0x1f < (local_1e0[0] - *(longlong *)(local_1e0[0] + -8)) - 8U) {
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(*(longlong *)(local_1e0[0] + -8),local_1c8 + 0x28);
            }
          }
          thunk_FUN_140205820();
        }
        ppppppbVar3 = ppppppbStack_228;
        ppppppbVar18 = local_230;
        pppppppbVar14 = local_240;
        pppppppbVar17 = (byte *******)&local_240;
        if ((byte ******)0xf < ppppppbStack_228) {
          pppppppbVar17 = local_240;
        }
        if (local_230 == (byte ******)0x0) {
LAB_1415e69cc:
          ppppppbVar24 = (byte ******)0xffffffffffffffff;
        }
        else {
          FUN_142ef8250(local_138,0,0x100);
          local_109 = 1;
          lVar11 = -1;
          if ((longlong)ppppppbVar18 + -1 != -1) {
            lVar11 = (longlong)ppppppbVar18 + -1;
          }
          pppppppbVar23 = (byte *******)((longlong)pppppppbVar17 + lVar11);
          cVar1 = local_138[*(byte *)pppppppbVar23];
          while (cVar1 == '\0') {
            if (pppppppbVar23 == pppppppbVar17) goto LAB_1415e69cc;
            pppppppbVar23 = (byte *******)((longlong)pppppppbVar23 + -1);
            cVar1 = local_138[*(byte *)pppppppbVar23];
          }
          ppppppbVar24 = (byte ******)((longlong)pppppppbVar23 - (longlong)pppppppbVar17);
        }
        local_160 = (byte *******)0x0;
        local_150 = 0;
        uStack_148 = 0xf;
        if (ppppppbVar18 < ppppppbVar24) {
          ppppppbVar24 = ppppppbVar18;
        }
        pppppppbVar17 = (byte *******)&local_240;
        if ((byte ******)0xf < ppppppbVar3) {
          pppppppbVar17 = pppppppbVar14;
        }
        FUN_1401d69c0(&local_160,pppppppbVar17,ppppppbVar24);
        if ((byte ******)0xf < ppppppbStack_228) {
          if (0xfff < (longlong)ppppppbStack_228 + 1U) {
            if ((byte *)0x1f < (byte *)((longlong)local_240 + (-8 - (longlong)local_240[-1]))) {
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(local_240[-1],ppppppbStack_228 + 5);
            }
          }
          thunk_FUN_140205820();
        }
        local_240 = local_160;
        ppppppbStack_238 = ppppppbStack_158;
        local_230 = (byte ******)local_150;
        ppppppbStack_228 = (byte ******)uStack_148;
        pcVar29 = *(code **)(*plVar20 + 0x38);
        pppppppbVar14 = (byte *******)&local_240;
        if (0xf < uStack_148) {
          pppppppbVar14 = local_160;
        }
        local_418[0] = pcVar29;
        plVar12 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
        plVar13 = plVar27;
        local_2a8 = plVar12;
        if (plVar12 != (longlong *)0x0) {
          plVar12[1] = 0;
          *(int *)(plVar12 + 2) = 1;
          plVar13 = plVar12;
          if (pppppppbVar14 == (byte *******)0x0) {
            *plVar12 = 0;
            pcVar29 = local_418[0];
          }
          else {
            local_440 = 0;
            local_448 = (longlong *)0x0;
            iVar8 = (*DAT_1432627f8)(0xfde9,0,pppppppbVar14,0xffffffff);
            uVar25 = (ulonglong)(longlong)(iVar8 * 2) >> 1;
            iVar8 = (int)uVar25;
            local_428 = (IUnknown *)CONCAT44(local_428._4_4_,iVar8 + -1);
            piVar15 = (int *)(*DAT_143ad5980)((uVar25 & 0xffffffff) * 2 + 4);
            plVar28 = plVar27;
            if (piVar15 != (int *)0x0) {
              *piVar15 = (int)local_428 * 2;
              *(undefined2 *)((longlong)(piVar15 + 1) + ((ulonglong)local_428 & 0xffffffff) * 2) = 0
              ;
              plVar28 = (longlong *)(piVar15 + 1);
            }
            local_448 = plVar28;
            local_440 = iVar8;
            (*DAT_1432627f8)(0xfde9,0,pppppppbVar14,0xffffffff);
            *plVar12 = (longlong)plVar28;
            pcVar29 = local_418[0];
          }
        }
        local_2d8 = plVar13;
        if (plVar13 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x8007000e);
        }
        (*pcVar29)(plVar20,*plVar13);
        thunk_FUN_1401be120(&local_2d8);
        if (0xf < ppppppbStack_228) {
          if (0xfff < (longlong)ppppppbStack_228 + 1U) {
            if (0x1f < (ulonglong)((longlong)local_240 + (-8 - (longlong)local_240[-1]))) {
                    /* WARNING: Subroutine does not return */
              FUN_142f04804(local_240[-1],(longlong)ppppppbStack_228 + 0x28);
            }
          }
          thunk_FUN_140205820();
        }
        local_230 = (byte ******)0x0;
        ppppppbStack_228 = (byte ******)0xf;
        local_240 = (byte *******)((ulonglong)local_240 & 0xffffffffffffff00);
        (**(code **)(*plVar20 + 0x10))(plVar20);
      }
    }
    FUN_14035ff20(local_390);
    plVar13 = plVar21;
  } while( true );
}


