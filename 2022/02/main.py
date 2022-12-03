import sys

OPTION_POINTS = {"ROCK": 1, "PAPER": 2, "SCISSORS": 3}

OPTIONS = {
    "A": "ROCK",
    "B": "PAPER",
    "C": "SCISSORS",
    # "X": "ROCK",
    # "Y": "PAPER",
    # "Z": "SCISSORS",
}

ACTUAL_OUTCOMES = {
    "X": 0,
    "Y": 3,
    "Z": 6,
}


def get_player2_win(player1: str, player2: str) -> int:
    """
    Returns 0 if player1 wins, 6 if player2 wins, 3 if tie
    """
    # get winner
    player1_option = OPTIONS[player1]
    player2_option = OPTIONS[player2]

    if player1_option == player2_option:
        return 3

    if player1_option == "ROCK":
        if player2_option == "SCISSORS":
            return 0
        else:
            return 6
    elif player1_option == "PAPER":
        if player2_option == "ROCK":
            return 0
        else:
            return 6
    elif player1_option == "SCISSORS":
        if player2_option == "PAPER":
            return 0
        else:
            return 6


def get_player2_input(player1: str, outcome: int) -> str:
    """
    Returns the player2 input that will result in the given outcome
    """
    for option in OPTIONS:
        if get_player2_win(player1, option) == outcome:
            return option


def main():
    total_points = 0

    # read file input from args
    if not len(sys.argv) > 1:
        print("No file input")
        return
    file = sys.argv[1]
    with open(file, "r") as f:
        while line := f.readline():
            player_inputs = line[:-1].split(" ")
            points = 0
            player2_input = get_player2_input(
                player_inputs[0], ACTUAL_OUTCOMES[player_inputs[1]]
            )
            points += ACTUAL_OUTCOMES[player_inputs[1]]
            points += OPTION_POINTS[OPTIONS[player2_input]]
            total_points += points

    print(f"you had {total_points} points")


if __name__ == "__main__":
    main()
