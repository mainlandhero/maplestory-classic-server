
//===========================================================
// FUN_1418486b0 @ 1418486b0   (7472 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Removing unreachable block (ram,0x000141848b7f) */
/* WARNING: Removing unreachable block (ram,0x000141849372) */
/* WARNING: Removing unreachable block (ram,0x000141848f4a) */

void FUN_1418486b0(longlong *param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong ****pppplVar2;
  undefined2 *puVar3;
  bool bVar4;
  char ****ppppcVar5;
  longlong ***ppplVar6;
  byte bVar7;
  char cVar8;
  char cVar9;
  undefined4 uVar10;
  int iVar11;
  longlong lVar12;
  int *piVar13;
  undefined8 uVar14;
  undefined8 *puVar15;
  undefined8 uVar16;
  longlong *plVar17;
  longlong *plVar18;
  longlong lVar19;
  ulonglong uVar20;
  char *pcVar21;
  longlong ****pppplVar22;
  int iVar23;
  int iVar24;
  undefined1 *puVar25;
  char ****ppppcVar26;
  char ****ppppcVar27;
  longlong ****pppplVar28;
  int *piVar29;
  longlong ****pppplVar30;
  longlong lVar31;
  longlong ***ppplVar32;
  longlong ***ppplVar33;
  char ****ppppcVar34;
  undefined8 uStack_680;
  undefined1 auStack_678 [32];
  char ****local_658;
  undefined8 local_650;
  undefined1 local_648 [8];
  longlong ****local_640;
  char ****local_638;
  longlong ****local_630;
  char ****local_628;
  undefined4 local_620 [2];
  longlong ***local_618;
  longlong ***local_610;
  longlong ***local_608;
  longlong ***local_600;
  char *local_5f8;
  longlong ***local_5f0;
  longlong ***local_5e8;
  longlong ***local_5e0;
  longlong ***local_5d8;
  longlong ****local_5d0;
  int *local_5c8;
  char ****local_5c0;
  longlong ***local_5b8;
  longlong *plStack_5b0;
  char ****local_5a8;
  char ***local_5a0;
  longlong ***local_598;
  longlong ***local_590;
  char ***local_588 [2];
  undefined4 local_578;
  undefined4 uStack_574;
  undefined4 uStack_570;
  undefined4 uStack_56c;
  undefined4 local_568;
  longlong ***local_560;
  char ****local_558;
  longlong local_550;
  longlong *local_548;
  longlong local_540;
  longlong local_538;
  char ****local_530;
  longlong *local_528;
  longlong *plStack_520;
  longlong local_518;
  longlong ****local_510;
  longlong ***local_508;
  longlong *local_500;
  undefined8 local_4f8;
  undefined8 uStack_4f0;
  longlong ***local_4e8;
  longlong ***local_4e0;
  longlong local_4d8 [2];
  char ***local_4c8;
  undefined8 uStack_4c0;
  longlong local_4b8;
  longlong local_4b0;
  undefined1 local_4a8 [16];
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  ppppcVar5 = DAT_143aa84a0;
  puVar25 = auStack_678;
  local_48 = DAT_143a8b908 ^ (ulonglong)&local_618;
  ppppcVar34 = (char ****)0x0;
  local_568 = 0;
  local_560 = (longlong ***)DAT_143aa84a0;
  uStack_680 = 0x14184870d;
  local_548 = param_1;
  bVar7 = FUN_1406e8ae0(param_2);
  switch(bVar7) {
  case 9:
  case 0x48:
    uStack_680 = 0x141849833;
    FUN_1406e9050(param_2,&local_600);
    uStack_680 = 0x14184983c;
    cVar8 = FUN_1406e8ae0(param_2);
    uStack_680 = 0x141849848;
    uVar10 = FUN_1406e8c20(param_2);
    ppplVar32 = local_600;
    ppppcVar26 = (char ****)0x0;
    local_5f0 = (longlong ***)0x0;
    local_5f8 = (char *)0x0;
    local_5e0 = (longlong ***)0x0;
    if ((bVar7 & 1) == 0) {
      puVar25 = auStack_678;
      ppppcVar27 = ppppcVar34;
      if ((bVar7 & 0x40) != 0) {
        local_610 = (longlong ***)0x0;
        ppplVar32 = (longlong ***)0x0;
        local_588[0] = (char ***)0x0;
        if (cVar8 == '\x02') {
          uStack_680 = 0x141849f14;
          puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0xde2);
          ppplVar33 = local_600;
          uVar16 = *puVar15;
          uStack_680 = 0x141849f2a;
          puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x3eb);
          uStack_680 = 0x141849f3d;
          FUN_14019ba10(&local_5f8,*puVar15,ppplVar33,uVar16);
          if ((char ****)local_618 != (char ****)0x0) {
            uStack_680 = 0x141849f50;
            FUN_14019f2c0(local_618 + -2);
          }
LAB_14184a090:
          ppppcVar27 = ppppcVar34;
          ppplVar33 = ppplVar32;
          ppppcVar26 = ppppcVar34;
          if ((char ****)local_608 != (char ****)0x0) {
            uStack_680 = 0x14184a0a6;
            FUN_14019f2c0(local_608 + -2);
          }
        }
        else {
          if (cVar8 == '\x05') {
            uStack_680 = 0x141849f6a;
            puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0xde3);
            ppplVar33 = local_600;
            uVar16 = *puVar15;
            uStack_680 = 0x141849f80;
            puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x3eb);
            uStack_680 = 0x141849f93;
            FUN_14019ba10(&local_5f8,*puVar15,ppplVar33,uVar16);
            if ((char ****)local_618 != (char ****)0x0) {
              uStack_680 = 0x141849fa6;
              FUN_14019f2c0(local_618 + -2);
            }
            goto LAB_14184a090;
          }
          ppppcVar27 = ppppcVar26;
          if (cVar8 == '\x03') {
            uStack_680 = 0x141849fc1;
            FUN_142cb92f0(ppppcVar5,&local_618,uVar10);
            ppplVar6 = local_600;
            ppplVar33 = local_618;
            if (((char ****)local_618 != (char ****)0x0) && (*(char *)local_618 != '\0')) {
              uStack_680 = 0x141849fe2;
              puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0x3eb);
              uStack_680 = 0x141849ff5;
              FUN_14019ba10(&local_5f8,*puVar15,ppplVar6,ppplVar33);
              if ((char ****)local_608 != (char ****)0x0) {
                uStack_680 = 0x14184a008;
                FUN_14019f2c0(local_608 + -2);
              }
            }
            ppplVar33 = ppplVar32;
            ppppcVar26 = ppppcVar34;
            if ((char ****)local_618 != (char ****)0x0) {
              uStack_680 = 0x14184a01f;
              FUN_14019f2c0(local_618 + -2);
              ppplVar33 = (longlong ***)0x0;
              ppppcVar26 = (char ****)0x0;
            }
          }
          else {
            ppplVar33 = (longlong ***)0x0;
            ppppcVar26 = (char ****)0x0;
            if (cVar8 == '\x01') {
              uStack_680 = 0x14184a035;
              cVar9 = FUN_14031e440(uVar10);
              if (cVar9 != '\0') {
                uStack_680 = 0x14184a041;
                iVar11 = FUN_142cb8a50(ppppcVar5);
                if (iVar11 == 0) {
                  uStack_680 = 0x14184a053;
                  puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0xde4);
                  ppplVar33 = local_600;
                  uVar16 = *puVar15;
                  uStack_680 = 0x14184a069;
                  puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x3eb);
                  uStack_680 = 0x14184a07c;
                  FUN_14019ba10(&local_5f8,*puVar15,ppplVar33,uVar16);
                  if ((char ****)local_618 != (char ****)0x0) {
                    uStack_680 = 0x14184a08f;
                    FUN_14019f2c0(local_618 + -2);
                  }
                  goto LAB_14184a090;
                }
              }
              uStack_680 = 0x14184a0b6;
              FUN_141815360(&local_5b8,uVar10);
              plVar17 = plStack_5b0;
              ppppcVar26 = ppppcVar34;
              if (plStack_5b0 != (longlong *)0x0) {
                uStack_680 = 0x14184a0df;
                puVar15 = (undefined8 *)
                          FUN_1403999e0(DAT_143aa8328,&local_618,uVar10,PTR_s_streetName_143a44e30);
                ppppcVar26 = (char ****)*puVar15;
                *puVar15 = 0;
                local_610 = (longlong ***)ppppcVar26;
                if ((char ****)local_618 != (char ****)0x0) {
                  uStack_680 = 0x14184a0fe;
                  FUN_14019f2c0(local_618 + -2);
                }
                uStack_680 = 0x14184a119;
                puVar15 = (undefined8 *)
                          FUN_1403999e0(DAT_143aa8328,&local_618,uVar10,PTR_s_mapName_143a49020);
                ppplVar32 = (longlong ***)*puVar15;
                *puVar15 = 0;
                local_588[0] = (char ***)ppplVar32;
                if ((char ****)local_618 != (char ****)0x0) {
                  uStack_680 = 0x14184a139;
                  FUN_14019f2c0(local_618 + -2);
                }
                if ((ppplVar32 == (longlong ***)0x0) ||
                   (pppplVar22 = (longlong ****)local_588, *(char *)ppplVar32 == '\0')) {
                  pppplVar22 = &local_610;
                }
                uStack_680 = 0x14184a159;
                FUN_14019a260(&local_5f0,pppplVar22);
                ppplVar33 = local_5f0;
                if (((char ****)local_5f0 == (char ****)0x0) ||
                   (ppppcVar34 = (char ****)local_5f0, *(char *)local_5f0 == '\0')) {
                  uStack_680 = 0x14184a175;
                  puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x13d5);
                  if ((char ****)ppplVar33 != (char ****)0x0) {
                    uStack_680 = 0x14184a186;
                    FUN_14019f2c0(ppplVar33 + -2);
                  }
                  ppppcVar34 = (char ****)*puVar15;
                  *puVar15 = 0;
                  local_5f0 = (longlong ***)ppppcVar34;
                  if ((char ****)local_618 != (char ****)0x0) {
                    uStack_680 = 0x14184a1a7;
                    FUN_14019f2c0(local_618 + -2);
                  }
                }
                ppplVar33 = local_600;
                uStack_680 = 0x14184a1ba;
                puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x3eb);
                uStack_680 = 0x14184a1cd;
                FUN_14019ba10(&local_5f8,*puVar15,ppplVar33,ppppcVar34);
                if ((char ****)local_618 != (char ****)0x0) {
                  uStack_680 = 0x14184a1e0;
                  FUN_14019f2c0(local_618 + -2);
                }
                uStack_680 = 0x14184a1eb;
                (**(code **)(*plVar17 + 0x10))(plVar17);
              }
              ppppcVar27 = ppppcVar34;
              ppplVar33 = ppplVar32;
              if ((char ****)local_5b8 != (char ****)0x0) {
                uStack_680 = 0x14184a1fb;
                (*(code *)(*local_5b8)[2])();
              }
            }
          }
        }
        local_5d0 = &local_5e8;
        local_5e8 = (longlong ***)0x0;
        uStack_680 = 0x14184a21c;
        FUN_14019a260(&local_5e8,local_588);
        local_558 = (char ****)&local_608;
        local_608 = (longlong ***)0x0;
        uStack_680 = 0x14184a23d;
        FUN_14019a260(&local_608,&local_610);
        local_618 = (longlong ***)0x0;
        uStack_680 = 0x14184a253;
        FUN_14019a260(&local_618,&local_5f8);
        uStack_680 = 0x14184a268;
        FUN_1411bb620(&local_618,&local_608,&local_5e8,cVar8);
        if (ppplVar33 != (longlong ***)0x0) {
          uStack_680 = 0x14184a277;
          FUN_14019f2c0(ppplVar33 + -2);
        }
        puVar25 = auStack_678;
        if (ppppcVar26 != (char ****)0x0) {
          uStack_680 = 0x14184a286;
          FUN_14019f2c0(ppppcVar26 + -2);
          puVar25 = auStack_678;
        }
      }
    }
    else {
      if (cVar8 == '\x02') {
        uStack_680 = 0x141849881;
        puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x89);
        uStack_680 = 0x141849891;
        FUN_14019ba10(&local_5f8,*puVar15,ppplVar32);
LAB_141849c7f:
        if ((char ****)local_618 != (char ****)0x0) {
          uStack_680 = 0x141849c91;
          FUN_14019f2c0(local_618 + -2);
        }
      }
      else {
        if (cVar8 == '\x05') {
          uStack_680 = 0x1418498af;
          puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x8a);
          uStack_680 = 0x1418498bf;
          FUN_14019ba10(&local_5f8,*puVar15,ppplVar32);
          goto LAB_141849c7f;
        }
        if (cVar8 == '\x03') {
          uStack_680 = 0x1418498de;
          FUN_142cb92f0(ppppcVar5,&local_618,uVar10);
          ppplVar33 = local_600;
          ppplVar32 = local_618;
          if (((char ****)local_618 == (char ****)0x0) || (*(char *)local_618 == '\0')) {
            uStack_680 = 0x141849927;
            puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0x87);
            uStack_680 = 0x141849937;
            FUN_14019ba10(&local_5f8,*puVar15,ppplVar33);
          }
          else {
            uStack_680 = 0x1418498ff;
            puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0x88);
            uStack_680 = 0x141849912;
            FUN_14019ba10(&local_5f8,*puVar15,ppplVar33,ppplVar32);
          }
          if ((char ****)local_608 != (char ****)0x0) {
            uStack_680 = 0x14184994a;
            FUN_14019f2c0(local_608 + -2);
          }
          goto LAB_141849c7f;
        }
        if (cVar8 == '\x04') {
          uStack_680 = 0x141849968;
          puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x87);
          uStack_680 = 0x141849978;
          FUN_14019ba10(&local_5f8,*puVar15,ppplVar32);
          goto LAB_141849c7f;
        }
        if (cVar8 != '\x01') {
          uStack_680 = 0x141849c6e;
          puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x87);
          uStack_680 = 0x141849c7e;
          FUN_14019ba10(&local_5f8,*puVar15,ppplVar32);
          goto LAB_141849c7f;
        }
        uStack_680 = 0x14184998e;
        cVar8 = FUN_14031e440(uVar10);
        if (cVar8 == '\0') {
LAB_1418499dc:
          uStack_680 = 0x1418499e7;
          FUN_141815360(&local_5b8,uVar10);
          plVar17 = plStack_5b0;
          if (plStack_5b0 != (longlong *)0x0) {
            uStack_680 = 0x141849a10;
            FUN_1403999e0(DAT_143aa8328,&local_610,uVar10,PTR_s_streetName_143a44e30);
            uStack_680 = 0x141849a2b;
            FUN_1403999e0(DAT_143aa8328,&local_608,uVar10,PTR_s_mapName_143a49020);
            if (((char ****)local_608 == (char ****)0x0) || (*(char *)local_608 == '\0')) {
              uStack_680 = 0x141849a5e;
              FUN_14019a260(&local_5f0,&local_610);
            }
            else {
              uStack_680 = 0x141849a4f;
              FUN_14019ba10(&local_5f0,PTR_s__s____s_143a44e50,local_610);
            }
            ppplVar32 = local_5f0;
            if (((char ****)local_5f0 == (char ****)0x0) ||
               (ppppcVar34 = (char ****)local_5f0, *(char *)local_5f0 == '\0')) {
              uStack_680 = 0x141849a7a;
              puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x13d5);
              if ((char ****)ppplVar32 != (char ****)0x0) {
                uStack_680 = 0x141849a8b;
                FUN_14019f2c0(ppplVar32 + -2);
              }
              ppppcVar34 = (char ****)*puVar15;
              *puVar15 = 0;
              local_5f0 = (longlong ***)ppppcVar34;
              if ((char ****)local_618 != (char ****)0x0) {
                uStack_680 = 0x141849aad;
                FUN_14019f2c0(local_618 + -2);
              }
            }
            ppplVar32 = local_600;
            uStack_680 = 0x141849ac0;
            puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x8b);
            uStack_680 = 0x141849ad3;
            FUN_14019ba10(&local_5f8,*puVar15,ppplVar32,ppppcVar34);
            if ((char ****)local_618 != (char ****)0x0) {
              uStack_680 = 0x141849ae6;
              FUN_14019f2c0(local_618 + -2);
            }
            uStack_680 = 0x141849af3;
            iVar11 = FUN_142cb8a50(local_560);
            if (iVar11 != 0) {
              local_618 = (longlong ***)0x0;
              uStack_680 = 0x141849b10;
              puVar15 = (undefined8 *)FUN_1408a9e40(local_588,0x61b);
              uStack_680 = 0x141849b20;
              plVar18 = (longlong *)FUN_14019ba10(&local_618,*puVar15,uVar10);
              lVar12 = *plVar18;
              if (lVar12 != 0) {
                iVar11 = *(int *)(lVar12 + -8);
                lVar31 = (longlong)iVar11;
                if (iVar11 != 0) {
                  if ((local_5f8 == (char *)0x0) || (*local_5f8 == '\0')) {
                    uStack_680 = 0x141849b9a;
                    uVar16 = FUN_14019bd40(&local_5f8,iVar11,0);
                    uStack_680 = 0x141849ba8;
                    FUN_142ef7ba0(uVar16,lVar12,lVar31);
                  }
                  else {
                    iVar11 = *(int *)(local_5f8 + -8) + iVar11;
                    for (iVar23 = *(int *)(local_5f8 + -0xc); iVar23 < iVar11; iVar23 = iVar23 * 2)
                    {
                    }
                    uStack_680 = 0x141849b66;
                    lVar19 = FUN_14019bd40(&local_5f8,iVar23,1);
                    if (local_5f8 == (char *)0x0) {
                      iVar23 = 0;
                    }
                    else {
                      iVar23 = *(int *)(local_5f8 + -8);
                    }
                    uStack_680 = 0x141849b87;
                    FUN_142ef7ba0(iVar23 + lVar19,lVar12,lVar31);
                  }
                  uStack_680 = 0x141849bb4;
                  FUN_14019c870(&local_5f8,iVar11);
                }
              }
              if ((longlong ***)local_588[0] != (longlong ***)0x0) {
                uStack_680 = 0x141849bca;
                FUN_14019f2c0(local_588[0] + -2);
              }
              if ((char ****)local_618 != (char ****)0x0) {
                uStack_680 = 0x141849bdd;
                FUN_14019f2c0(local_618 + -2);
              }
            }
            if (DAT_143aceed4 != 0) {
              uStack_680 = 0x141849bf5;
              puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x61c);
              uStack_680 = 0x141849c05;
              FUN_14019ba10(&local_5e0,*puVar15,uVar10);
              if ((char ****)local_618 != (char ****)0x0) {
                uStack_680 = 0x141849c18;
                FUN_14019f2c0(local_618 + -2);
              }
            }
            if ((char ****)local_608 != (char ****)0x0) {
              uStack_680 = 0x141849c2b;
              FUN_14019f2c0(local_608 + -2);
            }
            if ((char ****)local_610 != (char ****)0x0) {
              uStack_680 = 0x141849c3e;
              FUN_14019f2c0(local_610 + -2);
            }
            uStack_680 = 0x141849c49;
            (**(code **)(*plVar17 + 0x10))(plVar17);
          }
          if ((char ****)local_5b8 != (char ****)0x0) {
            uStack_680 = 0x141849c59;
            (*(code *)(*local_5b8)[2])();
          }
        }
        else {
          uStack_680 = 0x14184999a;
          iVar11 = FUN_142cb8a50(ppppcVar5);
          ppplVar32 = local_600;
          if (iVar11 != 0) goto LAB_1418499dc;
          uStack_680 = 0x1418499b0;
          puVar15 = (undefined8 *)FUN_1408a9e40(&local_618,0x8c);
          uStack_680 = 0x1418499c0;
          FUN_14019ba10(&local_5f8,*puVar15,ppplVar32);
          if ((char ****)local_618 != (char ****)0x0) {
            uStack_680 = 0x1418499d7;
            FUN_14019f2c0(local_618 + -2);
          }
        }
      }
      if ((char ****)local_600 == (char ****)0x0) {
        iVar11 = 2;
      }
      else {
        local_650 = (longlong ****)((ulonglong)local_650 & 0xffffffff00000000);
        local_658 = (char ****)0x0;
        uStack_680 = 0x141849cc2;
        iVar11 = (*DAT_1432627f8)(0xfde9,0,local_600,0xffffffff);
        iVar11 = iVar11 * 2;
      }
      ppplVar32 = local_600;
      uVar20 = (longlong)iVar11 + 0xf;
      if (uVar20 <= (ulonglong)(longlong)iVar11) {
        uVar20 = 0xffffffffffffff0;
      }
      uStack_680 = 0x141849ce9;
      lVar12 = -(uVar20 & 0xfffffffffffffff0);
      puVar3 = (undefined2 *)((longlong)&local_618 + lVar12);
      if ((char ****)local_600 == (char ****)0x0) {
        if (puVar3 != (undefined2 *)0x0) {
          *puVar3 = 0;
        }
      }
      else {
        *(undefined4 *)(local_648 + lVar12 + -8) = 0x100000;
        *(undefined2 **)((longlong)&local_658 + lVar12) = puVar3;
        *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849d1f;
        (*DAT_1432627f8)(0xfde9,0,ppplVar32,0xffffffff);
      }
      *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849d2e;
      FUN_1403edf80(&local_608,puVar3,0xffffffff);
      local_5d0 = (longlong ****)&local_4f8;
      local_4f8 = 0;
      uStack_4f0 = 0;
      local_558 = local_588;
      local_588[0] = (char ***)0x0;
      *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849d6a;
      FUN_1401c1fb0(local_588,&local_608);
      local_530 = (char ****)&local_5e8;
      local_5e8 = (longlong ***)0x0;
      *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849d88;
      FUN_14019bd40(&local_5e8,0,0);
      ppplVar32 = local_5e8;
      if (*(int *)(local_5e8 + -2) != -1) {
        *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849d9e;
        FUN_142e52dd0(0x8b);
      }
      iVar11 = *(int *)((longlong)ppplVar32 + -0xc);
      if (iVar11 < 0) {
        *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849db2;
        FUN_142e54290(0x90,iVar11,0);
      }
      *(undefined4 *)(ppplVar32 + -2) = 1;
      *(undefined1 *)local_5e8 = 0;
      if (*(int *)((longlong)ppplVar32 + -0xc) + 1 < 1) {
        *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849dd5;
        FUN_142e54290(0x9c,0);
      }
      *(undefined4 *)(ppplVar32 + -1) = 0;
      local_510 = &local_610;
      local_610 = (longlong ***)0x0;
      *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849df5;
      FUN_14019bd40(&local_610,0,0);
      ppplVar32 = local_610;
      if (*(int *)(local_610 + -2) != -1) {
        *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849e0b;
        FUN_142e52dd0(0x8b);
      }
      iVar11 = *(int *)((longlong)ppplVar32 + -0xc);
      if (iVar11 < 0) {
        *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849e1f;
        FUN_142e54290(0x90,iVar11,0);
      }
      *(undefined4 *)(ppplVar32 + -2) = 1;
      *(char *)local_610 = '\0';
      if (*(int *)((longlong)ppplVar32 + -0xc) + 1 < 1) {
        *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849e42;
        FUN_142e54290(0x9c,0);
      }
      *(undefined4 *)(ppplVar32 + -1) = 0;
      local_618 = (longlong ***)0x0;
      local_500 = (longlong *)0x0;
      bVar4 = true;
      if ((ppppcVar34 != (char ****)0x0) && (*(char *)ppppcVar34 != '\0')) {
        bVar4 = false;
      }
      uVar16 = 6;
      if (bVar4) {
        uVar16 = 0xb;
      }
      local_5d0 = &local_618;
      *(undefined4 *)((longlong)local_620 + lVar12) = 0;
      *(undefined8 **)((longlong)local_620 + lVar12 + -8) = &local_4f8;
      *(char *****)((longlong)&local_630 + lVar12) = local_588;
      *(longlong *****)((longlong)&local_638 + lVar12) = &local_5e8;
      *(longlong *****)((longlong)&local_640 + lVar12) = &local_610;
      *(longlong *****)(local_648 + lVar12) = &local_618;
      local_648[lVar12 + -8] = 0xff;
      *(longlong *****)((longlong)&local_658 + lVar12) = &local_508;
      pcVar21 = local_5f8;
      *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849ecc;
      FUN_1415a87a0(pcVar21,uVar16,0xffffffff,0);
      puVar25 = auStack_678 + lVar12;
      ppppcVar27 = ppppcVar34;
      if ((char ****)local_608 != (char ****)0x0) {
        ppppcVar34 = (char ****)(local_608 + -2);
        *(undefined8 *)(auStack_678 + lVar12 + -8) = 0x141849ee3;
        FUN_1401bebb0(ppppcVar34);
        puVar25 = auStack_678 + lVar12;
      }
    }
    plVar17 = local_548;
    *(undefined8 *)(puVar25 + -8) = 0x14184a29b;
    FUN_141845de0(plVar17,0);
    DAT_143aceed4 = 0;
    if (((char ****)local_5e0 != (char ****)0x0) && (*(char *)local_5e0 != '\0')) {
      *(undefined8 *)(puVar25 + -8) = 0x14184a2bc;
      FUN_1418cd030(plVar17,&local_5e0);
    }
    if ((char ****)local_5e0 != (char ****)0x0) {
      ppppcVar34 = (char ****)(local_5e0 + -2);
      *(undefined8 *)(puVar25 + -8) = 0x14184a2ce;
      FUN_14019f2c0(ppppcVar34);
    }
    if (local_5f8 != (char *)0x0) {
      pcVar21 = local_5f8 + -0x10;
      *(undefined8 *)(puVar25 + -8) = 0x14184a2e1;
      FUN_14019f2c0(pcVar21);
    }
    local_598 = local_600;
    if (ppppcVar27 != (char ****)0x0) {
      *(undefined8 *)(puVar25 + -8) = 0x14184a2f0;
      FUN_14019f2c0(ppppcVar27 + -2);
      local_598 = local_600;
    }
    break;
  case 10:
  case 0x8a:
    uStack_680 = 0x1418492a6;
    cVar8 = FUN_1406e8ae0(param_2);
    uStack_680 = 0x1418492b8;
    FUN_1406e9050(param_2,&local_610);
    uStack_680 = 0x1418492c1;
    bVar7 = FUN_1406e8ae0(param_2);
    local_5f8 = (char *)CONCAT44(local_5f8._4_4_,(uint)bVar7);
    local_5e8 = (longlong ***)0x0;
    if ((*(char **)(param_1[0x15] + 0x420) != (char *)0x0) &&
       (**(char **)(param_1[0x15] + 0x420) != '\0')) {
      local_600 = (longlong ***)0x0;
      uStack_680 = 0x1418492fb;
      FUN_14019a260(&local_600);
      local_658 = (char ****)0x0;
      uStack_680 = 0x14184931d;
      FUN_1408bee80(DAT_143ac2f58,&local_600,0,1);
      if (*(int *)(param_1[0x15] + 0x430) == 1) {
        if ((char ****)local_600 != (char ****)0x0) {
          ppppcVar34 = (char ****)(ulonglong)*(uint *)(local_600 + -1);
        }
        uStack_680 = 0x14184934f;
        FUN_1401abc80(param_1[0x15] + 0x428,&local_608,local_600,ppppcVar34);
        if ((char ****)local_600 != (char ****)0x0) {
          uStack_680 = 0x141849362;
          FUN_14019f2c0(local_600 + -2);
        }
        local_600 = local_608;
      }
      ppplVar33 = local_600;
      ppplVar32 = local_610;
      uStack_680 = 0x141849395;
      puVar15 = (undefined8 *)FUN_1408a9e40(local_588,0x61a);
      uStack_680 = 0x1418493a8;
      FUN_14019ba10(&local_5e8,*puVar15,ppplVar32,ppplVar33);
      if ((longlong ***)local_588[0] != (longlong ***)0x0) {
        uStack_680 = 0x1418493be;
        FUN_14019f2c0(local_588[0] + -2);
      }
      local_5d0 = &local_5f0;
      local_5f0 = (longlong ***)0x0;
      uStack_680 = 0x1418493d9;
      FUN_14019bd40(&local_5f0,0,0);
      ppplVar32 = local_5f0;
      if (*(int *)(local_5f0 + -2) != -1) {
        uStack_680 = 0x1418493ef;
        FUN_142e52dd0(0x8b);
      }
      if (*(int *)((longlong)ppplVar32 + -0xc) < 0) {
        uStack_680 = 0x141849403;
        FUN_142e54290(0x90,*(int *)((longlong)ppplVar32 + -0xc),0);
      }
      *(undefined4 *)(ppplVar32 + -2) = 1;
      *(undefined1 *)local_5f0 = 0;
      if (*(int *)((longlong)ppplVar32 + -0xc) + 1 < 1) {
        uStack_680 = 0x141849426;
        FUN_142e54290(0x9c,0);
      }
      *(undefined4 *)(ppplVar32 + -1) = 0;
      lVar12 = param_1[0x15];
      uStack_680 = 0x14184943d;
      uVar16 = FUN_142cb9610(local_560);
      uStack_680 = 0x141849453;
      FUN_140196ed0(&local_618,uVar16,0xffffffff);
      local_650 = (longlong ****)((ulonglong)local_650._4_4_ << 0x20);
      local_658 = (char ****)&local_5f0;
      uStack_680 = 0x141849482;
      FUN_1408d6900(&local_4f8,&local_618,lVar12 + 0x420,0x1183);
      if ((char ****)local_618 != (char ****)0x0) {
        uStack_680 = 0x141849495;
        FUN_14019f2c0(local_618 + -2);
      }
      lVar12 = param_1[0x15];
      uStack_680 = 0x1418494ae;
      plVar17 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x30);
      if (plVar17 == (longlong *)0x0) {
        plVar17 = (longlong *)0x0;
      }
      else {
        *plVar17 = 0;
        plVar17[1] = 0;
        *(undefined4 *)(plVar17 + 1) = 1;
        *(undefined4 *)((longlong)plVar17 + 0xc) = 1;
        *plVar17 = (longlong)&PTR_FUN_14338bf98;
        local_558 = (char ****)(plVar17 + 2);
        *(undefined4 *)local_558 = *(undefined4 *)(lVar12 + 0x430);
        uStack_680 = 0x1418494f5;
        local_5d0 = (longlong ****)plVar17;
        FUN_140232590(plVar17 + 3,lVar12 + 0x438);
        plVar17[5] = 0;
        uStack_680 = 0x14184950d;
        FUN_14019a260(plVar17 + 5,lVar12 + 0x448);
      }
      local_5b8 = (longlong ***)(plVar17 + 2);
      local_5d0 = &local_5b8;
      if (plVar17 != (longlong *)0x0) {
        LOCK();
        *(int *)(plVar17 + 1) = (int)plVar17[1] + 1;
        UNLOCK();
      }
      local_558 = (char ****)&local_5e0;
      local_5e0 = (longlong ***)0x0;
      uStack_680 = 0x141849576;
      plStack_5b0 = plVar17;
      local_508 = local_5b8;
      local_500 = plVar17;
      FUN_14019a260(&local_5e0,param_1[0x15] + 0x448);
      local_530 = (char ****)&local_5f0;
      local_5f0 = (longlong ***)0x0;
      uStack_680 = 0x141849594;
      FUN_14019bd40(&local_5f0,0,0);
      ppplVar32 = local_5f0;
      if (*(int *)(local_5f0 + -2) != -1) {
        uStack_680 = 0x1418495aa;
        FUN_142e52dd0(0x8b);
      }
      if (*(int *)((longlong)ppplVar32 + -0xc) < 0) {
        uStack_680 = 0x1418495be;
        FUN_142e54290(0x90,*(int *)((longlong)ppplVar32 + -0xc),0);
      }
      *(undefined4 *)(ppplVar32 + -2) = 1;
      *(char *)local_5f0 = '\0';
      if (*(int *)((longlong)ppplVar32 + -0xc) + 1 < 1) {
        uStack_680 = 0x1418495e1;
        FUN_142e54290(0x9c,0);
      }
      *(undefined4 *)(ppplVar32 + -1) = 0;
      local_618 = (longlong ***)0x0;
      uStack_680 = 0x14184960d;
      local_510 = &local_618;
      local_650 = (longlong ****)FUN_140232590(local_588,param_1[0x15] + 0x438);
      local_620[0] = 0;
      local_628 = (char ****)&local_5b8;
      local_630 = &local_5e0;
      local_638 = (char ****)&local_5f0;
      local_648[0] = 0xff;
      local_658 = (char ****)((ulonglong)local_658 & 0xffffffff00000000);
      uStack_680 = 0x14184965a;
      local_640 = &local_618;
      FUN_1415a8b80(&local_4f8,local_5e8,1,0xffffffff);
      uStack_680 = 0x14184966d;
      FUN_1408da210(param_1[0x15] + 0x430);
      local_618 = (longlong ***)0x0;
      uStack_680 = 0x14184967f;
      FUN_14019bd40(&local_618,0,0);
      ppplVar32 = local_618;
      if (*(int *)(local_618 + -2) != -1) {
        uStack_680 = 0x141849695;
        FUN_142e52dd0(0x8b);
      }
      if (*(int *)((longlong)ppplVar32 + -0xc) < 0) {
        uStack_680 = 0x1418496a9;
        FUN_142e54290(0x90,*(int *)((longlong)ppplVar32 + -0xc),0);
      }
      *(undefined4 *)(ppplVar32 + -2) = 1;
      *(undefined1 *)ppplVar32 = 0;
      if (*(int *)((longlong)ppplVar32 + -0xc) + 1 < 1) {
        uStack_680 = 0x1418496c8;
        FUN_142e54290(0x9c,0);
      }
      *(undefined4 *)(ppplVar32 + -1) = 0;
      lVar12 = param_1[0x15];
      lVar31 = *(longlong *)(lVar12 + 0x420);
      if (lVar31 != 0) {
        uStack_680 = 0x1418496e7;
        FUN_14019f2c0(lVar31 + -0x10);
      }
      *(longlong ****)(lVar12 + 0x420) = ppplVar32;
      local_618 = (longlong ***)0x0;
      uStack_680 = 0x141849702;
      FUN_14019bd40(&local_618,0,0);
      ppplVar32 = local_618;
      if (*(int *)(local_618 + -2) != -1) {
        uStack_680 = 0x141849718;
        FUN_142e52dd0(0x8b);
      }
      if (*(int *)((longlong)ppplVar32 + -0xc) < 0) {
        uStack_680 = 0x14184972c;
        FUN_142e54290(0x90,*(int *)((longlong)ppplVar32 + -0xc),0);
      }
      *(undefined4 *)(ppplVar32 + -2) = 1;
      *(char *)ppplVar32 = '\0';
      if (*(int *)((longlong)ppplVar32 + -0xc) + 1 < 1) {
        uStack_680 = 0x14184974b;
        FUN_142e54290(0x9c,0);
      }
      *(undefined4 *)(ppplVar32 + -1) = 0;
      lVar12 = param_1[0x15];
      lVar31 = *(longlong *)(lVar12 + 0x428);
      if (lVar31 != 0) {
        uStack_680 = 0x14184976a;
        FUN_14019f2c0(lVar31 + -0x10);
      }
      *(longlong ****)(lVar12 + 0x428) = ppplVar32;
      if ((cVar8 == '\0') && (DAT_143acab70 != 0)) {
        uStack_680 = 0x14184978b;
        FUN_1415a7a00(DAT_143acab70,&local_610);
      }
      if (plVar17 != (longlong *)0x0) {
        uStack_680 = 0x141849799;
        FUN_1402abcb0(plVar17);
      }
      uStack_680 = 0x1418497a6;
      FUN_1411b2070(&local_4f8);
      if ((char ****)local_600 != (char ****)0x0) {
        uStack_680 = 0x1418497b9;
        FUN_14019f2c0(local_600 + -2);
      }
    }
    ppplVar32 = local_610;
    if ((int)local_5f8 == 0) {
      uStack_680 = 0x1418497d2;
      puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0x87);
      uStack_680 = 0x1418497e2;
      FUN_14019ba10(&local_5e8,*puVar15,ppplVar32);
      if ((char ****)local_608 != (char ****)0x0) {
        uStack_680 = 0x1418497f5;
        FUN_14019f2c0(local_608 + -2);
      }
      uStack_680 = 0x141849804;
      FUN_1415eca30(&local_5e8,0xb);
    }
    uStack_680 = 0x14184980e;
    FUN_141845de0(param_1,0);
    local_598 = local_610;
    puVar25 = auStack_678;
    if ((char ****)local_5e8 != (char ****)0x0) {
      uStack_680 = 0x141849821;
      FUN_14019f2c0(local_5e8 + -2);
      local_598 = local_610;
      puVar25 = auStack_678;
    }
    break;
  default:
    goto switchD_14184873b_caseD_b;
  case 0x12:
    uStack_680 = 0x141848745;
    uVar10 = FUN_1406e8c20(param_2);
    local_5f8 = (char *)CONCAT44(local_5f8._4_4_,uVar10);
    uStack_680 = 0x141848757;
    FUN_1406e9050(param_2,&local_598);
    uStack_680 = 0x141848760;
    FUN_1406e8c20(param_2);
    uStack_680 = 0x141848768;
    cVar8 = FUN_1406e8ae0(param_2);
    local_5f0 = (longlong ***)CONCAT44(local_5f0._4_4_,(int)cVar8);
    uStack_680 = 0x141848776;
    bVar7 = FUN_1406e8ae0(param_2);
    local_600 = (longlong ***)CONCAT44(local_600._4_4_,(uint)bVar7);
    uStack_680 = 0x141848788;
    FUN_1406e9050(param_2,&local_5d8);
    uVar16 = DAT_143ac2f58;
    uStack_680 = 0x1418487a7;
    uVar14 = FUN_14019bd40(&local_5d8,0x400,1);
    local_658 = (char ****)0x0;
    uStack_680 = 0x1418487bd;
    FUN_1408bed00(uVar16,uVar14,0,1);
    ppplVar32 = local_5d8;
    if (*(int *)(local_5d8 + -2) != -1) {
      uStack_680 = 0x1418487d3;
      FUN_142e52dd0(0x8b);
    }
    *(undefined4 *)(ppplVar32 + -2) = 1;
    if ((char ****)ppplVar32 == (char ****)0x0) {
      uVar20 = 0;
      iVar11 = iRamfffffffffffffff4;
LAB_141848807:
      iVar23 = (int)uVar20;
      if (iVar11 + 1 <= iVar23) goto LAB_14184880b;
    }
    else {
      uVar20 = 0xffffffffffffffff;
      do {
        uVar20 = uVar20 + 1;
      } while (*(char *)((longlong)ppplVar32 + uVar20) != '\0');
      iVar11 = *(int *)((longlong)ppplVar32 - 0xc);
      if (-1 < (int)uVar20) goto LAB_141848807;
LAB_14184880b:
      iVar23 = (int)uVar20;
      uStack_680 = 0x141848817;
      FUN_142e54290(0x9c,uVar20 & 0xffffffff);
    }
    *(int *)(ppplVar32 + -1) = iVar23;
    local_528 = (longlong *)0x0;
    plStack_520 = (longlong *)0x0;
    uStack_680 = 0x141848836;
    plVar17 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x58);
    local_548 = plVar17;
    if (plVar17 == (longlong *)0x0) {
      plVar17 = (longlong *)0x0;
    }
    else {
      *plVar17 = 0;
      plVar17[1] = 0;
      *(undefined4 *)(plVar17 + 1) = 1;
      *(undefined4 *)((longlong)plVar17 + 0xc) = 1;
      *plVar17 = (longlong)&PTR_FUN_143300d78;
      uStack_680 = 0x141848866;
      FUN_1408d6690(plVar17 + 2);
    }
    plVar18 = plStack_520;
    local_568 = 1;
    local_528 = plVar17 + 2;
    if (plStack_520 != (longlong *)0x0) {
      LOCK();
      plVar1 = plStack_520 + 1;
      lVar12 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -1;
      UNLOCK();
      if ((int)lVar12 == 1) {
        puVar15 = (undefined8 *)*plStack_520;
        uStack_680 = 0x1418488a4;
        plStack_520 = plVar17;
        (*(code *)*puVar15)(plVar18);
        LOCK();
        piVar29 = (int *)((longlong)plVar18 + 0xc);
        iVar11 = *piVar29;
        *piVar29 = *piVar29 + -1;
        UNLOCK();
        plVar17 = plStack_520;
        if (iVar11 == 1) {
          uStack_680 = 0x1418488ba;
          (**(code **)(*plVar18 + 8))(plVar18);
          plVar17 = plStack_520;
        }
      }
    }
    plStack_520 = plVar17;
    uStack_680 = 0x1418488c9;
    FUN_1408d6760(local_528,param_2);
    pppplVar22 = (longlong ****)0x0;
    local_5c0 = (char ****)0x0;
    uStack_680 = 0x1418488de;
    FUN_14189e7e0(&local_578);
    piVar29 = (int *)CONCAT44(uStack_574,local_578);
    if (piVar29 == (int *)0x0) {
      if (CONCAT44(uStack_56c,uStack_570) != 0) {
        LOCK();
        piVar29 = (int *)(CONCAT44(uStack_56c,uStack_570) + 8);
        iVar11 = *piVar29;
        *piVar29 = *piVar29 + -1;
        UNLOCK();
        if (iVar11 == 1) {
          puVar15 = (undefined8 *)CONCAT44(uStack_56c,uStack_570);
          uStack_680 = 0x141848917;
          (**(code **)*puVar15)(puVar15);
          LOCK();
          piVar29 = (int *)((longlong)puVar15 + 0xc);
          iVar11 = *piVar29;
          *piVar29 = *piVar29 + -1;
          UNLOCK();
          if (iVar11 == 1) {
            uStack_680 = 0x141848931;
            (**(code **)(*(longlong *)CONCAT44(uStack_56c,uStack_570) + 8))();
          }
        }
      }
      plVar17 = plStack_520;
      if (plStack_520 != (longlong *)0x0) {
        LOCK();
        plVar18 = plStack_520 + 1;
        lVar12 = *plVar18;
        *(int *)plVar18 = (int)*plVar18 + -1;
        UNLOCK();
        if ((int)lVar12 == 1) {
          uStack_680 = 0x141848953;
          (**(code **)*plStack_520)(plStack_520);
          LOCK();
          piVar29 = (int *)((longlong)plVar17 + 0xc);
          iVar11 = *piVar29;
          *piVar29 = *piVar29 + -1;
          UNLOCK();
          if (iVar11 == 1) {
            uStack_680 = 0x141848968;
            (**(code **)(*plVar17 + 8))(plVar17);
          }
        }
      }
      puVar25 = auStack_678;
      if ((char ****)local_5d8 != (char ****)0x0) {
        uStack_680 = 0x14184897b;
        FUN_14019f2c0(local_5d8 + -2);
        puVar25 = auStack_678;
      }
    }
    else {
      uStack_680 = 0x141848993;
      FUN_1408da090(piVar29,param_2);
      uVar16 = DAT_143aa8328;
      pppplVar30 = pppplVar22;
      if ((*piVar29 == 1) && (pppplVar30 = (longlong ****)0x0, *(longlong *)(piVar29 + 4) != 0)) {
        if ((*(char **)(piVar29 + 6) == (char *)0x0) || (**(char **)(piVar29 + 6) == '\0')) {
          uStack_680 = 0x1418489d9;
          lVar12 = FUN_1401a19e0(piVar29 + 2);
          uStack_680 = 0x1418489e2;
          uVar10 = FUN_14019a5d0(lVar12 + 0x20);
          uStack_680 = 0x1418489f4;
          puVar15 = (undefined8 *)FUN_140398ba0(uVar16,&local_4b0,uVar10);
          pppplVar30 = (longlong ****)*puVar15;
          *puVar15 = 0;
          local_5c0 = (char ****)pppplVar30;
          if (local_4b0 != 0) {
            uStack_680 = 0x141848a14;
            FUN_14019f2c0(local_4b0 + -0x10);
          }
        }
        else {
          uStack_680 = 0x1418489c3;
          FUN_14019a260(&local_5c0);
          pppplVar30 = (longlong ****)local_5c0;
        }
        local_5c8 = (int *)0x0;
        uStack_680 = 0x141848a2a;
        piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,0x12);
        piVar13[1] = 1;
        *piVar13 = -1;
        local_5c8 = piVar13 + 4;
        piVar13[2] = 0;
        *(undefined1 *)local_5c8 = 0;
        *(undefined1 *)local_5c8 = DAT_143278810;
        if (*piVar13 != -1) {
          uStack_680 = 0x141848a66;
          FUN_142e52dd0(0x8b);
        }
        if (piVar13[1] < 1) {
          uStack_680 = 0x141848a7b;
          FUN_142e54290(0x90,piVar13[1],1);
        }
        *piVar13 = 1;
        *(undefined1 *)((longlong)local_5c8 + 1) = 0;
        if (piVar13[1] + 1 < 2) {
          uStack_680 = 0x141848aa1;
          FUN_142e54290(0x9c,1);
        }
        piVar13[2] = 1;
        pppplVar28 = pppplVar22;
        if (pppplVar30 != (longlong ****)0x0) {
          pppplVar28 = (longlong ****)(ulonglong)*(uint *)(pppplVar30 + -1);
        }
        uStack_680 = 0x141848abf;
        uVar16 = FUN_14019d8c0(&local_5c8,pppplVar30,pppplVar28);
        local_540 = 0;
        uStack_680 = 0x141848ad5;
        FUN_14019a260(&local_540,uVar16);
        local_568 = 3;
        if (local_5c8 != (int *)0x0) {
          uStack_680 = 0x141848af1;
          FUN_14019f2c0(local_5c8 + -4);
        }
        lVar12 = -1;
        do {
          pcVar21 = &DAT_143275b9d + lVar12;
          lVar12 = lVar12 + 1;
        } while (*pcVar21 != '\0');
        uStack_680 = 0x141848b1d;
        FUN_1401abc80(&local_540,&local_518);
        if (local_540 != 0) {
          uStack_680 = 0x141848b33;
          FUN_14019f2c0(local_540 + -0x10);
        }
        if ((char ****)local_5d8 != (char ****)0x0) {
          pppplVar22 = (longlong ****)(ulonglong)*(uint *)(local_5d8 + -1);
        }
        uStack_680 = 0x141848b59;
        FUN_1401abc80(&local_518,&local_4e0,local_5d8,pppplVar22);
        if ((char ****)local_5d8 != (char ****)0x0) {
          uStack_680 = 0x141848b6c;
          FUN_14019f2c0(local_5d8 + -2);
        }
        local_5d8 = local_4e0;
        if (local_518 != 0) {
          uStack_680 = 0x141848b9e;
          FUN_14019f2c0(local_518 + -0x10);
        }
      }
      local_550 = 0;
      uStack_680 = 0x141848bb2;
      iVar11 = FUN_142cb9260(DAT_143aa84a0);
      ppplVar33 = local_598;
      ppplVar32 = local_5d8;
      if ((int)local_5f0 == iVar11) {
        uStack_680 = 0x141848bd7;
        puVar15 = (undefined8 *)FUN_1408a9e40(local_4d8,0x617);
        uStack_680 = 0x141848bed;
        FUN_14019ba10(&local_550,*puVar15,ppplVar33,ppplVar32);
        if (local_4d8[0] != 0) {
          uStack_680 = 0x141848c07;
          FUN_14019f2c0(local_4d8[0] + -0x10);
        }
      }
      else {
        uStack_680 = 0x141848c20;
        puVar15 = (undefined8 *)FUN_142cb92f0(DAT_143aa84a0,&local_548);
        ppplVar33 = local_598;
        uVar16 = *puVar15;
        uStack_680 = 0x141848c3c;
        puVar15 = (undefined8 *)FUN_1408a9e40(&local_4b8,0x618);
        local_658 = (char ****)ppplVar32;
        uStack_680 = 0x141848c57;
        FUN_14019ba10(&local_550,*puVar15,ppplVar33,uVar16);
        if (local_4b8 != 0) {
          uStack_680 = 0x141848c6d;
          FUN_14019f2c0(local_4b8 + -0x10);
        }
        if (local_548 != (longlong *)0x0) {
          uStack_680 = 0x141848c83;
          FUN_14019f2c0(local_548 + -2);
        }
      }
      if ((int)local_600 == 0) {
        if (*(int *)(DAT_143ac87a0 + 0x13c) != 0) {
          uStack_680 = 0x141848cae;
          iVar11 = FUN_142d01050(DAT_143aa84a0,(ulonglong)local_5f8 & 0xffffffff);
          if (iVar11 == 0) goto LAB_141848d24;
        }
        uStack_680 = 0x141848cc3;
        FUN_1406ed520(local_498,0x17b);
        uStack_680 = 0x141848cd2;
        FUN_1406ed840(local_498,0x22);
        uStack_680 = 0x141848cd7;
        uVar10 = FUN_1429e3ef0();
        uStack_680 = 0x141848ce5;
        FUN_1406ed9d0(local_498,uVar10);
        uStack_680 = 0x141848cf8;
        FUN_1406edc80(local_498,&local_598);
        uStack_680 = 0x141848d04;
        FUN_1415d01c0(local_498);
        uStack_680 = 0x141848d11;
        FUN_1406ed610(local_498);
        lVar12 = CONCAT44(uStack_56c,uStack_570);
        lVar31 = local_550;
      }
      else {
LAB_141848d24:
        uVar16 = DAT_143ac2f58;
        local_538 = 0;
        uStack_680 = 0x141848d43;
        uVar14 = FUN_14019bd40(&local_5d8,0x400,1);
        uStack_680 = 0x141848d55;
        iVar11 = FUN_1408bef90(uVar16,uVar14,&local_538);
        ppplVar32 = local_5d8;
        if (*(int *)(local_5d8 + -2) != -1) {
          uStack_680 = 0x141848d6d;
          FUN_142e52dd0(0x8b);
        }
        *(undefined4 *)(ppplVar32 + -2) = 1;
        if ((char ****)ppplVar32 == (char ****)0x0) {
          uVar20 = 0;
          iVar23 = iRamfffffffffffffff4;
LAB_141848da1:
          iVar24 = (int)uVar20;
          if (iVar23 + 1 <= iVar24) goto LAB_141848da5;
        }
        else {
          uVar20 = 0xffffffffffffffff;
          do {
            uVar20 = uVar20 + 1;
          } while (*(char *)((longlong)ppplVar32 + uVar20) != '\0');
          iVar23 = *(int *)((longlong)ppplVar32 - 0xc);
          if (-1 < (int)uVar20) goto LAB_141848da1;
LAB_141848da5:
          iVar24 = (int)uVar20;
          uStack_680 = 0x141848db1;
          FUN_142e54290(0x9c,uVar20 & 0xffffffff);
        }
        *(int *)(ppplVar32 + -1) = iVar24;
        if (iVar11 != 0) {
          uStack_680 = 0x141848dc9;
          FUN_1415eca30(&local_538,7);
        }
        local_4e8 = (longlong ***)&local_4c8;
        pppplVar22 = pppplVar30;
        if (CONCAT44(uStack_56c,uStack_570) != 0) {
          LOCK();
          piVar29 = (int *)(CONCAT44(uStack_56c,uStack_570) + 8);
          *piVar29 = *piVar29 + 1;
          UNLOCK();
          piVar29 = (int *)CONCAT44(uStack_574,local_578);
          pppplVar22 = (longlong ****)local_5c0;
        }
        uStack_4c0 = CONCAT44(uStack_56c,uStack_570);
        local_510 = (longlong ****)&local_5a8;
        local_5a8 = (char ****)0x0;
        lVar12 = uStack_4c0;
        pppplVar30 = pppplVar22;
        pppplVar28 = (longlong ****)local_5a8;
        if ((pppplVar22 != (longlong ****)0x0) &&
           (pppplVar2 = pppplVar22 + -2, pppplVar2 != (longlong ****)0x0)) {
          if (*(int *)pppplVar2 == -1) {
            uStack_680 = 0x141848e51;
            FUN_142e52d50(0xcb,0xffffff01);
            uVar20 = 0xffffffffffffffff;
            do {
              uVar20 = uVar20 + 1;
            } while (*(char *)((longlong)pppplVar22 + uVar20) != '\0');
            iVar23 = (int)uVar20;
            iVar11 = 0;
            if (0 < iVar23) {
              iVar11 = iVar23;
            }
            uStack_680 = 0x141848e77;
            piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
            piVar13[1] = iVar11;
            *piVar13 = -1;
            pppplVar28 = (longlong ****)(piVar13 + 4);
            piVar13[2] = 0;
            *(undefined1 *)pppplVar28 = 0;
            local_590 = (longlong ***)(longlong)iVar23;
            uStack_680 = 0x141848eaa;
            local_530 = (char ****)pppplVar28;
            FUN_142ef7ba0(pppplVar28,pppplVar22,local_590);
            if (*piVar13 != -1) {
              uStack_680 = 0x141848ebc;
              FUN_142e52dd0(0x8b);
            }
            if ((iVar23 == -1) || (iVar23 <= piVar13[1])) {
              *piVar13 = 1;
              if (iVar23 != -1) goto LAB_141848ed9;
              if (pppplVar28 == (longlong ****)0x0) {
                uVar20 = 0;
              }
              else {
                uVar20 = 0xffffffffffffffff;
                do {
                  uVar20 = uVar20 + 1;
                } while (*(char *)((longlong)pppplVar28 + uVar20) != '\0');
              }
            }
            else {
              uStack_680 = 0x141848ed5;
              FUN_142e54290(0x90,piVar13[1],uVar20 & 0xffffffff);
              *piVar13 = 1;
LAB_141848ed9:
              *(undefined1 *)((longlong)local_590 + (longlong)pppplVar28) = 0;
            }
            iVar11 = (int)uVar20;
            if ((iVar11 < 0) || (piVar13[1] + 1 <= iVar11)) {
              uStack_680 = 0x141848f00;
              FUN_142e54290(0x9c,uVar20 & 0xffffffff);
            }
            piVar13[2] = iVar11;
            if ((longlong ****)local_5a8 != (longlong ****)0x0) {
              uStack_680 = 0x141848f15;
              FUN_14019f2c0(local_5a8 + -2);
            }
          }
          else {
            if (*(int *)pppplVar2 < 1) {
              uStack_680 = 0x141848f64;
              FUN_142e52dd0(0xd2);
            }
            LOCK();
            *(int *)pppplVar2 = *(int *)pppplVar2 + 1;
            UNLOCK();
            if ((longlong ****)local_5a8 != (longlong ****)0x0) {
              uStack_680 = 0x141848f79;
              FUN_14019f2c0(local_5a8 + -2);
            }
            piVar29 = (int *)CONCAT44(uStack_574,local_578);
            lVar12 = CONCAT44(uStack_56c,uStack_570);
            pppplVar30 = (longlong ****)local_5c0;
            pppplVar28 = pppplVar22;
          }
        }
        local_5a8 = (char ****)pppplVar28;
        local_590 = (longlong ***)&local_5a0;
        local_5a0 = (char ***)0x0;
        uStack_680 = 0x141848fb3;
        piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
        piVar13[1] = 0;
        *piVar13 = -1;
        local_5a0 = (char ***)(piVar13 + 4);
        piVar13[2] = 0;
        *(undefined1 *)local_5a0 = 0;
        if (*piVar13 != -1) {
          uStack_680 = 0x141848fe1;
          FUN_142e52dd0(0x8b);
        }
        if (piVar13[1] < 0) {
          uStack_680 = 0x141848ff5;
          FUN_142e54290(0x90,piVar13[1],0);
        }
        *piVar13 = 1;
        *(undefined1 *)local_5a0 = 0;
        if (piVar13[1] + 1 < 1) {
          uStack_680 = 0x141849018;
          FUN_142e54290(0x9c,0);
        }
        piVar13[2] = 0;
        local_558 = (char ****)&local_560;
        local_560 = (longlong ***)0x0;
        uStack_680 = 0x141849040;
        local_650 = (longlong ****)FUN_140232590(local_4a8,piVar29 + 2);
        lVar31 = local_550;
        local_620[0] = 0;
        local_628 = &local_4c8;
        local_630 = (longlong ****)&local_5a8;
        local_638 = &local_5a0;
        local_640 = &local_560;
        local_648[0] = 0xff;
        local_658 = (char ****)((ulonglong)local_658 & 0xffffffff00000000);
        uStack_680 = 0x141849099;
        FUN_1415a8b80(&local_528,local_550,1,0xffffffff);
        local_590 = (longlong ***)0x0;
        uStack_680 = 0x1418490b3;
        FUN_14019a260(&local_590,&local_598);
        uStack_680 = 0x1418490bf;
        FUN_1415a8350(&local_590);
        if (local_538 != 0) {
          uStack_680 = 0x1418490d5;
          FUN_14019f2c0(local_538 + -0x10);
        }
      }
      if (lVar31 != 0) {
        uStack_680 = 0x1418490e4;
        FUN_14019f2c0(lVar31 + -0x10);
      }
      if (lVar12 != 0) {
        LOCK();
        piVar29 = (int *)(lVar12 + 8);
        iVar11 = *piVar29;
        *piVar29 = *piVar29 + -1;
        UNLOCK();
        pppplVar30 = (longlong ****)local_5c0;
        if (iVar11 == 1) {
          puVar15 = (undefined8 *)CONCAT44(uStack_56c,uStack_570);
          uStack_680 = 0x141849106;
          (**(code **)*puVar15)(puVar15);
          LOCK();
          piVar29 = (int *)((longlong)puVar15 + 0xc);
          iVar11 = *piVar29;
          *piVar29 = *piVar29 + -1;
          UNLOCK();
          pppplVar30 = (longlong ****)local_5c0;
          if (iVar11 == 1) {
            uStack_680 = 0x141849120;
            (**(code **)(*(longlong *)CONCAT44(uStack_56c,uStack_570) + 8))();
            pppplVar30 = (longlong ****)local_5c0;
          }
        }
      }
      if (pppplVar30 != (longlong ****)0x0) {
        uStack_680 = 0x141849132;
        FUN_14019f2c0(pppplVar30 + -2);
      }
      plVar17 = plStack_520;
      if (plStack_520 != (longlong *)0x0) {
        LOCK();
        plVar18 = plStack_520 + 1;
        lVar12 = *plVar18;
        *(int *)plVar18 = (int)*plVar18 + -1;
        UNLOCK();
        if ((int)lVar12 == 1) {
          uStack_680 = 0x141849154;
          (**(code **)*plStack_520)(plStack_520);
          LOCK();
          piVar29 = (int *)((longlong)plVar17 + 0xc);
          iVar11 = *piVar29;
          *piVar29 = *piVar29 + -1;
          UNLOCK();
          if (iVar11 == 1) {
            uStack_680 = 0x141849169;
            (**(code **)(*plVar17 + 8))(plVar17);
          }
        }
      }
      puVar25 = auStack_678;
      if ((char ****)local_5d8 != (char ****)0x0) {
        uStack_680 = 0x14184917c;
        FUN_14019f2c0(local_5d8 + -2);
        puVar25 = auStack_678;
      }
    }
    break;
  case 0x22:
    local_610 = (longlong ***)0x0;
    uStack_680 = 0x14184a30a;
    FUN_1406e9050(param_2,&local_618);
    uStack_680 = 0x14184a313;
    cVar8 = FUN_1406e8ae0(param_2);
    ppplVar32 = local_618;
    if (cVar8 == '\0') {
      uStack_680 = 0x14184a35e;
      puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0x8d);
      uStack_680 = 0x14184a36e;
      FUN_14019ba10(&local_610,*puVar15,ppplVar32);
      if ((char ****)local_608 != (char ****)0x0) {
        uStack_680 = 0x14184a381;
        FUN_14019f2c0(local_608 + -2);
      }
      uVar16 = 1;
    }
    else {
      uStack_680 = 0x14184a329;
      puVar15 = (undefined8 *)FUN_1408a9e40(&local_608,0x87);
      uStack_680 = 0x14184a339;
      FUN_14019ba10(&local_610,*puVar15,ppplVar32);
      if ((char ****)local_608 != (char ****)0x0) {
        uStack_680 = 0x14184a34c;
        FUN_14019f2c0(local_608 + -2);
      }
      uVar16 = 0xb;
    }
    uStack_680 = 0x14184a390;
    FUN_1415eca30(&local_610,uVar16);
    local_598 = local_610;
    puVar25 = auStack_678;
    if ((char ****)local_618 != (char ****)0x0) {
      uStack_680 = 0x14184a3a3;
      FUN_14019f2c0(local_618 + -2);
      local_598 = local_610;
      puVar25 = auStack_678;
    }
    break;
  case 0x92:
    uStack_680 = 0x141849195;
    FUN_1406e9050(param_2,&local_618);
    uStack_680 = 0x14184919e;
    FUN_1406e8ae0(param_2);
    uStack_680 = 0x1418491aa;
    FUN_1406e9050(param_2,&local_610);
    uVar16 = DAT_143ac2f58;
    uStack_680 = 0x1418491c9;
    uVar14 = FUN_14019bd40(&local_610,0x400,1);
    local_658 = (char ****)0x0;
    uStack_680 = 0x1418491df;
    FUN_1408bed00(uVar16,uVar14,0,1);
    ppplVar32 = local_610;
    if (*(int *)(local_610 + -2) != -1) {
      uStack_680 = 0x1418491f5;
      FUN_142e52dd0();
    }
    *(undefined4 *)(ppplVar32 + -2) = 1;
    iVar11 = iRamfffffffffffffff4;
    if ((char ****)ppplVar32 == (char ****)0x0) {
LAB_141849226:
      iVar23 = (int)ppppcVar34;
      if (iVar11 + 1 <= iVar23) goto LAB_14184922b;
    }
    else {
      ppppcVar34 = (char ****)0xffffffffffffffff;
      do {
        ppppcVar34 = (char ****)((longlong)ppppcVar34 + 1);
      } while (*(char *)((longlong)ppplVar32 + (longlong)ppppcVar34) != '\0');
      iVar11 = *(int *)((longlong)ppplVar32 + -0xc);
      if (-1 < (int)ppppcVar34) goto LAB_141849226;
LAB_14184922b:
      iVar23 = (int)ppppcVar34;
      uStack_680 = 0x141849238;
      FUN_142e54290(0x9c,(ulonglong)ppppcVar34 & 0xffffffff);
    }
    *(int *)(ppplVar32 + -1) = iVar23;
    local_5d0 = &local_5b8;
    plStack_5b0 = (longlong *)0x0;
    local_5e0 = (longlong ***)0x0;
    uStack_680 = 0x14184925b;
    FUN_14019a260(&local_5e0,&local_610);
    local_650 = &local_5b8;
    local_658 = (char ****)CONCAT44(local_658._4_4_,10000);
    uStack_680 = 0x141849281;
    FUN_14185b1c0(param_1,0x4e2019,&local_5e0,1);
    local_598 = local_618;
    puVar25 = auStack_678;
    if ((char ****)local_610 != (char ****)0x0) {
      uStack_680 = 0x141849294;
      FUN_14019f2c0(local_610 + -2);
      local_598 = local_618;
      puVar25 = auStack_678;
    }
  }
  if ((char ****)local_598 != (char ****)0x0) {
    *(undefined8 *)(puVar25 + -8) = 0x14184a3b6;
    FUN_14019f2c0(local_598 + -2);
  }
switchD_14184873b_caseD_b:
  *(undefined8 *)(puVar25 + -8) = 0x14184a3c6;
  return;
}



//===========================================================
// FUN_14185b1c0 @ 14185b1c0   (4298 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014185bdd2) */

void FUN_14185b1c0(longlong param_1,int param_2,longlong *param_3,undefined4 param_4,
                  undefined4 param_5,undefined8 param_6)

{
  short *psVar1;
  IUnknown *pIVar2;
  int *piVar3;
  short sVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  undefined4 uVar9;
  undefined8 *puVar10;
  undefined4 *puVar11;
  int *piVar12;
  int *piVar13;
  undefined8 uVar14;
  int *piVar15;
  int *piVar16;
  int *piVar17;
  undefined2 *puVar18;
  longlong lVar19;
  undefined2 *puVar20;
  ulonglong uVar21;
  longlong *plVar22;
  undefined8 *local_res8;
  int local_res10;
  longlong *local_res18;
  undefined4 local_res20;
  ulonglong in_stack_fffffffffffffd18;
  uint *puVar23;
  uint in_stack_fffffffffffffd28;
  undefined8 *local_2b8;
  IUnknown *local_2b0;
  short *local_2a8;
  char *local_2a0;
  int *local_298;
  int *local_290;
  undefined4 local_288;
  undefined4 local_284;
  int *local_280;
  int *local_278;
  uint local_270;
  undefined4 uStack_26c;
  undefined4 uStack_268;
  undefined4 uStack_264;
  undefined8 local_260;
  uint local_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined8 local_248;
  uint local_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined8 local_228;
  undefined4 local_220;
  int *local_218 [2];
  uint local_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined8 local_1f8;
  uint local_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined8 local_1e0;
  uint local_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined8 local_1c8;
  uint local_1c0;
  undefined4 uStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined8 local_1b0;
  uint local_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined8 local_198;
  uint local_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined8 local_180;
  uint local_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  int **local_158;
  undefined1 local_150 [8];
  undefined1 local_148 [8];
  undefined1 local_140 [8];
  undefined1 local_138 [8];
  undefined1 local_130 [8];
  undefined1 local_128 [8];
  undefined1 local_120 [8];
  uint local_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 local_108;
  uint local_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 local_f0;
  uint local_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 local_d8;
  uint local_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined8 local_c0;
  uint local_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 local_a8;
  uint local_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined8 local_90;
  uint local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_78;
  undefined8 *local_70 [2];
  undefined8 local_60;
  undefined8 uStack_58;
  
  *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x40c) = param_4;
  local_res10 = param_2;
  local_res18 = param_3;
  if (param_2 == 0) {
    iVar6 = *(int *)(*(longlong *)(param_1 + 0x68) + 0x2c);
    if (iVar6 != 0) {
      FUN_142086f60(DAT_143abfea0,iVar6,0);
    }
    FUN_141ba36f0(param_1);
    if (DAT_143acaf20 != 0) {
      local_278 = (int *)0x0;
      FUN_1425d0140(DAT_143acaf20,0,0,5000,&local_280,0xa0,in_stack_fffffffffffffd28 & 0xffffff00);
    }
    *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x408) = 0;
    *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x400) = 0;
    *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x410) = 0;
    *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x414) = 1;
    if (*param_3 != 0) {
      FUN_14019f2c0(*param_3 + -0x10);
    }
    FUN_1418b9130(param_6);
    return;
  }
  uVar5 = FUN_1429e3ef0();
  *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x408) = uVar5;
  FUN_14039f600(DAT_143aa8328,&local_2b0,param_2,0);
  if (local_2b0 == (IUnknown *)0x0) {
    if (*param_3 != 0) {
      FUN_14019f2c0(*param_3 + -0x10);
    }
    FUN_1418b9130(param_6);
    return;
  }
  iVar6 = FUN_1404187a0(param_2);
  pIVar2 = local_2b0;
  if (iVar6 == 0) {
    if (local_2b0 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_2b0 + 0x10))();
    }
    if (*param_3 != 0) {
      FUN_14019f2c0(*param_3 + -0x10);
    }
    FUN_1418b9130(param_6);
    return;
  }
  if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar10 = (undefined8 *)FUN_1401a5890(local_120,PTR_DAT_143a45b78);
  local_res8 = puVar10;
  (*DAT_143262a20)(&local_118);
  piVar15 = (int *)0x0;
  piVar16 = piVar15;
  if ((undefined8 *)*puVar10 != (undefined8 *)0x0) {
    piVar16 = *(int **)*puVar10;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,piVar16,&local_118);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_190 = local_118;
  uStack_18c = uStack_114;
  uStack_188 = uStack_110;
  uStack_184 = uStack_10c;
  local_180 = local_108;
  local_118 = local_118 & 0xffff0000;
  FUN_1401be120(puVar10);
  uVar5 = FUN_14022ee40(&local_190,0);
  local_res8 = (undefined8 *)CONCAT44(local_res8._4_4_,uVar5);
  if ((short)local_190 == 8) {
    local_190 = local_190 & 0xffff0000;
    if (CONCAT44(uStack_184,uStack_188) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_184,uStack_188) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_190);
  }
  pIVar2 = local_2b0;
  if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar10 = (undefined8 *)FUN_1401a5890(local_128,PTR_u_floatType_143a48340);
  local_2b8 = puVar10;
  (*DAT_143262a20)(&local_100);
  piVar16 = piVar15;
  if ((undefined8 *)*puVar10 != (undefined8 *)0x0) {
    piVar16 = *(int **)*puVar10;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,piVar16,&local_100);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_1a8 = local_100;
  uStack_1a4 = uStack_fc;
  uStack_1a0 = uStack_f8;
  uStack_19c = uStack_f4;
  local_198 = local_f0;
  local_100 = local_100 & 0xffff0000;
  FUN_1401be120(puVar10);
  local_res20 = FUN_14022ee40(&local_1a8,0);
  if ((short)local_1a8 == 8) {
    local_1a8 = local_1a8 & 0xffff0000;
    if (CONCAT44(uStack_19c,uStack_1a0) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_19c,uStack_1a0) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1a8);
  }
  pIVar2 = local_2b0;
  if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar10 = (undefined8 *)FUN_1401a5890(local_130,PTR_u_direction_143a45cf8);
  local_2b8 = puVar10;
  (*DAT_143262a20)(&local_e8);
  piVar16 = piVar15;
  if ((undefined8 *)*puVar10 != (undefined8 *)0x0) {
    piVar16 = *(int **)*puVar10;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,piVar16,&local_e8);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_1c0 = local_e8;
  uStack_1bc = uStack_e4;
  uStack_1b8 = uStack_e0;
  uStack_1b4 = uStack_dc;
  local_1b0 = local_d8;
  local_e8 = local_e8 & 0xffff0000;
  FUN_1401be120(puVar10);
  local_284 = FUN_14022ee40(&local_1c0,0);
  if ((short)local_1c0 == 8) {
    local_1c0 = local_1c0 & 0xffff0000;
    if (CONCAT44(uStack_1b4,uStack_1b8) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1b4,uStack_1b8) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1c0);
  }
  pIVar2 = local_2b0;
  if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar10 = (undefined8 *)FUN_1401a5890(local_138,PTR_u_speed_143a48060);
  local_2b8 = puVar10;
  (*DAT_143262a20)(&local_d0);
  piVar16 = piVar15;
  if ((undefined8 *)*puVar10 != (undefined8 *)0x0) {
    piVar16 = *(int **)*puVar10;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,piVar16,&local_d0);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_1d8 = local_d0;
  uStack_1d4 = uStack_cc;
  uStack_1d0 = uStack_c8;
  uStack_1cc = uStack_c4;
  local_1c8 = local_c0;
  local_d0 = local_d0 & 0xffff0000;
  FUN_1401be120(puVar10);
  local_288 = FUN_14022ee40(&local_1d8,0);
  if ((short)local_1d8 == 8) {
    local_1d8 = local_1d8 & 0xffff0000;
    if (CONCAT44(uStack_1cc,uStack_1d0) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1cc,uStack_1d0) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1d8);
  }
  pIVar2 = local_2b0;
  if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar10 = (undefined8 *)FUN_1401a5890(local_140,PTR_u_path_143a48348);
  local_2b8 = puVar10;
  (*DAT_143262a20)(&local_b8);
  piVar16 = piVar15;
  if ((undefined8 *)*puVar10 != (undefined8 *)0x0) {
    piVar16 = *(int **)*puVar10;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,piVar16,&local_b8);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_258 = local_b8;
  uStack_254 = uStack_b4;
  uStack_250 = uStack_b0;
  uStack_24c = uStack_ac;
  local_248 = local_a8;
  local_b8 = local_b8 & 0xffff0000;
  FUN_1401be120(puVar10);
  puVar18 = &DAT_143278568;
  uVar21 = 0xffffffffffffffff;
  local_2a8 = (short *)0x0;
  iVar6 = 0;
  if ((short)local_258 == 8) {
    puVar20 = (undefined2 *)CONCAT44(uStack_24c,uStack_250);
    lVar19 = 0;
    sVar4 = 8;
    if (puVar20 != (undefined2 *)0x0) goto LAB_14185b771;
  }
  else {
    puVar20 = &DAT_143278568;
LAB_14185b771:
    piVar16 = (int *)0xffffffffffffffff;
    do {
      piVar16 = (int *)((longlong)piVar16 + 1);
    } while (puVar20[(longlong)piVar16] != 0);
    iVar8 = (int)piVar16;
    iVar7 = iVar6;
    if (0 < iVar8) {
      iVar7 = iVar8;
    }
    puVar11 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar7 * 2 + 0x12));
    puVar11[1] = iVar7;
    *puVar11 = 0xffffffff;
    local_2a8 = (short *)(puVar11 + 4);
    puVar11[2] = 0;
    *local_2a8 = 0;
    local_290 = (int *)((longlong)iVar8 * 2);
    FUN_142ef7ba0(local_2a8,puVar20,local_290);
    psVar1 = local_2a8;
    if (*(int *)(local_2a8 + -8) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar8 == -1) || (iVar8 <= *(int *)(psVar1 + -6))) {
      psVar1[-8] = 1;
      psVar1[-7] = 0;
      if (iVar8 != -1) goto LAB_14185b808;
      piVar16 = piVar15;
      if (psVar1 != (short *)0x0) {
        piVar16 = (int *)0xffffffffffffffff;
        do {
          piVar16 = (int *)((longlong)piVar16 + 1);
        } while (psVar1[(longlong)piVar16] != 0);
      }
    }
    else {
      FUN_142e54290(0x90,*(int *)(psVar1 + -6),(ulonglong)piVar16 & 0xffffffff);
      psVar1[-8] = 1;
      psVar1[-7] = 0;
LAB_14185b808:
      *(undefined2 *)((longlong)local_290 + (longlong)local_2a8) = 0;
    }
    iVar7 = (int)piVar16;
    if ((iVar7 < 0) || (*(int *)(psVar1 + -6) + 1 <= iVar7)) {
      FUN_142e54290(0x9c,(ulonglong)piVar16 & 0xffffffff);
    }
    *(int *)(psVar1 + -4) = iVar7 * 2;
    lVar19 = CONCAT44(uStack_24c,uStack_250);
    sVar4 = (short)local_258;
  }
  if (sVar4 == 8) {
    local_258 = local_258 & 0xffff0000;
    if (lVar19 != 0) {
      (*DAT_143ad5990)(lVar19 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_258);
  }
  pIVar2 = local_2b0;
  if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar10 = (undefined8 *)FUN_1401a5890(local_148,PTR_u_bgmPath_143a48350);
  local_2b8 = puVar10;
  (*DAT_143262a20)(&local_a0);
  piVar16 = piVar15;
  if ((undefined8 *)*puVar10 != (undefined8 *)0x0) {
    piVar16 = *(int **)*puVar10;
  }
  iVar7 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,piVar16,&local_a0);
  if (iVar7 < 0) {
    _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_270 = local_a0;
  uStack_26c = uStack_9c;
  uStack_268 = uStack_98;
  uStack_264 = uStack_94;
  local_260 = local_90;
  local_a0 = local_a0 & 0xffff0000;
  FUN_1401be120(puVar10);
  if ((short)local_270 == 8) {
    puVar18 = (undefined2 *)CONCAT44(uStack_264,uStack_268);
    local_290 = (int *)0x0;
    lVar19 = 0;
    piVar16 = piVar15;
    sVar4 = 8;
    if (puVar18 != (undefined2 *)0x0) goto LAB_14185b940;
  }
  else {
LAB_14185b940:
    piVar17 = (int *)0xffffffffffffffff;
    do {
      piVar17 = (int *)((longlong)piVar17 + 1);
    } while (puVar18[(longlong)piVar17] != 0);
    iVar8 = (int)piVar17;
    iVar7 = iVar6;
    if (0 < iVar8) {
      iVar7 = iVar8;
    }
    piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar7 * 2 + 0x12));
    piVar12[1] = iVar7;
    *piVar12 = -1;
    piVar16 = piVar12 + 4;
    piVar12[2] = 0;
    *(short *)piVar16 = 0;
    local_290 = piVar16;
    FUN_142ef7ba0(piVar16,puVar18,(longlong)iVar8 * 2);
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar8 == -1) || (iVar8 <= piVar12[1])) {
      *piVar12 = 1;
      if (iVar8 != -1) goto LAB_14185b9c8;
      piVar17 = piVar15;
      if (piVar16 != (int *)0x0) {
        piVar17 = (int *)0xffffffffffffffff;
        do {
          piVar17 = (int *)((longlong)piVar17 + 1);
        } while (*(short *)((longlong)piVar16 + (longlong)piVar17 * 2) != 0);
      }
    }
    else {
      FUN_142e54290(0x90,piVar12[1],(ulonglong)piVar17 & 0xffffffff);
      *piVar12 = 1;
LAB_14185b9c8:
      *(short *)((longlong)iVar8 * 2 + (longlong)piVar16) = 0;
    }
    iVar7 = (int)piVar17;
    if ((iVar7 < 0) || (piVar12[1] + 1 <= iVar7)) {
      FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
    }
    piVar12[2] = iVar7 * 2;
    lVar19 = CONCAT44(uStack_264,uStack_268);
    sVar4 = (short)local_270;
  }
  if (sVar4 == 8) {
    local_270 = local_270 & 0xffff0000;
    if (lVar19 != 0) {
      (*DAT_143ad5990)(lVar19 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_270);
  }
  pIVar2 = local_2b0;
  if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar10 = (undefined8 *)FUN_1401a5890(local_150,PTR_u_isBgmOrEffect_143a48358);
  local_2b8 = puVar10;
  (*DAT_143262a20)(&local_88);
  piVar17 = piVar15;
  if ((undefined8 *)*puVar10 != (undefined8 *)0x0) {
    piVar17 = *(int **)*puVar10;
  }
  iVar7 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,piVar17,&local_88);
  if (iVar7 < 0) {
    _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_1f0 = local_88;
  uStack_1ec = uStack_84;
  uStack_1e8 = uStack_80;
  uStack_1e4 = uStack_7c;
  local_1e0 = local_78;
  local_88 = local_88 & 0xffff0000;
  FUN_1401be120(puVar10);
  iVar7 = FUN_14022ee40(&local_1f0,0);
  if ((short)local_1f0 == 8) {
    local_1f0 = local_1f0 & 0xffff0000;
    if (CONCAT44(uStack_1e4,uStack_1e8) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1e4,uStack_1e8) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_1f0);
  }
  pIVar2 = local_2b0;
  if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  puVar10 = (undefined8 *)FUN_1401a5890(local_218,PTR_u_repeat_143a485f0);
  local_2b8 = puVar10;
  (*DAT_143262a20)(&local_238);
  piVar17 = piVar15;
  if ((undefined8 *)*puVar10 != (undefined8 *)0x0) {
    piVar17 = *(int **)*puVar10;
  }
  iVar8 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,piVar17,&local_238);
  if (iVar8 < 0) {
    _com_issue_errorex(iVar8,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_208 = local_238;
  uStack_204 = uStack_234;
  uStack_200 = uStack_230;
  uStack_1fc = uStack_22c;
  local_1f8 = local_228;
  local_238 = local_238 & 0xffff0000;
  FUN_1401be120(puVar10);
  iVar8 = FUN_14022ee40(&local_208,0);
  if ((short)local_208 == 8) {
    local_208 = local_208 & 0xffff0000;
    if (CONCAT44(uStack_1fc,uStack_200) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_1fc,uStack_200) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_208);
  }
  uVar5 = (undefined4)(in_stack_fffffffffffffd18 >> 0x20);
  if ((local_2a8 == (short *)0x0) || (*local_2a8 == 0)) {
    if (piVar16 != (int *)0x0) {
      FUN_1401bebb0(piVar16 + -4);
    }
    if (local_2a8 != (short *)0x0) {
      FUN_1401bebb0(local_2a8 + -8);
    }
    if (local_2b0 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_2b0 + 0x10))();
    }
    if (*local_res18 != 0) {
      FUN_14019f2c0(*local_res18 + -0x10);
    }
    FUN_1418b9130(param_6);
    return;
  }
  if ((piVar16 == (int *)0x0) || ((short)*piVar16 == 0)) goto LAB_14185bed7;
  if (iVar7 == 0) {
    in_stack_fffffffffffffd28 = 0;
    in_stack_fffffffffffffd18 = in_stack_fffffffffffffd18 & 0xffffffff00000000;
    uVar9 = FUN_142085330(DAT_143abfea0,piVar16,100,iVar8 != 0,in_stack_fffffffffffffd18,0,3,0,0,4);
    uVar5 = (undefined4)(in_stack_fffffffffffffd18 >> 0x20);
    *(undefined4 *)(*(longlong *)(param_1 + 0x68) + 0x2c) = uVar9;
    goto LAB_14185bed7;
  }
  FUN_14208a6d0(DAT_143abfea0,local_70);
  local_2b8 = local_70[0];
  piVar17 = piVar16;
  if (local_70[0] != (undefined8 *)0x0) {
    uStack_174 = 1000;
    uStack_170 = 1000;
    uStack_16c = 1;
    uStack_168 = 0;
    uStack_164 = 0;
    local_158 = &local_280;
    local_178 = (uint)(iVar8 != 0);
    piVar12 = (int *)FUN_14019b780(&DAT_143ad68a0,0x18);
    local_278 = piVar15;
    if (piVar12 != (int *)0x0) {
      piVar12[0] = 0;
      piVar12[1] = 0;
      piVar12[2] = 0;
      piVar12[3] = 0;
      piVar12[2] = 1;
      piVar12[3] = 1;
      *(undefined ***)piVar12 = &PTR_LAB_143376160;
      *(undefined ***)(piVar12 + 4) = &PTR_FUN_1433760b0;
      local_278 = piVar12;
    }
    local_280 = local_278 + 4;
    local_60 = 0;
    uStack_58 = 0;
    local_298 = (int *)0x0;
    piVar12 = piVar16 + -4;
    piVar3 = local_298;
    if (piVar12 != (int *)0x0) {
      if (*piVar12 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        piVar12 = (int *)0xffffffffffffffff;
        do {
          piVar12 = (int *)((longlong)piVar12 + 1);
        } while (*(short *)((longlong)piVar16 + (longlong)piVar12 * 2) != 0);
        iVar7 = (int)piVar12;
        if (0 < iVar7) {
          iVar6 = iVar7;
        }
        piVar13 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar6 * 2 + 0x12));
        piVar13[1] = iVar6;
        *piVar13 = -1;
        piVar3 = piVar13 + 4;
        piVar13[2] = 0;
        *(short *)piVar3 = 0;
        local_218[0] = piVar3;
        FUN_142ef7ba0(piVar3,piVar16,(longlong)iVar7 * 2);
        if (*piVar13 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar7 == -1) || (iVar7 <= piVar13[1])) {
          *piVar13 = 1;
          if (iVar7 != -1) goto LAB_14185bd87;
          if (piVar3 != (int *)0x0) {
            do {
              uVar21 = uVar21 + 1;
            } while (*(short *)((longlong)piVar3 + uVar21 * 2) != 0);
            piVar15 = (int *)(uVar21 & 0xffffffff);
          }
        }
        else {
          FUN_142e54290(0x90,piVar13[1],(ulonglong)piVar12 & 0xffffffff);
          *piVar13 = 1;
LAB_14185bd87:
          *(short *)((longlong)piVar3 + (longlong)iVar7 * 2) = 0;
          piVar15 = piVar12;
        }
        iVar6 = (int)piVar15;
        if ((iVar6 < 0) || (piVar13[1] + 1 <= iVar6)) {
          FUN_142e54290(0x9c,(ulonglong)piVar15 & 0xffffffff);
        }
        piVar13[2] = iVar6 * 2;
        if (local_298 != (int *)0x0) {
          FUN_1401bebb0(local_298 + -4);
        }
      }
      else {
        if (*piVar12 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar12 = *piVar12 + 1;
        UNLOCK();
        piVar17 = local_290;
        piVar3 = piVar16;
        if (local_298 != (int *)0x0) {
          FUN_1401bebb0(local_298 + -4);
          piVar17 = local_290;
        }
      }
    }
    local_298 = piVar3;
    local_238 = local_178;
    uStack_234 = uStack_174;
    uStack_230 = uStack_170;
    uStack_22c = uStack_16c;
    local_228 = CONCAT44(uStack_164,uStack_168);
    local_220 = 0;
    in_stack_fffffffffffffd28 = in_stack_fffffffffffffd28 & 0xffffff00;
    puVar23 = &local_238;
    FUN_140fe01b0(local_2b8,&local_298,8,&local_280,puVar23,0,in_stack_fffffffffffffd28);
    uVar5 = (undefined4)((ulonglong)puVar23 >> 0x20);
  }
  *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x410) = 1;
  FUN_140d6d530(local_70);
  piVar16 = piVar17;
LAB_14185bed7:
  iVar6 = local_res10;
  *(int *)(*(longlong *)(param_1 + 0xa8) + 0x400) = local_res10;
  lVar19 = DAT_143acaf20;
  plVar22 = local_res18;
  if (DAT_143acaf20 != 0) {
    uVar14 = FUN_14189f610(local_218,param_6);
    plVar22 = local_res18;
    FUN_1425d0140(lVar19,*local_res18,local_res20,param_5,uVar14,0xa0,
                  in_stack_fffffffffffffd28 & 0xffffff00);
    uVar5 = (undefined4)((ulonglong)uVar14 >> 0x20);
  }
  if ((int)local_res8 == 3) {
    local_res8 = (undefined8 *)0x0;
    FUN_1401c1fb0(&local_res8);
    FUN_141bc8e40(param_1,&local_res8);
    if (piVar16 != (int *)0x0) {
      FUN_1401bebb0(piVar16 + -4);
    }
    if (local_2a8 != (short *)0x0) {
      FUN_1401bebb0(local_2a8 + -8);
    }
    if (local_2b0 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_2b0 + 0x10))();
    }
    if (*plVar22 != 0) {
      FUN_14019f2c0(*plVar22 + -0x10);
    }
    FUN_1418b9130(param_6);
  }
  else {
    FUN_141b9e970(param_1,&local_2a8,(ulonglong)local_res8 & 0xffffffff,local_284,
                  CONCAT44(uVar5,local_288));
    FUN_1403996a0(DAT_143aa8328,&local_2a0,iVar6,PTR_DAT_143a44dc8);
    if (((((char *)*plVar22 != (char *)0x0) && (*(char *)*plVar22 != '\0')) &&
        (local_2a0 != (char *)0x0)) && (*local_2a0 != '\0')) {
      puVar10 = (undefined8 *)FUN_14040dab0(&local_2a0,&local_res8,&DAT_143272c80,&DAT_1434b2af1);
      if (local_2a0 != (char *)0x0) {
        FUN_14019f2c0(local_2a0 + -0x10);
      }
      local_2a0 = (char *)*puVar10;
      *puVar10 = 0;
      if (local_res8 != (undefined8 *)0x0) {
        FUN_14019f2c0(local_res8 + -2);
      }
      if ((local_2a0 == (char *)0x0) || (*local_2a0 == '\0')) {
        if (local_2a0 != (char *)0x0) {
          FUN_14019f2c0(local_2a0 + -0x10);
        }
        if (piVar16 != (int *)0x0) {
          FUN_1401bebb0(piVar16 + -4);
        }
        if (local_2a8 != (short *)0x0) {
          FUN_1401bebb0(local_2a8 + -8);
        }
        if (local_2b0 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_2b0 + 0x10))();
        }
        if (*plVar22 != 0) {
          FUN_14019f2c0(*plVar22 + -0x10);
        }
        FUN_1418b9130(param_6);
        return;
      }
      puVar10 = (undefined8 *)FUN_14040d860(plVar22,&local_2b8,local_2a0,&DAT_143278814);
      if (local_2a0 != (char *)0x0) {
        FUN_14019f2c0(local_2a0 + -0x10);
      }
      local_2a0 = (char *)*puVar10;
      *puVar10 = 0;
      if (local_2b8 != (undefined8 *)0x0) {
        FUN_14019f2c0(local_2b8 + -2);
      }
      FUN_1415eca30(&local_2a0,0x1f);
    }
    *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x414) = 0;
    if (local_2a0 != (char *)0x0) {
      FUN_14019f2c0(local_2a0 + -0x10);
    }
    if (piVar16 != (int *)0x0) {
      FUN_1401bebb0(piVar16 + -4);
    }
    if (local_2a8 != (short *)0x0) {
      FUN_1401bebb0(local_2a8 + -8);
    }
    if (local_2b0 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_2b0 + 0x10))();
    }
    if (*plVar22 != 0) {
      FUN_14019f2c0(*plVar22 + -0x10);
    }
    FUN_1418b9130(param_6);
  }
  return;
}



//===========================================================
// FUN_14185b120 @ 14185b120   (144 bytes)
//===========================================================

void FUN_14185b120(undefined8 param_1,undefined4 param_2,longlong *param_3,undefined4 param_4)

{
  undefined8 local_28;
  undefined1 *local_20;
  undefined1 local_18 [8];
  undefined8 local_10;
  
  local_20 = local_18;
  local_10 = 0;
  local_28 = 0;
  FUN_14019a260(&local_28,param_3);
  FUN_14185b1c0(param_1,param_2,&local_28,param_4,200000,local_18);
  if (*param_3 != 0) {
    FUN_14019f2c0(*param_3 + -0x10);
  }
  return;
}


