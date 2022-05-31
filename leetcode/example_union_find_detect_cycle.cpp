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
using pii=int;
using ll=long long;
using ull=unsigned long long;
using bits=bitset<50>;

using namespace std;

std::ostream& operator<<(std::ostream& strm,
                         const std::pair<int,int>& kvPair)
{
  strm << "(" << kvPair.first << "," << kvPair.second << ")";
  return strm;
}



///////////
// this is testing code. here is a graph
////////////////
// a structure to represent an edge in graph
class Edge
{
public:
    int src, dest;
};
 
// a structure to represent a graph
class Graph
{
public:
  Graph(int nV, int nE):numV(nV),numE(nE)
  {
    edge = new Edge[numE * sizeof(Edge)];
  }
    // numV-> Number of vertices, E-> Number of edges
    int numV, numE;
 
    // graph is represented as an array of edges
    Edge* edge;
};

//////////////////////////////////////////////
// disjoint union. great for finding cc of an image and min spanning sets
//////////////////////////////////////////////////////////////

map<pii, pii> parent;

//make sure you initialize this with everything in your set
void make_set(pii v) {
  parent[v] = v;
}

//this feature has path compression 
pii find_set(pii v) {
  if (v == parent[v])
    return v;
  return parent[v] = find_set(parent[v]);
}

//this particular union set choose the parent that is the
//smallest in dictionary ordering. theres many other ways to go here
//especially to optimize for size & rank.
void union_sets(pii a, pii b) {
  a = find_set(a);
  b = find_set(b);
  //in case of  a tie, how to resolve? the answer here is to take smallest index
  //if this were pii, you woudl dictionary ordering
  if(a<=b) parent[a]=b;
  else parent[b]=a; 
    
    // if(a.first==b.first){
    //   if(a.second<b.second) parent[b]=a;
    //   else parent[a]=b;
    // }
    // else {
    //   if(a.first<b.first) parent[b] = a;
    //   else parent[a]=b;
    // }
};
 

// The main function to check whether a given graph contains
// cycle or not
bool isCycle(Graph* graph)
{
 
    // Iterate through all edges of graph, find subset of
    // both vertices of every edge, if both subsets are
    // same, then there is cycle in graph.
    for (int i = 0; i < graph->numE; ++i) {
        auto x = find_set(graph->edge[i].src);
        auto y = find_set(graph->edge[i].dest);
 
        if (x == y)
            return true;

        union_sets(x,y);
    }
    return 0;
}



// Driver code
int main()
{
  /* Let us create the following graph
     0
     | \
     |  \
     1---2 */
  int V = 3, E = 3;
  Graph* graph = new Graph(V, E);
 
  // add edge 0-1
  graph->edge[0].src = 0;
  graph->edge[0].dest = 1;
 
  // add edge 1-2
  graph->edge[1].src = 1;
  graph->edge[1].dest = 2;
 
  // add edge 0-2
  graph->edge[2].src = 0;
  graph->edge[2].dest = 2;
 
  if (isCycle(graph))
    cout << "graph contains cycle";
  else
    cout << "graph doesn't contain cycle";
 
  return 0;
}
