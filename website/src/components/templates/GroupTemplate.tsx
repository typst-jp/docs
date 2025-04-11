import type { FC } from "hono/jsx";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";
import type { GroupBody, Page } from "../../types/model";
import {
  Tooltip,
  FunctionDisplay,
  FunctionDefinition,
  FunctionParameters,
} from "../ui";

export type GroupTemplateProps = Omit<BaseTemplateProps, "page"> & {
  page: Omit<Page, "body"> & {
    body: GroupBody;
  };
};

export const GroupTemplate: FC<GroupTemplateProps> = ({
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
      <h1 id="summary">{content.title}</h1>
      <div dangerouslySetInnerHTML={{ __html: content.details }} />

      {content.functions.length > 0 && (
        <>
          <h2 id="functions">Function</h2>

          {content.functions.map((method, index) => (
            <div>
              <h3 id={`definitions-${method.name}`} class="method-head">
                <span>
                  <code>{method.name}</code>
                </span>
                <small>
                  <Tooltip
                    name="Element"
                    desc="Element functions can be customized with <code>set</code> and <code>show</code> rules."
                  />
                </small>
              </h3>
              <FunctionDisplay
                func={method}
                prefix={`definitions-${method.name}`}
                isExampleFolding={false}
              />
            </div>
          ))}
        </>
      )}
    </BaseTemplate>
  );
};

export default GroupTemplate;
