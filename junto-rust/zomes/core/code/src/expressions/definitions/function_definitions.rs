use hdk::{
    error::{
        ZomeApiResult,
        ZomeApiError
    },
    holochain_core_types::{
        cas::content::Address, 
        hash::HashString,
        json::{
            JsonString,
            default_to_json
        },
        error::HolochainError,
        dna::capabilities::CapabilityRequest
    }
};

use std::collections::HashMap;
use serde::Serialize;

use super::app_definitions;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Env {
    pub dna_name: String,
    pub dna_address: String,
    pub agent_id: String,
    pub agent_address: String,
    pub cap_request: CapabilityRequest,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, DefaultJson)]
pub struct CreateUserInformation{
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: String,
    pub bio: String
}

//Basic struct to be used to describe a function and its parameters to the handle_hooks & handle_contextual_links functions
pub struct FunctionDescriptor{  
    pub name: &'static str,
    pub parameters: FunctionParameters,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct UserDens{
    pub private_den: EntryAndAddress<app_definitions::Channel>,
    pub shared_den: EntryAndAddress<app_definitions::Channel>,
    pub public_den: EntryAndAddress<app_definitions::Channel>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct JuntoUser{
    pub private_den: EntryAndAddress<app_definitions::Channel>,
    pub shared_den: EntryAndAddress<app_definitions::Channel>,
    pub public_den: EntryAndAddress<app_definitions::Channel>,
    pub pack: EntryAndAddress<app_definitions::Group>,
    pub profile: EntryAndAddress<app_definitions::User>,
    pub username: EntryAndAddress<app_definitions::UserName>,
}

#[derive(Serialize, Deserialize, DefaultJson, Debug, Clone)]
pub struct UserPack{
    pub pack: EntryAndAddress<app_definitions::Group>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMembers{
    pub members: Vec<EntryAndAddress<app_definitions::UserName>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpressionResults<T>{
    pub expressions: Vec<EntryAndAddress<T>>
}

#[derive(Clone)]
pub enum HooksResultTypes{
    TimeToExpression(Vec<Address>),
    CreatePack(UserPack),
    CreateDen(UserDens),
    LinkExpression(String),
    CreateQueryPoints(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryTarget{
    ExpressionPost,
    User
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryOptions {
    FilterPopular,
    FilterNew,
    FilterOld
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryType {
    And,
    Or
}

pub type EntryAndAddressResult<T> = Vec<EntryAndAddress<T>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryAndAddress<T>{
	pub address: HashString,
	pub entry: T
}

impl HooksResultTypes{
    // pub fn time_to_expression_result(self) -> ZomeApiResult<Vec<Address>> {
    //     match self {
    //         HooksResultTypes::TimeToExpression(r) => Ok(r),
    //         _ => Err(ZomeApiError::from("Hook result enum value not: TimeToExpression".to_string())),
    //     }
    // }
    pub fn create_pack_result(self) -> ZomeApiResult<UserPack> {
        match self {
            HooksResultTypes::CreatePack(r) => Ok(r),
            _ => Err(ZomeApiError::from("Hook result enum value not: CreatePack".to_string())),
        }
    }
    pub fn create_den_result(self) -> ZomeApiResult<UserDens> {
        match self {
            HooksResultTypes::CreateDen(r) => Ok(r),
            _ => Err(ZomeApiError::from("Hook result enum value not: CreateDen".to_string())),
        }
    }
    // pub fn link_expression_result(self) -> ZomeApiResult<String> {
    //     match self {
    //         HooksResultTypes::LinkExpression(r) => Ok(r),
    //         _ => Err(ZomeApiError::from("Hook result enum value not: LinkExpression".to_string())),
    //     }
    // }
    // pub fn create_query_points_result(self) -> ZomeApiResult<String> {
    //     match self {
    //         HooksResultTypes::CreateQueryPoints(r) => Ok(r),
    //         _ => Err(ZomeApiError::from("Hook result enum value not: CreateQueryPoints".to_string())),
    //     }
    // }
}

impl<T> PartialEq for EntryAndAddress<T>{
    fn eq(self: &Self, other: &EntryAndAddress<T>) -> bool {
        self.address == other.address
    }
}

impl From<GroupMembers> for JsonString {
    fn from(result: GroupMembers) -> JsonString {
        JsonString::from(json!(default_to_json(result)))
    }
}

impl From<ExpressionResults<app_definitions::UserName>> for JsonString {
    fn from(result: ExpressionResults<app_definitions::UserName>) -> JsonString{
        JsonString::from(default_to_json(result.expressions))
    }
}

impl From<ExpressionResults<app_definitions::ExpressionPost>> for JsonString {
    fn from(result: ExpressionResults<app_definitions::ExpressionPost>) -> JsonString{
        JsonString::from(default_to_json(result.expressions))
    }
}

impl<T: Into<JsonString>> From<EntryAndAddress<T>> for JsonString  where T: Serialize{
    fn from(result: EntryAndAddress<T>) -> JsonString {
        let entry = serde_json::to_string(&result.entry);
        let entry_string: String;
        match entry {
            Ok(entry) => entry_string = entry,
            Err(e) => return JsonString::from(HolochainError::SerializationError(e.to_string()))
        };
        let address = serde_json::to_string(&result.address);
        let address_string: String;
        match address{
            Ok(address) => address_string = address,
            Err(e) => return JsonString::from(HolochainError::SerializationError(e.to_string()))
        }

        json!(&format!("{{\"address\": {}, \"entry\": {}}}", address_string, entry_string)).into()
    }
}

//Parameters for each function in holochain application
pub enum FunctionParameters{
    TimeToExpression{
        link_type: &'static str,
        tag: &'static str, 
        direction: &'static str, 
        expression_address: Address,
        context: Address,
    },
    CreatePack{
        username_address: Address,
        first_name: String
    },
    CreateDen{
        username_address: Address,
        first_name: String
    },
    LinkExpression{
        link_type: &'static str,
        tag: &'static str, 
        direction: &'static str, 
        parent_expression: Address, 
        child_expression: Address
    },
    CreateQueryPoints{
        query_points: Vec<HashMap<String, String>>, 
        context: Address, 
        privacy: app_definitions::Privacy,
        query_type: String,
        expression: Address
    }
}
