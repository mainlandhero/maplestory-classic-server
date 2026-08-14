
//===========================================================
// NMCO_CallNMFunction @ 18001bc40   (254 bytes)
//===========================================================

undefined4
NMCO_CallNMFunction(undefined4 param_1,int *param_2,ulonglong param_3,undefined8 param_4,
                   undefined8 param_5)

{
  code *pcVar1;
  undefined4 uVar2;
  longlong lVar3;
  undefined *puVar4;
  ulonglong uVar5;
  undefined8 uVar6;
  undefined8 local_38;
  longlong local_30 [5];
  
                    /* 0x1bc40  1  NMCO_CallNMFunction */
  puVar4 = &DAT_18006cc00;
  uVar5 = param_3;
  uVar6 = param_4;
  FUN_18001c6e0(&local_38,(undefined8 *)&DAT_18006cc00);
  if (DAT_18006a058 != 0) {
    if (DAT_18006a040 != (code *)0x0) {
      uVar2 = (*DAT_18006a040)(param_1,param_2,param_3 & 0xff,param_4,param_5);
      goto LAB_18001bd1a;
    }
    if (DAT_18006a028 != (code *)0x0) {
      FUN_1800132f0(local_30);
      FUN_180013720(local_30,param_2,0);
      FUN_180013430(local_30,0);
      pcVar1 = DAT_18006a028;
      lVar3 = FUN_1800134f0(local_30);
      uVar2 = (*pcVar1)(param_1,lVar3,param_4,param_5);
      FUN_1800136d0(local_30);
      goto LAB_18001bd1a;
    }
  }
  FUN_1800168e0(L"Fail to load messenger module!",puVar4,uVar5,uVar6);
  uVar2 = 0;
LAB_18001bd1a:
  FUN_18001c730(&local_38);
  return uVar2;
}



//===========================================================
// FUN_18001c6e0 @ 18001c6e0   (29 bytes)
//===========================================================

undefined8 * FUN_18001c6e0(undefined8 *param_1,undefined8 *param_2)

{
  *param_1 = param_2;
  FUN_18001ca70(param_2);
  return param_1;
}



//===========================================================
// FUN_1800136d0 @ 1800136d0   (78 bytes)
//===========================================================

void FUN_1800136d0(undefined8 *param_1)

{
  LPVOID lpMem;
  HANDLE hHeap;
  
  if ((*(int *)(param_1 + 1) == 0) && (lpMem = (LPVOID)*param_1, lpMem != (LPVOID)0x0)) {
    hHeap = GetProcessHeap();
    HeapFree(hHeap,0,lpMem);
  }
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  *(undefined4 *)(param_1 + 3) = 0;
  *(undefined1 *)((longlong)param_1 + 0x1c) = 0;
  return;
}



//===========================================================
// FUN_1800134f0 @ 1800134f0   (145 bytes)
//===========================================================

longlong FUN_1800134f0(longlong *param_1)

{
  undefined4 uVar1;
  longlong *plVar2;
  longlong *plVar3;
  
  plVar3 = DAT_18006c320;
  if (DAT_18006c320 == (longlong *)0x0) {
    plVar2 = operator_new(0x18);
    plVar3 = (longlong *)0x0;
    if (plVar2 != (longlong *)0x0) {
      plVar3 = FUN_180015e30(plVar2);
    }
  }
  DAT_18006c320 = plVar3;
  plVar3 = (longlong *)FUN_1800162b0(DAT_18006c320,*(byte *)((longlong)param_1 + 0x1c));
  if (plVar3 != (longlong *)0x0) {
    uVar1 = (**(code **)(*plVar3 + 0x20))(plVar3);
    *(undefined4 *)*param_1 = uVar1;
    *(undefined4 *)(*param_1 + 4) = *(undefined4 *)((longlong)param_1 + 0xc);
    uVar1 = (**(code **)(*plVar3 + 0x28))(plVar3);
    *(undefined4 *)((ulonglong)*(uint *)((longlong)param_1 + 0xc) + 8 + *param_1) = uVar1;
  }
  return *param_1;
}



//===========================================================
// FUN_1800168e0 @ 1800168e0   (314 bytes)
//===========================================================

/* WARNING: Function: _alloca_probe replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1800168e0(wchar_t *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  FILE *_File;
  ulonglong *puVar1;
  undefined8 local_res10;
  undefined8 local_res18;
  undefined8 local_res20;
  undefined1 auStackY_1088 [32];
  _SYSTEMTIME local_1038;
  undefined2 local_1028 [2048];
  ulonglong local_28;
  
  local_28 = DAT_18006a250 ^ (ulonglong)auStackY_1088;
  local_res10 = param_2;
  local_res18 = param_3;
  local_res20 = param_4;
  if (((*(int *)(DAT_18006c360 + -8) != 0) ||
      (FUN_180016830(DAT_18006c360,param_2,param_3,param_4), *(int *)(DAT_18006c360 + -8) != 0)) &&
     (_File = (FILE *)FUN_180035f60(DAT_18006c360,L"at"), _File != (FILE *)0x0)) {
    GetLocalTime(&local_1038);
    puVar1 = (ulonglong *)FUN_180002ea0();
    __stdio_common_vswprintf
              (*puVar1,local_1028,0x7fffffff,param_1,(__crt_locale_pointers *)0x0,
               (char *)&local_res10);
    FUN_180016a20((longlong)_File,0x18004fa60,(ulonglong)local_1038.wYear,
                  (ulonglong)local_1038.wMonth);
    fclose(_File);
  }
  return;
}



//===========================================================
// FUN_18001c730 @ 18001c730   (27 bytes)
//===========================================================

void FUN_18001c730(undefined8 *param_1)

{
  FUN_18001cc90((undefined8 *)*param_1);
  return;
}



//===========================================================
// FUN_180013430 @ 180013430   (189 bytes)
//===========================================================

undefined8 FUN_180013430(longlong *param_1,undefined1 param_2)

{
  int iVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  
  uVar5 = 0xfffffffffffffffe;
  *(undefined1 *)((longlong)param_1 + 0x1c) = param_2;
  plVar3 = DAT_18006c320;
  if (DAT_18006c320 == (longlong *)0x0) {
    plVar2 = operator_new(0x18);
    plVar3 = (longlong *)0x0;
    if (plVar2 != (longlong *)0x0) {
      plVar3 = FUN_180015e30(plVar2);
    }
  }
  DAT_18006c320 = plVar3;
  plVar3 = (longlong *)FUN_1800162b0(DAT_18006c320,*(byte *)((longlong)param_1 + 0x1c));
  if (plVar3 != (longlong *)0x0) {
    iVar1 = (**(code **)(*plVar3 + 0x30))(plVar3,*(undefined4 *)((longlong)param_1 + 0xc));
    uVar4 = FUN_180013590(param_1,iVar1 - *(int *)((longlong)param_1 + 0xc));
    if (((int)uVar4 != 0) &&
       (iVar1 = (**(code **)(*plVar3 + 8))
                          (plVar3,*param_1 + 8,(int)param_1[2],(longlong)param_1 + 0xc,uVar5),
       iVar1 == 0)) {
      *(int *)((longlong)param_1 + 0x14) = *(int *)((longlong)param_1 + 0xc) + 8;
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_1800132f0 @ 1800132f0   (50 bytes)
//===========================================================

longlong * FUN_1800132f0(longlong *param_1)

{
  *(undefined4 *)(param_1 + 4) = 0;
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  *(undefined4 *)(param_1 + 3) = 0;
  *(undefined1 *)((longlong)param_1 + 0x1c) = 0;
  FUN_180013590(param_1,0x400);
  return param_1;
}



//===========================================================
// FUN_180013720 @ 180013720   (400 bytes)
//===========================================================

undefined8 FUN_180013720(longlong *param_1,int *param_2,int param_3)

{
  int iVar1;
  uint uVar2;
  LPVOID pvVar3;
  undefined1 uVar4;
  int iVar5;
  HANDLE pvVar6;
  longlong *plVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  
  if (param_2 != (int *)0x0) {
    if (((int)param_1[1] == 0) && (pvVar3 = (LPVOID)*param_1, pvVar3 != (LPVOID)0x0)) {
      pvVar6 = GetProcessHeap();
      HeapFree(pvVar6,0,pvVar3);
    }
    *param_1 = 0;
    param_1[1] = 0;
    param_1[2] = 0;
    *(undefined4 *)(param_1 + 3) = 0;
    *(undefined1 *)((longlong)param_1 + 0x1c) = 0;
    iVar1 = *param_2;
    plVar8 = DAT_18006c320;
    if ((DAT_18006c320 == (longlong *)0x0) &&
       (plVar7 = operator_new(0x18), plVar8 = (longlong *)0x0, plVar7 != (longlong *)0x0)) {
      plVar8 = FUN_180015e30(plVar7);
    }
    DAT_18006c320 = plVar8;
    plVar8 = (longlong *)FUN_180016300(DAT_18006c320,iVar1);
    if (plVar8 != (longlong *)0x0) {
      uVar2 = param_2[1];
      *(uint *)((longlong)param_1 + 0xc) = uVar2;
      iVar1 = *(int *)((ulonglong)uVar2 + 8 + (longlong)param_2);
      iVar5 = (**(code **)(*plVar8 + 0x28))(plVar8);
      if (iVar1 == iVar5) {
        *(int *)(param_1 + 1) = param_3;
        if (param_3 != 0) {
          *param_1 = (longlong)param_2;
LAB_180013837:
          *(undefined4 *)((longlong)param_1 + 0x14) = 8;
          uVar4 = (**(code **)(*plVar8 + 0x18))(plVar8);
          *(undefined1 *)((longlong)param_1 + 0x1c) = uVar4;
          *(undefined4 *)(param_1 + 3) = 1;
          FUN_180013330(param_1);
          return 1;
        }
        iVar1 = *(int *)((longlong)param_1 + 0xc);
        pvVar6 = GetProcessHeap();
        puVar9 = HeapAlloc(pvVar6,0,(ulonglong)(iVar1 + 0xc));
        *param_1 = (longlong)puVar9;
        if (puVar9 != (undefined8 *)0x0) {
          FUN_18002de80(puVar9,(undefined8 *)param_2,
                        (ulonglong)(*(int *)((longlong)param_1 + 0xc) + 0xc));
          *(int *)(param_1 + 2) = *(int *)((longlong)param_1 + 0xc) + 0xc;
          goto LAB_180013837;
        }
      }
    }
    if (((int)param_1[1] == 0) && (pvVar3 = (LPVOID)*param_1, pvVar3 != (LPVOID)0x0)) {
      pvVar6 = GetProcessHeap();
      HeapFree(pvVar6,0,pvVar3);
    }
    *param_1 = 0;
    param_1[1] = 0;
    param_1[2] = 0;
    *(undefined4 *)(param_1 + 3) = 0;
    *(undefined1 *)((longlong)param_1 + 0x1c) = 0;
  }
  return 0;
}



//===========================================================
// FUN_18001ca70 @ 18001ca70   (241 bytes)
//===========================================================

void FUN_18001ca70(undefined8 *param_1)

{
  int *piVar1;
  int iVar2;
  undefined1 uVar3;
  DWORD DVar4;
  undefined7 extraout_var;
  undefined *puVar5;
  DWORD dwMilliseconds;
  
  uVar3 = FUN_18001cd10((longlong)param_1);
  if ((int)CONCAT71(extraout_var,uVar3) == 0) {
    DVar4 = GetCurrentThreadId();
    LOCK();
    piVar1 = (int *)(param_1[2] + 8);
    iVar2 = *piVar1;
    *piVar1 = *piVar1 + 1;
    UNLOCK();
    if (iVar2 == 0) {
      *(DWORD *)(param_1[2] + 0xc) = DVar4;
    }
    else {
      piVar1 = (int *)param_1[2];
      if (piVar1[3] == DVar4) {
        piVar1[4] = piVar1[4] + 1;
        return;
      }
      iVar2 = *piVar1;
      if (iVar2 != 0) {
        if (param_1[1] == 0) {
          puVar5 = FUN_18001cc00();
        }
        else {
          puVar5 = FUN_18001cb70();
        }
        FUN_180029970((longlong)(puVar5 + 8),iVar2);
      }
      dwMilliseconds = 10000;
      if (*(int *)(param_1 + 3) == 0) {
        dwMilliseconds = 300000;
      }
      WaitForSingleObject((HANDLE)*param_1,dwMilliseconds);
      *(DWORD *)(param_1[2] + 0xc) = DVar4;
    }
    *(undefined4 *)(param_1[2] + 0x10) = 1;
    iVar2 = *(int *)param_1[2];
    if (iVar2 != 0) {
      if (param_1[1] == 0) {
        puVar5 = FUN_18001cc00();
      }
      else {
        puVar5 = FUN_18001cb70();
      }
      FUN_180029600((longlong)(puVar5 + 8),iVar2);
    }
  }
  return;
}



//===========================================================
// GetProcessHeap @ EXTERNAL:00000024   (0 bytes)
//===========================================================
// decompilation failed: Exception while decompiling EXTERNAL:00000024: Cannot marshal address space: EXTERNAL


//===========================================================
// HeapFree @ EXTERNAL:00000026   (0 bytes)
//===========================================================
// decompilation failed: Exception while decompiling EXTERNAL:00000026: Cannot marshal address space: EXTERNAL


//===========================================================
// FUN_1800162b0 @ 1800162b0   (68 bytes)
//===========================================================

undefined8 FUN_1800162b0(longlong *param_1,byte param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  
  puVar2 = (undefined8 *)*param_1;
  cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
  puVar4 = puVar2;
  puVar3 = (undefined8 *)puVar2[1];
  while (cVar1 == '\0') {
    if (*(byte *)(puVar3 + 4) < param_2) {
      puVar5 = (undefined8 *)puVar3[2];
      puVar3 = puVar4;
    }
    else {
      puVar5 = (undefined8 *)*puVar3;
    }
    puVar4 = puVar3;
    puVar3 = puVar5;
    cVar1 = *(char *)((longlong)puVar5 + 0x19);
  }
  if ((puVar4 == puVar2) || (param_2 < *(byte *)(puVar4 + 4))) {
    puVar4 = puVar2;
  }
  if (puVar4 == puVar2) {
    return 0;
  }
  return puVar4[5];
}



//===========================================================
// operator_new @ 18002b480   (60 bytes)
//===========================================================

/* Library Function - Single Match
    void * __ptr64 __cdecl operator new(unsigned __int64)
   
   Library: Visual Studio 2015 Release */

void * __cdecl operator_new(__uint64 param_1)

{
  int iVar1;
  LPVOID pvVar2;
  
  while( true ) {
    do {
      pvVar2 = _malloc_base(param_1);
      if (pvVar2 != (LPVOID)0x0) {
        return pvVar2;
      }
      iVar1 = _callnewh(param_1);
    } while (iVar1 != 0);
    if (param_1 != 0xffffffffffffffff) break;
    FUN_18002c4cc();
  }
                    /* WARNING: Subroutine does not return */
  FUN_18002b420();
}



//===========================================================
// FUN_180015e30 @ 180015e30   (445 bytes)
//===========================================================

longlong * FUN_180015e30(longlong *param_1)

{
  undefined1 uVar1;
  longlong lVar2;
  longlong *plVar3;
  int *piVar4;
  undefined1 local_30 [8];
  undefined **local_28;
  
  *param_1 = 0;
  param_1[1] = 0;
  lVar2 = FUN_1800165a0();
  *param_1 = lVar2;
  piVar4 = (int *)(*(longlong *)((longlong)ThreadLocalStoragePointer + (ulonglong)_tls_index * 8) +
                  4);
  if (*piVar4 < DAT_18006c358) {
    _Init_thread_header(&DAT_18006c358);
    if (DAT_18006c358 == -1) {
      PTR_vftable_18006a000 = (undefined *)CStreamCryptor_v0::vftable;
      atexit((_func_5014 *)&LAB_18004c930);
      _Init_thread_footer(&DAT_18006c358);
    }
  }
  local_30[0] = (**(code **)(PTR_vftable_18006a000 + 0x18))(&PTR_vftable_18006a000);
  local_28 = &PTR_vftable_18006a000;
  plVar3 = (longlong *)FUN_1800159f0(param_1,local_30);
  FUN_180015cd0(param_1,(undefined8 *)local_30,'\0',(byte *)(plVar3 + 4),plVar3);
  if (*piVar4 < DAT_18006c35c) {
    _Init_thread_header(&DAT_18006c35c);
    if (DAT_18006c35c == -1) {
      PTR_vftable_18006a008 = (undefined *)CStreamCryptor_v1::vftable;
      atexit((_func_5014 *)&LAB_18004c940);
      _Init_thread_footer(&DAT_18006c35c);
    }
  }
  local_30[0] = (**(code **)(PTR_vftable_18006a008 + 0x18))(&PTR_vftable_18006a008);
  local_28 = &PTR_vftable_18006a008;
  plVar3 = (longlong *)FUN_1800159f0(param_1,local_30);
  FUN_180015cd0(param_1,(undefined8 *)local_30,'\0',(byte *)(plVar3 + 4),plVar3);
  if (*piVar4 < DAT_18006c35c) {
    _Init_thread_header(&DAT_18006c35c);
    if (DAT_18006c35c == -1) {
      PTR_vftable_18006a008 = (undefined *)CStreamCryptor_v1::vftable;
      atexit((_func_5014 *)&LAB_18004c940);
      _Init_thread_footer(&DAT_18006c35c);
    }
  }
  uVar1 = (**(code **)(PTR_vftable_18006a008 + 0x18))(&PTR_vftable_18006a008);
  *(undefined1 *)(param_1 + 2) = uVar1;
  return param_1;
}



//===========================================================
// FUN_180035f60 @ 180035f60   (11 bytes)
//===========================================================

void FUN_180035f60(wchar_t *param_1,wchar_t *param_2)

{
  common_fsopen<wchar_t>(param_1,param_2,0x40);
  return;
}



//===========================================================
// fclose @ 180035ff0   (105 bytes)
//===========================================================

/* Library Function - Single Match
    fclose
   
   Library: Visual Studio 2015 Release */

int __cdecl fclose(FILE *_File)

{
  int iVar1;
  ulong *puVar2;
  
  if (_File == (FILE *)0x0) {
    puVar2 = __doserrno();
    *puVar2 = 0x16;
    FUN_18002fdf4();
  }
  else {
    if ((*(uint *)((longlong)&_File->_base + 4) >> 0xc & 1) == 0) {
      FUN_180036d88((longlong)_File);
      iVar1 = _fclose_nolock(_File);
      FUN_180036d94((longlong)_File);
      return iVar1;
    }
    __acrt_stdio_free_stream();
  }
  return -1;
}



//===========================================================
// _alloca_probe @ 18002bd20   (81 bytes)
//===========================================================

/* WARNING: This is an inlined function */
/* Library Function - Single Match
    _alloca_probe
   
   Libraries: Visual Studio 2015, Visual Studio 2017, Visual Studio 2019 */

void _alloca_probe(void)

{
  undefined1 *in_RAX;
  undefined1 *puVar1;
  undefined1 *puVar2;
  undefined1 local_res8 [32];
  
  puVar1 = local_res8 + -(longlong)in_RAX;
  if (local_res8 < in_RAX) {
    puVar1 = (undefined1 *)0x0;
  }
  if (puVar1 < StackLimit) {
    puVar2 = StackLimit;
    do {
      puVar2 = puVar2 + -0x1000;
      *puVar2 = 0;
    } while ((undefined1 *)((ulonglong)puVar1 & 0xfffffffffffff000) != puVar2);
  }
  return;
}



//===========================================================
// FUN_180016a20 @ 180016a20   (68 bytes)
//===========================================================

void FUN_180016a20(longlong param_1,longlong param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 *puVar1;
  undefined8 local_res18;
  undefined8 local_res20;
  
  local_res18 = param_3;
  local_res20 = param_4;
  puVar1 = (undefined8 *)FUN_180002ea0();
  FID_conflict___stdio_common_vfwprintf(*puVar1,param_1,param_2,0,&local_res18);
  return;
}



//===========================================================
// FUN_180016830 @ 180016830   (161 bytes)
//===========================================================

void FUN_180016830(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  int *piVar1;
  int iVar2;
  BOOL BVar3;
  longlong *plVar4;
  undefined8 uVar5;
  longlong local_res8 [4];
  
  plVar4 = (longlong *)FUN_180010f00();
  if (plVar4 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_1800106f0(0x80004005);
  }
  local_res8[0] = (**(code **)(*plVar4 + 0x18))(plVar4);
  local_res8[0] = local_res8[0] + 0x18;
  uVar5 = FUN_180014320(local_res8,1,param_3,param_4);
  if ((int)uVar5 != 0) {
    FUN_180010c70((longlong *)&DAT_18006c360,L"%s\\nmcogame.log",local_res8[0],param_4);
    BVar3 = PathFileExistsW(DAT_18006c360);
    if (BVar3 != 0) {
      DeleteFileW(DAT_18006c360);
    }
  }
  LOCK();
  piVar1 = (int *)(local_res8[0] + -8);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  if (iVar2 < 2) {
    (**(code **)(**(longlong **)(local_res8[0] + -0x18) + 8))();
  }
  return;
}



//===========================================================
// GetLocalTime @ EXTERNAL:00000014   (0 bytes)
//===========================================================
// decompilation failed: Exception while decompiling EXTERNAL:00000014: Cannot marshal address space: EXTERNAL


//===========================================================
// __stdio_common_vswprintf @ 1800344a8   (567 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* Library Function - Single Match
    __stdio_common_vswprintf
   
   Library: Visual Studio 2015 Release */

ulonglong __stdio_common_vswprintf
                    (ulonglong param_1,undefined2 *param_2,ulonglong param_3,wchar_t *param_4,
                    __crt_locale_pointers *param_5,char *param_6)

{
  int iVar1;
  ulong *puVar2;
  ulonglong uVar3;
  undefined1 auStackY_538 [32];
  longlong local_508;
  __crt_locale_pointers local_500 [16];
  char local_4f0;
  undefined2 *local_4e8;
  ulonglong local_4e0;
  ulonglong local_4d8;
  undefined1 local_4d0;
  undefined2 **local_4c8 [2];
  output_processor<wchar_t,__crt_stdio_output::string_output_adapter<wchar_t>,__crt_stdio_output::standard_base<wchar_t,__crt_stdio_output::string_output_adapter<wchar_t>_>_>
  local_4b8 [1120];
  LPVOID local_58;
  ulonglong local_38;
  
  local_38 = DAT_18006a250 ^ (ulonglong)auStackY_538;
  if ((param_4 == (wchar_t *)0x0) || ((param_3 != 0 && (param_2 == (undefined2 *)0x0)))) {
    puVar2 = __doserrno();
    *puVar2 = 0x16;
    FUN_18002fdf4();
  }
  else {
    _LocaleUpdate::_LocaleUpdate((_LocaleUpdate *)&local_508,param_5);
    FUN_18002ea50((undefined1 (*) [16])&local_4e8,0,0x20);
    local_4d8 = 0;
    if (((param_1 & 2) != 0) || (local_4d0 = 0, param_2 == (undefined2 *)0x0)) {
      local_4d0 = 1;
    }
    local_4c8[0] = &local_4e8;
    local_4e8 = param_2;
    local_4e0 = param_3;
    __crt_stdio_output::
    output_processor<wchar_t,__crt_stdio_output::string_output_adapter<wchar_t>,__crt_stdio_output::standard_base<wchar_t,__crt_stdio_output::string_output_adapter<wchar_t>_>_>
    ::
    output_processor<wchar_t,__crt_stdio_output::string_output_adapter<wchar_t>,__crt_stdio_output::standard_base<wchar_t,__crt_stdio_output::string_output_adapter<wchar_t>_>_>
              (local_4b8,(string_output_adapter<wchar_t> *)local_4c8,param_1,param_4,local_500,
               param_6);
    iVar1 = __crt_stdio_output::
            output_processor<wchar_t,__crt_stdio_output::string_output_adapter<wchar_t>,__crt_stdio_output::standard_base<wchar_t,__crt_stdio_output::string_output_adapter<wchar_t>_>_>
            ::process(local_4b8);
    uVar3 = (ulonglong)iVar1;
    if (param_2 == (undefined2 *)0x0) {
      _free_base(local_58);
      if (local_4f0 != '\0') {
        *(uint *)(local_508 + 0x3a8) = *(uint *)(local_508 + 0x3a8) & 0xfffffffd;
      }
      goto LAB_1800346b2;
    }
    if ((param_1 & 1) == 0) {
      if ((param_1 & 2) != 0) {
        if (param_3 != 0) {
          if (iVar1 < 0) {
            *param_2 = 0;
          }
          else {
            if (local_4d8 != param_3) goto LAB_180034687;
            param_2[param_3 - 1] = 0;
          }
        }
LAB_18003468c:
        _free_base(local_58);
        if (local_4f0 != '\0') {
          *(uint *)(local_508 + 0x3a8) = *(uint *)(local_508 + 0x3a8) & 0xfffffffd;
        }
        goto LAB_1800346b2;
      }
      if (param_3 != 0) {
        if (local_4d8 == param_3) {
          param_2[param_3 - 1] = 0;
          _free_base(local_58);
          if (local_4f0 != '\0') {
            *(uint *)(local_508 + 0x3a8) = *(uint *)(local_508 + 0x3a8) & 0xfffffffd;
          }
          uVar3 = 0xfffffffe;
          goto LAB_1800346b2;
        }
        goto LAB_180034687;
      }
    }
    else if ((param_3 != 0) || (iVar1 == 0)) {
      if (local_4d8 != param_3) {
LAB_180034687:
        param_2[local_4d8] = 0;
        goto LAB_18003468c;
      }
      if ((iVar1 < 0) || (uVar3 <= param_3)) goto LAB_18003468c;
    }
    _free_base(local_58);
    if (local_4f0 != '\0') {
      *(uint *)(local_508 + 0x3a8) = *(uint *)(local_508 + 0x3a8) & 0xfffffffd;
    }
  }
  uVar3 = 0xffffffff;
LAB_1800346b2:
  return uVar3 & 0xffffffff;
}



//===========================================================
// __security_check_cookie @ 18002bd90   (33 bytes)
//===========================================================

/* WARNING: This is an inlined function */
/* Library Function - Single Match
    __security_check_cookie
   
   Libraries: Visual Studio 2015, Visual Studio 2017, Visual Studio 2019 */

void __cdecl __security_check_cookie(uintptr_t _StackCookie)

{
  if ((_StackCookie == DAT_18006a250) && ((short)(_StackCookie >> 0x30) == 0)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __report_gsfailure(_StackCookie);
}



//===========================================================
// FUN_180002ea0 @ 180002ea0   (8 bytes)
//===========================================================

undefined * FUN_180002ea0(void)

{
  return &DAT_18006df80;
}



//===========================================================
// FUN_18001cc90 @ 18001cc90   (128 bytes)
//===========================================================

void FUN_18001cc90(undefined8 *param_1)

{
  int *piVar1;
  int iVar2;
  undefined *puVar3;
  
  piVar1 = (int *)(param_1[2] + 0x10);
  *piVar1 = *piVar1 + -1;
  if (*piVar1 != 0) {
    LOCK();
    piVar1 = (int *)(param_1[2] + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    return;
  }
  *(undefined4 *)(param_1[2] + 0xc) = 0;
  iVar2 = *(int *)param_1[2];
  if (iVar2 != 0) {
    if (param_1[1] == 0) {
      puVar3 = FUN_18001cc00();
    }
    else {
      puVar3 = FUN_18001cb70();
    }
    FUN_1800297d0((longlong)(puVar3 + 8),iVar2);
  }
  LOCK();
  piVar1 = (int *)(param_1[2] + 8);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  if (1 < iVar2) {
                    /* WARNING: Could not recover jumptable at 0x00018001cd03. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    SetEvent((HANDLE)*param_1);
    return;
  }
  return;
}



//===========================================================
// FUN_180013590 @ 180013590   (316 bytes)
//===========================================================

undefined8 FUN_180013590(longlong *param_1,int param_2)

{
  HANDLE pvVar1;
  undefined8 *puVar2;
  LPVOID pvVar3;
  uint uVar4;
  
  if ((int)param_1[1] == 0) {
    if ((int)param_1[3] == 0) {
      pvVar3 = (LPVOID)*param_1;
      if (pvVar3 != (LPVOID)0x0) {
        pvVar1 = GetProcessHeap();
        HeapFree(pvVar1,0,pvVar3);
      }
      goto LAB_180013655;
    }
    if (*param_1 != 0) {
      uVar4 = param_2 + *(int *)((longlong)param_1 + 0x14);
      if (uVar4 <= (int)param_1[2] - 4U) {
        return 1;
      }
      uVar4 = (uVar4 + 4 & 0xfffff000) + 0x1000;
      pvVar1 = GetProcessHeap();
      puVar2 = HeapAlloc(pvVar1,0,(ulonglong)uVar4);
      if (puVar2 != (undefined8 *)0x0) {
        FUN_18002de80(puVar2,(undefined8 *)*param_1,(ulonglong)*(uint *)((longlong)param_1 + 0x14));
        pvVar1 = GetProcessHeap();
        HeapFree(pvVar1,0,(LPVOID)*param_1);
        *param_1 = (longlong)puVar2;
        *(uint *)(param_1 + 2) = uVar4;
        return 1;
      }
      goto LAB_1800136ad;
    }
  }
  else {
LAB_180013655:
    *param_1 = 0;
    param_1[1] = 0;
    param_1[2] = 0;
    *(undefined4 *)(param_1 + 3) = 0;
    *(undefined1 *)((longlong)param_1 + 0x1c) = 0;
  }
  uVar4 = (param_2 + 0xcU & 0xfffff000) + 0x1000;
  *(uint *)(param_1 + 2) = uVar4;
  pvVar1 = GetProcessHeap();
  pvVar3 = HeapAlloc(pvVar1,0,(ulonglong)uVar4);
  *param_1 = (longlong)pvVar3;
  *(undefined4 *)((longlong)param_1 + 0xc) = 0;
  *(undefined4 *)(param_1 + 3) = 1;
  if (pvVar3 != (LPVOID)0x0) {
    *(undefined4 *)((longlong)param_1 + 0x14) = 8;
    return 1;
  }
LAB_1800136ad:
  FUN_1800136d0(param_1);
  return 0;
}



//===========================================================
// FUN_18002de80 @ 18002de80   (947 bytes)
//===========================================================

undefined8 * FUN_18002de80(undefined8 *param_1,undefined8 *param_2,ulonglong param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined1 uVar5;
  undefined2 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  longlong lVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  undefined4 uVar21;
  undefined4 uVar22;
  
  switch(param_3) {
  case 0:
    return param_1;
  case 1:
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    return param_1;
  case 2:
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    return param_1;
  case 3:
    uVar5 = *(undefined1 *)((longlong)param_2 + 2);
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    *(undefined1 *)((longlong)param_1 + 2) = uVar5;
    return param_1;
  case 4:
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    return param_1;
  case 5:
    uVar5 = *(undefined1 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined1 *)((longlong)param_1 + 4) = uVar5;
    return param_1;
  case 6:
    uVar6 = *(undefined2 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar6;
    return param_1;
  case 7:
    uVar6 = *(undefined2 *)((longlong)param_2 + 4);
    uVar5 = *(undefined1 *)((longlong)param_2 + 6);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar6;
    *(undefined1 *)((longlong)param_1 + 6) = uVar5;
    return param_1;
  case 8:
    *param_1 = *param_2;
    return param_1;
  case 9:
    uVar5 = *(undefined1 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined1 *)(param_1 + 1) = uVar5;
    return param_1;
  case 10:
    uVar6 = *(undefined2 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar6;
    return param_1;
  case 0xb:
    uVar6 = *(undefined2 *)(param_2 + 1);
    uVar5 = *(undefined1 *)((longlong)param_2 + 10);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar6;
    *(undefined1 *)((longlong)param_1 + 10) = uVar5;
    return param_1;
  case 0xc:
    uVar19 = *(undefined4 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar19;
    return param_1;
  case 0xd:
    uVar19 = *(undefined4 *)(param_2 + 1);
    uVar5 = *(undefined1 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar19;
    *(undefined1 *)((longlong)param_1 + 0xc) = uVar5;
    return param_1;
  case 0xe:
    uVar19 = *(undefined4 *)(param_2 + 1);
    uVar6 = *(undefined2 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar19;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar6;
    return param_1;
  case 0xf:
    uVar19 = *(undefined4 *)(param_2 + 1);
    uVar6 = *(undefined2 *)((longlong)param_2 + 0xc);
    uVar5 = *(undefined1 *)((longlong)param_2 + 0xe);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar19;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar6;
    *(undefined1 *)((longlong)param_1 + 0xe) = uVar5;
    return param_1;
  case 0x10:
    uVar15 = param_2[1];
    *param_1 = *param_2;
    param_1[1] = uVar15;
    return param_1;
  }
  if (0x20 < param_3) {
    lVar12 = (longlong)param_2 - (longlong)param_1;
    if ((param_2 < param_1) && ((longlong)param_1 < (longlong)((longlong)param_2 + param_3))) {
      puVar11 = (undefined8 *)((longlong)param_1 + lVar12 + -0x10 + param_3);
      uVar15 = *puVar11;
      uVar17 = puVar11[1];
      puVar10 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
      uVar13 = param_3 - 0x10;
      puVar11 = puVar10;
      uVar16 = uVar15;
      uVar18 = uVar17;
      if (((ulonglong)puVar10 & 0xf) != 0) {
        puVar11 = (undefined8 *)((ulonglong)puVar10 & 0xfffffffffffffff0);
        uVar16 = *(undefined8 *)(lVar12 + (longlong)puVar11);
        uVar18 = ((undefined8 *)(lVar12 + (longlong)puVar11))[1];
        *puVar10 = uVar15;
        *(undefined8 *)((longlong)param_1 + (param_3 - 8)) = uVar17;
        uVar13 = (longlong)puVar11 - (longlong)param_1;
      }
      uVar14 = uVar13 >> 7;
      if (uVar14 != 0) {
        *puVar11 = uVar16;
        puVar11[1] = uVar18;
        puVar10 = puVar11;
        while( true ) {
          puVar3 = (undefined8 *)(lVar12 + -0x10 + (longlong)puVar10);
          uVar15 = puVar3[1];
          puVar11 = (undefined8 *)(lVar12 + -0x20 + (longlong)puVar10);
          uVar17 = *puVar11;
          uVar16 = puVar11[1];
          puVar11 = puVar10 + -0x10;
          puVar10[-2] = *puVar3;
          puVar10[-1] = uVar15;
          puVar10[-4] = uVar17;
          puVar10[-3] = uVar16;
          puVar3 = (undefined8 *)(lVar12 + 0x50 + (longlong)puVar11);
          uVar15 = puVar3[1];
          puVar4 = (undefined8 *)(lVar12 + 0x40 + (longlong)puVar11);
          uVar17 = *puVar4;
          uVar16 = puVar4[1];
          uVar14 = uVar14 - 1;
          puVar10[-6] = *puVar3;
          puVar10[-5] = uVar15;
          puVar10[-8] = uVar17;
          puVar10[-7] = uVar16;
          puVar3 = (undefined8 *)(lVar12 + 0x30 + (longlong)puVar11);
          uVar15 = puVar3[1];
          puVar4 = (undefined8 *)(lVar12 + 0x20 + (longlong)puVar11);
          uVar17 = *puVar4;
          uVar16 = puVar4[1];
          puVar10[-10] = *puVar3;
          puVar10[-9] = uVar15;
          puVar10[-0xc] = uVar17;
          puVar10[-0xb] = uVar16;
          puVar3 = (undefined8 *)(lVar12 + 0x10 + (longlong)puVar11);
          uVar15 = *puVar3;
          uVar17 = puVar3[1];
          uVar16 = *(undefined8 *)(lVar12 + (longlong)puVar11);
          uVar18 = ((undefined8 *)(lVar12 + (longlong)puVar11))[1];
          if (uVar14 == 0) break;
          puVar10[-0xe] = uVar15;
          puVar10[-0xd] = uVar17;
          *puVar11 = uVar16;
          puVar10[-0xf] = uVar18;
          puVar10 = puVar11;
        }
        puVar10[-0xe] = uVar15;
        puVar10[-0xd] = uVar17;
        uVar13 = uVar13 & 0x7f;
      }
      for (uVar14 = uVar13 >> 4; uVar14 != 0; uVar14 = uVar14 - 1) {
        *puVar11 = uVar16;
        puVar11[1] = uVar18;
        puVar11 = puVar11 + -2;
        uVar16 = *(undefined8 *)(lVar12 + (longlong)puVar11);
        uVar18 = ((undefined8 *)(lVar12 + (longlong)puVar11))[1];
      }
      if ((uVar13 & 0xf) != 0) {
        uVar15 = param_2[1];
        *param_1 = *param_2;
        param_1[1] = uVar15;
      }
      *puVar11 = uVar16;
      puVar11[1] = uVar18;
      return param_1;
    }
    if (param_3 < 0x81) {
      puVar1 = (undefined4 *)(lVar12 + (longlong)param_1);
      uVar19 = *puVar1;
      uVar20 = puVar1[1];
      uVar21 = puVar1[2];
      uVar22 = puVar1[3];
      puVar11 = param_1 + 2;
      uVar13 = param_3 - 0x10;
    }
    else {
      if ((DAT_18006d380 >> 1 & 1) != 0) {
        puVar11 = param_1;
        for (; param_3 != 0; param_3 = param_3 - 1) {
          *(undefined1 *)puVar11 = *(undefined1 *)param_2;
          param_2 = (undefined8 *)((longlong)param_2 + 1);
          puVar11 = (undefined8 *)((longlong)puVar11 + 1);
        }
        return param_1;
      }
      puVar1 = (undefined4 *)(lVar12 + (longlong)param_1);
      uVar7 = puVar1[1];
      uVar8 = puVar1[2];
      uVar9 = puVar1[3];
      puVar10 = param_1 + 2;
      uVar19 = *puVar1;
      uVar20 = uVar7;
      uVar21 = uVar8;
      uVar22 = uVar9;
      if (((ulonglong)param_1 & 0xf) != 0) {
        puVar2 = (undefined4 *)(lVar12 + ((ulonglong)puVar10 & 0xfffffffffffffff0));
        uVar19 = *puVar2;
        uVar20 = puVar2[1];
        uVar21 = puVar2[2];
        uVar22 = puVar2[3];
        puVar10 = (undefined8 *)(((ulonglong)puVar10 & 0xfffffffffffffff0) + 0x10);
        *(undefined4 *)param_1 = *puVar1;
        *(undefined4 *)((longlong)param_1 + 4) = uVar7;
        *(undefined4 *)(param_1 + 1) = uVar8;
        *(undefined4 *)((longlong)param_1 + 0xc) = uVar9;
      }
      uVar13 = (longlong)param_1 + (param_3 - (longlong)puVar10);
      uVar14 = uVar13 >> 7;
      puVar11 = puVar10;
      if (uVar14 != 0) {
        *(undefined4 *)(puVar10 + -2) = uVar19;
        *(undefined4 *)((longlong)puVar10 + -0xc) = uVar20;
        *(undefined4 *)(puVar10 + -1) = uVar21;
        *(undefined4 *)((longlong)puVar10 + -4) = uVar22;
        if (DAT_18006a270 < uVar14) {
          while( true ) {
            uVar15 = ((undefined8 *)(lVar12 + (longlong)puVar10))[1];
            puVar11 = (undefined8 *)(lVar12 + 0x10 + (longlong)puVar10);
            uVar17 = *puVar11;
            uVar16 = puVar11[1];
            puVar11 = puVar10 + 0x10;
            *puVar10 = *(undefined8 *)(lVar12 + (longlong)puVar10);
            puVar10[1] = uVar15;
            puVar10[2] = uVar17;
            puVar10[3] = uVar16;
            puVar3 = (undefined8 *)(lVar12 + -0x60 + (longlong)puVar11);
            uVar15 = puVar3[1];
            puVar4 = (undefined8 *)(lVar12 + -0x50 + (longlong)puVar11);
            uVar17 = *puVar4;
            uVar16 = puVar4[1];
            uVar14 = uVar14 - 1;
            puVar10[4] = *puVar3;
            puVar10[5] = uVar15;
            puVar10[6] = uVar17;
            puVar10[7] = uVar16;
            puVar3 = (undefined8 *)(lVar12 + -0x40 + (longlong)puVar11);
            uVar15 = puVar3[1];
            puVar4 = (undefined8 *)(lVar12 + -0x30 + (longlong)puVar11);
            uVar17 = *puVar4;
            uVar16 = puVar4[1];
            puVar10[8] = *puVar3;
            puVar10[9] = uVar15;
            puVar10[10] = uVar17;
            puVar10[0xb] = uVar16;
            puVar3 = (undefined8 *)(lVar12 + -0x20 + (longlong)puVar11);
            uVar15 = *puVar3;
            uVar17 = puVar3[1];
            puVar1 = (undefined4 *)(lVar12 + -0x10 + (longlong)puVar11);
            uVar19 = *puVar1;
            uVar20 = puVar1[1];
            uVar21 = puVar1[2];
            uVar22 = puVar1[3];
            if (uVar14 == 0) break;
            puVar10[0xc] = uVar15;
            puVar10[0xd] = uVar17;
            *(undefined4 *)(puVar10 + 0xe) = uVar19;
            *(undefined4 *)((longlong)puVar10 + 0x74) = uVar20;
            *(undefined4 *)(puVar10 + 0xf) = uVar21;
            *(undefined4 *)((longlong)puVar10 + 0x7c) = uVar22;
            puVar10 = puVar11;
          }
        }
        else {
          while( true ) {
            uVar15 = ((undefined8 *)(lVar12 + (longlong)puVar10))[1];
            puVar11 = (undefined8 *)(lVar12 + 0x10 + (longlong)puVar10);
            uVar17 = *puVar11;
            uVar16 = puVar11[1];
            puVar11 = puVar10 + 0x10;
            *puVar10 = *(undefined8 *)(lVar12 + (longlong)puVar10);
            puVar10[1] = uVar15;
            puVar10[2] = uVar17;
            puVar10[3] = uVar16;
            puVar3 = (undefined8 *)(lVar12 + -0x60 + (longlong)puVar11);
            uVar15 = puVar3[1];
            puVar4 = (undefined8 *)(lVar12 + -0x50 + (longlong)puVar11);
            uVar17 = *puVar4;
            uVar16 = puVar4[1];
            uVar14 = uVar14 - 1;
            puVar10[4] = *puVar3;
            puVar10[5] = uVar15;
            puVar10[6] = uVar17;
            puVar10[7] = uVar16;
            puVar3 = (undefined8 *)(lVar12 + -0x40 + (longlong)puVar11);
            uVar15 = puVar3[1];
            puVar4 = (undefined8 *)(lVar12 + -0x30 + (longlong)puVar11);
            uVar17 = *puVar4;
            uVar16 = puVar4[1];
            puVar10[8] = *puVar3;
            puVar10[9] = uVar15;
            puVar10[10] = uVar17;
            puVar10[0xb] = uVar16;
            puVar3 = (undefined8 *)(lVar12 + -0x20 + (longlong)puVar11);
            uVar15 = *puVar3;
            uVar17 = puVar3[1];
            puVar1 = (undefined4 *)(lVar12 + -0x10 + (longlong)puVar11);
            uVar19 = *puVar1;
            uVar20 = puVar1[1];
            uVar21 = puVar1[2];
            uVar22 = puVar1[3];
            if (uVar14 == 0) break;
            puVar10[0xc] = uVar15;
            puVar10[0xd] = uVar17;
            *(undefined4 *)(puVar10 + 0xe) = uVar19;
            *(undefined4 *)((longlong)puVar10 + 0x74) = uVar20;
            *(undefined4 *)(puVar10 + 0xf) = uVar21;
            *(undefined4 *)((longlong)puVar10 + 0x7c) = uVar22;
            puVar10 = puVar11;
          }
        }
        puVar11[-4] = uVar15;
        puVar11[-3] = uVar17;
        uVar13 = uVar13 & 0x7f;
      }
    }
    for (uVar14 = uVar13 >> 4; uVar14 != 0; uVar14 = uVar14 - 1) {
      *(undefined4 *)(puVar11 + -2) = uVar19;
      *(undefined4 *)((longlong)puVar11 + -0xc) = uVar20;
      *(undefined4 *)(puVar11 + -1) = uVar21;
      *(undefined4 *)((longlong)puVar11 + -4) = uVar22;
      puVar1 = (undefined4 *)(lVar12 + (longlong)puVar11);
      uVar19 = *puVar1;
      uVar20 = puVar1[1];
      uVar21 = puVar1[2];
      uVar22 = puVar1[3];
      puVar11 = puVar11 + 2;
    }
    uVar13 = uVar13 & 0xf;
    if (uVar13 != 0) {
      puVar10 = (undefined8 *)((longlong)puVar11 + lVar12 + -0x10 + uVar13);
      uVar15 = puVar10[1];
      *(undefined8 *)((longlong)puVar11 + (uVar13 - 0x10)) = *puVar10;
      *(undefined8 *)((longlong)puVar11 + (uVar13 - 8)) = uVar15;
    }
    *(undefined4 *)(puVar11 + -2) = uVar19;
    *(undefined4 *)((longlong)puVar11 + -0xc) = uVar20;
    *(undefined4 *)(puVar11 + -1) = uVar21;
    *(undefined4 *)((longlong)puVar11 + -4) = uVar22;
    return param_1;
  }
  uVar15 = param_2[1];
  puVar11 = (undefined8 *)((param_3 - 0x10) + (longlong)param_2);
  uVar17 = *puVar11;
  uVar16 = puVar11[1];
  *param_1 = *param_2;
  param_1[1] = uVar15;
  puVar11 = (undefined8 *)((param_3 - 0x10) + (longlong)param_1);
  *puVar11 = uVar17;
  puVar11[1] = uVar16;
  return param_1;
}



//===========================================================
// FUN_180016300 @ 180016300   (169 bytes)
//===========================================================

undefined8 FUN_180016300(longlong *param_1,int param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  int iVar6;
  
  puVar2 = (undefined8 *)*param_1;
  puVar3 = (undefined8 *)*puVar2;
  while( true ) {
    if (puVar3 == puVar2) {
      return 0;
    }
    iVar6 = (**(code **)(*(longlong *)puVar3[5] + 0x20))();
    if (param_2 == iVar6) break;
    if (*(char *)((longlong)puVar3 + 0x19) == '\0') {
      puVar4 = (undefined8 *)puVar3[2];
      if (*(char *)((longlong)puVar4 + 0x19) == '\0') {
        cVar1 = *(char *)((longlong)*puVar4 + 0x19);
        puVar3 = puVar4;
        puVar4 = (undefined8 *)*puVar4;
        while (cVar1 == '\0') {
          cVar1 = *(char *)((longlong)*puVar4 + 0x19);
          puVar3 = puVar4;
          puVar4 = (undefined8 *)*puVar4;
        }
      }
      else {
        cVar1 = *(char *)((longlong)puVar3[1] + 0x19);
        puVar5 = (undefined8 *)puVar3[1];
        puVar4 = puVar3;
        while ((puVar3 = puVar5, cVar1 == '\0' && (puVar4 == (undefined8 *)puVar3[2]))) {
          cVar1 = *(char *)((longlong)puVar3[1] + 0x19);
          puVar5 = (undefined8 *)puVar3[1];
          puVar4 = puVar3;
        }
      }
    }
  }
  return puVar3[5];
}



//===========================================================
// FUN_180013330 @ 180013330   (255 bytes)
//===========================================================

undefined8 FUN_180013330(longlong *param_1)

{
  int iVar1;
  longlong *plVar2;
  longlong *plVar3;
  HANDLE hHeap;
  undefined8 *puVar4;
  uint uVar5;
  undefined8 uVar6;
  
  uVar6 = 0xfffffffffffffffe;
  if (*(char *)((longlong)param_1 + 0x1c) == '\0') {
LAB_180013357:
    uVar6 = 1;
  }
  else {
    plVar3 = DAT_18006c320;
    if (DAT_18006c320 == (longlong *)0x0) {
      plVar2 = operator_new(0x18);
      plVar3 = (longlong *)0x0;
      if (plVar2 != (longlong *)0x0) {
        plVar3 = FUN_180015e30(plVar2);
      }
    }
    DAT_18006c320 = plVar3;
    plVar3 = (longlong *)FUN_1800162b0(DAT_18006c320,*(byte *)((longlong)param_1 + 0x1c));
    if (plVar3 != (longlong *)0x0) {
      if ((int)param_1[1] != 0) {
        uVar5 = (*(int *)((longlong)param_1 + 0xc) + 0xcU & 0xfffff000) + 0x1000;
        hHeap = GetProcessHeap();
        puVar4 = HeapAlloc(hHeap,0,(ulonglong)uVar5);
        if (puVar4 == (undefined8 *)0x0) goto LAB_180013417;
        FUN_18002de80(puVar4,(undefined8 *)*param_1,
                      (ulonglong)(*(int *)((longlong)param_1 + 0xc) + 0xc));
        *param_1 = (longlong)puVar4;
        *(uint *)(param_1 + 2) = uVar5;
        *(undefined4 *)(param_1 + 1) = 0;
      }
      iVar1 = (**(code **)(*plVar3 + 0x10))
                        (plVar3,*param_1 + 8,(int)param_1[2],(longlong)param_1 + 0xc,uVar6);
      if (iVar1 == 0) goto LAB_180013357;
    }
LAB_180013417:
    uVar6 = 0;
  }
  return uVar6;
}



//===========================================================
// HeapAlloc @ EXTERNAL:00000028   (0 bytes)
//===========================================================
// decompilation failed: Exception while decompiling EXTERNAL:00000028: Cannot marshal address space: EXTERNAL


//===========================================================
// FUN_18001cd10 @ 18001cd10   (235 bytes)
//===========================================================

undefined1 FUN_18001cd10(longlong param_1)

{
  int *piVar1;
  int iVar2;
  longlong lVar3;
  DWORD DVar4;
  undefined *puVar5;
  int iVar6;
  bool bVar7;
  
  DVar4 = GetCurrentThreadId();
  iVar6 = *(int *)(*(longlong *)(param_1 + 0x10) + 4) + 1;
  if (*(longlong *)(param_1 + 0x10) == 0) {
    uRam0000000000000000 = 1;
    return 0;
  }
  while( true ) {
    piVar1 = (int *)(*(longlong *)(param_1 + 0x10) + 8);
    LOCK();
    bVar7 = *piVar1 == 0;
    if (bVar7) {
      *piVar1 = 1;
    }
    UNLOCK();
    lVar3 = *(longlong *)(param_1 + 0x10);
    if (bVar7) {
      *(DWORD *)(lVar3 + 0xc) = DVar4;
      *(undefined4 *)(*(longlong *)(param_1 + 0x10) + 0x10) = 1;
      iVar2 = **(int **)(param_1 + 0x10);
      if (iVar2 != 0) {
        if (*(longlong *)(param_1 + 8) == 0) {
          puVar5 = FUN_18001cc00();
          FUN_180029600((longlong)(puVar5 + 8),iVar2);
        }
        else {
          puVar5 = FUN_18001cb70();
          FUN_180029600((longlong)(puVar5 + 8),iVar2);
        }
      }
    }
    else if (*(DWORD *)(lVar3 + 0xc) == DVar4) {
      LOCK();
      *(int *)(lVar3 + 8) = *(int *)(lVar3 + 8) + 1;
      UNLOCK();
      piVar1 = (int *)(*(longlong *)(param_1 + 0x10) + 0x10);
      *piVar1 = *piVar1 + 1;
      return 1;
    }
    if (bVar7) break;
    iVar6 = iVar6 + -1;
    if (iVar6 == 0) {
      return 0;
    }
  }
  return 1;
}



//===========================================================
// FUN_18001cb70 @ 18001cb70   (129 bytes)
//===========================================================

undefined * FUN_18001cb70(void)

{
  if ((*(int *)(*(longlong *)((longlong)ThreadLocalStoragePointer + (ulonglong)_tls_index * 8) + 4)
       < DAT_18006cce0) && (_Init_thread_header(&DAT_18006cce0), DAT_18006cce0 == -1)) {
    FUN_180028bf0((undefined8 *)&DAT_18006cca0);
    atexit((_func_5014 *)&LAB_18004ce70);
    _Init_thread_footer(&DAT_18006cce0);
  }
  return &DAT_18006cca0;
}



//===========================================================
// FUN_180029970 @ 180029970   (354 bytes)
//===========================================================

longlong FUN_180029970(longlong param_1,int param_2)

{
  DWORD DVar1;
  DWORD DVar2;
  longlong lVar3;
  longlong lVar4;
  bool bVar5;
  
  lVar3 = 0;
  DVar1 = GetCurrentThreadId();
  lVar4 = lVar3;
  if (*(int *)(param_1 + 0x18) == 0) {
    do {
      LOCK();
      bVar5 = **(int **)(param_1 + 0x10) == 0;
      if (bVar5) {
        **(int **)(param_1 + 0x10) = 1;
      }
      UNLOCK();
    } while (bVar5);
  }
  else {
    LOCK();
    bVar5 = **(int **)(param_1 + 0x10) == 0;
    if (bVar5) {
      **(int **)(param_1 + 0x10) = 1;
    }
    UNLOCK();
    while (bVar5) {
      Sleep(0);
      LOCK();
      bVar5 = **(int **)(param_1 + 0x10) == 0;
      if (bVar5) {
        **(int **)(param_1 + 0x10) = 1;
      }
      UNLOCK();
    }
  }
  do {
    if (((int)lVar4 == 0) && (*(int *)(lVar3 + 0x7d0c + *(longlong *)(param_1 + 0x10)) == 0)) {
      *(int *)(lVar3 + 0x7d0c + *(longlong *)(param_1 + 0x10)) = param_2;
      *(DWORD *)(lVar3 + 0x7d10 + *(longlong *)(param_1 + 0x10)) = DVar1;
      DVar2 = GetTickCount();
      lVar4 = 1;
      *(DWORD *)(lVar3 + 0x7d14 + *(longlong *)(param_1 + 0x10)) = DVar2;
LAB_180029a80:
      *(undefined4 *)(lVar3 + 0x7d18 + *(longlong *)(param_1 + 0x10)) = 0;
    }
    else if ((*(int *)(lVar3 + 0x7d0c + *(longlong *)(param_1 + 0x10)) == param_2) &&
            (*(DWORD *)(lVar3 + 0x7d10 + *(longlong *)(param_1 + 0x10)) == DVar1)) {
      (**(code **)(**(longlong **)(param_1 + 8) + 8))
                (*(longlong **)(param_1 + 8),
                 L"This thread already waits lock! ThreadID = %d, CSLockID = %d",DVar1,param_2);
      *(undefined4 *)(lVar3 + 0x7d0c + *(longlong *)(param_1 + 0x10)) = 0;
      *(undefined4 *)(lVar3 + 0x7d14 + *(longlong *)(param_1 + 0x10)) = 0;
      *(undefined4 *)(lVar3 + 0x7d10 + *(longlong *)(param_1 + 0x10)) = 0;
      goto LAB_180029a80;
    }
    lVar3 = lVar3 + 0x10;
    if (31999 < lVar3) {
      LOCK();
      **(int **)(param_1 + 0x10) = **(int **)(param_1 + 0x10) + -1;
      UNLOCK();
      if ((int)lVar4 == 0) {
        (**(code **)(**(longlong **)(param_1 + 8) + 8))
                  (*(longlong **)(param_1 + 8),
                   L"There is no space to register this item! (Function WaitLock())");
      }
      return lVar4;
    }
  } while( true );
}



//===========================================================
// FUN_180029600 @ 180029600   (439 bytes)
//===========================================================

ulonglong FUN_180029600(longlong param_1,int param_2)

{
  DWORD DVar1;
  DWORD DVar2;
  int *piVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  bool bVar9;
  
  uVar7 = 0;
  DVar1 = GetCurrentThreadId();
  if (*(int *)(param_1 + 0x18) == 0) {
    do {
      LOCK();
      bVar9 = **(int **)(param_1 + 0x10) == 0;
      if (bVar9) {
        **(int **)(param_1 + 0x10) = 1;
      }
      UNLOCK();
    } while (bVar9);
  }
  else {
    LOCK();
    bVar9 = **(int **)(param_1 + 0x10) == 0;
    if (bVar9) {
      **(int **)(param_1 + 0x10) = 1;
    }
    UNLOCK();
    while (bVar9) {
      Sleep(0);
      LOCK();
      bVar9 = **(int **)(param_1 + 0x10) == 0;
      if (bVar9) {
        **(int **)(param_1 + 0x10) = 1;
      }
      UNLOCK();
    }
  }
  piVar3 = (int *)(*(longlong *)(param_1 + 0x10) + 0x7d0c);
  uVar4 = uVar7;
  uVar6 = uVar7;
  do {
    uVar8 = uVar7;
    if ((piVar3[1] == DVar1) && (*piVar3 == param_2)) {
      lVar5 = (longlong)(int)uVar6;
      *(undefined4 *)(*(longlong *)(param_1 + 0x10) + 0x7d0c + lVar5 * 0x10) = 0;
      *(undefined4 *)(*(longlong *)(param_1 + 0x10) + 0x7d14 + lVar5 * 0x10) = 0;
      *(undefined4 *)(*(longlong *)(param_1 + 0x10) + (lVar5 + 0x7d1) * 0x10) = 0;
      *(undefined4 *)(*(longlong *)(param_1 + 0x10) + 0x7d18 + lVar5 * 0x10) = 0;
      break;
    }
    uVar6 = (ulonglong)((int)uVar6 + 1);
    uVar4 = uVar4 + 1;
    piVar3 = piVar3 + 4;
  } while ((longlong)uVar4 < 2000);
  do {
    if (((int)uVar8 == 0) && (*(int *)(uVar7 + 0xc + *(longlong *)(param_1 + 0x10)) == 0)) {
      *(int *)(uVar7 + 0xc + *(longlong *)(param_1 + 0x10)) = param_2;
      *(DWORD *)(uVar7 + 0x10 + *(longlong *)(param_1 + 0x10)) = DVar1;
      DVar2 = GetTickCount();
      uVar8 = 1;
      *(DWORD *)(uVar7 + 0x14 + *(longlong *)(param_1 + 0x10)) = DVar2;
LAB_180029765:
      *(undefined4 *)(uVar7 + 0x18 + *(longlong *)(param_1 + 0x10)) = 0;
    }
    else if (*(int *)(uVar7 + 0xc + *(longlong *)(param_1 + 0x10)) == param_2) {
      (**(code **)(**(longlong **)(param_1 + 8) + 8))
                (*(longlong **)(param_1 + 8),
                 L"Attempt to enter lock which is already owned! ThreadID = %d, CSLockID = %d",DVar1
                 ,param_2);
      if (*(DWORD *)(uVar7 + 0x10 + *(longlong *)(param_1 + 0x10)) != DVar1) {
        *(undefined4 *)(uVar7 + 0xc + *(longlong *)(param_1 + 0x10)) = 0;
        *(undefined4 *)(uVar7 + 0x14 + *(longlong *)(param_1 + 0x10)) = 0;
        *(undefined4 *)(uVar7 + 0x10 + *(longlong *)(param_1 + 0x10)) = 0;
        goto LAB_180029765;
      }
    }
    uVar7 = uVar7 + 0x10;
    if (31999 < (longlong)uVar7) {
      LOCK();
      **(int **)(param_1 + 0x10) = **(int **)(param_1 + 0x10) + -1;
      UNLOCK();
      if ((int)uVar8 == 0) {
        (**(code **)(**(longlong **)(param_1 + 8) + 8))
                  (*(longlong **)(param_1 + 8),
                   L"There is no space to register this item! (Function EnterLock())");
      }
      return uVar8;
    }
  } while( true );
}



//===========================================================
// WaitForSingleObject @ EXTERNAL:00000059   (0 bytes)
//===========================================================
// decompilation failed: Exception while decompiling EXTERNAL:00000059: Cannot marshal address space: EXTERNAL


//===========================================================
// GetCurrentThreadId @ EXTERNAL:00000057   (0 bytes)
//===========================================================
// decompilation failed: Exception while decompiling EXTERNAL:00000057: Cannot marshal address space: EXTERNAL


//===========================================================
// FUN_18001cc00 @ 18001cc00   (129 bytes)
//===========================================================

undefined * FUN_18001cc00(void)

{
  if ((*(int *)(*(longlong *)((longlong)ThreadLocalStoragePointer + (ulonglong)_tls_index * 8) + 4)
       < DAT_18006cc90) && (_Init_thread_header(&DAT_18006cc90), DAT_18006cc90 == -1)) {
    FUN_180028d20((undefined8 *)&DAT_18006cc58);
    atexit((_func_5014 *)&LAB_18004ce80);
    _Init_thread_footer(&DAT_18006cc90);
  }
  return &DAT_18006cc58;
}


