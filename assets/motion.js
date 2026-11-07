// Dew motion engine, ported from the district site (agn-lagos/assets/motion.js).
// Rust/Dioxus owns the DOM and state; this file only drives scroll- and pointer-linked visuals
// that need 60fps. Call AGN.mount() after every route render (done from Rust via document::eval).
(function () {
  const reduce = matchMedia('(prefers-reduced-motion: reduce)').matches;
  const clamp = (v, a = 0, b = 1) => Math.min(Math.max(v, a), b);
  let raf = 0, io = null, co = null, mx = 0, my = 0, els = null, mounted = false;

  addEventListener('pointermove', e => {
    mx = e.clientX / innerWidth - .5;
    my = e.clientY / innerHeight - .5;
  }, { passive: true });

  function qa(s) { return [...document.querySelectorAll(s)]; }
  function q(s) { return document.querySelector(s); }

  function collect() {
    return {
      lnav: q('.lnav'),
      hero: q('#heroFrame'),
      floaters: qa('[data-depth]'),
      pars: qa('[data-par]'),
      progress: q('#readProgress'),
      article: q('#article'),
      tabbar: q('.tab-bar'),
    };
  }

  // One pass: stagger-reveal everything marked .rv as it enters.
  function observe() {
    io && io.disconnect();
    co && co.disconnect();
    io = new IntersectionObserver(es => es.forEach(e => {
      if (e.isIntersecting) { e.target.classList.add('in'); io.unobserve(e.target); }
    }), { threshold: .12, rootMargin: '0px 0px -6% 0px' });
    qa('.rv:not(.in)').forEach(el => io.observe(el));

    // Count-up numbers (data-count), once.
    co = new IntersectionObserver(es => es.forEach(e => {
      if (!e.isIntersecting) return;
      const el = e.target, to = +el.dataset.count, s = performance.now(), d = 1200;
      const f = n => {
        const p = Math.min((n - s) / d, 1);
        el.textContent = Math.round(to * (1 - Math.pow(1 - p, 3)));
        if (p < 1) requestAnimationFrame(f);
      };
      reduce ? (el.textContent = to) : requestAnimationFrame(f);
      co.unobserve(el);
    }), { threshold: .4 });
    qa('[data-count]').forEach(el => co.observe(el));
  }

  function frame() {
    const E = els, h = innerHeight;

    // Hero art grows and squares up as it rises into view.
    if (E.hero && !reduce) {
      const hp = clamp(1 - E.hero.getBoundingClientRect().top / h);
      E.hero.style.transform = `scale(${.94 + hp * .06})`;
      E.hero.style.borderRadius = `${28 * (1 - hp * .4)}px`;
    }

    // Reading progress for the reader and long lesson pages.
    if (E.progress && E.article) {
      const r = E.article.getBoundingClientRect();
      E.progress.style.transform = `scaleX(${clamp(-r.top / Math.max(r.height - h, 1))})`;
    }

    // Sticky bar lifts off the page once content scrolls under it.
    if (E.lnav) E.lnav.classList.toggle('stuck', scrollY > 8);

    if (!reduce) {
      E.floaters.forEach(f => {
        const d = +f.dataset.depth;
        if (!d) return;
        const r = f.parentElement.getBoundingClientRect();
        f.style.transform = `translate3d(${mx * 46 * d}px,${my * 46 * d + r.top * d * .3}px,0)`;
      });
      E.pars.forEach(im => {
        const b = im.parentElement.getBoundingClientRect();
        im.style.transform = `translateY(${(b.top / h - .5) * -6}%)`;
      });
    }
    raf = requestAnimationFrame(frame);
  }

  function mount() {
    cancelAnimationFrame(raf);
    els = collect();
    observe();
    raf = requestAnimationFrame(frame);
    mounted = true;
  }

  // Route changes replace the DOM under us; re-bind on the next frames.
  function remount() {
    requestAnimationFrame(() => requestAnimationFrame(mount));
  }

  function scrollCarousel(id, dir) {
    const sc = document.getElementById(id);
    if (!sc) return;
    const card = sc.firstElementChild;
    const step = card ? card.offsetWidth + 16 : 300;
    sc.scrollBy({ left: dir * step, behavior: 'smooth' });
  }

  window.AGN = { mount, remount, scrollCarousel };
  if (document.readyState !== 'loading') remount(); else addEventListener('DOMContentLoaded', remount);
})();
