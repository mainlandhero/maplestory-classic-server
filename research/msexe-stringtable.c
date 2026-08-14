
//===========================================================
// FUN_1408a9e40 @ 1408a9e40   (75 bytes)
//===========================================================

undefined8 * FUN_1408a9e40(undefined8 *param_1,int param_2)

{
  longlong lVar1;
  
  FUN_14073d6f0(param_2);
  lVar1 = FUN_1408aa4b0((longlong)param_2);
  if (lVar1 == 0) {
    lVar1 = FUN_1408aac40(param_2);
  }
  *param_1 = 0;
  FUN_14019a260(param_1,lVar1);
  return param_1;
}



//===========================================================
// FUN_1408aa4b0 @ 1408aa4b0   (101 bytes)
//===========================================================

undefined8 * FUN_1408aa4b0(int param_1)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  
  if (DAT_143ac3470 != (longlong *)0x0) {
    puVar2 = (undefined8 *)*DAT_143ac3470;
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar5 = puVar2;
    puVar4 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)(puVar4 + 4) < param_1) {
        puVar3 = (undefined8 *)puVar4[2];
        puVar4 = puVar5;
      }
      else {
        puVar3 = (undefined8 *)*puVar4;
      }
      puVar5 = puVar4;
      puVar4 = puVar3;
      cVar1 = *(char *)((longlong)puVar3 + 0x19);
    }
    if ((*(char *)((longlong)puVar5 + 0x19) != '\0') || (param_1 < *(int *)(puVar5 + 4))) {
      puVar5 = puVar2;
    }
    puVar4 = puVar5 + 4;
    if (puVar5 == puVar2) {
      puVar4 = (undefined8 *)0x0;
    }
    if ((puVar4 != (undefined8 *)0x0) && (puVar4 != (undefined8 *)&DAT_fffffffffffffff8)) {
      return puVar4 + 3;
    }
  }
  return (undefined8 *)0x0;
}



//===========================================================
// FUN_1408aac40 @ 1408aac40   (300 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * FUN_1408aac40(int param_1)

{
  undefined4 uVar1;
  int iVar2;
  void *pvVar3;
  int local_res8 [2];
  void *local_res10;
  
  if (*(int *)(*(longlong *)((longlong)ThreadLocalStoragePointer + (ulonglong)DAT_143ae2c08 * 8) + 4
              ) < DAT_143ac2f08) {
    _Init_thread_header(&DAT_143ac2f08);
    if (DAT_143ac2f08 == -1) {
      pvVar3 = (void *)FUN_14019b780(&DAT_143ad68a0,0x302a0);
      local_res10 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        _DAT_143ac2f00 = (void *)0x0;
      }
      else {
        _eh_vector_constructor_iterator_
                  (pvVar3,0x20,0x1815,(_func_void_void_ptr *)&LAB_1408aafc0,FUN_1408ab1f0);
        _DAT_143ac2f00 = pvVar3;
      }
      _Init_thread_footer(&DAT_143ac2f08);
    }
  }
  uVar1 = DAT_143aa9d20;
  pvVar3 = (void *)((longlong)param_1 * 0x20 + (longlong)_DAT_143ac2f00);
  iVar2 = (*DAT_143262778)((longlong)pvVar3 + 0x18,0,local_res8,0);
  if (iVar2 != 0) {
    if (local_res8[0] != 0) {
      FUN_1408aa850(pvVar3,param_1,uVar1);
      iVar2 = (*DAT_143262770)((longlong)pvVar3 + 0x18,0,0);
      if (iVar2 == 0) goto LAB_1408aad01;
    }
    return pvVar3;
  }
LAB_1408aad01:
                    /* WARNING: Subroutine does not return */
  FUN_142f048cc();
}



//===========================================================
// FUN_14073d6f0 @ 14073d6f0   (3 bytes)
//===========================================================

void FUN_14073d6f0(void)

{
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
// FUN_142f048cc @ 142f048cc   (85 bytes)
//===========================================================

void FUN_142f048cc(void)

{
  code *pcVar1;
  longlong lVar2;
  int iVar3;
  undefined1 *puVar4;
  undefined1 auStack_28 [8];
  undefined1 auStack_20 [32];
  
  puVar4 = auStack_28;
  lVar2 = __acrt_get_sigabrt_handler();
  if (lVar2 != 0) {
    FUN_142f29cb8(0x16);
  }
  if (((byte)DAT_143a8b970 & 2) != 0) {
    iVar3 = (*DAT_143262740)(0x17);
    puVar4 = auStack_28;
    if (iVar3 != 0) {
      pcVar1 = (code *)swi(0x29);
      (*pcVar1)(7);
      puVar4 = auStack_20;
    }
    *(undefined8 *)(puVar4 + -8) = 0x142f04917;
    __acrt_call_reportfault(3,0x40000015);
  }
                    /* WARNING: Subroutine does not return */
  *(undefined **)(puVar4 + -8) = &UNK_142f04921;
  FUN_142f27b44(3);
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
// _Init_thread_footer @ 142ef41d4   (96 bytes)
//===========================================================

/* Library Function - Single Match
    _Init_thread_footer
   
   Library: Visual Studio 2019 Release */

void _Init_thread_footer(int *param_1)

{
  ulonglong uVar1;
  
  (*DAT_143262448)(&DAT_143ae2bd0);
  uVar1 = (ulonglong)DAT_143ae2c08;
  DAT_143a8b8f4 = DAT_143a8b8f4 + 1;
  *param_1 = DAT_143a8b8f4;
  *(int *)(*(longlong *)((longlong)ThreadLocalStoragePointer + uVar1 * 8) + 4) = DAT_143a8b8f4;
  (*DAT_143262450)(&DAT_143ae2bd0);
  _Init_thread_notify();
  return;
}



//===========================================================
// _Init_thread_header @ 142ef4234   (103 bytes)
//===========================================================

/* Library Function - Single Match
    _Init_thread_header
   
   Library: Visual Studio 2019 Release */

void _Init_thread_header(int *param_1)

{
  (*DAT_143262448)(&DAT_143ae2bd0);
  do {
    if (*param_1 == 0) {
      *param_1 = -1;
LAB_142ef4288:
      (*DAT_143262450)(&DAT_143ae2bd0);
      return;
    }
    if (*param_1 != -1) {
      *(undefined4 *)
       (*(longlong *)((longlong)ThreadLocalStoragePointer + (ulonglong)DAT_143ae2c08 * 8) + 4) =
           DAT_143a8b8f4;
      goto LAB_142ef4288;
    }
    _Init_thread_wait(100);
  } while( true );
}



//===========================================================
// FUN_1408aa850 @ 1408aa850   (661 bytes)
//===========================================================

void FUN_1408aa850(longlong *param_1,int param_2,int param_3)

{
  longlong **pplVar1;
  longlong *plVar2;
  uint uVar3;
  int *piVar4;
  longlong *plVar5;
  undefined *puVar6;
  int *piVar7;
  longlong lVar8;
  int iVar9;
  int iVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong *local_res20;
  
  puVar6 = (&PTR_PTR_143a563f8)[param_3];
  lVar8 = *(longlong *)(puVar6 + (longlong)param_2 * 8) + 1;
  piVar7 = (int *)0x0;
  if (lVar8 != 0) {
    uVar11 = 0xffffffffffffffff;
    do {
      uVar11 = uVar11 + 1;
    } while (*(char *)(uVar11 + lVar8) != '\0');
    iVar9 = (int)uVar11;
    iVar10 = 0;
    if (0 < iVar9) {
      iVar10 = iVar9;
    }
    piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar10 + 0x11));
    piVar4[1] = iVar10;
    *piVar4 = -1;
    piVar7 = piVar4 + 4;
    piVar4[2] = 0;
    *(undefined1 *)piVar7 = 0;
    FUN_142ef7ba0(piVar7,lVar8,(longlong)iVar9);
    if (*piVar4 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar9 == -1) || (iVar9 <= piVar4[1])) {
      *piVar4 = 1;
      if (iVar9 != -1) goto LAB_1408aa917;
      if (piVar7 == (int *)0x0) {
        uVar11 = 0;
      }
      else {
        uVar11 = 0xffffffffffffffff;
        do {
          uVar11 = uVar11 + 1;
        } while (*(char *)((longlong)piVar7 + uVar11) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar4[1],uVar11 & 0xffffffff);
      *piVar4 = 1;
LAB_1408aa917:
      *(undefined1 *)((longlong)piVar7 + (longlong)iVar9) = 0;
    }
    iVar10 = (int)uVar11;
    if ((iVar10 < 0) || (piVar4[1] + 1 <= iVar10)) {
      FUN_142e54290(0x9c,uVar11 & 0xffffffff);
    }
    piVar4[2] = iVar10;
  }
  if (*param_1 != 0) {
    FUN_14019f2c0(*param_1 + -0x10);
  }
  *param_1 = (longlong)piVar7;
  FUN_1408aadb0(param_1,&DAT_1432ba470,0x10,(longlong)**(char **)(puVar6 + (longlong)param_2 * 8));
  puVar6 = (undefined *)*param_1;
  if (puVar6 == (undefined *)0x0) {
    puVar6 = &DAT_1432780f0;
    iVar10 = 0;
  }
  else {
    iVar10 = *(int *)(puVar6 + -8);
  }
  FUN_1408abf30(param_1 + 1,puVar6,puVar6 + iVar10);
  lVar8 = param_1[1];
  plVar5 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
  local_res20 = plVar5;
  if (plVar5 == (longlong *)0x0) {
    plVar5 = (longlong *)0x0;
  }
  else {
    plVar5[1] = 0;
    *(undefined4 *)(plVar5 + 2) = 1;
    if (lVar8 != 0) {
      lVar12 = -1;
      do {
        lVar12 = lVar12 + 1;
      } while (*(short *)(lVar8 + lVar12 * 2) != 0);
      uVar3 = (int)lVar12 + 1;
      piVar7 = (int *)(*DAT_143ad5980)((ulonglong)uVar3 * 2 + 4);
      if (piVar7 == (int *)0x0) {
        *plVar5 = 0;
      }
      else {
        *piVar7 = (int)lVar12 * 2;
        piVar7 = piVar7 + 1;
        FUN_142ef7ba0(piVar7,lVar8,(ulonglong)uVar3 * 2);
        *plVar5 = (longlong)piVar7;
        if (piVar7 != (int *)0x0) goto LAB_1408aaa59;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    *plVar5 = 0;
  }
LAB_1408aaa59:
  if (plVar5 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x8007000e);
  }
  pplVar1 = (longlong **)(param_1 + 2);
  if (pplVar1 != &local_res20) {
    FUN_1401be120(pplVar1);
    *pplVar1 = plVar5;
    LOCK();
    *(int *)(plVar5 + 2) = (int)plVar5[2] + 1;
    UNLOCK();
  }
  LOCK();
  plVar2 = plVar5 + 2;
  lVar8 = *plVar2;
  *(int *)plVar2 = (int)*plVar2 + -1;
  UNLOCK();
  if ((int)lVar8 == 1) {
    if (*plVar5 != 0) {
      (*DAT_143ad5990)(*plVar5 + -4);
      *plVar5 = 0;
    }
    if (plVar5[1] != 0) {
      FUN_14019b4e0();
      plVar5[1] = 0;
    }
    thunk_FUN_140205820(plVar5,0x18);
  }
  return;
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
// FUN_14019d3c0 @ 14019d3c0   (144 bytes)
//===========================================================

undefined8 * FUN_14019d3c0(longlong param_1,longlong param_2)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  
  puVar3 = (undefined8 *)FUN_14019d350((param_1 + 8) * param_2 + 8,param_1 * param_2);
  *puVar3 = 0;
  puVar1 = puVar3 + 2;
  puVar3[1] = param_1;
  param_2 = param_2 + -1;
  puVar3 = puVar1;
  if (param_2 == 0) {
    *puVar1 = 0;
    return puVar1;
  }
  do {
    puVar2 = (undefined8 *)((longlong)puVar3 + param_1 + 8);
    *puVar3 = puVar2;
    puVar2[-1] = param_1;
    param_2 = param_2 + -1;
    puVar3 = puVar2;
  } while (param_2 != 0);
  *puVar2 = 0;
  return puVar1;
}



//===========================================================
// FUN_14019d350 @ 14019d350   (105 bytes)
//===========================================================

longlong * FUN_14019d350(longlong param_1,ulonglong param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong *plVar3;
  
  if (0xc7fffff < param_2) {
    FUN_142e541f0(0x3a);
  }
  pcVar1 = DAT_143ad5528;
  uVar2 = (*DAT_143ad5538)();
  plVar3 = (longlong *)(*pcVar1)(uVar2,0,param_1 + 8);
  if (plVar3 != (longlong *)0x0) {
    *plVar3 = param_1;
    return plVar3 + 1;
  }
  return (longlong *)0x0;
}



//===========================================================
// __acrt_call_reportfault @ 142f04518   (347 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* Library Function - Single Match
    __acrt_call_reportfault
   
   Libraries: Visual Studio 2017 Release, Visual Studio 2019 Release */

void __acrt_call_reportfault(int param_1,undefined4 param_2,undefined4 param_3)

{
  int iVar1;
  int iVar2;
  longlong lVar3;
  undefined1 local_res8 [8];
  undefined1 auStack_608 [32];
  undefined1 *local_5e8;
  undefined1 *local_5e0;
  undefined1 *local_5d8;
  undefined8 local_5d0;
  undefined8 local_5c8;
  undefined4 *local_5c0;
  undefined1 *local_5b8;
  undefined1 local_5b0 [8];
  undefined1 local_5a8 [16];
  undefined4 local_598;
  undefined4 local_594;
  undefined1 local_4f8 [152];
  undefined1 *local_460;
  undefined8 local_400;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_608;
  if (param_1 != -1) {
    FUN_142ef4c14();
  }
  FUN_142ef8250(&local_598,0,0x98);
  FUN_142ef8250(local_4f8,0,0x4d0);
  local_5c0 = &local_598;
  local_5b8 = local_4f8;
  (*DAT_143262840)(local_4f8);
  lVar3 = (*DAT_143262fe8)(local_400,&local_5c8,0);
  if (lVar3 != 0) {
    local_5d0 = 0;
    local_5d8 = local_5b0;
    local_5e0 = local_5a8;
    local_5e8 = local_4f8;
    (*DAT_143262fe0)(0,local_5c8,local_400,lVar3);
  }
  local_460 = local_res8;
  local_598 = param_2;
  local_594 = param_3;
  iVar1 = (*DAT_143262680)();
  (*DAT_143262498)(0);
  iVar2 = (*DAT_1432626a0)(&local_5c0);
  if (((iVar2 == 0) && (iVar1 == 0)) && (param_1 != -1)) {
    FUN_142ef4c14(param_1);
  }
  return;
}



//===========================================================
// FUN_142f29cb8 @ 142f29cb8   (631 bytes)
//===========================================================

undefined8 FUN_142f29cb8(uint param_1)

{
  bool bVar1;
  ulong *puVar2;
  longlong lVar3;
  longlong lVar4;
  byte bVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong *puVar8;
  longlong *plVar9;
  undefined4 local_res10;
  longlong *plVar10;
  
  plVar10 = (longlong *)0x0;
  plVar9 = (longlong *)0x0;
  local_res10 = 0;
  bVar1 = true;
  if (param_1 == 2) {
LAB_142f29d0f:
    if (param_1 == 2) {
      puVar8 = &DAT_143ae2eb0;
    }
    else if (param_1 == 6) {
LAB_142f29db1:
      puVar8 = &DAT_143ae2ec0;
      plVar9 = plVar10;
    }
    else if (param_1 == 0xf) {
      puVar8 = (ulonglong *)&DAT_143ae2ec8;
    }
    else if (param_1 == 0x15) {
      puVar8 = &DAT_143ae2eb8;
      plVar9 = plVar10;
    }
    else {
      if (param_1 == 0x16) goto LAB_142f29db1;
      puVar8 = (ulonglong *)0x0;
      plVar9 = plVar10;
    }
  }
  else {
    if (param_1 != 4) {
      if (param_1 != 6) {
        if ((param_1 == 8) || (param_1 == 0xb)) goto LAB_142f29d3f;
        if ((param_1 != 0xf) && ((param_1 != 0x15 && (param_1 != 0x16)))) goto LAB_142f29d91;
      }
      goto LAB_142f29d0f;
    }
LAB_142f29d3f:
    plVar9 = (longlong *)FUN_142f318fc();
    if (plVar9 == (longlong *)0x0) {
      return 0xffffffff;
    }
    lVar4 = *plVar9;
    lVar3 = DAT_1434a9790 * 0x10 + lVar4;
    for (; lVar4 != lVar3; lVar4 = lVar4 + 0x10) {
      if (*(uint *)(lVar4 + 4) == param_1) goto LAB_142f29d8c;
    }
    lVar4 = 0;
LAB_142f29d8c:
    if (lVar4 == 0) {
LAB_142f29d91:
      puVar2 = __doserrno();
      *puVar2 = 0x16;
      FUN_142f047e4();
      return 0xffffffff;
    }
    puVar8 = (ulonglong *)(lVar4 + 8);
    bVar1 = false;
  }
  lVar4 = 0;
  if (bVar1) {
    __acrt_lock(3);
  }
  uVar7 = *puVar8;
  if (bVar1) {
    bVar5 = (byte)DAT_143a8b908 & 0x3f;
    uVar7 = (uVar7 ^ DAT_143a8b908) >> bVar5 | (uVar7 ^ DAT_143a8b908) << 0x40 - bVar5;
  }
  if (uVar7 == 1) goto LAB_142f29e9a;
  if (uVar7 == 0) {
    if (bVar1) {
      __acrt_unlock(3);
    }
                    /* WARNING: Subroutine does not return */
    FUN_142f27b44(3);
  }
  if ((param_1 < 0xc) && ((0x910U >> (param_1 & 0x1f) & 1) != 0)) {
    lVar4 = plVar9[1];
    plVar9[1] = 0;
    if (param_1 == 8) {
      lVar3 = FUN_142f31784();
      local_res10 = *(undefined4 *)(lVar3 + 0x10);
      lVar3 = FUN_142f31784();
      *(undefined4 *)(lVar3 + 0x10) = 0x8c;
      goto LAB_142f29e52;
    }
  }
  else {
LAB_142f29e52:
    if (param_1 == 8) {
      lVar3 = DAT_1434a97a0 * 0x10 + *plVar9;
      lVar6 = DAT_1434a97a8 * 0x10 + lVar3;
      for (; lVar3 != lVar6; lVar3 = lVar3 + 0x10) {
        *(undefined8 *)(lVar3 + 8) = 0;
      }
      goto LAB_142f29e9a;
    }
  }
  *puVar8 = DAT_143a8b908;
LAB_142f29e9a:
  if (bVar1) {
    __acrt_unlock(3);
  }
  if (uVar7 != 1) {
    if (param_1 == 8) {
      lVar3 = FUN_142f31784();
      (*(code *)PTR_thunk_FUN_142f44a90_1432630e0)(8,*(undefined4 *)(lVar3 + 0x10));
    }
    else {
      (*(code *)PTR_thunk_FUN_142f44a90_1432630e0)(param_1);
    }
    if (((param_1 < 0xc) && ((0x910U >> (param_1 & 0x1f) & 1) != 0)) &&
       (plVar9[1] = lVar4, param_1 == 8)) {
      lVar4 = FUN_142f31784();
      *(undefined4 *)(lVar4 + 0x10) = local_res10;
    }
  }
  return 0;
}



//===========================================================
// __acrt_get_sigabrt_handler @ 142f29c68   (46 bytes)
//===========================================================

/* Library Function - Single Match
    __acrt_get_sigabrt_handler
   
   Libraries: Visual Studio 2015 Release, Visual Studio 2017 Release, Visual Studio 2019 Release */

void __acrt_get_sigabrt_handler(void)

{
  undefined1 local_res8 [8];
  undefined4 local_res10 [2];
  undefined4 local_res18 [4];
  
  local_res10[0] = 3;
  local_res18[0] = 3;
  FUN_142f29b14(local_res8,local_res18,local_res8,local_res10);
  return;
}



//===========================================================
// FUN_142f27b44 @ 142f27b44   (12 bytes)
//===========================================================

void FUN_142f27b44(undefined8 param_1)

{
  FUN_142f2798c(param_1,2);
  return;
}



//===========================================================
// FUN_142f44a90 @ 142f44a90   (2 bytes)
//===========================================================

void FUN_142f44a90(void)

{
  code *UNRECOVERED_JUMPTABLE;
  
                    /* WARNING: Could not recover jumptable at 0x000142f44a90. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*UNRECOVERED_JUMPTABLE)();
  return;
}



//===========================================================
// _Init_thread_notify @ 142ef429c   (65 bytes)
//===========================================================

/* Library Function - Single Match
    _Init_thread_notify
   
   Library: Visual Studio 2019 Release */

void _Init_thread_notify(void)

{
  if (DAT_143ae2c00 != 0) {
    (*(code *)PTR_FUN_1432630d8)(&DAT_143ae2bc0);
    return;
  }
  (*DAT_1432623d8)(DAT_143ae2bc8);
  (*DAT_1432622c8)(DAT_143ae2bc8);
  return;
}



//===========================================================
// _Init_thread_wait @ 142ef42e0   (99 bytes)
//===========================================================

/* Library Function - Single Match
    _Init_thread_wait
   
   Library: Visual Studio 2019 Release */

void _Init_thread_wait(undefined4 param_1)

{
  if (DAT_143ae2bf8 != 0) {
    (*(code *)PTR_FUN_1432630d8)(&DAT_143ae2bc0,&DAT_143ae2bd0,param_1);
    return;
  }
  (*DAT_143262450)(&DAT_143ae2bd0);
  (*DAT_143262910)(DAT_143ae2bc8,param_1,0);
  (*DAT_143262448)(&DAT_143ae2bd0);
  return;
}



//===========================================================
// FUN_1408aadb0 @ 1408aadb0   (303 bytes)
//===========================================================

void FUN_1408aadb0(longlong *param_1)

{
  byte bVar1;
  longlong lVar2;
  uint uVar3;
  byte *pbVar4;
  ulonglong uVar5;
  byte bVar6;
  ulonglong uVar7;
  int iVar8;
  int iVar9;
  undefined4 uVar10;
  longlong local_res8;
  
  FUN_1408aa600(&local_res8);
  pbVar4 = (byte *)FUN_14019bd40(param_1,0,1);
  iVar9 = 0;
  bVar1 = *pbVar4;
  while (bVar1 != 0) {
    if (local_res8 == 0) {
      uVar3 = 0;
    }
    else {
      uVar3 = *(uint *)(local_res8 + -8);
    }
    uVar7 = (ulonglong)(longlong)iVar9 % (ulonglong)uVar3;
    if (local_res8 == 0) {
      uVar5 = 0;
    }
    else {
      uVar5 = (ulonglong)*(uint *)(local_res8 + -8);
    }
    if (uVar5 <= uVar7) {
      if (local_res8 == 0) {
        uVar10 = 0;
      }
      else {
        uVar10 = *(undefined4 *)(local_res8 + -8);
      }
      FUN_142e54290(0xc6,uVar7,uVar10);
    }
    bVar1 = *(byte *)(uVar7 + local_res8);
    *pbVar4 = *pbVar4 ^ bVar1;
    bVar6 = *pbVar4;
    if (*pbVar4 == 0) {
      bVar6 = bVar1;
    }
    *pbVar4 = bVar6;
    iVar9 = iVar9 + 1;
    pbVar4 = pbVar4 + 1;
    bVar1 = *pbVar4;
  }
  lVar2 = *param_1;
  if (*(int *)(lVar2 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  *(undefined4 *)(lVar2 + -0x10) = 1;
  if (lVar2 == 0) {
    uVar7 = 0;
    iVar9 = iRamfffffffffffffff4;
LAB_1408aaea1:
    iVar8 = (int)uVar7;
    if (iVar8 < iVar9 + 1) goto LAB_1408aaeb1;
  }
  else {
    uVar7 = 0xffffffffffffffff;
    do {
      uVar7 = uVar7 + 1;
    } while (*(char *)(lVar2 + uVar7) != '\0');
    iVar9 = *(int *)(lVar2 + -0xc);
    if (-1 < (int)uVar7) goto LAB_1408aaea1;
  }
  iVar8 = (int)uVar7;
  FUN_142e54290(0x9c,uVar7 & 0xffffffff);
LAB_1408aaeb1:
  *(int *)(lVar2 + -8) = iVar8;
  if (local_res8 != 0) {
    thunk_FUN_140205820(local_res8 + -8,0);
  }
  return;
}



//===========================================================
// FUN_1401be120 @ 1401be120   (114 bytes)
//===========================================================

void FUN_1401be120(undefined8 *param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  
  plVar2 = (longlong *)*param_1;
  if (plVar2 != (longlong *)0x0) {
    LOCK();
    plVar1 = plVar2 + 2;
    lVar3 = *plVar1;
    *(int *)plVar1 = (int)*plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      if (*plVar2 != 0) {
        (*DAT_143ad5990)(*plVar2 + -4);
        *plVar2 = 0;
      }
      if (plVar2[1] != 0) {
        FUN_14019b4e0();
        plVar2[1] = 0;
      }
      thunk_FUN_140205820(plVar2,0x18);
    }
    *param_1 = 0;
  }
  return;
}



//===========================================================
// FUN_1408abf30 @ 1408abf30   (468 bytes)
//===========================================================

void FUN_1408abf30(longlong *param_1,byte *param_2,byte *param_3)

{
  byte bVar1;
  longlong lVar2;
  int *piVar3;
  ushort uVar4;
  undefined4 *puVar5;
  int iVar6;
  int iVar7;
  int *piVar8;
  ushort *puVar9;
  
  lVar2 = *param_1;
  if (param_2 == (byte *)0x0) {
    if (lVar2 == 0) {
      return;
    }
    FUN_1401bebb0(lVar2 + -0x10);
    *param_1 = 0;
    return;
  }
  iVar6 = ((int)param_3 - (int)param_2) + 4;
  piVar8 = (int *)(lVar2 + -0x10);
  if (lVar2 == 0) {
    piVar8 = (int *)0x0;
  }
  iVar7 = 0;
  if (piVar8 == (int *)0x0) {
LAB_1408abfc8:
    if (iVar7 < iVar6) {
      iVar7 = iVar6;
    }
    puVar5 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar7 * 2 + 0x12));
    puVar5[1] = iVar7;
    *puVar5 = 0xffffffff;
    *param_1 = (longlong)(puVar5 + 4);
    puVar5[2] = 0;
    *(undefined2 *)*param_1 = 0;
    if (piVar8 != (int *)0x0) {
      FUN_1401bebb0(piVar8);
    }
  }
  else {
    if ((1 < *piVar8) || (piVar8[1] < iVar6)) {
      iVar7 = (int)((ulonglong)(longlong)piVar8[2] >> 1);
      goto LAB_1408abfc8;
    }
    if (*piVar8 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar8 = -1;
  }
  piVar8 = (int *)0xffffffffffffffff;
  puVar9 = (ushort *)*param_1;
  while (param_2 < param_3) {
    bVar1 = *param_2;
    uVar4 = (ushort)(char)bVar1;
    if ((char)bVar1 < '\0') {
      if ((bVar1 & 0xe0) == 0xc0) {
        uVar4 = (ushort)(bVar1 & 0x1f) << 6 | (ushort)(param_2[1] & 0x3f);
        param_2 = param_2 + 2;
      }
      else {
        if ((bVar1 & 0xf0) != 0xe0) {
          uVar4 = 0x3f;
          goto LAB_1408ac087;
        }
        uVar4 = ((ushort)(param_2[1] & 0x3f) | uVar4 << 6) << 6 | (ushort)(param_2[2] & 0x3f);
        param_2 = param_2 + 3;
      }
    }
    else {
LAB_1408ac087:
      param_2 = param_2 + 1;
    }
    *puVar9 = uVar4;
    puVar9 = puVar9 + 1;
  }
  *puVar9 = 0;
  lVar2 = *param_1;
  if (*(int *)(lVar2 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  *(undefined4 *)(lVar2 + -0x10) = 1;
  piVar3 = (int *)0x0;
  if (lVar2 == 0) {
LAB_1408ac0d1:
    piVar8 = piVar3;
    iVar6 = (int)piVar8;
    if (iVar6 < *(int *)(lVar2 + -0xc) + 1) goto LAB_1408ac0ea;
  }
  else {
    do {
      piVar8 = (int *)((longlong)piVar8 + 1);
    } while (*(short *)(lVar2 + (longlong)piVar8 * 2) != 0);
    piVar3 = piVar8;
    if (-1 < (int)piVar8) goto LAB_1408ac0d1;
  }
  iVar6 = (int)piVar8;
  FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff,*(undefined4 *)(lVar2 + -0xc));
LAB_1408ac0ea:
  *(int *)(lVar2 + -8) = iVar6 * 2;
  return;
}



//===========================================================
// thunk_FUN_140205820 @ 142ef3bb8   (5 bytes)
//===========================================================

void thunk_FUN_140205820(undefined8 param_1)

{
  FUN_14019bb50(&DAT_143ad68a0,param_1);
  return;
}



//===========================================================
// FUN_142ef3ac0 @ 142ef3ac0   (16 bytes)
//===========================================================

void FUN_142ef3ac0(undefined8 param_1)

{
  (*(code *)PTR_FUN_1432630d8)(param_1,0);
  return;
}



//===========================================================
// FUN_14019b4e0 @ 14019b4e0   (288 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019b4e0(undefined8 *param_1)

{
  code *pcVar1;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  int *piVar6;
  longlong lVar7;
  bool bVar8;
  
  pvVar2 = Self;
  pcVar1 = DAT_143ad5530;
  if (param_1 == (undefined8 *)0x0) {
    return;
  }
  uVar3 = param_1[-1];
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x21) {
    uVar5 = (ulonglong)(0x10 < uVar3);
  }
  else {
    if (uVar3 < 0x41) {
      uVar5 = 2;
      goto LAB_14019b52b;
    }
    uVar5 = 0xffffffff;
    if (uVar3 < 0x81) {
      uVar5 = 3;
    }
  }
  if ((int)uVar5 < 0) {
    uVar4 = (*DAT_143ad5538)();
    (*pcVar1)(uVar4,0,param_1 + -1);
    return;
  }
LAB_14019b52b:
  lVar7 = uVar5 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad68c8 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad68c8 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019b5b5:
    *(undefined4 *)(&DAT_143ad68d0 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad68c8 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad68c8 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad68c8 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019b5b5;
        if (*(void **)(&DAT_143ad68c8 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad68d0 + lVar7) = *(int *)(&DAT_143ad68d0 + lVar7) + 1;
  }
  piVar6 = (int *)(&DAT_143ad68d0 + lVar7);
  *param_1 = *(undefined8 *)(&DAT_143ad6908 + uVar5 * 8);
  *(undefined8 **)(&DAT_143ad6908 + uVar5 * 8) = param_1;
  _DAT_143ad6948 = *param_1;
  *(int *)(&DAT_143ad68b4 + uVar5 * 4) = *(int *)(&DAT_143ad68b4 + uVar5 * 4) + -1;
  *piVar6 = *piVar6 + -1;
  if (*piVar6 == 0) {
    *(undefined8 *)(&DAT_143ad68c8 + lVar7) = 0;
  }
  return;
}


