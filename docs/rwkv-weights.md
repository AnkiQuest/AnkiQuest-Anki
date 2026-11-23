# RWKV weights: how a player gets them, and why nothing ships them

AnkiQuest never ships RWKV weights: no checkpoint of the SRS RWKV models
has a public licence, so no repository, release, APK or add-on of ours
carries one, and F-Droid (which requires everything it installs to be
licensed for redistribution) cannot include them either. The licence
history is tracked in
[open-spaced-repetition/srs-benchmark#353](https://github.com/open-spaced-repetition/srs-benchmark/issues/353):
the commonly used checkpoint `RWKV_trained_on_5000_10000.pth` was trained
on `anki-revlogs-10k`, whose licence permits memory research only, and
the V1-derived weights need written permission from JSchoreels and
1DWalker. `build.rs` and `.gitignore` keep any weights out of this
repository; never commit one.

## How other RWKV builds distribute weights

| Build | Weights |
|---|---|
| JSchoreels/anki (desktop) | Bundled: all models, about 54 MB, in `qt/aqt/rwkv_inference/` |
| Pedrubik2000/kuma3 (Android) | Bundled as APK assets, with JSchoreels' private permission, which does not grant redistribution |
| zaguk/RWKV4Anki (add-on) | Not bundled: `rwkv-models.lock.json` pins each model (name, SHA-256, a URL into zaguk/rwkv-srs at a fixed commit), and the user's first run downloads and verifies them |
| Pedrubik2000/anki-rwkv-forecast | None; it reads the JSchoreels fork's bundled models |
| AnkiQuest | Not bundled, none downloaded yet (no client enables RWKV: AnkiQuest/AnkiQuest-Android#433 records the plan) |

## How a player obtains a checkpoint

Until a licence lets us ship or point at one, a player who enables RWKV
in a local build supplies the file themself:

1. Take a checkpoint from the build's documentation: JSchoreels'
   [RWKV_FAQ.md](https://github.com/JSchoreels/anki/blob/main/RWKV_FAQ.md)
   for desktop, kuma3's `RWKV.md` for Android. The small models
   (SM92/SM98, about 331k parameters) are 1.35 MB f32 or 0.44 MB int8;
   the big model is about 11 MB. Prefer int8 on a phone (kuma3 14: 505
   MB instead of 1.69 GB RAM on a Pixel 8a).
2. Note the file's SHA-256 (`sha256sum <checkpoint>`) and which dataset it
   was trained on. A checkpoint trained on `anki-revlogs-10k` is for the
   player's own use only: research licence, no redistribution.
3. Place it where the enabled build reads it (the build reports the
   path; the desktop engine expects the folder JSchoreels'
   `rwkv_inference` uses) and confirm the build logs the model name and
   hash it loaded.

## The flow to build when a client enables RWKV

RWKV4Anki's is the pattern to copy, because it works without shipping
anything: a lock file pins each model (name, SHA-256, URL), the first
run downloads and hash-verifies it into the app's own storage, and the
release notes name the source and licence. Exact pins (URL plus SHA-256)
in one committed lock file, one download on first use, no bundled
weights, and the licence shown before the download starts. When the
written, redistributable licence exists, the lock file's URL becomes the
source we serve, and F-Droid unblocks (its metadata PR is
AnkiQuest/AnkiQuest-Android#320).
