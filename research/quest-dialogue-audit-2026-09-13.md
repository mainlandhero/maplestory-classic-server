# The repeat-dialogue audit - 2026-09-13

The owner: *"Please audit all of the questline and make sure repeat dialogue is no longer a concern."*

Three repeats were fixed today before this was asked, and each was the same mistake in a
different place: the server spoke a line the client was already showing on its own.

| when | what repeated | why | fix |
|---|---|---|---|
| Accept on Nina's 1003 | the two opening lines | no `0.yes`, so the answer fell back to `Say.0` | an accept with no `yes` branch sends the record alone |
| Rain's quiz, right answer | the next quiz's opening | the completion went out before the question; the client offered the chain's next quest under the menu | a quiz's completion waits for the right answer |
| Rain's quiz, the question | (the client died) | a `#L` list in a Say box | an `ask` node's first line is a menu |

## 1. Who speaks which line

Measured across the archive and today's runs:

* **The opening, `Say.0`, is the client's.** It shows it before the Accept button (2026-08-19,
  Heena: *"'You must be the new traveler' exists and gets handled on client side"*) and sends
  `0x0151` action 1 only after. **[L]**
* **The `yes` branch, `Say.0.yes`, is the server's**, in answer to action 1 (2026-08-20, quest
  1000, on screen). **[L]**
* **The completion, `Say.1`, is the server's**, in answer to action 2. If the client showed it
  itself, Rain's question would have been drawn twice and the first attempt (a Say with `#L`
  tags) could not have faulted the client on OUR packet. **[D]**
* **The opening SCRIPT (action 4) is the server's.** A scripted quest has no local text; the
  archive holds 27 action-4 requests, every one quest 1002 (Roger, whose lines are authored),
  answered with `Say.0` and a yes/no, and none of them was ever reported as a repeat. **[L]**
* **The chain.** A turn-in whose quest has no `Say.1` but a `nextQuest` with a `Say.0` speaks
  that opening and STARTS the next quest in the same reply, so the client - which offers a
  quest only while it is offerable - does not show the opening again. Seen for 1000 -> 1001 on
  2026-08-20 (*"How am I going to hang all these up?"*, the line the owner expected). **[L]**

So the invariant: **the server never answers a quest's action 1 or action 2 with a `Say.0`
line of that same quest, and never opens two boxes for one request.**

## 2. The shapes, from the data

`gm-handbook/quests.json`, every quest's `Say` tree reduced to which nodes exist:

```text
(0, 0.yes, 0.no, 1, 1.yes, ask on 1, nextQuest)     count   example
 y  y   y   y   y   -   -                             74     10001
 y  -   -   y   y   -   -                             54     10002
 y  y   y   y   -   -   next                          46     1009
 y  -   -   y   -   -   next                          45     10106
 y  -   -   y   -   -   -                             42     10212
 y  y   y   y   -   -   -                             14     10100
 -  -   -   -   -   -   -                              9     1002   (authored elsewhere)
 y  y   -   y   -   -   next                           9     12118
 y  y   y   -   -   -   next                           6     1000   (the chain shape)
 y  y   y   y   y   -   next                           5     10000
 -  -   -   -   -   -   next                           4     20002
 y  -   -   -   -   -   -                              4     20003
 y  -   -   y   y   -   next                           3     10109
 y  y   -   y   y   -   -                              3     10316
 y  -   y   y   y   -   -                              2     1001
 y  -   y   -   -   -   next                           1     1003
 y  -   y   y   -   -   -                              1     1004
```

Eighteen nodes carry `ask`: eleven on path `1` (the quizzes the server asks) and seven on
path `0` (openings the client shows itself, choices included).

## 3. The test

`crates/world/src/session/tests.rs::no_quest_answers_its_accept_or_turn_in_with_its_own_opening_lines`
walks every quest with dialogue through action 1 and action 2 in one session and asserts the
invariant, plus the positive half per shape:

```text
audited 316 quests
  accept   -> 157 spoke the yes branch, 159 sent the record alone
  turn-in  -> 287 spoke Say.1, 11 asked a quiz, 7 chained, 11 sent the record alone
```

It found one more case on its first run: **quest 1002's turn-in** (no `Say.1`, no chain) fell to
the NPC's `d0` greeting - the "unknown quest" fallback applied to a known quest with nothing to
say. A finished quest's window closes on the record; the greeting is not what it sounds like.
The `silent_accept` return covers turn-ins now.

## 4. Left as is, and why

* **Action 6** (a requirement check failed on the client) takes the `"0"` arm and would speak
  the opening with a yes/no. It has never arrived in any capture; if it does, the right answer
  is probably nothing (the client shows the `stop` text itself), and that is a one-line change
  once a capture says which quest and what the screen showed.
* **A completion prompt's Yes** (`1.yes`, 141 quests) calls `accept_quest`, which is a no-op for
  a quest already complete (`record_quest_start` returns early on `Complete`) and then speaks
  the branch. Not a repeat; noted because the name misleads.
* **The chain's Yes/No.** A chained-to quest whose own `Say.0` ends with a `yes` branch gets a
  yes/no on its last line; Yes re-sends the start record for a quest already started and speaks
  `0.yes`. One line each, once.
