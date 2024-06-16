/*
A gene string can be represented by an 8-character long string, with choices from 'A', 'C', 'G', and 'T'.

Suppose we need to investigate a mutation from a gene string startGene to a gene string endGene where one mutation is defined as one single character changed in the gene string.

    For example, "AACCGGTT" --> "AACCGGTA" is one mutation.

There is also a gene bank bank that records all the valid gene mutations. A gene must be in bank to make it a valid gene string.

Given the two gene strings startGene and endGene and the gene bank bank, return the minimum number of mutations needed to mutate from startGene to endGene. If there is no such a mutation, return -1.

Note that the starting point is assumed to be valid, so it might not be included in the bank.

 

Example 1:

Input: startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"]
Output: 1

Example 2:

Input: startGene = "AACCGGTT", endGene = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
Output: 2

 

Constraints:

    0 <= bank.length <= 10
    startGene.length == endGene.length == bank[i].length == 8
    startGene, endGene, and bank[i] consist of only the characters ['A', 'C', 'G', 'T'].

 */

class Solution {
public:

    map<string, int> cost;
    set<string> banky;
    string endg;

    vector<string> nbrs(string ss){

        vector<string> ret;
        int sz = ss.size();
        vector<char> chs = {'A','G','C','T'};

        for(int i=0; i<sz; i++){
            for(auto ch : chs){
                string tmp = ss;
                tmp[i] = ch;
                if(tmp != ss && banky.contains(tmp)) ret.push_back(tmp);
            }
            
        }

        return ret;
    }
    
    int minMutation(string startGene, string endGene, vector<string>& bank) {

        banky = set<string>(bank.begin(),bank.end());
        endg = endGene;

        auto cmp = [&] (string &lft, string &rt)->bool{
            return cost[lft]>cost[rt];
        };
        std::priority_queue<string, vector<string>, decltype(cmp)> pq(cmp);

        cost[startGene] = 0;
        pq.push(startGene);

        while(!pq.empty()){
            auto z = pq.top();
            pq.pop();

            if(z==endGene) return cost[endGene];
            for(auto nbr : nbrs(z)) {
                if(!cost.contains(nbr)){ cost[nbr] = cost[z] + 1; pq.push(nbr);}
                else{
                    if(cost[z]+1<cost[nbr]){
                        cost[nbr] = cost[z]+1;
                        pq.push(nbr);
                    }   
                }
            }


        }
        
        return -1;
        
    }
};
