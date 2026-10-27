use crate::args::*;
use crate::infrastructure::sequence::*;
use crate::infrastructure::*;
use crate::times::*;

pub(crate) fn panic_unused_configs_error<TMock>(
    calls_vertification_results: Vec<CallsVerificationResult<TMock>>,
) {
    let configs_fmts: Vec<_> = calls_vertification_results
        .into_iter()
        .flat_map(|x| {
            let config_fmts: Vec<_> = x
                .unused_configs
                .into_iter()
                .map(|rc_config| {
                    let config = rc_config.borrow();
                    let args_checker_fmt = config.args_checker.fmt_args();
                    let generics_info = config.args_checker.get_generic_parameter_infos();
                    let generics_info_str = fmt_generic_parameter_infos(&generics_info);
                    let times_fmt = format_times(config.return_value_sources.len().times());
                    let fn_fmt = format!(
                        "{}{generics_info_str}({})",
                        x.formatted_fn_name, args_checker_fmt
                    );
                    return (fn_fmt, times_fmt);
                })
                .collect();
            return config_fmts;
        })
        .collect();
    let Some(longest_line_length) = configs_fmts.iter().map(|x| x.0.len()).max() else {
        panic!(
            "[ERROR] `panic_unused_configs_error` must receive non empty `VerifyCallsResult` vector, but received vector was empty."
        )
    };
    let fmts_with_time: Vec<_> = configs_fmts
        .into_iter()
        .map(|(fn_fmt, times_fmt)| {
            format!(
                "{fn_fmt}{} {}",
                " ".repeat(longest_line_length - fn_fmt.len()),
                times_fmt
            )
        })
        .collect();
    let result = fmts_with_time.join("\n\t");
    panic!("Expected following calls:\n\t{result}");
}

pub(crate) fn panic_no_suitable_fn_configuration_found(
    fn_name: &str,
    formatted_fn_name: &str,
    unexpected_call: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
    matching_config_search_err: MatchingConfigSearchErr,
) -> ! {
    let call_msg =
        format_unexpected_call(formatted_fn_name, unexpected_call, generic_parameter_infos);
    let calls = matching_config_search_err
        .args_check_results_sorted_by_number_of_correctly_matched_args_descending;
    let configs_report = if !calls.calls_args_check_results.is_empty() {
        let args_check_results_msgs: Vec<_> = calls
            .calls_args_check_results
            .into_iter()
            .enumerate()
            .map(|(i, args_check_result)| {
                let number = i + 1;
                let matched_arguments_count = args_check_result.iter().filter(|x| x.is_ok()).count();
                let total_arguments_count = args_check_result.len();
                let args_msg = fmt_fn_parameters_msg(fn_name, args_check_result);
                return format!("{number}. Matched {matched_arguments_count}/{total_arguments_count} arguments: {args_msg}");
            })
            .collect();
        let args_check_results_msg = args_check_results_msgs.join("\n\t");
        format!(
            "
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
\t{args_check_results_msg}"
        )
    } else {
        String::new()
    };
    let error_msg = format!(
        "Mock wasn't configured to handle following call:
\t{call_msg}{configs_report}"
    );
    panic!("{error_msg}");
}

#[cfg_attr(test, automock::mock)]
pub(crate) fn format_unexpected_call(
    formatted_fn_name: &str,
    call_args: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
) -> String {
    let call_args_msgs: Vec<_> = call_args
        .into_iter()
        .map(|call_arg| call_arg.clone_arg_debug_string())
        .collect();
    let call_args_msg = call_args_msgs.join(", ");
    let generic_parameters_msg = fmt_generic_parameter_infos(&generic_parameter_infos);
    let error_msg = format!("{formatted_fn_name}{generic_parameters_msg}({call_args_msg})");
    return error_msg;
}

pub(crate) fn panic_return_values_exhausted(
    formatted_fn_name: &str,
    call_args: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
) -> ! {
    let call_msg = format_unexpected_call(formatted_fn_name, call_args, generic_parameter_infos);
    let error_msg = format!("Return values for following call have been exhausted: {call_msg}");
    panic!("{error_msg}");
}

pub(crate) fn panic_invalid_calls_order(mut actual_calls_orders: Vec<CallOrderEntry>) -> ! {
    let actual_order_string = fmt_call_order_entries(&actual_calls_orders);
    let expected_calls_order = {
        actual_calls_orders.sort_by_key(|a| a.call_order_number);
        actual_calls_orders
    };
    let expected_order_string = fmt_call_order_entries(&expected_calls_order);
    let error_msg = format!(
        "Expected to receive these calls in order:

\t{expected_order_string}

Actually received matching calls in this order:

\t{actual_order_string}
"
    );
    panic!("{error_msg}")
}

fn fmt_call_order_entries(call_order_entries: &[CallOrderEntry]) -> String {
    let formatted_strings: Vec<_> = call_order_entries
        .iter()
        .map(|x| x.formatted_string.clone())
        .collect();
    let string = formatted_strings.join("\n\t");
    return string;
}

fn format_times(times: Times) -> String {
    let result = match times.0 {
        1 => "1 time".to_owned(),
        x => format!("{x} times"),
    };
    return result;
}
