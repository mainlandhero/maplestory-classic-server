
//===========================================================
// FUN_14030e790 @ 14030e790   (637 bytes)
//===========================================================

longlong * FUN_14030e790(longlong *param_1,longlong *param_2,longlong *param_3)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong *plVar10;
  
  param_1[1] = param_1[1] + 1;
  plVar2 = (longlong *)*param_1;
  plVar8 = (longlong *)*param_2;
  param_3[1] = (longlong)plVar8;
  if (plVar8 == plVar2) {
    *plVar2 = (longlong)param_3;
    plVar2[1] = (longlong)param_3;
    plVar2[2] = (longlong)param_3;
    *(undefined1 *)(param_3 + 3) = 1;
    return param_3;
  }
  if ((int)param_2[1] == 0) {
    plVar8[2] = (longlong)param_3;
    if (plVar8 == (longlong *)plVar2[2]) {
      plVar2[2] = (longlong)param_3;
    }
  }
  else {
    *plVar8 = (longlong)param_3;
    if (plVar8 == (longlong *)*plVar2) {
      *plVar2 = (longlong)param_3;
    }
  }
  cVar1 = *(char *)(param_3[1] + 0x18);
  plVar8 = param_3;
  do {
    if (cVar1 != '\0') {
      *(undefined1 *)(plVar2[1] + 0x18) = 1;
      return param_3;
    }
    plVar9 = (longlong *)plVar8[1];
    plVar7 = plVar8 + 1;
    plVar10 = plVar9 + 1;
    lVar5 = *(longlong *)plVar9[1];
    if (plVar9 == (longlong *)lVar5) {
      lVar5 = ((longlong *)plVar9[1])[2];
      if (*(char *)(lVar5 + 0x18) != '\0') {
        plVar3 = (longlong *)plVar9[2];
        if (plVar8 == plVar3) {
          plVar9[2] = *plVar3;
          if (*(char *)(*plVar3 + 0x19) == '\0') {
            *(longlong **)(*plVar3 + 8) = plVar9;
          }
          plVar3[1] = *plVar10;
          if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
            *(longlong **)(*param_1 + 8) = plVar3;
            *plVar3 = (longlong)plVar9;
            *plVar10 = (longlong)plVar3;
            plVar8 = plVar9;
            plVar9 = plVar3;
            plVar7 = plVar10;
          }
          else {
            plVar8 = (longlong *)*plVar10;
            if (plVar9 == (longlong *)*plVar8) {
              *plVar8 = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
            else {
              plVar8[2] = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
          }
        }
        *(undefined1 *)(plVar9 + 3) = 1;
        *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
        plVar7 = *(longlong **)(*plVar7 + 8);
        plVar10 = (longlong *)*plVar7;
        *plVar7 = plVar10[2];
        if (*(char *)(plVar10[2] + 0x19) == '\0') {
          *(longlong **)(plVar10[2] + 8) = plVar7;
        }
        plVar10[1] = plVar7[1];
        if (plVar7 == *(longlong **)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar10;
          plVar10[2] = (longlong)plVar7;
        }
        else {
          plVar9 = (longlong *)plVar7[1];
          if (plVar7 == (longlong *)plVar9[2]) {
            plVar9[2] = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
          else {
            *plVar9 = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
        }
        goto LAB_14030e9e5;
      }
LAB_14030e917:
      *(undefined1 *)(plVar9 + 3) = 1;
      *(undefined1 *)(lVar5 + 0x18) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar8 = *(longlong **)(*plVar7 + 8);
    }
    else {
      if (*(char *)(lVar5 + 0x18) == '\0') goto LAB_14030e917;
      plVar3 = (longlong *)*plVar9;
      plVar6 = plVar9;
      if (plVar8 == plVar3) {
        *plVar9 = plVar3[2];
        if (*(char *)(plVar3[2] + 0x19) == '\0') {
          *(longlong **)(plVar3[2] + 8) = plVar9;
        }
        plVar3[1] = *plVar10;
        if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar3;
        }
        else {
          puVar4 = (undefined8 *)*plVar10;
          if (plVar9 == (longlong *)puVar4[2]) {
            puVar4[2] = plVar3;
          }
          else {
            *puVar4 = plVar3;
          }
        }
        plVar3[2] = (longlong)plVar9;
        *plVar10 = (longlong)plVar3;
        plVar6 = plVar3;
        plVar8 = plVar9;
        plVar7 = plVar10;
      }
      *(undefined1 *)(plVar6 + 3) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar7 = *(longlong **)(*plVar7 + 8);
      plVar10 = (longlong *)plVar7[2];
      plVar7[2] = *plVar10;
      if (*(char *)(*plVar10 + 0x19) == '\0') {
        *(longlong **)(*plVar10 + 8) = plVar7;
      }
      plVar10[1] = plVar7[1];
      if (plVar7 == *(longlong **)(*param_1 + 8)) {
        *(longlong **)(*param_1 + 8) = plVar10;
      }
      else {
        puVar4 = (undefined8 *)plVar7[1];
        if (plVar7 == (longlong *)*puVar4) {
          *puVar4 = plVar10;
        }
        else {
          puVar4[2] = plVar10;
        }
      }
      *plVar10 = (longlong)plVar7;
LAB_14030e9e5:
      plVar7[1] = (longlong)plVar10;
    }
    cVar1 = *(char *)(plVar8[1] + 0x18);
  } while( true );
}


