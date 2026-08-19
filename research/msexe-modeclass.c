
//===========================================================
// FUN_1401e7bf0 @ 1401e7bf0   (21 bytes)
//===========================================================

undefined1 FUN_1401e7bf0(int param_1)

{
  if (((param_1 != 3) && (param_1 != 4)) && (param_1 != 5)) {
    return 0;
  }
  return 1;
}



//===========================================================
// FUN_142c50390 @ 142c50390   (27 bytes)
//===========================================================

undefined8 FUN_142c50390(longlong param_1,undefined8 param_2)

{
  FUN_142c95ef0(*(undefined8 *)(param_1 + 8));
  return param_2;
}



//===========================================================
// FUN_1411284e0 @ 1411284e0   (7 bytes)
//===========================================================

void FUN_1411284e0(undefined4 param_1)

{
  DAT_143a871e4 = param_1;
  return;
}



//===========================================================
// FUN_141b3fd10 @ 141b3fd10   (27 bytes)
//===========================================================

bool FUN_141b3fd10(void)

{
  int iVar1;
  
  iVar1 = FUN_142c4a810(DAT_143ac1898);
  return iVar1 == 5;
}



//===========================================================
// FUN_142d3c670 @ 142d3c670   (183 bytes)
//===========================================================

void FUN_142d3c670(longlong param_1)

{
  int iVar1;
  longlong lVar2;
  undefined8 uVar3;
  
  lVar2 = *(longlong *)(param_1 + 0x29e0);
  if (lVar2 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar2 = *(longlong *)(param_1 + 0x29e0);
  }
  FUN_14252d560(lVar2);
  lVar2 = *(longlong *)(param_1 + 0x29f0);
  if (lVar2 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar2 = *(longlong *)(param_1 + 0x29f0);
  }
  FUN_1425fd4f0(lVar2);
  FUN_142c48ca0(DAT_143ac1898,0);
  lVar2 = FUN_14019b780(&DAT_143ad68a0,0x278);
  uVar3 = 0;
  if (lVar2 != 0) {
    uVar3 = FUN_141b219e0(lVar2);
  }
  FUN_14209ee50(uVar3,0);
  *(undefined1 *)(param_1 + 0x33f4) = 0;
  iVar1 = FUN_142c4a810(DAT_143ac1898);
  if (iVar1 == 2) {
    FUN_141d60f20();
  }
  FUN_142d0eda0(param_1);
  return;
}



//===========================================================
// FUN_142e156d0 @ 142e156d0   (319 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142e156d0(void)

{
  undefined8 uVar1;
  undefined4 local_res8 [8];
  
  uVar1 = FUN_142c49f00(DAT_143ac1898);
  (*DAT_143262178)(L"mscw_gllive",3);
  (*DAT_143262170)(3);
  if (DAT_143ade330 != '\0') {
    _DAT_143a8a400 = 0;
    (*DAT_143262498)(&LAB_142e4bde0);
  }
  set_unexpected(&LAB_142e14e70);
  set_terminate(&LAB_142e14e70);
  std::set_new_handler((_func_void *)&LAB_142e14df0);
  _set_invalid_parameter_handler((_invalid_parameter_handler)&LAB_142e14d80);
  FUN_142ef692c(&LAB_142e14da0);
  FUN_142f2abc0(1);
  _set_new_handler(FUN_142e14dc0);
  FUN_142f048b0(2);
  FUN_142f29f34(0x16,&LAB_142e14e10);
  FUN_142f29f34(2,&LAB_142e14e30);
  FUN_142f29f34(0xf,&LAB_142e14e50);
  (*DAT_143262158)(FUN_142e15340);
  (*DAT_143262148)();
  if (DAT_143ac1898 != 0) {
    local_res8[0] = FUN_142c4a810();
    (*DAT_143262128)(L"GAME_START_MODE",local_res8);
  }
  (*DAT_143262130)(uVar1);
  (*DAT_143262118)(1);
  (*_DAT_143262140)(1);
  return;
}


