use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    global_identifier, AnyJsExpression, JsCallExpression, JsNewExpression, JsNewOrCallExpression, T,
};
use biome_rowan::{chain_trivia_pieces, AstNode, BatchMutationExt, TriviaPieceKind};

declare_rule! {
    /// Enforce the use of new for all builtins, except `String`, `Number`, `Boolean`, `Symbol` and `BigInt`.
    ///
    /// `new Builtin()` and `Builtin()` work the same, but new should be preferred for consistency with other constructors.
    /// Enforces the use of new for following builtins:
    ///
    /// - Object
    /// - Array
    /// - ArrayBuffer
    /// - BigInt64Array
    /// - BigUint64Array
    /// - DataView
    /// - Date
    /// - Error
    /// - Float32Array
    /// - Float64Array
    /// - Function
    /// - Int8Array
    /// - Int16Array
    /// - Int32Array
    /// - Map
    /// - WeakMap
    /// - Set
    /// - WeakSet
    /// - Promise
    /// - RegExp
    /// - Uint8Array
    /// - Uint16Array
    /// - Uint32Array
    /// - Uint8ClampedArray
    /// - SharedArrayBuffer
    /// - Proxy
    /// - WeakRef
    /// - FinalizationRegistry
    ///
    /// Disallows the use of new for following builtins:
    ///
    /// - String
    /// - Number
    /// - Boolean
    ///
    /// These builtins are handled by [noInvalidBuiltin](https://biomejs.dev/linter/rules/no-invalid-new-builtin/) rule:
    ///
    /// - Symbol
    /// - BigInt
    ///
    /// > These should not use new as that would create object wrappers for the primitive values, which is not what you want. However, without new they can be useful for coercing a value to that type.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const text = new String(10);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const now = Date();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const map = Map([
    ///   ['foo', 'bar']
    /// ]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const text = String(10);
    /// ```
    ///
    /// ```js
    /// const now = new Date();
    /// ```
    ///
    /// ```js
    /// const map = new Map([
    ///  ['foo', 'bar']
    /// ]);
    /// ```
    ///
    pub UseConsistentNewBuiltin {
        version: "next",
        name: "useConsistentNewBuiltin",
        sources: &[RuleSource::EslintUnicorn("new-for-builtins"), RuleSource::Eslint("no-new-wrappers")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseConsistentNewBuiltin {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = UseConsistentNewBuiltinState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let (callee, creation_rule) = extract_callee_and_rule(node)?;
        let (reference, name) = global_identifier(&callee.omit_parentheses())?;

        let name_text = name.text();

        if creation_rule
            .forbidden_builtins_list()
            .binary_search(&name_text)
            .is_ok()
        {
            return ctx.model().binding(&reference).is_none().then_some(
                UseConsistentNewBuiltinState {
                    name: name_text.to_string(),
                    creation_rule,
                },
            );
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let name = &state.name;

        let (use_this, instead_of) = match state.creation_rule {
            BuiltinCreationRule::MustUseNew => ("new ", ""),
            BuiltinCreationRule::MustNotUseNew => ("", "new "),
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Use "<Emphasis>{use_this}{name}"()"</Emphasis>" instead of "<Emphasis>{instead_of}{name}"()"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let mut mutation = ctx.root().begin();

        match node {
            JsNewOrCallExpression::JsNewExpression(node) => {
                let call_expression = convert_new_expression_to_call_expression(node)?;

                mutation
                    .replace_node::<AnyJsExpression>(node.clone().into(), call_expression.into());
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Remove "<Emphasis>"new"</Emphasis>" keyword." }.to_owned(),
                    mutation,
                })
            }
            JsNewOrCallExpression::JsCallExpression(node) => {
                let new_expression = convert_call_expression_to_new_expression(node)?;

                mutation
                    .replace_node::<AnyJsExpression>(node.clone().into(), new_expression.into());
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Add "<Emphasis>"new"</Emphasis>" keyword." }.to_owned(),
                    mutation,
                })
            }
        }
    }
}

/// Sorted array of builtins that require new keyword.
const BUILTINS_REQUIRING_NEW: &[&str] = &[
    "Array",
    "ArrayBuffer",
    "BigInt64Array",
    "BigUint64Array",
    "DataView",
    "Date",
    "Error",
    "FinalizationRegistry",
    "Float32Array",
    "Float64Array",
    "Function",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Map",
    "Object",
    "Promise",
    "Proxy",
    "RegExp",
    "Set",
    "SharedArrayBuffer",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "WeakMap",
    "WeakRef",
    "WeakSet",
];

/// Sorted array of builtins that should not use new keyword.
const BUILTINS_NOT_REQUIRING_NEW: &[&str] = &["Boolean", "Number", "String"];

enum BuiltinCreationRule {
    MustUseNew,
    MustNotUseNew,
}

impl BuiltinCreationRule {
    fn forbidden_builtins_list(&self) -> &[&str] {
        match self {
            BuiltinCreationRule::MustUseNew => BUILTINS_REQUIRING_NEW,
            BuiltinCreationRule::MustNotUseNew => BUILTINS_NOT_REQUIRING_NEW,
        }
    }
}

pub struct UseConsistentNewBuiltinState {
    name: String,
    creation_rule: BuiltinCreationRule,
}

fn extract_callee_and_rule(
    node: &JsNewOrCallExpression,
) -> Option<(AnyJsExpression, BuiltinCreationRule)> {
    match node {
        JsNewOrCallExpression::JsNewExpression(node) => {
            let callee = node.callee().ok()?;

            Some((callee, BuiltinCreationRule::MustNotUseNew))
        }
        JsNewOrCallExpression::JsCallExpression(node) => {
            let callee: AnyJsExpression = node.callee().ok()?;

            Some((callee, BuiltinCreationRule::MustUseNew))
        }
    }
}

fn convert_new_expression_to_call_expression(expr: &JsNewExpression) -> Option<JsCallExpression> {
    let new_token = expr.new_token().ok()?;
    let mut callee = expr.callee().ok()?;
    if new_token.has_leading_comments() || new_token.has_trailing_comments() {
        callee = callee.prepend_trivia_pieces(chain_trivia_pieces(
            new_token.leading_trivia().pieces(),
            new_token.trailing_trivia().pieces(),
        ))?;
    }
    Some(make::js_call_expression(callee, expr.arguments()?).build())
}

fn convert_call_expression_to_new_expression(expr: &JsCallExpression) -> Option<JsNewExpression> {
    let mut callee = expr.callee().ok()?;
    let leading_trivia_pieces = callee.syntax().first_leading_trivia()?.pieces();

    let new_token = make::token(T![new])
        .with_leading_trivia_pieces(leading_trivia_pieces)
        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);

    callee = callee.with_leading_trivia_pieces([])?;

    Some(
        make::js_new_expression(new_token, callee)
            .with_arguments(expr.arguments().ok()?)
            .build(),
    )
}

#[test]
fn test_order() {
    for items in BUILTINS_REQUIRING_NEW.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in BUILTINS_NOT_REQUIRING_NEW.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
