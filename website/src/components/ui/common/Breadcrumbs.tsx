import type { Page } from "../../../types/model";
import { ChevronRightIcon, HomeIcon } from "../../icons";

export type BreadcrumbsProps = {
  path: Page[];
};

export const Breadcrumbs = ({ path }: BreadcrumbsProps) => {
  return (
    <nav class="flex justify-between px-3.5 py-1 border border-neutral-200/60 rounded-md">
      <ol class="inline-flex items-center mb-3 space-x-1 text-xs text-neutral-500 [&_.active-breadcrumb]:text-neutral-600 [&_.active-breadcrumb]:font-medium sm:mb-0">
        <li class="flex items-center h-full">
          <a href="/docs/" class="py-1 hover:text-neutral-900">
            <div class="w-4 h-4">
              <HomeIcon />
            </div>
          </a>
        </li>
        {path.map((item, idx) => (
          <>
            <div class="w-4 h-4 text-gray-400/70">
              <ChevronRightIcon />
            </div>
            <li>
              {idx === path.length - 1 ? (
                <a class="inline-flex items-center py-1 font-semibold text-neutral-700 rounded cursor-default focus:outline-none">
                  {item.title}
                </a>
              ) : (
                <a
                  href={item.route}
                  class="inline-flex items-center py-1 font-normal hover:text-neutral-900 focus:outline-none"
                >
                  {item.title}
                </a>
              )}
            </li>
          </>
        ))}
      </ol>
    </nav>
  );
};
