from io import StringIO
import sys
from component import Node, show_tree


class TestNode:
    def test_node_construction_no_children(self):
        node = Node(1, "test_node", "leaf", [])

        assert node.id == 1
        assert node.name == "test_node"
        assert node.type == "leaf"
        assert node.children == []

    def test_node_construction_with_children(self):
        child1 = Node(2, "child1", "child", [])
        child2 = Node(3, "child2", "child", [])
        parent = Node(1, "parent", "parent", [child1, child2])

        assert parent.id == 1
        assert parent.name == "parent"
        assert parent.type == "parent"
        assert len(parent.children) == 2
        assert parent.children[0] == child1
        assert parent.children[1] == child2

    def test_node_construction_nested_children(self):
        grandchild = Node(4, "grandchild", "leaf", [])
        child = Node(3, "child", "branch", [grandchild])
        parent = Node(1, "parent", "root", [child])

        assert parent.children[0].children[0] == grandchild
        assert parent.children[0].children[0].name == "grandchild"

    def test_add_child(self):
        parent = Node(1, "parent", "parent", [])
        child = Node(2, "child", "child", [])

        parent.add_child(child)

        assert len(parent.children) == 1
        assert parent.children[0] == child


class TestShowTree:
    def setup_method(self):
        # Reset next_id before each test
        Node.next_id = 1

    def test_show_tree_single_node(self, capsys):
        node = Node(0, "root", "root", [])

        show_tree(node)

        captured = capsys.readouterr()
        assert "root (root : 1)" in captured.out

    def test_show_tree_with_children(self, capsys):
        child1 = Node(0, "child1", "leaf", [])
        child2 = Node(0, "child2", "leaf", [])
        parent = Node(0, "parent", "root", [child1, child2])

        show_tree(parent)

        captured = capsys.readouterr()
        lines = captured.out.strip().split('\n')
        assert "parent (root : 1)" in lines[0]
        assert "  child1 (leaf : 2)" in lines[1]
        assert "  child2 (leaf : 3)" in lines[2]

    def test_show_tree_nested_children(self, capsys):
        grandchild = Node(0, "grandchild", "leaf", [])
        child = Node(0, "child", "branch", [grandchild])
        parent = Node(0, "parent", "root", [child])

        show_tree(parent)

        captured = capsys.readouterr()
        lines = captured.out.strip().split('\n')
        assert "parent (root : 1)" in lines[0]
        assert "  child (branch : 2)" in lines[1]
        assert "    grandchild (leaf : 3)" in lines[2]

    def test_show_tree_assigns_sequential_ids(self, capsys):
        nodes = [
            Node(0, "node1", "type1", []),
            Node(0, "node2", "type2", []),
            Node(0, "node3", "type3", [])
        ]
        root = Node(0, "root", "root", nodes)

        show_tree(root)

        captured = capsys.readouterr()
        lines = captured.out.strip().split('\n')
        assert ": 1)" in lines[0]  # root gets id 1
        assert ": 2)" in lines[1]  # first child gets id 2
        assert ": 3)" in lines[2]  # second child gets id 3
        assert ": 4)" in lines[3]  # third child gets id 4
