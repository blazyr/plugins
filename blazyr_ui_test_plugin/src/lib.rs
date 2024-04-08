use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    std_types::{
        RBoxError,
        ROption::{self, RNone, RSome},
        RResult::{self, RErr, ROk},
        RStr, RString, RVec,
    },
};
use blazyr_extension::{
    ui::{RComponent, RComponentClickable},
    Plugin, Plugin_Ref, REntity, REntityActionResponse,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect,
    #[error("unknown data store error")]
    Unknown,
}

#[export_root_module]
fn instantiate_root_module() -> Plugin_Ref {
    Plugin {
        entities,
        on_entity_action,
        on_mount,
        on_dispose,
        component_clickable,
    }
    .leak_into_prefix() // Converts the `MinMod` into `MinMod_Ref` and leaks it
}

#[sabi_extern_fn]
fn component_clickable(component: RComponentClickable, id: RString) -> RResult<(), RBoxError> {
    println!("{:?} {}", component, id);
    ROk(())
}

#[sabi_extern_fn]
pub fn entities() -> RResult<RVec<REntity>, RBoxError> {
    let entity = REntity::builder(0, "UI TEST")
        .description("TEST YOUR UI")
        .build();
    ROk(vec![entity].into())
}

#[sabi_extern_fn]
pub fn on_mount() -> RResult<(), RBoxError> {
    ROk(())
}

#[sabi_extern_fn]
pub fn on_dispose() -> RResult<(), RBoxError> {
    RErr(RBoxError::new(DataStoreError::Unknown))
}

#[sabi_extern_fn]
pub fn on_entity_action(_id: u64, _: ROption<RStr>) -> RResult<REntityActionResponse, RBoxError> {
    RResult::ROk(REntityActionResponse::Ui(RComponent::Column {
        children: RSome(
            vec![
                blazyr_extension::ui::RComponent::Container {
                    child: RNone,
                    on_click: RNone,
                },
                blazyr_extension::ui::RComponent::Container {
                    child: RNone,
                    on_click: RNone,
                },
                blazyr_extension::ui::RComponent::Container {
                    child: RNone,
                    on_click: RNone,
                },
            ]
            .into(),
        ),
    }))
}
