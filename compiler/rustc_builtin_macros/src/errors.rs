use rustc_macros::SessionDiagnostic;
use rustc_span::{Span, Symbol};

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_args_after_clobber_abi)]
pub(crate) struct AsmArgsAfterClobberAbi {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(builtin_macros::abi_label)]
    pub(crate) abi_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_args_after_options)]
pub(crate) struct AsmArgsAfterOptions {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(builtin_macros::previous_options_label)]
    pub(crate) options_spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_args_named_after_explicit_register)]
pub(crate) struct AsmArgsNamedAfterExplicitRegister {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(builtin_macros::register_label)]
    pub(crate) register_spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_args_positional_after_named_or_explicit_register)]
pub(crate) struct AsmArgsPositionalAfterNamedOrExplicitRegister {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(builtin_macros::named_label)]
    pub(crate) named_spans: Vec<Span>,
    #[label(builtin_macros::register_label)]
    pub(crate) register_spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_cannot_be_used_with)]
pub(crate) struct AsmCannotBeUsedWith {
    pub(crate) left: &'static str,
    pub(crate) right: &'static str,
    #[primary_span]
    pub(crate) spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_clobber_abi_after_options)]
pub(crate) struct AsmClobberAbiAfterOptions {
    #[primary_span]
    pub(crate) span: Span,
    #[label(builtin_macros::options_label)]
    pub(crate) options_spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_clobber_abi_needs_an_abi)]
pub(crate) struct AsmClobberAbiNeedsAnAbi {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_clobber_abi_needs_explicit_registers)]
pub(crate) struct AsmClobberAbiNeedsExplicitRegisters {
    #[primary_span]
    #[label]
    pub(crate) spans: Vec<Span>,
    #[label(builtin_macros::abi_label)]
    pub(crate) abi_spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_duplicate_argument)]
pub(crate) struct AsmDuplicateArgument {
    pub(crate) name: Symbol,
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(builtin_macros::previously_label)]
    pub(crate) prev_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_duplicate_option)]
pub(crate) struct AsmDuplicateOption {
    pub(crate) symbol: Symbol,
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    // TODO: This should be a `tool_only_span_suggestion`
    #[suggestion(code = "", applicability = "machine-applicable")]
    pub(crate) full_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_expected_operand_options_or_template_string)]
pub(crate) struct AsmExpectedOperandOptionsOrTemplateString {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_expected_operand_clobber_abi_options_or_template_string)]
pub(crate) struct AsmExpectedOperandClobberAbiOptionsOrTemplateString {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_expected_path_arg_to_sym)]
pub(crate) struct AsmExpectedPathArgToSym {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_expected_register_class_or_explicit_register)]
pub(crate) struct AsmExpectedRegisterClassOrExplicitRegister {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_expected_string_literal)]
pub(crate) struct AsmExpectedStringLiteral {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_expected_token_comma)]
pub(crate) struct AsmExpectedTokenComma {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_explicit_register_arg_with_name)]
pub(crate) struct AsmExplicitRegisterArgWithName {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_no_argument_named)]
pub(crate) struct AsmNoArgumentNamed {
    pub(crate) name: String,
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_option_must_be_combined_with_either)]
pub(crate) struct AsmOptionMustBeCombinedWithEither {
    pub(crate) option: &'static str,
    pub(crate) left: &'static str,
    pub(crate) right: &'static str,
    #[primary_span]
    pub(crate) spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_option_noreturn_with_outputs)]
pub(crate) struct AsmOptionNoreturnWithOutputs {
    #[primary_span]
    pub(crate) spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_option_pure_needs_one_output)]
pub(crate) struct AsmOptionPureNeedsOneOutput {
    #[primary_span]
    pub(crate) spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_options_mutually_exclusive)]
pub(crate) struct AsmOptionsMutuallyExclusive {
    pub(crate) left: &'static str,
    pub(crate) right: &'static str,
    #[primary_span]
    pub(crate) spans: Vec<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_requires_template_string_arg)]
pub(crate) struct AsmRequiresTemplateStringArg {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_template_modifier_single_char)]
pub(crate) struct AsmTemplateModifierSingleChar {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::asm_underscore_for_input_operands)]
pub(crate) struct AsmUnderscoreForInputOperands {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::boolean_expression_required)]
pub struct BooleanExpressionRequired {
    #[primary_span]
    pub span: Span,
}

// let mut err = cx.struct_span_err(parser.token.span, "unexpected string literal");
// let comma_span = parser.prev_token.span.shrink_to_hi();
// err.span_suggestion_short(
//     comma_span,
//     "try adding a comma",
//     ", ",
//     Applicability::MaybeIncorrect,
// );

// FIXME:
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::unexpected_string_literal)]
pub struct UnexpectedStringLiteral {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::argument_expression_required)]
pub struct ArgumentExpressionRequired {
    #[primary_span]
    #[suggestion(code = "", applicability = "maybe-incorrect")]
    pub span: Span,
}

// Some([]) => ecx.span_err(mi.span, "`cfg_accessible` path is not specified"),
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::not_specified)]
pub struct NotSpecified {
    #[primary_span]
    pub span: Span,
}

//ecx.span_err(l.span(), "multiple `cfg_accessible` paths are specified");
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::multiple_paths_specified)]
pub struct MultiplePathsSpecified {
    #[primary_span]
    pub span: Span,
}

// None => ecx.span_err(nmi.span(), "`cfg_accessible` path cannot be a literal"),
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::unallowed_literal_path)]
pub struct UnallowedLiteralPath {
    #[primary_span]
    pub span: Span,
}

//         ecx.span_err(mi.span, "`cfg_accessible` path cannot accept arguments");
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::unaccepted_arguments)]
pub struct UnacceptedArguments {
    #[primary_span]
    pub span: Span,
}

//ecx.span_err(span, "cannot determine whether the path is accessible or not");
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::nondeterministic_access)]
pub struct NondeterministicAccess {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::compile_error)]
pub struct CompileError {
    #[primary_span]
    pub span: Span,
    pub msg: String,
}

// cx.span_err(e.span, "cannot concatenate a byte string literal");
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::byte_string_literal_concatenate)]
pub struct ByteStringLiteralConcatenate {
    #[primary_span]
    pub span: Span,
}

// let mut err = cx.struct_span_err(missing_literal, "expected a literal");
// err.note("only literals (like `\"foo\"`, `42` and `3.14`) can be passed to `concat!()`");
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::missing_literal)]
#[note]
pub struct MissingLiteral {
    #[primary_span]
    pub span: Span,
}

// let mut err = cx.create_err(expr.span, "cannot concatenate character literals");
//             if let Ok(snippet) = cx.sess.source_map().span_to_snippet(expr.span) {
//                 err.span_suggestion(
//                     expr.span,
//                     "try using a byte character",
//                     format!("b{}", snippet),
//                     Applicability::MachineApplicable,
//                 )
//                 .emit();
//             }
#[derive(SessionDiagnostic)]
#[diag(builtin_macros::character_literals_concatenate)]
pub struct CharacterLiteralsConcatenate {
    #[primary_span]
    #[suggestion(code = "{snippet}", applicability = "machine-applicable")]
    pub span: Span,
    pub snippet: Option<String>,
}

// let mut err = cx.struct_span_err(expr.span, "cannot concatenate string literals");
//             // suggestion would be invalid if we are nested
//             if !is_nested {
//                 if let Ok(snippet) = cx.sess.source_map().span_to_snippet(expr.span) {
//                     err.span_suggestion(
//                         expr.span,
//                         "try using a byte string",
//                         format!("b{}", snippet),
//                         Applicability::MachineApplicable,
//                     );
//                 }
//             }

#[derive(SessionDiagnostic)]
#[diag(builtin_macros::string_literals_concatenate)]
pub struct StringLiteralsConcatenate {
    #[primary_span]
    #[suggestion(code = "{snippet}", applicability = "machine-applicable")]
    pub span: Span,
    pub snippet: Option<String>,
}
