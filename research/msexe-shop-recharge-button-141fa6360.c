
//===========================================================
// FUN_141fa6360 @ 141fa6360   (217 bytes)
//===========================================================

void FUN_141fa6360(longlong *param_1,uint param_2)

{
  int iVar1;
  longlong lVar2;
  
  if (param_2 < 0x7d1) {
    if (param_2 == 2000) {
switchD_141fa63c7_caseD_7d1:
      lVar2 = param_1[0x61];
      if (lVar2 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar2 = param_1[0x61];
      }
      iVar1 = *(int *)(lVar2 + 0x80);
      *(undefined4 *)(param_1 + 0x94) = 0xffffffff;
      *(uint *)(param_1 + 0x95) = iVar1 + -2000 + param_2;
      (**(code **)(*param_1 + 0x90))(param_1,0);
      FUN_141fb9240(param_1);
      return;
    }
    if (param_2 == 1000) {
      FUN_141fb55e0();
      return;
    }
    if (param_2 == 0x3e9) {
      FUN_141fb8ea0(param_1,0);
      return;
    }
  }
  else {
    switch(param_2) {
    case 0x7d1:
    case 0x7d2:
    case 0x7d3:
    case 0x7d4:
    case 0x7d5:
    case 0x7d6:
    case 0x7d7:
    case 0x7d8:
      goto switchD_141fa63c7_caseD_7d1;
    }
  }
  FUN_14177fb00(param_1,param_2);
  return;
}


