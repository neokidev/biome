// This file contains the list of all diagnostic categories for the Biome
// toolchain
//
// The `define_categories` macro is preprocessed in the build script for the
// crate in order to generate the static registry. The body of the macro
// consists of a list of key-value pairs defining the categories that have an
// associated hyperlink, then a list of string literals defining the remaining
// categories without a link.

// PLEASE, DON'T EDIT THIS FILE BY HAND.
// Use `just new-lintrule` to create a new rule.
// lint rules are lexicographically sorted and
// must be between `define_categories! {\n` and `\n    ;\n`.

define_categories! {
    "lint/a11y/noAccessKey": "https://biomejs.dev/linter/rules/no-access-key",
    "lint/a11y/noAriaHiddenOnFocusable": "https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable",
    "lint/a11y/noAriaUnsupportedElements": "https://biomejs.dev/linter/rules/no-aria-unsupported-elements",
    "lint/a11y/noAutofocus": "https://biomejs.dev/linter/rules/no-autofocus",
    "lint/a11y/noBlankTarget": "https://biomejs.dev/linter/rules/no-blank-target",
    "lint/a11y/noDistractingElements": "https://biomejs.dev/linter/rules/no-distracting-elements",
    "lint/a11y/noHeaderScope": "https://biomejs.dev/linter/rules/no-header-scope",
    "lint/a11y/noInteractiveElementToNoninteractiveRole": "https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role",
    "lint/a11y/noNoninteractiveElementToInteractiveRole": "https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role",
    "lint/a11y/noNoninteractiveTabindex": "https://biomejs.dev/linter/rules/no-noninteractive-tabindex",
    "lint/a11y/noPositiveTabindex": "https://biomejs.dev/linter/rules/no-positive-tabindex",
    "lint/a11y/noRedundantAlt": "https://biomejs.dev/linter/rules/no-redundant-alt",
    "lint/a11y/noRedundantRoles": "https://biomejs.dev/linter/rules/no-redundant-roles",
    "lint/a11y/noSvgWithoutTitle": "https://biomejs.dev/linter/rules/no-svg-without-title",
    "lint/a11y/useAltText": "https://biomejs.dev/linter/rules/use-alt-text",
    "lint/a11y/useAnchorContent": "https://biomejs.dev/linter/rules/use-anchor-content",
    "lint/a11y/useAriaActivedescendantWithTabindex": "https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex",
    "lint/a11y/useAriaPropsForRole": "https://biomejs.dev/linter/rules/use-aria-props-for-role",
    "lint/a11y/useButtonType": "https://biomejs.dev/linter/rules/use-button-type",
    "lint/a11y/useHeadingContent": "https://biomejs.dev/linter/rules/use-heading-content",
    "lint/a11y/useHtmlLang": "https://biomejs.dev/linter/rules/use-html-lang",
    "lint/a11y/useIframeTitle": "https://biomejs.dev/linter/rules/use-iframe-title",
    "lint/a11y/useKeyWithClickEvents": "https://biomejs.dev/linter/rules/use-key-with-click-events",
    "lint/a11y/useKeyWithMouseEvents": "https://biomejs.dev/linter/rules/use-key-with-mouse-events",
    "lint/a11y/useMediaCaption": "https://biomejs.dev/linter/rules/use-media-caption",
    "lint/a11y/useValidAnchor": "https://biomejs.dev/linter/rules/use-valid-anchor",
    "lint/a11y/useValidAriaProps": "https://biomejs.dev/linter/rules/use-valid-aria-props",
    "lint/a11y/useValidAriaRole": "https://biomejs.dev/linter/rules/use-valid-aria-role",
    "lint/a11y/useValidAriaValues": "https://biomejs.dev/linter/rules/use-valid-aria-values",
    "lint/a11y/useValidLang": "https://biomejs.dev/linter/rules/use-valid-lang",
    "lint/complexity/noBannedTypes": "https://biomejs.dev/linter/rules/no-banned-types",
    "lint/complexity/noEmptyTypeParameters": "https://biomejs.dev/linter/rules/no-empty-type-parameters",
    "lint/complexity/noExcessiveCognitiveComplexity": "https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity",
    "lint/complexity/noExcessiveNestedTestSuites": "https://biomejs.dev/linter/rules/no-excessive-nested-test-suites",
    "lint/complexity/noExtraBooleanCast": "https://biomejs.dev/linter/rules/no-extra-boolean-cast",
    "lint/complexity/noForEach": "https://biomejs.dev/linter/rules/no-for-each",
    "lint/complexity/noMultipleSpacesInRegularExpressionLiterals": "https://biomejs.dev/linter/rules/no-multiple-spaces-in-regular-expression-literals",
    "lint/complexity/noStaticOnlyClass": "https://biomejs.dev/linter/rules/no-static-only-class",
    "lint/complexity/noThisInStatic": "https://biomejs.dev/linter/rules/no-this-in-static",
    "lint/complexity/noUselessCatch": "https://biomejs.dev/linter/rules/no-useless-catch",
    "lint/complexity/noUselessConstructor": "https://biomejs.dev/linter/rules/no-useless-constructor",
    "lint/complexity/noUselessEmptyExport": "https://biomejs.dev/linter/rules/no-useless-empty-export",
    "lint/complexity/noUselessFragments": "https://biomejs.dev/linter/rules/no-useless-fragments",
    "lint/complexity/noUselessLabel": "https://biomejs.dev/linter/rules/no-useless-label",
    "lint/complexity/noUselessLoneBlockStatements": "https://biomejs.dev/linter/rules/no-useless-lone-block-statements",
    "lint/complexity/noUselessRename": "https://biomejs.dev/linter/rules/no-useless-rename",
    "lint/complexity/noUselessSwitchCase": "https://biomejs.dev/linter/rules/no-useless-switch-case",
    "lint/complexity/noUselessTernary": "https://biomejs.dev/linter/rules/no-useless-ternary",
    "lint/complexity/noUselessThisAlias": "https://biomejs.dev/linter/rules/no-useless-this-alias",
    "lint/complexity/noUselessTypeConstraint": "https://biomejs.dev/linter/rules/no-useless-type-constraint",
    "lint/complexity/noVoid": "https://biomejs.dev/linter/rules/no-void",
    "lint/complexity/noWith": "https://biomejs.dev/linter/rules/no-with",
    "lint/complexity/useArrowFunction": "https://biomejs.dev/linter/rules/use-arrow-function",
    "lint/complexity/useFlatMap": "https://biomejs.dev/linter/rules/use-flat-map",
    "lint/complexity/useLiteralKeys": "https://biomejs.dev/linter/rules/use-literal-keys",
    "lint/complexity/useOptionalChain": "https://biomejs.dev/linter/rules/use-optional-chain",
    "lint/complexity/useRegexLiterals": "https://biomejs.dev/linter/rules/use-regex-literals",
    "lint/complexity/useSimpleNumberKeys": "https://biomejs.dev/linter/rules/use-simple-number-keys",
    "lint/complexity/useSimplifiedLogicExpression": "https://biomejs.dev/linter/rules/use-simplified-logic-expression",
    "lint/correctness/noChildrenProp": "https://biomejs.dev/linter/rules/no-children-prop",
    "lint/correctness/noConstAssign": "https://biomejs.dev/linter/rules/no-const-assign",
    "lint/correctness/noConstantCondition": "https://biomejs.dev/linter/rules/no-constant-condition",
    "lint/correctness/noConstructorReturn": "https://biomejs.dev/linter/rules/no-constructor-return",
    "lint/correctness/noEmptyCharacterClassInRegex": "https://biomejs.dev/linter/rules/no-empty-character-class-in-regex",
    "lint/correctness/noEmptyPattern": "https://biomejs.dev/linter/rules/no-empty-pattern",
    "lint/correctness/noGlobalObjectCalls": "https://biomejs.dev/linter/rules/no-global-object-calls",
    "lint/correctness/noInnerDeclarations": "https://biomejs.dev/linter/rules/no-inner-declarations",
    "lint/correctness/noInvalidConstructorSuper": "https://biomejs.dev/linter/rules/no-invalid-constructor-super",
    "lint/correctness/noInvalidNewBuiltin": "https://biomejs.dev/linter/rules/no-invalid-new-builtin",
    "lint/correctness/noInvalidUseBeforeDeclaration": "https://biomejs.dev/linter/rules/no-invalid-use-before-declaration",
    "lint/correctness/noNewSymbol": "https://biomejs.dev/linter/rules/no-new-symbol",
    "lint/correctness/noNonoctalDecimalEscape": "https://biomejs.dev/linter/rules/no-nonoctal-decimal-escape",
    "lint/correctness/noPrecisionLoss": "https://biomejs.dev/linter/rules/no-precision-loss",
    "lint/correctness/noRenderReturnValue": "https://biomejs.dev/linter/rules/no-render-return-value",
    "lint/correctness/noSelfAssign": "https://biomejs.dev/linter/rules/no-self-assign",
    "lint/correctness/noSetterReturn": "https://biomejs.dev/linter/rules/no-setter-return",
    "lint/correctness/noStringCaseMismatch": "https://biomejs.dev/linter/rules/no-string-case-mismatch",
    "lint/correctness/noSwitchDeclarations": "https://biomejs.dev/linter/rules/no-switch-declarations",
    "lint/correctness/noUndeclaredVariables": "https://biomejs.dev/linter/rules/no-undeclared-variables",
    "lint/correctness/noUnnecessaryContinue": "https://biomejs.dev/linter/rules/no-unnecessary-continue",
    "lint/correctness/noUnreachable": "https://biomejs.dev/linter/rules/no-unreachable",
    "lint/correctness/noUnreachableSuper": "https://biomejs.dev/docs/linter/rules/no-unreachable-super",
    "lint/correctness/noUnsafeFinally": "https://biomejs.dev/linter/rules/no-unsafe-finally",
    "lint/correctness/noUnsafeOptionalChaining": "https://biomejs.dev/linter/rules/no-unsafe-optional-chaining",
    "lint/correctness/noUnusedImports": "https://biomejs.dev/linter/rules/no-unused-imports",
    "lint/correctness/noUnusedLabels": "https://biomejs.dev/linter/rules/no-unused-labels",
    "lint/correctness/noUnusedPrivateClassMembers": "https://biomejs.dev/linter/rules/no-unused-private-class-members",
    "lint/correctness/noUnusedVariables": "https://biomejs.dev/linter/rules/no-unused-variables",
    "lint/correctness/noVoidElementsWithChildren": "https://biomejs.dev/linter/rules/no-void-elements-with-children",
    "lint/correctness/noVoidTypeReturn": "https://biomejs.dev/linter/rules/no-void-type-return",
    "lint/correctness/useExhaustiveDependencies": "https://biomejs.dev/linter/rules/use-exhaustive-dependencies",
    "lint/correctness/useHookAtTopLevel": "https://biomejs.dev/linter/rules/use-hook-at-top-level",
    "lint/correctness/useIsNan": "https://biomejs.dev/linter/rules/use-is-nan",
    "lint/correctness/useJsxKeyInIterable": "https://biomejs.dev/linter/rules/use-jsx-key-in-iterable",
    "lint/correctness/useValidForDirection": "https://biomejs.dev/linter/rules/use-valid-for-direction",
    "lint/correctness/useYield": "https://biomejs.dev/linter/rules/use-yield",
    "lint/nursery/colorNoInvalidHex": "https://biomejs.dev/linter/rules/color-no-invalid-hex",
    "lint/nursery/noColorInvalidHex": "https://biomejs.dev/linter/rules/no-color-invalid-hex",
    "lint/nursery/noConsole": "https://biomejs.dev/linter/rules/no-console",
    "lint/nursery/noConstantMathMinMaxClamp": "https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp",
    "lint/nursery/noCssEmptyBlock": "https://biomejs.dev/linter/rules/no-css-empty-block",
    "lint/nursery/noDoneCallback": "https://biomejs.dev/linter/rules/no-done-callback",
    "lint/nursery/noDuplicateElseIf": "https://biomejs.dev/linter/rules/no-duplicate-else-if",
    "lint/nursery/noDuplicateFontNames": "https://biomejs.dev/linter/rules/no-font-family-duplicate-names",
    "lint/nursery/noDuplicateJsonKeys": "https://biomejs.dev/linter/rules/no-duplicate-json-keys",
    "lint/nursery/noDuplicateSelectorsKeyframeBlock": "https://biomejs.dev/linter/rules/no-duplicate-selectors-keyframe-block",
    "lint/nursery/noEvolvingAny": "https://biomejs.dev/linter/rules/no-evolving-any",
    "lint/nursery/noFlatMapIdentity": "https://biomejs.dev/linter/rules/no-flat-map-identity",
    "lint/nursery/noMisplacedAssertion": "https://biomejs.dev/linter/rules/no-misplaced-assertion",
    "lint/nursery/noNodejsModules": "https://biomejs.dev/linter/rules/no-nodejs-modules",
    "lint/nursery/noReactSpecificProps": "https://biomejs.dev/linter/rules/no-react-specific-props",
    "lint/nursery/noRestrictedImports": "https://biomejs.dev/linter/rules/no-restricted-imports",
    "lint/nursery/noTypeOnlyImportAttributes": "https://biomejs.dev/linter/rules/no-type-only-import-attributes",
    "lint/nursery/noUndeclaredDependencies": "https://biomejs.dev/linter/rules/no-undeclared-dependencies",
    "lint/nursery/noUnknownUnit": "https://biomejs.dev/linter/rules/no-unknown-unit",
    "lint/nursery/useBiomeSuppressionComment": "https://biomejs.dev/linter/rules/use-biome-suppression-comment",
    "lint/nursery/useImportRestrictions": "https://biomejs.dev/linter/rules/use-import-restrictions",
    "lint/nursery/useSortedClasses": "https://biomejs.dev/linter/rules/use-sorted-classes",
    "lint/performance/noAccumulatingSpread": "https://biomejs.dev/linter/rules/no-accumulating-spread",
    "lint/performance/noBarrelFile": "https://biomejs.dev/linter/rules/no-barrel-file",
    "lint/performance/noDelete": "https://biomejs.dev/linter/rules/no-delete",
    "lint/performance/noReExportAll": "https://biomejs.dev/linter/rules/no-re-export-all",
    "lint/security/noDangerouslySetInnerHtml": "https://biomejs.dev/linter/rules/no-dangerously-set-inner-html",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://biomejs.dev/linter/rules/no-dangerously-set-inner-html-with-children",
    "lint/security/noGlobalEval": "https://biomejs.dev/linter/rules/no-global-eval",
    "lint/style/noArguments": "https://biomejs.dev/linter/rules/no-arguments",
    "lint/style/noCommaOperator": "https://biomejs.dev/linter/rules/no-comma-operator",
    "lint/style/noDefaultExport": "https://biomejs.dev/linter/rules/no-default-export",
    "lint/style/noImplicitBoolean": "https://biomejs.dev/linter/rules/no-implicit-boolean",
    "lint/style/noInferrableTypes": "https://biomejs.dev/linter/rules/no-inferrable-types",
    "lint/style/noNamespace": "https://biomejs.dev/linter/rules/no-namespace",
    "lint/style/noNamespaceImport": "https://biomejs.dev/linter/rules/no-namespace-import",
    "lint/style/noNegationElse": "https://biomejs.dev/linter/rules/no-negation-else",
    "lint/style/noNonNullAssertion": "https://biomejs.dev/linter/rules/no-non-null-assertion",
    "lint/style/noParameterAssign": "https://biomejs.dev/linter/rules/no-parameter-assign",
    "lint/style/noParameterProperties": "https://biomejs.dev/linter/rules/no-parameter-properties",
    "lint/style/noRestrictedGlobals": "https://biomejs.dev/linter/rules/no-restricted-globals",
    "lint/style/noShoutyConstants": "https://biomejs.dev/linter/rules/no-shouty-constants",
    "lint/style/noUnusedTemplateLiteral": "https://biomejs.dev/linter/rules/no-unused-template-literal",
    "lint/style/noUselessElse": "https://biomejs.dev/linter/rules/no-useless-else",
    "lint/style/noVar": "https://biomejs.dev/linter/rules/no-var",
    "lint/style/useAsConstAssertion": "https://biomejs.dev/linter/rules/use-as-const-assertion",
    "lint/style/useBlockStatements": "https://biomejs.dev/linter/rules/use-block-statements",
    "lint/style/useCollapsedElseIf": "https://biomejs.dev/linter/rules/use-collapsed-else-if",
    "lint/style/useConsistentArrayType": "https://biomejs.dev/linter/rules/use-consistent-array-type",
    "lint/style/useConst": "https://biomejs.dev/linter/rules/use-const",
    "lint/style/useDefaultParameterLast": "https://biomejs.dev/linter/rules/use-default-parameter-last",
    "lint/style/useEnumInitializers": "https://biomejs.dev/linter/rules/use-enum-initializers",
    "lint/style/useExponentiationOperator": "https://biomejs.dev/linter/rules/use-exponentiation-operator",
    "lint/style/useExportType": "https://biomejs.dev/linter/rules/use-export-type",
    "lint/style/useFilenamingConvention": "https://biomejs.dev/linter/rules/use-filenaming-convention",
    "lint/style/useForOf": "https://biomejs.dev/linter/rules/use-for-of",
    "lint/style/useFragmentSyntax": "https://biomejs.dev/linter/rules/use-fragment-syntax",
    "lint/style/useImportType": "https://biomejs.dev/linter/rules/use-import-type",
    "lint/style/useLiteralEnumMembers": "https://biomejs.dev/linter/rules/use-literal-enum-members",
    "lint/style/useNamingConvention": "https://biomejs.dev/linter/rules/use-naming-convention",
    "lint/style/useNodeAssertStrict": "https://biomejs.dev/linter/rules/use-node-assert-strict",
    "lint/style/useNodejsImportProtocol": "https://biomejs.dev/linter/rules/use-nodejs-import-protocol",
    "lint/style/useNumberNamespace": "https://biomejs.dev/linter/rules/use-number-namespace",
    "lint/style/useNumericLiterals": "https://biomejs.dev/linter/rules/use-numeric-literals",
    "lint/style/useSelfClosingElements": "https://biomejs.dev/linter/rules/use-self-closing-elements",
    "lint/style/useShorthandArrayType": "https://biomejs.dev/linter/rules/use-shorthand-array-type",
    "lint/style/useShorthandAssign": "https://biomejs.dev/linter/rules/use-shorthand-assign",
    "lint/style/useShorthandFunctionType": "https://biomejs.dev/linter/rules/use-shorthand-function-type",
    "lint/style/useSingleCaseStatement": "https://biomejs.dev/linter/rules/use-single-case-statement",
    "lint/style/useSingleVarDeclarator": "https://biomejs.dev/linter/rules/use-single-var-declarator",
    "lint/style/useTemplate": "https://biomejs.dev/linter/rules/use-template",
    "lint/style/useWhile": "https://biomejs.dev/linter/rules/use-while",
    "lint/suspicious/noApproximativeNumericConstant": "https://biomejs.dev/linter/rules/no-approximative-numeric-constant",
    "lint/suspicious/noArrayIndexKey": "https://biomejs.dev/linter/rules/no-array-index-key",
    "lint/suspicious/noAssignInExpressions": "https://biomejs.dev/linter/rules/no-assign-in-expressions",
    "lint/suspicious/noAsyncPromiseExecutor": "https://biomejs.dev/linter/rules/no-async-promise-executor",
    "lint/suspicious/noCatchAssign": "https://biomejs.dev/linter/rules/no-catch-assign",
    "lint/suspicious/noClassAssign": "https://biomejs.dev/linter/rules/no-class-assign",
    "lint/suspicious/noCommentText": "https://biomejs.dev/linter/rules/no-comment-text",
    "lint/suspicious/noCompareNegZero": "https://biomejs.dev/linter/rules/no-compare-neg-zero",
    "lint/suspicious/noConfusingLabels": "https://biomejs.dev/linter/rules/no-confusing-labels",
    "lint/suspicious/noConfusingVoidType": "https://biomejs.dev/linter/rules/no-confusing-void-type",
    "lint/suspicious/noConsoleLog": "https://biomejs.dev/linter/rules/no-console-log",
    "lint/suspicious/noConstEnum": "https://biomejs.dev/linter/rules/no-const-enum",
    "lint/suspicious/noControlCharactersInRegex": "https://biomejs.dev/linter/rules/no-control-characters-in-regex",
    "lint/suspicious/noDebugger": "https://biomejs.dev/linter/rules/no-debugger",
    "lint/suspicious/noDoubleEquals": "https://biomejs.dev/linter/rules/no-double-equals",
    "lint/suspicious/noDuplicateCase": "https://biomejs.dev/linter/rules/no-duplicate-case",
    "lint/suspicious/noDuplicateClassMembers": "https://biomejs.dev/linter/rules/no-duplicate-class-members",
    "lint/suspicious/noDuplicateJsxProps": "https://biomejs.dev/linter/rules/no-duplicate-jsx-props",
    "lint/suspicious/noDuplicateObjectKeys": "https://biomejs.dev/linter/rules/no-duplicate-object-keys",
    "lint/suspicious/noDuplicateParameters": "https://biomejs.dev/linter/rules/no-duplicate-parameters",
    "lint/suspicious/noDuplicateTestHooks": "https://biomejs.dev/linter/rules/no-duplicate-test-hooks",
    "lint/suspicious/noEmptyBlockStatements": "https://biomejs.dev/linter/rules/no-empty-block-statements",
    "lint/suspicious/noEmptyInterface": "https://biomejs.dev/linter/rules/no-empty-interface",
    "lint/suspicious/noExplicitAny": "https://biomejs.dev/linter/rules/no-explicit-any",
    "lint/suspicious/noExportsInTest": "https://biomejs.dev/linter/rules/no-exports-in-test",
    "lint/suspicious/noExtraNonNullAssertion": "https://biomejs.dev/linter/rules/no-extra-non-null-assertion",
    "lint/suspicious/noFallthroughSwitchClause": "https://biomejs.dev/linter/rules/no-fallthrough-switch-clause",
    "lint/suspicious/noFocusedTests": "https://biomejs.dev/linter/rules/no-focused-tests",
    "lint/suspicious/noFunctionAssign": "https://biomejs.dev/linter/rules/no-function-assign",
    "lint/suspicious/noGlobalAssign": "https://biomejs.dev/linter/rules/no-global-assign",
    "lint/suspicious/noGlobalIsFinite": "https://biomejs.dev/linter/rules/no-global-is-finite",
    "lint/suspicious/noGlobalIsNan": "https://biomejs.dev/linter/rules/no-global-is-nan",
    "lint/suspicious/noImplicitAnyLet": "https://biomejs.dev/linter/rules/no-implicit-any-let",
    "lint/suspicious/noImportAssign": "https://biomejs.dev/linter/rules/no-import-assign",
    "lint/suspicious/noLabelVar": "https://biomejs.dev/linter/rules/no-label-var",
    "lint/suspicious/noMisleadingCharacterClass": "https://biomejs.dev/linter/rules/no-misleading-character-class",
    "lint/suspicious/noMisleadingInstantiator": "https://biomejs.dev/linter/rules/no-misleading-instantiator",
    "lint/suspicious/noMisrefactoredShorthandAssign": "https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign",
    "lint/suspicious/noPrototypeBuiltins": "https://biomejs.dev/linter/rules/no-prototype-builtins",
    "lint/suspicious/noRedeclare": "https://biomejs.dev/linter/rules/no-redeclare",
    "lint/suspicious/noRedundantUseStrict": "https://biomejs.dev/linter/rules/no-redundant-use-strict",
    "lint/suspicious/noSelfCompare": "https://biomejs.dev/linter/rules/no-self-compare",
    "lint/suspicious/noShadowRestrictedNames": "https://biomejs.dev/linter/rules/no-shadow-restricted-names",
    "lint/suspicious/noSkippedTests": "https://biomejs.dev/linter/rules/no-skipped-tests",
    "lint/suspicious/noSparseArray": "https://biomejs.dev/linter/rules/no-sparse-array",
    "lint/suspicious/noSuspiciousSemicolonInJsx": "https://biomejs.dev/linter/rules/no-suspicious-semicolon-in-jsx",
    "lint/suspicious/noThenProperty": "https://biomejs.dev/linter/rules/no-then-property",
    "lint/suspicious/noUnsafeDeclarationMerging": "https://biomejs.dev/linter/rules/no-unsafe-declaration-merging",
    "lint/suspicious/noUnsafeNegation": "https://biomejs.dev/linter/rules/no-unsafe-negation",
    "lint/suspicious/useAwait": "https://biomejs.dev/linter/rules/use-await",
    "lint/suspicious/useDefaultSwitchClauseLast": "https://biomejs.dev/linter/rules/use-default-switch-clause-last",
    "lint/suspicious/useGetterReturn": "https://biomejs.dev/linter/rules/use-getter-return",
    "lint/suspicious/useIsArray": "https://biomejs.dev/linter/rules/use-is-array",
    "lint/suspicious/useNamespaceKeyword": "https://biomejs.dev/linter/rules/use-namespace-keyword",
    "lint/suspicious/useValidTypeof": "https://biomejs.dev/linter/rules/use-valid-typeof",
    ;
    // General categories
    "files/missingHandler",
    "format",
    "check",
    "ci",
    "configuration",
    "organizeImports",
    "migrate",
    "deserialize",
    "project",
    "search",
    "internalError/io",
    "internalError/fs",
    "internalError/panic",
    // parse categories
    "parse",
    "parse/noSuperWithoutExtends",
    "parse/noInitializerWithDefinite",
    "parse/noDuplicatePrivateClassMembers",

    // Lint groups
    "lint",
    "lint/a11y",
    "lint/complexity",
    "lint/correctness",
    "lint/nursery",
    "lint/performance",
    "lint/security",
    "lint/style",
    "lint/suspicious",

    // Suppression comments
    "suppressions/parse",
    "suppressions/unknownGroup",
    "suppressions/unknownRule",
    "suppressions/unused",
    "suppressions/deprecatedSuppressionComment",

    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
