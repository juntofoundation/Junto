//Module to handle all channel related operations
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address
    }
};

use std::collections::HashMap;

use super::definitions::app_definitions;
use super::group;
use super::user;
use super::channel;

pub fn create_post_attributes(query_points: &Vec<HashMap<String, String>>, expression: &Address) -> ZomeApiResult<String>{
    //Creates links between expression and its global attributes (tags, types, times etc)
    for query_param in query_points{
        match query_param["type"].as_ref(){
            "tag" => {
                //save channels/types and times to a global anchor for each point - these anchors will eventually use bucket set pattern - for now they are like this - may also be removed - but its nice to be able to see all of these values
                let channel_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "tag".to_string()}.into()))?;
                let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                parent: channel_anchor.clone(), privacy: app_definitions::Privacy::Public, channel_type: app_definitions::ChannelType::Tag}.into()).into();
                hdk::debug("Commiting tag channel anchor")?;
                let address = hdk::commit_entry(&entry)?;
                hdk::debug(format!("Committed address: {}", address))?;
                hdk::api::link_entries(&channel_anchor, &address, "tag", &query_param["value"])?;
                hdk::debug(format!("Linked tag: {} to tag anchor", query_param["value"]))?;
                hdk::api::link_entries(&expression, &address, "expression_channels", &query_param["value"])?;
                hdk::debug("Linked entry to global tag entry")?;
            },

            "type" => {
                hdk::debug("Linking type to expression")?;
                let type_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "expression_type".to_string()}.into()))?;
                let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                                    parent: type_anchor.clone(), privacy: app_definitions::Privacy::Public, channel_type: app_definitions::ChannelType::Type}.into()).into();
                hdk::debug("Commiting type channel anchor")?;
                let address = hdk::commit_entry(&entry)?;
                hdk::debug(format!("Committed address: {}", address))?;
                hdk::api::link_entries(&type_anchor, &address, "expression_type", &query_param["value"])?;
                hdk::debug(format!("Linked type: {} to type anchor", query_param["value"]))?;
                hdk::api::link_entries(expression, &address, "expression_type", &query_param["value"])?;
            },

            "time:y" => {
                hdk::debug("Linking time:y to expression")?;
                let time_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "time".to_string()}.into()))?;   
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: time_anchor.clone(), time_type: app_definitions::TimeType::Year}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&time_anchor, &address, "time", &query_param["value"])?;
                hdk::debug(format!("Linked time: {} to time anchor", query_param["value"]))?;
                hdk::api::link_entries(expression, &address, "time", "year")?;
            },

            "time:m" => {
                hdk::debug("Linking time:m to expression")?;
                let time_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "time".to_string()}.into()))?;   
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: time_anchor.clone(), time_type: app_definitions::TimeType::Month}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&time_anchor, &address, "time", &query_param["value"])?;
                hdk::debug(format!("Linked time: {} to time anchor", query_param["value"]))?;
                hdk::api::link_entries(&expression, &address, "time", "month")?;
            },

            "time:d" => {
                hdk::debug("Linking time:d to expression")?;
                let time_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "time".to_string()}.into()))?;   
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: time_anchor.clone(), time_type: app_definitions::TimeType::Day}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&time_anchor, &address, "time", &query_param["value"])?;
                hdk::debug(format!("Linked time: {} to time anchor", query_param["value"]))?;
                hdk::api::link_entries(&expression, &address, "time", "day")?;
            },

            "time:h" => {
                hdk::debug("Linking time:h to expression")?;
                let time_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "time".to_string()}.into()))?;   
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: time_anchor.clone(), time_type: app_definitions::TimeType::Hour}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&time_anchor, &address, "time", &query_param["value"])?;
                hdk::debug(format!("Linked time: {} to time anchor", query_param["value"]))?;
                hdk::api::link_entries(&expression, &address, "time", "hour")?;
            },

            "user" => {
                //nothing currently needs to be done for user - expression -> owner link has already been done in handle_post_expression
            },

            _ => {
                return Err(ZomeApiError::from("That query parameter type does not exist".to_string()))
            }
        };
    };
    Ok("ok".to_string())
}

pub fn create_post_index(query_points: Vec<HashMap<String, String>>, context: &Address, privacy: &app_definitions::Privacy, 
                            expression: &Address, index_string: String, link_type: String) -> ZomeApiResult<String>{
    let current_user_hash = user::get_user_username_by_agent_address()?.address;
    match hdk::utils::get_as_type::<app_definitions::Channel>(context.clone()) {
        Ok(context_entry) => {
            hdk::debug("Context type channel, running auth")?;
            if context_entry.channel_type != app_definitions::ChannelType::Den{
                return Err(ZomeApiError::from("When context is a channel it must be of type den - you cannot post into other channel types".to_string()))
            };
            //check that current user making post is owner of den they are trying to post into
            if channel::is_den_owner(context.clone(), current_user_hash.clone())? == false{
                return Err(ZomeApiError::from("You are attempting to get results from a private channel which you do not own".to_string()))
            };
            //make link on channel (den) context
            hdk::api::link_entries(&context, expression, link_type, index_string)?;
        },
        Err(_err) => {
            hdk::debug("Context type group, running auth")?;
            let context_entry = hdk::utils::get_as_type::<app_definitions::Group>(context.clone()).map_err(|_err| ZomeApiError::from("Context address was not a channel, group or dna address (global context)".to_string()))?;
            if context_entry.privacy != app_definitions::Privacy::Public {
                if (group::is_group_owner(context.clone(), current_user_hash.clone())? == false) & (group::is_group_member(context.clone(), current_user_hash.clone())? == false){
                    return Err(ZomeApiError::from("You are attempting to post an expression into a group you are not permitted to interact with".to_string()))
                };
            };
            //make link on group context
            hdk::api::link_entries(&context, expression, link_type, index_string)?;
        }
    };
    
    //Code below is used to allow a given context to see which index points posts exist on in their context
    //TODO might make more sense just to link to global entry - why do we need more entries for the same thing? - we are no longer linking from these entries so there is no scaling considerations
    hdk::debug("Creating entries for each index in each context and linking expression")?;
    for query_param in query_points{
        match query_param["type"].as_ref(){
            "tag" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                parent: context.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Tag}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "tag", &query_param["value"])?;
            },

            "type" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                parent: context.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Type}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "expression_type", &query_param["value"])?;
            },

            "time:y" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeType::Year}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "time", &query_param["value"])?;
            },

            "time:m" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeType::Month}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "time", &query_param["value"])?;
            },

            "time:d" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeType::Day}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "time", &query_param["value"])?;
            },

            "time:h" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeType::Hour}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "time", &query_param["value"])?;
            },

            "user" => {
                //nothing currently needs to be done for user - expression -> owner link has already been done in handle_post_expression
            },

            _ => {
                return Err(ZomeApiError::from("That query parameter type does not exist".to_string()))
            }
        };
    };
    Ok("ok".to_string())
}