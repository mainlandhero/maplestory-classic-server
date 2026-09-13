
//===========================================================
// FUN_142d9ac30 @ 142d9ac30   (273 bytes)
//===========================================================

void FUN_142d9ac30(undefined8 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  longlong *plVar1;
  char cVar2;
  longlong lVar3;
  undefined8 *puVar4;
  
  cVar2 = FUN_141b1f960(DAT_143abea80,1);
  if (cVar2 == '\0') {
    lVar3 = FUN_142cbe730(param_1);
    if (lVar3 != 0) {
      puVar4 = (undefined8 *)0x0;
      lVar3 = FUN_14019b780(&DAT_143ad68a0,0x68);
      if (lVar3 != 0) {
        puVar4 = (undefined8 *)FUN_141f0e110(lVar3,param_2,param_3);
      }
      if (puVar4 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar4[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar4[1] = puVar4[1] + 1;
        UNLOCK();
      }
      if (puVar4 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_141f0e4c0(puVar4,param_4);
      if (puVar4 != (undefined8 *)0x0) {
        if (0xffffe < puVar4[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = puVar4 + 1;
        lVar3 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar3 == 1) {
          (**(code **)*puVar4)(puVar4,1);
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_140711d70 @ 140711d70   (205 bytes)
//===========================================================

undefined8
FUN_140711d70(undefined8 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
             undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined4 param_8,
             undefined4 param_9,undefined8 param_10)

{
  int iVar1;
  undefined8 uVar2;
  int iVar3;
  
  iVar1 = FUN_140721a00(param_2);
  iVar3 = 1;
  if (0 < iVar1) {
    do {
      uVar2 = FUN_140711e50(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,
                            iVar3,param_10);
      if ((int)uVar2 != 0) {
        return uVar2;
      }
      iVar3 = iVar3 + 1;
    } while (iVar3 <= iVar1);
  }
  return 0;
}



//===========================================================
// FUN_14070fa90 @ 14070fa90   (72 bytes)
//===========================================================

longlong FUN_14070fa90(longlong param_1,int param_2)

{
  longlong lVar1;
  
  if ((*(longlong *)(param_1 + 0x1a8) != 0) &&
     (lVar1 = *(longlong *)
               (*(longlong *)(param_1 + 0x1a8) +
               ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x1b0)) * 8),
     lVar1 != 0)) {
    while (*(int *)(lVar1 + 0x10) != param_2) {
      lVar1 = *(longlong *)(lVar1 + 8);
      if (lVar1 == 0) {
        return 0;
      }
    }
    if ((lVar1 != -0x18) && (*(longlong *)(lVar1 + 0x20) != 0)) {
      return *(longlong *)(lVar1 + 0x20);
    }
  }
  return 0;
}



//===========================================================
// FUN_142518060 @ 142518060   (869 bytes)
//===========================================================

void FUN_142518060(longlong param_1,int param_2,int param_3)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  longlong lVar5;
  longlong *plVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  int *piVar9;
  longlong local_res20;
  
  iVar2 = FUN_140713fc0(DAT_143aa9d98);
  if (iVar2 != 0) {
    return;
  }
  if (param_2 < 0x8786) {
    if (param_2 != 0x8785) {
      if (param_2 < 0x5e2d) {
        if (param_2 != 0x5e2c) {
          if (param_2 < 0x550f) {
            if (((param_2 != 0x550e) && (param_2 != 0x51e2)) &&
               ((param_2 != 0x51e3 && (param_2 != 0x550d)))) goto LAB_14251822b;
          }
          else if (((param_2 != 0x585b) && (param_2 != 0x585c)) && (param_2 != 0x5e2b))
          goto LAB_14251822b;
        }
      }
      else if (param_2 < 0x644f) {
        if ((param_2 != 0x644e) &&
           (((param_2 != 0x6367 && (param_2 != 0x6368)) && (param_2 != 0x644d))))
        goto LAB_14251822b;
      }
      else if ((param_2 != 0x7dea) && (param_2 != 0x7dec)) goto LAB_14251822b;
    }
  }
  else if (param_2 < 0x8f5f) {
    if (param_2 != 0x8f5e) {
      if (param_2 < 0x8860) {
        if ((param_2 != 0x885f) &&
           (((param_2 != 0x8786 && (param_2 != 0x880d)) && (param_2 != 0x880e))))
        goto LAB_14251822b;
      }
      else if ((param_2 != 0x8860) && (param_2 != 0x8f5d)) goto LAB_14251822b;
    }
  }
  else if (param_2 < 0x9a26) {
    if (((param_2 != 0x9a25) && ((param_2 != 0x94c2 && (param_2 != 0x94c3)))) && (param_2 != 0x9a24)
       ) goto LAB_14251822b;
  }
  else if ((param_2 != 0x9aea) && (param_2 != 0x9aeb)) goto LAB_14251822b;
  lVar5 = FUN_142c0b7c0(DAT_143abfdf8);
  if (lVar5 != 0) {
    lVar5 = FUN_142c0b7c0(DAT_143abfdf8);
    iVar2 = (**(code **)(*(longlong *)(lVar5 + 8) + 0xd0))
                      ((longlong *)(lVar5 + 8),&PTR_PTR_143a87e00);
    if (((iVar2 != 0) &&
        (plVar6 = (longlong *)FUN_142c0b7c0(DAT_143abfdf8), plVar6 != (longlong *)0x0)) &&
       (cVar1 = (**(code **)(*plVar6 + 0x140))(plVar6), cVar1 != '\0')) {
      return;
    }
  }
LAB_14251822b:
  iVar2 = FUN_1425165a0(param_1,param_2);
  if (iVar2 == 0) {
    *(int *)(param_1 + 0x20) = param_2;
    piVar9 = *(int **)(param_1 + 0x28);
    if (*piVar9 != 1) {
      piVar9[4] = 1;
      *piVar9 = 1;
      iVar3 = FUN_1429e3ef0();
      piVar9[2] = iVar3 + param_3;
    }
    FUN_1425183d0(param_1,0,param_3);
    FUN_142518580(param_1);
  }
  else {
    piVar9 = (int *)(*(longlong *)(param_1 + 0x28) + (longlong)iVar2 * 0x58);
    if (*piVar9 != 2) {
      piVar9[4] = 1;
      *piVar9 = (*piVar9 != 0) + 1;
      iVar3 = FUN_1429e3ef0();
      piVar9[2] = iVar3 + param_3;
    }
    FUN_1425183d0(param_1,iVar2,param_3);
    FUN_142518580(param_1);
    if (iVar2 == 1) {
      return;
    }
  }
  iVar3 = FUN_1429e3ef0();
  cVar1 = FUN_142cf1c20(DAT_143aa84a0);
  if ((cVar1 == '\0') && (2999 < iVar3 - *(int *)(param_1 + 0x80))) {
    iVar4 = FUN_140715c10(DAT_143aa9d98,param_2);
    if ((iVar4 == 0) && (cVar1 = FUN_141b1f960(DAT_143abea80,1), cVar1 == '\0')) {
      if (iVar2 == 0) {
        uVar7 = FUN_1408a9e40(&local_res20,0xb04);
        FUN_1415eca30(uVar7,6);
      }
      else {
        uVar7 = FUN_1408a9e40(&local_res20,0xb03);
        FUN_1415eca30(uVar7,6);
      }
      if (local_res20 != 0) {
        FUN_14019f2c0(local_res20 + -0x10);
      }
      puVar8 = (undefined8 *)FUN_1408a9d20(&local_res20,0xb11);
      FUN_1429f14c0(*puVar8,100);
      if (local_res20 != 0) {
        FUN_1401bebb0(local_res20 + -0x10);
      }
    }
    *(int *)(param_1 + 0x80) = iVar3;
  }
  return;
}



//===========================================================
// FUN_1407158a0 @ 1407158a0   (68 bytes)
//===========================================================

undefined8 FUN_1407158a0(longlong param_1,int param_2)

{
  longlong lVar1;
  
  if (*(longlong *)(param_1 + 0x498) != 0) {
    for (lVar1 = *(longlong *)
                  (*(longlong *)(param_1 + 0x498) +
                  ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x4a0)) * 8);
        lVar1 != 0; lVar1 = *(longlong *)(lVar1 + 8)) {
      if (*(int *)(lVar1 + 0x10) == param_2) {
        if (lVar1 == -0x14) {
          return 0;
        }
        return 1;
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_140715940 @ 140715940   (110 bytes)
//===========================================================

undefined8 FUN_140715940(longlong param_1,int param_2)

{
  int *piVar1;
  ulonglong uVar2;
  
  piVar1 = *(int **)(param_1 + 0x658);
  while( true ) {
    if (piVar1 == (int *)0x0) {
      return 0;
    }
    if (*piVar1 == param_2) break;
    uVar2 = *(ulonglong *)(piVar1 + -8);
    if ((uVar2 != 0) && (uVar2 < 0x10001)) {
      FUN_142e52ed0(0x33e);
      uVar2 = *(ulonglong *)(piVar1 + -8);
    }
    piVar1 = (int *)0x0;
    if (uVar2 != 0) {
      piVar1 = (int *)(uVar2 + 0x28);
    }
  }
  return 1;
}



//===========================================================
// FUN_142d28eb0 @ 142d28eb0   (273 bytes)
//===========================================================

undefined8 * FUN_142d28eb0(longlong *param_1,undefined8 *param_2,int *param_3,undefined4 *param_4)

{
  longlong *plVar1;
  bool bVar2;
  undefined1 uVar3;
  longlong *plVar4;
  longlong *plVar5;
  longlong *local_38;
  undefined8 uStack_30;
  longlong *local_28;
  uint uStack_20;
  undefined4 uStack_1c;
  
  plVar1 = (longlong *)*param_1;
  local_28 = (longlong *)plVar1[1];
  uStack_20 = 0;
  plVar5 = plVar1;
  if (*(char *)((longlong)local_28 + 0x19) == '\0') {
    plVar4 = local_28;
    do {
      local_28 = plVar4;
      bVar2 = *param_3 <= *(int *)((longlong)local_28 + 0x1c);
      if (bVar2) {
        plVar4 = (longlong *)*local_28;
        plVar5 = local_28;
      }
      else {
        plVar4 = (longlong *)local_28[2];
      }
      uStack_20 = (uint)bVar2;
    } while (*(char *)((longlong)plVar4 + 0x19) == '\0');
  }
  if ((*(char *)((longlong)plVar5 + 0x19) == '\0') &&
     (*(int *)((longlong)plVar5 + 0x1c) <= *param_3)) {
    uVar3 = 0;
  }
  else {
    if (param_1[1] == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    uStack_30 = 0;
    local_38 = param_1;
    plVar5 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
    *(int *)((longlong)plVar5 + 0x1c) = *param_3;
    *(undefined4 *)(plVar5 + 4) = *param_4;
    *plVar5 = (longlong)plVar1;
    plVar5[1] = (longlong)plVar1;
    plVar5[2] = (longlong)plVar1;
    *(undefined2 *)(plVar5 + 3) = 0;
    local_38 = local_28;
    uStack_30 = CONCAT44(uStack_1c,uStack_20);
    plVar5 = (longlong *)FUN_1401a1df0(param_1,&local_38);
    uVar3 = 1;
  }
  *param_2 = plVar5;
  *(undefined1 *)(param_2 + 1) = uVar3;
  return param_2;
}



//===========================================================
// FUN_142da3e70 @ 142da3e70   (186 bytes)
//===========================================================

undefined8 FUN_142da3e70(longlong *param_1,undefined4 param_2)

{
  longlong lVar1;
  undefined4 uVar2;
  int iVar3;
  undefined4 uVar4;
  
  if (DAT_143aaa058 == 0) {
    return 1;
  }
  uVar2 = FUN_142cafb20();
  iVar3 = FUN_142da37f0(param_1,9,uVar2);
  lVar1 = DAT_143aaa058;
  if (iVar3 != 0) {
    uVar2 = (**(code **)(*param_1 + 0xa8))(param_1);
    iVar3 = FUN_14025b010(lVar1,uVar2,param_2);
    if (iVar3 == 0) {
      uVar2 = FUN_140716320(DAT_143aa9d98,param_2);
      lVar1 = DAT_143aaa058;
      uVar4 = (**(code **)(*param_1 + 0xa8))(param_1);
      iVar3 = FUN_14025b290(lVar1,uVar4,uVar2);
      if (iVar3 == 0) {
        return 1;
      }
    }
  }
  return 0;
}


