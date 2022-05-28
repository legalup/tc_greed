class Solution {
public:

  //ANOTHER cmp for pii!
   auto cmp = [&](pii left, pii right)->{
          if(left.first != right.first) return left.first<right.first;
          else return left.second < right.second;
        };
   

    string customSortString(string order, string s) {
        int sz = order.size();
         map<char,int> ord;
            
        for(int i=0;i<sz;i++){
            char ch = order[i];
            ord[ch]=i;
        }
        
        auto cmp = [&](char left, char right) { return ord[left]>ord[right]; };
        std::priority_queue<char, std::vector<char>, decltype(cmp)> pq(cmp);
        
        deque<int> inds;
        for(int i=0;i<s.size();i++){
            char ch = s[i];
            if(ord.count(ch)>0){
                pq.push(ch);
                inds.push_back(i);
            }
        }
        
        for(int j:inds){
            char ch = pq.top();
            pq.pop();
            s[j]=ch;
        }
        
        return s;
    }
};
