#include <iostream>

struct Node {
    int value;
    Node* left;
    Node* right;

    Node(int val) : value(val), left(nullptr), right(nullptr) {}
};

void insert(Node*& root, Node* node, Node*& result) {
    if (!root) {
        root = node;
        result = node;
    } else if (node->value < root->value) {
        if (!root->left) {
            root->left = node;
            result = node;
        } else {
            insert(root->left, node, result);
        }
    } else {
        if (!root->right) {
            root->right = node;
            result = node;
        } else {
            insert(root->right, node, result);
        }
    }
}

void has_one_son(Node* node, Node*& one_son) {
    if (!node->left && !node->right) {
        one_son = nullptr;
    } else if (!node->left) {
        one_son = node->right;
    } else if (!node->right) {
        one_son = node->left;
    } else {
        one_son = nullptr;
    }
}


void is_leaf(Node* node, bool& result) {
    result = !node->left && !node->right;
}

void min(Node* node, Node*& leaf) {
    if (!node) {
        leaf = nullptr;
        return;
    }

    Node* current = node;
    while (current && current->left) {
        current = current->left;
    }
    leaf = current;
}

void rem(Node* father, Node*& node, int val, Node*& result) {
    if (!node) {
        std::cout << "Element not found" << std::endl;
        result = nullptr;
        return;
    }

    if (node->value == val) {
        bool isLeaf;
        is_leaf(node, isLeaf);

        if (isLeaf) {
            if (father) {
                if (father->left == node) {
                    father->left = nullptr;
                } else {
                    father->right = nullptr;
                }
            } else {
                node = nullptr;
            }
            delete node;
            result = nullptr;
            return;
        }

        Node* one_son;
        has_one_son(node, one_son);

        if (one_son) {
            if (father) {
                if (father->left == node) {
                    father->left = one_son;
                } else {
                    father->right = one_son;
                }
            } else {
                node = one_son;
            }
            delete node;
            result = one_son;
            return;
        }

        Node* leaf;
        min(node->right, leaf);

        node->value = leaf->value;
        rem(node, node->right, leaf->value, result);
        return;
    }

    if (val < node->value) {
        if (!node->left) {
            std::cout << "Element not found" << std::endl;
            result = nullptr;
            return;
        }
        rem(node, node->left, val, result);
    } else {
        if (!node->right) {
            std::cout << "Element not found" << std::endl;
            result = nullptr;
            return;
        }
        rem(node, node->right, val, result);
    }
}

void print_tree(Node* tree) {
    if (!tree) return;
    print_tree(tree->left);
    std::cout << tree->value << std::endl;
    print_tree(tree->right);
}

int main() {
    Node* root = new Node(100);
    Node* result;

    insert(root, new Node(20), result);
    insert(root, new Node(50), result);
    insert(root, new Node(300), result);
    insert(root, new Node(150), result);
    insert(root, new Node(10), result);

    print_tree(root);
    std::cout << "----------" << std::endl;

    rem(nullptr, root, 20, result);
    print_tree(root);

    return 0;
}