import { invoke } from "@tauri-apps/api/core";

// ─── State ──────────────────────────────────────────────
// Each split is a fractional width (they all sum to 1.0).
// We store raw weights and normalize on render / apply.
let splits = []; // array of numbers (weights)
const PHYSICAL_W = 3840;
const PHYSICAL_H = 1080;
const COLORS = ["#5b9bf7", "#6c6", "#f7a35b", "#c77dff", "#e55", "#5be5e5"];

const canvas = document.getElementById("layout-canvas");
const ctx = canvas.getContext("2d");

// ─── Normalize ──────────────────────────────────────────
function normalized() {
  const total = splits.reduce((a, b) => a + b, 0);
  if (total === 0) return [];
  return splits.map((s) => s / total);
}

// ─── Regions from splits ────────────────────────────────
function toRegions() {
  const fracs = normalized();
  let x = 0;
  return fracs.map((w) => {
    const r = { x: round(x), y: 0, width: round(w), height: 1.0 };
    x += w;
    return r;
  });
}
function round(n) { return Math.round(n * 10000) / 10000; }

// ─── Draw ───────────────────────────────────────────────
function draw() {
  const W = canvas.width, H = canvas.height;
  ctx.fillStyle = "#0a0a0a";
  ctx.fillRect(0, 0, W, H);

  const m = 16;
  const mW = W - m * 2, mH = H - m * 2;

  // Bezel
  ctx.strokeStyle = "#2a2a2a";
  ctx.lineWidth = 2;
  ctx.strokeRect(m, m, mW, mH);

  if (splits.length === 0) {
    ctx.fillStyle = "#555";
    ctx.font = "14px system-ui";
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.fillText("Add splits or pick a preset", W / 2, H / 2);
    return;
  }

  const fracs = normalized();
  let x = m;
  fracs.forEach((w, i) => {
    const rw = w * mW;
    const color = COLORS[i % COLORS.length];

    // Fill
    ctx.fillStyle = color + "20";
    ctx.fillRect(x + 1, m + 1, rw - 2, mH - 2);

    // Border
    ctx.strokeStyle = color + "88";
    ctx.lineWidth = 1.5;
    ctx.strokeRect(x + 1, m + 1, rw - 2, mH - 2);

    // Label
    const px = Math.round(w * PHYSICAL_W);
    ctx.fillStyle = color;
    ctx.font = "bold 13px system-ui";
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.fillText(`${px}×${PHYSICAL_H}`, x + rw / 2, H / 2 - 8);

    ctx.fillStyle = color + "99";
    ctx.font = "11px system-ui";
    ctx.fillText(`${Math.round(w * 100)}%`, x + rw / 2, H / 2 + 10);

    x += rw;
  });
}

// ─── Render split list ──────────────────────────────────
function renderSplitList() {
  const container = document.getElementById("split-list");
  if (splits.length === 0) {
    container.innerHTML = "";
    draw();
    return;
  }

  const fracs = normalized();
  container.innerHTML = splits
    .map((weight, i) => {
      const pct = Math.round(fracs[i] * 100);
      const px = Math.round(fracs[i] * PHYSICAL_W);
      return `<div class="split-row">
        <span class="label" style="color:${COLORS[i % COLORS.length]}">${i + 1}</span>
        <input type="range" min="1" max="100" value="${weight}" data-idx="${i}" />
        <span class="val">${px}px (${pct}%)</span>
      </div>`;
    })
    .join("");

  // Bind sliders
  container.querySelectorAll("input[type=range]").forEach((el) => {
    el.addEventListener("input", (e) => {
      splits[parseInt(e.target.dataset.idx)] = parseInt(e.target.value);
      renderSplitList();
    });
  });

  draw();
}

// ─── Apply to driver ────────────────────────────────────
async function applySplit() {
  if (splits.length === 0) return;
  const config = { monitor_index: 0, regions: toRegions(), preset_name: null };
  try {
    await invoke("apply_split", { config });
  } catch (e) {
    console.error("apply_split:", e);
  }
}

// Whenever splits change, auto-apply
function update() {
  renderSplitList();
  applySplit();
}

// ─── Preset definitions ─────────────────────────────────
const PRESETS = {
  "2h": [50, 50],
  "3h": [33, 34, 33],
  "21": [67, 33],
  "12": [33, 67],
  "121": [25, 50, 25],
  "grid": [50, 50], // grid would need rows; for now just 2-way
};

// ─── Event wiring ───────────────────────────────────────
document.getElementById("btn-add-split").addEventListener("click", () => {
  splits.push(splits.length === 0 ? 50 : splits[splits.length - 1]);
  update();
});

document.getElementById("btn-remove-last").addEventListener("click", () => {
  splits.pop();
  update();
});

document.getElementById("btn-reset").addEventListener("click", async () => {
  splits = [];
  try { await invoke("remove_all"); } catch (_) {}
  update();
});

document.querySelectorAll(".preset-chip").forEach((btn) => {
  btn.addEventListener("click", () => {
    const key = btn.dataset.preset;
    splits = [...PRESETS[key]];
    document.querySelectorAll(".preset-chip").forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");
    update();
  });
});

document.getElementById("btn-save").addEventListener("click", async () => {
  if (splits.length === 0) return;
  const name = prompt("Name:");
  if (!name) return;
  const hotkey = prompt("Hotkey (e.g. Ctrl+Alt+1, blank for none):");
  const preset = {
    name,
    config: { monitor_index: 0, regions: toRegions(), preset_name: name },
    hotkey: hotkey || null,
  };
  try { await invoke("save_preset", { preset }); } catch (e) { console.error(e); }
});

// ─── Init ───────────────────────────────────────────────
async function init() {
  try {
    await invoke("get_physical_monitors");
    const s = document.getElementById("driver-status");
    s.textContent = "Connected";
    s.classList.add("ok");
  } catch (_) {}
  draw();
}
init();
