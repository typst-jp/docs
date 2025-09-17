import type { Page } from "../../../types/model";
import { ChevronRightIcon } from "../../icons";

export type SideNavigationProps = {
	docs: Page[];
	currentRoute: string;
	currentPath: Page[];
};

export const SideNavigation = ({
	docs,
	currentRoute,
	currentPath,
}: SideNavigationProps) => {
	return (
		<nav class="folding flex-none w-full bg-white border border-neutral-200/60 rounded-md sticky top-[80px] mt-4 mb-8 h-[calc(100vh-80px-1rem)] overflow-auto px-3.5 py-3">
			<ul class="space-y-1 text-sm text-neutral-700">
				{docs?.map((firstLevel, _idx) => (
					<>
						{firstLevel.part && (
							<li
								key={firstLevel.part}
								class="category py-2 text-sm font-semibold text-gray-500 uppercase tracking-wider"
							>
								{firstLevel.part}
							</li>
						)}
						<li
							key={firstLevel.route}
							class="relative"
							x-data={`{ open: ${
								firstLevel.route === currentRoute ||
								currentPath.some((p) => p.route === firstLevel.route)
							} }`}
						>
							<a
								href={firstLevel.route}
								class="block px-2 py-1 rounded hover:bg-neutral-100 transition-colors"
								aria-current={
									firstLevel.route === currentRoute ? "page" : undefined
								}
							>
								{firstLevel.title}
							</a>
							{firstLevel.children?.length > 0 && (
								<>
									<button
										type="button"
										class="absolute right-2 top-0 h-6 w-6 flex items-center justify-center p-1 rounded-full hover:bg-gray-100"
										x-on:click="open = !open"
									>
										<div
											class="w-4 h-4 text-gray-400 transition-transform duration-200"
											x-bind:class="open ? 'rotate-90 transform' : ''"
										>
											<ChevronRightIcon />
										</div>
									</button>
									<ul
										class="pl-4 border-l border-gray-100 ml-4 mt-1 space-y-1"
										x-show="open"
										x-transition:enter="transition ease-out duration-200"
										x-transition:enter-start="opacity-0 transform -translate-y-2"
										x-transition:enter-end="opacity-100 transform translate-y-0"
									>
										{firstLevel.children.map((secondLevel, _idx2) => (
											<div key={secondLevel.route}>
												{secondLevel.part && (
													<li class="category py-1 text-sm font-semibold text-gray-500 tracking-wide">
														{secondLevel.part}
													</li>
												)}
												<li
													class="relative"
													x-data={`{ open: ${
														secondLevel.route === currentRoute ||
														currentPath.some(
															(p) => p.route === secondLevel.route,
														)
													} }`}
												>
													<a
														href={secondLevel.route}
														class="block px-2 py-1 rounded hover:bg-neutral-100 transition-colors"
														aria-current={
															secondLevel.route === currentRoute
																? "page"
																: undefined
														}
													>
														{secondLevel.title}
													</a>
													{secondLevel.children?.length > 0 && (
														<>
															<button
																type="button"
																class="absolute right-2 top-0 h-6 w-6 flex items-center justify-center p-1 rounded-full hover:bg-gray-100 z-10"
																x-on:click="open = !open"
															>
																<div
																	class="w-4 h-4 text-gray-400 transition-transform duration-200"
																	x-bind:class="open ? 'rotate-90 transform' : ''"
																>
																	<ChevronRightIcon />
																</div>
															</button>
															<ul
																class="pl-4 border-l border-gray-100 ml-4 mt-1 space-y-1"
																x-show="open"
																x-transition:enter="transition ease-out duration-200"
																x-transition:enter-start="opacity-0 transform -translate-y-2"
																x-transition:enter-end="opacity-100 transform translate-y-0"
															>
																{secondLevel.children.map(
																	(thirdLevel, _idx3) => (
																		<div key={thirdLevel.route}>
																			{thirdLevel.part && (
																				<li class="category">
																					{thirdLevel.part}
																				</li>
																			)}
																			<li>
																				<a
																					href={thirdLevel.route}
																					class="block px-2 py-1 rounded hover:bg-neutral-100 transition-colors"
																					aria-current={
																						thirdLevel.route === currentRoute
																							? "page"
																							: undefined
																					}
																				>
																					{thirdLevel.title}
																				</a>
																			</li>
																		</div>
																	),
																)}
															</ul>
														</>
													)}
												</li>
											</div>
										))}
									</ul>
								</>
							)}
						</li>
					</>
				))}
			</ul>
		</nav>
	);
};
