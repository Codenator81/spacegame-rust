#[cfg(feature = "client")]
use graphics::Context;
#[cfg(feature = "client")]
use opengl_graphics::Gl;

use battle_state::BattleContext;
use module;
use module::{IModule, Module, ModuleBase, ModuleRef};
use net::{InPacket, OutPacket};
use ship::{ShipRef, ShipState};
use sim::SimEventAdder;
use vec::{Vec2, Vec2f};

#[cfg(feature = "client")]
use sim_visuals::SpriteVisual;
#[cfg(feature = "client")]
use sim::{SimEffects, SimVisual};
#[cfg(feature = "client")]
use sprite_sheet::{SpriteSheet, SpriteAnimation};
#[cfg(feature = "client")]
use asset_store::AssetStore;

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct EngineModule;

impl EngineModule {
    pub fn new() -> Module<EngineModule> {
        Module {
            base: ModuleBase::new(2, 1, 2, 2, 3),
            module: EngineModule,
        }
    }
}

impl IModule for EngineModule {
    fn server_preprocess(&mut self, base: &mut ModuleBase, ship_state: &mut ShipState) {
    }
    
    fn before_simulation(&mut self, base: &mut ModuleBase, ship: &ShipRef, events: &mut SimEventAdder) {
    }
    
    #[cfg(feature = "client")]
    fn add_plan_effects(&self, base: &ModuleBase, asset_store: &AssetStore, effects: &mut SimEffects, ship: &ShipRef) {
        let mut engine_sprite = SpriteSheet::new(asset_store.get_sprite_info_str("modules/engine1.png"));
        engine_sprite.add_animation(SpriteAnimation::Stay(0.0, 7.0, 0));
    
        effects.add_visual(ship.borrow().id, 0, box SpriteVisual {
            position: base.get_render_position().clone(),
            sprite_sheet: engine_sprite,
        });
        
        // Propulsion sprite
        if base.is_active() {
            let mut prop_sprite = SpriteSheet::new(asset_store.get_sprite_info_str("effects/propulsion_sprite.png"));
            prop_sprite.add_animation(SpriteAnimation::Loop(0.0, 7.0, 0, 7, 0.05));
        
            effects.add_visual(ship.borrow().id, 0, box SpriteVisual {
                position: base.get_render_position().clone() + Vec2{x: -48.0, y: 2.0},
                sprite_sheet: prop_sprite,
            });
        }
    }
    
    #[cfg(feature = "client")]
    fn add_simulation_effects(&self, base: &ModuleBase, asset_store: &AssetStore, effects: &mut SimEffects, ship: &ShipRef) {
        self.add_plan_effects(base, asset_store, effects, ship);
    }
    
    fn after_simulation(&mut self, base: &mut ModuleBase, ship_state: &mut ShipState) {
    }
    
    fn on_activated(&mut self, base: &mut ModuleBase, ship_state: &mut ShipState, modules: &Vec<ModuleRef>) {
        ship_state.thrust += 1;
    }
    
    fn on_deactivated(&mut self, base: &mut ModuleBase, ship_state: &mut ShipState, modules: &Vec<ModuleRef>) {
        ship_state.thrust -= 1;
    }
    
    fn get_target_mode(&self, base: &ModuleBase) -> Option<module::TargetMode> {
        None
    }
}
