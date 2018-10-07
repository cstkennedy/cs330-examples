
def main():

    tom = Player("Thomas")
    jay = Player("Jay")

    game = Game(tom, jay)

    # while(!(game.getBoard().isFull())) {
    while game.isNotOver():
        game.playRound()

    print(game.getBoard())

    if game.endedWithWin():
        print("Congratulations {}!".format(game.getWinner()))


if __name__ == "__main__":
    main()

    # @todo add exception handling
