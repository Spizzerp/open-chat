use crate::utils::{build_ic_agent, build_identity, build_management_canister, delay, get_canister_wasm, CanisterWasmName};
use crate::{CanisterIds, TestIdentity};
use candid::{CandidType, Principal};
use ic_agent::identity::BasicIdentity;
use ic_agent::Identity;
use ic_utils::interfaces::ManagementCanister;
use ic_utils::Canister;
use std::path::Path;
use types::CanisterId;

pub async fn create_and_install_service_canisters(url: String) -> CanisterIds {
    let identity = build_identity(TestIdentity::Controller);
    let principal = identity.sender().unwrap();
    let agent = build_ic_agent(url, identity).await;
    let management_canister = build_management_canister(&agent);

    let (user_index_canister_id, group_index_canister_id, notifications_canister_id) = futures::future::join3(
        create_empty_canister(&management_canister),
        create_empty_canister(&management_canister),
        create_empty_canister(&management_canister),
    )
    .await;

    println!("user_index canister id: {}", user_index_canister_id.to_string());
    println!("group_index canister id: {}", group_index_canister_id.to_string());
    println!("notifications canister id: {}", notifications_canister_id.to_string());

    let canister_ids = CanisterIds {
        user_index: user_index_canister_id,
        group_index: group_index_canister_id,
        notifications: notifications_canister_id,
    };

    install_service_canisters_impl(principal, &canister_ids, &management_canister).await;

    canister_ids
}

pub async fn install_service_canisters(url: String, controller: String, canister_ids: CanisterIds) {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let pem_file_path = home_dir.join(Path::new(&format!(".config/dfx/identity/{}/identity.pem", controller)));
    let identity = BasicIdentity::from_pem_file(pem_file_path).expect("Failed to create identity");
    let principal = identity.sender().unwrap();
    let agent = build_ic_agent(url, identity).await;
    let management_canister = build_management_canister(&agent);

    install_service_canisters_impl(principal, &canister_ids, &management_canister).await;
}

async fn install_service_canisters_impl(
    principal: Principal,
    canister_ids: &CanisterIds,
    management_canister: &Canister<'_, ManagementCanister>,
) {
    let user_index_canister_wasm = get_canister_wasm(CanisterWasmName::UserIndex);
    let user_canister_wasm = get_canister_wasm(CanisterWasmName::User);
    let user_index_init_args = user_index_canister::init::Args {
        service_principals: vec![principal],
        sms_service_principals: Vec::new(),
        user_canister_wasm,
        group_index_canister_id: canister_ids.group_index,
        notifications_canister_id: canister_ids.notifications,
        test_mode: true,
    };

    let group_index_canister_wasm = get_canister_wasm(CanisterWasmName::GroupIndex);
    let group_canister_wasm = get_canister_wasm(CanisterWasmName::Group);
    let group_index_init_args = group_index_canister::init::Args {
        group_canister_wasm,
        notifications_canister_id: canister_ids.notifications,
    };

    let notifications_canister_wasm = get_canister_wasm(CanisterWasmName::Notifications);
    let notifications_init_args = notifications_canister::init::Args {
        push_service_principals: Vec::new(),
    };

    futures::future::join3(
        install_wasm(
            management_canister,
            &canister_ids.user_index,
            &user_index_canister_wasm.module,
            user_index_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.group_index,
            &group_index_canister_wasm.module,
            group_index_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.notifications,
            &notifications_canister_wasm.module,
            notifications_init_args,
        ),
    )
    .await;

    println!("Canister wasms installed");
}

async fn create_empty_canister(management_canister: &Canister<'_, ManagementCanister>) -> CanisterId {
    let (canister_id,) = management_canister
        .create_canister()
        .as_provisional_create_with_amount(None)
        .call_and_wait(delay())
        .await
        .expect("Failed to create canister");

    canister_id
}

async fn install_wasm<A: CandidType + Sync + Send>(
    management_canister: &Canister<'_, ManagementCanister>,
    canister_id: &CanisterId,
    wasm_bytes: &[u8],
    init_args: A,
) {
    management_canister
        .install_code(canister_id, wasm_bytes)
        .with_arg(init_args)
        .call_and_wait(delay())
        .await
        .expect("Failed to install wasm");
}
