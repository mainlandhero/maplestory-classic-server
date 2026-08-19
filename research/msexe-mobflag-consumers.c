
//===========================================================
// FUN_14047a100 @ 14047a100   (62 bytes)
//===========================================================

longlong FUN_14047a100(longlong param_1)

{
  undefined4 uVar1;
  char cVar2;
  int iVar3;
  
  uVar1 = *(undefined4 *)(param_1 + 0x60);
  cVar2 = FUN_1404afb40(uVar1);
  if (cVar2 != '\0') {
    iVar3 = FUN_1404af330(uVar1);
    return (longlong)iVar3;
  }
  return *(longlong *)(param_1 + 0x20);
}



//===========================================================
// FUN_141c55ab0 @ 141c55ab0   (7 bytes)
//===========================================================

void FUN_141c55ab0(longlong param_1,undefined1 param_2)

{
  *(undefined1 *)(param_1 + 0x508) = param_2;
  return;
}


