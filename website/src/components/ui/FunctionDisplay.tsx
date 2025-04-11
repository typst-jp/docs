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
      <div dangerouslySetInnerHTML={{ __html: func.details }} />

      <FunctionDefinition func={func} prefix={prefix} />

      {func.example && isExampleFolding && (
        <details class="folding-example">
          <summary>
            <img
              src="/assets/icons/16-arrow-right.svg"
              alt=""
              width="16"
              height="16"
            />
            View example
          </summary>
          <div
            class="folding"
            dangerouslySetInnerHTML={{ __html: func.example }}
          />
        </details>
      )}

      {func.example && !isExampleFolding && (
        <div dangerouslySetInnerHTML={{ __html: func.example }} />
      )}

      <FunctionParameters func={func} prefix={prefix} />
    </>
  );
};
