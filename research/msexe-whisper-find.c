
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


