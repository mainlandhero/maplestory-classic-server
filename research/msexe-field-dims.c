
//===========================================================
// FUN_14187e880 @ 14187e880   (28 bytes)
//===========================================================

void FUN_14187e880(longlong param_1,undefined4 param_2,undefined4 param_3)

{
  *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x1ad8) = param_2;
  *(undefined4 *)(*(longlong *)(param_1 + 0xa8) + 0x1adc) = param_3;
  return;
}



//===========================================================
// FUN_142d16ef0 @ 142d16ef0   (118 bytes)
//===========================================================

void FUN_142d16ef0(longlong param_1)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  longlong *plVar4;
  
  plVar4 = (longlong *)**(longlong **)(param_1 + 0x3bd0);
  cVar1 = *(char *)((longlong)plVar4 + 0x19);
  while (cVar1 == '\0') {
    if ((int)plVar4[5] != 0) {
      plVar4[5] = 0;
    }
    plVar2 = (longlong *)plVar4[2];
    if (*(char *)((longlong)plVar2 + 0x19) == '\0') {
      cVar1 = *(char *)(*plVar2 + 0x19);
      plVar4 = plVar2;
      plVar2 = (longlong *)*plVar2;
      while (cVar1 == '\0') {
        cVar1 = *(char *)(*plVar2 + 0x19);
        plVar4 = plVar2;
        plVar2 = (longlong *)*plVar2;
      }
    }
    else {
      cVar1 = *(char *)(plVar4[1] + 0x19);
      plVar3 = (longlong *)plVar4[1];
      plVar2 = plVar4;
      while ((plVar4 = plVar3, cVar1 == '\0' && (plVar2 == (longlong *)plVar4[2]))) {
        cVar1 = *(char *)(plVar4[1] + 0x19);
        plVar3 = (longlong *)plVar4[1];
        plVar2 = plVar4;
      }
    }
    cVar1 = *(char *)((longlong)plVar4 + 0x19);
  }
  return;
}


