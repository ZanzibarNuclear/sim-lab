class Node:
    id: int
    name: str
    type: str
    children: list

    next_id: int = 1

    def __init__(self, name, type, children):
        self.name = name
        self.type = type
        self.children = children

    def add_child(self, node):
        self.children.append(node)


def init_tree(node, level=0):
    node.id = Node.next_id
    Node.next_id += 1
    print(f"{level * '  '}{node.name} ({node.type} : {node.id})")
    for child in node.children:
        init_tree(child, level + 1)
