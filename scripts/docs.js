const hoverQuery = window.matchMedia("(hover: hover)");
let isHover = hoverQuery.matches;
hoverQuery.addEventListener("change", t=>{
    isHover = t.matches
}
);
const hasTransitionEnd = "ontransitionend"in window
  , prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches
  , collapseFactory = (t,e,s)=>(o,n)=>{
    e.removeEventListener("transitionend", s),
    n && (e.style.maxHeight = `${o}px`),
    window.requestAnimationFrame(()=>{
        e.style.pointerEvents = "none",
        e.style.userSelect = "none",
        e.style.maxHeight = "0px",
        e.style.opacity = "0"
    }
    ),
    t.setAttribute("aria-expanded", "false")
}
  , expandFactory = (t,e)=>{
    const s = ()=>{
        e.style.maxHeight = "none"
    }
    ;
    return {
        expand: (o,n)=>{
            hasTransitionEnd && !prefersReducedMotion && n ? (e.addEventListener("transitionend", s, {
                once: !0
            }),
            e.style.maxHeight = `${o}px`,
            e.style.pointerEvents = "auto",
            e.style.userSelect = "auto",
            e.style.opacity = "1") : s(),
            t.setAttribute("aria-expanded", "true")
        }
        ,
        handler: s
    }
}
  , toggleFactory = (t,e)=>{
    const {expand: s, handler: o} = expandFactory(t, e)
      , n = collapseFactory(t, e, o);
    return {
        toggle: l=>{
            t.getAttribute("aria-expanded") === "true" ? n(l, !0) : s(l, !0)
        }
        ,
        expand: s,
        collapse: n
    }
}
;
function setUpAccordeon(t) {
    const e = Array.from(t.children)
      , s = e.some(n=>n.querySelector("li > a[aria-current]"));
    let o = e.filter(n=>n.tagName === "LI").reduce((n,l)=>{
        const r = l.getElementsByTagName("button")
          , a = l.getElementsByTagName("ul")
          , p = l.getElementsByTagName("a");
        if (r.length === 0 || a.length === 0)
            return n;
        const h = a[0];
        return n.push({
            button: r[0],
            child: h,
            target: l,
            anchor: p.length > 0 ? p[0] : null
        }),
        n
    }
    , []);
    return o.length === 0 || (o.forEach(n=>{
        const l = setUpAccordeon(n.child)
          , r = n.anchor.getAttribute("aria-current") === "page";
        setUpSingleAccordeon(n.target, n.button, n.child, l || r, !1)
    }
    ),
    t.classList.add("animated")),
    s
}
function setUpSingleAccordeon(t, e, s, o, n=!1) {
    const {toggle: l, expand: r, collapse: a} = toggleFactory(t, s);
    s.style.overflow = "visible hidden";
    let p = s.offsetHeight;
    o ? r(p, !1) : a(p, !1),
    e.addEventListener("click", h=>{
        n && h.preventDefault(),
        l(p)
    }
    )
}
function isElementInViewport(t) {
    const e = t.getBoundingClientRect();
    return e.top >= 0 && e.left >= 0 && e.bottom <= (window.innerHeight || document.documentElement.clientHeight) && e.right <= (window.innerWidth || document.documentElement.clientWidth)
}
function setUpOnPageNavigation(t) {
    const e = Array.from(t.querySelectorAll("li[data-assoc]")).reduce((o,n)=>{
        const l = n.getAttribute("data-assoc")
          , r = n.getElementsByTagName("a")
          , a = document.getElementById(l);
        return a && r.length > 0 && o.push({
            item: n,
            assoc: a,
            anchor: r[0]
        }),
        o
    }
    , [])
      , s = ()=>{
        const o = e.find(n=>isElementInViewport(n.assoc));
        o && (e.forEach(n=>n.item.removeAttribute("aria-current")),
        o.item.setAttribute("aria-current", "true"))
    }
    ;
    s(),
    window.addEventListener("scroll", s),
    window.addEventListener("resize", s)
}
function setUpTooltip(t) {
    const e = t.querySelector("svg")
      , s = t.querySelector("div[role='tooltip']");
    let o = !1;
    const n = e.getElementsByTagName("title")[0];
    let l = n.textContent
      , r = 0;
    const a = 256
      , p = ()=>{
        const m = window.innerWidth
          , c = e.getBoundingClientRect();
        c.left + a / 2 > m ? (s.style.left = `${m - a - c.left - 32}px`,
        s.classList.add("mobile")) : c.left - a / 2 < 0 ? (s.style.left = `${-c.left + 32}px`,
        s.classList.add("mobile")) : (s.style.left = "-120px",
        s.classList.remove("mobile")),
        o = !0,
        n.innerHTML = "",
        s.style.display = "block",
        window.requestAnimationFrame(()=>{
            s.style.opacity = "1"
        }
        ),
        r = Date.now()
    }
      , h = ()=>{
        n.innerHTML = l,
        o = !1;
        const m = ()=>{
            s.style.display = "none"
        }
        ;
        hasTransitionEnd && !prefersReducedMotion ? s.addEventListener("transitionend", m, {
            once: !0
        }) : m(),
        window.requestAnimationFrame(()=>{
            s.style.opacity = "0",
            hasTransitionEnd && !prefersReducedMotion && s.removeEventListener("transitionend", m)
        }
        )
    }
    ;
    e.addEventListener("click", ()=>{
        isHover || (o && Date.now() - r > 100 ? h() : p())
    }
    ),
    e.addEventListener("mouseenter", m=>{
        m.preventDefault(),
        p()
    }
    ),
    e.addEventListener("mousemove", m=>{
        o && m.preventDefault()
    }
    ),
    e.addEventListener("focus", p),
    e.addEventListener("blur", h),
    e.addEventListener("mouseleave", h),
    window.addEventListener("click", m=>{
        o && !t.contains(m.target) && h()
    }
    ),
    window.addEventListener("mouseleave", h),
    window.addEventListener("keydown", m=>{
        o && m.key === "Escape" && h()
    }
    )
}
function alertClose(t) {
    t.addEventListener("click", () => {
        var alertElement = document.querySelector(".alert");
            alertElement.style.display = "none";
    }
    )
}
function setUpCollapsingSidebar(t) {
    const e = document.querySelector("button.hamburger")
      , s = t.querySelector("button.close");
    t.classList.add("mobile-hidden"),
    t.style.opacity = "1";
    const o = a=>{
        t.classList.contains("mobile-hidden") || t.contains(a.target) || (l(),
        a.preventDefault())
    }
      , n = ()=>{
        t.classList.add("mobile-hidden"),
        t.style.transform = "translateX(0)"
    }
      , l = ()=>{
        window.removeEventListener("click", o),
        hasTransitionEnd && !prefersReducedMotion ? (t.style.transform = "translateX(-100%)",
        t.addEventListener("transitionend", n, {
            once: !0
        })) : n()
    }
      , r = ()=>{
        t.removeEventListener("transitionend", n),
        t.style.transform = "translateX(-100%)",
        t.classList.remove("mobile-hidden"),
        requestAnimationFrame(()=>{
            window.addEventListener("click", o),
            t.style.transform = "translateX(0)"
        }
        )
    }
    ;
    e.addEventListener("click", r),
    s.addEventListener("click", l)
}
function copyText(t) {
    if ("clipboard"in navigator) {
        navigator.clipboard.writeText(t);
        return
    } else {
        const e = document.createElement("input");
        e.value = t,
        document.body.appendChild(e),
        e.select(),
        document.execCommand("copy"),
        document.body.removeChild(e)
    }
}
function setUpSymbolFlyout(t) {
    const e = document.getElementById("flyout-template")
      , s = document.getElementById("flyout-sym-row");
    if (!t || !e || !s)
        return;
    if (!("content"in document.createElement("template"))) {
        console.warn("Browser does not support template elements");
        return
    }
    const o = e.content.firstElementChild.cloneNode(!0);
    o.style.display = "none",
    t.appendChild(o);
    const n = o.querySelector(".info button")
      , l = o.querySelector(".info button .sym")
      , r = o.querySelector(".sym-name code")
      , a = o.querySelector(".info .unic-name")
      , p = o.querySelector(".info .codepoint span")
      , h = o.querySelector(".info .accent")
      , m = o.querySelector(".info .accent img")
      , c = o.querySelector(".variants-box")
      , d = o.querySelector(".variants-box .symbol-grid")
      , f = o.querySelector(".shorthand")
      , u = f.querySelector(".remark")
      , y = o.querySelector(".shorthand code")
      , E = o.querySelector(".sym-name .copy")
      , T = o.querySelector(".shorthand .copy")
      , x = o.querySelector(".codepoint .copy")
      , L = "/assets/icons/16-close.svg"
      , N = "/assets/icons/16-check.svg";
    let w;
    const b = ()=>{
        const i = document.getElementById(w);
        i && (i.ariaHasPopup = "false"),
        w = void 0,
        o.style.display = "none",
        window.removeEventListener("click", k);
        for (const {target: g, listener: v} of C)
            g.removeEventListener("click", v)
    }
      , k = i=>{
        w !== void 0 && w !== i.target.id && !o.contains(i.target) && (b(),
        i.preventDefault())
    }
    ;
    let C = [];
    const A = i=>{
        i.ariaHasPopup = "true",
        w = i.id,
        o.style.display = "block";
        const g = i.id.replace(/^symbol-/, "")
          , v = i.dataset.unicName
          , M = parseInt(i.dataset.codepoint, 10)
          , H = i.dataset.accent != null
          , z = i.dataset.alternates ? i.dataset.alternates.split(" ") : []
          , P = i.dataset.mathShorthand
          , F = i.dataset.markupShorthand
          , S = F || P
          , D = String.fromCodePoint(M)
          , R = i.dataset.override;
        copyText(D);
        let q = M.toString(16).toUpperCase();
        q.length < 4 && (q = "0".repeat(4 - q.length) + q),
        o.classList.toggle("override", R != null),
        l.textContent = R ?? D,
        r.textContent = g,
        a.textContent = v,
        p.textContent = q,
        h.style.display = H ? "block" : "none",
        m.src = H ? N : L,
        m.setAttribute("alt", H ? "Yes" : "No"),
        f.style.display = S && S.length > 0 ? "block" : "none",
        y.textContent = S;
        let $ = ()=>{
            copyText(g)
        }
        ;
        if (E.addEventListener("click", $),
        C.push({
            target: E,
            listener: $
        }),
        S && S.length > 0) {
            let B = ()=>{
                copyText(S)
            }
            ;
            T.addEventListener("click", B),
            C.push({
                target: T,
                listener: B
            }),
            u && (P && !F ? (u.textContent = "(only in math)",
            u.style.display = "inline") : !P && F ? (u.textContent = "(only in markup)",
            u.style.display = "inline") : u.style.display = "none")
        }
        let O = ()=>{
            copyText("\\u{" + q + "}")
        }
        ;
        if (x.addEventListener("click", O),
        C.push({
            target: x,
            listener: O
        }),
        c !== null && d !== null) {
            const B = z.map(U=>{
                const W = s.content.firstElementChild.cloneNode(!0)
                  , Y = W.querySelector(".sym")
                  , V = W.querySelector("button")
                  , I = document.getElementById(`symbol-${U}`)
                  , X = I?.dataset.override;
                if (I)
                    V.classList.toggle("override", X != null),
                    Y.textContent = X ?? String.fromCodePoint(parseInt(I.dataset.codepoint, 10));
                else
                    return null;
                return V.addEventListener("click", J=>{
                    A(I),
                    I.scrollIntoView(),
                    J.preventDefault()
                }
                ),
                W
            }
            ).filter(U=>U != null);
            c.style.display = B.length > 0 ? "block" : "none",
            d.replaceChildren(...B)
        }
        let j = i.offsetLeft - 12;
        const Q = i.offsetTop - 12;
        i.getBoundingClientRect().left + 408 > window.innerWidth && (j = Math.max(8, window.innerWidth - t.getBoundingClientRect().left - 424)),
        o.style.left = `${j}px`,
        o.style.top = `${Q}px`
    }
    ;
    n.addEventListener("click", b),
    window.addEventListener("keydown", i=>{
        i.key === "Escape" && w !== void 0 && (b(),
        i.preventDefault())
    }
    );
    for (let i = 0; i < t.children.length; i++) {
        const g = t.children[i];
        g.tagName === "LI" && g.addEventListener("click", v=>{
            A(g),
            v.preventDefault()
        }
        )
    }
    window.requestAnimationFrame(()=>{
        window.removeEventListener("click", k),
        window.addEventListener("click", k, {
            capture: !0
        })
    }
    )
}
function setUpSymbolSearch() {
    const t = document.querySelector("main > .symbol-grid")
      , e = document.getElementById("symbol-search");
    if (!t || !e)
        return;
    const s = Array.from(t.children).filter(p=>p.tagName === "LI")
      , o = s.map(p=>({
        name: p.id.replace(/^symbol-/, ""),
        unicName: p.dataset.unicName,
        symbol: String.fromCodePoint(parseInt(p.dataset.codepoint, 10)),
        shorthand: p.dataset.shorthand
    }))
      , n = new Fuse(o,{
        keys: [{
            name: "symbol",
            weight: .95
        }, {
            name: "name",
            weight: .9
        }, {
            name: "unicName",
            weight: .7
        }, {
            name: "shorthand",
            weight: .5
        }],
        threshold: .3,
        ignoreLocation: !0
    })
      , l = ()=>{
        if (e.value.length === 0) {
            s.forEach(h=>h.style.display = "block");
            return
        }
        const p = n.search(e.value).map(h=>h.item.name);
        s.forEach(h=>{
            p.includes(h.id.replace(/^symbol-/, "")) ? h.style.display = "block" : h.style.display = "none"
        }
        )
    }
    ;
    e.addEventListener("input", l),
    e.addEventListener("keydown", l);
    const a = new URLSearchParams(window.location.search).get("query");
    a && (e.value = a),
    l()
}
async function setUpSearch(t, e) {
    const s = new AbortController
      , o = setTimeout(()=>s.abort(), 1e4)
      , n = await fetch("/assets/search.json?bust=20230915", {
        signal: s.signal
    }).then(r=>r.ok ? r.json() : null).catch(r=>(console.error(r),
    null));
    if (clearTimeout(o),
    !n)
        return;
    const l = ()=>{
        let r = []
          , a = [];
        const p = t.value
          , h = t.value.toLowerCase().split(/[!-/:-@[-`{-~\s]/g).filter(c=>c.length > 0);
        function m(c) {
            if (r.includes(c))
                return;
            const d = n.items[c];
            if (a[c] = score(d, p),
            r.length < 10) {
                r.push(c);
                return
            }
            let f = 0;
            for (let u = 0; u < r.length; u++)
                a[r[u]] < a[r[f]] && (f = u);
            if (a[c] > a[r[f]]) {
                r[f] = c;
                return
            }
        }
        for (const c of h) {
            let d = 0
              , f = n.words.length - 1;
            for (; d < f; ) {
                let y = Math.floor((d + f) / 2);
                if (n.words[y].startsWith(c)) {
                    d = y;
                    break
                } else
                    c < n.words[y] ? f = y - 1 : d = y + 1
            }
            let u = d;
            for (; u > 0 && n.words[u].startsWith(c); )
                u--;
            for (u++; u < n.words.length && n.words[u].startsWith(c); ) {
                for (const y of n.hits[u])
                    m(y);
                u++
            }
        }
        r.sort((c,d)=>a[d] - a[c]),
        e.replaceChildren(...r.map(c=>{
            const d = n.items[c];
            let f = d.route;
            d.kind == "Symbols" && (f += "?query=" + p);
            const u = document.createElement("li")
              , y = document.createElement("a")
              , E = document.createElement("span");
            return y.href = f,
            y.textContent = d.title,
            E.classList.add("type"),
            E.textContent = d.kind,
            y.appendChild(E),
            u.appendChild(y),
            u
        }
        )),
        e.classList.toggle("hidden", r.length === 0)
    }
    ;
    t.addEventListener("keydown", l),
    t.addEventListener("keyup", l),
    l()
}
const FACTORS = {
    Type: 1,
    Function: .9,
    Parameter: .8,
    Method: .7,
    Symbols: .6,
    Chapter: .5,
    Category: .5
};
function score(t, e) {
    return (FACTORS[t.kind.split(" ")[0]] || .4) * Math.max(stringScore(t.title, e), ...(t.keywords || []).map(o=>stringScore(o, e)))
}
function stringScore(t, e) {
    const s = simplify(t)
      , o = simplify(e);
    return s.includes(o) ? 100 - t.length : 0
}
function simplify(t) {
    return t.toLowerCase().replaceAll(/[!-/:-@[-`{-~\s]/g, "")
}
async function setUpPackages() {
    const t = document.querySelector("main > .package-list > tbody")
      , e = document.getElementById("package-search")
      , s = document.getElementById("package-show-all-versions");
    if (!t || !e)
        return;
    const n = await (await fetch("https://packages.typst.org/preview/index.json")).json();
    // 对 n 中的每一项 name 为 key 的项设置 description 为 value，不存在则不替换
    const index2ja = await (await fetch("/assets/index2ja.json")).json();
    if (index2ja) {
      n.forEach(item => {
          if (index2ja[item.name]) {
              item.description = index2ja[item.name];
          }
      });
    }
    const l = new Fuse(n,{
        keys: [{
            name: "name",
            weight: .9
        }, {
            name: "keywords",
            weight: .8
        }, {
            name: "description",
            weight: .6
        }],
        threshold: .3,
        ignoreLocation: !0
    });
    let r = [];
    function a() {
        for (const {target: d, listener: f} of r)
            d.removeEventListener("click", f);
        r = [];
        let m;
        e.value.length === 0 ? m = n : m = l.search(e.value).map(d=>d.item),
        t.replaceChildren();
        const c = s.checked;
        for (let d = 0; d < m.length; d++) {
            const {name: f, version: u, description: y, repository: E, homepage: T} = m[d]
              , x = T || E;
            if (!c && d + 1 < m.length && f === m[d + 1].name)
                continue;
            const L = document.createElement("tr")
              , N = ()=>{
                copyText(`#import "@preview/${f}:${u}"`)
            }
              , w = document.createElement("td")
              , b = document.createElement("a");
            b.href = `https://github.com/typst/packages/tree/main/packages/preview/${f}/${u}`,
            b.textContent = f,
            w.appendChild(b),
            L.appendChild(w);
            const k = document.createElement("td");
            k.textContent = y,
            L.appendChild(k);
            const C = document.createElement("td");
            C.textContent = u,
            L.appendChild(C);
            const A = document.createElement("td")
              , i = document.createElement("button");
            i.classList.add("copy"),
            i.innerHTML = '<img width="16" height="16" alt="Copy" src="/assets/icons/16-copy.svg">',
            i.addEventListener("click", N),
            A.appendChild(i),
            r.push({
                target: i,
                listener: N
            }),
            L.appendChild(A);
            const g = document.createElement("td");
            if (x) {
                const v = document.createElement("a");
                v.href = x,
                v.innerHTML = '<img width="16" height="16" alt="Copy" src="/assets/icons/16-link.svg">',
                g.appendChild(v)
            }
            L.appendChild(g),
            t.appendChild(L)
        }
    }
    e.addEventListener("input", a),
    e.addEventListener("keydown", a),
    s.addEventListener("input", a);
    const h = new URLSearchParams(window.location.search).get("query");
    h && (e.value = h),
    a()
}
document.addEventListener("DOMContentLoaded", ()=>{
    for (const n of document.querySelectorAll("nav.folding > ul"))
        setUpAccordeon(n);
    const t = document.querySelector("#page-overview > ul");
    t && setUpOnPageNavigation(t),
    setUpCollapsingSidebar(document.querySelector("nav.folding"));
    alertClose(document.getElementById("closeAlertButton"));
    for (const n of document.querySelectorAll("div.tooltip-context"))
        setUpTooltip(n);
    const e = document.querySelector(".page-end-buttons a.previous")
      , s = document.querySelector(".page-end-buttons a.next");
    window.addEventListener("keydown", n=>{
        document.querySelectorAll("textarea:focus, input:focus").length > 0 || (n.key === "ArrowLeft" && e ? window.location.href = e.href : n.key === "ArrowRight" && s && (window.location.href = s.href))
    }
    );
    for (const n of document.querySelectorAll(".previewed-code > pre"))
        n.clientWidth < n.scrollWidth && n.classList.add("big");
    setUpPackages(),
    setUpSearch(document.getElementById("docs-search"), document.getElementById("search-results"));
    const o = ()=>{
        for (const n of document.querySelectorAll("main > .symbol-grid"))
            setUpSymbolFlyout(n);
        setUpSymbolSearch()
    }
    ;
    window.requestIdleCallback ? window.requestIdleCallback(o) : o()
}
);
