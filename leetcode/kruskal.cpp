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
using pii=pair<int,int>;
using ll=long long;
using ull=unsigned long long;
using bits=bitset<50>;
using vert=int;
using edge=pair<vert,vert>;

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

/********************************
 * Given an undirected connected and weighted grasph, find the minimum
 * spanning tree. this uses the union find algo. this also works
 * if vert type is set to pair<int,int>. except for main, of course.
 *
 * Huzzah!
 ****************************************/

set<vert> verts;
vector<tuple<int, vert, vert>> weights;

//////////////////////////////////////////////
// disjoint union. great for finding cc of an image and min spanning sets
//////////////////////////////////////////////////////////////

map<vert, vert> parent;
map<vert,int> rnk; //this is used to determine whcih parent to go with, in union


//this feature has path compression 
vert find_set(vert v) {
  if (v == parent[v])
    return v;
  return parent[v] = find_set(parent[v]);
}

//this particular union set choose the parent that is the
//smallest in dictionary ordering. theres many other ways to go here
//especially to optimize for size & rank.
void union_sets(vert a, vert b) {
  a = find_set(a);
  b = find_set(b);
  // for now, will use rank. 
  if(rnk[a]<rnk[b]) parent[b]=a;
  else parent[a]=b;
  if(rnk[a]==rnk[b]) rnk[b]++;

  // can do other things. can look at some other natural ordering, depending
  // on problem 

};

//assuming that verts, graph, and weights have already been constructed

int kruskal()
{
  int mst_wt(0);

  //initialize the parents and rank
  forzin(verts) {
    parent[z]=z;
    rnk[z]=0;
  }
  
  // sort edges
  sort(a2z(weights));

  //here we go
  forzin(weights){
    vert set_u = find_set(get<1>(z));
    vert set_v = find_set(get<2>(z));

    if(set_u != set_v){
      //ok, this selected to join our mst
      mst_wt += get<0>(z);

      union_sets(set_u, set_v);
      
    }   
  };
  
  return mst_wt;
};

// Driver code
int main()
{
	// Create a graph given in the above diagram=
  // here we are assuming pii is of type int
  auto addEdge = [&](int u, int v, int wt){

    verts.insert(u);
    verts.insert(v);
    weights.push_back({wt,u,v});
  };

  addEdge(0, 1, 4);
  addEdge(0, 7, 8);
  addEdge(1, 2, 8);
  addEdge(1, 7, 11);
  addEdge(2, 3, 7);
  addEdge(2, 8, 2);
  addEdge(2, 5, 4);
  addEdge(3, 4, 9);
  addEdge(3, 5, 14);
  addEdge(4, 5, 10);
  addEdge(5, 6, 2);
  addEdge(6, 7, 1);
  addEdge(6, 8, 6);
  addEdge(7, 8, 7);

  
  cout << "mst weight is:" << kruskal() << endl;
	return 0;
}
