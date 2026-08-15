
//===========================================================
// FUN_140c759a0 @ 140c759a0   (919 bytes)
//===========================================================

void FUN_140c759a0(void)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  uint *puVar11;
  longlong lVar12;
  uint uVar13;
  longlong lVar14;
  uint uVar15;
  uint uVar16;
  char *pcVar17;
  uint uVar18;
  uint uVar19;
  uint uVar20;
  
  uVar15 = 1;
  DAT_143ac3907 = 0;
  uVar6 = 1;
  uVar16 = 0;
  lVar12 = 0x100;
  pcVar17 = &DAT_143ac3a00;
  do {
    *pcVar17 = (char)uVar6;
    (&DAT_143ac3900)[uVar6] = (char)uVar16;
    uVar16 = uVar16 + 1;
    uVar6 = -(uint)((uVar6 & 0x80) != 0) & 0x1b ^ (uint)(byte)((char)uVar6 * '\x02') ^ uVar6;
    pcVar17 = pcVar17 + 1;
  } while (uVar16 < 0x100);
  DAT_143ac3901 = 0;
  puVar11 = &DAT_143ac38d8;
  lVar14 = 10;
  do {
    *puVar11 = uVar15;
    puVar11 = puVar11 + 1;
    uVar15 = -(uint)((uVar15 & 0x80) != 0) & 0x1b ^ (uint)(byte)((char)uVar15 * '\x02');
    lVar14 = lVar14 + -1;
  } while (lVar14 != 0);
  uVar6 = 0;
  do {
    if (uVar6 == 0) {
      bVar5 = 0;
    }
    else {
      bVar5 = (&DAT_143ac3aff)[-(ulonglong)(byte)(&DAT_143ac3900)[uVar6]];
    }
    bVar1 = bVar5 * '\x02' | bVar5 >> 7;
    bVar2 = bVar1 * '\x02';
    bVar3 = bVar2 | (bVar5 & 0x7f) >> 6;
    bVar4 = bVar3 * '\x02' | (bVar5 * '\x02' & 0x7f) >> 6;
    uVar15 = (byte)((bVar4 * '\x02' | (bVar2 & 0x7f) >> 6) ^ bVar4 ^ bVar5 ^ bVar1 ^ bVar3) ^ 99;
    (&DAT_143ac3b00)[uVar6] = (char)uVar15;
    (&DAT_143ac3c00)[uVar15] = (char)uVar6;
    uVar6 = uVar6 + 1;
  } while (uVar6 < 0x100);
  uVar19 = (uint)DAT_143ac390e;
  lVar14 = 0;
  uVar20 = (uint)DAT_143ac390d;
  uVar16 = (uint)DAT_143ac390b;
  uVar18 = (uint)DAT_143ac3909;
  uVar15 = (uint)DAT_143ac3903;
  uVar6 = (uint)DAT_143ac3902;
  do {
    bVar5 = (&DAT_143ac3b00)[lVar14];
    (&DAT_143ac5d00)[lVar14] = (uint)bVar5;
    (&DAT_143ac6100)[lVar14] = (uint)bVar5 << 8;
    (&DAT_143ac6500)[lVar14] = (uint)bVar5 << 0x10;
    (&DAT_143ac6900)[lVar14] = (uint)bVar5 << 0x18;
    if (bVar5 == 0) {
      uVar10 = 0;
      uVar13 = 0;
    }
    else {
      uVar10 = (uint)(byte)(&DAT_143ac3a00)[(int)(((byte)(&DAT_143ac3900)[bVar5] + uVar6) % 0xff)];
      uVar13 = (uint)(byte)(&DAT_143ac3a00)[(int)(((byte)(&DAT_143ac3900)[bVar5] + uVar15) % 0xff)];
    }
    uVar7 = uVar13 << 8 | (uint)bVar5;
    uVar8 = uVar7 << 8 | (uint)bVar5;
    uVar9 = uVar8 << 8 | uVar10;
    (&DAT_143ac3d00)[lVar14] = uVar9;
    (&DAT_143ac4100)[lVar14] = uVar9 << 8 | uVar13;
    (&DAT_143ac4500)[lVar14] = uVar9 << 0x10 | uVar7;
    (&DAT_143ac4900)[lVar14] = uVar10 << 0x18 | uVar8;
    bVar5 = (&DAT_143ac3c00)[lVar14];
    (&DAT_143ac6d00)[lVar14] = (uint)bVar5;
    (&DAT_143ac7100)[lVar14] = (uint)bVar5 << 8;
    (&DAT_143ac7500)[lVar14] = (uint)bVar5 << 0x10;
    (&DAT_143ac7900)[lVar14] = (uint)bVar5 << 0x18;
    if (bVar5 == 0) {
      uVar8 = 0;
      uVar7 = 0;
      uVar10 = 0;
      uVar13 = 0;
    }
    else {
      uVar13 = (uint)(byte)(&DAT_143ac3900)[bVar5];
      uVar8 = (uint)(byte)(&DAT_143ac3a00)[(int)((uVar13 + uVar19) % 0xff)];
      uVar7 = (uint)(byte)(&DAT_143ac3a00)[(int)((uVar13 + uVar18) % 0xff)];
      uVar10 = (uint)(byte)(&DAT_143ac3a00)[(int)((uVar13 + uVar20) % 0xff)];
      uVar13 = (uint)(byte)(&DAT_143ac3a00)[(int)((uVar13 + uVar16) % 0xff)];
    }
    uVar10 = uVar13 << 8 | uVar10;
    uVar7 = uVar10 << 8 | uVar7;
    uVar9 = uVar7 << 8 | uVar8;
    (&DAT_143ac4d00)[lVar14] = uVar9;
    (&DAT_143ac5100)[lVar14] = uVar9 << 8 | uVar13;
    (&DAT_143ac5500)[lVar14] = uVar9 << 0x10 | uVar10;
    (&DAT_143ac5900)[lVar14] = uVar8 << 0x18 | uVar7;
    lVar14 = lVar14 + 1;
    lVar12 = lVar12 + -1;
  } while (lVar12 != 0);
  DAT_143ac38d0 = 1;
  return;
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


