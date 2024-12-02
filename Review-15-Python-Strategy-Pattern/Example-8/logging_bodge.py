import logging
import sys

# -------------------------------------------------------------------------------
# Set up logging before tictactoe.factory is imported... so that
# we can actually log module initialization
# -------------------------------------------------------------------------------
logger = logging.getLogger("tictactoe")
logger.setLevel(logging.WARN)

handler = logging.StreamHandler(sys.stderr)

handler.setFormatter(
    logging.Formatter("%(name)s - %(levelname)s - %(message)s")
)

logger.addHandler(handler)
