use crate::args::*;

pub(crate) enum GenericParameterInfosFormattingPolicy<'a> {
    Perform(&'a [GenericParameterInfo]),
    Skip,
}

pub(crate) fn fmt_call(
    fn_name: &str,
    args_check_results: Vec<ArgCheckResult>,
    generic_parameter_infos_formatting: GenericParameterInfosFormattingPolicy,
) -> String {
    let error_msgs: Vec<_> = args_check_results
        .iter()
        .filter_map(ArgCheckResult::as_err)
        .enumerate()
        .map(|(i, x)| {
            let error_number = i + 1;
            format!(
                "{}. {} ({}):
{}",
                error_number,
                x.arg_info.arg_name(),
                x.arg_info.arg_type_name(),
                x.error_msg
            )
        })
        .collect();
    let errors_count = error_msgs.len();
    let errors_report = if errors_count == 0 {
        String::new()
    } else {
        let error_msgs_joined = error_msgs.join("\n\t");
        format!(
            "
\t{error_msgs_joined}"
        )
    };
    let args_msg = fmt_fn_parameters_msg(
        fn_name,
        args_check_results,
        generic_parameter_infos_formatting,
    );
    format!("{args_msg}{errors_report}")
}

pub(crate) fn fmt_fn_parameters_msg(
    fn_name: &str,
    args_check_results: Vec<ArgCheckResult>,
    generic_parameter_infos_formatting: GenericParameterInfosFormattingPolicy,
) -> String {
    let args_msgs: Vec<_> = args_check_results
        .into_iter()
        .map(|x| match x {
            ArgCheckResult::Ok(x) => x.arg_info.clone_arg_debug_string(),
            ArgCheckResult::Err(x) => format!("*{}*", x.arg_info.clone_arg_debug_string()),
        })
        .collect();
    let args_msgs_joined = args_msgs.join(", ");
    let generic_parameters_msg = fmt_generic_parameter_infos(generic_parameter_infos_formatting);
    let args_msg = format!("{fn_name}{generic_parameters_msg}({args_msgs_joined})");
    return args_msg;
}

pub(crate) fn fmt_generic_parameter_infos(
    generic_parameter_infos_formatting: GenericParameterInfosFormattingPolicy,
) -> String {
    let result = match generic_parameter_infos_formatting {
        GenericParameterInfosFormattingPolicy::Perform(generic_parameter_infos)
            if !generic_parameter_infos.is_empty() =>
        {
            let generic_parameters_msgs: Vec<_> = generic_parameter_infos
                .iter()
                .map(|x| x.to_string())
                .collect();
            let generic_parameters_msgs_joined = generic_parameters_msgs.join(", ");
            format!("<{generic_parameters_msgs_joined}>")
        }
        _ => String::new(),
    };
    return result;
}
