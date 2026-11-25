// Copyright: Anki Software, LLC and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::sync::Mutex;

use super::*;
use crate::services::SchedulerService;
use crate::tests::open_fs_test_collection;
use crate::tests::CardAdder;

/// [REGISTERED_MODEL] and [BUILDING] are process-wide: these tests take turns.
static SERIAL: Mutex<()> = Mutex::new(());

/// A registered model that is never loaded (no model file in CI).
fn register_model() {
    *REGISTERED_MODEL.lock().unwrap() = Some(
        OfflineModel {
            path: PathBuf::from("missing-rwkv-model.bin"),
            precision: StatePrecision::Int8,
        }
        .into(),
    );
}

fn building_for(col: &Collection) -> bool {
    BUILDING
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|(path, _)| *path == col.col_path)
}

#[test]
fn unload_forgets_the_model_and_its_scores() -> Result<()> {
    let _serial = SERIAL.lock().unwrap_or_else(|err| err.into_inner());
    let (mut col, _dir) = open_fs_test_collection("rwkv-unload");
    col.update_default_deck_config(|config| {
        config.rwkv_review_instant_order_enabled = true;
    });
    let card = CardAdder::new().add(&mut col)[0].id;
    col.set_due_date(&[card], "0", None)?;
    register_model();
    col.set_rwkv_review_queue_scores(DeckId(1), [(card, 0.5)].into())?;
    assert!(col.rwkv_offline_enabled());

    SchedulerService::rwkv_unload_offline(&mut col)?;

    assert!(!col.rwkv_offline_enabled());
    assert!(col.state.rwkv_offline.is_none());
    assert!(col.state.rwkv_retrievability_scores.is_none());
    // the next queue build uses the standard order and starts no build
    col.rwkv_offline_before_queue_build(DeckId(1));
    assert!(!building_for(&col));
    assert!(col.state.rwkv_retrievability_scores.is_none());
    Ok(())
}

#[test]
fn unload_detaches_a_build_under_way() -> Result<()> {
    let _serial = SERIAL.lock().unwrap_or_else(|err| err.into_inner());
    let (mut col, _dir) = open_fs_test_collection("rwkv-unload-build");
    register_model();
    let handle = std::thread::spawn(|| -> Result<Box<RwkvOfflineRuntime>> {
        invalid_input!("never installed")
    });
    *BUILDING.lock().unwrap() = Some((col.col_path.clone(), handle));

    SchedulerService::rwkv_unload_offline(&mut col)?;

    assert!(!building_for(&col));
    assert!(col.rwkv_offline_take_built().is_none());
    Ok(())
}

#[test]
fn unload_is_idempotent() -> Result<()> {
    let _serial = SERIAL.lock().unwrap_or_else(|err| err.into_inner());
    let (mut col, _dir) = open_fs_test_collection("rwkv-unload-twice");
    SchedulerService::rwkv_unload_offline(&mut col)?;
    SchedulerService::rwkv_unload_offline(&mut col)?;
    assert!(!col.rwkv_offline_enabled());
    Ok(())
}
