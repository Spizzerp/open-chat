import React from "react";
import { useDispatch } from "react-redux";
import UserAvatar from "./UserAvatar";
import GroupChatIcon from "../assets/icons/groupChatIcon.svg";
import selectChat from "../actions/chats/selectChat";
import { Option } from "../domain/model/common";
import { UserId } from "../domain/model/users";
import { formatMessageDate } from "../formatters/date";
import ParticipantsTyping from "./ParticipantsTyping";
import TextContent from "./TextContent";
import ThemTyping from "./ThemTyping";

type Props = {
    name: string,
    date?: Date,
    index: number,
    selected: boolean,
    latestMessage: string,
    isGroup: boolean,
    userId: Option<UserId>,
    userImageId: Option<string>,
    unreadCount: number,
    themTyping: boolean,
    userOnline: boolean,
    participantsTyping: string[]
}

export default React.memo(ChatListItem);

function ChatListItem(props: Props) {
    const dispatch = useDispatch();
    const className = props.selected ? "selected" : "";
    const icon = props.isGroup
        ? <GroupChatIcon className="avatar" />
        : <UserAvatar 
            isUserOnline={props.userOnline} 
            userId={props.userId} 
            imageId={props.userImageId} />;

    let snippet: JSX.Element;
    if (props.themTyping) {
        snippet = <ThemTyping />;
    } else if (props.participantsTyping.length) {
        snippet = <ParticipantsTyping usernames={props.participantsTyping} />
    } else {
        snippet = <TextContent text={props.latestMessage} insertLineBreaks={false} />;
    }

    return (
        <li className={className} onClick={() => dispatch(selectChat(props.index))}>
            {icon}
            <div className="message-container">
                <div>
                    <div className="date">{props.date ? formatMessageDate(props.date) : null}</div>
                    <div className="name">{props.name}</div>
                </div>
                <div>
                    {props.unreadCount > 0 ? <div className="unread-count">{props.unreadCount.toString()}</div> : null} 
                    <div className="chats-message">
                        {snippet}
                    </div>
                </div>
            </div>
        </li>
    );
}
