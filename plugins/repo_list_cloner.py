from ast import arg
from os import system
from sys import argv


def get_repos(file_path: str, location_path: str):
    system("cd {}".format(location_path))
    with open(file_path) as fp:
        Lines = fp.readlines()
        for line in Lines:
            print("Cloning {}...".format(line.strip()))
            clone_repo(line.strip())


def clone_repo(url: str):
    system("git clone {}".format(url))


if __name__ == "__main__":
    print(argv[1], argv[2])
    get_repos(str(argv[1]), str(argv[2]))
