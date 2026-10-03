
//===========================================================
// FUN_141fa77a0 @ 141fa77a0   (1290 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141fa77a0(longlong param_1,int param_2,undefined8 param_3)

{
  longlong *plVar1;
  undefined2 *puVar2;
  code *pcVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  int iVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined8 uVar9;
  longlong lVar10;
  longlong *plVar11;
  longlong lVar12;
  ulonglong uVar13;
  uint uVar14;
  undefined1 *puVar15;
  undefined1 *puVar16;
  longlong **pplVar17;
  longlong lVar18;
  int iVar19;
  double dVar20;
  undefined1 auStack_a8 [48];
  longlong *local_78 [2];
  double local_68;
  longlong local_60;
  longlong local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined1 local_40 [8];
  undefined8 *local_38;
  ulonglong local_30;
  
  puVar15 = auStack_a8;
  puVar16 = auStack_a8;
  local_30 = DAT_143a8b908 ^ (ulonglong)local_78;
  local_50 = param_3;
  if (param_2 == 0) {
    lVar10 = *(longlong *)(param_1 + 0x2e0);
    if (lVar10 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar10 = *(longlong *)(param_1 + 0x2e0);
    }
    param_2 = *(int *)(lVar10 + 0x90) + 1;
  }
  uVar9 = FUN_142cbe730(DAT_143aa84a0);
  local_48 = uVar9;
  FUN_141fbd740(param_3);
  iVar19 = 1;
  iVar6 = FUN_14030ccb0(uVar9,param_2);
  if (0 < iVar6) {
    do {
      *(undefined8 *)(puVar15 + -8) = 0x141fa7862;
      lVar10 = FUN_1402e3cd0(uVar9,local_40,param_2,iVar19);
      puVar4 = local_38;
      plVar11 = *(longlong **)(lVar10 + 8);
      if (local_38 != (undefined8 *)0x0) {
        if (0xffffe < local_38[1] - 1) {
          *(undefined8 *)(puVar15 + -8) = 0x141fa7889;
          FUN_142e541f0(0x31e);
        }
        puVar5 = local_38;
        LOCK();
        plVar1 = puVar4 + 1;
        lVar10 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if (((int)lVar10 == 1) && (local_38 != (undefined8 *)0x0)) {
          pcVar3 = *(code **)*local_38;
          *(undefined8 *)(puVar15 + -8) = 0x141fa78b0;
          (*pcVar3)(puVar5,1);
        }
        local_38 = (undefined8 *)0x0;
      }
      lVar10 = DAT_143aa8328;
      puVar16 = puVar15;
      if (plVar11 != (longlong *)0x0) {
        *(undefined8 *)(puVar15 + -8) = 0x141fa78d1;
        uVar7 = FUN_14019a5d0(plVar11 + 4);
        *(undefined8 *)(puVar15 + -8) = 0x141fa78db;
        iVar6 = FUN_14038a860(lVar10,uVar7);
        lVar10 = DAT_143aa8328;
        if (iVar6 == 0) {
          *(undefined8 *)(puVar15 + -8) = 0x141fa78f3;
          uVar7 = FUN_14019a5d0(plVar11 + 4);
          *(undefined8 *)(puVar15 + -8) = 0x141fa78fd;
          iVar6 = FUN_14038bf90(lVar10,uVar7);
          lVar10 = DAT_143aa8328;
          if (iVar6 == 0) {
            *(undefined8 *)(puVar15 + -8) = 0x141fa7915;
            uVar7 = FUN_14019a5d0(plVar11 + 4);
            *(undefined8 *)(puVar15 + -8) = 0x141fa791f;
            iVar6 = FUN_140389c10(lVar10,uVar7);
            lVar10 = DAT_143aa8328;
            if ((iVar6 == 0) && (plVar11[7] == 0)) {
              if (DAT_143aa8328 != 0) {
                *(undefined8 *)(puVar15 + -8) = 0x141fa7947;
                uVar8 = FUN_14019a5d0(plVar11 + 4);
                uVar7 = *(undefined4 *)(param_1 + 0x314);
                *(undefined8 *)(puVar15 + -8) = 0x141fa7959;
                iVar6 = FUN_1403e1180(lVar10,uVar7,uVar8);
                if (iVar6 != 0) goto LAB_141fa7c60;
              }
              *(undefined8 *)(puVar15 + -8) = 0x141fa796e;
              lVar10 = FUN_141fbd400(param_3,0xffffffff);
              *(undefined8 *)(puVar15 + -8) = 0x141fa797a;
              uVar7 = FUN_14019a5d0(plVar11 + 4);
              *(undefined4 *)(lVar10 + 8) = uVar7;
              *(int *)(lVar10 + 0x128) = iVar19;
              *(undefined8 *)(puVar15 + -8) = 0x141fa799a;
              FUN_14038c960(DAT_143aa8328,uVar7,lVar10 + 0x38,&local_68);
              pcVar3 = *(code **)(*plVar11 + 0x98);
              *(undefined8 *)(puVar15 + -8) = 0x141fa79a6;
              iVar6 = (*pcVar3)(plVar11);
              *(int *)(lVar10 + 0xc) = iVar6;
              *(undefined8 *)(lVar10 + 0x40) = 0;
              dVar20 = (double)iVar6 * local_68;
              *(undefined8 *)(puVar15 + -8) = 0x141fa79c3;
              dVar20 = ceil(dVar20);
              *(longlong *)(lVar10 + 0x38) =
                   (longlong)(dVar20 + (double)*(longlong *)(lVar10 + 0x38));
              if ((*(int *)(lVar10 + 8) - 0x1f95f0U < 10000) ||
                 (*(int *)(lVar10 + 8) - 0x238d90U < 10000)) {
                uVar14 = 0;
                lVar18 = 0;
                while( true ) {
                  lVar12 = *(longlong *)(param_1 + 0x358);
                  if ((lVar12 == 0) || (*(uint *)(lVar12 + -8) <= uVar14)) goto LAB_141fa7a5d;
                  if ((int)uVar14 < 0) {
                    *(undefined8 *)(puVar15 + -8) = 0x141fa7a25;
                    FUN_142e54290(0xbc,uVar14);
                    lVar12 = *(longlong *)(param_1 + 0x358);
                  }
                  if (*(int *)(lVar12 + 8 + lVar18) == *(int *)(lVar10 + 8)) break;
                  uVar14 = uVar14 + 1;
                  lVar18 = lVar18 + 0x130;
                }
                *(double *)(lVar10 + 0x40) =
                     (double)(*(int *)(lVar12 + 0x10c + lVar18) - *(int *)(lVar10 + 0xc)) *
                     *(double *)(lVar12 + 0x40 + lVar18);
              }
LAB_141fa7a5d:
              if ((*(longlong *)(lVar10 + 0x120) - 1U < 999) ||
                 (*(longlong *)(lVar10 + 0x120) == -1)) {
                *(undefined8 *)(puVar15 + -8) = 0x141fa7a80;
                FUN_142e52ed0(0x447);
              }
              if (0xfffff < (ulonglong)plVar11[1]) {
                *(undefined8 *)(puVar15 + -8) = 0x141fa7a97;
                FUN_142e541f0(0x30f);
              }
              LOCK();
              plVar11[1] = plVar11[1] + 1;
              UNLOCK();
              puVar4 = *(undefined8 **)(lVar10 + 0x120);
              *(longlong **)(lVar10 + 0x120) = plVar11;
              if (puVar4 != (undefined8 *)0x0) {
                if (0xffffe < puVar4[1] - 1) {
                  *(undefined8 *)(puVar15 + -8) = 0x141fa7ac9;
                  FUN_142e541f0(0x31e);
                }
                LOCK();
                plVar11 = puVar4 + 1;
                lVar18 = *plVar11;
                *plVar11 = *plVar11 + -1;
                UNLOCK();
                if ((int)lVar18 == 1) {
                  pcVar3 = *(code **)*puVar4;
                  *(undefined8 *)(puVar15 + -8) = 0x141fa7aea;
                  (*pcVar3)(puVar4,1);
                }
              }
              uVar7 = *(undefined4 *)(lVar10 + 8);
              *(undefined8 *)(puVar15 + -8) = 0x141fa7aff;
              plVar11 = (longlong *)FUN_140398ba0(DAT_143aa8328,&local_58,uVar7);
              lVar18 = *plVar11;
              if (lVar18 == 0) {
                iVar6 = 2;
              }
              else {
                *(undefined4 *)(puVar15 + 0x28) = 0;
                *(undefined8 *)(puVar15 + 0x20) = 0;
                *(undefined8 *)(puVar15 + -8) = 0x141fa7b30;
                iVar6 = (*DAT_1432627f8)(0xfde9,0,lVar18,0xffffffff);
                iVar6 = iVar6 * 2;
              }
              uVar13 = (longlong)iVar6 + 0xf;
              if (uVar13 <= (ulonglong)(longlong)iVar6) {
                uVar13 = 0xffffffffffffff0;
              }
              *(undefined8 *)(puVar15 + -8) = 0x141fa7b53;
              lVar12 = -(uVar13 & 0xfffffffffffffff0);
              puVar2 = (undefined2 *)(puVar15 + lVar12 + 0x30);
              uVar7 = *(undefined4 *)(lVar10 + 8);
              *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7b6f;
              plVar11 = (longlong *)FUN_140398ba0(DAT_143aa8328,&local_60,uVar7);
              lVar18 = *plVar11;
              if (lVar18 == 0) {
                if (puVar2 != (undefined2 *)0x0) {
                  *puVar2 = 0;
                }
              }
              else {
                *(undefined4 *)(puVar15 + lVar12 + 0x28) = 0x100000;
                *(undefined2 **)(puVar15 + lVar12 + 0x20) = puVar2;
                *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7ba2;
                (*DAT_1432627f8)(0xfde9,0,lVar18,0xffffffff);
              }
              *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7bae;
              FUN_1401a5890(local_78,puVar2);
              pplVar17 = (longlong **)(lVar10 + 0x110);
              if (pplVar17 == local_78) {
LAB_141fa7bd6:
                plVar11 = local_78[0];
                if (local_78[0] != (longlong *)0x0) {
                  LOCK();
                  plVar1 = local_78[0] + 2;
                  lVar10 = *plVar1;
                  *(int *)plVar1 = (int)*plVar1 + -1;
                  UNLOCK();
                  if ((int)lVar10 == 1) {
                    lVar10 = *local_78[0];
                    if (lVar10 != 0) {
                      *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7c04;
                      (*DAT_143ad5990)(lVar10 + -4);
                      *plVar11 = 0;
                    }
                    if (plVar11[1] != 0) {
                      *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7c19;
                      FUN_14019b4e0();
                      plVar11[1] = 0;
                    }
                    *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7c2e;
                    thunk_FUN_140205820(plVar11,0x18);
                  }
                  local_78[0] = (longlong *)0x0;
                }
              }
              else {
                *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7bc6;
                FUN_1401be120(pplVar17);
                *pplVar17 = local_78[0];
                if (local_78[0] != (longlong *)0x0) {
                  LOCK();
                  *(int *)(local_78[0] + 2) = (int)local_78[0][2] + 1;
                  UNLOCK();
                  goto LAB_141fa7bd6;
                }
              }
              if (local_60 != 0) {
                lVar10 = local_60 + -0x10;
                *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7c48;
                FUN_14019f2c0(lVar10);
              }
              puVar16 = puVar15 + lVar12;
              param_3 = local_50;
              if (local_58 != 0) {
                lVar10 = local_58 + -0x10;
                *(undefined8 *)(puVar15 + lVar12 + -8) = 0x141fa7c5b;
                FUN_14019f2c0(lVar10);
                puVar16 = puVar15 + lVar12;
                param_3 = local_50;
              }
            }
          }
        }
      }
LAB_141fa7c60:
      uVar9 = local_48;
      iVar19 = iVar19 + 1;
      *(undefined8 *)(puVar16 + -8) = 0x141fa7c72;
      iVar6 = FUN_14030ccb0(uVar9,param_2);
      puVar15 = puVar16;
    } while (iVar19 <= iVar6);
  }
  *(undefined8 *)(puVar16 + -8) = 0x141fa7c87;
  return;
}



//===========================================================
// FUN_141fb9cf0 @ 141fb9cf0   (162 bytes)
//===========================================================

longlong FUN_141fb9cf0(longlong param_1,uint param_2)

{
  longlong lVar1;
  
  if ((*(uint *)(param_1 + 0x4d8) != 0) && (param_2 == *(uint *)(param_1 + 0x4d8))) {
    return param_1 + 0x348;
  }
  if ((*(uint *)(param_1 + 0x4dc) != 0) && (param_2 == *(uint *)(param_1 + 0x4dc))) {
    return param_1 + 0x370;
  }
  lVar1 = *(longlong *)(param_1 + 0x380);
  if (lVar1 != 0) {
    if (*(uint *)(lVar1 + -8) != 0) {
      if (((int)param_2 < 0) || (*(uint *)(lVar1 + -8) <= param_2)) {
        FUN_142e54290(0xbc,param_2);
        lVar1 = *(longlong *)(param_1 + 0x380);
      }
      return lVar1 + (longlong)(int)param_2 * 8;
    }
  }
  return param_1 + 0x340;
}



//===========================================================
// FUN_141fbd740 @ 141fbd740   (105 bytes)
//===========================================================

void FUN_141fbd740(ulonglong *param_1)

{
  ulonglong uVar1;
  ulonglong uVar2;
  
  uVar1 = *param_1;
  if (uVar1 != 0) {
    uVar2 = *(longlong *)(uVar1 - 8) * 0x130 + uVar1;
    if (uVar1 < uVar2) {
      do {
        FUN_141fbc290(uVar1);
        uVar1 = uVar1 + 0x130;
      } while (uVar1 < uVar2);
      uVar1 = *param_1;
    }
    thunk_FUN_140205820(uVar1 - 8,0);
    *param_1 = 0;
  }
  return;
}



//===========================================================
// __chkstk @ 142ef4450   (78 bytes)
//===========================================================

/* WARNING: This is an inlined function */
/* Library Function - Single Match
    __chkstk
   
   Libraries: Visual Studio 2005, Visual Studio 2008, Visual Studio 2010, Visual Studio 2012 */

void __chkstk(void)

{
  undefined1 *in_RAX;
  undefined1 *puVar1;
  undefined1 *puVar2;
  longlong unaff_GS_OFFSET;
  undefined1 local_res8 [32];
  
  puVar1 = local_res8 + -(longlong)in_RAX;
  if (local_res8 < in_RAX) {
    puVar1 = (undefined1 *)0x0;
  }
  puVar2 = *(undefined1 **)(unaff_GS_OFFSET + 0x10);
  if (puVar1 < puVar2) {
    do {
      puVar2 = puVar2 + -0x1000;
      *puVar2 = 0;
    } while ((undefined1 *)((ulonglong)puVar1 & 0xfffffffffffff000) != puVar2);
  }
  return;
}



//===========================================================
// FUN_142e541f0 @ 142e541f0   (146 bytes)
//===========================================================

void FUN_142e541f0(undefined4 param_1,undefined8 param_2)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18 [2];
  longlong local_res20;
  
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    FUN_142e5cd30("LogCallStack4",&DAT_1434997dc,local_res18,&DAT_1434997f8,local_res8,"Info1",
                  &local_res10,&local_res20);
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// __security_check_cookie @ 142ef44b0   (30 bytes)
//===========================================================

/* WARNING: This is an inlined function */

void __cdecl __security_check_cookie(uintptr_t _StackCookie)

{
  if ((_StackCookie == DAT_143a8b908) && ((short)(_StackCookie >> 0x30) == 0)) {
    return;
  }
  FUN_142ef3e44(_StackCookie);
  return;
}



//===========================================================
// FUN_142cbe730 @ 142cbe730   (8 bytes)
//===========================================================

undefined8 FUN_142cbe730(longlong param_1)

{
  return *(undefined8 *)(param_1 + 0x2358);
}



//===========================================================
// FUN_14038c960 @ 14038c960   (512 bytes)
//===========================================================

undefined8 FUN_14038c960(longlong param_1,int param_2,longlong *param_3,undefined8 *param_4)

{
  undefined *puVar1;
  int iVar2;
  undefined4 uVar3;
  longlong lVar4;
  longlong *plVar5;
  int *piVar6;
  undefined8 uVar7;
  IUnknown *local_78;
  longlong *local_70;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  IUnknown *local_50;
  uint local_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 local_38;
  
  FUN_14039f600(param_1,&local_50,param_2,0);
  if (local_50 == (IUnknown *)0x0) {
    uVar7 = 0;
  }
  else {
    lVar4 = FUN_140841970(param_2);
    puVar1 = PTR_u_price_143a46298;
    if (lVar4 == 0) {
      local_78 = local_50;
      (**(code **)(*(longlong *)local_50 + 8))(local_50);
      lVar4 = FUN_140911500(&local_78,puVar1,0);
      *param_3 = lVar4;
      plVar5 = (longlong *)FUN_1401a5890(&local_78,PTR_u_unitPrice_143a46528);
      local_70 = plVar5;
      (*DAT_143262a20)(&local_48);
      uVar7 = 0;
      if ((undefined8 *)*plVar5 != (undefined8 *)0x0) {
        uVar7 = *(undefined8 *)*plVar5;
      }
      iVar2 = (**(code **)(*(longlong *)local_50 + 0x28))(local_50,uVar7,&local_48);
      if (iVar2 < 0) {
        _com_issue_errorex(iVar2,local_50,(_GUID *)&DAT_143272478);
      }
      local_68 = local_48;
      uStack_64 = uStack_44;
      uStack_60 = uStack_40;
      uStack_5c = uStack_3c;
      local_58 = local_38;
      local_48 = local_48 & 0xffff0000;
      FUN_1401be120(plVar5);
      uVar7 = FUN_1404161d0(&local_68,0);
      *param_4 = uVar7;
      if ((short)local_68 == 8) {
        local_68 = local_68 & 0xffff0000;
        if (CONCAT44(uStack_5c,uStack_60) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_5c,uStack_60) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_68);
      }
      puVar1 = PTR_u_autoPrice_143a45c00;
      if (*param_3 == 0) {
        local_78 = local_50;
        (**(code **)(*(longlong *)local_50 + 8))(local_50);
        iVar2 = FUN_140910eb0(&local_78,puVar1,0);
        if (iVar2 != 0) {
          local_78 = local_50;
          (**(code **)(*(longlong *)local_50 + 8))(local_50);
          uVar3 = FUN_140910eb0(&local_78,&DAT_143284804,0);
          local_70 = (longlong *)CONCAT44(local_70._4_4_,param_2 / 10000);
          uVar7 = FUN_1403fbf10(param_1 + 0x398,&local_70);
          local_78 = (IUnknown *)CONCAT44(local_78._4_4_,uVar3);
          piVar6 = (int *)FUN_1403fbe10(uVar7,&local_78);
          *param_3 = (longlong)*piVar6;
        }
      }
    }
    else {
      *param_3 = lVar4;
      *param_4 = 0;
    }
    uVar7 = 1;
  }
  if (local_50 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_50 + 0x10))(local_50);
  }
  return uVar7;
}



//===========================================================
// FUN_1401be120 @ 1401be120   (114 bytes)
//===========================================================

void FUN_1401be120(undefined8 *param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  
  plVar2 = (longlong *)*param_1;
  if (plVar2 != (longlong *)0x0) {
    LOCK();
    plVar1 = plVar2 + 2;
    lVar3 = *plVar1;
    *(int *)plVar1 = (int)*plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      if (*plVar2 != 0) {
        (*DAT_143ad5990)(*plVar2 + -4);
        *plVar2 = 0;
      }
      if (plVar2[1] != 0) {
        FUN_14019b4e0();
        plVar2[1] = 0;
      }
      thunk_FUN_140205820(plVar2,0x18);
    }
    *param_1 = 0;
  }
  return;
}



//===========================================================
// FUN_14030ccb0 @ 14030ccb0   (27 bytes)
//===========================================================

int FUN_14030ccb0(longlong param_1,int param_2)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 0x5d0 + (longlong)param_2 * 8);
  if (lVar1 == 0) {
    return -1;
  }
  return *(int *)(lVar1 + -8) + -1;
}



//===========================================================
// FUN_141fbd400 @ 141fbd400   (460 bytes)
//===========================================================

longlong FUN_141fbd400(ulonglong *param_1,uint param_2)

{
  longlong lVar1;
  ulonglong uVar2;
  undefined8 *puVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  uint uVar6;
  ulonglong uVar7;
  uint uVar8;
  
  uVar5 = *param_1;
  uVar8 = 0;
  uVar6 = uVar8;
  if (uVar5 != 0) {
    uVar6 = *(uint *)(uVar5 - 8);
  }
  if (param_2 == 0xffffffff) {
    param_2 = uVar6;
  }
  if (uVar5 != 0) {
    uVar7 = *(ulonglong *)(uVar5 - 0x10);
    uVar2 = ~uVar7;
    if (-1 < (longlong)uVar7) {
      uVar2 = uVar7;
    }
    if (uVar6 < (uint)((uVar2 - 8) / 0x130)) goto LAB_141fbd50c;
  }
  uVar7 = 1;
  if (uVar6 != 0) {
    uVar7 = (ulonglong)(uVar6 * 2);
  }
  if (uVar5 != 0) {
    uVar2 = *(ulonglong *)(uVar5 - 0x10);
    uVar4 = ~uVar2;
    if (-1 < (longlong)uVar2) {
      uVar4 = uVar2;
    }
    uVar8 = (uint)((uVar4 - 8) / 0x130);
  }
  if (uVar8 != (uint)uVar7) {
    uVar2 = 0;
    if (uVar5 != 0) {
      uVar2 = (ulonglong)*(uint *)(uVar5 - 8);
    }
    lVar1 = FUN_14019b780(&DAT_143ad68a0,uVar7 * 0x130 + 8);
    uVar5 = lVar1 + 8;
    if (lVar1 == 0) {
      uVar5 = 0;
    }
    if (*param_1 != 0) {
      FUN_142ef7ba0(uVar5,*param_1,uVar2 * 0x130);
      thunk_FUN_140205820(*param_1 - 8,0);
    }
    *param_1 = uVar5;
    *(ulonglong *)(uVar5 - 8) = uVar2;
  }
LAB_141fbd50c:
  *(longlong *)(*param_1 - 8) = *(longlong *)(*param_1 - 8) + 1;
  lVar1 = (longlong)(int)param_2 * 0x130;
  FUN_142ef7ba0(*param_1 + lVar1 + 0x130,*param_1 + lVar1,(ulonglong)(uVar6 - param_2) * 0x130);
  puVar3 = (undefined8 *)(*param_1 + lVar1);
  *puVar3 = 0;
  puVar3[5] = DAT_14341cb10;
  *(undefined4 *)(puVar3 + 6) = 0;
  puVar3[0xe] = 0;
  puVar3[0xf] = 0;
  puVar3[0x11] = 0;
  puVar3[0x18] = 0;
  puVar3[0x1a] = 0;
  *(undefined4 *)((longlong)puVar3 + 0xdc) = 0;
  puVar3[0x1c] = 0;
  *(undefined4 *)(puVar3 + 0x1d) = 0;
  puVar3[0x1f] = 0;
  puVar3[0x20] = 0;
  *(undefined4 *)(puVar3 + 0x21) = 0;
  puVar3[0x22] = 0;
  puVar3[0x24] = 0;
  return *param_1 + lVar1;
}



//===========================================================
// FUN_14019a5d0 @ 14019a5d0   (1057 bytes)
//===========================================================

ulonglong FUN_14019a5d0(int *param_1)

{
  uint *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  byte bVar10;
  byte bVar11;
  int iVar12;
  uint uVar13;
  byte *pbVar14;
  longlong lVar15;
  ushort uVar16;
  int iVar17;
  ushort uVar18;
  byte *pbVar19;
  uint uVar20;
  ulonglong uVar21;
  undefined1 local_res8 [8];
  undefined1 local_res10 [8];
  byte local_res18 [8];
  ushort local_res20 [4];
  undefined4 local_78;
  undefined4 local_70;
  undefined4 local_6c;
  ulonglong local_68;
  undefined8 local_60;
  longlong local_58 [3];
  
  puVar1 = *(uint **)(param_1 + 2);
  local_70 = *puVar1;
  uVar20 = 0;
  uVar13 = 0;
  local_res18[0] = (byte)puVar1[1];
  local_res20[0] = 0x9a65;
  pbVar19 = (byte *)&local_70;
  pbVar14 = (byte *)((longlong)puVar1 + 2);
  do {
    if (local_res18[0] == 0) {
      local_res18[0] = 0x2a;
    }
    bVar10 = pbVar19[(longlong)puVar1 - (longlong)&local_70];
    *pbVar19 = bVar10 ^ local_res18[0];
    bVar10 = bVar10 + local_res18[0] + 0x2a;
    uVar16 = (local_res20[0] >> 0xd) + (ushort)bVar10;
    uVar18 = local_res20[0] << 3;
    if (bVar10 == 0) {
      bVar10 = 0x2a;
    }
    bVar11 = pbVar14[-1];
    pbVar19[1] = bVar11 ^ bVar10;
    bVar11 = bVar11 + bVar10 + 0x2a;
    uVar6 = (ushort)bVar11;
    if (bVar11 == 0) {
      bVar11 = 0x2a;
    }
    local_res18[0] = *pbVar14;
    pbVar19[2] = local_res18[0] ^ bVar11;
    local_res18[0] = local_res18[0] + bVar11 + 0x2a;
    uVar7 = (ushort)local_res18[0];
    if (local_res18[0] == 0) {
      local_res18[0] = 0x2a;
    }
    bVar10 = pbVar14[1];
    pbVar19[3] = bVar10 ^ local_res18[0];
    local_res18[0] = bVar10 + 0x2a + local_res18[0];
    local_res20[0] =
         ((uVar16 | uVar18 & 0x3ff) >> 7) + (ushort)local_res18[0] |
         (((uVar18 & 0x1fff) >> 10) + uVar7 |
         (((local_res20[0] & 0x1fff) >> 10) + uVar6 | (uVar16 | uVar18) << 3) << 3) << 3;
    uVar13 = uVar13 + 4;
    pbVar19 = pbVar19 + 4;
    pbVar14 = pbVar14 + 4;
  } while (uVar13 < 4);
  FUN_140c78f50(0x56,(longlong)param_1 + 0x21a3f060fc2daf);
  uVar13 = local_70;
  lVar2 = *(longlong *)(param_1 + 2);
  uVar21 = (ulonglong)(int)local_70;
  if (((local_res20[0] != *(ushort *)(lVar2 + 8)) || ((char)param_1[1] != *(char *)(lVar2 + 5))) ||
     ((char)param_1[4] != *(char *)(lVar2 + 6))) {
    local_res8[0] = (undefined1)param_1[4];
    local_res10[0] = (undefined1)param_1[1];
    local_6c = 1;
    local_68 = uVar21;
    local_60 = FUN_1418039d0(5);
    puVar8 = (undefined8 *)
             FUN_140197ac0(local_58,&local_60,&local_6c,&local_68,local_res18,local_res20,
                           (ushort *)(lVar2 + 8),local_res10,lVar2 + 5,local_res8,lVar2 + 6);
    FUN_141804970(&DAT_143271f04,0x61,5,*puVar8);
    if (local_58[0] != 0) {
      FUN_14019f2c0(local_58[0] + -0x10);
    }
  }
  iVar12 = *param_1;
  iVar17 = iVar12 + 1;
  *param_1 = iVar17;
  if (iVar17 == (iVar17 / 0x37) * 0x37) {
    local_78 = uVar13;
    iVar12 = iVar12 + 2;
    *param_1 = iVar12;
    if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
      puVar8 = *(undefined8 **)(param_1 + 2);
      puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 **)(param_1 + 2) = puVar9;
      *puVar9 = *puVar8;
      *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar8 + 1);
      thunk_FUN_140205820(puVar8,0xc);
    }
    uVar5 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(param_1 + 2) + 4) = uVar5;
    pbVar19 = *(byte **)(param_1 + 2);
    bVar10 = pbVar19[4];
    pbVar19[8] = 0x65;
    pbVar19[9] = 0x9a;
    lVar15 = (longlong)&local_78 - (longlong)pbVar19;
    lVar2 = 1 - (longlong)pbVar19;
    lVar3 = 2 - (longlong)pbVar19;
    lVar4 = 3 - (longlong)pbVar19;
    do {
      if (bVar10 == 0) {
        bVar10 = 0x2a;
      }
      bVar11 = pbVar19[lVar15];
      *pbVar19 = bVar10 ^ bVar11;
      bVar10 = bVar10 + (bVar10 ^ bVar11) + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar10 | uVar16 << 3;
      bVar11 = 0x2a;
      if (bVar10 != 0) {
        bVar11 = bVar10;
      }
      bVar10 = pbVar19[(longlong)&local_78 + lVar2];
      pbVar19[1] = bVar11 ^ bVar10;
      bVar11 = (bVar11 ^ bVar10) + bVar11 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar11 | uVar16 << 3;
      bVar10 = 0x2a;
      if (bVar11 != 0) {
        bVar10 = bVar11;
      }
      bVar11 = pbVar19[(longlong)&local_78 + lVar3];
      pbVar19[2] = bVar10 ^ bVar11;
      bVar11 = (bVar10 ^ bVar11) + bVar10 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar11 | uVar16 << 3;
      bVar10 = 0x2a;
      if (bVar11 != 0) {
        bVar10 = bVar11;
      }
      bVar11 = pbVar19[(longlong)&local_78 + lVar4];
      pbVar19[3] = bVar10 ^ bVar11;
      bVar10 = (bVar10 ^ bVar11) + bVar10 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar10 | uVar16 << 3;
      uVar20 = uVar20 + 4;
      pbVar19 = pbVar19 + 4;
    } while (uVar20 < 4);
    uVar21 = (ulonglong)local_70;
  }
  FUN_140c79130(0x67,param_1 + 0xc0a84bdc4a);
  return uVar21 & 0xffffffff;
}



//===========================================================
// FUN_14019f2c0 @ 14019f2c0   (345 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019f2c0(int *param_1)

{
  int iVar1;
  code *UNRECOVERED_JUMPTABLE;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  int *piVar5;
  uint uVar6;
  longlong lVar7;
  bool bVar8;
  
  if (0x100000 < *param_1 + 1U) {
    FUN_142e52dd0(0x23);
  }
  LOCK();
  iVar1 = *param_1;
  *param_1 = *param_1 + -1;
  UNLOCK();
  if (1 < iVar1) {
    return;
  }
  if (*param_1 != 0) {
    FUN_142e52dd0(0x32);
  }
  pvVar2 = Self;
  UNRECOVERED_JUMPTABLE = DAT_143ad5530;
  uVar3 = *(ulonglong *)(param_1 + -2);
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x39) {
    uVar6 = (uint)(0x28 < uVar3);
  }
  else {
    if (uVar3 < 0x59) {
      uVar6 = 2;
      goto LAB_14019f332;
    }
    uVar6 = 0xffffffff;
    if (uVar3 < 0x99) {
      uVar6 = 3;
    }
  }
  if ((int)uVar6 < 0) {
    uVar4 = (*DAT_143ad5538)();
                    /* WARNING: Could not recover jumptable at 0x00014019f3a6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(uVar4,0,param_1 + -2);
    return;
  }
LAB_14019f332:
  uVar3 = (ulonglong)uVar6;
  lVar7 = (ulonglong)uVar6 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad6a58 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019f3d5:
    *(undefined4 *)(&DAT_143ad6a60 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad6a58 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad6a58 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019f3d5;
        if (*(void **)(&DAT_143ad6a58 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad6a60 + lVar7) = *(int *)(&DAT_143ad6a60 + lVar7) + 1;
  }
  piVar5 = (int *)(&DAT_143ad6a60 + lVar7);
  *(undefined8 *)param_1 = *(undefined8 *)(&DAT_143ad6a98 + uVar3 * 8);
  *(int **)(&DAT_143ad6a98 + uVar3 * 8) = param_1;
  _DAT_143ad6ad8 = *(undefined8 *)param_1;
  *(int *)(&DAT_143ad6a44 + uVar3 * 4) = *(int *)(&DAT_143ad6a44 + uVar3 * 4) + -1;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *(undefined8 *)(&DAT_143ad6a58 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_1402e3cd0 @ 1402e3cd0   (336 bytes)
//===========================================================

longlong FUN_1402e3cd0(undefined8 param_1,longlong param_2,undefined4 param_3,undefined4 param_4)

{
  longlong *plVar1;
  uint uVar2;
  longlong lVar3;
  undefined8 *local_18;
  
  lVar3 = FUN_1402e3770(param_1,param_3,param_4,param_4,0);
  if (lVar3 == 0) {
    local_18 = (undefined8 *)0x0;
    uVar2 = 2;
  }
  else {
    local_18 = *(undefined8 **)(lVar3 + 8);
    if (local_18 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)local_18[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      local_18[1] = local_18[1] + 1;
      UNLOCK();
    }
    uVar2 = 1;
  }
  *(undefined8 **)(param_2 + 8) = local_18;
  if (local_18 != (undefined8 *)0x0) {
    if (0xfffff < (ulonglong)local_18[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    local_18[1] = local_18[1] + 1;
    UNLOCK();
  }
  if (((uVar2 & 2) != 0) && (uVar2 = uVar2 & 0xfffffffd, local_18 != (undefined8 *)0x0)) {
    if (0xffffe < local_18[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = local_18 + 1;
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      (**(code **)*local_18)(local_18,1);
    }
  }
  if (((uVar2 & 1) != 0) && (local_18 != (undefined8 *)0x0)) {
    if (0xffffe < local_18[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = local_18 + 1;
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      (**(code **)*local_18)(local_18,1);
    }
  }
  return param_2;
}



//===========================================================
// FUN_140389c10 @ 140389c10   (83 bytes)
//===========================================================

undefined4 FUN_140389c10(undefined8 param_1,int param_2)

{
  longlong lVar1;
  
  if (param_2 - 5000000U < 1000000) {
    return 1;
  }
  if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
    lVar1 = FUN_140388c60();
  }
  else {
    lVar1 = FUN_14039b100();
  }
  if (lVar1 != 0) {
    return *(undefined4 *)(lVar1 + 0x18);
  }
  return 0;
}



//===========================================================
// FUN_1401a5890 @ 1401a5890   (236 bytes)
//===========================================================

undefined8 * FUN_1401a5890(undefined8 *param_1,longlong param_2)

{
  uint uVar1;
  undefined8 *puVar2;
  int *piVar3;
  longlong lVar4;
  
  puVar2 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
  if (puVar2 == (undefined8 *)0x0) {
    puVar2 = (undefined8 *)0x0;
  }
  else {
    puVar2[1] = 0;
    *(undefined4 *)(puVar2 + 2) = 1;
    if (param_2 != 0) {
      lVar4 = -1;
      do {
        lVar4 = lVar4 + 1;
      } while (*(short *)(param_2 + lVar4 * 2) != 0);
      uVar1 = (int)lVar4 + 1;
      piVar3 = (int *)(*DAT_143ad5980)((ulonglong)uVar1 * 2 + 4);
      if (piVar3 == (int *)0x0) {
        *puVar2 = 0;
      }
      else {
        *piVar3 = (int)lVar4 * 2;
        piVar3 = piVar3 + 1;
        FUN_142ef7ba0(piVar3,param_2,(ulonglong)uVar1 * 2);
        *puVar2 = piVar3;
        if (piVar3 != (int *)0x0) goto LAB_1401a5940;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    *puVar2 = 0;
  }
LAB_1401a5940:
  *param_1 = puVar2;
  if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x8007000e);
  }
  return param_1;
}



//===========================================================
// FUN_140398ba0 @ 140398ba0   (344 bytes)
//===========================================================

longlong * FUN_140398ba0(longlong param_1,longlong *param_2,int param_3)

{
  undefined2 *puVar1;
  undefined4 *puVar2;
  longlong lVar3;
  undefined *local_res10 [3];
  
  local_res10[0] = PTR_DAT_143a49018;
  if (param_1 == 0) {
    *param_2 = 0;
    puVar2 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x14);
    puVar2[1] = 3;
    *puVar2 = 0xffffffff;
    *param_2 = (longlong)(puVar2 + 4);
    puVar2[2] = 0;
    *(undefined1 *)*param_2 = 0;
    puVar1 = (undefined2 *)*param_2;
    *puVar1 = DAT_1432841a0;
    *(undefined1 *)(puVar1 + 1) = DAT_1432841a2;
    lVar3 = *param_2;
    if (*(int *)(lVar3 + -0x10) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (*(int *)(lVar3 + -0xc) < 3) {
      FUN_142e54290(0x90,*(int *)(lVar3 + -0xc),3);
    }
    *(undefined4 *)(lVar3 + -0x10) = 1;
    *(undefined1 *)(*param_2 + 3) = 0;
    if (*(int *)(lVar3 + -0xc) + 1 < 4) {
      FUN_142e54290(0x9c,3);
    }
    *(undefined4 *)(lVar3 + -8) = 3;
    return param_2;
  }
  if (*(longlong *)(param_1 + 0xb8) != 0) {
    for (lVar3 = *(longlong *)
                  (*(longlong *)(param_1 + 0xb8) +
                  ((ulonglong)(longlong)param_3 % (ulonglong)*(uint *)(param_1 + 0xc0)) * 8);
        lVar3 != 0; lVar3 = *(longlong *)(lVar3 + 8)) {
      if (*(int *)(lVar3 + 0x10) == param_3) {
        if ((lVar3 + 0x18 != 0) && (lVar3 = FUN_14019bc60(lVar3 + 0x18,local_res10), lVar3 != 0)) {
          *param_2 = 0;
          FUN_14019a260(param_2,lVar3);
          return param_2;
        }
        break;
      }
    }
  }
  *param_2 = 0;
  return param_2;
}



//===========================================================
// ceil @ 142f23580   (194 bytes)
//===========================================================

/* Library Function - Single Match
    ceil
   
   Libraries: Visual Studio 2015 Release, Visual Studio 2017 Release, Visual Studio 2019 Release */

double __cdecl ceil(double _X)

{
  double dVar1;
  double dVar2;
  
  dVar1 = ABS(_X);
  dVar2 = _X;
  if ((ulonglong)dVar1 < 0x4340000000000000) {
    if ((ulonglong)dVar1 < 0x3ff0000000000000) {
      if ((dVar1 != 0.0) && (dVar2 = DAT_1434b95e0, _X != dVar1)) {
        dVar2 = -0.0;
      }
    }
    else {
      dVar2 = (double)(~((1L << (0x33U - (char)((ulonglong)_X >> 0x34) & 0x3f)) - 1U) &
                      (ulonglong)_X);
      if ((_X == dVar1) && (dVar2 != _X)) {
        dVar2 = dVar2 + DAT_1434b95e0;
      }
    }
  }
  else if (0x7ff0000000000000 < (ulonglong)dVar1) {
    dVar2 = (double)_handle_nan(_X);
    return dVar2;
  }
  return dVar2;
}



//===========================================================
// FUN_14038a860 @ 14038a860   (116 bytes)
//===========================================================

undefined8 FUN_14038a860(undefined8 param_1,undefined4 param_2)

{
  undefined *puVar1;
  int iVar2;
  undefined8 uVar3;
  longlong *local_res18;
  longlong *local_res20;
  
  FUN_14039f600(param_1,&local_res20,param_2,0);
  puVar1 = PTR_u_quest_143a45ad8;
  if (local_res20 != (longlong *)0x0) {
    local_res18 = local_res20;
    (**(code **)(*local_res20 + 8))(local_res20);
    iVar2 = FUN_140910eb0(&local_res18,puVar1,0);
    if (iVar2 != 0) {
      uVar3 = 1;
      goto LAB_14038a8b8;
    }
  }
  uVar3 = 0;
LAB_14038a8b8:
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))(local_res20);
  }
  return uVar3;
}



//===========================================================
// FUN_14019b4e0 @ 14019b4e0   (288 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019b4e0(undefined8 *param_1)

{
  code *pcVar1;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  int *piVar6;
  longlong lVar7;
  bool bVar8;
  
  pvVar2 = Self;
  pcVar1 = DAT_143ad5530;
  if (param_1 == (undefined8 *)0x0) {
    return;
  }
  uVar3 = param_1[-1];
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x21) {
    uVar5 = (ulonglong)(0x10 < uVar3);
  }
  else {
    if (uVar3 < 0x41) {
      uVar5 = 2;
      goto LAB_14019b52b;
    }
    uVar5 = 0xffffffff;
    if (uVar3 < 0x81) {
      uVar5 = 3;
    }
  }
  if ((int)uVar5 < 0) {
    uVar4 = (*DAT_143ad5538)();
    (*pcVar1)(uVar4,0,param_1 + -1);
    return;
  }
LAB_14019b52b:
  lVar7 = uVar5 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad68c8 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad68c8 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019b5b5:
    *(undefined4 *)(&DAT_143ad68d0 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad68c8 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad68c8 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad68c8 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019b5b5;
        if (*(void **)(&DAT_143ad68c8 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad68d0 + lVar7) = *(int *)(&DAT_143ad68d0 + lVar7) + 1;
  }
  piVar6 = (int *)(&DAT_143ad68d0 + lVar7);
  *param_1 = *(undefined8 *)(&DAT_143ad6908 + uVar5 * 8);
  *(undefined8 **)(&DAT_143ad6908 + uVar5 * 8) = param_1;
  _DAT_143ad6948 = *param_1;
  *(int *)(&DAT_143ad68b4 + uVar5 * 4) = *(int *)(&DAT_143ad68b4 + uVar5 * 4) + -1;
  *piVar6 = *piVar6 + -1;
  if (*piVar6 == 0) {
    *(undefined8 *)(&DAT_143ad68c8 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_14038bf90 @ 14038bf90   (96 bytes)
//===========================================================

undefined4 FUN_14038bf90(undefined8 param_1,int param_2)

{
  longlong lVar1;
  
  if ((param_2 - 0x2206f0U < 10000) || (param_2 - 5000000U < 1000000)) {
    return 1;
  }
  if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
    lVar1 = FUN_140388c60();
  }
  else {
    lVar1 = FUN_14039b100();
  }
  if (lVar1 != 0) {
    return *(undefined4 *)(lVar1 + 0x30);
  }
  return 0;
}



//===========================================================
// FUN_1403e1180 @ 1403e1180   (705 bytes)
//===========================================================

longlong FUN_1403e1180(undefined8 param_1,int param_2,undefined8 param_3)

{
  longlong *plVar1;
  longlong *plVar2;
  IUnknown *pIVar3;
  int iVar4;
  longlong *plVar5;
  longlong lVar6;
  longlong lVar7;
  undefined4 local_res20 [2];
  longlong *local_88;
  longlong *local_80 [2];
  IUnknown *local_70;
  IUnknown *local_68;
  short local_60 [4];
  longlong local_58;
  longlong *local_48;
  longlong *local_40;
  longlong *local_38;
  longlong *local_30;
  
  FUN_14039f600(param_1,&local_38,param_3,0);
  plVar5 = local_38;
  local_48 = local_38;
  if (local_38 != (longlong *)0x0) {
    (**(code **)(*local_38 + 8))(local_38);
  }
  FUN_14090f200(&local_70,&local_48,L"npcShopSellLimitByTemplateID");
  pIVar3 = local_70;
  lVar6 = 0;
  if (local_70 != (IUnknown *)0x0) {
    local_40 = (longlong *)0x0;
    iVar4 = (**(code **)(*(longlong *)local_70 + 0x38))(local_70,&local_40);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar3,(_GUID *)&DAT_143272478);
    }
    plVar5 = local_40;
    local_30 = local_40;
    FUN_14023b290(local_80,&local_30);
    if (plVar5 != (longlong *)0x0) {
      (**(code **)(*plVar5 + 0x10))(plVar5);
    }
    (*DAT_143262a20)(local_60);
    local_res20[0] = 0;
    plVar5 = local_80[0];
    while( true ) {
      if (plVar5 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      iVar4 = (**(code **)(*plVar5 + 0x18))(plVar5,1,local_60,local_res20);
      lVar7 = 1;
      if (iVar4 != 0) break;
      FUN_1401a5cf0(&local_88,local_60);
      plVar1 = local_88;
      lVar7 = lVar6;
      if (local_88 != (longlong *)0x0) {
        lVar7 = *local_88;
      }
      local_68 = local_70;
      if (local_70 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_70 + 8))();
      }
      iVar4 = FUN_140910eb0(&local_68,lVar7,0);
      plVar2 = local_88;
      if (iVar4 == param_2) {
        lVar7 = lVar6;
        if (plVar1 != (longlong *)0x0) {
          LOCK();
          plVar1 = plVar1 + 2;
          lVar6 = *plVar1;
          *(int *)plVar1 = (int)*plVar1 + -1;
          UNLOCK();
          plVar5 = local_80[0];
          if ((int)lVar6 == 1) {
            if (*local_88 != 0) {
              (*DAT_143ad5990)(*local_88 + -4);
              *plVar2 = 0;
            }
            if (plVar2[1] != 0) {
              FUN_14019b4e0();
              plVar2[1] = 0;
            }
            thunk_FUN_140205820(plVar2,0x18);
            plVar5 = local_80[0];
          }
        }
        break;
      }
      if (plVar1 != (longlong *)0x0) {
        LOCK();
        plVar1 = plVar1 + 2;
        lVar7 = *plVar1;
        *(int *)plVar1 = (int)*plVar1 + -1;
        UNLOCK();
        if ((int)lVar7 == 1) {
          if (*local_88 != 0) {
            (*DAT_143ad5990)(*local_88 + -4);
            *plVar2 = 0;
          }
          if (plVar2[1] != 0) {
            FUN_14019b4e0();
            plVar2[1] = 0;
          }
          thunk_FUN_140205820(plVar2,0x18);
        }
        local_88 = (longlong *)0x0;
        plVar5 = local_80[0];
      }
      if (local_60[0] == 8) {
        local_60[0] = 0;
        if (local_58 != 0) {
          (*DAT_143ad5990)(local_58 + -4);
        }
      }
      else {
        iVar4 = (*DAT_143262a18)(local_60);
        if (iVar4 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar4);
        }
      }
    }
    if (local_60[0] == 8) {
      local_60[0] = 0;
      if (local_58 != 0) {
        (*DAT_143ad5990)(local_58 + -4);
      }
    }
    else {
      (*DAT_143262a18)(local_60);
    }
    (**(code **)(*plVar5 + 0x10))(plVar5);
    plVar5 = local_38;
    lVar6 = lVar7;
  }
  if (local_70 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_70 + 0x10))(local_70);
  }
  if (plVar5 != (longlong *)0x0) {
    (**(code **)(*plVar5 + 0x10))(plVar5);
  }
  return lVar6;
}



//===========================================================
// FUN_142e52ed0 @ 142e52ed0   (4890 bytes)
//===========================================================

void FUN_142e52ed0(undefined4 param_1,undefined8 param_2)

{
  char *pcVar1;
  longlong lVar2;
  char cVar3;
  undefined8 uVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  int *piVar8;
  int iVar9;
  int iVar10;
  int *piVar11;
  int *piVar12;
  int *piVar13;
  int iVar14;
  int iVar15;
  longlong lVar16;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18;
  undefined4 local_res20 [2];
  char *local_a8;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50 [2];
  
  piVar13 = (int *)0x0;
  iVar9 = 0;
  local_res18 = 0;
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar3 = FUN_142e559e0();
  if (cVar3 == '\0') {
    return;
  }
  FUN_140194c60(&local_78);
  local_res20[0] = FUN_14091a3e0(&local_78);
  uVar4 = FUN_142a1d8a0(local_50);
  local_a8 = (char *)0x0;
  local_res18 = 1;
  FUN_1408bc980(&local_98,uVar4);
  lVar2 = local_98;
  pcVar1 = local_a8;
  local_res18 = 7;
  piVar12 = (int *)0xffffffffffffffff;
  if (local_98 != 0) {
    iVar10 = *(int *)(local_98 + -8);
    piVar8 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar11 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e530bb;
      if (*local_a8 == '\0') {
        if ((local_a8 == (char *)0x0) ||
           (piVar11 = (int *)(local_a8 + -0x10), piVar11 == (int *)0x0)) {
LAB_142e530bb:
          if (iVar15 < iVar10) {
            iVar15 = iVar10;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
          puVar5[1] = iVar15;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          puVar5[2] = 0;
          *local_a8 = '\0';
          if (piVar11 != (int *)0x0) {
            FUN_14019f2c0(piVar11);
          }
        }
        else {
          if ((1 < *piVar11) || (*(int *)(local_a8 + -0xc) < iVar10)) {
            iVar15 = *(int *)(local_a8 + -8);
            goto LAB_142e530bb;
          }
          if (*piVar11 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar11 = -1;
        }
        FUN_142ef7ba0(local_a8,lVar2,piVar8);
        pcVar1 = local_a8;
        if (*(int *)(local_a8 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
          if (iVar10 != -1) goto LAB_142e53148;
          piVar8 = piVar13;
          if (pcVar1 != (char *)0x0) {
            do {
              piVar12 = (int *)((longlong)piVar12 + 1);
              piVar8 = piVar12;
            } while (pcVar1[(longlong)piVar12] != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
LAB_142e53148:
          *(char *)((longlong)piVar8 + (longlong)local_a8) = '\0';
        }
        iVar10 = (int)piVar8;
        if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
        }
        *(int *)(pcVar1 + -8) = iVar10;
        goto LAB_142e53173;
      }
      iVar15 = *(int *)(local_a8 + -8);
      for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
      }
      piVar12 = (int *)(local_a8 + -0x10);
      iVar14 = iVar9;
      if (piVar12 == (int *)0x0) {
LAB_142e52fbf:
        if (iVar14 < iVar7) {
          iVar14 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        puVar5[1] = iVar14;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        if (piVar12 == (int *)0x0) {
          puVar5[2] = 0;
          *local_a8 = '\0';
        }
        else {
          iVar7 = *(int *)(pcVar1 + -8) + 1;
          if (iVar14 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar14 + 1);
            iVar7 = iVar14 + 1;
          }
          FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
          puVar5[2] = *(undefined4 *)(pcVar1 + -8);
          local_a8[iVar14] = '\0';
          FUN_14019f2c0(piVar12);
        }
      }
      else {
        if ((1 < *piVar12) || (*(int *)(local_a8 + -0xc) < iVar7)) {
          iVar14 = *(int *)(local_a8 + -8);
          goto LAB_142e52fbf;
        }
        if (*piVar12 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar12 = -1;
      }
      iVar7 = iVar9;
      if (local_a8 != (char *)0x0) {
        iVar7 = *(int *)(local_a8 + -8);
      }
      FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar8);
      FUN_14019c870(&local_a8,iVar15 + iVar10);
    }
  }
LAB_142e53173:
  piVar12 = (int *)0xffffffffffffffff;
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  local_70 = 0;
  uVar4 = FUN_14019ba10(&local_70,&DAT_143272338,"LogCallStack3");
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x27;
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0x1f;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar8 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar11 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e53377;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar12 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar12 == (int *)0x0) {
LAB_142e5327f:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar12 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar12);
          }
        }
        else {
          if ((1 < *piVar12) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e5327f;
          }
          if (*piVar12 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar12 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar8);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e5342f;
      }
      if ((local_a8 == (char *)0x0) || (piVar11 = (int *)(local_a8 + -0x10), piVar11 == (int *)0x0))
      {
LAB_142e53377:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar11 != (int *)0x0) {
          FUN_14019f2c0(piVar11);
        }
      }
      else {
        if ((1 < *piVar11) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e53377;
        }
        if (*piVar11 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar11 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar8);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e53407;
        piVar8 = piVar13;
        if (pcVar1 != (char *)0x0) {
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
            piVar8 = piVar12;
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e53407:
        *(char *)((longlong)piVar8 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar8;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e5342f:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  local_68 = 0;
  uVar4 = FUN_14019ba10(&local_68,&DAT_143272338,&DAT_1434997dc);
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x11f;
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0xdf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e5363e;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar8 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar8 == (int *)0x0) {
LAB_142e5353f:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar8 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e5353f;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e536fd;
      }
      if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e5363e:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar8 != (int *)0x0) {
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e5363e;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar12);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e536d5;
        piVar12 = piVar13;
        if (pcVar1 != (char *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e536d5:
        *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar12;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e536fd:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc140(&local_90,local_res20);
  lVar2 = local_90;
  pcVar1 = local_a8;
  local_res18 = 0x6df;
  if (local_90 != 0) {
    iVar10 = *(int *)(local_90 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e538ca;
      if (*local_a8 == '\0') {
        if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0))
        {
LAB_142e538ca:
          if (iVar15 < iVar10) {
            iVar15 = iVar10;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
          puVar5[1] = iVar15;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          puVar5[2] = 0;
          *local_a8 = '\0';
          if (piVar8 != (int *)0x0) {
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
            iVar15 = *(int *)(local_a8 + -8);
            goto LAB_142e538ca;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        FUN_142ef7ba0(local_a8,lVar2,piVar12);
        pcVar1 = local_a8;
        if (*(int *)(local_a8 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
          if (iVar10 != -1) goto LAB_142e5395e;
          piVar12 = piVar13;
          if (pcVar1 != (char *)0x0) {
            piVar12 = (int *)0xffffffffffffffff;
            do {
              piVar12 = (int *)((longlong)piVar12 + 1);
            } while (pcVar1[(longlong)piVar12] != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
LAB_142e5395e:
          *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
        }
        iVar10 = (int)piVar12;
        if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
        }
        *(int *)(pcVar1 + -8) = iVar10;
        goto LAB_142e53989;
      }
      iVar15 = *(int *)(local_a8 + -8);
      for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
      }
      piVar8 = (int *)(local_a8 + -0x10);
      iVar14 = iVar9;
      if (piVar8 == (int *)0x0) {
LAB_142e537cf:
        if (iVar14 < iVar7) {
          iVar14 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        puVar5[1] = iVar14;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        if (piVar8 == (int *)0x0) {
          puVar5[2] = 0;
          *local_a8 = '\0';
        }
        else {
          iVar7 = *(int *)(pcVar1 + -8) + 1;
          if (iVar14 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar14 + 1);
            iVar7 = iVar14 + 1;
          }
          FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
          puVar5[2] = *(undefined4 *)(pcVar1 + -8);
          local_a8[iVar14] = '\0';
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
          iVar14 = *(int *)(local_a8 + -8);
          goto LAB_142e537cf;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      iVar7 = iVar9;
      if (local_a8 != (char *)0x0) {
        iVar7 = *(int *)(local_a8 + -8);
      }
      FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
      FUN_14019c870(&local_a8,iVar15 + iVar10);
    }
  }
LAB_142e53989:
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  local_60 = 0;
  uVar4 = FUN_14019ba10(&local_60,&DAT_143272338,&DAT_1434997f8);
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x26df;
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0x1edf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e53b90;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar8 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar8 == (int *)0x0) {
LAB_142e53a91:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar8 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e53a91;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e53c4f;
      }
      if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e53b90:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar8 != (int *)0x0) {
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e53b90;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar12);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e53c27;
        piVar12 = piVar13;
        if (pcVar1 != (char *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e53c27:
        *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar12;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e53c4f:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc020(&local_88,local_res8);
  lVar2 = local_88;
  pcVar1 = local_a8;
  local_res18 = 0xdedf;
  if (local_88 == 0) goto LAB_142e53edb;
  iVar10 = *(int *)(local_88 + -8);
  piVar12 = (int *)(longlong)iVar10;
  if (iVar10 == 0) goto LAB_142e53edb;
  piVar8 = piVar13;
  iVar15 = iVar9;
  if (local_a8 == (char *)0x0) goto LAB_142e53e1c;
  if (*local_a8 == '\0') {
    if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e53e1c:
      if (iVar15 < iVar10) {
        iVar15 = iVar10;
      }
      puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
      puVar5[1] = iVar15;
      *puVar5 = 0xffffffff;
      local_a8 = (char *)(puVar5 + 4);
      puVar5[2] = 0;
      *local_a8 = '\0';
      if (piVar8 != (int *)0x0) {
        FUN_14019f2c0(piVar8);
      }
    }
    else {
      if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
        iVar15 = *(int *)(local_a8 + -8);
        goto LAB_142e53e1c;
      }
      if (*piVar8 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar8 = -1;
    }
    piVar8 = (int *)0xffffffffffffffff;
    FUN_142ef7ba0(local_a8,lVar2,piVar12);
    pcVar1 = local_a8;
    if (*(int *)(local_a8 + -0x10) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
      if (iVar10 != -1) goto LAB_142e53eb0;
      if (pcVar1 != (char *)0x0) {
        do {
          piVar13 = (int *)((longlong)piVar8 + 1);
          piVar8 = piVar13;
        } while (pcVar1[(longlong)piVar13] != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
LAB_142e53eb0:
      *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      piVar13 = piVar12;
    }
    iVar10 = (int)piVar13;
    if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
      FUN_142e54290(0x9c,(ulonglong)piVar13 & 0xffffffff);
    }
    *(int *)(pcVar1 + -8) = iVar10;
    goto LAB_142e53edb;
  }
  iVar15 = *(int *)(local_a8 + -8);
  for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
  }
  piVar13 = (int *)(local_a8 + -0x10);
  iVar14 = iVar9;
  if (piVar13 == (int *)0x0) {
LAB_142e53d21:
    if (iVar14 < iVar7) {
      iVar14 = iVar7;
    }
    puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
    puVar5[1] = iVar14;
    *puVar5 = 0xffffffff;
    local_a8 = (char *)(puVar5 + 4);
    if (piVar13 == (int *)0x0) {
      puVar5[2] = 0;
      *local_a8 = '\0';
    }
    else {
      iVar7 = *(int *)(pcVar1 + -8) + 1;
      if (iVar14 + 1 < iVar7) {
        FUN_142e54290(0x5c,iVar7,iVar14 + 1);
        iVar7 = iVar14 + 1;
      }
      FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
      puVar5[2] = *(undefined4 *)(pcVar1 + -8);
      local_a8[iVar14] = '\0';
      FUN_14019f2c0(piVar13);
    }
  }
  else {
    if ((1 < *piVar13) || (*(int *)(local_a8 + -0xc) < iVar7)) {
      iVar14 = *(int *)(local_a8 + -8);
      goto LAB_142e53d21;
    }
    if (*piVar13 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar13 = -1;
  }
  iVar7 = iVar9;
  if (local_a8 != (char *)0x0) {
    iVar7 = *(int *)(local_a8 + -8);
  }
  FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
  FUN_14019c870(&local_a8,iVar15 + iVar10);
LAB_142e53edb:
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  local_58 = 0;
  uVar4 = FUN_14019ba10(&local_58,&DAT_143272338,"Info1");
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x4dedf;
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  lVar2 = local_a0;
  local_res18 = 0x3dedf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        iVar15 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar15 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar15 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc3a0(&local_80,&local_res10);
  lVar2 = local_80;
  local_res18 = 0x1bdedf;
  if (local_80 != 0) {
    iVar10 = *(int *)(local_80 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        iVar15 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar15 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar15 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  FUN_1408bc980(&local_a0,&local_78);
  lVar2 = local_a0;
  local_res18 = 0x7bdedf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        if (local_a8 != (char *)0x0) {
          iVar9 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar9 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  FUN_142a1ec10(&local_a8);
  if (local_a8 != (char *)0x0) {
    FUN_14019f2c0(local_a8 + -0x10);
  }
  if (local_50[0] != 0) {
    FUN_14019f2c0(local_50[0] + -0x10);
  }
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  return;
}



//===========================================================
// thunk_FUN_140205820 @ 142ef3bb8   (5 bytes)
//===========================================================

void thunk_FUN_140205820(undefined8 param_1)

{
  FUN_14019bb50(&DAT_143ad68a0,param_1);
  return;
}



//===========================================================
// FUN_142e54290 @ 142e54290   (185 bytes)
//===========================================================

void FUN_142e54290(undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  undefined4 local_res20 [2];
  longlong local_18 [3];
  
  local_res8[0] = param_1;
  local_res10[0] = param_2;
  local_res18[0] = param_3;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(local_18);
    local_res20[0] = FUN_14091a3e0(local_18);
    FUN_142e5c990("LogCallStack5",&DAT_1434997dc,local_res20,&DAT_1434997f8,local_res8,"Info1",
                  local_res10,"info2",local_res18,local_18);
    if (local_18[0] != 0) {
      FUN_14019f2c0(local_18[0] + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_141fbc290 @ 141fbc290   (152 bytes)
//===========================================================

void FUN_141fbc290(longlong param_1)

{
  FUN_1401abd80(param_1 + 0x118);
  FUN_1401be120(param_1 + 0x110);
  if (*(longlong *)(param_1 + 0xf8) != 0) {
    FUN_14019f2c0(*(longlong *)(param_1 + 0xf8) + -0x10);
  }
  if (*(longlong *)(param_1 + 0xe0) != 0) {
    FUN_14019f2c0(*(longlong *)(param_1 + 0xe0) + -0x10);
  }
  if (*(longlong *)(param_1 + 0xd0) != 0) {
    FUN_14019f2c0(*(longlong *)(param_1 + 0xd0) + -0x10);
  }
  if (*(longlong *)(param_1 + 0xc0) != 0) {
    FUN_14019f2c0(*(longlong *)(param_1 + 0xc0) + -0x10);
  }
  FUN_1404bb3d0(param_1 + 0x80);
  FUN_1404bb440(param_1 + 0x68);
  return;
}



//===========================================================
// FUN_142e5cd30 @ 142e5cd30   (764 bytes)
//===========================================================

void FUN_142e5cd30(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8)

{
  undefined8 uVar1;
  undefined1 local_84;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  longlong local_38;
  longlong local_30;
  longlong local_28;
  longlong local_20;
  
  uVar1 = FUN_142a1d8a0(&local_20);
  local_80 = 0;
  FUN_140198700(&local_80,uVar1,local_84);
  local_58 = 0;
  uVar1 = FUN_14019ba10(&local_58,&DAT_143272338,param_1);
  local_78 = 0;
  FUN_14019a260(&local_78,uVar1);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_78);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  local_50 = 0;
  uVar1 = FUN_14019ba10(&local_50,&DAT_143272338,param_2);
  local_70 = 0;
  FUN_14019a260(&local_70,uVar1);
  if (local_50 != 0) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_70);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  FUN_1408bc140(&local_48,param_3);
  FUN_1401a1c50(&local_80,&local_48);
  if (local_48 != 0) {
    FUN_14019f2c0(local_48 + -0x10);
  }
  local_40 = 0;
  uVar1 = FUN_14019ba10(&local_40,&DAT_143272338,param_4);
  local_68 = 0;
  FUN_14019a260(&local_68,uVar1);
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_68);
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  FUN_1408bc020(&local_38,param_5);
  FUN_1401a1c50(&local_80,&local_38);
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
  local_30 = 0;
  uVar1 = FUN_14019ba10(&local_30,&DAT_143272338,param_6);
  local_60 = 0;
  FUN_14019a260(&local_60,uVar1);
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_60);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  FUN_1408bc280(&local_28,param_7);
  FUN_1401a1c50(&local_80,&local_28);
  if (local_28 != 0) {
    FUN_14019f2c0(local_28 + -0x10);
  }
  FUN_140198700(&local_80,param_8,local_84);
  FUN_142a1ec10(&local_80);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  if (local_20 != 0) {
    FUN_14019f2c0(local_20 + -0x10);
  }
  return;
}



//===========================================================
// FUN_14091a3e0 @ 14091a3e0   (62 bytes)
//===========================================================

uint FUN_14091a3e0(undefined8 *param_1)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  ulonglong uVar4;
  
  pbVar3 = (byte *)*param_1;
  if (pbVar3 != (byte *)0x0) {
    uVar2 = 0x811c9dc5;
    if (*(uint *)(pbVar3 + -8) != 0) {
      uVar4 = (ulonglong)*(uint *)(pbVar3 + -8);
      do {
        bVar1 = *pbVar3;
        pbVar3 = pbVar3 + 1;
        uVar2 = (bVar1 ^ uVar2) * 0x1000193;
        uVar4 = uVar4 - 1;
      } while (uVar4 != 0);
    }
    return uVar2;
  }
  return 0x811c9dc5;
}



//===========================================================
// FUN_142e559e0 @ 142e559e0   (127 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_142e559e0(void)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  
  iVar2 = FUN_14090d160(0x116,1);
  if (iVar2 != 0) {
    cVar1 = FUN_14090d340(0x11a);
    if (cVar1 != '\0') {
      _DAT_00000000 = 1;
    }
    if (DAT_143ae1514 < 3) {
      DAT_143ae1514 = DAT_143ae1514 + 1;
      return 1;
    }
    uVar3 = (*DAT_143262db0)();
    cVar1 = FUN_1408fcaa0(DAT_143ae1548,1800000,uVar3);
    if (cVar1 != '\0') {
      DAT_143ae1548 = uVar3;
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_140194c60 @ 140194c60   (6267 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 * FUN_140194c60(undefined8 *param_1)

{
  code *pcVar1;
  undefined8 uVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  longlong *plVar7;
  int *piVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulonglong *puVar11;
  undefined4 *puVar12;
  longlong lVar13;
  longlong lVar14;
  char *pcVar15;
  undefined8 uVar16;
  int iVar17;
  int *piVar18;
  int *piVar19;
  int *piVar20;
  ulonglong uVar21;
  undefined8 *puVar22;
  int iVar23;
  int *piVar24;
  undefined1 auStack_aa8 [32];
  int **local_a88;
  int **local_a80;
  undefined8 local_a78;
  undefined8 local_a70;
  undefined8 local_a68;
  int *local_a58;
  longlong local_a50;
  undefined8 *local_a48;
  ulonglong local_a40;
  int local_a38;
  ulonglong local_a30;
  longlong local_a28;
  undefined8 *local_a20;
  undefined4 local_a18 [2];
  undefined8 local_a10;
  undefined8 uStack_a08;
  undefined8 local_a00;
  undefined8 uStack_9f8;
  undefined8 local_9e8;
  undefined1 local_9e0 [4];
  undefined4 local_9dc;
  longlong local_9c8;
  undefined4 local_9bc;
  undefined8 local_9b8;
  undefined4 local_9ac;
  undefined4 local_8d8;
  undefined8 local_8d4;
  undefined8 uStack_8cc;
  undefined8 local_8c4;
  undefined8 uStack_8bc;
  undefined8 local_8b4;
  undefined8 uStack_8ac;
  undefined8 local_8a4;
  undefined8 uStack_89c;
  undefined8 local_894;
  undefined8 uStack_88c;
  undefined8 local_884;
  undefined8 uStack_87c;
  undefined8 local_874;
  undefined8 uStack_86c;
  undefined8 local_864;
  undefined8 uStack_85c;
  undefined8 local_854;
  undefined8 uStack_84c;
  undefined1 local_838 [48];
  undefined4 local_808;
  undefined8 local_7a0;
  longlong local_798;
  undefined8 local_740;
  int *local_368 [34];
  undefined4 local_258 [6];
  undefined4 local_240;
  undefined1 local_23c [516];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_aa8;
  piVar24 = (int *)0x0;
  iVar3 = 0;
  local_a58 = (int *)0x0;
  local_a48 = param_1;
  local_a20 = param_1;
  if (DAT_143aa8288 == '\0') {
    local_a50 = 0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a50,"Not Init\r\n");
    lVar14 = *plVar7;
    piVar18 = piVar24;
    if (lVar14 == 0) goto LAB_140194d86;
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    param_1 = local_a20;
    if (iVar4 == 0) goto LAB_140194d86;
    if (0 < iVar4) {
      iVar3 = iVar4;
    }
    piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
    piVar8[1] = iVar3;
    *piVar8 = -1;
    piVar18 = piVar8 + 4;
    piVar8[2] = 0;
    *(char *)piVar18 = '\0';
    local_a58 = piVar18;
    FUN_142ef7ba0(piVar18,lVar14,piVar20);
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
      *piVar8 = 1;
      if (iVar4 != -1) goto LAB_140194d5f;
      piVar20 = (int *)0xffffffffffffffff;
      if (piVar18 != (int *)0x0) {
        do {
          piVar24 = (int *)((longlong)piVar20 + 1);
          piVar20 = piVar24;
        } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar8[1],iVar4);
      *piVar8 = 1;
LAB_140194d5f:
      *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      piVar24 = piVar20;
    }
    iVar3 = (int)piVar24;
    if ((iVar3 < 0) || (piVar8[1] + 1 <= iVar3)) {
      FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
    }
    piVar8[2] = iVar3;
    param_1 = local_a20;
LAB_140194d86:
    if (local_a50 != 0) {
      FUN_14019f2c0(local_a50 + -0x10);
    }
    *param_1 = piVar18;
    return param_1;
  }
  FUN_142ef8250(local_838,0,0x4d0);
  local_808 = 0x10001f;
  (*DAT_143262840)(local_838);
  local_a50 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a50,&DAT_143271d00);
  lVar14 = *plVar7;
  piVar18 = piVar24;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    piVar18 = (int *)0x0;
    if (iVar4 != 0) {
      iVar6 = 0;
      if (0 < iVar4) {
        iVar6 = iVar4;
      }
      piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      piVar8[1] = iVar6;
      *piVar8 = -1;
      piVar18 = piVar8 + 4;
      piVar8[2] = 0;
      *(char *)piVar18 = '\0';
      local_a58 = piVar18;
      FUN_142ef7ba0(piVar18,lVar14,piVar20);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar4 != -1) goto LAB_140194ea0;
        piVar20 = piVar24;
        if (piVar18 != (int *)0x0) {
          piVar20 = (int *)0xffffffffffffffff;
          do {
            piVar20 = (int *)((longlong)piVar20 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar20) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],iVar4);
        *piVar8 = 1;
LAB_140194ea0:
        *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      }
      iVar4 = (int)piVar20;
      if ((iVar4 < 0) || (piVar8[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,(ulonglong)piVar20 & 0xffffffff);
      }
      piVar8[2] = iVar4;
    }
  }
  if (local_a50 != 0) {
    FUN_14019f2c0(local_a50 + -0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Major:%d Minor:%d\r\n",1);
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar4;
    if (iVar4 != 0) {
      piVar19 = piVar24;
      if (piVar18 == (int *)0x0) goto LAB_14019509b;
      if ((char)*piVar18 != '\0') {
        iVar6 = piVar18[-2];
        for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar18 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140194f9f:
          if (iVar3 < iVar17) {
            iVar3 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          local_a58 = piVar20;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar20 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar18[-2] + 1;
            iVar23 = iVar3 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
            puVar12[2] = piVar18[-2];
            *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
            iVar3 = piVar18[-2];
            goto LAB_140194f9f;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar20 == (int *)0x0) {
          iVar3 = 0;
        }
        else {
          iVar3 = piVar20[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar4);
        goto LAB_140195140;
      }
      if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019509b:
        if (iVar3 < iVar4) {
          iVar3 = iVar4;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
        puVar12[1] = iVar3;
        *puVar12 = 0xffffffff;
        piVar20 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        local_a58 = piVar20;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
          iVar3 = piVar18[-2];
          goto LAB_14019509b;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar20,lVar14,piVar8);
      if (piVar20[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
        piVar20[-4] = 1;
        if (iVar4 != -1) goto LAB_14019511d;
        if (piVar20 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar20[-3],iVar4);
        piVar20[-4] = 1;
LAB_14019511d:
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar20[-2] = iVar3;
    }
  }
LAB_140195140:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Call stack:\r\n");
  lVar14 = *plVar7;
  piVar18 = piVar20;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar3 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar3;
    if (iVar3 != 0) {
      iVar4 = 0;
      piVar19 = piVar24;
      if (piVar20 == (int *)0x0) goto LAB_14019530e;
      if ((char)*piVar20 != '\0') {
        iVar6 = piVar20[-2];
        for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar20 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140195212:
          if (iVar4 < iVar17) {
            iVar4 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          local_a58 = piVar18;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar20[-2] + 1;
            iVar23 = iVar4 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
            puVar12[2] = piVar20[-2];
            *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
            iVar4 = piVar20[-2];
            goto LAB_140195212;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar18 == (int *)0x0) {
          iVar4 = 0;
        }
        else {
          iVar4 = piVar18[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar3);
        goto LAB_1401953b3;
      }
      if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_14019530e:
        if (iVar4 < iVar3) {
          iVar4 = iVar3;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
        puVar12[1] = iVar4;
        *puVar12 = 0xffffffff;
        piVar18 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar18 = '\0';
        local_a58 = piVar18;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
          iVar4 = piVar20[-2];
          goto LAB_14019530e;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar18,lVar14,piVar8);
      if (piVar18[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
        piVar18[-4] = 1;
        if (iVar3 != -1) goto LAB_140195390;
        if (piVar18 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar18[-3],iVar3);
        piVar18[-4] = 1;
LAB_140195390:
        *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar18[-2] = iVar3;
    }
  }
LAB_1401953b3:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Address   Frame\r\n");
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 == 0) goto LAB_140195630;
  iVar3 = *(int *)(lVar14 + -8);
  piVar8 = (int *)(longlong)iVar3;
  if (iVar3 == 0) goto LAB_140195630;
  iVar4 = 0;
  piVar19 = piVar24;
  if (piVar18 == (int *)0x0) goto LAB_14019558b;
  if ((char)*piVar18 != '\0') {
    iVar6 = piVar18[-2];
    for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
    }
    piVar24 = piVar18 + -4;
    if (piVar24 == (int *)0x0) {
LAB_14019548f:
      if (iVar4 < iVar17) {
        iVar4 = iVar17;
      }
      puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
      puVar12[1] = iVar4;
      *puVar12 = 0xffffffff;
      piVar20 = puVar12 + 4;
      local_a58 = piVar20;
      if (piVar24 == (int *)0x0) {
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        lVar14 = local_a50;
      }
      else {
        iVar17 = piVar18[-2] + 1;
        iVar23 = iVar4 + 1;
        if (iVar23 < iVar17) {
          FUN_142e54290(0x5c,iVar17,iVar23);
          iVar17 = iVar23;
        }
        FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
        puVar12[2] = piVar18[-2];
        *(char *)((longlong)iVar4 + (longlong)piVar20) = '\0';
        FUN_14019f2c0(piVar24);
        lVar14 = local_a50;
      }
    }
    else {
      if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
        iVar4 = piVar18[-2];
        goto LAB_14019548f;
      }
      if (*piVar24 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar24 = -1;
    }
    if (piVar20 == (int *)0x0) {
      iVar4 = 0;
    }
    else {
      iVar4 = piVar20[-2];
    }
    FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar20),lVar14,piVar8);
    FUN_14019c870(&local_a58,iVar6 + iVar3);
    goto LAB_140195630;
  }
  if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019558b:
    if (iVar4 < iVar3) {
      iVar4 = iVar3;
    }
    puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
    puVar12[1] = iVar4;
    *puVar12 = 0xffffffff;
    piVar20 = puVar12 + 4;
    puVar12[2] = 0;
    *(char *)piVar20 = '\0';
    local_a58 = piVar20;
    if (piVar19 != (int *)0x0) {
      FUN_14019f2c0(piVar19);
    }
  }
  else {
    if ((1 < *piVar19) || (piVar18[-3] < iVar3)) {
      iVar4 = piVar18[-2];
      goto LAB_14019558b;
    }
    if (*piVar19 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar19 = -1;
  }
  FUN_142ef7ba0(piVar20,lVar14,piVar8);
  if (piVar20[-4] != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar3 == -1) || (iVar3 <= piVar20[-3])) {
    piVar20[-4] = 1;
    if (iVar3 != -1) goto LAB_14019560d;
    if (piVar20 != (int *)0x0) {
      piVar24 = (int *)0xffffffffffffffff;
      do {
        piVar24 = (int *)((longlong)piVar24 + 1);
      } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
    }
  }
  else {
    FUN_142e54290(0x90,piVar20[-3],iVar3);
    piVar20[-4] = 1;
LAB_14019560d:
    *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
    piVar24 = piVar8;
  }
  iVar3 = (int)piVar24;
  if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
    FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
  }
  piVar20[-2] = iVar3;
LAB_140195630:
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  FUN_142ef8250(local_9e0,0,0x100);
  uVar2 = DAT_143aa8260;
  uVar16 = DAT_143aa8258;
  pcVar1 = DAT_143aa8250;
  local_9e8 = local_740;
  local_9dc = 3;
  local_9b8 = local_7a0;
  local_9ac = 3;
  local_9c8 = local_798;
  local_9bc = 3;
  uVar9 = (*DAT_143ad5440)();
  uVar10 = (*DAT_143ad5408)();
  local_a68 = 0;
  local_a70 = uVar2;
  local_a78 = uVar16;
  local_a80 = (int **)0x0;
  local_a88 = (int **)local_838;
  iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  do {
    if ((iVar3 == 0) || (piVar24 = (int *)0x0, local_9c8 == 0)) {
      *local_a20 = piVar20;
      return local_a20;
    }
    local_a30 = 0;
    puVar11 = (ulonglong *)FUN_14019ba10(&local_a30,"%016X  %016X  ",local_9e8);
    uVar21 = *puVar11;
    piVar18 = piVar20;
    local_a40 = uVar21;
    if (uVar21 != 0) {
      iVar3 = *(int *)(uVar21 - 8);
      piVar8 = (int *)(longlong)iVar3;
      if (iVar3 != 0) {
        iVar4 = 0;
        piVar19 = piVar24;
        if (piVar20 == (int *)0x0) goto LAB_1401958b6;
        if ((char)*piVar20 != '\0') {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar20 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401957c2:
            if (iVar4 < iVar17) {
              iVar4 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
            puVar12[1] = iVar4;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            local_a58 = piVar18;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar18 = '\0';
              uVar21 = local_a40;
            }
            else {
              iVar17 = piVar20[-2] + 1;
              iVar23 = iVar4 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
              puVar12[2] = piVar20[-2];
              *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
              FUN_14019f2c0(piVar24);
              uVar21 = local_a40;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
              iVar4 = piVar20[-2];
              goto LAB_1401957c2;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          if (piVar18 == (int *)0x0) {
            iVar4 = 0;
          }
          else {
            iVar4 = piVar18[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),uVar21,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar3);
          goto LAB_14019595b;
        }
        if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_1401958b6:
          if (iVar4 < iVar3) {
            iVar4 = iVar3;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar18 = '\0';
          local_a58 = piVar18;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
            iVar4 = piVar20[-2];
            goto LAB_1401958b6;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar18,uVar21,piVar8);
        if (piVar18[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
          piVar18[-4] = 1;
          if (iVar3 != -1) goto LAB_140195938;
          if (piVar18 != (int *)0x0) {
            piVar24 = (int *)0xffffffffffffffff;
            do {
              piVar24 = (int *)((longlong)piVar24 + 1);
            } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar18[-3],iVar3);
          piVar18[-4] = 1;
LAB_140195938:
          *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
          piVar24 = piVar8;
        }
        iVar3 = (int)piVar24;
        if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
        }
        piVar18[-2] = iVar3;
      }
    }
LAB_14019595b:
    piVar24 = (int *)0x0;
    if (local_a30 != 0) {
      FUN_14019f2c0(local_a30 - 0x10);
    }
    local_258[0] = 0x20;
    local_240 = 0x200;
    local_a50 = 0;
    FUN_142ef8250(local_368,0,0x104);
    iVar3 = 0;
    local_a40 = local_a40 & 0xffffffff00000000;
    local_a30 = local_a30 & 0xffffffff00000000;
    local_a18[0] = 0x28;
    local_a10 = 0;
    local_a00 = 0;
    uStack_9f8 = 0;
    uStack_a08 = 0xffffffff;
    if (DAT_143aa8280 == 0) {
      local_8d8 = 0x94;
      local_8d4 = 0;
      uStack_8cc = 0;
      local_8c4 = 0;
      uStack_8bc = 0;
      local_8b4 = 0;
      uStack_8ac = 0;
      local_8a4 = 0;
      uStack_89c = 0;
      local_894 = 0;
      uStack_88c = 0;
      local_884 = 0;
      uStack_87c = 0;
      local_874 = 0;
      uStack_86c = 0;
      local_864 = 0;
      uStack_85c = 0;
      local_854 = 0;
      uStack_84c = 0;
      (*DAT_143262820)(&local_8d8);
      if (uStack_8cc._4_4_ == 2) {
        DAT_143aa8280 = (*DAT_143ad5408)();
      }
      else {
        iVar4 = (*DAT_143ad5410)();
        DAT_143aa8280 = (longlong)iVar4;
      }
      if (DAT_143aa8280 == 0) {
        local_a28 = 0;
        plVar7 = (longlong *)FUN_14019ba10(&local_a28,"m_hProcess is Null\r\n");
        puVar22 = (undefined8 *)*plVar7;
        piVar20 = piVar18;
        local_a48 = puVar22;
        if (puVar22 == (undefined8 *)0x0) goto LAB_140196462;
        iVar4 = *(int *)(puVar22 + -1);
        piVar8 = (int *)(longlong)iVar4;
        if (iVar4 == 0) goto LAB_140196462;
        piVar19 = piVar24;
        if (piVar18 == (int *)0x0) goto LAB_1401963be;
        if ((char)*piVar18 != '\0') {
          iVar6 = piVar18[-2];
          for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar18 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401962c5:
            if (iVar3 < iVar17) {
              iVar3 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar20 = puVar12 + 4;
            local_a58 = piVar20;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar20 = '\0';
              puVar22 = local_a48;
            }
            else {
              iVar17 = piVar18[-2] + 1;
              iVar23 = iVar3 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
              puVar12[2] = piVar18[-2];
              *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
              FUN_14019f2c0(piVar24);
              puVar22 = local_a48;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
              iVar3 = piVar18[-2];
              goto LAB_1401962c5;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          iVar3 = 0;
          if (piVar20 != (int *)0x0) {
            iVar3 = piVar20[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),puVar22,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
          goto LAB_140196462;
        }
        if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_1401963be:
          if (iVar3 < iVar4) {
            iVar3 = iVar4;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar20 = '\0';
          local_a58 = piVar20;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
            iVar3 = piVar18[-2];
            goto LAB_1401963be;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar20,puVar22,piVar8);
        if (piVar20[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
          piVar20[-4] = 1;
          if (iVar4 == -1) {
            piVar18 = (int *)0xffffffffffffffff;
            if (piVar20 != (int *)0x0) {
              do {
                piVar24 = (int *)((longlong)piVar18 + 1);
                piVar18 = piVar24;
              } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
            }
LAB_140196443:
            iVar3 = (int)piVar24;
            if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
              FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
            }
            piVar20[-2] = iVar3;
LAB_140196462:
            if (local_a28 != 0) {
              FUN_14019f2c0(local_a28 + -0x10);
            }
            *local_a20 = piVar20;
            return local_a20;
          }
        }
        else {
          FUN_142e54290(0x90,piVar20[-3],iVar4);
          piVar20[-4] = 1;
        }
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
        goto LAB_140196443;
      }
    }
    iVar4 = (*DAT_143aa8268)(DAT_143aa8280,local_9e8,&local_a50,local_258);
    local_a38 = iVar4;
    if (iVar4 == 0) {
      uVar5 = (*DAT_143262838)();
      local_a48 = (undefined8 *)0x0;
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"_SymGetLineFromAddr Error : %x ",uVar5);
      lVar14 = *plVar7;
      local_a28 = lVar14;
      if (lVar14 != 0) {
        iVar6 = *(int *)(lVar14 + -8);
        piVar20 = (int *)(longlong)iVar6;
        iVar4 = local_a38;
        if (iVar6 != 0) {
          piVar8 = piVar24;
          if (piVar18 == (int *)0x0) goto LAB_140195c6d;
          if ((char)*piVar18 != '\0') {
            iVar4 = piVar18[-2];
            for (iVar17 = piVar18[-3]; iVar17 < iVar4 + iVar6; iVar17 = iVar17 * 2) {
            }
            piVar24 = piVar18 + -4;
            if (piVar24 == (int *)0x0) {
LAB_140195b6f:
              if (iVar3 < iVar17) {
                iVar3 = iVar17;
              }
              puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
              puVar12[1] = iVar3;
              *puVar12 = 0xffffffff;
              piVar8 = puVar12 + 4;
              local_a58 = piVar8;
              if (piVar24 == (int *)0x0) {
                puVar12[2] = 0;
                *(char *)piVar8 = '\0';
                lVar14 = local_a28;
              }
              else {
                iVar17 = piVar18[-2] + 1;
                iVar23 = iVar3 + 1;
                if (iVar23 < iVar17) {
                  FUN_142e54290(0x5c,iVar17,iVar23);
                  iVar17 = iVar23;
                }
                FUN_142ef7ba0(piVar8,piVar18,(longlong)iVar17);
                puVar12[2] = piVar18[-2];
                *(char *)((longlong)iVar3 + (longlong)piVar8) = '\0';
                FUN_14019f2c0(piVar24);
                lVar14 = local_a28;
              }
            }
            else {
              if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
                iVar3 = piVar18[-2];
                goto LAB_140195b6f;
              }
              if (*piVar24 != 1) {
                FUN_142e52dd0(0x74);
              }
              *piVar24 = -1;
              piVar8 = piVar18;
            }
            if (piVar8 == (int *)0x0) {
              iVar3 = 0;
            }
            else {
              iVar3 = piVar8[-2];
            }
            FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar8),lVar14,piVar20);
            FUN_14019c870(&local_a58,iVar4 + iVar6);
            iVar4 = local_a38;
            goto LAB_140195d1d;
          }
          if ((piVar18 == (int *)0x0) || (piVar8 = piVar18 + -4, piVar8 == (int *)0x0)) {
LAB_140195c6d:
            if (iVar3 < iVar6) {
              iVar3 = iVar6;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            local_a58 = piVar18;
            if (piVar8 != (int *)0x0) {
              FUN_14019f2c0(piVar8);
            }
          }
          else {
            if ((1 < *piVar8) || (piVar18[-3] < iVar6)) {
              iVar3 = piVar18[-2];
              goto LAB_140195c6d;
            }
            if (*piVar8 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar8 = -1;
          }
          FUN_142ef7ba0(piVar18,lVar14,piVar20);
          if (piVar18[-4] != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar6 == -1) || (iVar6 <= piVar18[-3])) {
            piVar18[-4] = 1;
            if (iVar6 != -1) goto LAB_140195cf6;
            if (piVar18 != (int *)0x0) {
              piVar24 = (int *)0xffffffffffffffff;
              do {
                piVar24 = (int *)((longlong)piVar24 + 1);
              } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar18[-3],iVar6);
            piVar18[-4] = 1;
LAB_140195cf6:
            *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
            piVar24 = piVar20;
          }
          iVar3 = (int)piVar24;
          if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
            FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
          }
          piVar18[-2] = iVar3;
          iVar4 = local_a38;
        }
      }
LAB_140195d1d:
      if (local_a48 != (undefined8 *)0x0) {
        FUN_14019f2c0(local_a48 + -2);
      }
    }
    else if (DAT_143aa8270 != (code *)0x0) {
      (*DAT_143aa8270)(DAT_143aa8280,local_9e8,&local_a50,local_a18);
    }
    local_a80 = &local_a58;
    local_a88 = (int **)&local_a30;
    iVar6 = FUN_140194a90(local_9e8,local_368,0x104,&local_a40);
    local_a48 = (undefined8 *)0x0;
    iVar3 = 0;
    if ((int)uStack_a08 == -1) {
      if (iVar4 == 0) {
        local_a88 = local_368;
        plVar7 = (longlong *)
                 FUN_14019ba10(&local_a48,"%04X:%08X [%s]",local_a40 & 0xffffffff,
                               local_a30 & 0xffffffff);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
      else {
        local_a88 = local_368;
        plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs()+%X [%s]",local_23c,local_a50);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
    }
    else {
      local_a80 = local_368;
      local_a88 = (int **)CONCAT44(local_a88._4_4_,(int)uStack_a08);
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs() %hs(%lu) [%s]",local_23c,local_a00);
      lVar14 = *plVar7;
      piVar20 = local_a58;
      if (lVar14 != 0) {
        iVar4 = *(int *)(lVar14 + -8);
        if (iVar4 != 0) {
          if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
            uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
            FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar4);
            piVar20 = local_a58;
          }
          else {
            iVar17 = local_a58[-2];
            for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
            }
            lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
            piVar20 = local_a58;
            iVar23 = iVar3;
            if (local_a58 != (int *)0x0) {
              iVar23 = local_a58[-2];
            }
            FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar17 + iVar4);
          }
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    if (iVar6 == 0) {
      if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
        pcVar15 = (char *)FUN_14019bd40(&local_a58,0xc);
        *(undefined8 *)pcVar15 = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)(pcVar15 + 8) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,0xc);
        piVar20 = local_a58;
      }
      else {
        iVar4 = piVar20[-2];
        for (iVar6 = piVar20[-3]; iVar6 < iVar4 + 0xc; iVar6 = iVar6 * 2) {
        }
        lVar14 = FUN_14019bd40(&local_a58,iVar6,1);
        piVar20 = local_a58;
        iVar6 = iVar3;
        if (local_a58 != (int *)0x0) {
          iVar6 = local_a58[-2];
        }
        *(undefined8 *)(iVar6 + lVar14) = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)((longlong)iVar6 + 8 + lVar14) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,iVar4 + 0xc);
      }
    }
    local_a48 = (undefined8 *)0x0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a48,&DAT_143271d00);
    lVar14 = *plVar7;
    if (lVar14 != 0) {
      iVar4 = *(int *)(lVar14 + -8);
      if (iVar4 != 0) {
        if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
          uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
          FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar4);
          piVar20 = local_a58;
        }
        else {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          lVar13 = FUN_14019bd40(&local_a58,iVar17,1);
          piVar20 = local_a58;
          if (local_a58 != (int *)0x0) {
            iVar3 = local_a58[-2];
          }
          FUN_142ef7ba0(iVar3 + lVar13,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    uVar2 = DAT_143aa8260;
    uVar16 = DAT_143aa8258;
    pcVar1 = DAT_143aa8250;
    uVar9 = (*DAT_143ad5440)();
    uVar10 = (*DAT_143ad5408)();
    local_a68 = 0;
    local_a70 = uVar2;
    local_a78 = uVar16;
    local_a80 = (int **)0x0;
    local_a88 = (int **)local_838;
    iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  } while( true );
}



//===========================================================
// FUN_142ef3e44 @ 142ef3e44   (210 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142ef3e44(void)

{
  code *pcVar1;
  int iVar2;
  undefined1 *puVar3;
  undefined1 auStack_38 [8];
  undefined1 auStack_30 [48];
  
  puVar3 = auStack_38;
  iVar2 = (*DAT_143262740)(0x17);
  if (iVar2 != 0) {
    pcVar1 = (code *)swi(0x29);
    (*pcVar1)(2);
    puVar3 = auStack_30;
  }
  *(undefined8 *)(puVar3 + -8) = 0x142ef3e6f;
  capture_previous_context(&DAT_143ae26f0);
  _DAT_143ae2660 = *(undefined8 *)(puVar3 + 0x38);
  _DAT_143ae2788 = puVar3 + 0x40;
  _DAT_143ae2770 = *(undefined8 *)(puVar3 + 0x40);
  _DAT_143ae2650 = 0xc0000409;
  _DAT_143ae2654 = 1;
  _DAT_143ae2668 = 1;
  DAT_143ae2670 = 2;
  *(undefined8 *)(puVar3 + 0x20) = DAT_143a8b908;
  *(undefined8 *)(puVar3 + 0x28) = DAT_143a8b900;
  *(undefined8 *)(puVar3 + -8) = 0x142ef3f11;
  DAT_143ae27e8 = _DAT_143ae2660;
  __raise_securityfailure(&PTR_DAT_1434a1978);
  return;
}



//===========================================================
// FUN_140910eb0 @ 140910eb0   (336 bytes)
//===========================================================

undefined4 FUN_140910eb0(longlong *param_1,longlong param_2,undefined4 param_3)

{
  longlong *plVar1;
  int iVar2;
  undefined4 uVar3;
  short local_38 [4];
  undefined4 local_30;
  undefined4 uStack_2c;
  short local_20 [4];
  undefined4 local_18;
  undefined4 uStack_14;
  
  plVar1 = (longlong *)*param_1;
  uVar3 = param_3;
  if ((plVar1 == (longlong *)0x0) || (param_2 == 0)) {
    if (plVar1 != (longlong *)0x0) {
      (**(code **)(*plVar1 + 0x10))();
    }
  }
  else {
    FUN_14090e210(local_20,param_1,param_2);
    if (local_20[0] == 3) {
      uVar3 = local_18;
    }
    if ((local_20[0] != 0) && (local_20[0] != 10)) {
      (*DAT_143262a20)(local_38);
      if (local_38[0] == 8) {
        local_38[0] = 0;
        if (CONCAT44(uStack_2c,local_30) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_2c,local_30) + -4);
        }
      }
      iVar2 = (*DAT_143262a10)(local_38,local_20,0,3);
      if (-1 < iVar2) {
        param_3 = local_30;
      }
      uVar3 = param_3;
      if (local_38[0] == 8) {
        local_38[0] = 0;
        if (CONCAT44(uStack_2c,local_30) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_2c,local_30) + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_38);
      }
    }
    if (local_20[0] == 8) {
      local_20[0] = 0;
      if (CONCAT44(uStack_14,local_18) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_14,local_18) + -4);
      }
    }
    else {
      (*DAT_143262a18)(local_20);
    }
    if ((longlong *)*param_1 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_1 + 0x10))();
    }
  }
  return uVar3;
}



//===========================================================
// FUN_1403fbf10 @ 1403fbf10   (313 bytes)
//===========================================================

longlong * FUN_1403fbf10(longlong *param_1,int *param_2)

{
  longlong *plVar1;
  bool bVar2;
  longlong *plVar3;
  longlong *plVar4;
  longlong lVar5;
  longlong *local_48;
  undefined8 uStack_40;
  longlong *local_38;
  uint uStack_30;
  undefined4 uStack_2c;
  
  plVar1 = (longlong *)*param_1;
  local_38 = (longlong *)plVar1[1];
  uStack_30 = 0;
  plVar4 = plVar1;
  if (*(char *)((longlong)local_38 + 0x19) == '\0') {
    plVar3 = local_38;
    do {
      local_38 = plVar3;
      bVar2 = *param_2 <= (int)local_38[4];
      if (bVar2) {
        plVar3 = (longlong *)*local_38;
        plVar4 = local_38;
      }
      else {
        plVar3 = (longlong *)local_38[2];
      }
      uStack_30 = (uint)bVar2;
    } while (*(char *)((longlong)plVar3 + 0x19) == '\0');
  }
  if ((*(char *)((longlong)plVar4 + 0x19) != '\0') || (*param_2 < (int)plVar4[4])) {
    if (param_1[1] == 0x492492492492492) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    uStack_40 = (longlong *)0x0;
    local_48 = param_1;
    plVar4 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x38);
    *(int *)(plVar4 + 4) = *param_2;
    plVar4[5] = 0;
    plVar4[6] = 0;
    uStack_40 = plVar4;
    lVar5 = FUN_14019b780(&DAT_143ad68a0,0x28);
    *(longlong *)lVar5 = lVar5;
    *(longlong *)(lVar5 + 8) = lVar5;
    *(longlong *)(lVar5 + 0x10) = lVar5;
    *(undefined2 *)(lVar5 + 0x18) = 0x101;
    plVar4[5] = lVar5;
    *plVar4 = (longlong)plVar1;
    plVar4[1] = (longlong)plVar1;
    plVar4[2] = (longlong)plVar1;
    *(undefined2 *)(plVar4 + 3) = 0;
    local_48 = local_38;
    uStack_40 = (longlong *)CONCAT44(uStack_2c,uStack_30);
    plVar4 = (longlong *)FUN_1404114c0(param_1,&local_48,plVar4);
  }
  return plVar4 + 5;
}



//===========================================================
// FUN_14039f600 @ 14039f600   (463 bytes)
//===========================================================

longlong * FUN_14039f600(undefined8 param_1,longlong *param_2,ulonglong param_3,longlong param_4)

{
  int iVar1;
  longlong lVar2;
  undefined *puVar3;
  longlong lVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  longlong *plVar7;
  int iVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *local_res20;
  longlong *local_30;
  longlong *local_28 [2];
  
  uVar9 = param_3 & 0xffffffff;
  iVar8 = (int)param_3;
  if (param_4 != 0) {
    local_res20 = (longlong *)CONCAT44(local_res20._4_4_,iVar8);
    uVar10 = ((((param_3 & 0xff ^ 0xcbf29ce484222325) * 0x100000001b3 ^ param_3 >> 8 & 0xff) *
               0x100000001b3 ^ uVar9 >> 0x10 & 0xff) * 0x100000001b3 ^ uVar9 >> 0x18) *
             0x100000001b3;
    plVar6 = (longlong *)
             ((*(ulonglong *)(param_4 + 0x30) & uVar10) * 0x10 + *(longlong *)(param_4 + 0x18));
    lVar4 = plVar6[1];
    lVar2 = *(longlong *)(param_4 + 8);
    if (lVar4 != lVar2) {
      iVar1 = *(int *)(lVar4 + 0x10);
      while (iVar8 != iVar1) {
        if (lVar4 == *plVar6) goto LAB_14039f6ec;
        lVar4 = *(longlong *)(lVar4 + 8);
        iVar1 = *(int *)(lVar4 + 0x10);
      }
      if (lVar4 == 0) {
        lVar4 = lVar2;
      }
      if (lVar4 != lVar2) {
        plVar6 = (longlong *)FUN_140415e70(param_4,&local_res20,param_3,uVar10,0);
        plVar6 = (longlong *)*plVar6;
        if (plVar6 != (longlong *)0x0) {
          (**(code **)(*plVar6 + 8))(plVar6);
          *param_2 = (longlong)plVar6;
          return param_2;
        }
      }
    }
  }
LAB_14039f6ec:
  FUN_14039e630(param_1,&local_30,uVar9);
  puVar3 = PTR_DAT_143a45980;
  if (local_30 == (longlong *)0x0) {
    *param_2 = 0;
  }
  else if (iVar8 - 9100000U < 10000) {
    *param_2 = (longlong)local_30;
  }
  else {
    local_res20 = local_30;
    (**(code **)(*local_30 + 8))(local_30);
    puVar5 = (undefined8 *)FUN_14090f200(local_28,&local_res20,puVar3);
    plVar6 = (longlong *)*puVar5;
    plVar7 = (longlong *)0x0;
    if (plVar6 != (longlong *)0x0) {
      *puVar5 = 0;
      plVar7 = plVar6;
    }
    if (local_28[0] != (longlong *)0x0) {
      (**(code **)(*local_28[0] + 0x10))();
    }
    if ((param_4 != 0) && (plVar7 != (longlong *)0x0)) {
      local_res20 = plVar7;
      (**(code **)(*plVar7 + 8))(plVar7);
      FUN_140382a00(param_4,uVar9,&local_res20);
    }
    *param_2 = (longlong)plVar7;
    (**(code **)(*local_30 + 0x10))(local_30);
  }
  return param_2;
}



//===========================================================
// FUN_1403fbe10 @ 1403fbe10   (249 bytes)
//===========================================================

longlong * FUN_1403fbe10(longlong *param_1,int *param_2)

{
  longlong *plVar1;
  bool bVar2;
  longlong *plVar3;
  longlong *plVar4;
  longlong *local_38;
  undefined8 uStack_30;
  longlong *local_28;
  uint uStack_20;
  undefined4 uStack_1c;
  
  plVar1 = (longlong *)*param_1;
  local_28 = (longlong *)plVar1[1];
  uStack_20 = 0;
  plVar4 = plVar1;
  if (*(char *)((longlong)local_28 + 0x19) == '\0') {
    plVar3 = local_28;
    do {
      local_28 = plVar3;
      bVar2 = *param_2 <= *(int *)((longlong)local_28 + 0x1c);
      if (bVar2) {
        plVar3 = (longlong *)*local_28;
        plVar4 = local_28;
      }
      else {
        plVar3 = (longlong *)local_28[2];
      }
      uStack_20 = (uint)bVar2;
    } while (*(char *)((longlong)plVar3 + 0x19) == '\0');
  }
  if ((*(char *)((longlong)plVar4 + 0x19) != '\0') || (*param_2 < *(int *)((longlong)plVar4 + 0x1c))
     ) {
    if (param_1[1] == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    uStack_30 = 0;
    local_38 = param_1;
    plVar4 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
    *(int *)((longlong)plVar4 + 0x1c) = *param_2;
    *(undefined4 *)(plVar4 + 4) = 0;
    *plVar4 = (longlong)plVar1;
    plVar4[1] = (longlong)plVar1;
    plVar4[2] = (longlong)plVar1;
    *(undefined2 *)(plVar4 + 3) = 0;
    local_38 = local_28;
    uStack_30 = CONCAT44(uStack_1c,uStack_20);
    plVar4 = (longlong *)FUN_140411240(param_1,&local_38);
  }
  return plVar4 + 4;
}



//===========================================================
// FUN_140841970 @ 140841970   (79 bytes)
//===========================================================

undefined8 FUN_140841970(int param_1)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  
  if (DAT_143ac20a8 != 0) {
    cVar1 = *(char *)((longlong)DAT_143ac20a0[1] + 0x19);
    puVar3 = (undefined8 *)DAT_143ac20a0[1];
    puVar2 = DAT_143ac20a0;
    while (puVar4 = puVar3, cVar1 == '\0') {
      if (*(int *)(puVar4 + 4) < param_1) {
        puVar3 = (undefined8 *)puVar4[2];
        puVar4 = puVar2;
      }
      else {
        puVar3 = (undefined8 *)*puVar4;
      }
      cVar1 = *(char *)((longlong)puVar3 + 0x19);
      puVar2 = puVar4;
    }
    if (((*(char *)((longlong)puVar2 + 0x19) == '\0') && (*(int *)(puVar2 + 4) <= param_1)) &&
       (puVar2 != DAT_143ac20a0)) {
      return puVar2[5];
    }
  }
  return 0;
}



//===========================================================
// _com_issue_errorex @ 142ef3ad0   (184 bytes)
//===========================================================

/* Library Function - Single Match
    void __cdecl _com_issue_errorex(long,struct IUnknown * __ptr64,struct _GUID const & __ptr64)
   
   Libraries: Visual Studio 2017 Release, Visual Studio 2019 Release */

void __cdecl _com_issue_errorex(long param_1,IUnknown *param_2,_GUID *param_3)

{
  int iVar1;
  undefined8 local_res10;
  undefined8 local_res20;
  
  local_res10 = 0;
  if ((param_2 != (IUnknown *)0x0) &&
     (iVar1 = (*(code *)PTR_FUN_1432630d8)(param_2,&DAT_1434a1968,&local_res20), -1 < iVar1)) {
    iVar1 = (*(code *)PTR_FUN_1432630d8)(local_res20,param_3);
    (*(code *)PTR_FUN_1432630d8)();
    if ((iVar1 == 0) && (iVar1 = (*DAT_1432629c0)(0,&local_res10), iVar1 != 0)) {
      local_res10 = 0;
    }
  }
  (*(code *)PTR_FUN_1432630d8)(param_1,local_res10);
  return;
}



//===========================================================
// FUN_140911500 @ 140911500   (343 bytes)
//===========================================================

longlong FUN_140911500(longlong *param_1,longlong param_2,longlong param_3)

{
  longlong *plVar1;
  int iVar2;
  longlong lVar3;
  short local_38 [4];
  longlong local_30;
  short local_20 [4];
  longlong local_18;
  
  plVar1 = (longlong *)*param_1;
  if ((plVar1 == (longlong *)0x0) || (param_2 == 0)) {
    if (plVar1 != (longlong *)0x0) {
      (**(code **)(*plVar1 + 0x10))();
    }
  }
  else {
    FUN_14090e210(local_20,param_1,param_2);
    lVar3 = param_3;
    if (local_20[0] == 0x14) {
      lVar3 = local_18;
    }
    if ((local_20[0] != 0) && (local_20[0] != 10)) {
      (*DAT_143262a20)(local_38);
      if ((local_38[0] == 8) && (local_38[0] = 0, local_30 != 0)) {
        (*DAT_143ad5990)(local_30 + -4);
      }
      iVar2 = (*DAT_143262a10)(local_38,local_20,0,0x14);
      if (-1 < iVar2) {
        param_3 = local_30;
      }
      lVar3 = param_3;
      if (local_38[0] == 8) {
        local_38[0] = 0;
        if (local_30 != 0) {
          (*DAT_143ad5990)(local_30 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_38);
      }
    }
    param_3 = lVar3;
    if (local_20[0] == 8) {
      local_20[0] = 0;
      if (local_18 != 0) {
        (*DAT_143ad5990)(local_18 + -4);
      }
    }
    else {
      (*DAT_143262a18)(local_20);
    }
    if ((longlong *)*param_1 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_1 + 0x10))();
    }
  }
  return param_3;
}



//===========================================================
// FUN_1404161d0 @ 1404161d0   (273 bytes)
//===========================================================

undefined8 FUN_1404161d0(uint *param_1,undefined8 param_2)

{
  short sVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  uint local_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 local_38;
  uint local_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined8 local_20;
  
  uVar3 = (undefined4)param_2;
  uVar4 = (undefined4)((ulonglong)param_2 >> 0x20);
  (*DAT_143262a20)(&local_48);
  sVar1 = (short)*param_1;
  if ((sVar1 != 0) && (sVar1 != 10)) {
    if ((&local_48 == param_1) && (sVar1 == 8)) {
      local_30 = local_30 & 0xffff0000;
      iVar2 = (*DAT_143262a10)(&local_30,param_1,0,5);
      if (-1 < iVar2) {
        FUN_1401a6450(&local_48);
        local_48 = local_30;
        uStack_44 = uStack_2c;
        uStack_40 = uStack_28;
        uStack_3c = uStack_24;
        local_38 = local_20;
      }
    }
    else {
      if ((short)local_48 == 8) {
        local_48 = local_48 & 0xffff0000;
        if (CONCAT44(uStack_3c,uStack_40) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_3c,uStack_40) + -4);
        }
      }
      iVar2 = (*DAT_143262a10)(&local_48,param_1,0,5);
    }
    if (-1 < iVar2) {
      uVar3 = uStack_40;
      uVar4 = uStack_3c;
    }
  }
  if ((short)local_48 == 8) {
    local_48 = local_48 & 0xffff0000;
    if (CONCAT44(uStack_3c,uStack_40) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_3c,uStack_40) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_48);
  }
  return CONCAT44(uVar4,uVar3);
}



//===========================================================
// FUN_14019b780 @ 14019b780   (374 bytes)
//===========================================================

void FUN_14019b780(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  int iVar6;
  undefined8 uVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar7 = 0x80;
  if (param_2 < 0x21) {
    uVar10 = (uint)(0x10 < param_2);
LAB_14019b7de:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x10;
      goto LAB_14019b825;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x20;
      goto LAB_14019b825;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b825;
    }
  }
  else {
    if (0x40 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x81) {
        uVar10 = 3;
      }
      goto LAB_14019b7de;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x40;
LAB_14019b825:
  lVar11 = (longlong)(int)uVar10;
  lVar8 = lVar11 * 0x10 + param_1;
  plVar1 = (longlong *)(lVar8 + 0x28);
  LOCK();
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar4 == 0) {
LAB_14019b889:
    *(undefined4 *)(lVar8 + 0x30) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar4 = *plVar1;
      if (lVar4 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar4 == 0) goto LAB_14019b889;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar7,iVar6);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar6;
    puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  }
  piVar2 = (int *)(param_1 + 0x14 + lVar11 * 4);
  *piVar2 = *piVar2 + 1;
  *(undefined8 *)(param_1 + 0x68 + lVar11 * 8) = *puVar5;
  *piVar9 = *piVar9 + -1;
  if (*piVar9 == 0) {
    *plVar1 = 0;
  }
  return;
}



//===========================================================
// FUN_142ef7ba0 @ 142ef7ba0   (1379 bytes)
//===========================================================

undefined8 * FUN_142ef7ba0(undefined8 *param_1,undefined8 *param_2,ulonglong param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [32];
  undefined1 auVar4 [32];
  undefined1 auVar5 [32];
  undefined1 auVar6 [32];
  undefined1 uVar7;
  undefined2 uVar8;
  undefined4 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  undefined8 *puVar23;
  undefined1 (*pauVar24) [32];
  undefined1 (*pauVar25) [32];
  undefined8 *puVar26;
  undefined1 (*pauVar27) [32];
  undefined1 (*pauVar28) [32];
  ulonglong uVar29;
  longlong lVar30;
  ulonglong uVar31;
  undefined8 uVar32;
  undefined8 uVar33;
  
  puVar23 = param_1;
  switch(param_3) {
  case 0:
    return puVar23;
  case 1:
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    return puVar23;
  case 2:
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    return puVar23;
  case 3:
    uVar7 = *(undefined1 *)((longlong)param_2 + 2);
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    *(undefined1 *)((longlong)param_1 + 2) = uVar7;
    return puVar23;
  case 4:
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    return puVar23;
  case 5:
    uVar7 = *(undefined1 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined1 *)((longlong)param_1 + 4) = uVar7;
    return puVar23;
  case 6:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    return puVar23;
  case 7:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    uVar7 = *(undefined1 *)((longlong)param_2 + 6);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    *(undefined1 *)((longlong)param_1 + 6) = uVar7;
    return puVar23;
  case 8:
    *param_1 = *param_2;
    return puVar23;
  case 9:
    uVar7 = *(undefined1 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined1 *)(param_1 + 1) = uVar7;
    return puVar23;
  case 10:
    uVar8 = *(undefined2 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    return puVar23;
  case 0xb:
    uVar8 = *(undefined2 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 10);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    *(undefined1 *)((longlong)param_1 + 10) = uVar7;
    return puVar23;
  case 0xc:
    uVar9 = *(undefined4 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    return puVar23;
  case 0xd:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined1 *)((longlong)param_1 + 0xc) = uVar7;
    return puVar23;
  case 0xe:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    return puVar23;
  case 0xf:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xe);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    *(undefined1 *)((longlong)param_1 + 0xe) = uVar7;
    return puVar23;
  }
  if (param_3 < 0x21) {
    uVar10 = param_2[1];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x10));
    uVar11 = *puVar26;
    uVar32 = puVar26[1];
    *param_1 = *param_2;
    param_1[1] = uVar10;
    param_1 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    *param_1 = uVar11;
    param_1[1] = uVar32;
    return puVar23;
  }
  if ((param_2 < param_1) && (param_1 < (undefined8 *)((longlong)param_2 + param_3))) {
    lVar30 = (longlong)param_2 - (longlong)param_1;
    puVar23 = (undefined8 *)((longlong)param_1 + lVar30 + (param_3 - 0x10));
    uVar10 = *puVar23;
    uVar11 = puVar23[1];
    puVar26 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    uVar29 = param_3 - 0x10;
    puVar23 = puVar26;
    uVar32 = uVar10;
    uVar33 = uVar11;
    if (((ulonglong)puVar26 & 0xf) != 0) {
      puVar23 = (undefined8 *)((ulonglong)puVar26 & 0xfffffffffffffff0);
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
      *puVar26 = uVar10;
      *(undefined8 *)((longlong)param_1 + (param_3 - 8)) = uVar11;
      uVar29 = (longlong)puVar23 - (longlong)param_1;
    }
    uVar31 = uVar29 >> 7;
    if (uVar31 != 0) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar26 = puVar23;
      while( true ) {
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x10);
        uVar10 = puVar1[1];
        puVar23 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x20);
        uVar11 = *puVar23;
        uVar32 = puVar23[1];
        puVar23 = puVar26 + -0x10;
        puVar26[-2] = *puVar1;
        puVar26[-1] = uVar10;
        puVar26[-4] = uVar11;
        puVar26[-3] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x30);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x40);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        uVar31 = uVar31 - 1;
        puVar26[-6] = *puVar1;
        puVar26[-5] = uVar10;
        puVar26[-8] = uVar11;
        puVar26[-7] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x50);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x60);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        puVar26[-10] = *puVar1;
        puVar26[-9] = uVar10;
        puVar26[-0xc] = uVar11;
        puVar26[-0xb] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x70);
        uVar10 = *puVar1;
        uVar11 = puVar1[1];
        uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
        uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
        if (uVar31 == 0) break;
        puVar26[-0xe] = uVar10;
        puVar26[-0xd] = uVar11;
        *puVar23 = uVar32;
        puVar26[-0xf] = uVar33;
        puVar26 = puVar23;
      }
      puVar26[-0xe] = uVar10;
      puVar26[-0xd] = uVar11;
      uVar29 = uVar29 & 0x7f;
    }
    for (uVar31 = uVar29 >> 4; uVar31 != 0; uVar31 = uVar31 - 1) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar23 = puVar23 + -2;
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
    }
    if ((uVar29 & 0xf) != 0) {
      uVar10 = param_2[1];
      *param_1 = *param_2;
      param_1[1] = uVar10;
    }
    *puVar23 = uVar32;
    puVar23[1] = uVar33;
    return param_1;
  }
  if (DAT_143a8b918 < 3) {
    if ((param_3 < 0x801) || (((byte)DAT_143ae2c20 & 2) == 0)) {
      if (0x80 < param_3) {
        lVar30 = ((ulonglong)param_1 & 0xf) - 0x10;
        param_1 = (undefined8 *)((longlong)param_1 - lVar30);
        param_2 = (undefined8 *)((longlong)param_2 - lVar30);
        param_3 = param_3 + lVar30;
        if (0x80 < param_3) {
          do {
            uVar10 = param_2[1];
            uVar11 = param_2[2];
            uVar32 = param_2[3];
            uVar33 = param_2[4];
            uVar12 = param_2[5];
            uVar13 = param_2[6];
            uVar14 = param_2[7];
            *param_1 = *param_2;
            param_1[1] = uVar10;
            param_1[2] = uVar11;
            param_1[3] = uVar32;
            param_1[4] = uVar33;
            param_1[5] = uVar12;
            param_1[6] = uVar13;
            param_1[7] = uVar14;
            uVar10 = param_2[9];
            uVar11 = param_2[10];
            uVar32 = param_2[0xb];
            uVar33 = param_2[0xc];
            uVar12 = param_2[0xd];
            uVar13 = param_2[0xe];
            uVar14 = param_2[0xf];
            param_1[8] = param_2[8];
            param_1[9] = uVar10;
            param_1[10] = uVar11;
            param_1[0xb] = uVar32;
            param_1[0xc] = uVar33;
            param_1[0xd] = uVar12;
            param_1[0xe] = uVar13;
            param_1[0xf] = uVar14;
            param_1 = param_1 + 0x10;
            param_2 = param_2 + 0x10;
            param_3 = param_3 - 0x80;
          } while (0x7f < param_3);
        }
      }
                    /* WARNING: Could not recover jumptable at 0x000142ef80b6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      puVar23 = (undefined8 *)
                (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                          *(uint *)(&DAT_143c47088 + (param_3 + 0xf >> 4) * 4)))();
      return puVar23;
    }
  }
  else if (((param_3 < 0x2001) || (0x180000 < param_3)) || (((byte)DAT_143ae2c20 & 2) == 0)) {
    uVar10 = *param_2;
    uVar11 = param_2[1];
    uVar32 = param_2[2];
    uVar33 = param_2[3];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x20));
    uVar12 = *puVar26;
    uVar13 = puVar26[1];
    uVar14 = puVar26[2];
    uVar15 = puVar26[3];
    if (0x100 < param_3) {
      lVar30 = ((ulonglong)param_1 & 0x1f) - 0x20;
      pauVar24 = (undefined1 (*) [32])((longlong)param_1 - lVar30);
      pauVar27 = (undefined1 (*) [32])((longlong)param_2 - lVar30);
      param_3 = param_3 + lVar30;
      if (0x100 < param_3) {
        if (0x180000 < param_3) {
          do {
            uVar29 = param_3;
            pauVar28 = pauVar27;
            pauVar25 = pauVar24;
            auVar3 = pauVar28[1];
            auVar4 = pauVar28[2];
            auVar5 = pauVar28[3];
            auVar6 = vmovntdq_avx(*pauVar28);
            *pauVar25 = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[1] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[2] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[3] = auVar3;
            auVar3 = pauVar28[5];
            auVar4 = pauVar28[6];
            auVar5 = pauVar28[7];
            auVar6 = vmovntdq_avx(pauVar28[4]);
            pauVar25[4] = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[5] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[6] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[7] = auVar3;
            pauVar24 = pauVar25 + 8;
            pauVar27 = pauVar28 + 8;
            param_3 = uVar29 - 0x100;
          } while (0xff < uVar29 - 0x100);
          uVar31 = uVar29 - 0xe1 & 0xffffffffffffffe0;
          switch(uVar29) {
          case 0x1e1:
          case 0x1e2:
          case 0x1e3:
          case 0x1e4:
          case 0x1e5:
          case 0x1e6:
          case 0x1e7:
          case 0x1e8:
          case 0x1e9:
          case 0x1ea:
          case 0x1eb:
          case 0x1ec:
          case 0x1ed:
          case 0x1ee:
          case 0x1ef:
          case 0x1f0:
          case 0x1f1:
          case 0x1f2:
          case 499:
          case 500:
          case 0x1f5:
          case 0x1f6:
          case 0x1f7:
          case 0x1f8:
          case 0x1f9:
          case 0x1fa:
          case 0x1fb:
          case 0x1fc:
          case 0x1fd:
          case 0x1fe:
          case 0x1ff:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(*pauVar28 + uVar31));
            *(undefined1 (*) [32])(*pauVar25 + uVar31) = auVar3;
          case 0x1c1:
          case 0x1c2:
          case 0x1c3:
          case 0x1c4:
          case 0x1c5:
          case 0x1c6:
          case 0x1c7:
          case 0x1c8:
          case 0x1c9:
          case 0x1ca:
          case 0x1cb:
          case 0x1cc:
          case 0x1cd:
          case 0x1ce:
          case 0x1cf:
          case 0x1d0:
          case 0x1d1:
          case 0x1d2:
          case 0x1d3:
          case 0x1d4:
          case 0x1d5:
          case 0x1d6:
          case 0x1d7:
          case 0x1d8:
          case 0x1d9:
          case 0x1da:
          case 0x1db:
          case 0x1dc:
          case 0x1dd:
          case 0x1de:
          case 0x1df:
          case 0x1e0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[1] + uVar31));
            *(undefined1 (*) [32])(pauVar25[1] + uVar31) = auVar3;
          case 0x1a1:
          case 0x1a2:
          case 0x1a3:
          case 0x1a4:
          case 0x1a5:
          case 0x1a6:
          case 0x1a7:
          case 0x1a8:
          case 0x1a9:
          case 0x1aa:
          case 0x1ab:
          case 0x1ac:
          case 0x1ad:
          case 0x1ae:
          case 0x1af:
          case 0x1b0:
          case 0x1b1:
          case 0x1b2:
          case 0x1b3:
          case 0x1b4:
          case 0x1b5:
          case 0x1b6:
          case 0x1b7:
          case 0x1b8:
          case 0x1b9:
          case 0x1ba:
          case 0x1bb:
          case 0x1bc:
          case 0x1bd:
          case 0x1be:
          case 0x1bf:
          case 0x1c0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[2] + uVar31));
            *(undefined1 (*) [32])(pauVar25[2] + uVar31) = auVar3;
          case 0x181:
          case 0x182:
          case 0x183:
          case 0x184:
          case 0x185:
          case 0x186:
          case 0x187:
          case 0x188:
          case 0x189:
          case 0x18a:
          case 0x18b:
          case 0x18c:
          case 0x18d:
          case 0x18e:
          case 399:
          case 400:
          case 0x191:
          case 0x192:
          case 0x193:
          case 0x194:
          case 0x195:
          case 0x196:
          case 0x197:
          case 0x198:
          case 0x199:
          case 0x19a:
          case 0x19b:
          case 0x19c:
          case 0x19d:
          case 0x19e:
          case 0x19f:
          case 0x1a0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[3] + uVar31));
            *(undefined1 (*) [32])(pauVar25[3] + uVar31) = auVar3;
          case 0x161:
          case 0x162:
          case 0x163:
          case 0x164:
          case 0x165:
          case 0x166:
          case 0x167:
          case 0x168:
          case 0x169:
          case 0x16a:
          case 0x16b:
          case 0x16c:
          case 0x16d:
          case 0x16e:
          case 0x16f:
          case 0x170:
          case 0x171:
          case 0x172:
          case 0x173:
          case 0x174:
          case 0x175:
          case 0x176:
          case 0x177:
          case 0x178:
          case 0x179:
          case 0x17a:
          case 0x17b:
          case 0x17c:
          case 0x17d:
          case 0x17e:
          case 0x17f:
          case 0x180:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[4] + uVar31));
            *(undefined1 (*) [32])(pauVar25[4] + uVar31) = auVar3;
          case 0x141:
          case 0x142:
          case 0x143:
          case 0x144:
          case 0x145:
          case 0x146:
          case 0x147:
          case 0x148:
          case 0x149:
          case 0x14a:
          case 0x14b:
          case 0x14c:
          case 0x14d:
          case 0x14e:
          case 0x14f:
          case 0x150:
          case 0x151:
          case 0x152:
          case 0x153:
          case 0x154:
          case 0x155:
          case 0x156:
          case 0x157:
          case 0x158:
          case 0x159:
          case 0x15a:
          case 0x15b:
          case 0x15c:
          case 0x15d:
          case 0x15e:
          case 0x15f:
          case 0x160:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[5] + uVar31));
            *(undefined1 (*) [32])(pauVar25[5] + uVar31) = auVar3;
          case 0x121:
          case 0x122:
          case 0x123:
          case 0x124:
          case 0x125:
          case 0x126:
          case 0x127:
          case 0x128:
          case 0x129:
          case 0x12a:
          case 299:
          case 300:
          case 0x12d:
          case 0x12e:
          case 0x12f:
          case 0x130:
          case 0x131:
          case 0x132:
          case 0x133:
          case 0x134:
          case 0x135:
          case 0x136:
          case 0x137:
          case 0x138:
          case 0x139:
          case 0x13a:
          case 0x13b:
          case 0x13c:
          case 0x13d:
          case 0x13e:
          case 0x13f:
          case 0x140:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[6] + uVar31));
            *(undefined1 (*) [32])(pauVar25[6] + uVar31) = auVar3;
          default:
            puVar26 = (undefined8 *)(pauVar25[-1] + uVar29);
            *puVar26 = uVar12;
            puVar26[1] = uVar13;
            puVar26[2] = uVar14;
            puVar26[3] = uVar15;
          case 0x100:
            *param_1 = uVar10;
            param_1[1] = uVar11;
            param_1[2] = uVar32;
            param_1[3] = uVar33;
            return puVar23;
          }
        }
        do {
          uVar10 = *(undefined8 *)(*pauVar27 + 8);
          uVar11 = *(undefined8 *)(*pauVar27 + 0x10);
          uVar32 = *(undefined8 *)(*pauVar27 + 0x18);
          uVar33 = *(undefined8 *)pauVar27[1];
          uVar12 = *(undefined8 *)(pauVar27[1] + 8);
          uVar13 = *(undefined8 *)(pauVar27[1] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[1] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[2];
          uVar16 = *(undefined8 *)(pauVar27[2] + 8);
          uVar17 = *(undefined8 *)(pauVar27[2] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[2] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[3];
          uVar20 = *(undefined8 *)(pauVar27[3] + 8);
          uVar21 = *(undefined8 *)(pauVar27[3] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[3] + 0x18);
          *(undefined8 *)*pauVar24 = *(undefined8 *)*pauVar27;
          *(undefined8 *)(*pauVar24 + 8) = uVar10;
          *(undefined8 *)(*pauVar24 + 0x10) = uVar11;
          *(undefined8 *)(*pauVar24 + 0x18) = uVar32;
          *(undefined8 *)pauVar24[1] = uVar33;
          *(undefined8 *)(pauVar24[1] + 8) = uVar12;
          *(undefined8 *)(pauVar24[1] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[1] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[2] = uVar15;
          *(undefined8 *)(pauVar24[2] + 8) = uVar16;
          *(undefined8 *)(pauVar24[2] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[2] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[3] = uVar19;
          *(undefined8 *)(pauVar24[3] + 8) = uVar20;
          *(undefined8 *)(pauVar24[3] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[3] + 0x18) = uVar22;
          uVar10 = *(undefined8 *)(pauVar27[4] + 8);
          uVar11 = *(undefined8 *)(pauVar27[4] + 0x10);
          uVar32 = *(undefined8 *)(pauVar27[4] + 0x18);
          uVar33 = *(undefined8 *)pauVar27[5];
          uVar12 = *(undefined8 *)(pauVar27[5] + 8);
          uVar13 = *(undefined8 *)(pauVar27[5] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[5] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[6];
          uVar16 = *(undefined8 *)(pauVar27[6] + 8);
          uVar17 = *(undefined8 *)(pauVar27[6] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[6] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[7];
          uVar20 = *(undefined8 *)(pauVar27[7] + 8);
          uVar21 = *(undefined8 *)(pauVar27[7] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[7] + 0x18);
          *(undefined8 *)pauVar24[4] = *(undefined8 *)pauVar27[4];
          *(undefined8 *)(pauVar24[4] + 8) = uVar10;
          *(undefined8 *)(pauVar24[4] + 0x10) = uVar11;
          *(undefined8 *)(pauVar24[4] + 0x18) = uVar32;
          *(undefined8 *)pauVar24[5] = uVar33;
          *(undefined8 *)(pauVar24[5] + 8) = uVar12;
          *(undefined8 *)(pauVar24[5] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[5] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[6] = uVar15;
          *(undefined8 *)(pauVar24[6] + 8) = uVar16;
          *(undefined8 *)(pauVar24[6] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[6] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[7] = uVar19;
          *(undefined8 *)(pauVar24[7] + 8) = uVar20;
          *(undefined8 *)(pauVar24[7] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[7] + 0x18) = uVar22;
          pauVar24 = pauVar24 + 8;
          pauVar27 = pauVar27 + 8;
          param_3 = param_3 - 0x100;
        } while (0xff < param_3);
      }
    }
                    /* WARNING: Could not recover jumptable at 0x000142ef7e12. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    puVar23 = (undefined8 *)
              (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                        *(uint *)(&DAT_143c47040 + (param_3 + 0x1f >> 5) * 4)))();
    return puVar23;
  }
  for (; param_3 != 0; param_3 = param_3 - 1) {
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    param_2 = (undefined8 *)((longlong)param_2 + 1);
    param_1 = (undefined8 *)((longlong)param_1 + 1);
  }
  return puVar23;
}


