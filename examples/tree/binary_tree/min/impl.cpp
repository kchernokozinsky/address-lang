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

int main() {
    Node* root = new Node(100);
    Node* result;

    insert(root, new Node(20), result);
    insert(root, new Node(50), result);
    insert(root, new Node(300), result);
    insert(root, new Node(150), result);
    insert(root, new Node(10), result);


    std::cout << "----------" << std::endl;
    Node* m;
    min(root, m);
    std::cout << m->value << std::endl;

    return 0;
}