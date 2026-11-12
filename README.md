# Fake FPS

> *"They spend thousands on GPUs to chase a number on a screen. You don't have to."*

Let's be honest about something.

There is a whole culture out there — streamers, content creators, benchmark warriors — that has convinced itself the only way to prove a machine is worth anything is to push a framerate counter into the thousands. They upgrade from a 4090 to a 5090 not because the game runs badly, but because the number got bigger. They buy a 6090 next year for the same reason. They are not chasing smoothness. They already had smoothness at 60. They are chasing a number they can put on stream and say: *"Look what I have."*

And here's the uncomfortable truth: **60 FPS in a single-player game is already a smooth, comfortable, beautiful experience.** You don't need 1000 FPS to enjoy Cyberpunk, Elden Ring, or Black Myth. You never did. The framerate arms race isn't about the game — it's about the *performance*.

Fake FPS was born from that observation. Not to cheat anyone, not to fake benchmarks, not to deceive — but to give people who *already* have a perfectly good experience the ability to see what the number-lovers see, on their own terms, in their own offline world.

**This tool is for offline, single-player games only. It is not, and will never be, for online play.**

---

## What Fake FPS Actually Does

Fake FPS is a small, dependency-free launcher that modifies the framerate constant inside **RTSS** (RivaTuner Statistics Server) — the overlay most games already show in the corner of the screen. It does not inject a DLL into the game. It does not touch the game's memory. It does not alter gameplay in any way.

What it does is simpler and more elegant:

- It launches the game normally, exactly as you would.
- RTSS hooks the game on its own, as it always does.
- Fake FPS finds the framerate constant inside RTSS's hook module (from the outside, using `VirtualQueryEx` + `ReadProcessMemory`) and multiplies it by a scale you choose.
- The overlay now shows your real FPS multiplied by that scale — the game itself runs at its real speed, with its real performance, unchanged.

In other words: **the game is untouched. Only the number RTSS displays changes.**

---

## How It Works (Technical Overview)

Fake FPS is a single Rust binary with zero external dependencies beyond the Windows API. It is built with `#![windows_subsystem = "windows"]`, so no console window appears.

### Finding the constant

RTSS's hook module contains a very specific instruction sequence that computes the framerate:
64-bit: 48 69 C1 <imm32> 8B CA 33 D2 48 F7 F1
32-bit: BB <imm32> 85 F6 74 11 8D 41 01 F7 E3

Fake FPS walks the target's mapped images with `VirtualQueryEx`, reads them in 1 MB slices with enough overlap to catch matches spanning boundaries, and locates the `imm32` (the constant) inside this pattern.

**Why not `Toolhelp32Snapshot`?** Because module enumeration is bitness-locked: a 64-bit process cannot enumerate modules of a 32-bit target (it gets `ERROR_PARTIAL_COPY`). But `VirtualQueryEx` and `ReadProcessMemory` work across bitness. This is the core trick that lets **one 64-bit launcher patch both 32-bit and 64-bit games**.

### Applying the patch

Once the site is found, Fake FPS writes `10,000,000 × scale` to it using `WriteProcessMemory` (after `VirtualProtectEx` and `FlushInstructionCache`). It then polls every 400 ms to keep the constant at the desired value — because RTSS occasionally resets it, and because the slider can move while the game is running.

### Following launchers

Many games start through their own launcher (`PlayGTAV.exe` → `GTA5.exe`, EA/Ubisoft wrappers, and so on). Fake FPS watches the process tree and patches every child process that lives inside the game's folder.

---

## Features

- **One window, one job.** Pick the game `.exe`, set the scale with a slider, press Play.
- **No DLL injection.** Nothing is copied into the game folder. Nothing is injected.
- **Bitness-agnostic.** One 64-bit launcher patches both 32-bit and 64-bit games.
- **Launcher-aware.** Follows child processes started by the game's own launcher.
- **Persistent state.** Scale, last game, and the game shelf are remembered in the registry.
- **Minimal footprint.** A single `fps.ini` with one line (`scale=`) is created if none exists — and never touched again.
- **Custom UI.** A hand-painted dark interface with a slider, a game shelf, and per-tile icons extracted directly from the game `.exe` files.
- **CLI mode.** `--play <path>` launches without a window; `--patch <pid>` attaches to an existing process.

---

## Performance Notes

Fake FPS itself is intentionally lightweight:

- **CPU:** The patch loop sleeps 400 ms between ticks. During a 56-hour session, this translates to roughly 500,000 wake-ups — well within acceptable overhead. The `find_site` scan is the heaviest operation, and it runs only every ~2 seconds until the constant is located, then stops.
- **Memory:** The process holds a small set of handles and a few dozen KB of working set. It does not keep the game's memory mapped.
- **Game impact:** None. The game's memory is never touched. RTSS's own overhead is unchanged — Fake FPS only changes the number RTSS reports, not how it reports it.
- **Stability:** Tested in continuous sessions exceeding 56 hours with no crashes, no handle leaks, and no interference with the game or the overlay.

---

## How to Use

1. Make sure **RTSS is running**. Fake FPS will not work without it.
2. Open **Fake FPS**.
3. Click **Browse** and select the game's `.exe` — or add it to the shelf with the **+** button.
4. Set the **scale** using the slider or by typing a value between 1.0 and 10.0.
5. Press **PLAY**.
6. Keep the window open until the game reaches its menu. After that, you can close it — the patch thread continues in the background.
7. Right-click a tile on the shelf to remove it. Click to select. Double-click to play.

---

## Supported Games

Fake FPS works with **offline, single-player games** that RTSS already hooks. This includes (but is not limited to):

- Cyberpunk 2077
- Elden Ring
- Black Myth: Wukong
- Red Dead Redemption 2
- The Witcher 3
- Hogwarts Legacy
- Baldur's Gate 3
- Any DirectX 11/12 or Vulkan game that RTSS supports

If RTSS shows an overlay for the game, Fake FPS can scale it.

---

## ⚠️ Online Play — Read This

**Do not use Fake FPS in online multiplayer games.**

This is not a suggestion. It is a hard rule. Fake FPS modifies the memory of the RTSS hook module inside the game's process. While this does not alter gameplay, online anti-cheat systems (Vanguard, EAC, BattlEye) monitor memory writes to game processes and may treat any such write — even to a third-party overlay — as a violation.

- **Single-player:** ✅ Safe.
- **Online multiplayer:** ❌ Do not use. Account bans are possible and will not be reversed.

Fake FPS was built for people who want to enjoy their offline games. It was never built for competitive play, and it will never be supported there.

---

## Known Limitations (Beta)

- **RTSS version dependency.** The byte patterns above are matched against RTSS 7.3.5.28314. A future RTSS update may change the instruction sequence and require an updated pattern.
- **No restore button yet.** If Fake FPS is closed while a patch is active, RTSS will continue showing the multiplied value until the game restarts. A "Restore" button is planned.
- **Fixed window layout.** The UI does not scale with DPI. Use 100% display scaling for the intended appearance.
- **Windows 10 1809+ recommended.** Dark title bar attributes are applied on Windows 10 1809 and later; older builds will fall back gracefully.

---

## Roadmap

- [ ] **Restore button** — reset RTSS to its original value (`10,000,000`) without restarting the game.
- [ ] **Hotkey toggle** — `Ctrl+Alt+F` to enable/disable the multiplier during play.
- [ ] **RTSS version check** — warn the user when the detected RTSS version is outside the tested range.
- [ ] **Reduced permission footprint** — request only `VM_READ | VM_WRITE | VM_OPERATION` in the patch loop.
- [ ] **DPI-aware layout.**
- [ ] **Log file** for troubleshooting.

---

## Philosophy

Fake FPS is not a cheat. It is not a benchmark tool. It is not a way to deceive anyone.

It is a small, honest answer to a small, honest observation: **the framerate arms race is not about the game.** It never was. It is about the number — and the number is a story people tell themselves and each other.

If you already have a smooth, comfortable experience at 60 FPS, you already have everything the game has to offer. Fake FPS lets you see the story that the number-lovers tell — on your own terms, in your own offline world, without spending a single dollar on a GPU you don't need.

Enjoy your games. Enjoy the number. But remember: **the game was always the point.**

---

## License

**Mit license**

You are free to use, study, modify, and redistribute this software under the terms of the GNU General Public License, version 3 or later. Any derivative work must remain open-source and must credit the original author.

See the [`LICENSE`](LICENSE) file for the full text.

---

## Credits

Built with Rust, the Windows API, and a stubborn refusal to install a dependency.

*Fake FPS — because the number was never the point.*
