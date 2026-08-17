
//===========================================================
// FUN_1415e3b60 @ 1415e3b60   (37 bytes)
//===========================================================

void FUN_1415e3b60(longlong *param_1)

{
  if (*param_1 != -1) {
    (*DAT_143262e38)();
    *param_1 = -1;
  }
  return;
}



//===========================================================
// FUN_1415d35f0 @ 1415d35f0   (113 bytes)
//===========================================================

void FUN_1415d35f0(longlong param_1)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_1415e4080(param_1);
  if (cVar1 != '\0') {
    uVar2 = FUN_140caa510();
    FUN_142cf4350(uVar2);
  }
  FUN_1415d5aa0(param_1);
  if (*(int *)(param_1 + 0xc) == 4) {
    FUN_140c91e30();
  }
  *(undefined4 *)(param_1 + 0xc) = 0;
  *(undefined1 *)(param_1 + 0x10) = 0;
  FUN_1415e3b60(param_1 + 0x20);
  FUN_1415db820();
  return;
}



//===========================================================
// FUN_142c44350 @ 142c44350   (828 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142c44350(undefined8 *param_1)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined1 auStack_58 [32];
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_58;
  *param_1 = &PTR_FUN_1434923c0;
  if (param_1[0x32] != 0) {
    FUN_142e60550();
  }
  FUN_142e15840();
  FUN_142c46b80(param_1);
  local_38 = 8;
  local_34 = *(undefined4 *)(param_1 + 0x17);
  (*DAT_143262b98)(0x3b,8,&local_38);
  local_30 = 0x18;
  local_2c = *(undefined4 *)((longlong)param_1 + 0xbc);
  local_28 = 0x7fffffff;
  local_24 = 0x7fffffff;
  local_20 = 0x7fffffff;
  local_1c = 0x7fffffff;
  (*DAT_143262b98)(0x33,0x18,&local_30);
  puVar1 = (undefined8 *)param_1[0x1a];
  if (puVar1 != (undefined8 *)0x0) {
    (**(code **)*puVar1)(puVar1,1);
  }
  plVar2 = (longlong *)param_1[0x1b];
  if (plVar2 != (longlong *)0x0) {
    if (plVar2[6] != 0) {
      FUN_14019f2c0(plVar2[6] + -0x10);
    }
    if (plVar2[5] != 0) {
      FUN_14019f2c0(plVar2[5] + -0x10);
    }
    if (plVar2[3] != 0) {
      FUN_14019f2c0(plVar2[3] + -0x10);
    }
    if (plVar2[2] != 0) {
      FUN_14019f2c0(plVar2[2] + -0x10);
    }
    if (plVar2[1] != 0) {
      FUN_14019f2c0(plVar2[1] + -0x10);
    }
    if (*plVar2 != 0) {
      FUN_14019f2c0(*plVar2 + -0x10);
    }
    thunk_FUN_140205820(plVar2,0x38);
  }
  lVar3 = param_1[0x36];
  if (lVar3 != 0) {
    FUN_142e0ebb0(lVar3);
    thunk_FUN_140205820(lVar3,8);
  }
  if (param_1[0x38] != 0) {
    FUN_14019f2c0(param_1[0x38] + -0x10);
  }
  if (param_1[0x37] != 0) {
    FUN_14019f2c0(param_1[0x37] + -0x10);
  }
  if ((longlong *)param_1[0x35] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x35] + 0x10))();
  }
  FUN_142c88440(param_1 + 0x31);
  puVar1 = (undefined8 *)param_1[0x2f];
  *(undefined8 *)puVar1[1] = 0;
  puVar1 = (undefined8 *)*puVar1;
  while (puVar1 != (undefined8 *)0x0) {
    puVar4 = (undefined8 *)*puVar1;
    thunk_FUN_140205820(puVar1,0x38);
    puVar1 = puVar4;
  }
  thunk_FUN_140205820(param_1[0x2f],0x38);
  puVar1 = (undefined8 *)param_1[0x2d];
  *(undefined8 *)puVar1[1] = 0;
  puVar1 = (undefined8 *)*puVar1;
  while (puVar1 != (undefined8 *)0x0) {
    puVar4 = (undefined8 *)*puVar1;
    thunk_FUN_140205820(puVar1,0x28);
    puVar1 = puVar4;
  }
  thunk_FUN_140205820(param_1[0x2d],0x28);
  puVar1 = (undefined8 *)param_1[0x2b];
  *(undefined8 *)puVar1[1] = 0;
  puVar1 = (undefined8 *)*puVar1;
  while (puVar1 != (undefined8 *)0x0) {
    puVar4 = (undefined8 *)*puVar1;
    thunk_FUN_140205820(puVar1,0x20);
    puVar1 = puVar4;
  }
  thunk_FUN_140205820(param_1[0x2b],0x20);
  if (param_1[0x29] != 0) {
    FUN_14019f2c0(param_1[0x29] + -0x10);
  }
  if ((longlong *)param_1[0x27] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0x27] + 0x10))();
  }
  FUN_142c8c510(param_1 + 0x24);
  FUN_142c8c470(param_1 + 0x22);
  FUN_1402bf500(param_1 + 0x1e);
  if (param_1[0x1c] != 0) {
    thunk_FUN_140205820(param_1[0x1c] + -8,0);
    param_1[0x1c] = 0;
  }
  FUN_142c88010(param_1 + 0xf,param_1 + 0xf,*(undefined8 *)(param_1[0xf] + 8));
  thunk_FUN_140205820(param_1[0xf],0x40);
  DAT_143ac1898 = 0;
  return;
}


