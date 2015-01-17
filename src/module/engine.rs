#[cfg(feature = "client")]
use piston::graphics::Context;
#[cfg(feature = "client")]
use opengl_graphics::Gl;

use battle_state::BattleContext;
use assets::{ENGINE_TEXTURE, PROPULSION_TEXTURE};
use module::{IModule, Module, ModuleBase, ModuleRef};
use net::{InPacket, OutPacket};
use ship::{ShipRef, ShipState};
use sim::SimEventAdder;
use vec::{Vec2, Vec2f};

#[cfg(feature = "client")]
use sim::{SimVisuals, SimVisual};
#[cfg(feature = "client")]
use sprite_sheet::{SpriteSheet, SpriteAnimation};
#[cfg(feature = "client")]
use asset_store::AssetStore;

#[derive(RustcEncodable, RustcDecodable)]
pub struct EngineModule {
    pub base: ModuleBase,
}

impl EngineModule {
    pub fn new() -> Module {
        Module::Engine(EngineModule {
            base: ModuleBase::new(2, 1, 2, 2, 3),
        })
    }
}

impl IModule for EngineModule {
    fn server_preprocess(&mut self, ship_state: &mut ShipState) {
    }
    
    fn before_simulation(&mut self, ship_state: &mut ShipState, events: &mut SimEventAdder) {
    }
    
    #[cfg(feature = "client")]
    fn add_plan_visuals(&self, asset_store: &AssetStore, visuals: &mut SimVisuals, ship: &ShipRef) {
        let mut engine_sprite = SpriteSheet::new(asset_store.get_sprite_info(ENGINE_TEXTURE));
        engine_sprite.add_animation(SpriteAnimation::Stay(0.0, 5.0, 0));
    
        visuals.add(ship.borrow().id, 0, box SpriteVisual {
            position: self.base.get_render_position().clone(),
            sprite_sheet: engine_sprite,
        });
        
        // Propulsion sprite
        if self.base.is_active() {
            let mut prop_sprite = SpriteSheet::new(asset_store.get_sprite_info(PROPULSION_TEXTURE));
            prop_sprite.add_animation(SpriteAnimation::Loop(0.0, 5.0, 0, 7, 0.05));
        
            visuals.add(ship.borrow().id, 0, box SpriteVisual {
                position: self.base.get_render_position().clone() + Vec2{x: -48.0, y: 2.0},
                sprite_sheet: prop_sprite,
            });
        }
    }
    
    #[cfg(feature = "client")]
    fn add_simulation_visuals(&self, asset_store: &AssetStore, visuals: &mut SimVisuals, ship: &ShipRef) {
        self.add_plan_visuals(asset_store, visuals, ship);
    }
    
    fn after_simulation(&mut self, ship_state: &mut ShipState) {
    }

    fn write_plans(&self, packet: &mut OutPacket) {
    }
    
    fn read_plans(&mut self, context: &BattleContext, packet: &mut InPacket) {
    }
    
    fn write_results(&self, packet: &mut OutPacket) {
    }
    
    fn read_results(&mut self, packet: &mut InPacket) {
    }
    
    fn on_activated(&mut self, ship_state: &mut ShipState, modules: &Vec<ModuleRef>) {
        ship_state.thrust += 1;
    }
    
    fn on_deactivated(&mut self, ship_state: &mut ShipState, modules: &Vec<ModuleRef>) {
        ship_state.thrust -= 1;
    }
    
    fn on_icon_clicked(&mut self) -> bool {
        false
    }
    
    fn on_module_clicked(&mut self, ship: &ShipRef, module: &ModuleRef) -> bool {
        false
    }
}

// Sprite sheet sim visual
#[cfg(feature = "client")]
pub struct SpriteVisual {
    position: Vec2f,
    sprite_sheet: SpriteSheet,
}

#[cfg(feature = "client")]
impl SimVisual for SpriteVisual {
    fn draw(&mut self, context: &Context, gl: &mut Gl, time: f64) {
        self.sprite_sheet.draw(context, gl, self.position.x, self.position.y, 0.0, time);
    }
}