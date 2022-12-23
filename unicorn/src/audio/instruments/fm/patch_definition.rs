use serde::{Deserialize, Serialize};

use crate::audio::instruments::fm::operator_definition::OperatorDefinitionBundle;

use crate::audio::instruments::fm::algorithm::Algorithm;
use crate::audio::instruments::fm::feedback::FeedbackLevel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchDefinition {
    pub operators: OperatorDefinitionBundle,
    pub algorithm: Algorithm,
    pub feedback: FeedbackLevel,
}
