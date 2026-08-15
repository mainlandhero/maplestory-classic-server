
//===========================================================
// FUN_1415e56e0 @ 1415e56e0   (9 bytes)
//===========================================================

bool FUN_1415e56e0(longlong param_1)

{
  return *(longlong *)(param_1 + 0x20) == 0;
}



//===========================================================
// FUN_1415e5a40 @ 1415e5a40   (65 bytes)
//===========================================================

void FUN_1415e5a40(longlong param_1)

{
  longlong *plVar1;
  
  FUN_1415e54a0(*(undefined8 *)
                 (*(longlong *)(param_1 + 8) +
                 (*(longlong *)(param_1 + 0x10) - 1U & *(ulonglong *)(param_1 + 0x18)) * 8));
  plVar1 = (longlong *)(param_1 + 0x20);
  *plVar1 = *plVar1 + -1;
  if (*plVar1 == 0) {
    *(undefined8 *)(param_1 + 0x18) = 0;
    return;
  }
  *(longlong *)(param_1 + 0x18) = *(longlong *)(param_1 + 0x18) + 1;
  return;
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


