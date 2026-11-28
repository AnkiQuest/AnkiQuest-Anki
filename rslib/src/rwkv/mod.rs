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

//! Both verified RWKV architectures, with one specialized engine selected per
//! profile.

// Both engines intentionally compile the same sources with different dimensions.
#![allow(clippy::duplicate_mod)]
// Rust 1.99 suggests `as_chunks` for constant-size `chunks_exact`. The RWKV
// kernels keep their verified, bit-exact loops until they are migrated and
// re-benchmarked separately.
#![allow(clippy::chunks_exact_to_as_chunks)]

// The feature controls only the primary engine used by the offline benchmark
// API.
const SMALL_MODEL: bool = cfg!(feature = "rwkv-sm40");
<<<<<<< HEAD
// kuma3's offline runtime runs the small model through this one
// (scheduler/rwkv/offline_engine.rs)
pub(crate) mod alternate;
mod answer_time;
=======
mod alternate;
mod answer_time;
mod answer_time_recal;
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
mod api;
pub mod paired;
mod runtime;
pub use answer_time::AnswerTimeHistory;
pub use answer_time::AnswerTimePrediction;
pub use answer_time::ANSWER_TIME_BINS;
pub use answer_time::ANSWER_TIME_FEATURES;
<<<<<<< HEAD
=======
pub use answer_time_recal::AnswerTimeRecalibration;
pub use answer_time_recal::TimedEvents;
pub use answer_time_recal::ANSWER_TIME_RECAL_SECONDS;
pub use runtime::AnswerTimeRecalibrator;
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
pub use runtime::ModelKind;
pub use runtime::RwkvRuntime;
pub use runtime::RwkvRuntimeState;
include!("engine.rs");
#[cfg(test)]
mod app_speed_bench;
#[cfg(test)]
mod checkpoint_compression_bench;
<<<<<<< HEAD
#[cfg(all(test, rwkv_vectors, not(feature = "rwkv-sm40")))]
#[path = "legacy_tests.rs"]
mod legacy_tests;
#[cfg(test)]
mod model_size_bench;
#[cfg(all(test, rwkv_vectors, feature = "rwkv-sm40"))]
=======
#[cfg(all(test, not(feature = "rwkv-sm40")))]
#[path = "legacy_tests.rs"]
mod legacy_tests;
#[cfg(test)]
mod long_card_tests;
#[cfg(test)]
mod model_size_bench;
#[cfg(all(test, feature = "rwkv-sm40"))]
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
#[path = "sm40_tests.rs"]
mod sm40_tests;

#[cfg(all(test, feature = "rwkv-sm40"))]
mod curve_reference_probe;
