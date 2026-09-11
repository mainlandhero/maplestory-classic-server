# The blank character-select screen, measured against `0x007A` - 2026-09-10

The owner: *"Sometimes when the login happens too fast through transitions, the characters on
character select do not render at all. I cannot re-produce it consistently though, but I do
know it happens more for clients that are further away than those that have lower ping."*

## What was already known

`login::session::CHARACTER_LIST_PAUSE_MS` (2026-09-02): the avatars are not drawn on a
client's first visit to character select; the `0x0010` bytes of a visit that drew and one
that did not are identical; the server answers the login request with four packets in one
millisecond where a real service takes three round trips. A fixed 400 ms wait before the
character list made the avatars draw. Labelled **[I]** at the time: nothing had been read
out of the client saying what it was waiting for.

## The instrument: the client's own completion report

`0x007A CLIENT_TASK_TIMING_REPORT` is sent by the client's background worker
(`FUN_141b0ef00`) at the moment **all four of its background tasks are done**, with each
task's duration and their sum (`research/msexe-client-opcodes.md`, established from the
builder). It is sent once per process. That makes it a timestamp for "the client finished
its start-up work", visible in every archived login log.

## The measurement: when `0x007A` arrives, relative to the login request

Every login in `previous-runs/login*.log` and `research/fixtures/*login*.log`, paired with
the login request before it and the list's send time:

```text
                                 list sent at      0x007A arrives at
  2026-08-19 .. 2026-09-02        +0.000 s          +0.27 .. +0.46 s   (median ~0.35)
  2026-09-03 .. 2026-09-07        +0.401 s          +0.401 .. +0.447 s (mostly +0.409)
  2026-09-08 .. 2026-09-10        +0.401 s          +0.60 .. +0.81 s   (today +0.814)
```

Three regimes, and one reading fits all three [I]:

* **One of the four tasks ends when the character list arrives.** In the middle regime the
  report lands 5-10 ms behind the list on every login; if the tasks were independent of the
  list they would have kept landing at ~0.35 s, in front of it.
* **The other tasks take a machine- and build-dependent time.** ~0.35 s until 2026-09-07,
  0.6-0.8 s since 2026-09-08 (the build that day added the guard-page hook and the
  session-token work; something in it lengthened the client's start-up).
* **The avatars draw when the list arrives after those other tasks, and not when it arrives
  before them.** Pre-pause: list at 0, tasks until 0.35 - blank. Pause era: list at 0.40,
  tasks done by 0.35 - drew. Since 09-08: list at 0.40, tasks until 0.6-0.8 - **the race is
  back**, and whether a given login wins it depends on how fast that machine gets through
  its tasks. A slower or more distant client loses it more often, which is the owner's report.

Today's four durations were `787, 151, 128, 162` ms (sum 1228): the first task alone
outlasts the 400 ms pause by a wide margin.

## Why "wait for the report" is not the fix

If a task ends on the list's arrival, a server that withholds the list until the report
arrives never sends it. The report cannot be the trigger for the first send.

## What is wired instead - an experiment, labelled

The list still goes out 400 ms after the login request. When `0x007A` then arrives, the
server compares its arrival with the list's send time:

* report within `LIST_RESEND_THRESHOLD_MS` of the list -> the tasks were already done when
  the list landed (the middle regime); nothing more is sent;
* report later than that -> the tasks finished **after** the list landed, the client built
  its select screen before it was ready, and the server **sends the character list again**,
  now that every task is done.

**[I]:** the effect of a second `0x0010` in the select stage has not been read out of the
client. The leave-world path re-sends a list into the same stage and the avatars draw, but
that path also re-sends the world list first. The re-send is behind a kill switch
(`--no-list-resend`) and the plan step says what each screen outcome means. The four
durations are now parsed into the log on every report, so the next question - which task is
the slow one - has data waiting for it.
