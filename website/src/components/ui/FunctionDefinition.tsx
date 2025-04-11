import { FC } from "hono/jsx";
import type { Func } from "../../types/model";
import { genPath } from "./genPath";
import { type2href } from "./type2href";
import { type2class } from "./type2class";

type FunctionDefinitionProps = {
  func: Func;
  prefix?: string;
};

export const FunctionDefinition: FC<FunctionDefinitionProps> = ({
  func,
  prefix = "",
}) => {
  const isSingleArg = func.params.length <= 1;

  return (
    <div class={`code code-definition ${isSingleArg ? "single-arg" : ""}`}>
      {func.self ? "self." : genPath(func)}
      <span class="typ-func">{func.name}</span>(
      <div class="arguments">
        {func.params.map((param, index) => (
          <span class="overview-param">
            {!param.positional && (
              <a href={`#parameters-${param.name}`}>
                {param.name}
                {/* --> */}:
              </a>
            )}
            {param.types.map((t, i) => {
              const href = type2href(t);
              return href ? (
                <a
                  href={`/docs/reference/${href}`}
                  class={`pill ${type2class(t)}`}
                >
                  {t}
                </a>
              ) : (
                <span class={`pill ${type2class(t)}`}>{t}</span>
              );
            })}
            {func.params.length > 1 ? "," : ""}
          </span>
        ))}
      </div>
      )
      {func.returns.length > 0 && (
        <>
          {/* --> */}-&gt;
          {func.returns.map((ret, i) => {
            const href = type2href(ret);
            return (
              <>
                {href ? (
                  <a
                    href={`/docs/reference/${href}`}
                    class={`pill ${type2class(ret)}`}
                  >
                    {ret}
                  </a>
                ) : (
                  <span class={`pill ${type2class(ret)}`}>{ret}</span>
                )}
                {i < func.returns.length - 1 && ", "}
              </>
            );
          })}
        </>
      )}
    </div>
  );
};
