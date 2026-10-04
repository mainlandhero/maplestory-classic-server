
//===========================================================
// FUN_142a91ad0 @ 142a91ad0   (346 bytes)
//===========================================================

void FUN_142a91ad0(longlong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined4 *puVar3;
  longlong lVar4;
  undefined4 local_res8 [2];
  
  switch(*(undefined4 *)(param_1 + 0x70)) {
  case 0xb:
  case 0xc:
  case 0xf:
  case 0x15:
  case 0x16:
  case 0x19:
    if (*(longlong *)(param_1 + 0x10) != 0) {
      local_res8[0] = 0;
      FUN_1401a7220(*(longlong *)(param_1 + 0x10),*(undefined4 *)(param_1 + 0x70),local_res8);
      puVar3 = *(undefined4 **)(param_1 + 0xd0);
      if ((puVar3 != (undefined4 *)0x0) && (puVar3[-2] != 0)) {
        while( true ) {
          uVar1 = FUN_1401a8660(*(undefined4 *)(param_1 + 0x70),local_res8[0],*puVar3);
          *puVar3 = uVar1;
          if ((undefined4 *)
              (*(longlong *)(param_1 + 0xd0) + *(longlong *)(*(longlong *)(param_1 + 0xd0) + -8) * 4
              + -4) <= puVar3) break;
          puVar3 = puVar3 + 1;
          if (puVar3 == (undefined4 *)0x0) {
            return;
          }
        }
      }
    }
    break;
  case 0xd:
  case 0x10:
  case 0x11:
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x17:
    break;
  case 0xe:
  case 0x18:
    if (((*(longlong *)(param_1 + 0x80) != 0) &&
        (puVar3 = *(undefined4 **)(param_1 + 0xd0), puVar3 != (undefined4 *)0x0)) &&
       (puVar3[-2] != 0)) {
      do {
        lVar4 = *(longlong *)(param_1 + 0x80);
        if (lVar4 == 0) {
          FUN_142e52ed0(0x428,0);
          lVar4 = *(longlong *)(param_1 + 0x80);
        }
        iVar2 = FUN_14041a270(lVar4);
        if (iVar2 == 0) {
          uVar1 = FUN_140419eb0(*puVar3);
        }
        else {
          uVar1 = FUN_14041a990();
        }
        *puVar3 = uVar1;
      } while ((puVar3 < (undefined4 *)
                         (*(longlong *)(param_1 + 0xd0) +
                          *(longlong *)(*(longlong *)(param_1 + 0xd0) + -8) * 4 + -4)) &&
              (puVar3 = puVar3 + 1, puVar3 != (undefined4 *)0x0));
    }
    break;
  default:
    goto switchD_142a91b01_default;
  }
switchD_142a91b01_default:
  return;
}



//===========================================================
// FUN_142a91c70 @ 142a91c70   (192 bytes)
//===========================================================

void FUN_142a91c70(longlong param_1,undefined1 param_2)

{
  int iVar1;
  uint uVar2;
  byte *pbVar3;
  ulonglong uVar4;
  undefined1 *local_res8;
  undefined1 local_res10 [8];
  
  local_res8 = local_res10;
  if (*(longlong *)(param_1 + 0x80) != 0) {
    local_res10[0] = param_2;
    iVar1 = FUN_14041a270();
    if (iVar1 != 0) {
      pbVar3 = *(byte **)(param_1 + 0x80);
      if (pbVar3 == (byte *)0x0) {
        FUN_142e52ed0(0x428,0);
        pbVar3 = *(byte **)(param_1 + 0x80);
      }
      FUN_142a93710(&local_res8,param_1 + 0x98,*pbVar3,pbVar3[1]);
      uVar4 = (ulonglong)*pbVar3;
      uVar2 = (uint)pbVar3[1];
      goto LAB_142a91d11;
    }
  }
  uVar2 = 0xffffffff;
  local_res10[0] = 1;
  FUN_142a93710(&local_res8,param_1 + 0x98,0xffffffff,0xffffffff);
  uVar4 = 0xffffffff;
LAB_142a91d11:
  FUN_142a93710(&local_res8,param_1 + 0xb0,uVar2,uVar4);
  return;
}



//===========================================================
// FUN_142a91e50 @ 142a91e50   (69 bytes)
//===========================================================

void FUN_142a91e50(undefined4 *param_1,undefined8 param_2)

{
  int iVar1;
  undefined4 uVar2;
  
  iVar1 = FUN_14041a270(param_2);
  if (iVar1 != 0) {
    uVar2 = FUN_14041a990(*param_1,param_2);
    *param_1 = uVar2;
    return;
  }
  uVar2 = FUN_140419eb0(*param_1);
  *param_1 = uVar2;
  return;
}



//===========================================================
// FUN_142a93870 @ 142a93870   (587 bytes)
//===========================================================

void FUN_142a93870(longlong *param_1,longlong param_2,undefined4 param_3,undefined4 param_4)

{
  longlong lVar1;
  undefined8 uVar2;
  IUnknown *pIVar3;
  longlong *plVar4;
  int iVar5;
  undefined8 *puVar6;
  longlong *plVar7;
  longlong *plVar8;
  undefined4 uVar9;
  undefined8 local_res10;
  undefined4 local_48 [2];
  longlong *local_40;
  longlong *local_38;
  longlong *local_30;
  
  lVar1 = *(longlong *)(param_2 + 0x80);
  if ((lVar1 != 0) && (iVar5 = FUN_14041a270(lVar1), iVar5 != 0)) {
    uVar2 = *(undefined8 *)(*param_1 + 0x6c8);
    puVar6 = (undefined8 *)FUN_142bf6010(*param_1,&local_30);
    pIVar3 = (IUnknown *)*puVar6;
    if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    plVar8 = (longlong *)0x0;
    local_res10 = (ulonglong)local_res10._4_4_ << 0x20;
    iVar5 = (**(code **)(*(longlong *)pIVar3 + 0x1b0))(pIVar3,&local_res10);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_14327fcb0);
    }
    puVar6 = (undefined8 *)FUN_142bf6010(*param_1,&local_38);
    pIVar3 = (IUnknown *)*puVar6;
    if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_48[0] = 0;
    iVar5 = (**(code **)(*(longlong *)pIVar3 + 0x1a0))(pIVar3,local_48);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_14327fcb0);
    }
    uVar9 = 0;
    plVar7 = (longlong *)FUN_141aa5a30(uVar2,&local_40,local_48[0],local_res10 & 0xffffffff,5,0,0,0)
    ;
    plVar4 = *(longlong **)(param_2 + 0x48);
    if (plVar4 != (longlong *)*plVar7) {
      *(longlong **)(param_2 + 0x48) = (longlong *)*plVar7;
      *plVar7 = 0;
      if (plVar4 != (longlong *)0x0) {
        (**(code **)(*plVar4 + 0x10))();
      }
    }
    if (local_40 != (longlong *)0x0) {
      (**(code **)(*local_40 + 0x10))();
    }
    if (local_38 != (longlong *)0x0) {
      (**(code **)(*local_38 + 0x10))();
    }
    if (local_30 != (longlong *)0x0) {
      (**(code **)(*local_30 + 0x10))();
    }
    local_res10 = FUN_14019b780(&DAT_143ad68a0,0x108);
    if (local_res10 != 0) {
      plVar8 = (longlong *)FUN_1416ed1a0(local_res10);
    }
    (**(code **)(*plVar8 + 0x80))(plVar8,*param_1,param_3,0,4,CONCAT44(uVar9,0x9b),param_4,0x6a,0);
    FUN_1416ee330(plVar8,99);
    uVar9 = 0x87;
    if (*(char *)param_1[1] != '\0') {
      uVar9 = 0;
    }
    *(undefined4 *)(plVar8 + 0xf) = uVar9;
    FUN_1416ee2d0(plVar8,*(byte *)(lVar1 + 2) - 1);
    (**(code **)(plVar8[1] + 0x70))(plVar8 + 1,*(char *)param_1[1] == '\0');
    FUN_140d2cd60(param_2 + 0x88,plVar8);
  }
  return;
}



//===========================================================
// FUN_142a93ac0 @ 142a93ac0   (351 bytes)
//===========================================================

void FUN_142a93ac0(undefined8 *param_1,longlong param_2,longlong *param_3)

{
  undefined1 *puVar1;
  undefined8 uVar2;
  int iVar3;
  longlong lVar4;
  longlong lVar5;
  undefined1 local_res10 [8];
  longlong *local_res18;
  undefined1 local_res20 [8];
  
  puVar1 = *(undefined1 **)(param_2 + 0x80);
  local_res18 = param_3;
  if (puVar1 == (undefined1 *)0x0) {
    lVar4 = *param_3;
  }
  else {
    iVar3 = FUN_14041a270(puVar1);
    if (iVar3 != 0) {
      uVar2 = *param_1;
      lVar4 = -1;
      lVar5 = -1;
      do {
        lVar5 = lVar5 + 1;
      } while (L"/BaseProb"[lVar5] != L'\0');
      FUN_14040ea40(param_3,local_res10);
      lVar5 = -1;
      do {
        lVar5 = lVar5 + 1;
      } while (L"/BaseColor"[lVar5] != L'\0');
      FUN_14040ea40(param_3,local_res20);
      FUN_142a94040(uVar2,*puVar1,100 - (uint)(byte)puVar1[2],local_res20,local_res10);
      uVar2 = *param_1;
      lVar5 = -1;
      do {
        lVar5 = lVar5 + 1;
      } while (L"/AddProb"[lVar5] != L'\0');
      FUN_14040ea40(param_3,local_res10);
      do {
        lVar4 = lVar4 + 1;
      } while (L"/AddColor"[lVar4] != L'\0');
      FUN_14040ea40(param_3,local_res20,L"/AddColor",lVar4);
      FUN_142a94040(uVar2,puVar1[1],puVar1[2],local_res20,local_res10);
    }
    lVar4 = *param_3;
  }
  if (lVar4 != 0) {
    FUN_1401bebb0(lVar4 + -0x10);
  }
  return;
}



//===========================================================
// FUN_142a8b6a0 @ 142a8b6a0   (249 bytes)
//===========================================================

void FUN_142a8b6a0(longlong param_1,int param_2)

{
  int iVar1;
  byte *pbVar2;
  
  iVar1 = *(int *)(param_1 + 0x2a8);
  if ((((iVar1 == 0x16) || (iVar1 == 0x17)) || (iVar1 == 0x19)) ||
     ((iVar1 == 0x1c || (iVar1 == 0x1d)))) {
    pbVar2 = *(byte **)(param_1 + 0x498);
    if ((pbVar2 != (byte *)0x0) && (iVar1 = FUN_14041a270(pbVar2), iVar1 != 0)) {
      if (((param_2 % 100) / 10 & 1U) != 0) {
        pbVar2 = pbVar2 + 1;
      }
      if ((uint)*pbVar2 != param_2 % 10) {
        *pbVar2 = (byte)(param_2 % 10);
        FUN_142a91ad0(param_1 + 0x418);
        FUN_142a91c70(param_1 + 0x418,0);
        FUN_142a8ff80(param_1,param_1 + 0x418);
      }
    }
  }
  return;
}


