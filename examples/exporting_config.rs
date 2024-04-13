use bevy::prelude::*;
use bevy::reflect::ReflectMut;
use qevy::auto_create_config::register_types::{
    entities::{QevyEntity, QevyRegisterSolidClass, ReflectQevyEntity},
    properties::{QevyAngles, QevyProperty, ReflectQevyProperty},
};
use qevy_derive::QevyEntity;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            qevy::MapAssetLoaderPlugin::default(),
            qevy::auto_create_config::AutoCreateConfigPlugin::default(),
        ))
        .register_qevy_entity::<Worldspawn>()
        .register_qevy_entity::<TestBaseClass>()
        .register_qevy_entity::<TestSolidClass>()
        .register_qevy_entity::<APointClass>()
        .register_type::<EnumTestFlag>()
        .register_type::<EnumTestChoices>()
        .run();
}

/// World Entity
#[derive(Reflect, QevyEntity, Default)]
#[reflect(QevyEntity, Default)]
#[qevy_entity(entity_type = "Solid", entity_name = "worldspawn")]
struct Worldspawn;

/// This is a simple testing class, showcasing the different property types.
#[derive(Reflect, QevyEntity)]
#[reflect(QevyEntity, Default)]
#[qevy_entity(entity_type = "Point", model = ("test", None, None, None), size = (8, 8, 8, 8, 8, 8), color = (100, 255, 50))]
struct APointClass {
    /// This is a String property!
    test_string: String,
    /// This is a usize property!
    test_usize: usize,
    /// This is a bool property!
    test_bool: bool,
    /// This is a f32 property!
    test_f32: f32,
    test_f64: f64,
    test_i32: i32,
    test_i64: i64,
    test_u32: u32,
    test_u64: u64,
    /// This is a flag property!
    test_flag: EnumTestFlag,
    /// This is a choices property!
    test_choices: EnumTestChoices,
    /// This is a color property!
    test_color: Color,
    /// These are the angles of the entity!
    angles: QevyAngles,
    /// this is a base class, and won't be included in the fgd as a property, but as a base class!
    #[qevy_entity(is_base_class = true)]
    test_base_class: TestBaseClass,
}

impl Default for APointClass {
    fn default() -> Self {
        Self {
            test_string: "HELLO WORLD!".to_string(),
            test_usize: 69,
            test_bool: true,
            test_f32: 69.420,
            test_f64: 420.69,
            test_i32: i32::default(),
            test_i64: i64::default(),
            test_u32: u32::default(),
            test_u64: u64::default(),
            test_flag: EnumTestFlag::EnumVariantTest,
            test_choices: EnumTestChoices::EnumVariantTest,
            test_base_class: TestBaseClass,
            test_color: Color::rgb(1.0, 0.5, 0.75), // some random color, idk
            angles: QevyAngles::default(),
        }
    }
}

#[derive(Reflect, Default, QevyProperty)]
#[reflect(QevyProperty, Default)]
#[qevy_property(property_type = "flags")]
enum EnumTestFlag {
    #[default]
    #[qevy_property(selected_by_default = true)]
    Test,
    #[qevy_property(selected_by_default = false)]
    EnumVariantTest,
    TestTestTest,
}

#[derive(Reflect, Default, QevyProperty)]
#[reflect(QevyProperty, Default)]
#[qevy_property(property_type = "choices")]
enum EnumTestChoices {
    #[default]
    #[qevy_property(selected_by_default = true)]
    Test,
    #[qevy_property(key_override = "VariantTest!!!!")]
    EnumVariantTest,
    AnotherEnumVariant,
}

#[derive(Reflect, Default, QevyEntity)]
#[reflect(QevyEntity, Default)]
#[qevy_entity(entity_type = "Solid")]
struct TestSolidClass;

#[derive(Reflect, Default, QevyEntity)]
#[reflect(QevyEntity, Default)]
#[qevy_entity(entity_type = "Base")]
struct TestBaseClass;
