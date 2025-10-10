use ratatui::widgets::BorderType as RBorderType;
use serde::{Deserialize, Serialize, de, ser};

#[derive(Default, Serialize, Deserialize)]
pub enum BorderType {
    #[default]
    Plain,
    Rounded,
    Double,
    Thick,
    QuadrantInside,
    QuadrantOutside,
}

impl BorderType {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<RBorderType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bt: BorderType = de::Deserialize::deserialize(deserializer)?;
        Ok(RBorderType::from(bt))
    }

    pub fn serialize<S>(border_type: &RBorderType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(border_type.to_string().as_str())
    }
}

impl From<BorderType> for RBorderType {
    fn from(value: BorderType) -> Self {
        match value {
            BorderType::Plain => RBorderType::Plain,
            BorderType::Rounded => RBorderType::Rounded,
            BorderType::Double => RBorderType::Double,
            BorderType::Thick => RBorderType::Thick,
            BorderType::QuadrantInside => RBorderType::QuadrantInside,
            BorderType::QuadrantOutside => RBorderType::QuadrantOutside,
        }
    }
}
