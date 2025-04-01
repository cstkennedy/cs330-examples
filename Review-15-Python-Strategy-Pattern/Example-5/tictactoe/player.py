import copy
from dataclasses import dataclass, field

from .board import RenderStrategy
from .strategy import Strategy

DEFAULT_NAME: str = "I. C. Generic"


@dataclass(kw_only=True, unsafe_hash=True)
class Player:
    """
    This is more a Player interface than a Player class.
    <p>
    However, such distinctions and discussions belong in
    the OOP and Inheritance Modules
    """

    name: str = field(default=DEFAULT_NAME, compare=True)
    strategy: Strategy = field(default=None, compare=False)  # type: ignore
    humanity: bool = field(default=False, compare=False)
    preferred_renderer: RenderStrategy = field(compare=False)

    def next_move(self) -> int:
        """
        Retrieve the next move.

        Returns:
            board cell id representing the selected move

        Raises:
            IOException if the move can not be retreived from the player.
        """

        return self.strategy.next_move()

    def is_human(self) -> bool:
        """
        Is this a Human Player?

        Returns:
            True if the player is a human
        """

        return self.humanity

    def is_computer(self) -> bool:
        """
        Is this a Computer Player?

        In this discussion, always no :(

        Returns:
            True if the player is a Cylon

        """

        return not self.is_human()

    def get_render_preference(self) -> RenderStrategy:
        return self.preferred_renderer

    def __str__(self):
        """
        Generate a player string, but only the name.
        """

        return self.name

    def __deepcopy__(self, memo) -> "Player":
        """
        Create a new duplicate Player.
        """

        cpy = Player(
            name=self.name,
            strategy=copy.deepcopy(self.strategy),
            preferred_renderer=self.preferred_renderer,
        )

        return cpy


def is_generic(possible_cylon: Player) -> bool:
    """
    Checks whether a player is a placeholder or
    an actual player.

    Args:
        possible_cylon (Player): player whose humanity is in question

    Returns:
        True if the player is a Cylon

    """

    return possible_cylon.name == DEFAULT_NAME
