import fs from "node:fs";
import path from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { TranslationStatusMap } from "./translationStatus";

vi.mock("node:fs");
vi.mock("node:path");
vi.mock("node:process", () => ({
	cwd: vi.fn(() => "/mock"),
}));

const mockFs = vi.mocked(fs);
const mockPath = vi.mocked(path);

describe("translationStatus", () => {
	const mockFilePath = "/mock/translation-status.json";

	beforeEach(() => {
		mockPath.resolve.mockReturnValue(mockFilePath);
		mockFs.writeFileSync.mockImplementation(() => {});
		mockFs.existsSync.mockReturnValue(false);
		mockFs.readFileSync.mockReturnValue("{}");
	});

	afterEach(() => {
		vi.clearAllMocks();
	});

	describe("loadTranslationStatus", () => {
		it("should read and return JSON when the file exists", async () => {
			const { loadTranslationStatus } = await import("./translationStatus");

			const mockData = {
				"/docs/": "translated",
				"/docs/tutorial/": "partially_translated",
			};

			mockFs.existsSync.mockReturnValue(true);
			mockFs.readFileSync.mockReturnValue(JSON.stringify(mockData));

			const result = loadTranslationStatus();

			expect(mockFs.existsSync).toHaveBeenCalledWith(mockFilePath);
			expect(mockFs.readFileSync).toHaveBeenCalledWith(mockFilePath, "utf-8");
			expect(result).toEqual(mockData);
		});

		it("should return an empty object when the file does not exist", async () => {
			const { loadTranslationStatus } = await import("./translationStatus");

			mockFs.existsSync.mockReturnValue(false);

			const result = loadTranslationStatus();

			expect(mockFs.existsSync).toHaveBeenCalledWith(mockFilePath);
			expect(mockFs.readFileSync).not.toHaveBeenCalled();
			expect(result).toEqual({});
		});
	});

	describe("saveTranslationStatus", () => {
		it("should save translation status to a JSON file", async () => {
			const { saveTranslationStatus } = await import("./translationStatus");

			const status: TranslationStatusMap = {
				"/docs/": "translated",
				"/docs/tutorial/": "partially_translated",
			};

			const expectedJson = {
				$schema: "./translation-status.schema.json",
				...status,
			};

			saveTranslationStatus(status);

			expect(mockFs.writeFileSync).toHaveBeenCalledWith(
				mockFilePath,
				JSON.stringify(expectedJson, null, 2),
				"utf-8",
			);
		});
	});

	describe("registerRoutes", () => {
		beforeEach(() => {
			mockFs.existsSync.mockReturnValue(true);
		});

		it("should register new routes as untranslated", async () => {
			const { registerRoutes } = await import("./translationStatus");

			const existingStatus = {
				"/docs/": "translated",
			};

			mockFs.readFileSync.mockReturnValue(JSON.stringify(existingStatus));

			const routes = ["/docs/", "/docs/new-page/"];
			registerRoutes(routes);

			expect(mockFs.writeFileSync).toHaveBeenCalled();

			const savedContent = mockFs.writeFileSync.mock.calls[0][1] as string;
			const savedData = JSON.parse(savedContent);

			expect(savedData["/docs/"]).toBe("translated");
			expect(savedData["/docs/new-page/"]).toBe("untranslated");
		});

		it("should not modify existing routes", async () => {
			const { registerRoutes } = await import("./translationStatus");

			const existingStatus = {
				"/docs/": "translated",
				"/docs/tutorial/": "partially_translated",
			};

			mockFs.readFileSync.mockReturnValue(JSON.stringify(existingStatus));

			const routes = ["/docs/", "/docs/tutorial/"];
			registerRoutes(routes);

			expect(mockFs.writeFileSync).not.toHaveBeenCalled();
		});
	});

	describe("calculateTranslationProgressRate", () => {
		beforeEach(() => {
			mockFs.existsSync.mockReturnValue(true);
		});

		it("correctly calculates the translation progress rate", async () => {
			const { calculateTranslationProgressRate } = await import(
				"./translationStatus"
			);

			const status = {
				$schema: "./translation-status.schema.json",
				"/docs/page1/": "translated", // 1.0
				"/docs/page2/": "translated", // 1.0
				"/docs/page3/": "partially_translated", // 0.5
				"/docs/page4/": "untranslated", // 0.0
			};

			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			const result = calculateTranslationProgressRate();

			// (1.0 + 1.0 + 0.5 + 0.0) / 4 = 0.625
			expect(result).toBe(0.625);
		});

		it("original pages are excluded from calculation", async () => {
			const { calculateTranslationProgressRate } = await import(
				"./translationStatus"
			);

			const status = {
				$schema: "./translation-status.schema.json",
				"/docs/page1/": "translated", // 1.0
				"/docs/page2/": "partially_translated", // 0.5
				"/docs/original1/": "community", // Excluded
				"/docs/original2/": "community", // Excluded
				"/docs/page3/": "untranslated", // 0.0
			};

			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			const result = calculateTranslationProgressRate();

			// (1.0 + 0.5 + 0.0) / 3 = 0.5
			expect(result).toBe(0.5);
		});

		it("returns 0 when all pages are original", async () => {
			const { calculateTranslationProgressRate } = await import(
				"./translationStatus"
			);

			const status = {
				$schema: "./translation-status.schema.json",
				"/docs/original1/": "community",
				"/docs/original2/": "community",
			};

			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			const result = calculateTranslationProgressRate();

			expect(result).toBe(0);
		});

		it("returns 0 when a page does not exist", async () => {
			const { calculateTranslationProgressRate } = await import(
				"./translationStatus"
			);

			const status = {
				$schema: "./translation-status.schema.json",
			};

			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			const result = calculateTranslationProgressRate();

			expect(result).toBe(0);
		});

		it("excludes the $schema key from calculation", async () => {
			const { calculateTranslationProgressRate } = await import(
				"./translationStatus"
			);

			const status = {
				$schema: "./translation-status.schema.json",
				"/docs/page1/": "translated",
			};

			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			const result = calculateTranslationProgressRate();

			expect(result).toBe(1.0);
		});
	});

	describe("getTranslationStatus", () => {
		it("returns the translation status of the specified route", async () => {
			const { getTranslationStatus } = await import("./translationStatus");

			const status = {
				"/docs/": "translated",
				"/docs/tutorial/": "partially_translated",
			};

			mockFs.existsSync.mockReturnValue(true);
			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			expect(getTranslationStatus("/docs/")).toBe("translated");
			expect(getTranslationStatus("/docs/tutorial/")).toBe(
				"partially_translated",
			);
		});

		it('returns "untranslated" for a non-existent route', async () => {
			const { getTranslationStatus } = await import("./translationStatus");

			const status = {
				"/docs/": "translated",
			};

			mockFs.existsSync.mockReturnValue(true);
			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			expect(getTranslationStatus("/docs/nonexistent/")).toBe("untranslated");
		});

		it("uses cache for the second and subsequent calls", async () => {
			vi.resetModules();
			const { getTranslationStatus: freshGetTranslationStatus } = await import(
				"./translationStatus"
			);

			const status = {
				"/docs/": "translated",
			};

			mockFs.existsSync.mockReturnValue(true);
			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			freshGetTranslationStatus("/docs/");
			freshGetTranslationStatus("/docs/");

			expect(mockFs.readFileSync).toHaveBeenCalledTimes(1);
		});
	});
});
