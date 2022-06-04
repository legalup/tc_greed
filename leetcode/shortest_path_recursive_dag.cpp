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
map<pii,int> dist; //is the distance to the end

int shortest_path(pii start)
{

  if isin(start,dist) return dist[start];
    
  dist[start]=1e9;
  forzin(graph[start]) dist[start] = min(dist[start], weights[{start,z}]+shortest_path(z));

  return dist[start];
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


  
  fi(0,n){
    //here, we are telling it what the end is.
    dist.clear();
    dist[i]=0;
    printf("\nshortest path from 1 to %d is %d", i, shortest_path(1));
  }
                     

  cout << endl;
	return 0;
}
