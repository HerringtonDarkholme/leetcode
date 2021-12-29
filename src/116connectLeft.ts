function connect(root: Node | null): Node | null {
    connectLeft(root, null)
    return root
};
    

function connectLeft(prev: Node|null, next: Node|null) {
    if (!prev) return
    prev.next = next
    connectLeft(prev.right, next?.left)
    connectLeft(prev.left, prev.right)
    // PERFECT BINARY TREE!
}
