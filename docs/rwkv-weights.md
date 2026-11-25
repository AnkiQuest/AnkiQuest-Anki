# RWKV weights: how a player gets them, and why nothing ships them

AnkiQuest never ships RWKV weights: no checkpoint of the SRS RWKV models
has a public licence, so no repository, release, APK or add-on of ours
carries one, and F-Droid (which requires everything it installs to be
licensed for redistribution) cannot include them either. The licence
history is tracked in
[open-spaced-repetition/srs-benchmark#353](https://github.com/open-spaced-repetition/srs-benchmark/issues/353)
and summarised, with who works on what, in
[rwkv-landscape.md](rwkv-landscape.md):

- every model was trained on `anki-revlogs-10k`, directly or through the
  srs-benchmark checkpoint used as teacher, and that dataset's licence
  permits research use; other uses need permission from Anki;
- srs-benchmark, which holds `RWKV_trained_on_5000_10000.pth`, has no
  licence file;
- JSchoreels' `WEIGHTS-NOTICE.md` excludes his weights from the code's MIT
  licence and says to use them for personal and research use only, and to
  ask the dataset's owners before shipping them in an application.

AnkiQuest asked both sources for permission
([JSchoreels/anki#29](https://github.com/JSchoreels/anki/issues/29),
[srs-benchmark#364](https://github.com/open-spaced-repetition/srs-benchmark/issues/364));
neither answered, and both point to #353. `.gitignore` keeps any weights
out of this repository; never commit one.

## How other RWKV builds distribute weights

SRS RWKV builds, as of 2026-11-25:

| Build | Where the weights come from | Size | Integrity check | Licence or permission |
|---|---|---|---|---|
| [JSchoreels/anki-rwkv](https://github.com/JSchoreels/anki-rwkv/releases) | GitHub release per model or family (`rwkv-family-2026-11-13`: six zips, `SHA256SUMS`, `WEIGHTS-NOTICE.md`, `INTEGRATION.md`); the notice also names Hugging Face uploads | Big zip 23.6 MB, Small 2.9 to 4.0 MB, Tiny 3.8 to 3.9 MB | `SHA256SUMS`; each package's `manifest.json` holds per-file SHA-256; `check_testvectors.py` must print `RESULT: PASS` | Code MIT; weights not licensed |
| [JSchoreels/anki](https://github.com/JSchoreels/anki) (desktop) | Bundled in `qt/aqt/rwkv_inference/` (one folder per model: `weights.f32.bin`, `weights.int8.bin`, `architecture.json`, `manifest.json`, `time_head.*`) and in every installer; a copy of `WEIGHTS-NOTICE.md` beside each | Big `weights.f32.bin` 11,082,637 B; Tiny f32 1,270,749 B, int8 409,483 B; installers 217 to 345 MB | `manifest.json` records the source checkpoint's SHA-256; the docs record each file's hash; installers' SHA-256 in the release notes | Anki's AGPL for code; weights under the notice |
| [Pedrubik2000/kuma3-anki-app](https://github.com/Pedrubik2000/kuma3-anki-app) (Android) | Bundled as APK assets (`assets/rwkv/{big,small,tiny}{,-instant}.bin`), copied to app storage on first use; only the chosen size stays installed | 11,082,637 B (Big), 1,351,553 B (Small), 1,270,749 B (Tiny) each | Size check against a constant in `RwkvOffline.kt`; APK SHA-256 in the release notes | "shipped in kuma3 with his permission" (`RWKV.md`); no redistribution grant |
| [zaguk/RWKV4Anki](https://github.com/zaguk/RWKV4Anki) (stock Anki add-on) | Bundled in each `.ankiaddon`. The packager downloads the three models pinned in `rwkv-models.lock.json` (name, SHA-256, raw URL into zaguk/rwkv-srs at a fixed commit) at build time; "Anki does not download the runtime or models after installation" | about 11 MB per model (V1 safetensors 11,112,152 B, V5o-a 10,360,156 B) | Packager checks each SHA-256; releases carry `SHA256SUMS` and `RELEASE_PROVENANCE.json`, checked by `sha256sum --check` in CI | "No license is granted"; permission to redistribute obtained privately (rwkv-srs `PROVENANCE.md`) |
| [Pedrubik2000/anki-rwkv-forecast](https://github.com/Pedrubik2000/anki-rwkv-forecast) | None; uses the JSchoreels fork's models | none | none | MIT, no weights |
| AnkiQuest | None bundled, none downloaded. The Android app has a switch and an engine seam (AnkiQuest/AnkiQuest-Android#442) with no engine and no model source yet | none | none | none |

The earlier version of this guide said RWKV4Anki downloads its models on the
player's first run. It does not: the lock file is read by the packager, and the
models ship inside the add-on.

General RWKV language models, for comparison:

| Project | Hosting | Size | Integrity | Licence | Download |
|---|---|---|---|---|---|
| [BlinkDL/ChatRWKV](https://github.com/BlinkDL/ChatRWKV), [RWKV-LM](https://github.com/BlinkDL/RWKV-LM) | Hugging Face (`BlinkDL/*`, `RWKV/*`), `.pth` | 0.1B to 14B parameters, GB-sized | Hugging Face's per-file SHA-256 (LFS) | Code and weights Apache-2.0 ([rwkv-7-world model card](https://huggingface.co/BlinkDL/rwkv-7-world)) | Manual |
| [RWKV/rwkv.cpp](https://github.com/RWKV/rwkv.cpp) | None; the user downloads a `.pth` from Hugging Face and converts and quantises it locally | as above | none | Code MIT | Manual, then convert |
| [josStorer/RWKV-Runner](https://github.com/josStorer/RWKV-Runner) (desktop) | `manifest.json` lists 183 models with `size`, `SHA256` and a Hugging Face or ModelScope URL; optional hf-mirror.com | 3,055,445,757 B for the 1.5B `.pth` | Manifest has SHA-256, but the frontend uses it only to de-duplicate list entries; no check of a downloaded file was found | Code MIT; weights per model | In-app |
| [RWKV-APP/RWKV_APP](https://github.com/RWKV-APP/RWKV_APP) (Android, iOS, desktop) | A remote config (`remote/<build>.json`) lists each model's Hugging Face path, `fileSize`, optional `sha256`, platforms and backends; mirrors ModelScope, aifasthub, hf-mirror, Hugging Face | 0.2 to 1.7 GB quantised | Downloads are checked by file size; the hash is used for telemetry ids, not checked | Code Apache-2.0 | In-app |

Pattern: projects whose weights are licensed host them on Hugging Face and
download in-app from a manifest; the SRS RWKV projects, whose weights are not
licensed, bundle them under private permission. None of the in-app downloaders
read here verifies a SHA-256 before use; RWKV4Anki verifies at build time.

## How a player obtains a checkpoint

Until a licence lets us ship or point at one, a player who enables RWKV
in a local build supplies the file themself, for their own use:

1. Download a family release from
   [JSchoreels/anki-rwkv releases](https://github.com/JSchoreels/anki-rwkv/releases)
   (the newest is `rwkv-family-2026-11-13`), with its `SHA256SUMS` and
   `WEIGHTS-NOTICE.md`. Read the notice. The engine AnkiQuest-Anki carries
   (`rslib/src/scheduler/rwkv/offline_engine.rs`) reads the fork's
   `ARWKVWEIGHTS1` `.bin` files, not `.pth` or safetensors: a two-head model
   and an Instant model of one size. Prefer Small or Tiny on a phone: Big is
   11 MB per file and needs far more state memory (kuma3 14: the int8 Small
   model used 505 MB of RAM for 22,000 cards on a Pixel 8a).
2. Verify the download: `sha256sum --check SHA256SUMS` in the folder
   holding the zips. After unpacking, compare each file with the SHA-256 in
   the package's `manifest.json`, and run the package's
   `check_testvectors.py`; it must print `RESULT: PASS`.
3. Place the files where the build reads them (the build reports the path;
   the desktop fork reads `qt/aqt/rwkv_inference/<model>/`) and confirm the
   build logs the model name and hash it loaded.

## Recommendation for AnkiQuest

| Option | Licence today | APK and F-Droid | Verdict |
|---|---|---|---|
| Bundle in the APK or add-on | Not allowed: no redistribution grant | +2.5 to 22 MB per size; F-Droid refuses non-free assets; against the slim-APK plan | No |
| In-app download from a pinned anki-rwkv release, verified by SHA-256 | Open question 4 in [rwkv-landscape.md](rwkv-landscape.md): needs written permission from JSchoreels and the dataset owners first | No APK growth; F-Droid would likely mark it with an anti-feature for non-free add-ons | Yes, once permitted |
| Manual: the player picks files they downloaded | Allowed: the player's own research use, our app distributes nothing | No APK growth; no F-Droid issue | Yes, now |

Ship the manual flow first and keep the download path behind it: both use
the same lock file and the same verification, so the download is a second
source, not a second implementation. Switch the download on only after
JSchoreels and the dataset's owners confirm in writing that a client may
fetch the weights for its players; link that confirmation from the pull
request.

## What the app needs

One lock file, committed in AnkiQuest-Android (and the add-on if desktop
gets RWKV), per model: id (`sm118`), size, kind (two-head or Instant), file
name, byte size, SHA-256, the anki-rwkv release tag and asset URL, and the
notice URL. It is the list of models the engine accepts, whatever the source.

Manual flow:

- A file picker (Storage Access Framework, `ACTION_OPEN_DOCUMENT`) behind
  `RwkvModelSource`, replacing `NoRwkvModel`.
- Copy into app-private storage, hash while copying, and accept the file
  only when size and SHA-256 match a lock-file entry; name the model from
  that entry. Reject anything else with the reason.
- Keep both models of the chosen size; delete the others.

Download flow, in addition:

- Show the weights notice and the download size, and require consent
  before the first byte; on a metered network ask again.
- Download only the lock file's URLs over HTTPS, to a temporary file;
  resume on interruption; check size and SHA-256 before an atomic rename.
  A mismatch deletes the file and reports it; never load an unverified
  file.
- Share the downloader with the runtime companion-art packs: both fetch
  a pinned release asset by hash.
- An F-Droid build flag that hides the download (`INTERNET` is already
  declared for the AnkiQuest server).
- A model update is a lock-file change in an app release. Key the model
  state by the model's SHA-256, as the desktop fork keys its state cache,
  so a new model rebuilds state.
</content>
</invoke>
