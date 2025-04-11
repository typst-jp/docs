import { FC } from "hono/jsx";

type TooltipProps = {
  name: string;
  desc?: string;
  prefix?: string;
};

export const Tooltip: FC<TooltipProps> = ({ name, desc = "", prefix = "" }) => {
  return (
    <>
      <span aria-describedby={`${prefix}-tooltip`}>{name}</span>
      <div class="tooltip-context">
        <svg
          width="12"
          height="12"
          viewBox="0 0 12 12"
          tabIndex={0}
          role="img"
          aria-labelledby={`${prefix}-tooltip-qm-label`}
        >
          <title id={`${prefix}-tooltip-qm-label`}>{desc}</title>
          <use href="/assets/icons/12-tooltip.svg#icon"></use>
        </svg>
        <div
          role="tooltip"
          id={`${prefix}-tooltip`}
          tabIndex={-1}
          dangerouslySetInnerHTML={{ __html: desc }}
        />
      </div>
    </>
  );
};
