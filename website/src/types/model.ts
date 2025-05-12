// docs.jsonの型
// cf. docs/src/model.rs

/**
 * ページ情報
 */
export type Page = {
	route: string;
	title: string;
	description: string;
	part: string | null;
	outline: OutlineItem[];
	body: Body;
	children: Page[];
};

/**
 * アウトライン情報
 */
export type OutlineItem = {
	id: string;
	name: string;
	children: OutlineItem[];
};

/**
 * 本文情報
 */
export type Body =
	| HtmlBody
	| CategoryBody
	| FuncBody
	| GroupBody
	| TypeBody
	| SymbolsBody;

export type HtmlBody = {
	kind: "html";
	content: Html;
};

type Html = string;

export type CategoryBody = {
	kind: "category";
	content: {
		name: string;
		title: string;
		details: string;
		items: CategoryItem[];
		shorthands: Shorthands | null;
	};
};

type CategoryItem = {
	name: string;
	route: string;
	oneliner: string;
	code: boolean;
};

export type FuncBody = {
	kind: "func";
	content: Func;
};

export type Func = {
	path: string[];
	name: string;
	title: string;
	keywords: string[];
	oneliner: string;
	element: boolean;
	contextual: boolean;
	deprecation: string | null;
	details: Html;
	example: Html | null;
	self: boolean;
	params: Param[];
	returns: string[];
	scope: Func[];
};

type Param = {
	name: string;
	details: Html;
	example: Html | null;
	types: string[];
	strings: Str[];
	default: Html | null;
	positional: boolean;
	named: boolean;
	required: boolean;
	variadic: boolean;
	settable: boolean;
};

type Str = {
	string: string;
	details: Html;
};

export type GroupBody = {
	kind: "group";
	content: {
		name: string;
		title: string;
		details: Html;
		functions: Func[];
	};
};

export type TypeBody = {
	kind: "type";
	content: {
		name: string;
		title: string;
		keywords: string[];
		oneliner: string;
		details: Html;
		constructor: Func | null;
		scope: Func[];
	};
};

export type SymbolsBody = {
	kind: "symbols";
	content: {
		name: string;
		title: string;
		details: Html;
		list: Symbol[];
	};
};

type Symbol = {
	name: string;
	codepoint: number;
	accent: boolean;
	alternates: string[];
	markup_shorthand: null | string;
	math_shorthand: null | string;
	math_class: null | string;
	deprecation: null | string;
};

type Shorthands = {
	markup: Symbol[];
	math: Symbol[];
};
