
//===========================================================
// FUN_142e9e9e0 @ 142e9e9e0   (114 bytes)
//===========================================================

void FUN_142e9e9e0(longlong *param_1,char param_2)

{
  if ((*(int *)((longlong)param_1 + 0x34) != 0) && (param_2 == '\x01')) {
    FUN_142e9e080(*param_1);
  }
  if ((char)param_1[1] != '\0') {
    FUN_142e9e080(param_1 + 1,0x20);
  }
  if (param_1[5] != 0) {
    FUN_14019b4e0();
    *param_1 = (longlong)(param_1 + 1);
    param_1[5] = 0;
    *(undefined4 *)(param_1 + 6) = 0;
    *(undefined4 *)(param_1 + 7) = 0;
    return;
  }
  *(undefined4 *)(param_1 + 7) = 0;
  return;
}


