
//===========================================================
// FUN_142e56b40 @ 142e56b40   (119 bytes)
//===========================================================

undefined8 * FUN_142e56b40(void)

{
  int iVar1;
  undefined8 *puVar2;
  undefined1 local_res8 [8];
  int local_res10 [6];
  
  iVar1 = (*DAT_143262778)(&DAT_143ae1518,0,local_res10,0);
  if (iVar1 != 0) {
    if (local_res10[0] != 0) {
      FUN_142e5ee50(local_res8);
      iVar1 = (*DAT_143262770)(&DAT_143ae1518,0,0);
      if (iVar1 == 0) goto LAB_142e56bb2;
    }
    puVar2 = &DAT_143a8b588;
    if (0xf < DAT_143a8b5a0) {
      puVar2 = DAT_143a8b588;
    }
    return puVar2;
  }
LAB_142e56bb2:
                    /* WARNING: Subroutine does not return */
  FUN_142f048cc();
}



//===========================================================
// FUN_142e56bc0 @ 142e56bc0   (119 bytes)
//===========================================================

undefined8 * FUN_142e56bc0(void)

{
  int iVar1;
  undefined8 *puVar2;
  undefined1 local_res8 [8];
  int local_res10 [6];
  
  iVar1 = (*DAT_143262778)(&DAT_143ae1520,0,local_res10,0);
  if (iVar1 != 0) {
    if (local_res10[0] != 0) {
      FUN_142e5e7c0(local_res8);
      iVar1 = (*DAT_143262770)(&DAT_143ae1520,0,0);
      if (iVar1 == 0) goto LAB_142e56c32;
    }
    puVar2 = &DAT_143a8b5a8;
    if (0xf < DAT_143a8b5c0) {
      puVar2 = DAT_143a8b5a8;
    }
    return puVar2;
  }
LAB_142e56c32:
                    /* WARNING: Subroutine does not return */
  FUN_142f048cc();
}



//===========================================================
// FUN_142c4ad20 @ 142c4ad20   (143 bytes)
//===========================================================

undefined1 * FUN_142c4ad20(void)

{
  char *pcVar1;
  uint uVar2;
  longlong lVar3;
  int iVar4;
  longlong lVar5;
  ulonglong uVar6;
  
  if (DAT_143ade000 == '\0') {
    uVar2 = (*DAT_143ad5618)(&DAT_143ade000,0x104);
    uVar6 = (ulonglong)uVar2;
    lVar3 = -1;
    do {
      lVar5 = lVar3 + 1;
      pcVar1 = &DAT_143ade001 + lVar3;
      lVar3 = lVar5;
    } while (*pcVar1 != '\0');
    iVar4 = (int)lVar5;
    if (0 < iVar4) {
      lVar3 = 0;
      do {
        if ((&DAT_143ade000)[lVar3] == '/') {
          (&DAT_143ade000)[lVar3] = 0x5c;
        }
        lVar3 = lVar3 + 1;
      } while (lVar3 < iVar4);
    }
    if ((uVar2 != 0) && ((&DAT_143ade000)[uVar2 - 1] != '\\')) {
      (&DAT_143ade000)[uVar6] = 0x5c;
      uVar6 = (ulonglong)(uVar2 + 1);
    }
    (*DAT_143ad5658)(&DAT_143ade000 + uVar6,PTR_s_msexcr_ini_143a45090);
    return &DAT_143ade000;
  }
  return &DAT_143ade000;
}



//===========================================================
// FUN_1415ddd10 @ 1415ddd10   (358 bytes)
//===========================================================

undefined8 FUN_1415ddd10(undefined8 param_1,undefined8 param_2,uint param_3)

{
  uint uVar1;
  undefined1 local_b4 [4];
  uint local_b0;
  uint local_ac;
  undefined4 local_a8;
  undefined1 local_a0 [8];
  undefined4 local_98;
  undefined8 local_90;
  undefined8 local_88;
  code *local_80;
  undefined8 local_78;
  undefined1 local_68 [104];
  
  local_b0 = 0;
  FUN_1415e1df0(local_a0);
  local_78 = FUN_1415e2410(local_68);
  thunk_FUN_1408e8c70(local_68,param_2,3,0x80,1,0x80000000,0,0);
  uVar1 = FUN_1401bd210(local_68);
  local_ac = uVar1;
  if ((uVar1 != 0) && (uVar1 < param_3)) {
    local_90 = FUN_140ca6ea0(local_b4,0);
    local_88 = thunk_FUN_1406f1840(local_a0,uVar1,local_90);
    FUN_1401bd3a0(local_68,local_88,uVar1);
  }
  FUN_1401bca50(local_68);
  local_80 = DAT_143ad55c0;
  local_a8 = (*DAT_143ad55c0)(param_2);
  local_98 = local_a8;
  FUN_1401bc130(local_68);
  FUN_1413f4860(param_1,local_a0);
  local_b0 = local_b0 | 1;
  FUN_14035a250(local_a0);
  return param_1;
}



//===========================================================
// FUN_1415e3fd0 @ 1415e3fd0   (21 bytes)
//===========================================================

bool FUN_1415e3fd0(longlong *param_1)

{
  int iVar1;
  
  iVar1 = 0;
  if (*param_1 != 0) {
    iVar1 = *(int *)(*param_1 + -8);
  }
  return iVar1 == 0;
}


