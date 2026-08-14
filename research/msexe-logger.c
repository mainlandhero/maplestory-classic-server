
//===========================================================
// FUN_14019cfe0 @ 14019cfe0   (867 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019cfe0(undefined1 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  code *pcVar1;
  char cVar2;
  uint uVar3;
  undefined4 uVar4;
  int iVar5;
  longlong lVar6;
  ulonglong *puVar7;
  char *pcVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  undefined1 *puVar11;
  undefined8 local_res10;
  undefined8 local_res18;
  undefined8 local_res20;
  undefined1 auStack_10e8 [32];
  undefined8 local_10c8;
  undefined8 *local_10c0;
  undefined8 local_10b8;
  uint local_10b0;
  uint local_10a8 [2];
  undefined2 local_10a0;
  undefined2 local_109e;
  ushort local_109a;
  ushort local_1098;
  ushort local_1096;
  ushort local_1094;
  undefined1 local_1088 [63];
  char cStack_1049;
  undefined8 local_1048;
  undefined1 local_48;
  ulonglong local_38;
  undefined8 uStack_30;
  
  uStack_30 = 0x14019d00c;
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_10e8;
  local_res10 = param_2;
  local_res18 = param_3;
  local_res20 = param_4;
  cVar2 = FUN_140933f30();
  if (cVar2 != '\0') {
    return;
  }
  if ((*(int *)(*(longlong *)((longlong)ThreadLocalStoragePointer + (ulonglong)DAT_143ae2c08 * 8) +
               4) < DAT_143aa82e8) && (_Init_thread_header(&DAT_143aa82e8), DAT_143aa82e8 == -1)) {
    uVar3 = (*DAT_143ad5410)();
    _DAT_143aa82d0 = (ulonglong)uVar3;
    _DAT_143aa82d8 = 0;
    _DAT_143aa82e0 = (undefined8 *)0x0;
    _Init_thread_footer(&DAT_143aa82e8);
  }
  uVar9 = 0xffffffffffffffff;
  if (((DAT_143aa82c8 == 0) && (DAT_143aa82c0 == 0)) &&
     (DAT_143aa82c8 = (*DAT_143ad5928)(0,"ZtlLog"), DAT_143aa82c8 == 0)) {
    DAT_143aa82c8 = -1;
    uVar3 = (*DAT_143ad53f8)(0,&local_1048,0x1001);
    pcVar8 = &cStack_1049 + uVar3;
    if (&local_1048 <= pcVar8) {
      do {
        if (((byte)(*pcVar8 - 0x2eU) < 2) || (*pcVar8 == '\\')) break;
        pcVar8 = pcVar8 + -1;
      } while (&local_1048 <= pcVar8);
    }
    (*DAT_143ad5708)(pcVar8 + 1,&DAT_143271e9c);
    local_10b8 = 0;
    local_10c0 = (undefined8 *)CONCAT44(local_10c0._4_4_,0x80000080);
    local_10c8 = CONCAT44(local_10c8._4_4_,4);
    DAT_143aa82c0 = (*DAT_143ad5588)(&local_1048,0x40000000,3);
    if (DAT_143aa82c0 != -1) {
      (*DAT_143ad55b0)(DAT_143aa82c0,0,0,2);
      pcVar1 = DAT_143ad5708;
      uVar4 = (*DAT_143ad5410)();
      (*pcVar1)(local_1088,"ZtlLog%08lx",uVar4);
      lVar6 = (*DAT_143262830)(0,0,local_1088);
      if ((lVar6 != 0) && (iVar5 = (*DAT_143ad5628)(), iVar5 != 0xb7)) {
        (*DAT_143ad5640)(&local_10a0);
        local_10b0 = (uint)local_1094;
        local_10b8 = CONCAT44(local_10b8._4_4_,(uint)local_1096);
        local_10c0 = (undefined8 *)CONCAT44(local_10c0._4_4_,(uint)local_1098);
        local_10c8 = CONCAT44(local_10c8._4_4_,(uint)local_109a);
        uVar4 = (*DAT_143ad5708)(&local_1048,
                                 "\r\n\r\nLOG %d/%02d/%02d %02d:%02d:%02d ------------------------\r\n\r\n"
                                 ,local_10a0,local_109e);
        local_10c8 = 0;
        (*DAT_143ad5598)(DAT_143aa82c0,&local_1048,uVar4,local_10a8);
      }
      goto LAB_14019d1f2;
    }
  }
  else {
LAB_14019d1f2:
    if (DAT_143aa82c0 != -1) goto LAB_14019d208;
  }
  if (DAT_143aa82c8 == -1) {
    return;
  }
LAB_14019d208:
  puVar11 = &DAT_1434b2af1;
  if (param_1 != (undefined1 *)0x0) {
    puVar11 = param_1;
  }
  puVar7 = (ulonglong *)FUN_14012c130();
  local_10c8 = 0;
  local_10c0 = &local_res10;
  uVar3 = FUN_142f0f514(*puVar7 | 1,&local_1048,0x1000,puVar11);
  if ((int)uVar3 < 0) {
    uVar3 = 0xffffffff;
  }
  if (((int)uVar3 < 0) || (0xfff < uVar3)) {
    local_48 = 0;
  }
  do {
    uVar10 = uVar9 + 1;
    lVar6 = uVar9 + 1;
    uVar9 = uVar10;
  } while (*(char *)((longlong)&local_1048 + lVar6) != '\0');
  uVar3 = (uint)uVar10;
  if (DAT_143aa82c8 == -1) {
    while (0 < (int)uVar3) {
      local_10c8 = 0;
      iVar5 = (*DAT_143ad5598)(DAT_143aa82c0,&local_1048,uVar10 & 0xffffffff,local_10a8);
      if (iVar5 == 0) {
        return;
      }
      local_1048 = local_1048 + (ulonglong)local_10a8[0];
      uVar3 = (int)uVar10 - local_10a8[0];
      uVar10 = (ulonglong)uVar3;
    }
  }
  else {
    _DAT_143aa82d8 = CONCAT44(DAT_143aa82d8_4,uVar3);
    _DAT_143aa82e0 = &local_1048;
    (*DAT_143ad57c0)(DAT_143aa82c8,0x4a,0,&DAT_143aa82d0);
  }
  return;
}



//===========================================================
// FUN_142e54720 @ 142e54720   (96 bytes)
//===========================================================

undefined1 FUN_142e54720(int param_1,undefined1 param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  
  cVar1 = *(char *)((longlong)DAT_143ae15c8[1] + 0x19);
  puVar3 = (undefined8 *)DAT_143ae15c8[1];
  puVar2 = DAT_143ae15c8;
  while (puVar4 = puVar3, cVar1 == '\0') {
    if (*(int *)((longlong)puVar4 + 0x1c) < param_1) {
      puVar3 = (undefined8 *)puVar4[2];
      puVar4 = puVar2;
    }
    else {
      puVar3 = (undefined8 *)*puVar4;
    }
    cVar1 = *(char *)((longlong)puVar3 + 0x19);
    puVar2 = puVar4;
  }
  if (((*(char *)((longlong)puVar2 + 0x19) == '\0') &&
      (*(int *)((longlong)puVar2 + 0x1c) <= param_1)) && (puVar2 != DAT_143ae15c8)) {
    puVar3 = puVar2 + 4;
    if (puVar2 == (undefined8 *)0xffffffffffffffe4) {
      puVar3 = (undefined8 *)0x0;
    }
    if (puVar3 != (undefined8 *)0x0) {
      return *(undefined1 *)puVar3;
    }
  }
  return param_2;
}


