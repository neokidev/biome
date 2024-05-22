use std::collections::HashSet;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_css_syntax::{CssDeclarationOrRuleList, CssGenericProperty, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNodeCast, TextRange};

use crate::utils::{get_longhand_sub_properties, get_reset_to_initial_properties};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding stylelint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   color: red;
    /// }
    /// ```
    ///
    pub NoDeclarationBlockShorthandPropertyOverrides {
        version: "next",
        name: "noDeclarationBlockShorthandPropertyOverrides",
        language: "css",
        recommended: true,
    }
}

fn get_css_declaration_list(property: &CssGenericProperty) -> Option<CssDeclarationOrRuleList> {
    for ancestor in property.syntax().ancestors() {
        if matches!(ancestor.kind(), CssSyntaxKind::CSS_DECLARATION_OR_RULE_LIST) {
            return Some(ancestor.cast::<CssDeclarationOrRuleList>()?);
        }
    }

    return None;
}

fn get_prior_property_names_in_block(
    target_property: &CssGenericProperty,
) -> Option<HashSet<String>> {
    let declaration_list = get_css_declaration_list(target_property)?;

    let mut prior_declarations = HashSet::new();
    for declaration in declaration_list {
        if let Some(declaration) = declaration.as_css_declaration_with_semicolon() {
            let current_property = declaration.declaration().ok()?.property().ok()?;

            let current_property =
                if let Some(current_property) = current_property.as_css_generic_property() {
                    current_property
                } else {
                    continue;
                };

            if current_property == target_property {
                break;
            }

            let current_property_name = current_property.name().ok()?.text().to_lowercase();
            prior_declarations.insert(current_property_name);
        }
    }

    Some(prior_declarations)
}

pub struct NoDeclarationBlockShorthandPropertyOverridesState {
    target_property_name: String,
    override_property_name: String,
    span: TextRange,
}

impl Rule for NoDeclarationBlockShorthandPropertyOverrides {
    type Query = Ast<CssGenericProperty>;
    type State = NoDeclarationBlockShorthandPropertyOverridesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        let target_property_name_node = node.name().ok()?;
        let target_property_name = target_property_name_node.text().to_lowercase();

        let prior_property_names = get_prior_property_names_in_block(node)?;
        let longhand_sub_properties = get_longhand_sub_properties(&target_property_name);
        let reset_to_initial_properties = get_reset_to_initial_properties(&target_property_name);

        for prior_property_name in prior_property_names {
            if longhand_sub_properties.contains(&prior_property_name.as_str())
                || reset_to_initial_properties.contains(&prior_property_name.as_str())
            {
                return Some(NoDeclarationBlockShorthandPropertyOverridesState {
                    target_property_name,
                    override_property_name: prior_property_name,
                    span: target_property_name_node.range(),
                });
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.span,
            markup! {
                "Unexpected shorthand property "<Emphasis>{state.target_property_name}</Emphasis>" after "<Emphasis>{state.override_property_name}</Emphasis>
            },
        ))
    }
}