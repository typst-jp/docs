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
		it("ファイルが存在する場合にJSONを読み込んで返す", async () => {
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

		it("ファイルが存在しない場合に空のオブジェクトを返す", async () => {
			const { loadTranslationStatus } = await import("./translationStatus");

			mockFs.existsSync.mockReturnValue(false);

			const result = loadTranslationStatus();

			expect(mockFs.existsSync).toHaveBeenCalledWith(mockFilePath);
			expect(mockFs.readFileSync).not.toHaveBeenCalled();
			expect(result).toEqual({});
		});
	});

	describe("saveTranslationStatus", () => {
		it("翻訳状態をJSONファイルに保存する", async () => {
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

		it("新しいルートを未翻訳として登録する", async () => {
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

		it("既存のルートは変更しない", async () => {
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

		it("翻訳進捗率を正しく計算する", async () => {
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

		it("originalページは計算から除外される", async () => {
			const { calculateTranslationProgressRate } = await import(
				"./translationStatus"
			);

			const status = {
				$schema: "./translation-status.schema.json",
				"/docs/page1/": "translated", // 1.0
				"/docs/page2/": "partially_translated", // 0.5
				"/docs/original1/": "community", // 除外
				"/docs/original2/": "community", // 除外
				"/docs/page3/": "untranslated", // 0.0
			};

			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			const result = calculateTranslationProgressRate();

			// (1.0 + 0.5 + 0.0) / 3 = 0.5
			expect(result).toBe(0.5);
		});

		it("全てoriginalページの場合は0を返す", async () => {
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

		it("ページが存在しない場合は0を返す", async () => {
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

		it("$schemaキーは計算から除外する", async () => {
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
		it("指定されたルートの翻訳状態を返す", async () => {
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

		it('存在しないルートの場合は"untranslated"を返す', async () => {
			const { getTranslationStatus } = await import("./translationStatus");

			const status = {
				"/docs/": "translated",
			};

			mockFs.existsSync.mockReturnValue(true);
			mockFs.readFileSync.mockReturnValue(JSON.stringify(status));

			expect(getTranslationStatus("/docs/nonexistent/")).toBe("untranslated");
		});

		it("2回目の呼び出しではキャッシュを使用する", async () => {
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
