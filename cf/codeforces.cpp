

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



vector<int> biggest_sub(vector<int> as)
{
  vector<int> ret;
  return ret;
  
};

template<typename T>
void run(T & cc){

  int t;
  cc >> t;
  int n;

  for(int i=0; i<t; i++){
    cc >> n;
    vector<int> as;
    for(int j=0; j<n; j++){
      int a;
      cc >> a;
      as.push_back(a);
    }

    vector<int> ret = biggest_sub(as);
    cout << ret.size() << endl;
    for(auto ii : ret) cout << ii << " ";
    cout << endl;
  }
  
}
 int
 main(int argc, char**argv)
 {

   ios_base::sync_with_stdio(false); cout.setf(ios::fixed); cout.precision(20);
   
   //#define TEST

   #ifdef TEST
   ifstream myfile ("data.txt");
   run(myfile);
   #else
   run(cin);
   #endif
  
 }
 
