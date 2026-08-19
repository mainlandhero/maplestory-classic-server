
//===========================================================
// FUN_140388c60 @ 140388c60   (416 bytes)
//===========================================================

longlong FUN_140388c60(longlong param_1,int param_2)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  short *local_res8;
  int local_res10 [2];
  undefined1 local_28 [8];
  longlong local_20;
  
  local_res10[0] = param_2;
  if (*(longlong *)(param_1 + 0x88) != 0) {
    for (lVar4 = *(longlong *)
                  (*(longlong *)(param_1 + 0x88) +
                  ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x90)) * 8);
        lVar4 != 0; lVar4 = *(longlong *)(lVar4 + 8)) {
      if (*(int *)(lVar4 + 0x10) == param_2) {
        if (lVar4 != -0x18) {
          FUN_1403ddaa0(param_1,param_2);
          FUN_1403ddbb0(param_1);
          return *(longlong *)(lVar4 + 0x20);
        }
        break;
      }
    }
  }
  if (param_2 == 0) {
    return 0;
  }
  FUN_1403e18a0(&local_res8,param_2);
  if ((local_res8 == (short *)0x0) || (*local_res8 == 0)) {
    lVar4 = 0;
  }
  else {
    FUN_1403b5130(param_1,local_28,param_2,local_res8);
    FUN_140406850(param_1 + 0x88,local_res10,local_28);
    if (local_20 != 0) {
      FUN_1403ddaa0(param_1,param_2);
      FUN_1403ddbb0(param_1);
    }
    lVar4 = local_20;
    if (local_20 != 0) {
      puVar1 = (undefined8 *)(local_20 + -0x28);
      if (0xffffe < *(longlong *)(local_20 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar2 = (longlong *)(lVar4 + -0x20);
      lVar3 = *plVar2;
      *plVar2 = *plVar2 + -1;
      UNLOCK();
      if ((int)lVar3 == 1) {
        if ((local_20 != 0) && (*(longlong *)(local_20 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_20 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_20 + -0x10) + 4) != 0);
        }
        if (puVar1 != (undefined8 *)0x0) {
          (**(code **)*puVar1)(puVar1,1);
        }
      }
      local_20 = 0;
    }
  }
  if (local_res8 != (short *)0x0) {
    FUN_1401bebb0(local_res8 + -8);
  }
  return lVar4;
}



//===========================================================
// FUN_140396250 @ 140396250   (179 bytes)
//===========================================================

void FUN_140396250(undefined8 param_1,int *param_2,longlong param_3,int param_4,int param_5)

{
  int iVar1;
  undefined1 auVar2 [16];
  byte bVar3;
  byte bVar4;
  short sVar5;
  undefined4 uVar6;
  int iVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  undefined8 *puVar11;
  int iVar12;
  short sVar13;
  ulonglong uVar14;
  bool bVar15;
  int *piVar16;
  undefined1 uVar17;
  int aiStack_48 [4];
  
  if (DAT_143ac0180 != 0) {
    uVar6 = FUN_1401b0340(param_3 + 0x20);
    lVar8 = FUN_140388c60(param_1,uVar6);
    bVar3 = FUN_1401b0050(param_3 + 0x16e,*(undefined4 *)(param_3 + 0x172));
    iVar1 = *(int *)(lVar8 + 0x78);
    uVar9 = FUN_1401a1790(param_3 + 0x1ae,*(undefined4 *)(param_3 + 0x1be));
    if (0 < (longlong)uVar9) {
      do {
        aiStack_48[0] = 0;
        uVar14 = uVar9 / 10;
        iVar12 = (int)uVar9 + (int)uVar14 * -10;
        lVar10 = SUB168(SEXT816(-0x5c28f5c28f5c28f5) * SEXT816((longlong)uVar14),8) + uVar14;
        uVar14 = uVar14 + ((lVar10 >> 6) - (lVar10 >> 0x3f)) * -100;
        auVar2._8_8_ = 0;
        auVar2._0_8_ = uVar9;
        lVar10 = SUB168(ZEXT816(0x624dd2f1a9fbe77) * auVar2,8);
        uVar9 = (uVar9 - lVar10 >> 1) + lVar10 >> 9;
        uVar6 = FUN_1401b0340(param_3 + 0x20);
        iVar7 = FUN_140255180(uVar6);
        puVar11 = &DAT_143ac0170;
        bVar15 = iVar7 != 0;
        if (iVar7 == 0) {
          puVar11 = &DAT_143ac0168;
        }
        piVar16 = aiStack_48;
        FUN_140371ad0(DAT_143ac0180,puVar11,(int)((uint)bVar3 + iVar1) / 10,uVar14 & 0xffffffff,
                      piVar16);
        uVar6 = (undefined4)((ulonglong)piVar16 >> 0x20);
        aiStack_48[0] = aiStack_48[0] * iVar12;
        switch((int)uVar14) {
        case 0:
          *param_2 = *param_2 + aiStack_48[0];
          break;
        case 4:
          *param_2 = *param_2 + aiStack_48[0];
        case 1:
          param_2[1] = param_2[1] + aiStack_48[0];
          break;
        case 5:
          *param_2 = *param_2 + aiStack_48[0];
        case 2:
          param_2[2] = param_2[2] + aiStack_48[0];
          break;
        case 6:
          *param_2 = *param_2 + aiStack_48[0];
        case 3:
          param_2[3] = param_2[3] + aiStack_48[0];
          break;
        case 7:
          param_2[1] = param_2[1] + aiStack_48[0];
          param_2[2] = param_2[2] + aiStack_48[0];
          break;
        case 8:
          param_2[1] = param_2[1] + aiStack_48[0];
          param_2[3] = param_2[3] + aiStack_48[0];
          break;
        case 9:
          param_2[2] = param_2[2] + aiStack_48[0];
          param_2[3] = param_2[3] + aiStack_48[0];
          break;
        case 10:
          param_2[5] = param_2[5] + aiStack_48[0];
          break;
        case 0xb:
          param_2[6] = param_2[6] + aiStack_48[0];
          break;
        case 0xc:
          param_2[4] = param_2[4] + aiStack_48[0];
          break;
        case 0xd:
          param_2[7] = param_2[7] + aiStack_48[0];
          break;
        case 0x11:
          sVar5 = *(short *)(lVar8 + 0xcc);
          if (sVar5 == 0) {
            sVar5 = *(short *)(lVar8 + 0xce);
          }
          sVar13 = (short)param_4;
          if (param_4 == 0) {
            sVar13 = sVar5;
          }
          if (*(int *)(lVar8 + 300) == 0) {
            if (bVar15) {
              bVar4 = FUN_1401b0050(param_3 + 0x16e,*(undefined4 *)(param_3 + 0x172));
              iVar7 = FUN_140395850(param_1,(int)sVar13,aiStack_48[0],iVar12,
                                    CONCAT44(uVar6,(uint)bVar4),0,0);
              param_2[0xd] = param_2[0xd] + iVar7;
            }
            else {
              param_2[0xd] = param_2[0xd] + aiStack_48[0];
            }
          }
          else if (bVar15) {
            bVar4 = FUN_1401b0050(param_3 + 0x16e,*(undefined4 *)(param_3 + 0x172));
            iVar7 = FUN_140395850(param_1,(int)sVar13,aiStack_48[0],iVar12,
                                  CONCAT44(uVar6,(uint)bVar4),1,0);
            param_2[0xd] = param_2[0xd] + iVar7;
          }
          else {
            param_2[0xd] = param_2[0xd] + iVar12;
          }
          break;
        case 0x12:
          sVar5 = *(short *)(lVar8 + 0xce);
          if (sVar5 == 0) {
            sVar5 = *(short *)(lVar8 + 0xcc);
          }
          sVar13 = (short)param_5;
          if (param_5 == 0) {
            sVar13 = sVar5;
          }
          if (*(int *)(lVar8 + 300) == 0) {
            if (bVar15) {
              bVar4 = FUN_1401b0050(param_3 + 0x16e,*(undefined4 *)(param_3 + 0x172));
              uVar17 = 0;
              goto LAB_1403965c5;
            }
            param_2[0xe] = param_2[0xe] + aiStack_48[0];
          }
          else if (bVar15) {
            bVar4 = FUN_1401b0050(param_3 + 0x16e,*(undefined4 *)(param_3 + 0x172));
            uVar17 = 1;
LAB_1403965c5:
            iVar7 = FUN_140395850(param_1,(int)sVar13,aiStack_48[0],iVar12,
                                  CONCAT44(uVar6,(uint)bVar4),uVar17,0);
            param_2[0xe] = param_2[0xe] + iVar7;
          }
          else {
            param_2[0xe] = param_2[0xe] + iVar12;
          }
          break;
        case 0x13:
          param_2[0xf] = param_2[0xf] + aiStack_48[0];
          break;
        case 0x14:
          param_2[0x10] = param_2[0x10] + aiStack_48[0];
          break;
        case 0x15:
          param_2[0x11] = param_2[0x11] + aiStack_48[0];
          break;
        case 0x16:
          param_2[0x12] = param_2[0x12] + aiStack_48[0];
          break;
        case 0x17:
          param_2[0x13] = param_2[0x13] + aiStack_48[0];
          break;
        case 0x18:
          param_2[0x14] = param_2[0x14] + aiStack_48[0];
        }
      } while (uVar9 != 0);
    }
  }
  return;
}



//===========================================================
// FUN_1402fd610 @ 1402fd610   (49 bytes)
//===========================================================

bool FUN_1402fd610(longlong param_1)

{
  byte bVar1;
  
  if (*(longlong *)(param_1 + 0x38) != 0) {
    return false;
  }
  bVar1 = FUN_1401b0050(param_1 + 0x1a6,*(undefined4 *)(param_1 + 0x1aa));
  return bVar1 < 0x15;
}


