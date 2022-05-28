/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* parent;
};
*/
#define a2z(v)            (v).begin(),(v).end()

class Solution {
public:
    Node* lowestCommonAncestor(Node* p, Node * q) {
      vector<Node*> pp = getparents(p);
      vector<Node*> pq = getparents(q);

      auto vit = find_first_of(a2z(pp),a2z(pq));
      return *vit;
          
    }
  vector<Node*> getparents(Node * x)
  {
    vector<Node*> ret;
    ret.push_back(x);
    while(x->parent){
      x=x->parent;
      ret.push_back(x);
    }
    return ret;
  }
};
