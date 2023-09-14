use opentelemetry::trace::{SpanRef, TraceId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TraceInfo {
    pub trace_id: String,
    pub span_id: String,
}

pub(crate) fn trace_info_from_ref(span_ref: SpanRef<'_>) -> Option<TraceInfo> {
    let span_context = span_ref.span_context();
    let trace_id = span_context.trace_id();

    if trace_id == TraceId::INVALID {
        return None;
    }

    Some(TraceInfo {
        trace_id: trace_id.to_string(),
        span_id: span_context.span_id().to_string(),
    })
}
