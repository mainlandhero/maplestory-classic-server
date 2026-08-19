
//===========================================================
// FUN_141b3f050 @ 141b3f050   (560 bytes)
//===========================================================

void FUN_141b3f050(longlong param_1,int param_2,int param_3)

{
  int iVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong *local_res8;
  longlong local_res20;
  ushort uStack_4e;
  
  iVar1 = *(int *)(param_1 + 0xd0);
  *(int *)(param_1 + 0x238) = param_2;
  iVar3 = FUN_1429e3ef0();
  *(undefined1 *)(param_1 + 0x240) = 1;
  *(bool *)(param_1 + 0x241) = param_3 == 0;
  *(int *)(param_1 + 0x244) = iVar3;
  *(int *)(param_1 + 0x248) = (int)((double)param_3 * DAT_14327aa58 + (double)iVar3);
  if (param_3 == 0) {
    *(undefined4 *)(param_1 + 0x23c) = 0;
    FUN_141b3f290(param_1);
    FUN_141b3e060(param_1,*(undefined4 *)(param_1 + 0x238));
    *(undefined8 *)(param_1 + 0x238) = 0;
    *(ulonglong *)(param_1 + 0x240) = (ulonglong)uStack_4e << 0x10;
    *(undefined8 *)(param_1 + 0x248) = 0;
    *(undefined8 *)(param_1 + 0x250) = 0;
    *(undefined4 *)(param_1 + 600) = 0;
  }
  else if ((((iVar1 == 3) || (iVar1 == 4)) || (iVar1 == 5)) &&
          (((param_2 == 3 || (param_2 == 4)) || (param_2 == 5)))) {
    *(undefined4 *)(param_1 + 0x23c) = 2;
    FUN_141b3e1d0(param_1,param_2,param_3);
  }
  else {
    *(undefined4 *)(param_1 + 0x23c) = 1;
    iVar4 = param_3 / 2;
    FUN_140de1ce0(DAT_143abfdf0,&local_res8,iVar4,0,param_3 - iVar4,0x27e2,0xff,0xff000000,0,0);
    if (local_res8 != (longlong *)0x0) {
      (**(code **)(*local_res8 + 0x10))();
    }
    *(int *)(param_1 + 0x244) = iVar4 + iVar3;
  }
  uVar5 = FUN_141b3b980(param_1,&local_res20,iVar1);
  uVar6 = FUN_141b3b980(param_1,&local_res8,*(undefined4 *)(param_1 + 0x238));
  cVar2 = FUN_1407330d0(uVar5,uVar6);
  if (local_res8 != (longlong *)0x0) {
    FUN_1401bebb0(local_res8 + -2);
  }
  if (local_res20 != 0) {
    FUN_1401bebb0(local_res20 + -0x10);
  }
  if ((cVar2 != '\0') && (DAT_143abfea0 != 0)) {
    FUN_142081a20(DAT_143abfea0,1,1000,1000,0,0);
  }
  return;
}



//===========================================================
// FUN_141b3ff10 @ 141b3ff10   (310 bytes)
//===========================================================

void FUN_141b3ff10(longlong param_1)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int iVar5;
  int *piVar6;
  wchar_t *local_res8;
  undefined8 local_res10;
  undefined8 *local_res18;
  
  if (*(longlong *)(param_1 + 0x270) == 0) {
    iVar5 = FUN_141b2ba60(param_1,0x50,0,0,0);
    if (iVar5 == 0) {
      local_res18 = &local_res10;
      local_res10 = 0;
      local_res8 = (wchar_t *)0x0;
      piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
      piVar6[1] = 0x14;
      *piVar6 = -1;
      local_res8 = (wchar_t *)(piVar6 + 4);
      piVar6[2] = 0;
      *local_res8 = L'\0';
      uVar1 = u_unableLogOnToGameSvr_1433d5c40._8_8_;
      *(undefined8 *)local_res8 = u_unableLogOnToGameSvr_1433d5c40._0_8_;
      *(undefined8 *)(piVar6 + 6) = uVar1;
      uVar4 = u_unableLogOnToGameSvr_1433d5c40._28_4_;
      uVar3 = u_unableLogOnToGameSvr_1433d5c40._24_4_;
      uVar2 = u_unableLogOnToGameSvr_1433d5c40._20_4_;
      piVar6[8] = u_unableLogOnToGameSvr_1433d5c40._16_4_;
      piVar6[9] = uVar2;
      piVar6[10] = uVar3;
      piVar6[0xb] = uVar4;
      *(undefined8 *)(piVar6 + 0xc) = u_unableLogOnToGameSvr_1433d5c40._32_8_;
      if (*piVar6 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar6[1] < 0x14) {
        FUN_142e54290(0x90,piVar6[1],0x14);
      }
      *piVar6 = 1;
      local_res8[0x14] = L'\0';
      if (piVar6[1] + 1 < 0x15) {
        FUN_142e54290(0x9c,0x14);
      }
      piVar6[2] = 0x28;
      FUN_141b4ac80(&local_res8,&local_res10,0);
      return;
    }
    FUN_141b3fd40(param_1,1);
  }
  return;
}



//===========================================================
// FUN_141b3f290 @ 141b3f290   (1788 bytes)
//===========================================================

void FUN_141b3f290(longlong param_1)

{
  int *_Buf2;
  longlong *plVar1;
  int **ppiVar2;
  int *_Buf1;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined4 *puVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  undefined4 *puVar11;
  longlong lVar12;
  longlong *local_res8;
  longlong *local_res10;
  longlong *local_res18;
  longlong local_res20;
  longlong local_98;
  int *local_90;
  int *local_88;
  longlong local_80;
  undefined4 *local_78;
  undefined4 *local_70;
  undefined1 local_68 [40];
  
  *(undefined1 *)(param_1 + 0x240) = 0;
  FUN_141b21910(param_1 + 0x150);
  if (*(int *)(param_1 + 0x23c) == 1) {
    FUN_141b3e1d0(param_1,*(undefined4 *)(param_1 + 0x238),0);
  }
  FUN_141b3cf70(param_1,*(undefined4 *)(param_1 + 0x238));
  FUN_141b3bb40(param_1,*(undefined4 *)(param_1 + 0x238));
  iVar4 = *(int *)(param_1 + 0x238);
  FUN_141b27da0(param_1,iVar4);
  FUN_141b26700(param_1,&local_80,iVar4);
  iVar3 = FUN_142c4a810(DAT_143ac1898);
  if ((iVar3 != 5) && (DAT_143a886e0 == iVar4)) {
    local_res8 = (longlong *)CONCAT44(local_res8._4_4_,1);
    if (local_78 == local_70) {
      FUN_141b42fb0(&local_80,local_78,&local_res8);
    }
    else {
      *local_78 = 1;
      local_78 = local_78 + 1;
    }
  }
  lVar12 = *(longlong *)(param_1 + 0x138);
  if (lVar12 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar12 = *(longlong *)(param_1 + 0x138);
  }
  uVar5 = FUN_1401d0680(local_68,&local_80);
  FUN_141b468b0(lVar12,uVar5);
  lVar12 = *(longlong *)(param_1 + 0x138);
  if (iVar4 == 2) {
    if (lVar12 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar12 = *(longlong *)(param_1 + 0x138);
    }
    uVar5 = 1;
  }
  else {
    if (lVar12 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar12 = *(longlong *)(param_1 + 0x138);
    }
    uVar5 = 4;
  }
  FUN_141b48090(lVar12,uVar5);
  if (DAT_143aca790 != 0) {
    if (iVar4 != 4) {
      FUN_141179970(DAT_143aca790,0);
      goto LAB_141b3f3f4;
    }
LAB_141b3f6cc:
    FUN_14090e740(&local_res10,"UI/Login.img/CharSelect/particle");
    local_res8 = local_res10;
    if (local_res10 != (longlong *)0x0) {
      (**(code **)(*local_res10 + 8))();
    }
    FUN_141b20dc0(&local_res8,0,param_1 + 0x150);
    *(undefined4 *)(param_1 + 0x188) = 0;
    *(undefined2 *)(param_1 + 0x18c) = 0;
    if (local_res10 != (longlong *)0x0) {
      (**(code **)(*local_res10 + 0x10))();
    }
    goto LAB_141b3f92f;
  }
LAB_141b3f3f4:
  if (iVar4 == 1) {
    if (*(longlong *)(param_1 + 0x138) != 0) {
      FUN_141b46e30();
    }
    *(undefined4 *)(param_1 + 0xd4) = 0;
    *(undefined4 *)(param_1 + 0x128) = 0xffffffff;
    *(undefined4 *)(param_1 + 0xe8) = 0;
    FUN_14022d860(&local_res8,&DAT_143278568,0xffffffff);
    if (*(longlong *)(param_1 + 400) != 0) {
      FUN_14019f2c0(*(longlong *)(param_1 + 400) + -0x10);
    }
    *(longlong **)(param_1 + 400) = local_res8;
    *(undefined4 *)(param_1 + 0x188) = 0;
    *(undefined2 *)(param_1 + 0x18c) = 0;
    DAT_143ad2100 = 0;
    if ((DAT_143acd9a0 != 0) && (FUN_142bf3f70(), DAT_143acd9a0 != 0)) {
      (*(code *)**(undefined8 **)(DAT_143acd9a0 + 8))((undefined8 *)(DAT_143acd9a0 + 8),1);
    }
    goto LAB_141b3f92f;
  }
  if (iVar4 == 2) {
    if (*(longlong *)(param_1 + 0x138) != 0) {
      FUN_141b46e30();
    }
    *(undefined4 *)(param_1 + 0x128) = 0xffffffff;
    *(undefined4 *)(param_1 + 0xe8) = 0;
    FUN_14022d860(&local_res8,&DAT_143278568,0xffffffff);
    if (*(longlong *)(param_1 + 400) != 0) {
      FUN_14019f2c0(*(longlong *)(param_1 + 400) + -0x10);
    }
    *(longlong **)(param_1 + 400) = local_res8;
    *(undefined4 *)(param_1 + 0x188) = 0;
    *(undefined2 *)(param_1 + 0x18c) = 0;
    if (((DAT_143abfdf8 != 0) && (DAT_143ad2230 != 0)) && (*(char *)(param_1 + 0x1a8) == '\0')) {
      lVar12 = DAT_143ad2230 + 8;
      if (DAT_143ad2230 == 0) {
        lVar12 = 0;
      }
      FUN_142c0bf50(DAT_143abfdf8,lVar12,0);
    }
    goto LAB_141b3f92f;
  }
  if (iVar4 == 3) {
    if ((*(int *)(param_1 + 0xd0) == 2) && (*(longlong *)(param_1 + 0x138) != 0)) {
      uVar5 = FUN_141b43aa0(param_1 + 0x130);
      FUN_141b46a70(uVar5,3);
    }
    iVar4 = 0;
    *(undefined4 *)(param_1 + 0x188) = 0;
    *(undefined2 *)(param_1 + 0x18c) = 0;
    puVar11 = (undefined4 *)(param_1 + 0x220);
    for (puVar7 = puVar11; puVar7 != (undefined4 *)(param_1 + 0x230); puVar7 = puVar7 + 1) {
      *puVar7 = 4;
      iVar4 = iVar4 + 4;
    }
    iVar4 = 0x19 - iVar4;
    while (0 < iVar4) {
      iVar3 = FUN_140739110(0,3);
      if ((int)puVar11[iVar3] < 0xc) {
        puVar11[iVar3] = puVar11[iVar3] + 1;
        iVar4 = iVar4 + -1;
      }
    }
    if (DAT_143aca358 != (longlong *)0x0) {
      (**(code **)(*DAT_143aca358 + 0x90))(DAT_143aca358,0);
    }
    goto LAB_141b3f92f;
  }
  if (iVar4 == 4) goto LAB_141b3f6cc;
  if (iVar4 != 5) {
    FUN_142c0bf50(DAT_143abfdf8,param_1 + 8,0);
    goto LAB_141b3f92f;
  }
  FUN_14109d7e0(&local_res20,*(undefined4 *)(param_1 + 0x188));
  uVar8 = 0xffffffffffffffff;
  lVar12 = -1;
  do {
    lVar12 = lVar12 + 1;
  } while (L"particle"[lVar12] != L'\0');
  FUN_14040ea40(&local_res20,&local_98);
  FUN_14090ead0(&local_res18,local_98);
  if (local_98 != 0) {
    FUN_1401bebb0(local_98 + -0x10);
  }
  local_res10 = local_res18;
  if (local_res18 != (longlong *)0x0) {
    (**(code **)(*local_res18 + 8))();
  }
  FUN_141b20dc0(&local_res10,1,param_1 + 0x150);
  piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  uVar9 = 0;
  piVar6[1] = 0;
  *piVar6 = -1;
  _Buf2 = piVar6 + 4;
  piVar6[2] = 0;
  *piVar6 = 1;
  *(undefined1 *)_Buf2 = 0;
  piVar6[2] = 0;
  ppiVar2 = (int **)(param_1 + 0x230);
  local_90 = _Buf2;
  local_88 = piVar6;
  if (ppiVar2 == &local_90) goto LAB_141b3f69e;
  _Buf1 = *ppiVar2;
  iVar4 = 0;
  if (_Buf1 != (int *)0x0) {
    iVar4 = _Buf1[-2];
  }
  if (((iVar4 == piVar6[2]) && (iVar4 != 0)) && (_Buf1 != (int *)0x0)) {
    if (_Buf2 != (int *)0x0) {
      iVar4 = memcmp(_Buf1,_Buf2,(longlong)iVar4);
      if (iVar4 == 0) goto LAB_141b3f69e;
      goto LAB_141b3f54c;
    }
LAB_141b3f691:
    FUN_14019f2c0(_Buf1 + -4);
    *ppiVar2 = (int *)0x0;
  }
  else {
LAB_141b3f54c:
    if ((_Buf2 == (int *)0x0) || (piVar6 == (int *)0x0)) {
      if (_Buf1 != (int *)0x0) goto LAB_141b3f691;
    }
    else if (*piVar6 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar10 = 0xffffffffffffffff;
      do {
        uVar10 = uVar10 + 1;
      } while (*(char *)((longlong)_Buf2 + uVar10) != '\0');
      iVar3 = (int)uVar10;
      iVar4 = 0;
      if (0 < iVar3) {
        iVar4 = iVar3;
      }
      piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
      piVar6[1] = iVar4;
      *piVar6 = -1;
      plVar1 = (longlong *)(piVar6 + 4);
      piVar6[2] = 0;
      *(undefined1 *)plVar1 = 0;
      local_res8 = plVar1;
      FUN_142ef7ba0(plVar1,_Buf2,(longlong)iVar3);
      if (*piVar6 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar3 == -1) || (iVar3 <= piVar6[1])) {
        *piVar6 = 1;
        if (iVar3 != -1) goto LAB_141b3f5fa;
        if (plVar1 != (longlong *)0x0) {
          do {
            uVar8 = uVar8 + 1;
          } while (*(char *)((longlong)plVar1 + uVar8) != '\0');
          uVar9 = uVar8 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar6[1],uVar10 & 0xffffffff);
        *piVar6 = 1;
LAB_141b3f5fa:
        *(undefined1 *)((longlong)plVar1 + (longlong)iVar3) = 0;
        uVar9 = uVar10;
      }
      iVar4 = (int)uVar9;
      if ((iVar4 < 0) || (piVar6[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,uVar9 & 0xffffffff);
      }
      piVar6[2] = iVar4;
      if (*ppiVar2 != (int *)0x0) {
        FUN_14019f2c0(*ppiVar2 + -4);
      }
      *ppiVar2 = (int *)plVar1;
    }
    else {
      if (*piVar6 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar6 = *piVar6 + 1;
      UNLOCK();
      if (*ppiVar2 != (int *)0x0) {
        FUN_14019f2c0(*ppiVar2 + -4);
      }
      *ppiVar2 = _Buf2;
    }
  }
LAB_141b3f69e:
  FUN_14019f2c0(local_88);
  if (local_res18 != (longlong *)0x0) {
    (**(code **)(*local_res18 + 0x10))();
  }
  if (local_res20 != 0) {
    FUN_1401bebb0(local_res20 + -0x10);
  }
LAB_141b3f92f:
  FUN_1429edb20(PTR_u_ScrollUp_143a47be8);
  if (local_80 != 0) {
    uVar8 = (longlong)local_70 - local_80 & 0xfffffffffffffffc;
    if (0xfff < uVar8) {
      if (0x1f < (local_80 - *(longlong *)(local_80 + -8)) - 8U) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(*(longlong *)(local_80 + -8),uVar8 + 0x27);
      }
    }
    thunk_FUN_140205820();
  }
  return;
}



//===========================================================
// FUN_141b5bb50 @ 141b5bb50   (743 bytes)
//===========================================================

undefined8 * FUN_141b5bb50(undefined8 *param_1,undefined8 param_2)

{
  IUnknown *pIVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  int iVar5;
  int local_res18;
  int iStackX_1c;
  longlong local_res20;
  ulonglong uVar6;
  IUnknown *local_58;
  undefined8 local_50;
  undefined1 local_48 [32];
  
  uVar4 = FUN_141d5d8c0(local_48,&DAT_143271f04,&DAT_143271f04,0x37,0xffffffff);
  FUN_141805fd0(param_1,uVar4);
  DAT_143ad2220 = param_1;
  if (param_1 == (undefined8 *)0xfffffffffffffd20) {
    DAT_143ad2220 = (undefined8 *)0x0;
  }
  *param_1 = &PTR_FUN_143400040;
  param_1[1] = &PTR_LAB_1434001b0;
  param_1[3] = &PTR_LAB_143400288;
  FUN_141aa3a20(param_1 + 0x5d);
  _eh_vector_constructor_iterator_
            (param_1 + 0x5f,8,2,(_func_void_void_ptr *)&LAB_140297ad0,FUN_1402887e0);
  param_1[100] = 0;
  param_1[0x66] = 0;
  param_1[0x68] = 0;
  param_1[0x6a] = 0;
  param_1[0x6c] = 0;
  param_1[0x6e] = 0;
  param_1[0x70] = 0;
  param_1[0x72] = 0;
  param_1[0x74] = 0;
  param_1[0x76] = 0;
  param_1[0x78] = 0;
  param_1[0x79] = 0;
  *(undefined4 *)(param_1 + 0x7a) = 0;
  local_res20 = 0;
  FUN_1401d66b0(&local_res20,PTR_s_UI_Login_img_Title_new_backgrd_143a44338,0xffffffff);
  FUN_14090f3b0(&local_58,&local_res20);
  if (local_res20 != 0) {
    FUN_14019f2c0(local_res20 + -0x10);
  }
  pIVar1 = local_58;
  if (local_58 == (IUnknown *)0x0) {
    local_res18 = 0xf4;
  }
  else {
    local_res18 = 0;
    iVar2 = (**(code **)(*(longlong *)local_58 + 0x98))(local_58,&local_res18);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327ac98);
    }
    pIVar1 = local_58;
    iVar2 = local_res18;
    if (local_58 != (IUnknown *)0x0) {
      local_res18 = 0;
      iVar3 = (**(code **)(*(longlong *)local_58 + 0xa0))(local_58,&local_res18);
      iVar5 = local_res18;
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_14327ac98);
        iVar5 = local_res18;
      }
      goto LAB_141b5bd44;
    }
  }
  iVar2 = local_res18;
  iVar5 = 0x9e;
LAB_141b5bd44:
  local_res18 = 1;
  local_50 = FUN_141127fb0(&local_res18);
  local_res18 = (int)local_50 - iVar2 / 2;
  iStackX_1c = FUN_142a11a80();
  iStackX_1c = (local_50._4_4_ - iVar5 / 2) - iStackX_1c;
  uVar6 = CONCAT44(iStackX_1c,local_res18);
  FUN_141806060(param_1,0xff,0xff,0xff,uVar6,uVar6,uVar6,0,0,0);
  FUN_1418060c0(param_1,iVar2,iVar5,PTR_u_UI_Login_img_Title_new_backgrd_143a46c30,0x271a,
                uVar6 & 0xffffffff00000000,param_2,1,0);
  if (DAT_143a88df8 != 0) {
    FUN_1415daff0(DAT_143ac18a0);
  }
  if (local_58 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_58 + 0x10))();
  }
  return param_1;
}



//===========================================================
// FUN_141d60f20 @ 141d60f20   (21 bytes)
//===========================================================

void FUN_141d60f20(void)

{
  undefined8 uVar1;
  
  uVar1 = FUN_141d610f0();
  FUN_141df7090(uVar1);
  return;
}



//===========================================================
// FUN_141d60f80 @ 141d60f80   (21 bytes)
//===========================================================

void FUN_141d60f80(void)

{
  undefined8 uVar1;
  
  uVar1 = FUN_141d610f0();
  FUN_141df6f80(uVar1);
  return;
}


