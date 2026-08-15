
//===========================================================
// FUN_1415e7090 @ 1415e7090   (2255 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1415e7090(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  char cVar3;
  short sVar4;
  ushort uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  undefined4 uVar10;
  ulonglong uVar11;
  undefined8 uVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  ulonglong uVar17;
  undefined8 *puVar18;
  bool bVar19;
  undefined1 auStack_688 [32];
  undefined4 local_668;
  undefined4 local_660;
  undefined8 local_658;
  undefined8 local_650;
  undefined1 local_648 [16];
  undefined **local_638;
  undefined8 *local_630;
  int local_628;
  ulonglong local_620;
  longlong local_618;
  longlong lStack_610;
  longlong local_608;
  undefined8 local_600;
  undefined8 local_5f8;
  undefined8 *local_5f0;
  undefined4 local_5e8;
  undefined8 local_5d8;
  int local_5d0 [4];
  undefined8 *local_5c0;
  undefined8 *local_5b0;
  longlong local_5a8;
  longlong local_5a0;
  undefined1 local_598 [16];
  undefined1 local_588 [48];
  undefined1 local_558 [208];
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_688;
  puVar18 = (undefined8 *)0x0;
  *(undefined4 *)(param_1 + 0x14c) = 0;
  local_5d8 = param_1;
  iVar6 = (*DAT_143262268)(PTR_s_Data_wz_143a87bc8);
  if (iVar6 == -1) goto LAB_1415e747e;
  local_638 = &PTR_LAB_143272c88;
  local_630 = (undefined8 *)0x0;
  local_620 = 0;
  local_618 = _DAT_143303fa0;
  lStack_610 = _UNK_143303fa8;
  local_608 = 0;
  local_600 = 0;
  local_5f8 = 0;
  local_5f0 = (undefined8 *)0x0;
  local_5e8 = 0;
  local_650 = 0;
  local_658 = 0;
  local_660 = 0x80000000;
  local_668 = 1;
  FUN_1408e8c70(&local_638,PTR_s_Data_wz_143a87bc8,3,0x80);
  if ((code *)local_638[0xf] == FUN_1401bd1b0) {
    uVar11 = FUN_1401bd1b0();
  }
  else {
    uVar11 = (*(code *)local_638[0xf])(&local_638);
  }
  uVar15 = uVar11 & 0xffffffff;
  if (uVar11 != uVar15) {
    FUN_1401bb8c0(local_598,1,"ZFileStream::GetLength(): need 64bit interface");
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_598,(ThrowInfo *)&DAT_143a3b118);
  }
  if ((int)uVar11 != 0) {
    uVar17 = DAT_143ace3b8 - DAT_143ace3b0;
    if (uVar15 < uVar17) {
      DAT_143ace3b8 = DAT_143ace3b0 + uVar15;
    }
    else if (uVar17 < uVar15) {
      if ((ulonglong)(DAT_143ace3c0 - DAT_143ace3b0) < uVar15) {
        FUN_1408b62f0(&DAT_143ace3b0,uVar15,local_648);
      }
      else {
        lVar2 = (uVar15 - uVar17) + DAT_143ace3b8;
        FUN_142ef8250(DAT_143ace3b8,0,uVar15 - uVar17);
        DAT_143ace3b8 = lVar2;
      }
    }
    FUN_1401bd3a0(&local_638,DAT_143ace3b0,uVar11 & 0xffffffff);
    FUN_1401bca50(&local_638);
    FUN_14075aa50(local_558);
    FUN_14075aaf0(local_558,DAT_143ace3b0,uVar11 & 0xffffffff);
    FUN_14075adf0(local_558);
    FUN_14075b1b0(local_558,param_1 + 0x14c,4);
    FUN_14075aa90(local_558);
  }
  local_638 = &PTR_LAB_143272c88;
  LOCK();
  bVar19 = local_630 == (undefined8 *)0x0;
  if (bVar19) {
    local_630 = Self;
  }
  UNLOCK();
  if (bVar19) {
LAB_1415e72e5:
    local_628 = 1;
  }
  else {
    if (local_630 != Self) {
      while( true ) {
        LOCK();
        bVar19 = local_630 == (undefined8 *)0x0;
        if (bVar19) {
          local_630 = Self;
        }
        UNLOCK();
        if (bVar19) goto LAB_1415e72e5;
        if (local_630 == Self) break;
        (*DAT_143262828)(0);
      }
    }
    local_628 = local_628 + 1;
  }
  LOCK();
  bVar19 = local_630 == (undefined8 *)0x0;
  if (bVar19) {
    local_630 = Self;
  }
  UNLOCK();
  if (bVar19) {
LAB_1415e7335:
    local_628 = 1;
  }
  else {
    if (local_630 != Self) {
      while( true ) {
        LOCK();
        bVar19 = local_630 == (undefined8 *)0x0;
        if (bVar19) {
          local_630 = Self;
        }
        UNLOCK();
        if (bVar19) goto LAB_1415e7335;
        if (local_630 == Self) break;
        (*DAT_143262828)(0);
      }
    }
    local_628 = local_628 + 1;
  }
  iVar6 = 1;
  puVar13 = puVar18;
  if (local_608 != 0) {
    iVar6 = (*DAT_143ad5558)();
    iVar7 = iVar6;
    if (iVar6 != 0) {
      local_608 = 0;
      local_5d8 = local_620;
      iVar7 = (*DAT_143ad55b0)(local_618,local_620 & 0xffffffff,(longlong)&local_5d8 + 4,0);
      local_5d8 = CONCAT44(local_5d8._4_4_,iVar7);
      puVar13 = (undefined8 *)0x0;
      if ((iVar7 != -1) || (iVar8 = (*DAT_143ad5628)(), iVar7 = 0, iVar8 == 0)) goto LAB_1415e73b4;
    }
    iVar6 = iVar7;
    uVar9 = (*DAT_143ad5628)();
    puVar13 = (undefined8 *)(ulonglong)uVar9;
  }
LAB_1415e73b4:
  if (lStack_610 != 0) {
    iVar7 = (*DAT_143ad5400)();
    if ((iVar7 == 0) || (iVar6 == 0)) {
      iVar6 = 0;
      if ((int)puVar13 == 0) {
        uVar9 = (*DAT_143ad5628)();
        puVar13 = (undefined8 *)(ulonglong)uVar9;
        iVar6 = 0;
      }
    }
    else {
      iVar6 = 1;
    }
    lStack_610 = 0;
  }
  (*DAT_143ad5630)(puVar13);
  local_628 = local_628 + -1;
  if (local_628 == 0) {
    local_630 = puVar18;
  }
  iVar7 = (*DAT_143ad5628)();
  if (local_618 != -1) {
    if ((((local_618 != 0) && (iVar8 = (*DAT_143ad5400)(), iVar8 == 0)) || (iVar6 == 0)) &&
       (iVar7 == 0)) {
      iVar7 = (*DAT_143ad5628)();
    }
    local_618 = -1;
  }
  local_5e8 = 0;
  (*DAT_143ad5630)(iVar7);
  local_628 = local_628 + -1;
  if (local_628 == 0) {
    local_630 = puVar18;
  }
  if (local_5f0 != (undefined8 *)0x0) {
    (**(code **)*local_5f0)(local_5f0,1);
  }
LAB_1415e747e:
  FUN_1406ed520(local_488,0xa1);
  FUN_1406ed9d0(local_488,*(undefined4 *)(param_1 + 0x14c));
  FUN_1415d3990(param_1,local_488);
  cVar3 = *(char *)(param_1 + 0x150);
  do {
    if (cVar3 != '\0') {
LAB_1415e790c:
      FUN_1406ed610(local_488);
      return;
    }
    while( true ) {
      while( true ) {
        uVar12 = FUN_14019b780(&DAT_143ad68a0,0x5b4);
        local_5a8 = FUN_14019b780(&DAT_143ad68a0,0x40);
        puVar13 = puVar18;
        if (local_5a8 != 0) {
          puVar13 = (undefined8 *)FUN_1415e7ab0(local_5a8,0x5b4,uVar12,0);
        }
        local_5c0 = puVar13;
        if (puVar13 != (undefined8 *)0x0) {
          if (0xfffff < (ulonglong)puVar13[1]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          puVar13[1] = puVar13[1] + 1;
          UNLOCK();
        }
        puVar16 = local_5c0;
        if (puVar13 == (undefined8 *)0x0) {
          FUN_142e52ed0(0x431,0);
        }
        iVar6 = (*DAT_143262e40)(*(undefined8 *)(param_1 + 0x20),puVar13[5],
                                 *(undefined4 *)(puVar13 + 4));
        if (iVar6 != -1) break;
        iVar6 = (*DAT_143ad5b60)();
        if (iVar6 != 0x2733) {
          if (0xffffe < puVar13[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar1 = puVar13 + 1;
          lVar2 = *plVar1;
          *plVar1 = *plVar1 + -1;
          UNLOCK();
          if ((int)lVar2 == 1) {
            (**(code **)*puVar13)(puVar13,1);
          }
          local_5c0 = (undefined8 *)0x0;
          goto LAB_1415e77de;
        }
        if (0xffffe < puVar13[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = puVar13 + 1;
        lVar2 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar2 == 1) {
          (**(code **)*puVar13)(puVar13,1);
        }
      }
      local_5a0 = FUN_14019b780(&DAT_143ad68a0,0x40);
      puVar14 = puVar18;
      if (local_5a0 != 0) {
        puVar14 = (undefined8 *)FUN_1415e7ab0(local_5a0,iVar6,puVar13[5],puVar13);
      }
      local_5b0 = puVar14;
      if (puVar14 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar14[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar14[1] = puVar14[1] + 1;
        UNLOCK();
        puVar16 = local_5c0;
      }
      puVar13 = (undefined8 *)FUN_1415e17d0(param_1 + 0x70,*(undefined8 *)(param_1 + 0x80),0);
      lVar2 = *(longlong *)(param_1 + 0x80);
      if (lVar2 == 0) {
        *(undefined8 **)(param_1 + 0x78) = puVar13;
      }
      else {
        puVar14 = puVar18;
        if (puVar13 != (undefined8 *)0x0) {
          puVar14 = puVar13 + -5;
        }
        if (*(longlong *)(lVar2 + -0x20) - 1U < 0x10000) {
          FUN_142e52ed0(0x33e);
        }
        *(undefined8 **)(lVar2 + -0x20) = puVar14;
        puVar13 = puVar18;
        if (puVar14 != (undefined8 *)0x0) {
          puVar13 = puVar14 + 5;
        }
      }
      puVar14 = local_5b0;
      *(undefined8 **)(param_1 + 0x80) = puVar13;
      if (local_5b0 != (undefined8 *)0x0) {
        if (0xffffe < local_5b0[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = puVar14 + 1;
        lVar2 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar2 == 1) {
          (**(code **)*local_5b0)(local_5b0,1);
        }
        local_5b0 = (undefined8 *)0x0;
        puVar16 = local_5c0;
      }
      if (iVar6 < 0x5b4) break;
      if (puVar16 != (undefined8 *)0x0) {
        if (0xffffe < puVar16[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = puVar16 + 1;
        lVar2 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar2 == 1) {
          (**(code **)*local_5c0)(local_5c0,1);
        }
      }
    }
    if (puVar16 != (undefined8 *)0x0) {
      if (0xffffe < puVar16[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = puVar16 + 1;
      lVar2 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar2 == 1) {
        (**(code **)*local_5c0)(local_5c0,1);
      }
      local_5c0 = (undefined8 *)0x0;
    }
LAB_1415e77de:
    if (*(int *)(param_1 + 0x74) != 0) {
      iVar6 = *(int *)(param_1 + 0x48);
      lVar2 = *(longlong *)(param_1 + 0x78);
      iVar7 = FUN_1406e9530(param_1 + 0xb0,lVar2,local_5d0);
      if (*(longlong *)(lVar2 + 8) == 0) {
        FUN_1415e4330(param_1 + 0x70,*(undefined8 *)(param_1 + 0x78));
      }
      if (((0 < iVar7) && (local_5d0[0] < 1)) &&
         ((sVar4 = FUN_1406e97e0(param_1 + 0xb0,*(undefined4 *)(param_1 + 0xec)), sVar4 != -2 ||
          (uVar5 = FUN_1406e9810(param_1 + 0xb0), 0x20000 < uVar5)))) {
        FUN_1415d33c0(param_1,0,0);
        goto LAB_1415e790c;
      }
      if (iVar7 == 2) {
        FUN_1406e88d0(local_588,param_1 + 0xb0);
        FUN_1406e99e0(local_588,*(undefined4 *)(param_1 + 0xec),2 - (uint)(iVar6 != 0));
        uVar10 = FUN_140c78630(param_1 + 0xec,4,0);
        *(undefined4 *)(param_1 + 0xec) = uVar10;
        FUN_1415d60e0(param_1,local_588);
        if (*(char *)(param_1 + 0x150) != '\0') {
          FUN_1406e89e0(local_588);
          goto LAB_1415e78fe;
        }
        FUN_1406e89e0(local_588);
      }
      goto LAB_1415e77de;
    }
LAB_1415e78fe:
    cVar3 = *(char *)(param_1 + 0x150);
  } while( true );
}



//===========================================================
// FUN_1406e97e0 @ 1406e97e0   (11 bytes)
//===========================================================

ushort FUN_1406e97e0(longlong param_1,undefined4 param_2)

{
  return (ushort)((uint)param_2 >> 0x10) ^ *(ushort *)(param_1 + 0x1c);
}



//===========================================================
// FUN_1415d9cd0 @ 1415d9cd0   (77 bytes)
//===========================================================

void FUN_1415d9cd0(longlong param_1)

{
  undefined4 local_28 [2];
  longlong local_20;
  undefined1 local_18 [24];
  
  FUN_1408460c0(param_1 + 0xf8);
  local_20 = param_1 + 0xf8;
  local_28[0] = 0x3f0;
  FUN_140319730(local_20,local_18,local_28);
  return;
}


