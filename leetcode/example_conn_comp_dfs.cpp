

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
#define len(v)   (v).size()

#define MAXTOP 10000000 //this is the biggest topcoder can do

using di=deque<int>;
using vi=vector<int>;
using si=set<int>;
using pii=pair<int,int>;
using vpii=vector<pii>;
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

struct TheQuestForGold
{
  vector<string> cave;
  int n;
  map<pii, vector<pii>> graph ;
  set<pii> used;
  int rows,cols;
  string explore(vector<string> cave)
  {
    this->cave = cave;
    rows = len(cave);
    cols = len(cave[0]);
    auto isslime = [&](pii v)->bool{

      for(auto z : getadj(v)) if(cave[z.first][z.second]=='P') return true;
     
      return false;
    };

    pii start = {-1,-1};
    fi(0,rows)fj(0,cols){
      if(isslime({i,j}) && this->cave[i][j] != 'T')
      {this->cave[i][j]='X';}
      if(this->cave[i][j]=='S') start = {i,j};
    }

    if(start.first<0) return "no gold";

    if(dfs(start)) return "gold";
    return "no gold";
    
  }

  vpii getadj(pii v)
  {
    vpii ret;
    if(v.first>0) ret.push_back({v.first-1,v.second});
    if(v.first<rows-1) ret.push_back({v.first+1,v.second});
    if(v.second>0) ret.push_back({v.first,v.second-1});
    if(v.second<cols-1) ret.push_back({v.first,v.second+1});
    return ret;
  }
  vpii getnbrs(pii v)
  {
    vpii tret = getadj(v);
    vpii ret;
    for(auto z : tret) if(cave[z.first][z.second]=='.' || cave[z.first][z.second]=='T' ) ret.push_back(z);
    return ret;
    
  }

  bool dfs(pii v) {
    used.insert(v) ;
    for(auto to : getnbrs(v)) {
      if(cave[to.first][to.second]=='T') {
        return true;
      }
      if notin(to,used) {
          if(dfs(to)) return true;
        }
    }

    return false;
  }


};

// CUT begin
ifstream data("TheQuestForGold.sample");

string next_line() {
    string s;
    getline(data, s);
    return s;
}

template <typename T> void from_stream(T &t) {
    stringstream ss(next_line());
    ss >> t;
}

void from_stream(string &s) {
    s = next_line();
}

template <typename T> void from_stream(vector<T> &ts) {
    int len;
    from_stream(len);
    ts.clear();
    for (int i = 0; i < len; ++i) {
        T t;
        from_stream(t);
        ts.push_back(t);
    }
}

template <typename T>
string to_string(T t) {
    stringstream s;
    s << t;
    return s.str();
}

string to_string(string t) {
    return "\"" + t + "\"";
}

bool do_test(vector<string> cave, string __expected) {
    time_t startClock = clock();
    TheQuestForGold *instance = new TheQuestForGold();
    string __result = instance->explore(cave);
    double elapsed = (double)(clock() - startClock) / CLOCKS_PER_SEC;
    delete instance;

    if (__result == __expected) {
        cout << "PASSED!" << " (" << elapsed << " seconds)" << endl;
        return true;
    }
    else {
        cout << "FAILED!" << " (" << elapsed << " seconds)" << endl;
        cout << "           Expected: " << to_string(__expected) << endl;
        cout << "           Received: " << to_string(__result) << endl;
        return false;
    }
}

int run_test(bool mainProcess, const set<int> &case_set, const string command) {
    int cases = 0, passed = 0;
    while (true) {
        if (next_line().find("--") != 0)
            break;
        vector<string> cave;
        from_stream(cave);
        next_line();
        string __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(cave, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1654169023;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 500 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
    }
    return 0;
}

int main(int argc, char *argv[]) {
    cout.setf(ios::fixed, ios::floatfield);
    cout.precision(2);
    set<int> cases;
    bool mainProcess = true;
    for (int i = 1; i < argc; ++i) {
        if ( string(argv[i]) == "-") {
            mainProcess = false;
        } else {
            cases.insert(atoi(argv[i]));
        }
    }
    if (mainProcess) {
        cout << "TheQuestForGold (500 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end

