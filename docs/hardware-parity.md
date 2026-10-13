# Hardware parity with Microsoft Word

> **Last reviewed:** 2026-10-12 · **Last updated:** 2026-10-13 · **Change:** minor (iPad hardware keyboard via GameController; previously trivial: 32-bit Windows starts on OpenGL) · **Target:** Microsoft Word (Microsoft 365) for Mac 16.113.4 and Word for Windows

> **Printing update:** 2026-10-12 · direct desktop printing through craft-print; details below.

Word is not a hardware-heavy app, but a few devices matter to its users: printers above all, then
pens, microphones (Dictate) and speakers (Read Aloud), and displays.

**Dimension: ~40% (estimated), 20–40 h to full** (pen/ink hours are counted with the Draw tab in
[`target-app-parity.md`](target-app-parity.md)).

| Hardware | Word | WordCraft macOS | Windows | Linux / BSD | Web | Parity | Hours |
|---|---|---|---|---|---|---|---|
| Printers: native print dialog, printer choice, copies, duplex, page ranges, tray, scaling | ✅ (`WordPDE.plugin` print-dialog extension on Mac) | 🟡 Direct CUPS printing; printer, copies, collation, duplex, ranges and grayscale | 🟡 Direct spooler printing; printer, copies, collation, duplex, ranges and grayscale | 🟡 Direct spooler printing; printer, copies, collation, duplex, ranges and grayscale | ✅ browser print dialog (#209) | 35% | 5–8 |
| GPU rendering, HiDPI/Retina, pixel-aligned text | ✅ | ✅ wgpu + vello_cpu rasteriser, whole-pixel text (#110, #165) | ✅ DirectX 12 default (#50), OpenGL when DirectX 12 has no GPU driver (#316) or on 32-bit; startup crash on Intel UHD (#170) | ✅ (soft text on KDE at some zooms, #140) | ✅ | 80% | 3–5 (bugs) |
| Multiple monitors, per-monitor DPI, remembered window position | ✅ | ✅ (#38, #76) | ✅ (winit DPI fix pending, #155) | ✅ | — | 85% | 1 |
| Pen / stylus: ink, pressure, eraser, ink to shape/math | ✅ (Draw tab, `InkRender.bundle`) | ❌ | ❌ | ❌ | ❌ | 0% | (Draw tab, 15–25) |
| Microphone: Dictate | ✅ (cloud speech service) | ❌ | ❌ | ❌ | ❌ | 0% | 8–15 + owner (speech model choice) |
| Speakers: Read Aloud | ✅ | ✅ system speech, speed, sentence skip (#190) | ✅ | 🟡 | 🟡 | 75% | 1–2 |
| Touch screens and touchpads: pinch zoom, inertial scroll | ✅ | ✅ pinch (#179) | ✅ | ✅ inertial scroll (#252) | ✅ | 85% | 0–1 |
| Scanners/cameras (Insert from device / Continuity Camera on Mac) | ✅ | ❌ | ❌ | ❌ | ❌ | 0% | 2–4 |

## iPadOS (fork)

Only what has been demonstrated. On the iPad Air 11-inch (M3) Simulator with iPadOS 26.2 (2026-10-13): the
Debug app builds for Apple Silicon, installs, launches, shows the Start screen in landscape and lays out inside
the safe area (UIKit bottom inset 20 pt, top inset 32 pt while the status bar shows, then 0). Printing, folder
access and the system clipboard are not wired up.

User-reported on a physical iPad Air 13-inch (M3), Debug build (2026-10-13; not instrumented): Open and Save
through the Files picker work, and touch works. The same device and a Logitech combo keyboard: typing works, but
Enter, the arrow keys and Cmd+A did nothing, and most buttons did not respond. Cause of the keys: winit's UIKit
view only receives text and Backspace from a keyboard. The runner now reads the hardware keyboard through
GameController (`gc_keyboard.rs`, translated in `hw_keys.rs`, unit-tested) and injects Enter, arrows, Home/End,
Page Up/Down, Delete, Esc, Tab, F-keys, Cmd/Ctrl shortcuts and the modifier state, with key repeat. Not yet
verified on a device. Known gap: Option/Cmd+Backspace (delete word/line) is left to UIKit's plain
Backspace. The unresponsive buttons are not explained yet. Not verified: Apple Pencil, multitasking, Release builds. The on-screen keyboard is
a low priority.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-13 | minor | iPadOS fork section: user-reported keyboard failures (Enter, arrows, Cmd+A) and the GameController hardware-keyboard path |
| 2026-10-13 | minor | iPadOS fork section: simulator launch and safe-area layout, plus user-reported Files open/save and touch on a physical iPad |
| 2026-10-12 | trivial | 32-bit Windows starts on OpenGL: DirectX 12 graphics pipeline creation returns "The parameter is incorrect" and the app exited before the window appeared |
| 2026-10-12 | minor | Desktop File > Print now discovers printers and submits PDF jobs through pinned craft-print with printer choice, copies, duplex and grayscale; browser printing and PDF export remain available. The upstream preview, page ranges, parity and Print Markup remain available. Remaining print parity includes driver-specific tray/options and pages-per-sheet UI; physical printer fidelity needs hardware validation. |
| 2026-10-10 | trivial | Windows falls back to OpenGL when DirectX 12 offers only the software adapter (#316) |
| 2026-10-10 | trivial | Desktop File › Print opens the PDF in the system viewer (#286) |
| 2026-10-10 | trivial | Inertial touchpad scrolling landed (#252) |
| 2026-10-10 | major | First version |
