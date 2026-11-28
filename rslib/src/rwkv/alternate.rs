<<<<<<< HEAD
<<<<<<< HEAD
// Copyright: Anki Software, LLC and contributors
=======
// Copyright: Ankitects Pty Ltd and contributors
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
=======
// Copyright: Anki Software, LLC and contributors
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345
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
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
#[path = "sm40_tests.rs"]
mod sm40_tests;
