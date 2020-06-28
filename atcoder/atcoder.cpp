

#include <iostream>
#include <fstream>
#include <sstream>
#include <iterator>
#include <limits>
#include <queue>
#include <vector>
#include <tuple>
#include <unordered_map>
#include <set>
#include <string>
#include <algorithm>
#include <bitset>
#include <numeric>
#include <map>
#include <unordered_map>

using namespace std;


#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define fk(a, b) for(int k=((int)(a)); k < ((int)(b)); k++)
#define fori(a) for(int i=((int)(0)); i < ((int)(a.size())); i++)
#define forj(a) for(int j=((int)(0)); j < ((int)(a.size())); j++)
#define a2z(v)            (v).begin(),(v).end()
#define isin(b,a)        find(all(a),(b))!=(a).end()
#define isnotin(b,a)        find(all(a),(b))==(a).end()
#define len(v)   (v).size()

using graph=map<int,set<int>>;
using pii=pair<int,int>;

struct piilt
{
  bool operator(pii & p1, pii & p2){
    if(p1.first != p2.first) return p1.first < p2.first;
    return p1.second < p2.second;
  }
};

set<int> closers(graph g, graph ps, int n)
{

  int k=(4*n)/7;

  set<int> ret;

  while(ret.size() <= k){
    priority_queue<pii, vector<pii>, piigt> maxheap;
    for(auto ii : g){
      int nn = ii.first;
      int indeg = ps.count(nn)>0 ? ps[nn].size() : 0;
      int outdeg = g.count(nn)>0 ? g[nn].size() : 0;
      indeg += 1;
      if(outdeg>0){
        maxheap.push(make_pair(indeg*outdeg, nn));
      }
    }

    if(maxheap.empty()) return ret;
    auto z = maxheap.top();
    int togo = z.second;
    ret.insert(togo);

    
  }
    
    
  

        
};

template<typename T>
void run(T & cc){

  int t;
  cc >> t;
  int n, m;

  for(int i=0; i<t; i++){
    cc >> n >> m;
    vector<int> starts, ends;

    for(int i=0; i<m; i++){
      int start, end;
      cc >> start >> end;
      starts.push_back(start);
      ends.push_back(end);
    }

    vector<int> ret = closers(starts,ends,n);
    cout << ret.size() << endl;
    for(auto ii : ret) cout << ii << " ";
    cout << endl;
  }
  
}
 int
 main(int argc, char**argv)
 {

   ios_base::sync_with_stdio(false); cout.setf(ios::fixed); cout.precision(20);
   
   #define TEST

   #ifdef TEST
   ifstream myfile ("data.txt");
   run(myfile);
   #else
   run(cin);
   #endif
  
 }
 
