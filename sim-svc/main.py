import json
import logging as log
import argparse as argparse
from component import Node, init_tree

log.basicConfig(level=log.INFO)


def main():
    root = Node("root", "root", [])
    init_tree(root)

    sprout = Node("root", "root", [Node("shoot", "shoot", [])])
    init_tree(sprout)

    sapling = Node("root", "root", [
        Node("trunk", "trunk", [
            Node("branch", "branch", [
                Node('leaf', "leaf", [])
            ]),
            Node("branch", "branch", [
                Node('leaf', "leaf", []), Node('leaf', "leaf", [])
            ])])
    ])
    init_tree(sapling)


if __name__ == "__main__":
    main()
