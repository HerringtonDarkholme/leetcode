/**
 * Definition for Node.
 * class Node {
 *     val: number
 *     neighbors: Node[]
 *     constructor(val?: number, neighbors?: Node[]) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.neighbors = (neighbors===undefined ? [] : neighbors)
 *     }
 * }
 */

function cloneGraph(node: Node | null): Node | null {
    const seen = new Map()
    return clone(node, seen)
};

function clone(node: Node | null, seen: Map<Node, Node>): Node | null {
    if (!node) {
        return node;
    }
    if (seen.has(node)) {
        return seen.get(node)
    }
    let cloned = new Node(node.val, [])
    seen.set(node, cloned)
    const neighbors = node.neighbors.map(n => clone(n, seen))
    cloned.neighbors = neighbors
    return cloned
}
