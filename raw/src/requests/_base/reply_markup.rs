use std::ops::Not;

use serde::ser::{Serialize, Serializer};

use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(value: InlineKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboardMarkup(value)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(value: ReplyKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardMarkup(value)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(value: ReplyKeyboardRemove) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardRemove(value)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(value: ForceReply) -> ReplyMarkup {
        ReplyMarkup::ForceReply(value)
    }
}

impl Serialize for ReplyMarkup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            ReplyMarkup::InlineKeyboardMarkup(ref value) => value.serialize(serializer),
            ReplyMarkup::ReplyKeyboardMarkup(ref value) => value.serialize(serializer),
            ReplyMarkup::ReplyKeyboardRemove(ref value) => value.serialize(serializer),
            ReplyMarkup::ForceReply(ref value) => value.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Not::not")]
    pub resize_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool
}

impl ReplyKeyboardMarkup {
    pub fn new() -> Self {
        ReplyKeyboardMarkup {
            keyboard: Vec::new(),
            resize_keyboard: false,
            one_time_keyboard: false,
            selective: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_location: bool,
}

impl<'a> From<&'a str> for KeyboardButton {
    fn from(value: &'a str) -> KeyboardButton {
        KeyboardButton {
            text: value.to_string(),
            request_contact: false,
            request_location: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: True,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub kind: InlineKeyboardButtonKind
}

impl Serialize for InlineKeyboardButton {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use self::InlineKeyboardButtonKind::*;

        let mut raw = InlineKeyboardButtonRaw {
            text: &self.text,
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
//            callback_game: None,
        };

        match self.kind {
            Url(ref data) => raw.url = Some(data),
            CallbackData(ref data) => raw.callback_data = Some(data),
            SwitchInlineQuery(ref data) => raw.switch_inline_query = Some(data),
            SwitchInlineQueryCurrentChat(ref data) => raw.switch_inline_query_current_chat = Some(data),
//            CallbackGame(ref data) => raw.callback_game = Some(data),
        }

        Serialize::serialize(&raw, serializer)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InlineKeyboardButtonKind {
    Url(String), // TODO(knsd): Url?
    CallbackData(String),  //TODO(knsd) Validate size?
    SwitchInlineQuery(String),
    SwitchInlineQueryCurrentChat(String),
//    CallbackGame(CallbackGame),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
struct InlineKeyboardButtonRaw<'a> {
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>, // TODO(knsd): Url?
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_data: Option<&'a str>, //TODO(knsd) Validate size?
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query_current_chat: Option<&'a str>,
//    callback_game: Option<CallbackGame>,
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ForceReply {
    pub force_reply: True,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[macro_export]
macro_rules! reply_keyboard {
    () => {
        {
            use $crate::requests::_base::reply_markup::ReplyKeyboardMarkup;
            ReplyKeyboardMarkup::new()
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            use $crate::requests::_base::reply_markup::ReplyKeyboardMarkup;
            let mut keyboard = ReplyKeyboardMarkup::new();
            let data_as_nested_array = [ $( [ $(KeyboardButton::from($x)),* ] ),* ];

            let mut vec = Vec::new();
            for row in data_as_nested_array.iter() {
                let row = row.iter().cloned().collect();
                vec.push(row)
            }

            keyboard.keyboard = vec;
            keyboard
        }
    }
}
