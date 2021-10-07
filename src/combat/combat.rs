use crate::clasher::clasher::Clasher;
use wasm_bindgen::prelude::*;

pub struct Combat {
    hero: Clasher,
    enemy: Clasher,
    pub power_gained: usize,
    pub dexterity_gained: usize,
    pub agility_gained: usize,
    pub resitency_gained: usize,
    pub hp_gained: usize,
    pub en_gained: usize,
}

pub struct CombatSate<S> {
    sate: S,
    combat: Combat,
}

impl<S> CombatSate<S> {
    pub fn new(sate: S, combat: Combat) -> Self {
        Self { sate, combat }
    }
}

pub fn start_combat(player: Clasher) -> CombatSate<BattleStart> {
    CombatSate::new(
        BattleStart {},
        Combat {
            hero: player.clone(),
            enemy: Clasher::generate_random_enemy(),
            power_gained: 0,
            dexterity_gained: 0,
            agility_gained: 0,
            resitency_gained: 0,
            hp_gained: 0,
            en_gained: 0,
        },
    )
}

pub struct BattleStart {}
pub struct TurnStart {}
pub struct PlayerChoosesAttack {}
pub struct PlayerChoosesDefense {}
pub struct PlayerSecondAction {}
pub struct SystemApplyDamagePlayer {}
pub struct MonsterChoosesAttack {}
pub struct MonsterChoosesDefense {}
pub struct MonsterSecondAction {}
pub struct SystemApplyDamageMonster {}
pub struct EndCombat {}

impl Into<CombatSate<TurnStart>> for BattleStart {
    fn into(self) -> CombatSate<TurnStart> {
        todo!()
    }
}
impl Into<CombatSate<PlayerChoosesAttack>> for TurnStart {
    fn into(self) -> CombatSate<PlayerChoosesAttack> {
        todo!()
    }
}
impl Into<CombatSate<MonsterChoosesAttack>> for TurnStart {
    fn into(self) -> CombatSate<MonsterChoosesAttack> {
        todo!()
    }
}
impl Into<CombatSate<MonsterChoosesDefense>> for PlayerChoosesAttack {
    fn into(self) -> CombatSate<MonsterChoosesDefense> {
        todo!()
    }
}
impl Into<CombatSate<SystemApplyDamagePlayer>> for PlayerChoosesDefense {
    fn into(self) -> CombatSate<SystemApplyDamagePlayer> {
        todo!()
    }
}
impl Into<CombatSate<PlayerChoosesAttack>> for PlayerSecondAction {
    fn into(self) -> CombatSate<PlayerChoosesAttack> {
        todo!()
    }
}
impl Into<CombatSate<EndCombat>> for SystemApplyDamagePlayer {
    fn into(self) -> CombatSate<EndCombat> {
        todo!()
    }
}
impl Into<CombatSate<PlayerSecondAction>> for SystemApplyDamagePlayer {
    fn into(self) -> CombatSate<PlayerSecondAction> {
        todo!()
    }
}
impl Into<CombatSate<TurnStart>> for SystemApplyDamagePlayer {
    fn into(self) -> CombatSate<TurnStart> {
        todo!()
    }
}
impl Into<CombatSate<PlayerChoosesDefense>> for MonsterChoosesAttack {
    fn into(self) -> CombatSate<PlayerChoosesDefense> {
        todo!()
    }
}
impl Into<CombatSate<MonsterChoosesAttack>> for MonsterSecondAction {
    fn into(self) -> CombatSate<MonsterChoosesAttack> {
        todo!()
    }
}
impl Into<CombatSate<TurnStart>> for SystemApplyDamageMonster {
    fn into(self) -> CombatSate<TurnStart> {
        todo!()
    }
}
impl Into<CombatSate<EndCombat>> for SystemApplyDamageMonster {
    fn into(self) -> CombatSate<EndCombat> {
        todo!()
    }
}
impl Into<CombatSate<PlayerSecondAction>> for SystemApplyDamageMonster {
    fn into(self) -> CombatSate<PlayerSecondAction> {
        todo!()
    }
}

impl From<CombatSate<BattleStart>> for CombatSate<TurnStart> {
    fn from(_val: CombatSate<BattleStart>) -> Self {
        todo!()
    }
}

impl From<CombatSate<TurnStart>> for CombatSate<PlayerChoosesAttack> {
    fn from(_val: CombatSate<TurnStart>) -> Self {
        todo!()
    }
}

impl From<CombatSate<PlayerChoosesAttack>> for CombatSate<MonsterChoosesDefense> {
    fn from(_val: CombatSate<PlayerChoosesAttack>) -> Self {
        todo!()
    }
}

impl From<CombatSate<MonsterChoosesDefense>> for CombatSate<SystemApplyDamageMonster> {
    fn from(_val: CombatSate<MonsterChoosesDefense>) -> Self {
        todo!()
    }
}

impl From<CombatSate<MonsterChoosesAttack>> for CombatSate<PlayerChoosesDefense> {
    fn from(_val: CombatSate<MonsterChoosesAttack>) -> Self {
        todo!()
    }
}

impl From<CombatSate<PlayerChoosesDefense>> for CombatSate<SystemApplyDamagePlayer> {
    fn from(_val: CombatSate<PlayerChoosesDefense>) -> Self {
        todo!()
    }
}

impl From<CombatSate<SystemApplyDamagePlayer>> for CombatSate<EndCombat> {
    fn from(_val: CombatSate<SystemApplyDamagePlayer>) -> Self {
        todo!()
    }
}

impl From<CombatSate<SystemApplyDamagePlayer>> for CombatSate<PlayerSecondAction> {
    fn from(_val: CombatSate<SystemApplyDamagePlayer>) -> Self {
        todo!()
    }
}

impl From<CombatSate<SystemApplyDamagePlayer>> for CombatSate<TurnStart> {
    fn from(_val: CombatSate<SystemApplyDamagePlayer>) -> Self {
        todo!()
    }
}

impl From<CombatSate<SystemApplyDamageMonster>> for CombatSate<EndCombat> {
    fn from(_val: CombatSate<SystemApplyDamageMonster>) -> Self {
        todo!()
    }
}

impl From<CombatSate<SystemApplyDamageMonster>> for CombatSate<MonsterSecondAction> {
    fn from(_val: CombatSate<SystemApplyDamageMonster>) -> Self {
        todo!()
    }
}

impl From<CombatSate<SystemApplyDamageMonster>> for CombatSate<TurnStart> {
    fn from(_val: CombatSate<SystemApplyDamageMonster>) -> Self {
        todo!()
    }
}
