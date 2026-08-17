
//===========================================================
// FUN_141b2dd00 @ 141b2dd00   (4475 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141b2e412) */
/* WARNING: Removing unreachable block (ram,0x000141b2e1f4) */

void FUN_141b2dd00(longlong param_1,undefined8 param_2)

{
  undefined2 *puVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ushort uVar6;
  undefined2 uVar7;
  ushort uVar8;
  ushort uVar9;
  byte bVar10;
  char cVar11;
  undefined1 uVar12;
  int iVar13;
  undefined4 uVar14;
  uint uVar15;
  undefined4 uVar16;
  int *piVar17;
  longlong lVar18;
  undefined8 *puVar19;
  longlong *plVar20;
  undefined8 uVar21;
  char *pcVar22;
  undefined8 uVar23;
  ulonglong uVar24;
  longlong **pplVar25;
  longlong *plVar26;
  undefined1 *puVar27;
  int *piVar28;
  int iVar29;
  longlong lVar30;
  longlong *plVar31;
  ulonglong uVar32;
  undefined8 auStack_5b0 [5];
  uint local_588 [2];
  ulonglong local_580;
  uint local_578 [2];
  undefined8 local_570;
  longlong local_568;
  undefined4 local_560 [2];
  longlong local_558;
  undefined4 local_550 [2];
  longlong local_548 [2];
  longlong *local_538;
  undefined1 *local_530;
  longlong *local_528;
  undefined1 local_520;
  undefined1 local_51f;
  undefined1 local_51e;
  undefined1 local_51d;
  longlong *local_518 [2];
  undefined8 local_508;
  undefined4 uStack_500;
  undefined4 uStack_4fc;
  longlong local_4f8;
  longlong **local_4f0;
  longlong local_4e8;
  int *local_4e0;
  undefined4 local_4d8 [2];
  longlong local_4d0;
  longlong local_4c8;
  longlong local_4c0;
  longlong local_4b8;
  longlong local_4b0;
  undefined4 local_4a8;
  undefined4 uStack_4a4;
  undefined4 uStack_4a0;
  undefined4 uStack_49c;
  undefined1 local_498 [8];
  undefined1 local_490 [8];
  undefined8 local_488;
  undefined8 uStack_480;
  undefined8 local_478;
  undefined8 uStack_470;
  undefined4 local_468;
  undefined4 uStack_464;
  undefined4 uStack_460;
  undefined4 uStack_45c;
  undefined1 local_458 [16];
  undefined1 local_448 [1024];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)&local_538;
  piVar28 = (int *)0x0;
  *(undefined4 *)(param_1 + 0xd4) = 0;
  auStack_5b0[0] = 0x141b2dd4a;
  local_4e8 = param_1;
  bVar10 = FUN_1406e8ae0(param_2);
  uVar15 = (uint)bVar10;
  local_518[0] = (longlong *)CONCAT44(local_518[0]._4_4_,(uint)bVar10);
  auStack_5b0[0] = 0x141b2dd5e;
  FUN_1406e9050(param_2,&local_4d0);
  uVar32 = 0xffffffffffffffff;
  if (local_4d0 == 0) {
    iVar13 = 2;
  }
  else {
    local_580._0_4_ = 0;
    local_588[0] = 0;
    local_588[1] = 0;
    auStack_5b0[0] = 0x141b2dd8a;
    iVar13 = (*DAT_1432627f8)(0xfde9,0,local_4d0,0xffffffff);
    iVar13 = iVar13 * 2;
  }
  lVar30 = local_4d0;
  uVar24 = (longlong)iVar13 + 0xf;
  if (uVar24 <= (ulonglong)(longlong)iVar13) {
    uVar24 = 0xffffffffffffff0;
  }
  auStack_5b0[0] = 0x141b2ddb1;
  lVar3 = -(uVar24 & 0xfffffffffffffff0);
  puVar1 = (undefined2 *)((longlong)&local_538 + lVar3);
  uVar24 = uVar32;
  if (local_4d0 == 0) {
    if (puVar1 == (undefined2 *)0x0) goto LAB_141b2dde8;
    *puVar1 = 0;
LAB_141b2de00:
    do {
      uVar24 = uVar24 + 1;
    } while (puVar1[uVar24] != 0);
    iVar29 = (int)uVar24;
    iVar13 = 0;
    if (0 < iVar29) {
      iVar13 = iVar29;
    }
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2de2a;
    piVar17 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar13 * 2 + 0x12));
    piVar17[1] = iVar13;
    *piVar17 = -1;
    piVar28 = piVar17 + 4;
    piVar17[2] = 0;
    *(undefined2 *)piVar28 = 0;
    local_4e0 = piVar28;
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2de55;
    FUN_142ef7ba0(piVar28,puVar1,(longlong)iVar29 * 2);
    if (*piVar17 != -1) {
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2de67;
      FUN_142e52dd0(0x8b);
    }
    if ((iVar29 == -1) || (iVar13 = piVar17[1], iVar29 <= iVar13)) {
      *piVar17 = 1;
      if (iVar29 != -1) goto LAB_141b2de87;
      uVar24 = uVar32;
      if (piVar28 == (int *)0x0) {
        uVar24 = 0;
      }
      else {
        do {
          uVar24 = uVar24 + 1;
        } while (*(short *)((longlong)piVar28 + uVar24 * 2) != 0);
      }
    }
    else {
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2de80;
      FUN_142e54290(0x90,iVar13,uVar24 & 0xffffffff);
      *piVar17 = 1;
LAB_141b2de87:
      *(undefined2 *)((longlong)iVar29 * 2 + (longlong)piVar28) = 0;
    }
    iVar13 = (int)uVar24;
    if ((iVar13 < 0) || (piVar17[1] + 1 <= iVar13)) {
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2dea9;
      FUN_142e54290(0x9c,uVar24 & 0xffffffff);
    }
    piVar17[2] = iVar13 * 2;
    param_1 = local_4e8;
  }
  else {
    *(undefined4 *)((longlong)&local_580 + lVar3) = 0x100000;
    *(undefined2 **)((longlong)local_588 + lVar3) = puVar1;
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2dde8;
    (*DAT_1432627f8)(0xfde9,0,lVar30,0xffffffff);
LAB_141b2dde8:
    local_4e0 = (int *)0x0;
    if (puVar1 != (undefined2 *)0x0) goto LAB_141b2de00;
  }
  lVar30 = 0;
  if (bVar10 != 0) {
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2dec6;
    uVar14 = FUN_142c4a810(DAT_143ac1898);
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2decd;
    cVar11 = FUN_1401e7bf0(uVar14);
    if (cVar11 == '\0') {
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df07;
      FUN_141d60f20();
    }
    else {
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ded6;
      FUN_141d60f80();
    }
  }
  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df0f;
  uVar12 = FUN_1406e8ae0(param_2);
  *(undefined1 *)(param_1 + 0x204) = uVar12;
  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df1d;
  FUN_1406e8c20(param_2);
  if (DAT_143ad2220 != 0) {
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df33;
    FUN_141b5df90(DAT_143ad2220,1);
  }
  if (bVar10 != 0) {
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df4e;
    lVar18 = FUN_142c4eea0(DAT_143ac1898);
    if ((*(char **)(lVar18 + 0x18) == (char *)0x0) || (**(char **)(lVar18 + 0x18) == '\0')) {
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df69;
      FUN_140d21280(lVar18,6);
    }
    if (bVar10 == 2) {
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df7b;
      bVar10 = FUN_1406e8ae0(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df92;
      FUN_1406e9170(param_2,&local_4a8,8);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2df9e;
      FUN_1406e9050(param_2,&local_4f8);
      if (bVar10 != 0) {
        if (bVar10 - 0x15 < 0x28) {
          bVar10 = 0xc;
        }
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2dfcc;
        (*DAT_143ad5650)(&local_4a8,&local_488);
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2dfd1;
        local_528 = (longlong *)FUN_1408f6690();
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2dfe3;
        local_508 = (undefined1 **)FUN_1408f63b0(&local_528,0x438);
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2dff8;
        (*DAT_143ad5648)(&local_508,&local_4a8);
        local_530 = (undefined1 *)0x0;
        if ((ushort)local_488 < 0x81f) {
          if (0xc < (ushort)uStack_480) {
            lVar30 = 8;
          }
          uVar21 = *(undefined8 *)((longlong)&DAT_143ad2210 + lVar30);
          uVar23 = *(undefined8 *)
                    (&DAT_143ad21b0 + (longlong)((int)(local_488._2_2_ - 1) % 0xc) * 8);
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e06b;
          puVar19 = (undefined8 *)FUN_1408a9e40(&local_4f0,0x7da);
          uVar2 = *puVar19;
          uVar9 = uStack_480._2_2_;
          uVar8 = (ushort)uStack_480;
          uVar6 = (ushort)local_488;
          uVar7 = local_488._6_2_;
          *(undefined8 *)((longlong)&local_570 + lVar3) = uVar21;
          *(uint *)((longlong)local_578 + lVar3) = (uint)uVar9;
          *(uint *)((longlong)&local_580 + lVar3) = (uint)uVar8;
          *(uint *)((longlong)local_588 + lVar3) = (uint)uVar6;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e0ac;
          FUN_14019ba10(&local_530,uVar2,uVar23,uVar7);
          if (local_4f0 != (longlong **)0x0) {
            pplVar25 = local_4f0 + -2;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e0bf;
            FUN_14019f2c0(pplVar25);
          }
LAB_141b2e0c1:
          puVar27 = local_530;
          if (bVar10 == 99) {
            local_518[0] = (longlong *)0x0;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e0e6;
            FUN_14019ba10(local_518,
                          "Your account was locked after five invalid password or pin code attempts.\r\n%s"
                          ,puVar27);
            plVar20 = local_518[0];
            local_538 = (longlong *)0x0;
            plVar31 = local_538;
            if ((local_518[0] != (longlong *)0x0) &&
               (plVar26 = local_518[0] + -2, plVar26 != (longlong *)0x0)) {
              if ((int)*plVar26 == -1) {
                *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e11b;
                FUN_142e52d50(0xcb,0xffffff01);
                uVar24 = uVar32;
                do {
                  uVar24 = uVar24 + 1;
                } while (*(char *)((longlong)plVar20 + uVar24) != '\0');
                iVar29 = (int)uVar24;
                iVar13 = 0;
                if (0 < iVar29) {
                  iVar13 = iVar29;
                }
                *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e149;
                piVar17 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar13 + 0x11));
                piVar17[1] = iVar13;
                *piVar17 = -1;
                plVar31 = (longlong *)(piVar17 + 4);
                piVar17[2] = 0;
                *(char *)plVar31 = '\0';
                local_528 = plVar31;
                *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e16f;
                FUN_142ef7ba0(plVar31,plVar20,(longlong)iVar29);
                if (*piVar17 != -1) {
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e181;
                  FUN_142e52dd0(0x8b);
                }
                if ((iVar29 == -1) || (iVar13 = piVar17[1], iVar29 <= iVar13)) {
                  *piVar17 = 1;
                  if (iVar29 != -1) goto LAB_141b2e1ab;
                  if (plVar31 != (longlong *)0x0) {
                    do {
                      uVar32 = uVar32 + 1;
                    } while (*(char *)((longlong)plVar31 + uVar32) != '\0');
                    uVar24 = uVar32 & 0xffffffff;
                    goto LAB_141b2e1b0;
                  }
                  iVar13 = 0;
                }
                else {
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e1a4;
                  FUN_142e54290(0x90,iVar13,uVar24 & 0xffffffff);
                  *piVar17 = 1;
LAB_141b2e1ab:
                  *(char *)((longlong)plVar31 + (longlong)iVar29) = '\0';
LAB_141b2e1b0:
                  iVar13 = (int)uVar24;
                }
                if ((iVar13 < 0) || (piVar17[1] + 1 <= iVar13)) {
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e1d2;
                  FUN_142e54290(0x9c,iVar13);
                }
                piVar17[2] = iVar13;
                if (local_538 != (longlong *)0x0) {
                  plVar26 = local_538 + -2;
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e1e8;
                  FUN_14019f2c0(plVar26);
                }
              }
              else {
                if ((int)*plVar26 < 1) {
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e28b;
                  FUN_142e52dd0(0xd2);
                }
                LOCK();
                *(int *)plVar26 = (int)*plVar26 + 1;
                UNLOCK();
                if (local_538 != (longlong *)0x0) {
                  plVar31 = local_538 + -2;
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e2a0;
                  FUN_14019f2c0(plVar31);
                }
                local_538 = plVar20;
                piVar28 = local_4e0;
                plVar20 = local_518[0];
                puVar27 = local_530;
                plVar31 = local_538;
              }
            }
            local_538 = plVar31;
            *(undefined4 *)((longlong)local_560 + lVar3) = 0;
            *(undefined4 *)((longlong)&local_568 + lVar3) = 0;
            *(undefined4 *)((longlong)&local_570 + lVar3) = 0;
            *(undefined4 *)((longlong)local_578 + lVar3) = 0;
            *(undefined4 *)((longlong)&local_580 + lVar3) = 0;
            *(undefined4 *)((longlong)local_588 + lVar3) = 0;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e2e0;
            FUN_142a26280(&local_538,0,0,1);
          }
          else {
            if (bVar10 != 199) goto LAB_141b2e4e5;
            local_518[0] = (longlong *)0x0;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e30e;
            FUN_14019ba10(local_518,
                          "Your account was locked after ten invalid password or pin code attempts.\r\n%s"
                          ,puVar27);
            plVar20 = local_518[0];
            local_538 = (longlong *)0x0;
            plVar31 = local_538;
            if ((local_518[0] != (longlong *)0x0) &&
               (plVar26 = local_518[0] + -2, plVar26 != (longlong *)0x0)) {
              if ((int)*plVar26 == -1) {
                *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e343;
                FUN_142e52d50(0xcb,0xffffff01);
                uVar24 = uVar32;
                do {
                  uVar24 = uVar24 + 1;
                } while (*(char *)((longlong)plVar20 + uVar24) != '\0');
                iVar29 = (int)uVar24;
                iVar13 = 0;
                if (0 < iVar29) {
                  iVar13 = iVar29;
                }
                *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e36f;
                piVar17 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar13 + 0x11));
                piVar17[1] = iVar13;
                *piVar17 = -1;
                plVar31 = (longlong *)(piVar17 + 4);
                piVar17[2] = 0;
                *(char *)plVar31 = '\0';
                local_528 = plVar31;
                *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e395;
                FUN_142ef7ba0(plVar31,plVar20,(longlong)iVar29);
                if (*piVar17 != -1) {
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e3a7;
                  FUN_142e52dd0(0x8b);
                }
                if ((iVar29 == -1) || (iVar13 = piVar17[1], iVar29 <= iVar13)) {
                  *piVar17 = 1;
                  if (iVar29 != -1) goto LAB_141b2e3c9;
                  if (plVar31 != (longlong *)0x0) {
                    do {
                      uVar32 = uVar32 + 1;
                    } while (*(char *)((longlong)plVar31 + uVar32) != '\0');
                    uVar24 = uVar32 & 0xffffffff;
                    goto LAB_141b2e3ce;
                  }
                  iVar13 = 0;
                }
                else {
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e3c2;
                  FUN_142e54290(0x90,iVar13,uVar24 & 0xffffffff);
                  *piVar17 = 1;
LAB_141b2e3c9:
                  *(char *)((longlong)plVar31 + (longlong)iVar29) = '\0';
LAB_141b2e3ce:
                  iVar13 = (int)uVar24;
                }
                if ((iVar13 < 0) || (piVar17[1] + 1 <= iVar13)) {
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e3f0;
                  FUN_142e54290(0x9c,iVar13);
                }
                piVar17[2] = iVar13;
                if (local_538 != (longlong *)0x0) {
                  plVar26 = local_538 + -2;
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e406;
                  FUN_14019f2c0(plVar26);
                }
              }
              else {
                if ((int)*plVar26 < 1) {
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e455;
                  FUN_142e52dd0(0xd2);
                }
                LOCK();
                *(int *)plVar26 = (int)*plVar26 + 1;
                UNLOCK();
                if (local_538 != (longlong *)0x0) {
                  plVar31 = local_538 + -2;
                  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e46a;
                  FUN_14019f2c0(plVar31);
                }
                local_538 = plVar20;
                piVar28 = local_4e0;
                plVar20 = local_518[0];
                puVar27 = local_530;
                plVar31 = local_538;
              }
            }
            local_538 = plVar31;
            *(undefined4 *)((longlong)local_560 + lVar3) = 0;
            *(undefined4 *)((longlong)&local_568 + lVar3) = 0;
            *(undefined4 *)((longlong)&local_570 + lVar3) = 0;
            *(undefined4 *)((longlong)local_578 + lVar3) = 0;
            *(undefined4 *)((longlong)&local_580 + lVar3) = 0;
            *(undefined4 *)((longlong)local_588 + lVar3) = 0;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e4aa;
            FUN_142a26280(&local_538,0,0,1);
          }
          if (plVar20 != (longlong *)0x0) {
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e4b9;
            FUN_14019f2c0(plVar20 + -2);
          }
          if (puVar27 != (undefined1 *)0x0) {
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e4c8;
            FUN_14019f2c0(puVar27 + -0x10);
          }
          if (local_4f8 != 0) {
            lVar30 = local_4f8 + -0x10;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e4db;
            FUN_14019f2c0(lVar30);
          }
          goto LAB_141b2ee30;
        }
        if (bVar10 != 200) {
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e21e;
          puVar19 = (undefined8 *)FUN_1408a9e40(&local_4f0,0x804);
          uVar21 = *puVar19;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e22b;
          FUN_14019ba10(&local_530,uVar21);
          if (local_4f0 != (longlong **)0x0) {
            pplVar25 = local_4f0 + -2;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e242;
            FUN_14019f2c0(pplVar25);
          }
          goto LAB_141b2e0c1;
        }
LAB_141b2e4e5:
        puVar27 = local_530;
        local_538 = (longlong *)0x0;
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e4f6;
        FUN_14019a260(&local_538,&local_4f8);
        plVar31 = local_538;
        if ((local_538 == (longlong *)0x0) || ((char)*local_538 == '\0')) {
          if (bVar10 == 0x65) {
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e64c;
            puVar19 = (undefined8 *)FUN_1408a9e40(&local_528,0x16d7);
            uVar21 = *puVar19;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e65c;
            FUN_14019ba10(&local_538,uVar21,puVar27);
          }
          else {
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e669;
            puVar19 = (undefined8 *)FUN_1408a9e40(&local_528,0x16d6);
            uVar21 = *puVar19;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e679;
            FUN_14019ba10(&local_538,uVar21,puVar27);
          }
          if (local_528 != (longlong *)0x0) {
            plVar31 = local_528 + -2;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e68c;
            FUN_14019f2c0(plVar31);
          }
          local_518[0] = (longlong *)0x0;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e6a1;
          FUN_14019a260(local_518,&local_538);
          *(undefined4 *)((longlong)local_560 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_568 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_570 + lVar3) = 0;
          *(undefined4 *)((longlong)local_578 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_580 + lVar3) = 0;
          *(undefined4 *)((longlong)local_588 + lVar3) = 0;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e6d1;
          FUN_142a26280(local_518,0,0,1);
          plVar31 = local_538;
        }
        else {
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e528;
          plVar20 = (longlong *)FUN_14040dab0(&local_538,&local_528,&DAT_14329e064,&DAT_143295a78);
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e534;
          FUN_14019f2c0(plVar31 + -2);
          plVar31 = (longlong *)*plVar20;
          *plVar20 = 0;
          local_538 = plVar31;
          if (local_528 != (longlong *)0x0) {
            plVar20 = local_528 + -2;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e552;
            FUN_14019f2c0(plVar20);
          }
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e56e;
          puVar19 = (undefined8 *)FUN_14040dab0(&local_538,&local_528,&DAT_143295a70,&DAT_1434b7dfc)
          ;
          if (plVar31 != (longlong *)0x0) {
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e57f;
            FUN_14019f2c0(plVar31 + -2);
          }
          plVar31 = (longlong *)*puVar19;
          *puVar19 = 0;
          local_538 = plVar31;
          if (local_528 != (longlong *)0x0) {
            plVar20 = local_528 + -2;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e59e;
            FUN_14019f2c0(plVar20);
          }
          do {
            pcVar22 = &DAT_1433a9e11 + uVar32;
            uVar32 = uVar32 + 1;
          } while (*pcVar22 != '\0');
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e5c7;
          FUN_1401abc80(&local_538,&local_528);
          uVar14 = 0;
          if (puVar27 != (undefined1 *)0x0) {
            uVar14 = *(undefined4 *)(puVar27 + -8);
          }
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e5e9;
          FUN_1401abc80(&local_528,&local_4f0,puVar27,uVar14);
          *(undefined4 *)((longlong)local_560 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_568 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_570 + lVar3) = 0;
          *(undefined4 *)((longlong)local_578 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_580 + lVar3) = 0;
          *(undefined4 *)((longlong)local_588 + lVar3) = 0;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e61b;
          FUN_142a26280(&local_4f0,0,0,1);
          if (local_528 != (longlong *)0x0) {
            plVar20 = local_528 + -2;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e62e;
            FUN_14019f2c0(plVar20);
          }
          uVar15 = (uint)local_518[0];
        }
        if (plVar31 != (longlong *)0x0) {
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e6e3;
          FUN_14019f2c0(plVar31 + -2);
        }
        param_1 = local_4e8;
        if (puVar27 != (undefined1 *)0x0) {
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e6f2;
          FUN_14019f2c0(puVar27 + -0x10);
          param_1 = local_4e8;
        }
      }
      if (local_4f8 != 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e709;
        FUN_14019f2c0(local_4f8 + -0x10);
      }
    }
    else {
      if (bVar10 == 0x2d) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e7b4;
        uVar21 = FUN_1403edf80(&local_508,L"cannotDeleteCharacterOnMapleTogether",0xffffffff);
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e7c6;
        iVar13 = FUN_141b49b80(2,uVar21,0,0);
        if (iVar13 != 0) {
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e7da;
          FUN_141d60b50(local_448);
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e7eb;
          FUN_1429e4fa0(local_448,0,0);
        }
        goto LAB_141b2ee30;
      }
      if (bVar10 == 0x8d) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e80b;
        uVar21 = FUN_1408a9e40(&local_508,0x174f);
        *(undefined4 *)((longlong)&local_558 + lVar3) = 0;
        *(undefined4 *)((longlong)local_560 + lVar3) = 0;
        *(undefined4 *)((longlong)&local_568 + lVar3) = 3;
        *(undefined4 *)((longlong)&local_570 + lVar3) = 0;
        *(undefined4 *)((longlong)local_578 + lVar3) = 0;
        *(undefined4 *)((longlong)&local_580 + lVar3) = 0xffffffff;
        *(undefined4 *)((longlong)local_588 + lVar3) = 0;
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e83f;
        iVar13 = FUN_142a269c0(uVar21,0,0,1);
        if (iVar13 == 6) {
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e852;
          puVar19 = (undefined8 *)FUN_1408a9e40(&local_508,0x1756);
          uVar21 = *puVar19;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e860;
          FUN_1429e4fa0(uVar21,0,0);
          if (local_508 != (undefined1 **)0x0) {
            lVar30 = (longlong)local_508 + -0x10;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e873;
            FUN_14019f2c0(lVar30);
          }
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e88b;
          FUN_140cc2350(&DAT_143271f04,0x824,0x2100000f);
        }
        else {
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e89a;
          uVar21 = FUN_1408a9e40(&local_508,0x1724);
          *(undefined4 *)((longlong)local_560 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_568 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_570 + lVar3) = 0;
          *(undefined4 *)((longlong)local_578 + lVar3) = 0;
          *(undefined4 *)((longlong)&local_580 + lVar3) = 0;
          *(undefined4 *)((longlong)local_588 + lVar3) = 0;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e8c5;
          FUN_142a26280(uVar21,0,0,1);
        }
        goto LAB_141b2ee30;
      }
    }
  }
  local_4f8 = 0;
  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e723;
  FUN_1401c1fb0(&local_4f8,&local_4e0);
  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e735;
  cVar11 = FUN_141b267c0(param_1,uVar15,0,&local_4f8);
  if (cVar11 != '\0') {
    cVar11 = *(char *)(param_1 + 0x204);
    if ((cVar11 == '\0') || (cVar11 == '\x01')) {
      if (DAT_143ad2220 != 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e94c;
        FUN_141b5df90(DAT_143ad2220,0);
      }
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e95b;
      FUN_1406e9050(param_2,&local_4b0);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e96f;
      FUN_142cb7e50(DAT_143aa84a0,&local_4b0);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e977;
      FUN_1406e8f10(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e97f;
      uVar14 = FUN_1406e8c20(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e98a;
      local_51d = FUN_1406e8ae0(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e995;
      uVar15 = FUN_1406e8c20(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e9aa;
      FUN_1406e9170(param_2,local_4d8,4);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e9b2;
      local_51e = FUN_1406e8ae0(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e9c4;
      FUN_1406e9050(param_2,&local_4b8);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e9cd;
      uVar16 = FUN_1406e8c20(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e9db;
      FUN_142cb8400(DAT_143aa84a0,uVar16);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e9e3;
      local_51f = FUN_1406e8ae0(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e9ee;
      local_520 = FUN_1406e8ae0(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea03;
      FUN_1406e9170(param_2,local_498,8);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea17;
      (*DAT_143ad5650)(local_498,&local_468);
      if ((uVar15 >> 0x15 & 1) != 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea2a;
        FUN_140d2d4e0(DAT_143aa84a0);
      }
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea3c;
      FUN_1406e9170(param_2,local_490,8);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea50;
      (*DAT_143ad5650)(local_490,&local_478);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea58;
      uVar16 = FUN_1406e8c20(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea66;
      FUN_1406e9050(param_2,&local_4c0);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea79;
      FUN_1406e9170(param_2,local_518,4);
      uVar32 = (ulonglong)local_518[0] & 0xffffffff;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ea88;
      FUN_142ce9780(DAT_143aa84a0,uVar32);
      uVar21 = DAT_143aa84a0;
      local_508 = &local_530;
      local_530 = (undefined1 *)0x0;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2eaab;
      FUN_14019bd40(&local_530,0,0);
      puVar27 = local_530;
      if (*(int *)(local_530 + -0x10) != -1) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2eac1;
        FUN_142e52dd0();
      }
      iVar13 = *(int *)(puVar27 + -0xc);
      if (iVar13 < 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ead5;
        FUN_142e54290(0x90,iVar13,0);
      }
      *(undefined4 *)(puVar27 + -0x10) = 1;
      *local_530 = 0;
      if (*(int *)(puVar27 + -0xc) + 1 < 1) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2eafb;
        FUN_142e54290(0x9c,0);
      }
      *(undefined4 *)(puVar27 + -8) = 0;
      local_528 = &local_4f8;
      local_4f8 = 0;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2eb19;
      FUN_14019a260(&local_4f8,&local_4c0);
      local_488 = local_478;
      uStack_480 = uStack_470;
      local_4a8 = local_468;
      uStack_4a4 = uStack_464;
      uStack_4a0 = uStack_460;
      uStack_49c = uStack_45c;
      local_538 = (longlong *)0x0;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2eb4f;
      FUN_14019a260(&local_538,&local_4b8);
      local_508 = (undefined1 **)CONCAT44(uStack_4a4,local_4a8);
      uStack_500 = uStack_4a0;
      uStack_4fc = uStack_49c;
      *(undefined1 ***)((longlong)local_548 + lVar3 + 8) = &local_530;
      *(longlong **)((longlong)local_548 + lVar3) = &local_4f8;
      *(undefined4 *)((longlong)local_550 + lVar3) = uVar16;
      *(undefined8 **)((longlong)&local_558 + lVar3) = &local_488;
      *(undefined4 *)((longlong)local_560 + lVar3) = local_4d8[0];
      *(undefined8 **)((longlong)&local_568 + lVar3) = &local_508;
      *(undefined1 *)((longlong)&local_570 + lVar3) = local_520;
      *(undefined1 *)((longlong)local_578 + lVar3) = local_51f;
      *(longlong ***)((longlong)&local_580 + lVar3) = &local_538;
      *(undefined1 *)((longlong)local_588 + lVar3) = local_51e;
      uVar12 = local_51d;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ebcb;
      FUN_142cb7e60(uVar21,uVar14,uVar12,uVar15);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ebd3;
      FUN_1406e8ae0(param_2);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ebdb;
      cVar11 = FUN_1406e8ae0(param_2);
      lVar30 = local_4e8;
      *(uint *)(local_4e8 + 0x1a4) = (uint)(cVar11 != '\0');
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ebfd;
      iVar13 = FUN_142c4f6d0(DAT_143ac1898);
      if (iVar13 != 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ec0a;
        cVar11 = FUN_14057e4c0();
        if (cVar11 != '\0') {
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ec17;
          uVar21 = FUN_1404c6160();
          local_530 = (undefined1 *)0x0;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ec2e;
          pcVar22 = (char *)FUN_14019bd40(&local_530,0x12,0);
          puVar27 = local_530;
          uVar5 = s_GC_LoginGameServer_1433fe150._12_4_;
          uVar4 = s_GC_LoginGameServer_1433fe150._8_4_;
          uVar16 = s_GC_LoginGameServer_1433fe150._4_4_;
          *(undefined4 *)pcVar22 = s_GC_LoginGameServer_1433fe150._0_4_;
          *(undefined4 *)(pcVar22 + 4) = uVar16;
          *(undefined4 *)(pcVar22 + 8) = uVar4;
          *(undefined4 *)(pcVar22 + 0xc) = uVar5;
          *(undefined2 *)(pcVar22 + 0x10) = s_GC_LoginGameServer_1433fe150._16_2_;
          if (*(int *)(local_530 + -0x10) != -1) {
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ec59;
            FUN_142e52dd0(0x8b);
          }
          iVar13 = *(int *)(puVar27 + -0xc);
          if (iVar13 < 0x12) {
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ec6f;
            FUN_142e54290(0x90,iVar13,0x12);
          }
          *(undefined4 *)(puVar27 + -0x10) = 1;
          local_530[0x12] = 0;
          if (*(int *)(puVar27 + -0xc) + 1 < 0x13) {
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ec9a;
            FUN_142e54290(0x9c,0x12);
          }
          *(undefined4 *)(puVar27 + -8) = 0x12;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ecb2;
          FUN_1404c7800(uVar21,0xc,&local_530);
          if (local_530 != (undefined1 *)0x0) {
            puVar27 = local_530 + -0x10;
            *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ecc5;
            FUN_14019f2c0(puVar27);
          }
        }
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2eccb;
        FUN_1415db7e0();
      }
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ecd3;
      uVar12 = FUN_1406e8ae0(param_2);
      *(undefined1 *)(lVar30 + 0xdc) = uVar12;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ecf1;
      FUN_1406e9170(param_2,local_458,8);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ed07;
      FUN_142cb6330(DAT_143aa84a0,local_458,8);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ed13;
      FUN_1406e9050(param_2,&local_4c8);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ed24;
      FUN_142cb8370(DAT_143aa84a0,&local_4c8);
      uVar21 = DAT_143ac8210;
      local_530 = (undefined1 *)0x0;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ed3d;
      FUN_14019bd40(&local_530,0,0);
      puVar27 = local_530;
      if (*(int *)(local_530 + -0x10) != -1) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ed53;
        FUN_142e52dd0(0x8b);
      }
      iVar13 = *(int *)(puVar27 + -0xc);
      if (iVar13 < 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ed67;
        FUN_142e54290(0x90,iVar13,0);
      }
      *(undefined4 *)(puVar27 + -0x10) = 1;
      *local_530 = 0;
      if (*(int *)(puVar27 + -0xc) + 1 < 1) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ed8d;
        FUN_142e54290(0x9c,0);
      }
      *(undefined4 *)(puVar27 + -8) = 0;
      local_528 = (longlong *)0x0;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2eda8;
      uVar23 = FUN_14019ba10(&local_528,&DAT_1433fe164,uVar14);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2edb7;
      FUN_140c7a870(uVar21,uVar23,&local_530);
      if (local_528 != (longlong *)0x0) {
        plVar31 = local_528 + -2;
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2edca;
        FUN_14019f2c0(plVar31);
      }
      if (local_530 != (undefined1 *)0x0) {
        puVar27 = local_530 + -0x10;
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2eddd;
        FUN_14019f2c0(puVar27);
      }
      if (local_4c8 != 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2edf0;
        FUN_14019f2c0(local_4c8 + -0x10);
      }
      if (local_4c0 != 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ee03;
        FUN_14019f2c0(local_4c0 + -0x10);
      }
      if (local_4b8 != 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ee19;
        FUN_14019f2c0(local_4b8 + -0x10);
      }
      if (local_4b0 != 0) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ee2f;
        FUN_14019f2c0(local_4b0 + -0x10);
      }
    }
    else if ((cVar11 == '\x02') || (cVar11 == '\x03')) {
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e8dd;
      uVar21 = FUN_1403edf80(&local_508,L"accountHasNotBeenVerified",0xffffffff);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e8ef;
      iVar13 = FUN_141b49b80(2,uVar21,0);
      if ((iVar13 != 0) && (*(int *)(param_1 + 0xd0) == 1)) {
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e912;
        puVar19 = (undefined8 *)FUN_1408a9e40(&local_508,0x1757);
        uVar21 = *puVar19;
        *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e920;
        FUN_1429e4fa0(uVar21,0,0);
        if (local_508 != (undefined1 **)0x0) {
          lVar30 = (longlong)local_508 + -0x10;
          *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e933;
          FUN_14019f2c0(lVar30);
        }
      }
    }
    else {
      local_4f0 = &local_528;
      local_528 = (longlong *)0x0;
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e786;
      uVar21 = FUN_1403edf80(&local_508,L"loginTroubleAskSupport",0xffffffff);
      *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2e796;
      FUN_141b4ac80(uVar21,&local_528,0);
    }
  }
LAB_141b2ee30:
  if (piVar28 != (int *)0x0) {
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ee3e;
    FUN_1401bebb0(piVar28 + -4);
  }
  if (local_4d0 != 0) {
    *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ee51;
    FUN_14019f2c0(local_4d0 + -0x10);
  }
  *(undefined8 *)((longlong)auStack_5b0 + lVar3) = 0x141b2ee61;
  return;
}



//===========================================================
// FUN_141b2ee90 @ 141b2ee90   (1868 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141b2f18a) */

void FUN_141b2ee90(longlong param_1,undefined8 param_2)

{
  int **ppiVar1;
  undefined2 *puVar2;
  code *pcVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  ulonglong uVar7;
  int **ppiVar8;
  char cVar9;
  undefined1 uVar10;
  int iVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  int *piVar14;
  undefined8 *puVar15;
  ulonglong uVar16;
  longlong lVar17;
  int **ppiVar18;
  int **ppiVar19;
  int iVar20;
  undefined8 auStack_190 [5];
  undefined8 local_168;
  longlong local_160;
  undefined1 local_158 [8];
  undefined1 local_150 [8];
  longlong local_148;
  undefined4 local_140 [2];
  longlong local_138;
  undefined4 local_130 [2];
  longlong local_128 [2];
  undefined2 local_118;
  undefined1 local_116;
  int *local_110;
  int **local_108;
  undefined8 local_100;
  int **local_f8;
  undefined4 local_f0 [2];
  undefined8 local_e8;
  ulonglong local_e0;
  int **local_d8;
  longlong local_d0;
  longlong local_c8;
  longlong local_c0;
  undefined1 local_b8 [8];
  undefined1 local_b0 [8];
  longlong local_a8;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 *local_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined4 local_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined1 local_50 [8];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)&local_118;
  auStack_190[0] = 0x141b2eed2;
  local_a8 = param_1;
  cVar9 = FUN_1406e8ae0(param_2);
  auStack_190[0] = 0x141b2eee2;
  FUN_1406e9050(param_2,&local_e0);
  ppiVar18 = (int **)0x0;
  if (local_e0 == 0) {
    iVar11 = 2;
  }
  else {
    local_160._0_4_ = 0;
    local_168 = 0;
    auStack_190[0] = 0x141b2ef10;
    iVar11 = (*DAT_1432627f8)(0xfde9,0,local_e0,0xffffffff);
    iVar11 = iVar11 * 2;
  }
  uVar7 = local_e0;
  uVar16 = (longlong)iVar11 + 0xf;
  if (uVar16 <= (ulonglong)(longlong)iVar11) {
    uVar16 = 0xffffffffffffff0;
  }
  auStack_190[0] = 0x141b2ef37;
  lVar5 = -(uVar16 & 0xfffffffffffffff0);
  puVar2 = (undefined2 *)((longlong)&local_118 + lVar5);
  if (local_e0 == 0) {
    if (puVar2 == (undefined2 *)0x0) goto LAB_141b2ef6e;
    *puVar2 = 0;
LAB_141b2ef7e:
    uVar16 = 0xffffffffffffffff;
    do {
      uVar16 = uVar16 + 1;
    } while (puVar2[uVar16] != 0);
    iVar20 = (int)uVar16;
    iVar11 = 0;
    if (0 < iVar20) {
      iVar11 = iVar20;
    }
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2efad;
    piVar14 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar11 * 2 + 0x12));
    piVar14[1] = iVar11;
    *piVar14 = -1;
    ppiVar18 = (int **)(piVar14 + 4);
    piVar14[2] = 0;
    *(undefined2 *)ppiVar18 = 0;
    local_d8 = ppiVar18;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2efdb;
    FUN_142ef7ba0(ppiVar18,puVar2,(longlong)iVar20 * 2);
    if (*piVar14 != -1) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2efed;
      FUN_142e52dd0(0x8b);
    }
    if ((iVar20 == -1) || (iVar11 = piVar14[1], iVar20 <= iVar11)) {
      *piVar14 = 1;
      if (iVar20 != -1) goto LAB_141b2f015;
      if (ppiVar18 == (int **)0x0) {
        uVar16 = 0;
      }
      else {
        uVar16 = 0xffffffffffffffff;
        do {
          uVar16 = uVar16 + 1;
        } while (*(short *)((longlong)ppiVar18 + uVar16 * 2) != 0);
      }
    }
    else {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f00e;
      FUN_142e54290(0x90,iVar11,uVar16 & 0xffffffff);
      *piVar14 = 1;
LAB_141b2f015:
      *(undefined2 *)((longlong)iVar20 * 2 + (longlong)ppiVar18) = 0;
    }
    iVar11 = (int)uVar16;
    if ((iVar11 < 0) || (piVar14[1] + 1 <= iVar11)) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f03b;
      FUN_142e54290(0x9c,uVar16 & 0xffffffff);
    }
    piVar14[2] = iVar11 * 2;
  }
  else {
    *(undefined4 *)((longlong)&local_160 + lVar5) = 0x100000;
    *(undefined2 **)((longlong)&local_168 + lVar5) = puVar2;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2ef6e;
    (*DAT_1432627f8)(0xfde9,0,uVar7,0xffffffff);
LAB_141b2ef6e:
    local_d8 = (int **)0x0;
    if (puVar2 != (undefined2 *)0x0) goto LAB_141b2ef7e;
  }
  if ((DAT_143aa8520 != 0) && (cVar9 != '\0')) {
    pcVar3 = *(code **)(*(longlong *)(DAT_143aa8520 + 8) + 0xd0);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f066;
    iVar11 = (*pcVar3)((longlong *)(DAT_143aa8520 + 8),&PTR_PTR_143a886f8);
    lVar17 = 0;
    if (iVar11 != 0) {
      lVar17 = DAT_143aa8520;
    }
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f078;
    FUN_141b6e530(lVar17);
  }
  local_108 = (int **)0x0;
  ppiVar19 = ppiVar18;
  ppiVar8 = local_108;
  if ((ppiVar18 == (int **)0x0) || (ppiVar1 = ppiVar18 + -2, ppiVar1 == (int **)0x0))
  goto LAB_141b2f239;
  if (*(int *)ppiVar1 != -1) {
    if (*(int *)ppiVar1 < 1) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f218;
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *(int *)ppiVar1 = *(int *)ppiVar1 + 1;
    UNLOCK();
    ppiVar19 = local_d8;
    ppiVar8 = ppiVar18;
    if (local_108 != (int **)0x0) {
      ppiVar18 = local_108 + -2;
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f22d;
      FUN_1401bebb0(ppiVar18);
      ppiVar19 = local_d8;
    }
    goto LAB_141b2f239;
  }
  *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f0a9;
  FUN_142e52d50(0xcb,0xffffff01);
  uVar16 = 0xffffffffffffffff;
  do {
    uVar16 = uVar16 + 1;
  } while (*(short *)((longlong)ppiVar18 + uVar16 * 2) != 0);
  iVar20 = (int)uVar16;
  iVar11 = 0;
  if (0 < iVar20) {
    iVar11 = iVar20;
  }
  *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f0d8;
  piVar14 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar11 * 2 + 0x12));
  piVar14[1] = iVar11;
  *piVar14 = -1;
  ppiVar8 = (int **)(piVar14 + 4);
  piVar14[2] = 0;
  *(undefined2 *)ppiVar8 = 0;
  local_f8 = ppiVar8;
  *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f106;
  FUN_142ef7ba0(ppiVar8,ppiVar18,(longlong)iVar20 * 2);
  if (*piVar14 != -1) {
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f118;
    FUN_142e52dd0(0x8b);
  }
  if ((iVar20 == -1) || (iVar11 = piVar14[1], iVar20 <= iVar11)) {
    *piVar14 = 1;
    if (iVar20 != -1) goto LAB_141b2f140;
    if (ppiVar8 == (int **)0x0) {
      uVar16 = 0;
    }
    else {
      uVar16 = 0xffffffffffffffff;
      do {
        uVar16 = uVar16 + 1;
      } while (*(short *)((longlong)ppiVar8 + uVar16 * 2) != 0);
    }
  }
  else {
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f139;
    FUN_142e54290(0x90,iVar11,uVar16 & 0xffffffff);
    *piVar14 = 1;
LAB_141b2f140:
    *(undefined2 *)((longlong)ppiVar8 + (longlong)iVar20 * 2) = 0;
  }
  iVar11 = (int)uVar16;
  if ((iVar11 < 0) || (piVar14[1] + 1 <= iVar11)) {
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f166;
    FUN_142e54290(0x9c,uVar16 & 0xffffffff);
  }
  piVar14[2] = iVar11 * 2;
  if (local_108 != (int **)0x0) {
    ppiVar18 = local_108 + -2;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f17e;
    FUN_1401bebb0(ppiVar18);
  }
LAB_141b2f239:
  local_108 = ppiVar8;
  *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f24b;
  cVar9 = FUN_141b267c0(param_1,cVar9,1,&local_108);
  if (cVar9 != '\0') {
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f25b;
    uVar12 = FUN_1406e8c20(param_2);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f266;
    uVar10 = FUN_1406e8ae0(param_2);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f272;
    uVar13 = FUN_1406e8c20(param_2);
    local_108 = (int **)CONCAT44(local_108._4_4_,uVar13);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f287;
    FUN_1406e9170(param_2,local_f0,4);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f28f;
    local_116 = FUN_1406e8ae0(param_2);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f29e;
    FUN_1406e9050(param_2,&local_c0);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f2a7;
    local_118._1_1_ = FUN_1406e8ae0(param_2);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f2b2;
    local_118._0_1_ = FUN_1406e8ae0(param_2);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f2c7;
    FUN_1406e9170(param_2,local_b8,8);
    uVar6 = DAT_143aa84a0;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f2da;
    puVar15 = (undefined8 *)FUN_1406e9050(param_2,&local_f8);
    uVar4 = *puVar15;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f2e6;
    FUN_142cb6000(uVar6,uVar4);
    if (local_f8 != (int **)0x0) {
      ppiVar18 = local_f8 + -2;
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f2f9;
      FUN_14019f2c0(ppiVar18);
    }
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f30b;
    (*DAT_143ad5650)(local_b8,&local_60);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f31d;
    FUN_1406e9170(param_2,local_b0,8);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f32e;
    (*DAT_143ad5650)(local_b0,&local_70);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f336;
    uVar13 = FUN_1406e8c20(param_2);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f34d;
    FUN_1406e9170(param_2,local_50,8);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f366;
    FUN_142cb6330(DAT_143aa84a0,local_50,8);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f372;
    FUN_1406e9050(param_2,&local_c8);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f385;
    FUN_1406e9170(param_2,&local_110,4);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f394;
    FUN_142ce9780(DAT_143aa84a0,(ulonglong)local_110 & 0xffffffff);
    uVar4 = DAT_143aa84a0;
    local_f8 = &local_110;
    local_110 = (int *)0x0;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f3b8;
    piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar14[1] = 0;
    *piVar14 = -1;
    local_110 = piVar14 + 4;
    piVar14[2] = 0;
    *(undefined1 *)local_110 = 0;
    if (*piVar14 != -1) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f3e9;
      FUN_142e52dd0();
    }
    iVar11 = piVar14[1];
    if (iVar11 < 0) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f3fd;
      FUN_142e54290(0x90,iVar11,0);
    }
    *piVar14 = 1;
    *(undefined1 *)local_110 = 0;
    if (piVar14[1] + 1 < 1) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f423;
      FUN_142e54290(0x9c,0);
    }
    piVar14[2] = 0;
    local_78 = &local_e8;
    local_e8 = 0;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f444;
    FUN_14019a260(&local_e8,&local_c8);
    local_98 = local_70;
    uStack_90 = uStack_68;
    local_88 = local_60;
    uStack_84 = uStack_5c;
    uStack_80 = uStack_58;
    uStack_7c = uStack_54;
    local_100 = 0;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f477;
    FUN_14019a260(&local_100,&local_c0);
    *(int ***)((longlong)local_128 + lVar5 + 8) = &local_110;
    *(undefined8 **)((longlong)local_128 + lVar5) = &local_e8;
    *(undefined4 *)((longlong)local_130 + lVar5) = uVar13;
    *(undefined8 **)((longlong)&local_138 + lVar5) = &local_98;
    *(undefined4 *)((longlong)local_140 + lVar5) = local_f0[0];
    *(undefined4 **)((longlong)&local_148 + lVar5) = &local_88;
    local_150[lVar5] = (undefined1)local_118;
    local_158[lVar5] = local_118._1_1_;
    *(undefined8 **)((longlong)&local_160 + lVar5) = &local_100;
    *(undefined1 *)((longlong)&local_168 + lVar5) = local_116;
    uVar16 = (ulonglong)local_108 & 0xffffffff;
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f4f9;
    FUN_142cb7e60(uVar4,uVar12,uVar10,uVar16);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f505;
    FUN_142ceb900(DAT_143aa84a0);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f50d;
    FUN_1406e8ae0(param_2);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f515;
    cVar9 = FUN_1406e8ae0(param_2);
    lVar17 = local_a8;
    *(uint *)(local_a8 + 0x1a4) = (uint)(cVar9 != '\0');
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f530;
    cVar9 = FUN_1406e8ae0(param_2);
    if (cVar9 == '\0') {
      *(undefined4 *)(lVar17 + 0xd4) = 0;
    }
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f546;
    FUN_1406e9050(param_2,&local_d0);
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f557;
    FUN_142cb8370(DAT_143aa84a0,&local_d0);
    if (local_d0 != 0) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f56a;
      FUN_14019f2c0(local_d0 + -0x10);
    }
    if (local_c8 != 0) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f57d;
      FUN_14019f2c0(local_c8 + -0x10);
    }
    if (local_c0 != 0) {
      *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f590;
      FUN_14019f2c0(local_c0 + -0x10);
    }
  }
  if (ppiVar19 != (int **)0x0) {
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f59f;
    FUN_1401bebb0(ppiVar19 + -2);
  }
  if (local_e0 != 0) {
    *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f5b2;
    FUN_14019f2c0(local_e0 - 0x10);
  }
  *(undefined8 *)((longlong)auStack_190 + lVar5) = 0x141b2f5c2;
  return;
}


