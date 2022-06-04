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
#include <stack>
#include <string>
#include <algorithm>
#include <bitset>
#include <numeric>
#include <cstring>
#include <cstdlib>
#include <cmath>
#include <cassert>
        
using namespace std;

#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define fk(a, b) for(int k=((int)(a)); k < ((int)(b)); k++)
#define fie(a, b) for(int i=((int)(a)); i <= ((int)(b)); i++)
#define fje(a, b) for(int j=((int)(a)); j <= ((int)(b)); j++)
#define fke(a, b) for(int k=((int)(a)); k <= ((int)(b)); k++)
#define isin(a, aset) (aset.count((a))>0)
#define notin(a, aset) (aset.count((a))==0)
#define a2z(v)            (v).begin(),(v).end()
#define len(v)   (v).size()
#define forzin(acont) for(auto z : acont)
  
#define MAXTOP 10000000 //this is the biggest topcoder can do

using di=deque<int>;
using vi=vector<int>;
using si=set<int>;
using pii=int; //pair<int,int>;
using ll=long long;
using ull=unsigned long long;
using bits=bitset<50>;

template <class T>
void printv(vector<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}

std::ostream& operator<<(std::ostream& strm, const std::pair<int,int>& kvPair)
{
 strm << "(" << kvPair.first << "," << kvPair.second << ")";
 return strm;
}

//#define MAXN 100
//int dp[MAXN][MAXN][MAXN][MAXN];
//memset(dp, -1, sizeof(dp));
//CODE STARTS NOW<<<<<<<<<<<<<<<<<<<<<<<<<<<


int n;
map<pii, vector<pii>> graph ;
map<pair<pii,pii>,int> weights;
set<pii> visited;
map<pii,int> dist; 

void topsort(pii v, stack<pii>& stack)
{
  visited.insert(v);
  forzin(graph[v])
    if notin(z,visited) topsort(z,stack);

  stack.push(v);
}


void shortest_path(pii start)
{
  //do topological sort first, for the nonrecursive aspect
  stack<pii> ts;

  //iterated through all nodes. for this example, assume nodes are ints in range(0,n)
  fi(0,n) if notin(i,visited) topsort(i,ts);

  //reset the distances
  dist.clear();
  dist[start]=0;

  //now do the relaxation step
  while(!ts.empty()){
    auto u = ts.top();
    ts.pop();

    if isin(u,dist){
        forzin(graph[u]) {
          if isin(z,dist) dist[z] = min(dist[z],dist[u]+weights[{u,z}]);
          else dist[z] = dist[u]+weights[{u,z}];
        };
      };
  };
};


// Driver code
int main()
{
	// Create a graph given in the above diagram=
  n=6;
  graph[0].push_back(1); weights[{0,1}] = 5;
  graph[0].push_back(2); weights[{0,2}] = 3;
  graph[1].push_back(3); weights[{1,3}] = 6;
  graph[1].push_back(2); weights[{1,2}] = 2;
  graph[2].push_back(4); weights[{2,4}] = 4;
  graph[2].push_back(5); weights[{2,5}] = 2;
  graph[2].push_back(3); weights[{2,3}] = 7;
  graph[3].push_back(4); weights[{3,4}] = -1;
  graph[4].push_back(5); weights[{4,5}] = -2;

  shortest_path(1);
  fi(0,n) if isin(i,dist) printf("\nshortest path to %d is %d",i,dist[i]);
                     

  cout << endl;
	return 0;
}
