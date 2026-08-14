
//===========================================================
// FUN_1408aadb0 @ 1408aadb0   (303 bytes)
//===========================================================

void FUN_1408aadb0(longlong *param_1)

{
  byte bVar1;
  longlong lVar2;
  uint uVar3;
  byte *pbVar4;
  ulonglong uVar5;
  byte bVar6;
  ulonglong uVar7;
  int iVar8;
  int iVar9;
  undefined4 uVar10;
  longlong local_res8;
  
  FUN_1408aa600(&local_res8);
  pbVar4 = (byte *)FUN_14019bd40(param_1,0,1);
  iVar9 = 0;
  bVar1 = *pbVar4;
  while (bVar1 != 0) {
    if (local_res8 == 0) {
      uVar3 = 0;
    }
    else {
      uVar3 = *(uint *)(local_res8 + -8);
    }
    uVar7 = (ulonglong)(longlong)iVar9 % (ulonglong)uVar3;
    if (local_res8 == 0) {
      uVar5 = 0;
    }
    else {
      uVar5 = (ulonglong)*(uint *)(local_res8 + -8);
    }
    if (uVar5 <= uVar7) {
      if (local_res8 == 0) {
        uVar10 = 0;
      }
      else {
        uVar10 = *(undefined4 *)(local_res8 + -8);
      }
      FUN_142e54290(0xc6,uVar7,uVar10);
    }
    bVar1 = *(byte *)(uVar7 + local_res8);
    *pbVar4 = *pbVar4 ^ bVar1;
    bVar6 = *pbVar4;
    if (*pbVar4 == 0) {
      bVar6 = bVar1;
    }
    *pbVar4 = bVar6;
    iVar9 = iVar9 + 1;
    pbVar4 = pbVar4 + 1;
    bVar1 = *pbVar4;
  }
  lVar2 = *param_1;
  if (*(int *)(lVar2 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  *(undefined4 *)(lVar2 + -0x10) = 1;
  if (lVar2 == 0) {
    uVar7 = 0;
    iVar9 = iRamfffffffffffffff4;
LAB_1408aaea1:
    iVar8 = (int)uVar7;
    if (iVar8 < iVar9 + 1) goto LAB_1408aaeb1;
  }
  else {
    uVar7 = 0xffffffffffffffff;
    do {
      uVar7 = uVar7 + 1;
    } while (*(char *)(lVar2 + uVar7) != '\0');
    iVar9 = *(int *)(lVar2 + -0xc);
    if (-1 < (int)uVar7) goto LAB_1408aaea1;
  }
  iVar8 = (int)uVar7;
  FUN_142e54290(0x9c,uVar7 & 0xffffffff);
LAB_1408aaeb1:
  *(int *)(lVar2 + -8) = iVar8;
  if (local_res8 != 0) {
    thunk_FUN_140205820(local_res8 + -8,0);
  }
  return;
}


