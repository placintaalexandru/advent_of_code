from typing import TypeVar, Dict, Optional, Tuple, List

TreeNode = TypeVar('TreeNode')


class TreeNode:
    parent: Optional[TreeNode]
    children: Dict[str, TreeNode]

    def __init__(self, name: str, parent: TreeNode):
        self.parent = parent
        self.name = name
        self.children = {}
        self.size = None

    @staticmethod
    def parse(path: str) -> Tuple[TreeNode, List[TreeNode]]:
        result = crt = TreeNode('/', None)
        dirs = [crt]

        with open(path, 'r') as f:
            for line in f.readlines():

                tokens = line.strip().split(' ')

                if tokens[0] == '$':
                    if tokens[0] == 'ls':
                        continue

                    if tokens[1] == 'cd':
                        dst = tokens[2]

                        if dst == '..':
                            if crt.parent:
                                crt = crt.parent
                        else:
                            if dst == '/':
                                crt = result
                            elif dst in crt.children:
                                crt = crt.children[dst]
                elif tokens[0] == 'dir':
                    crt.children[tokens[1]] = TreeNode(tokens[1], crt)
                    dirs.append(crt.children[tokens[1]])
                else:
                    crt.children[tokens[1]] = TreeFile(tokens[1], int(tokens[0], 10), crt)

        return result, dirs

    @staticmethod
    def size(tree: TreeNode) -> int:
        size = 0

        for name, child in tree.children.items():
            if isinstance(child, TreeFile):
                size += child.size
                continue

            size += TreeNode.size(child)

        tree.size = size
        return size

class TreeFile:
    name: str
    size: int
    parent: TreeNode

    def __init__(self, name: str, size: int, parent: TreeNode):
        self.name = name
        self.size = size
        self.parent = parent

    def __repr__(self):
        return f'{self.name}-{self.size}'


def main():
    tree, dirs = TreeNode.parse('src/day7/input')

    TreeNode.size(tree), len(dirs)

    print('Part 1: %d' % sum([d.size for d in dirs if d.size < 100000]))

    need_to_free = 30000000 - (70000000 - tree.size)
    min_to_deleted = float('inf')

    for dir in dirs:
        if need_to_free <= dir.size < min_to_deleted:
            min_to_deleted = dir.size

    print('Part 2: %d' % min_to_deleted)


if __name__ == '__main__':
    main()
