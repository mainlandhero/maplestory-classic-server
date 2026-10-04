
//===========================================================
// FUN_14170f8a0 @ 14170f8a0   (234 bytes)
//===========================================================

void FUN_14170f8a0(longlong param_1)

{
  bool bVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  int iVar5;
  int iVar6;
  
  if ((((*(char **)(param_1 + 0x11a0) == (char *)0x0) || (**(char **)(param_1 + 0x11a0) == '\0')) &&
      (*(longlong *)(param_1 + 0x11d8) == 0)) && (*(int *)(param_1 + 0x11c0) < 1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (((*(int *)(param_1 + 0x11ac) != 0) && (*(int *)(param_1 + 0x11b0) != 0)) &&
     ((iVar3 = (**(code **)(*(longlong *)(param_1 + 8) + 0x78))(param_1 + 8), iVar3 != 0 &&
      ((bVar1 && (*(int *)(param_1 + 0xb0) == 0)))))) {
    uVar4 = (*DAT_143262db0)();
    cVar2 = FUN_1408fc970(*(undefined4 *)(param_1 + 0x11b0),*(undefined4 *)(param_1 + 0x11ac),uVar4)
    ;
    if (cVar2 != '\0') {
      iVar5 = (**(code **)(*(longlong *)(param_1 + 8) + 0x90))(param_1 + 8);
      iVar3 = *(int *)(param_1 + 0x11b4);
      iVar6 = (**(code **)(*(longlong *)(param_1 + 8) + 0x98))(param_1 + 8);
      FUN_14170f990(param_1,CONCAT44(iVar6 + *(int *)(param_1 + 0x11b8) + 0x14,iVar5 + iVar3 + 0x14)
                   );
    }
  }
  return;
}



//===========================================================
// FUN_14170ff50 @ 14170ff50   (3 bytes)
//===========================================================

undefined8 FUN_14170ff50(void)

{
  return 0;
}



//===========================================================
// FUN_14170f310 @ 14170f310   (494 bytes)
//===========================================================

void FUN_14170f310(longlong param_1,undefined8 param_2,undefined4 param_3,undefined4 param_4)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  longlong in_stack_00000040;
  undefined1 local_48 [8];
  undefined8 *local_40;
  undefined1 local_38 [8];
  undefined8 *local_30;
  
  *(undefined8 *)(param_1 + 0x11c4) = 0x1400000014;
  if (in_stack_00000040 != 0) {
    FUN_14019a260(param_1 + 0x11a0,in_stack_00000040);
    *(undefined4 *)(param_1 + 0x11a8) = *(undefined4 *)(in_stack_00000040 + 8);
    *(undefined4 *)(param_1 + 0x11ac) = *(undefined4 *)(in_stack_00000040 + 0xc);
    *(undefined4 *)(param_1 + 0x11c0) = *(undefined4 *)(in_stack_00000040 + 0x14);
    if (0 < *(int *)(in_stack_00000040 + 0x10)) {
      FUN_1403d2200(DAT_143aa8328,local_38,*(int *)(in_stack_00000040 + 0x10),0,0);
      puVar3 = local_30;
      if (local_30 != (undefined8 *)0x0) {
        local_40 = local_30;
        if (0xfffff < (ulonglong)local_30[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar3[1] = puVar3[1] + 1;
        UNLOCK();
        if ((*(longlong *)(param_1 + 0x11d8) - 1U < 999) || (*(longlong *)(param_1 + 0x11d8) == -1))
        {
          FUN_142e52ed0(0x447);
        }
        if ((undefined1 *)(param_1 + 0x11d0) == local_48) {
          FUN_142e52d50(0x45c,1);
        }
        puVar3 = local_40;
        if (local_40 != (undefined8 *)0x0) {
          if (0xfffff < (ulonglong)local_40[1]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          puVar3[1] = puVar3[1] + 1;
          UNLOCK();
        }
        FUN_1401abd80((undefined1 *)(param_1 + 0x11d0));
        *(undefined8 **)(param_1 + 0x11d8) = local_40;
        FUN_1401abd80(local_48);
      }
      puVar3 = local_30;
      if (local_30 != (undefined8 *)0x0) {
        if (0xffffe < local_30[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = puVar3 + 1;
        lVar2 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if (((int)lVar2 == 1) && (local_30 != (undefined8 *)0x0)) {
          (**(code **)*local_30)(local_30,1);
        }
      }
    }
    *(undefined4 *)(param_1 + 0x11e0) = *(undefined4 *)(in_stack_00000040 + 0x18);
  }
  FUN_141710020(param_1,param_2,param_3,param_4);
  return;
}



//===========================================================
// FUN_141710480 @ 141710480   (102 bytes)
//===========================================================

void FUN_141710480(longlong *param_1)

{
  if ((int)param_1[7] != -1) {
    if (DAT_143abfdf8 != 0) {
      FUN_142c12830(DAT_143abfdf8,param_1 + 1);
    }
    (**(code **)(*param_1 + 0x28))(param_1);
    FUN_142bf8670(param_1[10],param_1);
    param_1[10] = 0;
    if ((longlong *)param_1[8] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[8] + 0x10))();
    }
    param_1[8] = 0;
    *(undefined4 *)(param_1 + 7) = 0xffffffff;
  }
  return;
}



//===========================================================
// FUN_1417104f0 @ 1417104f0   (3 bytes)
//===========================================================

void FUN_1417104f0(void)

{
  return;
}



//===========================================================
// FUN_1410b9d00 @ 1410b9d00   (175 bytes)
//===========================================================

void FUN_1410b9d00(longlong param_1)

{
  if (*(longlong **)(param_1 + 0x11f8) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x11f8) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x11f8) = 0;
  if (*(longlong **)(param_1 + 0x11c0) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x11c0) + 0x18))();
    FUN_140cbca60(param_1 + 0x11b8);
  }
  if (*(longlong **)(param_1 + 0x11d0) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x11d0) + 0x18))();
    FUN_140cbca60(param_1 + 0x11c8);
  }
  if (*(longlong **)(param_1 + 0x11e0) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x11e0) + 0x18))();
    FUN_1410ac970(param_1 + 0x11d8);
  }
  if (*(longlong **)(param_1 + 0x11f0) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x11f0) + 0x18))();
    FUN_1410ac970(param_1 + 0x11e8);
  }
  return;
}



//===========================================================
// FUN_141710510 @ 141710510   (29 bytes)
//===========================================================

undefined8 FUN_141710510(longlong param_1,int param_2,int param_3)

{
  if ((((-1 < param_2) && (param_2 < *(int *)(param_1 + 0x48))) && (-1 < param_3)) &&
     (param_3 < *(int *)(param_1 + 0x4c))) {
    return 1;
  }
  return 0;
}



//===========================================================
// FUN_141710540 @ 141710540   (306 bytes)
//===========================================================

int * FUN_141710540(longlong param_1,int *param_2)

{
  IUnknown *pIVar1;
  int iVar2;
  int local_res8 [2];
  int local_res18 [4];
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  uint local_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined8 local_28;
  
  pIVar1 = *(IUnknown **)(param_1 + 0x40);
  if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_58);
  iVar2 = FUN_14023c4c0(&local_58,&DAT_143a8b8d8);
  if (-1 < iVar2) {
    local_38 = local_58;
    uStack_34 = uStack_54;
    uStack_30 = uStack_50;
    uStack_2c = uStack_4c;
    local_28 = local_48;
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x138))
                      (pIVar1,0,0,local_res8,local_res18,0,0,0,0,&local_38);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_143273488);
    }
    if ((short)local_58 == 8) {
      local_58 = local_58 & 0xffff0000;
      if (CONCAT44(uStack_4c,uStack_50) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_4c,uStack_50) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_58);
    }
    *param_2 = local_res8[0];
    param_2[1] = local_res18[0];
    param_2[2] = local_res8[0] + *(int *)(param_1 + 0x48);
    param_2[3] = *(int *)(param_1 + 0x4c) + local_res18[0];
    return param_2;
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(iVar2);
}



//===========================================================
// FUN_141710680 @ 141710680   (542 bytes)
//===========================================================

undefined8 FUN_141710680(undefined8 param_1,undefined8 param_2,int *param_3)

{
  int iVar1;
  int iVar2;
  int iVar3;
  bool bVar4;
  bool bVar5;
  bool bVar6;
  int iVar7;
  uint *puVar8;
  IUnknown *local_res20;
  uint local_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined8 local_a0;
  uint local_98 [2];
  longlong local_90;
  uint local_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 local_70;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  uint local_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 local_38;
  
  FUN_141710af0(param_1,&local_res20,0);
  if (local_res20 == (IUnknown *)0x0) {
    (*DAT_143262a20)(local_98);
    iVar7 = FUN_14023c4c0(local_98,&DAT_143ad4a38);
    if (iVar7 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar7);
    }
    puVar8 = local_98;
    bVar6 = false;
    bVar5 = false;
    bVar4 = true;
  }
  else {
    local_b0 = CONCAT22(local_b0._2_2_,3);
    uStack_a8 = 1;
    iVar7 = param_3[1];
    iVar1 = param_3[3];
    iVar2 = *param_3;
    iVar3 = param_3[2];
    (*DAT_143262a20)(&local_68);
    local_48 = local_b0;
    uStack_44 = uStack_ac;
    uStack_40 = uStack_a8;
    uStack_3c = uStack_a4;
    local_38 = local_a0;
    iVar7 = (**(code **)(*(longlong *)local_res20 + 0x118))
                      (local_res20,iVar2,iVar7,iVar3 - iVar2,iVar1 - iVar7,&local_48,&local_68);
    if (iVar7 < 0) {
      _com_issue_errorex(iVar7,local_res20,(_GUID *)&DAT_14327ac98);
    }
    local_80 = local_68;
    uStack_7c = uStack_64;
    uStack_78 = uStack_60;
    uStack_74 = uStack_5c;
    local_70 = local_58;
    puVar8 = &local_80;
    bVar6 = true;
    bVar5 = true;
    bVar4 = false;
  }
  (*DAT_143262a20)(param_2);
  iVar7 = FUN_14023c4c0(param_2,puVar8);
  if (-1 < iVar7) {
    if (bVar4) {
      if ((short)local_98[0] == 8) {
        local_98[0]._0_2_ = 0;
        if (local_90 != 0) {
          (*DAT_143ad5990)(local_90 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_98);
      }
    }
    if (bVar5) {
      if ((short)local_80 == 8) {
        local_80 = local_80 & 0xffff0000;
        if (CONCAT44(uStack_74,uStack_78) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_74,uStack_78) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_80);
      }
    }
    if (bVar6) {
      if ((short)local_b0 == 8) {
        local_b0 = local_b0 & 0xffff0000;
        if (CONCAT44(uStack_a4,uStack_a8) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_a4,uStack_a8) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_b0);
      }
    }
    if (local_res20 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_res20 + 0x10))(local_res20);
    }
    return param_2;
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(iVar7);
}



//===========================================================
// FUN_1417108b0 @ 1417108b0   (254 bytes)
//===========================================================

void FUN_1417108b0(longlong param_1,longlong param_2)

{
  longlong *plVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 *puVar4;
  longlong lVar5;
  undefined8 local_10;
  
  lVar3 = 0;
  lVar5 = param_1 + 0x18;
  if (param_1 == 0) {
    lVar5 = lVar3;
  }
  if (lVar5 == 0) {
    local_10 = 0;
  }
  else {
    local_10 = lVar5 + -0x18;
    if (local_10 != 0) {
      if (0xfffff < *(ulonglong *)(lVar5 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar5 + 8) = *(longlong *)(lVar5 + 8) + 1;
      UNLOCK();
    }
  }
  FUN_142bf8670(*(undefined8 *)(param_1 + 0x50),param_1);
  uVar2 = *(undefined8 *)(param_1 + 0x50);
  if (param_2 != 0) {
    lVar3 = FUN_142bf93d0(uVar2,param_2);
  }
  FUN_142bf94e0(uVar2,param_1,lVar3);
  if (local_10 != 0) {
    if (0xffffe < *(longlong *)(local_10 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(local_10 + 0x20);
    lVar5 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar5 == 1) && (puVar4 = (undefined8 *)(local_10 + 0x18), puVar4 != (undefined8 *)0x0)
       ) {
      (**(code **)*puVar4)(puVar4,1);
    }
  }
  return;
}



//===========================================================
// FUN_141710fc0 @ 141710fc0   (3 bytes)
//===========================================================

void FUN_141710fc0(void)

{
  return;
}



//===========================================================
// FUN_141711120 @ 141711120   (695 bytes)
//===========================================================

void FUN_141711120(longlong param_1,undefined4 param_2,undefined4 param_3)

{
  IUnknown *pIVar1;
  int iVar2;
  uint local_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined8 local_118;
  short local_110 [4];
  longlong lStack_108;
  undefined8 local_100;
  short local_f8 [4];
  longlong lStack_f0;
  undefined8 local_e8;
  short local_e0 [4];
  longlong lStack_d8;
  undefined8 local_d0;
  short local_c8 [4];
  longlong lStack_c0;
  undefined8 local_b8;
  undefined1 local_a8 [8];
  longlong lStack_a0;
  undefined8 local_98;
  undefined1 local_88 [8];
  longlong lStack_80;
  undefined8 local_78;
  undefined1 local_68 [8];
  longlong lStack_60;
  undefined8 local_58;
  undefined1 local_48 [8];
  longlong lStack_40;
  undefined8 local_38;
  uint local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 local_18;
  
  pIVar1 = *(IUnknown **)(param_1 + 0x40);
  if (pIVar1 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(local_c8);
  iVar2 = FUN_14023c4c0(local_c8,&DAT_143a8b8d8);
  if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar2);
  }
  (*DAT_143262a20)(local_e0);
  iVar2 = FUN_14023c4c0(local_e0,&DAT_143a8b8d8);
  if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar2);
  }
  (*DAT_143262a20)(local_f8);
  iVar2 = FUN_14023c4c0(local_f8,&DAT_143a8b8d8);
  if (-1 < iVar2) {
    (*DAT_143262a20)(local_110);
    iVar2 = FUN_14023c4c0(local_110,&DAT_143a8b8d8);
    if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
    (*DAT_143262a20)(&local_128);
    iVar2 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
    if (-1 < iVar2) {
      lStack_a0 = lStack_c0;
      local_98 = local_b8;
      lStack_80 = lStack_d8;
      local_78 = local_d0;
      lStack_60 = lStack_f0;
      local_58 = local_e8;
      lStack_40 = lStack_108;
      local_38 = local_100;
      local_28 = local_128;
      uStack_24 = uStack_124;
      uStack_20 = uStack_120;
      uStack_1c = uStack_11c;
      local_18 = local_118;
      iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x140))
                        (pIVar1,param_2,param_3,&local_28,local_48,local_68,local_88,local_a8);
      if (iVar2 < 0) {
        _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_143273488);
      }
      if ((short)local_128 == 8) {
        local_128 = local_128 & 0xffff0000;
        if (CONCAT44(uStack_11c,uStack_120) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_11c,uStack_120) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_128);
      }
      if (local_110[0] == 8) {
        local_110[0] = 0;
        if (lStack_108 != 0) {
          (*DAT_143ad5990)(lStack_108 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_110);
      }
      if (local_f8[0] == 8) {
        local_f8[0] = 0;
        if (lStack_f0 != 0) {
          (*DAT_143ad5990)(lStack_f0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_f8);
      }
      if (local_e0[0] == 8) {
        local_e0[0] = 0;
        if (lStack_d8 != 0) {
          (*DAT_143ad5990)(lStack_d8 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_e0);
      }
      if (local_c8[0] == 8) {
        local_c8[0] = 0;
        if (lStack_c0 != 0) {
          (*DAT_143ad5990)(lStack_c0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_c8);
      }
      return;
    }
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar2);
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(iVar2);
}



//===========================================================
// FUN_141711be0 @ 141711be0   (251 bytes)
//===========================================================

int FUN_141711be0(longlong param_1)

{
  undefined8 *puVar1;
  int iVar2;
  int iVar3;
  IUnknown *pIVar4;
  int local_res8 [2];
  IUnknown *local_res10;
  IUnknown *local_res18;
  
  iVar2 = 0;
  puVar1 = *(undefined8 **)(param_1 + 0x68);
  if (puVar1 != (undefined8 *)0x0) {
    local_res10 = (IUnknown *)0x0;
    iVar2 = (**(code **)*puVar1)(puVar1,&DAT_143273488,&local_res10);
    pIVar4 = (IUnknown *)0x0;
    if (-1 < iVar2) {
      pIVar4 = local_res10;
    }
    local_res18 = pIVar4;
    if (((iVar2 + 0x80000000U & 0x80000000) == 0) && (iVar2 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
    if (pIVar4 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8[0] = 0;
    iVar2 = (**(code **)(*(longlong *)pIVar4 + 0xd0))(pIVar4,local_res8);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar4,(_GUID *)&DAT_143273488);
    }
    iVar2 = local_res8[0];
    (**(code **)(*(longlong *)pIVar4 + 0x10))(pIVar4);
  }
  pIVar4 = *(IUnknown **)(param_1 + 0x40);
  if (pIVar4 != (IUnknown *)0x0) {
    local_res8[0] = 0;
    iVar3 = (**(code **)(*(longlong *)pIVar4 + 0xd0))(pIVar4,local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar4,(_GUID *)&DAT_143273488);
    }
    return local_res8[0] + iVar2;
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_141711cf0 @ 141711cf0   (251 bytes)
//===========================================================

int FUN_141711cf0(longlong param_1)

{
  undefined8 *puVar1;
  int iVar2;
  int iVar3;
  IUnknown *pIVar4;
  int local_res8 [2];
  IUnknown *local_res10;
  IUnknown *local_res18;
  
  iVar2 = 0;
  puVar1 = *(undefined8 **)(param_1 + 0x68);
  if (puVar1 != (undefined8 *)0x0) {
    local_res10 = (IUnknown *)0x0;
    iVar2 = (**(code **)*puVar1)(puVar1,&DAT_143273488,&local_res10);
    pIVar4 = (IUnknown *)0x0;
    if (-1 < iVar2) {
      pIVar4 = local_res10;
    }
    local_res18 = pIVar4;
    if (((iVar2 + 0x80000000U & 0x80000000) == 0) && (iVar2 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
    if (pIVar4 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8[0] = 0;
    iVar2 = (**(code **)(*(longlong *)pIVar4 + 0xe0))(pIVar4,local_res8);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar4,(_GUID *)&DAT_143273488);
    }
    iVar2 = local_res8[0];
    (**(code **)(*(longlong *)pIVar4 + 0x10))(pIVar4);
  }
  pIVar4 = *(IUnknown **)(param_1 + 0x40);
  if (pIVar4 != (IUnknown *)0x0) {
    local_res8[0] = 0;
    iVar3 = (**(code **)(*(longlong *)pIVar4 + 0xe0))(pIVar4,local_res8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar4,(_GUID *)&DAT_143273488);
    }
    return local_res8[0] + iVar2;
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_14170feb0 @ 14170feb0   (15 bytes)
//===========================================================

void FUN_14170feb0(longlong param_1)

{
                    /* WARNING: Could not recover jumptable at 0x00014170febb. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(*(longlong *)(*(longlong *)(param_1 + 0x48) + 8) + 8))();
  return;
}



//===========================================================
// FUN_14170fef0 @ 14170fef0   (3 bytes)
//===========================================================

void FUN_14170fef0(void)

{
  return;
}



//===========================================================
// FUN_14170f560 @ 14170f560   (191 bytes)
//===========================================================

void FUN_14170f560(longlong *param_1,int param_2,int param_3)

{
  int iVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  
  if (((((char *)param_1[0x233] != (char *)0x0) && (*(char *)param_1[0x233] != '\0')) ||
      (param_1[0x23a] != 0)) || (0 < (int)param_1[0x237])) {
    if (*(int *)((longlong)param_1 + 0x11a4) == 0) {
      iVar2 = (**(code **)(*param_1 + 0x90))();
      iVar1 = *(int *)((longlong)param_1 + 0x11bc);
      iVar3 = (**(code **)(*param_1 + 0x98))(param_1);
      FUN_14170f990(param_1 + -1,
                    CONCAT44(iVar3 + (int)param_1[0x238] + param_3,iVar2 + iVar1 + param_2));
    }
    else {
      uVar4 = (*DAT_143262db0)();
      *(undefined4 *)(param_1 + 0x235) = uVar4;
      *(int *)((longlong)param_1 + 0x11ac) = param_2;
      *(int *)(param_1 + 0x236) = param_3;
    }
  }
  FUN_14170ff00(param_1,param_2,param_3);
  return;
}



//===========================================================
// FUN_14170f630 @ 14170f630   (252 bytes)
//===========================================================

void FUN_14170f630(longlong param_1,int param_2,int param_3,undefined4 param_4)

{
  longlong lVar1;
  IUnknown *pIVar2;
  code *UNRECOVERED_JUMPTABLE;
  int iVar3;
  int aiStackX_8 [2];
  
  lVar1 = *(longlong *)(param_1 + 0x48);
  if (lVar1 == 0) {
    FUN_141d5f5a0();
    return;
  }
  pIVar2 = *(IUnknown **)(param_1 + 0x38);
  UNRECOVERED_JUMPTABLE = *(code **)(*(longlong *)(lVar1 + 8) + 0x28);
  if (pIVar2 != (IUnknown *)0x0) {
    aiStackX_8[0] = 0;
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 0xe0))(pIVar2,aiStackX_8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
    }
    pIVar2 = *(IUnknown **)(param_1 + 0x38);
    param_3 = aiStackX_8[0] + param_3;
    if (pIVar2 != (IUnknown *)0x0) {
      aiStackX_8[0] = 0;
      iVar3 = (**(code **)(*(longlong *)pIVar2 + 0xd0))(pIVar2,aiStackX_8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
      }
                    /* WARNING: Could not recover jumptable at 0x00014170f711. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*UNRECOVERED_JUMPTABLE)(lVar1 + 8,aiStackX_8[0] + param_2,param_3,param_4);
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_14170f510 @ 14170f510   (72 bytes)
//===========================================================

void FUN_14170f510(longlong param_1,int param_2)

{
  undefined4 uVar1;
  
  FUN_14170ff10();
  if (param_2 == 0) {
    *(undefined4 *)(param_1 + 0x11a8) = 0;
    FUN_142645170(param_1 + 0x70);
    return;
  }
  uVar1 = (*DAT_143262db0)();
  *(undefined4 *)(param_1 + 0x11a8) = uVar1;
  return;
}



//===========================================================
// FUN_142645170 @ 142645170   (630 bytes)
//===========================================================

void FUN_142645170(longlong param_1)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined4 *puVar4;
  longlong *plVar5;
  longlong lVar6;
  
  *(undefined8 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x40) = 0;
  if (*(longlong **)(param_1 + 0x48) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x48) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x48) = 0;
  if (*(longlong **)(param_1 + 0x50) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x50) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x50) = 0;
  if (*(longlong **)(param_1 + 0x68) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x68) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x68) = 0;
  if (*(longlong **)(param_1 + 0x70) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x70) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x70) = 0;
  if (*(longlong **)(param_1 + 0x58) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x58) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x58) = 0;
  if (*(longlong **)(param_1 + 0x60) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x60) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x60) = 0;
  *(undefined4 *)(param_1 + 0xa0) = 0;
  *(undefined8 *)(param_1 + 0xc70) = 0;
  *(undefined4 *)(param_1 + 0xc78) = 0;
  *(undefined4 *)(param_1 + 0x10c0) = 0;
  *(undefined4 *)(param_1 + 0x10c4) = 0xffffffff;
  *(undefined4 *)(param_1 + 0x1100) = 0;
  *(undefined1 *)(param_1 + 0x4e8) = 0;
  *(undefined8 *)(param_1 + 0x4ec) = 0;
  plVar5 = (longlong *)(param_1 + 0x938);
  puVar4 = (undefined4 *)(param_1 + 0xc4);
  lVar6 = 0x22;
  do {
    *(undefined8 *)(puVar4 + -7) = 0;
    *puVar4 = 0;
    *(undefined8 *)(puVar4 + 0x2ef) = 0;
    puVar4[0x2f6] = 0;
    lVar3 = plVar5[1];
    lVar2 = *plVar5;
    if (lVar2 != lVar3) {
      do {
        if (*(longlong *)(lVar2 + 8) != 0) {
          FUN_14019f2c0(*(longlong *)(lVar2 + 8) + -0x10);
        }
        lVar2 = lVar2 + 0x10;
      } while (lVar2 != lVar3);
      lVar2 = *plVar5;
    }
    plVar5[1] = lVar2;
    *(undefined8 *)(puVar4 + 0x10d) = 0;
    puVar4[0x114] = 0;
    plVar5 = plVar5 + 3;
    puVar4 = puVar4 + 8;
    lVar6 = lVar6 + -1;
  } while (lVar6 != 0);
  lVar6 = *(longlong *)(param_1 + 0x10e0);
  lVar3 = *(longlong *)(param_1 + 0x10e8);
  if (lVar6 != lVar3) {
    do {
      puVar1 = *(undefined8 **)(lVar6 + 8);
      if (puVar1 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar1[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar1[1] = puVar1[1] + 1;
        UNLOCK();
      }
      if (puVar1 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_142645170(puVar1);
      if (puVar1 != (undefined8 *)0x0) {
        if (0xffffe < puVar1[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar5 = puVar1 + 1;
        lVar2 = *plVar5;
        *plVar5 = *plVar5 + -1;
        UNLOCK();
        if (((int)lVar2 == 1) && (puVar1 != (undefined8 *)0x0)) {
          (**(code **)*puVar1)(puVar1,1);
        }
      }
      lVar6 = lVar6 + 0x10;
    } while (lVar6 != lVar3);
    lVar6 = *(longlong *)(param_1 + 0x10e8);
    lVar3 = *(longlong *)(param_1 + 0x10e0);
    if (lVar3 != lVar6) {
      do {
        FUN_140336390(lVar3);
        lVar3 = lVar3 + 0x10;
      } while (lVar3 != lVar6);
      lVar3 = *(longlong *)(param_1 + 0x10e0);
    }
    *(longlong *)(param_1 + 0x10e8) = lVar3;
  }
  return;
}



//===========================================================
// FUN_14170ff00 @ 14170ff00   (3 bytes)
//===========================================================

undefined8 FUN_14170ff00(void)

{
  return 0;
}


