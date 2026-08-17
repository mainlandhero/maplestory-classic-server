
//===========================================================
// FUN_1415f1c50 @ 1415f1c50   (8 bytes)
//===========================================================

longlong FUN_1415f1c50(longlong param_1)

{
  return param_1 + 0x1c8;
}



//===========================================================
// FUN_1413f4690 @ 1413f4690   (298 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001413f46e8) */
/* WARNING: Removing unreachable block (ram,0x0001413f46f3) */
/* WARNING: Removing unreachable block (ram,0x0001413f46f6) */
/* WARNING: Removing unreachable block (ram,0x0001413f470a) */

void FUN_1413f4690(longlong *param_1)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  
  pcVar1 = (char *)*param_1;
  if (((((char)param_1[1] != '\0') || (pcVar1 == (char *)0x0)) || (*(int *)(pcVar1 + -8) != 1)) ||
     (*pcVar1 != '\0')) {
    lVar3 = 0;
    if (pcVar1 != (char *)0x0) {
      thunk_FUN_140205820(pcVar1 + -8,0);
      *param_1 = 0;
    }
    lVar2 = FUN_14019b780(&DAT_143ad68a0,9);
    if (lVar2 != 0) {
      lVar3 = lVar2 + 8;
    }
    if (*param_1 != 0) {
      FUN_142ef7ba0(lVar3,*param_1,0);
      thunk_FUN_140205820(*param_1 + -8,0);
    }
    *param_1 = lVar3;
    *(undefined8 *)(lVar3 + -8) = 0;
    *(longlong *)(*param_1 + -8) = *(longlong *)(*param_1 + -8) + 1;
    FUN_142ef7ba0(*param_1 + 1,*param_1,0);
    *(undefined1 *)*param_1 = 0;
    *(undefined1 *)(param_1 + 1) = 0;
    FUN_1415efc80(DAT_143ac87a0,0,PTR_DAT_143a45378,param_1);
    FUN_1415ef8a0(DAT_143ac87a0,0,PTR_s_HWID_MARKING_DAY_143a45380,(char)param_1[1]);
  }
  return;
}



//===========================================================
// FUN_141b0eb80 @ 141b0eb80   (32 bytes)
//===========================================================

void FUN_141b0eb80(undefined1 *param_1)

{
  longlong lVar1;
  undefined1 *puVar2;
  
  puVar2 = &DAT_143ad1ee0;
  for (lVar1 = 0x10; lVar1 != 0; lVar1 = lVar1 + -1) {
    *puVar2 = *param_1;
    param_1 = param_1 + 1;
    puVar2 = puVar2 + 1;
  }
  return;
}



//===========================================================
// FUN_1415db7e0 @ 1415db7e0   (8 bytes)
//===========================================================

void FUN_1415db7e0(void)

{
  DAT_143ace258 = 1;
  return;
}



//===========================================================
// FUN_142c4f6d0 @ 142c4f6d0   (7 bytes)
//===========================================================

undefined4 FUN_142c4f6d0(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x140);
}


