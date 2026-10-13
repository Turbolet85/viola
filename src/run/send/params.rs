//! The `send` params.

use super::*;

/// `params` = `{text, from?}`; anything else is `-32602`.
pub(super) fn parse(params: &Value) -> Result<(&str, Option<ViolaName>), ProtocolError> {
    let text = params["text"]
        .as_str()
        .ok_or(ProtocolError::InvalidParams)?;
    Ok((text, parse_from(params)?))
}

/// `from`: absent, `null` or a valid name, self-reported; anything else is `-32602`.
pub(crate) fn parse_from(params: &Value) -> Result<Option<ViolaName>, ProtocolError> {
    match params.get("from") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(from)) => ViolaName::try_new(from.clone())
            .map(Some)
            .map_err(|_| ProtocolError::InvalidParams),
        Some(_) => Err(ProtocolError::InvalidParams),
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::*;

    use super::*;
    use std::sync::Arc;

    use rstest::rstest;

    #[rstest]
    #[case::no_text(json!({"v": 1}))]
    #[case::text_not_string(json!({"v": 1, "text": 5}))]
    #[case::from_not_a_name(json!({"v": 1, "text": "x", "from": "../x"}))]
    #[case::from_not_string(json!({"v": 1, "text": "x", "from": 3}))]
    fn send_params_it_cannot_take_are_invalid_params(#[case] params: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
        assert_eq!(
            send(&slot, &name(), tmp.path(), &call(&params, 1)),
            Err(ProtocolError::InvalidParams)
        );
        assert!(events(tmp.path()).is_empty());
    }

    #[test]
    fn send_from_null_is_no_from() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
        let params = json!({"v": 1, "text": "x", "from": null});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply["detail"], "input-not-ready");
    }
}
