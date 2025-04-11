import type { FC } from "hono/jsx";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";
import type { TypeBody, Page } from "../../types/model";
import { Tooltip, FunctionDisplay } from "../ui";
import { type2class } from "../ui/type2class";

export type TypeTemplateProps = Omit<BaseTemplateProps, "page"> & {
  page: Omit<Page, "body"> & {
    body: TypeBody;
  };
};

export const TypeTemplate: FC<TypeTemplateProps> = ({
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
        <span class={`pill ${type2class(content.name)}`}>{content.name}</span>
      </h1>

      <div dangerouslySetInnerHTML={{ __html: content.details }} />

      {content.constructor && (
        <>
          <h2 id="constructor">
            <Tooltip
              name="コンストラクタ"
              desc="If a type has a constructor, you can call it like a function to create a new value of the type."
              prefix="constructor"
            />
          </h2>

          <FunctionDisplay
            func={content.constructor}
            prefix="constructor"
            isExampleFolding={false}
          />
        </>
      )}

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
                <code>{method.name}</code>
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

export default TypeTemplate;
