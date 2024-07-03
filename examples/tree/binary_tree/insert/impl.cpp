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

    return 0;
}