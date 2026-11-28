<<<<<<< HEAD
<<<<<<< HEAD
# Copyright: Anki Software, LLC and contributors
=======
# Copyright: Ankitects Pty Ltd and contributors
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
=======
# Copyright: Anki Software, LLC and contributors
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

from __future__ import annotations

import os
from pathlib import Path
<<<<<<< HEAD
<<<<<<< HEAD
from typing import Any, Callable
from unittest.mock import MagicMock, patch
=======
from unittest.mock import patch
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
=======
from typing import Any, Callable
from unittest.mock import MagicMock, patch
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345

import pytest

import aqt
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345
from aqt.profiles import ProfileManager, VideoDriver


class FakeApp:
    def secondInstance(self) -> bool:
        # makes _run() return right after creating the app
        return True


def get_fake_app_class(on_init: Callable) -> type[FakeApp]:
    class FakeAppSub(FakeApp):
        def __init__(self, *args: Any, **kwargs: Any) -> None:
            on_init()

    return FakeAppSub
<<<<<<< HEAD
=======
from aqt.profiles import ProfileManager
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
=======
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345


@patch.dict(os.environ)
def test_ui_scale_is_applied_before_app_is_created(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    # Qt reads QT_SCALE_FACTOR when the app is constructed, so setting it
    # afterwards has no effect (#5676)
<<<<<<< HEAD
<<<<<<< HEAD
=======
    monkeypatch.delenv("QT_SCALE_FACTOR", raising=False)
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345
    monkeypatch.setattr(ProfileManager, "uiScale", lambda self: 2.0)
    seen: list[str | None] = []
    fake_app_class = get_fake_app_class(
        lambda: seen.append(os.environ.get("QT_SCALE_FACTOR"))
    )
    monkeypatch.setattr(aqt, "AnkiApp", fake_app_class)
    monkeypatch.setattr(aqt, "setupGL", MagicMock())
<<<<<<< HEAD
=======
    monkeypatch.delenv("QT_SCALE_FACTOR", raising=False)
    monkeypatch.setattr(ProfileManager, "uiScale", lambda self: 2.0)
    seen: list[str | None] = []

    class FakeApp:
        def __init__(self, argv: list[str]) -> None:
            seen.append(os.environ.get("QT_SCALE_FACTOR"))

        def secondInstance(self) -> bool:
            # makes _run() return right after creating the app
            return True

    monkeypatch.setattr(aqt, "AnkiApp", FakeApp)
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
=======
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345

    aqt._run(["anki", "-b", str(tmp_path)], exec=False)

    assert seen == ["2.0"]
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345


@pytest.mark.parametrize(
    ("extra_args", "shift_held", "expected_driver"),
    [
        ([], False, VideoDriver.OpenGL),
        (["--safemode"], False, VideoDriver.Software),
        ([], True, VideoDriver.Software),
    ],
)
def test_run_applies_video_driver_before_app_is_created(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    extra_args: list[str],
    shift_held: bool,
    expected_driver: VideoDriver,
) -> None:
    # Some of setupGL()'s settings, such as AA_UseSoftwareOpenGL on macOS and
    # QT_OPENGL on Windows, are ignored once the app exists
    monkeypatch.setattr(ProfileManager, "video_driver", lambda self: VideoDriver.OpenGL)
    monkeypatch.setattr(aqt, "shift_held_before_app", lambda: shift_held)
    events: list[tuple[str, VideoDriver | None]] = []

    def fake_setup_gl(pm: Any, driver: VideoDriver | None = None) -> None:
        events.append(("setupGL", driver))

    fake_app_class = get_fake_app_class(lambda: events.append(("app created", None)))
    monkeypatch.setattr(aqt, "AnkiApp", fake_app_class)
    monkeypatch.setattr(aqt, "setupGL", fake_setup_gl)

    aqt._run(["anki", "-b", str(tmp_path), *extra_args], exec=False)

    assert events == [("setupGL", expected_driver), ("app created", None)]
<<<<<<< HEAD
=======
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
=======
>>>>>>> 9fe094a700d1815eb4cd7070637b5cb84399a345
