/* This file is automatically generated by packets.clj
Do not manually edit this file, if you wish to make
changes here, then edit and rerun packets.clj */

/// Represents a single packet
#[derive(Debug, PartialEq, Clone)]
pub enum ServerboundPacket {
    Handshake(Handshake),
    StatusRequest(StatusRequest),
    StatusPing(StatusPing),
    LoginStart(LoginStart),
    EncryptionResponse(EncryptionResponse),
    TeleportConfirm(TeleportConfirm),
    TabComplete(TabComplete),
    ChatMessage(ChatMessage),
    ClientStatus(ClientStatus),
    ClientSettings(ClientSettings),
    ConfirmTransaction(ConfirmTransaction),
    EnchantItem(EnchantItem),
    ClickWindow(ClickWindow),
    CloseWindow(CloseWindow),
    PluginMessage(PluginMessage),
    UseEntity(UseEntity),
    KeepAlive(KeepAlive),
    Player(Player),
    PlayerPosition(PlayerPosition),
    PlayerPositionAndLook(PlayerPositionAndLook),
    PlayerLook(PlayerLook),
    VehicleMove(VehicleMove),
    SteerBoat(SteerBoat),
    CraftRecipeRequest(CraftRecipeRequest),
    PlayerAbilities(PlayerAbilities),
    PlayerDigging(PlayerDigging),
    EntityAction(EntityAction),
    SteerVehicle(SteerVehicle),
    CraftingBookData(CraftingBookData),
    ResourcePackStatus(ResourcePackStatus),
    AdvancementTab(AdvancementTab),
    HeldItemChange(HeldItemChange),
    CreativeInventoryAction(CreativeInventoryAction),
    UpdateSign(UpdateSign),
    Animation(Animation),
    Spectate(Spectate),
    PlayerBlockPlacement(PlayerBlockPlacement),
    UseItem(UseItem),

}

impl Packet for ServerboundPacket {
    pub fn deserialize<R: Read>(r: &mut R, state: &ClientState) -> Result<Self> {
        let packet_id = read_varint(r)?;
        match state {
        &ClientState::Handshake => {
            match packet_id {
            0 => Ok(Handshake::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },
        &ClientState::Status => {
            match packet_id {
            0 => Ok(StatusRequest::deserialize(r)?),
            1 => Ok(StatusPing::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },
        &ClientState::Login => {
            match packet_id {
            0 => Ok(LoginStart::deserialize(r)?),
            1 => Ok(EncryptionResponse::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },
        &ClientState::Play => {
            match packet_id {
            0 => Ok(TeleportConfirm::deserialize(r)?),
            1 => Ok(TabComplete::deserialize(r)?),
            2 => Ok(ChatMessage::deserialize(r)?),
            3 => Ok(ClientStatus::deserialize(r)?),
            4 => Ok(ClientSettings::deserialize(r)?),
            5 => Ok(ConfirmTransaction::deserialize(r)?),
            6 => Ok(EnchantItem::deserialize(r)?),
            7 => Ok(ClickWindow::deserialize(r)?),
            8 => Ok(CloseWindow::deserialize(r)?),
            9 => Ok(PluginMessage::deserialize(r)?),
            10 => Ok(UseEntity::deserialize(r)?),
            11 => Ok(KeepAlive::deserialize(r)?),
            12 => Ok(Player::deserialize(r)?),
            13 => Ok(PlayerPosition::deserialize(r)?),
            14 => Ok(PlayerPositionAndLook::deserialize(r)?),
            15 => Ok(PlayerLook::deserialize(r)?),
            16 => Ok(VehicleMove::deserialize(r)?),
            17 => Ok(SteerBoat::deserialize(r)?),
            18 => Ok(CraftRecipeRequest::deserialize(r)?),
            19 => Ok(PlayerAbilities::deserialize(r)?),
            20 => Ok(PlayerDigging::deserialize(r)?),
            21 => Ok(EntityAction::deserialize(r)?),
            22 => Ok(SteerVehicle::deserialize(r)?),
            23 => Ok(CraftingBookData::deserialize(r)?),
            24 => Ok(ResourcePackStatus::deserialize(r)?),
            25 => Ok(AdvancementTab::deserialize(r)?),
            26 => Ok(HeldItemChange::deserialize(r)?),
            27 => Ok(CreativeInventoryAction::deserialize(r)?),
            28 => Ok(UpdateSign::deserialize(r)?),
            29 => Ok(Animation::deserialize(r)?),
            30 => Ok(Spectate::deserialize(r)?),
            31 => Ok(PlayerBlockPlacement::deserialize(r)?),
            32 => Ok(UseItem::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },

        }
    }
    pub fn get_packet_name(&self) -> &str {
        match self {
        &ServerboundPacket::Handshake(..) => "Handshake",
        &ServerboundPacket::StatusRequest(..) => "StatusRequest",
        &ServerboundPacket::StatusPing(..) => "StatusPing",
        &ServerboundPacket::LoginStart(..) => "LoginStart",
        &ServerboundPacket::EncryptionResponse(..) => "EncryptionResponse",
        &ServerboundPacket::TeleportConfirm(..) => "TeleportConfirm",
        &ServerboundPacket::TabComplete(..) => "TabComplete",
        &ServerboundPacket::ChatMessage(..) => "ChatMessage",
        &ServerboundPacket::ClientStatus(..) => "ClientStatus",
        &ServerboundPacket::ClientSettings(..) => "ClientSettings",
        &ServerboundPacket::ConfirmTransaction(..) => "ConfirmTransaction",
        &ServerboundPacket::EnchantItem(..) => "EnchantItem",
        &ServerboundPacket::ClickWindow(..) => "ClickWindow",
        &ServerboundPacket::CloseWindow(..) => "CloseWindow",
        &ServerboundPacket::PluginMessage(..) => "PluginMessage",
        &ServerboundPacket::UseEntity(..) => "UseEntity",
        &ServerboundPacket::KeepAlive(..) => "KeepAlive",
        &ServerboundPacket::Player(..) => "Player",
        &ServerboundPacket::PlayerPosition(..) => "PlayerPosition",
        &ServerboundPacket::PlayerPositionAndLook(..) => "PlayerPositionAndLook",
        &ServerboundPacket::PlayerLook(..) => "PlayerLook",
        &ServerboundPacket::VehicleMove(..) => "VehicleMove",
        &ServerboundPacket::SteerBoat(..) => "SteerBoat",
        &ServerboundPacket::CraftRecipeRequest(..) => "CraftRecipeRequest",
        &ServerboundPacket::PlayerAbilities(..) => "PlayerAbilities",
        &ServerboundPacket::PlayerDigging(..) => "PlayerDigging",
        &ServerboundPacket::EntityAction(..) => "EntityAction",
        &ServerboundPacket::SteerVehicle(..) => "SteerVehicle",
        &ServerboundPacket::CraftingBookData(..) => "CraftingBookData",
        &ServerboundPacket::ResourcePackStatus(..) => "ResourcePackStatus",
        &ServerboundPacket::AdvancementTab(..) => "AdvancementTab",
        &ServerboundPacket::HeldItemChange(..) => "HeldItemChange",
        &ServerboundPacket::CreativeInventoryAction(..) => "CreativeInventoryAction",
        &ServerboundPacket::UpdateSign(..) => "UpdateSign",
        &ServerboundPacket::Animation(..) => "Animation",
        &ServerboundPacket::Spectate(..) => "Spectate",
        &ServerboundPacket::PlayerBlockPlacement(..) => "PlayerBlockPlacement",
        &ServerboundPacket::UseItem(..) => "UseItem",

        }
    }
    pub fn get_clientstate(&self) -> ClientState {
        match self {
        &ServerboundPacket::Handshake(..) => ClientState::Handshake,
        &ServerboundPacket::StatusRequest(..) => ClientState::Status,
        &ServerboundPacket::StatusPing(..) => ClientState::Status,
        &ServerboundPacket::LoginStart(..) => ClientState::Login,
        &ServerboundPacket::EncryptionResponse(..) => ClientState::Login,
        &ServerboundPacket::TeleportConfirm(..) => ClientState::Play,
        &ServerboundPacket::TabComplete(..) => ClientState::Play,
        &ServerboundPacket::ChatMessage(..) => ClientState::Play,
        &ServerboundPacket::ClientStatus(..) => ClientState::Play,
        &ServerboundPacket::ClientSettings(..) => ClientState::Play,
        &ServerboundPacket::ConfirmTransaction(..) => ClientState::Play,
        &ServerboundPacket::EnchantItem(..) => ClientState::Play,
        &ServerboundPacket::ClickWindow(..) => ClientState::Play,
        &ServerboundPacket::CloseWindow(..) => ClientState::Play,
        &ServerboundPacket::PluginMessage(..) => ClientState::Play,
        &ServerboundPacket::UseEntity(..) => ClientState::Play,
        &ServerboundPacket::KeepAlive(..) => ClientState::Play,
        &ServerboundPacket::Player(..) => ClientState::Play,
        &ServerboundPacket::PlayerPosition(..) => ClientState::Play,
        &ServerboundPacket::PlayerPositionAndLook(..) => ClientState::Play,
        &ServerboundPacket::PlayerLook(..) => ClientState::Play,
        &ServerboundPacket::VehicleMove(..) => ClientState::Play,
        &ServerboundPacket::SteerBoat(..) => ClientState::Play,
        &ServerboundPacket::CraftRecipeRequest(..) => ClientState::Play,
        &ServerboundPacket::PlayerAbilities(..) => ClientState::Play,
        &ServerboundPacket::PlayerDigging(..) => ClientState::Play,
        &ServerboundPacket::EntityAction(..) => ClientState::Play,
        &ServerboundPacket::SteerVehicle(..) => ClientState::Play,
        &ServerboundPacket::CraftingBookData(..) => ClientState::Play,
        &ServerboundPacket::ResourcePackStatus(..) => ClientState::Play,
        &ServerboundPacket::AdvancementTab(..) => ClientState::Play,
        &ServerboundPacket::HeldItemChange(..) => ClientState::Play,
        &ServerboundPacket::CreativeInventoryAction(..) => ClientState::Play,
        &ServerboundPacket::UpdateSign(..) => ClientState::Play,
        &ServerboundPacket::Animation(..) => ClientState::Play,
        &ServerboundPacket::Spectate(..) => ClientState::Play,
        &ServerboundPacket::PlayerBlockPlacement(..) => ClientState::Play,
        &ServerboundPacket::UseItem(..) => ClientState::Play,

        }
    }
    pub fn get_id(&self) -> i32 {
        match self {
        &ServerboundPacket::Handshake(..) => 0,
        &ServerboundPacket::StatusRequest(..) => 0,
        &ServerboundPacket::StatusPing(..) => 1,
        &ServerboundPacket::LoginStart(..) => 0,
        &ServerboundPacket::EncryptionResponse(..) => 1,
        &ServerboundPacket::TeleportConfirm(..) => 0,
        &ServerboundPacket::TabComplete(..) => 1,
        &ServerboundPacket::ChatMessage(..) => 2,
        &ServerboundPacket::ClientStatus(..) => 3,
        &ServerboundPacket::ClientSettings(..) => 4,
        &ServerboundPacket::ConfirmTransaction(..) => 5,
        &ServerboundPacket::EnchantItem(..) => 6,
        &ServerboundPacket::ClickWindow(..) => 7,
        &ServerboundPacket::CloseWindow(..) => 8,
        &ServerboundPacket::PluginMessage(..) => 9,
        &ServerboundPacket::UseEntity(..) => 10,
        &ServerboundPacket::KeepAlive(..) => 11,
        &ServerboundPacket::Player(..) => 12,
        &ServerboundPacket::PlayerPosition(..) => 13,
        &ServerboundPacket::PlayerPositionAndLook(..) => 14,
        &ServerboundPacket::PlayerLook(..) => 15,
        &ServerboundPacket::VehicleMove(..) => 16,
        &ServerboundPacket::SteerBoat(..) => 17,
        &ServerboundPacket::CraftRecipeRequest(..) => 18,
        &ServerboundPacket::PlayerAbilities(..) => 19,
        &ServerboundPacket::PlayerDigging(..) => 20,
        &ServerboundPacket::EntityAction(..) => 21,
        &ServerboundPacket::SteerVehicle(..) => 22,
        &ServerboundPacket::CraftingBookData(..) => 23,
        &ServerboundPacket::ResourcePackStatus(..) => 24,
        &ServerboundPacket::AdvancementTab(..) => 25,
        &ServerboundPacket::HeldItemChange(..) => 26,
        &ServerboundPacket::CreativeInventoryAction(..) => 27,
        &ServerboundPacket::UpdateSign(..) => 28,
        &ServerboundPacket::Animation(..) => 29,
        &ServerboundPacket::Spectate(..) => 30,
        &ServerboundPacket::PlayerBlockPlacement(..) => 31,
        &ServerboundPacket::UseItem(..) => 32,

        }
    }
    pub fn to_u8(&self) -> Result<Vec<u8>> {
        match self {
        &ServerboundPacket::Handshake(ref x) => x.to_u8(),
        &ServerboundPacket::StatusRequest(ref x) => x.to_u8(),
        &ServerboundPacket::StatusPing(ref x) => x.to_u8(),
        &ServerboundPacket::LoginStart(ref x) => x.to_u8(),
        &ServerboundPacket::EncryptionResponse(ref x) => x.to_u8(),
        &ServerboundPacket::TeleportConfirm(ref x) => x.to_u8(),
        &ServerboundPacket::TabComplete(ref x) => x.to_u8(),
        &ServerboundPacket::ChatMessage(ref x) => x.to_u8(),
        &ServerboundPacket::ClientStatus(ref x) => x.to_u8(),
        &ServerboundPacket::ClientSettings(ref x) => x.to_u8(),
        &ServerboundPacket::ConfirmTransaction(ref x) => x.to_u8(),
        &ServerboundPacket::EnchantItem(ref x) => x.to_u8(),
        &ServerboundPacket::ClickWindow(ref x) => x.to_u8(),
        &ServerboundPacket::CloseWindow(ref x) => x.to_u8(),
        &ServerboundPacket::PluginMessage(ref x) => x.to_u8(),
        &ServerboundPacket::UseEntity(ref x) => x.to_u8(),
        &ServerboundPacket::KeepAlive(ref x) => x.to_u8(),
        &ServerboundPacket::Player(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerPosition(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerPositionAndLook(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerLook(ref x) => x.to_u8(),
        &ServerboundPacket::VehicleMove(ref x) => x.to_u8(),
        &ServerboundPacket::SteerBoat(ref x) => x.to_u8(),
        &ServerboundPacket::CraftRecipeRequest(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerAbilities(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerDigging(ref x) => x.to_u8(),
        &ServerboundPacket::EntityAction(ref x) => x.to_u8(),
        &ServerboundPacket::SteerVehicle(ref x) => x.to_u8(),
        &ServerboundPacket::CraftingBookData(ref x) => x.to_u8(),
        &ServerboundPacket::ResourcePackStatus(ref x) => x.to_u8(),
        &ServerboundPacket::AdvancementTab(ref x) => x.to_u8(),
        &ServerboundPacket::HeldItemChange(ref x) => x.to_u8(),
        &ServerboundPacket::CreativeInventoryAction(ref x) => x.to_u8(),
        &ServerboundPacket::UpdateSign(ref x) => x.to_u8(),
        &ServerboundPacket::Animation(ref x) => x.to_u8(),
        &ServerboundPacket::Spectate(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerBlockPlacement(ref x) => x.to_u8(),
        &ServerboundPacket::UseItem(ref x) => x.to_u8(),

        }
    }
}
impl fmt::Display for ServerboundPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ServerboundPacket of type {}", self.get_packet_name())
    }
}