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
set<pii> visited;

void topsort(pii v, stack<pii>& stack)
{
  visited.insert(v);
  forzin(graph[v])
    if notin(z,visited) topsort(z,stack);

  stack.push(v);
}


// Driver code
int main()
{
	// Create a graph given in the above diagram=
  graph[5].push_back(2);
  graph[5].push_back(0);
  graph[4].push_back(0);
  graph[4].push_back(1);
  graph[2].push_back(3);
  graph[3].push_back(1);

  stack<pii> ts;
  //NB: i only run ts if not already visited
  fi(0,6) if notin(i,visited) topsort(i,ts);

  printf("here is the stack:\n");
  while(!ts.empty()){
    auto z = ts.top();
    ts.pop();
    printf("%d,",z);
  }

  cout << endl;
	return 0;
}
