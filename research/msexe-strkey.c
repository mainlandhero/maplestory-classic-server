
//===========================================================
// FUN_1408aa600 @ 1408aa600   (448 bytes)
//===========================================================

longlong * FUN_1408aa600(longlong *param_1,longlong param_2,uint param_3,ulonglong param_4)

{
  ulonglong *puVar1;
  byte *pbVar2;
  byte *pbVar3;
  byte *pbVar4;
  ulonglong uVar5;
  uint uVar6;
  byte bVar7;
  ulonglong uVar8;
  byte bVar9;
  byte bVar10;
  ulonglong uVar11;
  
  uVar11 = 0;
  *param_1 = 0;
  uVar5 = uVar11;
  uVar8 = uVar11;
  if (param_3 != 0) {
    puVar1 = (ulonglong *)FUN_14019b780(&DAT_143ad68a0);
    if (puVar1 == (ulonglong *)0x0) {
      *param_1 = 0;
    }
    else {
      *param_1 = (longlong)(puVar1 + 1);
      if (puVar1 + 1 != (ulonglong *)0x0) {
        *puVar1 = (ulonglong)param_3;
      }
    }
  }
  while (pbVar2 = (byte *)*param_1, pbVar2 != (byte *)0x0) {
    uVar6 = (uint)uVar5;
    if (*(uint *)(pbVar2 + -8) <= uVar6) {
      if (pbVar2 != (byte *)0x0) {
        uVar11 = (ulonglong)*(uint *)(pbVar2 + -8);
      }
      break;
    }
    bVar9 = *(byte *)(uVar8 + param_2);
    if ((int)uVar6 < 0) {
      FUN_142e54290(0xbc);
      pbVar2 = (byte *)*param_1;
    }
    pbVar2[uVar8] = bVar9;
    uVar5 = (ulonglong)(uVar6 + 1);
    uVar8 = uVar8 + 1;
  }
  if ((int)uVar11 == 0) {
    return param_1;
  }
  if (param_4 == 0) {
    return param_1;
  }
  if (7 < param_4) {
    uVar5 = (param_4 >> 3) % uVar11;
    if (uVar5 != 0) {
      pbVar3 = (byte *)FUN_14019b780(&DAT_143ad68a0);
      if (uVar11 != 0) {
        pbVar4 = pbVar3;
        uVar8 = uVar11;
        do {
          *pbVar4 = pbVar2[(ulonglong)(pbVar4 + (uVar5 - (longlong)pbVar3)) % uVar11];
          pbVar4 = pbVar4 + 1;
          uVar8 = uVar8 - 1;
        } while (uVar8 != 0);
        pbVar4 = pbVar2;
        uVar5 = uVar11;
        do {
          *pbVar4 = pbVar4[(longlong)pbVar3 - (longlong)pbVar2];
          pbVar4 = pbVar4 + 1;
          uVar5 = uVar5 - 1;
        } while (uVar5 != 0);
      }
      FUN_14019b4e0(pbVar3);
    }
    uVar5 = param_4 & 7;
    param_4 = (ulonglong)((uint)param_4 & 7);
    if (uVar5 == 0) {
      return param_1;
    }
  }
  bVar9 = 8 - (byte)param_4;
  bVar10 = 0;
  if (uVar11 < 2) {
    if (uVar11 == 0) goto LAB_1408aa7a1;
  }
  else {
    bVar10 = *pbVar2 >> (bVar9 & 0x1f);
  }
  pbVar3 = pbVar2;
  do {
    bVar7 = 0;
    if ((longlong)pbVar3 + -(longlong)pbVar2 != uVar11 - 1) {
      bVar7 = pbVar3[1] >> (bVar9 & 0x1f);
    }
    *pbVar3 = *pbVar3 << ((byte)param_4 & 0x1f) | bVar7;
    pbVar3 = pbVar3 + 1;
  } while ((ulonglong)((longlong)pbVar3 + -(longlong)pbVar2) < uVar11);
LAB_1408aa7a1:
  pbVar2[uVar11 - 1] = pbVar2[uVar11 - 1] | bVar10;
  return param_1;
}


