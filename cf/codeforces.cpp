#ifndef _GLIBCXX_NO_ASSERT
#include <cassert>
#endif
#include <cctype>
#include <cerrno>
#include <cfloat>
#include <ciso646>
#include <climits>
#include <clocale>
#include <cmath>
#include <csetjmp>
#include <csignal>
#include <cstdarg>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>

#if __cplusplus >= 201103L
#include <ccomplex>
#include <cfenv>
#include <cinttypes>
#include <cstdbool>
#include <cstdint>
#include <ctgmath>
#include <cwchar>
#include <cwctype>
#endif

#include <algorithm>
#include <bitset>
#include <complex>
#include <deque>
#include <exception>
#include <fstream>
#include <functional>
#include <iomanip>
#include <ios>
#include <iosfwd>
#include <iostream>
#include <istream>
#include <iterator>
#include <limits>
#include <list>
#include <locale>
#include <map>
#include <memory>
#include <new>
#include <numeric>
#include <ostream>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <streambuf>
#include <string>
#include <typeinfo>
#include <utility>
#include <valarray>
#include <vector>

#if __cplusplus >= 201103L
#include <array>
#include <atomic>
#include <chrono>
#include <condition_variable>
#include <forward_list>
#include <future>
#include <initializer_list>
#include <mutex>
#include <random>
#include <ratio>
#include <regex>
#include <scoped_allocator>
#include <system_error>
#include <thread>
#include <tuple>
#include <typeindex>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#endif

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

typedef deque<int> di;
typedef vector<int> vi;
typedef set<int> si;
typedef unsigned long long ull;

template <class T>
void printv(vector<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}

//#define MAXN 100
//int dp[MAXN][MAXN][MAXN][MAXN];
//memset(dp, -1, sizeof(dp));

int
main(int argc, char**argv)
{

  ios_base::sync_with_stdio(false); cout.setf(ios::fixed); cout.precision(20);
  
}