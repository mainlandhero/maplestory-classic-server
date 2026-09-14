
//===========================================================
// FUN_1533fd730 @ 1533fd730   (59 bytes)
//===========================================================

undefined8 FUN_1533fd730(longlong param_1,undefined4 param_2)

{
  longlong lVar1;
  
  lVar1 = param_1 + 0x10;
  if (param_1 == 0x10) {
    lVar1 = 0;
  }
  FUN_1532f2530(*(undefined8 *)(param_1 + 0x38),lVar1,param_2,*(undefined8 *)(param_1 + 0x48));
  FUN_1532f3ac0(param_1 + 0x10);
  return 0;
}



//===========================================================
// FUN_1533fe1d0 @ 1533fe1d0   (9 bytes)
//===========================================================

undefined8 FUN_1533fe1d0(longlong param_1,undefined4 param_2)

{
  *(undefined4 *)(param_1 + 0xf4) = param_2;
  return 0;
}



//===========================================================
// FUN_1533fe560 @ 1533fe560   (366 bytes)
//===========================================================

undefined8 FUN_1533fe560(longlong param_1,uint param_2)

{
  longlong *plVar1;
  longlong lVar2;
  IUnknown *pIVar3;
  _func_5156 *ppvObject;
  void *pvVar4;
  HRESULT HVar5;
  ULONG UVar6;
  int *piVar7;
  void **ppvObject_00;
  
  pvVar4 = Self;
  plVar1 = (longlong *)(param_1 + 0xa0);
  LOCK();
  lVar2 = *plVar1;
  if (lVar2 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar2 == 0) {
LAB_1533fe5dc:
    *(undefined4 *)(param_1 + 0xa8) = 1;
  }
  else if ((void *)*plVar1 == pvVar4) {
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  else {
    while( true ) {
      pvVar4 = Self;
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_1533fe5dc;
      if ((void *)*plVar1 == pvVar4) break;
      Sleep(0);
    }
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  piVar7 = (int *)(param_1 + 0xa8);
  pIVar3 = *(IUnknown **)(param_1 + 0xd8);
  if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_153513900(0x80004003);
  }
  ppvObject = pIVar3->lpVtbl[3].QueryInterface;
  HVar5 = (*ppvObject)(pIVar3,(IID *)((ulonglong)(param_2 >> 0x10) & 0xff),(void **)ppvObject);
  if (HVar5 < 0) {
    _com_issue_errorex(HVar5,pIVar3,(_GUID *)&DAT_1535daa58);
  }
  pIVar3 = *(IUnknown **)(param_1 + 0xe0);
  if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_153513900(0x80004003);
  }
  ppvObject_00 = (void **)((ulonglong)param_2 & 0xff);
  UVar6 = (*pIVar3->lpVtbl[5].AddRef)(pIVar3);
  if ((int)UVar6 < 0) {
    ppvObject_00 = (void **)&DAT_1535daa58;
    _com_issue_errorex(UVar6,pIVar3,(_GUID *)&DAT_1535daa58);
  }
  pIVar3 = *(IUnknown **)(param_1 + 0xd0);
  if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_153513900(0x80004003);
  }
  HVar5 = (*pIVar3->lpVtbl[3].QueryInterface)
                    (pIVar3,(IID *)(ulonglong)(param_2 >> 0x18),ppvObject_00);
  if (HVar5 < 0) {
    _com_issue_errorex(HVar5,pIVar3,(_GUID *)&DAT_1535daa58);
  }
  *piVar7 = *piVar7 + -1;
  if (*piVar7 == 0) {
    *plVar1 = 0;
  }
  return 0;
}



//===========================================================
// FUN_1533fe8f0 @ 1533fe8f0   (633 bytes)
//===========================================================

undefined4 FUN_1533fe8f0(longlong param_1,_union_2707 *param_2)

{
  longlong lVar1;
  VARTYPE VVar2;
  void *pvVar3;
  longlong *plVar4;
  HRESULT HVar5;
  int iVar6;
  undefined4 uVar7;
  int *piVar8;
  longlong *plVar9;
  longlong *local_res10;
  longlong *local_res18;
  _union_2707 local_50;
  _union_2707 local_38;
  
  local_res10 = (longlong *)0x0;
  VVar2 = (((_union_2707 *)&(param_2->n2).vt)->n2).vt;
  if ((VVar2 == 0) || (VVar2 == 10)) goto LAB_1533feab3;
  VariantInit((VARIANTARG *)&local_50.n2);
  if (&local_50 == param_2) {
    if (local_50.n2.vt != 0xd) {
      if ((((_union_2707 *)&(param_2->n2).vt)->n2).vt != 8) goto LAB_1533fe991;
      local_38._0_4_ = local_38._0_4_ & 0xffff0000;
      HVar5 = VariantChangeType((VARIANTARG *)&local_38.n2,(VARIANTARG *)&param_2->n2,0,0xd);
      if (-1 < HVar5) {
        FUN_153315820(&local_50);
        local_50._0_4_ = local_38._0_4_;
        local_50.decVal.Hi32 = local_38.decVal.Hi32;
        local_50._8_4_ = local_38._8_4_;
        local_50._12_4_ = local_38._12_4_;
        local_50._16_8_ = local_38._16_8_;
      }
      goto LAB_1533fe9c9;
    }
  }
  else {
LAB_1533fe991:
    if (local_50.n2.vt == 8) {
      local_50._0_4_ = local_50._0_4_ & 0xffff0000;
      if (CONCAT44(local_50._12_4_,local_50._8_4_) != 0) {
        (*DAT_153775ef0)(CONCAT44(local_50._12_4_,local_50._8_4_) + -4);
      }
    }
    HVar5 = VariantChangeType((VARIANTARG *)&local_50.n2,(VARIANTARG *)&param_2->n2,0,0xd);
LAB_1533fe9c9:
    if (HVar5 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_153513900(HVar5);
    }
  }
  plVar4 = local_res10;
  plVar9 = (longlong *)CONCAT44(local_50._12_4_,local_50._8_4_);
  if (local_res10 != plVar9) {
    if (plVar9 == (longlong *)0x0) {
LAB_1533fea15:
      local_res10 = (longlong *)0x0;
    }
    else {
      iVar6 = (**(code **)*plVar9)(plVar9,&DAT_15364c108,&local_res10);
      plVar9 = (longlong *)CONCAT44(local_50._12_4_,local_50._8_4_);
      if (iVar6 < 0) goto LAB_1533fea15;
    }
    if (plVar4 != (longlong *)0x0) {
      (**(code **)(*plVar4 + 0x10))(plVar4);
      plVar9 = (longlong *)CONCAT44(local_50._12_4_,local_50._8_4_);
    }
  }
  if (local_res10 == (longlong *)0x0) {
    if (local_50.n2.vt == 8) {
      local_50._0_4_ = (uint)local_50.n2.wReserved1 << 0x10;
      if (plVar9 != (longlong *)0x0) {
        (*DAT_153775ef0)((longlong)plVar9 + -4);
      }
    }
    else {
      VariantClear((VARIANTARG *)&local_50.n2);
    }
    if (local_res10 != (longlong *)0x0) {
      (**(code **)(*local_res10 + 0x10))();
    }
    return 0x80070057;
  }
  if (local_50.n2.vt == 8) {
    local_50._0_4_ = (uint)local_50.n2.wReserved1 << 0x10;
    if (plVar9 != (longlong *)0x0) {
      (*DAT_153775ef0)((longlong)plVar9 + -4);
    }
  }
  else {
    VariantClear((VARIANTARG *)&local_50.n2);
  }
LAB_1533feab3:
  pvVar3 = Self;
  plVar9 = (longlong *)(param_1 + 0xa0);
  LOCK();
  lVar1 = *plVar9;
  if (lVar1 == 0) {
    *plVar9 = (longlong)Self;
  }
  UNLOCK();
  local_res18 = plVar9;
  if (lVar1 == 0) {
LAB_1533feb1c:
    *(undefined4 *)(param_1 + 0xa8) = 1;
  }
  else if ((void *)*plVar9 == pvVar3) {
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar1 = *plVar9;
      if (lVar1 == 0) {
        *plVar9 = (longlong)Self;
      }
      UNLOCK();
      if (lVar1 == 0) goto LAB_1533feb1c;
      if ((void *)*plVar9 == pvVar3) break;
      Sleep(0);
    }
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  piVar8 = (int *)(param_1 + 0xa8);
  uVar7 = FUN_1532f3e00(param_1 + 0x10,local_res10,param_1 + -0x10);
  *piVar8 = *piVar8 + -1;
  if (*piVar8 == 0) {
    *plVar9 = 0;
  }
  if (local_res10 != (longlong *)0x0) {
    (**(code **)(*local_res10 + 0x10))();
  }
  return uVar7;
}



//===========================================================
// FUN_1533fee90 @ 1533fee90   (1441 bytes)
//===========================================================

undefined8
FUN_1533fee90(longlong param_1,IUnknown *param_2,short *param_3,short *param_4,short *param_5,
             short *param_6,short *param_7,undefined2 *param_8)

{
  longlong *plVar1;
  longlong *plVar2;
  IUnknown *This;
  bool bVar3;
  bool bVar4;
  longlong lVar5;
  void *pvVar6;
  ULONG UVar7;
  int iVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined8 uVar11;
  undefined8 *puVar12;
  longlong lVar13;
  longlong **pplVar14;
  undefined8 *puVar15;
  int *piVar16;
  longlong *plVar17;
  int *local_98;
  longlong *local_90;
  longlong *local_88;
  longlong *local_80;
  undefined1 local_78 [8];
  undefined8 *local_70;
  longlong *local_68;
  longlong *local_60;
  longlong *local_58;
  longlong *local_50;
  longlong *local_40;
  longlong *local_38;
  
  if (param_2 == (IUnknown *)0x0) {
    uVar11 = 0x80070057;
  }
  else {
    local_88 = (longlong *)0x0;
    UVar7 = (*param_2->lpVtbl[5].AddRef)(param_2);
    if ((int)UVar7 < 0) {
      _com_issue_errorex(UVar7,param_2,(_GUID *)&DAT_1535d2948);
    }
    plVar2 = local_88;
    local_60 = local_88;
    local_90 = (longlong *)0x0;
    if ((local_88 != (longlong *)0x0) &&
       (iVar8 = (**(code **)*local_88)(local_88,&DAT_1535d1248,&local_90), iVar8 < 0)) {
      local_90 = (longlong *)0x0;
    }
    pvVar6 = Self;
    if ((plVar2 == (longlong *)0x0) || (local_90 != (longlong *)0x0)) {
      plVar2 = (longlong *)(param_1 + 0x1b8);
      LOCK();
      lVar13 = *plVar2;
      if (lVar13 == 0) {
        *plVar2 = (longlong)Self;
      }
      UNLOCK();
      local_58 = plVar2;
      if (lVar13 == 0) {
        *(undefined4 *)(param_1 + 0x1c0) = 1;
      }
      else if ((void *)*plVar2 == pvVar6) {
        *(int *)(param_1 + 0x1c0) = *(int *)(param_1 + 0x1c0) + 1;
      }
      else {
        while( true ) {
          pvVar6 = Self;
          LOCK();
          lVar13 = *plVar2;
          if (lVar13 == 0) {
            *plVar2 = (longlong)Self;
          }
          UNLOCK();
          if (lVar13 == 0) {
            *(undefined4 *)(param_1 + 0x1c0) = 1;
            goto LAB_1533fefed;
          }
          if ((void *)*plVar2 == pvVar6) break;
          Sleep(0);
        }
        *(int *)(param_1 + 0x1c0) = *(int *)(param_1 + 0x1c0) + 1;
      }
LAB_1533fefed:
      pvVar6 = Self;
      lVar5 = DAT_1537757a0;
      local_98 = (int *)(param_1 + 0x1c0);
      local_70 = (undefined8 *)0x0;
      plVar1 = (longlong *)(DAT_1537757a0 + 0x18);
      LOCK();
      lVar13 = *plVar1;
      if (lVar13 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      local_50 = plVar1;
      if (lVar13 == 0) {
LAB_1533ff060:
        *(undefined4 *)(lVar5 + 0x20) = 1;
      }
      else if ((void *)*plVar1 == pvVar6) {
        *(int *)(lVar5 + 0x20) = *(int *)(lVar5 + 0x20) + 1;
      }
      else {
        while( true ) {
          pvVar6 = Self;
          LOCK();
          lVar13 = *plVar1;
          if (lVar13 == 0) {
            *plVar1 = (longlong)Self;
          }
          UNLOCK();
          if (lVar13 == 0) goto LAB_1533ff060;
          if ((void *)*plVar1 == pvVar6) break;
          Sleep(0);
        }
        *(int *)(lVar5 + 0x20) = *(int *)(lVar5 + 0x20) + 1;
      }
      piVar16 = (int *)(lVar5 + 0x20);
      puVar12 = *(undefined8 **)(lVar5 + 0x28);
      if (puVar12 == (undefined8 *)0x0) {
        puVar12 = (undefined8 *)FUN_1532f55f0(0x70);
        *(undefined8 **)(lVar5 + 0x28) = puVar12;
      }
      *(undefined8 *)(lVar5 + 0x28) = *puVar12;
      *piVar16 = *piVar16 + -1;
      if (*piVar16 == 0) {
        *plVar1 = 0;
      }
      puVar12[6] = 0;
      puVar12[7] = 0;
      puVar12[8] = 0;
      puVar12[9] = 0;
      puVar12[10] = 0;
      puVar12[0xb] = 0;
      puVar12[0xc] = 0;
      puVar12[0xd] = 0;
      puVar12[3] = 0;
      puVar12[2] = 0;
      *puVar12 = &PTR_FUN_153699bc8;
      puVar12[4] = &PTR_LAB_153699bd0;
      puVar12[5] = 0;
      *(undefined8 *)((longlong)puVar12 + 0x54) = 0;
      *(undefined8 *)((longlong)puVar12 + 0x5c) = 0;
      *(undefined8 *)((longlong)puVar12 + 100) = 0;
      puVar12[1] = 1;
      puVar15 = (undefined8 *)0x0;
      if (puVar12 != (undefined8 *)0x0) {
        puVar15 = puVar12 + 5;
      }
      local_70 = puVar15;
      (*param_2->lpVtbl->AddRef)(param_2);
      This = (IUnknown *)*puVar15;
      if (This != param_2) {
        *puVar15 = param_2;
        param_2 = (IUnknown *)0x0;
        if (This != (IUnknown *)0x0) {
          (*This->lpVtbl->Release)(This);
        }
      }
      if (param_2 != (IUnknown *)0x0) {
        (*param_2->lpVtbl->Release)(param_2);
      }
      do {
        iVar8 = *(int *)(param_1 + 0x1fc) + 1;
        *(int *)(param_1 + 0x1fc) = iVar8;
        *(int *)(puVar15 + 2) = iVar8;
      } while (iVar8 == 0);
      lVar13 = FUN_153406860(param_1 + 0x1e0);
      if (puVar15 != (undefined8 *)0x0) {
        LOCK();
        puVar15[-4] = puVar15[-4] + 1;
        UNLOCK();
        puVar15 = local_70;
      }
      FUN_153407d60();
      *(undefined8 **)(lVar13 + 8) = puVar15;
      puVar15[1] = lVar13;
      if (*param_3 == 3) {
        uVar9 = *(undefined4 *)(param_3 + 4);
      }
      else if (*param_3 == 0x13) {
        uVar9 = *(undefined4 *)(param_3 + 4);
      }
      else {
        uVar9 = 0xffffffff;
      }
      *(undefined4 *)((longlong)puVar15 + 0x14) = uVar9;
      uVar9 = 0xffffffff;
      uVar10 = uVar9;
      if (*param_4 == 3) {
        uVar10 = *(undefined4 *)(param_4 + 4);
      }
      *(undefined4 *)(puVar15 + 3) = uVar10;
      if (*param_5 == 3) {
        uVar9 = *(undefined4 *)(param_5 + 4);
      }
      *(undefined4 *)((longlong)puVar15 + 0x1c) = uVar9;
      if (*param_6 == 3) {
        uVar9 = *(undefined4 *)(param_6 + 4);
      }
      else {
        uVar9 = 0;
      }
      *(undefined4 *)(puVar15 + 4) = uVar9;
      if (*param_7 == 3) {
        uVar9 = *(undefined4 *)(param_7 + 4);
      }
      else {
        uVar9 = 0;
      }
      *(undefined4 *)((longlong)puVar15 + 0x24) = uVar9;
      FUN_153406e90(param_1 + 0x1c8,puVar15 + 2,local_78);
      *(int *)(param_1 + 0x1f8) = *(int *)(param_1 + 0x1f8) + *(int *)((longlong)puVar15 + 0x14);
      bVar4 = false;
      bVar3 = true;
      if (param_8 != (undefined2 *)0x0) {
        *param_8 = 6;
        *(longlong *)(param_8 + 4) = (longlong)*(int *)(puVar15 + 2) << 0x20;
      }
      pvVar6 = Self;
      plVar1 = (longlong *)(param_1 + 0xa0);
      LOCK();
      lVar13 = *plVar1;
      if (lVar13 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      local_40 = plVar1;
      if (lVar13 == 0) {
LAB_1533ff2bb:
        iVar8 = 1;
        *(undefined4 *)(param_1 + 0xa8) = 1;
      }
      else if ((void *)*plVar1 == pvVar6) {
        piVar16 = (int *)(param_1 + 0xa8);
        *piVar16 = *piVar16 + 1;
        iVar8 = *piVar16;
      }
      else {
        while( true ) {
          pvVar6 = Self;
          LOCK();
          lVar13 = *plVar1;
          if (lVar13 == 0) {
            *plVar1 = (longlong)Self;
          }
          UNLOCK();
          if (lVar13 == 0) goto LAB_1533ff2bb;
          if ((void *)*plVar1 == pvVar6) break;
          Sleep(0);
        }
        piVar16 = (int *)(param_1 + 0xa8);
        *piVar16 = *piVar16 + 1;
        iVar8 = *piVar16;
      }
      plVar17 = *(longlong **)(param_1 + 0x218);
      if (plVar17 != *(longlong **)(*(longlong *)(param_1 + 0x1e8) + 8)) {
        if (plVar17 == (longlong *)0x0) {
          pplVar14 = &local_68;
          plVar17 = (longlong *)0x0;
        }
        else {
          plVar17 = (longlong *)*plVar17;
          local_80 = plVar17;
          if (plVar17 != (longlong *)0x0) {
            (**(code **)(*plVar17 + 8))(plVar17);
          }
          pplVar14 = &local_80;
          bVar4 = true;
          bVar3 = false;
        }
        *pplVar14 = (longlong *)0x0;
        local_38 = plVar17;
        if ((bVar3) && (local_68 != (longlong *)0x0)) {
          (**(code **)(*local_68 + 0x10))();
        }
        if ((bVar4) && (local_80 != (longlong *)0x0)) {
          (**(code **)(*local_80 + 0x10))();
        }
        FUN_153406380(param_1 + 0x210,*(undefined8 *)(param_1 + 0x1e8));
        FUN_1533fb2d0(param_1 + -0x10,plVar17);
        if (plVar17 != (longlong *)0x0) {
          (**(code **)(*plVar17 + 0x10))(plVar17);
        }
        iVar8 = *(int *)(param_1 + 0xa8);
      }
      *(int *)(param_1 + 0xa8) = iVar8 + -1;
      if (iVar8 + -1 == 0) {
        *plVar1 = 0;
      }
      puVar12 = local_70 + -5;
      LOCK();
      plVar1 = local_70 + -4;
      lVar13 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar13 == 1) {
        if (local_70[-2] != 0) {
          LOCK();
          *(undefined8 *)(local_70[-2] + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(local_70[-2] + 4) != 0);
        }
        if (puVar12 != (undefined8 *)0x0) {
          (**(code **)*puVar12)(puVar12,1);
        }
      }
      *local_98 = *local_98 + -1;
      if (*local_98 == 0) {
        *plVar2 = 0;
      }
      if (local_90 != (longlong *)0x0) {
        (**(code **)(*local_90 + 0x10))();
      }
      if (local_88 != (longlong *)0x0) {
        (**(code **)(*local_88 + 0x10))();
      }
      uVar11 = 0;
    }
    else {
      (**(code **)(*plVar2 + 0x10))(plVar2);
      uVar11 = 0x80070057;
    }
  }
  return uVar11;
}



//===========================================================
// FUN_1533ff5c0 @ 1533ff5c0   (1078 bytes)
//===========================================================

undefined4 FUN_1533ff5c0(longlong *param_1,undefined8 param_2,longlong *param_3)

{
  longlong *plVar1;
  longlong lVar2;
  longlong *plVar3;
  bool bVar4;
  bool bVar5;
  void *pvVar6;
  int iVar7;
  undefined4 uVar8;
  longlong **pplVar9;
  longlong *plVar10;
  longlong *plVar11;
  longlong *plVar12;
  undefined4 local_res8 [2];
  longlong *local_78;
  longlong *local_70;
  longlong *local_68;
  longlong *local_60;
  longlong *local_58;
  longlong *local_50;
  undefined1 local_40 [8];
  longlong *local_38;
  
  pvVar6 = Self;
  plVar1 = param_1 + 0x37;
  LOCK();
  lVar2 = *plVar1;
  if (lVar2 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  local_60 = plVar1;
  if (lVar2 == 0) {
LAB_1533ff64d:
    *(undefined4 *)(param_1 + 0x38) = 1;
  }
  else if ((void *)*plVar1 == pvVar6) {
    *(int *)(param_1 + 0x38) = (int)param_1[0x38] + 1;
  }
  else {
    while( true ) {
      pvVar6 = Self;
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_1533ff64d;
      if ((void *)*plVar1 == pvVar6) break;
      Sleep(0);
    }
    *(int *)(param_1 + 0x38) = (int)param_1[0x38] + 1;
  }
  plVar12 = param_1 + 0x38;
  local_38 = (longlong *)0x0;
  local_68 = param_1 + -2;
  iVar7 = FUN_1533fb1b0(local_68,param_2,local_40);
  plVar3 = local_38;
  if (iVar7 == -2) {
    (**(code **)(*param_1 + 0x2c8))(param_1,0);
    FUN_153373cd0(param_1 + 0x3c);
    FUN_153373db0(param_1 + 0x39);
    *(undefined4 *)(param_1 + 0x3f) = 0;
  }
  else {
    if (iVar7 != -1) {
      if (iVar7 != 1) {
        uVar8 = FUN_1534069b0();
        if (local_38 != (longlong *)0x0) {
          plVar10 = local_38 + -5;
          LOCK();
          plVar3 = local_38 + -4;
          lVar2 = *plVar3;
          *plVar3 = *plVar3 + -1;
          UNLOCK();
          if ((int)lVar2 == 1) {
            if (local_38[-2] != 0) {
              LOCK();
              *(undefined8 *)(local_38[-2] + 8) = 0;
              UNLOCK();
              do {
              } while (*(int *)(local_38[-2] + 4) != 0);
            }
            if (plVar10 != (longlong *)0x0) {
              (**(code **)*plVar10)(plVar10,1);
            }
          }
        }
        *(int *)plVar12 = (int)*plVar12 + -1;
        if ((int)*plVar12 != 0) {
          return uVar8;
        }
        *plVar1 = 0;
        return uVar8;
      }
      *(int *)(param_1 + 0x3f) = (int)param_1[0x3f] - *(int *)((longlong)local_38 + 0x14);
      FUN_1534071c0(param_1 + 0x3c,local_38[1]);
      local_res8[0] = (undefined4)plVar3[2];
      FUN_153407310(param_1 + 0x39,local_res8);
      if (param_3 != (longlong *)0x0) {
        plVar3 = (longlong *)*plVar3;
        *param_3 = (longlong)plVar3;
        if (plVar3 != (longlong *)0x0) {
          (**(code **)(*plVar3 + 8))();
        }
      }
      goto LAB_1533ff811;
    }
    if (*(int *)((longlong)param_1 + 0x1e4) != 0) {
      if (param_3 != (longlong *)0x0) {
        plVar3 = (longlong *)**(longlong **)(param_1[0x3e] + 8);
        *param_3 = (longlong)plVar3;
        if (plVar3 != (longlong *)0x0) {
          (**(code **)(*plVar3 + 8))();
        }
      }
      *(int *)(param_1 + 0x3f) =
           (int)param_1[0x3f] - *(int *)(*(longlong *)(param_1[0x3e] + 8) + 0x14);
      local_res8[0] = *(undefined4 *)(*(longlong *)(param_1[0x3e] + 8) + 0x10);
      FUN_153407310(param_1 + 0x39,local_res8);
      FUN_1534071c0(param_1 + 0x3c,param_1[0x3e]);
      goto LAB_1533ff811;
    }
  }
  if (param_3 != (longlong *)0x0) {
    *param_3 = 0;
  }
LAB_1533ff811:
  pvVar6 = Self;
  plVar3 = param_1 + 0x14;
  LOCK();
  lVar2 = *plVar3;
  if (lVar2 == 0) {
    *plVar3 = (longlong)Self;
  }
  UNLOCK();
  local_58 = plVar3;
  if (lVar2 == 0) {
LAB_1533ff86d:
    *(undefined4 *)(param_1 + 0x15) = 1;
  }
  else if ((void *)*plVar3 == pvVar6) {
    *(int *)(param_1 + 0x15) = (int)param_1[0x15] + 1;
  }
  else {
    while( true ) {
      pvVar6 = Self;
      LOCK();
      lVar2 = *plVar3;
      if (lVar2 == 0) {
        *plVar3 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_1533ff86d;
      if ((void *)*plVar3 == pvVar6) break;
      Sleep(0);
    }
    *(int *)(param_1 + 0x15) = (int)param_1[0x15] + 1;
  }
  plVar10 = param_1 + 0x15;
  FUN_153407150(param_1 + 0x45);
  if ((*(int *)((longlong)param_1 + 0x1e4) == 0) ||
     (param_1[0x43] != *(longlong *)(param_1[0x3d] + 8))) {
    if ((longlong *)param_1[0x43] == (longlong *)0x0) {
      pplVar9 = &local_70;
      bVar5 = false;
      bVar4 = true;
      plVar11 = (longlong *)0x0;
    }
    else {
      plVar11 = *(longlong **)param_1[0x43];
      local_78 = plVar11;
      if (plVar11 != (longlong *)0x0) {
        (**(code **)(*plVar11 + 8))(plVar11);
      }
      pplVar9 = &local_78;
      bVar5 = true;
      bVar4 = false;
    }
    *pplVar9 = (longlong *)0x0;
    local_50 = plVar11;
    if ((bVar4) && (local_70 != (longlong *)0x0)) {
      (**(code **)(*local_70 + 0x10))();
    }
    if ((bVar5) && (local_78 != (longlong *)0x0)) {
      (**(code **)(*local_78 + 0x10))();
    }
    if (*(int *)((longlong)param_1 + 0x1e4) == 0) {
      FUN_153407d60();
    }
    else {
      FUN_153406380(param_1 + 0x42,param_1[0x3d]);
    }
    FUN_1533fb2d0(local_68,plVar11);
    if (plVar11 != (longlong *)0x0) {
      (**(code **)(*plVar11 + 0x10))(plVar11);
    }
  }
  *(int *)plVar10 = (int)*plVar10 + -1;
  if ((int)*plVar10 == 0) {
    *plVar3 = 0;
  }
  if (local_38 != (longlong *)0x0) {
    plVar10 = local_38 + -5;
    LOCK();
    plVar3 = local_38 + -4;
    lVar2 = *plVar3;
    *plVar3 = *plVar3 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      if (local_38[-2] != 0) {
        LOCK();
        *(undefined8 *)(local_38[-2] + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(local_38[-2] + 4) != 0);
      }
      if (plVar10 != (longlong *)0x0) {
        (**(code **)*plVar10)(plVar10,1);
      }
    }
  }
  *(int *)plVar12 = (int)*plVar12 + -1;
  if ((int)*plVar12 == 0) {
    *plVar1 = 0;
  }
  return 0;
}



//===========================================================
// FUN_1533fffd0 @ 1533fffd0   (6887 bytes)
//===========================================================

undefined4 FUN_1533fffd0(IUnknown *param_1,uint param_2,undefined8 param_3,undefined8 param_4)

{
  code *pcVar1;
  _func_5157 **pp_Var2;
  IUnknown *pIVar3;
  int *piVar4;
  uint uVar5;
  _func_5157 *p_Var6;
  IUnknown *pIVar7;
  _func_5158 *p_Var8;
  float fVar9;
  float fVar10;
  void *pvVar11;
  int iVar12;
  HRESULT HVar13;
  ULONG UVar14;
  undefined4 uVar15;
  int iVar16;
  int iVar17;
  longlong lVar18;
  IUnknownVtbl *pIVar19;
  undefined8 *puVar20;
  uint uVar21;
  code *pcVar22;
  IUnknownVtbl *pIVar23;
  IUnknownVtbl *pIVar24;
  _func_5156 *p_Var25;
  IID *riid;
  IUnknownVtbl *pIVar26;
  IUnknown *pIVar27;
  IUnknown *pIVar28;
  void **ppvVar29;
  uint uVar30;
  IUnknownVtbl *pIVar31;
  int iVar32;
  bool bVar33;
  _func_5157 *local_3b0;
  undefined1 local_398 [8];
  _func_5157 *local_390;
  int local_388;
  int local_384;
  int local_380;
  int local_37c;
  int local_378;
  int local_374;
  int local_370;
  undefined1 local_36c [16];
  undefined4 local_35c;
  undefined4 local_358;
  undefined1 local_354 [12];
  longlong lStack_348;
  undefined8 local_340;
  _union_2707 local_338;
  _union_2707 local_320;
  _union_2707 local_308;
  _union_2707 local_2f0;
  _func_5157 *local_2d0;
  IUnknownVtbl *local_2c8;
  _union_2707 local_2c0;
  _union_2707 local_2a8;
  _union_2707 local_290;
  _union_2707 local_278;
  _union_2707 local_260;
  _union_2707 local_248;
  _union_2707 local_230;
  _union_2707 local_218;
  IUnknown *local_200;
  IUnknown *local_1f8;
  undefined2 local_1f0;
  undefined6 uStack_1ee;
  longlong lStack_1e8;
  undefined8 local_1e0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 local_1a8;
  longlong lStack_1a0;
  undefined8 local_198;
  undefined8 local_188;
  _union_1719 _Stack_180;
  undefined8 local_178;
  undefined8 local_168;
  _union_1719 _Stack_160;
  undefined8 local_158;
  undefined8 local_148;
  _union_1719 _Stack_140;
  undefined8 local_138;
  undefined8 local_128;
  _union_1719 _Stack_120;
  undefined8 local_118;
  undefined8 local_108;
  longlong lStack_100;
  undefined8 local_f8;
  _union_1719 _Stack_e0;
  undefined8 local_d8;
  _union_1719 _Stack_c0;
  undefined8 local_b8;
  _union_1719 _Stack_a0;
  undefined8 local_98;
  _union_1719 _Stack_80;
  undefined8 local_78;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  
  ppvVar29 = (void **)0x3e8;
  iVar12 = FUN_15333abb0(&local_380,param_3);
  pvVar11 = Self;
  if (iVar12 < 0) {
    return 0x80070057;
  }
  if (param_1[0x17].lpVtbl == (IUnknownVtbl *)0x0) {
    return 0;
  }
  pIVar7 = param_1 + 0x37;
  LOCK();
  bVar33 = pIVar7->lpVtbl == (IUnknownVtbl *)0x0;
  if (bVar33) {
    pIVar7->lpVtbl = Self;
  }
  UNLOCK();
  local_200 = pIVar7;
  if (bVar33) {
LAB_1534000a2:
    *(undefined4 *)&param_1[0x38].lpVtbl = 1;
  }
  else if (pIVar7->lpVtbl == pvVar11) {
    *(int *)&param_1[0x38].lpVtbl = *(int *)&param_1[0x38].lpVtbl + 1;
  }
  else {
    while( true ) {
      pvVar11 = Self;
      LOCK();
      bVar33 = pIVar7->lpVtbl == (IUnknownVtbl *)0x0;
      if (bVar33) {
        pIVar7->lpVtbl = Self;
      }
      UNLOCK();
      if (bVar33) goto LAB_1534000a2;
      if (pIVar7->lpVtbl == pvVar11) break;
      Sleep(0);
    }
    *(int *)&param_1[0x38].lpVtbl = *(int *)&param_1[0x38].lpVtbl + 1;
  }
  pvVar11 = Self;
  pIVar28 = param_1 + 0x38;
  pIVar3 = param_1 + 0x14;
  LOCK();
  bVar33 = pIVar3->lpVtbl == (IUnknownVtbl *)0x0;
  if (bVar33) {
    pIVar3->lpVtbl = Self;
  }
  UNLOCK();
  local_1f8 = pIVar3;
  if (bVar33) {
    *(undefined4 *)&param_1[0x15].lpVtbl = 1;
  }
  else if (pIVar3->lpVtbl == pvVar11) {
    *(int *)&param_1[0x15].lpVtbl = *(int *)&param_1[0x15].lpVtbl + 1;
  }
  else {
    while( true ) {
      pvVar11 = Self;
      LOCK();
      bVar33 = pIVar3->lpVtbl == (IUnknownVtbl *)0x0;
      if (bVar33) {
        pIVar3->lpVtbl = Self;
      }
      UNLOCK();
      if (bVar33) {
        *(undefined4 *)&param_1[0x15].lpVtbl = 1;
        goto LAB_153400127;
      }
      if (pIVar3->lpVtbl == pvVar11) break;
      Sleep(0);
    }
    *(int *)&param_1[0x15].lpVtbl = *(int *)&param_1[0x15].lpVtbl + 1;
  }
LAB_153400127:
  pIVar27 = param_1 + 0x15;
  riid = (IID *)0x0;
  (*param_1->lpVtbl[0x1d].Release)(param_1);
  if (((byte)param_2 & 0x30) == 0x30) {
    *(int *)&pIVar27->lpVtbl = *(int *)&pIVar27->lpVtbl + -1;
    if (*(int *)&pIVar27->lpVtbl == 0) {
      pIVar3->lpVtbl = (IUnknownVtbl *)0x0;
    }
    *(int *)&pIVar28->lpVtbl = *(int *)&pIVar28->lpVtbl + -1;
    if (*(int *)&pIVar28->lpVtbl == 0) {
      pIVar7->lpVtbl = (IUnknownVtbl *)0x0;
    }
    return 0x80070057;
  }
  if (*(int *)((longlong)&param_1[0x3c].lpVtbl + 4) == 0) {
    *(int *)&pIVar27->lpVtbl = *(int *)&pIVar27->lpVtbl + -1;
    if (*(int *)&pIVar27->lpVtbl == 0) {
      pIVar3->lpVtbl = (IUnknownVtbl *)0x0;
    }
    *(int *)&pIVar28->lpVtbl = *(int *)&pIVar28->lpVtbl + -1;
    if (*(int *)&pIVar28->lpVtbl == 0) {
      pIVar7->lpVtbl = (IUnknownVtbl *)0x0;
    }
    return 0;
  }
  if ((param_2 & 0x600) != 0) {
    if (((param_2 >> 9 & 1) != 0) &&
       (HVar13 = (*param_1->lpVtbl[0x1a].QueryInterface)(param_1,riid,ppvVar29), HVar13 < 0)) {
      _com_issue_errorex(HVar13,param_1,(_GUID *)&DAT_1535daa68);
    }
    FUN_153407150(param_1 + 0x45);
    if ((param_2 & 0x40) == 0) {
      *(int *)&pIVar27->lpVtbl = *(int *)&pIVar27->lpVtbl + -1;
      if (*(int *)&pIVar27->lpVtbl == 0) {
        pIVar3->lpVtbl = (IUnknownVtbl *)0x0;
      }
      *(int *)&pIVar28->lpVtbl = *(int *)&pIVar28->lpVtbl + -1;
      if (*(int *)&pIVar28->lpVtbl == 0) {
        pIVar7->lpVtbl = (IUnknownVtbl *)0x0;
      }
      return 0;
    }
    local_388 = 0;
    UVar14 = (*param_1->lpVtbl[0x1c].AddRef)(param_1);
    if ((int)UVar14 < 0) {
      _com_issue_errorex(UVar14,param_1,(_GUID *)&DAT_1535daa68);
    }
    local_2c0._8_4_ = local_388 + -1;
    local_2c0.n2.vt = 3;
    local_1c8 = CONCAT62(local_2c0._2_6_,3);
    uStack_1c0 = CONCAT44(local_2c0._12_4_,local_2c0._8_4_);
    local_1b8 = local_2c0._16_8_;
    UVar14 = (*param_1->lpVtbl[0x1a].AddRef)(param_1);
    if ((int)UVar14 < 0) {
      _com_issue_errorex(UVar14,param_1,(_GUID *)&DAT_1535daa68);
    }
    if (local_2c0.n2.vt == 8) {
      local_2c0.n2.vt = 0;
      if (CONCAT44(local_2c0._12_4_,local_2c0._8_4_) != 0) {
        (*DAT_153775ef0)(CONCAT44(local_2c0._12_4_,local_2c0._8_4_) + -4);
      }
    }
    else {
      VariantClear((VARIANTARG *)&local_2c0.n2);
    }
  }
  pIVar26 = (IUnknownVtbl *)0x0;
  uVar21 = param_2 >> 6;
  if ((uVar21 & 1) != 0) {
    FUN_153359ee0(param_1 + 0x3c,param_1 + 0x3c,0,param_1[0x3d].lpVtbl,0);
  }
  iVar12 = FUN_153406b20(param_4,&local_384);
  if ((((*(int *)((longlong)&param_1[0x45].lpVtbl + 4) != 0) &&
       (*(int *)((longlong)&param_1[0x47].lpVtbl[1].QueryInterface + 4) != -1)) &&
      ((param_2 >> 8 & 1) == 0)) && (local_384 != 0)) {
    local_1f0 = 6;
    lStack_1e8 = (longlong)*(int *)((longlong)&param_1[0x3f].lpVtbl + 4) << 0x20;
    local_1a8 = CONCAT62(uStack_1ee,6);
    local_198 = local_1e0;
    lStack_1a0 = lStack_1e8;
    (*param_1->lpVtbl[0x1a].AddRef)(param_1);
  }
  if ((local_384 == 2) && (iVar12 < 0)) {
    if (param_2 == 0) {
      FUN_153407150(param_1 + 0x45);
    }
    *(int *)&pIVar27->lpVtbl = *(int *)&pIVar27->lpVtbl + -1;
    if (*(int *)&pIVar27->lpVtbl == 0) {
      pIVar3->lpVtbl = (IUnknownVtbl *)0x0;
    }
    *(int *)&pIVar28->lpVtbl = *(int *)&pIVar28->lpVtbl + -1;
    if (*(int *)&pIVar28->lpVtbl == 0) {
      pIVar7->lpVtbl = (IUnknownVtbl *)0x0;
    }
    return 0;
  }
  local_390 = (_func_5157 *)0x0;
  if ((uVar21 & 1) == 0) {
    pIVar24 = param_1[0x3e].lpVtbl;
  }
  else {
    pIVar24 = param_1[0x3d].lpVtbl;
  }
  local_3b0 = pIVar24->AddRef;
  if (local_3b0 != (_func_5157 *)0x0) {
    LOCK();
    *(longlong *)(local_3b0 + -0x20) = *(longlong *)(local_3b0 + -0x20) + 1;
    UNLOCK();
  }
  local_2d0 = local_3b0;
  iVar12 = FUN_1533fb1b0(param_1 + -2,param_4,local_398);
  if (iVar12 != 1) {
    if (iVar12 != 2) {
      uVar15 = FUN_1534069b0();
      if (local_3b0 != (_func_5157 *)0x0) {
        pcVar1 = local_3b0 + -0x28;
        LOCK();
        pcVar22 = local_3b0 + -0x20;
        lVar18 = *(longlong *)pcVar22;
        *(longlong *)pcVar22 = *(longlong *)pcVar22 + -1;
        UNLOCK();
        if ((int)lVar18 == 1) {
          if (*(longlong *)(local_3b0 + -0x10) != 0) {
            LOCK();
            *(undefined8 *)(*(longlong *)(local_3b0 + -0x10) + 8) = 0;
            UNLOCK();
            do {
            } while (*(int *)(*(longlong *)(local_3b0 + -0x10) + 4) != 0);
          }
          if (pcVar1 != (code *)0x0) {
            (*(code *)**(undefined8 **)pcVar1)(pcVar1,1);
          }
        }
      }
      if (local_390 != (_func_5157 *)0x0) {
        pcVar22 = local_390 + -0x28;
        LOCK();
        pcVar1 = local_390 + -0x20;
        lVar18 = *(longlong *)pcVar1;
        *(longlong *)pcVar1 = *(longlong *)pcVar1 + -1;
        UNLOCK();
        if ((int)lVar18 == 1) {
          if (*(longlong *)(local_390 + -0x10) != 0) {
            LOCK();
            *(undefined8 *)(*(longlong *)(local_390 + -0x10) + 8) = 0;
            UNLOCK();
            do {
            } while (*(int *)(*(longlong *)(local_390 + -0x10) + 4) != 0);
          }
          if (pcVar22 != (code *)0x0) {
            (*(code *)**(undefined8 **)pcVar22)(pcVar22,1);
          }
        }
      }
      *(int *)&pIVar27->lpVtbl = *(int *)&pIVar27->lpVtbl + -1;
      if (*(int *)&pIVar27->lpVtbl == 0) {
        pIVar3->lpVtbl = (IUnknownVtbl *)0x0;
      }
      *(int *)&pIVar28->lpVtbl = *(int *)&pIVar28->lpVtbl + -1;
      if (*(int *)&pIVar28->lpVtbl != 0) {
        return uVar15;
      }
      pIVar7->lpVtbl = (IUnknownVtbl *)0x0;
      return uVar15;
    }
    if ((uVar21 & 1) == 0) {
      pIVar24 = param_1[0x3d].lpVtbl;
    }
    else {
      pIVar24 = param_1[0x3e].lpVtbl;
    }
    if (pIVar24->AddRef != (_func_5157 *)0x0) {
      LOCK();
      pcVar1 = pIVar24->AddRef + -0x20;
      *(longlong *)pcVar1 = *(longlong *)pcVar1 + 1;
      UNLOCK();
      local_3b0 = local_2d0;
    }
    if (local_390 != (_func_5157 *)0x0) {
      pcVar22 = local_390 + -0x28;
      LOCK();
      pcVar1 = local_390 + -0x20;
      lVar18 = *(longlong *)pcVar1;
      *(longlong *)pcVar1 = *(longlong *)pcVar1 + -1;
      UNLOCK();
      if ((int)lVar18 == 1) {
        if (*(longlong *)(local_390 + -0x10) != 0) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_390 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_390 + -0x10) + 4) != 0);
        }
        if (pcVar22 != (code *)0x0) {
          (*(code *)**(undefined8 **)pcVar22)(pcVar22,1);
          local_3b0 = local_2d0;
          goto LAB_1534005a7;
        }
      }
      local_3b0 = local_2d0;
    }
LAB_1534005a7:
    local_390 = pIVar24->AddRef;
  }
  if (param_2 == 0x200) {
    FUN_153407150(param_1 + 0x45);
    if (local_3b0 != (_func_5157 *)0x0) {
      pcVar1 = local_3b0 + -0x28;
      LOCK();
      pcVar22 = local_3b0 + -0x20;
      lVar18 = *(longlong *)pcVar22;
      *(longlong *)pcVar22 = *(longlong *)pcVar22 + -1;
      UNLOCK();
      if ((int)lVar18 == 1) {
        if (*(longlong *)(local_2d0 + -0x10) != 0) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_2d0 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_2d0 + -0x10) + 4) != 0);
        }
        if (pcVar1 != (code *)0x0) {
          (*(code *)**(undefined8 **)pcVar1)(pcVar1,1);
        }
      }
    }
    if (local_390 != (_func_5157 *)0x0) {
      pcVar1 = local_390 + -0x28;
      LOCK();
      pcVar22 = local_390 + -0x20;
      lVar18 = *(longlong *)pcVar22;
      *(longlong *)pcVar22 = *(longlong *)pcVar22 + -1;
      UNLOCK();
      if ((int)lVar18 == 1) {
        if (*(longlong *)(local_390 + -0x10) != 0) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_390 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_390 + -0x10) + 4) != 0);
        }
        if (pcVar1 != (code *)0x0) {
          (*(code *)**(undefined8 **)pcVar1)(pcVar1,1);
        }
      }
    }
    *(int *)&pIVar27->lpVtbl = *(int *)&pIVar27->lpVtbl + -1;
    if (*(int *)&pIVar27->lpVtbl == 0) {
      pIVar3->lpVtbl = (IUnknownVtbl *)0x0;
    }
    *(int *)&pIVar28->lpVtbl = *(int *)&pIVar28->lpVtbl + -1;
    if (*(int *)&pIVar28->lpVtbl == 0) {
      pIVar7->lpVtbl = (IUnknownVtbl *)0x0;
    }
    return 0;
  }
  if ((param_2 >> 8 & 1) == 0) {
    FUN_153407150(param_1 + 0x45);
  }
  if ((*(int *)((longlong)&param_1[0x45].lpVtbl + 4) != 0) &&
     (*(int *)((longlong)&param_1[0x47].lpVtbl[1].QueryInterface + 4) != -1)) {
    if (local_3b0 != (_func_5157 *)0x0) {
      pcVar1 = local_3b0 + -0x28;
      LOCK();
      pcVar22 = local_3b0 + -0x20;
      lVar18 = *(longlong *)pcVar22;
      *(longlong *)pcVar22 = *(longlong *)pcVar22 + -1;
      UNLOCK();
      if ((int)lVar18 == 1) {
        if (*(longlong *)(local_2d0 + -0x10) != 0) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_2d0 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_2d0 + -0x10) + 4) != 0);
        }
        if (pcVar1 != (code *)0x0) {
          (*(code *)**(undefined8 **)pcVar1)(pcVar1,1);
        }
      }
    }
    if (local_390 != (_func_5157 *)0x0) {
      pcVar1 = local_390 + -0x28;
      LOCK();
      pcVar22 = local_390 + -0x20;
      lVar18 = *(longlong *)pcVar22;
      *(longlong *)pcVar22 = *(longlong *)pcVar22 + -1;
      UNLOCK();
      if ((int)lVar18 == 1) {
        if (*(longlong *)(local_390 + -0x10) != 0) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_390 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_390 + -0x10) + 4) != 0);
        }
        if (pcVar1 != (code *)0x0) {
          (*(code *)**(undefined8 **)pcVar1)(pcVar1,1);
        }
      }
    }
    *(int *)&pIVar27->lpVtbl = *(int *)&pIVar27->lpVtbl + -1;
    if (*(int *)&pIVar27->lpVtbl == 0) {
      pIVar3->lpVtbl = (IUnknownVtbl *)0x0;
    }
    *(int *)&pIVar28->lpVtbl = *(int *)&pIVar28->lpVtbl + -1;
    if (*(int *)&pIVar28->lpVtbl == 0) {
      pIVar7->lpVtbl = (IUnknownVtbl *)0x0;
    }
    return 0;
  }
  uVar30 = 0;
  if ((param_2 & 0x20) != 0) {
    if ((uVar21 & 1) == 0) {
      pIVar24 = param_1[0x3d].lpVtbl;
    }
    else {
      pIVar24 = param_1[0x3e].lpVtbl;
    }
    while (pIVar24 != (IUnknownVtbl *)0x0) {
      pp_Var2 = &pIVar24->AddRef;
      p_Var6 = *pp_Var2;
      pIVar31 = pIVar26;
      if (local_390 == p_Var6) goto LAB_1534008b0;
      if ((uVar21 & 1) == 0) {
        p_Var25 = (_func_5156 *)pIVar24[-2].Release;
      }
      else {
        p_Var25 = pIVar24[-1].QueryInterface;
      }
      pIVar24 = pIVar26;
      if (p_Var25 != (_func_5156 *)0x0) {
        pIVar24 = (IUnknownVtbl *)(p_Var25 + 0x28);
      }
      if (p_Var6 != (_func_5157 *)0x0) {
        LOCK();
        *(longlong *)(p_Var6 + -0x20) = *(longlong *)(p_Var6 + -0x20) + 1;
        UNLOCK();
        local_3b0 = local_2d0;
      }
      if (local_3b0 != (_func_5157 *)0x0) {
        pcVar1 = local_3b0 + -0x28;
        LOCK();
        pcVar22 = local_3b0 + -0x20;
        lVar18 = *(longlong *)pcVar22;
        *(longlong *)pcVar22 = *(longlong *)pcVar22 + -1;
        UNLOCK();
        if ((int)lVar18 == 1) {
          if (*(longlong *)(local_2d0 + -0x10) != 0) {
            LOCK();
            *(undefined8 *)(*(longlong *)(local_2d0 + -0x10) + 8) = 0;
            UNLOCK();
            do {
            } while (*(int *)(*(longlong *)(local_2d0 + -0x10) + 4) != 0);
          }
          if (pcVar1 != (code *)0x0) {
            (*(code *)**(undefined8 **)pcVar1)(pcVar1,1);
          }
        }
      }
      local_3b0 = *pp_Var2;
      local_2d0 = local_3b0;
    }
    goto LAB_1534008ef;
  }
  local_378 = -1;
  lVar18 = FUN_153406ab0(param_1 + 0x39,(undefined1 *)((longlong)&param_1[0x3f].lpVtbl + 4));
  FUN_153359ee0(param_1 + 0x3c,param_1 + 0x3c,0,*(undefined8 *)(*(longlong *)(lVar18 + 8) + 8),0);
LAB_153400918:
  bVar33 = false;
  iVar12 = *(int *)((longlong)&param_1[0x45].lpVtbl + 4);
  if ((*(char *)((longlong)&param_1[0x22].lpVtbl + 1) == '\0') || (iVar12 != 0)) {
    *(undefined4 *)((longlong)&param_1[0x1f].lpVtbl + 4) = 0;
    if (iVar12 == 0) goto LAB_153400962;
    iVar16 = *(int *)&param_1[0x47].lpVtbl[1].QueryInterface;
  }
  else {
    uVar15 = FUN_153327520(param_1[0x17].lpVtbl);
    *(undefined4 *)((longlong)&param_1[0x22].lpVtbl + 4) = uVar15;
    *(undefined4 *)((longlong)&param_1[0x1f].lpVtbl + 4) = 0;
LAB_153400962:
    iVar16 = FUN_153327510(param_1[0x17].lpVtbl);
  }
  if (*(char *)((longlong)&param_1[0x22].lpVtbl + 1) == '\0') {
    local_37c = 0;
  }
  else {
    iVar17 = FUN_153327510(param_1[0x17].lpVtbl);
    local_37c = FUN_153327520(param_1[0x17].lpVtbl);
    local_37c = iVar17 - local_37c;
  }
  fVar10 = DAT_1536f719c;
  fVar9 = DAT_1536f7120;
  if ((uVar21 & 1) == 0) {
    pIVar24 = param_1[0x3d].lpVtbl;
  }
  else {
    pIVar24 = param_1[0x3e].lpVtbl;
  }
  while (pIVar24 != (IUnknownVtbl *)0x0) {
    if ((uVar21 & 1) == 0) {
      p_Var25 = (_func_5156 *)pIVar24[-2].Release;
    }
    else {
      p_Var25 = pIVar24[-1].QueryInterface;
    }
    pIVar31 = pIVar26;
    if (p_Var25 != (_func_5156 *)0x0) {
      pIVar31 = (IUnknownVtbl *)(p_Var25 + 0x28);
    }
    p_Var6 = pIVar24->AddRef;
    if (local_390 == p_Var6) {
      bVar33 = true;
    }
    iVar16 = iVar16 + (uint)(*(int *)(p_Var6 + 0x14) * local_380) / 1000;
    piVar4 = (int *)((longlong)&param_1[0x1f].lpVtbl + 4);
    *piVar4 = *piVar4 + *(int *)(p_Var6 + 0x14);
    ppvVar29 = (void **)0x0;
    local_2c8 = pIVar31;
    pIVar19 = (IUnknownVtbl *)FUN_153407ae0(param_1 + 0x45,param_1[0x47].lpVtbl);
    if (param_1[0x47].lpVtbl == (IUnknownVtbl *)0x0) {
      param_1[0x46].lpVtbl = pIVar19;
    }
    else {
      pIVar24 = pIVar26;
      if (pIVar19 != (IUnknownVtbl *)0x0) {
        pIVar24 = (IUnknownVtbl *)&pIVar19[-2].AddRef;
      }
      param_1[0x47].lpVtbl[-2].Release = (_func_5158 *)pIVar24;
      pIVar19 = pIVar26;
      if (pIVar24 != (IUnknownVtbl *)0x0) {
        pIVar19 = (IUnknownVtbl *)&pIVar24[1].Release;
      }
    }
    param_1[0x47].lpVtbl = pIVar19;
    *(undefined2 *)&pIVar19->QueryInterface = 6;
    if (pIVar31 == (IUnknownVtbl *)0x0) {
      uVar30 = *(uint *)(local_3b0 + 0x10);
    }
    else {
      uVar30 = *(uint *)(p_Var6 + 0x10);
    }
    if ((uVar21 & 1) != 0) {
      uVar5 = *(uint *)((longlong)&param_1[0x3c].lpVtbl + 4);
      uVar30 = (uint)((ulonglong)(uVar30 - 2) % (ulonglong)uVar5);
      if ((int)uVar30 < 1) {
        uVar30 = uVar5;
      }
    }
    pIVar19->AddRef = (_func_5157 *)((longlong)(int)uVar30 << 0x20);
    *(int *)&pIVar19[1].QueryInterface = iVar16;
    *(int *)&pIVar19[2].QueryInterface = local_37c;
    iVar17 = -1;
    if (bVar33) {
      iVar17 = local_378;
    }
    *(int *)((longlong)&pIVar19[1].QueryInterface + 4) = iVar17;
    iVar17 = *(int *)(p_Var6 + 0x18);
    iVar32 = *(int *)(p_Var6 + 0x1c);
    if (iVar17 != -1) {
      pIVar24 = param_1[0x1a].lpVtbl;
      if (pIVar24 == (IUnknownVtbl *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      local_374 = 0;
      UVar14 = (*((IUnknownVtbl *)pIVar24->QueryInterface)[2].Release)((IUnknown *)pIVar24);
      if ((int)UVar14 < 0) {
        ppvVar29 = (void **)&DAT_1535daa58;
        _com_issue_errorex(UVar14,(IUnknown *)pIVar24,(_GUID *)&DAT_1535daa58);
      }
      iVar17 = (int)((float)(local_374 * *(int *)(p_Var6 + 0x18)) / fVar10 + fVar9);
    }
    if (iVar32 != -1) {
      pIVar24 = param_1[0x1a].lpVtbl;
      if (pIVar24 == (IUnknownVtbl *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      local_370 = 0;
      UVar14 = (*((IUnknownVtbl *)pIVar24->QueryInterface)[2].Release)((IUnknown *)pIVar24);
      if ((int)UVar14 < 0) {
        ppvVar29 = (void **)&DAT_1535daa58;
        _com_issue_errorex(UVar14,(IUnknown *)pIVar24,(_GUID *)&DAT_1535daa58);
      }
      iVar32 = (int)((float)(local_370 * *(int *)(p_Var6 + 0x1c)) / fVar10 + fVar9);
    }
    *(int *)&pIVar19[1].AddRef = iVar17;
    *(int *)((longlong)&pIVar19[1].AddRef + 4) = iVar32;
    *(undefined4 *)&pIVar19[1].Release = *(undefined4 *)(p_Var6 + 0x20);
    *(undefined4 *)((longlong)&pIVar19[1].Release + 4) = *(undefined4 *)(p_Var6 + 0x24);
    pIVar7 = *(IUnknown **)p_Var6;
    if (pIVar7 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_153513900(0x80004003);
    }
    local_36c._0_4_ = 0;
    HVar13 = (*pIVar7->lpVtbl[0x14].QueryInterface)(pIVar7,(IID *)local_36c,ppvVar29);
    if (HVar13 < 0) {
      ppvVar29 = (void **)&DAT_1535d2948;
      _com_issue_errorex(HVar13,pIVar7,(_GUID *)&DAT_1535d2948);
    }
    *(bool *)((longlong)&pIVar19[2].QueryInterface + 4) = local_36c._0_4_ != 0;
    pIVar24 = local_2c8;
    if (local_36c._0_4_ != 0) {
      pIVar7 = *(IUnknown **)p_Var6;
      if (pIVar7 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      local_36c._4_2_ = 0;
      local_36c._6_2_ = 0;
      UVar14 = (*pIVar7->lpVtbl[0x14].AddRef)(pIVar7);
      if ((int)UVar14 < 0) {
        ppvVar29 = (void **)&DAT_1535d2948;
        _com_issue_errorex(UVar14,pIVar7,(_GUID *)&DAT_1535d2948);
      }
      *(undefined4 *)&pIVar19[2].AddRef = local_36c._4_4_;
      pIVar7 = *(IUnknown **)p_Var6;
      if (pIVar7 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      local_36c[8] = '\0';
      local_36c[9] = '\0';
      local_36c[10] = '\0';
      local_36c[0xb] = '\0';
      UVar14 = (*pIVar7->lpVtbl[0x14].Release)(pIVar7);
      if ((int)UVar14 < 0) {
        ppvVar29 = (void **)&DAT_1535d2948;
        _com_issue_errorex(UVar14,pIVar7,(_GUID *)&DAT_1535d2948);
      }
      *(undefined4 *)((longlong)&pIVar19[2].AddRef + 4) = local_36c._8_4_;
      pIVar7 = *(IUnknown **)p_Var6;
      if (pIVar7 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      local_36c[0xc] = '\0';
      local_36c[0xd] = '\0';
      local_36c[0xe] = '\0';
      local_36c[0xf] = '\0';
      HVar13 = (*pIVar7->lpVtbl[0x15].QueryInterface)(pIVar7,(IID *)(local_36c + 0xc),ppvVar29);
      if (HVar13 < 0) {
        ppvVar29 = (void **)&DAT_1535d2948;
        _com_issue_errorex(HVar13,pIVar7,(_GUID *)&DAT_1535d2948);
      }
      *(undefined4 *)&pIVar19[2].Release = local_36c._12_4_;
      pIVar7 = *(IUnknown **)p_Var6;
      if (pIVar7 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      local_35c = 0;
      UVar14 = (*pIVar7->lpVtbl[0x15].AddRef)(pIVar7);
      if ((int)UVar14 < 0) {
        ppvVar29 = (void **)&DAT_1535d2948;
        _com_issue_errorex(UVar14,pIVar7,(_GUID *)&DAT_1535d2948);
      }
      *(undefined4 *)((longlong)&pIVar19[2].Release + 4) = local_35c;
      pIVar7 = *(IUnknown **)p_Var6;
      if (pIVar7 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      local_358 = 0;
      UVar14 = (*pIVar7->lpVtbl[0x15].Release)(pIVar7);
      if ((int)UVar14 < 0) {
        ppvVar29 = (void **)&DAT_1535d2948;
        _com_issue_errorex(UVar14,pIVar7,(_GUID *)&DAT_1535d2948);
      }
      *(undefined4 *)&pIVar19[3].QueryInterface = local_358;
      pIVar7 = *(IUnknown **)p_Var6;
      if (pIVar7 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      local_354._0_4_ = 0;
      HVar13 = (*pIVar7->lpVtbl[0x16].QueryInterface)(pIVar7,(IID *)local_354,ppvVar29);
      if (HVar13 < 0) {
        _com_issue_errorex(HVar13,pIVar7,(_GUID *)&DAT_1535d2948);
      }
      *(undefined4 *)((longlong)&pIVar19[3].QueryInterface + 4) = local_354._0_4_;
      pIVar24 = local_2c8;
    }
  }
  if ((param_2 & 0x10) != 0) {
    puVar20 = (undefined8 *)FUN_153406800(param_1 + 0x45);
    pIVar24 = param_1[0x46].lpVtbl;
    p_Var6 = pIVar24->AddRef;
    *puVar20 = pIVar24->QueryInterface;
    puVar20[1] = p_Var6;
    p_Var25 = pIVar24[1].QueryInterface;
    puVar20[2] = pIVar24->Release;
    puVar20[3] = p_Var25;
    p_Var8 = pIVar24[1].Release;
    puVar20[4] = pIVar24[1].AddRef;
    puVar20[5] = p_Var8;
    p_Var6 = pIVar24[2].AddRef;
    puVar20[6] = pIVar24[2].QueryInterface;
    puVar20[7] = p_Var6;
    p_Var25 = pIVar24[3].QueryInterface;
    puVar20[8] = pIVar24[2].Release;
    puVar20[9] = p_Var25;
    *(undefined4 *)((longlong)puVar20 + 0x24) = 0xffffffff;
    *(undefined4 *)((longlong)puVar20 + 0x2c) = 0;
  }
  if ((iVar12 == 0) && (*(int *)((longlong)&param_1[0x45].lpVtbl + 4) != 0)) {
    pIVar24 = param_1[0x46].lpVtbl;
    if (-1 < *(int *)&pIVar24[1].AddRef) {
      pIVar31 = param_1[0x1a].lpVtbl;
      if (pIVar31 == (IUnknownVtbl *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      VariantInit((VARIANTARG *)&local_2f0.n2);
      if (DAT_153770dc8 == 8) {
        if (local_2f0.n2.vt == 8) {
          local_2f0.n2.vt = 0;
          if (local_2f0.decVal.u2.Lo64 != 0) {
            (*DAT_153775ef0)((longlong)local_2f0.decVal.u2 - 4);
          }
        }
        else {
          HVar13 = VariantClear((VARIANTARG *)&local_2f0.n2);
          if (HVar13 < 0) goto LAB_153401a67;
        }
        local_2f0.n2.vt = 8;
        pIVar19 = pIVar26;
        if (DAT_153770dd0 != 0) {
          pIVar19 = (IUnknownVtbl *)(ulonglong)(*(uint *)(DAT_153770dd0 + -4) >> 1);
        }
        local_2f0.decVal.u2 = (_union_1719)FUN_1532d8730(DAT_153770dd0,pIVar19);
      }
      else {
        if ((local_2f0.n2.vt == 8) && (local_2f0.n2.vt = 0, local_2f0.decVal.u2.Lo64 != 0)) {
          (*DAT_153775ef0)((longlong)local_2f0.decVal.u2 - 4);
        }
        HVar13 = VariantCopy((VARIANTARG *)&local_2f0.n2,(VARIANTARG *)&DAT_153770dc8);
        if (HVar13 < 0) {
LAB_153401a67:
                    /* WARNING: Subroutine does not return */
          FUN_153513900(HVar13);
        }
      }
      VariantInit((VARIANTARG *)&local_338.n2);
      if (DAT_153770dc8 == 8) {
        if (local_338.n2.vt == 8) {
          local_338.n2.vt = 0;
          if (local_338.decVal.u2.Lo64 != 0) {
            (*DAT_153775ef0)((longlong)local_338.decVal.u2 - 4);
          }
        }
        else {
          HVar13 = VariantClear((VARIANTARG *)&local_338.n2);
          if (HVar13 < 0) goto LAB_153401a6f;
        }
        local_338.n2.vt = 8;
        pIVar19 = pIVar26;
        if (DAT_153770dd0 != 0) {
          pIVar19 = (IUnknownVtbl *)(ulonglong)(*(uint *)(DAT_153770dd0 + -4) >> 1);
        }
        local_338.decVal.u2 = (_union_1719)FUN_1532d8730(DAT_153770dd0,pIVar19);
      }
      else {
        if ((local_338.n2.vt == 8) && (local_338.n2.vt = 0, local_338.decVal.u2.Lo64 != 0)) {
          (*DAT_153775ef0)((longlong)local_338.decVal.u2 - 4);
        }
        HVar13 = VariantCopy((VARIANTARG *)&local_338.n2,(VARIANTARG *)&DAT_153770dc8);
        if (HVar13 < 0) {
LAB_153401a6f:
                    /* WARNING: Subroutine does not return */
          FUN_153513900(HVar13);
        }
      }
      VariantInit((VARIANTARG *)&local_320.n2);
      if (DAT_153770dc8 == 8) {
        if (local_320.n2.vt == 8) {
          local_320.n2.vt = 0;
          if (local_320.decVal.u2.Lo64 != 0) {
            (*DAT_153775ef0)((longlong)local_320.decVal.u2 - 4);
          }
        }
        else {
          HVar13 = VariantClear((VARIANTARG *)&local_320.n2);
          if (HVar13 < 0) goto LAB_153401a77;
        }
        local_320.n2.vt = 8;
        pIVar19 = pIVar26;
        if (DAT_153770dd0 != 0) {
          pIVar19 = (IUnknownVtbl *)(ulonglong)(*(uint *)(DAT_153770dd0 + -4) >> 1);
        }
        local_320.decVal.u2 = (_union_1719)FUN_1532d8730(DAT_153770dd0,pIVar19);
      }
      else {
        if ((local_320.n2.vt == 8) && (local_320.n2.vt = 0, local_320.decVal.u2.Lo64 != 0)) {
          (*DAT_153775ef0)((longlong)local_320.decVal.u2 - 4);
        }
        HVar13 = VariantCopy((VARIANTARG *)&local_320.n2,(VARIANTARG *)&DAT_153770dc8);
        if (HVar13 < 0) {
LAB_153401a77:
                    /* WARNING: Subroutine does not return */
          FUN_153513900(HVar13);
        }
      }
      VariantInit((VARIANTARG *)&local_308.n2);
      if (DAT_153770dc8 == 8) {
        if (local_308.n2.vt == 8) {
          local_308.n2.vt = 0;
          if (local_308.decVal.u2.Lo64 != 0) {
            (*DAT_153775ef0)((longlong)local_308.decVal.u2 - 4);
          }
        }
        else {
          HVar13 = VariantClear((VARIANTARG *)&local_308.n2);
          if (HVar13 < 0) goto LAB_153401a7f;
        }
        local_308.n2.vt = 8;
        pIVar19 = pIVar26;
        if (DAT_153770dd0 != 0) {
          pIVar19 = (IUnknownVtbl *)(ulonglong)(*(uint *)(DAT_153770dd0 + -4) >> 1);
        }
        local_308.decVal.u2 = (_union_1719)FUN_1532d8730(DAT_153770dd0,pIVar19);
      }
      else {
        if ((local_308.n2.vt == 8) && (local_308.n2.vt = 0, local_308.decVal.u2.Lo64 != 0)) {
          (*DAT_153775ef0)((longlong)local_308.decVal.u2 - 4);
        }
        HVar13 = VariantCopy((VARIANTARG *)&local_308.n2,(VARIANTARG *)&DAT_153770dc8);
        if (HVar13 < 0) {
LAB_153401a7f:
                    /* WARNING: Subroutine does not return */
          FUN_153513900(HVar13);
        }
      }
      VariantInit((VARIANTARG *)(local_354 + 4));
      if (DAT_153770dc8 == 8) {
        if (local_354._4_2_ == 8) {
          local_354._4_2_ = 0;
          if (lStack_348 != 0) {
            (*DAT_153775ef0)(lStack_348 + -4);
          }
        }
        else {
          HVar13 = VariantClear((VARIANTARG *)(local_354 + 4));
          if (HVar13 < 0) goto LAB_153401a87;
        }
        local_354._4_2_ = 8;
        if (DAT_153770dd0 != 0) {
          pIVar26 = (IUnknownVtbl *)(ulonglong)(*(uint *)(DAT_153770dd0 + -4) >> 1);
        }
        lStack_348 = FUN_1532d8730(DAT_153770dd0,pIVar26);
      }
      else {
        if ((local_354._4_2_ == 8) && (local_354._4_2_ = 0, lStack_348 != 0)) {
          (*DAT_153775ef0)(lStack_348 + -4);
        }
        HVar13 = VariantCopy((VARIANTARG *)(local_354 + 4),(VARIANTARG *)&DAT_153770dc8);
        if (HVar13 < 0) {
LAB_153401a87:
                    /* WARNING: Subroutine does not return */
          FUN_153513900(HVar13);
        }
      }
      local_188 = CONCAT62(local_2f0._2_6_,local_2f0.n2.vt);
      _Stack_180 = local_2f0.decVal.u2;
      local_178 = local_2f0._16_8_;
      local_168 = CONCAT62(local_338._2_6_,local_338.n2.vt);
      _Stack_160 = local_338.decVal.u2;
      local_158 = local_338._16_8_;
      local_148 = CONCAT62(local_320._2_6_,local_320.n2.vt);
      _Stack_140.Lo64 = (ULONGLONG)local_320.decVal.u2;
      local_138 = local_320._16_8_;
      local_128 = CONCAT62(local_308._2_6_,local_308.n2.vt);
      _Stack_120.Lo64 = (ULONGLONG)local_308.decVal.u2;
      local_118 = local_308._16_8_;
      local_108 = CONCAT62(local_354._6_6_,local_354._4_2_);
      lStack_100 = lStack_348;
      local_f8 = local_340;
      UVar14 = (*((IUnknownVtbl *)pIVar31->QueryInterface)[0xd].AddRef)((IUnknown *)pIVar31);
      if ((int)UVar14 < 0) {
        _com_issue_errorex(UVar14,(IUnknown *)pIVar31,(_GUID *)&DAT_15364c348);
      }
      if (local_354._4_2_ == 8) {
        local_354._4_2_ = 0;
        if (lStack_348 != 0) {
          (*DAT_153775ef0)(lStack_348 + -4);
        }
      }
      else {
        VariantClear((VARIANTARG *)(local_354 + 4));
      }
      if (local_308.n2.vt == 8) {
        local_308.n2.vt = 0;
        if (local_308.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_308.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_308.n2);
      }
      if (local_320.n2.vt == 8) {
        local_320.n2.vt = 0;
        if (local_320.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_320.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_320.n2);
      }
      if (local_338.n2.vt == 8) {
        local_338.n2.vt = 0;
        if (local_338.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_338.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_338.n2);
      }
      if (local_2f0.n2.vt == 8) {
        local_2f0.n2.vt = 0;
        if (local_2f0.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_2f0.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_2f0.n2);
      }
    }
    if (-1 < *(int *)((longlong)&pIVar24[1].AddRef + 4)) {
      pIVar26 = param_1[0x1a].lpVtbl;
      if (pIVar26 == (IUnknownVtbl *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(0x80004003);
      }
      VariantInit((VARIANTARG *)&local_248.n2);
      iVar12 = FUN_153315860(&local_248,&DAT_153770dc8);
      if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(iVar12);
      }
      VariantInit((VARIANTARG *)&local_260.n2);
      iVar12 = FUN_153315860(&local_260,&DAT_153770dc8);
      if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(iVar12);
      }
      VariantInit((VARIANTARG *)&local_278.n2);
      iVar12 = FUN_153315860(&local_278,&DAT_153770dc8);
      if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(iVar12);
      }
      VariantInit((VARIANTARG *)&local_290.n2);
      iVar12 = FUN_153315860(&local_290,&DAT_153770dc8);
      if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(iVar12);
      }
      local_2a8._8_4_ = *(undefined4 *)&pIVar24[1].QueryInterface;
      local_2a8.n2.vt = 3;
      _Stack_e0 = local_248.decVal.u2;
      local_d8 = local_248._16_8_;
      _Stack_c0 = local_260.decVal.u2;
      local_b8 = local_260._16_8_;
      _Stack_a0.Lo64 = (ULONGLONG)local_278.decVal.u2;
      local_98 = local_278._16_8_;
      _Stack_80.Lo64 = (ULONGLONG)local_290.decVal.u2;
      local_78 = local_290._16_8_;
      local_68 = CONCAT62(local_2a8._2_6_,3);
      uStack_60 = CONCAT44(local_2a8._12_4_,local_2a8._8_4_);
      local_58 = local_2a8._16_8_;
      UVar14 = (*((IUnknownVtbl *)pIVar26->QueryInterface)[0xd].AddRef)((IUnknown *)pIVar26);
      if ((int)UVar14 < 0) {
        _com_issue_errorex(UVar14,(IUnknown *)pIVar26,(_GUID *)&DAT_15364c348);
      }
      if (local_2a8.n2.vt == 8) {
        local_2a8.n2.vt = 0;
        if (CONCAT44(local_2a8._12_4_,local_2a8._8_4_) != 0) {
          (*DAT_153775ef0)(CONCAT44(local_2a8._12_4_,local_2a8._8_4_) + -4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_2a8.n2);
      }
      if (local_290.n2.vt == 8) {
        local_290.n2.vt = 0;
        if (local_290.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_290.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_290.n2);
      }
      if (local_278.n2.vt == 8) {
        local_278.n2.vt = 0;
        if (local_278.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_278.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_278.n2);
      }
      if (local_260.n2.vt == 8) {
        local_260.n2.vt = 0;
        if (local_260.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_260.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_260.n2);
      }
      if (local_248.n2.vt == 8) {
        local_248.n2.vt = 0;
        if (local_248.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_248.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_248.n2);
      }
    }
    if (*(int *)&pIVar24[1].Release != 0) {
      VariantInit((VARIANTARG *)&local_230.n2);
      iVar12 = FUN_153315860(&local_230,&DAT_153770dc8);
      if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_153513900(iVar12);
      }
      FUN_1533fc120(param_1 + -2,*(undefined4 *)&pIVar24[1].Release,&local_230);
      if (local_230.n2.vt == 8) {
        local_230.n2.vt = 0;
        if (local_230.decVal.u2.Lo64 != 0) {
          (*DAT_153775ef0)((longlong)local_230.decVal.u2 - 4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_230.n2);
      }
    }
    iVar12 = *(int *)((longlong)&pIVar24[1].Release + 4);
    if (iVar12 != 0) {
      local_218._8_4_ = *(undefined4 *)&pIVar24[1].QueryInterface;
      local_218.n2.vt = 3;
      FUN_1533fc120(param_1 + -2,iVar12,&local_218);
      if (local_218.n2.vt == 8) {
        local_218.n2.vt = 0;
        if (CONCAT44(local_218._12_4_,local_218._8_4_) != 0) {
          (*DAT_153775ef0)(CONCAT44(local_218._12_4_,local_218._8_4_) + -4);
        }
      }
      else {
        VariantClear((VARIANTARG *)&local_218.n2);
      }
    }
  }
  *(uint *)&param_1[0x40].lpVtbl = param_2;
  FUN_15333abb0((undefined1 *)((longlong)&param_1[0x40].lpVtbl + 4),param_4,0);
  if (local_3b0 != (_func_5157 *)0x0) {
    pcVar1 = local_3b0 + -0x28;
    LOCK();
    pcVar22 = local_3b0 + -0x20;
    lVar18 = *(longlong *)pcVar22;
    *(longlong *)pcVar22 = *(longlong *)pcVar22 + -1;
    UNLOCK();
    if ((int)lVar18 == 1) {
      if (*(longlong *)(local_2d0 + -0x10) != 0) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_2d0 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_2d0 + -0x10) + 4) != 0);
      }
      if (pcVar1 != (code *)0x0) {
        (*(code *)**(undefined8 **)pcVar1)(pcVar1,1);
      }
    }
  }
  if (local_390 != (_func_5157 *)0x0) {
    pcVar1 = local_390 + -0x28;
    LOCK();
    pcVar22 = local_390 + -0x20;
    lVar18 = *(longlong *)pcVar22;
    *(longlong *)pcVar22 = *(longlong *)pcVar22 + -1;
    UNLOCK();
    if ((int)lVar18 == 1) {
      if (*(longlong *)(local_390 + -0x10) != 0) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_390 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_390 + -0x10) + 4) != 0);
      }
      if (pcVar1 != (code *)0x0) {
        (*(code *)**(undefined8 **)pcVar1)(pcVar1,1);
      }
    }
  }
  pIVar7 = local_1f8 + 1;
  *(int *)&pIVar7->lpVtbl = *(int *)&pIVar7->lpVtbl + -1;
  if (*(int *)&pIVar7->lpVtbl == 0) {
    local_1f8->lpVtbl = (IUnknownVtbl *)0x0;
  }
  pIVar7 = local_200 + 1;
  *(int *)&pIVar7->lpVtbl = *(int *)&pIVar7->lpVtbl + -1;
  if (*(int *)&pIVar7->lpVtbl == 0) {
    local_200->lpVtbl = (IUnknownVtbl *)0x0;
  }
  return 0;
LAB_1534008b0:
  do {
    if ((uVar21 & 1) == 0) {
      pIVar19 = (IUnknownVtbl *)&pIVar24[-2].Release;
      pIVar23 = (IUnknownVtbl *)&DAT_00000008;
    }
    else {
      pIVar19 = pIVar24 + -1;
      pIVar23 = (IUnknownVtbl *)&DAT_00000010;
    }
    if (pIVar24 != (IUnknownVtbl *)0x0) {
      pIVar23 = pIVar19;
    }
    pIVar19 = pIVar26;
    if (pIVar23->QueryInterface != (_func_5156 *)0x0) {
      pIVar19 = (IUnknownVtbl *)(pIVar23->QueryInterface + 0x28);
    }
    uVar30 = (int)pIVar31 + *(int *)(pIVar24->AddRef + 0x14);
    pIVar24 = pIVar19;
    pIVar31 = (IUnknownVtbl *)(ulonglong)uVar30;
  } while (pIVar19 != (IUnknownVtbl *)0x0);
LAB_1534008ef:
  local_378 = (int)(uVar30 * local_380) / 1000;
  goto LAB_153400918;
}



//===========================================================
// FUN_153401cb0 @ 153401cb0   (171 bytes)
//===========================================================

undefined8 FUN_153401cb0(longlong param_1,uint *param_2)

{
  int *piVar1;
  void *pvVar2;
  bool bVar3;
  
  pvVar2 = Self;
  if (param_2 == (uint *)0x0) {
    return 0x80004003;
  }
  LOCK();
  bVar3 = *(longlong *)(param_1 + 0xa0) == 0;
  if (bVar3) {
    *(longlong *)(param_1 + 0xa0) = (longlong)Self;
  }
  UNLOCK();
  if (bVar3) {
LAB_153401d29:
    *(undefined4 *)(param_1 + 0xa8) = 1;
  }
  else {
    if (*(void **)(param_1 + 0xa0) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar3 = *(longlong *)(param_1 + 0xa0) == 0;
        if (bVar3) {
          *(longlong *)(param_1 + 0xa0) = (longlong)Self;
        }
        UNLOCK();
        if (bVar3) goto LAB_153401d29;
        if (*(void **)(param_1 + 0xa0) == pvVar2) break;
        Sleep(0);
      }
    }
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  *param_2 = (uint)*(byte *)(param_1 + 0x110);
  piVar1 = (int *)(param_1 + 0xa8);
  *piVar1 = *piVar1 + -1;
  if (*piVar1 == 0) {
    *(undefined8 *)(param_1 + 0xa0) = 0;
  }
  return 0;
}



//===========================================================
// FUN_153401d70 @ 153401d70   (186 bytes)
//===========================================================

undefined8 FUN_153401d70(longlong *param_1,int param_2)

{
  longlong *plVar1;
  longlong lVar2;
  void *pvVar3;
  longlong *plVar4;
  
  pvVar3 = Self;
  plVar1 = param_1 + 0x14;
  LOCK();
  lVar2 = *plVar1;
  if (lVar2 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar2 == 0) {
LAB_153401ddc:
    *(undefined4 *)(param_1 + 0x15) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(param_1 + 0x15) = (int)param_1[0x15] + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_153401ddc;
      if ((void *)*plVar1 == pvVar3) break;
      Sleep(0);
    }
    *(int *)(param_1 + 0x15) = (int)param_1[0x15] + 1;
  }
  plVar4 = param_1 + 0x15;
  if ((bool)(char)param_1[0x22] != (param_2 != 0)) {
    (**(code **)(*param_1 + 0x2c8))(param_1,0);
  }
  *(bool *)(param_1 + 0x22) = param_2 != 0;
  *(int *)plVar4 = (int)*plVar4 + -1;
  if ((int)*plVar4 == 0) {
    *plVar1 = 0;
  }
  return 0;
}



//===========================================================
// FUN_153402270 @ 153402270   (149 bytes)
//===========================================================

undefined8 FUN_153402270(longlong param_1,uint param_2)

{
  int *piVar1;
  void *pvVar2;
  bool bVar3;
  
  pvVar2 = Self;
  LOCK();
  bVar3 = *(longlong *)(param_1 + 0xa0) == 0;
  if (bVar3) {
    *(longlong *)(param_1 + 0xa0) = (longlong)Self;
  }
  UNLOCK();
  if (bVar3) {
LAB_1534022d9:
    *(undefined4 *)(param_1 + 0xa8) = 1;
  }
  else {
    if (*(void **)(param_1 + 0xa0) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar3 = *(longlong *)(param_1 + 0xa0) == 0;
        if (bVar3) {
          *(longlong *)(param_1 + 0xa0) = (longlong)Self;
        }
        UNLOCK();
        if (bVar3) goto LAB_1534022d9;
        if (*(void **)(param_1 + 0xa0) == pvVar2) break;
        Sleep(0);
      }
    }
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  **(uint **)(param_1 + 0x120) = **(uint **)(param_1 + 0x120) | param_2;
  piVar1 = (int *)(param_1 + 0xa8);
  *piVar1 = *piVar1 + -1;
  if (*piVar1 == 0) {
    *(undefined8 *)(param_1 + 0xa0) = 0;
  }
  return 0;
}



//===========================================================
// FUN_153402320 @ 153402320   (149 bytes)
//===========================================================

undefined8 FUN_153402320(longlong param_1,uint param_2)

{
  int *piVar1;
  void *pvVar2;
  bool bVar3;
  
  pvVar2 = Self;
  LOCK();
  bVar3 = *(longlong *)(param_1 + 0xa0) == 0;
  if (bVar3) {
    *(longlong *)(param_1 + 0xa0) = (longlong)Self;
  }
  UNLOCK();
  if (bVar3) {
LAB_153402389:
    *(undefined4 *)(param_1 + 0xa8) = 1;
  }
  else {
    if (*(void **)(param_1 + 0xa0) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar3 = *(longlong *)(param_1 + 0xa0) == 0;
        if (bVar3) {
          *(longlong *)(param_1 + 0xa0) = (longlong)Self;
        }
        UNLOCK();
        if (bVar3) goto LAB_153402389;
        if (*(void **)(param_1 + 0xa0) == pvVar2) break;
        Sleep(0);
      }
    }
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  **(uint **)(param_1 + 0x120) = **(uint **)(param_1 + 0x120) | param_2;
  piVar1 = (int *)(param_1 + 0xa8);
  *piVar1 = *piVar1 + -1;
  if (*piVar1 == 0) {
    *(undefined8 *)(param_1 + 0xa0) = 0;
  }
  return 0;
}



//===========================================================
// FUN_153402480 @ 153402480   (151 bytes)
//===========================================================

undefined8 FUN_153402480(longlong param_1,uint param_2)

{
  int *piVar1;
  void *pvVar2;
  bool bVar3;
  
  pvVar2 = Self;
  LOCK();
  bVar3 = *(longlong *)(param_1 + 0xa0) == 0;
  if (bVar3) {
    *(longlong *)(param_1 + 0xa0) = (longlong)Self;
  }
  UNLOCK();
  if (bVar3) {
LAB_1534024e9:
    *(undefined4 *)(param_1 + 0xa8) = 1;
  }
  else {
    if (*(void **)(param_1 + 0xa0) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar3 = *(longlong *)(param_1 + 0xa0) == 0;
        if (bVar3) {
          *(longlong *)(param_1 + 0xa0) = (longlong)Self;
        }
        UNLOCK();
        if (bVar3) goto LAB_1534024e9;
        if (*(void **)(param_1 + 0xa0) == pvVar2) break;
        Sleep(0);
      }
    }
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  **(uint **)(param_1 + 0x120) = **(uint **)(param_1 + 0x120) & ~param_2;
  piVar1 = (int *)(param_1 + 0xa8);
  *piVar1 = *piVar1 + -1;
  if (*piVar1 == 0) {
    *(undefined8 *)(param_1 + 0xa0) = 0;
  }
  return 0;
}



//===========================================================
// FUN_153402530 @ 153402530   (387 bytes)
//===========================================================

undefined8 FUN_153402530(longlong param_1,uint param_2,int param_3)

{
  longlong *plVar1;
  longlong lVar2;
  char cVar3;
  longlong *plVar4;
  longlong *plVar5;
  void *pvVar6;
  longlong *plVar7;
  longlong *plVar8;
  longlong *plVar9;
  int *piVar10;
  bool bVar11;
  longlong *local_58;
  undefined8 uStack_50;
  longlong *local_48;
  uint uStack_40;
  undefined4 uStack_3c;
  
  pvVar6 = Self;
  plVar1 = (longlong *)(param_1 + 0xa0);
  LOCK();
  lVar2 = *plVar1;
  if (lVar2 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar2 == 0) {
LAB_1534025ac:
    *(undefined4 *)(param_1 + 0xa8) = 1;
  }
  else if ((void *)*plVar1 == pvVar6) {
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  else {
    while( true ) {
      pvVar6 = Self;
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_1534025ac;
      if ((void *)*plVar1 == pvVar6) break;
      Sleep(0);
    }
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  piVar10 = (int *)(param_1 + 0xa8);
  plVar9 = (longlong *)(*(longlong *)(param_1 + 0x120) + 0x10);
  plVar4 = (longlong *)*plVar9;
  plVar7 = (longlong *)plVar4[1];
  uStack_40 = 0;
  cVar3 = *(char *)((longlong)plVar7 + 0x19);
  plVar8 = plVar4;
  local_48 = plVar7;
  while (plVar5 = plVar7, cVar3 == '\0') {
    bVar11 = param_2 <= *(uint *)((longlong)plVar5 + 0x1c);
    if (bVar11) {
      plVar7 = (longlong *)*plVar5;
      plVar8 = plVar5;
    }
    else {
      plVar7 = (longlong *)plVar5[2];
    }
    uStack_40 = (uint)bVar11;
    cVar3 = *(char *)((longlong)plVar7 + 0x19);
    local_48 = plVar5;
  }
  if ((*(char *)((longlong)plVar8 + 0x19) != '\0') || (param_2 < *(uint *)((longlong)plVar8 + 0x1c))
     ) {
    if (*(longlong *)(*(longlong *)(param_1 + 0x120) + 0x18) == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
      FUN_153273a80();
    }
    uStack_50 = 0;
    local_58 = plVar9;
    plVar7 = (longlong *)FUN_1532c96e0(&DAT_153776230,0x28);
    *(uint *)((longlong)plVar7 + 0x1c) = param_2;
    *(undefined4 *)(plVar7 + 4) = 0;
    *plVar7 = (longlong)plVar4;
    plVar7[1] = (longlong)plVar4;
    plVar7[2] = (longlong)plVar4;
    *(undefined2 *)(plVar7 + 3) = 0;
    uStack_50 = CONCAT44(uStack_3c,uStack_40);
    local_58 = local_48;
    plVar8 = (longlong *)FUN_15331ccf0(plVar9,&local_58,plVar7);
  }
  *(float *)(plVar8 + 4) = (float)param_3;
  *piVar10 = *piVar10 + -1;
  if (*piVar10 == 0) {
    *plVar1 = 0;
  }
  return 0;
}



//===========================================================
// FUN_1534027b0 @ 1534027b0   (177 bytes)
//===========================================================

undefined8 FUN_1534027b0(longlong param_1,undefined4 param_2,undefined4 param_3)

{
  longlong *plVar1;
  longlong lVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int *piVar5;
  undefined4 local_res10 [2];
  
  pvVar3 = Self;
  plVar1 = (longlong *)(param_1 + 0xa0);
  LOCK();
  lVar2 = *plVar1;
  if (lVar2 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  local_res10[0] = param_2;
  if (lVar2 == 0) {
LAB_15340282c:
    *(undefined4 *)(param_1 + 0xa8) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_15340282c;
      if ((void *)*plVar1 == pvVar3) break;
      Sleep(0);
    }
    *(int *)(param_1 + 0xa8) = *(int *)(param_1 + 0xa8) + 1;
  }
  piVar5 = (int *)(param_1 + 0xa8);
  puVar4 = (undefined4 *)FUN_1534063c0(*(longlong *)(param_1 + 0x120) + 0x20,local_res10);
  *puVar4 = param_3;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *plVar1 = 0;
  }
  return 0;
}


