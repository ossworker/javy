use anyhow::Result;
use quickjs_wasm_rs::{Deserializer, JSContextRef, JSValueRef, Serializer};

pub fn transcode_input<'a>(context: &'a JSContextRef, bytes: &[u8]) -> Result<JSValueRef<'a>> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let mut serializer = Serializer::from_context(context)?;
    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    Ok(serializer.value)
}

pub fn transcode_output(val: JSValueRef) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut deserializer = Deserializer::from(val);
    let mut serializer = serde_json::Serializer::new(&mut output);
    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    Ok(output)
}