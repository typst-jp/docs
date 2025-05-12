import { FC } from "hono/jsx";
import type { Func } from "../../types/model";
import { Tooltip } from "./Tooltip";
import { type2href } from "./type2href";
import { TypeIcon } from "./TypeIcon";

type FunctionParametersProps = {
  func: Func;
  prefix?: string;
};

export const FunctionParameters: FC<FunctionParametersProps> = ({
  func,
  prefix = "",
}) => {
  return (
    <div class="space-y-6">
      {func.params.map((param, index) => (
        <div class="bg-gray-50 rounded-md p-4 border border-gray-100">
          <h4
            id={`${prefix}-${func.name}-parameters-${param.name}`}
            class="flex flex-wrap items-center gap-2 mb-3"
          >
            <code class="text-base font-medium">{param.name}</code>
            <div class="flex flex-wrap items-center gap-2 text-sm">
              <div class="flex flex-wrap gap-1">
                {param.types.map((t) => {
                  const href = type2href(t);
                  return (
                    <TypeIcon
                      type={t}
                      href={href ? `/docs/reference/${href}` : undefined}
                    />
                  );
                })}
              </div>

              {param.required && <Tooltip kind="required" />}
              {param.positional && <Tooltip kind="positional" />}
              {param.variadic && <Tooltip kind="variadic" />}
              {param.settable && <Tooltip kind="settable" />}
            </div>
          </h4>

          <div
            class="mb-3 text-gray-700"
            dangerouslySetInnerHTML={{ __html: param.details }}
          />

          {param.strings.length > 0 && (
            <div class="mt-3">
              <h5 class="text-sm font-medium text-gray-700 mb-2">
                使用可能な文字列値:
              </h5>
              <ul class="type-args space-y-2 pl-4">
                {param.strings.map((string) => (
                  <li>
                    <div class="break-box">
                      <div class="mb-1">
                        <code class="text-gray-800">{string.string}</code>
                      </div>
                      <div
                        class="text-sm text-gray-700"
                        dangerouslySetInnerHTML={{ __html: string.details }}
                      />
                    </div>
                  </li>
                ))}
              </ul>
            </div>
          )}

          {param.default && (
            <p class="mt-3 text-sm">
              <span class="font-medium">デフォルト値:</span>{" "}
              <span
                class="text-gray-700"
                dangerouslySetInnerHTML={{ __html: param.default }}
              />
            </p>
          )}

          {param.example && (
            <details class="mt-4 folding-example">
              <summary class="flex items-center gap-1 text-sm font-medium text-blue-600 cursor-pointer hover:text-blue-800">
                <img
                  src="/assets/icons/16-arrow-right.svg"
                  alt=""
                  width="16"
                  height="16"
                  class="transform transition-transform duration-200 group-open:rotate-90"
                />
                例を表示
              </summary>
              <div
                class="mt-2 bg-white p-3 rounded-md border border-gray-200 text-sm [&_img]:mx-auto [&_img]:block [&_img]:max-w-full"
                dangerouslySetInnerHTML={{ __html: param.example }}
              />
            </details>
          )}
        </div>
      ))}
    </div>
  );
};
