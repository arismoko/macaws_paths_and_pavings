//! The rules of the original's three block classes, over plain values so `cargo test` checks
//! them without a game.

/// `FacingPathBlock.getStateForPlacement`: the facing is `getHorizontalDirection().getClockWise()`.
/// The horizontal direction is `Direction.fromYRot(yaw)`.
pub fn placed_facing(yaw: f32) -> &'static str {
    // 2D data values: south 0, west 1, north 2, east 3.
    let looking = ((f64::from(yaw) / 90.0 + 0.5).floor() as i64).rem_euclid(4);
    match looking {
        0 => "west",
        1 => "north",
        2 => "east",
        _ => "south",
    }
}

/// `state.cycle(property)` on a boolean: the state string with `property` flipped, or `None`
/// when the state has no such boolean.
pub fn cycled(state: &str, property: &str) -> Option<String> {
    let (block, props) = state.strip_suffix(']')?.split_once('[')?;
    let mut found = false;
    let flipped: Vec<String> = props
        .split(',')
        .map(|pair| match pair.split_once('=') {
            Some((key, "true")) if key == property => {
                found = true;
                format!("{key}=false")
            }
            Some((key, "false")) if key == property => {
                found = true;
                format!("{key}=true")
            }
            _ => pair.to_owned(),
        })
        .collect();
    found.then(|| format!("{block}[{}]", flipped.join(",")))
}

/// What a tool does to a block it is used on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    /// `EngravedBlock`: a pickaxe.
    Pickaxe,
    /// `FlattenedBlock`: a shovel.
    Shovel,
}

impl Tool {
    /// The block-state property it cycles.
    pub fn property(self) -> &'static str {
        match self {
            Tool::Pickaxe => "engraved",
            Tool::Shovel => "flattened",
        }
    }

    /// The item tag a stack must be in (`ItemTags.PICKAXES`, `ItemTags.SHOVELS`).
    pub fn tag(self) -> &'static str {
        match self {
            Tool::Pickaxe => "minecraft:pickaxes",
            Tool::Shovel => "minecraft:shovels",
        }
    }

    /// `SoundEvents.UI_STONECUTTER_TAKE_RESULT`, `SoundEvents.SHOVEL_FLATTEN`.
    pub fn sound(self) -> &'static str {
        match self {
            Tool::Pickaxe => "minecraft:ui.stonecutter.take_result",
            Tool::Shovel => "minecraft:item.shovel.flatten",
        }
    }
}

/// `level.getRandom().nextFloat() * 0.1F + 0.9F` from a unit sample.
pub fn pitch(unit: f32) -> f32 {
    unit.clamp(0.0, 1.0) * 0.1 + 0.9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facing_is_a_quarter_clockwise_of_where_the_placer_looks() {
        // Yaw 0 looks south, 90 west, 180 north, 270 (or -90) east.
        assert_eq!(placed_facing(0.0), "west");
        assert_eq!(placed_facing(90.0), "north");
        assert_eq!(placed_facing(180.0), "east");
        assert_eq!(placed_facing(-90.0), "south");
        assert_eq!(placed_facing(44.0), "west");
        assert_eq!(placed_facing(46.0), "north");
        assert_eq!(placed_facing(-360.0), "west");
    }

    #[test]
    fn cycling_flips_only_its_property() {
        assert_eq!(
            cycled("mcwpaths:stone_flagstone[engraved=false]", "engraved").as_deref(),
            Some("mcwpaths:stone_flagstone[engraved=true]")
        );
        assert_eq!(
            cycled("mcwpaths:dirt_path_block[flattened=true]", "flattened").as_deref(),
            Some("mcwpaths:dirt_path_block[flattened=false]")
        );
        assert_eq!(
            cycled("mcwpaths:oak_planks_path[facing=east]", "engraved"),
            None
        );
        assert_eq!(cycled("mcwpaths:oak_planks_path", "engraved"), None);
    }

    #[test]
    fn pitch_is_between_nine_tenths_and_one() {
        assert_eq!(pitch(0.0), 0.9);
        assert!((pitch(1.0) - 1.0).abs() < 1e-6);
    }
}
