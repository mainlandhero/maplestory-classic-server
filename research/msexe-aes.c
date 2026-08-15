
//===========================================================
// FUN_140c76070 @ 140c76070   (372 bytes)
//===========================================================

void FUN_140c76070(undefined4 *param_1,undefined4 *param_2)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  
  if (DAT_143ac38d0 == '\0') {
    FUN_140c759a0();
  }
  *param_2 = *param_1;
  uVar4 = 0;
  param_2[1] = param_1[4];
  param_2[2] = param_1[8];
  param_2[3] = param_1[0xc];
  param_2[4] = param_1[0x10];
  param_2[5] = param_1[0x14];
  param_2[6] = param_1[0x18];
  uVar2 = param_1[0x1c];
  param_2[7] = uVar2;
  puVar1 = &DAT_143ac38d8;
  do {
    uVar3 = uVar4 * 8;
    uVar4 = uVar4 + 1;
    uVar2 = (&DAT_143ac6900)[uVar2 & 0xff] ^ (&DAT_143ac6500)[uVar2 >> 0x18] ^
            (&DAT_143ac6100)[(uVar2 >> 8 & 0xff00) >> 8] ^ (&DAT_143ac5d00)[uVar2 >> 8 & 0xff] ^
            param_2[uVar3] ^ *puVar1;
    param_2[uVar3 + 8] = uVar2;
    uVar2 = uVar2 ^ param_2[uVar3 + 1];
    param_2[uVar3 + 9] = uVar2;
    uVar2 = uVar2 ^ param_2[uVar3 + 2];
    param_2[uVar3 + 10] = uVar2;
    uVar2 = uVar2 ^ param_2[uVar3 + 3];
    param_2[uVar3 + 0xb] = uVar2;
    uVar2 = (&DAT_143ac6900)[uVar2 >> 0x18] ^ (&DAT_143ac6500)[uVar2 >> 0x10 & 0xff] ^
            (&DAT_143ac6100)[uVar2 >> 8 & 0xff] ^ param_2[uVar3 + 4] ^
            (&DAT_143ac5d00)[uVar2 & 0xff];
    param_2[uVar3 + 0xc] = uVar2;
    uVar2 = uVar2 ^ param_2[uVar3 + 5];
    param_2[uVar3 + 0xd] = uVar2;
    uVar2 = uVar2 ^ param_2[uVar3 + 6];
    param_2[uVar3 + 0xe] = uVar2;
    uVar2 = uVar2 ^ param_2[uVar3 + 7];
    param_2[uVar3 + 0xf] = uVar2;
    puVar1 = puVar1 + 1;
  } while (uVar4 < 7);
  return;
}



//===========================================================
// FUN_140c78270 @ 140c78270   (443 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_140c78270(uint *param_1,uint *param_2,int param_3,uint *param_4,uint *param_5)

{
  uint *puVar1;
  uint uVar2;
  uint *puVar3;
  uint uVar4;
  ulonglong uVar5;
  undefined1 auStack_2798 [32];
  bool local_2778;
  uint local_2774;
  uint local_2768 [2500];
  ulonglong local_58;
  undefined8 uStack_48;
  
  uStack_48 = 0x140c78287;
  local_58 = DAT_143a8b908 ^ (ulonglong)auStack_2798;
  uVar4 = param_1[8];
  puVar3 = (uint *)0x0;
  local_2778 = false;
  local_2774 = param_3 + uVar4;
  *param_5 = local_2774;
  if (local_2774 < 0x11) {
    FUN_142ef7ba0((longlong)param_1 + (ulonglong)uVar4 + 0x10,param_2,(longlong)param_3);
    param_1[8] = param_1[8] + param_3;
    *param_5 = 0;
  }
  else {
    if (param_2 == param_4) {
      if (param_3 < 0x2711) {
        puVar3 = local_2768;
      }
      else {
        puVar3 = (uint *)_malloc_base((longlong)param_3);
      }
      local_2778 = param_3 >= 0x2711;
      FUN_142ef7ba0(puVar3,param_2,param_3);
      param_2 = puVar3;
    }
    *param_5 = local_2774;
    FUN_142ef7ba0((longlong)param_1 + (ulonglong)uVar4 + 0x10,param_2,(longlong)(int)(0x10 - uVar4))
    ;
    uVar2 = local_2774;
    param_2 = (uint *)((longlong)param_2 + (ulonglong)(0x10 - uVar4));
    uVar4 = local_2774 - 0x10;
    FUN_140c761f0(param_1 + 9,param_1);
    *param_4 = *param_1 ^ param_1[4];
    param_4[1] = param_1[5] ^ param_1[1];
    param_4[2] = param_1[6] ^ param_1[2];
    param_4[3] = param_1[7] ^ param_1[3];
    if (0x10 < uVar4) {
      uVar5 = (ulonglong)((uVar2 - 0x21 >> 4) + 1);
      do {
        FUN_140c761f0(param_1 + 9,param_1);
        uVar4 = uVar4 - 0x10;
        param_4[4] = *param_2 ^ *param_1;
        param_4[5] = param_2[1] ^ param_1[1];
        param_4[6] = param_2[2] ^ param_1[2];
        puVar1 = param_2 + 3;
        param_2 = param_2 + 4;
        param_4[7] = *puVar1 ^ param_1[3];
        uVar5 = uVar5 - 1;
        param_4 = param_4 + 4;
      } while (uVar5 != 0);
    }
    FUN_142ef7ba0(param_1 + 4,param_2,(longlong)(int)uVar4);
    param_1[8] = (param_1[8] & 0xf0000000) + uVar4;
    *param_5 = *param_5 - uVar4;
    if (local_2778 != false) {
      FUN_142f11bf4(puVar3);
    }
  }
  return 1;
}


