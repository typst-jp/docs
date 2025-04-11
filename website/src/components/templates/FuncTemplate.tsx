import type { FC } from "hono/jsx";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";
import type { FuncBody, Page } from "../../types/model";
import {
  Tooltip,
  FunctionDisplay,
  FunctionDefinition,
  FunctionParameters,
} from "../ui";

export type FuncTemplateProps = Omit<BaseTemplateProps, "page"> & {
  page: Omit<Page, "body"> & {
    body: FuncBody;
  };
};

export const FuncTemplate: FC<FuncTemplateProps> = ({
  page,
  docs,
  path,
  previousPage,
  nextPage,
}) => {
  const content = page.body.content;

  return (
    <BaseTemplate
      page={page}
      docs={docs}
      path={path}
      previousPage={previousPage}
      nextPage={nextPage}
    >
      <h1 id="summary">
        <code>{content.name}</code>
        <small>
          {content.element && (
            <Tooltip
              name="Element"
              desc="Element functions can be customized with <code>set</code> and <code>show</code> rules."
            />
          )}
          {content.contextual && (
            <Tooltip
              name="Contextual"
              desc="Contextual functions can only be used when the context is known"
            />
          )}
        </small>
        {content.deprecation && (
          <small class="deprecation">
            <div class="tooltip-context">
              <svg
                width="16"
                height="16"
                viewBox="0 0 16 16"
                tabIndex={0}
                role="img"
                aria-labelledby={`${content.name}-deprecation-tooltip`}
              >
                <title id={`${content.name}-deprecation-tooltip`}>
                  Warning
                </title>
                <use href="/assets/icons/16-warn.svg#icon"></use>
              </svg>
            </div>
            <span>
              <span>{content.deprecation}</span>
            </span>
          </small>
        )}
      </h1>

      <div dangerouslySetInnerHTML={{ __html: content.details }} />

      <h2 id="parameters">
        <Tooltip
          name="引数"
          desc="Parameters are the inputs to a function. They are specified in parentheses after the function name."
          prefix="parameters"
        />
      </h2>

      <FunctionDefinition func={content} prefix={content.name} />

      {content.example && (
        <div dangerouslySetInnerHTML={{ __html: content.example }} />
      )}

      <FunctionParameters func={content} prefix={content.name} />

      {content.scope.length > 0 && (
        <>
          <h2 id="definitions">
            <Tooltip
              name="定義"
              desc="Functions and types and can have associated definitions. These are accessed by specifying the function or type, followed by a period, and then the definition's name."
              prefix="definitions"
            />
          </h2>

          {content.scope.map((method, index) => (
            <div>
              <h3 id={`definitions-${method.name}`} class="method-head">
                <code
                  style={
                    method.deprecation
                      ? { textDecoration: "line-through" }
                      : undefined
                  }
                >
                  {method.name}
                </code>

                <small>
                  {method.element && (
                    <Tooltip
                      name="Element"
                      desc="Element functions can be customized with <code>set</code> and <code>show</code> rules."
                    />
                  )}
                  {method.contextual && (
                    <Tooltip
                      name="Contextual"
                      desc="Contextual functions can only be used when the context is known"
                    />
                  )}
                </small>

                {/* 非推奨表示 */}
                {method.deprecation && (
                  <small class="deprecation">
                    <div class="tooltip-context">
                      <svg
                        width="16"
                        height="16"
                        viewBox="0 0 16 16"
                        tabIndex={0}
                        role="img"
                        aria-labelledby={`definitions-${method.name}-deprecation-tooltip`}
                      >
                        <title
                          id={`definitions-${method.name}-deprecation-tooltip`}
                        >
                          Warning
                        </title>
                        <use href="/assets/icons/16-warn.svg#icon"></use>
                      </svg>
                    </div>
                    <span>
                      <span>{method.deprecation}</span>
                    </span>
                  </small>
                )}
              </h3>

              <FunctionDisplay
                func={method}
                prefix={`definitions-${method.name}`}
              />
            </div>
          ))}
        </>
      )}
    </BaseTemplate>
  );
};

export default FuncTemplate;
