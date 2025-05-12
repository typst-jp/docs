import { FC } from "hono/jsx";
import type { Func } from "../../types/model";
import { FunctionDefinition } from "./FunctionDefinition";
import { FunctionParameters } from "./FunctionParameters";

type FunctionDisplayProps = {
  func: Func;
  prefix?: string;
  isExampleFolding?: boolean;
};

export const FunctionDisplay: FC<FunctionDisplayProps> = ({
  func,
  prefix = "",
  isExampleFolding = true,
}) => {
  return (
    <>
      <div
        class="[&_img]:mx-auto [&_img]:block [&_img]:max-w-full"
        dangerouslySetInnerHTML={{ __html: func.details }}
      />

      <div class="my-4">
        <FunctionDefinition func={func} prefix={prefix} />
      </div>

      {func.example && isExampleFolding && (
        <details class="my-4 folding-example">
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
            dangerouslySetInnerHTML={{ __html: func.example }}
          />
        </details>
      )}

      {func.example && !isExampleFolding && (
        <div
          class="my-6 bg-gray-50 p-4 rounded-md border border-gray-200 [&_img]:mx-auto [&_img]:block [&_img]:max-w-full"
          dangerouslySetInnerHTML={{ __html: func.example }}
        />
      )}

      <div class="my-4">
        <FunctionParameters func={func} prefix={prefix} />
      </div>
    </>
  );
};
