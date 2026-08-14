
//===========================================================
// FUN_141804740 @ 141804740   (187 bytes)
//===========================================================

undefined8 * FUN_141804740(int *param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  longlong local_res8;
  longlong local_res18 [2];
  
  if (*param_1 == 0x2200000c) {
    local_res8 = 0;
    uVar4 = *(undefined8 *)(param_1 + 8);
    uVar1 = *(undefined8 *)(param_1 + 6);
    uVar2 = *(undefined8 *)(param_1 + 4);
    puVar3 = (undefined8 *)FUN_1408a9e40(local_res18,0x73);
    uVar4 = FUN_14019ba10(&local_res8,*puVar3,uVar2,uVar1,uVar4);
    *param_2 = 0;
    FUN_14019a260(param_2,uVar4);
    if (local_res18[0] != 0) {
      FUN_14019f2c0(local_res18[0] + -0x10);
    }
    if (local_res8 != 0) {
      FUN_14019f2c0(local_res8 + -0x10);
    }
    return param_2;
  }
  FUN_141803cd0();
  return param_2;
}



//===========================================================
// FUN_141803cd0 @ 141803cd0   (627 bytes)
//===========================================================

longlong * FUN_141803cd0(int *param_1,longlong *param_2)

{
  int iVar1;
  longlong lVar2;
  undefined4 *puVar3;
  
  iVar1 = *param_1;
  if (iVar1 < 0x22000002) {
    if (iVar1 == 0x22000001) {
      FUN_1408a9e40(param_2,100);
      return param_2;
    }
    switch(iVar1) {
    case 0x21000001:
      FUN_1408a9e40(param_2,0x61);
      return param_2;
    case 0x21000002:
      FUN_1408a9e40(param_2,0x62);
      return param_2;
    case 0x21000003:
      FUN_1408a9e40(param_2,99);
      return param_2;
    case 0x21000004:
      FUN_1408a9e40(param_2,0x6d);
      return param_2;
    case 0x21000006:
      FUN_1408a9e40(param_2,0x6f);
      return param_2;
    case 0x21000007:
      FUN_1408a9e40(param_2,0x70);
      return param_2;
    }
  }
  else {
    switch(iVar1) {
    case 0x22000002:
    case 0x2200000f:
      FUN_1408a9e40(param_2,0x65);
      return param_2;
    case 0x22000003:
      FUN_1408a9e40(param_2,0x66);
      return param_2;
    case 0x22000004:
      FUN_1408a9e40(param_2,0x67);
      return param_2;
    case 0x22000005:
      FUN_1408a9e40(param_2,0x68);
      return param_2;
    case 0x22000006:
      FUN_1408a9e40(param_2,0x69);
      return param_2;
    case 0x22000007:
      FUN_1408a9e40(param_2,0x6a);
      return param_2;
    case 0x22000008:
      FUN_1408a9e40(param_2,0x6c);
      return param_2;
    case 0x2200000a:
      FUN_1408a9e40(param_2,0x6e);
      return param_2;
    case 0x2200000b:
      FUN_1408a9e40(param_2,0x72);
      return param_2;
    case 0x2200000e:
      FUN_1408a9e40(param_2,0x74);
      return param_2;
    }
  }
  *param_2 = 0;
  puVar3 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
  puVar3[1] = 0;
  *puVar3 = 0xffffffff;
  *param_2 = (longlong)(puVar3 + 4);
  puVar3[2] = 0;
  *(undefined1 *)*param_2 = 0;
  lVar2 = *param_2;
  if (*(int *)(lVar2 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)(lVar2 + -0xc) < 0) {
    FUN_142e54290(0x90,*(int *)(lVar2 + -0xc),0);
  }
  *(undefined4 *)(lVar2 + -0x10) = 1;
  *(undefined1 *)*param_2 = 0;
  if (*(int *)(lVar2 + -0xc) + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  *(undefined4 *)(lVar2 + -8) = 0;
  return param_2;
}



//===========================================================
// FUN_141803fa0 @ 141803fa0   (299 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined4 * FUN_141803fa0(undefined4 *param_1,undefined2 param_2,undefined4 param_3)

{
  undefined4 *puVar1;
  char cVar2;
  char *pcVar3;
  longlong lVar4;
  undefined1 auStack_148 [32];
  char local_128 [272];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_148;
  *param_1 = 0x20000000;
  *(undefined8 *)(param_1 + 2) = 0;
  if (DAT_143ac1898 != 0) {
    FUN_142c4ef20(DAT_143ac1898,3,0x20000000);
  }
  FUN_142ef8250(param_1 + 4,0,0x504);
  (*DAT_143ad5608)(0x104,local_128);
  (*DAT_143ad5658)(local_128,&DAT_14328d904);
  *(undefined2 *)((longlong)param_1 + 0x12) = param_2;
  *(undefined2 *)(param_1 + 4) = 1;
  param_1[5] = 0;
  param_1[6] = param_3;
  pcVar3 = (char *)(*DAT_143ad56b0)();
  puVar1 = param_1 + 0xc5;
  if (puVar1 != (undefined4 *)0x0) {
    if (pcVar3 == (char *)0x0) {
      *(undefined1 *)puVar1 = 0;
    }
    else {
      lVar4 = (longlong)puVar1 - (longlong)pcVar3;
      do {
        cVar2 = *pcVar3;
        pcVar3[lVar4] = cVar2;
        pcVar3 = pcVar3 + 1;
      } while (cVar2 != '\0');
    }
  }
  if (param_1 + 0x105 != (undefined4 *)0x0) {
    pcVar3 = local_128;
    do {
      cVar2 = *pcVar3;
      pcVar3[(longlong)(param_1 + 0x105) - (longlong)local_128] = cVar2;
      pcVar3 = pcVar3 + 1;
    } while (cVar2 != '\0');
  }
  return param_1;
}


