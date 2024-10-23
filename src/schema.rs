use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Hospital {
    pub affirmation: Affirmation,
    pub hospital_address: Vec<String>,
    pub hospital_location: Vec<String>,
    pub hospital_name: String,
    pub last_updated_on: chrono::NaiveDate,
    pub license_information: LicenseInformation,
    pub standard_charge_information: Vec<StandardChargeInformation>,
    pub version: String,
    pub modifier_information: Option<Vec<ModifierInformation>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModifierInformation {
    description: String,
    code: String,
    modifier_payer_information: Vec<ModifierPayerInformation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModifierPayerInformation {
    payer_name: String,
    plan_name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Affirmation {
    pub affirmation: String,
    pub confirm_affirmation: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LicenseInformation {
    pub license_number: String,
    pub state: State,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct StandardChargeInformation {
    description: String,
    drug_information: Option<DrugInformation>,
    code_information: Vec<CodeInformation>,
    standard_charges: Vec<StandardCharge>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeInformation {
    code: String,
    #[serde(rename = "type")]
    code_type: CodeType,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct StandardCharge {
    minimum: Option<f64>,
    maximum: Option<f64>,
    gross_charge: Option<f64>,
    discounted_cash: Option<f64>,
    setting: Setting,
    payers_information: Option<Vec<PayersInformation>>,
    additional_generic_notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Setting {
    Inpatient,
    Outpatient,
    Both,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PayersInformation {
    payer_name: String,
    plan_name: String,
    additional_payer_notes: Option<String>,
    standard_charge_dollar: Option<f64>,
    standard_charge_algorithm: Option<String>,
    standard_charge_percentage: Option<f64>,
    estimated_amount: Option<f64>,
    methodology: Methodology,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Methodology {
    #[serde(rename = "case rate")]
    CaseRate,
    #[serde(rename = "fee schedule")]
    FeeSchedule,
    #[serde(rename = "percent of total billed charges")]
    PercentOfTotalBilledCharges,
    #[serde(rename = "per diem")]
    PerDiem,
    #[serde(rename = "other")]
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrugInformation {
    unit: String,
    drug_type: DrugType,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum DrugType {
    GR,
    ML,
    ME,
    UN,
    F2,
    GM,
    EA,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum CodeType {
    CPT,
    HCPCS,
    ICD,
    DRG,

    #[serde(rename = "MS-DRG")]
    MSDRG,
    #[serde(rename = "R-DRG")]
    RDRG,
    #[serde(rename = "S-DRG")]
    SDRG,
    #[serde(rename = "APS-DRG")]
    APSDRG,
    #[serde(rename = "AP-DRG")]
    APDRG,
    #[serde(rename = "APR-DRG")]
    APRDRG,
    #[serde(rename = "TRIS-DRG")]
    TRISDRG,
    APC,
    NDC,
    HIPPS,
    LOCAL,
    EAPG,
    CDT,
    RC,
    CDM,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum State {
    AL,
    AK,
    AS,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    DC,
    FM,
    FL,
    GA,
    GU,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MH,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    MP,
    OH,
    OK,
    OR,
    PW,
    PA,
    PR,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VI,
    VA,
    WA,
    WV,
    WI,
    WY,
}
