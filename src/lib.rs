//! Macaw's Paths and Pavings for Sunrise: the blocks are content in `data/` and `assets/`;
//! this code is what the original's block and item classes do.
//!
//! | Moment | Where | What it does |
//! |---|---|---|
//! | `register` | [`declare`] | attaches the parts to the blocks and items |
//! | `use_item_on` | [`Use`] | a pickaxe engraves a paving, a shovel flattens soil |
//! | `state_for_placement` | [`Facing`] | a facing path is placed a quarter clockwise from the placer |
//! | `tooltip` | [`Tip`] | the engraving and flattening instructions |

pub mod names;
pub mod rules;

use mod_api::{
    self, BlockPart, BlockUpdateFlags, ClientHost, Direction, EquipmentSlot, Hand, Hooks, HostSide,
    InteractionResult, ItemPart, LevelSide, LivingRef, PlacementSource, Registrar,
    RegistrationContext, SoundSource, Text, TooltipEvent, UseItemOnEvent,
};

use rules::Tool;

const MOD: &str = "mcwpaths";
/// `ChatFormatting.GRAY`, `rgba` packed.
const GRAY: u32 = 0xAAAA_AAFF;

/// `EngravedBlock.useItemOn` and `FlattenedBlock.useItemOn`.
pub struct Use(pub Tool);

#[mod_api::sunrise_part]
impl BlockPart for Use {
    fn use_item_on(
        &self,
        host: HostSide<'_>,
        event: &mut UseItemOnEvent<'_>,
    ) -> Option<InteractionResult> {
        // The client predicts nothing of its own: the template's answer stands there.
        let HostSide::Server(host) = host else {
            return None;
        };
        let click = event.click;
        let stack = click.stack?;
        let wanted = self.0.tag();
        let held = host
            .tags()
            .item_tags(stack.item())?
            .iter()
            .any(|tag| tag.to_string() == wanted);
        if !held {
            return Some(InteractionResult::Pass);
        }
        let next = rules::cycled(&host.block_state_string(event.state)?, self.0.property())?;
        let next = host.block_state_id(&next)?;
        let level = host.level(click.dimension.key().id())?;
        let [x, y, z] = click.pos;
        // `level.getRandom().nextFloat()`, drawn from the level's own stream.
        let unit = level
            .with_random(host, |random| random.next_float())
            .unwrap_or(0.5);
        let _ = level.play_sound(
            host,
            self.0.sound(),
            [f64::from(x) + 0.5, f64::from(y) + 0.5, f64::from(z) + 0.5],
            SoundSource::Blocks,
            1.0,
            rules::pitch(unit),
            None,
        );
        let _ = level.set_block(
            host,
            click.pos,
            next,
            BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS,
        );
        let player = host.player(click.player)?;
        if !player.is_creative(&*host).unwrap_or(false) {
            let slot = match click.hand {
                Hand::Main => EquipmentSlot::MainHand,
                Hand::Off => EquipmentSlot::OffHand,
            };
            let _ = LivingRef::new(player.entity()).hurt_and_break(host, slot, 1);
        }
        Some(InteractionResult::Success)
    }
}

/// `FacingPathBlock.getStateForPlacement`.
pub struct Facing;

#[mod_api::sunrise_part]
impl BlockPart for Facing {
    fn state_for_placement(
        &self,
        level: LevelSide,
        event: &mut mod_api::StateForPlacementEvent<'_>,
    ) -> Option<Option<mod_api::BlockState>> {
        let (LevelSide::Server(level) | LevelSide::Client(level)) = level;
        let block = level.state_block(event.state)?;
        // `getHorizontalDirection`: the player's view, or a `DirectionalPlaceContext`'s
        // direction (north when it points up or down).
        let horizontal = match event.context.source {
            PlacementSource::Player { yaw, .. } => rules::looking(yaw),
            PlacementSource::Directional { direction } => match direction {
                Direction::South => "south",
                Direction::West => "west",
                Direction::East => "east",
                _ => "north",
            },
            _ => "north",
        };
        let facing = rules::placed_facing(horizontal);
        Some(level.block_state_id(&format!("{block}[facing={facing}]")))
    }
}

/// The instruction line on an engraving or flattening block's item
/// (`EngravedBlockTooltip`, `FlattenedBlockTooltip`).
pub struct Tip(&'static str);

#[mod_api::sunrise_part]
impl ItemPart for Tip {
    fn tooltip(&self, _host: &dyn ClientHost, event: &mut TooltipEvent<'_, '_>) {
        let _ = event
            .lines
            .push((Text::translate(self.0, Vec::new()), GRAY));
    }
}

/// Attach the parts to the blocks and items the package declares.
pub fn declare(r: &mut dyn Registrar) {
    for (names, tool, key) in [
        (
            &names::ENGRAVED[..],
            Tool::Pickaxe,
            "mcwpaths.engraved.desc",
        ),
        (
            &names::FLATTENED[..],
            Tool::Shovel,
            "mcwpaths.flattened.desc",
        ),
    ] {
        for name in names {
            let block = r.block_key(&format!("{MOD}:{name}"));
            r.block_part(block, Use(tool));
            let item = r.item_key(&format!("{MOD}:{name}"));
            r.item_part(item, Tip(key));
        }
    }
    for name in names::FACING {
        let block = r.block_key(&format!("{MOD}:{name}"));
        r.block_part(block, Facing);
    }
}

#[derive(Default)]
struct Mod;

#[mod_api::sunrise_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        declare(r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn declaration(name: &str) -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("data/mcwpaths/sunrise/block/{name}.json"));
        std::fs::read_to_string(path).unwrap_or_default()
    }

    #[test]
    fn every_listed_block_has_the_state_its_rule_needs() {
        for name in names::ENGRAVED {
            assert!(declaration(name).contains("\"engraved\""), "{name}");
        }
        for name in names::FLATTENED {
            assert!(declaration(name).contains("\"flattened\""), "{name}");
        }
        for name in names::FACING {
            assert!(
                declaration(name).contains("minecraft:carved_pumpkin"),
                "{name}"
            );
        }
    }

    #[test]
    fn pale_oak_and_poplar_are_among_the_facing_paths() {
        assert!(names::FACING.contains(&"pale_oak_planks_path"));
        assert!(names::FACING.contains(&"poplar_planks_path"));
        assert_eq!(names::FACING.len(), 26);
    }
}
