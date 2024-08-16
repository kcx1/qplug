from enum import Enum

num = "0.0.0"


class VersionIncrement(Enum):
    Major: str = 0
    Minor: str = 1
    Patch: str = 2


def update_version(current_version: str, increment: VersionIncrement) -> str:
    current_version = current_version.split(".")
    # match increment:
    #     case VersionIncrement.Patch:
    #         current_version[2] = str(int(current_version[2]) + 1)
    #         return current_version.join(".")
    #     case VersionIncrement.Minor:
    #         current_version[1] = str(int(current_version[1]) + 1)
    #         current_version[2] = "0"
    #         return current_version.join(".")
    #     case VersionIncrement.Major:
    #         current_version[0] = str(int(current_version[0]) + 1)
    #         current_version[1] = "0"
    #         current_version[2] = "0"
    #         return current_version.join(".")

    for i in range(len(current_version)):
        if increment.value < i:
            pass
        elif increment.value == i:
            current_version[i] = str(int(current_version[i]) + 1)
        else:
            current_version[i] = "0"

    return current_version
