
//===========================================================
// FUN_140c75880 @ 140c75880   (282 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c75880(longlong param_1,undefined8 param_2,int param_3,undefined4 *param_4,int param_5)

{
  uint uVar1;
  byte *pbVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  undefined1 auStack_1b8 [32];
  uint *local_198;
  uint local_188 [4];
  undefined4 local_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  byte local_168 [16];
  uint local_158;
  undefined4 local_154;
  undefined1 local_150 [280];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_1b8;
  if (param_5 != 0) {
    FUN_140c755d0();
  }
  uVar4 = 0;
  local_154 = 8;
  local_188[0] = 0;
  FUN_140c76070(&DAT_143a86810,local_150);
  local_158 = 0;
  if (param_4 == (undefined4 *)0x0) {
    local_178 = _DAT_143307778;
    uStack_174 = _UNK_14330777c;
    uStack_170 = _UNK_143307780;
    uStack_16c = _UNK_143307784;
  }
  else {
    local_178 = *param_4;
    uStack_174 = local_178;
    uStack_170 = local_178;
    uStack_16c = local_178;
  }
  if (param_3 != 0) {
    local_198 = local_188;
    FUN_140c78270(&local_178,param_2,param_3,param_1);
    uVar4 = (ulonglong)local_188[0];
  }
  uVar1 = local_158;
  FUN_140c761f0(&local_154,&local_178);
  if (uVar1 != 0) {
    uVar3 = (ulonglong)uVar1;
    pbVar2 = (byte *)&local_178;
    do {
      pbVar2[(uVar4 + param_1) - (longlong)&local_178] = pbVar2[0x10] ^ *pbVar2;
      pbVar2 = pbVar2 + 1;
      uVar3 = uVar3 - 1;
    } while (uVar3 != 0);
  }
  return;
}



//===========================================================
// FUN_140c75760 @ 140c75760   (282 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c75760(longlong param_1,undefined8 param_2,int param_3,undefined4 *param_4,int param_5)

{
  uint uVar1;
  byte *pbVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  undefined1 auStack_1b8 [32];
  uint *local_198;
  uint local_188 [4];
  undefined4 local_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  byte local_168 [16];
  uint local_158;
  undefined4 local_154;
  undefined1 local_150 [280];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_1b8;
  if (param_5 != 0) {
    FUN_140c755d0();
  }
  uVar4 = 0;
  local_154 = 8;
  local_188[0] = 0;
  FUN_140c76070(&DAT_143a86810,local_150);
  local_158 = 0;
  if (param_4 == (undefined4 *)0x0) {
    local_178 = _DAT_143307778;
    uStack_174 = _UNK_14330777c;
    uStack_170 = _UNK_143307780;
    uStack_16c = _UNK_143307784;
  }
  else {
    local_178 = *param_4;
    uStack_174 = local_178;
    uStack_170 = local_178;
    uStack_16c = local_178;
  }
  if (param_3 != 0) {
    local_198 = local_188;
    FUN_140c78030(&local_178,param_2,param_3,param_1);
    uVar4 = (ulonglong)local_188[0];
  }
  uVar1 = local_158;
  FUN_140c761f0(&local_154,&local_178);
  if (uVar1 != 0) {
    uVar3 = (ulonglong)uVar1;
    pbVar2 = (byte *)&local_178;
    do {
      pbVar2[(uVar4 + param_1) - (longlong)&local_178] = pbVar2[0x10] ^ *pbVar2;
      pbVar2 = pbVar2 + 1;
      uVar3 = uVar3 - 1;
    } while (uVar3 != 0);
  }
  return;
}


