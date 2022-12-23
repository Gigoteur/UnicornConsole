pub mod fm;
pub mod index_interpolator;
pub mod instrument_data_definition;
pub mod sampler;
pub mod wavetable;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstrumentKind {
    Sampler,
    FMSynth,
    Wavetable,
}

pub(crate) fn de_audio_data<'de, D>(deserializer: D) -> Result<Box<[i16]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let bytes = if deserializer.is_human_readable() {
        let text: String = serde::Deserialize::deserialize(deserializer)?;
        base64::decode(&text).map_err(serde::de::Error::custom)?
    } else {
        serde::Deserialize::deserialize(deserializer)?
    };
    let bytes: Vec<i16> = bytes
        .chunks_exact(2)
        .map(|slice| i16::from_be_bytes([slice[0], slice[1]]))
        .collect();
    Ok(bytes.into_boxed_slice())
}

pub(crate) fn ser_audio_data<S>(data: &[i16], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let data: Vec<u8> = data.iter().flat_map(|x| x.to_be_bytes()).collect();
    if serializer.is_human_readable() {
        let data = base64::encode(data);
        serializer.serialize_str(&data)
    } else {
        serializer.serialize_bytes(&data)
    }
}
