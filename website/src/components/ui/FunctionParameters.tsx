import { FC } from "hono/jsx";
import type { Func } from "../../types/model";
import { Tooltip } from "./Tooltip";
import { type2href } from "./type2href";
import { type2class } from "./type2class";

type FunctionParametersProps = {
  func: Func;
  prefix?: string;
};

export const FunctionParameters: FC<FunctionParametersProps> = ({
  func,
  prefix = "",
}) => {
  return (
    <>
      {func.params.map((param, index) => (
        <div>
          <h4 id={`${prefix}-${func.name}-parameters-${param.name}`}>
            <code>{param.name}</code>
            <div class="additional-info">
              <div>
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
              </div>

              {param.required && <small>Required</small>}

              {param.positional && (
                <small>
                  <Tooltip
                    name="Positional"
                    desc="Positional parameters are specified in order, without names."
                    prefix={`${prefix}-${func.name}-parameters-${param.name}-positional`}
                  />
                </small>
              )}

              {param.variadic && (
                <small>
                  <Tooltip
                    name="Variadic"
                    desc="Variadic parameters can be specified multiple times."
                    prefix={`${prefix}-${func.name}-parameters-${param.name}-variadic`}
                  />
                </small>
              )}

              {param.settable && (
                <small>
                  <Tooltip
                    name="Settable"
                    desc="Settable parameters can be customized for all following uses of the function with a <code>set</code> rule."
                    prefix={`${prefix}-${func.name}-parameters-${param.name}-settable`}
                  />
                </small>
              )}
            </div>
          </h4>

          <div dangerouslySetInnerHTML={{ __html: param.details }} />

          {param.strings.length > 0 && (
            <ul class="type-args">
              {param.strings.map((string, i) => (
                <li>
                  <div class="break-box">
                    <div>
                      <code class="typ-str">
                        &quot;{/* --> */}
                        {string.string}
                        {/* --> */}&quot;
                      </code>
                    </div>
                    <div dangerouslySetInnerHTML={{ __html: string.details }} />
                  </div>
                </li>
              ))}
            </ul>
          )}

          {param.default && (
            <p>
              Default:{" "}
              <span dangerouslySetInnerHTML={{ __html: param.default }} />
            </p>
          )}

          {param.example && (
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
                dangerouslySetInnerHTML={{ __html: param.example }}
              />
            </details>
          )}
        </div>
      ))}
    </>
  );
};
