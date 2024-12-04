import logging
import sys


def set_up_logging(level: int = logging.WARN) -> None:
    logger = logging.getLogger("tictactoe")
    logger.setLevel(level)

    handler = logging.StreamHandler(sys.stderr)

    handler.setFormatter(
        logging.Formatter("%(name)s - %(levelname)s - %(message)s")
    )

    logger.addHandler(handler)


# -------------------------------------------------------------------------------
# Set up logging before tictactoe.factory is imported... so that
# we can actually log module initialization
# -------------------------------------------------------------------------------
#  set_up_logging(level=logging.INFO)
