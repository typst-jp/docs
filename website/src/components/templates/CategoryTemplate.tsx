import type { FC } from "hono/jsx";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";
import type { CategoryBody, Page } from "../../types/model";

export type CategoryTemplateProps = Omit<BaseTemplateProps, "page"> & {
  page: Omit<Page, "body"> & {
    body: CategoryBody;
  };
};

export const CategoryTemplate: FC<CategoryTemplateProps> = ({
  page,
  docs,
  path,
  previousPage,
  nextPage,
}) => {
  return (
    <BaseTemplate
      page={page}
      docs={docs}
      path={path}
      previousPage={previousPage}
      nextPage={nextPage}
    >
      <h1 id="summary">{page.body.content.name}</h1>
      <div dangerouslySetInnerHTML={{ __html: page.body.content.details }} />
      <h2 id="definitions">定義</h2>
      <ul class="subgridded">
        {page.body.content.items.map((item) => (
          <li>
            <a href={item.route}>
              {item.code ? <code>{item.name}</code> : item.name}
            </a>
            <span>{item.oneliner}</span>
          </li>
        ))}
      </ul>
    </BaseTemplate>
  );
};
