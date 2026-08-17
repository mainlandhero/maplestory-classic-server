
//===========================================================
// FUN_142c4f490 @ 142c4f490   (215 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142c4f490(longlong param_1)

{
  char cVar1;
  undefined4 uVar2;
  int iVar3;
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  FUN_1406ed520(local_468,0x7a);
  FUN_1406ed840(local_468,*(char *)(param_1 + 0xee) == '\0');
  if (DAT_143ad1ff8 == 0) {
    FUN_1406ed840(local_468,0);
  }
  else {
    cVar1 = FUN_141b0f910();
    FUN_1406ed840(local_468,cVar1);
    if (cVar1 != '\0') {
      iVar3 = 0;
      do {
        uVar2 = FUN_141b0ec20(iVar3);
        FUN_1406ed9d0(local_468,uVar2);
        iVar3 = iVar3 + 1;
      } while (iVar3 < 4);
      uVar2 = FUN_141b0ec60();
      FUN_1406ed9d0(local_468,uVar2);
    }
  }
  FUN_1415d01c0(local_468);
  FUN_1406ed610(local_468);
  return;
}


