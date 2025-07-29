import json
import os
import sys
import logging as log
import argparse as argparse
from component import Node, show_tree

log.basicConfig(level=log.INFO)


class Config:
    def __init__(self, config_path):
        self.config_path = config_path

    def load(self):
        with open(self.config_path, "r") as f:
            self.config = json.load(f)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", type=str, required=True)
    args = parser.parse_args()

    config = Config(args.config)
    config.load()

    log.info(f"Starting simulation service with config: {config.config}")

    root = Node(0, "root", "root", [])
    for node in config.config["nodes"]:
        root.add_child(Node(node["id"], node["name"], node["type"], []))

    show_tree(root)


if __name__ == "__main__":
    main()
