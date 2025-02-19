use crate::guards::caller_is_local_user_index;
use crate::{mutate_state, openchat_bot, RuntimeState};
use canister_api_macros::update_msgpack;
use canister_tracing_macros::trace;
use types::{Achievement, ChitEarnedReason, DiamondMembershipPlanDuration, MessageContentInitial, Timestamped};
use user_canister::c2c_notify_events::{Response::*, *};
use user_canister::mark_read::ChannelMessagesRead;
use user_canister::Event;

#[update_msgpack(guard = "caller_is_local_user_index")]
#[trace]
fn c2c_notify_events(args: Args) -> Response {
    mutate_state(|state| c2c_notify_events_impl(args, state))
}

fn c2c_notify_events_impl(args: Args, state: &mut RuntimeState) -> Response {
    for event in args.events {
        process_event(event, state);
    }
    Success
}

fn process_event(event: Event, state: &mut RuntimeState) {
    let now = state.env.now();

    match event {
        Event::UsernameChanged(ev) => {
            state.data.username = Timestamped::new(ev.username, now);
        }
        Event::DisplayNameChanged(ev) => {
            state.data.display_name = Timestamped::new(ev.display_name, now);
            state.data.award_achievement_and_notify(Achievement::SetDisplayName, now);
        }
        Event::PhoneNumberConfirmed(ev) => {
            state.data.phone_is_verified = true;
            state.data.storage_limit = ev.new_storage_limit;
            openchat_bot::send_phone_number_confirmed_bot_message(&ev, state);
        }
        Event::StorageUpgraded(ev) => {
            state.data.storage_limit = ev.new_storage_limit;
            openchat_bot::send_storage_ugraded_bot_message(&ev, state);
        }
        Event::ReferredUserRegistered(ev) => {
            openchat_bot::send_referred_user_joined_message(&ev, state);
        }
        Event::UserSuspended(ev) => {
            openchat_bot::send_user_suspended_message(&ev, state);
        }
        Event::OpenChatBotMessage(content) => {
            let initial_content: MessageContentInitial = (*content).into();
            openchat_bot::send_message(initial_content.into(), Vec::new(), false, state);
        }
        Event::OpenChatBotMessageV2(message) => {
            openchat_bot::send_message(message.content.into(), message.mentioned, false, state);
        }
        Event::UserJoinedGroup(ev) => {
            state
                .data
                .group_chats
                .join(ev.chat_id, ev.local_user_index_canister_id, ev.latest_message_index, now);
            state.data.hot_group_exclusions.remove(&ev.chat_id, now);
            state.data.award_achievement_and_notify(Achievement::JoinedGroup, now);
        }
        Event::UserJoinedCommunityOrChannel(ev) => {
            let (community, _) = state
                .data
                .communities
                .join(ev.community_id, ev.local_user_index_canister_id, now);
            community.mark_read(
                ev.channels
                    .into_iter()
                    .map(|c| ChannelMessagesRead {
                        channel_id: c.channel_id,
                        read_up_to: c.latest_message_index,
                        threads: Vec::new(),
                        date_read_pinned: None,
                    })
                    .collect(),
                now,
            );
            state.data.award_achievement_and_notify(Achievement::JoinedCommunity, now);
        }
        Event::DiamondMembershipPaymentReceived(ev) => {
            let mut awarded = state.data.award_achievement(Achievement::UpgradedToDiamond, now);

            if matches!(ev.duration, DiamondMembershipPlanDuration::Lifetime) {
                awarded |= state.data.award_achievement(Achievement::UpgradedToGoldDiamond, now);
            }

            if awarded {
                state.data.notify_user_index_of_chit(now);
            }

            state.data.diamond_membership_expires_at = Some(ev.expires_at);

            if ev.send_bot_message {
                openchat_bot::send_text_message(
                    "Payment received for Diamond membership!".to_string(),
                    Vec::new(),
                    false,
                    state,
                );
            }
        }
        // TODO: LEGACY - delete this once the website has switched to calling the new user::claim_daily_chit endpoint
        Event::ChitEarned(ev) => {
            let timestamp = ev.timestamp;
            let is_daily_claim = matches!(ev.reason, ChitEarnedReason::DailyClaim);

            state.data.chit_balance = Timestamped::new(state.data.chit_balance.value + ev.amount, now);

            state.data.chit_events.push(*ev);

            if is_daily_claim && state.data.streak.claim(timestamp) {
                let streak = state.data.streak.days(timestamp);

                if streak >= 3 {
                    state.data.award_achievement(Achievement::Streak3, now);
                }

                if streak >= 7 {
                    state.data.award_achievement(Achievement::Streak7, now);
                }

                if streak >= 14 {
                    state.data.award_achievement(Achievement::Streak14, now);
                }

                if streak >= 30 {
                    state.data.award_achievement(Achievement::Streak30, now);
                }
            }

            state.data.notify_user_index_of_chit(now);
        }
    }
}
