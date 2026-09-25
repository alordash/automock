use crate::args::*;
use crate::fn_parameters::*;
use crate::infrastructure::MatchingConfigSearchErr;
use crate::infrastructure::call_order_verification::CallOrderEntry;
use crate::settings::*;
use crate::times::*;

pub(crate) fn panic_received_verification_error(
    fn_name: &str,
    formatted_fn_name: &str,
    args_checker: &DynArgsChecker,
    matching_calls_check_result: OrderedCallsCheckResult,
    non_matching_calls_check_result: OrderedCallsCheckResult,
    times: Times,
) -> ! {
    let matching_calls_count = matching_calls_check_result.calls_args_check_results.len();

    let generic_parameter_infos = args_checker.get_generic_parameter_infos();
    let generic_parameters_msg = fmt_generic_parameter_infos(
        GenericParameterInfosFormattingPolicy::Perform(&generic_parameter_infos),
    );
    let expected_call_msg = format!(
        "\t{formatted_fn_name}{}({})",
        generic_parameters_msg,
        args_checker.fmt_args(),
    );
    let matching_calls_report = if matching_calls_count == 0 {
        "Actually received no matching calls".to_string()
    } else {
        let max_invalid_calls_listed_count = read_settings().max_invalid_calls_listed_count;
        let matching_calls_args_msgs: Vec<_> = matching_calls_check_result
            .calls_args_check_results
            .into_iter()
            .take(max_invalid_calls_listed_count)
            .map(|x| {
                fmt_call(
                    fn_name,
                    x.args_check_results,
                    GenericParameterInfosFormattingPolicy::Skip,
                )
            })
            .collect();
        let trimmed_output_disclaimer = if matching_calls_count > max_invalid_calls_listed_count {
            format!(" (listing only first {})", max_invalid_calls_listed_count)
        } else {
            String::new()
        };
        let matching_calls_args_msg = matching_calls_args_msgs.join("\n\t");
        let call_fmt = fmt_calls(matching_calls_count);
        format!(
            "Actually received {matching_calls_count} matching {call_fmt}{trimmed_output_disclaimer}:
\t{matching_calls_args_msg}"
        )
    };
    let non_matching_calls_count = non_matching_calls_check_result
        .calls_args_check_results
        .len();
    let non_matching_calls_report = if non_matching_calls_count == 0 {
        "Received no non-matching calls".to_string()
    } else {
        let max_invalid_calls_listed_count = read_settings().max_invalid_calls_listed_count;
        let call_fmt = fmt_calls(non_matching_calls_count);
        let non_matching_calls_args_msgs: Vec<_> = non_matching_calls_check_result
            .calls_args_check_results
            .into_iter()
            .take(max_invalid_calls_listed_count)
            .map(|x| {
                fmt_call(
                    fn_name,
                    x.args_check_results,
                    GenericParameterInfosFormattingPolicy::Skip,
                )
            })
            .collect();
        let trimmed_output_disclaimer = if non_matching_calls_count > max_invalid_calls_listed_count
        {
            format!(" (listing only first {})", max_invalid_calls_listed_count)
        } else {
            String::new()
        };
        let non_matching_calls_args_msg = non_matching_calls_args_msgs.join("\n");
        format!(
            "Received {non_matching_calls_count} non-matching {call_fmt}{trimmed_output_disclaimer} (non-matching arguments indicated with '*' characters):
{non_matching_calls_args_msg}"
        )
    };
    let times_format = format_times(times);
    let error_msg = format!(
        r"{times_format} matching:
{expected_call_msg}
{matching_calls_report}
{non_matching_calls_report}"
    );
    panic!("{error_msg}");
}

pub(crate) fn panic_no_suitable_fn_configuration_found(
    fn_name: &str,
    formatted_fn_name: &str,
    unexpected_call: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
    matching_config_search_err: MatchingConfigSearchErr,
) -> ! {
    let call_msg = format_received_unexpected_call_error(
        formatted_fn_name,
        unexpected_call,
        generic_parameter_infos,
    );
    let calls = matching_config_search_err
        .args_check_results_sorted_by_number_of_correctly_matched_args_descending;
    let needed_return_value = matching_config_search_err.needed_return_value;
    let configs_report = if !calls.calls_args_check_results.is_empty() {
        let args_check_results_msgs: Vec<_> = calls
            .calls_args_check_results
            .into_iter()
            .enumerate()
            .map(|(i, args_check_result)| {
                let number = i + 1;
                let matched_arguments_count = args_check_result.iter().filter(|x| x.is_ok()).count();
                let total_arguments_count = args_check_result.len();
                let args_msg = fmt_fn_parameters_msg(fn_name, args_check_result, GenericParameterInfosFormattingPolicy::Skip);
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
    let needed_return_value_msg = if needed_return_value {
        " because no return value was provided"
    } else {
        ""
    };
    let error_msg = format!(
        "Mock wasn't configured to handle following call{needed_return_value_msg}:
\t{call_msg}{configs_report}"
    );
    panic!("{error_msg}");
}

pub(crate) fn format_received_unexpected_call_error(
    formatted_fn_name: &str,
    call_args: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
) -> String {
    let call_args_msgs: Vec<_> = call_args
        .into_iter()
        .map(|call_arg| call_arg.clone_arg_debug_string())
        .collect();
    let call_args_msg = call_args_msgs.join(", ");
    let generic_parameters_msg = fmt_generic_parameter_infos(
        GenericParameterInfosFormattingPolicy::Perform(&generic_parameter_infos),
    );
    let error_msg = format!("{formatted_fn_name}{generic_parameters_msg}({call_args_msg})");
    return error_msg;
}

pub(crate) fn panic_received_unexpected_calls_error(error_msgs: Vec<String>) -> ! {
    let unexpected_calls_count = error_msgs.len();
    let call_fmt = fmt_calls(unexpected_calls_count);
    let max_invalid_calls_listed_count = read_settings().max_invalid_calls_listed_count;
    let unexpected_calls_msgs: Vec<_> = error_msgs
        .into_iter()
        .take(max_invalid_calls_listed_count)
        .enumerate()
        .map(|(i, error_msg)| {
            let error_number = i + 1;
            return format!("{error_number}. {error_msg}");
        })
        .collect();
    let unexpected_calls_msg = unexpected_calls_msgs.join("\n");
    let trimmed_output_disclaimer = if unexpected_calls_count > max_invalid_calls_listed_count {
        format!(" (listing only first {})", max_invalid_calls_listed_count)
    } else {
        String::new()
    };
    let error_msg = format!(
        "Did not expect to receive any other calls. Received {unexpected_calls_count} unexpected {call_fmt}{trimmed_output_disclaimer}:
{unexpected_calls_msg}"
    );
    panic!("{error_msg}");
}

pub(crate) fn panic_no_return_value_was_configured(
    formatted_fn_name: &str,
    call_args: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
) -> ! {
    let call_msg = format_received_unexpected_call_error(
        formatted_fn_name,
        call_args,
        generic_parameter_infos,
    );
    let error_msg = format!("No return value found for following call: {call_msg}");
    panic!("{error_msg}");
}

fn fmt_calls(calls_count: usize) -> &'static str {
    assert_ne!(calls_count, 0);
    return if calls_count == 1 { "call" } else { "calls" };
}

pub(crate) fn panic_invalid_calls_order(expected_calls_order: &mut [CallOrderEntry]) -> ! {
    let expected_order_string = fmt_call_order_entries(expected_calls_order);
    let actual_calls_order = {
        expected_calls_order.sort_by_key(|a| a.call_order_number);
        expected_calls_order
    };
    let actual_order_string = fmt_call_order_entries(actual_calls_order);
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
    let result = match times {
        Times::Never | Times::Exactly(0) => "Expected to never receive a call".to_owned(),
        Times::Once | Times::Exactly(1) => "Expected to receive a call exactly once".to_owned(),
        Times::Exactly(exact_count) => {
            format!("Expected to receive a call {exact_count} times")
        }
        #[allow(deprecated)]
        Times::Any => "Expected to receive a call any number of times.".to_owned(),
    };
    return result;
}
