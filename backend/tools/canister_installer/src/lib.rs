use candid::Principal;
use canister_agent_utils::{build_ic_agent, get_canister_wasm, install_wasm, set_controllers, CanisterIds, CanisterName};
use ic_agent::{Agent, Identity};
use ic_utils::interfaces::ManagementCanister;
use types::{BuildVersion, CanisterWasm, Cycles};
use utils::consts::{SNS_GOVERNANCE_CANISTER_ID, SNS_LEDGER_CANISTER_ID};

const T: Cycles = 1_000_000_000_000;

pub async fn install_service_canisters(identity: Box<dyn Identity>, url: String, canister_ids: CanisterIds, test_mode: bool) {
    let principal = identity.sender().unwrap();
    let agent = build_ic_agent(url, identity).await;
    let management_canister = ManagementCanister::create(&agent);

    install_service_canisters_impl(principal, &canister_ids, &agent, &management_canister, test_mode).await;
}

async fn install_service_canisters_impl(
    principal: Principal,
    canister_ids: &CanisterIds,
    agent: &Agent,
    management_canister: &ManagementCanister<'_>,
    test_mode: bool,
) {
    let controllers = vec![principal];
    let video_call_operators =
        vec![Principal::from_text("wp3oc-ig6b4-6xvef-yoj27-qt3kw-u2xmp-qbvuv-2grco-s2ndy-wv3ud-7qe").unwrap()];

    futures::future::join_all(vec![
        set_controllers(management_canister, &canister_ids.user_index, controllers.clone()),
        set_controllers(management_canister, &canister_ids.group_index, controllers.clone()),
        set_controllers(management_canister, &canister_ids.notifications_index, controllers.clone()),
        set_controllers(management_canister, &canister_ids.identity, controllers.clone()),
        set_controllers(management_canister, &canister_ids.online_users, controllers.clone()),
        set_controllers(management_canister, &canister_ids.proposals_bot, controllers.clone()),
        set_controllers(management_canister, &canister_ids.storage_index, controllers.clone()),
        set_controllers(management_canister, &canister_ids.cycles_dispenser, controllers.clone()),
        set_controllers(management_canister, &canister_ids.registry, controllers.clone()),
        set_controllers(management_canister, &canister_ids.market_maker, controllers.clone()),
        set_controllers(management_canister, &canister_ids.neuron_controller, controllers.clone()),
        set_controllers(management_canister, &canister_ids.escrow, controllers.clone()),
        set_controllers(management_canister, &canister_ids.translations, controllers.clone()),
        set_controllers(management_canister, &canister_ids.event_relay, controllers.clone()),
        set_controllers(management_canister, &canister_ids.event_store, controllers.clone()),
        set_controllers(management_canister, &canister_ids.sign_in_with_email, controllers.clone()),
        set_controllers(management_canister, &canister_ids.sign_in_with_ethereum, controllers.clone()),
        set_controllers(management_canister, &canister_ids.sign_in_with_solana, controllers.clone()),
        set_controllers(
            management_canister,
            &canister_ids.local_user_index,
            vec![canister_ids.user_index],
        ),
        set_controllers(
            management_canister,
            &canister_ids.local_group_index,
            vec![canister_ids.group_index],
        ),
        set_controllers(
            management_canister,
            &canister_ids.notifications,
            vec![canister_ids.notifications_index],
        ),
    ])
    .await;

    let version = BuildVersion::min();

    let user_index_canister_wasm = get_canister_wasm(CanisterName::UserIndex, version);
    let user_index_init_args = user_index_canister::init::Args {
        governance_principals: vec![principal],
        user_canister_wasm: CanisterWasm::default(),
        local_user_index_canister_wasm: CanisterWasm::default(),
        group_index_canister_id: canister_ids.group_index,
        notifications_index_canister_id: canister_ids.notifications_index,
        identity_canister_id: canister_ids.identity,
        proposals_bot_canister_id: canister_ids.proposals_bot,
        storage_index_canister_id: canister_ids.storage_index,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        escrow_canister_id: canister_ids.escrow,
        event_relay_canister_id: canister_ids.event_relay,
        nns_governance_canister_id: canister_ids.nns_governance,
        internet_identity_canister_id: canister_ids.nns_internet_identity,
        translations_canister_id: canister_ids.translations,
        video_call_operators: video_call_operators.clone(),
        ic_root_key: agent.read_root_key(),
        wasm_version: version,
        test_mode,
    };

    let group_index_canister_wasm = get_canister_wasm(CanisterName::GroupIndex, version);
    let group_index_init_args = group_index_canister::init::Args {
        governance_principals: vec![principal],
        group_canister_wasm: CanisterWasm::default(),
        community_canister_wasm: CanisterWasm::default(),
        local_group_index_canister_wasm: CanisterWasm::default(),
        user_index_canister_id: canister_ids.user_index,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        proposals_bot_user_id: canister_ids.proposals_bot.into(),
        escrow_canister_id: canister_ids.escrow,
        event_relay_canister_id: canister_ids.event_relay,
        internet_identity_canister_id: canister_ids.nns_internet_identity,
        video_call_operators: video_call_operators.clone(),
        ic_root_key: agent.read_root_key(),
        wasm_version: version,
        test_mode,
    };

    let notifications_index_canister_wasm = get_canister_wasm(CanisterName::NotificationsIndex, version);
    let notifications_index_init_args = notifications_index_canister::init::Args {
        governance_principals: vec![principal],
        push_service_principals: vec![principal],
        user_index_canister_id: canister_ids.user_index,
        authorizers: vec![canister_ids.user_index, canister_ids.group_index],
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        notifications_canister_wasm: CanisterWasm::default(),
        wasm_version: version,
        test_mode,
    };

    let identity_canister_wasm = get_canister_wasm(CanisterName::Identity, version);
    let identity_init_args = identity_canister::init::Args {
        governance_principals: vec![principal],
        user_index_canister_id: canister_ids.user_index,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        skip_captcha_whitelist: vec![canister_ids.nns_internet_identity, canister_ids.sign_in_with_email],
        ic_root_key: agent.read_root_key(),
        wasm_version: version,
        test_mode,
    };

    let translations_canister_wasm = get_canister_wasm(CanisterName::Translations, version);
    let translations_init_args = translations_canister::init::Args {
        deployment_operators: vec![principal],
        user_index_canister_id: canister_ids.user_index,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        wasm_version: version,
        test_mode,
    };

    let online_users_canister_wasm = get_canister_wasm(CanisterName::OnlineUsers, version);
    let online_users_init_args = online_users_canister::init::Args {
        user_index_canister_id: canister_ids.user_index,
        event_relay_canister_id: canister_ids.event_relay,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        wasm_version: version,
        test_mode,
    };

    let proposals_bot_canister_wasm = get_canister_wasm(CanisterName::ProposalsBot, version);
    let proposals_bot_init_args = proposals_bot_canister::init::Args {
        service_owner_principals: vec![principal],
        user_index_canister_id: canister_ids.user_index,
        group_index_canister_id: canister_ids.group_index,
        registry_canister_id: canister_ids.registry,
        nns_governance_canister_id: canister_ids.nns_governance,
        sns_wasm_canister_id: canister_ids.nns_sns_wasm,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        wasm_version: version,
        test_mode,
    };

    let storage_index_canister_wasm = get_canister_wasm(CanisterName::StorageIndex, version);
    let storage_index_init_args = storage_index_canister::init::Args {
        governance_principals: vec![principal],
        user_controllers: vec![canister_ids.user_index, canister_ids.group_index],
        bucket_canister_wasm: get_canister_wasm(CanisterName::StorageBucket, version),
        cycles_dispenser_config: storage_index_canister::init::CyclesDispenserConfig {
            canister_id: canister_ids.cycles_dispenser,
            min_cycles_balance: 200 * T,
        },
        wasm_version: version,
        test_mode,
    };

    let cycles_dispenser_canister_wasm = get_canister_wasm(CanisterName::CyclesDispenser, version);
    let cycles_dispenser_init_args = cycles_dispenser_canister::init::Args {
        governance_principals: vec![principal],
        canisters: vec![
            canister_ids.user_index,
            canister_ids.group_index,
            canister_ids.notifications_index,
            canister_ids.local_user_index,
            canister_ids.local_group_index,
            canister_ids.notifications,
            canister_ids.online_users,
            canister_ids.proposals_bot,
            canister_ids.storage_index,
        ],
        max_top_up_amount: 20 * T,
        min_interval: 5 * 60 * 1000, // 5 minutes
        min_cycles_balance: 200 * T,
        icp_burn_amount_e8s: 1_000_000_000, // 10 ICP
        ledger_canister: canister_ids.nns_ledger,
        cycles_minting_canister: canister_ids.nns_cmc,
        wasm_version: version,
        test_mode,
    };

    let registry_canister_wasm = get_canister_wasm(CanisterName::Registry, version);
    let registry_init_args = registry_canister::init::Args {
        user_index_canister_id: canister_ids.user_index,
        governance_principals: vec![principal],
        proposals_bot_canister_id: canister_ids.proposals_bot,
        nns_ledger_canister_id: canister_ids.nns_ledger,
        nns_governance_canister_id: canister_ids.nns_governance,
        nns_root_canister_id: canister_ids.nns_root,
        sns_wasm_canister_id: canister_ids.nns_sns_wasm,
        nns_index_canister_id: canister_ids.nns_index,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        wasm_version: version,
        test_mode,
    };

    let market_maker_canister_wasm = get_canister_wasm(CanisterName::MarketMaker, version);
    let market_maker_init_args = market_maker_canister::init::Args {
        user_index_canister_id: canister_ids.user_index,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        icp_ledger_canister_id: canister_ids.nns_ledger,
        chat_ledger_canister_id: canister_ids.nns_ledger, // TODO This should be the CHAT ledger
        wasm_version: version,
        test_mode,
    };

    let neuron_controller_canister_wasm = get_canister_wasm(CanisterName::NeuronController, version);
    let neuron_controller_init_args = neuron_controller_canister::init::Args {
        governance_principals: vec![principal],
        nns_governance_canister_id: canister_ids.nns_governance,
        nns_ledger_canister_id: canister_ids.nns_ledger,
        cycles_minting_canister_id: canister_ids.nns_cmc,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        wasm_version: version,
        test_mode,
    };

    let escrow_canister_wasm = get_canister_wasm(CanisterName::Escrow, version);
    let escrow_init_args = escrow_canister::init::Args {
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        wasm_version: version,
        test_mode,
    };

    let event_relay_canister_wasm = get_canister_wasm(CanisterName::EventRelay, version);
    let event_relay_init_args = event_relay_canister::init::Args {
        push_events_whitelist: vec![
            canister_ids.user_index,
            canister_ids.online_users,
            canister_ids.local_user_index,
            canister_ids.local_group_index,
        ],
        event_store_canister_id: canister_ids.event_store,
        cycles_dispenser_canister_id: canister_ids.cycles_dispenser,
        chat_ledger_canister_id: SNS_LEDGER_CANISTER_ID,
        chat_governance_canister_id: SNS_GOVERNANCE_CANISTER_ID,
        wasm_version: version,
        test_mode,
    };

    let event_store_canister_wasm = get_canister_wasm(CanisterName::EventStore, version);
    let event_store_init_args = event_store_canister::InitArgs {
        push_events_whitelist: vec![canister_ids.event_relay],
        read_events_whitelist: vec![principal],
        time_granularity: None,
    };

    let sign_in_with_email_wasm = get_canister_wasm(CanisterName::SignInWithEmail, version);
    let sign_in_with_email_init_args = sign_in_with_email_canister_test_utils::default_init_args();

    let sign_in_with_ethereum_wasm = get_canister_wasm(CanisterName::SignInWithEthereum, version);
    let sign_in_with_ethereum_init_args = siwe::SettingsInput {
        domain: "oc.app".to_string(),
        uri: "https://oc.app".to_string(),
        salt: "OpenChat".to_string(),
        chain_id: None,
        scheme: None,
        statement: None,
        sign_in_expires_in: None,
        session_expires_in: None,
        targets: None,
        runtime_features: None,
    };

    let sign_in_with_solana_wasm = get_canister_wasm(CanisterName::SignInWithSolana, version);
    let sign_in_with_solana_init_args = siws::SettingsInput {
        domain: "oc.app".to_string(),
        uri: "https://oc.app".to_string(),
        salt: "OpenChat".to_string(),
        chain_id: None,
        scheme: None,
        statement: None,
        sign_in_expires_in: None,
        session_expires_in: None,
        targets: None,
        runtime_features: None,
    };

    futures::future::join5(
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
            &canister_ids.notifications_index,
            &notifications_index_canister_wasm.module,
            notifications_index_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.identity,
            &identity_canister_wasm.module,
            identity_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.online_users,
            &online_users_canister_wasm.module,
            online_users_init_args,
        ),
    )
    .await;

    futures::future::join5(
        install_wasm(
            management_canister,
            &canister_ids.proposals_bot,
            &proposals_bot_canister_wasm.module,
            proposals_bot_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.storage_index,
            &storage_index_canister_wasm.module,
            storage_index_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.cycles_dispenser,
            &cycles_dispenser_canister_wasm.module,
            cycles_dispenser_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.registry,
            &registry_canister_wasm.module,
            registry_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.market_maker,
            &market_maker_canister_wasm.module,
            market_maker_init_args,
        ),
    )
    .await;

    futures::future::join5(
        install_wasm(
            management_canister,
            &canister_ids.neuron_controller,
            &neuron_controller_canister_wasm.module,
            neuron_controller_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.escrow,
            &escrow_canister_wasm.module,
            escrow_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.translations,
            &translations_canister_wasm.module,
            translations_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.event_relay,
            &event_relay_canister_wasm.module,
            event_relay_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.event_store,
            &event_store_canister_wasm.module,
            event_store_init_args,
        ),
    )
    .await;

    futures::future::join3(
        install_wasm(
            management_canister,
            &canister_ids.sign_in_with_email,
            &sign_in_with_email_wasm.module,
            sign_in_with_email_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.sign_in_with_ethereum,
            &sign_in_with_ethereum_wasm.module,
            sign_in_with_ethereum_init_args,
        ),
        install_wasm(
            management_canister,
            &canister_ids.sign_in_with_solana,
            &sign_in_with_solana_wasm.module,
            sign_in_with_solana_init_args,
        ),
    )
    .await;

    let user_canister_wasm = get_canister_wasm(CanisterName::User, version);
    let group_canister_wasm = get_canister_wasm(CanisterName::Group, version);
    let community_canister_wasm = get_canister_wasm(CanisterName::Community, version);
    let local_group_index_canister_wasm = get_canister_wasm(CanisterName::LocalGroupIndex, version);
    let local_user_index_canister_wasm = get_canister_wasm(CanisterName::LocalUserIndex, version);
    let notifications_canister_wasm = get_canister_wasm(CanisterName::Notifications, version);

    futures::future::try_join(
        user_index_canister_client::upgrade_local_user_index_canister_wasm(
            agent,
            &canister_ids.user_index,
            &user_index_canister::upgrade_local_user_index_canister_wasm::Args {
                wasm: local_user_index_canister_wasm,
                filter: None,
                use_for_new_canisters: None,
            },
        ),
        group_index_canister_client::upgrade_local_group_index_canister_wasm(
            agent,
            &canister_ids.group_index,
            &group_index_canister::upgrade_local_group_index_canister_wasm::Args {
                wasm: local_group_index_canister_wasm,
                filter: None,
                use_for_new_canisters: None,
            },
        ),
    )
    .await
    .unwrap();

    futures::future::try_join4(
        user_index_canister_client::upgrade_user_canister_wasm(
            agent,
            &canister_ids.user_index,
            &user_index_canister::upgrade_user_canister_wasm::Args {
                wasm: user_canister_wasm,
                filter: None,
                use_for_new_canisters: None,
            },
        ),
        group_index_canister_client::upgrade_group_canister_wasm(
            agent,
            &canister_ids.group_index,
            &group_index_canister::upgrade_group_canister_wasm::Args {
                wasm: group_canister_wasm,
                filter: None,
                use_for_new_canisters: None,
            },
        ),
        group_index_canister_client::upgrade_community_canister_wasm(
            agent,
            &canister_ids.group_index,
            &group_index_canister::upgrade_community_canister_wasm::Args {
                wasm: community_canister_wasm,
                filter: None,
                use_for_new_canisters: None,
            },
        ),
        notifications_index_canister_client::upgrade_notifications_canister_wasm(
            agent,
            &canister_ids.notifications_index,
            &notifications_index_canister::upgrade_notifications_canister_wasm::Args {
                wasm: notifications_canister_wasm,
                filter: None,
                use_for_new_canisters: None,
            },
        ),
    )
    .await
    .unwrap();

    let add_local_group_index_canister_response = group_index_canister_client::add_local_group_index_canister(
        agent,
        &canister_ids.group_index,
        &group_index_canister::add_local_group_index_canister::Args {
            canister_id: canister_ids.local_group_index,
            local_user_index_canister_id: canister_ids.local_user_index,
            notifications_canister_id: canister_ids.notifications,
        },
    )
    .await
    .unwrap();

    if !matches!(
        add_local_group_index_canister_response,
        group_index_canister::add_local_group_index_canister::Response::Success
    ) {
        panic!("{add_local_group_index_canister_response:?}");
    }

    let add_local_user_index_canister_response = user_index_canister_client::add_local_user_index_canister(
        agent,
        &canister_ids.user_index,
        &user_index_canister::add_local_user_index_canister::Args {
            canister_id: canister_ids.local_user_index,
            notifications_canister_id: canister_ids.notifications,
        },
    )
    .await
    .unwrap();

    if !matches!(
        add_local_user_index_canister_response,
        user_index_canister::add_local_user_index_canister::Response::Success
    ) {
        panic!("{add_local_user_index_canister_response:?}");
    }

    let add_notifications_canister_response = notifications_index_canister_client::add_notifications_canister(
        agent,
        &canister_ids.notifications_index,
        &notifications_index_canister::add_notifications_canister::Args {
            canister_id: canister_ids.notifications,
            authorizers: vec![canister_ids.local_user_index, canister_ids.local_group_index],
        },
    )
    .await
    .unwrap();

    if !matches!(
        add_notifications_canister_response,
        notifications_index_canister::add_notifications_canister::Response::Success
    ) {
        panic!("{add_notifications_canister_response:?}");
    }

    println!("Canister wasms installed");
}

mod siwe {
    use candid::CandidType;

    #[allow(dead_code)]
    #[derive(CandidType)]
    pub enum RuntimeFeature {
        IncludeUriInSeed,
        DisableEthToPrincipalMapping,
        DisablePrincipalToEthMapping,
    }

    #[derive(CandidType)]
    pub struct SettingsInput {
        pub domain: String,
        pub uri: String,
        pub salt: String,
        pub chain_id: Option<u32>,
        pub scheme: Option<String>,
        pub statement: Option<String>,
        pub sign_in_expires_in: Option<u64>,
        pub session_expires_in: Option<u64>,
        pub targets: Option<Vec<String>>,
        pub runtime_features: Option<Vec<RuntimeFeature>>,
    }
}

mod siws {
    use candid::CandidType;

    #[allow(dead_code)]
    #[derive(CandidType)]
    pub enum RuntimeFeature {
        IncludeUriInSeed,
        DisableSolToPrincipalMapping,
        DisablePrincipalToSolMapping,
    }

    #[derive(CandidType)]
    pub struct SettingsInput {
        pub domain: String,
        pub uri: String,
        pub salt: String,
        pub chain_id: Option<String>,
        pub scheme: Option<String>,
        pub statement: Option<String>,
        pub sign_in_expires_in: Option<u64>,
        pub session_expires_in: Option<u64>,
        pub targets: Option<Vec<String>>,
        pub runtime_features: Option<Vec<RuntimeFeature>>,
    }
}
