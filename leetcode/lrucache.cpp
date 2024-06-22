/*

Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

Implement the LRUCache class:

    LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
    int get(int key) Return the value of the key if the key exists, otherwise return -1.
    void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.

The functions get and put must each run in O(1) average time complexity.
 */

class LRUCache {
public:

    struct node{

        int key=0;
        int val=0;
        node * next = nullptr;
        node * prev = nullptr;
    };

    map<int,node*> keys;
    node * head = nullptr;
    node * tail = nullptr;

    int cap=0;
    LRUCache(int capacity): cap(capacity) {
        
    }

    void move2head(node * nn){
        if(nn == head) return;

        node * prev = nn->prev;
        node * next = nn->next;

        //update tail
        if(tail == nn){
            tail = prev;
        }

        
        if(prev != nullptr) prev->next = next;
        if(next != nullptr) next->prev = prev;

        //set it to head
        nn->next = head;
        nn->prev = nullptr;
        head->prev = nn;
        head = nn;

        
    }
    
    int get(int key) {
        
        if(keys.contains(key)){
            //remove it
            //is it at head already. if so, do nothing but return val
            node * curr = keys[key];
            move2head(curr);

            //return val
            return curr->val;
        }

        return -1;
    }
    
    void put(int key, int value) {
        
        if(keys.contains(key)){
            node * curr = keys[key];
            curr->val = value;
            move2head(curr);
            return;
        }
        //otherwise, make new node
        node * curr = new node();
        curr->key = key;
        curr->val = value;

        //move it to head
        curr->next = head;
        if(head != nullptr) head->prev = curr;
        head = curr;

        //if tail has never been inited
        if(tail == nullptr) tail = curr;

        //have to update map
        keys[key] = curr;

        //if bigger than cap, removed the last
        if(keys.size() > cap){
            
            node * last = tail;
            int deadkey = last->key;
            
            tail = last->prev;
            tail->next = nullptr;

            //removed from map
            keys.erase(deadkey);
            delete last;
            
        }

    }
};
