EXPECTED_BAKE_TIME = 40
PREPARATION_TIME = 2

def bake_time_remaining(elapsed_bake_time):
    """ Takes elapsed bake time and returns bake time remaining"""
    return EXPECTED_BAKE_TIME - elapsed_bake_time


def preparation_time_in_minutes(number_of_layers):
    """Takes number of layers and returns prep time in mins"""
    return number_of_layers * PREPARATION_TIME


def elapsed_time_in_minutes(number_of_layers,elapsed_bake_time):
    """Takes number of layers and elapsed bake time; returns prep time in mins"""

    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time
