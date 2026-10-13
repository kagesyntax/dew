// Dew website. Everything here comes from version.json, which every release rewrites, so the
// page updates itself: version, date, What's New and the download links.
(() => {
  const $ = (s) => document.querySelector(s);
  const $$ = (s) => document.querySelectorAll(s);
  const SPARK = '<svg viewBox="0 0 24 24"><path d="M12 3l1.9 5.8L20 11l-6.1 2.2L12 19l-1.9-5.8L4 11l6.1-2.2z"/></svg>';

  // Which device is this? Android, iOS, Windows, Mac or Linux.
  const ua = navigator.userAgent;
  const os = /android/i.test(ua) ? "android"
    : /iphone|ipad|ipod/i.test(ua) || (/Macintosh/.test(ua) && navigator.maxTouchPoints > 1) ? "ios"
    : /windows/i.test(ua) ? "windows"
    : /mac os x|macintosh/i.test(ua) ? "macos"
    : /linux|cros/i.test(ua) ? "linux" : "web";
  const label = { android: "Download for Android", ios: "Add to iPhone", windows: "Download for Windows", macos: "Download for Mac", linux: "Download for Linux", web: "Open Dew" }[os];

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
      b.href = mine ? mine.getAttribute("href") : "#download";
      if (os === "ios") b.href = "#download";
    });
    $$("[data-version]").forEach((e) => (e.textContent = `Version ${v.version}`));
    $$("[data-version-short]").forEach((e) => (e.textContent = `Dew ${v.version}`));
    const d = new Date(v.published_at);
    const when = isNaN(d) ? "" : d.toLocaleDateString(undefined, { day: "numeric", month: "long", year: "numeric" });
    $("[data-release-line]").textContent = `Version ${v.version}${when ? `, released ${when}` : ""}. Updates arrive inside the app.`;
    $("[data-date]").textContent = when ? `Released ${when}` : "";
    $("#highlights").innerHTML = (v.highlights || [v.notes]).filter(Boolean)
      .map((h) => `<li>${SPARK}<span>${h.replace(/[<>&]/g, (c) => ({ "<": "&lt;", ">": "&gt;", "&": "&amp;" })[c])}</span></li>`).join("");
    if (repo) {
      $$("[data-repo]").forEach((a) => (a.href = repo));
      $$("[data-feedback]").forEach((a) => (a.href = `${repo}/issues/new?template=feedback.yml`));
      $$("[data-notes]").forEach((a) => (a.href = `${repo}/releases`));
    }
  }

  fetch("version.json", { cache: "no-cache" }).then((r) => r.json()).then(render).catch(() => {
    $$("[data-primary]").forEach((b, i) => { if (i > 0) b.textContent = label; });
  });

  // "Start over": wipe only the demo's sandbox (the real web app keeps its own data) and reload.
  $("#restart").addEventListener("click", () => {
    Object.keys(localStorage).filter((k) => k.startsWith("dew.demo.v1.")).forEach((k) => localStorage.removeItem(k));
    $("#demo").src = "app/";
  });
})();
