from io import StringIO
import sys
from component import Node, init_tree


class TestNode:
    def test_node_construction_no_children(self):
        node = Node("test_node", "leaf", [])

        assert node.id == 1
        assert node.name == "test_node"
        assert node.type == "leaf"
        assert node.children == []

    def test_node_construction_with_children(self):
        child1 = Node("child1", "child", [])
        child2 = Node("child2", "child", [])
        parent = Node("parent", "parent", [child1, child2])

        assert parent.id == 1
        assert parent.name == "parent"
        assert parent.type == "parent"
        assert len(parent.children) == 2
        assert parent.children[0] == child1
        assert parent.children[1] == child2

    def test_node_construction_nested_children(self):
        grandchild = Node("grandchild", "leaf", [])
        child = Node("child", "branch", [grandchild])
        parent = Node("parent", "root", [child])

        assert parent.children[0].children[0] == grandchild
        assert parent.children[0].children[0].name == "grandchild"

    def test_add_child(self):
        parent = Node("parent", "parent", [])
        child = Node("child", "child", [])

        parent.add_child(child)

        assert len(parent.children) == 1
        assert parent.children[0] == child


class TestShowTree:
    def setup_method(self):
        # Reset next_id before each test
        Node.next_id = 1

    def test_show_tree_single_node(self, capsys):
        node = Node("root", "root", [])

        init_tree(node)

        captured = capsys.readouterr()
        assert "root (root : 1)" in captured.out

    def test_show_tree_with_children(self, capsys):
        child1 = Node("child1", "leaf", [])
        child2 = Node("child2", "leaf", [])
        parent = Node("parent", "root", [child1, child2])

        init_tree(parent)

        captured = capsys.readouterr()
        lines = captured.out.strip().split('\n')
        assert "parent (root : 1)" in lines[0]
        assert "  child1 (leaf : 2)" in lines[1]
        assert "  child2 (leaf : 3)" in lines[2]

    def test_show_tree_nested_children(self, capsys):
        grandchild = Node("grandchild", "leaf", [])
        child = Node("child", "branch", [grandchild])
        parent = Node("parent", "root", [child])

        init_tree(parent)

        captured = capsys.readouterr()
        lines = captured.out.strip().split('\n')
        assert "parent (root : 1)" in lines[0]
        assert "  child (branch : 2)" in lines[1]
        assert "    grandchild (leaf : 3)" in lines[2]

    def test_show_tree_assigns_sequential_ids(self, capsys):
        nodes = [
            Node("node1", "type1", []),
            Node("node2", "type2", []),
            Node("node3", "type3", [])
        ]
        root = Node("root", "root", nodes)

        init_tree(root)

        captured = capsys.readouterr()
        lines = captured.out.strip().split('\n')
        assert ": 1)" in lines[0]  # root gets id 1
        assert ": 2)" in lines[1]  # first child gets id 2
        assert ": 3)" in lines[2]  # second child gets id 3
        assert ": 4)" in lines[3]  # third child gets id 4
