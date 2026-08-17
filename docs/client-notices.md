# Client notices: the baked dialog table

`Login.img` `/Notice/text/` is the client's dialog message table, and every entry is a
**bitmap**, not a string. None of this text can be found by searching the executable. Render
it instead:

```bash
./target/release/wz-dump.exe canvas client-patched/Data/UI/_Canvas/_Canvas_000.wz \
    Login.img <outdir> Notice
python tools/wz_png.py <outdir>
```

170 text-bearing canvases: `text/` 165, `textL/` 1, `textL2/` 2, `textL3/` 2. All decode -
169 are canvas format 1, one (`text/155`) is format 513. See `docs/session.md` for how this
capability was built and why it matters.

## Named entries

The code raises these **by name**, as wide string literals, so a name found here is
greppable in the binary and leads to the branch that raises it. The login-result mapping in
`FUN_141b2a280` (and its near-duplicate `FUN_141b267c0`) is in `docs/session.md`.

Known names include `loginTroubleAskSupport`, `incorrectPassword`, `notRegisteredID`,
`blockedID`, `blockedIPAddr`, `loginAlready`, `loginTimeout`, `notAdult`, `invalidRegion`,
`notRegisteredAccount`, `accountNotVerified`, `outOfServiceRegion`, `notVerifiedEmail`,
`cannotProcessRequest`, `accountSuspended`, `temporaryBlockedIPAddr`, `incorrectPIC`,
`incorrectSecondPassword`, `unableLogOnToGameSvr`, `unavailableClass`.

**`loginTroubleAskSupport` is unique** - an exhaustive render of all 170 found no other node
with that wording, and no numeric twin. Its text:

> Having trouble logging in?
> Try logging in at maplestory.nexon.net
> or visit the Nexon homepage
> to view support options.

## Numeric entries

Indexed by message id. Note these ids are **not** login result codes - there is no `101`
entry, and the login result resolves through the named entries above.

| # | Text (abridged) |
|---|---|
| 0 | Restriction on character names that include or suggest vulgarisms |
| 1 | To delete your character, enter your 8-digit birthday code |
| 2 | This is not a registered email or an incorrect email |
| 4 | This is an incorrect email |
| 8 | This name is currently being used |
| 12 | Press CHECK to see if the character name is available |
| 28 | You have entered an invalid login ID |
| 30 | This account has not been verified |
| 32 | The server is under maintenance |
| 36 | This world is already full of adventurers |
| 37 | Please select a channel |
| 38 | GameGuard has been updated. Please restart the game |
| 39 | Engaged / wedding-booked characters cannot be deleted |
| 40 | You are attempting to login from outside the service region |
| 41 | The two PIC you entered do not match |
| 44 | If disabled, you will be exposed to hacking and other risky factors |
| 57 | Deleting this character removes them from your Family |
| 58 | ...and deletes any Cash Items they hold |
| 59 | (KR) SSN collection consent |
| 60 | (KR) invalid resident number |
| 61 | A system error occurred during identification |
| 68 | Character is still at their Part-Time Job |
| 69 | Cannot delete while a Hired Merchant is in the Free Market |
| 70 | For security, please set a 2nd password |
| 73 | Please login using your Nexon e-mail ID |
| 76 | Maple ID successfully created |
| 77 | This Maple ID cannot be created |
| 78 | Please login using Maple ID or Nexon e-mail ID |
| 88 | (KR) Parallel coupon expired |
| 105 | Character transfer complete |
| 109-113 | Star Planet entry / Shining Star character |
| 118 | Wrong password too many times; temporarily blocked |
| 120 | Character only available until a given date |
| 121 | Your PIC has been set |
| 123 | Delete the selected character? (Burning warning) |
| 129 | Complete authentication on the website, then click Confirm |
| 130 | This job cannot be a Burning character |
| 133 | Character creation restricted in this world |
| 136 | This character is already being copied |
| 137 | Unavailable - too many people waiting |
| 140 | Characters only in one of Burning / Burning 2 / Burning 3 |
| 142 | Characters with locked items cannot be deleted |
| 145 | Yeti x Pink Bean world character creation |
| 147 | Designating an existing character as Hyper Burning |
| 150 | Temporarily blocked for suspicious activity |
| 155 | Exhausted all available Burning World leaps |
| 902-912 | PIC management (set, deactivated, reuse, insecure, repetitive, must set) |
| 904 | Please select a channel |
| 905 | GameGuard has been updated |
| 906 | Your country's IP is blocked from Global MapleStory |
| 10000 | This class is currently unavailable |

Aliases - the only non-identity `_outlink`s under `/Notice/`, all in `UI_000.wz`:
`text/901` -> `text/61`, `text/10001` -> `text/10000`, `text/unavailableClass` -> `text/10000`.

Also under `/Notice/`: `Loading/backgrnd` ("Connecting to server") and `Loading/backgrnd2`
("Loading the selected character") - the spinner the owner describes when the live client's Login
button is pressed. All eight `/Notice/backgrnd/*` frames are blank; they are the dialog
frame, not text.

`textL3/blockMapleID` is dark text on transparent and needs compositing onto white to read.
