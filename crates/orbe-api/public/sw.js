// Orbe service worker — app-shell cache only.
//
// Design goals (SPEC §G "instant"): after the first load the shell boots from
// cache, and API traffic always goes to the network — a CRM must never show
// stale stock.
//
// Bump CACHE when the shell strategy changes; old caches are dropped on activate.

const CACHE = "orbe-shell-v1";
const SHELL = [
  "/",
  "/main.css",
  "/manifest.json",
  "/icon-192.svg",
  "/icon-512.svg",
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE).then((cache) => cache.addAll(SHELL)).then(() => self.skipWaiting())
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) => Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k))))
      .then(() => self.clients.claim())
  );
});

self.addEventListener("fetch", (event) => {
  const req = event.request;

  // Only GET leaves this worker alone-ish: never cache mutations or API reads.
  if (req.method !== "GET") return;

  const url = new URL(req.url);
  if (url.origin !== self.location.origin) return;
  if (url.pathname.startsWith("/api/")) return; // API is always live

  event.respondWith(
    caches.match(req).then((hit) => {
      if (hit) return hit;
      return fetch(req)
        .then((res) => {
          // Cache successful same-origin shell assets (wasm, js, css, icons).
          if (res.ok && (url.pathname.startsWith("/wasm/") || /\\.(css|js|svg|json)$/.test(url.pathname))) {
            const copy = res.clone();
            caches.open(CACHE).then((cache) => cache.put(req, copy));
          }
          return res;
        })
        .catch(() => caches.match("/"));
    })
  );
});
