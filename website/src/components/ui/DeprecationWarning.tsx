import type { FC } from "hono/jsx";
import { AlertTriangleIcon } from "../icons";

type DeprecationWarningProps = {
	message: string;
};

export const DeprecationWarning: FC<DeprecationWarningProps> = ({
	message,
}) => {
	return (
		<small className="deprecation ml-2 flex items-center bg-yellow-50 px-2 py-1 rounded-md border border-yellow-200">
			<div className="w-4 h-4 text-yellow-500 mr-1.5 flex-shrink-0">
				<AlertTriangleIcon />
			</div>
			<span className="text-xs text-yellow-800">{message}</span>
		</small>
	);
};
