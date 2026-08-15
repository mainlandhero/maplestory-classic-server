
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



//===========================================================
// FUN_140c78030 @ 140c78030   (428 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_140c78030(uint *param_1,uint *param_2,int param_3,uint *param_4,uint *param_5)

{
  uint *puVar1;
  uint *puVar2;
  ulonglong uVar3;
  uint uVar4;
  uint *puVar5;
  undefined1 auStack_2798 [32];
  bool local_2778;
  uint local_2774;
  uint local_2768 [2500];
  ulonglong local_58;
  undefined8 uStack_48;
  
  uStack_48 = 0x140c78047;
  local_58 = DAT_143a8b908 ^ (ulonglong)auStack_2798;
  uVar4 = param_1[8];
  local_2778 = false;
  local_2774 = param_3 + uVar4;
  *param_5 = local_2774;
  if (local_2774 < 0x10) {
    FUN_142ef7ba0((longlong)param_1 + (ulonglong)uVar4 + 0x10,param_2,(longlong)param_3);
    param_1[8] = param_1[8] + param_3;
    *param_5 = 0;
  }
  else {
    puVar2 = param_2;
    puVar5 = (uint *)0x0;
    if (param_2 == param_4) {
      if (param_3 < 0x2711) {
        puVar2 = local_2768;
      }
      else {
        puVar2 = (uint *)_malloc_base((longlong)param_3);
      }
      local_2778 = param_3 >= 0x2711;
      FUN_142ef7ba0(puVar2,param_2,param_3);
      puVar5 = puVar2;
    }
    FUN_142ef7ba0((longlong)param_1 + (ulonglong)uVar4 + 0x10,puVar2,(longlong)(int)(0x10 - uVar4));
    puVar2 = (uint *)((longlong)puVar2 + (ulonglong)(0x10 - uVar4));
    uVar4 = local_2774 - 0x10;
    FUN_140c761f0(param_1 + 9,param_1);
    *param_4 = *param_1 ^ param_1[4];
    param_4[1] = param_1[5] ^ param_1[1];
    param_4[2] = param_1[6] ^ param_1[2];
    param_4[3] = param_1[7] ^ param_1[3];
    if (0xf < uVar4) {
      uVar3 = (ulonglong)(uVar4 >> 4);
      do {
        FUN_140c761f0(param_1 + 9,param_1);
        uVar4 = uVar4 - 0x10;
        param_4[4] = *puVar2 ^ *param_1;
        param_4[5] = puVar2[1] ^ param_1[1];
        param_4[6] = puVar2[2] ^ param_1[2];
        puVar1 = puVar2 + 3;
        puVar2 = puVar2 + 4;
        param_4[7] = *puVar1 ^ param_1[3];
        uVar3 = uVar3 - 1;
        param_4 = param_4 + 4;
      } while (uVar3 != 0);
    }
    FUN_142ef7ba0(param_1 + 4,puVar2,(longlong)(int)uVar4);
    param_1[8] = (param_1[8] & 0xf0000000) + uVar4;
    *param_5 = *param_5 - uVar4;
    if (local_2778 != false) {
      FUN_142f11bf4(puVar5);
    }
  }
  return 1;
}



//===========================================================
// FUN_140c761f0 @ 140c761f0   (3762 bytes)
//===========================================================

void FUN_140c761f0(longlong param_1,uint *param_2)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  uint uVar12;
  uint uVar13;
  uint uVar14;
  uint uVar15;
  uint uVar16;
  uint uVar17;
  uint uVar18;
  uint uVar19;
  
  uVar8 = *(uint *)(param_1 + 8) ^ param_2[1];
  uVar16 = *(uint *)(param_1 + 0x10) ^ param_2[3];
  uVar14 = *(uint *)(param_1 + 0xc) ^ param_2[2];
  uVar10 = *(uint *)(param_1 + 4) ^ *param_2;
  uVar18 = (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar16 >> 0x18] ^
           (&DAT_143ac4500)[(ulonglong)(uVar14 >> 0x10) & 0xff] ^ (&DAT_143ac3d00)[uVar10 & 0xff] ^
           *(uint *)(param_1 + 0x14);
  uVar11 = (&DAT_143ac4500)[uVar16 >> 0x10 & 0xff] ^ (&DAT_143ac4100)[uVar14 >> 8 & 0xff] ^
           (&DAT_143ac4900)[uVar10 >> 0x18] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
           *(uint *)(param_1 + 0x18);
  uVar12 = (&DAT_143ac4900)[uVar8 >> 0x18] ^ (&DAT_143ac4100)[uVar16 >> 8 & 0xff] ^
           (&DAT_143ac4500)[uVar10 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar14 & 0xff] ^
           *(uint *)(param_1 + 0x1c);
  uVar8 = (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^ (&DAT_143ac4900)[uVar14 >> 0x18] ^
          (&DAT_143ac4100)[uVar10 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar16 & 0xff] ^
          *(uint *)(param_1 + 0x20);
  uVar10 = (&DAT_143ac4900)[uVar8 >> 0x18] ^ (&DAT_143ac4500)[uVar12 >> 0x10 & 0xff] ^
           (&DAT_143ac4100)[uVar11 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar18 & 0xff] ^
           *(uint *)(param_1 + 0x24);
  uVar14 = (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^ (&DAT_143ac4100)[uVar12 >> 8 & 0xff] ^
           (&DAT_143ac4900)[uVar18 >> 0x18] ^ (&DAT_143ac3d00)[uVar11 & 0xff] ^
           *(uint *)(param_1 + 0x28);
  uVar15 = (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar11 >> 0x18] ^
           (&DAT_143ac4500)[uVar18 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar12 & 0xff] ^
           *(uint *)(param_1 + 0x2c);
  uVar8 = (&DAT_143ac4900)[uVar12 >> 0x18] ^ (&DAT_143ac4500)[uVar11 >> 0x10 & 0xff] ^
          (&DAT_143ac4100)[uVar18 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0x30);
  uVar11 = (&DAT_143ac4100)[uVar14 >> 8 & 0xff] ^
           (&DAT_143ac4500)[(ulonglong)(uVar15 >> 0x10) & 0xff] ^ (&DAT_143ac4900)[uVar8 >> 0x18] ^
           (&DAT_143ac3d00)[uVar10 & 0xff] ^ *(uint *)(param_1 + 0x34);
  uVar12 = (&DAT_143ac4100)[uVar15 >> 8 & 0xff] ^ (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^
           (&DAT_143ac4900)[uVar10 >> 0x18] ^ (&DAT_143ac3d00)[uVar14 & 0xff] ^
           *(uint *)(param_1 + 0x38);
  uVar16 = (&DAT_143ac4900)[uVar14 >> 0x18] ^ (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^
           (&DAT_143ac4500)[uVar10 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar15 & 0xff] ^
           *(uint *)(param_1 + 0x3c);
  uVar8 = (&DAT_143ac4500)[uVar14 >> 0x10 & 0xff] ^ (&DAT_143ac4900)[uVar15 >> 0x18] ^
          (&DAT_143ac4100)[uVar10 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0x40);
  uVar10 = (&DAT_143ac4100)[uVar12 >> 8 & 0xff] ^ (&DAT_143ac4500)[uVar16 >> 0x10 & 0xff] ^
           (&DAT_143ac4900)[uVar8 >> 0x18] ^ (&DAT_143ac3d00)[uVar11 & 0xff] ^
           *(uint *)(param_1 + 0x44);
  uVar14 = (&DAT_143ac4900)[uVar11 >> 0x18] ^ (&DAT_143ac4100)[uVar16 >> 8 & 0xff] ^
           (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[uVar12 & 0xff] ^
           *(uint *)(param_1 + 0x48);
  uVar15 = (&DAT_143ac4500)[uVar11 >> 0x10 & 0xff] ^ (&DAT_143ac4900)[uVar12 >> 0x18] ^
           (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar16 & 0xff] ^
           *(uint *)(param_1 + 0x4c);
  uVar8 = (&DAT_143ac4100)[uVar11 >> 8 & 0xff] ^ (&DAT_143ac4500)[uVar12 >> 0x10 & 0xff] ^
          (&DAT_143ac4900)[uVar16 >> 0x18] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0x50);
  uVar18 = (&DAT_143ac4100)[uVar14 >> 8 & 0xff] ^
           (&DAT_143ac4500)[(ulonglong)(uVar15 >> 0x10) & 0xff] ^ (&DAT_143ac4900)[uVar8 >> 0x18] ^
           (&DAT_143ac3d00)[uVar10 & 0xff] ^ *(uint *)(param_1 + 0x54);
  uVar12 = (&DAT_143ac4100)[uVar15 >> 8 & 0xff] ^ (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^
           (&DAT_143ac4900)[uVar10 >> 0x18] ^ (&DAT_143ac3d00)[uVar14 & 0xff] ^
           *(uint *)(param_1 + 0x58);
  uVar16 = (&DAT_143ac4900)[uVar14 >> 0x18] ^ (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^
           (&DAT_143ac4500)[uVar10 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar15 & 0xff] ^
           *(uint *)(param_1 + 0x5c);
  uVar8 = (&DAT_143ac4500)[uVar14 >> 0x10 & 0xff] ^ (&DAT_143ac4900)[uVar15 >> 0x18] ^
          (&DAT_143ac4100)[uVar10 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0x60);
  uVar11 = (&DAT_143ac4900)[uVar8 >> 0x18] ^ (&DAT_143ac4500)[uVar16 >> 0x10 & 0xff] ^
           (&DAT_143ac4100)[uVar12 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar18 & 0xff] ^
           *(uint *)(param_1 + 100);
  uVar14 = (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^ (&DAT_143ac4100)[uVar16 >> 8 & 0xff] ^
           (&DAT_143ac4900)[uVar18 >> 0x18] ^ (&DAT_143ac3d00)[uVar12 & 0xff] ^
           *(uint *)(param_1 + 0x68);
  uVar10 = (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar12 >> 0x18] ^
           (&DAT_143ac4500)[uVar18 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar16 & 0xff] ^
           *(uint *)(param_1 + 0x6c);
  uVar8 = (&DAT_143ac4900)[uVar16 >> 0x18] ^ (&DAT_143ac4500)[uVar12 >> 0x10 & 0xff] ^
          (&DAT_143ac4100)[uVar18 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0x70);
  uVar15 = (&DAT_143ac4100)[uVar14 >> 8 & 0xff] ^
           (&DAT_143ac4500)[(ulonglong)(uVar10 >> 0x10) & 0xff] ^ (&DAT_143ac4900)[uVar8 >> 0x18] ^
           (&DAT_143ac3d00)[uVar11 & 0xff] ^ *(uint *)(param_1 + 0x74);
  uVar12 = (&DAT_143ac4900)[uVar11 >> 0x18] ^ (&DAT_143ac4100)[uVar10 >> 8 & 0xff] ^
           (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[uVar14 & 0xff] ^
           *(uint *)(param_1 + 0x78);
  uVar16 = (&DAT_143ac4500)[uVar11 >> 0x10 & 0xff] ^ (&DAT_143ac4900)[uVar14 >> 0x18] ^
           (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar10 & 0xff] ^
           *(uint *)(param_1 + 0x7c);
  uVar8 = (&DAT_143ac4100)[uVar11 >> 8 & 0xff] ^ (&DAT_143ac4500)[uVar14 >> 0x10 & 0xff] ^
          (&DAT_143ac4900)[uVar10 >> 0x18] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0x80);
  uVar11 = (&DAT_143ac4900)[uVar8 >> 0x18] ^ (&DAT_143ac4500)[(ulonglong)(uVar16 >> 0x10) & 0xff] ^
           (&DAT_143ac4100)[uVar12 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar15 & 0xff] ^
           *(uint *)(param_1 + 0x84);
  uVar14 = (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^ (&DAT_143ac4100)[uVar16 >> 8 & 0xff] ^
           (&DAT_143ac4900)[uVar15 >> 0x18] ^ (&DAT_143ac3d00)[uVar12 & 0xff] ^
           *(uint *)(param_1 + 0x88);
  uVar10 = (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar12 >> 0x18] ^
           (&DAT_143ac4500)[uVar15 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar16 & 0xff] ^
           *(uint *)(param_1 + 0x8c);
  uVar8 = (&DAT_143ac4900)[uVar16 >> 0x18] ^ (&DAT_143ac4500)[uVar12 >> 0x10 & 0xff] ^
          (&DAT_143ac4100)[uVar15 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0x90);
  uVar12 = (&DAT_143ac4100)[uVar14 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar8 >> 0x18] ^
           (&DAT_143ac4500)[uVar10 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[uVar11 & 0xff] ^
           *(uint *)(param_1 + 0x94);
  uVar16 = (&DAT_143ac4900)[uVar11 >> 0x18] ^ (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^
           (&DAT_143ac4100)[(ulonglong)(uVar10 >> 8) & 0xff] ^ (&DAT_143ac3d00)[uVar14 & 0xff] ^
           *(uint *)(param_1 + 0x98);
  uVar15 = (&DAT_143ac4500)[uVar11 >> 0x10 & 0xff] ^ (&DAT_143ac4900)[uVar14 >> 0x18] ^
           (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar10 & 0xff] ^
           *(uint *)(param_1 + 0x9c);
  uVar8 = (&DAT_143ac4100)[uVar11 >> 8 & 0xff] ^ (&DAT_143ac4500)[uVar14 >> 0x10 & 0xff] ^
          (&DAT_143ac4900)[uVar10 >> 0x18] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0xa0);
  uVar11 = (&DAT_143ac4900)[uVar8 >> 0x18] ^ (&DAT_143ac4500)[(ulonglong)(uVar15 >> 0x10) & 0xff] ^
           (&DAT_143ac4100)[uVar16 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar12 & 0xff] ^
           *(uint *)(param_1 + 0xa4);
  uVar14 = (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^ (&DAT_143ac4100)[uVar15 >> 8 & 0xff] ^
           (&DAT_143ac4900)[uVar12 >> 0x18] ^ (&DAT_143ac3d00)[uVar16 & 0xff] ^
           *(uint *)(param_1 + 0xa8);
  uVar10 = (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar16 >> 0x18] ^
           (&DAT_143ac4500)[uVar12 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar15 & 0xff] ^
           *(uint *)(param_1 + 0xac);
  uVar8 = (&DAT_143ac4900)[uVar15 >> 0x18] ^ (&DAT_143ac4500)[uVar16 >> 0x10 & 0xff] ^
          (&DAT_143ac4100)[uVar12 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0xb0);
  uVar15 = (&DAT_143ac4100)[uVar14 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar8 >> 0x18] ^
           (&DAT_143ac4500)[uVar10 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[uVar11 & 0xff] ^
           *(uint *)(param_1 + 0xb4);
  uVar12 = (&DAT_143ac4900)[uVar11 >> 0x18] ^ (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^
           (&DAT_143ac4100)[(ulonglong)(uVar10 >> 8) & 0xff] ^ (&DAT_143ac3d00)[uVar14 & 0xff] ^
           *(uint *)(param_1 + 0xb8);
  uVar16 = (&DAT_143ac4500)[uVar11 >> 0x10 & 0xff] ^ (&DAT_143ac4900)[uVar14 >> 0x18] ^
           (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar10 & 0xff] ^
           *(uint *)(param_1 + 0xbc);
  uVar8 = (&DAT_143ac4100)[uVar11 >> 8 & 0xff] ^ (&DAT_143ac4500)[uVar14 >> 0x10 & 0xff] ^
          (&DAT_143ac4900)[uVar10 >> 0x18] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0xc0);
  uVar11 = (&DAT_143ac4900)[uVar8 >> 0x18] ^ (&DAT_143ac4500)[(ulonglong)(uVar16 >> 0x10) & 0xff] ^
           (&DAT_143ac4100)[uVar12 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar15 & 0xff] ^
           *(uint *)(param_1 + 0xc4);
  uVar14 = (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^ (&DAT_143ac4100)[uVar16 >> 8 & 0xff] ^
           (&DAT_143ac4900)[uVar15 >> 0x18] ^ (&DAT_143ac3d00)[uVar12 & 0xff] ^
           *(uint *)(param_1 + 200);
  uVar10 = (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar12 >> 0x18] ^
           (&DAT_143ac4500)[uVar15 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar16 & 0xff] ^
           *(uint *)(param_1 + 0xcc);
  uVar8 = (&DAT_143ac4900)[uVar16 >> 0x18] ^ (&DAT_143ac4500)[uVar12 >> 0x10 & 0xff] ^
          (&DAT_143ac4100)[uVar15 >> 8 & 0xff] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0xd0);
  uVar19 = (&DAT_143ac4100)[uVar14 >> 8 & 0xff] ^ (&DAT_143ac4900)[uVar8 >> 0x18] ^
           (&DAT_143ac4500)[uVar10 >> 0x10 & 0xff] ^ (&DAT_143ac3d00)[uVar11 & 0xff] ^
           *(uint *)(param_1 + 0xd4);
  uVar13 = (&DAT_143ac4900)[uVar11 >> 0x18] ^ (&DAT_143ac4500)[uVar8 >> 0x10 & 0xff] ^
           (&DAT_143ac4100)[(ulonglong)(uVar10 >> 8) & 0xff] ^ (&DAT_143ac3d00)[uVar14 & 0xff] ^
           *(uint *)(param_1 + 0xd8);
  uVar17 = (&DAT_143ac4500)[uVar11 >> 0x10 & 0xff] ^ (&DAT_143ac4900)[uVar14 >> 0x18] ^
           (&DAT_143ac4100)[uVar8 >> 8 & 0xff] ^ (&DAT_143ac3d00)[(ulonglong)uVar10 & 0xff] ^
           *(uint *)(param_1 + 0xdc);
  uVar9 = (&DAT_143ac4100)[uVar11 >> 8 & 0xff] ^ (&DAT_143ac4500)[uVar14 >> 0x10 & 0xff] ^
          (&DAT_143ac4900)[uVar10 >> 0x18] ^ (&DAT_143ac3d00)[uVar8 & 0xff] ^
          *(uint *)(param_1 + 0xe0);
  uVar8 = (&DAT_143ac6900)[uVar9 >> 0x18];
  uVar10 = (&DAT_143ac6500)[(ulonglong)(uVar17 >> 0x10) & 0xff];
  uVar11 = (&DAT_143ac6100)[uVar13 >> 8 & 0xff];
  uVar12 = (&DAT_143ac5d00)[uVar19 & 0xff];
  uVar14 = *(uint *)(param_1 + 0xe4);
  uVar16 = (&DAT_143ac6500)[uVar9 >> 0x10 & 0xff];
  uVar15 = (&DAT_143ac6100)[uVar17 >> 8 & 0xff];
  uVar18 = (&DAT_143ac6900)[uVar19 >> 0x18];
  uVar1 = (&DAT_143ac5d00)[uVar13 & 0xff];
  uVar2 = *(uint *)(param_1 + 0xe8);
  uVar3 = (&DAT_143ac6100)[uVar9 >> 8 & 0xff];
  uVar4 = (&DAT_143ac6900)[uVar13 >> 0x18];
  uVar5 = (&DAT_143ac6500)[uVar19 >> 0x10 & 0xff];
  uVar6 = (&DAT_143ac5d00)[(ulonglong)uVar17 & 0xff];
  uVar7 = *(uint *)(param_1 + 0xec);
  param_2[3] = (&DAT_143ac6900)[uVar17 >> 0x18] ^ (&DAT_143ac6500)[uVar13 >> 0x10 & 0xff] ^
               (&DAT_143ac6100)[uVar19 >> 8 & 0xff] ^ (&DAT_143ac5d00)[uVar9 & 0xff] ^
               *(uint *)(param_1 + 0xf0);
  *param_2 = uVar8 ^ uVar10 ^ uVar11 ^ uVar12 ^ uVar14;
  param_2[1] = uVar16 ^ uVar15 ^ uVar18 ^ uVar1 ^ uVar2;
  param_2[2] = uVar3 ^ uVar4 ^ uVar5 ^ uVar6 ^ uVar7;
  return;
}



//===========================================================
// FUN_140c78630 @ 140c78630   (149 bytes)
//===========================================================

uint FUN_140c78630(longlong param_1,int param_2,uint *param_3)

{
  byte bVar1;
  uint uVar2;
  uint *puVar3;
  longlong lVar4;
  uint local_res10 [2];
  
  local_res10[0] = 0xc65053f2;
  puVar3 = local_res10;
  if (param_3 != (uint *)0x0) {
    puVar3 = param_3;
  }
  if (0 < param_2) {
    lVar4 = 0;
    do {
      bVar1 = *(byte *)(lVar4 + param_1);
      lVar4 = lVar4 + 1;
      *(byte *)puVar3 = (char)*puVar3 + ((&DAT_143a86990)[*(byte *)((longlong)puVar3 + 1)] - bVar1);
      *(byte *)((longlong)puVar3 + 1) =
           *(byte *)((longlong)puVar3 + 1) -
           (*(byte *)((longlong)puVar3 + 2) ^ (&DAT_143a86990)[bVar1]);
      *(byte *)((longlong)puVar3 + 2) =
           *(byte *)((longlong)puVar3 + 2) ^
           bVar1 + (&DAT_143a86990)[*(byte *)((longlong)puVar3 + 3)];
      *(byte *)((longlong)puVar3 + 3) =
           (*(byte *)((longlong)puVar3 + 3) - (char)*puVar3) + (&DAT_143a86990)[bVar1];
      uVar2 = *puVar3 << 3 | *puVar3 >> 0x1d;
      *puVar3 = uVar2;
    } while (lVar4 < param_2);
    return uVar2;
  }
  return *puVar3;
}


