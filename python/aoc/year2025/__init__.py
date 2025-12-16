from pathlib import Path
from ..year import Year
import importlib

# Location of input files
_DEFAULT_INPUT_FILES = Path(__file__).parent.joinpath("..", "..", "..", "input", "2025")

year2025 = Year("2025", input_files=_DEFAULT_INPUT_FILES)

_submoduldes = [
    f"day{x}"
    for x in range(1, 24)
    if Path(__file__).parent.joinpath(f"day{x}.py").exists()
]

# Force input of each day to trigger decorator registration
for _mod in _submoduldes:
    importlib.import_module(f".{_mod}", __package__)

__all__ = _submoduldes + [year2025]
