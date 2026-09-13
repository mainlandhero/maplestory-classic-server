
//===========================================================
// FUN_142d490d0 @ 142d490d0   (239 bytes)
//===========================================================

void FUN_142d490d0(undefined8 param_1,undefined4 param_2,int param_3,undefined4 param_4,
                  undefined4 param_5)

{
  int iVar1;
  longlong lVar2;
  int *piVar3;
  undefined8 local_18 [2];
  
  piVar3 = (int *)FUN_142dc0950(DAT_143aa9d98);
  if (piVar3 != (int *)0x0) {
    if (param_3 == 0) {
      piVar3 = piVar3 + 6;
    }
    iVar1 = *piVar3;
    if ((iVar1 != 0) && (iVar1 != 999999999)) {
      FUN_142d48850(param_1,iVar1,param_4,param_5,param_2,0);
      lVar2 = DAT_143ad2898;
      if ((*(longlong *)(piVar3 + 2) != 0) &&
         ((0 < *(int *)(*(longlong *)(piVar3 + 2) + -8) && (DAT_143ad2898 != 0)))) {
        local_18[0] = 0;
        FUN_14019a260(local_18,piVar3 + 2);
        FUN_141c48780(lVar2,1,piVar3[4],local_18);
        local_18[0] = CONCAT44(local_18[0]._4_4_,1);
        FUN_141c486b0(DAT_143ad2898,local_18);
      }
    }
  }
  return;
}



//===========================================================
// FUN_14180d7f0 @ 14180d7f0   (947 bytes)
//===========================================================

void FUN_14180d7f0(longlong param_1,undefined4 param_2)

{
  wchar_t *pwVar1;
  int *piVar2;
  int iVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int *piVar9;
  int *piVar10;
  undefined8 uVar11;
  uint uVar12;
  longlong *plVar13;
  uint uVar14;
  longlong *plVar15;
  longlong *local_res18;
  longlong *local_res20;
  int *piVar16;
  undefined4 uVar17;
  ulonglong uVar18;
  short local_58 [4];
  longlong local_50;
  
  *(undefined4 *)(param_1 + 0x300) = 9;
  *(undefined4 *)(param_1 + 0x32c) = param_2;
  *(undefined4 *)(param_1 + 0x2e0) = 0;
  *(undefined4 *)(param_1 + 0x2e4) = 0xad;
  *(undefined4 *)(param_1 + 0x2e8) = 0x2c;
  iVar6 = FUN_1410a4dc0();
  iVar7 = FUN_1410a4dd0();
  iVar3 = DAT_143acee70 * -5;
  piVar9 = (int *)FUN_1401bc720(&DAT_143ad6980,0x5a);
  piVar9[1] = 0x24;
  plVar13 = (longlong *)0xffffffffffffffff;
  *piVar9 = -1;
  pwVar1 = (wchar_t *)(piVar9 + 4);
  piVar9[2] = 0;
  *pwVar1 = L'\0';
  uVar11 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._8_8_;
  *(undefined8 *)pwVar1 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._0_8_;
  *(undefined8 *)(piVar9 + 6) = uVar11;
  uVar11 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._24_8_;
  *(undefined8 *)(piVar9 + 8) = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._16_8_;
  *(undefined8 *)(piVar9 + 10) = uVar11;
  uVar11 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._40_8_;
  *(undefined8 *)(piVar9 + 0xc) = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._32_8_;
  *(undefined8 *)(piVar9 + 0xe) = uVar11;
  uVar5 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._60_4_;
  uVar4 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._56_4_;
  uVar17 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._52_4_;
  piVar9[0x10] = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._48_4_;
  piVar9[0x11] = uVar17;
  piVar9[0x12] = uVar4;
  piVar9[0x13] = uVar5;
  *(undefined8 *)(piVar9 + 0x14) = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6820._64_8_;
  if (*piVar9 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar9[1] < 0x24) {
    FUN_142e54290(0x90,piVar9[1],0x24);
  }
  *piVar9 = 1;
  *(undefined2 *)(piVar9 + 0x16) = 0;
  if (piVar9[1] + 1 < 0x25) {
    FUN_142e54290(0x9c);
  }
  piVar9[2] = 0x48;
  iVar8 = (*DAT_1432627f0)(0xfde9,0,L"UI/FadeYesNo.img/FadeYesNo/icon6",0xffffffff,0,0,0,0);
  uVar14 = iVar8 - 1;
  uVar12 = 0;
  if (0 < (int)uVar14) {
    uVar12 = uVar14;
  }
  piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(int)(uVar12 + 0x11));
  piVar10[1] = uVar12;
  *piVar10 = -1;
  piVar2 = piVar10 + 4;
  piVar10[2] = 0;
  *(undefined1 *)piVar2 = 0;
  uVar18 = 0;
  piVar16 = piVar2;
  (*DAT_1432627f0)(0xfde9,0,L"UI/FadeYesNo.img/FadeYesNo/icon6",0xffffffff,piVar2,iVar8,0,0);
  uVar17 = (undefined4)((ulonglong)piVar16 >> 0x20);
  if (*piVar10 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((uVar14 != 0xffffffff) && (piVar10[1] < (int)uVar14)) {
    FUN_142e54290(0x90,piVar10[1],uVar14);
  }
  *piVar10 = 1;
  if (uVar14 == 0xffffffff) {
    plVar15 = (longlong *)0x0;
    if (piVar2 != (int *)0x0) {
      do {
        plVar13 = (longlong *)((longlong)plVar13 + 1);
        plVar15 = plVar13;
      } while (*(char *)((longlong)piVar2 + (longlong)plVar13) != '\0');
    }
  }
  else {
    *(undefined1 *)((longlong)(int)uVar14 + (longlong)piVar2) = 0;
    plVar15 = (longlong *)(ulonglong)uVar14;
  }
  iVar8 = (int)plVar15;
  if ((iVar8 < 0) || (piVar10[1] + 1 <= iVar8)) {
    FUN_142e54290(0x9c,(ulonglong)plVar15 & 0xffffffff);
  }
  piVar10[2] = iVar8;
  uVar11 = FUN_14090df00(local_58,piVar2);
  uVar11 = FUN_1409339d0(&local_res20,uVar11);
  FUN_1403ee040(&local_res18,uVar11);
  plVar13 = *(longlong **)(param_1 + 0x308);
  if (plVar13 != local_res18) {
    *(longlong **)(param_1 + 0x308) = local_res18;
    local_res18 = (longlong *)0x0;
    if (plVar13 != (longlong *)0x0) {
      (**(code **)(*plVar13 + 0x10))();
      local_res18 = (longlong *)0x0;
    }
  }
  if (local_res18 != (longlong *)0x0) {
    (**(code **)(*local_res18 + 0x10))(local_res18);
  }
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))();
  }
  if (local_58[0] == 8) {
    local_58[0] = 0;
    if (local_50 != 0) {
      (*DAT_143ad5990)(local_50 + -4);
    }
  }
  else {
    (*DAT_143262a18)(local_58);
  }
  *(undefined4 *)(param_1 + 0x298) = 0;
  *(undefined8 *)(param_1 + 0x29c) = 0xff;
  uVar11 = CONCAT44(iVar7 + iVar3 + -0x62,iVar6 / 2 + 0x17);
  *(undefined8 *)(param_1 + 0x2b0) = uVar11;
  *(undefined8 *)(param_1 + 0x2b8) = uVar11;
  *(undefined8 *)(param_1 + 0x2c0) = uVar11;
  *(undefined4 *)(param_1 + 0x2a4) = 1000;
  *(undefined4 *)(param_1 + 0x2a8) = 6000;
  *(undefined4 *)(param_1 + 0x2ac) = 1000;
  FUN_1418060c0(param_1,0xad,0x2c,pwVar1,CONCAT44(uVar17,0x271a),1,0,uVar18 & 0xffffffff00000000,0);
  FUN_14180cc00(param_1);
  FUN_14019f2c0(piVar10);
  FUN_1401bebb0(piVar9);
  return;
}



//===========================================================
// FUN_141808b90 @ 141808b90   (370 bytes)
//===========================================================

undefined8 * FUN_141808b90(undefined8 *param_1)

{
  longlong *plVar1;
  undefined8 uVar2;
  longlong *plVar3;
  int iVar4;
  longlong *local_res10;
  undefined1 local_28 [32];
  
  uVar2 = FUN_141d5d8c0(local_28,&DAT_143271f04,&DAT_143271f04,0xb3,0xffffffff);
  FUN_14177edb0(param_1,uVar2);
  *(undefined4 *)(param_1 + 0x59) = 0;
  *(undefined4 *)((longlong)param_1 + 0x2d4) = 0;
  *(undefined4 *)(param_1 + 0x5b) = 1;
  *param_1 = &PTR_FUN_1433d6530;
  param_1[1] = &PTR_LAB_1433d66a0;
  param_1[3] = &PTR_FUN_1433d6778;
  FUN_141aa3a20(param_1 + 0x5f);
  *(undefined4 *)(param_1 + 0x60) = 0;
  param_1[0x61] = 0;
  param_1[0x62] = 0;
  param_1[99] = 0;
  param_1[100] = 0;
  *(undefined4 *)(param_1 + 0x65) = 0;
  *(undefined4 *)((longlong)param_1 + 0x32c) = 0xffffffff;
  param_1[0x66] = 0;
  *(undefined4 *)(param_1 + 0x67) = 0;
  param_1[0x68] = 0;
  param_1[0x69] = 0;
  *(undefined4 *)(param_1 + 0x6a) = 0;
  param_1[0x6b] = 0;
  param_1[0x6c] = 0xffffffffffffffff;
  *(undefined1 *)(param_1 + 0x6d) = 0;
  LOCK();
  UNLOCK();
  iVar4 = DAT_143acee70 + 1;
  DAT_143acee70 = DAT_143acee70 + 1;
  *(int *)((longlong)param_1 + 0x2ec) = iVar4;
  plVar3 = (longlong *)FUN_1429fa100(&local_res10,0xffffffcc,0xc,0,4);
  plVar1 = (longlong *)param_1[99];
  if (plVar1 != (longlong *)*plVar3) {
    param_1[99] = (longlong *)*plVar3;
    *plVar3 = 0;
    if (plVar1 != (longlong *)0x0) {
      (**(code **)(*plVar1 + 0x10))();
    }
  }
  if (local_res10 != (longlong *)0x0) {
    (**(code **)(*local_res10 + 0x10))();
  }
  return param_1;
}



//===========================================================
// FUN_142d97880 @ 142d97880   (651 bytes)
//===========================================================

undefined8 FUN_142d97880(longlong param_1,longlong param_2)

{
  longlong *plVar1;
  char cVar2;
  longlong lVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  longlong lVar7;
  longlong *plVar8;
  longlong *plVar9;
  undefined1 local_48 [8];
  longlong local_40;
  
  lVar3 = *(longlong *)(param_2 + 8);
  if (lVar3 != 0) {
    if (*(ulonglong *)(param_1 + 0x2c18) < 100) {
      lVar3 = FUN_141892840();
      if (lVar3 == 0) {
LAB_142d978f6:
        cVar2 = FUN_142d1c460(param_1);
        lVar3 = *(longlong *)(param_2 + 8);
        if (cVar2 != '\0') {
          if (lVar3 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar3 = *(longlong *)(param_2 + 8);
          }
          FUN_142bf3f70(lVar3);
          FUN_140cbb030(param_2);
          return 0;
        }
        if (lVar3 == 0) {
          FUN_142e52ed0(0x431,0);
        }
        plVar1 = (longlong *)(param_1 + 0x2c10);
        plVar9 = *(longlong **)*plVar1;
        if (plVar9 != (longlong *)*plVar1) {
          do {
            lVar3 = plVar9[3];
            if (lVar3 != 0) {
              if (0xfffff < *(ulonglong *)(lVar3 + 0x20)) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              *(longlong *)(lVar3 + 0x20) = *(longlong *)(lVar3 + 0x20) + 1;
              UNLOCK();
            }
            if (lVar3 != 0) {
              lVar7 = *(longlong *)(param_2 + 8);
              local_40 = lVar7;
              if (lVar7 != 0) {
                if (0xfffff < *(ulonglong *)(lVar7 + 0x20)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                *(longlong *)(lVar7 + 0x20) = *(longlong *)(lVar7 + 0x20) + 1;
                UNLOCK();
              }
              cVar2 = FUN_141811e20(lVar3,local_48);
              if (cVar2 != '\0') {
                FUN_142bf3f70(lVar3);
                plVar8 = (longlong *)*plVar9;
                *(longlong **)plVar9[1] = plVar8;
                *(longlong *)(*plVar9 + 8) = plVar9[1];
                *(longlong *)(param_1 + 0x2c18) = *(longlong *)(param_1 + 0x2c18) + -1;
                FUN_140cbb030(plVar9 + 2);
                thunk_FUN_140205820(plVar9,0x20);
                if (plVar8 != (longlong *)*plVar1) {
                  do {
                    lVar7 = plVar8[3];
                    if (lVar7 == 0) {
                      FUN_142e52ed0(0x431,0);
                      lVar7 = plVar8[3];
                    }
                    FUN_141811f60(lVar7);
                    plVar8 = (longlong *)*plVar8;
                  } while (plVar8 != (longlong *)*plVar1);
                }
                if (0xffffe < *(longlong *)(lVar3 + 0x20) - 1U) {
                  FUN_142e541f0(0x31e);
                }
                LOCK();
                plVar9 = (longlong *)(lVar3 + 0x20);
                lVar7 = *plVar9;
                *plVar9 = *plVar9 + -1;
                UNLOCK();
                if (((int)lVar7 == 1) &&
                   (puVar6 = (undefined8 *)(lVar3 + 0x18), puVar6 != (undefined8 *)0x0)) {
                  (**(code **)*puVar6)(puVar6,1);
                }
                break;
              }
              if (0xffffe < *(longlong *)(lVar3 + 0x20) - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar8 = (longlong *)(lVar3 + 0x20);
              lVar7 = *plVar8;
              *plVar8 = *plVar8 + -1;
              UNLOCK();
              if (((int)lVar7 == 1) &&
                 (puVar6 = (undefined8 *)(lVar3 + 0x18), puVar6 != (undefined8 *)0x0)) {
                (**(code **)*puVar6)(puVar6,1);
              }
            }
            plVar9 = (longlong *)*plVar9;
          } while (plVar9 != (longlong *)*plVar1);
        }
        FUN_142dbb750(plVar1,*plVar1,param_2);
        FUN_140cbb030(param_2);
        return 1;
      }
      uVar4 = FUN_141892840();
      lVar3 = FUN_14182a070(uVar4);
      uVar5 = FUN_1403326d0(lVar3 + 0x18);
      if ((uVar5 & 2) == 0) goto LAB_142d978f6;
      lVar3 = FUN_141813000(param_2);
    }
    FUN_142bf3f70(lVar3);
  }
  FUN_140cbb030(param_2);
  return 0;
}



//===========================================================
// FUN_1407140b0 @ 1407140b0   (127 bytes)
//===========================================================

undefined8 FUN_1407140b0(longlong param_1,int param_2)

{
  longlong lVar1;
  
  if (*(longlong *)(param_1 + 0x128) != 0) {
    for (lVar1 = *(longlong *)
                  (*(longlong *)(param_1 + 0x128) +
                  ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x130)) * 8);
        lVar1 != 0; lVar1 = *(longlong *)(lVar1 + 8)) {
      if (*(int *)(lVar1 + 0x10) == param_2) {
        if (lVar1 != -0x14) {
          return 1;
        }
        break;
      }
    }
  }
  if (*(longlong *)(param_1 + 0x98) != 0) {
    for (lVar1 = *(longlong *)
                  (*(longlong *)(param_1 + 0x98) +
                  ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0xa0)) * 8);
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


