
//===========================================================
// FUN_140ceffd0 @ 140ceffd0   (99 bytes)
//===========================================================

ulonglong FUN_140ceffd0(IUnknown *param_1)

{
  ulonglong uVar1;
  
  uVar1 = (**(code **)(*(longlong *)param_1 + 0x128))();
  if ((int)uVar1 < 0) {
    _com_issue_errorex((int)uVar1,param_1,(_GUID *)&DAT_14327ac98);
    uVar1 = uVar1 & 0xffffffff;
  }
  return uVar1;
}



//===========================================================
// _com_issue_errorex @ 142ef3ad0   (184 bytes)
//===========================================================

/* Library Function - Single Match
    void __cdecl _com_issue_errorex(long,struct IUnknown * __ptr64,struct _GUID const & __ptr64)
   
   Libraries: Visual Studio 2017 Release, Visual Studio 2019 Release */

void __cdecl _com_issue_errorex(long param_1,IUnknown *param_2,_GUID *param_3)

{
  int iVar1;
  undefined8 local_res10;
  undefined8 local_res20;
  
  local_res10 = 0;
  if ((param_2 != (IUnknown *)0x0) &&
     (iVar1 = (*(code *)PTR_FUN_1432630d8)(param_2,&DAT_1434a1968,&local_res20), -1 < iVar1)) {
    iVar1 = (*(code *)PTR_FUN_1432630d8)(local_res20,param_3);
    (*(code *)PTR_FUN_1432630d8)();
    if ((iVar1 == 0) && (iVar1 = (*DAT_1432629c0)(0,&local_res10), iVar1 != 0)) {
      local_res10 = 0;
    }
  }
  (*(code *)PTR_FUN_1432630d8)(param_1,local_res10);
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


