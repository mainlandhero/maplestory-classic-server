//===========================================================
// FUN_142cbaa80 @ 142cbaa80   (12107 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

void FUN_142cbaa80(longlong ******param_1,undefined4 param_2,longlong *******param_3)

{
  longlong ****pppplVar1;
  code *pcVar2;
  longlong ******pppppplVar3;
  undefined8 *puVar4;
  bool bVar5;
  longlong ******pppppplVar6;
  longlong lVar7;
  byte bVar8;
  char cVar9;
  char cVar10;
  undefined2 uVar11;
  ushort uVar12;
  int iVar13;
  undefined4 uVar14;
  uint uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  uint uVar18;
  undefined4 uVar19;
  longlong *****ppppplVar20;
  undefined8 *puVar21;
  longlong ******pppppplVar22;
  int *piVar23;
  char *pcVar24;
  undefined8 uVar25;
  longlong *******ppppppplVar26;
  longlong *plVar27;
  undefined8 *puVar28;
  longlong lVar29;
  longlong ******pppppplVar30;
  longlong *****ppppplVar31;
  undefined8 *puVar32;
  int iVar33;
  ulonglong uVar34;
  longlong *****ppppplVar35;
  longlong *******ppppppplVar36;
  longlong *****ppppplVar37;
  undefined1 auStack_698 [32];
  undefined8 local_678;
  undefined8 local_670;
  undefined8 local_668;
  undefined8 local_660;
  uint local_658;
  int local_650;
  longlong *******local_648;
  undefined4 local_640;
  undefined1 local_638;
  undefined8 local_630;
  longlong *******local_628;
  undefined8 local_620;
  longlong *******local_618;
  undefined8 uStack_610;
  longlong local_608;
  longlong ******local_5f8;
  uint uStack_5f0;
  longlong *******local_5d8;
  undefined8 local_5d0;
  uint local_5c8;
  undefined4 uStack_5c4;
  undefined1 local_5c0;
  undefined1 uStack_5bf;
  undefined2 uStack_5be;
  undefined4 uStack_5bc;
  longlong *******local_5b8;
  undefined8 local_5b0;
  undefined4 local_5a8;
  undefined1 local_5a0 [8];
  undefined8 local_598;
  undefined8 local_590;
  undefined1 local_57f;
  undefined1 local_578 [112];
  undefined1 local_508 [1104];
  undefined1 local_b8 [112];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_698;
  local_628 = param_3;
  local_5f8 = param_1;
  local_5a8 = param_2;
  iVar13 = FUN_142c4b720(DAT_143ac1898);
  if (iVar13 != 0) {
    return;
  }
  if (DAT_143ac87a0 != 0) {
    FUN_1415ed8e0(local_5a0);
    local_57f = 1;
    FUN_1415f0bf0(DAT_143ac87a0,local_5a0);
    FUN_1415ed910(local_5a0);
  }
  lVar7 = DAT_143ac3748;
  lVar29 = DAT_143aa84a0;
  switch(param_2) {
  case 0x70:
    FUN_142d51930(param_1,param_3);
    break;
  case 0x7b:
    FUN_142d54700(param_1,param_3);
    break;
  case 0x7c:
    FUN_142d54780(param_1,param_3);
    break;
  case 0x7d:
    FUN_142d563d0(param_1,param_3);
    break;
  case 0x7e:
    FUN_142d56f80(param_1,param_3);
    break;
  case 0x7f:
    FUN_142d57ea0(param_1,param_3);
    break;
  case 0x80:
    FUN_142d57ee0(param_1,param_3);
    break;
  case 0x81:
    FUN_142d57f20(param_1,param_3);
    break;
  case 0x82:
    FUN_142d608f0(param_1,param_3);
    break;
  case 0x83:
    FUN_142d60aa0(param_1,param_3);
    break;
  case 0x84:
    FUN_142d58bb0(param_1,param_3);
    break;
  case 0x85:
    FUN_142d58c40(param_1,param_3);
    break;
  case 0x86:
    FUN_142d58c50(param_1,param_3);
    break;
  case 0x87:
    FUN_142d58c60(param_1,param_3);
    break;
  case 0x88:
    FUN_142d59010(param_1,param_3);
    break;
  case 0x89:
    FUN_142d43ee0(param_1,param_3);
    break;
  case 0x8a:
    FUN_142d99ea0(param_1,param_3);
    break;
  case 0x8b:
    FUN_142cd8bd0(param_1,param_3);
    break;
  case 0x8c:
    FUN_142d634c0(param_1,param_3);
    break;
  case 0x8d:
    FUN_142d91560(param_1,param_3);
    break;
  case 0x8e:
    FUN_142d92bd0(param_1,param_3);
    break;
  case 0x8f:
    FUN_142dea760(param_1,param_3);
    break;
  case 0x91:
    FUN_142d93b40(param_1,param_3);
    break;
  case 0x92:
    FUN_142d94300(param_1,param_3);
    break;
  case 0x93:
    FUN_142d94250(param_1,param_3);
    break;
  case 0x94:
    FUN_142d94280(param_1,param_3);
    break;
  case 0x95:
    FUN_142d942b0(param_1,param_3);
    break;
  case 0x96:
    FUN_142d94350(param_1,param_3);
    break;
  case 0x97:
    FUN_142d93310(param_1,param_3);
    break;
  case 0x98:
    FUN_142d948e0(param_1,param_3);
    break;
  case 0x99:
    FUN_142d94d10(param_1,param_3);
    break;
  case 0x9a:
    FUN_142d94f00(param_1,param_3);
    break;
  case 0x9b:
    FUN_142d950f0(param_1,param_3);
    break;
  case 0x9c:
    FUN_142d95350(param_1,param_3);
    break;
  case 0x9d:
    FUN_142d95580(param_1,param_3);
    break;
  case 0x9e:
    FUN_142d955d0(param_1,param_3);
    break;
  case 0x9f:
    FUN_142d95600(param_1,param_3);
    break;
  case 0xa0:
    FUN_142d56370(param_1,param_3);
    break;
  case 0xa1:
    FUN_142d1cdf0(param_1,param_3);
    break;
  case 0xa2:
    FUN_142cd87d0(param_1,param_3);
    break;
  case 0xa3:
    FUN_141183ec0(param_3);
    break;
  case 0xa4:
    FUN_1406e9170(param_3,&local_5c8,4);
    FUN_141351aa0(local_5c8);
    break;
  case 0xa5:
    FUN_1413bab80(param_3);
    break;
  case 0xa6:
    FUN_142defd30(param_1,param_3);
    break;
  case 0xa7:
    FUN_142defd40(param_1,param_3);
    break;
  case 0xa8:
    FUN_142de58b0(param_1,param_3);
    break;
  case 0xa9:
    FUN_142ddfd10(param_1,param_3);
    break;
  case 0xaa:
    FUN_14035d880(param_3);
    break;
  case 0xab:
    FUN_142d60c70(param_1,param_3);
    break;
  case 0xac:
    FUN_142d60d40(param_1,param_3);
    break;
  case 0xad:
    FUN_142dae820(param_1,param_3);
    break;
  case 0xae:
    FUN_142dafc80(param_1,param_3);
    break;
  case 0xb2:
    iVar13 = FUN_1406e8c20(param_3);
    *(int *)(param_1 + 0x47a) = iVar13;
    iVar13 = FUN_1406e8c20(param_3);
    *(int *)((longlong)param_1 + 0x23d4) = iVar13;
    break;
  case 0xb3:
    uVar18 = thunk_FUN_1406e8c20(param_3);
    if (0 < (int)uVar18) {
      uVar34 = (ulonglong)uVar18;
      do {
        uVar17 = thunk_FUN_1406e8c20(param_3);
        FUN_1406e9050(param_3,&local_5f8);
        FUN_142d06960(param_1 + 0x6d7,uVar17,&local_5f8);
        if (local_5f8 != (longlong ******)0x0) {
          FUN_14019f2c0(local_5f8 + -2);
        }
        uVar34 = uVar34 - 1;
      } while (uVar34 != 0);
    }
    break;
  case 0xb4:
    FUN_142d09280(param_1,param_3);
    break;
  case 0xb5:
    FUN_142d92e30(param_1,param_3);
    break;
  case 0xb6:
    FUN_142d94670(param_1,param_3);
    break;
  case 0xb7:
    *(int *)(param_1 + 0x466) = 0;
    iVar13 = FUN_1429e3ef0();
    *(int *)((longlong)param_1 + 0x2334) = iVar13;
    uVar17 = FUN_1429e3ef0();
    FUN_142e54b20(uVar17);
    uVar17 = FUN_1429e3ef0();
    FUN_142e54f40(uVar17);
    bVar8 = FUN_1406e8ae0(param_3);
    cVar10 = FUN_1406e8ae0(param_3);
    if (cVar10 == '\0') {
      uVar25 = FUN_1408a9e40(&local_618,0x1b3);
      FUN_1415eca30(uVar25,0xb);
      if (local_618 != (longlong *******)0x0) {
        FUN_14019f2c0(local_618 + -2);
      }
      *(int *)(param_1 + 0x63a) = 0;
    }
    else {
      if (bVar8 == 0) {
        uVar25 = FUN_1408a9e40(&local_618,0xfdc);
        FUN_1415eca30(uVar25,0xb);
      }
      else {
        uVar25 = FUN_1408a9e40(&local_618,0xfdb);
        FUN_1415eca30(uVar25,0xb);
      }
      if (local_618 != (longlong *******)0x0) {
        FUN_14019f2c0(local_618 + -2);
      }
      *(uint *)((longlong)param_1 + 0x31d4) = (uint)bVar8;
      *(uint *)(param_1 + 0x63a) = (uint)bVar8;
    }
    break;
  case 0xb8:
  case 0xf5:
    goto switchD_142cbab2d_caseD_b8;
  case 0xb9:
    FUN_142cd8ee0(param_1,param_3);
    break;
  case 0xba:
    FUN_142cd92f0(param_1,param_3);
    break;
  case 0xbb:
    FUN_142d95630(param_1,param_3);
    break;
  case 0xbc:
    FUN_142d956f0(param_1,param_3);
    break;
  case 0xbd:
    FUN_142cd9980(param_1,param_3);
    break;
  case 0xbe:
    if (DAT_143aa84f8 != 0) {
      FUN_141e75800(DAT_143aa84f8,0xbe,param_3);
    }
    break;
  case 0xbf:
    uVar11 = FUN_1406e8b80(param_3);
    FUN_1406e8b80(param_3);
    uVar25 = FUN_140cf1560();
    FUN_141779560(uVar25,uVar11);
    break;
  case 0xc0:
    bVar8 = FUN_1406e8ae0(param_3);
    *(uint *)((longlong)param_1 + 0x2fa4) = (uint)bVar8;
    if (DAT_143acf088 != 0) {
      FUN_14246fd50();
    }
    break;
  case 0xc1:
    FUN_142cd9bb0(param_1,param_3);
    break;
  case 0xc2:
    FUN_142cd9de0(param_1,param_3);
    break;
  case 0xc3:
    FUN_142cda010(param_1,param_3);
    break;
  case 0xc4:
    FUN_142cda260(param_1,param_3);
    break;
  case 0xc5:
    FUN_142cdafc0(param_1,param_3);
    break;
  case 0xc6:
    FUN_142cdb120(param_1,param_3);
    break;
  case 199:
    FUN_1406e9050(param_3,&local_5c8);
    FUN_1406e9050(param_3,&local_5b8);
    plVar27 = (longlong *)FUN_141892840();
    pcVar2 = *(code **)(*plVar27 + 0xa8);
    local_618 = (longlong *******)&local_628;
    local_628 = (longlong *******)0x0;
    FUN_14019a260(&local_628,&local_5b8);
    local_5d8 = (longlong *******)0x0;
    FUN_14019a260(&local_5d8,&local_5c8);
    (*pcVar2)(plVar27,&local_5d8,&local_628);
    if (local_5b8 != (longlong *******)0x0) {
      FUN_14019f2c0(local_5b8 + -2);
    }
    ppppppplVar26 = (longlong *******)CONCAT44(uStack_5c4,local_5c8);
    goto LAB_142cbd7c7;
  case 200:
    FUN_142cdb4c0(param_1,param_3);
    break;
  case 0xc9:
    ppppplVar20 = param_1[0x470];
    if (ppppplVar20 == (longlong *****)0x0) {
      FUN_142e52ed0(0x428,0);
      ppppplVar20 = param_1[0x470];
    }
    FUN_14211df30(ppppplVar20,7);
    break;
  case 0xca:
    FUN_142d9dc70(param_1,param_3);
    break;
  case 0xcb:
    FUN_1406e9050(param_3,&local_5d8);
    uVar17 = FUN_1406e8c20(param_3);
    ppppppplVar26 = local_5d8;
    if ((local_5d8 != (longlong *******)0x0) && (*(char *)local_5d8 != '\0')) {
      ppppplVar20 = param_1[0x540];
      if (ppppplVar20 == (longlong *****)0x0) {
        FUN_142e52ed0(0x431,0);
        ppppplVar20 = param_1[0x540];
      }
      local_628 = (longlong *******)0x0;
      FUN_14019a260(&local_628,&local_5d8);
      local_678 = (longlong *******)CONCAT44(local_678._4_4_,uVar17);
      FUN_141e2c6b0(ppppplVar20,&local_628,0,1);
      ppppppplVar26 = local_5d8;
    }
    goto LAB_142cbd7c7;
  case 0xcc:
    FUN_1406e9050(param_3,&local_5d8);
    ppppppplVar26 = local_5d8;
    if ((local_5d8 != (longlong *******)0x0) && (*(char *)local_5d8 != '\0')) {
      ppppplVar20 = param_1[0x540];
      if (ppppplVar20 == (longlong *****)0x0) {
        FUN_142e52ed0(0x431,0);
        ppppplVar20 = param_1[0x540];
      }
      local_628 = (longlong *******)0x0;
      FUN_14019a260(&local_628,&local_5d8);
      local_678 = (longlong *******)((ulonglong)local_678._4_4_ << 0x20);
      FUN_141e2c6b0(ppppplVar20,&local_628,0,0);
      ppppppplVar26 = local_5d8;
    }
    goto LAB_142cbd7c7;
  case 0xcd:
    uVar17 = FUN_1406e8c20(param_3);
    FUN_1406e9050(param_3,&local_5d8);
    ppppppplVar26 = local_5d8;
    if ((local_5d8 != (longlong *******)0x0) && (*(char *)local_5d8 != '\0')) {
      ppppplVar20 = param_1[0x540];
      if (ppppplVar20 == (longlong *****)0x0) {
        FUN_142e52ed0(0x431,0);
        ppppplVar20 = param_1[0x540];
      }
      local_628 = (longlong *******)0x0;
      FUN_14019a260(&local_628,&local_5d8);
      local_678 = (longlong *******)((ulonglong)local_678._4_4_ << 0x20);
      FUN_141e2c6b0(ppppplVar20,&local_628,uVar17,0);
      ppppppplVar26 = local_5d8;
    }
    goto LAB_142cbd7c7;
  case 0xce:
    cVar10 = FUN_1406e8ae0(param_3);
    FUN_1406e9050(param_3,&local_5d8);
    cVar9 = FUN_1406e8ae0(param_3);
    ppppppplVar26 = local_5d8;
    if ((local_5d8 != (longlong *******)0x0) && (*(char *)local_5d8 != '\0')) {
      ppppplVar20 = param_1[0x540];
      if (ppppplVar20 == (longlong *****)0x0) {
        FUN_142e52ed0(0x431,0);
        ppppplVar20 = param_1[0x540];
      }
      local_628 = (longlong *******)0x0;
      FUN_14019a260(&local_628,&local_5d8);
      FUN_141e2c860(ppppplVar20,&local_628,cVar10 != '\0',cVar9 != '\0');
      ppppppplVar26 = local_5d8;
    }
    goto LAB_142cbd7c7;
  case 0xcf:
    ppppplVar20 = param_1[0x540];
    if (ppppplVar20 == (longlong *****)0x0) {
      FUN_142e52ed0(0x431,0);
      ppppplVar20 = param_1[0x540];
    }
    FUN_141e2c990(ppppplVar20);
    break;
  case 0xd0:
    FUN_1406e9050(param_3,&local_628);
    uVar17 = FUN_1406e8c20(param_3);
    uVar16 = FUN_1406e8c20(param_3);
    cVar10 = FUN_1406e8ae0(param_3);
    ppppppplVar26 = local_628;
    if ((local_628 != (longlong *******)0x0) && (*(char *)local_628 != '\0')) {
      if (DAT_143acaf20 == 0) {
        if (cVar10 == '\0') goto LAB_142cbd7c7;
        lVar29 = FUN_142d24200();
        local_5d0 = 0;
        local_678 = (longlong *******)&local_5d8;
      }
      else {
        uStack_610 = (longlong *******)0x0;
        local_678 = (longlong *******)&local_618;
        lVar29 = DAT_143acaf20;
      }
      local_668 = (longlong *****)((ulonglong)local_668 & 0xffffffffffffff00);
      local_670 = (longlong *****)CONCAT44(local_670._4_4_,0xa0);
      FUN_1425d0140(lVar29,local_628,uVar17,uVar16);
      ppppppplVar26 = local_628;
    }
    goto LAB_142cbd7c7;
  case 0xd1:
    FUN_1406e9050(param_3,&local_628);
    uVar17 = FUN_1406e8c20(param_3);
    uVar16 = FUN_1406e8c20(param_3);
    uVar19 = FUN_1406e8c20(param_3);
    ppppppplVar26 = local_628;
    if (((local_628 != (longlong *******)0x0) && (*(char *)local_628 != '\0')) &&
       (DAT_143acaf20 != 0)) {
      uStack_610 = (longlong *******)0x0;
      local_668 = (longlong *****)((ulonglong)local_668 & 0xffffffffffffff00);
      local_670 = (longlong *****)CONCAT44(local_670._4_4_,uVar19);
      local_678 = (longlong *******)&local_618;
      FUN_1425d0140(DAT_143acaf20,local_628,uVar17,uVar16);
      ppppppplVar26 = local_628;
    }
    goto LAB_142cbd7c7;
  case 0xd2:
    FUN_142cdcef0(param_1,param_3);
    break;
  case 0xd3:
    FUN_1406e9050(param_3,&local_5f8);
    uVar17 = FUN_1406e8c20(param_3);
    uVar16 = FUN_1406e8c20(param_3);
    FUN_1406e8ae0(param_3);
    iVar13 = FUN_1406e8c20(param_3);
    if ((local_5f8 != (longlong ******)0x0) && (*(char *)local_5f8 != '\0')) {
      iVar33 = 0xa0;
      if (iVar13 != 0) {
        iVar33 = iVar13;
      }
      if (DAT_143acaf20 != 0) {
        local_668 = (longlong *****)CONCAT71(local_668._1_7_,1);
        uStack_610 = (longlong *******)0x0;
        local_670 = (longlong *****)CONCAT44(local_670._4_4_,iVar33);
        local_678 = (longlong *******)&local_618;
        FUN_1425d0140(DAT_143acaf20,local_5f8,uVar17,uVar16);
      }
    }
    if (local_5f8 != (longlong ******)0x0) {
      FUN_14019f2c0(local_5f8 + -2);
    }
    break;
  case 0xd4:
    FUN_1406e9050(param_3,&local_5f8);
    uVar17 = FUN_1406e8c20(param_3);
    uVar16 = FUN_1406e8c20(param_3);
    cVar10 = FUN_1406e8ae0(param_3);
    uVar19 = FUN_1406e8c20(param_3);
    cVar9 = FUN_1409095e0(param_1,uVar19);
    if (((cVar9 != '\0') && (local_5f8 != (longlong ******)0x0)) && (*(char *)local_5f8 != '\0')) {
      if (DAT_143acaf20 == 0) {
        if (cVar10 == '\0') goto LAB_142cbb7a0;
        lVar29 = FUN_142d24200();
        local_620 = 0;
        local_678 = (longlong *******)&local_628;
      }
      else {
        uStack_610 = (longlong *******)0x0;
        local_678 = (longlong *******)&local_618;
        lVar29 = DAT_143acaf20;
      }
      local_668 = (longlong *****)((ulonglong)local_668 & 0xffffffffffffff00);
      local_670 = (longlong *****)CONCAT44(local_670._4_4_,0xa0);
      FUN_1425d0140(lVar29,local_5f8,uVar17,uVar16);
    }
LAB_142cbb7a0:
    if (local_5f8 != (longlong ******)0x0) {
      FUN_14019f2c0(local_5f8 + -2);
    }
    break;
  case 0xd5:
    uVar17 = FUN_1406e8c20(param_3);
    uVar16 = FUN_1406e8c20(param_3);
    uVar19 = FUN_1406e8c20(param_3);
    uVar14 = FUN_1406e8c20(param_3);
    local_5c0 = (undefined1)uVar14;
    uStack_5bf = (undefined1)((uint)uVar14 >> 8);
    uStack_5be = (undefined2)((uint)uVar14 >> 0x10);
    bVar8 = FUN_1406e8ae0(param_3);
    local_5c8 = (uint)bVar8;
    FUN_1406e9050(param_3,&local_5d8);
    ppppppplVar26 = local_5d8;
    if ((local_5d8 != (longlong *******)0x0) && (*(char *)local_5d8 != '\0')) {
      ppppplVar20 = param_1[0x540];
      if (ppppplVar20 == (longlong *****)0x0) {
        FUN_142e52ed0(0x431,0);
        ppppplVar20 = param_1[0x540];
      }
      local_628 = (longlong *******)0x0;
      FUN_14019a260(&local_628,&local_5d8);
      local_658 = local_5c8;
      local_660 = (longlong ******)
                  CONCAT44(local_660._4_4_,CONCAT22(uStack_5be,CONCAT11(uStack_5bf,local_5c0)));
      local_668 = (longlong *****)((ulonglong)local_668._4_4_ << 0x20);
      local_670 = (longlong *****)((ulonglong)local_670._4_4_ << 0x20);
      local_678 = (longlong *******)CONCAT44(local_678._4_4_,uVar19);
      FUN_141e2ca60(ppppplVar20,&local_628,uVar17,uVar16);
      ppppppplVar26 = local_5d8;
    }
    goto LAB_142cbd7c7;
  case 0xd6:
    FUN_142cdd720(param_1,param_3);
    break;
  case 0xd7:
    FUN_1406e9170(param_3,&local_5c0,4);
    FUN_1406e9170(param_3,&local_5c8,4);
    FUN_140909d80(local_5a0,CONCAT22(uStack_5be,CONCAT11(uStack_5bf,local_5c0)),local_5c8);
    uVar17 = FUN_1407a3230(local_598);
    local_5d8 = (longlong *******)CONCAT44(local_5d8._4_4_,uVar17);
    uVar17 = FUN_140799b90(local_590);
    local_628 = (longlong *******)CONCAT44(local_628._4_4_,uVar17);
    FUN_14079bec0(local_590,&local_618);
    local_5b8 = (longlong *******)CONCAT44(local_5b8._4_4_,9);
    FUN_1406ed520(local_508,0x1ed);
    FUN_1406ede20(local_508,&local_5b8,4);
    FUN_1406ede20(local_508,&local_5c0,4);
    FUN_1406ede20(local_508,&local_5c8,4);
    FUN_1406ede20(local_508,&local_5d8,4);
    FUN_1406ede20(local_508,&local_628,4);
    FUN_1406edc80(local_508,&local_618);
    FUN_1406ef4e0(local_508);
    FUN_1406ed610(local_508);
    ppppppplVar26 = local_618;
    goto LAB_142cbd7c7;
  case 0xd8:
    FUN_142cddaa0(param_1,param_3);
    break;
  case 0xd9:
    FUN_142cddf10(param_1,param_3);
    break;
  case 0xda:
    FUN_142cde3c0(param_1,param_3);
    break;
  case 0xdb:
    FUN_142d9c9f0(param_1);
    break;
  case 0xdc:
    FUN_142dd22b0(param_1,param_3);
    break;
  case 0xdd:
    FUN_142dd43b0(param_1,param_3);
    break;
  case 0xde:
    FUN_142dd4570(param_1,param_3);
    break;
  case 0xdf:
    FUN_142de9210(param_1,param_3);
    break;
  case 0xe0:
    FUN_142de9540(param_1,param_3);
    break;
  case 0xe1:
    iVar13 = FUN_1406e8c20(param_3);
    local_628 = (longlong *******)CONCAT44(local_628._4_4_,iVar13);
    local_5f8 = DAT_1434936b8;
    FUN_1406e9170(param_3,&local_5f8,8);
    iVar33 = (*DAT_143ad5648)(&local_5f8);
    ppppplVar20 = param_1[0x46b];
    if (iVar33 == 0) {
      lVar29 = *(longlong *)((longlong)ppppplVar20 + 0xfa1);
      if (lVar29 != 0) {
        uVar34 = (ulonglong)(longlong)iVar13 % (ulonglong)*(uint *)((longlong)ppppplVar20 + 0xfa9);
        puVar21 = *(undefined8 **)(lVar29 + uVar34 * 8);
        if (puVar21 != (undefined8 *)0x0) {
          puVar4 = puVar21;
          puVar32 = (undefined8 *)puVar21[1];
          if (*(int *)(puVar21 + 2) == iVar13) {
            *(undefined8 **)(lVar29 + uVar34 * 8) = (undefined8 *)puVar21[1];
LAB_142cbbae8:
            (**(code **)*puVar21)(puVar21,1);
          }
          else {
            do {
              puVar21 = puVar32;
              puVar28 = puVar4;
              if (puVar21 == (undefined8 *)0x0) goto switchD_142cbab2d_caseD_71;
              puVar4 = puVar21;
              puVar32 = (undefined8 *)puVar21[1];
            } while (*(int *)(puVar21 + 2) != iVar13);
            puVar28[1] = (undefined8 *)puVar21[1];
            if (puVar21 != (undefined8 *)0x0) goto LAB_142cbbae8;
          }
          *(int *)((longlong)ppppplVar20 + 0xfad) = *(int *)((longlong)ppppplVar20 + 0xfad) + -1;
        }
      }
    }
    else {
      FUN_1402fce50((longlong)ppppplVar20 + 0xfa1,&local_628,&local_5f8);
    }
    break;
  case 0xe2:
    FUN_142cef640(param_1,param_3);
    break;
  case 0xe3:
    FUN_142d96920(param_1,param_3);
    break;
  case 0xe4:
    FUN_142d96960(param_1,param_3);
    break;
  case 0xe5:
    FUN_142ce7780(param_1,param_3);
    break;
  case 0xe6:
    uVar17 = FUN_1406e8c20(param_3);
    ppppplVar20 = param_1[0x46b];
    uVar18 = FUN_1406e8c20(param_3);
    local_628 = (longlong *******)CONCAT44(local_628._4_4_,uVar18);
    uVar15 = FUN_1406e8c20(param_3);
    uVar34 = (ulonglong)uVar15;
    plVar27 = (longlong *)FUN_1402d2c10(ppppplVar20,uVar17);
    puVar21 = (undefined8 *)*plVar27;
    cVar10 = *(char *)((longlong)puVar21[1] + 0x19);
    puVar4 = puVar21;
    puVar32 = (undefined8 *)puVar21[1];
    while (cVar10 == '\0') {
      if (*(uint *)(puVar32 + 4) < uVar18) {
        puVar28 = (undefined8 *)puVar32[2];
        puVar32 = puVar4;
      }
      else {
        puVar28 = (undefined8 *)*puVar32;
      }
      puVar4 = puVar32;
      puVar32 = puVar28;
      cVar10 = *(char *)((longlong)puVar28 + 0x19);
    }
    param_1 = local_5f8;
    if (((*(char *)((longlong)puVar4 + 0x19) == '\0') && (*(uint *)(puVar4 + 4) <= uVar18)) &&
       (puVar4 != puVar21)) {
      if (uVar15 == 0) {
LAB_142cbbbf8:
        FUN_142d3bae0(plVar27,&local_628);
        param_1 = local_5f8;
      }
      else if (0 < (int)uVar15) {
        do {
          uVar11 = FUN_1406e8b80(param_3);
          local_5c0 = (undefined1)uVar11;
          uStack_5bf = (undefined1)((ushort)uVar11 >> 8);
          FUN_142d3b6c0(puVar4 + 5,&local_5c0);
          uVar34 = uVar34 - 1;
        } while (uVar34 != 0);
        param_1 = local_5f8;
        if (puVar4[6] == 0) goto LAB_142cbbbf8;
      }
    }
    break;
  case 0xe7:
    uVar12 = FUN_1406e8b80(param_3);
    if (uVar12 != 0) {
      uVar34 = (ulonglong)uVar12;
      do {
        uVar12 = FUN_1406e8b80(param_3);
        local_628 = (longlong *******)CONCAT44(local_628._4_4_,(uint)uVar12);
        cVar10 = FUN_1406e8ae0(param_3);
        local_5c0 = cVar10 != '\0';
        FUN_1404093e0(param_1 + 0x629,&local_628,&local_5c0);
        uVar34 = uVar34 - 1;
      } while (uVar34 != 0);
    }
    break;
  case 0xe8:
    FUN_142cdecc0(param_1,param_3);
    break;
  case 0xe9:
    FUN_142cdf2b0(param_1,param_3);
    break;
  case 0xea:
    FUN_142d9fa90(param_1,param_3);
    break;
  case 0xeb:
    FUN_142d9fc00(param_1,param_3);
    break;
  case 0xec:
    FUN_142d9fcf0(param_1,param_3);
    break;
  case 0xed:
    FUN_142da08c0(param_1,param_3);
    break;
  case 0xee:
    FUN_142da0950(param_1,param_3);
    break;
  case 0xef:
    FUN_142cf1e40(param_1,param_3);
    break;
  case 0xf0:
    FUN_142cf1c50(param_1,param_3);
    break;
  case 0xf1:
    FUN_142d96c10(param_1,param_3);
    break;
  case 0xf2:
    FUN_142d96c60(param_1,param_3);
    break;
  case 0xf3:
    FUN_142d96d90(param_1,param_3);
    break;
  case 0xf6:
    FUN_142d4a670(param_1,param_3);
    break;
  case 0xf8:
    *(int *)(param_1 + 0x466) = 0;
    iVar13 = FUN_1429e3ef0();
    *(int *)((longlong)param_1 + 0x2334) = iVar13;
    uVar17 = FUN_1429e3ef0();
    FUN_142e54b20(uVar17);
    uVar17 = FUN_1429e3ef0();
    FUN_142e54f40(uVar17);
    cVar10 = FUN_1406e8ae0(param_3);
    if ((cVar10 == '\x03') || (cVar10 == '\x04')) {
      FUN_142cb4530(param_1,0x50);
      uVar25 = 0xfff;
    }
    else {
      if (cVar10 != '\x05') {
        if (cVar10 == '\x06') {
          if (DAT_143ad7528 != 0) {
            FUN_142238940();
          }
        }
        else if ((cVar10 == '\a') && (DAT_143ad7528 != 0)) {
          FUN_142238f40();
        }
        break;
      }
      FUN_142cb4530(param_1,0x50);
      uVar25 = 0x1000;
    }
    goto LAB_142cbbe33;
  case 0xf9:
    *(int *)(param_1 + 0x466) = 0;
    iVar13 = FUN_1429e3ef0();
    *(int *)((longlong)param_1 + 0x2334) = iVar13;
    uVar17 = FUN_1429e3ef0();
    FUN_142e54b20(uVar17);
    uVar17 = FUN_1429e3ef0();
    FUN_142e54f40(uVar17);
    cVar10 = FUN_1406e8ae0(param_3);
    if ((cVar10 == '\x03') || (cVar10 == '\x04')) {
      FUN_142cb4530(param_1,0x7f);
      uVar25 = 0x10d5;
    }
    else {
      if (cVar10 != '\x05') {
        if (cVar10 == '\x06') {
          if (DAT_143ad7530 != 0) {
            FUN_14223c3d0();
          }
        }
        else if ((cVar10 == '\a') && (DAT_143ad7530 != 0)) {
          FUN_14223c9e0();
        }
        break;
      }
      FUN_142cb4530(param_1,0x7f);
      uVar25 = 0x10d6;
    }
LAB_142cbbe33:
    uVar25 = FUN_1408a9e40(&local_618,uVar25);
    local_650 = 0;
    local_658 = 0;
    local_660 = (longlong ******)((ulonglong)local_660._4_4_ << 0x20);
    local_668 = (longlong *****)((ulonglong)local_668._4_4_ << 0x20);
    local_670 = (longlong *****)((ulonglong)local_670._4_4_ << 0x20);
    local_678 = (longlong *******)((ulonglong)local_678._4_4_ << 0x20);
    FUN_142a26280(uVar25,0,0,1);
    break;
  case 0xfa:
    local_5c8 = 0;
    FUN_1406e9170(param_3,&local_5c8,4);
    *(uint *)(param_1 + 0x460) = local_5c8;
    local_670 = (longlong *****)0x0;
    local_678 = (longlong *******)((ulonglong)local_678 & 0xffffffff00000000);
    FUN_142cbefd0(param_1,0,0,0);
    break;
  case 0xfb:
    FUN_142cf22d0(param_1,param_3);
    break;
  case 0xfc:
    iVar13 = FUN_1406e8c20(param_3);
    *(int *)(param_1 + 0x471) = iVar13;
    local_670 = (longlong *****)0x0;
    local_678 = (longlong *******)((ulonglong)local_678 & 0xffffffff00000000);
    FUN_142cbefd0(param_1,0,0,0);
    break;
  case 0xfd:
    FUN_1408c67b0((int *)((longlong)param_1 + 0x238c),param_3);
    break;
  case 0xfe:
    FUN_1414b0970(param_3);
    break;
  case 0xff:
    FUN_142d95e20(param_1,param_3);
    break;
  case 0x100:
    FUN_142d96060(param_1,param_3);
    break;
  case 0x101:
    FUN_142d96090(param_1,param_3);
    break;
  case 0x103:
    FUN_142d95790(param_1,param_3);
    break;
  case 0x104:
    FUN_142d95920(param_1,param_3);
    break;
  case 0x106:
    FUN_142aa2810(1);
    FUN_1406e9050(param_3,&local_628);
    if ((local_628 != (longlong *******)0x0) && (*(char *)local_628 != '\0')) {
      local_5f8 = (longlong ******)0x0;
      FUN_14019a260(&local_5f8,&local_628);
      local_618 = &local_5f8;
      FUN_14019a260(param_1 + 0x63e,&local_5f8);
      if (local_5f8 != (longlong ******)0x0) {
        FUN_14019f2c0(local_5f8 + -2);
      }
      FUN_142d18520(param_1,2);
      FUN_142d3c670(param_1);
    }
    if (local_628 != (longlong *******)0x0) {
      FUN_14019f2c0(local_628 + -2);
    }
    break;
  case 0x107:
    FUN_142d95ab0(param_1,param_3);
    break;
  case 0x108:
    FUN_1413ce880(param_3);
    break;
  case 0x109:
    FUN_142cf5f20(param_1,param_3);
    break;
  case 0x10a:
    if (DAT_143ad7758 != 0) {
      FUN_14227d660(DAT_143ad7758,param_3);
    }
    break;
  case 0x10b:
    FUN_142da1140(param_1,param_3);
    break;
  case 0x10c:
    FUN_142da1320(param_1,param_3);
    break;
  case 0x10d:
    thunk_FUN_142e31720(param_1,param_3);
    break;
  case 0x10e:
    FUN_142da37e0(param_1,param_3);
    break;
  case 0x10f:
    if (DAT_143ad20e8 != 0) {
      FUN_1424efd70(DAT_143ad20e8,0);
    }
    break;
  case 0x110:
    FUN_142cf4f20(param_1,param_3);
    break;
  case 0x111:
    FUN_142da9aa0(param_1,param_3);
    break;
  case 0x112:
    FUN_142d97010(param_1,param_3);
    break;
  case 0x113:
    FUN_142da4710(param_1,param_3);
    break;
  case 0x114:
    FUN_142da4820(param_1,param_3);
    break;
  case 0x115:
    FUN_142da5190(param_1,param_3);
    break;
  case 0x116:
    DAT_143ad7ab0 = FUN_1406e8c20(param_3);
    cVar10 = FUN_1406e8ae0(param_3);
    if (((cVar10 == '\0') || (iVar13 = FUN_142cf6d40(param_1,param_3), iVar13 != 0)) &&
       (FUN_14042cc90(param_3,param_1 + 0x671), lVar29 = DAT_143ad7ab8, DAT_143ad7ab8 != 0)) {
      local_618 = (longlong *******)param_1[0x670];
      if (local_618 != (longlong *******)0x0) {
        (*(code *)(*local_618)[1])();
      }
      FUN_1423219d0(lVar29,&local_618);
    }
    break;
  case 0x117:
    FUN_142cfacd0(param_1,param_3);
    break;
  case 0x118:
    if (DAT_143adb218 == 0) {
      FUN_142d24300();
    }
    FUN_1426181c0(DAT_143adb218,param_3);
    break;
  case 0x11a:
    if (DAT_143adb218 == 0) {
      FUN_142d24300();
    }
    FUN_1426181a0(DAT_143adb218,0);
    break;
  case 0x11b:
    FUN_142da51e0(param_1,param_3);
    break;
  case 0x11c:
    FUN_142cf5290(param_1,param_3);
    break;
  case 0x11d:
    FUN_142cf5410(param_1,param_3);
    break;
  case 0x11e:
    FUN_142cf5860(param_1,param_3);
    break;
  case 0x11f:
    FUN_142da52e0(param_1,param_3);
    break;
  case 0x120:
    FUN_142da5450(param_1,param_3);
    break;
  case 0x129:
    FUN_1406e9050(param_3,&local_618);
    local_628 = (longlong *******)0x0;
    FUN_14019a260(&local_628,&local_618);
    FUN_142cf07e0(param_1,&local_628);
    ppppppplVar26 = local_618;
    goto LAB_142cbd7c7;
  case 0x12a:
    FUN_142cea4b0(param_1,param_3);
    break;
  case 299:
    FUN_142cea890(param_1,param_3);
    break;
  case 300:
    FUN_142df3650(param_1,param_3);
    break;
  case 0x12d:
    FUN_142d95480(param_1,param_3);
    break;
  case 0x12e:
    FUN_142df3e40(param_1,param_3);
    break;
  case 0x12f:
    FUN_1406e8c20(param_3);
    break;
  case 0x130:
    FUN_142dc72d0(param_1,param_3);
    break;
  case 0x131:
    FUN_142dc7360(param_1,param_3);
    break;
  case 0x132:
    local_618 = (longlong *******)FUN_14019b780(&DAT_143ad68a0,0x158);
    if (local_618 == (longlong *******)0x0) {
      ppppplVar20 = (longlong *****)0x0;
    }
    else {
      ppppplVar20 = (longlong *****)FUN_140449c60(local_618,param_3);
    }
    if (((longlong)param_1[0x67b] - 1U < 999) ||
       (param_1[0x67b] == (longlong *****)0xffffffffffffffff)) {
      FUN_142e52ed0(0x447);
    }
    if (ppppplVar20 != (longlong *****)0x0) {
      if ((longlong ****)0xfffff < ppppplVar20[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      ppppplVar20[1] = (longlong ****)((longlong)ppppplVar20[1] + 1);
      UNLOCK();
    }
    ppppplVar37 = param_1[0x67b];
    param_1[0x67b] = ppppplVar20;
    goto LAB_142cbc59e;
  case 0x133:
    local_618 = (longlong *******)FUN_14019b780(&DAT_143ad68a0,0x138);
    if (local_618 == (longlong *******)0x0) {
      ppppplVar20 = (longlong *****)0x0;
    }
    else {
      ppppplVar20 = (longlong *****)FUN_140448c90(local_618,param_3);
    }
    if (((longlong)param_1[0x67d] - 1U < 999) ||
       (param_1[0x67d] == (longlong *****)0xffffffffffffffff)) {
      FUN_142e52ed0(0x447);
    }
    if (ppppplVar20 != (longlong *****)0x0) {
      if ((longlong ****)0xfffff < ppppplVar20[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      ppppplVar20[1] = (longlong ****)((longlong)ppppplVar20[1] + 1);
      UNLOCK();
    }
    ppppplVar37 = param_1[0x67d];
    param_1[0x67d] = ppppplVar20;
LAB_142cbc59e:
    param_1 = local_5f8;
    if (ppppplVar37 != (longlong *****)0x0) {
      if (0xffffe < (longlong)ppppplVar37[1] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      ppppplVar20 = ppppplVar37 + 1;
      pppplVar1 = *ppppplVar20;
      *ppppplVar20 = (longlong ****)((longlong)*ppppplVar20 + -1);
      UNLOCK();
      param_1 = local_5f8;
      if ((int)pppplVar1 == 1) {
        (*(code *)**ppppplVar37)(ppppplVar37,1);
        param_1 = local_5f8;
      }
    }
    break;
  case 0x134:
    iVar13 = FUN_1406e8c20(param_3);
    local_628 = (longlong *******)CONCAT44(local_628._4_4_,iVar13);
    local_5c8 = 0;
    if (0 < iVar13) {
      ppppppplVar26 = (longlong *******)(param_1 + 0x659);
      do {
        iVar13 = FUN_1406e8c20(param_3);
        pppppplVar3 = *ppppppplVar26;
        pppppplVar30 = (longlong ******)pppppplVar3[1];
        uStack_5f0 = 0;
        cVar10 = *(char *)((longlong)pppppplVar30 + 0x19);
        pppppplVar22 = pppppplVar3;
        local_5f8 = pppppplVar30;
        while (pppppplVar6 = pppppplVar30, cVar10 == '\0') {
          bVar5 = iVar13 <= *(int *)((longlong)pppppplVar6 + 0x1c);
          if (bVar5) {
            pppppplVar30 = (longlong ******)*pppppplVar6;
            pppppplVar22 = pppppplVar6;
          }
          else {
            pppppplVar30 = (longlong ******)pppppplVar6[2];
          }
          uStack_5f0 = (uint)bVar5;
          cVar10 = *(char *)((longlong)pppppplVar30 + 0x19);
          local_5f8 = pppppplVar6;
        }
        if ((*(char *)((longlong)pppppplVar22 + 0x19) != '\0') ||
           (iVar13 < *(int *)((longlong)pppppplVar22 + 0x1c))) {
          if (param_1[0x65a] == (longlong *****)0x555555555555555) {
                    /* WARNING: Subroutine does not return */
            FUN_14019f9d0();
          }
          uStack_610 = (longlong *******)0x0;
          local_618 = ppppppplVar26;
          puVar21 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x30);
          *(int *)((longlong)puVar21 + 0x1c) = iVar13;
          puVar21[4] = 0;
          puVar21[5] = 0;
          *(undefined8 *)((longlong)puVar21 + 0x24) = 0;
          *(undefined4 *)((longlong)puVar21 + 0x2c) = 0;
          *puVar21 = pppppplVar3;
          puVar21[1] = pppppplVar3;
          puVar21[2] = pppppplVar3;
          *(undefined2 *)(puVar21 + 3) = 0;
          uStack_610 = (longlong *******)0x0;
          pppppplVar22 = (longlong ******)FUN_142d37ea0(ppppppplVar26,&local_5f8,puVar21);
        }
        iVar13 = FUN_1406e8c20(param_3);
        *(int *)(pppppplVar22 + 4) = iVar13;
        iVar13 = FUN_1406e8c20(param_3);
        *(int *)((longlong)pppppplVar22 + 0x24) = iVar13;
        iVar13 = FUN_1406e8c20(param_3);
        *(int *)(pppppplVar22 + 5) = iVar13;
        bVar8 = FUN_1406e8ae0(param_3);
        *(uint *)((longlong)pppppplVar22 + 0x2c) = (uint)bVar8;
        local_5c8 = local_5c8 + 1;
      } while ((int)local_5c8 < (int)local_628);
    }
    break;
  case 0x135:
    FUN_1406e9170(param_3,param_1 + 0x64f,8);
    FUN_1406e9170(param_3,param_1 + 0x650,8);
    local_618 = (longlong *******)FUN_1408f6690();
    iVar13 = (*DAT_1432627e0)(&local_618,param_1 + 0x64f);
    if ((iVar13 < 0) || (iVar13 = (*DAT_1432627e0)(&local_618,param_1 + 0x650), 0 < iVar13)) {
      cVar10 = '\0';
    }
    else {
      cVar10 = '\x01';
    }
    *(char *)(param_1 + 0x64e) = cVar10;
    iVar13 = FUN_1429e3ef0();
    *(int *)((longlong)param_1 + 0x3274) = iVar13 + -10000;
    if (((*(char *)(param_1 + 0x64e) == '\0') && (DAT_143ad77e0 != 0)) &&
       (FUN_142bf3f70(), DAT_143ad77e0 != 0)) {
      (*(code *)**(undefined8 **)(DAT_143ad77e0 + 8))((undefined8 *)(DAT_143ad77e0 + 8),1);
    }
    break;
  case 0x136:
    FUN_142da6800(param_1,param_3);
    break;
  case 0x137:
    if (*(int *)(param_1 + 0x67e) == 0) {
      (*DAT_143ad5718)(0);
    }
    else {
      local_5f8 = (longlong ******)0x0;
      piVar23 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
      piVar23[1] = 0;
      *piVar23 = -1;
      local_5f8 = (longlong ******)(piVar23 + 4);
      piVar23[2] = 0;
      *(char *)local_5f8 = '\0';
      if (*piVar23 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar23[1] < 0) {
        FUN_142e54290(0x90,piVar23[1],0);
      }
      *piVar23 = 1;
      *(char *)local_5f8 = '\0';
      if (piVar23[1] + 1 < 1) {
        FUN_142e54290(0x9c,0);
      }
      piVar23[2] = 0;
      FUN_142cfb470(param_1,0,&local_5f8);
    }
    break;
  case 0x138:
    FUN_142d012e0(param_1,param_3);
    break;
  case 0x139:
    FUN_142cff620(param_1,param_3);
    break;
  case 0x13a:
    if (((*(int *)((longlong)param_1 + 0x226c) != 0) && (DAT_143adb238 == 0)) &&
       (local_618 = (longlong *******)FUN_14019b780(&DAT_143ad68a0,0x1390),
       local_618 != (longlong *******)0x0)) {
      FUN_142624620(local_618);
    }
    break;
  case 0x13b:
    bVar8 = FUN_1406e8ae0(param_3);
    if (bVar8 < 2) {
      *(uint *)(param_1 + ((ulonglong)bVar8 + 0x22d) * 3) =
           (uint)(*(int *)(param_1 + ((ulonglong)bVar8 + 0x22d) * 3) == 0);
    }
    break;
  case 0x13c:
    iVar13 = FUN_1406e8c20(param_3);
    uVar17 = FUN_1406e8c20(param_3);
    FUN_1406e9050(param_3,&local_5f8);
    iVar33 = FUN_14276df20(DAT_143aa8518);
    lVar29 = DAT_143aa8518;
    if (iVar33 == iVar13) {
      local_628 = (longlong *******)0x0;
      FUN_14019a260(&local_628,&local_5f8);
      FUN_142834ae0(lVar29,uVar17,&local_628);
    }
    else {
      lVar29 = FUN_1429b6c90(DAT_143ac1b90,iVar13);
      if (lVar29 != 0) {
        local_628 = (longlong *******)0x0;
        FUN_14019a260(&local_628,&local_5f8);
        FUN_142834ae0(lVar29,uVar17,&local_628);
      }
    }
    if (local_5f8 != (longlong ******)0x0) {
      FUN_14019f2c0(local_5f8 + -2);
    }
    break;
  case 0x13d:
    uVar17 = FUN_1406e8c20(param_3);
    FUN_1408a9e40(&local_5c0,0x12b0);
    FUN_1408a9e40(&local_5c8,0x12b1);
    lVar29 = DAT_143aa84a0;
    local_618 = (longlong *******)&local_628;
    local_628 = (longlong *******)0x0;
    FUN_14019a260(&local_628,&local_5c8);
    local_5d8 = (longlong *******)0x0;
    FUN_14019a260(&local_5d8,&local_5c0);
    local_678 = (longlong *******)&local_628;
    FUN_142d01960(lVar29,&local_618,0x18,&local_5d8);
    ppppppplVar26 = local_618;
    lVar29 = DAT_143aa84a0;
    if ((local_618 != (longlong *******)0x0) && (*(char *)local_618 != '\0')) {
      local_5b8 = (longlong *******)0x0;
      FUN_14019a260(&local_5b8,&local_618);
      FUN_142cc5f50(lVar29,uVar17,&local_5b8);
    }
    if (ppppppplVar26 != (longlong *******)0x0) {
      FUN_14019f2c0(ppppppplVar26 + -2);
    }
    if (CONCAT44(uStack_5c4,local_5c8) != 0) {
      FUN_14019f2c0(CONCAT44(uStack_5c4,local_5c8) + -0x10);
    }
    ppppppplVar26 =
         (longlong *******)CONCAT44(uStack_5bc,CONCAT22(uStack_5be,CONCAT11(uStack_5bf,local_5c0)));
    goto LAB_142cbd7c7;
  case 0x13e:
    cVar10 = FUN_1406e8ae0(param_3);
    if (cVar10 != '\x06') goto LAB_142cbd17c;
    uVar25 = FUN_1408a9e40(&local_618,0x106);
    FUN_1415eca30(uVar25,0xb);
    if (local_618 != (longlong *******)0x0) {
      FUN_14019f2c0(local_618 + -2);
    }
    FUN_142cc4430(param_1,0);
    break;
  case 0x13f:
    cVar10 = FUN_1406e8ae0(param_3);
    if (cVar10 != '\x01') goto LAB_142cbd17c;
    uVar25 = FUN_1408a9e40(&local_618,0x106);
    FUN_1415eca30(uVar25,0xb);
    if (local_618 != (longlong *******)0x0) {
      FUN_14019f2c0(local_618 + -2);
    }
    FUN_142cc4430(param_1,0);
    break;
  case 0x140:
    cVar10 = FUN_1406e8ae0(param_3);
    if (cVar10 != '\x02') goto LAB_142cbd17c;
    uVar25 = FUN_1408a9e40(&local_618,0x106);
    FUN_1415eca30(uVar25,0xb);
    if (local_618 != (longlong *******)0x0) {
      FUN_14019f2c0(local_618 + -2);
    }
    FUN_142cc4430(param_1,0);
    break;
  case 0x141:
    FUN_142dadbb0(param_1,param_3);
    break;
  case 0x142:
    cVar10 = FUN_1406e8ae0(param_3);
    if (cVar10 == '\x01') {
      uVar25 = FUN_1408a9e40(&local_618,0x106);
      FUN_1415eca30(uVar25,0xb);
      if (local_618 != (longlong *******)0x0) {
        FUN_14019f2c0(local_618 + -2);
      }
    }
LAB_142cbd17c:
    FUN_142cc4430(param_1,0);
    break;
  case 0x143:
    iVar13 = FUN_1406e8c20(param_3);
    iVar33 = FUN_1406e8c20(param_3);
    if ((-1 < iVar13) && (-1 < iVar33)) {
      *(int *)((longlong)param_1 + 0x34ac) = iVar13;
      *(int *)(param_1 + 0x696) = iVar33;
      uVar25 = FUN_141892840();
      FUN_141ba5f20(uVar25,0);
    }
    break;
  case 0x144:
    FUN_142da6d70(param_1,param_3);
    break;
  case 0x145:
    FUN_142da7550(param_1,param_3);
    break;
  case 0x146:
    if (DAT_143aa8518 != 0) {
      local_678 = (longlong *******)((ulonglong)local_678._4_4_ << 0x20);
      FUN_1428f4eb0(DAT_143aa8518,0x44b3,0,0);
    }
    break;
  case 0x147:
    FUN_142da58d0(param_1,param_3);
    break;
  case 0x148:
    FUN_142da59e0(param_1,param_3);
    break;
  case 0x149:
    FUN_142da97d0(param_1,param_3);
    break;
  case 0x14a:
    FUN_142d9e780(param_1,param_3);
    break;
  case 0x14b:
    FUN_142d4fd50(param_1,param_3);
    break;
  case 0x14c:
    FUN_142dea850(param_1,param_3);
    break;
  case 0x14d:
    local_618 = (longlong *******)param_1[0x46b];
    if (local_618 != (longlong *******)0x0) {
      uVar18 = 0;
      local_678 = (longlong *******)((ulonglong)local_678._4_4_ << 0x20);
      FUN_140304b20(local_618,local_578,param_3,0);
      pcVar24 = (char *)FUN_1402fa9a0(local_578,local_b8);
      do {
        if (*pcVar24 != '\0') {
          ppppplVar20 = param_1[0x46c];
          local_5d8 = (longlong *******)&local_5b8;
          local_5b0 = 0;
          iVar13 = *(int *)((longlong)param_1 + 0x3bb4);
          uVar18 = *(uint *)(param_1 + 0x776);
          ppppplVar37 = param_1[0x4b3];
          ppppplVar35 = param_1[0x4b2];
          uVar34 = (*(code *)(*param_1)[7])(param_1);
          uVar25 = (*(code *)(*local_5f8)[6])(local_5f8);
          uVar17 = FUN_141892890();
          local_660 = local_5f8 + 0x4d4;
          local_630 = 0;
          local_638 = 0;
          local_640 = 0;
          local_648 = (longlong *******)&local_5b8;
          local_678 = (longlong *******)uVar34;
          local_670 = ppppplVar35;
          local_668 = ppppplVar37;
          local_658 = uVar18;
          local_650 = iVar13;
          FUN_14085b3b0(ppppplVar20,uVar17,local_618,uVar25);
          param_3 = local_628;
          param_1 = local_5f8;
          break;
        }
        uVar18 = uVar18 + 1;
        pcVar24 = pcVar24 + 1;
      } while (uVar18 < 100);
      if (DAT_143aa8518 != 0) {
        local_678 = (longlong *******)((ulonglong)local_678 & 0xffffffff00000000);
        FUN_1428f4eb0(DAT_143aa8518,0x4324,1);
      }
      local_670 = (longlong *****)0x0;
      local_678 = (longlong *******)((ulonglong)local_678 & 0xffffffff00000000);
      FUN_142cbefd0(param_1,0,0,0);
      if (DAT_143ac8e08 != 0) {
        FUN_142208060();
      }
    }
    break;
  case 0x14e:
    FUN_142d4fe10(param_1,param_3);
    break;
  case 0x14f:
    uVar17 = FUN_1406e8c20(param_3);
    FUN_1406e8ae0(param_3);
    FUN_1406e8c20(param_3);
    FUN_1406e8c20(param_3);
    if (DAT_143ad85e8 != 0) {
      FUN_142449f90(DAT_143ad85e8,uVar17);
    }
    break;
  case 0x150:
    FUN_142da9c70(param_1,param_3);
    break;
  case 0x151:
    FUN_142da9d00(param_1,param_3);
    break;
  case 0x152:
    FUN_142da9d40(param_1,param_3);
    break;
  case 0x153:
    FUN_142da9db0(param_1,param_3);
    break;
  case 0x154:
    plVar27 = (longlong *)FUN_140f62a20();
    puVar21 = (undefined8 *)*plVar27;
    cVar10 = *(char *)((longlong)puVar21[1] + 0x19);
    puVar4 = puVar21;
    puVar32 = (undefined8 *)puVar21[1];
    while (cVar10 == '\0') {
      if (*(int *)(puVar32 + 4) < 3) {
        puVar28 = (undefined8 *)puVar32[2];
        puVar32 = puVar4;
      }
      else {
        puVar28 = (undefined8 *)*puVar32;
      }
      puVar4 = puVar32;
      puVar32 = puVar28;
      cVar10 = *(char *)((longlong)puVar28 + 0x19);
    }
    if (((*(char *)((longlong)puVar4 + 0x19) == '\0') && (*(int *)(puVar4 + 4) < 4)) &&
       (puVar4 != puVar21)) {
      puVar21 = (undefined8 *)puVar4[5];
      param_1 = local_5f8;
      for (puVar4 = (undefined8 *)*puVar21; local_5f8 = param_1, puVar4 != puVar21;
          puVar4 = (undefined8 *)*puVar4) {
        plVar27 = (longlong *)puVar4[2];
        if (plVar27 != (longlong *)0x0) {
          (**(code **)(*plVar27 + 8))(plVar27,&local_5c0);
        }
        param_1 = local_5f8;
      }
    }
    break;
  case 0x155:
    FUN_142d945d0(param_1,param_3);
    break;
  case 0x156:
    FUN_141174cc0(param_1,param_3);
    break;
  case 0x157:
    FUN_142d96dd0(param_1,param_3);
    break;
  case 0x158:
    FUN_142daa080(param_1,param_3);
    break;
  case 0x159:
    FUN_142d23bd0(param_3);
    break;
  case 0x15a:
    FUN_142d96e60(param_1,param_3);
    break;
  case 0x15b:
    if (*(int *)(DAT_143ac87a0 + 0x18c) != 0) {
      iVar13 = FUN_1406e8c20(param_3);
      if (iVar13 < 1) {
        FUN_141f6c630();
      }
      else {
        ppppppplVar26 = (longlong *******)FUN_14019b780(&DAT_143ad68a0,0x10);
        local_618 = ppppppplVar26;
        if (ppppppplVar26 != (longlong *******)0x0) {
          *ppppppplVar26 = (longlong ******)&PTR_FUN_143494910;
          iVar33 = (*DAT_143262db0)();
          *(int *)(ppppppplVar26 + 1) = iVar33 + iVar13;
          FUN_140904cc0(ppppppplVar26);
        }
      }
    }
    break;
  case 0x15c:
    FUN_140d2d4c0(param_1,param_3);
    break;
  case 0x15d:
    FUN_140d2d4d0(param_1,param_3);
    break;
  case 0x15e:
    FUN_140d2d510(param_1,param_3);
    break;
  case 0x15f:
    FUN_142da3f30(param_1,param_3);
    break;
  case 0x160:
    FUN_142da42b0(param_1,param_3);
    break;
  case 0x161:
    FUN_142d4a7f0(param_1,param_3);
    break;
  case 0x162:
    iVar13 = FUN_1406e8c20(param_3);
    if (DAT_143ac87a0 == 0) goto LAB_142cbbc25;
    if ((*(int *)(DAT_143ac87a0 + 0xf0) == 0) && (*(int *)(DAT_143ac87a0 + 0xc0) == 0)) {
      iVar33 = *(int *)(DAT_143ac87a0 + 0xbc) * *(int *)(DAT_143ac87a0 + 0xec);
      if (iVar33 < 1) {
        iVar33 = 0;
      }
      else {
        iVar33 = ((iVar33 / 0x14 + 1) * 100) / 0x14;
      }
    }
    else {
      iVar33 = -2;
    }
    if (iVar33 != iVar13) {
      FUN_1406ed520(local_508,0x175);
      FUN_1406ed9d0(local_508,iVar33);
      FUN_1415d01c0(local_508);
      FUN_1406ed610(local_508);
    }
    break;
  case 0x163:
    FUN_142daabd0(param_1,param_3);
    break;
  case 0x164:
    FUN_142daad10(param_1,param_3);
    break;
  case 0x165:
    FUN_142dad160(param_1,param_3);
    break;
  case 0x166:
    FUN_142dd9720(param_1,param_3);
    break;
  case 0x167:
    FUN_142dd9790(param_1,param_3);
    break;
  case 0x168:
    FUN_142dc5e20(param_1,param_3);
    break;
  case 0x169:
    ppppplVar20 = (longlong *****)FUN_1406e8fb0(param_3);
    param_1[0x6f9] = ppppplVar20;
    break;
  case 0x16a:
    local_618 = (longlong *******)0x0;
    uStack_610 = (longlong *******)0x0;
    FUN_142d2e450(param_1 + 0x6ff,&local_618);
    if (uStack_610 != (longlong *******)0x0) {
      FUN_1402abcb0();
    }
    uStack_610 = (longlong *******)FUN_14019b780(&DAT_143ad68a0,0x28);
    if (uStack_610 == (longlong *******)0x0) {
      uStack_610 = (longlong *******)0x0;
    }
    else {
      *uStack_610 = (longlong ******)0x0;
      uStack_610[1] = (longlong ******)0x0;
      *(int *)(uStack_610 + 1) = 1;
      *(int *)((longlong)uStack_610 + 0xc) = 1;
      *uStack_610 = (longlong ******)&PTR_LAB_143495568;
      uStack_610[2] = (longlong ******)0x0;
      uStack_610[3] = (longlong ******)0x0;
      uStack_610[2] = (longlong ******)0x0;
      uStack_610[3] = (longlong ******)0x0;
      uStack_610[4] = (longlong ******)0x0;
    }
    local_618 = uStack_610 + 2;
    FUN_142d2e450(param_1 + 0x6ff,&local_618);
    if (uStack_610 != (longlong *******)0x0) {
      FUN_1402abcb0();
    }
    cVar10 = FUN_1406e8ae0(param_3);
    if (cVar10 != '\0') {
      FUN_1404b52c0(param_1[0x6ff],param_3);
    }
    cVar10 = FUN_1406e8ae0(param_3);
    if (cVar10 != '\0') {
      FUN_1404b52c0(param_1[0x6ff],param_3);
    }
    cVar10 = FUN_1406e8ae0(param_3);
    if (cVar10 != '\0') {
      FUN_1404b52c0(param_1[0x6ff],param_3);
    }
    break;
  case 0x16b:
    iVar13 = (*DAT_143262db0)();
    if ((DAT_143ade7cc == 0) || (cVar10 = FUN_1408fcaa0(DAT_143ade7cc,1000,iVar13), cVar10 != '\0'))
    {
      DAT_143ade7cc = iVar13;
      FUN_142e15850(3,0xf,0x1002);
    }
    break;
  case 0x16d:
    FUN_142dc6f00(param_1,param_3);
    break;
  case 0x16e:
    FUN_142dc70c0(param_1,param_3);
    break;
  case 0x16f:
    FUN_142dcb0d0(param_1,param_3);
    break;
  case 0x170:
    FUN_1406e9170(param_3,&local_628,4);
    FUN_141f1bc00((ulonglong)local_628 & 0xffffffff);
    break;
  case 0x171:
    FUN_142dc9e90(param_1,param_3);
    break;
  case 0x172:
    FUN_142db6e10(param_1,param_3);
    break;
  case 0x173:
    FUN_142db7390(param_1,param_3);
    break;
  case 0x174:
    FUN_142d0a6e0(param_1,1);
    break;
  case 0x177:
    FUN_1408f5fa0(param_3);
    break;
  case 0x178:
    FUN_142130730(param_3);
    break;
  case 0x179:
    uVar17 = FUN_1406e8c20(param_3);
    switch(uVar17) {
    case 0:
    case 1:
    case 2:
    case 3:
    case 4:
      FUN_1406e8c20(param_3);
      FUN_1406e8ae0(param_3);
      break;
    case 7:
      FUN_1406e8c20(param_3);
      cVar10 = FUN_1406e8ae0(param_3);
      if (cVar10 != '\0') {
        FUN_1406e9170(param_3,&local_628,4);
      }
    }
    break;
  case 0x17a:
    FUN_142cb6360(param_1,param_3);
    break;
  case 0x17b:
    FUN_142d4ffa0(param_1,param_3);
    break;
  case 0x17c:
    bVar8 = FUN_1406e8ae0(param_3);
    local_618 = DAT_1434936b0;
    FUN_1406e9170(param_3,&local_618,8);
    *(uint *)((longlong)param_1 + 0x383c) = (uint)bVar8;
    param_1[0x708] = (longlong *****)local_618;
    local_5f8 = (longlong ******)FUN_1408f6690();
    iVar13 = (*DAT_1432627e0)(param_1 + 0x708,&local_5f8);
    *(uint *)((longlong)param_1 + 0x383c) = (uint)(0 < iVar13);
    break;
  case 0x17d:
    FUN_142d161a0(param_1,param_3);
    break;
  case 0x17e:
    if ((DAT_143addff8 != 0) && (uVar18 = FUN_1406e8c20(param_3), 0 < (int)uVar18)) {
      uVar34 = (ulonglong)uVar18;
      do {
        FUN_1406e9050(param_3,&local_628);
        iVar13 = FUN_1406e8c20(param_3);
        lVar29 = DAT_143addff8;
        local_5f8 = (longlong ******)0x0;
        if (iVar13 == 0) {
          FUN_14019a260(&local_5f8,&local_628);
          FUN_14114c9c0(lVar29,&local_5f8);
        }
        else {
          FUN_14019a260(&local_5f8,&local_628);
          FUN_14114d5c0(lVar29,&local_5f8);
        }
        if (local_628 != (longlong *******)0x0) {
          FUN_14019f2c0(local_628 + -2);
        }
        uVar34 = uVar34 - 1;
      } while (uVar34 != 0);
    }
    break;
  case 0x17f:
    if (DAT_143addff8 != 0) {
      FUN_14114d820();
    }
    break;
  case 0x180:
    FUN_140289a70(param_3);
    break;
  case 0x181:
    FUN_140352f40(param_3);
    break;
  case 0x182:
    iVar13 = FUN_1406e8c20(param_3);
    cVar9 = FUN_1406e8ae0(param_3);
    ppppplVar20 = param_1[0x77a];
    cVar10 = *(char *)((longlong)ppppplVar20[1] + 0x19);
    ppppplVar37 = ppppplVar20;
    ppppplVar35 = (longlong *****)ppppplVar20[1];
    while (cVar10 == '\0') {
      if (*(int *)(ppppplVar35 + 4) < iVar13) {
        ppppplVar31 = (longlong *****)ppppplVar35[2];
        ppppplVar35 = ppppplVar37;
      }
      else {
        ppppplVar31 = (longlong *****)*ppppplVar35;
      }
      ppppplVar37 = ppppplVar35;
      ppppplVar35 = ppppplVar31;
      cVar10 = *(char *)((longlong)ppppplVar31 + 0x19);
    }
    if (((*(char *)((longlong)ppppplVar37 + 0x19) == '\0') && (*(int *)(ppppplVar37 + 4) <= iVar13))
       && (ppppplVar37 != ppppplVar20)) {
      *(undefined4 *)(ppppplVar37 + 5) = 0;
      *(uint *)((longlong)ppppplVar37 + 0x2c) = (uint)(cVar9 != '\0');
    }
    break;
  case 0x183:
    if (DAT_143ac3748 != 0) {
      iVar13 = FUN_1406e8c20(param_3);
      if (iVar13 == 0) {
        FUN_140fcd220(lVar7);
      }
      else if (iVar13 == 1) {
        FUN_140fcd4a0(lVar7);
      }
      else if (iVar13 == 2) {
        FUN_140fd10c0(lVar7);
        FUN_140fd1200(lVar7);
      }
      else if (iVar13 == 3) {
        FUN_140fd1200(lVar7);
      }
    }
    break;
  case 0x185:
    FUN_142d9e760(param_1,param_3);
    break;
  case 0x186:
    uVar17 = FUN_1406e8c20(param_3);
    uVar16 = FUN_1406e8c20(param_3);
    uVar19 = FUN_1406e8c20(param_3);
    FUN_1411fda50(uVar17,uVar16,uVar19);
    param_1 = local_5f8;
    break;
  case 0x187:
    *(int *)(param_1 + 0x6c9) = 0;
    break;
  case 0x189:
    iVar13 = FUN_1406e8c20(param_3);
    *(int *)(param_1 + 0x805) = iVar13;
    if (DAT_143abfe00 != 0) {
      FUN_141d36720();
    }
    break;
  case 0x18a:
    FUN_142d4ff70(param_1,param_3);
    break;
  case 0x18b:
    uVar17 = FUN_1406e8c20(param_3);
    uVar16 = FUN_1406e8c20(param_3);
    local_618 = (longlong *******)0x0;
    uStack_610 = (longlong *******)0x0;
    local_608 = 0;
    FUN_140233d50(param_3,&local_618);
    ppppppplVar26 = uStack_610;
    ppppppplVar36 = local_618;
    if (local_618 == uStack_610) {
      local_5f8 = (longlong ******)0x0;
      FUN_14019ba10(&local_5f8,"BossFirstClearRecord Is Empty ( bossID[%d] worldID[%d]",uVar17,
                    uVar16);
      uVar17 = FUN_1415eca30(&local_5f8,0xb);
      if (local_5f8 != (longlong ******)0x0) {
        uVar17 = FUN_14019f2c0(local_5f8 + -2);
      }
    }
    else {
      do {
        uVar17 = FUN_1415eca30(ppppppplVar36,0xb);
        ppppppplVar36 = ppppppplVar36 + 1;
      } while (ppppppplVar36 != ppppppplVar26);
    }
    ppppppplVar36 = uStack_610;
    ppppppplVar26 = local_618;
    if (local_618 != (longlong *******)0x0) {
      for (; ppppppplVar26 != ppppppplVar36; ppppppplVar26 = ppppppplVar26 + 1) {
        if (*ppppppplVar26 != (longlong ******)0x0) {
          uVar17 = FUN_14019f2c0(*ppppppplVar26 + -2);
        }
      }
      uVar34 = local_608 - (longlong)local_618 & 0xfffffffffffffff8;
      ppppppplVar26 = local_618;
      if (0xfff < uVar34) {
        ppppppplVar26 = (longlong *******)local_618[-1];
        if ((char *)0x1f < (char *)((longlong)local_618 + (-8 - (longlong)ppppppplVar26))) {
                    /* WARNING: Subroutine does not return */
          FUN_142f04804(uVar17,uVar34 + 0x27);
        }
      }
      thunk_FUN_140205820(ppppppplVar26);
    }
    break;
  case 0x18c:
    FUN_142cd94e0(param_1,param_3);
    break;
  case 0x18d:
    FUN_1408a9e40(&local_5b8,0xe55);
    local_628 = (longlong *******)0x0;
    FUN_14019a260(&local_628,&local_5b8);
    local_618 = (longlong *******)&local_628;
    if (((local_628 != (longlong *******)0x0) && (*(char *)local_628 != '\0')) &&
       (ppppplVar20 = param_1[0x540], ppppplVar20 != (longlong *****)0x0)) {
      local_5d8 = (longlong *******)0x0;
      FUN_14019a260(&local_5d8,&local_628);
      local_658 = 0;
      local_660 = (longlong ******)CONCAT44(local_660._4_4_,5000);
      local_668 = (longlong *****)((ulonglong)local_668 & 0xffffffff00000000);
      local_670 = (longlong *****)((ulonglong)local_670 & 0xffffffff00000000);
      local_678 = (longlong *******)CONCAT44(local_678._4_4_,0x14);
      FUN_141e2ca60(ppppplVar20,&local_5d8,3,0x14);
    }
    if (local_628 != (longlong *******)0x0) {
      FUN_14019f2c0(local_628 + -2);
    }
    FUN_1415eca30(&local_5b8,0xb);
    ppppppplVar26 = local_5b8;
    goto LAB_142cbd7c7;
  case 0x18e:
    FUN_142128dc0(param_3);
    break;
  case 399:
    FUN_1406e8ae0(param_3);
    goto switchD_142cbab2d_caseD_b8;
  case 400:
    FUN_142db50f0(param_1,param_3);
    break;
  case 0x191:
    FUN_142d43be0(param_1,param_3);
    break;
  case 0x192:
    FUN_142d43cf0(param_1,param_3);
    break;
  case 0x193:
    FUN_142d1ba10(param_1,param_3);
    break;
  case 0x194:
    FUN_142dd4f10(param_1,param_3);
    break;
  case 0x195:
    FUN_142ddaa60(param_1,param_3);
    break;
  case 0x196:
    FUN_142ddb8c0(param_1,param_3);
    break;
  case 0x197:
    FUN_142267f60(param_3);
    break;
  case 0x198:
    FUN_14113d920(param_3);
    break;
  case 0x199:
    local_618 = (longlong *******)0x0;
    FUN_14019bd40(&local_618,0,0);
    FUN_14019c870(&local_618,0);
    uStack_610._0_2_ = (ushort)uStack_610 & 0xff00;
    uStack_610 = (longlong *******)CONCAT44(0xffffffff,CONCAT22(0xffff,(ushort)uStack_610));
    local_608 = 0;
    FUN_1404bf3c0(&local_618,param_3);
    FUN_142d18520(param_1,5,&local_618);
    ppppppplVar26 = local_618;
LAB_142cbd7c7:
    param_1 = local_5f8;
    if (ppppppplVar26 != (longlong *******)0x0) {
      FUN_14019f2c0(ppppppplVar26 + -2);
      param_1 = local_5f8;
    }
    break;
  case 0x19a:
    FUN_142d9ed20(param_1,param_3);
    break;
  case 0x19b:
    FUN_142d97560(param_1,param_3);
    break;
  case 0x19c:
    FUN_142db7b80(param_1,param_3);
    break;
  case 0x19e:
    lVar29 = FUN_142d24480(param_3);
    if (lVar29 != 0) {
      FUN_1410dcc50(lVar29);
    }
    break;
  case 0x19f:
    FUN_142d968f0(param_1,param_3);
    break;
  case 0x275:
    cVar10 = FUN_1406e8ae0(param_3);
    FUN_1406e8c20(param_3);
    if ((lVar29 != 0) && (cVar10 == '\0')) {
      FUN_1406ed520(local_508,0x17e);
      local_628 = (longlong *******)((ulonglong)local_628 & 0xffffffff00000000);
      FUN_1406ede20(local_508,&local_628,4);
      local_628 = (longlong *******)CONCAT44(local_628._4_4_,2);
      FUN_1406ede20(local_508,&local_628,4);
      FUN_1406ed9d0(local_508,*(int *)((longlong)param_1 + 0x232c));
      FUN_1415d01c0(local_508);
      FUN_1406ed610(local_508);
    }
    break;
  case 0x39a:
    FUN_1406e9170(param_3,&local_618,8);
    iVar13 = FUN_1406e8c20(param_3);
    param_1[0x831] = (longlong *****)local_618;
    *(int *)(param_1 + 0x832) = iVar13;
  }
switchD_142cbab2d_caseD_71:
  param_2 = local_5a8;
  if (DAT_143ac87a0 != 0) {
    FUN_1415f0d70();
    param_2 = local_5a8;
  }
LAB_142cbbc25:
  if (DAT_143ad1850 != 0) {
    switch(param_2) {
    case 0x121:
      FUN_141a3d3b0(DAT_143ad1850,param_3);
      break;
    case 0x122:
      FUN_141a3cd90(DAT_143ad1850,param_3);
      break;
    case 0x123:
      FUN_141a3d5d0(DAT_143ad1850,param_3);
      break;
    case 0x124:
      FUN_141a3e790(DAT_143ad1850,param_3);
      break;
    case 0x125:
      FUN_141a3e7b0(DAT_143ad1850,param_3);
      break;
    case 0x126:
      FUN_141a3e1c0(DAT_143ad1850,param_3);
    }
  }
  local_628 = (longlong *******)CONCAT44(local_628._4_4_,param_2);
  ppppplVar20 = param_1[0x74d];
  if (ppppplVar20 == param_1[0x74e]) {
    FUN_1408341b0(param_1 + 0x74c,ppppplVar20,&local_628);
  }
  else {
    *(undefined4 *)ppppplVar20 = param_2;
    param_1[0x74d] = (longlong *****)((longlong)param_1[0x74d] + 4);
  }
  ppppplVar20 = param_1[0x74c];
  if (0x14 < (ulonglong)((longlong)param_1[0x74d] - (longlong)ppppplVar20 >> 2)) {
    FUN_142ef7ba0(ppppplVar20,(longlong)ppppplVar20 + 4,
                  (longlong)param_1[0x74d] - ((longlong)ppppplVar20 + 4));
    param_1[0x74d] = (longlong *****)((longlong)param_1[0x74d] + -4);
  }
  return;
switchD_142cbab2d_caseD_b8:
  *(int *)(param_1 + 0x466) = 0;
  iVar13 = FUN_1429e3ef0();
  *(int *)((longlong)param_1 + 0x2334) = iVar13;
  uVar17 = FUN_1429e3ef0();
  FUN_142e54b20(uVar17);
  uVar17 = FUN_1429e3ef0();
  FUN_142e54f40(uVar17);
  goto switchD_142cbab2d_caseD_71;
}
