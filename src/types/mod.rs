//! API types.

pub use allowed_update::*;
pub use animation::*;
pub use audio::*;
pub use callback_game::*;
pub use callback_query::*;
pub use chat::*;
pub use chat_action::*;
pub use chat_id::*;
pub use chat_member::*;
pub use chat_or_inline_message::*;
pub use chat_permissions::*;
pub use chat_photo::*;
pub use chosen_inline_result::*;
pub use contact::*;
pub use document::*;
pub use encrypted_credentials::*;
pub use encrypted_passport_element::*;
pub use file::*;
pub use force_reply::*;
pub use game::*;
pub use game_high_score::*;
pub use inline_keyboard_button::*;
pub use inline_keyboard_markup::*;
pub use inline_query::*;
pub use inline_query_result::*;
pub use inline_query_result_article::*;
pub use inline_query_result_audio::*;
pub use inline_query_result_cached_audio::*;
pub use inline_query_result_cached_document::*;
pub use inline_query_result_cached_gif::*;
pub use inline_query_result_cached_mpeg4_gif::*;
pub use inline_query_result_cached_photo::*;
pub use inline_query_result_cached_sticker::*;
pub use inline_query_result_cached_video::*;
pub use inline_query_result_cached_voice::*;
pub use inline_query_result_contact::*;
pub use inline_query_result_document::*;
pub use inline_query_result_game::*;
pub use inline_query_result_gif::*;
pub use inline_query_result_location::*;
pub use inline_query_result_mpeg4_gif::*;
pub use inline_query_result_photo::*;
pub use inline_query_result_venue::*;
pub use inline_query_result_video::*;
pub use inline_query_result_voice::*;
pub use input_file::*;
pub use input_media::*;
pub use input_message_content::*;
pub use invoice::*;
pub use keyboard_button::*;
pub use keyboard_button_poll_type::*;
pub use label_price::*;
pub use location::*;
pub use login_url::*;
pub use mask_position::*;
pub use message::*;
pub use message_entity::*;
pub use order_info::*;
pub use parse_mode::*;
pub use passport_data::*;
pub use passport_element_error::*;
pub use passport_file::*;
pub use photo_size::*;
pub use poll_type::*;
pub use poll_answer::*;
pub use poll::*;
pub use pre_checkout_query::*;
pub use reply_keyboard_markup::*;
pub use reply_keyboard_remove::*;
pub use reply_markup::*;
pub use response_parameters::*;
pub use send_invoice::*;
pub use shipping_address::*;
pub use shipping_option::*;
pub use shipping_query::*;
pub use sticker::*;
pub use sticker_set::*;
pub use successful_payment::*;
pub use unit_false::*;
pub use unit_true::*;
pub use update::*;
pub use user::*;
pub use user_profile_photos::*;
pub use venue::*;
pub use video::*;
pub use video_note::*;
pub use voice::*;
pub use webhook_info::*;

mod allowed_update;
mod animation;
mod audio;
mod callback_game;
mod callback_query;
mod chat;
mod chat_action;
mod chat_id;
mod chat_member;
mod chat_or_inline_message;
mod chat_permissions;
mod chat_photo;
mod chosen_inline_result;
mod contact;
mod document;
mod file;
mod force_reply;
mod game;
mod game_high_score;
mod inline_keyboard_button;
mod inline_keyboard_markup;
mod input_file;
mod input_media;
mod input_message_content;
mod invoice;
mod keyboard_button;
mod keyboard_button_poll_type;
mod label_price;
mod location;
mod login_url;
mod mask_position;
mod message;
mod message_entity;
mod order_info;
mod parse_mode;
mod photo_size;
mod poll;
mod poll_answer;
mod poll_type;
mod pre_checkout_query;
mod reply_keyboard_markup;
mod reply_keyboard_remove;
mod reply_markup;
mod response_parameters;
mod send_invoice;
mod shipping_address;
mod shipping_option;
mod shipping_query;
mod sticker;
mod sticker_set;
mod successful_payment;
mod unit_false;
mod unit_true;
mod update;
mod user;
mod user_profile_photos;
mod venue;
mod video;
mod video_note;
mod voice;
mod webhook_info;

mod inline_query;
mod inline_query_result;
mod inline_query_result_article;
mod inline_query_result_audio;
mod inline_query_result_cached_audio;
mod inline_query_result_cached_document;
mod inline_query_result_cached_gif;
mod inline_query_result_cached_mpeg4_gif;
mod inline_query_result_cached_photo;
mod inline_query_result_cached_sticker;
mod inline_query_result_cached_video;
mod inline_query_result_cached_voice;
mod inline_query_result_contact;
mod inline_query_result_document;
mod inline_query_result_game;
mod inline_query_result_gif;
mod inline_query_result_location;
mod inline_query_result_mpeg4_gif;
mod inline_query_result_photo;
mod inline_query_result_venue;
mod inline_query_result_video;
mod inline_query_result_voice;

mod encrypted_credentials;
mod encrypted_passport_element;
mod passport_data;
mod passport_element_error;
mod passport_file;

pub use non_telegram_types::*;
mod non_telegram_types;
