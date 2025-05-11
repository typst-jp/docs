import type { FC, PropsWithChildren } from "hono/jsx";
import type { Page } from "../../types/model";
import {
  SiteNoticeBanner,
  Footer,
  Breadcrumbs,
  TableOfContents,
} from "../ui/common/";

export type BaseTemplateProps = PropsWithChildren<{
  page: Page;
  docs: Page[];
  path: Page[];
  previousPage?: Page;
  nextPage?: Page;
}>;

export const BaseTemplate: FC<BaseTemplateProps> = ({
  children,
  page,
  docs,
  path,
  previousPage,
  nextPage,
}) => {
  const title = page.title;
  const description = page.description;
  const route = page.route;
  const outline = page.outline;
  return (
    <html lang="ja">
      <head>
        <meta charSet="utf-8" />
        <title>{title} – Typstドキュメント日本語版</title>
        <meta name="description" content={description} />
        <meta name="viewport" content="width=device-width,initial-scale=1" />
        <meta name="theme-color" content="#239dad" />
        <meta
          property="og:url"
          content={`https://typst-jp.github.io${route}`}
        />
        <meta
          property="og:title"
          content={`${title} – Typstドキュメント日本語版`}
        />
        <meta property="og:site_name" content="Typst" />
        <meta property="og:description" content={description} />
        <meta property="og:type" content="" />
        <meta
          property="og:image"
          content="https://typst-jp.github.io/assets/social.png"
        />
        <meta property="og:image:width" content="1200" />
        <meta property="og:image:height" content="630" />
        <meta name="twitter:site" content="@typstapp" />
        <meta name="twitter:card" content="summary_large_image" />
        <link rel="canonical" href={`https://typst-jp.github.io${route}`} />
        <meta
          name="twitter:image:alt"
          content="The left side of a text editor with colorful cursors, as well as the text 'Compose papers faster, Typst'"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="32x32"
          href="/assets/favicon.ico"
        />
        <link
          rel="apple-touch-icon"
          sizes="180x180"
          href="/assets/apple-touch-icon.png"
        />
        <link
          rel="mask-icon"
          href="/assets/safari-pinned-tab.svg"
          color="#239dad"
        />
        <link rel="manifest" href="/assets/site.webmanifest" />
        <link
          rel="stylesheet"
          href="/styles/default.css?bust=20230913?d=2023-09-16"
        />
        <link
          rel="stylesheet"
          href="/styles/docs.css?bust=20230915?d=2023-09-16"
        />
        <link rel="stylesheet" href="/styles/custom.css" />
        <link
          rel="preload"
          href="/assets/fonts/HKGrotesk-Regular.woff2"
          as="font"
        />
        <link
          rel="preload"
          href="/assets/fonts/HKGrotesk-Bold.woff2"
          as="font"
        />
        <link
          rel="preload"
          href="/assets/fonts/HKGrotesk-SemiBold.woff2"
          as="font"
        />
        <link
          rel="preload"
          href="/assets/fonts/CascadiaMono-Regular-Sub.woff2"
          as="font"
        />
        <link rel="preload" href="/assets/images/blur.webp" as="image" />
        {route === "/docs/packages/" && (
          <link
            rel="preload"
            href="https://packages.typst.org/preview/index.json"
            as="fetch"
            crossOrigin="anonymous"
          />
        )}
        <script
          dangerouslySetInnerHTML={{
            __html: `
          document.documentElement.className = document.documentElement.className.replace("no-js", "js");
          document.addEventListener("DOMContentLoaded", (() => {
            const e = document.cookie.split("; ").find((e => e.startsWith("INSECURE_SIGNED_IN=")))?.split("=")[1].toLowerCase();
            if ("1" === e || "true" === e || "yes" === e) {
              document.documentElement.classList.add("signed-in");
              const e = document.querySelector("header nav a.nav-btn"),
                    t = document.querySelector("header nav a.sign-in");
              e && (e.innerHTML = "Go to app", e.setAttribute("href", "/app/")),
              t && (t.innerHTML = "Sign out", t.setAttribute("href", "https://api.typst.app/v1/auth/logout"))
            }
          }))
        `,
          }}
        />
        <link href="/src/styles.css" rel="stylesheet"></link>
        <script
          defer
          src="https://cdn.jsdelivr.net/npm/alpinejs@3.14.8/dist/cdn.min.js"
        ></script>
      </head>

      <body class="no-js docs has-outline">
        <SiteNoticeBanner />
        <header class="boring">
          <div>
            <a href="/docs" class="logo-box">
              <svg
                width="91"
                height="35"
                viewBox="0 0 91 35"
                role="img"
                aria-labelledby="logo-label"
              >
                <title id="logo-label">Typst</title>
                <use href="/assets/images/typst.svg#logo"></use>
              </svg>
              <span>ドキュメント日本語版</span>
            </a>
            <button class="hamburger">
              <img
                alt="Open navigation"
                src="/assets/icons/16-hamburger-dark.svg"
                width="16"
                height="16"
              />
            </button>
            <nav>
              <ul>
                <li class="social">
                  <a
                    href="https://x.com/mkpoli/status/1802227029447725538"
                    title="関連ツイート"
                  >
                    <svg
                      width="16"
                      height="16"
                      role="img"
                      aria-labelledby="svg-twitter-label"
                    >
                      <title id="svg-twitter-label">Twitter</title>
                      <use href="/assets/icons/social.svg#twitter"></use>
                    </svg>
                  </a>
                </li>
                <li class="social">
                  <a
                    href="https://discord.gg/9xF7k4aAuH"
                    title="「くみはんクラブ」Discordサーバー"
                  >
                    <svg
                      width="16"
                      height="16"
                      role="img"
                      aria-labelledby="svg-discord-label"
                    >
                      <title id="svg-discord-label">Discord</title>
                      <use href="/assets/icons/social.svg#discord"></use>
                    </svg>
                  </a>
                </li>
                <li class="social">
                  <a
                    href="https://github.com/typst-jp/typst-jp.github.io"
                    title="本プロジェクトのGitHubレポジトリ"
                  >
                    <svg
                      width="16"
                      height="16"
                      role="img"
                      aria-labelledby="svg-github-label"
                    >
                      <title id="svg-github-label">GitHub</title>
                      <use href="/assets/icons/social.svg#github"></use>
                    </svg>
                  </a>
                </li>
                <li class="secondary">
                  <a href="https://typst.app/">Typst公式サイト</a>
                </li>
                <li class="secondary">
                  <a href="https://typst.app/docs/">Typst公式ドキュメント</a>
                </li>
              </ul>
            </nav>
          </div>
        </header>

        <div class="main-grid">
          <nav class="folding">
            <a href="/docs" class="title-row">
              <svg
                width="91"
                height="35"
                viewBox="0 0 91 35"
                role="img"
                aria-labelledby="logo-label-nav"
              >
                <title id="logo-label-nav">Typst</title>
                <use href="/assets/images/typst.svg#logo"></use>
              </svg>
              <span>ドキュメント日本語版</span>
            </a>
            <button class="close">
              <img
                alt="Close"
                src="/assets/icons/16-close-dark.svg"
                width="16"
                height="16"
              />
            </button>
            <div class="search">
              <img
                src="/assets/icons/16-search-gray.svg"
                alt="Search"
                width="16"
                height="16"
              />
              <input type="search" placeholder="検索" id="docs-search" />
            </div>
            <ul class="search-results hidden" id="search-results"></ul>
            <ul>
              {docs &&
                docs.map((firstLevel, idx) => (
                  <>
                    {firstLevel.part && (
                      <li class="category">{firstLevel.part}</li>
                    )}
                    <li>
                      <a
                        href={firstLevel.route}
                        aria-current={
                          firstLevel.route === route ? "page" : undefined
                        }
                      >
                        {firstLevel.title}
                      </a>
                      {firstLevel.children?.length > 0 && (
                        <>
                          <button>
                            <img
                              alt="Expand"
                              src="/assets/icons/16-arrow-right.svg"
                              width="16"
                              height="16"
                            />
                          </button>
                          <ul>
                            {firstLevel.children.map((secondLevel, idx2) => (
                              <>
                                {secondLevel.part && (
                                  <li class="category">{secondLevel.part}</li>
                                )}
                                <li>
                                  <a
                                    href={secondLevel.route}
                                    aria-current={
                                      secondLevel.route === route
                                        ? "page"
                                        : undefined
                                    }
                                  >
                                    {secondLevel.title}
                                  </a>
                                  {secondLevel.children?.length > 0 && (
                                    <>
                                      <button>
                                        <img
                                          alt="Expand"
                                          src="/assets/icons/16-arrow-right.svg"
                                          width="16"
                                          height="16"
                                        />
                                      </button>
                                      <ul>
                                        {secondLevel.children.map(
                                          (thirdLevel, idx3) => (
                                            <>
                                              {thirdLevel.part && (
                                                <li class="category">
                                                  {thirdLevel.part}
                                                </li>
                                              )}
                                              <li>
                                                <a
                                                  href={thirdLevel.route}
                                                  aria-current={
                                                    thirdLevel.route === route
                                                      ? "page"
                                                      : undefined
                                                  }
                                                >
                                                  {thirdLevel.title}
                                                </a>
                                              </li>
                                            </>
                                          ),
                                        )}
                                      </ul>
                                    </>
                                  )}
                                </li>
                              </>
                            ))}
                          </ul>
                        </>
                      )}
                    </li>
                  </>
                ))}
            </ul>
          </nav>

          <main>
            <Breadcrumbs path={path} />

            {children}

            {route === "/docs/" ? (
              <div class="doc-categories">
                <a class="doc-category" href="/docs/tutorial">
                  <img
                    src="/assets/icons/32-tutorial-c.svg"
                    width="32"
                    height="32"
                    alt="Circled play Icon"
                  />
                  <strong>チュートリアル</strong>
                  <p>一歩一歩、Typstの使い方を学びましょう。</p>
                </a>
                <a class="doc-category" href="/docs/reference">
                  <img
                    src="/assets/icons/32-reference-c.svg"
                    width="32"
                    height="32"
                    alt="Circled information icon"
                  />
                  <strong>リファレンス</strong>
                  <p>
                    Typstのあらゆる構文、概念、型、関数についての詳細なリファレンスです。
                  </p>
                </a>
              </div>
            ) : (
              previousPage &&
              nextPage && (
                <div class="page-end-buttons">
                  <a href={previousPage.route} class="previous">
                    <img src="/assets/icons/16-arrow-right.svg" alt="←" />
                    <div>
                      <span class="page-title">{previousPage.title}</span>
                      <span class="hint">前に戻る</span>
                    </div>
                  </a>
                  <a href={nextPage.route} class="next">
                    <img src="/assets/icons/16-arrow-right.svg" alt="→" />
                    <div>
                      <span class="page-title">{nextPage.title}</span>
                      <span class="hint">次に進む</span>
                    </div>
                  </a>
                </div>
              )
            )}
          </main>

          <TableOfContents outline={outline} />
        </div>

        <Footer />

        <script src="/scripts/fuse.basic.min.js"></script>
        <script src="/scripts/docs.js?bust=20230913"></script>
        <script src="/scripts/analytics.js" defer></script>
      </body>
    </html>
  );
};

export default BaseTemplate;
