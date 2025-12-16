from pathlib import Path
import importlib
from ..year import Year

# Default input files locatin
_DEFAULT_INPUT_FILES = Path(__file__).parent.joinpath("..", "..", "..", "input", "2022")


year2022 = Year("2022", input_files=_DEFAULT_INPUT_FILES.absolute())

_submodules = [
    f"day{x}"
    for x in range(1, 24)
    if Path(__file__).parent.joinpath(f"day{x}.py").exists()
]
for _mod in _submodules:
    # Force import of each day to trigger day registration
    importlib.import_module(f".{_mod}", __package__)


__all__ = _submodules + [year2022]
