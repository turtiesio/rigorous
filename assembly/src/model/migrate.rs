//! Bringing stimuli written under an older log schema up to the current vocabulary.

/// Bring an external stimulus written under an older log schema up to the current vocabulary.
/// Total: every older event has exactly one current form.
///
/// * 1 -> 2 changed only the file's header.
/// * 3 -> 4: a complete log closes with an end mark. No stimulus changed shape.
/// * 2 -> 3: a charter says whether it is delegable (older charters were not); the norm rules
///   carry a maintenance period (older rule sets meant the then-fixed 30 days). Fields that
///   became optional (a scope's purpose) need no step.
#[must_use]
pub fn upcast(from_schema: u32, mut event: serde_json::Value) -> serde_json::Value {
    use serde_json::{json, Value};
    // A stimulus addressed to a nested organisation is upgraded inside its wrapping.
    if let Some(inner) = event.pointer_mut("/Scoped/inner") {
        *inner = upcast(from_schema, inner.take());
        return event;
    }
    if from_schema < 3 {
        if let Some(constitution) = event.get_mut("Genesis") {
            for charter in constitution.get_mut("charters").and_then(Value::as_array_mut).into_iter().flatten() {
                if let Some(c) = charter.as_object_mut() {
                    let _ = c.entry("delegable").or_insert(json!(false));
                }
            }
            if let Some(norms) = constitution.pointer_mut("/rules/norms").and_then(Value::as_object_mut) {
                let _ = norms.entry("maintenance_period").or_insert(json!(30 * 24 * 3600));
            }
        }
    }
    event
}
