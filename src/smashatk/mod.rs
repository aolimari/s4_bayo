use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*
};

    // Down smash (uses wicked weave leg so could crash shit idk :shrug:)


#[acmd_script( agent = "bayonetta", script = "game_wickedweave_attacklw4", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn bayowickedweave_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_show") as i64);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 260, 90, 0, 10, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 270, 96, 0, 10, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        WorkModule::on_flag(agent.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 72, 90, 0, 10, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 270, 80, 0, 10, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
}

    // !! INSTALL SCRIPTS GO HERE OR IT WON'T BUILD! !! (dumbass)

pub fn install() {
    smashline::install_acmd_scripts!(
        bayowickedweave_attacklw4
    );
}

