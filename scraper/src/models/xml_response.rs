use serde::{Serialize, Deserialize};

use crate::custom_de_serializers::empty_string;
use crate::custom_de_serializers::empty_float;
use crate::custom_de_serializers::date;

fn default_as_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Response {
    #[serde(rename(deserialize = "reportingOwner"))]
    pub reporting_owners: Vec<ReportingOwner>,

    #[serde(rename(deserialize = "derivativeTable"))]
    #[serde(default)]
    pub derivative_table: Option<DerivativeTable>,

    #[serde(rename(deserialize = "nonDerivativeTable"))]
    #[serde(default)]
    pub non_derivative_table: Option<NonDerivativeTable>,

    #[serde(rename(deserialize = "footnotes"))]
    #[serde(default)]
    pub footnotes: Option<Vec<Footnotes>>,
}

// OWNER
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ReportingOwner {
    #[serde(rename(deserialize = "reportingOwnerId"))]
    pub info: ReportingOwnerId,

    #[serde(rename(deserialize = "reportingOwnerAddress"))]
    pub address: ReportingOwnerAddress,

    #[serde(rename(deserialize = "reportingOwnerRelationship"))]
    pub relationship: ReportingOwnerRelationship,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ReportingOwnerId {
    #[serde(rename(deserialize = "rptOwnerCik"))]
    pub insider_cik: i32,
    #[serde(rename(deserialize = "rptOwnerName"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ReportingOwnerAddress {
    #[serde(rename(deserialize = "rptOwnerStreet1"))]
    pub street1: String,

    #[serde(rename(deserialize = "rptOwnerStreet2"))]
    #[serde(deserialize_with = "empty_string::deserialize")]
    #[serde(default)]
    pub street2: Option<String>,

    #[serde(rename(deserialize = "rptOwnerCity"))]
    pub city: String,

    #[serde(rename(deserialize = "rptOwnerState"))]
    pub state: String,

    #[serde(rename(deserialize = "rptOwnerZipCode"))]
    pub zip: String,

    #[serde(rename(deserialize = "rptOwnerStateDescription"))]
    #[serde(deserialize_with = "empty_string::deserialize")]
    #[serde(default)]
    pub state_description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ReportingOwnerRelationship {
    #[serde(rename(deserialize = "isDirector"))]
    #[serde(default = "default_as_false")]
    pub is_director: bool,

    #[serde(rename(deserialize = "isOfficer"))]
    #[serde(default = "default_as_false")]
    pub is_officer: bool,

    #[serde(rename(deserialize = "isTenPercentOwner"))]
    #[serde(default = "default_as_false")]
    pub is_ten_percent_owner: bool,

    #[serde(rename(deserialize = "isOther"))]
    #[serde(default = "default_as_false")]
    pub is_other: bool,

    #[serde(rename(deserialize = "officerTitle"))]
    #[serde(deserialize_with = "empty_string::deserialize")]
    #[serde(default)]
    pub officer_title: Option<String>,

    #[serde(rename(deserialize = "otherText"))]
    #[serde(deserialize_with = "empty_string::deserialize")]
    #[serde(default)]
    pub other_text: Option<String>
}

// TABLES
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DerivativeTable {
    #[serde(rename(deserialize = "$value"))]
    #[serde(default)]
    pub values: Option<Vec<DerivativeVar>>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct NonDerivativeTable {
    #[serde(rename(deserialize = "$value"))]
    #[serde(default)]
    pub values: Option<Vec<NonDerivativeVar>>,
}

// ENUMS || Non - Derivative
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NonDerivativeVar {
    #[serde(rename(deserialize = "nonDerivativeTransaction"))]
    NonDerivativeTransaction (NonDerivativeTransaction),

    #[serde(rename(deserialize = "nonDerivativeHolding"))]
    NonDerivativeHolding (NonDerivativeHolding)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeVar {
    #[serde(rename(deserialize = "derivativeTransaction"))]
    DerivativeTransaction (DerivativeTransaction),

    #[serde(rename(deserialize = "derivativeHolding"))]
    DerivativeHolding (DerivativeHolding)
}

// HOLDINGS
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DerivativeHolding {
    #[serde(rename(deserialize = "securityTitle"))]
    pub security_title: Value,

    #[serde(rename(deserialize = "conversionOrExercisePrice"))]
    pub conversion_or_exercise_price: FloatValue,

    #[serde(rename(deserialize = "exerciseDate"))]
    pub exercise_date: DateValue,

    #[serde(rename(deserialize = "expirationDate"))]
    pub expiration_date: DateValue,

    #[serde(rename(deserialize = "underlyingSecurity"))]
    pub underlying_security: UnderlyingSecurity,

    #[serde(rename(deserialize = "ownershipNature"))]
    pub ownership_nature: OwnershipNature,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct NonDerivativeHolding {
    #[serde(rename(deserialize = "securityTitle"))]
    pub security_title: Value,

    #[serde(rename(deserialize = "postTransactionAmounts"))]
    pub post_transaction_amounts: PostTransactionAmounts,

    #[serde(rename(deserialize = "ownershipNature"))]
    pub ownership_nature: OwnershipNature
}

// TRANSACTIONS
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DerivativeTransaction {
    #[serde(rename(deserialize = "securityTitle"))]
    pub security_title: Value,

    #[serde(rename(deserialize = "conversionOrExercisePrice"))]
    pub conversion_or_exercise_price: FloatValue,

    #[serde(rename(deserialize = "transactionDate"))]
    pub transaction_date: DateValue,

    #[serde(rename(deserialize = "deemedExecutionDate"))]
    pub deemed_execution_date: Option<DateValue>,

    #[serde(rename(deserialize = "transactionCoding"))]
    pub transaction_coding: TransactionCoding,

    #[serde(rename = "transactionTimeliness", skip)]
    transaction_timelines: String,

    #[serde(rename(deserialize = "transactionAmounts"))]
    pub transaction_amounts: TransactionAmounts,

    #[serde(rename(deserialize = "exerciseDate"))]
    pub exercise_date: DateValue,

    #[serde(rename(deserialize = "expirationDate"))]
    pub expiration_date: DateValue,

    #[serde(rename(deserialize = "underlyingSecurity"))]
    pub underlying_security: UnderlyingSecurity,

    #[serde(rename(deserialize = "postTransactionAmounts"))]
    pub post_transaction_amounts: PostTransactionAmounts,

    #[serde(rename(deserialize = "ownershipNature"))]
    pub ownership_nature: OwnershipNature,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct NonDerivativeTransaction {
    #[serde(rename = "securityTitle")]
    pub security_title: Value,
    #[serde(rename = "transactionDate")]
    pub transaction_date: DateValue,
    #[serde(rename = "transactionCoding")]
    pub transaction_coding: TransactionCoding,
    #[serde(rename = "transactionAmounts")]
    pub transaction_amounts: TransactionAmounts,
    #[serde(rename = "postTransactionAmounts")]
    pub post_transaction_amounts: PostTransactionAmounts,
    #[serde(rename = "ownershipNature")]
    pub ownership_nature: OwnershipNature,
}

// FOOTNOTES
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Footnotes {
    pub footnote: Option<Vec<Footnote>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Footnote {
    pub id: String,
    #[serde(rename(deserialize = "$value"))]
    pub text: String
}

// TRANSACTION AMOUNTS
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct TransactionAmounts {
    #[serde(rename(deserialize = "transactionShares"))]
    #[serde(default)]
    pub transaction_shares: FloatValue,

    #[serde(rename(deserialize = "transactionPricePerShare"))]
    pub transaction_price_per_share: FloatValue,

    #[serde(rename(deserialize = "transactionAcquiredDisposedCode"))]
    pub transaction_acquired_disposed_code: Value
}

// TRANSACTION CODING
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct TransactionCoding {
    #[serde(rename(deserialize = "transactionFormType"))]
    pub form_type: String,

    #[serde(rename(deserialize = "transactionCode"))]
    pub code: String,

    #[serde(rename(deserialize = "equitySwapInvolved"))]
    pub equity_swap_involved: bool
}

// POST TRANSACTION AMOUNTS
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct PostTransactionAmounts {
    #[serde(rename(deserialize = "sharesOwnedFollowingTransaction"))]
    pub shares_owned_following_transaction: Option<FloatValue>
}

// UNDERLYING SECURITY
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UnderlyingSecurity {
    #[serde(rename(deserialize = "underlyingSecurityTitle"))]
    pub security_title: Value,

    #[serde(rename(deserialize = "underlyingSecurityShares"))]
    #[serde(default)]
    pub share_price: FloatValue,
}

// OWNERSHIP NATURE
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct OwnershipNature {
    #[serde(rename(deserialize = "directOrIndirectOwnership"))]
    pub direct_or_indirect_ownership: Value,

    #[serde(rename(deserialize = "natureOfOwnership"))]
    pub nature_of_ownership: Option<Value>,
}

// VALUES
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Value {
    #[serde(rename = "footnoteId", skip)]
    footnote_id: String,

    #[serde(deserialize_with = "empty_string::deserialize")]
    #[serde(default)]
    pub value: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct FloatValue {
    #[serde(rename = "footnoteId", skip)]
    footnote_id: String,

    #[serde(deserialize_with = "empty_float::deserialize")]
    #[serde(default)]
    pub value: f32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DateValue {
    #[serde(rename = "footnoteId", skip)]
    footnote_id: String,

    #[serde(deserialize_with = "date::deserialize_option")]
    #[serde(default)]
    pub value: Option<i64>
}

impl Default for Value {
    fn default() -> Self {
        Self {
            footnote_id: "".to_string(),
            value: None
        }
    }
}
