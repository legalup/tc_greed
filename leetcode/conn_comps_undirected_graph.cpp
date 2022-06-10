// C++ program to print connected components in
// an undirected graph


#include <iostream>
#include <fstream>
#include <sstream>
#include <iterator>
#include <limits>
#include <queue>
#include <vector>
#include <tuple>
#include <unordered_map>
#include <map>
#include <set>
#include <list>
#include <string>
#include <algorithm>
#include <bitset>
#include <numeric>
#include <cstring>
#include <cstdlib>
#include <cmath>
        
using namespace std;


#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define fk(a, b) for(int k=((int)(a)); k < ((int)(b)); k++)
#define fie(a, b) for(int i=((int)(a)); i <= ((int)(b)); i++)
#define fje(a, b) for(int j=((int)(a)); j <= ((int)(b)); j++)
#define fke(a, b) for(int k=((int)(a)); k <= ((int)(b)); k++)
#define isin(a, aset) (aset.count((a))>0)
#define notin(a, aset) (!aset.count((a))>0)
#define a2z(v)            (v).begin(),(v).end()

#define MAXTOP 10000000 //this is the biggest topcoder can do

using di=deque<int>;
using vi=vector<int>;
using si=set<int>;
using pii=pair<int,int>;
using ll=long long;
using ull=unsigned long long;
using bits=bitset<50>;
using vert=int;

using namespace std;

std::ostream& operator<<(std::ostream& strm,
                         const std::pair<int,int>& kvPair)
{
  strm << "(" << kvPair.first << "," << kvPair.second << ")";
  return strm;
}

int n;
vector<vert> verts;
map<vert, vector<vert>> graph ;
set<vert> used;
set<vert> comp ;

void dfs(vert v) {
  used.insert(v) ;
  comp.insert(v);
  for(auto to : graph[v]) {
    if notin(to,used) dfs(to);
  }
}

void find_comps() {
  used.clear();
  for(const auto & i : verts){
    if notin(i,used) {
      comp.clear();
      dfs(i);
      cout << "Component:" ;
      for(auto cj : comp) cout << ' ' << cj;
      cout << endl ;
    }
  }
}

// Driver code
int main()
{
	// Create a graph given in the above diagram=
	cout << "Following are connected components \n";
  n = 100;
  int p=7;
  fi(0,n) verts.push_back(i);
  fi(0,n)fj(0,n){
    if((i%p)==(j%p)){
      graph[i].push_back(j);
      graph[j].push_back(i);
    }
  };

  find_comps();
	return 0;
}
