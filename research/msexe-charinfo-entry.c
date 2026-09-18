
//===========================================================
// FUN_1411991f0 @ 1411991f0   (243 bytes)
//===========================================================

undefined8 * FUN_1411991f0(undefined8 *param_1,longlong param_2)

{
  longlong lVar1;
  
  FUN_14170fd10();
  *param_1 = &PTR_FUN_14336e7b0;
  param_1[1] = &PTR_LAB_14336e840;
  param_1[3] = &PTR_FUN_14336e918;
  FUN_142644810(param_1 + 0xf);
  *(undefined1 *)(param_1 + 0x234) = 0;
  *param_1 = &PTR_FUN_143389e40;
  param_1[1] = &PTR_LAB_143389ed0;
  param_1[3] = &PTR_FUN_143389fa8;
  param_1[0x235] = 0;
  lVar1 = *(longlong *)(param_2 + 8);
  param_1[0x237] = lVar1;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 8) = *(longlong *)(lVar1 + 8) + 1;
    UNLOCK();
  }
  *(undefined4 *)(param_1 + 0x238) = 0;
  param_1[0x239] = 0;
  *(undefined4 *)(param_1 + 0x23a) = 0;
  FUN_1401c1fb0(param_1 + 0x235,&DAT_143acaa40);
  FUN_1401abd80(param_2);
  return param_1;
}



//===========================================================
// FUN_1401abd80 @ 1401abd80   (106 bytes)
//===========================================================

void FUN_1401abd80(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 8) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 8);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar2 == 1) && (puVar3 = *(undefined8 **)(param_1 + 8), puVar3 != (undefined8 *)0x0))
    {
      (**(code **)*puVar3)(puVar3,1);
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_14170fd10 @ 14170fd10   (194 bytes)
//===========================================================

undefined8 * FUN_14170fd10(undefined8 *param_1)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined1 local_28 [32];
  
  *param_1 = &PTR_LAB_143296d40;
  uVar2 = FUN_141d5d8c0(local_28,&DAT_143271f04,&DAT_143271f04,0x1c,0);
  FUN_141d5f4b0(param_1 + 1,uVar2);
  param_1[6] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  *param_1 = &PTR_FUN_1433ceb40;
  param_1[1] = &PTR_LAB_1433cebc0;
  param_1[3] = &PTR_FUN_1433cec98;
  *(undefined4 *)(param_1 + 7) = 0xffffffff;
  uVar1 = FUN_1429e3ef0();
  *(undefined4 *)((longlong)param_1 + 0x3c) = uVar1;
  param_1[8] = 0;
  param_1[10] = 0;
  *(undefined4 *)(param_1 + 0xb) = 1;
  *(undefined4 *)((longlong)param_1 + 0x5c) = 1;
  *(undefined4 *)(param_1 + 0xc) = 1;
  param_1[0xd] = 0;
  *(undefined4 *)(param_1 + 0xe) = 0;
  return param_1;
}



//===========================================================
// FUN_142644810 @ 142644810   (1337 bytes)
//===========================================================

undefined8 * FUN_142644810(undefined8 *param_1)

{
  int *piVar1;
  int iVar2;
  char cVar3;
  longlong *plVar4;
  longlong *plVar5;
  longlong *plVar6;
  longlong lVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong *plVar10;
  longlong lVar11;
  longlong lVar12;
  undefined4 *puVar13;
  undefined8 uStack_60;
  longlong *local_48;
  ulonglong uStack_40;
  
  param_1[3] = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  *param_1 = &PTR_FUN_14327fce8;
  param_1[4] = 0;
  param_1[5] = 0;
  param_1[6] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x11] = 0;
  param_1[0x12] = 0;
  lVar7 = FUN_14019b780(&DAT_143ad68a0,0x28);
  *(longlong *)lVar7 = lVar7;
  *(longlong *)(lVar7 + 8) = lVar7;
  *(longlong *)(lVar7 + 0x10) = lVar7;
  *(undefined2 *)(lVar7 + 0x18) = 0x101;
  param_1[0x11] = lVar7;
  param_1[0x13] = 0;
  lVar7 = 0x22;
  _eh_vector_constructor_iterator_
            (param_1 + 0x15,0x20,0x22,(_func_void_void_ptr *)&LAB_1426d4b80,FUN_140331880);
  _eh_vector_constructor_iterator_
            (param_1 + 0x9f,0x20,0x22,(_func_void_void_ptr *)&LAB_1426d4b80,FUN_140331880);
  plVar8 = param_1 + 0x127;
  _eh_vector_constructor_iterator_
            (plVar8,0x18,0x22,(_func_void_void_ptr *)&LAB_1426d5100,
             (_func_void_void_ptr *)&LAB_140331a20);
  _eh_vector_constructor_iterator_
            (param_1 + 400,0x20,0x22,(_func_void_void_ptr *)&LAB_1426d4b80,FUN_140331880);
  *(undefined4 *)(param_1 + 0x219) = 0;
  param_1[0x21b] = 0;
  param_1[0x21c] = 0;
  param_1[0x21d] = 0;
  param_1[0x21e] = 0;
  param_1[0x21f] = 0;
  *(undefined4 *)(param_1 + 0x220) = 0;
  param_1[0x221] = 0;
  param_1[0x222] = 0;
  param_1[0x223] = 0;
  param_1[0x224] = 0x271a;
  FUN_1426dc3a0(&DAT_143adb3b0);
  if (DAT_143adb3b8 != 0) {
    LOCK();
    *(int *)(DAT_143adb3b8 + 8) = *(int *)(DAT_143adb3b8 + 8) + 1;
    UNLOCK();
  }
  lVar12 = DAT_143adb3b8;
  param_1[5] = DAT_143adb3b0;
  plVar9 = (longlong *)param_1[6];
  param_1[6] = lVar12;
  if (plVar9 != (longlong *)0x0) {
    LOCK();
    plVar10 = plVar9 + 1;
    lVar12 = *plVar10;
    *(int *)plVar10 = (int)*plVar10 + -1;
    UNLOCK();
    if ((int)lVar12 == 1) {
      (**(code **)*plVar9)(plVar9);
      LOCK();
      piVar1 = (int *)((longlong)plVar9 + 0xc);
      iVar2 = *piVar1;
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (iVar2 == 1) {
        (**(code **)(*plVar9 + 8))(plVar9);
      }
    }
  }
  param_1[7] = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  if ((longlong *)param_1[9] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[9] + 0x10))();
  }
  param_1[9] = 0;
  if ((longlong *)param_1[10] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[10] + 0x10))();
  }
  param_1[10] = 0;
  if ((longlong *)param_1[0xd] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xd] + 0x10))();
  }
  param_1[0xd] = 0;
  if ((longlong *)param_1[0xe] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xe] + 0x10))();
  }
  param_1[0xe] = 0;
  if ((longlong *)param_1[0xb] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xb] + 0x10))();
  }
  param_1[0xb] = 0;
  if ((longlong *)param_1[0xc] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xc] + 0x10))();
  }
  param_1[0xc] = 0;
  *(undefined4 *)(param_1 + 0x14) = 0;
  param_1[0x18e] = 0;
  *(undefined4 *)(param_1 + 399) = 0;
  *(undefined4 *)(param_1 + 0x218) = 0;
  *(undefined4 *)((longlong)param_1 + 0x10c4) = 0xffffffff;
  *(undefined4 *)(param_1 + 0x220) = 0;
  *(undefined1 *)(param_1 + 0x9d) = 0;
  *(undefined8 *)((longlong)param_1 + 0x4ec) = 0;
  puVar13 = (undefined4 *)((longlong)param_1 + 0xc4);
  do {
    *(undefined8 *)(puVar13 + -7) = 0;
    *puVar13 = 0;
    *(undefined8 *)(puVar13 + 0x2ef) = 0;
    puVar13[0x2f6] = 0;
    lVar12 = plVar8[1];
    lVar11 = *plVar8;
    if (lVar11 != lVar12) {
      do {
        if (*(longlong *)(lVar11 + 8) != 0) {
          FUN_14019f2c0(*(longlong *)(lVar11 + 8) + -0x10);
        }
        lVar11 = lVar11 + 0x10;
      } while (lVar11 != lVar12);
      lVar11 = *plVar8;
    }
    plVar8[1] = lVar11;
    *(undefined8 *)(puVar13 + 0x10d) = 0;
    puVar13[0x114] = 0;
    plVar8 = plVar8 + 3;
    puVar13 = puVar13 + 8;
    lVar7 = lVar7 + -1;
  } while (lVar7 != 0);
  lVar7 = param_1[0x21c];
  lVar12 = param_1[0x21d];
  if (lVar7 != lVar12) {
    do {
      uStack_60 = *(undefined8 **)(lVar7 + 8);
      if (uStack_60 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)uStack_60[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        uStack_60[1] = uStack_60[1] + 1;
        UNLOCK();
      }
      if (uStack_60 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_142645170(uStack_60);
      if (uStack_60 != (undefined8 *)0x0) {
        if (0xffffe < uStack_60[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar8 = uStack_60 + 1;
        lVar11 = *plVar8;
        *plVar8 = *plVar8 + -1;
        UNLOCK();
        if (((int)lVar11 == 1) && (uStack_60 != (undefined8 *)0x0)) {
          (**(code **)*uStack_60)(uStack_60,1);
        }
        uStack_60 = (undefined8 *)0x0;
      }
      lVar7 = lVar7 + 0x10;
    } while (lVar7 != lVar12);
    lVar7 = param_1[0x21d];
    lVar12 = param_1[0x21c];
    if (lVar12 != lVar7) {
      do {
        FUN_140336390(lVar12);
        lVar12 = lVar12 + 0x10;
      } while (lVar12 != lVar7);
      lVar12 = param_1[0x21c];
    }
    param_1[0x21d] = lVar12;
  }
  if ((DAT_143adb3c8 == (longlong *)0x0) &&
     (plVar8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x10), plVar8 != (longlong *)0x0)) {
    *plVar8 = 0;
    plVar8[1] = 0;
    DAT_143adb3c8 = plVar8;
    *plVar8 = 0;
    plVar8[1] = 0;
    lVar7 = FUN_14019b780(&DAT_143ad68a0,0x28);
    *(longlong *)lVar7 = lVar7;
    *(longlong *)(lVar7 + 8) = lVar7;
    *(longlong *)(lVar7 + 0x10) = lVar7;
    *(undefined2 *)(lVar7 + 0x18) = 0x101;
    *plVar8 = lVar7;
  }
  plVar6 = DAT_143adb3c8;
  plVar8 = (longlong *)*DAT_143adb3c8;
  plVar9 = (longlong *)plVar8[1];
  uStack_60 = (undefined8 *)((ulonglong)uStack_60 & 0xffffffff00000000);
  cVar3 = *(char *)((longlong)plVar9 + 0x19);
  plVar10 = plVar8;
  plVar5 = plVar9;
  while (plVar4 = plVar9, cVar3 == '\0') {
    uStack_60._4_4_ = (uint)((ulonglong)uStack_60 >> 0x20);
    if ((undefined8 *)plVar4[4] < param_1) {
      uStack_60 = (undefined8 *)((ulonglong)uStack_60._4_4_ << 0x20);
      plVar9 = (longlong *)plVar4[2];
    }
    else {
      uStack_60 = (undefined8 *)CONCAT44(uStack_60._4_4_,1);
      plVar9 = (longlong *)*plVar4;
      plVar10 = plVar4;
    }
    cVar3 = *(char *)((longlong)plVar9 + 0x19);
    plVar5 = plVar4;
  }
  if ((*(char *)((longlong)plVar10 + 0x19) != '\0') || (param_1 < (undefined8 *)plVar10[4])) {
    if (DAT_143adb3c8[1] == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    local_48 = DAT_143adb3c8;
    uStack_40 = 0;
    plVar9 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
    plVar9[4] = (longlong)param_1;
    *plVar9 = (longlong)plVar8;
    plVar9[1] = (longlong)plVar8;
    plVar9[2] = (longlong)plVar8;
    *(undefined2 *)(plVar9 + 3) = 0;
    local_48 = plVar5;
    uStack_40 = (ulonglong)uStack_60;
    FUN_1426e2550(plVar6,&local_48,plVar9);
  }
  return param_1;
}



//===========================================================
// FUN_142e541f0 @ 142e541f0   (146 bytes)
//===========================================================

void FUN_142e541f0(undefined4 param_1,undefined8 param_2)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18 [2];
  longlong local_res20;
  
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    FUN_142e5cd30("LogCallStack4",&DAT_1434997dc,local_res18,&DAT_1434997f8,local_res8,"Info1",
                  &local_res10,&local_res20);
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1401c1fb0 @ 1401c1fb0   (518 bytes)
//===========================================================

longlong * FUN_1401c1fb0(longlong *param_1,longlong *param_2)

{
  void *_Buf1;
  void *_Buf2;
  longlong lVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  ulonglong uVar8;
  int iVar9;
  
  if (param_1 == param_2) {
    return param_1;
  }
  _Buf1 = (void *)*param_1;
  piVar7 = (int *)0x0;
  iVar9 = 0;
  iVar2 = iVar9;
  if (_Buf1 != (void *)0x0) {
    iVar2 = (int)((ulonglong)(longlong)*(int *)((longlong)_Buf1 + -8) >> 1);
  }
  _Buf2 = (void *)*param_2;
  iVar4 = iVar9;
  if (_Buf2 != (void *)0x0) {
    iVar4 = (int)((ulonglong)(longlong)*(int *)((longlong)_Buf2 + -8) >> 1);
  }
  if ((((iVar2 == iVar4) && (iVar2 != 0)) && (_Buf1 != (void *)0x0)) &&
     ((_Buf2 != (void *)0x0 && (iVar2 = memcmp(_Buf1,_Buf2,(longlong)iVar2 * 2), iVar2 == 0)))) {
    return param_1;
  }
  piVar5 = (int *)((longlong)_Buf2 + -0x10);
  if (_Buf2 == (void *)0x0) {
    piVar5 = piVar7;
  }
  if (piVar5 == (int *)0x0) {
    if (_Buf1 == (void *)0x0) {
      return param_1;
    }
    FUN_1401bebb0((longlong)_Buf1 + -0x10);
    *param_1 = 0;
    return param_1;
  }
  if (*piVar5 != -1) {
    if (*piVar5 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar5 = *piVar5 + 1;
    UNLOCK();
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
    }
    *param_1 = (longlong)(piVar5 + 4);
    return param_1;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  lVar1 = *param_2;
  piVar5 = piVar7;
  if (lVar1 == 0) goto LAB_1401c2137;
  uVar8 = 0xffffffffffffffff;
  piVar6 = (int *)0xffffffffffffffff;
  do {
    piVar6 = (int *)((longlong)piVar6 + 1);
  } while (*(short *)(lVar1 + (longlong)piVar6 * 2) != 0);
  iVar2 = (int)piVar6;
  if (0 < iVar2) {
    iVar9 = iVar2;
  }
  piVar3 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
  piVar3[1] = iVar9;
  *piVar3 = -1;
  piVar5 = piVar3 + 4;
  piVar3[2] = 0;
  *(undefined2 *)piVar5 = 0;
  FUN_142ef7ba0(piVar5,lVar1,(longlong)iVar2 * 2);
  if (*piVar3 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar2 == -1) || (iVar2 <= piVar3[1])) {
    *piVar3 = 1;
    if (iVar2 != -1) goto LAB_1401c2110;
    if (piVar5 != (int *)0x0) {
      do {
        uVar8 = uVar8 + 1;
      } while (*(short *)((longlong)piVar5 + uVar8 * 2) != 0);
      piVar7 = (int *)(uVar8 & 0xffffffff);
    }
  }
  else {
    FUN_142e54290(0x90,piVar3[1],(ulonglong)piVar6 & 0xffffffff);
    *piVar3 = 1;
LAB_1401c2110:
    *(undefined2 *)((longlong)piVar5 + (longlong)iVar2 * 2) = 0;
    piVar7 = piVar6;
  }
  iVar2 = (int)piVar7;
  if ((iVar2 < 0) || (piVar3[1] + 1 <= iVar2)) {
    FUN_142e54290(0x9c,(ulonglong)piVar7 & 0xffffffff);
  }
  piVar3[2] = iVar2 * 2;
LAB_1401c2137:
  if (*param_1 != 0) {
    FUN_1401bebb0(*param_1 + -0x10);
  }
  *param_1 = (longlong)piVar5;
  return param_1;
}



//===========================================================
// FUN_141d5d8c0 @ 141d5d8c0   (22 bytes)
//===========================================================

undefined8 *
FUN_141d5d8c0(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined4 param_4,
             undefined4 param_5)

{
  *(undefined4 *)((longlong)param_1 + 0x14) = param_5;
  *param_1 = param_2;
  param_1[1] = param_3;
  *(undefined4 *)(param_1 + 2) = param_4;
  return param_1;
}



//===========================================================
// FUN_1429e3ef0 @ 1429e3ef0   (12 bytes)
//===========================================================

void FUN_1429e3ef0(void)

{
  FUN_142c4a030(DAT_143ac1898);
  return;
}



//===========================================================
// FUN_141d5f4b0 @ 141d5f4b0   (82 bytes)
//===========================================================

undefined8 * FUN_141d5f4b0(undefined8 *param_1)

{
  int iVar1;
  longlong lVar2;
  
  *(undefined4 *)(param_1 + 1) = 0;
  *param_1 = &PTR_FUN_143409768;
  do {
    do {
      iVar1 = FUN_140738830();
    } while (iVar1 == 0);
    lVar2 = FUN_141d5f870(iVar1);
  } while (lVar2 != 0);
  FUN_141d5f6e0(iVar1,param_1);
  *(int *)(param_1 + 1) = iVar1;
  return param_1;
}



//===========================================================
// FUN_142e52ed0 @ 142e52ed0   (4890 bytes)
//===========================================================

void FUN_142e52ed0(undefined4 param_1,undefined8 param_2)

{
  char *pcVar1;
  longlong lVar2;
  char cVar3;
  undefined8 uVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  int *piVar8;
  int iVar9;
  int iVar10;
  int *piVar11;
  int *piVar12;
  int *piVar13;
  int iVar14;
  int iVar15;
  longlong lVar16;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18;
  undefined4 local_res20 [2];
  char *local_a8;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50 [2];
  
  piVar13 = (int *)0x0;
  iVar9 = 0;
  local_res18 = 0;
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar3 = FUN_142e559e0();
  if (cVar3 == '\0') {
    return;
  }
  FUN_140194c60(&local_78);
  local_res20[0] = FUN_14091a3e0(&local_78);
  uVar4 = FUN_142a1d8a0(local_50);
  local_a8 = (char *)0x0;
  local_res18 = 1;
  FUN_1408bc980(&local_98,uVar4);
  lVar2 = local_98;
  pcVar1 = local_a8;
  local_res18 = 7;
  piVar12 = (int *)0xffffffffffffffff;
  if (local_98 != 0) {
    iVar10 = *(int *)(local_98 + -8);
    piVar8 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar11 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e530bb;
      if (*local_a8 == '\0') {
        if ((local_a8 == (char *)0x0) ||
           (piVar11 = (int *)(local_a8 + -0x10), piVar11 == (int *)0x0)) {
LAB_142e530bb:
          if (iVar15 < iVar10) {
            iVar15 = iVar10;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
          puVar5[1] = iVar15;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          puVar5[2] = 0;
          *local_a8 = '\0';
          if (piVar11 != (int *)0x0) {
            FUN_14019f2c0(piVar11);
          }
        }
        else {
          if ((1 < *piVar11) || (*(int *)(local_a8 + -0xc) < iVar10)) {
            iVar15 = *(int *)(local_a8 + -8);
            goto LAB_142e530bb;
          }
          if (*piVar11 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar11 = -1;
        }
        FUN_142ef7ba0(local_a8,lVar2,piVar8);
        pcVar1 = local_a8;
        if (*(int *)(local_a8 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
          if (iVar10 != -1) goto LAB_142e53148;
          piVar8 = piVar13;
          if (pcVar1 != (char *)0x0) {
            do {
              piVar12 = (int *)((longlong)piVar12 + 1);
              piVar8 = piVar12;
            } while (pcVar1[(longlong)piVar12] != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
LAB_142e53148:
          *(char *)((longlong)piVar8 + (longlong)local_a8) = '\0';
        }
        iVar10 = (int)piVar8;
        if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
        }
        *(int *)(pcVar1 + -8) = iVar10;
        goto LAB_142e53173;
      }
      iVar15 = *(int *)(local_a8 + -8);
      for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
      }
      piVar12 = (int *)(local_a8 + -0x10);
      iVar14 = iVar9;
      if (piVar12 == (int *)0x0) {
LAB_142e52fbf:
        if (iVar14 < iVar7) {
          iVar14 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        puVar5[1] = iVar14;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        if (piVar12 == (int *)0x0) {
          puVar5[2] = 0;
          *local_a8 = '\0';
        }
        else {
          iVar7 = *(int *)(pcVar1 + -8) + 1;
          if (iVar14 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar14 + 1);
            iVar7 = iVar14 + 1;
          }
          FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
          puVar5[2] = *(undefined4 *)(pcVar1 + -8);
          local_a8[iVar14] = '\0';
          FUN_14019f2c0(piVar12);
        }
      }
      else {
        if ((1 < *piVar12) || (*(int *)(local_a8 + -0xc) < iVar7)) {
          iVar14 = *(int *)(local_a8 + -8);
          goto LAB_142e52fbf;
        }
        if (*piVar12 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar12 = -1;
      }
      iVar7 = iVar9;
      if (local_a8 != (char *)0x0) {
        iVar7 = *(int *)(local_a8 + -8);
      }
      FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar8);
      FUN_14019c870(&local_a8,iVar15 + iVar10);
    }
  }
LAB_142e53173:
  piVar12 = (int *)0xffffffffffffffff;
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  local_70 = 0;
  uVar4 = FUN_14019ba10(&local_70,&DAT_143272338,"LogCallStack3");
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x27;
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0x1f;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar8 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar11 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e53377;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar12 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar12 == (int *)0x0) {
LAB_142e5327f:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar12 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar12);
          }
        }
        else {
          if ((1 < *piVar12) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e5327f;
          }
          if (*piVar12 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar12 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar8);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e5342f;
      }
      if ((local_a8 == (char *)0x0) || (piVar11 = (int *)(local_a8 + -0x10), piVar11 == (int *)0x0))
      {
LAB_142e53377:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar11 != (int *)0x0) {
          FUN_14019f2c0(piVar11);
        }
      }
      else {
        if ((1 < *piVar11) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e53377;
        }
        if (*piVar11 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar11 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar8);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e53407;
        piVar8 = piVar13;
        if (pcVar1 != (char *)0x0) {
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
            piVar8 = piVar12;
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e53407:
        *(char *)((longlong)piVar8 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar8;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e5342f:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  local_68 = 0;
  uVar4 = FUN_14019ba10(&local_68,&DAT_143272338,&DAT_1434997dc);
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x11f;
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0xdf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e5363e;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar8 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar8 == (int *)0x0) {
LAB_142e5353f:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar8 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e5353f;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e536fd;
      }
      if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e5363e:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar8 != (int *)0x0) {
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e5363e;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar12);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e536d5;
        piVar12 = piVar13;
        if (pcVar1 != (char *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e536d5:
        *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar12;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e536fd:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc140(&local_90,local_res20);
  lVar2 = local_90;
  pcVar1 = local_a8;
  local_res18 = 0x6df;
  if (local_90 != 0) {
    iVar10 = *(int *)(local_90 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e538ca;
      if (*local_a8 == '\0') {
        if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0))
        {
LAB_142e538ca:
          if (iVar15 < iVar10) {
            iVar15 = iVar10;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
          puVar5[1] = iVar15;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          puVar5[2] = 0;
          *local_a8 = '\0';
          if (piVar8 != (int *)0x0) {
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
            iVar15 = *(int *)(local_a8 + -8);
            goto LAB_142e538ca;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        FUN_142ef7ba0(local_a8,lVar2,piVar12);
        pcVar1 = local_a8;
        if (*(int *)(local_a8 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
          if (iVar10 != -1) goto LAB_142e5395e;
          piVar12 = piVar13;
          if (pcVar1 != (char *)0x0) {
            piVar12 = (int *)0xffffffffffffffff;
            do {
              piVar12 = (int *)((longlong)piVar12 + 1);
            } while (pcVar1[(longlong)piVar12] != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
LAB_142e5395e:
          *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
        }
        iVar10 = (int)piVar12;
        if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
        }
        *(int *)(pcVar1 + -8) = iVar10;
        goto LAB_142e53989;
      }
      iVar15 = *(int *)(local_a8 + -8);
      for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
      }
      piVar8 = (int *)(local_a8 + -0x10);
      iVar14 = iVar9;
      if (piVar8 == (int *)0x0) {
LAB_142e537cf:
        if (iVar14 < iVar7) {
          iVar14 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        puVar5[1] = iVar14;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        if (piVar8 == (int *)0x0) {
          puVar5[2] = 0;
          *local_a8 = '\0';
        }
        else {
          iVar7 = *(int *)(pcVar1 + -8) + 1;
          if (iVar14 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar14 + 1);
            iVar7 = iVar14 + 1;
          }
          FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
          puVar5[2] = *(undefined4 *)(pcVar1 + -8);
          local_a8[iVar14] = '\0';
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
          iVar14 = *(int *)(local_a8 + -8);
          goto LAB_142e537cf;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      iVar7 = iVar9;
      if (local_a8 != (char *)0x0) {
        iVar7 = *(int *)(local_a8 + -8);
      }
      FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
      FUN_14019c870(&local_a8,iVar15 + iVar10);
    }
  }
LAB_142e53989:
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  local_60 = 0;
  uVar4 = FUN_14019ba10(&local_60,&DAT_143272338,&DAT_1434997f8);
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x26df;
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0x1edf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e53b90;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar8 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar8 == (int *)0x0) {
LAB_142e53a91:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar8 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e53a91;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e53c4f;
      }
      if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e53b90:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar8 != (int *)0x0) {
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e53b90;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar12);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e53c27;
        piVar12 = piVar13;
        if (pcVar1 != (char *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e53c27:
        *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar12;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e53c4f:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc020(&local_88,local_res8);
  lVar2 = local_88;
  pcVar1 = local_a8;
  local_res18 = 0xdedf;
  if (local_88 == 0) goto LAB_142e53edb;
  iVar10 = *(int *)(local_88 + -8);
  piVar12 = (int *)(longlong)iVar10;
  if (iVar10 == 0) goto LAB_142e53edb;
  piVar8 = piVar13;
  iVar15 = iVar9;
  if (local_a8 == (char *)0x0) goto LAB_142e53e1c;
  if (*local_a8 == '\0') {
    if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e53e1c:
      if (iVar15 < iVar10) {
        iVar15 = iVar10;
      }
      puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
      puVar5[1] = iVar15;
      *puVar5 = 0xffffffff;
      local_a8 = (char *)(puVar5 + 4);
      puVar5[2] = 0;
      *local_a8 = '\0';
      if (piVar8 != (int *)0x0) {
        FUN_14019f2c0(piVar8);
      }
    }
    else {
      if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
        iVar15 = *(int *)(local_a8 + -8);
        goto LAB_142e53e1c;
      }
      if (*piVar8 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar8 = -1;
    }
    piVar8 = (int *)0xffffffffffffffff;
    FUN_142ef7ba0(local_a8,lVar2,piVar12);
    pcVar1 = local_a8;
    if (*(int *)(local_a8 + -0x10) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
      if (iVar10 != -1) goto LAB_142e53eb0;
      if (pcVar1 != (char *)0x0) {
        do {
          piVar13 = (int *)((longlong)piVar8 + 1);
          piVar8 = piVar13;
        } while (pcVar1[(longlong)piVar13] != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
LAB_142e53eb0:
      *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      piVar13 = piVar12;
    }
    iVar10 = (int)piVar13;
    if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
      FUN_142e54290(0x9c,(ulonglong)piVar13 & 0xffffffff);
    }
    *(int *)(pcVar1 + -8) = iVar10;
    goto LAB_142e53edb;
  }
  iVar15 = *(int *)(local_a8 + -8);
  for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
  }
  piVar13 = (int *)(local_a8 + -0x10);
  iVar14 = iVar9;
  if (piVar13 == (int *)0x0) {
LAB_142e53d21:
    if (iVar14 < iVar7) {
      iVar14 = iVar7;
    }
    puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
    puVar5[1] = iVar14;
    *puVar5 = 0xffffffff;
    local_a8 = (char *)(puVar5 + 4);
    if (piVar13 == (int *)0x0) {
      puVar5[2] = 0;
      *local_a8 = '\0';
    }
    else {
      iVar7 = *(int *)(pcVar1 + -8) + 1;
      if (iVar14 + 1 < iVar7) {
        FUN_142e54290(0x5c,iVar7,iVar14 + 1);
        iVar7 = iVar14 + 1;
      }
      FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
      puVar5[2] = *(undefined4 *)(pcVar1 + -8);
      local_a8[iVar14] = '\0';
      FUN_14019f2c0(piVar13);
    }
  }
  else {
    if ((1 < *piVar13) || (*(int *)(local_a8 + -0xc) < iVar7)) {
      iVar14 = *(int *)(local_a8 + -8);
      goto LAB_142e53d21;
    }
    if (*piVar13 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar13 = -1;
  }
  iVar7 = iVar9;
  if (local_a8 != (char *)0x0) {
    iVar7 = *(int *)(local_a8 + -8);
  }
  FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
  FUN_14019c870(&local_a8,iVar15 + iVar10);
LAB_142e53edb:
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  local_58 = 0;
  uVar4 = FUN_14019ba10(&local_58,&DAT_143272338,"Info1");
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x4dedf;
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  lVar2 = local_a0;
  local_res18 = 0x3dedf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        iVar15 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar15 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar15 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc3a0(&local_80,&local_res10);
  lVar2 = local_80;
  local_res18 = 0x1bdedf;
  if (local_80 != 0) {
    iVar10 = *(int *)(local_80 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        iVar15 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar15 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar15 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  FUN_1408bc980(&local_a0,&local_78);
  lVar2 = local_a0;
  local_res18 = 0x7bdedf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        if (local_a8 != (char *)0x0) {
          iVar9 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar9 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  FUN_142a1ec10(&local_a8);
  if (local_a8 != (char *)0x0) {
    FUN_14019f2c0(local_a8 + -0x10);
  }
  if (local_50[0] != 0) {
    FUN_14019f2c0(local_50[0] + -0x10);
  }
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  return;
}



//===========================================================
// FUN_14019f9d0 @ 14019f9d0   (16 bytes)
//===========================================================

void FUN_14019f9d0(void)

{
                    /* WARNING: Subroutine does not return */
  FUN_142ed3068("map/set too long");
}



//===========================================================
// FUN_14019b780 @ 14019b780   (374 bytes)
//===========================================================

void FUN_14019b780(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  int iVar6;
  undefined8 uVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar7 = 0x80;
  if (param_2 < 0x21) {
    uVar10 = (uint)(0x10 < param_2);
LAB_14019b7de:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x10;
      goto LAB_14019b825;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x20;
      goto LAB_14019b825;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b825;
    }
  }
  else {
    if (0x40 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x81) {
        uVar10 = 3;
      }
      goto LAB_14019b7de;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x40;
LAB_14019b825:
  lVar11 = (longlong)(int)uVar10;
  lVar8 = lVar11 * 0x10 + param_1;
  plVar1 = (longlong *)(lVar8 + 0x28);
  LOCK();
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar4 == 0) {
LAB_14019b889:
    *(undefined4 *)(lVar8 + 0x30) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar4 = *plVar1;
      if (lVar4 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar4 == 0) goto LAB_14019b889;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar7,iVar6);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar6;
    puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  }
  piVar2 = (int *)(param_1 + 0x14 + lVar11 * 4);
  *piVar2 = *piVar2 + 1;
  *(undefined8 *)(param_1 + 0x68 + lVar11 * 8) = *puVar5;
  *piVar9 = *piVar9 + -1;
  if (*piVar9 == 0) {
    *plVar1 = 0;
  }
  return;
}



//===========================================================
// FUN_1426dc3a0 @ 1426dc3a0   (233 bytes)
//===========================================================

void FUN_1426dc3a0(undefined8 *param_1)

{
  longlong *plVar1;
  int *piVar2;
  int iVar3;
  longlong lVar4;
  longlong *plVar5;
  undefined1 uVar6;
  longlong *local_18;
  longlong *local_10;
  
  if (*(char *)(param_1 + 2) == '\0') {
    local_10 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x858);
    if (local_10 == (longlong *)0x0) {
      local_10 = (longlong *)0x0;
    }
    else {
      *local_10 = 0;
      local_10[1] = 0;
      *(undefined4 *)(local_10 + 1) = 1;
      *(undefined4 *)((longlong)local_10 + 0xc) = 1;
      *local_10 = (longlong)&PTR_FUN_143479c88;
      FUN_142ef8250(local_10 + 2,0,0x848);
      FUN_1426d4b90(local_10 + 2);
    }
    local_18 = local_10 + 2;
    FUN_1426d6260(param_1,&local_18);
    plVar5 = local_10;
    if (local_10 != (longlong *)0x0) {
      LOCK();
      plVar1 = local_10 + 1;
      lVar4 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -1;
      UNLOCK();
      if ((int)lVar4 == 1) {
        (**(code **)*local_10)(local_10);
        LOCK();
        piVar2 = (int *)((longlong)plVar5 + 0xc);
        iVar3 = *piVar2;
        *piVar2 = *piVar2 + -1;
        UNLOCK();
        if (iVar3 == 1) {
          (**(code **)(*local_10 + 8))();
        }
      }
    }
    uVar6 = FUN_1426c7360(*param_1);
    *(undefined1 *)(param_1 + 2) = uVar6;
  }
  return;
}



//===========================================================
// FUN_1426e2550 @ 1426e2550   (637 bytes)
//===========================================================

longlong * FUN_1426e2550(longlong *param_1,longlong *param_2,longlong *param_3)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong *plVar10;
  
  param_1[1] = param_1[1] + 1;
  plVar2 = (longlong *)*param_1;
  plVar8 = (longlong *)*param_2;
  param_3[1] = (longlong)plVar8;
  if (plVar8 == plVar2) {
    *plVar2 = (longlong)param_3;
    plVar2[1] = (longlong)param_3;
    plVar2[2] = (longlong)param_3;
    *(undefined1 *)(param_3 + 3) = 1;
    return param_3;
  }
  if ((int)param_2[1] == 0) {
    plVar8[2] = (longlong)param_3;
    if (plVar8 == (longlong *)plVar2[2]) {
      plVar2[2] = (longlong)param_3;
    }
  }
  else {
    *plVar8 = (longlong)param_3;
    if (plVar8 == (longlong *)*plVar2) {
      *plVar2 = (longlong)param_3;
    }
  }
  cVar1 = *(char *)(param_3[1] + 0x18);
  plVar8 = param_3;
  do {
    if (cVar1 != '\0') {
      *(undefined1 *)(plVar2[1] + 0x18) = 1;
      return param_3;
    }
    plVar9 = (longlong *)plVar8[1];
    plVar7 = plVar8 + 1;
    plVar10 = plVar9 + 1;
    lVar5 = *(longlong *)plVar9[1];
    if (plVar9 == (longlong *)lVar5) {
      lVar5 = ((longlong *)plVar9[1])[2];
      if (*(char *)(lVar5 + 0x18) != '\0') {
        plVar3 = (longlong *)plVar9[2];
        if (plVar8 == plVar3) {
          plVar9[2] = *plVar3;
          if (*(char *)(*plVar3 + 0x19) == '\0') {
            *(longlong **)(*plVar3 + 8) = plVar9;
          }
          plVar3[1] = *plVar10;
          if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
            *(longlong **)(*param_1 + 8) = plVar3;
            *plVar3 = (longlong)plVar9;
            *plVar10 = (longlong)plVar3;
            plVar8 = plVar9;
            plVar9 = plVar3;
            plVar7 = plVar10;
          }
          else {
            plVar8 = (longlong *)*plVar10;
            if (plVar9 == (longlong *)*plVar8) {
              *plVar8 = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
            else {
              plVar8[2] = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
          }
        }
        *(undefined1 *)(plVar9 + 3) = 1;
        *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
        plVar7 = *(longlong **)(*plVar7 + 8);
        plVar10 = (longlong *)*plVar7;
        *plVar7 = plVar10[2];
        if (*(char *)(plVar10[2] + 0x19) == '\0') {
          *(longlong **)(plVar10[2] + 8) = plVar7;
        }
        plVar10[1] = plVar7[1];
        if (plVar7 == *(longlong **)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar10;
          plVar10[2] = (longlong)plVar7;
        }
        else {
          plVar9 = (longlong *)plVar7[1];
          if (plVar7 == (longlong *)plVar9[2]) {
            plVar9[2] = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
          else {
            *plVar9 = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
        }
        goto LAB_1426e27a5;
      }
LAB_1426e26d7:
      *(undefined1 *)(plVar9 + 3) = 1;
      *(undefined1 *)(lVar5 + 0x18) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar8 = *(longlong **)(*plVar7 + 8);
    }
    else {
      if (*(char *)(lVar5 + 0x18) == '\0') goto LAB_1426e26d7;
      plVar3 = (longlong *)*plVar9;
      plVar6 = plVar9;
      if (plVar8 == plVar3) {
        *plVar9 = plVar3[2];
        if (*(char *)(plVar3[2] + 0x19) == '\0') {
          *(longlong **)(plVar3[2] + 8) = plVar9;
        }
        plVar3[1] = *plVar10;
        if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar3;
        }
        else {
          puVar4 = (undefined8 *)*plVar10;
          if (plVar9 == (longlong *)puVar4[2]) {
            puVar4[2] = plVar3;
          }
          else {
            *puVar4 = plVar3;
          }
        }
        plVar3[2] = (longlong)plVar9;
        *plVar10 = (longlong)plVar3;
        plVar6 = plVar3;
        plVar8 = plVar9;
        plVar7 = plVar10;
      }
      *(undefined1 *)(plVar6 + 3) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar7 = *(longlong **)(*plVar7 + 8);
      plVar10 = (longlong *)plVar7[2];
      plVar7[2] = *plVar10;
      if (*(char *)(*plVar10 + 0x19) == '\0') {
        *(longlong **)(*plVar10 + 8) = plVar7;
      }
      plVar10[1] = plVar7[1];
      if (plVar7 == *(longlong **)(*param_1 + 8)) {
        *(longlong **)(*param_1 + 8) = plVar10;
      }
      else {
        puVar4 = (undefined8 *)plVar7[1];
        if (plVar7 == (longlong *)*puVar4) {
          *puVar4 = plVar10;
        }
        else {
          puVar4[2] = plVar10;
        }
      }
      *plVar10 = (longlong)plVar7;
LAB_1426e27a5:
      plVar7[1] = (longlong)plVar10;
    }
    cVar1 = *(char *)(plVar8[1] + 0x18);
  } while( true );
}



//===========================================================
// FUN_14019f2c0 @ 14019f2c0   (345 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019f2c0(int *param_1)

{
  int iVar1;
  code *UNRECOVERED_JUMPTABLE;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  int *piVar5;
  uint uVar6;
  longlong lVar7;
  bool bVar8;
  
  if (0x100000 < *param_1 + 1U) {
    FUN_142e52dd0(0x23);
  }
  LOCK();
  iVar1 = *param_1;
  *param_1 = *param_1 + -1;
  UNLOCK();
  if (1 < iVar1) {
    return;
  }
  if (*param_1 != 0) {
    FUN_142e52dd0(0x32);
  }
  pvVar2 = Self;
  UNRECOVERED_JUMPTABLE = DAT_143ad5530;
  uVar3 = *(ulonglong *)(param_1 + -2);
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x39) {
    uVar6 = (uint)(0x28 < uVar3);
  }
  else {
    if (uVar3 < 0x59) {
      uVar6 = 2;
      goto LAB_14019f332;
    }
    uVar6 = 0xffffffff;
    if (uVar3 < 0x99) {
      uVar6 = 3;
    }
  }
  if ((int)uVar6 < 0) {
    uVar4 = (*DAT_143ad5538)();
                    /* WARNING: Could not recover jumptable at 0x00014019f3a6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(uVar4,0,param_1 + -2);
    return;
  }
LAB_14019f332:
  uVar3 = (ulonglong)uVar6;
  lVar7 = (ulonglong)uVar6 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad6a58 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019f3d5:
    *(undefined4 *)(&DAT_143ad6a60 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad6a58 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad6a58 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019f3d5;
        if (*(void **)(&DAT_143ad6a58 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad6a60 + lVar7) = *(int *)(&DAT_143ad6a60 + lVar7) + 1;
  }
  piVar5 = (int *)(&DAT_143ad6a60 + lVar7);
  *(undefined8 *)param_1 = *(undefined8 *)(&DAT_143ad6a98 + uVar3 * 8);
  *(int **)(&DAT_143ad6a98 + uVar3 * 8) = param_1;
  _DAT_143ad6ad8 = *(undefined8 *)param_1;
  *(int *)(&DAT_143ad6a44 + uVar3 * 4) = *(int *)(&DAT_143ad6a44 + uVar3 * 4) + -1;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *(undefined8 *)(&DAT_143ad6a58 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_142645170 @ 142645170   (630 bytes)
//===========================================================

void FUN_142645170(longlong param_1)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined4 *puVar4;
  longlong *plVar5;
  longlong lVar6;
  
  *(undefined8 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x40) = 0;
  if (*(longlong **)(param_1 + 0x48) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x48) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x48) = 0;
  if (*(longlong **)(param_1 + 0x50) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x50) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x50) = 0;
  if (*(longlong **)(param_1 + 0x68) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x68) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x68) = 0;
  if (*(longlong **)(param_1 + 0x70) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x70) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x70) = 0;
  if (*(longlong **)(param_1 + 0x58) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x58) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x58) = 0;
  if (*(longlong **)(param_1 + 0x60) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x60) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x60) = 0;
  *(undefined4 *)(param_1 + 0xa0) = 0;
  *(undefined8 *)(param_1 + 0xc70) = 0;
  *(undefined4 *)(param_1 + 0xc78) = 0;
  *(undefined4 *)(param_1 + 0x10c0) = 0;
  *(undefined4 *)(param_1 + 0x10c4) = 0xffffffff;
  *(undefined4 *)(param_1 + 0x1100) = 0;
  *(undefined1 *)(param_1 + 0x4e8) = 0;
  *(undefined8 *)(param_1 + 0x4ec) = 0;
  plVar5 = (longlong *)(param_1 + 0x938);
  puVar4 = (undefined4 *)(param_1 + 0xc4);
  lVar6 = 0x22;
  do {
    *(undefined8 *)(puVar4 + -7) = 0;
    *puVar4 = 0;
    *(undefined8 *)(puVar4 + 0x2ef) = 0;
    puVar4[0x2f6] = 0;
    lVar3 = plVar5[1];
    lVar2 = *plVar5;
    if (lVar2 != lVar3) {
      do {
        if (*(longlong *)(lVar2 + 8) != 0) {
          FUN_14019f2c0(*(longlong *)(lVar2 + 8) + -0x10);
        }
        lVar2 = lVar2 + 0x10;
      } while (lVar2 != lVar3);
      lVar2 = *plVar5;
    }
    plVar5[1] = lVar2;
    *(undefined8 *)(puVar4 + 0x10d) = 0;
    puVar4[0x114] = 0;
    plVar5 = plVar5 + 3;
    puVar4 = puVar4 + 8;
    lVar6 = lVar6 + -1;
  } while (lVar6 != 0);
  lVar6 = *(longlong *)(param_1 + 0x10e0);
  lVar3 = *(longlong *)(param_1 + 0x10e8);
  if (lVar6 != lVar3) {
    do {
      puVar1 = *(undefined8 **)(lVar6 + 8);
      if (puVar1 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar1[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar1[1] = puVar1[1] + 1;
        UNLOCK();
      }
      if (puVar1 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_142645170(puVar1);
      if (puVar1 != (undefined8 *)0x0) {
        if (0xffffe < puVar1[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar5 = puVar1 + 1;
        lVar2 = *plVar5;
        *plVar5 = *plVar5 + -1;
        UNLOCK();
        if (((int)lVar2 == 1) && (puVar1 != (undefined8 *)0x0)) {
          (**(code **)*puVar1)(puVar1,1);
        }
      }
      lVar6 = lVar6 + 0x10;
    } while (lVar6 != lVar3);
    lVar6 = *(longlong *)(param_1 + 0x10e8);
    lVar3 = *(longlong *)(param_1 + 0x10e0);
    if (lVar3 != lVar6) {
      do {
        FUN_140336390(lVar3);
        lVar3 = lVar3 + 0x10;
      } while (lVar3 != lVar6);
      lVar3 = *(longlong *)(param_1 + 0x10e0);
    }
    *(longlong *)(param_1 + 0x10e8) = lVar3;
  }
  return;
}



//===========================================================
// FUN_140336390 @ 140336390   (106 bytes)
//===========================================================

void FUN_140336390(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 8) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 8);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar2 == 1) && (puVar3 = *(undefined8 **)(param_1 + 8), puVar3 != (undefined8 *)0x0))
    {
      (**(code **)*puVar3)(puVar3,1);
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// `eh_vector_constructor_iterator' @ 142ef44fc   (112 bytes)
//===========================================================

/* Library Function - Single Match
    void __cdecl `eh vector constructor iterator'(void * __ptr64,unsigned __int64,unsigned
   __int64,void (__cdecl*)(void * __ptr64),void (__cdecl*)(void * __ptr64))
   
   Libraries: Visual Studio 2017 Release, Visual Studio 2019 Release */

void __cdecl
_eh_vector_constructor_iterator_
          (void *param_1,__uint64 param_2,__uint64 param_3,_func_void_void_ptr *param_4,
          _func_void_void_ptr *param_5)

{
  __uint64 _Var1;
  
  for (_Var1 = 0; _Var1 != param_3; _Var1 = _Var1 + 1) {
    (*(code *)PTR_FUN_1432630d8)(param_1);
    param_1 = (void *)((longlong)param_1 + param_2);
  }
  return;
}



//===========================================================
// FUN_142e559e0 @ 142e559e0   (127 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_142e559e0(void)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  
  iVar2 = FUN_14090d160(0x116,1);
  if (iVar2 != 0) {
    cVar1 = FUN_14090d340(0x11a);
    if (cVar1 != '\0') {
      _DAT_00000000 = 1;
    }
    if (DAT_143ae1514 < 3) {
      DAT_143ae1514 = DAT_143ae1514 + 1;
      return 1;
    }
    uVar3 = (*DAT_143262db0)();
    cVar1 = FUN_1408fcaa0(DAT_143ae1548,1800000,uVar3);
    if (cVar1 != '\0') {
      DAT_143ae1548 = uVar3;
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_14091a3e0 @ 14091a3e0   (62 bytes)
//===========================================================

uint FUN_14091a3e0(undefined8 *param_1)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  ulonglong uVar4;
  
  pbVar3 = (byte *)*param_1;
  if (pbVar3 != (byte *)0x0) {
    uVar2 = 0x811c9dc5;
    if (*(uint *)(pbVar3 + -8) != 0) {
      uVar4 = (ulonglong)*(uint *)(pbVar3 + -8);
      do {
        bVar1 = *pbVar3;
        pbVar3 = pbVar3 + 1;
        uVar2 = (bVar1 ^ uVar2) * 0x1000193;
        uVar4 = uVar4 - 1;
      } while (uVar4 != 0);
    }
    return uVar2;
  }
  return 0x811c9dc5;
}



//===========================================================
// FUN_140194c60 @ 140194c60   (6267 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 * FUN_140194c60(undefined8 *param_1)

{
  code *pcVar1;
  undefined8 uVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  longlong *plVar7;
  int *piVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulonglong *puVar11;
  undefined4 *puVar12;
  longlong lVar13;
  longlong lVar14;
  char *pcVar15;
  undefined8 uVar16;
  int iVar17;
  int *piVar18;
  int *piVar19;
  int *piVar20;
  ulonglong uVar21;
  undefined8 *puVar22;
  int iVar23;
  int *piVar24;
  undefined1 auStack_aa8 [32];
  int **local_a88;
  int **local_a80;
  undefined8 local_a78;
  undefined8 local_a70;
  undefined8 local_a68;
  int *local_a58;
  longlong local_a50;
  undefined8 *local_a48;
  ulonglong local_a40;
  int local_a38;
  ulonglong local_a30;
  longlong local_a28;
  undefined8 *local_a20;
  undefined4 local_a18 [2];
  undefined8 local_a10;
  undefined8 uStack_a08;
  undefined8 local_a00;
  undefined8 uStack_9f8;
  undefined8 local_9e8;
  undefined1 local_9e0 [4];
  undefined4 local_9dc;
  longlong local_9c8;
  undefined4 local_9bc;
  undefined8 local_9b8;
  undefined4 local_9ac;
  undefined4 local_8d8;
  undefined8 local_8d4;
  undefined8 uStack_8cc;
  undefined8 local_8c4;
  undefined8 uStack_8bc;
  undefined8 local_8b4;
  undefined8 uStack_8ac;
  undefined8 local_8a4;
  undefined8 uStack_89c;
  undefined8 local_894;
  undefined8 uStack_88c;
  undefined8 local_884;
  undefined8 uStack_87c;
  undefined8 local_874;
  undefined8 uStack_86c;
  undefined8 local_864;
  undefined8 uStack_85c;
  undefined8 local_854;
  undefined8 uStack_84c;
  undefined1 local_838 [48];
  undefined4 local_808;
  undefined8 local_7a0;
  longlong local_798;
  undefined8 local_740;
  int *local_368 [34];
  undefined4 local_258 [6];
  undefined4 local_240;
  undefined1 local_23c [516];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_aa8;
  piVar24 = (int *)0x0;
  iVar3 = 0;
  local_a58 = (int *)0x0;
  local_a48 = param_1;
  local_a20 = param_1;
  if (DAT_143aa8288 == '\0') {
    local_a50 = 0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a50,"Not Init\r\n");
    lVar14 = *plVar7;
    piVar18 = piVar24;
    if (lVar14 == 0) goto LAB_140194d86;
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    param_1 = local_a20;
    if (iVar4 == 0) goto LAB_140194d86;
    if (0 < iVar4) {
      iVar3 = iVar4;
    }
    piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
    piVar8[1] = iVar3;
    *piVar8 = -1;
    piVar18 = piVar8 + 4;
    piVar8[2] = 0;
    *(char *)piVar18 = '\0';
    local_a58 = piVar18;
    FUN_142ef7ba0(piVar18,lVar14,piVar20);
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
      *piVar8 = 1;
      if (iVar4 != -1) goto LAB_140194d5f;
      piVar20 = (int *)0xffffffffffffffff;
      if (piVar18 != (int *)0x0) {
        do {
          piVar24 = (int *)((longlong)piVar20 + 1);
          piVar20 = piVar24;
        } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar8[1],iVar4);
      *piVar8 = 1;
LAB_140194d5f:
      *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      piVar24 = piVar20;
    }
    iVar3 = (int)piVar24;
    if ((iVar3 < 0) || (piVar8[1] + 1 <= iVar3)) {
      FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
    }
    piVar8[2] = iVar3;
    param_1 = local_a20;
LAB_140194d86:
    if (local_a50 != 0) {
      FUN_14019f2c0(local_a50 + -0x10);
    }
    *param_1 = piVar18;
    return param_1;
  }
  FUN_142ef8250(local_838,0,0x4d0);
  local_808 = 0x10001f;
  (*DAT_143262840)(local_838);
  local_a50 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a50,&DAT_143271d00);
  lVar14 = *plVar7;
  piVar18 = piVar24;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    piVar18 = (int *)0x0;
    if (iVar4 != 0) {
      iVar6 = 0;
      if (0 < iVar4) {
        iVar6 = iVar4;
      }
      piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      piVar8[1] = iVar6;
      *piVar8 = -1;
      piVar18 = piVar8 + 4;
      piVar8[2] = 0;
      *(char *)piVar18 = '\0';
      local_a58 = piVar18;
      FUN_142ef7ba0(piVar18,lVar14,piVar20);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar4 != -1) goto LAB_140194ea0;
        piVar20 = piVar24;
        if (piVar18 != (int *)0x0) {
          piVar20 = (int *)0xffffffffffffffff;
          do {
            piVar20 = (int *)((longlong)piVar20 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar20) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],iVar4);
        *piVar8 = 1;
LAB_140194ea0:
        *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      }
      iVar4 = (int)piVar20;
      if ((iVar4 < 0) || (piVar8[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,(ulonglong)piVar20 & 0xffffffff);
      }
      piVar8[2] = iVar4;
    }
  }
  if (local_a50 != 0) {
    FUN_14019f2c0(local_a50 + -0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Major:%d Minor:%d\r\n",1);
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar4;
    if (iVar4 != 0) {
      piVar19 = piVar24;
      if (piVar18 == (int *)0x0) goto LAB_14019509b;
      if ((char)*piVar18 != '\0') {
        iVar6 = piVar18[-2];
        for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar18 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140194f9f:
          if (iVar3 < iVar17) {
            iVar3 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          local_a58 = piVar20;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar20 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar18[-2] + 1;
            iVar23 = iVar3 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
            puVar12[2] = piVar18[-2];
            *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
            iVar3 = piVar18[-2];
            goto LAB_140194f9f;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar20 == (int *)0x0) {
          iVar3 = 0;
        }
        else {
          iVar3 = piVar20[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar4);
        goto LAB_140195140;
      }
      if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019509b:
        if (iVar3 < iVar4) {
          iVar3 = iVar4;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
        puVar12[1] = iVar3;
        *puVar12 = 0xffffffff;
        piVar20 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        local_a58 = piVar20;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
          iVar3 = piVar18[-2];
          goto LAB_14019509b;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar20,lVar14,piVar8);
      if (piVar20[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
        piVar20[-4] = 1;
        if (iVar4 != -1) goto LAB_14019511d;
        if (piVar20 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar20[-3],iVar4);
        piVar20[-4] = 1;
LAB_14019511d:
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar20[-2] = iVar3;
    }
  }
LAB_140195140:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Call stack:\r\n");
  lVar14 = *plVar7;
  piVar18 = piVar20;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar3 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar3;
    if (iVar3 != 0) {
      iVar4 = 0;
      piVar19 = piVar24;
      if (piVar20 == (int *)0x0) goto LAB_14019530e;
      if ((char)*piVar20 != '\0') {
        iVar6 = piVar20[-2];
        for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar20 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140195212:
          if (iVar4 < iVar17) {
            iVar4 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          local_a58 = piVar18;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar20[-2] + 1;
            iVar23 = iVar4 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
            puVar12[2] = piVar20[-2];
            *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
            iVar4 = piVar20[-2];
            goto LAB_140195212;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar18 == (int *)0x0) {
          iVar4 = 0;
        }
        else {
          iVar4 = piVar18[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar3);
        goto LAB_1401953b3;
      }
      if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_14019530e:
        if (iVar4 < iVar3) {
          iVar4 = iVar3;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
        puVar12[1] = iVar4;
        *puVar12 = 0xffffffff;
        piVar18 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar18 = '\0';
        local_a58 = piVar18;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
          iVar4 = piVar20[-2];
          goto LAB_14019530e;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar18,lVar14,piVar8);
      if (piVar18[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
        piVar18[-4] = 1;
        if (iVar3 != -1) goto LAB_140195390;
        if (piVar18 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar18[-3],iVar3);
        piVar18[-4] = 1;
LAB_140195390:
        *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar18[-2] = iVar3;
    }
  }
LAB_1401953b3:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Address   Frame\r\n");
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 == 0) goto LAB_140195630;
  iVar3 = *(int *)(lVar14 + -8);
  piVar8 = (int *)(longlong)iVar3;
  if (iVar3 == 0) goto LAB_140195630;
  iVar4 = 0;
  piVar19 = piVar24;
  if (piVar18 == (int *)0x0) goto LAB_14019558b;
  if ((char)*piVar18 != '\0') {
    iVar6 = piVar18[-2];
    for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
    }
    piVar24 = piVar18 + -4;
    if (piVar24 == (int *)0x0) {
LAB_14019548f:
      if (iVar4 < iVar17) {
        iVar4 = iVar17;
      }
      puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
      puVar12[1] = iVar4;
      *puVar12 = 0xffffffff;
      piVar20 = puVar12 + 4;
      local_a58 = piVar20;
      if (piVar24 == (int *)0x0) {
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        lVar14 = local_a50;
      }
      else {
        iVar17 = piVar18[-2] + 1;
        iVar23 = iVar4 + 1;
        if (iVar23 < iVar17) {
          FUN_142e54290(0x5c,iVar17,iVar23);
          iVar17 = iVar23;
        }
        FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
        puVar12[2] = piVar18[-2];
        *(char *)((longlong)iVar4 + (longlong)piVar20) = '\0';
        FUN_14019f2c0(piVar24);
        lVar14 = local_a50;
      }
    }
    else {
      if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
        iVar4 = piVar18[-2];
        goto LAB_14019548f;
      }
      if (*piVar24 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar24 = -1;
    }
    if (piVar20 == (int *)0x0) {
      iVar4 = 0;
    }
    else {
      iVar4 = piVar20[-2];
    }
    FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar20),lVar14,piVar8);
    FUN_14019c870(&local_a58,iVar6 + iVar3);
    goto LAB_140195630;
  }
  if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019558b:
    if (iVar4 < iVar3) {
      iVar4 = iVar3;
    }
    puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
    puVar12[1] = iVar4;
    *puVar12 = 0xffffffff;
    piVar20 = puVar12 + 4;
    puVar12[2] = 0;
    *(char *)piVar20 = '\0';
    local_a58 = piVar20;
    if (piVar19 != (int *)0x0) {
      FUN_14019f2c0(piVar19);
    }
  }
  else {
    if ((1 < *piVar19) || (piVar18[-3] < iVar3)) {
      iVar4 = piVar18[-2];
      goto LAB_14019558b;
    }
    if (*piVar19 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar19 = -1;
  }
  FUN_142ef7ba0(piVar20,lVar14,piVar8);
  if (piVar20[-4] != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar3 == -1) || (iVar3 <= piVar20[-3])) {
    piVar20[-4] = 1;
    if (iVar3 != -1) goto LAB_14019560d;
    if (piVar20 != (int *)0x0) {
      piVar24 = (int *)0xffffffffffffffff;
      do {
        piVar24 = (int *)((longlong)piVar24 + 1);
      } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
    }
  }
  else {
    FUN_142e54290(0x90,piVar20[-3],iVar3);
    piVar20[-4] = 1;
LAB_14019560d:
    *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
    piVar24 = piVar8;
  }
  iVar3 = (int)piVar24;
  if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
    FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
  }
  piVar20[-2] = iVar3;
LAB_140195630:
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  FUN_142ef8250(local_9e0,0,0x100);
  uVar2 = DAT_143aa8260;
  uVar16 = DAT_143aa8258;
  pcVar1 = DAT_143aa8250;
  local_9e8 = local_740;
  local_9dc = 3;
  local_9b8 = local_7a0;
  local_9ac = 3;
  local_9c8 = local_798;
  local_9bc = 3;
  uVar9 = (*DAT_143ad5440)();
  uVar10 = (*DAT_143ad5408)();
  local_a68 = 0;
  local_a70 = uVar2;
  local_a78 = uVar16;
  local_a80 = (int **)0x0;
  local_a88 = (int **)local_838;
  iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  do {
    if ((iVar3 == 0) || (piVar24 = (int *)0x0, local_9c8 == 0)) {
      *local_a20 = piVar20;
      return local_a20;
    }
    local_a30 = 0;
    puVar11 = (ulonglong *)FUN_14019ba10(&local_a30,"%016X  %016X  ",local_9e8);
    uVar21 = *puVar11;
    piVar18 = piVar20;
    local_a40 = uVar21;
    if (uVar21 != 0) {
      iVar3 = *(int *)(uVar21 - 8);
      piVar8 = (int *)(longlong)iVar3;
      if (iVar3 != 0) {
        iVar4 = 0;
        piVar19 = piVar24;
        if (piVar20 == (int *)0x0) goto LAB_1401958b6;
        if ((char)*piVar20 != '\0') {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar20 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401957c2:
            if (iVar4 < iVar17) {
              iVar4 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
            puVar12[1] = iVar4;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            local_a58 = piVar18;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar18 = '\0';
              uVar21 = local_a40;
            }
            else {
              iVar17 = piVar20[-2] + 1;
              iVar23 = iVar4 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
              puVar12[2] = piVar20[-2];
              *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
              FUN_14019f2c0(piVar24);
              uVar21 = local_a40;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
              iVar4 = piVar20[-2];
              goto LAB_1401957c2;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          if (piVar18 == (int *)0x0) {
            iVar4 = 0;
          }
          else {
            iVar4 = piVar18[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),uVar21,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar3);
          goto LAB_14019595b;
        }
        if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_1401958b6:
          if (iVar4 < iVar3) {
            iVar4 = iVar3;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar18 = '\0';
          local_a58 = piVar18;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
            iVar4 = piVar20[-2];
            goto LAB_1401958b6;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar18,uVar21,piVar8);
        if (piVar18[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
          piVar18[-4] = 1;
          if (iVar3 != -1) goto LAB_140195938;
          if (piVar18 != (int *)0x0) {
            piVar24 = (int *)0xffffffffffffffff;
            do {
              piVar24 = (int *)((longlong)piVar24 + 1);
            } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar18[-3],iVar3);
          piVar18[-4] = 1;
LAB_140195938:
          *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
          piVar24 = piVar8;
        }
        iVar3 = (int)piVar24;
        if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
        }
        piVar18[-2] = iVar3;
      }
    }
LAB_14019595b:
    piVar24 = (int *)0x0;
    if (local_a30 != 0) {
      FUN_14019f2c0(local_a30 - 0x10);
    }
    local_258[0] = 0x20;
    local_240 = 0x200;
    local_a50 = 0;
    FUN_142ef8250(local_368,0,0x104);
    iVar3 = 0;
    local_a40 = local_a40 & 0xffffffff00000000;
    local_a30 = local_a30 & 0xffffffff00000000;
    local_a18[0] = 0x28;
    local_a10 = 0;
    local_a00 = 0;
    uStack_9f8 = 0;
    uStack_a08 = 0xffffffff;
    if (DAT_143aa8280 == 0) {
      local_8d8 = 0x94;
      local_8d4 = 0;
      uStack_8cc = 0;
      local_8c4 = 0;
      uStack_8bc = 0;
      local_8b4 = 0;
      uStack_8ac = 0;
      local_8a4 = 0;
      uStack_89c = 0;
      local_894 = 0;
      uStack_88c = 0;
      local_884 = 0;
      uStack_87c = 0;
      local_874 = 0;
      uStack_86c = 0;
      local_864 = 0;
      uStack_85c = 0;
      local_854 = 0;
      uStack_84c = 0;
      (*DAT_143262820)(&local_8d8);
      if (uStack_8cc._4_4_ == 2) {
        DAT_143aa8280 = (*DAT_143ad5408)();
      }
      else {
        iVar4 = (*DAT_143ad5410)();
        DAT_143aa8280 = (longlong)iVar4;
      }
      if (DAT_143aa8280 == 0) {
        local_a28 = 0;
        plVar7 = (longlong *)FUN_14019ba10(&local_a28,"m_hProcess is Null\r\n");
        puVar22 = (undefined8 *)*plVar7;
        piVar20 = piVar18;
        local_a48 = puVar22;
        if (puVar22 == (undefined8 *)0x0) goto LAB_140196462;
        iVar4 = *(int *)(puVar22 + -1);
        piVar8 = (int *)(longlong)iVar4;
        if (iVar4 == 0) goto LAB_140196462;
        piVar19 = piVar24;
        if (piVar18 == (int *)0x0) goto LAB_1401963be;
        if ((char)*piVar18 != '\0') {
          iVar6 = piVar18[-2];
          for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar18 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401962c5:
            if (iVar3 < iVar17) {
              iVar3 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar20 = puVar12 + 4;
            local_a58 = piVar20;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar20 = '\0';
              puVar22 = local_a48;
            }
            else {
              iVar17 = piVar18[-2] + 1;
              iVar23 = iVar3 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
              puVar12[2] = piVar18[-2];
              *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
              FUN_14019f2c0(piVar24);
              puVar22 = local_a48;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
              iVar3 = piVar18[-2];
              goto LAB_1401962c5;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          iVar3 = 0;
          if (piVar20 != (int *)0x0) {
            iVar3 = piVar20[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),puVar22,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
          goto LAB_140196462;
        }
        if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_1401963be:
          if (iVar3 < iVar4) {
            iVar3 = iVar4;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar20 = '\0';
          local_a58 = piVar20;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
            iVar3 = piVar18[-2];
            goto LAB_1401963be;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar20,puVar22,piVar8);
        if (piVar20[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
          piVar20[-4] = 1;
          if (iVar4 == -1) {
            piVar18 = (int *)0xffffffffffffffff;
            if (piVar20 != (int *)0x0) {
              do {
                piVar24 = (int *)((longlong)piVar18 + 1);
                piVar18 = piVar24;
              } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
            }
LAB_140196443:
            iVar3 = (int)piVar24;
            if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
              FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
            }
            piVar20[-2] = iVar3;
LAB_140196462:
            if (local_a28 != 0) {
              FUN_14019f2c0(local_a28 + -0x10);
            }
            *local_a20 = piVar20;
            return local_a20;
          }
        }
        else {
          FUN_142e54290(0x90,piVar20[-3],iVar4);
          piVar20[-4] = 1;
        }
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
        goto LAB_140196443;
      }
    }
    iVar4 = (*DAT_143aa8268)(DAT_143aa8280,local_9e8,&local_a50,local_258);
    local_a38 = iVar4;
    if (iVar4 == 0) {
      uVar5 = (*DAT_143262838)();
      local_a48 = (undefined8 *)0x0;
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"_SymGetLineFromAddr Error : %x ",uVar5);
      lVar14 = *plVar7;
      local_a28 = lVar14;
      if (lVar14 != 0) {
        iVar6 = *(int *)(lVar14 + -8);
        piVar20 = (int *)(longlong)iVar6;
        iVar4 = local_a38;
        if (iVar6 != 0) {
          piVar8 = piVar24;
          if (piVar18 == (int *)0x0) goto LAB_140195c6d;
          if ((char)*piVar18 != '\0') {
            iVar4 = piVar18[-2];
            for (iVar17 = piVar18[-3]; iVar17 < iVar4 + iVar6; iVar17 = iVar17 * 2) {
            }
            piVar24 = piVar18 + -4;
            if (piVar24 == (int *)0x0) {
LAB_140195b6f:
              if (iVar3 < iVar17) {
                iVar3 = iVar17;
              }
              puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
              puVar12[1] = iVar3;
              *puVar12 = 0xffffffff;
              piVar8 = puVar12 + 4;
              local_a58 = piVar8;
              if (piVar24 == (int *)0x0) {
                puVar12[2] = 0;
                *(char *)piVar8 = '\0';
                lVar14 = local_a28;
              }
              else {
                iVar17 = piVar18[-2] + 1;
                iVar23 = iVar3 + 1;
                if (iVar23 < iVar17) {
                  FUN_142e54290(0x5c,iVar17,iVar23);
                  iVar17 = iVar23;
                }
                FUN_142ef7ba0(piVar8,piVar18,(longlong)iVar17);
                puVar12[2] = piVar18[-2];
                *(char *)((longlong)iVar3 + (longlong)piVar8) = '\0';
                FUN_14019f2c0(piVar24);
                lVar14 = local_a28;
              }
            }
            else {
              if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
                iVar3 = piVar18[-2];
                goto LAB_140195b6f;
              }
              if (*piVar24 != 1) {
                FUN_142e52dd0(0x74);
              }
              *piVar24 = -1;
              piVar8 = piVar18;
            }
            if (piVar8 == (int *)0x0) {
              iVar3 = 0;
            }
            else {
              iVar3 = piVar8[-2];
            }
            FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar8),lVar14,piVar20);
            FUN_14019c870(&local_a58,iVar4 + iVar6);
            iVar4 = local_a38;
            goto LAB_140195d1d;
          }
          if ((piVar18 == (int *)0x0) || (piVar8 = piVar18 + -4, piVar8 == (int *)0x0)) {
LAB_140195c6d:
            if (iVar3 < iVar6) {
              iVar3 = iVar6;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            local_a58 = piVar18;
            if (piVar8 != (int *)0x0) {
              FUN_14019f2c0(piVar8);
            }
          }
          else {
            if ((1 < *piVar8) || (piVar18[-3] < iVar6)) {
              iVar3 = piVar18[-2];
              goto LAB_140195c6d;
            }
            if (*piVar8 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar8 = -1;
          }
          FUN_142ef7ba0(piVar18,lVar14,piVar20);
          if (piVar18[-4] != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar6 == -1) || (iVar6 <= piVar18[-3])) {
            piVar18[-4] = 1;
            if (iVar6 != -1) goto LAB_140195cf6;
            if (piVar18 != (int *)0x0) {
              piVar24 = (int *)0xffffffffffffffff;
              do {
                piVar24 = (int *)((longlong)piVar24 + 1);
              } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar18[-3],iVar6);
            piVar18[-4] = 1;
LAB_140195cf6:
            *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
            piVar24 = piVar20;
          }
          iVar3 = (int)piVar24;
          if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
            FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
          }
          piVar18[-2] = iVar3;
          iVar4 = local_a38;
        }
      }
LAB_140195d1d:
      if (local_a48 != (undefined8 *)0x0) {
        FUN_14019f2c0(local_a48 + -2);
      }
    }
    else if (DAT_143aa8270 != (code *)0x0) {
      (*DAT_143aa8270)(DAT_143aa8280,local_9e8,&local_a50,local_a18);
    }
    local_a80 = &local_a58;
    local_a88 = (int **)&local_a30;
    iVar6 = FUN_140194a90(local_9e8,local_368,0x104,&local_a40);
    local_a48 = (undefined8 *)0x0;
    iVar3 = 0;
    if ((int)uStack_a08 == -1) {
      if (iVar4 == 0) {
        local_a88 = local_368;
        plVar7 = (longlong *)
                 FUN_14019ba10(&local_a48,"%04X:%08X [%s]",local_a40 & 0xffffffff,
                               local_a30 & 0xffffffff);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
      else {
        local_a88 = local_368;
        plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs()+%X [%s]",local_23c,local_a50);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
    }
    else {
      local_a80 = local_368;
      local_a88 = (int **)CONCAT44(local_a88._4_4_,(int)uStack_a08);
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs() %hs(%lu) [%s]",local_23c,local_a00);
      lVar14 = *plVar7;
      piVar20 = local_a58;
      if (lVar14 != 0) {
        iVar4 = *(int *)(lVar14 + -8);
        if (iVar4 != 0) {
          if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
            uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
            FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar4);
            piVar20 = local_a58;
          }
          else {
            iVar17 = local_a58[-2];
            for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
            }
            lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
            piVar20 = local_a58;
            iVar23 = iVar3;
            if (local_a58 != (int *)0x0) {
              iVar23 = local_a58[-2];
            }
            FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar17 + iVar4);
          }
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    if (iVar6 == 0) {
      if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
        pcVar15 = (char *)FUN_14019bd40(&local_a58,0xc);
        *(undefined8 *)pcVar15 = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)(pcVar15 + 8) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,0xc);
        piVar20 = local_a58;
      }
      else {
        iVar4 = piVar20[-2];
        for (iVar6 = piVar20[-3]; iVar6 < iVar4 + 0xc; iVar6 = iVar6 * 2) {
        }
        lVar14 = FUN_14019bd40(&local_a58,iVar6,1);
        piVar20 = local_a58;
        iVar6 = iVar3;
        if (local_a58 != (int *)0x0) {
          iVar6 = local_a58[-2];
        }
        *(undefined8 *)(iVar6 + lVar14) = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)((longlong)iVar6 + 8 + lVar14) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,iVar4 + 0xc);
      }
    }
    local_a48 = (undefined8 *)0x0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a48,&DAT_143271d00);
    lVar14 = *plVar7;
    if (lVar14 != 0) {
      iVar4 = *(int *)(lVar14 + -8);
      if (iVar4 != 0) {
        if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
          uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
          FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar4);
          piVar20 = local_a58;
        }
        else {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          lVar13 = FUN_14019bd40(&local_a58,iVar17,1);
          piVar20 = local_a58;
          if (local_a58 != (int *)0x0) {
            iVar3 = local_a58[-2];
          }
          FUN_142ef7ba0(iVar3 + lVar13,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    uVar2 = DAT_143aa8260;
    uVar16 = DAT_143aa8258;
    pcVar1 = DAT_143aa8250;
    uVar9 = (*DAT_143ad5440)();
    uVar10 = (*DAT_143ad5408)();
    local_a68 = 0;
    local_a70 = uVar2;
    local_a78 = uVar16;
    local_a80 = (int **)0x0;
    local_a88 = (int **)local_838;
    iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  } while( true );
}



//===========================================================
// FUN_142e5cd30 @ 142e5cd30   (764 bytes)
//===========================================================

void FUN_142e5cd30(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8)

{
  undefined8 uVar1;
  undefined1 local_84;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  longlong local_38;
  longlong local_30;
  longlong local_28;
  longlong local_20;
  
  uVar1 = FUN_142a1d8a0(&local_20);
  local_80 = 0;
  FUN_140198700(&local_80,uVar1,local_84);
  local_58 = 0;
  uVar1 = FUN_14019ba10(&local_58,&DAT_143272338,param_1);
  local_78 = 0;
  FUN_14019a260(&local_78,uVar1);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_78);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  local_50 = 0;
  uVar1 = FUN_14019ba10(&local_50,&DAT_143272338,param_2);
  local_70 = 0;
  FUN_14019a260(&local_70,uVar1);
  if (local_50 != 0) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_70);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  FUN_1408bc140(&local_48,param_3);
  FUN_1401a1c50(&local_80,&local_48);
  if (local_48 != 0) {
    FUN_14019f2c0(local_48 + -0x10);
  }
  local_40 = 0;
  uVar1 = FUN_14019ba10(&local_40,&DAT_143272338,param_4);
  local_68 = 0;
  FUN_14019a260(&local_68,uVar1);
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_68);
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  FUN_1408bc020(&local_38,param_5);
  FUN_1401a1c50(&local_80,&local_38);
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
  local_30 = 0;
  uVar1 = FUN_14019ba10(&local_30,&DAT_143272338,param_6);
  local_60 = 0;
  FUN_14019a260(&local_60,uVar1);
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_60);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  FUN_1408bc280(&local_28,param_7);
  FUN_1401a1c50(&local_80,&local_28);
  if (local_28 != 0) {
    FUN_14019f2c0(local_28 + -0x10);
  }
  FUN_140198700(&local_80,param_8,local_84);
  FUN_142a1ec10(&local_80);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  if (local_20 != 0) {
    FUN_14019f2c0(local_20 + -0x10);
  }
  return;
}



//===========================================================
// FUN_142ef7ba0 @ 142ef7ba0   (1379 bytes)
//===========================================================

undefined8 * FUN_142ef7ba0(undefined8 *param_1,undefined8 *param_2,ulonglong param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [32];
  undefined1 auVar4 [32];
  undefined1 auVar5 [32];
  undefined1 auVar6 [32];
  undefined1 uVar7;
  undefined2 uVar8;
  undefined4 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  undefined8 *puVar23;
  undefined1 (*pauVar24) [32];
  undefined1 (*pauVar25) [32];
  undefined8 *puVar26;
  undefined1 (*pauVar27) [32];
  undefined1 (*pauVar28) [32];
  ulonglong uVar29;
  longlong lVar30;
  ulonglong uVar31;
  undefined8 uVar32;
  undefined8 uVar33;
  
  puVar23 = param_1;
  switch(param_3) {
  case 0:
    return puVar23;
  case 1:
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    return puVar23;
  case 2:
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    return puVar23;
  case 3:
    uVar7 = *(undefined1 *)((longlong)param_2 + 2);
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    *(undefined1 *)((longlong)param_1 + 2) = uVar7;
    return puVar23;
  case 4:
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    return puVar23;
  case 5:
    uVar7 = *(undefined1 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined1 *)((longlong)param_1 + 4) = uVar7;
    return puVar23;
  case 6:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    return puVar23;
  case 7:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    uVar7 = *(undefined1 *)((longlong)param_2 + 6);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    *(undefined1 *)((longlong)param_1 + 6) = uVar7;
    return puVar23;
  case 8:
    *param_1 = *param_2;
    return puVar23;
  case 9:
    uVar7 = *(undefined1 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined1 *)(param_1 + 1) = uVar7;
    return puVar23;
  case 10:
    uVar8 = *(undefined2 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    return puVar23;
  case 0xb:
    uVar8 = *(undefined2 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 10);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    *(undefined1 *)((longlong)param_1 + 10) = uVar7;
    return puVar23;
  case 0xc:
    uVar9 = *(undefined4 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    return puVar23;
  case 0xd:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined1 *)((longlong)param_1 + 0xc) = uVar7;
    return puVar23;
  case 0xe:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    return puVar23;
  case 0xf:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xe);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    *(undefined1 *)((longlong)param_1 + 0xe) = uVar7;
    return puVar23;
  }
  if (param_3 < 0x21) {
    uVar10 = param_2[1];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x10));
    uVar11 = *puVar26;
    uVar32 = puVar26[1];
    *param_1 = *param_2;
    param_1[1] = uVar10;
    param_1 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    *param_1 = uVar11;
    param_1[1] = uVar32;
    return puVar23;
  }
  if ((param_2 < param_1) && (param_1 < (undefined8 *)((longlong)param_2 + param_3))) {
    lVar30 = (longlong)param_2 - (longlong)param_1;
    puVar23 = (undefined8 *)((longlong)param_1 + lVar30 + (param_3 - 0x10));
    uVar10 = *puVar23;
    uVar11 = puVar23[1];
    puVar26 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    uVar29 = param_3 - 0x10;
    puVar23 = puVar26;
    uVar32 = uVar10;
    uVar33 = uVar11;
    if (((ulonglong)puVar26 & 0xf) != 0) {
      puVar23 = (undefined8 *)((ulonglong)puVar26 & 0xfffffffffffffff0);
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
      *puVar26 = uVar10;
      *(undefined8 *)((longlong)param_1 + (param_3 - 8)) = uVar11;
      uVar29 = (longlong)puVar23 - (longlong)param_1;
    }
    uVar31 = uVar29 >> 7;
    if (uVar31 != 0) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar26 = puVar23;
      while( true ) {
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x10);
        uVar10 = puVar1[1];
        puVar23 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x20);
        uVar11 = *puVar23;
        uVar32 = puVar23[1];
        puVar23 = puVar26 + -0x10;
        puVar26[-2] = *puVar1;
        puVar26[-1] = uVar10;
        puVar26[-4] = uVar11;
        puVar26[-3] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x30);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x40);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        uVar31 = uVar31 - 1;
        puVar26[-6] = *puVar1;
        puVar26[-5] = uVar10;
        puVar26[-8] = uVar11;
        puVar26[-7] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x50);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x60);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        puVar26[-10] = *puVar1;
        puVar26[-9] = uVar10;
        puVar26[-0xc] = uVar11;
        puVar26[-0xb] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x70);
        uVar10 = *puVar1;
        uVar11 = puVar1[1];
        uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
        uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
        if (uVar31 == 0) break;
        puVar26[-0xe] = uVar10;
        puVar26[-0xd] = uVar11;
        *puVar23 = uVar32;
        puVar26[-0xf] = uVar33;
        puVar26 = puVar23;
      }
      puVar26[-0xe] = uVar10;
      puVar26[-0xd] = uVar11;
      uVar29 = uVar29 & 0x7f;
    }
    for (uVar31 = uVar29 >> 4; uVar31 != 0; uVar31 = uVar31 - 1) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar23 = puVar23 + -2;
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
    }
    if ((uVar29 & 0xf) != 0) {
      uVar10 = param_2[1];
      *param_1 = *param_2;
      param_1[1] = uVar10;
    }
    *puVar23 = uVar32;
    puVar23[1] = uVar33;
    return param_1;
  }
  if (DAT_143a8b918 < 3) {
    if ((param_3 < 0x801) || (((byte)DAT_143ae2c20 & 2) == 0)) {
      if (0x80 < param_3) {
        lVar30 = ((ulonglong)param_1 & 0xf) - 0x10;
        param_1 = (undefined8 *)((longlong)param_1 - lVar30);
        param_2 = (undefined8 *)((longlong)param_2 - lVar30);
        param_3 = param_3 + lVar30;
        if (0x80 < param_3) {
          do {
            uVar10 = param_2[1];
            uVar11 = param_2[2];
            uVar32 = param_2[3];
            uVar33 = param_2[4];
            uVar12 = param_2[5];
            uVar13 = param_2[6];
            uVar14 = param_2[7];
            *param_1 = *param_2;
            param_1[1] = uVar10;
            param_1[2] = uVar11;
            param_1[3] = uVar32;
            param_1[4] = uVar33;
            param_1[5] = uVar12;
            param_1[6] = uVar13;
            param_1[7] = uVar14;
            uVar10 = param_2[9];
            uVar11 = param_2[10];
            uVar32 = param_2[0xb];
            uVar33 = param_2[0xc];
            uVar12 = param_2[0xd];
            uVar13 = param_2[0xe];
            uVar14 = param_2[0xf];
            param_1[8] = param_2[8];
            param_1[9] = uVar10;
            param_1[10] = uVar11;
            param_1[0xb] = uVar32;
            param_1[0xc] = uVar33;
            param_1[0xd] = uVar12;
            param_1[0xe] = uVar13;
            param_1[0xf] = uVar14;
            param_1 = param_1 + 0x10;
            param_2 = param_2 + 0x10;
            param_3 = param_3 - 0x80;
          } while (0x7f < param_3);
        }
      }
                    /* WARNING: Could not recover jumptable at 0x000142ef80b6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      puVar23 = (undefined8 *)
                (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                          *(uint *)(&DAT_143c47088 + (param_3 + 0xf >> 4) * 4)))();
      return puVar23;
    }
  }
  else if (((param_3 < 0x2001) || (0x180000 < param_3)) || (((byte)DAT_143ae2c20 & 2) == 0)) {
    uVar10 = *param_2;
    uVar11 = param_2[1];
    uVar32 = param_2[2];
    uVar33 = param_2[3];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x20));
    uVar12 = *puVar26;
    uVar13 = puVar26[1];
    uVar14 = puVar26[2];
    uVar15 = puVar26[3];
    if (0x100 < param_3) {
      lVar30 = ((ulonglong)param_1 & 0x1f) - 0x20;
      pauVar24 = (undefined1 (*) [32])((longlong)param_1 - lVar30);
      pauVar27 = (undefined1 (*) [32])((longlong)param_2 - lVar30);
      param_3 = param_3 + lVar30;
      if (0x100 < param_3) {
        if (0x180000 < param_3) {
          do {
            uVar29 = param_3;
            pauVar28 = pauVar27;
            pauVar25 = pauVar24;
            auVar3 = pauVar28[1];
            auVar4 = pauVar28[2];
            auVar5 = pauVar28[3];
            auVar6 = vmovntdq_avx(*pauVar28);
            *pauVar25 = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[1] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[2] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[3] = auVar3;
            auVar3 = pauVar28[5];
            auVar4 = pauVar28[6];
            auVar5 = pauVar28[7];
            auVar6 = vmovntdq_avx(pauVar28[4]);
            pauVar25[4] = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[5] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[6] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[7] = auVar3;
            pauVar24 = pauVar25 + 8;
            pauVar27 = pauVar28 + 8;
            param_3 = uVar29 - 0x100;
          } while (0xff < uVar29 - 0x100);
          uVar31 = uVar29 - 0xe1 & 0xffffffffffffffe0;
          switch(uVar29) {
          case 0x1e1:
          case 0x1e2:
          case 0x1e3:
          case 0x1e4:
          case 0x1e5:
          case 0x1e6:
          case 0x1e7:
          case 0x1e8:
          case 0x1e9:
          case 0x1ea:
          case 0x1eb:
          case 0x1ec:
          case 0x1ed:
          case 0x1ee:
          case 0x1ef:
          case 0x1f0:
          case 0x1f1:
          case 0x1f2:
          case 499:
          case 500:
          case 0x1f5:
          case 0x1f6:
          case 0x1f7:
          case 0x1f8:
          case 0x1f9:
          case 0x1fa:
          case 0x1fb:
          case 0x1fc:
          case 0x1fd:
          case 0x1fe:
          case 0x1ff:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(*pauVar28 + uVar31));
            *(undefined1 (*) [32])(*pauVar25 + uVar31) = auVar3;
          case 0x1c1:
          case 0x1c2:
          case 0x1c3:
          case 0x1c4:
          case 0x1c5:
          case 0x1c6:
          case 0x1c7:
          case 0x1c8:
          case 0x1c9:
          case 0x1ca:
          case 0x1cb:
          case 0x1cc:
          case 0x1cd:
          case 0x1ce:
          case 0x1cf:
          case 0x1d0:
          case 0x1d1:
          case 0x1d2:
          case 0x1d3:
          case 0x1d4:
          case 0x1d5:
          case 0x1d6:
          case 0x1d7:
          case 0x1d8:
          case 0x1d9:
          case 0x1da:
          case 0x1db:
          case 0x1dc:
          case 0x1dd:
          case 0x1de:
          case 0x1df:
          case 0x1e0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[1] + uVar31));
            *(undefined1 (*) [32])(pauVar25[1] + uVar31) = auVar3;
          case 0x1a1:
          case 0x1a2:
          case 0x1a3:
          case 0x1a4:
          case 0x1a5:
          case 0x1a6:
          case 0x1a7:
          case 0x1a8:
          case 0x1a9:
          case 0x1aa:
          case 0x1ab:
          case 0x1ac:
          case 0x1ad:
          case 0x1ae:
          case 0x1af:
          case 0x1b0:
          case 0x1b1:
          case 0x1b2:
          case 0x1b3:
          case 0x1b4:
          case 0x1b5:
          case 0x1b6:
          case 0x1b7:
          case 0x1b8:
          case 0x1b9:
          case 0x1ba:
          case 0x1bb:
          case 0x1bc:
          case 0x1bd:
          case 0x1be:
          case 0x1bf:
          case 0x1c0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[2] + uVar31));
            *(undefined1 (*) [32])(pauVar25[2] + uVar31) = auVar3;
          case 0x181:
          case 0x182:
          case 0x183:
          case 0x184:
          case 0x185:
          case 0x186:
          case 0x187:
          case 0x188:
          case 0x189:
          case 0x18a:
          case 0x18b:
          case 0x18c:
          case 0x18d:
          case 0x18e:
          case 399:
          case 400:
          case 0x191:
          case 0x192:
          case 0x193:
          case 0x194:
          case 0x195:
          case 0x196:
          case 0x197:
          case 0x198:
          case 0x199:
          case 0x19a:
          case 0x19b:
          case 0x19c:
          case 0x19d:
          case 0x19e:
          case 0x19f:
          case 0x1a0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[3] + uVar31));
            *(undefined1 (*) [32])(pauVar25[3] + uVar31) = auVar3;
          case 0x161:
          case 0x162:
          case 0x163:
          case 0x164:
          case 0x165:
          case 0x166:
          case 0x167:
          case 0x168:
          case 0x169:
          case 0x16a:
          case 0x16b:
          case 0x16c:
          case 0x16d:
          case 0x16e:
          case 0x16f:
          case 0x170:
          case 0x171:
          case 0x172:
          case 0x173:
          case 0x174:
          case 0x175:
          case 0x176:
          case 0x177:
          case 0x178:
          case 0x179:
          case 0x17a:
          case 0x17b:
          case 0x17c:
          case 0x17d:
          case 0x17e:
          case 0x17f:
          case 0x180:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[4] + uVar31));
            *(undefined1 (*) [32])(pauVar25[4] + uVar31) = auVar3;
          case 0x141:
          case 0x142:
          case 0x143:
          case 0x144:
          case 0x145:
          case 0x146:
          case 0x147:
          case 0x148:
          case 0x149:
          case 0x14a:
          case 0x14b:
          case 0x14c:
          case 0x14d:
          case 0x14e:
          case 0x14f:
          case 0x150:
          case 0x151:
          case 0x152:
          case 0x153:
          case 0x154:
          case 0x155:
          case 0x156:
          case 0x157:
          case 0x158:
          case 0x159:
          case 0x15a:
          case 0x15b:
          case 0x15c:
          case 0x15d:
          case 0x15e:
          case 0x15f:
          case 0x160:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[5] + uVar31));
            *(undefined1 (*) [32])(pauVar25[5] + uVar31) = auVar3;
          case 0x121:
          case 0x122:
          case 0x123:
          case 0x124:
          case 0x125:
          case 0x126:
          case 0x127:
          case 0x128:
          case 0x129:
          case 0x12a:
          case 299:
          case 300:
          case 0x12d:
          case 0x12e:
          case 0x12f:
          case 0x130:
          case 0x131:
          case 0x132:
          case 0x133:
          case 0x134:
          case 0x135:
          case 0x136:
          case 0x137:
          case 0x138:
          case 0x139:
          case 0x13a:
          case 0x13b:
          case 0x13c:
          case 0x13d:
          case 0x13e:
          case 0x13f:
          case 0x140:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[6] + uVar31));
            *(undefined1 (*) [32])(pauVar25[6] + uVar31) = auVar3;
          default:
            puVar26 = (undefined8 *)(pauVar25[-1] + uVar29);
            *puVar26 = uVar12;
            puVar26[1] = uVar13;
            puVar26[2] = uVar14;
            puVar26[3] = uVar15;
          case 0x100:
            *param_1 = uVar10;
            param_1[1] = uVar11;
            param_1[2] = uVar32;
            param_1[3] = uVar33;
            return puVar23;
          }
        }
        do {
          uVar10 = *(undefined8 *)(*pauVar27 + 8);
          uVar11 = *(undefined8 *)(*pauVar27 + 0x10);
          uVar32 = *(undefined8 *)(*pauVar27 + 0x18);
          uVar33 = *(undefined8 *)pauVar27[1];
          uVar12 = *(undefined8 *)(pauVar27[1] + 8);
          uVar13 = *(undefined8 *)(pauVar27[1] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[1] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[2];
          uVar16 = *(undefined8 *)(pauVar27[2] + 8);
          uVar17 = *(undefined8 *)(pauVar27[2] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[2] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[3];
          uVar20 = *(undefined8 *)(pauVar27[3] + 8);
          uVar21 = *(undefined8 *)(pauVar27[3] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[3] + 0x18);
          *(undefined8 *)*pauVar24 = *(undefined8 *)*pauVar27;
          *(undefined8 *)(*pauVar24 + 8) = uVar10;
          *(undefined8 *)(*pauVar24 + 0x10) = uVar11;
          *(undefined8 *)(*pauVar24 + 0x18) = uVar32;
          *(undefined8 *)pauVar24[1] = uVar33;
          *(undefined8 *)(pauVar24[1] + 8) = uVar12;
          *(undefined8 *)(pauVar24[1] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[1] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[2] = uVar15;
          *(undefined8 *)(pauVar24[2] + 8) = uVar16;
          *(undefined8 *)(pauVar24[2] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[2] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[3] = uVar19;
          *(undefined8 *)(pauVar24[3] + 8) = uVar20;
          *(undefined8 *)(pauVar24[3] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[3] + 0x18) = uVar22;
          uVar10 = *(undefined8 *)(pauVar27[4] + 8);
          uVar11 = *(undefined8 *)(pauVar27[4] + 0x10);
          uVar32 = *(undefined8 *)(pauVar27[4] + 0x18);
          uVar33 = *(undefined8 *)pauVar27[5];
          uVar12 = *(undefined8 *)(pauVar27[5] + 8);
          uVar13 = *(undefined8 *)(pauVar27[5] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[5] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[6];
          uVar16 = *(undefined8 *)(pauVar27[6] + 8);
          uVar17 = *(undefined8 *)(pauVar27[6] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[6] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[7];
          uVar20 = *(undefined8 *)(pauVar27[7] + 8);
          uVar21 = *(undefined8 *)(pauVar27[7] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[7] + 0x18);
          *(undefined8 *)pauVar24[4] = *(undefined8 *)pauVar27[4];
          *(undefined8 *)(pauVar24[4] + 8) = uVar10;
          *(undefined8 *)(pauVar24[4] + 0x10) = uVar11;
          *(undefined8 *)(pauVar24[4] + 0x18) = uVar32;
          *(undefined8 *)pauVar24[5] = uVar33;
          *(undefined8 *)(pauVar24[5] + 8) = uVar12;
          *(undefined8 *)(pauVar24[5] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[5] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[6] = uVar15;
          *(undefined8 *)(pauVar24[6] + 8) = uVar16;
          *(undefined8 *)(pauVar24[6] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[6] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[7] = uVar19;
          *(undefined8 *)(pauVar24[7] + 8) = uVar20;
          *(undefined8 *)(pauVar24[7] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[7] + 0x18) = uVar22;
          pauVar24 = pauVar24 + 8;
          pauVar27 = pauVar27 + 8;
          param_3 = param_3 - 0x100;
        } while (0xff < param_3);
      }
    }
                    /* WARNING: Could not recover jumptable at 0x000142ef7e12. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    puVar23 = (undefined8 *)
              (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                        *(uint *)(&DAT_143c47040 + (param_3 + 0x1f >> 5) * 4)))();
    return puVar23;
  }
  for (; param_3 != 0; param_3 = param_3 - 1) {
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    param_2 = (undefined8 *)((longlong)param_2 + 1);
    param_1 = (undefined8 *)((longlong)param_1 + 1);
  }
  return puVar23;
}



//===========================================================
// FUN_142e54290 @ 142e54290   (185 bytes)
//===========================================================

void FUN_142e54290(undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  undefined4 local_res20 [2];
  longlong local_18 [3];
  
  local_res8[0] = param_1;
  local_res10[0] = param_2;
  local_res18[0] = param_3;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(local_18);
    local_res20[0] = FUN_14091a3e0(local_18);
    FUN_142e5c990("LogCallStack5",&DAT_1434997dc,local_res20,&DAT_1434997f8,local_res8,"Info1",
                  local_res10,"info2",local_res18,local_18);
    if (local_18[0] != 0) {
      FUN_14019f2c0(local_18[0] + -0x10);
    }
  }
  return;
}



//===========================================================
// memcmp @ 142ef7aa0   (198 bytes)
//===========================================================

/* Library Function - Single Match
    memcmp
   
   Library: Visual Studio */

int __cdecl memcmp(void *_Buf1,void *_Buf2,size_t _Size)

{
  uint uVar1;
  ulonglong uVar2;
  longlong lVar3;
  ulonglong uVar4;
  bool bVar5;
  
  lVar3 = (longlong)_Buf2 - (longlong)_Buf1;
  if (7 < _Size) {
    for (; ((ulonglong)_Buf1 & 7) != 0; _Buf1 = (void *)((longlong)_Buf1 + 1)) {
      bVar5 = (byte)*(ulonglong *)_Buf1 < *(byte *)((longlong)_Buf1 + lVar3);
      if ((byte)*(ulonglong *)_Buf1 != *(byte *)((longlong)_Buf1 + lVar3)) goto LAB_142ef7ae3;
      _Size = _Size - 1;
    }
    if (_Size >> 3 != 0) {
      uVar4 = _Size >> 5;
      if (uVar4 != 0) {
        do {
          uVar2 = *(ulonglong *)_Buf1;
          if (uVar2 != *(ulonglong *)((longlong)_Buf1 + lVar3)) goto LAB_142ef7b54;
          uVar2 = *(ulonglong *)((longlong)_Buf1 + 8);
          if (uVar2 != *(ulonglong *)((longlong)_Buf1 + lVar3 + 8)) {
LAB_142ef7b50:
            _Buf1 = (void *)((longlong)_Buf1 + 8);
            goto LAB_142ef7b54;
          }
          uVar2 = *(ulonglong *)((longlong)_Buf1 + 0x10);
          if (uVar2 != *(ulonglong *)((longlong)_Buf1 + lVar3 + 0x10)) {
LAB_142ef7b4c:
            _Buf1 = (void *)((longlong)_Buf1 + 8);
            goto LAB_142ef7b50;
          }
          uVar2 = *(ulonglong *)((longlong)_Buf1 + 0x18);
          if (uVar2 != *(ulonglong *)((longlong)_Buf1 + lVar3 + 0x18)) {
            _Buf1 = (void *)((longlong)_Buf1 + 8);
            goto LAB_142ef7b4c;
          }
          _Buf1 = (void *)((longlong)_Buf1 + 0x20);
          uVar4 = uVar4 - 1;
        } while (uVar4 != 0);
        _Size = _Size & 0x1f;
      }
      uVar4 = _Size >> 3;
      if (uVar4 != 0) {
        do {
          uVar2 = *(ulonglong *)_Buf1;
          if (uVar2 != *(ulonglong *)((longlong)_Buf1 + lVar3)) {
LAB_142ef7b54:
            uVar4 = *(ulonglong *)(lVar3 + (longlong)_Buf1);
            uVar1 = (uint)((uVar2 >> 0x38 | (uVar2 & 0xff000000000000) >> 0x28 |
                            (uVar2 & 0xff0000000000) >> 0x18 | (uVar2 & 0xff00000000) >> 8 |
                            (uVar2 & 0xff000000) << 8 | (uVar2 & 0xff0000) << 0x18 |
                            (uVar2 & 0xff00) << 0x28 | uVar2 << 0x38) <
                          (uVar4 >> 0x38 | (uVar4 & 0xff000000000000) >> 0x28 |
                           (uVar4 & 0xff0000000000) >> 0x18 | (uVar4 & 0xff00000000) >> 8 |
                           (uVar4 & 0xff000000) << 8 | (uVar4 & 0xff0000) << 0x18 |
                           (uVar4 & 0xff00) << 0x28 | uVar4 << 0x38));
            return (1 - uVar1) - (uint)(uVar1 != 0);
          }
          _Buf1 = (void *)((longlong)_Buf1 + 8);
          uVar4 = uVar4 - 1;
        } while (uVar4 != 0);
        _Size = _Size & 7;
      }
    }
  }
  while( true ) {
    if (_Size == 0) {
      return 0;
    }
    bVar5 = (byte)*(ulonglong *)_Buf1 < *(byte *)((longlong)_Buf1 + lVar3);
    if ((byte)*(ulonglong *)_Buf1 != *(byte *)((longlong)_Buf1 + lVar3)) break;
    _Buf1 = (void *)((longlong)_Buf1 + 1);
    _Size = _Size - 1;
  }
LAB_142ef7ae3:
  return (1 - (uint)bVar5) - (uint)(bVar5 != 0);
}



//===========================================================
// FUN_142e52dd0 @ 142e52dd0   (245 bytes)
//===========================================================

void FUN_142e52dd0(undefined4 param_1,undefined4 param_2)

{
  undefined8 uVar1;
  char cVar2;
  undefined4 local_res8 [2];
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  longlong local_res20;
  longlong local_18;
  longlong local_10 [2];
  
  local_res8[0] = param_1;
  local_res10[0] = param_2;
  cVar2 = FUN_142e559e0();
  if (cVar2 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    uVar1 = FUN_142a1d8a0(local_10);
    uVar1 = FUN_142e5d800(&local_18,uVar1,"LogCallStack2",&DAT_1434997dc,local_res18,&DAT_1434997f8,
                          local_res8,"Info1",local_res10,&local_res20);
    FUN_142a1ec10(uVar1);
    if (local_18 != 0) {
      FUN_14019f2c0(local_18 + -0x10);
    }
    if (local_10[0] != 0) {
      FUN_14019f2c0(local_10[0] + -0x10);
    }
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1401bebb0 @ 1401bebb0   (347 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1401bebb0(int *param_1)

{
  int iVar1;
  code *UNRECOVERED_JUMPTABLE;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  int *piVar5;
  uint uVar6;
  longlong lVar7;
  bool bVar8;
  
  if (0x100000 < *param_1 + 1U) {
    FUN_142e52dd0(0x23);
  }
  LOCK();
  iVar1 = *param_1;
  *param_1 = *param_1 + -1;
  UNLOCK();
  if (1 < iVar1) {
    return;
  }
  if (*param_1 != 0) {
    FUN_142e52dd0(0x32);
  }
  pvVar2 = Self;
  UNRECOVERED_JUMPTABLE = DAT_143ad5530;
  uVar3 = *(ulonglong *)(param_1 + -2);
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x61) {
    uVar6 = (uint)(0x40 < uVar3);
  }
  else {
    if (uVar3 < 0xa1) {
      uVar6 = 2;
      goto LAB_1401bec24;
    }
    uVar6 = 0xffffffff;
    if (uVar3 < 0x121) {
      uVar6 = 3;
    }
  }
  if ((int)uVar6 < 0) {
    uVar4 = (*DAT_143ad5538)();
                    /* WARNING: Could not recover jumptable at 0x0001401bec98. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(uVar4,0,param_1 + -2);
    return;
  }
LAB_1401bec24:
  uVar3 = (ulonglong)uVar6;
  lVar7 = (ulonglong)uVar6 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad69a8 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad69a8 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_1401becc5:
    *(undefined4 *)(&DAT_143ad69b0 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad69a8 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad69a8 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad69a8 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_1401becc5;
        if (*(void **)(&DAT_143ad69a8 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad69b0 + lVar7) = *(int *)(&DAT_143ad69b0 + lVar7) + 1;
  }
  piVar5 = (int *)(&DAT_143ad69b0 + lVar7);
  *(undefined8 *)param_1 = *(undefined8 *)(&DAT_143ad69e8 + uVar3 * 8);
  *(int **)(&DAT_143ad69e8 + uVar3 * 8) = param_1;
  _DAT_143ad6a28 = *(undefined8 *)param_1;
  *(int *)(&DAT_143ad6994 + uVar3 * 4) = *(int *)(&DAT_143ad6994 + uVar3 * 4) + -1;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *(undefined8 *)(&DAT_143ad69a8 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_142e52d50 @ 142e52d50   (119 bytes)
//===========================================================

void FUN_142e52d50(undefined4 param_1)

{
  char cVar1;
  undefined4 local_res8 [4];
  undefined4 local_res18 [2];
  longlong local_res20;
  
  local_res8[0] = param_1;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    FUN_142e5d030("LogCallStack1",&DAT_1434997dc,local_res18,&DAT_1434997f8,local_res8,&local_res20)
    ;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1401bc720 @ 1401bc720   (378 bytes)
//===========================================================

void FUN_1401bc720(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  int iVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar6 = 0x120;
  if (param_2 < 0x61) {
    uVar10 = (uint)(0x40 < param_2);
LAB_1401bc782:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar7 = 0x40;
      uVar6 = 0x40;
      goto LAB_1401bc7c9;
    }
    if (uVar10 == 1) {
      iVar7 = 0x20;
      uVar6 = 0x60;
      goto LAB_1401bc7c9;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar7 = 8;
      }
      else {
        iVar7 = 0;
        uVar6 = 0;
      }
      goto LAB_1401bc7c9;
    }
  }
  else {
    if (0xa0 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x121) {
        uVar10 = 3;
      }
      goto LAB_1401bc782;
    }
    uVar10 = 2;
  }
  iVar7 = 0x10;
  uVar6 = 0xa0;
LAB_1401bc7c9:
  lVar11 = (longlong)(int)uVar10;
  lVar8 = lVar11 * 0x10 + param_1;
  plVar1 = (longlong *)(lVar8 + 0x28);
  LOCK();
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar4 == 0) {
LAB_1401bc829:
    *(undefined4 *)(lVar8 + 0x30) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar4 = *plVar1;
      if (lVar4 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar4 == 0) goto LAB_1401bc829;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar6,iVar7);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar7;
    puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  }
  piVar2 = (int *)(param_1 + 0x14 + lVar11 * 4);
  *piVar2 = *piVar2 + 1;
  *(undefined8 *)(param_1 + 0x68 + lVar11 * 8) = *puVar5;
  *piVar9 = *piVar9 + -1;
  if (*piVar9 == 0) {
    *plVar1 = 0;
  }
  return;
}



//===========================================================
// FUN_142c4a030 @ 142c4a030   (4 bytes)
//===========================================================

undefined4 FUN_142c4a030(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x60);
}



//===========================================================
// FUN_141d5f6e0 @ 141d5f6e0   (340 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141d5f6e0(int param_1,undefined8 param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  undefined1 auStack_88 [32];
  undefined8 local_68 [2];
  undefined8 *local_58;
  undefined8 uStack_50;
  undefined8 *local_48;
  uint uStack_40;
  undefined4 uStack_3c;
  undefined8 local_30;
  undefined8 uStack_28;
  undefined8 local_20;
  undefined8 uStack_18;
  ulonglong local_10;
  
  local_10 = DAT_143a8b908 ^ (ulonglong)auStack_88;
  local_30 = 0;
  uStack_28 = 0;
  local_20 = 0;
  uStack_18 = 0;
  local_68[0] = param_2;
  FUN_14075a550(local_68,&local_30);
  puVar3 = DAT_143ad2cc0;
  puVar4 = (undefined8 *)DAT_143ad2cc0[1];
  uStack_40 = 0;
  cVar1 = *(char *)((longlong)puVar4 + 0x19);
  local_48 = puVar4;
  puVar5 = DAT_143ad2cc0;
  while (puVar2 = puVar4, cVar1 == '\0') {
    if (param_1 <= *(int *)(puVar2 + 4)) {
      puVar4 = (undefined8 *)*puVar2;
      puVar5 = puVar2;
    }
    else {
      puVar4 = (undefined8 *)puVar2[2];
    }
    uStack_40 = (uint)(param_1 <= *(int *)(puVar2 + 4));
    cVar1 = *(char *)((longlong)puVar4 + 0x19);
    local_48 = puVar2;
  }
  if ((*(char *)((longlong)puVar5 + 0x19) != '\0') || (param_1 < *(int *)(puVar5 + 4))) {
    if (DAT_143ad2cc8 == 0x38e38e38e38e38e) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    local_58 = &DAT_143ad2cc0;
    uStack_50 = 0;
    puVar4 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x48);
    *(int *)(puVar4 + 4) = param_1;
    puVar4[5] = 0;
    puVar4[6] = 0;
    puVar4[7] = 0;
    puVar4[8] = 0;
    *puVar4 = puVar3;
    puVar4[1] = puVar3;
    puVar4[2] = puVar3;
    *(undefined2 *)(puVar4 + 3) = 0;
    local_58 = local_48;
    uStack_50 = CONCAT44(uStack_3c,uStack_40);
    puVar5 = (undefined8 *)FUN_141d600c0(&DAT_143ad2cc0,&local_58,puVar4);
  }
  puVar5[5] = local_30;
  puVar5[6] = uStack_28;
  puVar5[7] = local_20;
  puVar5[8] = uStack_18;
  return;
}



//===========================================================
// FUN_141d5f870 @ 141d5f870   (181 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_141d5f870(int param_1)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined1 auStack_58 [32];
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  ulonglong local_10;
  
  local_10 = DAT_143a8b908 ^ (ulonglong)auStack_58;
  cVar1 = *(char *)((longlong)DAT_143ad2cc0[1] + 0x19);
  puVar3 = (undefined8 *)DAT_143ad2cc0[1];
  puVar2 = DAT_143ad2cc0;
  while (puVar4 = puVar3, cVar1 == '\0') {
    if (*(int *)(puVar4 + 4) < param_1) {
      puVar3 = (undefined8 *)puVar4[2];
      puVar4 = puVar2;
    }
    else {
      puVar3 = (undefined8 *)*puVar4;
    }
    cVar1 = *(char *)((longlong)puVar3 + 0x19);
    puVar2 = puVar4;
  }
  if (((*(char *)((longlong)puVar2 + 0x19) == '\0') && (*(int *)(puVar2 + 4) <= param_1)) &&
     (puVar2 != DAT_143ad2cc0)) {
    local_30 = puVar2[5];
    local_28 = puVar2[6];
    local_20 = puVar2[7];
    local_18 = puVar2[8];
    local_38 = 0;
    FUN_14075a6d0(&local_38,&local_30);
    return local_38;
  }
  return 0;
}



//===========================================================
// FUN_140738830 @ 140738830   (12 bytes)
//===========================================================

void FUN_140738830(void)

{
  FUN_1407386b0(&DAT_143ac1ab0);
  return;
}



//===========================================================
// FUN_142a1d8a0 @ 142a1d8a0   (4667 bytes)
//===========================================================

ulonglong FUN_142a1d8a0(ulonglong param_1)

{
  longlong *plVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  longlong lVar8;
  int iVar9;
  int *piVar10;
  int iVar11;
  int *piVar12;
  ulonglong uVar13;
  int *piVar14;
  longlong lVar15;
  int iVar16;
  int *piVar17;
  longlong local_res10;
  undefined4 local_res18 [2];
  undefined4 local_res20;
  undefined4 uStackX_24;
  int *local_c0;
  longlong local_b8;
  undefined4 local_b0;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined4 local_a4;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  undefined8 local_68;
  longlong local_60;
  undefined8 local_58 [3];
  
  piVar17 = (int *)0x0;
  iVar11 = 0;
  uVar2 = FUN_142c4a030(DAT_143ac1898);
  local_res10 = CONCAT44(local_res10._4_4_,uVar2);
  local_68 = FUN_1408f6690();
  local_res18[0] = (*DAT_143262db0)();
  local_res20 = FUN_142c50c50();
  local_b0 = FUN_141892a90();
  uVar3 = FUN_1408fadb0(&local_60,&local_68);
  local_ac = 100;
  FUN_142a24230(param_1,"VERSION",&local_ac,"DATETIME",uVar3,&DAT_143487774,&local_b0,"LastUseName",
                &DAT_143adc708,"State",&local_res20,"Time1",local_res18,"Time2",&local_res10);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  plVar1 = DAT_143aa84a0;
  if (DAT_143aa84a0 == (longlong *)0x0) {
    uVar3 = FUN_142a245b0(&local_res20,param_1,&DAT_143273ad4,&DAT_143271f04,"Channel",
                          &DAT_143271f04,&DAT_143273ac0,&DAT_143271f04,&DAT_143487790,&DAT_143271f04
                         );
    FUN_140319ad0(param_1,uVar3);
    if (CONCAT44(uStackX_24,local_res20) != 0) {
      FUN_14019f2c0(CONCAT44(uStackX_24,local_res20) + -0x10);
    }
    goto LAB_142a1ea24;
  }
  local_a4 = (**(code **)(*DAT_143aa84a0 + 0xa8))(DAT_143aa84a0);
  local_58[0] = FUN_142cb9610(plVar1);
  local_a8 = FUN_142cb9260(plVar1);
  local_res20 = FUN_142cb9230(plVar1);
  local_c0 = (int *)0x0;
  FUN_1408bc980(&local_88,param_1);
  lVar8 = local_88;
  piVar10 = piVar17;
  if (local_88 != 0) {
    iVar16 = *(int *)(local_88 + -8);
    piVar12 = (int *)(longlong)iVar16;
    if (iVar16 != 0) {
      iVar7 = 0;
      if (0 < iVar16) {
        iVar7 = iVar16;
      }
      piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
      piVar4[1] = iVar7;
      *piVar4 = -1;
      piVar10 = piVar4 + 4;
      piVar4[2] = 0;
      *(char *)piVar10 = '\0';
      local_c0 = piVar10;
      FUN_142ef7ba0(piVar10,lVar8,piVar12);
      if (*piVar4 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= piVar4[1])) {
        *piVar4 = 1;
        if (iVar16 != -1) goto LAB_142a1dab0;
        piVar12 = piVar17;
        if (piVar10 != (int *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (*(char *)((longlong)piVar10 + (longlong)piVar12) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar4[1],iVar16);
        *piVar4 = 1;
LAB_142a1dab0:
        *(char *)((longlong)piVar12 + (longlong)piVar10) = '\0';
      }
      iVar16 = (int)piVar12;
      if ((iVar16 < 0) || (piVar4[1] + 1 <= iVar16)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      piVar4[2] = iVar16;
    }
  }
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  local_80 = 0;
  uVar3 = FUN_14019ba10(&local_80,&DAT_143272338,&DAT_143273ad4);
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  lVar8 = local_b8;
  piVar12 = piVar10;
  if (local_b8 != 0) {
    iVar16 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar16;
    if (iVar16 != 0) {
      piVar14 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1dcdf;
      if ((char)*piVar10 != '\0') {
        iVar7 = piVar10[-2];
        for (iVar9 = piVar10[-3]; iVar9 < iVar7 + iVar16; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar10 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1dbdf:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar5[1] = iVar11;
          *puVar5 = 0xffffffff;
          piVar12 = puVar5 + 4;
          local_c0 = piVar12;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar12 = '\0';
          }
          else {
            iVar9 = piVar10[-2] + 1;
            if (iVar11 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar11 + 1);
              iVar9 = iVar11 + 1;
            }
            FUN_142ef7ba0(piVar12,piVar10,(longlong)iVar9);
            puVar5[2] = piVar10[-2];
            *(char *)((longlong)iVar11 + (longlong)piVar12) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar10[-3] < iVar9)) {
            iVar11 = piVar10[-2];
            goto LAB_142a1dbdf;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        iVar11 = 0;
        if (piVar12 != (int *)0x0) {
          iVar11 = piVar12[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar11 + (longlong)piVar12),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar7 + iVar16);
        goto LAB_142a1dd80;
      }
      if ((piVar10 == (int *)0x0) || (piVar14 = piVar10 + -4, piVar14 == (int *)0x0)) {
LAB_142a1dcdf:
        if (iVar11 < iVar16) {
          iVar11 = iVar16;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
        puVar5[1] = iVar11;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar10[-3] < iVar16)) {
          iVar11 = piVar10[-2];
          goto LAB_142a1dcdf;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,piVar4);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar16 != -1) goto LAB_142a1dd5d;
        if (piVar12 != (int *)0x0) {
          piVar17 = (int *)0xffffffffffffffff;
          do {
            piVar17 = (int *)((longlong)piVar17 + 1);
          } while (*(char *)((longlong)piVar12 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar16);
        piVar12[-4] = 1;
LAB_142a1dd5d:
        *(char *)((longlong)piVar4 + (longlong)piVar12) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1dd80:
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc080(&local_a0,&local_res20);
  lVar8 = local_a0;
  if (local_a0 != 0) {
    iVar11 = *(int *)(local_a0 + -8);
    uVar13 = (ulonglong)iVar11;
    if (iVar11 != 0) {
      if (piVar12 == (int *)0x0) {
LAB_142a1df3d:
        piVar17 = (int *)0x0;
LAB_142a1df3f:
        iVar16 = 0;
LAB_142a1df41:
        if (iVar16 < iVar11) {
          iVar16 = iVar11;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar17 != (int *)0x0) {
          FUN_14019f2c0(piVar17);
        }
      }
      else {
        if ((char)*piVar12 != '\0') {
          iVar16 = piVar12[-2];
          for (iVar7 = piVar12[-3]; iVar7 < iVar16 + iVar11; iVar7 = iVar7 * 2) {
          }
          piVar17 = piVar12 + -4;
          if (piVar17 == (int *)0x0) {
            iVar9 = 0;
LAB_142a1de4f:
            if (iVar9 < iVar7) {
              iVar9 = iVar7;
            }
            puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
            puVar5[1] = iVar9;
            *puVar5 = 0xffffffff;
            piVar10 = puVar5 + 4;
            local_c0 = piVar10;
            if (piVar17 == (int *)0x0) {
              puVar5[2] = 0;
              *(char *)piVar10 = '\0';
            }
            else {
              iVar7 = piVar12[-2] + 1;
              if (iVar9 + 1 < iVar7) {
                FUN_142e54290(0x5c,iVar7,iVar9 + 1);
                iVar7 = iVar9 + 1;
              }
              FUN_142ef7ba0(piVar10,piVar12,(longlong)iVar7);
              puVar5[2] = piVar12[-2];
              *(char *)((longlong)iVar9 + (longlong)piVar10) = '\0';
              FUN_14019f2c0(piVar17);
            }
          }
          else {
            if ((1 < *piVar17) || (piVar12[-3] < iVar7)) {
              iVar9 = piVar12[-2];
              goto LAB_142a1de4f;
            }
            if (*piVar17 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar17 = -1;
            piVar10 = piVar12;
          }
          if (piVar10 == (int *)0x0) {
            iVar7 = 0;
          }
          else {
            iVar7 = piVar10[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar7 + (longlong)piVar10),lVar8,uVar13);
          FUN_14019c870(&local_c0,iVar16 + iVar11);
          piVar12 = piVar10;
          goto LAB_142a1dfe9;
        }
        if (piVar12 == (int *)0x0) goto LAB_142a1df3d;
        piVar17 = piVar12 + -4;
        if (piVar17 == (int *)0x0) goto LAB_142a1df3f;
        if ((1 < *piVar17) || (piVar12[-3] < iVar11)) {
          iVar16 = piVar12[-2];
          goto LAB_142a1df41;
        }
        if (*piVar17 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar17 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,uVar13);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar11 != -1) goto LAB_142a1dfc2;
        if (piVar12 == (int *)0x0) {
          uVar13 = 0;
        }
        else {
          uVar13 = 0xffffffffffffffff;
          do {
            uVar13 = uVar13 + 1;
          } while (*(char *)((longlong)piVar12 + uVar13) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar11);
        piVar12[-4] = 1;
LAB_142a1dfc2:
        *(char *)(uVar13 + (longlong)piVar12) = '\0';
      }
      iVar11 = (int)uVar13;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1dfe9:
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  iVar16 = 0;
  piVar17 = (int *)0x0;
  iVar11 = 0;
  local_78 = 0;
  uVar3 = FUN_14019ba10(&local_78,&DAT_143272338,"Channel");
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  lVar8 = local_b8;
  piVar10 = piVar12;
  if (local_b8 != 0) {
    iVar7 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar7;
    if (iVar7 != 0) {
      piVar14 = piVar17;
      if (piVar12 == (int *)0x0) goto LAB_142a1e1de;
      if ((char)*piVar12 != '\0') {
        iVar16 = piVar12[-2];
        for (iVar9 = piVar12[-3]; iVar9 < iVar16 + iVar7; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar12 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1e0e9:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar5[1] = iVar11;
          *puVar5 = 0xffffffff;
          piVar10 = puVar5 + 4;
          local_c0 = piVar10;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar10 = '\0';
          }
          else {
            iVar9 = piVar12[-2] + 1;
            if (iVar11 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar11 + 1);
              iVar9 = iVar11 + 1;
            }
            FUN_142ef7ba0(piVar10,piVar12,(longlong)iVar9);
            puVar5[2] = piVar12[-2];
            *(char *)((longlong)iVar11 + (longlong)piVar10) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar12[-3] < iVar9)) {
            iVar11 = piVar12[-2];
            goto LAB_142a1e0e9;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        if (piVar10 == (int *)0x0) {
          iVar11 = 0;
        }
        else {
          iVar11 = piVar10[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar11 + (longlong)piVar10),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar16 + iVar7);
        goto LAB_142a1e282;
      }
      if ((piVar12 == (int *)0x0) || (piVar14 = piVar12 + -4, piVar14 == (int *)0x0)) {
LAB_142a1e1de:
        if (iVar16 < iVar7) {
          iVar16 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar10 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar10 = '\0';
        local_c0 = piVar10;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar12[-3] < iVar7)) {
          iVar16 = piVar12[-2];
          goto LAB_142a1e1de;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar10,lVar8,piVar4);
      if (piVar10[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar10[-3])) {
        piVar10[-4] = 1;
        if (iVar7 != -1) goto LAB_142a1e25f;
        if (piVar10 != (int *)0x0) {
          piVar17 = (int *)0xffffffffffffffff;
          do {
            piVar17 = (int *)((longlong)piVar17 + 1);
          } while (*(char *)((longlong)piVar10 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar10[-3],iVar7);
        piVar10[-4] = 1;
LAB_142a1e25f:
        *(char *)((longlong)piVar4 + (longlong)piVar10) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar10[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar10[-2] = iVar11;
    }
  }
LAB_142a1e282:
  piVar17 = (int *)0x0;
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc080(&local_98,&local_a8);
  lVar8 = local_98;
  if (local_98 != 0) {
    iVar11 = *(int *)(local_98 + -8);
    piVar12 = (int *)(longlong)iVar11;
    if (iVar11 != 0) {
      iVar16 = 0;
      piVar4 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1e443;
      if ((char)*piVar10 == '\0') {
        if ((piVar10 == (int *)0x0) || (piVar4 = piVar10 + -4, piVar4 == (int *)0x0)) {
LAB_142a1e443:
          if (iVar16 < iVar11) {
            iVar16 = iVar11;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
          puVar5[1] = iVar16;
          *puVar5 = 0xffffffff;
          piVar10 = puVar5 + 4;
          puVar5[2] = 0;
          *(char *)piVar10 = '\0';
          local_c0 = piVar10;
          if (piVar4 != (int *)0x0) {
            FUN_14019f2c0(piVar4);
          }
        }
        else {
          if ((1 < *piVar4) || (piVar10[-3] < iVar11)) {
            iVar16 = piVar10[-2];
            goto LAB_142a1e443;
          }
          if (*piVar4 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar4 = -1;
        }
        FUN_142ef7ba0(piVar10,lVar8,piVar12);
        if (piVar10[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar11 == -1) || (iVar11 <= piVar10[-3])) {
          piVar10[-4] = 1;
          if (iVar11 != -1) goto LAB_142a1e4cb;
          if (piVar10 != (int *)0x0) {
            piVar17 = (int *)0xffffffffffffffff;
            do {
              piVar17 = (int *)((longlong)piVar17 + 1);
            } while (*(char *)((longlong)piVar10 + (longlong)piVar17) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar10[-3],iVar11);
          piVar10[-4] = 1;
LAB_142a1e4cb:
          *(char *)((longlong)piVar12 + (longlong)piVar10) = '\0';
          piVar17 = piVar12;
        }
        iVar11 = (int)piVar17;
        if ((iVar11 < 0) || (piVar10[-3] + 1 <= iVar11)) {
          FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
        }
        piVar10[-2] = iVar11;
        goto LAB_142a1e4f2;
      }
      iVar16 = piVar10[-2];
      for (iVar7 = piVar10[-3]; iVar7 < iVar16 + iVar11; iVar7 = iVar7 * 2) {
      }
      piVar17 = piVar10 + -4;
      if (piVar17 == (int *)0x0) {
        iVar9 = 0;
LAB_142a1e34f:
        if (iVar9 < iVar7) {
          iVar9 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
        puVar5[1] = iVar9;
        *puVar5 = 0xffffffff;
        piVar4 = puVar5 + 4;
        local_c0 = piVar4;
        if (piVar17 == (int *)0x0) {
          puVar5[2] = 0;
          *(char *)piVar4 = '\0';
        }
        else {
          iVar7 = piVar10[-2] + 1;
          if (iVar9 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar9 + 1);
            iVar7 = iVar9 + 1;
          }
          FUN_142ef7ba0(piVar4,piVar10,(longlong)iVar7);
          puVar5[2] = piVar10[-2];
          *(char *)((longlong)iVar9 + (longlong)piVar4) = '\0';
          FUN_14019f2c0(piVar17);
        }
      }
      else {
        if ((1 < *piVar17) || (piVar10[-3] < iVar7)) {
          iVar9 = piVar10[-2];
          goto LAB_142a1e34f;
        }
        if (*piVar17 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar17 = -1;
        piVar4 = piVar10;
      }
      if (piVar4 == (int *)0x0) {
        iVar7 = 0;
      }
      else {
        iVar7 = piVar4[-2];
      }
      FUN_142ef7ba0((char *)((longlong)iVar7 + (longlong)piVar4),lVar8,piVar12);
      FUN_14019c870(&local_c0,iVar16 + iVar11);
      piVar10 = piVar4;
    }
  }
LAB_142a1e4f2:
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  piVar17 = (int *)0x0;
  local_70 = 0;
  uVar3 = FUN_14019ba10(&local_70,&DAT_143272338,&DAT_143273ac0);
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  lVar8 = local_b8;
  piVar12 = piVar10;
  if (local_b8 != 0) {
    iVar11 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar11;
    if (iVar11 != 0) {
      iVar16 = 0;
      piVar14 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1e6fa;
      if ((char)*piVar10 != '\0') {
        iVar7 = piVar10[-2];
        for (iVar9 = piVar10[-3]; iVar9 < iVar7 + iVar11; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar10 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1e600:
          if (iVar16 < iVar9) {
            iVar16 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
          puVar5[1] = iVar16;
          *puVar5 = 0xffffffff;
          piVar12 = puVar5 + 4;
          local_c0 = piVar12;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar12 = '\0';
          }
          else {
            iVar9 = piVar10[-2] + 1;
            if (iVar16 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar16 + 1);
              iVar9 = iVar16 + 1;
            }
            FUN_142ef7ba0(piVar12,piVar10,(longlong)iVar9);
            puVar5[2] = piVar10[-2];
            *(char *)((longlong)iVar16 + (longlong)piVar12) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar10[-3] < iVar9)) {
            iVar16 = piVar10[-2];
            goto LAB_142a1e600;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        if (piVar12 == (int *)0x0) {
          iVar16 = 0;
        }
        else {
          iVar16 = piVar12[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar16 + (longlong)piVar12),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar7 + iVar11);
        goto LAB_142a1e7a5;
      }
      if ((piVar10 == (int *)0x0) || (piVar14 = piVar10 + -4, piVar14 == (int *)0x0)) {
LAB_142a1e6fa:
        if (iVar16 < iVar11) {
          iVar16 = iVar11;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar10[-3] < iVar11)) {
          iVar16 = piVar10[-2];
          goto LAB_142a1e6fa;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,piVar4);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar11 != -1) goto LAB_142a1e782;
        piVar10 = (int *)0xffffffffffffffff;
        if (piVar12 != (int *)0x0) {
          do {
            piVar17 = (int *)((longlong)piVar10 + 1);
            piVar10 = piVar17;
          } while (*(char *)((longlong)piVar12 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar11);
        piVar12[-4] = 1;
LAB_142a1e782:
        *(char *)((longlong)piVar4 + (longlong)piVar12) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1e7a5:
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc4c0(&local_90,local_58);
  lVar8 = local_90;
  iVar11 = 0;
  if (local_90 != 0) {
    iVar16 = *(int *)(local_90 + -8);
    lVar15 = (longlong)iVar16;
    if (iVar16 != 0) {
      if ((piVar12 == (int *)0x0) || ((char)*piVar12 == '\0')) {
        uVar3 = FUN_14019bd40(&local_c0,iVar16,0);
        FUN_142ef7ba0(uVar3,lVar8,lVar15);
      }
      else {
        iVar16 = piVar12[-2] + iVar16;
        for (iVar7 = piVar12[-3]; iVar7 < iVar16; iVar7 = iVar7 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_c0,iVar7,1);
        iVar7 = iVar11;
        if (local_c0 != (int *)0x0) {
          iVar7 = local_c0[-2];
        }
        FUN_142ef7ba0(iVar7 + lVar6,lVar8,lVar15);
      }
      FUN_14019c870(&local_c0,iVar16);
    }
  }
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  FUN_1401d7f50(&local_c0,&DAT_143487790,param_1 & 0xff);
  FUN_1408bc080(&local_b8,&local_a4);
  lVar8 = local_b8;
  if (local_b8 != 0) {
    iVar16 = *(int *)(local_b8 + -8);
    lVar15 = (longlong)iVar16;
    if (iVar16 != 0) {
      if ((local_c0 == (int *)0x0) || ((char)*local_c0 == '\0')) {
        uVar3 = FUN_14019bd40(&local_c0,iVar16,0);
        FUN_142ef7ba0(uVar3,lVar8,lVar15);
      }
      else {
        iVar16 = local_c0[-2] + iVar16;
        for (iVar7 = local_c0[-3]; iVar7 < iVar16; iVar7 = iVar7 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_c0,iVar7,1);
        if (local_c0 != (int *)0x0) {
          iVar11 = local_c0[-2];
        }
        FUN_142ef7ba0(iVar11 + lVar6,lVar8,lVar15);
      }
      FUN_14019c870(&local_c0,iVar16);
    }
  }
  if (local_b8 != 0) {
    FUN_14019f2c0(local_b8 + -0x10);
  }
  FUN_140319ad0(param_1,&local_c0);
  if (local_c0 != (int *)0x0) {
    FUN_14019f2c0(local_c0 + -4);
  }
LAB_142a1ea24:
  if (DAT_143ac18a0 == 0) {
    uVar3 = FUN_142a247e0(&local_res20,param_1,"Socket",&DAT_143271f04,&DAT_143271f04,&DAT_143271f04
                         );
    FUN_140319ad0(param_1,uVar3);
    lVar8 = CONCAT44(uStackX_24,local_res20);
  }
  else {
    local_res10 = 0;
    FUN_142a24da0(&local_res10,param_1,"Socket",DAT_143ac18a0 + 0x50,DAT_143ac18a0 + 0xc,
                  DAT_143ac18a0 + 0x154);
    FUN_140319ad0(param_1,&local_res10);
    lVar8 = local_res10;
  }
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_14019ba10 @ 14019ba10   (42 bytes)
//===========================================================

undefined8
FUN_14019ba10(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 local_res18;
  undefined8 local_res20;
  
  local_res18 = param_3;
  local_res20 = param_4;
  FUN_14019e480(param_1,param_2,&local_res18);
  return param_1;
}



//===========================================================
// FUN_14019c870 @ 14019c870   (175 bytes)
//===========================================================

void FUN_14019c870(longlong *param_1,int param_2)

{
  longlong lVar1;
  int iVar2;
  ulonglong uVar3;
  
  lVar1 = *param_1;
  uVar3 = (ulonglong)param_2;
  if (*(int *)(lVar1 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((param_2 == -1) || (param_2 <= *(int *)(lVar1 + -0xc))) {
    *(undefined4 *)(lVar1 + -0x10) = 1;
    if (param_2 != -1) goto LAB_14019c8ba;
    if (lVar1 != 0) {
      uVar3 = 0xffffffffffffffff;
      do {
        uVar3 = uVar3 + 1;
      } while (*(char *)(lVar1 + uVar3) != '\0');
      goto LAB_14019c8c1;
    }
    uVar3 = 0;
LAB_14019c8c5:
    iVar2 = (int)uVar3;
    if (iVar2 < *(int *)(lVar1 + -0xc) + 1) goto LAB_14019c8de;
  }
  else {
    FUN_142e54290(0x90,*(int *)(lVar1 + -0xc),param_2);
    *(undefined4 *)(lVar1 + -0x10) = 1;
LAB_14019c8ba:
    *(undefined1 *)(uVar3 + *param_1) = 0;
LAB_14019c8c1:
    if (-1 < (int)uVar3) goto LAB_14019c8c5;
  }
  iVar2 = (int)uVar3;
  FUN_142e54290(0x9c,uVar3 & 0xffffffff,*(undefined4 *)(lVar1 + -0xc));
LAB_14019c8de:
  *(int *)(lVar1 + -8) = iVar2;
  return;
}



//===========================================================
// FUN_1408bc3a0 @ 1408bc3a0   (90 bytes)
//===========================================================

undefined8 * FUN_1408bc3a0(undefined8 *param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  longlong local_res8 [4];
  
  local_res8[0] = 0;
  uVar1 = FUN_14019ba10(local_res8,"0x%p|",*param_2);
  *param_1 = 0;
  FUN_14019a260(param_1,uVar1);
  if (local_res8[0] != 0) {
    FUN_14019f2c0(local_res8[0] + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_142a1ec10 @ 142a1ec10   (61 bytes)
//===========================================================

void FUN_142a1ec10(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 local_res10;
  undefined8 *local_res18;
  
  local_res18 = &local_res10;
  local_res10 = 0;
  FUN_14019a260(&local_res10,param_1);
  uVar1 = FUN_142e56bc0();
  FUN_142a23830(uVar1,&local_res10);
  return;
}



//===========================================================
// FUN_1408bc140 @ 1408bc140   (90 bytes)
//===========================================================

undefined8 * FUN_1408bc140(undefined8 *param_1,undefined4 *param_2)

{
  undefined8 uVar1;
  longlong local_res8 [4];
  
  local_res8[0] = 0;
  uVar1 = FUN_14019ba10(local_res8,&DAT_1433005a0,*param_2);
  *param_1 = 0;
  FUN_14019a260(param_1,uVar1);
  if (local_res8[0] != 0) {
    FUN_14019f2c0(local_res8[0] + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_14019b600 @ 14019b600   (375 bytes)
//===========================================================

void FUN_14019b600(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  int iVar6;
  undefined8 uVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar7 = 0x98;
  if (param_2 < 0x39) {
    uVar10 = (uint)(0x28 < param_2);
LAB_14019b65e:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x28;
      goto LAB_14019b6a6;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x38;
      goto LAB_14019b6a6;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b6a6;
    }
  }
  else {
    if (0x58 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x99) {
        uVar10 = 3;
      }
      goto LAB_14019b65e;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x58;
LAB_14019b6a6:
  lVar11 = (longlong)(int)uVar10;
  lVar8 = lVar11 * 0x10 + param_1;
  plVar1 = (longlong *)(lVar8 + 0x28);
  LOCK();
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar4 == 0) {
LAB_14019b709:
    *(undefined4 *)(lVar8 + 0x30) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar4 = *plVar1;
      if (lVar4 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar4 == 0) goto LAB_14019b709;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar7,iVar6);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar6;
    puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  }
  piVar2 = (int *)(param_1 + 0x14 + lVar11 * 4);
  *piVar2 = *piVar2 + 1;
  *(undefined8 *)(param_1 + 0x68 + lVar11 * 8) = *puVar5;
  *piVar9 = *piVar9 + -1;
  if (*piVar9 == 0) {
    *plVar1 = 0;
  }
  return;
}



//===========================================================
// FUN_14019a260 @ 14019a260   (485 bytes)
//===========================================================

longlong * FUN_14019a260(longlong *param_1,longlong *param_2)

{
  void *_Buf1;
  void *_Buf2;
  longlong lVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  ulonglong uVar8;
  int iVar9;
  
  if (param_1 == param_2) {
    return param_1;
  }
  _Buf1 = (void *)*param_1;
  piVar7 = (int *)0x0;
  iVar9 = 0;
  iVar2 = iVar9;
  if (_Buf1 != (void *)0x0) {
    iVar2 = *(int *)((longlong)_Buf1 + -8);
  }
  _Buf2 = (void *)*param_2;
  iVar4 = iVar9;
  if (_Buf2 != (void *)0x0) {
    iVar4 = *(int *)((longlong)_Buf2 + -8);
  }
  if ((((iVar2 == iVar4) && (iVar2 != 0)) && (_Buf1 != (void *)0x0)) &&
     ((_Buf2 != (void *)0x0 && (iVar2 = memcmp(_Buf1,_Buf2,(longlong)iVar2), iVar2 == 0)))) {
    return param_1;
  }
  piVar5 = (int *)((longlong)_Buf2 + -0x10);
  if (_Buf2 == (void *)0x0) {
    piVar5 = piVar7;
  }
  if (piVar5 == (int *)0x0) {
    if (_Buf1 == (void *)0x0) {
      return param_1;
    }
    FUN_14019f2c0((longlong)_Buf1 + -0x10);
    *param_1 = 0;
    return param_1;
  }
  if (*piVar5 != -1) {
    if (*piVar5 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar5 = *piVar5 + 1;
    UNLOCK();
    if (*param_1 != 0) {
      FUN_14019f2c0(*param_1 + -0x10);
    }
    *param_1 = (longlong)(piVar5 + 4);
    return param_1;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  lVar1 = *param_2;
  piVar5 = piVar7;
  if (lVar1 == 0) goto LAB_14019a3c9;
  uVar8 = 0xffffffffffffffff;
  piVar6 = (int *)0xffffffffffffffff;
  do {
    piVar6 = (int *)((longlong)piVar6 + 1);
  } while (*(char *)(lVar1 + (longlong)piVar6) != '\0');
  iVar2 = (int)piVar6;
  if (0 < iVar2) {
    iVar9 = iVar2;
  }
  piVar3 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
  piVar3[1] = iVar9;
  *piVar3 = -1;
  piVar5 = piVar3 + 4;
  piVar3[2] = 0;
  *(undefined1 *)piVar5 = 0;
  FUN_142ef7ba0(piVar5,lVar1,(longlong)iVar2);
  if (*piVar3 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar2 == -1) || (iVar2 <= piVar3[1])) {
    *piVar3 = 1;
    if (iVar2 != -1) goto LAB_14019a3a6;
    if (piVar5 != (int *)0x0) {
      do {
        uVar8 = uVar8 + 1;
      } while (*(char *)((longlong)piVar5 + uVar8) != '\0');
      piVar7 = (int *)(uVar8 & 0xffffffff);
    }
  }
  else {
    FUN_142e54290(0x90,piVar3[1],(ulonglong)piVar6 & 0xffffffff);
    *piVar3 = 1;
LAB_14019a3a6:
    *(undefined1 *)((longlong)piVar5 + (longlong)iVar2) = 0;
    piVar7 = piVar6;
  }
  iVar2 = (int)piVar7;
  if ((iVar2 < 0) || (piVar3[1] + 1 <= iVar2)) {
    FUN_142e54290(0x9c,(ulonglong)piVar7 & 0xffffffff);
  }
  piVar3[2] = iVar2;
LAB_14019a3c9:
  if (*param_1 != 0) {
    FUN_14019f2c0(*param_1 + -0x10);
  }
  *param_1 = (longlong)piVar5;
  return param_1;
}



//===========================================================
// FUN_1408bc980 @ 1408bc980   (208 bytes)
//===========================================================

undefined8 * FUN_1408bc980(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  undefined8 uVar2;
  int iVar3;
  char *pcVar4;
  longlong local_res10;
  
  pcVar4 = (char *)*param_2;
  if ((pcVar4 != (char *)0x0) && (0 < *(int *)(pcVar4 + -8))) {
    iVar1 = *(int *)(pcVar4 + -8);
    iVar3 = iVar1 + -1;
    if ((iVar3 < 0) || (*(int *)(pcVar4 + -8) <= iVar3)) {
      FUN_142e54290(0xcc,iVar3,*(undefined4 *)(pcVar4 + -8));
      pcVar4 = (char *)*param_2;
    }
    if (pcVar4[(longlong)iVar1 + -1] == '|') {
      *param_1 = 0;
      FUN_14019a260(param_1,param_2);
      return param_1;
    }
  }
  local_res10 = 0;
  if ((pcVar4 == (char *)0x0) || (*pcVar4 == '\0')) {
    pcVar4 = "";
  }
  uVar2 = FUN_14019ba10(&local_res10,&DAT_143272338,pcVar4);
  *param_1 = 0;
  FUN_14019a260(param_1,uVar2);
  if (local_res10 != 0) {
    FUN_14019f2c0(local_res10 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_1408bc020 @ 1408bc020   (90 bytes)
//===========================================================

undefined8 * FUN_1408bc020(undefined8 *param_1,undefined4 *param_2)

{
  undefined8 uVar1;
  longlong local_res8 [4];
  
  local_res8[0] = 0;
  uVar1 = FUN_14019ba10(local_res8,&DAT_14330059c,*param_2);
  *param_1 = 0;
  FUN_14019a260(param_1,uVar1);
  if (local_res8[0] != 0) {
    FUN_14019f2c0(local_res8[0] + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_14019bd40 @ 14019bd40   (269 bytes)
//===========================================================

longlong FUN_14019bd40(longlong *param_1,int param_2,int param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  
  piVar4 = (int *)(*param_1 + -0x10);
  if (*param_1 == 0) {
    piVar4 = (int *)0x0;
  }
  iVar6 = 0;
  if (piVar4 != (int *)0x0) {
    if ((*piVar4 < 2) && (param_2 <= piVar4[1])) {
      if (*piVar4 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar4 = -1;
      goto LAB_14019be37;
    }
    iVar6 = piVar4[2];
  }
  if (iVar6 < param_2) {
    iVar6 = param_2;
  }
  puVar1 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
  puVar1[1] = iVar6;
  *puVar1 = 0xffffffff;
  puVar2 = puVar1 + 4;
  *param_1 = (longlong)puVar2;
  if ((param_3 == 0) || (piVar4 == (int *)0x0)) {
    puVar1[2] = 0;
    *(undefined1 *)*param_1 = 0;
    if (piVar4 == (int *)0x0) goto LAB_14019be37;
  }
  else {
    iVar5 = iVar6 + 1;
    iVar3 = piVar4[2] + 1;
    if (iVar5 < iVar3) {
      FUN_142e54290(0x5c,iVar3,iVar5);
      puVar2 = (undefined4 *)*param_1;
      iVar3 = iVar5;
    }
    FUN_142ef7ba0(puVar2,piVar4 + 4,(longlong)iVar3);
    puVar1[2] = piVar4[2];
    *(undefined1 *)((longlong)iVar6 + *param_1) = 0;
  }
  FUN_14019f2c0(piVar4);
LAB_14019be37:
  return *param_1;
}



//===========================================================
// FUN_142ed3068 @ 142ed3068   (34 bytes)
//===========================================================

void FUN_142ed3068(undefined8 param_1)

{
  undefined1 local_28 [40];
  
  FUN_142ed2ce0(local_28,param_1);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_28,(ThrowInfo *)&DAT_143a3c5d0);
}


