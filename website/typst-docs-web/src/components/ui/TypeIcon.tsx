import type { FC } from "hono/jsx";

type TypeIconProps = {
	type: string;
	href?: string;
	isHeading?: boolean;
};

const getTypeStyles = (
	type: string,
): { bgColor: string; textColor: string } => {
	const typeStylesMap: Record<string, { bgColor: string; textColor: string }> =
		{
			content: { bgColor: "bg-teal-100", textColor: "text-teal-800" },

			bool: { bgColor: "bg-yellow-100", textColor: "text-yellow-800" },
			boolean: { bgColor: "bg-yellow-100", textColor: "text-yellow-800" },

			string: { bgColor: "bg-green-100", textColor: "text-green-800" },
			str: { bgColor: "bg-green-100", textColor: "text-green-800" },

			none: { bgColor: "bg-red-100", textColor: "text-red-800" },
			auto: { bgColor: "bg-red-100", textColor: "text-red-800" },

			integer: { bgColor: "bg-purple-100", textColor: "text-purple-800" },
			int: { bgColor: "bg-purple-100", textColor: "text-purple-800" },
			ratio: { bgColor: "bg-purple-100", textColor: "text-purple-800" },
			length: { bgColor: "bg-purple-100", textColor: "text-purple-800" },
			"relative length": {
				bgColor: "bg-purple-100",
				textColor: "text-purple-800",
			},
			float: { bgColor: "bg-purple-100", textColor: "text-purple-800" },
			angle: { bgColor: "bg-purple-100", textColor: "text-purple-800" },
			fraction: { bgColor: "bg-purple-100", textColor: "text-purple-800" },

			function: { bgColor: "bg-pink-100", textColor: "text-pink-800" },

			label: { bgColor: "bg-blue-100", textColor: "text-blue-800" },

			color: {
				bgColor: "bg-gradient-to-r from-blue-300 via-green-200 to-yellow-200",
				textColor: "text-gray-800",
			},
		};

	return (
		typeStylesMap[type] || {
			bgColor: "bg-gray-200",
			textColor: "text-gray-800",
		}
	);
};

export const TypeIcon: FC<TypeIconProps> = ({
	type,
	href,
	isHeading = false,
}) => {
	const styles = getTypeStyles(type);

	const sizeClasses = isHeading
		? "text-3xl px-3 py-1.5"
		: "text-sm px-1.5 py-0.5";

	const baseClasses = `pill ${styles.bgColor} ${styles.textColor} rounded-md ${sizeClasses} font-mono whitespace-nowrap inline-flex items-center mx-0.5`;

	if (href) {
		return (
			<a href={href} class={baseClasses}>
				{type}
			</a>
		);
	}

	return <span class={baseClasses}>{type}</span>;
};
