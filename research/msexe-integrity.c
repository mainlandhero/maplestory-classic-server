
//===========================================================
// FUN_1415dde80 @ 1415dde80   (217 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1415dde80(void)

{
  int iVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined1 auStack_4a8 [32];
  undefined1 local_488 [8];
  undefined4 local_480;
  undefined8 local_478;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  uVar3 = FUN_142e56b40();
  FUN_1415ddd10(local_488,uVar3,0x2000);
  FUN_140c92fd0(1);
  iVar1 = FUN_1415e3fd0(local_488);
  if (iVar1 == 0) {
    FUN_1406ed520(local_468,0x8f);
    uVar2 = FUN_140a00790(local_488);
    FUN_1406ed9d0(local_468,uVar2);
    local_480 = FUN_140a00790(local_488);
    local_478 = FUN_1415e2f80(local_488);
    FUN_1406ede20(local_468,local_478,local_480);
    FUN_1415d01c0(local_468);
    FUN_1406ed610(local_468);
  }
  FUN_14035a250(local_488);
  return;
}



//===========================================================
// FUN_1415ddf60 @ 1415ddf60   (207 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1415ddf60(void)

{
  int iVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined1 auStack_4a8 [32];
  undefined1 local_488 [8];
  undefined4 local_480;
  undefined8 local_478;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  uVar3 = FUN_142e56bc0();
  FUN_1415ddd10(local_488,uVar3,0x2000);
  iVar1 = FUN_1415e3fd0(local_488);
  if (iVar1 == 0) {
    FUN_1406ed520(local_468,0x90);
    uVar2 = FUN_140a00790(local_488);
    FUN_1406ed9d0(local_468,uVar2);
    local_480 = FUN_140a00790(local_488);
    local_478 = FUN_1415e2f80(local_488);
    FUN_1406ede20(local_468,local_478,local_480);
    FUN_1415d01c0(local_468);
    FUN_1406ed610(local_468);
  }
  FUN_14035a250(local_488);
  return;
}



//===========================================================
// FUN_1415de040 @ 1415de040   (207 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1415de040(void)

{
  int iVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined1 auStack_4a8 [32];
  undefined1 local_488 [8];
  undefined4 local_480;
  undefined8 local_478;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  uVar3 = FUN_142c4ad20();
  FUN_1415ddd10(local_488,uVar3,0x2000);
  iVar1 = FUN_1415e3fd0(local_488);
  if (iVar1 == 0) {
    FUN_1406ed520(local_468,0x91);
    uVar2 = FUN_140a00790(local_488);
    FUN_1406ed9d0(local_468,uVar2);
    local_480 = FUN_140a00790(local_488);
    local_478 = FUN_1415e2f80(local_488);
    FUN_1406ede20(local_468,local_478,local_480);
    FUN_1415d01c0(local_468);
    FUN_1406ed610(local_468);
  }
  FUN_14035a250(local_488);
  return;
}



//===========================================================
// FUN_1415e2f80 @ 1415e2f80   (76 bytes)
//===========================================================

longlong FUN_1415e2f80(longlong *param_1)

{
  longlong lVar1;
  
  lVar1 = *param_1;
  if (lVar1 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar1 = *param_1;
    if (lVar1 == 0) goto LAB_1415e2fb7;
  }
  if (*(int *)(lVar1 + -8) != 0) {
    return lVar1;
  }
LAB_1415e2fb7:
  FUN_142e54290(0xbc,0,0);
  return *param_1;
}



//===========================================================
// FUN_140a00790 @ 140a00790   (13 bytes)
//===========================================================

undefined4 FUN_140a00790(longlong *param_1)

{
  if (*param_1 == 0) {
    return 0;
  }
  return *(undefined4 *)(*param_1 + -8);
}


