
//===========================================================
// FUN_14108d8d0 @ 14108d8d0   (203 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14108d8d0(void)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  FUN_1406ed520(local_468,0xa9);
  uVar2 = FUN_142cb8460(DAT_143aa84a0);
  FUN_1406ed9d0(local_468,uVar2);
  FUN_1406ed9d0(local_468,(longlong)DAT_143ac9880 - (longlong)DAT_143ac9878 >> 2);
  puVar1 = DAT_143ac9880;
  for (puVar3 = DAT_143ac9878; puVar3 != puVar1; puVar3 = puVar3 + 1) {
    FUN_1406ed9d0(local_468,*puVar3);
  }
  FUN_1415d01c0(local_468);
  FUN_1406ed610(local_468);
  return;
}


