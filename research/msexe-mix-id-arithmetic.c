
//===========================================================
// FUN_14041a270 @ 14041a270   (30 bytes)
//===========================================================

undefined8 FUN_14041a270(byte *param_1)

{
  if (((*param_1 < 10) && (param_1[1] < 10)) && ((byte)(param_1[2] - 1) < 99)) {
    return 1;
  }
  return 0;
}



//===========================================================
// FUN_14041a3d0 @ 14041a3d0   (52 bytes)
//===========================================================

int FUN_14041a3d0(byte *param_1)

{
  byte bVar1;
  
  bVar1 = *param_1;
  if (((bVar1 < 10) && (param_1[1] < 10)) && ((byte)(param_1[2] - 1) < 99)) {
    return ((uint)param_1[1] + ((uint)bVar1 + (uint)bVar1 * 4) * 2) * 1000 + (uint)param_1[2];
  }
  return 0;
}



//===========================================================
// FUN_14041aa00 @ 14041aa00   (108 bytes)
//===========================================================

void FUN_14041aa00(undefined4 param_1,int param_2)

{
  FUN_14041a820(param_1,param_2 / 10000 & 0xff,(param_2 % 10000) / 1000 & 0xff,
                (((char)(param_2 / 1000) + (char)(param_2 >> 0x1f)) -
                (char)((longlong)param_2 * 0x10624dd3 >> 0x3f)) * '\x18' + (char)param_2);
  return;
}



//===========================================================
// FUN_14041aa80 @ 14041aa80   (108 bytes)
//===========================================================

void FUN_14041aa80(undefined4 param_1,int param_2)

{
  FUN_14041a8d0(param_1,param_2 / 10000 & 0xff,(param_2 % 10000) / 1000 & 0xff,
                (((char)(param_2 / 1000) + (char)(param_2 >> 0x1f)) -
                (char)((longlong)param_2 * 0x10624dd3 >> 0x3f)) * '\x18' + (char)param_2);
  return;
}



//===========================================================
// FUN_1401a9eb0 @ 1401a9eb0   (164 bytes)
//===========================================================

bool FUN_1401a9eb0(int param_1,int param_2)

{
  char cVar1;
  int iVar2;
  undefined2 local_res18;
  undefined1 local_res1a;
  undefined2 local_res20;
  undefined1 local_res22;
  
  cVar1 = FUN_140419ea0();
  if (cVar1 == '\0') {
    cVar1 = FUN_140419ea0(param_2);
    if (cVar1 == '\0') {
      return param_1 == param_2;
    }
  }
  else {
    cVar1 = FUN_140419ea0(param_2);
    if (cVar1 != '\0') {
      cVar1 = FUN_14041a1e0(param_1,param_2);
      if (cVar1 != '\0') {
        local_res20 = 0xffff;
        local_res22 = 0;
        local_res18 = 0xffff;
        local_res1a = 0;
        FUN_14041a520(param_1,&local_res20);
        FUN_14041a520(param_2,&local_res18);
        iVar2 = FUN_14041a2a0(&local_res20,&local_res18);
        return iVar2 != 0;
      }
    }
  }
  return false;
}



//===========================================================
// FUN_1401a9e00 @ 1401a9e00   (164 bytes)
//===========================================================

bool FUN_1401a9e00(int param_1,int param_2)

{
  char cVar1;
  int iVar2;
  undefined2 local_res18;
  undefined1 local_res1a;
  undefined2 local_res20;
  undefined1 local_res22;
  
  cVar1 = FUN_140419ea0();
  if (cVar1 == '\0') {
    cVar1 = FUN_140419ea0(param_2);
    if (cVar1 == '\0') {
      return param_1 == param_2;
    }
  }
  else {
    cVar1 = FUN_140419ea0(param_2);
    if (cVar1 != '\0') {
      cVar1 = FUN_14041a130(param_1,param_2);
      if (cVar1 != '\0') {
        local_res20 = 0xffff;
        local_res22 = 0;
        local_res18 = 0xffff;
        local_res1a = 0;
        FUN_14041a480(param_1,&local_res20);
        FUN_14041a480(param_2,&local_res18);
        iVar2 = FUN_14041a2a0(&local_res20,&local_res18);
        return iVar2 != 0;
      }
    }
  }
  return false;
}



//===========================================================
// FUN_140417ed0 @ 140417ed0   (1631 bytes)
//===========================================================

char FUN_140417ed0(int param_1)

{
  char cVar1;
  int iVar2;
  
  switch(param_1 / 10000) {
  case 500:
    return '\b';
  case 0x1f5:
    return '\t';
  case 0x1f6:
    return '\n';
  case 0x1f7:
    return '\v';
  case 0x1f8:
    cVar1 = '\x16';
    if (param_1 % 10000 - 4000U < 1000) {
      cVar1 = '<';
    }
    return cVar1;
  case 0x1f9:
    iVar2 = param_1 % 0x4d0e90;
    if (iVar2 == 100) {
      return 'A';
    }
    if ((iVar2 != 1000) && (iVar2 != 0x3e9)) {
      return (param_1 != (param_1 / 10) * 10) + '\x17';
    }
    return '/';
  case 0x1fa:
    switch(param_1 / 1000) {
    case 0x13c4:
      if (param_1 != (param_1 / 10) * 10) {
        return '\x1a';
      }
      if ((param_1 / 10) % 10 == 1) {
        return 'S';
      }
      return '\x19';
    case 0x13c6:
      param_1 = param_1 % 1000;
      if (param_1 < 0xc9) {
        if (param_1 == 200) {
          return '>';
        }
        if (param_1 == 9) {
          return 'P';
        }
        if ((param_1 == 100) || (param_1 == 0x67)) {
          return '-';
        }
      }
      else if (param_1 < 0x1f5) {
        if (param_1 == 500) {
          return 'M';
        }
        switch(param_1) {
        case 0xc9:
          return '?';
        case 0xca:
          return '@';
        case 0x12d:
          return 'I';
        case 400:
        case 0x193:
        case 0x195:
          return 'F';
        case 0x191:
          return 'G';
        case 0x192:
          return 'H';
        case 0x196:
          return '[';
        case 0x197:
          return '\\';
        case 0x198:
          return ']';
        }
      }
      else {
        if (param_1 == 0x1f5) {
          return 'M';
        }
        if (param_1 == 800) {
          return 'P';
        }
      }
      return ',';
    case 0x13c7:
      if ((param_1 % 1000) / 100 == 1) {
        return '=';
      }
      return '1';
    case 0x13c8:
      iVar2 = (param_1 % 1000) / 100;
      if (iVar2 == 1) {
        return '7';
      }
      if (iVar2 == 3) {
        return ':';
      }
      return '0';
    case 0x13c9:
      cVar1 = 'D';
      if (param_1 % 0x4d4928 != 100) {
        cVar1 = '3';
      }
      return cVar1;
    case 0x13cc:
      iVar2 = (param_1 % 1000) / 100;
      if (iVar2 == 1) {
        return '8';
      }
      if (iVar2 == 2) {
        return ';';
      }
      if (iVar2 == 3) {
        return 'T';
      }
      return '2';
    }
    break;
  case 0x1fb:
    param_1 = param_1 % 10;
    if (param_1 == 0) {
      return '\f';
    }
    if (param_1 == 1) {
      return '\r';
    }
    if (param_1 == 6) {
      return '\x0e';
    }
    if (param_1 == 7) {
      return '&';
    }
    if (param_1 == 8) {
      return '\x0f';
    }
    break;
  case 0x1fc:
    return '\x12';
  case 0x1fd:
    return '\x15';
  case 0x1fe:
    return '\x14';
  case 0x200:
    return '\x10';
  case 0x201:
    param_1 = param_1 % 0x4e4710;
    if ((param_1 != 3000) && (param_1 != 0xbb9)) {
      if (param_1 == 4000) {
        return 'K';
      }
      return '\a';
    }
    break;
  case 0x202:
    return '\x04';
  case 0x203:
    switch(param_1 / 1000) {
    case 0x141e:
    case 0x1422:
      return '\x01';
    case 0x141f:
      if (param_1 == 0x4e9940) {
        return '^';
      }
      if (param_1 == 0x4e99e0) {
        return 'Y';
      }
      return 'X';
    case 0x1420:
      iVar2 = param_1 / 100;
      if (iVar2 != 0xc940) {
        if (iVar2 == 0xc941) {
          return '\x1f';
        }
        if (iVar2 != 0xc942) {
          if (iVar2 == 0xc943) {
            cVar1 = 'V';
            if (param_1 == 0x4e9e2e) {
              cVar1 = '_';
            }
            return cVar1;
          }
          if (iVar2 != 0xc944) {
            return '\0';
          }
        }
      }
      return '\x02';
    case 0x1421:
      return '\x03';
    case 0x1423:
      return '9';
    case 0x1425:
      return 'Q';
    case 0x1426:
      return 'R';
    }
    break;
  case 0x204:
    return '\x06';
  case 0x205:
    if (param_1 == (param_1 / 10000) * 10000) {
      return '\x11';
    }
    break;
  case 0x206:
    return '\x05';
  case 0x207:
    return '\x1b';
  case 0x208:
    cVar1 = '\x13';
    if (param_1 % 5200000 - 4000U < 1000) {
      cVar1 = 'E';
    }
    return cVar1;
  case 0x20b:
    return (param_1 % 0x4fcdb0 == 3) + '\x1c';
  case 0x20c:
    return '\x1e';
  case 0x20d:
    if (param_1 % 0x501bd0 != 500) {
      return (param_1 % 0x501fb8 == 100) + '\"';
    }
    return 'C';
  case 0x210:
    if (param_1 - 0x5094e8U < 1000) {
      return 'N';
    }
    break;
  case 0x215:
    return ' ';
  case 0x219:
    return '!';
  case 0x21b:
    return 'O';
  case 0x221:
    return '$';
  case 0x223:
    return '%';
  case 0x226:
    cVar1 = '\'';
    if (param_1 - 0x53f048U < 1000) {
      cVar1 = '4';
    }
    return cVar1;
  case 0x227:
    return '(';
  case 0x228:
    cVar1 = ')';
    if (param_1 % 10000 == 1000) {
      cVar1 = '6';
    }
    return cVar1;
  case 0x229:
    return '*';
  case 0x232:
    return '+';
  case 0x238:
    if (((param_1 == 0x56ac1d) || (param_1 == 0x56ac1f)) || (param_1 == 0x56ac79)) {
      return 'U';
    }
    if (param_1 == 0x56ac5e) {
      return 'W';
    }
    if (param_1 == 0x56aeae) {
      return 'Y';
    }
    break;
  case 0x23a:
    return '5';
  case 0x242:
    param_1 = param_1 / 1000;
    if (param_1 == 0x1695) {
      return 'L';
    }
    if (param_1 == 0x1696) {
      return 'Z';
    }
    if (param_1 == 0x1697) {
      return 'a';
    }
    return 'J';
  case 0x24b:
    if (param_1 - 0x5991b0U < 1000) {
      return '`';
    }
  }
  return '\0';
}



//===========================================================
// FUN_140419fb0 @ 140419fb0   (7 bytes)
//===========================================================

bool FUN_140419fb0(uint param_1)

{
  return param_1 < 8;
}



//===========================================================
// FUN_14041a0f0 @ 14041a0f0   (47 bytes)
//===========================================================

int FUN_14041a0f0(uint param_1)

{
  if (9999999 < (int)param_1) {
    param_1 = param_1 / 1000;
  }
  return (int)param_1 % 10;
}



//===========================================================
// FUN_14041a7e0 @ 14041a7e0   (48 bytes)
//===========================================================

int FUN_14041a7e0(uint param_1,int param_2)

{
  if (9999999 < (int)param_1) {
    param_1 = param_1 / 1000;
  }
  return param_2 + ((int)param_1 / 10) * 10;
}



//===========================================================
// FUN_140419fa0 @ 140419fa0   (7 bytes)
//===========================================================

bool FUN_140419fa0(uint param_1)

{
  return param_1 < 9;
}



//===========================================================
// FUN_14041a0a0 @ 14041a0a0   (73 bytes)
//===========================================================

int FUN_14041a0a0(uint param_1)

{
  if (9999999 < (int)param_1) {
    param_1 = param_1 / 1000;
  }
  return ((int)param_1 / 100) % 10;
}



//===========================================================
// FUN_14041a780 @ 14041a780   (82 bytes)
//===========================================================

int FUN_14041a780(uint param_1,int param_2)

{
  if (9999999 < (int)param_1) {
    param_1 = param_1 / 1000;
  }
  return ((((int)param_1 / 1000) * 10 - (int)param_1 / 100) + param_2) * 100 + param_1;
}


