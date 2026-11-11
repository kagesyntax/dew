// Dew website. The version, release notes and download links all come from
// version.json, which the release script rewrites, so the page updates itself.
// Everything else is static: no framework, no build step, and it must still read
// when opened straight off disk (file://), where fetch and the app frame are absent.
(() => {
  const $ = (s) => document.querySelector(s);
  const $$ = (s) => document.querySelectorAll(s);
  const SPARK = '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 3l1.9 5.8L20 11l-6.1 2.2L12 19l-1.9-5.8L4 11l6.1-2.2z"/></svg>';

  // Which device is this? Only used to sort the download list and label the button.
  const ua = navigator.userAgent;
  const os = /android/i.test(ua) ? "android"
    : /iphone|ipad|ipod/i.test(ua) || (/Macintosh/.test(ua) && navigator.maxTouchPoints > 1) ? "ios"
    : /windows/i.test(ua) ? "windows"
    : /mac os x|macintosh/i.test(ua) ? "macos"
    : /linux|cros/i.test(ua) ? "linux" : "web";
  const label = {
    android: "Download for Android", ios: "Add to iPhone", windows: "Download for Windows",
    macos: "Download for Mac", linux: "Download for Linux", web: "Open Dew",
  }[os];

  // ---- Live demo -------------------------------------------------------------
  // The poster capture shows immediately; the frame fades in over it only after it
  // has actually loaded. On a phone, or off disk, we never mount the frame at all.
  const demo = $("#demo");
  const poster = $("#poster");
  const canRunDemo = demo && location.protocol !== "file:" && window.innerWidth > 900;
  if (demo && !canRunDemo) demo.remove();
  else if (demo) demo.addEventListener("load", () => { if (poster) poster.style.opacity = "0"; });

  // ---- Mobile sheet ----------------------------------------------------------
  const menu = $("#menu");
  const sheet = $("#sheet");
  const closeSheet = () => {
    document.body.classList.remove("open");
    menu.setAttribute("aria-expanded", "false");
  };
  if (menu && sheet) {
    menu.addEventListener("click", () => {
      const open = !document.body.classList.contains("open");
      document.body.classList.toggle("open", open);
      menu.setAttribute("aria-expanded", String(open));
    });
    sheet.addEventListener("click", (e) => { if (e.target.tagName === "A") closeSheet(); });
    document.addEventListener("keydown", (e) => { if (e.key === "Escape") closeSheet(); });
  }

  // ---- "Start over" clears only the demo's own sandbox ------------------------
  const restart = $("#restart");
  if (restart) {
    restart.addEventListener("click", () => {
      Object.keys(localStorage).filter((k) => k.startsWith("dew.demo.v1.")).forEach((k) => localStorage.removeItem(k));
      const frame = $("#demo");
      if (frame) frame.src = "app/";
    });
  }

  // ---- Release data ----------------------------------------------------------
  function render(v) {
    const dl = v.downloads || {};
    const repo = (v.url || "").split("/releases")[0];
    $$("[data-os]").forEach((row) => {
      const href = dl[row.dataset.os];
      if (href) row.href = href;
      else if (!["ios", "web"].includes(row.dataset.os)) row.href = v.url || row.href;
      if (row.dataset.os === os) row.classList.add("suggested");
    });
    const mine = $(`[data-os="${os}"]`);
    $$("[data-primary]").forEach((b, i) => {
      if (i > 0) b.textContent = label;
      b.href = mine && mine.getAttribute("href") !== "#" ? mine.getAttribute("href") : "#download";
    });
    $$("[data-version]").forEach((e) => (e.textContent = `Version ${v.version}`));
    $$("[data-version-short]").forEach((e) => (e.textContent = v.version));
    const d = new Date(v.published_at);
    const when = isNaN(d) ? "" : d.toLocaleDateString(undefined, { day: "numeric", month: "long", year: "numeric" });
    const line = $("[data-release-line]");
    if (line) line.textContent = `Version ${v.version}${when ? `, released ${when}` : ""}. Updates arrive inside the app.`;
    const date = $("[data-date]");
    if (date) date.textContent = when ? `Released ${when}` : "";
    const notes = $("#highlights");
    if (notes) notes.innerHTML = (v.highlights || [v.notes]).filter(Boolean)
      .map((h) => `<li>${SPARK}<span>${h.replace(/[<>&]/g, (c) => ({ "<": "&lt;", ">": "&gt;", "&": "&amp;" })[c])}</span></li>`).join("");
    if (repo) {
      $$("[data-repo]").forEach((a) => (a.href = repo));
      $$("[data-feedback]").forEach((a) => (a.href = `${repo}/issues/new`));
      $$("[data-notes]").forEach((a) => (a.href = `${repo}/releases`));
    }
  }

  fetch("version.json", { cache: "no-cache" }).then((r) => r.json()).then(render).catch(() => {
    // Off disk, or offline: keep whatever the HTML says.
    $$("[data-primary]").forEach((b, i) => { if (i > 0) b.textContent = label; });
  });
})();
