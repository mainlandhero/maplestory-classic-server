# Rain's quiz: the CLIENT conducts it - 2026-09-13

The owner, with five timestamped screenshots (time.gov clock in shot) of quest 1016, "Rain's Maple
Quiz 4": *"Rain still repeats their dialogue when I select the right answer and then press OK. I
believe the quest should be immediately completed when I select the right answer, and then the
following OK dialogue should be the end of the conversation."*

## What the screenshots + log prove

The screenshots span 17:20:14 -> 17:21:09 UTC. `world.log` (channel 0, the channel the owner was on)
for that window:

```text
17:20:10  last unrelated packet
   ...    17:20:14 SS1 quest-list "Rain's Maple Quiz 4 (Complete)"
   ...    17:20:27 SS2 the question "Which of the following is not a correct way to recover health?"
   ...    17:20:48 SS3 "Yes, that's correct! ... 35 Exp", OK
          <- NOTHING inbound but 0x00B8/0x013D/0x02F4 (toggles, telemetry) this whole time
17:20:55  <- 0x0151 action 2 (turn-in), quest 1016      SS4 the question AGAIN
17:21:06  <- 0x00F3 06 01 03000000  choice 3 (Sleep)
          -> 0x0089 completed, 0x007C +350 exp LEVEL 10->11, 0x055B Say "That's right!"   SS5
17:21:19  <- 0x00F3 (OK on the Say)  -> nothing
```

**For 41 seconds the client showed the entire quiz - offer, question, and the "that's correct"
box - with no quest or script packet at all.** The EXP bar stayed at 88.05% through SS4 and only
jumped at SS5. So SS1-SS3 were drawn by the CLIENT from its own `Quest.wz`: the `#L` choices and
`stop.0.answer` are in the quest data, and the client renders the menu, grades the choice, and
shows the closing line itself. It sends the turn-in (`0x0151` action 2) **only after** a right
answer.

The server then answered that turn-in by asking the same question a **second** time (SS4), so
The owner answered twice. When the server's redundant menu was NOT wanted, the client dismissed it
unanswered - `06 00`, world.log 16:58:34, an earlier turn-in of the same quest.

The "worked" case (quest 1015, 16:37) is **[I]**: an 8 s packet-free gap sits between its accept
and its turn-in, which is room for the client to have run the quiz locally before the server's menu
was answered by hand - a double the owner did not screenshot. Not measured; only 1016 is.

**Unverified on screen:** a quiz turn-in answered by the completion record and no box. The
precedent is the silent accept (Nina, 2026-09-13), which the release the owner played did not freeze on;
the 16:58 dismissal says the client did not want a box. A turn-in is still a different request, so
plan step TO(t) watches for it.

## The rule

**The client conducts the quiz; the server only finalises it.** A `0x0151` action-2 turn-in of a
quiz quest (its completion path carries `ask`) means "the player answered correctly" - the client
withholds the turn-in on a wrong answer, showing the `stop` line and re-asking locally. So the
server records the completion (record, exp, fanfare) and says **nothing**: a box here repeats the
closing line the client already drew, and a menu here re-asks. `Session::on_quest_request`'s
`quiz_turn_in` arm returns right after the record, the same shape as `silent_accept`.

## What was removed

The server-side quiz driver was built on the theory that the server asks and grades the quiz
(`quiz_menu_answer`, `quiz_answer_key`, the `.quiz.retry` path, `quiz_completion_pending`, the
`quiz` branch in `say_line`). The screenshots show that theory is wrong for this client, so all of
it is gone - it was unreachable the moment the turn-in stopped sending a menu, and leaving it
would say "the server grades the quiz" when it does not. `git show` for the diff.

## Tests

* `a_quiz_turn_in_completes_silently_because_the_client_conducts_the_quiz` - a quiz turn-in
  records the completion and opens no box; a second click is silent.
* `no_quest_answers_its_accept_or_turn_in_with_its_own_opening_lines` - the all-quests audit now
  expects the 11 quiz turn-ins to complete silently rather than to send a menu.
