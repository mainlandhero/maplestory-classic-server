
//===========================================================
// FUN_1406e8380 @ 1406e8380   (56 bytes)
//===========================================================

undefined8 FUN_1406e8380(undefined4 *param_1,undefined4 *param_2,uint param_3)

{
  undefined1 local_18 [24];
  
  if (3 < param_3) {
    *param_1 = *param_2;
    return 4;
  }
  FUN_1401bb8b0(local_18,0x26);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_18,(ThrowInfo *)&DAT_143a3b118);
}



//===========================================================
// FUN_1406e82f0 @ 1406e82f0   (57 bytes)
//===========================================================

undefined8 FUN_1406e82f0(undefined1 *param_1,undefined1 *param_2,int param_3)

{
  undefined1 local_18 [24];
  
  if (param_3 != 0) {
    *param_1 = *param_2;
    return 1;
  }
  FUN_1401bb8b0(local_18,0x26);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_18,(ThrowInfo *)&DAT_143a3b118);
}



//===========================================================
// _CxxThrowException @ 142ef6d4c   (160 bytes)
//===========================================================

/* Library Function - Single Match
    _CxxThrowException
   
   Library: Visual Studio 2019 Release */

void __stdcall _CxxThrowException(void *pExceptionObject,ThrowInfo *pThrowInfo)

{
  undefined8 uVar1;
  longlong local_38;
  undefined8 local_30;
  void *local_28;
  ThrowInfo *local_20;
  longlong local_18;
  
  uVar1 = 0x19930520;
  if ((pThrowInfo != (ThrowInfo *)0x0) && ((pThrowInfo->attributes & 0x10) != 0)) {
    pThrowInfo = *(ThrowInfo **)(*(longlong *)(*(longlong *)pExceptionObject + -8) + 0x30);
    (*(code *)PTR_FUN_1432630d8)();
  }
  local_38 = (*DAT_143262ff0)(pThrowInfo,&local_38);
  if ((pThrowInfo != (ThrowInfo *)0x0) && (((pThrowInfo->attributes & 8) != 0 || (local_38 == 0))))
  {
    uVar1 = 0x1994000;
  }
  local_30 = uVar1;
  local_28 = pExceptionObject;
  local_20 = pThrowInfo;
  local_18 = local_38;
  (*DAT_143262280)(0xe06d7363,1,4,&local_30);
  return;
}



//===========================================================
// FUN_1401bb8b0 @ 1401bb8b0   (14 bytes)
//===========================================================

undefined4 * FUN_1401bb8b0(undefined4 *param_1,undefined4 param_2)

{
  *param_1 = param_2;
  *(undefined8 *)(param_1 + 2) = 0;
  return param_1;
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


