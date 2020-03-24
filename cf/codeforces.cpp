

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

#define MAXTOP 10000000 //this is the biggest topcoder can do

using di=deque<int>;
using vi=vector<int>;
using si=set<int>;
using ull=unsigned long long;
using pii=pair<int,int>;


template <class T>
void printv(vector<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}

#define MAXN 50




// int
// main(int argc, char**argv)
// {

//   ios_base::sync_with_stdio(false); cout.setf(ios::fixed); cout.precision(20);
//   string str = "abc";

//   int sz = str.size();
//   cout << "the last char of str is:" << str[sz-1] << endl;
    
  
// }
 
