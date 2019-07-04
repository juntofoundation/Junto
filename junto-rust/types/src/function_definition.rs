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
use std::fmt::Debug;

use super::app_definition;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Env {
    pub dna_name: String,
    pub dna_address: String,
    pub agent_id: String,
    pub agent_address: String,
    pub cap_request: Option<CapabilityRequest>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, DefaultJson)]
pub struct CreateUserInformation{
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: String,
    pub bio: String
}

//Basic struct to be used to describe a function and its parameters to the handle_hooks function
pub struct FunctionDescriptor<'a>{  
    pub name: &'static str,
    pub parameters: FunctionParameters<'a>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct UserDens{
    pub private_den: EntryAndAddress<app_definition::Collection>,
    pub shared_den: EntryAndAddress<app_definition::Collection>,
    pub public_den: EntryAndAddress<app_definition::Collection>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct JuntoUser{
    pub private_den: EntryAndAddress<app_definition::Collection>,
    pub shared_den: EntryAndAddress<app_definition::Collection>,
    pub public_den: EntryAndAddress<app_definition::Collection>,
    pub pack: EntryAndAddress<app_definition::Group>,
    pub profile: EntryAndAddress<app_definition::User>,
    pub username: EntryAndAddress<app_definition::UserName>,
    pub user_perspective: EntryAndAddress<app_definition::Perspective>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMembers{
    pub members: Vec<EntryAndAddress<app_definition::UserName>>
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct ExpressionData{
    pub expression: EntryAndAddress<app_definition::ExpressionPost>,
    pub sub_expressions: Vec<ExpressionData>,
    pub author_username: EntryAndAddress<app_definition::UserName>,
    pub author_profile: EntryAndAddress<app_definition::User>,
    pub resonations: Vec<EntryAndAddress<app_definition::UserName>>,
    pub timestamp: String,
    pub channels: Vec<EntryAndAddress<app_definition::Attribute>>
}

#[derive(Clone)]
pub enum HooksResultTypes{
    TimeToExpression(Vec<Address>),
    CreatePack(EntryAndAddress<app_definition::Group>),
    CreateDen(UserDens),
    LinkExpression(&'static str),
    CreatePostIndex(&'static str)
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

#[derive(Debug, Serialize, Deserialize)]
pub enum ContextAuthResult {
    Collection(app_definition::Collection),
    Group(app_definition::Group)
}

pub type EntryAndAddressResult<T> = Vec<EntryAndAddress<T>>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash)]
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
    pub fn create_pack_result(self) -> ZomeApiResult<EntryAndAddress<app_definition::Group>> {
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
    // pub fn create_post_index_result(self) -> ZomeApiResult<String> {
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

impl<T: Into<JsonString>> From<EntryAndAddress<T>> for JsonString  where T: Serialize + Debug{
    fn from(result: EntryAndAddress<T>) -> JsonString {
        JsonString::from(default_to_json(result))
    }
}

//Parameters for each function in holochain application
#[derive(Serialize, Debug)]
pub enum FunctionParameters<'a>{
    TimeToExpression{
        link_type: &'a str,
        tag: &'a str, 
        direction: &'a str, 
        expression_address: Address
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
        link_type: &'a str,
        tag: &'a str, 
        direction: &'a str, 
        parent_expression: Address, 
        child_expression: Address
    },
    CreatePostIndex{
        indexes: &'a Vec<HashMap<&'static str, String>>, 
        context: Address, 
        expression: Address,
        index_string: &'a str,
        link_type: &'a str
    }
}


impl<'a> From<FunctionParameters<'a>> for JsonString {
    fn from(result: FunctionParameters) -> JsonString {
        JsonString::from(default_to_json(result))
    }
}