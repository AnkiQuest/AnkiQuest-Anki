<<<<<<< HEAD
// Copyright: Anki Software, LLC and contributors
=======
// Copyright: Ankitects Pty Ltd and contributors
>>>>>>> a8cf880ce2907229d2edcadcd6977d855b7954a8
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

const SMALL_MODEL: bool = !super::SMALL_MODEL;
include!("engine.rs");
<<<<<<< HEAD
#[cfg(all(test, rwkv_vectors, feature = "rwkv-sm40"))]
#[path = "legacy_tests.rs"]
mod legacy_tests;
#[cfg(all(test, rwkv_vectors, not(feature = "rwkv-sm40")))]
=======
#[cfg(all(test, feature = "rwkv-sm40"))]
#[path = "legacy_tests.rs"]
mod legacy_tests;
#[cfg(all(test, not(feature = "rwkv-sm40")))]
>>>>>>> a8cf880ce2907229d2edcadcd6977d855b7954a8
#[path = "sm40_tests.rs"]
mod sm40_tests;
