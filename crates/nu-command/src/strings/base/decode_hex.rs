use nu_engine::command_prelude::*;

#[derive(Clone)]
pub struct DecodeBase;

impl Command for DecodeBase {
    fn name(&self) -> &str {
        "decode base"
    }

    fn signature(&self) -> Signature {
        Signature::build("decode base")
            .input_output_types(vec![(Type::String, Type::Binary)])
            .allow_variants_without_examples(true)
            .required("encoding", SyntaxShape::String, "encoding to use")
            .category(Category::Formats)
    }

    fn usage(&self) -> &str {
        "Decode a value."
    }

    fn extra_usage(&self) -> &str {
        "TODO"
    }

    fn examples(&self) -> Vec<Example> {
        vec![]
    }

    fn is_const(&self) -> bool {
        true
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let name: String = call.req(engine_state, stack, 0)?;

        decode(&name, call.span(), input)
    }

    fn run_const(
        &self,
        working_set: &StateWorkingSet,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let name: String = call.req_const(working_set, 0)?;

        decode(&name, call.span(), input)
    }
}

fn decode(name: &str, call_span: Span, input: PipelineData) -> Result<PipelineData, ShellError> {
    let encoding = super::encoding(&name, call_span, call_span)?;
    let metadata = input.metadata();
    let (input_str, input_span) = super::get_string(input, call_span)?;
    let output = encoding.decode(input_str.as_bytes()).unwrap();

    Ok(Value::binary(output, call_span).into_pipeline_data_with_metadata(metadata))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        crate::test_examples(DecodeBase)
    }
}
