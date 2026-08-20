//! The chat box: what the player says out loud, and the `!` commands typed into it.
//!
//! Every command acknowledges itself. A command that silently works and a command that
//! silently does nothing look identical on screen, and telling them apart has cost
//! client launches.

use super::*;

impl Session {

    /// GM commands typed into the chat box.
    ///
    /// **This is a debugging tool on a server where nothing authenticates**, so there is no
    /// permission check to write - every connection is already the same account, and adding
    /// one here would be theatre. Say so rather than implying otherwise.
    ///
    /// Chat is fire-and-forget: the client froze on none of the runs where it went
    /// unanswered, so a command that does nothing is safe.
    ///
    /// ## The prefix is `!`, not `/`, and that is not a preference
    ///
    /// **The client never transmits a `/` line.** The owner typed `/map 1` and the session's
    /// entire capture contains no `0x00E7` at all, while a plain "Hello" in the same tab had
    /// produced one. The client parses slash commands itself: `/find`, `/whisper`, `/party`,
    /// `/friend`, `/trade`, `/level` and `/h` are all baked into the executable as strings,
    /// and an unknown one is swallowed before it reaches the wire.
    ///
    /// So a server-side command has to look like ordinary chat. `!` is ordinary chat.
    pub(super) fn on_chat(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(text) = net::opcode::parse_chat(payload) else { return Vec::new() };
        let text = text.trim();

        // **Anything that is not a command is said out loud.** The client draws nothing for
        // its own chat: typing sends `0x00E7` and stops. The owner, 2026-08-19, typed "Hello",
        // "Hello2" and "Hello3" and saw nothing at all, because this function matched them
        // against `!map`, found nothing, and returned an empty reply. The balloon and the
        // chat-log line both come from `0x0231` coming back - see net::userchat.
        let Some(command) = text.strip_prefix('!') else {
            return self.say_out_loud(text);
        };
        let (name, arg) = match command.split_once(char::is_whitespace) {
            Some((n, a)) => (n, a.trim()),
            None => (command, ""),
        };
        match name {
            "map" => self.gm_map(arg),
            "item" => self.gm_item(arg),
            "exp" => self.gm_exp(arg),
            "heal" => self.gm_heal(),
            "help" => self.gm_ack(GM_COMMANDS.to_string()),
            "" => self.gm_ack(format!("Not a command. {GM_COMMANDS}")),
            other => self.gm_ack(format!("!{other} is not a command. {GM_COMMANDS}")),
        }
    }


    /// `!map <id>` - put the character on a map.
    pub(super) fn gm_map(&mut self, arg: &str) -> Vec<Reply> {
        let Ok(map) = arg.parse::<u32>() else {
            return self.gm_ack(format!("!map: {arg:?} is not a map id. Try !map 40."));
        };
        // Refuse a map the client cannot load. A character sent to an id with no field image
        // is stranded with no way back except another command.
        if !self.config.map_exists(map) {
            return self.gm_ack(format!(
                "!map REFUSED: {map} has no field image in this client, so it would strand you."
            ));
        }
        let Some(mut chr) = self.claimed_character() else {
            return self.gm_ack("!map REFUSED: no character is claimed on this connection.".to_string());
        };
        let mut out = self.gm_ack(format!(
            "Teleporting {} to map {map}, {}",
            chr.name,
            self.map_name(map)
        ));
        // Portal 0 is the map's spawn point, which is where a GM warp should land.
        out.extend(self.go_to_map(&mut chr, map, 0, format!("GM !map {map}")));
        out
    }


    /// `!exp <amount>` - award experience, and level up if it pays for a level.
    ///
    /// A shortcut to the same machinery a kill uses, so goal D can be exercised without
    /// finding a mob: [`Session::award_experience`] does the levelling, the persistence and
    /// the `0x007C`. It adds rather than sets, because what is being checked is that a
    /// number *moves*.
    ///
    /// Three things this doc used to say are no longer true and are worth naming, because
    /// each was fixed by something measured rather than by a rewrite: levelling was "blocked
    /// on why the client collects zero targets" (the mob size scale, fixed); the bar "will
    /// not move until the next field entry" (`0x007C` moves it in place); and `exp` was "a
    /// hardcoded zero" in the record (it is a real column now).
    pub(super) fn gm_exp(&mut self, arg: &str) -> Vec<Reply> {
        let Ok(amount) = arg.parse::<u64>() else {
            return self.gm_ack(format!("!exp: {arg:?} is not an amount. Try !exp 100."));
        };
        let Some(chr) = self.claimed_character() else {
            return self
                .gm_ack("!exp REFUSED: no character is claimed on this connection.".to_string());
        };
        // One path for every source of experience - see `Session::award_experience`. `!exp`
        // used to add and persist on its own, which meant a GM award could never level
        // anyone while a kill could, and nothing would have said so.
        let before = chr.exp;
        let mut out = self.award_experience(amount, "!exp");
        if out.is_empty() {
            return self.gm_ack(format!("!exp {amount}: nothing to award."));
        }
        let now = self.claimed_character().map(|c| c.exp).unwrap_or(before);
        let mut ack = self.gm_ack(format!("{} gains {amount} experience: {before} -> {now}.", chr.name));
        ack.append(&mut out);
        ack
    }


    /// `!heal` - back to full HP and MP.
    ///
    /// **This exists because death does not.** Mobs deal contact damage now, and a character
    /// that reaches zero HP is disabled by the client with no way back: `research/user-hit.md`
    /// established that `hp = 0` will not hang the client, but not what plays the death and
    /// revive sequence. Until that is decoded, this is the way out - and it is better than
    /// silently clamping HP at 1, which would hide the fact that death is missing.
    pub(super) fn gm_heal(&mut self) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else {
            return self.gm_ack("!heal REFUSED: no character is claimed on this connection.".to_string());
        };
        chr.hp = chr.max_hp;
        chr.mp = chr.max_mp;
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.gm_ack(format!("!heal FAILED: {e}"));
        }
        let mut out = self.gm_ack(format!("{} is restored to {} HP.", chr.name, chr.max_hp));
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { hp: Some(chr.hp), mp: Some(chr.mp), ..Default::default() }
                .build(),
            what: format!("StatChanged: healed to {}/{} hp, {}/{} mp", chr.hp, chr.max_hp, chr.mp, chr.max_mp),
        });
        out
    }


    /// `!item <itemId> [count]` - put an item in the bag, in the tab its id belongs to.
    ///
    /// **The owner asked for this as a safety net**, in these words: *"since most likely my item
    /// will disappear, I need to request a new GM command called `!item <itemID>` which will
    /// add that item into my inventory in the proper tab."* Dropping is not built, so an item
    /// dragged out of the window is currently refused rather than lost - but the moment
    /// dropping *is* built, this is what puts a mistake right.
    ///
    /// The tab comes from the id's leading digit (`store::InventoryType::for_item`), which is
    /// this game's own convention: 1 equip, 2 use, 3 setup, 4 etc, 5 cash.
    ///
    /// **An unknown id is refused rather than sent.** The client has to render whatever
    /// arrives, and an item body for an id with no `Item.wz` entry is exactly the kind of
    /// thing that has faulted it before.
    pub(super) fn gm_item(&mut self, arg: &str) -> Vec<Reply> {
        let mut parts = arg.split_whitespace();
        let Some(Ok(item_id)) = parts.next().map(str::parse::<u32>) else {
            return self.gm_ack(format!("!item: {arg:?} is not an item id. Try !item 1302000."));
        };
        let count: u16 = parts.next().and_then(|c| c.parse().ok()).unwrap_or(1).max(1);

        let Some(inv) = store::InventoryType::for_item(item_id) else {
            return self.gm_ack(format!(
                "!item REFUSED: {item_id} is not in any inventory tab - ids start 1..5."
            ));
        };
        if !self.config.item_names.contains_key(&item_id)
            && !self.config.shops.item_data.contains_key(&item_id)
        {
            return self.gm_ack(format!(
                "!item REFUSED: {item_id} is not in this client's Item.wz, so it has nothing to draw."
            ));
        }
        let Some(chr) = self.claimed_character() else {
            return self.gm_ack("!item REFUSED: no character is claimed on this connection.".to_string());
        };

        let is_equip = inv == store::InventoryType::Equip;
        let item = if is_equip {
            store::Item::equip(item_id)
        } else {
            store::Item::bundle(item_id, count)
        };
        let max_stack = self
            .config
            .shops
            .item_data
            .get(&item_id)
            .map(|d| d.slot_max.max(1))
            .unwrap_or(1);

        let placed = match self.store.add_item(chr.id, inv, &item, max_stack) {
            Ok(rows) => rows,
            Err(e) => return self.gm_ack(format!("!item REFUSED: {e}")),
        };

        let name = self.item_name(item_id);
        let mut out = self.gm_ack(format!(
            "Giving {} {count}x {name} ({item_id}) -> {inv:?} tab, slot {}",
            chr.name,
            placed.iter().map(|r| r.slot.to_string()).collect::<Vec<_>>().join(", ")
        ));
        out.extend(self.inventory_added_replies(inv, &placed, "GM !item"));
        out
    }


    /// A map's name, for a line a person reads. The id alone if there is no table.
    pub(super) fn map_name(&self, map: u32) -> String {
        self.config.map_names.get(&map).cloned().unwrap_or_else(|| "unnamed".to_string())
    }


    /// An item's name, for a line a person reads.
    pub(super) fn item_name(&self, item_id: u32) -> String {
        self.config.item_names.get(&item_id).cloned().unwrap_or_else(|| "unnamed".to_string())
    }


    /// Acknowledge a GM command on screen.
    ///
    /// **The owner asked for every GM command to say what it is about to do**, rather than the
    /// only feedback being a refusal. A command that silently works and a command that
    /// silently does nothing look identical on screen, and telling them apart has cost
    /// launches.
    ///
    /// It goes out as [`net::notice::CHAT_NOTICE`], the same `0x00BB` a refusal already
    /// used. **What colour that renders is not established.** It reaches the chat window
    /// through printer type 7, which is also how the client's own `[Welcome] Welcome to
    /// MapleStory!!` line arrives - and that line is yellow on screen - so yellow is the
    /// expectation. It is an expectation, not a measurement, and the run will settle it.
    /// `net::notice` records that colour is not controllable through this packet.
    pub(super) fn gm_ack(&self, text: String) -> Vec<Reply> {
        self.notice(text)
    }


    /// Say something as the player: a balloon over the head and a line in the chat log.
    ///
    /// **An empty message is dropped rather than sent.** The client's own box will not
    /// submit one, so an empty `0x00E7` means something else is going on, and a balloon
    /// with no text is a worse answer than none.
    ///
    /// This is a **local echo, not a broadcast**: it goes back to the one connection that
    /// spoke. There is nobody else on the field to send it to yet - the server has no
    /// concept of a second player in a field - and saying so here is cheaper than
    /// rediscovering it when there is.
    pub(super) fn say_out_loud(&mut self, text: &str) -> Vec<Reply> {
        if text.is_empty() {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        vec![Reply {
            opcode: net::userchat::USER_CHAT,
            body: net::userchat::user_chat(chr.id, text),
            what: format!("UserChat: {} ({}) says {:?}", chr.id, chr.name, text),
        }]
    }
}
